# データベース管理

## 環境の準備

### diesel\_cli のインストール

```sh
$ cargo install -f diesel_cli --no-default-features --feature "sqlite"
```

もしかすると、 sqlite をローカル環境にインストールしておく必要があるかもしれない。


## データベース操作

### データベース生成、アップグレード

`/burning-pro-server` にて実行する。

```sh
$ diesel migration run
```

### マイグレーション作成

`/burning-pro-server` にて実行する。

```sh
$ diesel migration generate (マイグレーションの名前)
```

これにより、 `/burning-pro-server/migrations/(日時)_(名前)` というディレクトリが生成され、そこに `up.sql` と `down.sql` ファイルが用意される。

### マイグレーションの簡単なテスト

`/burning-pro-server` にて実行する。

```sh
$ diesel migration redo
```

このコマンドは、最新のマイグレーションを取り消して再適用する。
これでエラーにならないことによって、 `up.sql` と `down.sql` が正しく対になる操作をしていることを(完全ではないが)確認できる。


## git によるファイル管理

マイグレーションによって `/burning-pro-server/db.sqlite3` のようなファイルが生成されるが、これは本番環境のサーバにおいて外部データにより更新されうるものである。
よって、 git リポジトリで管理すべきものではない。

一方、マイグレーション (`/burning-pro-server/migrations/`) はデータベースで管理するデータ構造を規定するものであり、アプリケーションのコードはこれを前提として書かれている。
よって、マイグレーションファイル群は git で管理する。

## 本番サーバにおけるデータベース

本番サーバでは、データベースファイルの有無やスキーマのバージョンが不明な状況からアプリケーションが起動できる必要がある。
しかし、 git リポジトリや docker イメージにはデータベースファイルは含まれない。

そのため、サーバアプリケーションが起動時か起動前にマイグレーションの適用やデータベースの作成を行う必要がある。