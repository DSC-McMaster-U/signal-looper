import styles from "./DropdownBarSelection.module.css";
import chevronDown from "../../assets/chevron-down.svg";

export interface DropdownOption<T extends string | number> {
    value: T;
    label: string;
}

interface DropdownBarSelectionProps<T extends string | number> {
    /** Small label on top, e.g. "Count-in" or "Length". */
    caption: string;
    value: T;
    options: DropdownOption<T>[];
    onChange: (value: T) => void;
    disabled?: boolean;
}

// Figma component "Drop-down Bar Selection" (181:9417).
// Used for Count-in (Metronome Controls) and Length (Timing).
// A native <select> sits invisibly on top so the menu, keyboard and screen readers all work.
export default function DropdownBarSelection<T extends string | number>({
    caption,
    value,
    options,
    onChange,
    disabled = false,
}: DropdownBarSelectionProps<T>) {
    const current = options.find((o) => o.value === value);

    return (
        <label className={styles.box} data-disabled={disabled || undefined}>
            <span className={styles.text}>
                <span className={styles.caption}>{caption}</span>
                <span className={styles.value}>{current?.label ?? String(value)}</span>
            </span>
            <img className={styles.chevron} src={chevronDown} alt="" />
            <select
                className={styles.select}
                value={String(value)}
                disabled={disabled}
                aria-label={caption}
                onChange={(e) => {
                    const picked = options.find((o) => String(o.value) === e.target.value);
                    if (picked) onChange(picked.value);
                }}
            >
                {options.map((o) => (
                    <option key={String(o.value)} value={String(o.value)}>
                        {o.label}
                    </option>
                ))}
            </select>
        </label>
    );
}
