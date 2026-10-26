"""🎬 Full Ventilation series — Chapters 1–6 in curriculum order (video 2 of 5).

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
_VENTILATION_ROOT = Path(__file__).resolve().parent
WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))
_runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
prepare_output = _runtime.prepare_output

prepare_output(sys.argv[1:])
_INTRO_SCRIPT = WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
SERIES_INTRO_SCENE = "Demo_Intro_Lueftung"
#endregion

begin_vo_beat = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️").begin_vo_beat


#region Module Loader
#endregion


#region Section Modules
_p1 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.❓️need.🐍️")
_p2 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🐍️")
_p3 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🌡️heat-flow.🐍️")
_p4 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🪟️natural.🐍️")
_p5 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.⚙️mechanical.🐍️")
_p6 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🤝️handoff.🐍️")
#endregion


#region Playlist
VENTILATION_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Kapitel 1 · Warum lüften?",
        [_p1.Beat1_Lueftungsbedarf, _p1.Beat2_Lueftungsweg, _p1.Beat3_Fahrplan],
    ),
    (
        "Kapitel 2 · Wie viel Luft?",
        [_p2.Beat1_Innenvolumen, _p2.Beat2_Luftwechselrate, _p2.Beat3_Volumenstrom],
    ),
    (
        "Kapitel 3 · Was die Luft trägt",
        [_p3.Beat1_Luftwaerme, _p3.Beat2_SensibleWaerme, _p3.Beat3_LatenteLast, _p3.Beat4_WinterUndSommer],
    ),
    (
        "Kapitel 4 · Natürliche Lüftung",
        [
            _p4.Beat1_Fensterregeln,
            _p4.Beat2_StossOderKipp,
            _p4.Beat3_Querlueftung,
            _p4.Beat4_Auftrieb,
            _p4.Beat5_Nachtlueftung,
            _p4.Beat6_GrenzenDerFreienLueftung,
        ],
    ),
    (
        "Kapitel 5 · Mechanische Lüftung und Wärmerückgewinnung",
        [_p5.Beat1_Grundtypen, _p5.Beat2_Waermerueckgewinnung, _p5.Beat3_Anlagenvergleich, _p5.Beat4_Kanalquerschnitt],
    ),
    (
        "Kapitel 6 · Übergabe an Heizung und Kühlung",
        [_p6.Beat1_Zusammenfassung, _p6.Beat2_UebergabeHeizungKuehlung],
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
class _VentilationSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Ventilation_01_WarumLueften(_VentilationSection):
    """1️⃣ Kapitel 1 — Warum lüften?"""

    section_beats = VENTILATION_PLAYLIST[0][1]


class Ventilation_02_Luftmenge(_VentilationSection):
    """2️⃣ Kapitel 2 — Wie viel Luft?"""

    section_beats = VENTILATION_PLAYLIST[1][1]


class Ventilation_03_WaermestromLuft(_VentilationSection):
    """3️⃣ Kapitel 3 — Was die Luft trägt."""

    section_beats = VENTILATION_PLAYLIST[2][1]


class Ventilation_04_NatuerlicheLueftung(_VentilationSection):
    """4️⃣ Kapitel 4 — Natürliche Lüftung."""

    section_beats = VENTILATION_PLAYLIST[3][1]


class Ventilation_05_MechanischeLueftung(_VentilationSection):
    """5️⃣ Kapitel 5 — Mechanische Lüftung und Wärmerückgewinnung."""

    section_beats = VENTILATION_PLAYLIST[4][1]


class Ventilation_06_Uebergabe(_VentilationSection):
    """6️⃣ Kapitel 6 — Übergabe an Heizung und Kühlung."""

    section_beats = VENTILATION_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Ventilation_01_WarumLueften,
    Ventilation_02_Luftmenge,
    Ventilation_03_WaermestromLuft,
    Ventilation_04_NatuerlicheLueftung,
    Ventilation_05_MechanischeLueftung,
    Ventilation_06_Uebergabe,
]
#endregion


#region Full Series Scene (fragile mega-combine)
class FullVentilationVideo(Scene):
    """🌬️ Entire Ventilation series in one Manim scene.

    Prefer running this file as a script (section renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _section, scenes in VENTILATION_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()
#endregion


#region Compose (recommended)
def compose_full_ventilation_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    output_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render the authored intro and chapters through the portable tool interface."""

    return _runtime.compose_video("ventilation", quality_flag=quality_flag, play=play,
                         media_dir=media_dir, output_dir=output_dir,
                         force=force, concat_only=concat_only)


def main(argv: list[str] | None = None) -> int:
    """▶️ Render the complete video with explicit output and cancellation support."""

    return _runtime.video_main("ventilation", argv)


if __name__ == "__main__":
    raise SystemExit(main())
