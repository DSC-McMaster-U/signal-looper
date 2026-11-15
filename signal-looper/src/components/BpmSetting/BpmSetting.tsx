import { useEffect, useState } from "react";
import styles from "./BpmSetting.module.css";
import arrowUp from "../../assets/arrow-up.svg";
import arrowDown from "../../assets/arrow-down.svg";
import { MAX_BPM, MIN_BPM, clampBpm } from "../../hooks/useMetronome";

interface BpmSettingProps {
    bpm: number;
    onChange: (bpm: number) => void;
    disabled?: boolean;
}

// Figma component "BPM Setting" (213:1323), inside "Timing" (177:9351).
// Type a tempo, use the arrows, or press Up/Down while the field is focused.
export default function BpmSetting({ bpm, onChange, disabled = false }: BpmSettingProps) {
    // Typed text is only committed on Enter/blur, so clearing the field to type
    // a new number doesn't snap the tempo to the minimum.
    const [draft, setDraft] = useState(String(bpm));
    useEffect(() => setDraft(String(bpm)), [bpm]);

    const commit = (value: number) => {
        const next = Number.isFinite(value) && value > 0 ? clampBpm(value) : bpm;
        setDraft(String(next));
        onChange(next);
    };

    return (
        <div className={styles.box} data-disabled={disabled || undefined}>
            <label className={styles.text}>
                <span className={styles.caption}>BPM</span>
                <input
                    className={styles.input}
                    type="text"
                    inputMode="numeric"
                    value={draft}
                    disabled={disabled}
                    aria-label={`Tempo in BPM, ${MIN_BPM} to ${MAX_BPM}`}
                    onChange={(e) => setDraft(e.target.value.replace(/\D/g, "").slice(0, 3))}
                    onBlur={() => commit(Number(draft))}
                    onKeyDown={(e) => {
                        if (e.key === "Enter") e.currentTarget.blur();
                        if (e.key === "ArrowUp") {
                            e.preventDefault();
                            commit(bpm + 1);
                        }
                        if (e.key === "ArrowDown") {
                            e.preventDefault();
                            commit(bpm - 1);
                        }
                    }}
                />
            </label>
            <div className={styles.arrows}>
                <button
                    type="button"
                    className={styles.arrow}
                    aria-label="Increase tempo"
                    disabled={disabled || bpm >= MAX_BPM}
                    onClick={() => commit(bpm + 1)}
                >
                    <img src={arrowUp} alt="" />
                </button>
                <button
                    type="button"
                    className={styles.arrow}
                    aria-label="Decrease tempo"
                    disabled={disabled || bpm <= MIN_BPM}
                    onClick={() => commit(bpm - 1)}
                >
                    <img src={arrowDown} alt="" />
                </button>
            </div>
        </div>
    );
}
