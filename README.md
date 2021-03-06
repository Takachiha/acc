# acc
AtCoderのテストや提出をおこなうコマンド

## ディレクトリ構成
```
<CONTEST_DIR>/
├── A.cpp
├── B.cpp
├── C.cpp
├── D.cpp
├── E.cpp
├── F.cpp
├── config.toml
└── testcase
    ├── A.toml
    ├── B.toml
    ├── C.toml
    ├── D.toml
    ├── E.toml
    └── F.toml
```

## 使い方

### ユーザ情報設定
以下のコマンドでAtCoderにログイン
```bash
$ acc login
```
- atcoderにログインするコマンド
- バージョン1.0.0を使っていたならば\<config\_dir\>/acc/userdata.tomlを削除してください
---

### コンテストプロジェクト作成
\<config\_dir\>/acc/config.tomlとコマンドオプションなどをもとにコンテストのプロジェクトを作成する．

```bash
$ acc init [OPTION] <DIR_NAME>

ex.)
$ acc init abc160 -l python
```

基本的にディレクトリ名をコンテスト名として使用するため，ディレクトリと異なるコンテストディレクトリを作成したい場合は，コマンド実行後にコンテストディレクトリ内のconfig.tomlを編集する必要がある．

オプションは以下の通り，なお，オプションが指定されていないときは\<config\_dir\>/acc/config.tomlで指定された値で解決しようとする．

| &nbsp;&nbsp;&nbsp;OPTION&nbsp;&nbsp;&nbsp; |                                                            説明                                                            |
| :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| -l <br> --lang                             | AtCoderで提出するときの言語を指定する(自分で設定したものを指定する必要)                                                                          |
| -t <br> --total                             | 最初に作成するファイルの数を指定する                                                                         |

---

### テスト
ソースコードのテストを行う.

```bash
$ acc test <CONTEST_INFO>...

ex.)
$ acc test 1
or
$ acc test practice practice_1.cpp
or
$ acc test practice practice_1 p1
```

- コンテストディレクトリ内で実行を行う必要がある．
- 初回実行時AtCoderからテストを取得し，プロジェクト内にtestcase/\<TASK\>.tomlとして保存する．
- テストケースを追加したい場合は，testcaseディレクトリ内のファイルを編集することで対応できる

---

### ファイル監視＆テスト
指定されたタスクのファイル監視をし，変更されたらテストを行う.

```bash
$ acc watch <CONTEST_INFO>...

ex.)
$ acc watch 1
or
$ acc watch practice practice_1.cpp
or
$ acc watch practice practice_1 p1
```

- テスト関連の仕様はtestと同じ

---

### 提出
ソースコードの提出を行う．

```bash
$ acc submit <CONTEST_INFO>...

ex.)
$ acc submit 1
or
$ acc submit practice practice_1.cpp
or
$ acc submit practice practice_1 p1
```
---

## 設定
### 詳細
<language\_name>はTOMLの使用に従う範囲で任意に設定可能

|           項目            |             型              |                              説明                                                       | 備考 |
| :------------------------ | :-------------------------- | --------------------------------------------------------------------------------------- |:----:|
| contest                   | String                      | コンテスト名(コンテストプロジェクト内の設定)                                            | 必須 |
| contest\_task\_name       | String                      | URLのタスク部分のコンテスト名(contestと異なるとき指定)                                  |      |
| selected\_language        | String                      | 使用する言語を指定                                                                      |      |
| languages.<language\_name>.extension                 | String                      | ファイルの拡張子を指定                                       | 必須 |
| languages.<language\_name>.language\_id              | Integer(符号なし１６ビット) | AtCoderの言語指定                                            | 必須 |
| languages.<language\_name>.test.compiler             | String                      | テスト時に使用するコンパイラを指定                           |      |
| languages.<language\_name>.test.compile\_arg         | String                      | コンパイル時のarg指定                                        |      |
| languages.<language\_name>.test.command              | String                      | 実行するコマンドを指定                                       | 必須 |
| languages.<language\_name>.test.command\_arg         | String                      | コマンドを実行するときのarg指定                              |      |
| languages.<language\_name>.test.tle\_time            | Integer(符号なし１６ビット) | TLEの時間指定[ms]                                            | 必須 |
| languages.<language\_name>.test.print\_wrong\_answer | Boolean                     | WAのときの出力                                               | 必須 |

---

### 言語ID一覧

| 使用する言語 | 指定する値 |
| :---: | :---: |
| C (GCC 9.2.1) | 4001 |
| C (Clang 10.0.0) | 4002 |
| C++ (GCC 9.2.1) | 4003 |
| C++ (Clang 10.0.0) | 4004 |
| Java (OpenJDK 11.0.6) | 4005 |
| Python (3.8.2) | 4006 |
| Bash (5.0.11) | 4007 |
| bc (1.07.1) | 4008 |
| Awk (GNU Awk 4.1.4) | 4009 |
| C# (.NET Core 3.1.201) | 4010 |
| C# (Mono-mcs 6.8.0.105) | 4011 |
| C# (Mono-csc 3.5.0) | 4012 |
| Clojure (1.10.1.536) | 4013 |
| Crystal (0.33.0) | 4014 |
| D (DMD 2.091.0) | 4015 |
| D (GDC 9.2.1) | 4016 |
| D (LDC 1.20.1) | 4017 |
| Dart (2.7.2) | 4018 |
| dc (1.4.1) | 4019 |
| Erlang (22.3) | 4020 |
| Elixir (1.10.2) | 4021 |
| F# (.NET Core 3.1.201) | 4022 |
| F# (Mono 10.2.3) | 4023 |
| Forth (gforth 0.7.3) | 4024 |
| Fortran(GNU Fortran 9.2.1) | 4025 |
| Go (1.14.1) | 4026 |
| Haskell (GHC 8.8.3) | 4027 |
| Haxe (4.0.3); js | 4028 |
| Haxe (4.0.3); Java | 4029 |
| JavaScript (Node.js 12.16.1) | 4030 |
| Julia (1.4.0) | 4031 |
| Kotlin (1.3.71) | 4032 |
| Lua (Lua 5.3.5) | 4033 |
| Lua (LuaJIT 2.1.0) | 4034 |
| Dash (0.5.8) | 4035 |
| Nim (1.0.6) | 4036 |
| Objective-C (Clang 10.0.0) | 4037 |
| Common Lisp (SBCL 2.0.3) | 4038 |
| OCaml (4.10.0) | 4039 |
| Octave (5.2.0) | 4040 |
| Pascal (FPC 3.0.4) | 4041 |
| Perl (5.26.1) | 4042 |
| Raku (Rakudo 2020.02.1) | 4043 |
| PHP (7.4.4) | 4044 |
| Prolog (SWI-Prolog 8.0.3) | 4045 |
| PyPy2 (7.3.0) | 4046 |
| PyPy3 (7.3.0) | 4047 |
| Racket (7.6) | 4048 |
| Ruby (2.7.1) | 4049 |
| Rust (1.42.0) | 4050 |
| Scala (2.13.1) | 4051 |
| Java (OpenJDK 1.8.0) | 4052 |
| Scheme (Gauche 0.9.9) | 4053 |
| Standard ML (MLton 20130715) | 4054 |
| Swift (5.2.1) | 4055 |
| Text (cat 8.28) | 4056 |
| TypeScript (3.8) | 4057 |
| Visual Basic (.NET Core 3.1.101) | 4058 |
| Zsh (5.4.2) | 4059 |
| COBOL - Fixed (OpenCOBOL 1.1.0) | 4060 |
| COBOL - Free (OpenCOBOL 1.1.0) | 4061 |
| Brainfuck (bf 20041219) | 4062 |
| Ada2012 (GNAT 9.2.1) | 4063 |
| Unlambda (2.0.0) | 4064 |
| Cython (0.29.16) | 4065 |
| Sed (4.4) | 4066 |
| Vim (8.2.0460) | 4067 |


### 設定例
\<config\_dir\>/acc/config.tomlの設定例
\<TASK\>はacc testで指定したものを代入するためのもの
#### C++

```toml
contest = "config"
selected_language = "cpp_gcc"

[languages.cpp_gcc]
extension = "cpp"
language_id = "4003"

[languages.cpp_gcc.test]
compiler = "g++"
compile_arg = "-std=gnu++17 -o <TASK> <TASK>.cpp"
command = "./<TASK>"
tle_time = 3000
print_wrong_answer = true

[languages.python]
extension = "py"
language_id = "4006"

[languages.python.test]
command = "python3"
command_arg = "<TASK>.py"
tle_time = 3000
print_wrong_answer = true

```

