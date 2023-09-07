/* eslint-disable */
// @ts-nocheck
/*
 * This file is a generated Typescript file for GRPC Gateway, DO NOT MODIFY
 */

import * as fm from "../../fetch.pb";
import * as GoogleProtobufEmpty from "../../google/protobuf/empty.pb";
export type CheckResponse = {
  msg?: string;
};

export class HealthService {
  static Check(
    req: GoogleProtobufEmpty.Empty,
    initReq?: fm.InitReq
  ): Promise<CheckResponse> {
    return fm.fetchReq<GoogleProtobufEmpty.Empty, CheckResponse>(
      `/greet.v1.HealthService/Check`,
      { ...initReq, method: "POST", body: JSON.stringify(req, fm.replacer) }
    );
  }
}
