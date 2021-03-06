use crate::acc_client::{self, AccClient};
use crate::config::Test;
use crate::{colortext, util};
use clap::{App, Arg, ArgMatches, SubCommand};
use easy_scraper::Pattern;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, process, thread, time};

pub const NAME: &str = "test";

pub const USAGE: &str ="acc test [<CONTEST_NAME>] [<CONTEST_TASK_NAME] <FILE_NAME>

    --- arg -------------------------------------------
      <CONTEST_NAME> <CONTEST_TASK_NAME> <FILE_NAME>
        Specify all
        ex.) $ acc test practice practice_1 p1(.cpp)

      <CONTEST_NAME> <FILE_NAME>
        CONTEST_TASK_NAME and FILE_NAME are the same.
        ex.) $ acc test practice practice_1(.cpp)

      <FILE_NAME>
        Use settings in config.toml and specify the FILE_NAME as task name.
        ex.) $ acc test 1(.cpp)
    ---------------------------------------------------

";


#[derive(Copy, Clone, Eq)]
enum Status {
    AC = 0,
    TLE = 1,
    RE = 2,
    WA = 3,
}

impl Ord for Status {
    fn cmp(&self, other: &Status) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Status) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Status) -> bool {
        *self as i32 == *other as i32
    }
}

impl Status {
    fn to_string(&self) -> String {
        match self {
            Status::AC => colortext::ac(),
            Status::TLE => colortext::tle(),
            Status::RE => colortext::re(),
            Status::WA => colortext::wa(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TestcaseFile {
    testcases: Vec<Testcase>,
}

impl TestcaseFile {
    fn new(testcases: Vec<Testcase>) -> TestcaseFile {
        TestcaseFile {
            testcases: testcases,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Testcase {
    input: String,
    output: String,
}

impl Testcase {
    fn new<S: Into<String>, T: Into<String>>(input: S, output: T) -> Testcase {
        let input = input.into();
        let output = output.into();
        Testcase {
            input: input,
            output: output,
        }
    }
}

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Run tests for <CONTEST_INFO>")
        .usage(USAGE)
        .arg(
            Arg::with_name("CONTEST_INFO")
            .required(true)
            .max_values(3)
        )
}

fn compile(config: &Test, file_name: &str) -> bool {
    let compiler = config.compiler.as_ref().unwrap();
    if config.compile_arg.is_none() {
        util::print_error("compile_arg in config.toml is not defined");
        return false;
    }
    println!("{}: starting compile", colortext::info());
    let arg = config.compile_arg.as_ref().unwrap();
    let arg = arg.replace("<TASK>", file_name);
    let args = arg.split(" ");
    let output = Command::new(compiler)
        .args(args)
        .output()
        .unwrap_or_else(|_| {
            util::print_error("fail to execute compile command");
            process::exit(1);
        });
    let status = output.status;
    if !status.success() {
        let output = String::from_utf8_lossy(&output.stderr);
        util::print_error("failed to compile");
        println!("{}\n\nresult: {}", output, colortext::ce());
        return false;
    }
    println!("{}: compiled successfully\n", colortext::info());
    return true;
}

fn execute(
    config: &Test,
    file_name: &str,
    testcase_input: &str,
    tle_time: u16,
) -> (bool, Option<String>) {
    let input = Command::new("echo")
        .args(&["-e", "-n"])
        .arg(testcase_input)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let input = input.stdout.unwrap();
    let command_name = config.command.replace("<TASK>", file_name);
    let mut command = Command::new(command_name);
    if let Some(arg) = config.command_arg.as_ref() {
        let arg = arg.replace("<TASK>", file_name);
        let args = arg.split(" ");
        command.args(args);
    }
    let mut command_child = command.stdin(input).stdout(Stdio::piped()).spawn().unwrap();
    let start = time::Instant::now();
    loop {
        match command_child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return (true, None);
                }
                let output = command_child.stdout.unwrap();
                let output = Command::new("cat").stdin(output).output().unwrap();
                return (
                    false,
                    Some(String::from_utf8_lossy(&output.stdout).to_string()),
                );
            }
            Ok(None) => {
                let duration = start.elapsed().as_millis();
                if duration > tle_time.into() {
                    let _ = command_child.kill().expect("command wasn't running");
                    return (false, None);
                }
            }
            Err(_e) => {
                util::print_error("command is not available");
                process::exit(1);
            }
        }
        thread::yield_now();
    }
}

pub fn get_testcases(
    contest_name: &str,
    contest_task_name: String,
) -> (Vec<String>, Vec<String>) {
    let mut path = env::current_dir().unwrap();
    path.push("testcase");
    if !path.exists() {
        util::make_dir(path.to_str().unwrap());
    }
    path.push([&contest_task_name, "toml"].join("."));
    let testcase_path = path.as_path();

    // すでにテストケースがあるならそれを返す
    if testcase_path.exists() {
        let testcase_path = testcase_path.to_str().unwrap();
        let content = util::read_file(testcase_path);
        let file: TestcaseFile = toml::from_str(&content).unwrap_or_else(|_| {
            util::print_error("testcase file is wrong");
            process::exit(1);
        });
        let inputs = file.testcases.iter().map(|x| x.input.clone()).collect();
        let outputs = file.testcases.iter().map(|x| x.output.clone()).collect();
        return (inputs, outputs);
    }

    // テストケースをAtCoderから取得
    let client = AccClient::new(true);
    let url = acc_client::TASK_URL.to_string();
    let url = url.replace("<CONTEST>", &contest_name);
    let url = url.replace("<CONTEST_TASK>", &contest_task_name);
    println!("{}: get testcase in \"{}\"", colortext::info(), &url);
    let result = client.get_page(&url).unwrap_or_else(|| {
        util::print_error("The correct test case could not be get");
        process::exit(1);
    });
    let (content, cookies) = result;
    util::save_state(&client.get_csrf_token().unwrap(), cookies);
    let pattern = Pattern::new(acc_client::TESTCASE_PATTERN).unwrap();
    let io_cases = pattern.matches(&content);
    let re = Regex::new(r"<h3>((.|\n)*)</h3><pre>(?P<io>(.|\n)*)</pre>").unwrap();
    let testcases: Vec<String> = io_cases
        .iter()
        .map(|x| re.captures(&x["io"]))
        .filter(Option::is_some)
        .map(|x| x.unwrap().name("io"))
        .filter(Option::is_some)
        .map(|x| x.unwrap().as_str().to_string())
        .collect();
    let inputs: Vec<String> = testcases.iter().step_by(2).cloned().collect();
    let outputs: Vec<String> = testcases.iter().skip(1).step_by(2).cloned().collect();
    if inputs.len() != outputs.len() || inputs.len() == 0 {
        util::print_error("getting testcase is failed");
        process::exit(1);
    }

    // テストケースファイルの作成
    let mut testcases = Vec::<Testcase>::new();
    for (input, output) in inputs.iter().zip(outputs.iter()) {
        testcases.push(Testcase::new(input, output));
    }
    let content = toml::to_string(&TestcaseFile::new(testcases)).unwrap();
    let testcase_path = testcase_path.to_str().unwrap();
    let mut file = File::create(testcase_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    (inputs, outputs)
}

pub fn test(file_name: &str, inputs: &Vec<String>, outputs: &Vec<String>, config: &Test) {
    let mut all_result = Status::AC;
    let mut count = 0;
    let needs_print = config.print_wrong_answer;
    if config.compiler.is_some() {
        let is_completed = compile(&config, file_name);
        if !is_completed {
            return;
        }
    }
    println!("{}: starting test ...", colortext::info());
    for (input, output) in inputs.iter().zip(outputs.iter()) {
        count += 1;
        print!("- testcase {} ... ", count);

        let tle_time = config.tle_time;
        let (caused_runtime_error, result) = execute(config, file_name, input, tle_time);
        if caused_runtime_error {
            all_result = all_result.max(Status::RE);
            println!("{}", colortext::re());
            continue;
        }
        if result.is_none() { all_result = all_result.max(Status::TLE);
            println!("{}", colortext::tle());
            continue;
        }
        let result = util::remove_last_indent(result.unwrap());
        let output = util::remove_last_indent(output);
        let is_correct = result == output;
        let status = if is_correct {
            colortext::ac()
        } else {
            all_result = all_result.max(Status::WA);
            colortext::wa()
        };
        println!("{}", status);
        if !is_correct && needs_print {
            println!("*** wrong answer ***");
            println!("{}", result);
            println!("********************");
        }
    }
    println!("result: {}", all_result.to_string());
}

pub fn run(matches: &ArgMatches) {
    let contest_info: Vec<&str> = matches.values_of("CONTEST_INFO").unwrap().collect();
    let config = util::load_config(true);

    let (contest_name, contest_task_name, file_name) = match contest_info.len() {
        1 => {
            let file_name = contest_info[0];
            let contest_task_name = config.contest_task_name.unwrap_or(config.contest.clone()) + "_" + &util::remove_extension(file_name).to_lowercase();
            (config.contest, contest_task_name, file_name)
        },
        2 => {
            let contest_task_name = util::remove_extension(contest_info[1]);
            (contest_info[0].to_string(), contest_task_name, contest_info[1])
        },
        _ => {
            let contest_task_name = util::remove_extension(contest_info[1]);
            (contest_info[0].to_string(), contest_task_name, contest_info[2])
        },
    };

    let language = if util::has_extension(file_name) {
        let extension = file_name.clone().split_terminator(".").last().unwrap();
        util::select_language(config.languages, &extension).unwrap_or_else(|| {
            util::print_error(format!("language setting for \".{}\" is not found", extension));
            process::exit(1);
        })
    } else {
        let language_name = config.selected_language.unwrap_or_else(|| {
            util::print_error("selected_language setting or file extension is needed");
            process::exit(1);
        });
        config.languages.get(&language_name).unwrap_or_else(|| {
            util::print_error(format!("\"{}\" is not found in languages", language_name));
            process::exit(1);
        }).clone()
    };
    let config = language.test;
    let extension = language.extension;
    let file_name = if util::has_extension(file_name) {
        util::remove_extension(file_name)
    } else {
        file_name.to_string()
    };

    let mut path = env::current_dir().unwrap();
    path.push([file_name.clone(), extension].join("."));
    if !path.exists() {
        util::print_error(format!("{} is not found", path.to_str().unwrap()));
        process::exit(1);
    }

    let (inputs, outputs) = get_testcases(&contest_name, contest_task_name);
    test(&file_name, &inputs, &outputs, &config);
}
