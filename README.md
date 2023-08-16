# m5stack_basic_no_std_template
[M5Stack Basic V2.7](https://www.switch-science.com/products/9010)を```no_std```環境で実行する最小限の構成です。

**Pythonの環境構築**などの準備が必要です。

[Introduction - The Rust on ESP Book](https://esp-rs.github.io/book/introduction.html)に沿って説明します。


# 環境構築
## [3.1. Rust installation](https://esp-rs.github.io/book/installation/rust.html)
### [Visual Studio Installer](https://visualstudio.microsoft.com/ja/downloads/)
下記の3項目をインストールします。
* .NET デスクトップ開発
* C++ によるデスクトップ開発
* ユニバーサル Windows プラットフォーム開発

### [rustup](https://www.rust-lang.org/ja/tools/install)
Rustのビルドツールをインストールします。
```1) Proceed with installation (default)```で進めます。

### [VS Code](https://code.visualstudio.com/)
左側のExtentions(拡張機能)から次の拡張機能をダウンロードします
* ```rust-analyzer```
* ```CodeLLDB```
* ```GitHub Pull Requests and Issues```
* ```Choose a License```

#### 遭遇したトラブル
* エラーメッセージ

```cargo : 用語 'cargo' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれている場合はそのパスが正しいことを確認してから、再試行してください。```

* 対策

環境変数のPATHに```C:\Users\USER_NAME/.cargo/bin```を設定します。
1. ```システム環境変数の編集```
2. ```環境変数(N)```
3. ```PATHを選択して編集(I)```
4. ```新規(N)を選択して環境変数を追加```
5. ```OK```


## (順番を入れ替えて)[3.4. std Development Requirements](https://esp-rs.github.io/book/installation/std-requirements.html)
環境変数に苦しむタイミングを早めます。

### python: Required by ESP-IDF
私は[3.11.4](https://www.python.org/downloads/release/python-3114/)にしました。


* 今回のインストール先

```C:\Users\USER_NAME\AppData\Local\Programs\Python\Python311```

* パス名が長いとエラーになるので下記のフォルダに移して環境変数PATHを設定します。

```C:\Python\Python311```


## [3.3. RISC-V and Xtensa targets](https://esp-rs.github.io/book/installation/riscv-and-xtensa.html)
ツールチェインをインストールします。
### 1. Install espup
```cargo install espup```

### 2. Install Necessary Toolchains
```espup install```

インストールするといくつかの警告がでます。

```[2023-08-15T08:03:06Z WARN ]  Your environments variables have been updated! Shell may need to be restarted for changes to be effective.```
```[2023-08-15T08:03:06Z WARN ]  A file was created at 'C:\Users\USER_NAME\export-esp.ps1' showing the injected environment variables.```

### 3. Set Up the Environment Variables
Windowsの人は前の句で環境変数が構築されているのでスキップです。
* On Windows (%USERPROFILE%\export-esp.ps1)

There is no need to execute the file for Windows users. It is only created to show the modified environment variables.

### Webページにない準備
プログラムをハードウェアに書き込むプログラムをインストールします。

```cargo install cargo-espflash```


## [3.4. std Development Requirements](https://esp-rs.github.io/book/installation/std-requirements.html)
Python等を準備します。
### python: Required by ESP-IDF
3.1. Rust installation で行っていれば飛ばせます。
私は[3.11.4](https://www.python.org/downloads/release/python-3114/)にしました。
Windowsは環境変数PATHの自動設定が失敗しやすいので手動で設定します。

### git: Required by ESP-IDF
https://git-scm.com/download/win


### ldproxy binary crate: A tool that forwards linker arguments to the actual linker that is also given as an argument to ldproxy. Install it by running:
```cargo install ldproxy```


## [4.1. Generating Projects from Templates](https://esp-rs.github.io/book/writing-your-own-application/generate-project/index.html)
テンプレートをダウンロードします。

### 1. Install cargo generate:
```cargo install cargo-generate```

### 2. Generate a project based on one of the templates:
#### std環境はエラー解消できなかったので省略
#### no_std環境
テンプレートをダウンロードします。

```cargo generate esp-rs/esp-template```

いくつかの質問に回答します。
順番は固定ではありません。
*  Project Name: ```m5stack_basic_no_std_template```
* Renaming project called `m5stack_basic_no_std_template` to `m5stack-basic-no-std-template`...
* Destination: C:\Users\USER_NAME\source\RustApps\m5stack-basic-no-std-template ...
* project-name: m5stack-basic-no-std-template ...
* Generating template ...
* ?  Which MCU to target? ・ ```esp32```
* ?  Configure advanced template options? ・ ```true```
* ?  Setup logging using the log crate? ・ ```false```
* ?  Configure project to support Wokwi simulation with Wokwi VS Code extension? ・ ```false```
* ?  Enable allocations via the esp-alloc crate? ・ ```true```
* ?  Add CI files for GitHub Action? ・ ```false```
* ?  Configure project to use Dev Containers (VS Code and GitHub Codespaces)? ・ ```false```

プロジェクト作成に成功すると次のようなメッセージが流れます。

```Moving generated files into: `C:\Users\USER_NAME\source\RustApps\m5stack-basic-no-std-template`...```
```Initializing a fresh Git repository```
```Done! New project created C:\Users\USER_NAME\source\RustApps\m5stack-basic-no-std-template```


### Cargo.tomlファイルにあなたのメールアドレスが記載されているので削除
```cargo generate```の影響かもしれません。

### パニック解析用環境変数設定
デバッグ用途。
```set RUST_BACKTRACE=1```

# プログラム実行
後少しです。
## [4.1. Generating Projects from Templates](https://esp-rs.github.io/book/writing-your-own-application/generate-project/index.html)
### 3. Build/Run the generated project:
#### ビルド
```cargo build```
#### プログラム実行
```cargo espflash flash```

プログラム実行時にM5Stack Basicを接続しているポート番号をPCに伝えます。

入出力インスタンスを書いていないので**何も起きないプログラム**です。
[GitHubに公開されている方々](https://github.com/search?q=m5stack++language%3A+Rust&type=repositories)がいらっしゃいますので、それらを見ながらプログラムを修正してください。

