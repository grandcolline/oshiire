import type { ActionFunction, LoaderFunction } from "@remix-run/server-runtime";
import { Form } from "@remix-run/react";
import type { V2_MetaFunction } from "@remix-run/cloudflare";
import { GreetService } from "~/pb/greet/v1/greet.pb";

export const meta: V2_MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Remix!" },
  ];
};

export let loader: LoaderFunction = async ({}) => {
  console.log("loader");
  return {greet: ""};
};

export let action: ActionFunction = async ({ request }) => {
  // フォームデータを取得
  const formData = await request.formData();
  const resp = await GreetService.Greet({ name: formData.get("name")?.toString() });

  console.log("action");
  // return {greet: resp.greeting};
  return { greet: "hello" };
};

export default function Index() {
  return (
    <div style={{ fontFamily: "system-ui, sans-serif", lineHeight: "1.8" }}>
      <h1>Welcome to Remix</h1>
      <Form method="post">
        <label>name: </label>
        <input type="text" name="name" />
        <button type="submit">GO</button>
      </Form>
    </div>
  );
}
