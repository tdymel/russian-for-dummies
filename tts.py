import subprocess
import tempfile
from pathlib import Path

import soundfile as sf
from fastapi import FastAPI, HTTPException
from fastapi.responses import FileResponse
from pydantic import BaseModel
from silero import silero_tts

app = FastAPI()

model, _ = silero_tts(language="ru", speaker="v5_ru")


class TTSRequest(BaseModel):
    text: str


@app.post("/tts")
def tts(req: TTSRequest):
    text = req.text.strip()
    if not text:
        raise HTTPException(status_code=400, detail="text must not be empty")

    audio = model.apply_tts(text=text)

    wav_file = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    mp3_file = tempfile.NamedTemporaryFile(suffix=".mp3", delete=False)

    wav_path = Path(wav_file.name)
    mp3_path = Path(mp3_file.name)

    wav_file.close()
    mp3_file.close()

    try:
        sf.write(wav_path, audio, 48000)

        subprocess.run(
            [
                "ffmpeg",
                "-y",
                "-i",
                str(wav_path),
                "-codec:a",
                "libmp3lame",
                "-q:a",
                "2",
                str(mp3_path),
            ],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )

        wav_path.unlink(missing_ok=True)

        return FileResponse(
            path=str(mp3_path),
            media_type="audio/mpeg",
            filename="tts.mp3",
            background=None,
        )
    except subprocess.CalledProcessError:
        wav_path.unlink(missing_ok=True)
        mp3_path.unlink(missing_ok=True)
        raise HTTPException(status_code=500, detail="ffmpeg conversion failed")
    except Exception:
        wav_path.unlink(missing_ok=True)
        mp3_path.unlink(missing_ok=True)
        raise
