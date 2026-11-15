import { useState, type CSSProperties } from "react";
import styles from "./VolumeControl.module.css";

export default function VolumeControl() {
    const [volume, setVolume] = useState(50);

    return (
        <div className={styles.control}>
            <div className={styles.heading}>
                <label className={styles.volumeLabel} htmlFor="volume-slider">
                    <svg className={styles.icon} viewBox="0 0 24 24" fill="none"> {/* Creating the Volume icon as an SVG */}
                        <path d="M11 5 6 9H3v6h3l5 4V5Z" />
                        <path d="M15.5 8.5a5 5 0 0 1 0 7M19 5a10 10 0 0 1 0 14" />
                    </svg>
                    <span>Master Volume</span>
                </label>
                <output className={styles.percentage} htmlFor="volume-slider">
                    {volume}%
                </output>
            </div>
            <input
                id="volume-slider"
                className={styles.slider}
                type="range"
                min="0"
                max="100"
                value={volume}
                onChange={(event) => setVolume(Number(event.target.value))}
                style={{ "--volume-progress": `${volume}%` } as CSSProperties}
            />
        </div>
    );
}
