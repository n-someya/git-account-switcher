# 開発環境ツールチェーン (Development Toolchain)

本プロジェクトの開発およびビルド・リリース自動化に使用するツールチェーン一覧です。

記録日: 2026-09-24

## ツールチェーン一覧

| ツール名 | バイナリパス | インストールバージョン | 最新バージョン (2026-09-24時点) | 主な役割 |
| :--- | :--- | :--- | :--- | :--- |
| **rustup** | `/home/nsomeya/.cargo/bin/rustup` | `1.29.1` | `1.29.1` | Rust ツールチェーンのインストーラ / バージョン管理マネージャ |
| **rustc** | `/home/nsomeya/.cargo/bin/rustc` | `1.98.1` (stable) | `1.98.1` (stable) | Rust コンパイラ本体 |
| **cargo** | `/home/nsomeya/.cargo/bin/cargo` | `1.98.1` | `1.98.1` | Rust パッケージマネージャおよびビルドツール |
| **cargo-dist (`dist`)** | `/home/nsomeya/.cargo/bin/dist` | `0.33.0` | `0.33.0` | マルチプラットフォーム向けビルド・パッケージング・リリース自動化ツール |

---

## 環境変数 / PATH 設定

上記バイナリはすべて `$HOME/.cargo/bin` 配下に配置されています。
シェルから直接コマンドを実行するため、シェルの設定ファイル（`~/.bashrc` や `~/.zshrc` など）に以下のパス設定が含まれていることを確認してください。

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
または
```bash
source "$HOME/.cargo/env"
```

## 各ツールの更新コマンド

将来的にツールチェーンを最新化する場合は、以下のコマンドを実行します。

```bash
# rustup 自身の更新
rustup self update

# Rust コンパイラ / cargo の最新化
rustup update stable

# cargo-dist の最新化
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/cargo-dist/releases/latest/download/cargo-dist-installer.sh | sh
```
