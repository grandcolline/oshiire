```
# proto 作成
buf generate proto
```

[フロントエンド(Remix)] -> [gRPC Gateway] -> [サーバ(Tonic)]

### サーバ

```
# サーバ起動
cd rust
cargo run
```

```
# サーバに直接アクセス
grpcurl -plaintext -proto proto/greet/v1/greet.proto \
    -d '{"name":"john"}' \
    localhost:50051 greet.v1.GreetService/Greet
```

### gRPC Gateway

```
cd gateway
go run main.go
```

参考:
- https://dev.classmethod.jp/articles/grpc-basic/
