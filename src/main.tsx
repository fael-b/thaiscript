import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { MantineProvider } from "@mantine/core";
import { thaiScriptTheme } from "./config/mantine-theme";
import "./styles.css";
import "@mantine/core/styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider theme={thaiScriptTheme}>
      <App />
    </MantineProvider>
  </React.StrictMode>,
);
