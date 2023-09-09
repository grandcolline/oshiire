import { createCookie, AppLoadContext } from "@remix-run/cloudflare";

interface Env {
  endpoint: string;
}

const cookie = createCookie("settings", {
  maxAge: 604_800, // 1week
  secrets: ["9zvcNsKoDU2HoSACMnTDFp2tUGnDcFDC"], // FIXME
});

export async function getEnv(
  request: Request,
  context: AppLoadContext
): Promise<Env> {
  const front = await cookie.parse(request.headers.get("Cookie"));
  const server = context.env as Env;

  return {
    endpoint: front?.endpoint ?? server.endpoint,
  };
}

// Set-Cookie するときの値を取得する
export async function toCookie(env: Env): Promise<string> {
  return await cookie.serialize(env);
}
