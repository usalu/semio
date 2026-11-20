"""🗺️ Heating roadmap — the Fahrplan that opens the Heizung video, ahead of the basics.

Six cards light up in the order of the chapters; ventilation hands the load over at card three, the last card hands the heating demand to the energy balance.
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
    """🗺️ Roadmap of the heating video as six cards in chapter order, with the ventilation load coming in and the heating demand going out."""

    NARRATION = [
        ("intro",
         "This video follows six steps.",
         "Dieses Video folgt sechs Schritten."),
        ("grundlagen",
         r"First, the basics: the heat flow $\dot{Q}$ and the temperature difference $\Delta\theta$.",
         r"Zuerst die Grundlagen: der Wärmestrom $\dot{Q}$ und die Temperaturdifferenz $\Delta\theta$."),
        ("leitung",
         r"Then conduction through the envelope: the $U$-value, the conductivity $\lambda$ and thermal bridges $\psi$.",
         r"Dann die Wärmeleitung durch die Hülle: der $U$-Wert, die Wärmeleitfähigkeit $\lambda$ und Wärmebrücken $\psi$."),
        ("lueftung",
         r"Next, ventilation in winter: the load $\dot{Q}_{\mathrm{V}}$ that the ventilation video hands over.",
         r"Danach die Lüftung im Winter: die Last $\dot{Q}_{\mathrm{V}}$, die das Lüftungsvideo übergibt."),
        ("intern",
         r"Then the internal gains $\dot{Q}_{i}$ from people, devices and lighting.",
         r"Es folgen die internen Gewinne $\dot{Q}_{i}$ durch Personen, Geräte und Beleuchtung."),
        ("solar",
         r"Then the solar gains $\dot{Q}_{\mathrm{S}}$ through windows, with the $g_\perp$-value and the sun-protection factor $F_{\mathrm{C}}$.",
         r"Dann die solaren Gewinne $\dot{Q}_{\mathrm{S}}$ durch Fenster, mit dem $g_\perp$-Wert und dem Sonnenschutzfaktor $F_{\mathrm{C}}$."),
        ("bilanz",
         r"Last, the balance: the heating demand $Q_{\mathrm{h}}$ is losses minus utilised gains, and goes on to the energy balance.",
         r"Zuletzt die Bilanz: Der Heizwärmebedarf $Q_{\mathrm{h}}$ ist Verluste minus genutzte Gewinne und geht an die Energiebilanz."),
    ]

    HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")
    STEPS = [
        {"key": "grundlagen", "name": ("Basics", "Grundlagen"), "symbols": r"\dot{Q},\;\Delta\theta", "pictogram": "thermometer"},
        {"key": "leitung", "name": ("Conduction", "Wärmeleitung"), "symbols": r"U,\;\lambda,\;\psi", "pictogram": "wall"},
        {"key": "lueftung", "name": ("Ventilation", "Lüftung"), "symbols": r"\dot{Q}_{\mathrm{V}}", "pictogram": "air-room",
         "receives": [{"text": (r"←\;\text{Ventilation:}\;\dot{Q}_{\mathrm{V}}", r"←\;\text{Lüftung:}\;\dot{Q}_{\mathrm{V}}"), "color": "P_ORANGE"}]},
        {"key": "intern", "name": ("Internal gains", "Interne Gewinne"), "symbols": r"\dot{Q}_{i}", "pictogram": "office-worker"},
        {"key": "solar", "name": ("Solar gains", "Solare Gewinne"), "symbols": r"\dot{Q}_{\mathrm{S}},\;g_\perp,\;F_{\mathrm{C}}", "pictogram": "sun"},
        {"key": "bilanz", "name": ("Balance", "Bilanz"), "symbols": r"Q_{\mathrm{h}}", "pictogram": "balance"},
    ]
    HANDOVERS = [
        {"text": (r"\to\;\text{Energy balance:}\;Q_{\mathrm{h}}", r"\to\;\text{Energiebilanz:}\;Q_{\mathrm{h}}"), "color": "P_ORANGE"},
    ]
#endregion
