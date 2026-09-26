# CI/CD パイプライン仕様 (Continuous Integration & Delivery)

本プロジェクトの CI/CD は、Rust エコシステムおよび `cargo-dist` の 2026 年ベストプラクティスに準拠して設計されています。

記録日: 2026-09-26

---

## ワークフローの構成

CI/CD は役割に応じて 2 つの独立したワークフローに分離されています。

| ワークフロー | ファイル | トリガー | 主な役割 |
| :--- | :--- | :--- | :--- |
| **CI** | `.github/workflows/ci.yml` | `pull_request`, `push (main)` | コード品質チェック、マルチプラットフォームテスト、設定ドリフト検出 |
| **Release** | `.github/workflows/release.yml` | `push (tags: v*)` | クロスコンパイル、GitHub Releases 公開、curl/npm インストーラ生成 |

---

## 1. CI ワークフロー (`ci.yml`)

日々の開発と Pull Request で動作する品質ゲートです。

### 主なジョブ
1. **Formatting (`fmt`)**:
   - `cargo fmt --check` を実行し、公式スタイルガイドとの完全一致を検証。
2. **Lint & Clippy (`clippy`)**:
   - `cargo clippy --all-targets -- -D warnings` を実行。潜在的なバグや非推奨パターンを警告ゼロでブロック。
3. **Multi-OS Test (`test`)**:
   - `ubuntu-latest` および `macos-latest` のマトリクスで `cargo test --all-targets` を実行。
   - ADR-0002 / ADR-0003 に記載されたパス判定や大文字小文字（case-insensitive）の挙動を OS 差異を含めて検証。
   - `Swatinem/rust-cache` によるビルド成果物キャッシュを活用して実行時間を短縮。
4. **cargo-dist ドリフト検知 (`dist-check`)**:
   - `dist generate --check` を実行。
   - `dist-workspace.toml` を更新した際に `.github/workflows/release.yml` の再生成をコミットし忘れていないかを自動検知。

---

## 2. Release ワークフロー (`release.yml`)

`cargo-dist` によって自動保守される配布用パイプラインです。

### 動作フロー
1. **プランニング (`plan`)**:
   - バージョンタグ（例: `v0.1.0`）からリリース対象アーティファクトとビルド計画を算出。
2. **クロスコンパイル (`build-local-artifacts`)**:
   - 以下の 5 ターゲット向けに並行ビルドを実行:
     - `x86_64-unknown-linux-gnu` (Linux 64bit)
     - `aarch64-unknown-linux-gnu` (Linux ARM64)
     - `x86_64-apple-darwin` (macOS Intel)
     - `aarch64-apple-darwin` (macOS Apple Silicon)
     - `x86_64-pc-windows-msvc` (Windows 64bit)
3. **配布パッケージ生成 & 公開 (`publish-github-release`)**:
   - tar.gz / zip アーカイブおよび SHA256 チェックサムの生成。
   - curl インストーラ（`git-account-switcher-installer.sh`）の生成。
   - PowerShell インストーラ（`git-account-switcher-installer.ps1`）の生成。
   - npm パッケージの自動 publish（`npx git-account-switcher` 対応）。
   - GitHub Releases へのアセット添付とリリースノート自動作成。

---

## リリース手順

```bash
# 1. Cargo.toml の version を更新
# 2. コミットを作成
git commit -am "chore: release v0.1.0"

# 3. タグを打って push
git tag v0.1.0
git push origin v0.1.0
```
タグが GitHub に push されると、Release ワークフローが自動起動して全配布チャネルへ展開されます。
