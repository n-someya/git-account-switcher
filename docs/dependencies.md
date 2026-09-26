# 利用パッケージ・ライブラリ一覧 (Dependencies)

本プロジェクトで使用している Rust クレートおよびツールの一覧です。
公式ドキュメントへのリンクと本プロジェクトにおける代表的な利用方法をまとめています。

記録日: 2026-09-25

---

## 本番依存パッケージ (Dependencies)

### 1. clap
- **概要**: 型安全で高機能なコマンドライン引数パーサ。
- **公式ドキュメント**: [docs.rs/clap](https://docs.rs/clap/latest/clap/)
- **リポジトリ**: [github.com/clap-rs/clap](https://github.com/clap-rs/clap)
- **本プロジェクトでの使い方**:
  `derive` マクロを用いて CLI コマンドやオプションを構造体・Enum として宣言的に定義します。

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-account-switcher", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init { force: bool },
    Apply { config: Option<std::path::PathBuf> },
}
```

### 2. serde & serde_derive
- **概要**: Rust のデータ構造をシリアライズ / デシリアライズするための標準フレームワーク。
- **公式ドキュメント**: [serde.rs](https://serde.rs/)
- **リポジトリ**: [github.com/serde-rs/serde](https://github.com/serde-rs/serde)
- **本プロジェクトでの使い方**:
  設定ファイル（`accounts.toml`）の構造体を自動シリアライズ・デシリアライズします。

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub roots: Vec<std::path::PathBuf>,
}
```

### 3. toml
- **概要**: TOML 形式の設定ファイルパーサおよびエンコーダ。
- **公式ドキュメント**: [docs.rs/toml](https://docs.rs/toml/latest/toml/)
- **リポジトリ**: [github.com/toml-rs/toml](https://github.com/toml-rs/toml)
- **本プロジェクトでの使い方**:
  マニフェスト文字列から Rust 構造体への変換、および設定の検証を行います。

```rust
let manifest: Manifest = toml::from_str(toml_string)?;
```

### 4. anyhow
- **概要**: アプリケーション層に特化した慣用的なエラーハンドリングライブラリ。
- **公式ドキュメント**: [docs.rs/anyhow](https://docs.rs/anyhow/latest/anyhow/)
- **リポジトリ**: [github.com/dtolnay/anyhow](https://github.com/dtolnay/anyhow)
- **本プロジェクトでの使い方**:
  ファイル I/O やコマンド実行時の失敗に対して、コンテキストメッセージを付加して伝播させます。

```rust
use anyhow::{Context, Result};

let content = std::fs::read_to_string(path)
    .with_context(|| format!("Failed to read file: {}", path.display()))?;
```

### 5. thiserror
- **概要**: 構造体や Enum から `std::error::Error` 実装を自動生成するマクロライブラリ。
- **公式ドキュメント**: [docs.rs/thiserror](https://docs.rs/thiserror/latest/thiserror/)
- **リポジトリ**: [github.com/dtolnay/thiserror](https://github.com/dtolnay/thiserror)
- **本プロジェクトでの使い方**:
  ドメイン層で厳密に区別すべき独自エラー型の定義に使用します。

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Account root does not exist: {0}")]
    RootNotFound(std::path::PathBuf),
}
```

### 6. dirs
- **概要**: 各プラットフォーム（Linux, macOS, Windows）の標準設定・ホームディレクトリを取得するライブラリ。
- **公式ドキュメント**: [docs.rs/dirs](https://docs.rs/dirs/latest/dirs/)
- **リポジトリ**: [github.com/dirs-dev/dirs-rs](https://github.com/dirs-dev/dirs-rs)
- **本プロジェクトでの使い方**:
  `~/.config/gh-multiaccount` や `~/.gitconfig` などの絶対パスを OS 差異なく解決します。

```rust
let config_dir = dirs::config_dir().unwrap().join("gh-multiaccount");
let home_dir = dirs::home_dir().unwrap();
```

### 7. anstream & anstyle
- **概要**: ターミナルのカラー・スタイル機能、および TTY / `NO_COLOR` 自動判定ストリーム。
- **公式ドキュメント**: [docs.rs/anstream](https://docs.rs/anstream/latest/anstream/) / [docs.rs/anstyle](https://docs.rs/anstyle/latest/anstyle/)
- **リポジトリ**: [github.com/rust-cli/anstyle](https://github.com/rust-cli/anstyle)
- **本プロジェクトでの使い方**:
  `Reporter` モジュール（`src/ui.rs`）で ANSI スタイル付きのステータスバッジ（✔, ⚠, ✖）を出力します。パイプ時は自動でエスケープシーケンスを除去します。

```rust
use anstyle::{AnsiColor, Color, Style};

let green = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))).bold();
writeln!(writer, "{green}✔{green:#} {message}")?;
```

---

## 開発・テスト用パッケージ (Dev-Dependencies)

### 1. tempfile
- **概要**: テスト終了時に自動削除される一時ディレクトリおよび一時ファイルの生成。
- **公式ドキュメント**: [docs.rs/tempfile](https://docs.rs/tempfile/latest/tempfile/)
- **リポジトリ**: [github.com/Stebalien/tempfile](https://github.com/Stebalien/tempfile)
- **本プロジェクトでの使い方**:
  ユーザーの実環境を汚さずに、隔離されたディレクトリツリーで `apply` や `git init` を実行します。

```rust
use tempfile::TempDir;

let temp = TempDir::new().unwrap();
let fake_home = temp.path().join("home");
```

### 2. assert_cmd
- **概要**: CLI バイナリを実行し、標準出力・標準エラー出力・終了ステータスをアサートするツール。
- **公式ドキュメント**: [docs.rs/assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/)
- **リポジトリ**: [github.com/assert-rs/assert_cmd](https://github.com/assert-rs/assert_cmd)
- **本プロジェクトでの使い方**:
  ビルドされた `git-account-switcher` コマンド自体の引数処理やヘルプ出力を検証します。

```rust
use assert_cmd::Command;

let mut cmd = Command::cargo_bin("git-account-switcher").unwrap();
cmd.arg("--help").assert().success();
```

### 3. predicates
- **概要**: テスト用のアサーション条件（文字列含有、正規表現一致、ファイル存在等）を宣言的に構築するライブラリ。
- **公式ドキュメント**: [docs.rs/predicates](https://docs.rs/predicates/latest/predicates/)
- **リポジトリ**: [github.com/assert-rs/predicates-rs](https://github.com/assert-rs/predicates-rs)
- **本プロジェクトでの使い方**:
  `assert_cmd` の標準出力検証と組み合わせて使用します。

```rust
use predicates::prelude::*;

cmd.arg("--help")
    .assert()
    .stdout(predicate::str::contains("doctor"));
```

### 4. insta
- **概要**: スナップショットテスト（ゴールデンテスト）ライブラリ。
- **公式ドキュメント**: [insta.rs](https://insta.rs/)
- **リポジトリ**: [github.com/mitsuhiko/insta](https://github.com/mitsuhiko/insta)
- **本プロジェクトでの使い方**:
  生成された `includes.gitconfig` や `ssh/config` などの出力全体が期待通りのテキストと完全一致するかスナップショット比較します。
