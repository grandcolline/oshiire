import { useState } from "react";
import "./App.css";
import { GreetService } from "@buf/grandcolline_buf-sample.bufbuild_connect-web/greet/v1/greet_connectweb";
import { createConnectTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";

function App() {
  const transport = createConnectTransport({
    baseUrl: "http://localhost:8080",
  });
  const client = createPromiseClient(GreetService, transport);

  const [inputValue, setInputValue] = useState("");
  const [message, setMessage] = useState("");

  return (
    <div className="App">
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          await client
            .greet({
              name: inputValue,
            })
            .then(({ greeting }) => {
              setMessage(greeting);
            });
        }}
      >
        <input
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
        />
        <button type="submit">Send</button>
      </form>
      <p>{message}</p>
    </div>
  );
}

export default App;
