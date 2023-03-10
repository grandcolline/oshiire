// @generated by protoc-gen-connect-web v0.8.1
// @generated from file greet/v1/greet.proto (package greet.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { GreetRequest, GreetResponse } from "./greet_pb.js";
import { MethodKind } from "@bufbuild/protobuf";

/**
 * @generated from service greet.v1.GreetService
 */
export declare const GreetService: {
  readonly typeName: "greet.v1.GreetService",
  readonly methods: {
    /**
     * @generated from rpc greet.v1.GreetService.Greet
     */
    readonly greet: {
      readonly name: "Greet",
      readonly I: typeof GreetRequest,
      readonly O: typeof GreetResponse,
      readonly kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc greet.v1.GreetService.GreetServerStream
     */
    readonly greetServerStream: {
      readonly name: "GreetServerStream",
      readonly I: typeof GreetRequest,
      readonly O: typeof GreetResponse,
      readonly kind: MethodKind.ServerStreaming,
    },
    /**
     * @generated from rpc greet.v1.GreetService.GreetClientStream
     */
    readonly greetClientStream: {
      readonly name: "GreetClientStream",
      readonly I: typeof GreetRequest,
      readonly O: typeof GreetResponse,
      readonly kind: MethodKind.ClientStreaming,
    },
    /**
     * @generated from rpc greet.v1.GreetService.GreetBiStream
     */
    readonly greetBiStream: {
      readonly name: "GreetBiStream",
      readonly I: typeof GreetRequest,
      readonly O: typeof GreetResponse,
      readonly kind: MethodKind.BiDiStreaming,
    },
  }
};

