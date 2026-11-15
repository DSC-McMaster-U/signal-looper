import styles from "./PlayButton.module.css";
import playIcon from "../../assets/play.svg";
import playPlayingIcon from "../../assets/play-playing.svg";
import playDisabledIcon from "../../assets/play-disabled.svg";

interface PlayButtonProps {
    /** Shows the green Playing state. Ignored while 'disabled'. */
    playing?: boolean;
    disabled?: boolean;
}

// Figma component set "Play Button" (262:1669).
export default function PlayButton({ playing = false, disabled = false }: PlayButtonProps) {
    const isPlaying = playing && !disabled;
    const icon = disabled ? playDisabledIcon : isPlaying ? playPlayingIcon : playIcon;

    return (
        <button
            type="button"
            className={styles.button}
            data-state={isPlaying ? "playing" : undefined}
            aria-label="Play"
            aria-pressed={isPlaying}
            disabled={disabled}
        >
            <img className={styles.icon} src={icon} alt="" />
        </button>
    );
}
