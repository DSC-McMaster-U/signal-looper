import styles from "./TransportBar.module.css";
import RecordButton from "../RecordButton/RecordButton";
import PlayButton from "../PlayButton/PlayButton";
import StopButton from "../StopButton/StopButton";

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
 *   None of the buttons accept `onClick` yet; add one to each when the Tauri
 *   commands exist.
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
 * Record's count-in look (state="count-in") is handled in the metronome ticket.
 */
export default function TransportBar() {
    return (
        <div className={styles.transportBar}>
            <div className={styles.controls}>
                <RecordButton />
                <PlayButton />
                <StopButton />
            </div>
        </div>
    );
}
