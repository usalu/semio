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
    (BASE_DIR / "🐍️.py", "Beat1_Grundtypen", SPEECH_DIR / "beat_1_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat2_Waermerueckgewinnung", SPEECH_DIR / "beat_2_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat3_Anlagenvergleich", SPEECH_DIR / "beat_3_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat4_Kanalquerschnitt", SPEECH_DIR / "beat_4_audio.mp3"),
]


def main() -> int:
    """▶️ Build every section beat, failing if any render or mux fails."""
    build_section_video(BASE_DIR, SCENES, "Full_Chapter5_Mechanische_Lueftung.mp4")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
