package main

import (
	"context"
	"fmt"
	"log"
	"net"
	"os"
	"os/signal"

	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
	"google.golang.org/protobuf/types/known/emptypb"

	grpc_middleware "github.com/grpc-ecosystem/go-grpc-middleware"
	grpc_auth "github.com/grpc-ecosystem/go-grpc-middleware/auth"

	// https://buf.build/grandcolline/buf-sample
	greetv1grpc "buf.build/gen/go/grandcolline/buf-sample/grpc/go/greet/v1/greetv1grpc"
	greetv1 "buf.build/gen/go/grandcolline/buf-sample/protocolbuffers/go/greet/v1"
)

func main() {
	// 1. 8080番portのLisnterを作成
	port := 8080
	listener, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		panic(err)
	}

	// 2. gRPCサーバーを作成
	// s := grpc.NewServer()
	s := grpc.NewServer(grpc_middleware.WithUnaryServerChain(
		grpc_auth.UnaryServerInterceptor(AuthFunc), // AuthFunc を追加
	))

	// 3. gRPCサーバーにGreetingServiceを登録
	greetv1grpc.RegisterGreetServiceServer(s, NewGreetServer())
	greetv1grpc.RegisterHealthServiceServer(s, NewHealthServer())

	// 4. サーバーリフレクションの設定
	reflection.Register(s)

	// 5. 作成したgRPCサーバーを、8080番ポートで稼働させる
	go func() {
		log.Printf("start gRPC server port: %v", port)
		s.Serve(listener)
	}()

	// 6. Ctrl+Cが入力されたらGraceful shutdownされるようにする
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt)
	<-quit
	log.Println("stopping gRPC server...")
	s.GracefulStop()
}

// ----------------------------------------------------------------
// -- Greet Server
// ----------------------------------------------------------------
type GreetServer struct {
	greetv1grpc.UnimplementedGreetServiceServer
}

func NewGreetServer() *GreetServer {
	return &GreetServer{}
}

func (s *GreetServer) Greet(ctx context.Context, req *greetv1.GreetRequest) (*greetv1.GreetResponse, error) {
	return &greetv1.GreetResponse{
		Greeting: fmt.Sprintf("Hello, %s!", req.GetName()),
	}, nil
}

// ----------------------------------------------------------------
// -- Health Server
// ----------------------------------------------------------------
type HealthServer struct {
	greetv1grpc.UnimplementedHealthServiceServer
}

func NewHealthServer() *HealthServer {
	return &HealthServer{}
}

func (s *HealthServer) Check(ctx context.Context, _ *emptypb.Empty) (*greetv1.CheckResponse, error) {
	return &greetv1.CheckResponse{
		Msg: "ok",
	}, nil
}

func (*HealthServer) AuthFuncOverride(ctx context.Context, fullMethodName string) (context.Context, error) {
	fmt.Println("---- AuthFuncOverride -------------------")
	fmt.Println("認証を skip します")
	return ctx, nil
}

// ----------------------------------------------------------------
// -- AuthFunc
// ----------------------------------------------------------------
// AuthFunc は go-grpc-middleware/auth で使用する認証用の関数
// この関数自体は別のパッケージに切り出しても良い
func AuthFunc(ctx context.Context) (context.Context, error) {

	fmt.Println("---- AuthFunc ------------------------")
	// ここでClientから投げられた認証ヘッダの値を取得
	key, err := grpc_auth.AuthFromMD(ctx, "bearer")
	if err != nil {
		return nil, err
	}

	fmt.Println("bearer: " + key)

	// context に Client の情報を詰めて返す
	newCtx := context.WithValue(ctx, "result", "ok")
	return newCtx, nil
}
