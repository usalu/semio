"""🎬 Render the section's authored beats with optional narration."""

from importlib import import_module
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent
SPEECH_DIR = BASE_DIR / "🔊️speech"
WORKSPACE_ROOT = next(parent for parent in BASE_DIR.parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))

build_section_video = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️").build_section_video

SCENES = [
    (BASE_DIR / "🐍️.py", "Beat1_VerlustZuGewinn", SPEECH_DIR / "beat_1_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat2_BestrahlungUndFlaeche", SPEECH_DIR / "beat_2_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat3_GWert", SPEECH_DIR / "beat_3_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat4_SaisonaleWinkel", SPEECH_DIR / "beat_4_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat5_Verschattung", SPEECH_DIR / "beat_5_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat6_Waermespeicherung", SPEECH_DIR / "beat_6_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat7_SpeichermasseFormel", SPEECH_DIR / "beat_7_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat8_Hauptgleichung", SPEECH_DIR / "beat_8_audio.mp3"),
]


def main() -> int:
    """▶️ Build every section beat, failing if any render or mux fails."""
    build_section_video(BASE_DIR, SCENES, "Full_Modul5_SolarerWaermegewinn.mp4")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
