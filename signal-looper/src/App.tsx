import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [seconds, setSeconds] = useState(5);
  const [status, setStatus] = useState("hit record to start");

  async function recordAudio() {
    setStatus(`recording for ${seconds} seconds...`);
    try {
      await invoke("record_audio", { seconds });
      setStatus(`saved a recording`);
    } catch (err) {
      setStatus(`error: ${err}`);
    }
  }

  async function playAudio() {
    setStatus("playing...");
    try {
      await invoke("play_audio");
      setStatus("done playing");
    } catch (err) {
      setStatus(`error: ${err}`);
    }
  }

  return (
    <main className="container">
      <p>{status}</p>
      <label>
        seconds:
        <input
          type="number"
          value={seconds}
          onChange={(e) => setSeconds(Number(e.target.value))}
        />
      </label>
      <button onClick={recordAudio}>Record</button>
      <button onClick={playAudio}>Play</button>
    </main>
  );
}

export default App;
