package main

import (
	"context"
	"fmt"
	"log"
	"net/http"

	"github.com/bufbuild/connect-go"
	"github.com/rs/cors"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"

	greetv1 "buf.build/gen/go/grandcolline/buf-sample/protocolbuffers/go/greet/v1"
	"buf.build/gen/go/grandcolline/buf-sample/bufbuild/connect-go/greet/v1/greetv1connect"
)

const address = "localhost:8080"

func main() {
	mux := http.NewServeMux()
	path, handler := greetv1connect.NewGreetServiceHandler(&GreetServer{})
	mux.Handle(path, handler)

	log.Println("... Listening on", address)
	http.ListenAndServe(
		address,
		// FIXME: debug のため CORS を allowall で hundler に追加
		cors.AllowAll().Handler(h2c.NewHandler(mux, &http2.Server{})),
	)
}

// ----------------------------------------------------------------
// -- Greet Server
// ----------------------------------------------------------------
type GreetServer struct {
	greetv1connect.UnimplementedGreetServiceHandler
}

func (s *GreetServer) Greet(
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

