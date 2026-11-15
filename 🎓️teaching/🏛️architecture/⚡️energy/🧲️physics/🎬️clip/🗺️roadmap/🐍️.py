"""🗺️ Physical-fundamentals roadmap — the Fahrplan that opens the video, ahead of power and energy.

Six cards light up in order: power and energy, conservation, the heat pump, radiation, thermal mass, sensible and latent heat.
"""

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

from manim import *

_roadmap = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🗺️roadmap.🐍️")
RoadmapScene = _roadmap.RoadmapScene
_visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
load_vo_timing = _visuals.load_vo_timing

_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)


#region Beat1 – Roadmap of the video
class Beat1_Fahrplan(RoadmapScene):
    """🗺️ Roadmap of the physical fundamentals as six cards that light up in order."""

    NARRATION = [
        ("intro",
         "This video follows six steps.",
         "Dieses Video folgt sechs Schritten."),
        ("leistung",
         "First, power and energy: how fast energy flows, and what the kilowatt-hour counts.",
         "Zuerst Leistung und Energie: wie schnell Energie fließt und was die Kilowattstunde zählt."),
        ("erhaltung",
         "Then energy conservation: nothing is lost, energy only changes its form.",
         "Dann die Energieerhaltung: Nichts geht verloren, Energie wechselt nur die Form."),
        ("waermepumpe",
         "Next, the heat pump, which lifts energy to a higher temperature level.",
         "Danach die Wärmepumpe, die Energie auf ein höheres Temperaturniveau hebt."),
        ("strahlung",
         "Then radiation: short-wave in, long-wave trapped.",
         "Es folgt die Strahlung: kurzwellig hinein, langwellig gefangen."),
        ("masse",
         "Then thermal mass, which stores heat and releases it later.",
         "Dann die thermische Masse, die Wärme speichert und zeitversetzt abgibt."),
        ("latent",
         "Last, sensible and latent heat.",
         "Zuletzt sensible und latente Wärme."),
    ]

    HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")
    STEPS = [
        {"key": "leistung", "name": ("Power, energy", "Leistung, Energie"), "symbols": r"P,\;E", "pictogram": "bars"},
        {"key": "erhaltung", "name": ("Conservation", "Erhaltung"), "symbols": (r"E_{\mathrm{in}},\;E_{\mathrm{out}}", r"E_{\mathrm{ein}},\;E_{\mathrm{aus}}"), "pictogram": "balance"},
        {"key": "waermepumpe", "name": ("Heat pump", "Wärmepumpe"), "symbols": r"\mathrm{COP}", "pictogram": "heat-pump"},
        {"key": "strahlung", "name": ("Radiation", "Strahlung"), "symbols": r"g,\;I_{\mathrm{s}}", "pictogram": "sun"},
        {"key": "masse", "name": ("Thermal mass", "Therm. Masse"), "symbols": r"c,\;m,\;\Delta\theta", "pictogram": "wall"},
        {"key": "latent", "name": ("Sensible, latent", "Sensibel, latent"), "symbols": r"Q_{\mathrm{sens}},\;Q_{\mathrm{lat}}", "pictogram": "droplets"},
    ]
#endregion
