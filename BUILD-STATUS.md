# Minutes — Build Status

> This file tracks implementation progress. Read this after compaction to know exactly where you left off.
> Update this file after completing each bead. Never leave it stale.

## Current Phase: 1a — Recording Pipeline

## Build Chunks

### Chunk 1: Scaffold + Core Pipeline (P1a.0-6)
| Bead | Status | Score | Notes |
|------|--------|-------|-------|
| P1a.0 | NOT STARTED | - | MCPB research blocker — do before Phase 2 |
| P1a.1 | DONE | 10/10 | Cargo workspace: `core` (lib) + `cli` (bin). 10 modules in core. |
| P1a.2 | PLACEHOLDER | 4/10 | Creates placeholder WAV. Real cpal+BlackHole capture not yet wired. |
| P1a.3 | DONE | 10/10 | WAV writing via hound. Temp WAV cleanup on pipeline completion. |
| P1a.4 | PLACEHOLDER | 3/10 | Pipeline placeholder returns fake transcript. whisper-rs + symphonia not yet integrated. |
| P1a.5 | DONE | 10/10 | Markdown writer: YAML frontmatter, 0600 perms, collision handling, memo/meeting templates, no-speech marker. 5 tests. |
| P1a.6 | DONE | 9/10 | CLI: record, stop, status, search, list, process, setup, logs. PID lifecycle. Signal handling (Ctrl-C). JSON output for MCPB. Missing: real audio capture blocks full 10/10. |

### Chunk 2: Config + Infrastructure (P1a.7-8, P1a.14-15)
| Bead | Status | Score | Notes |
|------|--------|-------|-------|
| P1a.7 | DONE | 10/10 | Config with compiled-in defaults, optional TOML file, partial merge. 4 tests. |
| P1a.8 | PARTIAL | 5/10 | `minutes setup --list` works. Actual model download not implemented (prints manual instructions). |
| P1a.14 | DONE | 8/10 | logging.rs: JSON line append, log rotation (7 days), log_step/log_error helpers. `minutes logs` CLI command. Missing: pipeline doesn't call log_step yet (uses tracing only). |
| P1a.15 | NOT STARTED | - | Test fixtures (5s WAV, mock data) — defer to P1a.16 edge case pass |

### Chunk 3: Watcher + Voice Memos (P1a.11-13, P1a.12)
| Bead | Status | Score | Notes |
|------|--------|-------|-------|
| P1a.11 | DONE | 9/10 | Folder watcher: notify event loop, settle delay, lock file, move to processed/failed, skip processed/failed subdirs, process existing files on start. 10 tests. Missing: real whisper transcription (uses placeholder). |
| P1a.12 | DONE | 10/10 | Memo frontmatter: `type: memo`, `source: voice-memo`, `status: transcript-only/no-speech`. Separate memos/ subdirectory. |
| P1a.13 | NOT STARTED | - | Apple Shortcut (.shortcut file) — needs manual creation in Shortcuts app |

### Chunk 4: Polish + Edge Cases (P1a.9-10, P1a.16)
| Bead | Status | Score | Notes |
|------|--------|-------|-------|
| P1a.9 | DONE | 9/10 | README.md with install, usage, config, Claude integration sections. LICENSE (MIT). Missing: CONTRIBUTING.md. |
| P1a.10 | DONE | 10/10 | Git repo initialized, main branch, 2 commits. GitHub repo creation pending (needs `gh repo create`). |
| P1a.16 | NOT STARTED | - | Edge case test pass |

## Chunk Gates
- [ ] Chunk 1 gate: `minutes record` → `minutes stop` → markdown file appears
- [ ] Chunk 2 gate: `minutes setup` works, structured logs appear, tests pass
- [ ] Chunk 3 gate: drop .m4a in inbox → markdown appears in memos/
- [ ] Chunk 4 gate: `cargo test` passes, `cargo clippy` clean, `cargo build --release` clean

## Resume Instructions (for post-compaction)
1. Read this file to see current status
2. Read PLAN.md for task details and architecture decisions
3. Read CLAUDE.md for project conventions
4. Check `cargo build` status
5. Continue from the first NOT STARTED or IN PROGRESS bead
