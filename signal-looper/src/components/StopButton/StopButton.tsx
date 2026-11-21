import styles from "./StopButton.module.css";

interface StopButtonProps {
  disabled?: boolean;
  onClick?: () => void;
}

// Figma component set "Stop Button" (268:1699).
export default function StopButton({
  disabled = false,
  onClick,
}: StopButtonProps) {
  return (
    <button
      type="button"
      className={styles.button}
      aria-label="Stop"
      disabled={disabled}
      onClick={onClick}
    >
      <span className={styles.icon} />
    </button>
  );
}
