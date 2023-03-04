package main

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/bufbuild/connect-go"

	greetv1 "github.com/grandcolline/oshiire/grpc-demo/gen/greet/v1"
	"github.com/grandcolline/oshiire/grpc-demo/gen/greet/v1/greetv1connect"
)

var (
	scanner *bufio.Scanner
	client  greetv1connect.GreetServiceClient
)

func main() {
	fmt.Println("start gRPC Client.")

	// 1. 標準入力から文字列を受け取るスキャナを用意
	scanner = bufio.NewScanner(os.Stdin)

	// 2. gRPCサーバーとのコネクションを確立し、クライアントを作成
	client = greetv1connect.NewGreetServiceClient(
		http.DefaultClient,
		"http://localhost:8080",
	)

	for {
		fmt.Println("")
		fmt.Println("1: Greet")
		fmt.Println("2: GreetServerStream")
		fmt.Println("3: GreetClientStream")
		fmt.Println("4: GreetBiStreams")
		fmt.Println("5: exit")
		fmt.Print("please enter >")

		scanner.Scan()
		in := scanner.Text()

		switch in {
		case "1":
			Greet()

		case "2":
			GreetServerStream()

		case "3":
			GreetClientStream()

		case "4":
			GreetBiStreams()

		case "5":
			fmt.Println("bye.")
			goto M
		}
	}
M:
}

func Greet() {
	fmt.Print("Please enter your name > ")
	scanner.Scan()
	name := scanner.Text()

	res, err := client.Greet(
		context.Background(),
		connect.NewRequest(&greetv1.GreetRequest{Name: name}),
	)
	if err != nil {
		log.Println(err)
		return
	}
	log.Println(res.Msg.GetGreeting())
}

// サーバストリーミング
func GreetServerStream() {
	fmt.Print("Please enter your name > ")
	scanner.Scan()
	name := scanner.Text()

	stream, err := client.GreetServerStream(
		context.Background(),
		connect.NewRequest(&greetv1.GreetRequest{Name: name}),
	)
	if err != nil {
		log.Println(err)
		return
	}

	for stream.Receive() {
		log.Println(stream.Msg().GetGreeting())
	}
	if err := stream.Err(); err != nil {
		fmt.Println(err)
	}
}

// クライアントストリーミング
func GreetClientStream() {
	stream := client.GreetClientStream(context.Background())

	sendCount := 5
	fmt.Printf("Please enter %d names.\n", sendCount)
	for i := 0; i < sendCount; i++ {
		scanner.Scan()
		name := scanner.Text()

		if err := stream.Send(&greetv1.GreetRequest{
			Name: name,
		}); err != nil {
			fmt.Println(err)
			return
		}
	}

	res, err := stream.CloseAndReceive()
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(res.Msg.GetGreeting())
	}
}

func GreetBiStreams() {
	stream := client.GreetBiStream(context.Background())

	sendNum := 5
	// fmt.Printf("Please enter %d names.\n", sendNum)

	var sendEnd, recvEnd bool
	sendCount := 0
	for !(sendEnd && recvEnd) {
		// 送信処理
		if !sendEnd {
			// scanner.Scan()
			// name := scanner.Text()

			fmt.Println("送信")
			sendCount++
			if err := stream.Send(&greetv1.GreetRequest{
				Name: "aaaaa",
			}); err != nil {
				fmt.Println("error ocured. [1]")
				fmt.Println(err)
				sendEnd = true
			}
			fmt.Print("sended. sendCount: ")
			fmt.Println(sendCount)

			if sendCount == sendNum {
				sendEnd = true
				if err := stream.CloseRequest(); err != nil {
					fmt.Println("error ocured. [2]")
					fmt.Println(err)
				}
			}
		}

		fmt.Println("")
		fmt.Println("--- ")

		// 受信処理
		if !recvEnd {
			fmt.Println("受信")
			res, err := stream.Receive()
			if err != nil {
				if !errors.Is(err, io.EOF) {
					fmt.Println("error ocured. [3]")
					fmt.Println(err) // EOF 以外の場合エラー出力
				}
				recvEnd = true
			}
			fmt.Println(res.GetGreeting())
		}

		fmt.Println("")
		fmt.Println("--- ")
	}
}
