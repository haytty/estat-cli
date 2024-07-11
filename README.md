# estat-cli

## 概要

eStat CLIは、日本の統計データを簡単に取得するためのコマンドラインインターフェースです。このツールを使用することで、各種統計データを手軽に利用できます。

## インストール方法

1. Githubからソースコードをダウンロード
    ```bash
    git clone git@github.com:haytty/estat-cli.git
    ```
2. リポジトリに移動
    ```bash
    cd ./estat-cli
    ```
3. cargoを使ってインストール
    ```bash
    cargo install --path .
    ```

## 使用方法

```bash
Usage: estat-cli <COMMAND>

Commands:
  region        
  indicator     
  term          
  social-event  
  stat          
  data          
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## コマンド一覧

[region](https://github.com/haytty/estat-cli/blob/master/src/cli/region/README.md)

[indicator](https://github.com/haytty/estat-cli/blob/master/src/cli/indicator/README.md)

[term](https://github.com/haytty/estat-cli/blob/master/src/cli/term/README.md)

[social-event](https://github.com/haytty/estat-cli/blob/master/src/cli/social_event/README.md)

[stat](https://github.com/haytty/estat-cli/blob/master/src/cli/stat/README.md)

[data](https://github.com/haytty/estat-cli/blob/master/src/cli/data/README.md)

## リンク

[eStat公式サイト](https://www.e-stat.go.jp/)
[eStatダッシュボードページ](https://dashboard.e-stat.go.jp/)
[eStatAPIページ](https://dashboard.e-stat.go.jp/static/api)
