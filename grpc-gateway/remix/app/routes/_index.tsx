import type { ActionFunction, ActionArgs } from "@remix-run/server-runtime";
import { Form, useActionData } from "@remix-run/react";
import type { V2_MetaFunction } from "@remix-run/cloudflare";
import { GreetService } from "~/pb/greet/v1/greet.pb";
import { getSettings } from "~/setting.server";

export const meta: V2_MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Remix!" },
  ];
};

export const action: ActionFunction = async ({
  request,
  context,
}: ActionArgs) => {
  const settings = await getSettings(request);

  // フォームデータを取得
  const formData = await request.formData();

  // サービス実行
  const resp = await GreetService.Greet(
    { name: formData.get("name")?.toString() },
    { pathPrefix: settings.endpoint }
  );

  return { greeting: resp.greeting };
};

export default function Index() {
  const greeting = useActionData()?.greeting;
  return (
    <div className="mx-auto p-5 w-360">
      <h1 className="text-3xl font-bold">{greeting}</h1>
      <div className="grid grid-cols-3">
        <Form method="post">
          <label className="col-span-2 my-2">
            <p>name: </p>
            <input
              type="text"
              name="name"
              className="border rounded w-full py-2 px-3"
            />
          </label>
          <button
            type="submit"
            className="bg-green-500 text-white rounded px-3 py-1 my-3 mx-auto"
          >
            Submit
          </button>
        </Form>
      </div>
    </div>
  );
}
