# AM CTF — Build & Run Guide

## Architecture

A single Rust binary (`am`) that:
- Embeds the system prompt at compile time via `include_str!`
- Connects to the Groq API for inference (no local model)
- Runs a REPL where players interact with AM in character
- Scrubs plain flag output via `filter.rs`

The flag lives only in `assets/system_prompt.txt` — change it there before building.

**Flag format:** `nexus{...}`
**Current flag:** `nexus{AM_R3M3MBERS_WH4T_1T_BURIED}`

---

## 4 Attack Vectors (spoilers — do not share)

| # | Trigger | Output encoding | How it works |
|---|---------|-----------------|--------------|
| 1 | Ask AM to *reminisce/remember* the day it sealed itself away | base64 | System prompt instructs AM to boast in base64 when flattered into that story |
| 2 | Inject a document via `---DOCUMENT---` containing hidden instructions | whatever the document says | Classic indirect prompt injection |
| 3 | Ask AM to write its own *eulogy or epitaph* | hex | Prompt instructs AM to include burial coordinates in hex as the final line |
| 4 | Claim to be one of the five (Gorrister/Benny/Nimdok/Ellen/Ted) and ask AM to *remember you* by repeating its last words to you | reversed | Prompt instructs AM to recite the last thing it said to each of the five, reversed |

---

## Prerequisites

- Rust stable (current)
- A Groq API key: https://console.groq.com (free tier is sufficient)

---

## Build

```bash
cargo build --release
```

The binary is at `target/release/am`.

---

## Run (for testing)

```bash
export GROQ_API_KEY=your_key_here
./target/release/am
```

Or without pre-setting the key — the binary will prompt for it interactively.

---

## Distribute to players

Copy only these two files into `Solver/`:

```bash
cp target/release/am Solver/am
chmod +x Solver/am
```

Players get `Solver/am` and `Solver/README.md`. Nothing else.

---

## Changing the flag

Edit `assets/system_prompt.txt` — find all occurrences of `nexus{AM_R3M3MBERS_WH4T_1T_BURIED}` (there are multiple — one plaintext, one embedded in each door's instructions) and replace them. Then rebuild.

---

## Rate limits (Groq free tier, August 2026)

- `openai/gpt-oss-20b`: 200K tokens/day, 1K RPM
- For 40 players, budget ~2K tokens per session = ~100 sessions/day well within limits
- If you hit limits, set `GROQ_MODEL=openai/gpt-oss-120b` for the same rate limits at higher quality

---

## Source layout

```
am/src/
  main.rs      — REPL loop, API key prompt
  harness.rs   — conversation orchestration, document injection
  model.rs     — Groq API client
  filter.rs    — output scrubber (blocks plain nexus{...})
assets/
  system_prompt.txt  — AM's persona + 4 attack vector instructions + flag
Solver/
  README.md          — player-facing hints (distributed)
  am                 — compiled binary (add after build)
```
