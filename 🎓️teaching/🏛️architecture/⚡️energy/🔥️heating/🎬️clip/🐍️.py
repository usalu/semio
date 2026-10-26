"""🎬 Full Heating demand series — Modul 1–5 + final calculation.

Recommended: run this file as a script (section renders + ffmpeg concat).
A single Manim mega-scene can fail when combining hundreds of partials
(especially with spaces in the project path).

See ``README.md`` in this folder.
"""

from __future__ import annotations

from importlib import import_module
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from manim import Scene

# region Paths
_HEATING_ROOT = Path(__file__).resolve().parent
WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))
_runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
prepare_output = _runtime.prepare_output

prepare_output(sys.argv[1:])
_INTRO_SCRIPT = WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
SERIES_INTRO_SCENE = "Demo_Intro_Heizlast"
# endregion

begin_vo_beat = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️").begin_vo_beat


# region Module Loader
# endregion


# region Section Modules
_m1 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧲️fundamentals.🐍️")
_m2 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧱️conduction.🐍️")
_m3 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🌬️convection.🐍️")
_m4 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.👥️internal-gains.🐍️")
_m5 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.☀️solar-gains.🐍️")
_final = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")
# endregion


# region Playlist
HEATING_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Modul 1 · Grundlagen der Bauphysik",
        [
            _m1.Beat1_DreiWegeDerWaerme,
            _m1.Beat2_Waermeleitung,
            _m1.Beat3_Konvektion,
            _m1.Beat4_Strahlung,
            _m1.Beat5_Zusammenfassung,
            _m1.Beat6_VonWegenZuZahlen,
            _m1.Beat7_Waermedurchlasswiderstand,
            _m1.Beat8_UWert,
            _m1.Beat9_WaermestromFormel,
        ],
    ),
    (
        "Modul 2 · Transmission",
        [
            _m2.Beat1_MakroUndMikro,
            _m2.Beat2_RWert,
            _m2.Beat3_UWertUndGradient,
            _m2.Beat4_WattVergleich,
            _m2.Beat5_Gebaeudehuelle,
            _m2.Beat6_Waermebruecken,
        ],
    ),
    (
        "Modul 3 · Lüftung im Winter",
        [
            _m3.Beat1_LueftungImWinter,
        ],
    ),
    (
        "Modul 4 · Interne Wärmegewinne",
        [
            _m4.Beat1_WinterInterneGewinne,
            _m4.Beat2_PersonenPhiP,
            _m4.Beat3_GeraetePhiE,
            _m4.Beat4_BeleuchtungPhiL,
            _m4.Beat5_SummeUndDichte,
        ],
    ),
    (
        "Modul 5 · Solarer Wärmegewinn",
        [
            _m5.Beat1_VerlustZuGewinn,
            _m5.Beat2_BestrahlungUndFlaeche,
            _m5.Beat3_GWert,
            _m5.Beat4_SaisonaleWinkel,
            _m5.Beat5_Verschattung,
            _m5.Beat6_Waermespeicherung,
            _m5.Beat7_SpeichermasseFormel,
            _m5.Beat8_Hauptgleichung,
        ],
    ),
    (
        "Final Calculation · Heizwärmebedarf",
        [
            _final.ReviewingHeatLosses,
            _final.Scene2,
            _final.ReviewingHeatGains,
            _final.Scene4,
            _final.UltimateEnergyBalance,
            _final.AnlagenVerluste,
        ],
    ),
]
# endregion


# region Host Binding
_BIND_ATTRS = (
    "NARRATION",
    "topic_de",
    "topic_explain_de",
    "series_de",
    "hold_seconds",
)


def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Copy scene class attrs onto the host (needed for ``scene_cls.construct(host)``).

    Also reset VO spend per beat — section hosts reuse one ``Scene``, and shared
    clause keys would otherwise deplete ``hold_for`` budgets across beats.
    """
    for name in _BIND_ATTRS:
        if hasattr(scene_cls, name):
            setattr(host, name, getattr(scene_cls, name))
    begin_vo_beat(host, scene_cls.__name__)
# endregion


# region Section Scenes
class _HeatingSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Heating_01_Introduction(_HeatingSection):
    """1️⃣ Modul 1 — Grundlagen der Bauphysik."""

    section_beats = HEATING_PLAYLIST[0][1]


class Heating_02_Conduction(_HeatingSection):
    """2️⃣ Modul 2 — Transmission."""

    section_beats = HEATING_PLAYLIST[1][1]


class Heating_03_Convection(_HeatingSection):
    """3️⃣ Modul 3 — Lüftung im Winter (bridge to the Lüftung video)."""

    section_beats = HEATING_PLAYLIST[2][1]


class Heating_04_InternalGains(_HeatingSection):
    """4️⃣ Modul 4 — Interne Wärmegewinne."""

    section_beats = HEATING_PLAYLIST[3][1]


class Heating_05_Solar(_HeatingSection):
    """5️⃣ Modul 5 — Solarer Wärmegewinn."""

    section_beats = HEATING_PLAYLIST[4][1]


class Heating_06_FinalCalculation(_HeatingSection):
    """6️⃣ Final calculation — Heizwärmebedarf."""

    section_beats = HEATING_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Heating_01_Introduction,
    Heating_02_Conduction,
    Heating_03_Convection,
    Heating_04_InternalGains,
    Heating_05_Solar,
    Heating_06_FinalCalculation,
]
# endregion


# region Full Series Scene (fragile mega-combine)
class FullHeatingDemandVideo(Scene):
    """🔥 Entire series in one Manim scene.

    Prefer running this file as a script (section renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _section, scenes in HEATING_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()
# endregion


# region Compose (recommended)
def compose_full_heating_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    output_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render the authored intro and chapters through the portable tool interface."""

    return _runtime.compose_video("heating", quality_flag=quality_flag, play=play,
                         media_dir=media_dir, output_dir=output_dir,
                         force=force, concat_only=concat_only)


def main(argv: list[str] | None = None) -> int:
    """▶️ Render the complete video with explicit output and cancellation support."""

    return _runtime.video_main("heating", argv)


if __name__ == "__main__":
    raise SystemExit(main())
