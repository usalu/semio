"""🎬 Full EnergyBalance series — Chapters 1–7 in curriculum order.

Recommended: run this file as a script (chapter renders + ffmpeg concat).
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
_SERIES_ROOT = Path(__file__).resolve().parent
WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))
_runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
prepare_output = _runtime.prepare_output

prepare_output(sys.argv[1:])
_INTRO_SCRIPT = WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
SERIES_INTRO_SCENE = "Demo_Intro_Energiebilanz"
#endregion

begin_vo_beat = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️").begin_vo_beat


#region Module Loader
#endregion


#region Chapter Modules
_c1 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🔁️recap.🐍️")
_c2 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🐍️")
_c3 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚡️demand-power.🐍️")
_c4 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.♨️system-losses.🐍️")
_c5 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🏭️primary-energy.🐍️")
_c6 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🏷️certificate.🐍️")
_c7 = import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🧪️practice.🐍️")
#endregion


#region Playlist
ENERGY_BALANCE_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Kapitel 1 · Die Energiebilanz des Gebäudes",
        [
            _c1.Beat1_Bilanzgrenze,
            _c1.Beat2_Verlustseite,
            _c1.Beat3_Gewinnseite,
            _c1.Beat4_WinterUndSommer,
        ],
    ),
    (
        "Kapitel 2 · Regelwerk & Klimadaten",
        [
            _c2.Beat1_GEGundNormen,
            _c2.Beat2_Monatsbilanz,
            _c2.Beat3_Klimadaten,
            _c2.Beat4_Gradtagszahl,
        ],
    ),
    (
        "Kapitel 3 · Bedarf vs. Leistung",
        [
            _c3.Beat1_LeistungUndEnergie,
            _c3.Beat2_Normheizlast,
            _c3.Beat3_Jahresdauerlinie,
            _c3.Beat4_RichtigAuslegen,
        ],
    ),
    (
        "Kapitel 4 · Von der Nutzenergie zur Endenergie",
        [
            _c4.Beat1_Verlustkette,
            _c4.Beat2_UebergabeVerteilungSpeicherung,
            _c4.Beat3_Erzeugung,
            _c4.Beat4_Anlagenaufwandszahl,
        ],
    ),
    (
        "Kapitel 5 · Primärenergie & das GEG",
        [
            _c5.Beat1_DreiBilanzgrenzen,
            _c5.Beat2_Primaerenergiefaktor,
            _c5.Beat3_Rechenbeispiel,
            _c5.Beat4_Referenzgebaeude,
        ],
    ),
    (
        "Kapitel 6 · Der Energieausweis",
        [
            _c6.Beat1_BedarfVsVerbrauch,
            _c6.Beat2_DieSkala,
            _c6.Beat3_PflichtangabenUndFolgen,
        ],
    ),
    (
        "Kapitel 7 · Theorie vs. Praxis",
        [
            _c7.Beat1_BerechnungVsAbrechnung,
            _c7.Beat2_HydraulischerAbgleich,
            _c7.Beat3_SerienAbschluss,
        ],
    ),
]
#endregion


#region Host Binding
def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Mirror beat class attrs/methods onto the host chapter scene.

    Chapter hosts call ``scene_cls.construct(self)``, so class constants
    and helpers must live on the host. Reset VO spend so shared clause keys
    (``intro`` / ``outro``) do not collapse later beats to ``min_wait``.
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


#region Chapter Scenes
class _EnergyBalanceChapter(Scene):
    """📚 One curriculum chapter — keeps Manim partial counts manageable."""

    chapter_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.chapter_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class EnergyBalance_01_Energiebilanz(_EnergyBalanceChapter):
    """1️⃣ Kapitel 1 — Die Energiebilanz des Gebäudes."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[0][1]


class EnergyBalance_02_RegelwerkKlima(_EnergyBalanceChapter):
    """2️⃣ Kapitel 2 — Regelwerk & Klimadaten."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[1][1]


class EnergyBalance_03_BedarfVsLeistung(_EnergyBalanceChapter):
    """3️⃣ Kapitel 3 — Bedarf vs. Leistung."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[2][1]


class EnergyBalance_04_NutzZuEndenergie(_EnergyBalanceChapter):
    """4️⃣ Kapitel 4 — Von der Nutzenergie zur Endenergie."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[3][1]


class EnergyBalance_05_Primaerenergie(_EnergyBalanceChapter):
    """5️⃣ Kapitel 5 — Primärenergie & das GEG."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[4][1]


class EnergyBalance_06_Energieausweis(_EnergyBalanceChapter):
    """6️⃣ Kapitel 6 — Der Energieausweis."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[5][1]


class EnergyBalance_07_TheorieVsPraxis(_EnergyBalanceChapter):
    """7️⃣ Kapitel 7 — Theorie vs. Praxis."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[6][1]


CHAPTER_SCENES: list[type[Scene]] = [
    EnergyBalance_01_Energiebilanz,
    EnergyBalance_02_RegelwerkKlima,
    EnergyBalance_03_BedarfVsLeistung,
    EnergyBalance_04_NutzZuEndenergie,
    EnergyBalance_05_Primaerenergie,
    EnergyBalance_06_Energieausweis,
    EnergyBalance_07_TheorieVsPraxis,
]
#endregion


#region Full Series Scene (fragile mega-combine)
class FullEnergyBalanceVideo(Scene):
    """📜 Entire EnergyBalance series in one Manim scene.

    Prefer running this file as a script (chapter renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _chapter, scenes in ENERGY_BALANCE_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()
#endregion


#region Compose (recommended)
def compose_full_energy_balance_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    output_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render the authored intro and chapters through the portable tool interface."""

    return _runtime.compose_video("energy_balance", quality_flag=quality_flag, play=play,
                         media_dir=media_dir, output_dir=output_dir,
                         force=force, concat_only=concat_only)


def main(argv: list[str] | None = None) -> int:
    """▶️ Render the complete video with explicit output and cancellation support."""

    return _runtime.video_main("energy_balance", argv)


if __name__ == "__main__":
    raise SystemExit(main())
