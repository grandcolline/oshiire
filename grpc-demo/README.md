[作ってわかる！ はじめての gRPC](https://zenn.dev/hsaki/books/golang-grpc-starting) のコードを [bufbuild/connect-go](https://github.com/bufbuild/connect-go) で生成したものでかきなおしてみる。

### Refs

- [作ってわかる！ はじめての gRPC](https://zenn.dev/hsaki/books/golang-grpc-starting)
- [connect-webやってみた](https://zenn.dev/silverbirder/articles/aaa2c196991b53)

### Note

```
mkdir -p proto/greet
cd proto
buf mod init
```

```
nvim proto/greet/greet.proto
nvim buf.gen.yaml
```

```
buf generate proto
```

## run

### serve

```
go run server/main.go
```

### access

```bash
# grpcurl
grpcurl -proto proto/greet/v1/greet.proto \
  -plaintext \
  -d '{"name": "john"}' \
  localhost:8080 greet.v1.GreetService/Greet

# curl
curl --header "Content-Type: application/json" \
  -d '{"name": "john"}' \
  localhost:8080/greet.v1.GreetService/Greet
```

```bash
# grpcurl
grpcurl -proto proto/greet/v1/greet.proto \
  -plaintext \
  -d '{"name": "john"}{"name": "paul"}' \
  localhost:8080 greet.v1.GreetService/GreetClientStream
```

```bash
# grpcurl
grpcurl -proto proto/greet/v1/greet.proto \
  -plaintext \
  -d '{"name": "hsaki"}{"name": "a-san"}{"name": "b-san"}{"name": "c-san"}{"name": "d-san"}' \
  localhost:8080 greet.v1.GreetService/GreetBiStream
```

## 参考

- [作ってわかる！ はじめての gRPC](https://zenn.dev/hsaki/books/golang-grpc-starting)
- [次世代 gRPC?『connect-go』やってみた](https://zenn.dev/rai_wtnb/articles/e07ad831ea8e34)
- https://connect.build/docs/introduction
