# minutes

> Every meeting, every idea, every voice note — searchable by your AI.

**minutes** is an open-source, privacy-first tool that turns any audio — meetings, voice memos, brain dumps — into searchable, AI-queryable memory. Not a meeting notes app — a conversation memory layer that integrates natively with the Claude ecosystem.

## What it does

```
minutes record          # Record a meeting (Ctrl-C to stop + transcribe)
minutes watch           # Watch a folder for voice memos, auto-process them
minutes search "pricing"  # Search all meetings and memos
minutes list            # List recent recordings
```

**Two input modes, one pipeline:**
- **Live recording** — `minutes record` captures system audio, transcribes locally, saves as searchable markdown
- **Folder watcher** — `minutes watch` monitors a folder for voice memos (from iPhone, AirDrop, etc.) and auto-processes them

All transcription happens locally via [whisper.cpp](https://github.com/ggerganov/whisper.cpp). Your audio never leaves your machine.

## Install

```bash
# From source (requires Rust)
cargo install --path crates/cli

# Setup whisper model (~466MB download)
minutes setup
```

### Prerequisites

- **macOS** (Windows/Linux support planned)
- **BlackHole** virtual audio device for system audio capture:
  ```bash
  brew install blackhole-2ch
  ```
  Then set up a Multi-Output Device in Audio MIDI Setup (one-time, ~3 min).

## Output

Meetings and memos are saved as markdown with YAML frontmatter:

```markdown
---
title: Advisor Pricing Discussion
type: meeting
date: 2026-03-17T14:00:00
duration: 42m
---

## Summary
- Agreed to price advisor platform at $499/mo minimum

## Transcript
[SPEAKER_0 0:00] So I think the pricing should...
```

Works with [Obsidian](https://obsidian.md), [QMD](https://github.com/matsilverstein/qmd), grep, or any markdown tool.

## Voice Memos (iPhone → Mac)

Record a voice memo on your iPhone, share it to Minutes via an Apple Shortcut, and it auto-processes into searchable markdown.

1. Install the "Save to Minutes" Apple Shortcut (included in this repo)
2. Run `minutes watch` on your Mac
3. Record in Voice Memos → Share → Save to Minutes
4. Markdown appears in `~/meetings/memos/`

No Full Disk Access required. No custom iOS app needed.

## Configuration

Config is optional — minutes works out of the box with sensible defaults.

```bash
# Config file location (created by `minutes setup`)
~/.config/minutes/config.toml
```

```toml
[transcription]
model = "small"          # tiny, base, small, medium, large-v3

[search]
engine = "builtin"       # or "qmd" for semantic search

[watch]
paths = ["~/.minutes/inbox"]
extensions = ["m4a", "wav", "mp3", "ogg", "webm"]
settle_delay_ms = 2000
```

## Claude Integration

minutes is designed to be a native extension for the Claude ecosystem:

- **Claude Desktop** — MCPB extension (coming soon) lets Claude query your meeting history
- **Claude Code** — Plugin with `/minutes search` and `/minutes record` skills
- **Cowork/Dispatch** — Trigger recordings from your phone, get summaries back

## Architecture

```
Audio Input → Transcribe (whisper.cpp) → [Diarize] → [Summarize] → Markdown
                                            ↑              ↑
                                       (Phase 1b)     (Phase 1b)
```

Built with Rust for speed and cross-platform support. The core library (`minutes-core`) is shared across CLI, MCPB, and the future Tauri menu bar app.

## License

MIT
