# Russian for Dummies

## How to run the application
```sh
// Install TTS model
python -m venv .venv
pip install scipy audioop fastapi uvicorn silero soundfile pydub torch
// Run Anki+Desktop and Install AnkiConnectPlus Addon
LIBGL_ALWAYS_SOFTWARE=1 anki
source .venv/bin/activate
// Run TTS model
uvicorn tts:app --host 0.0.0.0 --port 8000
// Generate and Sync deck
cargo run -- --anki
```