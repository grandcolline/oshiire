import type { ActionFunction, LoaderFunction } from "@remix-run/server-runtime";
import { Form, useActionData } from "@remix-run/react";
import type { V2_MetaFunction } from "@remix-run/cloudflare";
import { GreetService } from "~/pb/greet/v1/greet.pb";

export const meta: V2_MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Remix!" },
  ];
};

// export let loader: LoaderFunction = async ({}) => {
//   return {};
// };

export const action: ActionFunction = async ({ request }) => {
  // フォームデータを取得
  const formData = await request.formData();

  // サービス実行
  const resp = await GreetService.Greet(
    { name: formData.get("name")?.toString() },
    { pathPrefix: "http://localhost:8081" }
  );

  return { greeting: resp.greeting };
};

export default function Index() {
  const greeting = useActionData()?.greeting;
  return (
    <div style={{ fontFamily: "system-ui, sans-serif", lineHeight: "1.8" }}>
      <h1>{greeting}</h1>
      <Form method="post">
        <label>name: </label>
        <input type="text" name="name" />
        <button type="submit">GO</button>
      </Form>
    </div>
  );
}
