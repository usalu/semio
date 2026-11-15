"""🗺️ Energy-balance roadmap — the Fahrplan that opens the Energiebilanz video, ahead of the balance chapter.

Seven cards light up in the order of the chapters; the first card receives the heating and cooling demand from the two earlier videos.
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
    """🗺️ Roadmap of the energy-balance video as seven cards in chapter order, with the heating and cooling demand coming in."""

    NARRATION = [
        ("intro",
         "This video follows seven steps.",
         "Dieses Video folgt sieben Schritten."),
        ("bilanz",
         r"First, the energy balance: the heating demand $Q_{\mathrm{h}}$ and the cooling demand $Q_{\mathrm{c}}$ that the heating and cooling videos hand over.",
         r"Zuerst die Energiebilanz: der Heizwärmebedarf $Q_{\mathrm{h}}$ und der Kältebedarf $Q_{\mathrm{c}}$, die das Heizungs- und das Kühlungsvideo übergeben."),
        ("regelwerk",
         r"Then the rules and the climate data: monthly balances and the degree days $G_{t,20/15}$.",
         r"Dann Regelwerk und Klimadaten: Monatsbilanzen und die Gradtagszahl $G_{t,20/15}$."),
        ("leistung",
         r"Next, demand versus load: the heating load $\dot{Q}_{\mathrm{H}}$ is a power, the demand $Q_{\mathrm{h}}$ an energy.",
         r"Danach Bedarf und Leistung: Die Heizlast $\dot{Q}_{\mathrm{H}}$ ist eine Leistung, der Bedarf $Q_{\mathrm{h}}$ eine Energie."),
        ("anlage",
         r"Then from useful to final energy: the plant expenditure figure $e_{E}$ leads to the final energy $Q_{\mathrm{E}}$.",
         r"Dann von der Nutz- zur Endenergie: Die Anlagenaufwandszahl $e_{E}$ führt zur Endenergie $Q_{\mathrm{E}}$."),
        ("primaer",
         r"Then primary energy: the factor $f_{\mathrm{P}}$ turns the final energy into the primary energy $Q_{\mathrm{P}}$.",
         r"Dann die Primärenergie: Der Faktor $f_{\mathrm{P}}$ macht aus der Endenergie die Primärenergie $Q_{\mathrm{P}}$."),
        ("ausweis",
         "Then the energy performance certificate, which sorts a building into an efficiency class.",
         "Es folgt der Energieausweis, der ein Gebäude in eine Effizienzklasse einordnet."),
        ("praxis",
         "Last, theory versus practice: a calculated demand is not a measured consumption.",
         "Zuletzt Theorie und Praxis: Ein berechneter Bedarf ist kein gemessener Verbrauch."),
    ]

    HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")
    STEPS = [
        {"key": "bilanz", "name": ("Balance", "Bilanz"), "symbols": r"Q_{\mathrm{h}},\;Q_{\mathrm{c}}", "pictogram": "balance",
         "receives": [{"text": (r"←\;\text{Heating:}\;Q_{\mathrm{h}}", r"←\;\text{Heizung:}\;Q_{\mathrm{h}}"), "color": "P_ORANGE"},
                      {"text": (r"←\;\text{Cooling:}\;Q_{\mathrm{c}}", r"←\;\text{Kühlung:}\;Q_{\mathrm{c}}"), "color": "P_CYAN"}]},
        {"key": "regelwerk", "name": ("Climate", "Klima"), "symbols": r"\theta_{\mathrm{e}},\;G_{t,20/15}", "pictogram": "thermometer"},
        {"key": "leistung", "name": ("Load", "Leistung"), "symbols": r"\dot{Q}_{\mathrm{H}},\;Q_{\mathrm{h}}", "pictogram": "bars"},
        {"key": "anlage", "name": ("Final energy", "Endenergie"), "symbols": r"e_{E},\;Q_{\mathrm{E}}", "pictogram": "meter"},
        {"key": "primaer", "name": ("Primary energy", "Primärenergie"), "symbols": r"f_{\mathrm{P}},\;Q_{\mathrm{P}}", "pictogram": "factory"},
        {"key": "ausweis", "name": ("Label", "Ausweis"), "symbols": r"\mathrm{A}+\;\to\;\mathrm{H}", "pictogram": "label"},
        {"key": "praxis", "name": ("Practice", "Praxis"), "symbols": (r"Q_{\mathrm{calc}},\;Q_{\mathrm{meas}}", r"Q_{\mathrm{ber}},\;Q_{\mathrm{mess}}"), "pictogram": "valve"},
    ]
#endregion
