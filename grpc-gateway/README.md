準備

```
go install github.com/grpc-ecosystem/protoc-gen-grpc-gateway-ts@latest
```

```bash
# proto 作成
buf generate proto
```

[フロントエンド(Remix)] -> [gRPC Gateway] -> [サーバ(rust)]

### サーバ (rust)

```bash
# サーバ起動
cd rust
cargo run
```

- `localshot:50051`

```bash
# サーバに直接アクセス
grpcurl -plaintext -proto proto/greet/v1/greet.proto \
    -d '{"name":"john"}' \
    localhost:50051 greet.v1.GreetService/Greet
```

### gRPC Gateway

```bash
# サーバ起動
cd gateway
go run main.go
```

- `localshot:8081`

```bash
# gateway 経由でアクセス ( rust を起動してあること前提)
curl -X POST -d '{"name":"john"}' \
    http://localhost:8081/greet.v1.GreetService/Greet
```

### Remix

```bash
cd remix
npm run dev
```

- http://localshot:8788
- http://localshot:8788/debug

参考:

- https://dev.classmethod.jp/articles/grpc-basic/
