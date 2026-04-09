# Dual-Stream Audio Capture

Capture both sides of a call — your microphone and the other person's audio — into a single recording, fully local.

## How It Works

Minutes opens **two simultaneous capture streams**:

1. **Mic stream** — your voice, from the system default input device (MacBook mic, AirPods, webcam, etc.)
2. **Loopback stream** — system audio (the other person's voice), from a virtual audio device (BlackHole)

Both streams are resampled to 16 kHz mono, mixed sample-by-sample, and written to a single WAV file. Whisper transcribes the combined audio as usual.

```
┌─────────────┐     ┌──────────────┐
│  Your mic   │────▶│              │
│ (any device)│     │              │
└─────────────┘     │   Minutes    │     ┌─────────┐
                    │  mixer loop  │────▶│  .wav   │──▶ Whisper
┌─────────────┐     │  (16kHz mono)│     └─────────┘
│ BlackHole   │────▶│              │
│ (sys audio) │     │              │
└─────────────┘     └──────────────┘
```

### Why Not an Aggregate Device?

macOS Aggregate Devices combine multiple inputs into one virtual device, but they have limitations:

- **Rigid sub-devices** — you must pre-select which mic to include. Switching from MacBook mic to AirPods means reconfiguring the Aggregate Device.
- **Bluetooth unreliable** — AirPods and other Bluetooth devices don't work reliably as Aggregate Device sub-devices.
- **Channel mapping issues** — Aggregate Devices can show incorrect channel counts depending on sub-device configuration.

Dual-stream capture avoids all of this. The mic stream follows whatever macOS considers the "default input" — switch devices freely and Minutes adapts automatically.

## Prerequisites

### 1. Install BlackHole

BlackHole is a free, open-source virtual audio driver that creates a loopback device on macOS.

```bash
brew install blackhole-2ch
```

After installation, **restart your Mac** (or at minimum log out and back in) for the audio driver to load.

Verify it appears:

```bash
minutes devices
# Should show: BlackHole 2ch (48000Hz, 2 ch)
```

### 2. Create a Multi-Output Device

The Multi-Output Device sends audio to your speakers/headphones **and** BlackHole simultaneously, so you can hear the call while Minutes captures it.

1. Open **Audio MIDI Setup** (`/Applications/Utilities/Audio MIDI Setup.app`)
2. Click the **+** button at the bottom left → **Create Multi-Output Device**
3. Check these sub-devices:
   - **MacBook Pro Speakers** (or your preferred output)
   - **BlackHole 2ch**
4. Set **MacBook Pro Speakers** as the master device (clock source)
5. Rename it to something memorable (e.g., "Minutes Output") — right-click the device name to rename

### 3. Set the Multi-Output Device as System Output

Before starting a call:

1. Open **System Settings → Sound → Output**
2. Select the Multi-Output Device you just created

Or use the command line:

```bash
# Install SwitchAudioSource for easy switching
brew install switchaudio-osx

# Set output to Multi-Output Device
SwitchAudioSource -s "Minutes Output" -t output

# After the call, switch back
SwitchAudioSource -s "MacBook Pro Speakers" -t output
```

> **Note:** Volume control doesn't work on Multi-Output Devices — macOS disables the volume slider. Adjust volume in the app (Zoom, Teams, etc.) or use the sub-device's volume before creating the Multi-Output Device.

## Configuration

Add `loopback_device` to your Minutes config:

```toml
# ~/.config/minutes/config.toml

[recording]
loopback_device = "BlackHole 2ch"
```

That's it. When `loopback_device` is set, Minutes automatically uses dual-stream capture.

### Optional: Force a Specific Mic

By default, the mic stream uses whatever macOS reports as the system default input. To override:

```toml
[recording]
device = "MacBook Pro Microphone"    # Force a specific mic
loopback_device = "BlackHole 2ch"    # System audio loopback
```

This is usually unnecessary — leaving `device` unset lets you switch between AirPods, webcam mic, and built-in mic without touching the config.

## Usage

Recording works exactly the same as before:

```bash
minutes record --title "weekly standup"
# ... have your call ...
minutes stop
```

The console output confirms dual-stream mode:

```
[minutes] Dual-stream capture: mic=MacBook Pro Microphone, loopback=BlackHole 2ch
[minutes] Screen context capture enabled (every 30s)
```

If you switch audio input devices mid-call (e.g., connecting AirPods), Minutes detects the change and reconnects the mic stream automatically:

```
[minutes] Mic switched: MacBook Pro Microphone → AirPods Pro
```

The loopback stream (BlackHole) remains stable throughout.

## How the Mixing Works

Every 100 ms, the main loop:

1. Drains the mic sample buffer and the loopback sample buffer
2. Adds them sample-by-sample (with clipping at ±32767)
3. Writes the mixed samples to the WAV file

Both streams are independently resampled from their native sample rate (typically 48 kHz) down to 16 kHz mono before mixing. Small timing differences between the two streams (a few milliseconds) are imperceptible to Whisper.

The audio level meter reflects the **mic** input only, so silence detection and notifications work as expected — they track whether *you* are speaking, not whether system audio is playing.

## Troubleshooting

### "audio device 'BlackHole 2ch' not found"

BlackHole isn't installed or the driver hasn't loaded. Run:

```bash
brew install blackhole-2ch
# Then restart your Mac
```

### Recording captures my voice but not the other person

The Multi-Output Device isn't set as system output, so call audio isn't routing through BlackHole.

1. Check **System Settings → Sound → Output** — it should show your Multi-Output Device
2. If using SwitchAudioSource: `SwitchAudioSource -s "Minutes Output" -t output`
3. Some apps (Zoom, Teams) have their own audio output settings — check that the app is using "System Default" or the Multi-Output Device

### Recording captures the other person but not me

The mic device selection failed. Check:

```bash
minutes devices
```

Verify your mic appears in the list. If using `device` override in config, ensure the name matches exactly.

### Both sides captured but transcript is garbled

If the two audio streams have very different volumes (e.g., loud mic + quiet system audio), Whisper may struggle. Try:

- Adjusting the call app's volume
- Moving your mic further away (if it's much louder than the call audio)
- Using `--model large-v3` for better transcription quality

### "no-speech" result despite talking

If `peak audio level: 0` appears in the output, the mic stream isn't capturing audio:

1. Check that your mic isn't muted at the system level
2. Try without `device` override (let it use system default)
3. Test with single-stream mode by removing `loopback_device` from config

## Disabling Dual-Stream

To go back to single-stream capture (mic only), remove or comment out the loopback line:

```toml
[recording]
# loopback_device = "BlackHole 2ch"
```

Minutes will use the standard single-stream capture path — identical to the behavior before this feature was added.
