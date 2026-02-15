import React from "react";
import ReactDOM from "react-dom/client";
import { ErrorityConsole } from "./components/ErrorityConsole";
import init, { transform_to_knowledge } from "../public/wasm/errority_wasm";

async function bootstrap() {
  await init();

  const root = ReactDOM.createRoot(
    document.getElementById("root") as HTMLElement
  );

  root.render(
    <ErrorityConsole
      transform={async (input: string) => {
        const result = transform_to_knowledge(input);
        return result;
      }}
    />
  );
}

bootstrap();
