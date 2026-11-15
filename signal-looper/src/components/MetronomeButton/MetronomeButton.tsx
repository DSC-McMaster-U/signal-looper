import styles from "./MetronomeButton.module.css";
import metronomeIcon from "../../assets/metronome.svg";
import metronomeOnIcon from "../../assets/metronome-on.svg";

interface MetronomeButtonProps {
    /** Metronome on: clicks play along with the transport. */
    on?: boolean;
    onToggle?: () => void;
    disabled?: boolean;
}

// Figma component set "Metronome Button" (194:1284), inside "Metronome Controls" (181:9435).
// Figma only has State=Default so far; the On look reuses Play's Playing treatment.
export default function MetronomeButton({ on = false, onToggle, disabled = false }: MetronomeButtonProps) {
    const isOn = on && !disabled;

    return (
        <button
            type="button"
            className={styles.button}
            data-state={isOn ? "on" : undefined}
            aria-label="Metronome"
            aria-pressed={isOn}
            disabled={disabled}
            onClick={onToggle}
        >
            <img className={styles.icon} src={isOn ? metronomeOnIcon : metronomeIcon} alt="" />
        </button>
    );
}
