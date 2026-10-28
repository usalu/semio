"""🏠 Generate TTS audio for Heating Module 4 (Interne Wärmegewinne).

Narration text is imported from each Beat's NARRATION in scene_4.py.
"""

import argparse
from importlib import import_module
import sys
from pathlib import Path


BASE_DIR = Path(__file__).resolve().parent.parent
WORKSPACE_ROOT = next(parent for parent in BASE_DIR.parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))

_output = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️")
SPEECH_DIR = _output.speech_root(BASE_DIR)

_speech = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
synthesize_narration = _speech.synthesize_narration

_scene = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.👥️internal-gains.🐍️")
Beat1_WinterInterneGewinne = _scene.Beat1_WinterInterneGewinne
Beat2_PersonenPhiP = _scene.Beat2_PersonenPhiP
Beat3_GeraetePhiE = _scene.Beat3_GeraetePhiE
Beat4_BeleuchtungPhiL = _scene.Beat4_BeleuchtungPhiL
Beat5_SummeUndDichte = _scene.Beat5_SummeUndDichte
_visuals = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
narration_text = _visuals.narration_text

BEATS = [
    Beat1_WinterInterneGewinne,
    Beat2_PersonenPhiP,
    Beat3_GeraetePhiE,
    Beat4_BeleuchtungPhiL,
    Beat5_SummeUndDichte,
]


def main(argv: list[str] | None = None):
    argparse.ArgumentParser(description=__doc__).parse_args(argv)
    print("=== Modul 4 (Interne Wärmegewinne) TTS Audio Generation ===\n")
    for i, cls in enumerate(BEATS, start=1):
        text = narration_text(cls.NARRATION)
        out_path = SPEECH_DIR / f"beat_{i}_audio.mp3"
        print(f"Generating: {cls.__name__} -> beat_{i}_audio ...")
        try:
            result_path, skipped = synthesize_narration(text, out_path)
            if skipped:
                print("  ⚠ Skipped (no TTS API key configured or empty text)")
            elif result_path:
                print(f"  ✓ Saved → {result_path}")
            else:
                print("  ✗ Failed (no output)")
        except Exception as e:
            print(f"  ✗ Error: {e}")
    print("\nDone!")


if __name__ == "__main__":
    main()
