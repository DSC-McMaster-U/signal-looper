import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import TopBar from "./components/TopBar/TopBar";
import TransportBar from "./components/TransportBar/TransportBar";

function App() {
  return (
    <main className="container">
      <TopBar />
      <TransportBar />
    </main>
  );
}

export default App;
