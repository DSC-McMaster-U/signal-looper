import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import "./cpal-test.css";

const CpalTest: React.FC = () => {
	const [status, setStatus] = useState("");
	const [isPlaying, setIsPlaying] = useState(false);

	const playTestTone = async () => {
		setIsPlaying(true);
		setStatus("Playing 440 Hz test tone…");

		try {
			await invoke("play_test_tone");
			setStatus("Test tone finished.");
		} catch (error) {
			const message = error instanceof Error ? error.message : String(error);
			setStatus(`Could not play test tone: ${message}`);
		} finally {
			setIsPlaying(false);
		}
	};

	return (
		<main className="cpal-test">
			<section className="cpal-test__card" aria-labelledby="cpal-test-heading">
				<h1 id="cpal-test-heading">CPAL test tone</h1>
				<button type="button" onClick={playTestTone} disabled={isPlaying}>
					{isPlaying ? "Playing…" : "Play test tone"}
				</button>
				<p className="cpal-status" aria-live="polite">{status}</p>
			</section>
		</main>
	);
};

export default CpalTest;
