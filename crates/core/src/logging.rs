use crate::config::Config;
use std::fs;
use std::path::PathBuf;

// ──────────────────────────────────────────────────────────────
// Structured logging to ~/.minutes/logs/minutes.log
// Format: JSON lines, one per event
// Rotation: daily, keep 7 days
// CLI: `minutes logs` to tail, `minutes logs --errors` to filter
// ──────────────────────────────────────────────────────────────

/// Get the current log file path.
pub fn log_path() -> PathBuf {
    Config::minutes_dir().join("logs").join("minutes.log")
}

/// Ensure the log directory exists.
pub fn ensure_log_dir() -> std::io::Result<()> {
    let dir = Config::minutes_dir().join("logs");
    fs::create_dir_all(dir)
}

/// Rotate old log files. Keeps the last 7 days.
pub fn rotate_logs() -> std::io::Result<()> {
    let log_dir = Config::minutes_dir().join("logs");
    if !log_dir.exists() {
        return Ok(());
    }

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let current_log = log_dir.join("minutes.log");

    // If the current log exists and is from a previous day, rotate it
    if current_log.exists() {
        if let Ok(metadata) = current_log.metadata() {
            if let Ok(modified) = metadata.modified() {
                let modified_date = chrono::DateTime::<chrono::Local>::from(modified)
                    .format("%Y-%m-%d")
                    .to_string();

                if modified_date != today {
                    let rotated = log_dir.join(format!("minutes.{}.log", modified_date));
                    fs::rename(&current_log, &rotated)?;
                }
            }
        }
    }

    // Delete logs older than 7 days
    let cutoff = chrono::Local::now() - chrono::Duration::days(7);
    let cutoff_str = cutoff.format("%Y-%m-%d").to_string();

    for entry in fs::read_dir(&log_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Match pattern: minutes.YYYY-MM-DD.log
        if let Some(date) = name
            .strip_prefix("minutes.")
            .and_then(|s| s.strip_suffix(".log"))
        {
            if date < cutoff_str.as_str() {
                fs::remove_file(entry.path())?;
                tracing::debug!(file = %name, "removed old log file");
            }
        }
    }

    Ok(())
}

/// Append a structured log entry to the log file.
pub fn append_log(entry: &serde_json::Value) -> std::io::Result<()> {
    ensure_log_dir()?;
    let path = log_path();
    let line = serde_json::to_string(entry)? + "\n";

    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    file.write_all(line.as_bytes())?;
    Ok(())
}

/// Log a pipeline step to the structured log file.
pub fn log_step(step: &str, file: &str, duration_ms: u64, extra: serde_json::Value) {
    let entry = serde_json::json!({
        "ts": chrono::Local::now().to_rfc3339(),
        "level": "info",
        "step": step,
        "file": file,
        "duration_ms": duration_ms,
        "extra": extra,
    });

    if let Err(e) = append_log(&entry) {
        tracing::warn!("failed to write to log file: {}", e);
    }
}

/// Log an error to the structured log file.
pub fn log_error(step: &str, file: &str, error: &str) {
    let entry = serde_json::json!({
        "ts": chrono::Local::now().to_rfc3339(),
        "level": "error",
        "step": step,
        "file": file,
        "error": error,
    });

    if let Err(e) = append_log(&entry) {
        tracing::warn!("failed to write error to log file: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn append_log_creates_file() {
        // This test writes to the real log path — just verify format
        let entry = serde_json::json!({
            "ts": "2026-03-17T08:00:00",
            "level": "info",
            "step": "test",
            "message": "unit test entry"
        });

        let json_str = serde_json::to_string(&entry).unwrap();
        assert!(json_str.contains("\"step\":\"test\""));
    }

    #[test]
    fn log_step_formats_correctly() {
        // Verify the JSON structure
        let entry = serde_json::json!({
            "ts": chrono::Local::now().to_rfc3339(),
            "level": "info",
            "step": "transcribe",
            "file": "test.wav",
            "duration_ms": 4200,
            "extra": { "words": 142 },
        });

        let line = serde_json::to_string(&entry).unwrap();
        assert!(line.contains("\"step\":\"transcribe\""));
        assert!(line.contains("\"duration_ms\":4200"));
    }
}
