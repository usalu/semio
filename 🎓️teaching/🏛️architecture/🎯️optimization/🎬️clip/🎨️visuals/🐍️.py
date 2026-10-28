"""🎨 Local animation interface for the six architecture optimization chapters."""

from __future__ import annotations

import importlib
import os
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
_engine = importlib.import_module("manim")

INK = _art.P_WHITE
MUTED = _art.PASTEL_GREY
DESIGN = _art.P_CYAN
DATA = _art.P_YELLOW
GOOD = _art.P_GREEN
WARN = _art.P_ORANGE
Y_TICK_GAP = 0.14


def source_files() -> tuple[Path, ...]:
    """🧾 Include semantic examples, fixtures and drawing inputs in render provenance."""
    root = Path(__file__).resolve().parents[1]
    model = root / "🧮️model"
    return (Path(__file__), Path(_room.__file__), *sorted(model.rglob("*.py")),
            *sorted(model.rglob("*.json")), root / "🧪️cases/🔣️.json", Path(_fonts.__file__),
            Path(_art.__file__), Path(_art.__file__).with_name("🔣️.json"),
            Path(_art.__file__).parent / "🫟️silhouettes/🔣️.json", *_theme.source_files())


def _text(value, point=(0, 0, 0), *, size=24, color=INK, width=6):
    label = _fonts.body_text(str(value), font_size=size, color=color)
    if label.width > width:
        label.scale_to_fit_width(width)
    return label.move_to(point)


class CornerAxes(_engine.Axes):
    """🪜 Axes whose two lines meet in the lower left corner of the ranges, also when a range runs through zero."""

    @staticmethod
    def _origin_shift(axis_range):
        return axis_range[0]


class ClipScene(_room.RoomVisuals, _engine.Scene):
    """🎬 Keep localization, timing, typography and external rendering behind one interface."""

    mn = _engine

    def language(self) -> str:
        """🌐 Require an authored language instead of inheriting a host default."""
        language = os.environ.get("SEMIO_TEACHING_LANGUAGE")
        if language not in {"en", "de"}:
            raise ValueError("Set SEMIO_TEACHING_LANGUAGE to en or de")
        return language

    def tr(self, en: str, de: str) -> str:
        """🗣️ Select visible language using the narration contract."""
        return en if self.language() == "en" else de

    def number(self, value: float, digits=0) -> str:
        """🔢 Group thousands and punctuate decimals per language: en 1,250.5 · de 1.250,5."""
        return _room.number(value, digits, self.language() == "de")

    def quantity(self, value: float, unit: str, digits=0) -> str:
        """📐 A number with its unit, joined by a narrow no-break space."""
        return _room.unit_text(value, unit, digits, self.language() == "de")

    def parts(self, values, digits=0, total=None) -> list:
        """➗ Parts of one total rounded with the largest remainder so that they add up to the printed total (default: the rounded sum of ``values``).

        Print ``self.number(part, digits)`` for every returned part and ``self.number(sum(parts), digits)`` for the total: 34.41 + 424.24 + 139.89 prints as 35 + 424 + 140 = 599, never as 34 + 424 + 140 next to 599.
        See https://en.wikipedia.org/wiki/Largest_remainder_method
        """
        return _room.apportion(values, digits, total)

    def label(self, en, de, point=(0, 0, 0), *, size=24, color=INK, width=6):
        """🏷️ Create readable labels in the canonical bundled typeface."""
        return _text(self.tr(en, de), point, size=size, color=color, width=width)

    def step(self, key: str, *animations, seconds=1.2) -> None:
        """⏱️ Keep every animation attached to a complete, readable narration clause."""
        row = next(row for row in self.NARRATION if row[0] == key)
        caption = row[1] if self.language() == "en" else row[2]
        if self._caption is None:
            self._caption = _art.caption_bar(caption)
            self.play(_engine.FadeIn(self._caption), run_time=.3)
            self._vo_caption_t0 = self.renderer.time
        else:
            self._caption = _art.swap_caption(self, self._caption, caption)
        if animations:
            self.play(*animations, run_time=seconds)
        localized = [(code, en, self.tr(en, de)) for code, en, de in self.NARRATION]
        _art.hold_for(self, localized, key)

    def explain(self, key: str, *animations, seconds=1.2) -> None:
        """📖 Sequence short authored clauses while retaining one semantic diagram beat."""
        self.step(key, *animations, seconds=seconds)
        keys = {row[0] for row in self.NARRATION}
        index = 2
        while f"{key}_{index}" in keys:
            self.step(f"{key}_{index}")
            index += 1

    def clear_stage(self) -> None:
        """🧹 Transition between diagrams while retaining the active narration caption."""
        content = [item for item in self.mobjects if item is not self._caption]
        if content:
            self.play(*[_engine.FadeOut(item) for item in content], run_time=.5)

    def present(self, topic: str) -> None:
        """🧭 Apply the shared style before presenting one independently renderable topic."""
        _art.set_vo_language(self.language())
        _fonts.apply_scene_style(self)
        self._caption = None
        getattr(self, "present_" + topic)()
        self.play(_engine.FadeOut(self._caption), run_time=.3)
        self.wait(.3)

    def plot_axes(self, *, x_range, y_range, x_label=None, y_label=None, point=(-.8, .25, 0), width=7.5, height=3.6,
                  x_digits=0, y_digits=0, x_ticks=None, y_ticks=None, titles=True, corner=False):
        """📐 Axes in physical units with localized titles and numeric ticks, without a TeX dependency.

        ``corner=True`` lets both axis lines meet in the lower left corner even when a range runs through zero (otherwise the lines cross at zero, e.g. a y axis inside a plot
        whose x range starts at −0.3, next to tick numbers at the left edge); ``c2p`` is unaffected.
        Fixed slots: x title below the tick row, y title above the left edge, legend slot (``heat_map(legend=True)``) above the right edge.
        Vertical tick numbers are right-aligned ``Y_TICK_GAP`` (0.14) left of the axes' left edge (tick marks stick out 0.05), so they need about 0.14 + their width (≈ 0.3)
        of free space there; callers need no extra outdent. ``titles=False`` (or empty labels) draws the tick numbers only.
        """
        mn = _engine
        axes = (CornerAxes if corner else mn.Axes)(x_range=list(x_range), y_range=list(y_range), x_length=width, y_length=height, tips=False,
                                                   axis_config={"color": MUTED, "stroke_width": 2, "include_ticks": True}).move_to(point)
        labels = mn.VGroup()
        if titles and x_label:
            labels.add(_text(x_label, (point[0], point[1] - height / 2 - .6, 0), size=19, width=width))
        if titles and y_label:
            y_title = _text(y_label, (0, point[1] + height / 2 + .4, 0), size=19, width=width * .5)
            labels.add(y_title.shift(mn.RIGHT * (point[0] - width / 2 - .55 - y_title.get_left()[0])))
        for (low, high, stride), digits, ticks, horizontal in ((x_range, x_digits, x_ticks, True), (y_range, y_digits, y_ticks, False)):
            for value in ticks if ticks is not None else [low + index * stride for index in range(round((high - low) / stride) + 1)]:
                position = (axes.c2p(value, y_range[0]) + mn.DOWN * .22 if horizontal else
                            axes.c2p(x_range[0], value) + mn.LEFT * Y_TICK_GAP)
                tick = _text(self.number(value, digits), position, size=16, width=1.1)
                if not horizontal:
                    tick.shift(mn.LEFT * tick.width / 2)
                labels.add(tick)
        return axes, labels

    def axes(self, *, x_range=(0, 1, .5), y_range=(0, 1, .5), point=(0, .15, 0), width=7, height=3.3):
        """📏 Plain geometric axes without ticks labels."""
        return _engine.Axes(x_range=list(x_range), y_range=list(y_range), x_length=width, y_length=height,
                            axis_config={"color": MUTED, "include_tip": False, "stroke_width": 2},
                            tips=False).move_to(point)

    def _curve(self, axes, function, low, high, *, color=DESIGN):
        curve = _engine.VMobject(color=color, stroke_width=3)
        curve.set_points_smoothly([axes.c2p(x, function(x)) for x in
                                   (low + (high - low) * i / 160 for i in range(161))])
        return curve

    def live_marker(self, axes, trackers, x_key, value_fn, *, x_label="", x_unit="", x_digits=0, y_label="", y_unit="", y_digits=0,
                    color=INK, drops=True, readout=True, x_scale=1.0, avoid=()):
        """📍 Dot on ``axes`` that follows the trackers, with dashed drop lines and a unit readout chip.

        ``x_key`` is a design key or a function of the design; ``value_fn`` a model key or a function of the design. ``avoid`` (mobjects such as the plotted curve,
        or arrays of screen points) keeps the readout chip off those lines: it takes the position (above, below, or beside the dot) that covers the fewest of their points.
        """
        mn = _engine
        x_of = (lambda design: design[x_key] * x_scale) if isinstance(x_key, str) else x_key
        y_of = (lambda design: _room.result(design)[value_fn]) if isinstance(value_fn, str) else value_fn
        x_low, x_high = axes.x_range[0], axes.x_range[1]
        y_low, y_high = axes.y_range[0], axes.y_range[1]
        lines = [np.array([item.point_from_proportion(k / 240) for k in range(241)]) if isinstance(item, mn.VMobject) else np.asarray(item, dtype=float) for item in avoid]
        crowd = np.concatenate(lines) if lines else np.zeros((0, 3))

        def build():
            design = _room.values(trackers)
            x, y = x_of(design), y_of(design)
            spot = axes.c2p(x, y)
            group = mn.VGroup()
            if drops:
                group.add(_room.dashed(spot, axes.c2p(x, y_low), color=MUTED, stroke_width=1.6),
                          _room.dashed(spot, axes.c2p(x_low, y), color=MUTED, stroke_width=1.6))
            group.add(mn.Circle(radius=.15, color=color, stroke_width=1.8).move_to(spot), mn.Dot(spot, radius=.075, color=color))
            if readout:
                german = self.language() == "de"
                box = _room.chip([*([x_label] if x_label else []), *_room.parts(x, x_unit, x_digits, german), "·", *([y_label] if y_label else []),
                                  *_room.parts(y, y_unit, y_digits, german)], spot, size=15, color=color)
                left, right = axes.c2p(x_low, y_low)[0], axes.c2p(x_high, y_low)[0]
                reach, spread = .45 + box.height / 2, box.width / 2 + .3
                options = [mn.UP * reach, mn.DOWN * reach, mn.UP * reach + mn.RIGHT * spread, mn.UP * reach + mn.LEFT * spread,
                           mn.DOWN * reach + mn.RIGHT * spread, mn.DOWN * reach + mn.LEFT * spread]
                if spot[1] + .5 + box.height / 2 > axes.c2p(x_low, y_high)[1] + .35:
                    options = options[1:2] + options[4:] + options[:1] + options[2:4]

                def placed(offset):
                    centre = spot + offset
                    shift = max(0, left - (centre[0] - box.width / 2)) - max(0, centre[0] + box.width / 2 - right)
                    return centre + mn.RIGHT * shift

                def hits(centre):
                    return int(((np.abs(crowd[:, 0] - centre[0]) < box.width / 2 + .06) & (np.abs(crowd[:, 1] - centre[1]) < box.height / 2 + .06)).sum()) if len(crowd) else 0

                box.move_to(min((placed(offset) for offset in options), key=hits))
                group.add(box)
            return group

        return self._widget(build, _room.design_key(trackers))
