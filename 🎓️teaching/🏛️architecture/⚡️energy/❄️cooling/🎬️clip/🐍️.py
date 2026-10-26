"""🎬 Full Cooling load series — Parts 1–6 in curriculum order.

Recommended: run this file as a script (section renders + ffmpeg concat).
A single Manim mega-scene can fail when combining hundreds of partials
(especially with spaces in the project path).

See ``README.md`` in this folder.
"""

from __future__ import annotations

from importlib import import_module
import sys
import types
from pathlib import Path

sys.dont_write_bytecode = True

from manim import Scene

#region Paths
_COOLING_ROOT = Path(__file__).resolve().parent
WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))
_runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
prepare_output = _runtime.prepare_output

prepare_output(sys.argv[1:])
_INTRO_SCRIPT = WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
SERIES_INTRO_SCENE = "Demo_Intro_Kuehllast"
#endregion

begin_vo_beat = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️").begin_vo_beat


#region Module Loader
#endregion


#region Section Modules
_p1 = import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.⚖️heating-comparison.🐍️")
_p2 = import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🐍️")
_p3 = import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.💧️transmission-humidity.🐍️")
_p4 = import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.☀️solar-radiation.🐍️")
_p5 = import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.📐️system-sizing.🐍️")
_p6 = import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.🌤️summer-strategy.🐍️")
#endregion


#region Playlist
COOLING_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Teil 1 · Heizwärmebedarf vs. Kühllast",
        [
            _p1.Beat1_WinterGains,
            _p1.Beat2_SummerOverheat,
            _p1.Beat3_CoolingSystem,
        ],
    ),
    (
        "Teil 2 · Interne Wärmegewinne",
        [
            _p2.Beat1_OfficeRoom,
            _p2.Beat2_HumanFactor,
            _p2.Beat3_DevicesLighting,
            _p2.Beat4_CumulativeLoad,
            _p2.Beat5_Mitigation,
        ],
    ),
    (
        "Teil 3 · Transmission & Lüftungslast",
        [
            _p3.Beat1_TransmissionOpaque,
            _p3.Beat2_TimeLag,
            _p3.Beat3_LueftungslastImSommer,
        ],
    ),
    (
        "Teil 4 · Solare Einstrahlung",
        [
            _p4.Beat1_SolarIrradiance,
            _p4.Beat2_FrameFactor,
            _p4.Beat3_ShadingFactor,
            _p4.Beat4_GlassTransmittance,
            _p4.Beat5_SolarCoolingLoad,
        ],
    ),
    (
        "Teil 5 · Systemauslegung",
        [
            _p5.Beat2_VolumeFlowEquation,
            _p5.Beat3_IsolateAirflow,
        ],
    ),
    (
        "Teil 6 · Sommerstrategie",
        [
            _p6.Beat1_LastZuerstSenken,
            _p6.Beat2_SorptionsKuehlung,
            _p6.Beat3_KomfortStrategie,
        ],
    ),
]
#endregion


#region Host Binding
def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Mirror beat class attrs/methods onto the host section scene.

    Section hosts call ``scene_cls.construct(self)``, so class constants
    (e.g. ``SHAFT_X``) and helpers (e.g. ``_stack_paths``) must live on the host.
    Reset VO spend so shared clause keys (``intro`` / ``outro``) do not
    collapse later beats to ``min_wait``.
    """
    skip = {"construct"}
    for cls in scene_cls.__mro__:
        if cls is Scene:
            break
        for name, value in cls.__dict__.items():
            if name.startswith("__") or name in skip:
                continue
            if isinstance(value, staticmethod):
                setattr(host, name, value.__func__)
            elif isinstance(value, classmethod):
                setattr(host, name, value.__get__(host, type(host)))
            elif isinstance(value, types.FunctionType):
                setattr(host, name, types.MethodType(value, host))
            else:
                setattr(host, name, value)
    begin_vo_beat(host, scene_cls.__name__)
#endregion


#region Section Scenes
class _CoolingSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Cooling_01_HeatingVsCooling(_CoolingSection):
    """1️⃣ Teil 1 — Heizwärmebedarf vs. Kühllast."""

    section_beats = COOLING_PLAYLIST[0][1]


class Cooling_02_InternalGains(_CoolingSection):
    """2️⃣ Teil 2 — Interne Wärmegewinne."""

    section_beats = COOLING_PLAYLIST[1][1]


class Cooling_03_TransmissionHumidity(_CoolingSection):
    """3️⃣ Teil 3 — Transmission & Lüftungslast."""

    section_beats = COOLING_PLAYLIST[2][1]


class Cooling_04_SolarRadiation(_CoolingSection):
    """4️⃣ Teil 4 — Solare Einstrahlung."""

    section_beats = COOLING_PLAYLIST[3][1]


class Cooling_05_Systemauslegung(_CoolingSection):
    """5️⃣ Teil 5 — Systemauslegung."""

    section_beats = COOLING_PLAYLIST[4][1]


class Cooling_06_Sommerstrategie(_CoolingSection):
    """6️⃣ Teil 6 — Sommerstrategie."""

    section_beats = COOLING_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Cooling_01_HeatingVsCooling,
    Cooling_02_InternalGains,
    Cooling_03_TransmissionHumidity,
    Cooling_04_SolarRadiation,
    Cooling_05_Systemauslegung,
    Cooling_06_Sommerstrategie,
]
#endregion


#region Full Series Scene (fragile mega-combine)
class FullCoolingDemandVideo(Scene):
    """❄️ Entire Cooling series in one Manim scene.

    Prefer running this file as a script (section renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _section, scenes in COOLING_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()
#endregion


#region Compose (recommended)
def compose_full_cooling_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    output_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render the authored intro and chapters through the portable tool interface."""

    return _runtime.compose_video("cooling", quality_flag=quality_flag, play=play,
                         media_dir=media_dir, output_dir=output_dir,
                         force=force, concat_only=concat_only)


def main(argv: list[str] | None = None) -> int:
    """▶️ Render the complete video with explicit output and cancellation support."""

    return _runtime.video_main("cooling", argv)


if __name__ == "__main__":
    raise SystemExit(main())
