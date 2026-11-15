import { useCallback, useEffect, useRef, useState } from "react";

export const MIN_BPM = 40;
export const MAX_BPM = 240;

export const clampBpm = (bpm: number) =>
    Math.min(MAX_BPM, Math.max(MIN_BPM, Math.round(bpm)));

// Clicks are booked ahead of time on the AudioContext clock instead of being
// fired by setInterval. Browser timers drift when the UI is busy; the audio
// clock doesn't, so the beat stays tight. setInterval only wakes the scheduler
// up to book the next few clicks. (Pattern from "A Tale of Two Clocks", web.dev.)
const LOOKAHEAD_MS = 25;
const SCHEDULE_AHEAD_S = 0.1;

const ACCENT_FREQ = 1500;
const BEAT_FREQ = 1000;
const CLICK_LENGTH_S = 0.05;

interface UseMetronomeOptions {
    /** Clicks only while this is true. */
    running: boolean;
    bpm: number;
    beatsPerBar: number;
    /** 0..1 */
    volume?: number;
    /** Fires on every audible beat. 0 = downbeat. */
    onBeat?: (beat: number) => void;
}

export function useMetronome({ running, bpm, beatsPerBar, volume = 0.6, onBeat }: UseMetronomeOptions) {
    const [currentBeat, setCurrentBeat] = useState(-1);

    const ctxRef = useRef<AudioContext | null>(null);
    const timerRef = useRef<number | null>(null);
    const rafRef = useRef<number | null>(null);
    const nextNoteTimeRef = useRef(0);
    const beatRef = useRef(0);
    const queueRef = useRef<{ beat: number; time: number }[]>([]);

    // Latest values live in a ref so changing BPM mid-click takes effect on the
    // next beat without restarting.
    const settings = useRef({ bpm, beatsPerBar, volume, onBeat });
    settings.current = { bpm, beatsPerBar, volume, onBeat };

    const playClick = (ctx: AudioContext, time: number, accent: boolean) => {
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.frequency.value = accent ? ACCENT_FREQ : BEAT_FREQ;
        gain.gain.setValueAtTime(Math.max(0.0001, settings.current.volume), time);
        gain.gain.exponentialRampToValueAtTime(0.0001, time + CLICK_LENGTH_S);
        osc.connect(gain).connect(ctx.destination);
        osc.start(time);
        osc.stop(time + CLICK_LENGTH_S);
    };

    const schedule = useCallback(() => {
        const ctx = ctxRef.current;
        if (!ctx) return;
        while (nextNoteTimeRef.current < ctx.currentTime + SCHEDULE_AHEAD_S) {
            const beat = beatRef.current % settings.current.beatsPerBar;
            playClick(ctx, nextNoteTimeRef.current, beat === 0);
            queueRef.current.push({ beat, time: nextNoteTimeRef.current });
            nextNoteTimeRef.current += 60 / settings.current.bpm;
            beatRef.current = beat + 1;
        }
    }, []);

    // Report each beat at the moment it's heard, not when it was booked.
    const watchBeats = useCallback(() => {
        const ctx = ctxRef.current;
        if (!ctx) return;
        let latest: number | null = null;
        while (queueRef.current.length && queueRef.current[0].time <= ctx.currentTime) {
            latest = queueRef.current.shift()!.beat;
        }
        if (latest !== null) {
            setCurrentBeat(latest);
            settings.current.onBeat?.(latest);
        }
        rafRef.current = requestAnimationFrame(watchBeats);
    }, []);

    useEffect(() => {
        if (!running) return;

        if (!ctxRef.current) ctxRef.current = new AudioContext();
        const ctx = ctxRef.current;
        // Not awaited: resume() can stall when no output device is ready yet,
        // and scheduling against a paused clock is harmless.
        if (ctx.state === "suspended") void ctx.resume();

        beatRef.current = 0;
        queueRef.current = [];
        nextNoteTimeRef.current = ctx.currentTime + 0.05;
        schedule();
        timerRef.current = window.setInterval(schedule, LOOKAHEAD_MS);
        rafRef.current = requestAnimationFrame(watchBeats);

        return () => {
            if (timerRef.current !== null) window.clearInterval(timerRef.current);
            if (rafRef.current !== null) cancelAnimationFrame(rafRef.current);
            timerRef.current = null;
            rafRef.current = null;
            queueRef.current = [];
            setCurrentBeat(-1);
        };
    }, [running, schedule, watchBeats]);

    useEffect(
        () => () => {
            void ctxRef.current?.close();
            ctxRef.current = null;
        },
        [],
    );

    return { currentBeat };
}
