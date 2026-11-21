import { useState } from "react";
import styles from "./TransportBar.module.css";
import RecordButton from "../RecordButton/RecordButton";
import PlayButton from "../PlayButton/PlayButton";
import StopButton from "../StopButton/StopButton";
import TimeSig from "../TimeSig/TimeSig";
import BpmSetting from "../BpmSetting/BpmSetting";
import MetronomeButton from "../MetronomeButton/MetronomeButton";
import DropdownBarSelection, {
  type DropdownOption,
} from "../DropdownBarSelection/DropdownBarSelection";
import { useMetronome } from "../../hooks/useMetronome";
import VolumeControl from "../VolumeControl/VolumeControl";
import { invoke } from "@tauri-apps/api/core";

/**
 * Transport Bar (Figma 113:34) containing the Controls group (113:61):
 * Record, Play, Stop.
 *
 * TODO: Backend integration. The backend loop state machine doesn't exist yet.
 * Everything below is a suggestion, not a strict requirement. Whoever picks this
 * up is free to adjust it to fit the backend.
 *
 * There are two separate kinds of state:
 *
 * 1. LOOP STATE (backend, Rust): what the looper is actually doing.
 *    This is the single source of truth.
 *      idle -> recording -> playing -> overdubbing
 *    The UI also needs one extra fact from the backend: has a loop been recorded yet?
 *
 * 2. BUTTON STATE (frontend, React): only how each button looks, set with props.
 *    Every button shows its default state when given no props.
 *      RecordButton  default | state="count-in" | state="recording" | disabled
 *      PlayButton    default | playing | disabled
 *      StopButton    default | disabled
 *    Hover is handled in CSS and only applies to an enabled button in its default state.
 *    Buttons never change their own state. TransportBar works out their props
 *    from the loop state, using the table below.
 *
 * Loop state -> button props (anything not listed stays at its default):
 *   idle          Stop disabled; Play disabled if no loop has been recorded yet
 *   recording     Record state="recording"; Play disabled
 *   playing       Play playing
 *   overdubbing   Record state="recording"; Play disabled
 *
 * Button click -> new loop state (the backend makes the change):
 *   Record   idle        -> recording
 *            recording   -> playing      (closes the loop and plays it back)
 *            playing     -> overdubbing
 *            overdubbing -> playing      (ends the overdub; loop keeps playing)
 *   Play     idle        -> playing      (needs a recorded loop; resumes from current position)
 *            playing     -> idle         (pauses; playhead stays where it is)
 *   Stop     any         -> idle         (ends recording/overdub; resets playhead and
 *                                         position readout to bar 1, beat 1)
 *
 * Open questions:
 *   - Should Stop be enabled in idle after a pause (playhead not at bar 1)?
 *   - Record in idle when a loop already exists: overdub it, or record a new loop?
 *
 * Metronome (#21): tempo, time signature, metronome toggle and count-in.
 *   Tempo state lives here for now; move it to the shared store once there is one,
 *   since recording and the timeline will need BPM too.
 *   Until Play is wired up, turning the metronome on starts clicking straight away.
 *   Once loop state exists, pass `running: metronomeOn && loopState !== "idle"`.
 *   Count-in: when Record is pressed with countIn > 0, play countIn bars of clicks
 *   with Record in state="count-in", then start recording. Needs the backend
 *   record command, so it isn't wired up yet.
 */

type TransportState = "stopped" | "playing" | "paused";

const COUNT_IN_OPTIONS: DropdownOption<number>[] = [
  { value: 0, label: "Off" },
  { value: 1, label: "1 bar" },
  { value: 2, label: "2 bars" },
];

export default function TransportBar() {
  const [bpm, setBpm] = useState(120);
  const [beatsPerBar, setBeatsPerBar] = useState(4);
  const [metronomeOn, setMetronomeOn] = useState(false);
  const [countIn, setCountIn] = useState(2);

  const [transport, setTransport] = useState<TransportState>("stopped");
  const playing = transport === "playing";

  const handlePlay = async () => {
    try {
      if (transport === "playing") {
        await invoke("pause_audio");
        setTransport("paused");
      } else if (transport === "paused") {
        await invoke("resume_audio");
        setTransport("playing");
      } else {
        await invoke("play_audio");
        setTransport("playing");
      }
    } catch (e) {
      console.error("transport failed:", e);
    }
  };

  const handleStop = async () => {
    await invoke("stop_audio");
    setTransport("stopped");
  };

  useMetronome({ running: metronomeOn, bpm, beatsPerBar });

  return (
    <div className={styles.transportBar}>
      {/* Figma "Timing" (177:9351). Length and Position Readout go here later. */}
      <div className={styles.timing}>
        <TimeSig beatsPerBar={beatsPerBar} onChange={setBeatsPerBar} />
        <BpmSetting bpm={bpm} onChange={setBpm} />
      </div>
      <div className={styles.controls}>
        <RecordButton />
        <PlayButton playing={playing} onClick={handlePlay} />
        <StopButton onClick={handleStop} />
      </div>
      {/* Figma "Metronome Controls" (181:9435) */}
      <div className={styles.metronome}>
        <MetronomeButton
          on={metronomeOn}
          onToggle={() => setMetronomeOn((on) => !on)}
        />
        <DropdownBarSelection
          caption="Count-in"
          value={countIn}
          options={COUNT_IN_OPTIONS}
          onChange={setCountIn}
        />
      </div>

      <div className={styles.volume}>
        <VolumeControl />
      </div>
    </div>
  );
}
