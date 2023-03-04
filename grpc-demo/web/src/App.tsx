import { useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { GreetService } from "./gen/greet_connectweb";
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
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await client
              .greet({
                name: inputValue,
              })
              .then(({ greeting }) => {
                console.log(greeting);
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
      </header>
    </div>
  );
}

export default App;
