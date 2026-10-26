"""🧱 Generate TTS audio for Heating Module 2 (Transmission).

Narration text is imported from each Beat's NARRATION in scene_2.py.
"""

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

_scene = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧱️conduction.🐍️")
Beat1_MakroUndMikro = _scene.Beat1_MakroUndMikro
Beat2_RWert = _scene.Beat2_RWert
Beat3_UWertUndGradient = _scene.Beat3_UWertUndGradient
Beat4_WattVergleich = _scene.Beat4_WattVergleich
Beat5_Gebaeudehuelle = _scene.Beat5_Gebaeudehuelle
Beat6_Waermebruecken = _scene.Beat6_Waermebruecken
_visuals = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
narration_text = _visuals.narration_text

BEATS = [
    Beat1_MakroUndMikro,
    Beat2_RWert,
    Beat3_UWertUndGradient,
    Beat4_WattVergleich,
    Beat5_Gebaeudehuelle,
    Beat6_Waermebruecken,
]


def main(argv: list[str] | None = None):
    argparse.ArgumentParser(description=__doc__).parse_args(argv)
    print("=== Modul 2 (Transmission) TTS Audio Generation ===\n")
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
