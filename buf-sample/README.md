# buf-sample

buf を色々試してみるレポジトリ

## Schema Registory

https://buf.build/grandcolline/buf-sample


## 📝 Note

最初にやったやつ
```bash
buf mod init buf.build/grandcolline/buf-sample
# buf.yaml が作成された

buf registry login
# ~/.netrc に token 情報などが記録されたっぽい
```

proto ファイル作成
```bash
nvim greet/v1/greet.proto

buf format -w
buf lint
```

push
```bash
buf push
```
