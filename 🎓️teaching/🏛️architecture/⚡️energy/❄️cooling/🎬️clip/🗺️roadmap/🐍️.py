"""🗺️ Cooling roadmap — the Fahrplan that opens the Kühlung video, ahead of heating versus cooling.

Six cards light up in the order of the chapters; ventilation hands the load over at card three, the last card hands the cooling demand to the energy balance.
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
    """🗺️ Roadmap of the cooling video as six cards in chapter order, with the ventilation load coming in and the cooling demand going out."""

    NARRATION = [
        ("intro",
         "This video follows six steps.",
         "Dieses Video folgt sechs Schritten."),
        ("vergleich",
         "First, heating versus cooling: in summer the gains become the problem.",
         "Zuerst Heizen und Kühlen im Vergleich: Im Sommer werden die Gewinne zum Problem."),
        ("intern",
         r"Then the internal loads $\dot{Q}_{\mathrm{i}}$ of people, devices and lighting.",
         r"Dann die internen Lasten $\dot{Q}_{\mathrm{i}}$ von Personen, Geräten und Beleuchtung."),
        ("transmission",
         r"Next, transmission $\dot{Q}_{\mathrm{T}}$, thermal mass and the ventilation load $\dot{Q}_{\mathrm{V}}$ that the ventilation video hands over.",
         r"Danach die Transmission $\dot{Q}_{\mathrm{T}}$, die thermische Masse und die Lüftungslast $\dot{Q}_{\mathrm{V}}$, die das Lüftungsvideo übergibt."),
        ("solar",
         r"Then solar radiation $\dot{Q}_{\mathrm{S}}$ and the sun-protection factor $F_{\mathrm{C}}$.",
         r"Dann die Sonneneinstrahlung $\dot{Q}_{\mathrm{S}}$ und der Sonnenschutzfaktor $F_{\mathrm{C}}$."),
        ("system",
         r"Then sizing the system: from the cooling load $\dot{Q}_{\mathrm{C}}$ to the air flow $\dot{V}$.",
         r"Es folgt die Systemauslegung: von der Kühllast $\dot{Q}_{\mathrm{C}}$ zum Luftstrom $\dot{V}$."),
        ("strategie",
         r"Last, the summer strategy: cut the load first, then cool. The cooling demand $Q_{\mathrm{c}}$ goes on to the energy balance.",
         r"Zuletzt die Sommerstrategie: erst die Last senken, dann kühlen. Der Kältebedarf $Q_{\mathrm{c}}$ geht an die Energiebilanz."),
    ]

    HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")
    STEPS = [
        {"key": "vergleich", "name": ("Heating, cooling", "Heizen, Kühlen"), "symbols": r"\dot{Q}_{\mathrm{H}},\;\dot{Q}_{\mathrm{C}}", "pictogram": "thermometer"},
        {"key": "intern", "name": ("Internal loads", "Interne Lasten"), "symbols": r"\dot{Q}_{\mathrm{i}}", "pictogram": "seated-worker"},
        {"key": "transmission", "name": ("Transmission", "Transmission"), "symbols": r"\dot{Q}_{\mathrm{T}},\;\dot{Q}_{\mathrm{V}}", "pictogram": "wall",
         "receives": [{"text": (r"←\;\text{Ventilation:}\;\dot{Q}_{\mathrm{V}}", r"←\;\text{Lüftung:}\;\dot{Q}_{\mathrm{V}}"), "color": "P_CYAN"}]},
        {"key": "solar", "name": ("Solar load", "Solare Last"), "symbols": r"\dot{Q}_{\mathrm{S}},\;F_{\mathrm{C}}", "pictogram": "sun"},
        {"key": "system", "name": ("System sizing", "Auslegung"), "symbols": r"\dot{Q}_{\mathrm{C}},\;\dot{V}", "pictogram": "diffuser"},
        {"key": "strategie", "name": ("Summer strategy", "Sommerstrategie"), "symbols": r"Q_{\mathrm{c}}", "pictogram": "exchanger"},
    ]
    HANDOVERS = [
        {"text": (r"\to\;\text{Energy balance:}\;Q_{\mathrm{c}}", r"\to\;\text{Energiebilanz:}\;Q_{\mathrm{c}}"), "color": "P_CYAN"},
    ]
#endregion
