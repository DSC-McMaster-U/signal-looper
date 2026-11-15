import styles from "./TopBar.module.css";

export default function TopBar() {
    return (
        <header className={styles.topBar}>
            <h1 className={styles.title}>SignalLooper</h1>
            <button type="button" className={styles.fileButton}>
                File
            </button>
            <span className={styles.projectName}>placeholder_project.sl</span>
        </header>
    );
}
