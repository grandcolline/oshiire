/* eslint-disable */
// @ts-nocheck
/*
 * This file is a generated Typescript file for GRPC Gateway, DO NOT MODIFY
 */

import * as fm from "../../fetch.pb";
export type GreetRequest = {
  name?: string;
};

export type GreetResponse = {
  greeting?: string;
};

export class GreetService {
  static Greet(
    req: GreetRequest,
    initReq?: fm.InitReq
  ): Promise<GreetResponse> {
    return fm.fetchReq<GreetRequest, GreetResponse>(
      `/greet.v1.GreetService/Greet`,
      { ...initReq, method: "POST", body: JSON.stringify(req, fm.replacer) }
    );
  }
}
