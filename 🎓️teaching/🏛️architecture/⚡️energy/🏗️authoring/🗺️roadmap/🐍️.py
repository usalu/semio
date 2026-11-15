"""🗺️ Shared roadmap (Fahrplan) of an energy video: numbered step cards that light up in order, optional hand-over chips.

Every video opens with one roadmap scene that only declares data: ``NARRATION`` (``intro`` plus one clause per step), ``HEADING``,
``STEPS`` (key, name, pictogram, key symbols) and optionally ``BRACKET`` and ``HANDOVERS``. ``RoadmapScene`` draws it.

Step cards    ``{"key", "name": (en, de), "symbols": tex | (en_tex, de_tex), "pictogram", "color", "receives": [chip]}``
Bracket       ``{"last": index, "label": (en, de)}`` spans card 0 to ``index``
Hand-overs    ``[{"text": (en_tex, de_tex), "color"}]`` chips under the last card; ``receives`` chip lists hang under their own card

@see ../🎬️clip/🧬️schema/🔣️.schema.json — the manifest that places the roadmap as the first segment of every video
"""

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import numpy as np
from manim import *

_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")

apply_scene_style = _fonts.apply_scene_style
section_heading = _fonts.section_heading
body_text = _fonts.body_text
BEAT_SUBTITLE_FADE = _fonts.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _fonts.BODY_FONT_SIZE
LABEL_FONT_SIZE = _fonts.LABEL_FONT_SIZE
tr = _visuals.tr
math_label = _visuals.math_label
caption_bar = _visuals.caption_bar
swap_caption = _visuals.swap_caption
hold_for = _visuals.hold_for
subtitle_text = _visuals.subtitle_text
fitted_staffage = _visuals.fitted_staffage

FRAME_SPAN = 13.4
CARD_HEIGHT = 2.7
ROW_Y = 0.6
MAX_CARD_WIDTH = 2.25
COLOR_CYCLE = ("P_CYAN", "P_YELLOW", "P_GREEN", "P_ORANGE", "PASTEL_PURPLE", "P_PRIMARY", "PASTEL_GREY")


def _color(token: str) -> str:
    """🎨 Resolve a visuals colour token such as ``P_GREEN``."""
    return getattr(_visuals, token)


def _fit(mob, width: float, height: float, at):
    """📏 Scale a drawn pictogram uniformly into a ``width`` x ``height`` box and centre it on ``at``; every pictogram is built at its final place, so live parts keep their position."""
    mob.scale(min(width / mob.width, height / mob.height))
    return mob.move_to(at)


def _staffage(cast: str):
    """🧍 Pictogram that is a handcrafted staffage cast fitted into the box."""
    return lambda color, width, height, at: fitted_staffage(cast, width=width, height=height).move_to(at)


def _icon(kind: str):
    """🔣 Pictogram that is one filled diagram icon (sun, flame, factory)."""
    return lambda color, width, height, at: _fit(_visuals.icon_glyph(kind, color=color, width=1.0), width, height, at)


def _air_room(color, width, height, at):
    """🏠 Closed room section with an air stream passing through it."""
    t = 0.07
    section = _visuals.room_section(ORIGIN, w=1.0 - t, h=0.62 - t, wall=t, slab=t, window=(0.30, 0.88), window_side="none")
    body = VGroup(section["ceiling"], *[m for m in section["shell"] if m is not section["floor"] and m is not section["ceiling"]])
    room = _visuals.section_material(VGroup(body, section["floor"]))
    path = _visuals.smooth_path([np.array([-0.62, 0.05, 0.0]), np.array([0.0, 0.12, 0.0]), np.array([0.62, 0.05, 0.0])])
    return _fit(VGroup(room, _visuals.flow_guides([path], color, opacity=0.9, width=2.4)), width, height, at)


def _thermometer(color, width, height, at):
    """🌡️ Bulb thermometer filled to two thirds."""
    tube = min(0.8, height)
    glyph = _visuals.thermometer_glyph(at + DOWN * (tube / 2 + 0.06), height=tube, color=color, level=0.65)
    return VGroup(glyph["group"], glyph["column"])


def _window(color, width, height, at):
    """🪟 Window section with its sash swung open."""
    window = _visuals.window_glyph(-0.15, -0.4, 0.4, depth=0.55, color=color)
    window["sash"].become(_visuals.window_opening(window))
    return _fit(window["group"], width, height, at)


def _house(color, width, height, at):
    """🏠 Two-storey house section in the step colour."""
    return _fit(_visuals.house_section(scale=0.24)["group"].set_color(color), width, height, at)


def _wall(color, width, height, at):
    """🧱 Three wall layers with a heat arrow through them."""
    layers = VGroup(*[Rectangle(width=w, height=0.9, stroke_width=2, color=color, fill_color=fill, fill_opacity=opacity)
                      for w, fill, opacity in ((0.22, color, 0.2), (0.5, color, 0.55), (0.22, color, 0.2))]).arrange(RIGHT, buff=0)
    arrow = Arrow(np.array([-0.8, 0.0, 0.0]), np.array([0.8, 0.0, 0.0]), buff=0, color=_visuals.P_WHITE, stroke_width=4, max_tip_length_to_length_ratio=0.3)
    return _fit(VGroup(layers, arrow), width, height, at)


def _bars(color, width, height, at):
    """📊 Power-time axes with the energy area between them."""
    area = Rectangle(width=0.8, height=0.5, color=color, fill_color=color, fill_opacity=0.35, stroke_width=2).move_to(np.array([0.08, 0.05, 0.0]))
    axes = VGroup(Line(np.array([-0.4, -0.2, 0.0]), np.array([-0.4, 0.42, 0.0]), color=_visuals.P_WHITE, stroke_width=2.4),
                  Line(np.array([-0.4, -0.2, 0.0]), np.array([0.58, -0.2, 0.0]), color=_visuals.P_WHITE, stroke_width=2.4))
    return _fit(VGroup(area, axes), width, height, at)


def _balance(color, width, height, at):
    """⚖️ Beam balance with two pans."""
    parts = [Line(np.array([0.0, -0.45, 0.0]), np.array([0.0, 0.4, 0.0])), Line(np.array([-0.25, -0.45, 0.0]), np.array([0.25, -0.45, 0.0])),
             Line(np.array([-0.6, 0.4, 0.0]), np.array([0.6, 0.4, 0.0]))]
    for x in (-0.6, 0.6):
        parts += [Line(np.array([x, 0.4, 0.0]), np.array([x - 0.22, 0.02, 0.0])), Line(np.array([x, 0.4, 0.0]), np.array([x + 0.22, 0.02, 0.0])),
                  Line(np.array([x - 0.27, 0.02, 0.0]), np.array([x + 0.27, 0.02, 0.0]))]
    return _fit(VGroup(*parts).set_stroke(color=color, width=3), width, height, at)


def _label(color, width, height, at):
    """🏷️ Efficiency-class stripes growing from green to red."""
    palette = (_visuals.P_GREEN, _visuals.P_GREEN, _visuals.P_YELLOW, _visuals.P_YELLOW, _visuals.P_ORANGE, _visuals.P_ORANGE, _visuals.P_RED)
    stripes = VGroup(*[Rectangle(width=0.35 + 0.1 * i, height=0.1, stroke_width=0, fill_color=c, fill_opacity=0.9) for i, c in enumerate(palette)])
    stripes.arrange(DOWN, buff=0.04, aligned_edge=LEFT)
    return _fit(stripes, width, height, at)


def _droplets(color, width, height, at):
    """💧 Scattered droplets."""
    return _fit(_visuals.droplets(ORIGIN, n=7, spread=(0.45, 0.4), color=color), width, height, at)


PICTOGRAMS = {
    "air-room": _air_room, "thermometer": _thermometer, "window": _window, "house": _house, "wall": _wall, "bars": _bars,
    "balance": _balance, "label": _label, "droplets": _droplets,
    "sun": _icon("sun"), "flame": _icon("flame"), "factory": _icon("factory"),
    "mvhr": _staffage("shared-mvhr-normal"), "person": _staffage("shared-person-neutral"), "heat-pump": _staffage("shared-heat-pump-heating-sweep"),
    "office-worker": _staffage("heating-internal-gains-office-worker"), "radiator": _staffage("shared-radiator-winter"),
    "kettle": _staffage("physics-fundamentals-kettle"), "pot": _staffage("physics-fundamentals-pot"),
    "meter": _staffage("shared-gas-meter-run"), "blind": _staffage("cooling-summer-strategy-blind"),
    "seated-worker": _staffage("cooling-internal-gains-seated-worker"), "diffuser": _staffage("cooling-system-sizing-diffuser"),
    "exchanger": _staffage("cooling-summer-strategy-exchanger"), "valve": _staffage("demand-practice-valve"),
    "tree": _staffage("demand-primary-energy-tree"), "clerk": _staffage("physics-fundamentals-clerk"),
}


def _chip(text: str, color: str):
    """🔖 Typeset hand-over chip naming the quantity that moves between two videos."""
    label = math_label(text, size=BODY_FONT_SIZE, color=color)
    return VGroup(SurroundingRectangle(label, color=color, corner_radius=0, buff=_theme.UI_SPACING, stroke_width=_theme.CHROME_BORDER), label)


def _pair(value):
    """🌐 Pick the language of an ``(en, de)`` pair or pass a plain string through."""
    return tr(*value) if isinstance(value, (tuple, list)) else value


def layout(count: int) -> tuple[float, float, list[float]]:
    """📐 Card width, gap and centre x of every card for ``count`` cards centred on the frame."""
    gap = 0.4 if count <= 5 else 0.3
    card_w = min(MAX_CARD_WIDTH, (FRAME_SPAN - (count - 1) * gap) / count)
    return card_w, gap, [(i - (count - 1) / 2) * (card_w + gap) for i in range(count)]


def validate_roadmap(narration, steps, bracket=None, handovers=()) -> None:
    """🛂 Check that the declared steps, clauses and chips fit together before anything is drawn."""
    keys = [row[0] for row in narration]
    if keys != ["intro", *[step["key"] for step in steps]]:
        raise ValueError(f"Roadmap narration keys {keys} must be intro plus the step keys {[s['key'] for s in steps]}")
    if not 2 <= len(steps) <= 7:
        raise ValueError("A roadmap holds two to seven steps")
    for step in steps:
        if step["pictogram"] not in PICTOGRAMS:
            raise ValueError(f"Unknown roadmap pictogram {step['pictogram']}")
    if bracket is not None and not 0 < bracket["last"] < len(steps):
        raise ValueError("The bracket must end on a later card")
    if handovers and any("text" not in chip for chip in handovers):
        raise ValueError("Every hand-over needs a text")


def build_roadmap(scene, narration, steps, *, heading, bracket=None, handovers=()):
    """🗺️ Draw the roadmap: heading, numbered cards, then each card lights up on its own clause; hand-over chips follow the last card."""
    validate_roadmap(narration, steps, bracket, handovers)
    N = narration
    count = len(steps)
    card_w, gap, xs = layout(count)
    compact = card_w < 2.1
    name_size = LABEL_FONT_SIZE if compact else BODY_FONT_SIZE
    widest = max(body_text(_pair(step["name"]), font_size=name_size, color=_visuals.P_WHITE).width for step in steps)
    name_size = name_size * min(1.0, (card_w - 0.2) / widest)
    colors = [_color(step.get("color", COLOR_CYCLE[i % len(COLOR_CYCLE)])) for i, step in enumerate(steps)]

    subtitle = section_heading(tr(*heading))
    scene.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
    caption = caption_bar(subtitle_text(N, "intro"))
    scene.play(FadeIn(caption), run_time=0.3)

    cards, shades, receives = [], [], []
    for i, (x, color, step) in enumerate(zip(xs, colors, steps)):
        frame = Rectangle(width=card_w, height=CARD_HEIGHT, color=color, stroke_width=_theme.CHROME_BORDER).move_to(np.array([x, ROW_Y, 0.0]))
        badge = VGroup(Circle(radius=0.19, color=color, stroke_width=2, fill_color=color, fill_opacity=1.0),
                       body_text(str(i + 1), font_size=LABEL_FONT_SIZE, color=_visuals.P_DEEP_DARK))
        badge[1].move_to(badge[0].get_center())
        badge.move_to(np.array([x, ROW_Y + CARD_HEIGHT / 2 - 0.4, 0.0]))
        name = body_text(_pair(step["name"]), font_size=name_size, color=color)
        name.move_to(np.array([x, ROW_Y + CARD_HEIGHT / 2 - 0.88, 0.0]))
        pic = PICTOGRAMS[step["pictogram"]](color, card_w - 0.6, 0.9, np.array([x, ROW_Y - 0.12, 0.0]))
        sym = math_label(_pair(step["symbols"]), size=name_size, color=_visuals.P_WHITE)
        if sym.width > card_w - 0.2:
            sym.scale((card_w - 0.2) / sym.width)
        sym.move_to(np.array([x, ROW_Y - CARD_HEIGHT / 2 + 0.3, 0.0]), coor_mask=np.array([1, 0, 0]))
        sym.shift(UP * (ROW_Y - CARD_HEIGHT / 2 + 0.3 - sym.base.get_center()[1]))
        cards.append(VGroup(frame, badge, name, pic, sym))
        shades.append(Rectangle(width=card_w + 0.04, height=CARD_HEIGHT + 0.04, stroke_width=0, fill_color=_theme.CHROME_FILL, fill_opacity=0.74).move_to(frame))
    links = [Arrow(np.array([x + card_w / 2 + 0.03, ROW_Y, 0.0]), np.array([x + card_w / 2 + gap - 0.03, ROW_Y, 0.0]), buff=0,
                   color=_visuals.P_WHITE, stroke_width=3, max_tip_length_to_length_ratio=0.5) for x in xs[:-1]]

    span_y = ROW_Y - CARD_HEIGHT / 2 - 0.25
    chip_top = span_y - 0.18
    for i, step in enumerate(steps):
        stack = VGroup(*[_chip(_pair(chip["text"]), _color(chip["color"])) for chip in step.get("receives", [])])
        if len(stack):
            stack.arrange(DOWN, buff=0.1, aligned_edge=LEFT)
            stack.move_to(np.array([xs[i] - card_w / 2 + stack.width / 2, chip_top - stack.height / 2, 0.0]))
            receives.append((i, stack))
    bracket_group = None
    if bracket is not None:
        left, right = xs[0] - card_w / 2, xs[bracket["last"]] + card_w / 2
        ticks = [Line(np.array([x, span_y, 0.0]), np.array([x, span_y + 0.12, 0.0]), color=_visuals.P_PRIMARY, stroke_width=2) for x in (left, right)]
        tag = body_text(_pair(bracket["label"]), font_size=LABEL_FONT_SIZE, color=_visuals.P_PRIMARY).move_to(np.array([0.5 * (left + right), span_y - 0.32, 0.0]))
        bracket_group = (VGroup(Line(np.array([left, span_y, 0.0]), np.array([right, span_y, 0.0]), color=_visuals.P_PRIMARY, stroke_width=2), *ticks), tag)
    chips = VGroup(*[_chip(_pair(h["text"]), _color(h["color"])) for h in handovers])
    if len(chips):
        chips.arrange(DOWN, buff=0.1, aligned_edge=RIGHT)
        chips.move_to(np.array([xs[-1] + card_w / 2 - chips.width / 2, chip_top - chips.height / 2, 0.0]))
        for chip in chips:
            chip.set_x(chips.get_x())

    scene.play(LaggedStart(*[FadeIn(c) for c in cards], lag_ratio=0.12), *[FadeIn(s) for s in shades], run_time=1.6)
    hold_for(scene, N, "intro", used=0.3 + 1.6)

    for i, step in enumerate(steps):
        key = step["key"]
        caption = swap_caption(scene, caption, subtitle_text(N, key))
        anims = [FadeOut(shades[i])]
        if i > 0:
            anims.append(GrowArrow(links[i - 1]))
        scene.play(*anims, run_time=1.0)
        scene.play(Indicate(cards[i][0], color=colors[i], scale_factor=1.05), *[FadeIn(made, shift=UP * 0.15) for j, made in receives if j == i], run_time=1.0)
        last = i == count - 1
        if last and len(chips):
            scene.play(*[FadeIn(chip, shift=UP * 0.15) for chip in chips], run_time=1.0)
        hold_for(scene, N, key, min_wait=1.0 if last else 0.3)
        if bracket_group is not None and i == bracket["last"]:
            scene.play(Create(bracket_group[0]), FadeIn(bracket_group[1]), run_time=1.0)
    scene.play(FadeOut(caption), run_time=0.3)
    scene.wait(0.5)


class RoadmapScene(Scene):
    """🗺️ Scene of one video's roadmap: subclass it and declare ``NARRATION``, ``HEADING``, ``STEPS`` and optionally ``BRACKET`` and ``HANDOVERS``."""

    NARRATION: list = []
    HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")
    STEPS: list = []
    BRACKET = None
    HANDOVERS: list = []

    def construct(self):
        apply_scene_style(self)
        build_roadmap(self, self.NARRATION, self.STEPS, heading=self.HEADING, bracket=self.BRACKET, handovers=self.HANDOVERS)
