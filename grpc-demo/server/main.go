package main

import (
	"context"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/bufbuild/connect-go"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"

	greetv1 "github.com/grandcolline/oshiire/grpc-demo/gen/greet/v1"
	"github.com/grandcolline/oshiire/grpc-demo/gen/greet/v1/greetv1connect"
	"github.com/rs/cors"
)

const address = "localhost:8080"

func main() {
	mux := http.NewServeMux()
	path, handler := greetv1connect.NewGreetServiceHandler(&GreetServiceServer{})
	mux.Handle(path, handler)

	log.Println("... Listening on", address)
	http.ListenAndServe(
		address,
		// FIXME: debug のため CORS を allowall で hundler に追加
		cors.AllowAll().Handler(h2c.NewHandler(mux, &http2.Server{})),
	)
}

type GreetServiceServer struct {
	greetv1connect.UnimplementedGreetServiceHandler
}

// Unary RPC
func (s *GreetServiceServer) Greet(
	ctx context.Context,
	req *connect.Request[greetv1.GreetRequest],
) (
	*connect.Response[greetv1.GreetResponse],
	error,
) {
	return connect.NewResponse(
		&greetv1.GreetResponse{
			Greeting: fmt.Sprintf("Hello, %s!", req.Msg.GetName()),
		},
	), nil
}

// Server Streaming RPC
func (s *GreetServiceServer) GreetServerStream(
	ctx context.Context,
	req *connect.Request[greetv1.GreetRequest],
	stream *connect.ServerStream[greetv1.GreetResponse],
) error {
	resCount := 5

	for i := 0; i < resCount; i++ {
		if err := stream.Send(&greetv1.GreetResponse{
			Greeting: fmt.Sprintf("[%d] Hello, %s!", i, req.Msg.GetName()),
		}); err != nil {
			return err
		}
		time.Sleep(time.Second * 1)
	}
	return nil
}

// Client Streaming RPC
func (s *GreetServiceServer) GreetClientStream(
	ctx context.Context,
	stream *connect.ClientStream[greetv1.GreetRequest],
) (
	*connect.Response[greetv1.GreetResponse],
	error,
) {
	nameList := make([]string, 0)

	for stream.Receive() {
		nameList = append(nameList, stream.Msg().GetName())
	}
	if err := stream.Err(); err != nil {
		return nil, connect.NewError(connect.CodeUnknown, err)
	}

	return connect.NewResponse(&greetv1.GreetResponse{
		Greeting: fmt.Sprintf("Hello, %v!", nameList),
	}), nil
}

// Bidirectional streaming RPC
func (s *GreetServiceServer) GreetBiStream(
	ctx context.Context,
	stream *connect.BidiStream[greetv1.GreetRequest, greetv1.GreetResponse],
) error {
	for {
		req, err := stream.Receive()
		if errors.Is(err, io.EOF) {
			return nil // nil で正常終了
		}
		if err != nil {
			return err
		}

		if err := stream.Send(&greetv1.GreetResponse{
			Greeting: fmt.Sprintf("Hello, %v!", req.GetName()),
		}); err != nil {
			return err
		}
	}
}
