# サーバアプリケーション開発

## 開発環境の準備

### Rust

#### 必須

rustup (Rust ツールチェイン管理ツール)、 rustc (Rust コンパイラ) 、 cargo (Rust パッケージマネージャ、ビルドシステム) の安定版 (stable) のインストール:

```sh
$ curl https://sh.rustup.rs -sSf | sh
```

#### 任意

不安定版 (unstable) の rustc 、 cargo 、 rustfmt (コードフォーマッタ)、 clippy (lint) のインストール:

```sh
$ rustup toolchain install nightly
$ rustup component add rustfmt-preview --toolchain nightly
$ cargo +nightly install -f clippy
```

nightly コンパイラのバージョンと clippy のバージョンによっては、 clippy はビルドに失敗する。
(この場合、 [clippy のリポジトリ](https://github.com/rust-lang-nursery/rust-clippy)の master を使うか、諦める。)


### diesel-cli

データベース(スキーマ)管理ツール。
同時に、 ORM である diesel ライブラリ用のコード生成ツールでもある。

インストール:

```sh
$ cargo install diesel_cli --no-default-features --features "sqlite"
```


## ビルド

`/burning-pro-server` にて実行する。
デバッグビルドされたバイナリが生成される。

```sh
$ cargo build
```

### 型や文法等のチェックのみ行う

バイナリを生成しないかわりに、高速に完了する。
バイナリの生成や実行の必要がないとき(特にコーディング中)は、これをデフォルトとして使うのがよい。

```sh
$ cargo check
```


## 実行

`/burning-pro-server` にて実行する。
必要であればデバッグビルドが実行され、ビルドに成功すればアプリケーションが実行される。

```sh
$ cargo run
```

### リリースビルド

`/burning-pro-server` にて実行する。

デバッグビルドは、バグ検出用のコードやデバッグ情報などを付加してビルドされるため、実行が遅い。
サーバ等で本番運用する場合は、 `--release` フラグを使ってリリースビルドをする必要がある。
ただし、リリースビルドはビルド自体に長時間かかる(2倍かそれ以上を覚悟しておくべき)ため、日頃はデバッグビルドにしておくべきである。

cargo からビルドと実行:

```sh
$ cargo run --release
```

または、一度インストールして、生成物を直接実行:

```sh
$ cargo install .
$ burning-pro-server
```

ただし、直接実行する場合は `cargo install` 先のディレクトリにパスが通っている必要がある。
(docker の `rust` コンテナでは最初から設定済みである。)


## テスト

`/burning-pro-server` にて実行する。

```sh
$ cargo test
```

## ドキュメント生成

`/burning-pro-server` にて実行する。

```sh
$ cargo doc
```

ドキュメント生成後、自動的にブラウザで開くなら:

```sh
$ cargo doc --open
```

## コードフォーマット

不安定版の cargo と rustfmt を用いる。
(安定版は、本プロジェクトで利用する機能が不足している。)

`/burning-pro-server` にて実行する。

```sh
$ cargo +nightly fmt
```

## (参考) lint

不安定版の cargo と clippy を用いる。
正直面倒なので常用する必要はない(が、本当に lint が欲しいなら使った方がよい)。

`/burning-pro-server` にて実行する。

```sh
$ cargo +nightly clippy
```


## 参考 URL

* Rust
    + [インストール · プログラミング言語Rust](https://www.rust-lang.org/ja-JP/install.html)
* diesel\_cli:
    + [Getting Started - Diesel](http://diesel.rs/guides/getting-started/)
