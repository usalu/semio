"""🎬 Render the section's authored beats with optional narration."""

from importlib import import_module
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent
WORKSPACE_ROOT = next(parent for parent in BASE_DIR.parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))

SPEECH_DIR = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(BASE_DIR)

build_section_video = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️").build_section_video

SCENES = [
    (BASE_DIR / "🐍️.py", "Beat1_Fensterregeln", SPEECH_DIR / "beat_1_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat2_StossOderKipp", SPEECH_DIR / "beat_2_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat3_Querlueftung", SPEECH_DIR / "beat_3_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat4_Auftrieb", SPEECH_DIR / "beat_4_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat5_Nachtlueftung", SPEECH_DIR / "beat_5_audio.mp3"),
    (BASE_DIR / "🐍️.py", "Beat6_GrenzenDerFreienLueftung", SPEECH_DIR / "beat_6_audio.mp3"),
]


def main() -> int:
    """▶️ Build every section beat, failing if any render or mux fails."""
    build_section_video(BASE_DIR, SCENES, "Full_Chapter4_Natuerliche_Lueftung.mp4")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
