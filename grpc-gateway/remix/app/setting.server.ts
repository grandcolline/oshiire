import { createCookie, AppLoadContext } from "@remix-run/cloudflare";

const settingsCookie = createCookie("settings", {
  maxAge: 604_800, // 1week
});

interface Settings {
  endpoint: string;
}

export async function getSettings(
  request: Request,
  context: AppLoadContext
): Promise<Settings> {
  const cookie = await settingsCookie.parse(request.headers.get("Cookie"));
  const env = context.env as Settings;

  const endpoint = cookie?.endpoint ?? env.endpoint;

  return {
    endpoint: endpoint,
  };
}

// Set-Cookie するときの値を取得する
export async function toCookie(settings: Settings): Promise<string> {
  return await settingsCookie.serialize(settings);
}
