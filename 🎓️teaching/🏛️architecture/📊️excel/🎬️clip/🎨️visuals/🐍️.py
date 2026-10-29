"""🎨 Excel visual language of the Nakagin clips: the clip scene base over the BIM drafting widgets, the Nakagin drawings and the workbook widgets."""

from __future__ import annotations

import importlib
import os
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_workbook = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.📗️workbook.🐍️")
_engine = importlib.import_module("manim")

_NAKAGIN = "🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️"


def _nakagin_drawing() -> type:
    """🏢 The Nakagin drawing mixin; an empty mixin until ``🏢️nakagin/🎨️visuals`` exists."""
    try:
        return importlib.import_module(_NAKAGIN).NakaginDrawing
    except ModuleNotFoundError as error:
        if error.name is None or not _NAKAGIN.startswith(error.name):
            raise
        return type("NakaginDrawing", (), {})


INK, MUTED, ARCH, STRUCT, SERV = _drafting.INK, _drafting.MUTED, _drafting.ARCH, _drafting.STRUCT, _drafting.SERV
DATA, GOOD, WARN, GLASS = _drafting.DATA, _drafting.GOOD, _drafting.WARN, _drafting.GLASS
CONCRETE, GREEN_GROUND = _drafting.CONCRETE, _drafting.GREEN_GROUND
MINUS = "−"


def source_files() -> tuple[Path, ...]:
    """🧾 Expose widgets, Excel model and data, Nakagin model and drawings, palette and typography as render provenance inputs."""
    clip = Path(__file__).resolve().parents[1]
    nakagin = clip.parents[1] / "🏢️nakagin"
    widgets = sorted(Path(__file__).parent.rglob("🐍️.py"))
    own = (Path(__file__), *widgets, *Path(__file__).parent.rglob("🔣️.json"), clip / "🧱️model/🐍️.py", clip / "🧱️model/🔣️.json", clip / "🧱️model/🧬️schema/🔣️.schema.json",
           nakagin / "🐍️.py", nakagin / "🔣️.json", nakagin / "🧬️schema/🔣️.schema.json", nakagin / "🎨️visuals/🐍️.py")
    return tuple(dict.fromkeys(path for path in (*own, *_theme.source_files()) if path.is_file()))


class ClipScene(_drafting.Drafting, _nakagin_drawing(), _workbook.Workbook, _engine.Scene):
    """🎬 Present localized Excel concepts: localization, timing and every drawing widget behind one scene base."""

    def language(self) -> str:
        """🌐 Require an explicit language; never infer a default from the host."""
        language = os.environ.get("SEMIO_TEACHING_LANGUAGE")
        if language not in {"en", "de"}:
            raise ValueError("Set SEMIO_TEACHING_LANGUAGE to en or de")
        return language

    def tr(self, en: str, de: str) -> str:
        """🗣️ Select authored labels using the same explicit language as narration."""
        return en if self.language() == "en" else de

    def number(self, value: float, digits=2) -> str:
        """🔢 Format numbers in the selected language (en 1,250.5 · de 1.250,5) without a host-locale dependency."""
        rendered = f"{abs(value):,.{digits}f}"
        if self.language() == "de":
            rendered = rendered.translate({ord(","): ".", ord("."): ","})
        return (MINUS if round(value, digits) < 0 else "") + rendered

    def step(self, key: str, *animations, seconds=1.2) -> None:
        """⏱️ Bind animation, readable subtitles and source narration to one measured clause."""
        row = next(row for row in self.NARRATION if row[0] == key)
        caption = row[1] if self.language() == "en" else row[2]
        if self._caption is None:
            self._caption = _art.caption_bar(caption)
            self.play(_engine.FadeIn(self._caption), run_time=0.3)
            self._vo_caption_t0 = self.renderer.time
        else:
            self._caption = _art.swap_caption(self, self._caption, caption)
        if animations:
            self.play(*animations, run_time=seconds)
        localized = [(code, en, en if self.language() == "en" else de) for code, en, de in self.NARRATION]
        _art.hold_for(self, localized, key)

    def present(self, topic: str) -> None:
        """🧭 Initialize the canonical teaching style and render one chapter through the scene's own ``present_<topic>``."""
        _art.set_vo_language(self.language())
        _fonts.apply_scene_style(self)
        self._caption = None
        getattr(self, "present_" + topic)()
        self.play(_engine.FadeOut(self._caption), run_time=0.3)
        self.wait(0.3)
