[buf-sample](../buf-sample/) で、作ったやつを使ってみる。

### go

[grpc/go](https://buf.build/grpc/go) と [protocolbuffers/go](https://buf.build/protocolbuffers/go) を使う。

```
cd go
go run main.go

grpcurl -plaintext localhost:8080 greet.v1.HealthService/Check
```
