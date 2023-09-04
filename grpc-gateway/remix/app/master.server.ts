import { createCookie } from "@remix-run/cloudflare";

const MASTER_NAME = "debug-master"

export const userPrefs = createCookie("user-prefs", {
  maxAge: 604_800, // 1 week
});
