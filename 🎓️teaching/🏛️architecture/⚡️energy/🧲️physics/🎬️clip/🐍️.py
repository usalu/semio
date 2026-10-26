"""🎬 Full Physical Fundamentals video — all eight beats in curriculum order.

Recommended: run this file as a script (beat renders + ffmpeg concat).
"""

from __future__ import annotations

from importlib import import_module
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from manim import Scene

_PF_ROOT = Path(__file__).resolve().parent
WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(WORKSPACE_ROOT))
_runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
prepare_output = _runtime.prepare_output

prepare_output(sys.argv[1:])
# region Intro
_INTRO_SCENE = WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
INTRO_CLASS_NAME = "Demo_Intro_PhysikalischeGrundlagen"
# endregion



_m1 = import_module("🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🐍️")

PHYSICAL_FUNDAMENTALS_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    ("Physikalische Grundlagen", list(_m1.BEATS)),
]

_BIND_ATTRS = ("NARRATION", "topic_de", "topic_explain_de", "series_de", "hold_seconds")


def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    for name in _BIND_ATTRS:
        if hasattr(scene_cls, name):
            setattr(host, name, getattr(scene_cls, name))
    _m1.begin_vo_beat(host, scene_cls.__name__)


class PhysicalFundamentals_FullSection(Scene):
    section_beats = PHYSICAL_FUNDAMENTALS_PLAYLIST[0][1]

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class FullPhysicalFundamentalsVideo(Scene):
    def construct(self):
        for _section, scenes in PHYSICAL_FUNDAMENTALS_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()


def compose_full_physical_fundamentals_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    output_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render the authored intro and chapters through the portable tool interface."""

    return _runtime.compose_video("physical_fundamentals", quality_flag=quality_flag, play=play,
                         media_dir=media_dir, output_dir=output_dir,
                         force=force, concat_only=concat_only)


def main(argv: list[str] | None = None) -> int:
    """▶️ Render the complete video with explicit output and cancellation support."""

    return _runtime.video_main("physical_fundamentals", argv)


if __name__ == "__main__":
    raise SystemExit(main())
