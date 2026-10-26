"""🎙️ Generate a short English VO clip for the NGS intro card (optional mux)."""

import argparse
from importlib import import_module
import sys
from pathlib import Path


BASE_DIR = Path(__file__).resolve().parent.parent
SPEECH_DIR = Path(__file__).resolve().parent
WORKSPACE_ROOT = next(parent for parent in BASE_DIR.parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))

_speech = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
synthesize_narration = _speech.synthesize_narration

NARRATIONS = {
    "intro_audio": (
        "Welcome to this tutorial from Sustainable Building Systems "
        "at Leibniz University Hannover. "
        "In this video, we explain the topic shown on screen."
    ),
    "intro_physical_fundamentals": (
        "Welcome to this tutorial from Sustainable Building Systems "
        "at Leibniz University Hannover. "
        "This video is the physical foundation: force, work, power and energy — "
        "the language of heating and cooling demand."
    ),
}


def main(argv: list[str] | None = None):
    argparse.ArgumentParser(description=__doc__).parse_args(argv)
    for name, text in NARRATIONS.items():
        out_path = SPEECH_DIR / f"{name}.mp3"
        result_path, skipped = synthesize_narration(text, out_path)
        print(name, "skipped" if skipped else result_path)


if __name__ == "__main__":
    main()
