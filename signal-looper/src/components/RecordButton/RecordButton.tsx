import styles from "./RecordButton.module.css";

type RecordState = "default" | "count-in" | "recording";

interface RecordButtonProps {
    /** "count-in" = yellow Count-in look, "recording" = red Record look. */
    state?: RecordState;
    disabled?: boolean;
}

// Figma component set "Record Button" (249:1615).
export default function RecordButton({ state = "default", disabled = false }: RecordButtonProps) {
    return (
        <button
            type="button"
            className={styles.button}
            data-state={state === "default" ? undefined : state}
            aria-label="Record"
            aria-pressed={state === "recording"}
            disabled={disabled}
        >
            <span className={styles.icon} />
        </button>
    );
}
