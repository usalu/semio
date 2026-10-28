"""☀️ Generate TTS audio for Heating Module 5 (Solarer Wärmegewinn).

Narration text is imported from each Beat's NARRATION in scene_5.py.
"""

import argparse
import re
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

_scene = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.☀️solar-gains.🐍️")
Beat1_VerlustZuGewinn = _scene.Beat1_VerlustZuGewinn
Beat2_BestrahlungUndFlaeche = _scene.Beat2_BestrahlungUndFlaeche
Beat3_GWert = _scene.Beat3_GWert
Beat4_SaisonaleWinkel = _scene.Beat4_SaisonaleWinkel
Beat5_Verschattung = _scene.Beat5_Verschattung
Beat8_Hauptgleichung = _scene.Beat8_Hauptgleichung
_visuals = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
narration_text = _visuals.narration_text

BEATS = [
    Beat1_VerlustZuGewinn,
    Beat2_BestrahlungUndFlaeche,
    Beat3_GWert,
    Beat4_SaisonaleWinkel,
    Beat5_Verschattung,
    Beat8_Hauptgleichung,
]


def main(argv: list[str] | None = None):
    argparse.ArgumentParser(description=__doc__).parse_args(argv)
    print("=== Modul 5 (Solarer Wärmegewinn) TTS Audio Generation ===\n")
    for cls in BEATS:
        i = int(re.match(r"Beat(\d+)", cls.__name__).group(1))
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
