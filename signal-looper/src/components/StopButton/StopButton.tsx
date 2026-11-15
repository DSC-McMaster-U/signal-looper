import styles from "./StopButton.module.css";

interface StopButtonProps {
    disabled?: boolean;
}

// Figma component set "Stop Button" (268:1699).
export default function StopButton({ disabled = false }: StopButtonProps) {
    return (
        <button type="button" className={styles.button} aria-label="Stop" disabled={disabled}>
            <span className={styles.icon} />
        </button>
    );
}
