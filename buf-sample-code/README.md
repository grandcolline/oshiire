[buf-sample](../buf-sample/) で、作ったやつを使ってみる。

### go

[grpc/go](https://buf.build/grpc/go) と [protocolbuffers/go](https://buf.build/protocolbuffers/go) を使う。

```
cd go
go run main.go

grpcurl -plaintext localhost:8080 greet.v1.HealthService/Check
```

### connect-go

[bufbuild/connect-go](https://buf.build/bufbuild/connect-go) と [protocolbuffers/go](https://buf.build/protocolbuffers/go) を使う。

### connect-web

[bufbuild/ec](https://buf.build/bufbuild/es) と [bufbuild/connect-web](https://buf.build/bufbuild/connect-web) を使う。

```
cd connect-web
```

```
npm config set @buf:registry  https://buf.build/gen/npm/v1/
npm install @buf/grandcolline_buf-sample.bufbuild_connect-web
npm install @buf/grandcolline_buf-sample.bufbuild_es
```
