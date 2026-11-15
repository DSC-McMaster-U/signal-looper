import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import styles from "./TimeSig.module.css";

const BEATS_PER_BAR_OPTIONS = [1, 2, 3, 4, 5, 6];

interface TimeSigProps {
    beatsPerBar: number;
    onChange: (beatsPerBar: number) => void;
    disabled?: boolean;
}

// Figma frame "Time Sig" (239:1587), inside "Timing" (177:9351).
// Top number: component set 228:1304 with Default / Hover / Open states.
// The bottom number stays at 4 for now; the metronome counts quarter notes.
export default function TimeSig({ beatsPerBar, onChange, disabled = false }: TimeSigProps) {
    const [open, setOpen] = useState(false);
    const rootRef = useRef<HTMLDivElement>(null);
    const listRef = useRef<HTMLUListElement>(null);

    // Close when clicking anywhere else.
    useEffect(() => {
        if (!open) return;
        const onPointerDown = (e: PointerEvent) => {
            if (!rootRef.current?.contains(e.target as Node)) setOpen(false);
        };
        document.addEventListener("pointerdown", onPointerDown);
        return () => document.removeEventListener("pointerdown", onPointerDown);
    }, [open]);

    // Move keyboard focus to the selected item when the menu opens.
    useEffect(() => {
        if (open) listRef.current?.querySelector<HTMLElement>('[aria-selected="true"]')?.focus();
    }, [open]);

    const pick = (n: number) => {
        onChange(n);
        setOpen(false);
    };

    const onMenuKeyDown = (e: KeyboardEvent<HTMLUListElement>) => {
        const items = Array.from(e.currentTarget.querySelectorAll<HTMLElement>("[role=option]"));
        const i = items.indexOf(document.activeElement as HTMLElement);
        if (e.key === "ArrowDown") {
            e.preventDefault();
            items[Math.min(items.length - 1, i + 1)]?.focus();
        } else if (e.key === "ArrowUp") {
            e.preventDefault();
            items[Math.max(0, i - 1)]?.focus();
        } else if (e.key === "Escape" || e.key === "Tab") {
            setOpen(false);
        }
    };

    return (
        <div className={styles.timeSig} ref={rootRef}>
            <div className={styles.number}>
                <button
                    type="button"
                    className={styles.hitArea}
                    data-state={open ? "open" : undefined}
                    aria-label={`Beats per bar: ${beatsPerBar}`}
                    aria-haspopup="listbox"
                    aria-expanded={open}
                    disabled={disabled}
                    onClick={() => setOpen((o) => !o)}
                >
                    {beatsPerBar}
                </button>
                {open && (
                    <ul className={styles.menu} role="listbox" aria-label="Beats per bar" ref={listRef} onKeyDown={onMenuKeyDown}>
                        {BEATS_PER_BAR_OPTIONS.map((n) => (
                            <li
                                key={n}
                                className={styles.menuItem}
                                role="option"
                                aria-selected={n === beatsPerBar}
                                tabIndex={-1}
                                onClick={() => pick(n)}
                                onKeyDown={(e) => {
                                    if (e.key === "Enter" || e.key === " ") {
                                        e.preventDefault();
                                        pick(n);
                                    }
                                }}
                            >
                                {n}
                            </li>
                        ))}
                    </ul>
                )}
            </div>
            <span className={styles.slash} aria-hidden="true">
                /
            </span>
            <span className={styles.number}>
                <span className={styles.staticNumber} aria-label="Beat unit: quarter note">
                    4
                </span>
            </span>
        </div>
    );
}
