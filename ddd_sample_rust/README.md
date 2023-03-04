# ddd_sample_rust

下記を参考に、松岡さんのKotlinのサンプルコードをRustで実装していく。

* [ドメイン駆動設計 サンプルコード&FAQ](https://booth.pm/ja/items/3363104)
* [ライブモデリングとコーディングで理解するDDD - Youtube](https://youtu.be/A2EU0paEVJ0)
* [ライブコーディングで理解するDDDのテスト - Youtube](https://youtu.be/Z3Wc6mfvx7I)

## 実行

### 1. DBセットアップ

```bash
$ docker compose up -d
$ diesel migration run
```

| Name       | Value       |
|:-----------|:------------|
| ホスト     | `127.0.0.1` |
| ポート     | `3306`      |
| ユーザ     | `app`       |
| パスワード | `password`  |

参考: https://ozway.jp/2020/10/rust-mysql-diesel（1）/
