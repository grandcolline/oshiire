import type {
  LoaderArgs,
  V2_MetaFunction,
  ActionArgs,
} from "@remix-run/cloudflare";
import { json } from "@remix-run/cloudflare";
import { getSettings, toCookie } from "~/setting.server";
import { useLoaderData, Form } from "@remix-run/react";

export const meta: V2_MetaFunction = () => {
  return [{ title: "setting" }];
};

export async function loader({ request, context }: LoaderArgs) {
  const settings = await getSettings(request, context);
  return json(settings);
}

export async function action({ request, context }: ActionArgs) {
  const settings = await getSettings(request, context);
  const form = await request.formData();

  if (form.get("endpoint")) {
    settings.endpoint = form.get("endpoint") as string;
  }

  return json(
    {},
    {
      headers: {
        "Set-Cookie": await toCookie(settings),
      },
    }
  );
}

export default function Debug() {
  const data = useLoaderData<typeof loader>();

  return (
    <div className="mx-auto p-5 w-360">
      <h1 className="text-3xl font-bold">Environments</h1>
      <div className="grid grid-cols-3 my-6">
        <Form method="post">
          <label className="col-span-2 my-2">
            <p>ENDPOINT: </p>
            <input
              type="text"
              name="endpoint"
              defaultValue={data.endpoint}
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
