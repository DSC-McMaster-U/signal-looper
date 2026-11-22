import styles from "./PlayButton.module.css";
import playIcon from "../../assets/play.svg";
import playPlayingIcon from "../../assets/play-playing.svg";
import playDisabledIcon from "../../assets/play-disabled.svg";

interface PlayButtonProps {
  /** Shows the green Playing state. Ignored while 'disabled'. */
  playing?: boolean;
  disabled?: boolean;
  onClick?: () => void;
}

// Figma component set "Play Button" (262:1669).
export default function PlayButton({
  playing = false,
  disabled = false,
  onClick,
}: PlayButtonProps) {
  const isPlaying = playing && !disabled;
  const icon = disabled
    ? playDisabledIcon
    : isPlaying
      ? playPlayingIcon
      : playIcon;

  return (
    <button
      type="button"
      className={styles.button}
      data-state={isPlaying ? "playing" : undefined}
      aria-label="Play"
      aria-pressed={isPlaying}
      disabled={disabled}
      onClick={onClick}
    >
      <img className={styles.icon} src={icon} alt="" />
    </button>
  );
}
