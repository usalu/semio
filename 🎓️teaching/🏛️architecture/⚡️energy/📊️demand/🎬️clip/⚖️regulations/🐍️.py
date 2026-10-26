

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))
import numpy as np
from manim import *

from pathlib import Path as _Path
import sys as _sys


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
scene_title = _manim_module.scene_title
play_scene_title = _manim_module.play_scene_title
TITLE_RUN_TIME = _manim_module.TITLE_RUN_TIME
beat_subtitle = _manim_module.beat_subtitle
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
body_text = _manim_module.body_text
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
chip = _manim_module.chip
note_line = _manim_module.note_line
labeled_axes = _manim_module.labeled_axes
stacked_bar = _manim_module.stacked_bar
meter = _manim_module.meter
bind_meter = _manim_module.bind_meter
cross_mark = _manim_module.cross_mark
dim_arrow = _manim_module.dim_arrow
smooth_path = _manim_module.smooth_path
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
de_num = _manim_module.de_num
highlight_param = _manim_module.highlight_param
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing
house_section = _manim_module.house_section
window_glyph = _manim_module.window_glyph
radiator = _manim_module.radiator
sun_glyph = _manim_module.sun_glyph
sun_rays = _manim_module.sun_rays
shine = _manim_module.shine
ripples = _manim_module.ripples
person_glyph = _manim_module.person_glyph
lamp_glyph = _manim_module.lamp_glyph
thermometer_glyph = _manim_module.thermometer_glyph
clock_glyph = _manim_module.clock_glyph
cloud_glyph = _manim_module.cloud_glyph
card_grid = _manim_module.card_grid

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "⏱️timing" / "🔣️.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Regelwerk & Klimadaten"

LOSS_BLUE = "#38BDF8"
SOLAR_YELLOW = "#FDE047"
ETA_GREEN = "#22C55E"
DEMAND_RED = "#EF4444"

MONTHS = ("J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D")


#region Climate model
# One schematic year drives every beat of this chapter, so the same weather that
# fills the monthly balance in Beat 2 is what Beat 3 draws and Beat 4 integrates.
# Monthly mean outdoor temperature and relative global irradiation, DWD
# test-reference-year shape for a central-German site — the shape, not a
# certified data row.
THETA_E = (0.5, 1.4, 4.9, 9.4, 13.6, 16.8, 18.4, 18.0, 14.2, 9.6, 4.8, 1.6)
IRRADIATION = (0.10, 0.18, 0.33, 0.50, 0.66, 0.70, 0.68, 0.60, 0.42, 0.25, 0.12, 0.08)
IRRADIATION_PEAK = 0.70
IRRADIATION_FULL_KWH = 220.0

THETA_I = 20.0
THETA_HG = 15.0
THETA_LO, THETA_HI = -4.0, 22.0
INTERNAL_GAIN = 0.07
DAYS_PER_MONTH = 365.0 / 12.0

LOSS_SHAPE = tuple((THETA_I - t) / (THETA_I - THETA_E[0]) for t in THETA_E)
GAIN_SHAPE = tuple(INTERNAL_GAIN + s for s in IRRADIATION)


def _eta_h(gain: float, loss: float, a: float = 2.0) -> float:
    """🔋 Monthly utilisation factor η_h = (1 − γᵃ) / (1 − γᵃ⁺¹) with γ = Q_Quelle / Q_Senke (DIN V 18599-2)."""
    gamma = gain / loss
    if abs(gamma - 1.0) < 1e-6:
        return a / (a + 1.0)
    return (1.0 - gamma ** a) / (1.0 - gamma ** (a + 1.0))


ETA_SHAPE = tuple(_eta_h(g, l) for g, l in zip(GAIN_SHAPE, LOSS_SHAPE))
USABLE_SHAPE = tuple(e * g for e, g in zip(ETA_SHAPE, GAIN_SHAPE))
REST_SHAPE = tuple(max(0.0, l - u) for l, u in zip(LOSS_SHAPE, USABLE_SHAPE))
UNUSED_SHAPE = tuple(g - u for g, u in zip(GAIN_SHAPE, USABLE_SHAPE))
REST_MONTHS = tuple(i for i, r in enumerate(REST_SHAPE) if r > 0.004)


def _catmull(series, m: float) -> float:
    """〰️ Smooth periodic value of a monthly series at month position ``m`` (month ``i`` is centred on ``i + 0.5``)."""
    n = len(series)
    u = (m - 0.5) % n
    i = int(np.floor(u))
    f = u - i
    p0, p1, p2, p3 = (series[(i + k) % n] for k in (-1, 0, 1, 2))
    return float(0.5 * (2 * p1 + (p2 - p0) * f + (2 * p0 - 5 * p1 + 4 * p2 - p3) * f ** 2
                        + (3 * p1 - p0 - 3 * p2 + p3) * f ** 3))


def _theta_norm(theta: float) -> float:
    return (theta - THETA_LO) / (THETA_HI - THETA_LO)


def _degree_days(reach: float, shift: float = 0.0) -> float:
    """🌡️ Gradtagszahl G_t in Kd/a: the area between the heating limit and the curve, swept up to ``reach`` months."""
    xs = np.linspace(0.0, reach, max(3, int(reach * 40) + 1))
    deficit = np.array([max(0.0, THETA_HG - (_catmull(THETA_E, x) + shift)) for x in xs])
    return float(np.sum(0.5 * (deficit[1:] + deficit[:-1]) * np.diff(xs)) * DAYS_PER_MONTH)
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

CHART_ORIGIN = (-1.70, -1.28, 0.0)
CHART_W, CHART_H = 7.00, 2.80
HOUSE_C = np.array([-4.10, 0.05, 0.0])
HOUSE_S = 0.78
SUN_C = np.array([-6.45, 1.50, 0.0])
THERMO_X = -6.75
CLOCK_C = np.array([-2.70, 1.74, 0.0])
CLOCK_R = 0.30
LEGEND_Y = 1.82
SUB_W = 0.27
SUB_DX = 0.145
CENTER_DX = -1.8


#region Climate chart
def _month_ticks(axes, *, color=P_WHITE, y_offset=-0.24):
    """🗓️ Twelve single-letter month labels under an axis, evenly spaced."""
    labels = VGroup()
    for i, name in enumerate(MONTHS):
        label = body_text(name, font_size=LABEL_FONT_SIZE - 3, color=color)
        label.set_opacity(0.8)
        label.move_to(axes["pt"]((i + 0.5) / 12, 0) + UP * y_offset)
        labels.add(label)
    return labels


def _chart(y_latex: str, *, ticks=(), color=P_WHITE, dx: float = 0.0):
    """📐 Month axes with a typeset unit label, optional tick numerals and faint level lines.

    ``ticks`` are ``(axis fraction, numeral)``; ``dx`` places the chart left of its house position, and
    ``chart["pt"]`` follows every later ``_slide`` so new marks always land on the moved axes.
    """
    axes = labeled_axes((CHART_ORIGIN[0] + dx, CHART_ORIGIN[1], 0.0), x_len=CHART_W, y_len=CHART_H, color=P_WHITE)
    months = _month_ticks(axes)
    numerals = VGroup()
    levels = VGroup()
    for frac, text in ticks:
        n = math_label(text, size=LABEL_FONT_SIZE, color=color)
        n.next_to(axes["pt"](0.0, frac), LEFT, buff=0.12)
        numerals.add(n)
        levels.add(DashedLine(axes["pt"](0.0, frac), axes["pt"](1.0, frac), color=P_WHITE, stroke_width=1.2,
                              stroke_opacity=0.16, dash_length=0.12))
    unit = math_label(y_latex, size=LABEL_FONT_SIZE, color=color).rotate(PI / 2)
    unit.next_to(numerals if len(numerals) else axes["y_axis"], LEFT, buff=0.12)
    unit.set_y(CHART_ORIGIN[1] + 0.15 + unit.height / 2)
    group = VGroup(axes["group"], levels, numerals, unit)
    chart = {"axes": axes, "months": months, "group": group, "levels": levels, "numerals": numerals,
             "unit": unit, "offset": np.zeros(3)}
    chart["pt"] = lambda fx, fy: axes["pt"](fx, fy) + chart["offset"]
    return chart


def _slide(scene, chart, mobs, dx: float):
    """↔️ Slide the chart and its finished marks sideways; marks not on screen yet are moved unseen, and marks built afterwards follow the new axes."""
    chart["offset"] = chart["offset"] + RIGHT * dx
    present = set(map(id, scene.get_mobject_family_members()))
    for m in mobs:
        if id(m) not in present:
            m.shift(RIGHT * dx)
    return [m.animate.shift(RIGHT * dx) for m in mobs if id(m) in present]


def _right_axis(chart, y_latex: str, ticks, color):
    """➡️ Second value axis on the right edge of the chart, in its own colour."""
    pt = chart["pt"]
    line = Line(pt(1.0, 0.0), pt(1.0, 1.0), color=color, stroke_width=2.0, stroke_opacity=0.7)
    numerals = VGroup(*[math_label(text, size=LABEL_FONT_SIZE, color=color).next_to(pt(1.0, frac), RIGHT, buff=0.12)
                        for frac, text in ticks])
    unit = math_label(y_latex, size=LABEL_FONT_SIZE, color=color).rotate(-PI / 2)
    unit.next_to(numerals, RIGHT, buff=0.12)
    unit.set_y(CHART_ORIGIN[1] + 0.15 + unit.height / 2)
    return VGroup(line, numerals, unit)


def _cell(chart, i: int, side: int, lo: float, hi: float, color, *, opacity=0.85, fill=True, stroke=1.0):
    """▮ One month column between two axis fractions; ``side`` −1 / +1 picks the sink / source half, 0 the full slot."""
    width = SUB_W if side else 0.42
    base = chart["pt"]((i + 0.5) / 12, lo)
    height = max(0.01, (hi - lo) * CHART_H)
    rect = Rectangle(width=width, height=height, color=color, fill_color=color,
                     fill_opacity=opacity if fill else 0.10, stroke_width=stroke)
    return rect.move_to(base + RIGHT * side * SUB_DX + UP * height / 2)


def _layer(chart, month, base: float, cells, *, side: int, color, opacity=0.85, fill=True, stroke=1.0):
    """📊 Month columns that rise as the year cursor passes them — a layer of ``(lo, hi)`` cells, one per month or ``None``."""
    def build():
        reach = month.get_value() - base
        group = VGroup()
        for i, cell in enumerate(cells):
            f = float(np.clip(reach - i, 0.0, 1.0))
            if cell is None or f <= 0.0:
                continue
            lo, hi = cell
            group.add(_cell(chart, i, side, lo, lo + (hi - lo) * f, color, opacity=opacity, fill=fill, stroke=stroke))
        return group

    return always_redraw(build)


def _trace(chart, month, base: float, fn, color, *, width=3.0, dots=False):
    """✏️ Curve ``fn(month position) → axis fraction`` drawn up to the year cursor, with a dot per month once passed."""
    def build():
        reach = float(np.clip(month.get_value() - base, 0.0, 12.0))
        group = VGroup()
        if reach > 0.02:
            xs = np.linspace(0.0, reach, max(3, int(reach * 10) + 1))
            line = VMobject(color=color, stroke_width=width)
            line.set_points_as_corners([chart["pt"](x / 12, fn(x)) for x in xs])
            group.add(line)
        if dots:
            for i in range(12):
                if reach >= i + 0.5:
                    group.add(Dot(chart["pt"]((i + 0.5) / 12, fn(i + 0.5)), radius=0.055, color=color))
        return group

    return always_redraw(build)


def _curve(chart, fn, color, *, width=3.0, opacity=1.0):
    """〰️ Finished smooth curve ``fn(month position) → axis fraction`` over the whole year."""
    xs = np.linspace(0.0, 12.0, 121)
    line = VMobject(color=color, stroke_width=width, stroke_opacity=opacity)
    line.set_points_as_corners([chart["pt"](x / 12, fn(x)) for x in xs])
    return line


def _cursor(chart, month, base: float):
    """📍 Dashed year cursor on the chart, following the month tracker."""
    def build():
        x = float(np.clip(month.get_value() - base, 0.0, 12.0)) / 12
        return DashedLine(chart["pt"](x, 0.0), chart["pt"](x, 1.0), color=P_WHITE, stroke_width=1.6,
                          stroke_opacity=0.5, dash_length=0.1)

    return always_redraw(build)


def _legend(entries, dx: float = 0.0):
    """🗝️ Colour swatch plus typeset symbol per entry, laid out on one row above the chart; shown entry by entry."""
    items = {}
    for key, color, latex, filled in entries:
        swatch = Square(side_length=0.17, color=color, stroke_width=2.0, fill_color=color,
                        fill_opacity=0.85 if filled else 0.10)
        label = body_text(latex[1:], font_size=LABEL_FONT_SIZE, color=color) if latex.startswith("=") \
            else math_label(latex, size=LABEL_FONT_SIZE, color=color)
        items[key] = VGroup(swatch, label.next_to(swatch, RIGHT, buff=0.10))
    row = VGroup(*items.values()).arrange(RIGHT, buff=0.32, center=False)
    row.move_to(np.array([CHART_ORIGIN[0] + dx, LEGEND_Y, 0.0]), aligned_edge=LEFT)
    return items


def _gauge(symbol: str, *, color, length=1.9, thickness=0.32):
    """📏 0–1 gauge driven by a tracker, with a typeset symbol instead of plain text."""
    g = meter("x", length=length, thickness=thickness, color=color, vertical=True)
    g["group"].remove(g["label"])
    tracker = ValueTracker(0.0)
    bind_meter(g, tracker)
    tag = math_label(symbol, size=BODY_FONT_SIZE, color=color)
    g["tracker"] = tracker
    g["tag"] = tag
    g["group"] = VGroup(g["track"], g["fill"], tag)
    return g
#endregion


#region House stage
def _stand(floor_y: float, x: float, color=P_ORANGE):
    """🧍 Occupant at true scale — 1.75 m in a 2.5 m storey is 70 % of the clear height, feet on the slab's top edge."""
    height = 0.7 * HOUSE_S * 1.2
    return person_glyph(np.array([x, floor_y + 0.02 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _level_flow(paths, color, level_fn, *, waves=7, speed=0.5, radius=0.06):
    """💧 Particle stream whose density follows ``level_fn() ∈ [0, 1]`` — more flow, more particles."""
    dots = VGroup(*[Dot(radius=radius, color=color, stroke_width=0).set_fill(color, opacity=0.0)
                    for _ in range(waves * len(paths))])
    clock = {"t": 0.0}

    def update(group, dt):
        clock["t"] += dt
        level = float(np.clip(level_fn(), 0.0, 1.0))
        for k, dot in enumerate(group):
            p, w = divmod(k, waves)
            t = (clock["t"] * speed + w / waves + 0.31 * p) % 1.0
            dot.move_to(paths[p].point_from_proportion(t))
            on = float(np.clip(level * waves - w, 0.0, 1.0))
            dot.set_fill(color, opacity=max(0.0, min(1.0, t / 0.08, (1.0 - t) / 0.12) * on * 0.9))

    dots.add_updater(update)
    return dots


def _north_window(hs):
    """🧭 Second glazed opening in the right wall of the section — the north façade."""
    br, tr = hs["bottom_right"], hs["top_right"]
    up = hs["windows"][1]
    sill, lintel = up["sill"], up["lintel"]
    wall, inner = hs["wall_right"], hs["walls"][5][3]
    t = 0.08 * HOUSE_S
    lower = Line(br, np.array([br[0], sill, 0.0]), color=P_WHITE, stroke_width=wall.get_stroke_width())
    lower_in = Line(br + LEFT * t, np.array([br[0] - t, sill, 0.0]), color=P_WHITE,
                    stroke_width=inner.get_stroke_width())
    wall.put_start_and_end_on(np.array([br[0], lintel, 0.0]), tr)
    inner.put_start_and_end_on(np.array([br[0] - t, lintel, 0.0]), tr + LEFT * t)
    glass = window_glyph(float(br[0]), sill, lintel, depth=0.42 * HOUSE_S)
    hs["walls"].add(lower, lower_in)
    hs["window_group"].add(glass["group"])
    return glass


def _stage(*, north=False, with_radiator=False):
    """🏠 The Physical Fundamentals house with its weather: sun and straight rays, outdoor thermometer, loss streams,
    occupant, lamp and — for the heating limit — a radiator.

    Every effect follows one of the ``lvl`` trackers and starts switched off (``k`` scales the loss streams, ``sk``
    the sun), so a beat brings in exactly the effect the current clause explains.
    """
    hs = house_section(HOUSE_C, HOUSE_S)
    bl, tl = hs["bottom_left"], hs["top_left"]
    lo, up = hs["windows"]
    floor_y, level_y = float(bl[1]), float(hs["level_1"].get_center()[1])
    x0 = float(bl[0])
    glass_n = _north_window(hs) if north else None

    person = _stand(floor_y, x0 + 1.75)
    lamp = lamp_glyph(np.array([x0 + 1.55, tl[1] - 0.05, 0.0]), drop=0.12)
    sun = sun_glyph(SUN_C).scale(0.42)
    therm = thermometer_glyph(np.array([THERMO_X, -0.80, 0.0]), height=1.30, color=P_BLUE, level=0.4)
    rays = [sun_rays(SUN_C, w["x"], [w["center"][1] + dy for dy in (0.10, -0.10)], fy, gap=0.5)
            for w, fy in zip((lo, up), (floor_y, level_y))]

    def route(y, dy):
        return smooth_path([np.array([x0 + 1.0, y - 0.03, 0.0]), np.array([x0 + 0.35, y, 0.0]),
                            np.array([x0 - 0.35, y + dy / 2, 0.0]), np.array([x0 - 0.85, y + dy, 0.0])])

    p_lo, p_up, p_wall = route(lo["center"][1], 0.08), route(up["center"][1], 0.08), route(0.22, 0.06)
    lvl = {k: ValueTracker(v) for k, v in
           (("loss", 0.0), ("sun", 0.0), ("heat", 0.0), ("eta", 1.0), ("k", 0.0), ("sk", 0.0))}
    flows = [
        _level_flow([p_lo], LOSS_BLUE, lambda: lvl["loss"].get_value()),
        _level_flow([p_up, p_wall], LOSS_BLUE, lambda: lvl["loss"].get_value()),
    ]
    warm = [person.get_top() + UP * 0.05, lamp["bulb"].get_center() + UP * 0.1]

    def sun_tick(g):
        k = lvl["sun"].get_value()
        g[0].set_fill(opacity=0.10 + 0.30 * k)
        g[2].set_stroke(opacity=0.15 + 0.45 * k)
        g[3].set_stroke(opacity=0.20 + 0.80 * k)

    sun.add_updater(sun_tick)
    for r in rays:
        r["out"].add_updater(lambda g: [ln.set_stroke(opacity=0.95 * lvl["sun"].get_value()) for ln in g])
        r["in"].add_updater(lambda g: [ln.set_stroke(opacity=0.5 * lvl["sun"].get_value()) for ln in g])

    stage = {"hs": hs, "person": person, "lamp": lamp, "sun": sun, "therm": therm, "rays": rays, "flows": flows,
             "lvl": lvl, "warm": warm, "glass_n": glass_n, "x0": x0, "floor_y": floor_y, "level_y": level_y,
             "lo": lo, "up": up}
    if with_radiator:
        rad = radiator(np.array([x0 + 0.62, floor_y + 0.02 + 0.14, 0.0])).scale(0.55)
        rad.move_to(np.array([x0 + 0.62, floor_y + 0.03 + rad.height / 2, 0.0]))
        rise = smooth_path([np.array([x0 + 0.62, rad.get_top()[1] + 0.02, 0.0]), np.array([x0 + 0.78, -0.10, 0.0]),
                            np.array([x0 + 1.0, 0.08, 0.0])])
        flows.append(_level_flow([rise], P_RED, lambda: lvl["heat"].get_value(), waves=5, speed=0.4))

        def rad_tick(g):
            k = lvl["heat"].get_value()
            g.set_stroke(color=interpolate_color(ManimColor(P_WHITE), ManimColor(P_RED), k), opacity=0.45 + 0.55 * k)

        rad.add_updater(rad_tick)
        stage["radiator"] = rad
    return stage


def _enter_house(scene, st, run_time=1.4, extra=()):
    """🏡 Draw the house section, then bring in its occupant, lamp and radiator; the loss streams idle at zero until ``k`` rises."""
    scene.play(Create(st["hs"]["group"]), *extra, run_time=run_time)
    glyphs = [st["person"], st["lamp"]["group"], *([st["radiator"]] if "radiator" in st else [])]
    scene.play(*[FadeIn(g) for g in glyphs], run_time=0.8)
    scene.add(*st["flows"])


def _enter_therm(scene, st, run_time=0.8, extra=()):
    """🥶 Bring in the outdoor thermometer; its column follows the level tracker."""
    scene.play(FadeIn(st["therm"]["group"]), *extra, run_time=run_time)
    scene.add(st["therm"]["column"])


def _enter_sun(scene, st, run_time=0.8, extra=()):
    """🌞 Fade the sun in at full strength, then draw its rays through the glazing."""
    st["sun"].suspend_updating()
    scene.play(FadeIn(st["sun"]), *extra, run_time=run_time)
    st["sun"].resume_updating()
    scene.play(*[shine(r) for r in st["rays"]], run_time=1.4)


def _leave_stage(scene, st, *extra, anims=(), run_time=1.0):
    """🚪 Fade out whatever of the stage is on screen — house, sun, rays, thermometer, radiator — plus ``extra``.

    The caller removes the month driver first; the loss and heat streams fade with the picture.
    """
    lvl, therm = st["lvl"], st["therm"]
    scene.remove(*st["flows"])
    parts = [st["hs"]["group"], st["person"], st["lamp"]["group"], st["sun"], therm["group"], therm["column"],
             *([st["radiator"]] if "radiator" in st else []),
             *[ray for r in st["rays"] for ray in (*r["out"], *r["in"])]]
    present = set(map(id, scene.get_mobject_family_members()))
    shown = [m for m in (*parts, *extra) if id(m) in present]
    for m in shown:
        m.clear_updaters()
    scene.play(*[FadeOut(m) for m in shown], *anims, run_time=run_time)
    for key in ("loss", "heat", "k", "sk", "sun"):
        lvl[key].set_value(0.0)


def _warm(st, rt: float, r_max: float = 0.34, color=P_RED):
    """♨️ Internal gains: heat rippling off the occupant and the lamp for ``rt`` seconds."""
    return [ripples(st["warm"], r_max=r_max, cycles=max(1.0, rt / 1.5), color=color)]


def _year_clock(month):
    """🕐 Year dial — one turn of the hand is the twelve ticks of the twelve months."""
    clock = clock_glyph(CLOCK_C, r=CLOCK_R)
    dial = VGroup(clock["face"], clock["group"][1])

    def hand():
        a = TAU * (month.get_value() % 12) / 12
        return Line(CLOCK_C, CLOCK_C + CLOCK_R * 0.78 * np.array([np.sin(a), np.cos(a), 0.0]), color=P_CYAN,
                    stroke_width=3)

    return dial, always_redraw(hand)


def _drive(st, month):
    """⚙️ Let the month tracker set the stage's weather effects: loss, sun, heating, η_h and the thermometer."""
    lvl, therm = st["lvl"], st["therm"]

    def tick(_m):
        m = month.get_value()
        theta = _catmull(THETA_E, m)
        lvl["loss"].set_value(float(np.clip(lvl["k"].get_value() * (THETA_I - theta) / (THETA_I - THETA_E[0]), 0, 1)))
        lvl["sun"].set_value(float(np.clip(lvl["sk"].get_value() * _catmull(IRRADIATION, m) / IRRADIATION_PEAK, 0, 1)))
        lvl["heat"].set_value(float(np.clip((THETA_HG - theta) / 3.0, 0, 1)))
        lvl["eta"].set_value(float(np.clip(_catmull(ETA_SHAPE, m), 0, 1)))
        therm["level"].set_value(float(np.clip(_theta_norm(theta), 0, 1)))

    driver = Mobject()
    driver.add_updater(tick)
    return driver


def _wrap_up(scene, keep, run_time=0.6):
    """🧹 Stop every live effect, then fade the whole beat out except the chapter header."""
    for mob in scene.mobjects:
        mob.clear_updaters()
    scene.play(*[FadeOut(m) for m in scene.mobjects if m not in keep], run_time=run_time)
    scene.wait(0.4)
#endregion

#endregion


#region Beat 1 – Vom Gesetz zur Rechenmethode

class Beat1_GEGundNormen(Scene):
    NARRATION = [
        ("intro",
         "Everything we just balanced is not a private calculation — in Germany it is prescribed by law.",
         "Alles, was wir gerade bilanziert haben, ist keine private Rechnung — in Deutschland ist sie gesetzlich vorgeschrieben."),
        ("geg",
         "The Gebäudeenergiegesetz sets the target.",
         "Das Gebäudeenergiegesetz setzt das Ziel."),
        ("grenze",
         "It caps a building's primary energy demand.",
         "Es begrenzt den Primärenergiebedarf eines Gebäudes."),
        ("offen",
         "But it never says how to compute it.",
         "Wie man ihn berechnet, sagt es aber nicht."),
        ("din",
         "For that it points to DIN V 18599:2018-09.",
         "Dafür verweist es auf DIN V 18599:2018-09."),
        ("methode",
         "That is the calculation method the GEG references.",
         "Das ist die vom GEG referenzierte Berechnungsmethodik."),
        ("nachbarn",
         "Beside it sit the standards from the earlier videos.",
         "Daneben stehen die Normen der früheren Videos."),
        ("heizlast",
         "The peak heating load is calculated to DIN EN 12831-1.",
         "Die Heizlast wird nach DIN EN 12831-1 berechnet."),
        ("kuehllast",
         "The cooling load is calculated to VDI 2078.",
         "Die Kühllast wird nach VDI 2078 berechnet."),
        ("sommer",
         "And summer overheating protection is verified to DIN 4108-2.",
         "Und der sommerliche Wärmeschutz wird nach DIN 4108-2 nachgewiesen."),
    ]

    @staticmethod
    def _norm_card(lines, norm, color, size):
        """🪧 Standard card — topic on one or two lines, the DIN number below, in the shared card frame."""
        topic = VGroup(*[line if isinstance(line, Mobject) else body_text(line, font_size=LABEL_FONT_SIZE, color=P_TEAL)
                         for line in lines]).arrange(DOWN, buff=0.04)
        body = VGroup(topic, body_text(norm, font_size=LABEL_FONT_SIZE, color=color)).arrange(DOWN, buff=0.07)
        frame = RoundedRectangle(width=size[0], height=size[1], corner_radius=0.1, color=color, stroke_width=1.8,
                                 fill_color=P_DEEP_DARK, fill_opacity=0.82)
        body.move_to(frame.get_center())
        return VGroup(frame, body)

    @staticmethod
    def _law_card():
        return card_grid([("Gesetz", "GEG — Gebäudeenergiegesetz", P_TEAL)], rows=1, value_size=LABEL_FONT_SIZE + 2)

    @staticmethod
    def _method_card():
        return card_grid([("Rechenverfahren", "DIN V 18599:2018-09 — Energetische Bewertung", P_CYAN)], rows=1,
                         value_size=LABEL_FONT_SIZE + 1)

    @staticmethod
    def _topic(word):
        return VGroup(body_text(word, font_size=LABEL_FONT_SIZE, color=P_TEAL),
                      math_label(r"[\mathrm{kW}]", size=LABEL_FONT_SIZE, color=P_TEAL)).arrange(RIGHT, buff=0.1)

    def construct(self):
        apply_scene_style(self)
        t0 = self.renderer.time

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Vom Gesetz zur Rechenmethode", title)
        din = _din_ref("GEG § 20 ff.")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        law = self._law_card().scale(1.1).move_to(np.array([0.0, 0.3, 0.0]))
        self.play(FadeIn(law, shift=DOWN * 0.15), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=self.renderer.time - t0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "geg"))
        goal = note_line("Ziel: den Primärenergiebedarf begrenzen", color=P_TEAL)
        self.play(law.animate.move_to(np.array([-2.6, 1.0, 0.0])), FadeIn(goal), run_time=1.4)
        hold_for(self, self.NARRATION, "geg")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "grenze"))
        gauge = _gauge(r"Q_P", color=P_TEAL, length=2.2, thickness=0.5)
        gauge["group"][0].move_to(np.array([3.0, -0.2, 0.0]))
        gauge["fill"].move_to(gauge["track"].get_center())
        gauge["tag"].next_to(gauge["track"], UP, buff=0.16)
        cap_y = gauge["track"].get_bottom()[1] + 0.04 + 0.8 * gauge["span"]
        cap_line = DashedLine(np.array([3.0 - 0.45, cap_y, 0.0]), np.array([3.0 + 0.45, cap_y, 0.0]),
                              color=P_RED, stroke_width=3)
        self.play(Create(gauge["track"]), FadeIn(gauge["tag"]), FadeIn(gauge["fill"]), run_time=1.4)
        self.play(Create(cap_line), run_time=1.2)
        self.play(Indicate(cap_line, color=P_RED, scale_factor=1.15), run_time=1.0)
        hold_for(self, self.NARRATION, "grenze")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "offen"))
        unknown = body_text("?", font_size=72, color=P_ORANGE).move_to(np.array([4.4, -0.2, 0.0]))
        self.play(FadeIn(unknown, scale=0.6), run_time=1.2)
        self.play(Indicate(unknown, color=P_ORANGE, scale_factor=1.2), run_time=1.0)
        hold_for(self, self.NARRATION, "offen")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        method = self._method_card().scale(1.1).move_to(np.array([-2.6, -0.7, 0.0]))
        link = Arrow(law.get_bottom(), method.get_top(), buff=0.10, color=P_WHITE, stroke_width=3,
                     max_tip_length_to_length_ratio=0.22)
        self.play(FadeOut(unknown), FadeOut(goal), run_time=0.6)
        self.play(GrowArrow(link), FadeIn(method, shift=DOWN * 0.15), run_time=1.4)
        hold_for(self, self.NARRATION, "din")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "methode"))
        revision = note_line("Die Normenreihe wurde 2025 als DIN/TS 18599 überarbeitet.", color=P_TEAL)
        compute = Arrow(method.get_right(), gauge["track"].get_left() + LEFT * 0.05, buff=0.08, color=P_CYAN,
                        stroke_width=3, max_tip_length_to_length_ratio=0.12)
        self.play(GrowArrow(compute), FadeIn(revision), run_time=1.2)
        self.play(gauge["tracker"].animate.set_value(0.62), run_time=2.4)
        hold_for(self, self.NARRATION, "methode")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nachbarn"))
        self.play(FadeOut(law), FadeOut(link), FadeOut(compute), FadeOut(gauge["track"]), FadeOut(gauge["fill"]),
                  FadeOut(gauge["tag"]), FadeOut(cap_line), FadeOut(revision), run_time=0.8)
        self.play(method.animate.move_to(np.array([0.0, 1.2, 0.0])), run_time=1.2)
        hold_for(self, self.NARRATION, "nachbarn")

        size = (3.6, 1.5)
        cards = VGroup(
            self._norm_card([self._topic("Heizlast")], "DIN EN 12831-1", LOSS_BLUE, size),
            self._norm_card([self._topic("Kühllast")], "VDI 2078", P_RED, size),
            self._norm_card(["Sommerlicher", "Wärmeschutz"], "DIN 4108-2", SOLAR_YELLOW, size),
        ).arrange(RIGHT, buff=0.35)
        cards.move_to(np.array([0.0, -1.1, 0.0]))
        spokes = VGroup(*[
            Arrow(method.get_bottom() + DOWN * 0.0, c.get_top(), buff=0.10, color=P_WHITE, stroke_width=2,
                  stroke_opacity=0.7, max_tip_length_to_length_ratio=0.18)
            for c in cards
        ])
        for key, card, spoke in zip(("heizlast", "kuehllast", "sommer"), cards, spokes):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(GrowArrow(spoke), FadeIn(card, shift=DOWN * 0.12), run_time=1.4)
            self.play(Indicate(card, color=card[0].get_color(), scale_factor=1.06), run_time=1.0)
            hold_for(self, self.NARRATION, key)

        _wrap_up(self, keep=[title, subtitle, din], run_time=0.6)

#endregion


#region Beat 2 – Warum Monat für Monat bilanziert wird

class Beat2_Monatsbilanz(Scene):
    NARRATION = [
        ("intro",
         "The standard balances month by month.",
         "Die Norm bilanziert Monat für Monat."),
        ("warum",
         "Here is why an annual average would be wrong.",
         "Hier ist der Grund, warum ein Jahresmittel falsch wäre."),
        ("verluste",
         "These are the monthly losses.",
         "Das sind die monatlichen Verluste."),
        ("winter",
         "They are high in winter and small in summer, following the outdoor temperature.",
         "Im Winter sind sie hoch, im Sommer klein — sie folgen der Außentemperatur."),
        ("gewinne",
         "First the available gains from sun and internal loads.",
         "Zuerst die verfügbaren Gewinne aus Sonne und internen Lasten."),
        ("nutzbar",
         "Only part of them is usable — the utilisation factor eta-h says how much.",
         "Nur ein Teil davon ist nutzbar — der Ausnutzungsgrad Eta h sagt, wie viel."),
        ("deckt",
         "Where green covers blue, sun and internal loads already pay the bill.",
         "Wo Grün das Blau deckt, zahlen Sonne und interne Lasten schon die Rechnung."),
        ("rest",
         "Only the red remainder has to be heated.",
         "Nur der rote Rest muss beheizt werden."),
        ("sommer",
         "In summer there is no heating remainder.",
         "Im Sommer bleibt kein Heizrest."),
        ("ungenutzt",
         "Those unused gains sit in the cooling balance instead of paying January.",
         "Ungenutzte Gewinne gehören in die Kühlbilanz, nicht in den Januar."),
        ("jahr",
         "An annual average would credit that summer surplus against January.",
         "Ein Jahresmittel würde diesen Sommerüberschuss dem Januar gutschreiben."),
        ("klein",
         "The demand would then look far too small.",
         "Der Bedarf erschiene dann viel zu klein."),
        ("summe",
         "So the annual heating demand is the sum of twelve separate monthly balances, never one averaged one.",
         "Der Jahresheizwärmebedarf ist deshalb die Summe von zwölf getrennten Monatsbilanzen, nie eine gemittelte."),
        ("formel",
         "In formula form: for each month the losses minus the usable gains, added up.",
         "Als Formel: je Monat Verluste minus nutzbare Gewinne, aufsummiert."),
    ]

    def construct(self):
        apply_scene_style(self)
        t0 = self.renderer.time

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Warum Monat für Monat bilanziert wird", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        month = ValueTracker(0.0)
        st = _stage()
        lvl, therm = st["lvl"], st["therm"]
        chart = _chart(r"\mathrm{kWh/Monat}", dx=CENTER_DX)
        dial, hand = _year_clock(month)
        driver = _drive(st, month)
        legend = _legend([
            ("sink", LOSS_BLUE, r"Q_{\mathrm{Senke}}", False),
            ("src", SOLAR_YELLOW, r"Q_{\mathrm{Quelle}}", True),
            ("use", ETA_GREEN, r"\eta_h\cdot Q_{\mathrm{Quelle}}", True),
            ("rest", DEMAND_RED, r"Q_h", True),
            ("unused", P_TEAL, r"=ungenutzt", True),
        ], dx=CENTER_DX)
        frame = [chart["group"], chart["months"], *legend.values()]

        self.play(Create(chart["group"]), FadeIn(chart["months"]), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=self.renderer.time - t0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "warum"))
        _enter_house(self, st, 1.4, extra=_slide(self, chart, frame, -CENTER_DX))
        _enter_therm(self, st, 0.8, extra=[FadeIn(dial)])
        self.add(driver, hand)
        self.play(month.animate.set_value(12.0), run_time=4.0, rate_func=linear)
        hold_for(self, self.NARRATION, "warum")

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verluste"))
        sink = _layer(chart, month, base, [(0.0, v) for v in LOSS_SHAPE], side=-1, color=LOSS_BLUE, fill=False, stroke=2.0)
        cursor = _cursor(chart, month, base)
        self.add(sink, cursor)
        self.play(lvl["k"].animate.set_value(1.0), FadeIn(legend["sink"]), run_time=0.8)
        self.play(month.animate.set_value(base + 12), run_time=6.5, rate_func=linear)
        sink.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "verluste")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        self.remove(driver)
        cells = list(sink)
        self.play(Indicate(VGroup(cells[11], cells[0], cells[1]), color=LOSS_BLUE, scale_factor=1.12), run_time=1.4)
        self.play(Indicate(VGroup(cells[5], cells[6], cells[7]), color=LOSS_BLUE, scale_factor=1.12),
                  lvl["loss"].animate.set_value(0.2), therm["level"].animate.set_value(_theta_norm(THETA_E[6])),
                  run_time=1.6)
        self.play(lvl["loss"].animate.set_value(0.0), run_time=0.8)
        lvl["k"].set_value(0.0)
        hold_for(self, self.NARRATION, "winter")

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gewinne"))
        therm["column"].clear_updaters()
        lvl["sk"].set_value(1.0)
        lvl["sun"].set_value(1.0)
        self.play(FadeOut(therm["group"]), FadeOut(therm["column"]), run_time=0.6)
        _enter_sun(self, st)
        src = _layer(chart, month, base, [(0.0, v) for v in GAIN_SHAPE], side=1, color=SOLAR_YELLOW, opacity=0.6)
        cursor = _cursor(chart, month, base)
        self.add(src, cursor, driver)
        self.play(month.animate.set_value(base + 12), FadeIn(legend["src"]), *_warm(st, 7.0), run_time=7.0,
                  rate_func=linear)
        src.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "gewinne", during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nutzbar"))
        use_src = _layer(chart, month, base, [(0.0, v) for v in USABLE_SHAPE], side=1, color=ETA_GREEN, opacity=0.95)
        use_snk = _layer(chart, month, base, [(0.0, v) for v in USABLE_SHAPE], side=-1, color=ETA_GREEN, opacity=0.95)
        cursor = _cursor(chart, month, base)
        eta = _gauge(r"\eta_h", color=ETA_GREEN, length=1.9, thickness=0.30)
        eta["group"][0].move_to(np.array([6.1, -0.30, 0.0]))
        eta["fill"].move_to(eta["track"].get_center())
        eta["tag"].next_to(eta["track"], UP, buff=0.14)
        eta["fill"].add_updater(lambda _f: eta["tracker"].set_value(lvl["eta"].get_value()))
        self.add(use_src, use_snk, cursor)
        self.play(FadeIn(eta["track"]), FadeIn(eta["fill"]), FadeIn(eta["tag"]), FadeIn(legend["use"]), run_time=1.0)
        self.play(month.animate.set_value(base + 12), *_warm(st, 7.0), run_time=7.0, rate_func=linear)
        for layer in (use_src, use_snk):
            layer.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "nutzbar", during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "deckt"))
        self.remove(driver)
        eta["fill"].clear_updaters()
        _leave_stage(self, st, eta["track"], eta["fill"], eta["tag"], dial, hand,
                     anims=_slide(self, chart, [*frame, sink, src, use_src, use_snk], CENTER_DX), run_time=1.4)
        summer_pairs = VGroup(*[m[i] for m in (sink, use_snk) for i in range(4, 9)])
        self.play(Indicate(summer_pairs, color=ETA_GREEN, scale_factor=1.06), run_time=1.6)
        hold_for(self, self.NARRATION, "deckt")

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rest"))
        rest = _layer(chart, month, base, [(USABLE_SHAPE[i], LOSS_SHAPE[i]) if i in REST_MONTHS else None
                                           for i in range(12)], side=-1, color=DEMAND_RED, opacity=0.92)
        cursor = _cursor(chart, month, base)
        self.add(rest, cursor)
        self.play(month.animate.set_value(base + 12), FadeIn(legend["rest"]), run_time=6.0, rate_func=linear)
        rest.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "rest")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sommer"))
        dry = [i for i in range(12) if i not in REST_MONTHS]
        lo_pt, hi_pt = chart["pt"](dry[0] / 12, 0.0), chart["pt"]((dry[-1] + 1) / 12, 1.0)
        zone = Rectangle(width=hi_pt[0] - lo_pt[0], height=hi_pt[1] - lo_pt[1], color=P_TEAL, stroke_width=0,
                         fill_color=P_TEAL, fill_opacity=0.10).move_to((lo_pt + hi_pt) / 2)
        none_tag = body_text("kein Heizrest", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            chart["pt"]((dry[0] + dry[-1] + 1) / 24, 0.9))
        self.play(FadeIn(zone), FadeIn(none_tag), run_time=1.4)
        hold_for(self, self.NARRATION, "sommer")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ungenutzt"))
        self.play(src.animate.set_fill(P_TEAL, opacity=0.7), FadeIn(legend["unused"]), run_time=1.4)
        hold_for(self, self.NARRATION, "ungenutzt")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "jahr"))
        warn = note_line("Sommerüberschuss zählt nicht für die Heizbilanz.", color=P_RED)
        rest_cells = list(rest)
        credit = 1.04 / sum(REST_SHAPE)
        shrunk = VGroup(*[_cell(chart, i, -1, USABLE_SHAPE[i], USABLE_SHAPE[i] + REST_SHAPE[i] * credit, DEMAND_RED,
                                opacity=0.92) for i in REST_MONTHS])
        surplus = [i for i in range(12) if UNUSED_SHAPE[i] > 0.1]
        winter = [0, 11, 1, 10, 2, 9]
        chunks = VGroup(*[_cell(chart, i, 1, USABLE_SHAPE[i], GAIN_SHAPE[i], P_TEAL, opacity=0.9) for i in surplus])
        targets = [rest_cells[REST_MONTHS.index(j)].get_top() for j in winter[:len(surplus)]]
        self.play(FadeOut(zone), FadeOut(none_tag), FadeIn(warn), FadeIn(chunks), run_time=1.0)
        self.play(*[c.animate.move_to(t).scale(0.25).set_opacity(0.0) for c, t in zip(chunks, targets)],
                  run_time=1.8)
        hold_for(self, self.NARRATION, "jahr")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "klein"))
        wrong = cross_mark(P_RED, 0.28).move_to(chart["pt"](0.42, 0.78))
        self.play(Transform(rest, shrunk), run_time=1.6)
        self.play(FadeIn(wrong, scale=0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "klein")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summe"))
        full = VGroup(*[_cell(chart, i, -1, USABLE_SHAPE[i], LOSS_SHAPE[i], DEMAND_RED, opacity=0.92)
                        for i in REST_MONTHS])
        bar_x = 6.1 + CENTER_DX
        bar = stacked_bar([(None, REST_SHAPE[i], DEMAND_RED) for i in REST_MONTHS], base=np.array([bar_x, -1.28, 0.0]),
                          width=0.5, unit=0.5)
        total = math_label(r"Q_{h,a}", size=BODY_FONT_SIZE, color=DEMAND_RED).next_to(bar["top"], UP, buff=0.16)
        self.play(FadeOut(wrong), FadeOut(warn), Transform(rest, full), run_time=0.8)
        self.play(LaggedStart(*[ReplacementTransform(c, b) for c, b in zip(rest_cells, bar["bars"])], lag_ratio=0.1),
                  FadeIn(total), run_time=1.8)
        hold_for(self, self.NARRATION, "summe")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formel"))
        row, box, items = math_panel([
            ("qh", r"Q_{h,a}", DEMAND_RED), (None, "=", P_WHITE),
            ("sum", r"\Sigma_{\text{12 Monate}}", P_WHITE),
            (None, "(", P_WHITE), ("sink", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, "-", P_WHITE),
            ("eta", r"\eta_h", ETA_GREEN), (None, r"\cdot", P_WHITE), ("src", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW),
            (None, ")", P_WHITE),
        ], size=FORMULA_FONT_SIZE - 3, buff=0.12)
        self.play(Create(box), FadeIn(row), run_time=1.2)
        ring = highlight_param(items, "qh", color=P_ORANGE)
        self.play(Create(ring), Indicate(bar["bars"], color=DEMAND_RED, scale_factor=1.04), run_time=1.0)
        ring2 = highlight_param(items, "sum", color=P_ORANGE)
        self.play(ReplacementTransform(ring, ring2), Indicate(bar["bars"], color=P_ORANGE), run_time=1.0)
        ring3 = highlight_param(items, "sink", color=P_ORANGE)
        self.play(ReplacementTransform(ring2, ring3), Indicate(sink, color=LOSS_BLUE), run_time=1.0)
        ring4 = highlight_param(items, "eta", color=P_ORANGE)
        self.play(ReplacementTransform(ring3, ring4), Indicate(use_snk, color=ETA_GREEN), run_time=1.0)
        hold_for(self, self.NARRATION, "formel")

        self.play(FadeOut(box), FadeOut(row), FadeOut(ring4), run_time=0.4)
        _wrap_up(self, keep=[title, subtitle, din])

#endregion


#region Beat 3 – Klimadaten: das Testreferenzjahr

class Beat3_Klimadaten(Scene):
    NARRATION = [
        ("intro",
         "Every one of those twelve balances needs two inputs from outside the building: temperature and sun.",
         "Jede dieser zwölf Bilanzen braucht zwei Eingangsgrößen von außen: Temperatur und Sonne."),
        ("temperatur",
         "The monthly mean outdoor temperature drives the losses.",
         "Die mittlere monatliche Außentemperatur treibt die Verluste."),
        ("delta",
         "It is the delta theta in every transmission term.",
         "Sie ist das Delta Theta in jedem Transmissionsterm."),
        ("strahlung",
         "The monthly irradiation per orientation drives the solar gains.",
         "Die monatliche Einstrahlung je Orientierung treibt die solaren Gewinne."),
        ("orientierung",
         "That is why a south window counts more than a north one.",
         "Deshalb zählt ein Südfenster mehr als ein Nordfenster."),
        ("try",
         "Both come from the German weather service as a test reference year.",
         "Beides liefert der Deutsche Wetterdienst als Testreferenzjahr."),
        ("zwoelf",
         "That is twelve synthetic but representative months.",
         "Das sind zwölf synthetische, aber repräsentative Monate."),
        ("geg",
         "For certain GEG proofs a standardised reference climate is fixed.",
         "Für bestimmte GEG-Nachweise ist ein standardisiertes Referenzklima festgelegt."),
        ("potsdam",
         "The Potsdam reference climate zone plays the central role in it.",
         "Die Referenzklimazone Potsdam spielt dabei die zentrale Rolle."),
    ]

    @staticmethod
    def _source_card():
        return card_grid([("Datenquelle", "DWD · Testreferenzjahr (TRY)", P_TEAL)], rows=1, value_size=LABEL_FONT_SIZE + 1)

    @staticmethod
    def _delta(chart, month, base, theta_fn):
        """📏 Δθ between the indoor line and the outdoor curve at the year cursor."""
        x = float(np.clip(month.get_value() - base, 0.0, 12.0)) / 12
        lo_pt = chart["pt"](x, theta_fn(x * 12))
        hi_pt = chart["pt"](x, _theta_norm(THETA_I))
        arrow = dim_arrow(lo_pt, hi_pt, P_ORANGE) if hi_pt[1] - lo_pt[1] > 0.3 else VGroup()
        tag = math_label(r"\Delta\theta", size=LABEL_FONT_SIZE, color=P_ORANGE)
        tag.next_to(hi_pt, RIGHT, buff=0.12)
        tag.set_y(hi_pt[1] + 0.22)
        return VGroup(arrow, tag)

    def construct(self):
        apply_scene_style(self)
        t0 = self.renderer.time

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Klimadaten: das Testreferenzjahr", title)
        din = _din_ref("DIN V 18599-10 · DWD TRY")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        month = ValueTracker(0.0)
        st = _stage(north=True)
        lvl, therm, sun = st["lvl"], st["therm"], st["sun"]
        theta_fn = lambda x: _theta_norm(_catmull(THETA_E, x))  # noqa: E731
        chart = _chart(r"\theta_e\,[\mathrm{°C}]", ticks=[(_theta_norm(v), str(v)) for v in (0, 10, 20)], color=P_BLUE)
        right = _right_axis(chart, r"I_S\,[\mathrm{kWh/m^2}]",
                            [(v / IRRADIATION_FULL_KWH, str(v)) for v in (0, 100, 200)], SOLAR_YELLOW)
        dial, hand = _year_clock(month)
        driver = _drive(st, month)
        theta_tag = math_label(r"\theta_e", size=BODY_FONT_SIZE, color=P_BLUE).move_to(np.array([THERMO_X, 0.82, 0.0]))
        sun_tag = math_label(r"I_S", size=BODY_FONT_SIZE, color=SOLAR_YELLOW).move_to(np.array([-5.85, 1.82, 0.0]))
        lvl["sk"].set_value(1.0)
        lvl["sun"].set_value(1.0)

        self.play(Create(chart["group"]), FadeIn(chart["months"]), FadeIn(dial), run_time=1.2)
        self.add(hand)
        _enter_therm(self, st, 1.2, extra=[FadeIn(theta_tag, shift=DOWN * 0.1)])
        sun.suspend_updating()
        self.play(FadeIn(sun), FadeIn(sun_tag, shift=DOWN * 0.1), run_time=1.2)
        sun.resume_updating()
        hold_for(self, self.NARRATION, "intro", used=self.renderer.time - t0)

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "temperatur"))
        lvl["sk"].set_value(0.0)
        sun.suspend_updating()
        self.play(FadeOut(sun), FadeOut(sun_tag), run_time=0.6)
        sun.resume_updating()
        _enter_house(self, st, 1.4)
        theta = _trace(chart, month, base, theta_fn, P_BLUE, dots=True)
        cursor = _cursor(chart, month, base)
        self.add(theta, cursor, driver, hand)
        self.play(lvl["k"].animate.set_value(1.0), run_time=0.8)
        self.play(month.animate.set_value(base + 12), run_time=6.5, rate_func=linear)
        theta.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "temperatur")

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "delta"))
        indoor = DashedLine(chart["pt"](0.0, _theta_norm(THETA_I)), chart["pt"](1.0, _theta_norm(THETA_I)),
                            color=P_ORANGE, stroke_width=2.0, stroke_opacity=0.8)
        indoor_tag = math_label(r"\theta_i", size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(indoor, UP, buff=0.04).align_to(
            indoor, RIGHT)
        self.play(Create(indoor), FadeIn(indoor_tag), run_time=1.2)
        spread = always_redraw(lambda: self._delta(chart, month, base, theta_fn))
        cursor = _cursor(chart, month, base)
        self.add(spread, cursor)
        self.play(month.animate.set_value(base + 12), run_time=6.0, rate_func=linear)
        self.remove(cursor, spread, driver)
        hold_for(self, self.NARRATION, "delta")

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "strahlung"))
        therm["column"].clear_updaters()
        lvl["k"].set_value(0.0)
        self.play(FadeOut(indoor), FadeOut(indoor_tag), FadeOut(theta_tag), FadeOut(therm["group"]),
                  FadeOut(therm["column"]), lvl["loss"].animate.set_value(0.0), FadeIn(right), run_time=1.0)
        lvl["sk"].set_value(1.0)
        lvl["sun"].set_value(1.0)
        sun.suspend_updating()
        self.play(FadeIn(sun), FadeIn(sun_tag, shift=DOWN * 0.1), run_time=0.8)
        sun.resume_updating()
        self.play(*[shine(r) for r in st["rays"]], run_time=1.4)
        sun_cols = _layer(chart, month, base, [(0.0, v) for v in IRRADIATION], side=0, color=SOLAR_YELLOW, opacity=0.55)
        cursor = _cursor(chart, month, base)
        self.add(sun_cols, cursor, driver)
        self.play(month.animate.set_value(base + 12), run_time=6.5, rate_func=linear)
        sun_cols.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "strahlung")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "orientierung"))
        self.remove(hand)
        orient = self._orientation(st, month, month.get_value() + 6.5,
                                   [chart["group"], chart["months"], right, theta, sun_cols, dial])
        hold_for(self, self.NARRATION, "orientierung")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "try"))
        self.remove(driver, hand)
        source = self._source_card().scale(0.85).move_to(np.array([0.0, 1.92, 0.0]))
        frame = [chart["group"], chart["months"], right, theta, sun_cols]
        _leave_stage(self, st, sun_tag, *orient,
                     anims=[*_slide(self, chart, frame, CENTER_DX), *[FadeIn(m) for m in frame]], run_time=1.4)
        self.play(FadeIn(source, shift=DOWN * 0.12), run_time=1.0)
        hold_for(self, self.NARRATION, "try")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zwoelf"))
        years = VGroup(*[
            _curve(chart, lambda x, k=k: _theta_norm(_catmull(THETA_E, x) + 1.2 * np.sin(1.7 * x + 1.9 * k)),
                   P_BLUE, width=2.0, opacity=0.4)
            for k in range(4)
        ])
        mean = VGroup(*[_curve(chart, theta_fn, P_BLUE, width=3.0) for _ in range(4)])
        self.play(FadeOut(source), FadeOut(sun_cols), FadeOut(right), run_time=0.8)
        self.play(FadeIn(years), run_time=1.2)
        self.play(Transform(years, mean), run_time=3.0)
        self.remove(years)
        hold_for(self, self.NARRATION, "zwoelf")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "geg"))
        ref_note = note_line("GEG-Nachweis: standardisiertes Referenzklima", color=P_CYAN)
        self.play(FadeIn(ref_note), run_time=1.2)
        hold_for(self, self.NARRATION, "geg")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "potsdam"))
        peak = chart["pt"](6.5 / 12, theta_fn(6.5))
        pin = chip("Potsdam", P_CYAN, font_size=LABEL_FONT_SIZE).move_to(np.array([peak[0], 1.62, 0.0]))
        leader = Line(pin.get_bottom(), peak, color=P_CYAN, stroke_width=1.6, stroke_opacity=0.8)
        self.play(FadeIn(pin), Create(leader), run_time=1.2)
        self.play(Indicate(theta, color=P_CYAN, scale_factor=1.03), run_time=1.4)
        hold_for(self, self.NARRATION, "potsdam")

        self.play(FadeOut(ref_note), FadeOut(pin), FadeOut(leader), run_time=0.4)
        _wrap_up(self, keep=[title, subtitle, din])

    def _orientation(self, st, month, target, clear):
        """🧭 Same summer sun, two façades: the south window takes the rays, the north one only the diffuse sky."""
        hs = st["hs"]
        br = hs["bottom_right"]
        up = hs["windows"][1]
        sky_c = np.array([-0.95, 1.18, 0.0])
        marker = Rectangle(width=0.62, height=0.10, stroke_width=0, fill_opacity=0).move_to(sky_c)
        sky = cloud_glyph(marker, pad=(0.30, 0.22), bump=0.4)
        diffuse = sun_rays(sky_c, float(br[0]), [up["center"][1] + 0.08, up["center"][1] - 0.08], st["level_y"], gap=0.5)
        diffuse["out"].set_stroke(opacity=0.38)
        diffuse["in"].set_stroke(opacity=0.2)
        south = body_text("Süd", font_size=LABEL_FONT_SIZE, color=SOLAR_YELLOW).move_to(
            np.array([float(hs["bottom_left"][0]) + 0.1, -1.38, 0.0]))
        north = body_text("Nord", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(np.array([float(br[0]) - 0.1, -1.38, 0.0]))
        self.play(month.animate.set_value(target), *[FadeOut(m) for m in clear], run_time=1.4, rate_func=linear)
        self.play(FadeIn(south), run_time=1.0)
        self.play(FadeIn(north), FadeIn(sky), run_time=1.0)
        self.play(Create(diffuse["out"]), run_time=1.2)
        self.play(Create(diffuse["in"]), *[shine(r) for r in st["rays"]], run_time=1.2)
        return [sky, south, north, diffuse["out"], diffuse["in"]]

#endregion


#region Beat 4 – Heizgrenze & Gradtagszahl

class Beat4_Gradtagszahl(Scene):
    NARRATION = [
        ("intro",
         "The weather year is a whole curve of twelve monthly values.",
         "Das Wetterjahr ist eine ganze Kurve aus zwölf Monatswerten."),
        ("kennzahl",
         "One number compresses it into a single figure engineers actually quote.",
         "Eine einzige Kennzahl verdichtet sie zu einer Zahl, die Planer wirklich nennen."),
        ("heizgrenze",
         "Above a defined reference temperature the free gains cover the losses.",
         "Oberhalb einer festgelegten Referenztemperatur decken die freien Gewinne die Verluste."),
        ("konvention",
         "That is a calculation convention, not a physical limit.",
         "Das ist eine Rechenkonvention, keine physikalische Grenze."),
        ("flaeche",
         "Below it, every degree and every day counts.",
         "Darunter zählt jedes Grad und jeder Tag."),
        ("prinzip",
         "The area shows the principle.",
         "Die Fläche zeigt das Prinzip."),
        ("tageswerte",
         "The real degree-day figure comes from daily values on the heating days.",
         "Real wird die Gradtagszahl aus Tageswerten der Heiztage berechnet."),
        ("kalt",
         "A cold winter gives a large degree-day number.",
         "Ein kalter Winter ergibt eine große Gradtagszahl."),
        ("mild",
         "A mild winter gives a small one.",
         "Ein milder Winter ergibt eine kleine."),
        ("nutzen",
         "As a simplified indicator it makes cold and mild winters comparable.",
         "Als vereinfachter Indikator macht sie kalte und milde Winter vergleichbar."),
    ]

    def construct(self):
        apply_scene_style(self)
        t0 = self.renderer.time

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Heizgrenze & Gradtagszahl", title)
        din = _din_ref("VDI 3807 · DIN 4108-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        month = ValueTracker(0.0)
        st = _stage(with_radiator=True)
        lvl, therm = st["lvl"], st["therm"]
        theta_fn = lambda x: _theta_norm(_catmull(THETA_E, x))  # noqa: E731
        limit_f = _theta_norm(THETA_HG)
        chart = _chart(r"\theta_e\,[\mathrm{°C}]", ticks=[(_theta_norm(v), str(v)) for v in (0, 10, 20)], color=P_BLUE,
                       dx=CENTER_DX)
        dial, hand = _year_clock(month)
        driver = _drive(st, month)
        readout_x = 0.85 + CENTER_DX

        self.play(Create(chart["group"]), FadeIn(chart["months"]), FadeIn(dial), run_time=1.2)
        base = month.get_value()
        theta = _trace(chart, month, base, theta_fn, P_BLUE, dots=True)
        cursor = _cursor(chart, month, base)
        self.add(theta, cursor, hand)
        self.play(month.animate.set_value(base + 12), run_time=6.5, rate_func=linear)
        theta.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "intro", used=self.renderer.time - t0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kennzahl"))
        teaser = math_label(r"G_t=\,?", size=BODY_FONT_SIZE + 4, color=P_ORANGE).move_to(
            np.array([readout_x + 0.9, LEGEND_Y - 0.1, 0.0]))
        self.play(FadeIn(teaser, shift=DOWN * 0.1), run_time=1.4)
        self.play(Indicate(teaser, color=P_ORANGE, scale_factor=1.15), run_time=1.0)
        hold_for(self, self.NARRATION, "kennzahl")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "heizgrenze"))
        frame = [chart["group"], chart["months"], theta]
        _enter_house(self, st, 1.4, extra=[*_slide(self, chart, frame, -CENTER_DX), FadeOut(teaser)])
        _enter_therm(self, st, 0.8)
        limit = DashedLine(chart["pt"](0.0, limit_f), chart["pt"](1.0, limit_f), color=P_ORANGE, stroke_width=2.5)
        limit_tag = math_label(r"\theta_{HG}=15\,\mathrm{°C}", size=BODY_FONT_SIZE, color=P_ORANGE)
        limit_tag.next_to(chart["pt"](0.0, limit_f), UP, buff=0.16).align_to(chart["pt"](0.0, limit_f), LEFT).shift(RIGHT * 0.1)
        mark_y = therm["top"][1] - 1.30 + 0.05 + (1.30 - 0.12) * limit_f
        therm_mark = Line(np.array([THERMO_X - 0.2, mark_y, 0.0]), np.array([THERMO_X + 0.2, mark_y, 0.0]),
                          color=P_ORANGE, stroke_width=3)
        self.play(Create(limit), FadeIn(limit_tag), Create(therm_mark), run_time=1.2)
        base = month.get_value()
        strips = always_redraw(lambda: self._heating_strips(chart, month, base))
        cursor = _cursor(chart, month, base)
        self.add(strips, cursor, driver)
        self.play(month.animate.set_value(base + 12), run_time=7.0, rate_func=linear)
        strips.clear_updaters()
        self.remove(cursor, driver)
        hold_for(self, self.NARRATION, "heizgrenze")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "konvention"))
        self.play(Indicate(limit, color=P_ORANGE, scale_factor=1.04), Indicate(limit_tag, color=P_ORANGE),
                  lvl["heat"].animate.set_value(0.0), run_time=1.6)
        hold_for(self, self.NARRATION, "konvention")

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flaeche"))
        area = self._degree_area(chart, month, base, shift=0.0, color=P_ORANGE, opacity=0.34)
        gt = math_readout(lambda: rf"G_t={de_num(_degree_days(float(np.clip(month.get_value() - base, 0, 12))), 0)}"
                                  r"\,\mathrm{Kd/a}",
                          np.array([readout_x, LEGEND_Y - 0.1, 0.0]), size=BODY_FONT_SIZE, color=P_ORANGE, edge="left")
        cursor = _cursor(chart, month, base)
        _leave_stage(self, st, therm_mark, strips, dial, hand,
                     anims=_slide(self, chart, [*frame, limit, limit_tag], CENTER_DX), run_time=1.4)
        self.add(area, cursor, gt)
        self.play(month.animate.set_value(base + 12), run_time=7.0, rate_func=linear)
        area.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "flaeche")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "prinzip"))
        self.play(Indicate(area, color=P_ORANGE, scale_factor=1.03), run_time=1.6)
        hold_for(self, self.NARRATION, "prinzip")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "tageswerte"))
        daily = self._daily_values(chart)
        daily_note = note_line("Prinzipdarstellung — reale Gradtagszahlen aus Tageswerten", color=P_ORANGE)
        self.play(Create(daily, run_time=1.6, rate_func=linear), FadeIn(daily_note), run_time=1.6)
        hold_for(self, self.NARRATION, "tageswerte")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kalt"))
        g_cold, g_mild = _degree_days(12.0, -2.0), _degree_days(12.0, 2.0)
        cold = _curve(chart, lambda x: _theta_norm(_catmull(THETA_E, x) - 2.0), P_BLUE, width=3.0)
        mild = _curve(chart, lambda x: _theta_norm(_catmull(THETA_E, x) + 2.0), P_CYAN, width=3.0)
        cold_area = self._degree_area(chart, ValueTracker(12.0), 0.0, shift=-2.0, color=P_BLUE, opacity=0.22)
        mild_area = self._degree_area(chart, ValueTracker(12.0), 0.0, shift=2.0, color=P_CYAN, opacity=0.22)
        cold_area.clear_updaters()
        mild_area.clear_updaters()
        cold_gt = math_label(rf"G_{{t,\mathrm{{kalt}}}}={de_num(g_cold, 0)}\,\mathrm{{Kd/a}}", size=BODY_FONT_SIZE,
                             color=P_BLUE).move_to(np.array([-0.2 + CENTER_DX, LEGEND_Y - 0.1, 0.0]))
        mild_gt = math_label(rf"G_{{t,\mathrm{{mild}}}}={de_num(g_mild, 0)}\,\mathrm{{Kd/a}}", size=BODY_FONT_SIZE,
                             color=P_CYAN).move_to(np.array([3.6 + CENTER_DX, LEGEND_Y - 0.1, 0.0]))
        self.remove(gt)
        self.play(FadeOut(daily), FadeOut(daily_note), FadeOut(area), FadeOut(gt), FadeOut(limit_tag),
                  theta.animate.set_stroke(opacity=0.35), run_time=0.8)
        self.play(Create(cold), FadeIn(cold_area), FadeIn(cold_gt), run_time=2.0)
        hold_for(self, self.NARRATION, "kalt")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "mild"))
        self.play(Create(mild), FadeIn(mild_area), FadeIn(mild_gt), run_time=2.0)
        hold_for(self, self.NARRATION, "mild")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nutzen"))
        row, box, items = math_panel([
            ("q", r"Q_{\text{temp.-abh.}}", LOSS_BLUE), (None, "~", P_WHITE), ("gt", r"G_t", P_ORANGE),
            (None, r"\to", P_WHITE), (None, r"\text{vereinfachtes Prinzip der Witterungsbereinigung}", P_TEAL),
        ], size=FORMULA_FONT_SIZE - 4)
        self.play(Create(box), FadeIn(row), run_time=1.2)
        ring = highlight_param(items, "gt", color=P_ORANGE)
        self.play(Create(ring), Indicate(VGroup(cold_area, mild_area), color=P_ORANGE, scale_factor=1.02), run_time=1.2)
        hold_for(self, self.NARRATION, "nutzen")

        self.play(FadeOut(box), FadeOut(row), FadeOut(ring), run_time=0.4)
        _wrap_up(self, keep=[title, subtitle, din])

    @staticmethod
    def _degree_area(chart, month, base, *, shift, color, opacity):
        """🟧 Area between the heating limit and the outdoor curve, filled up to the year cursor."""
        limit_f = _theta_norm(THETA_HG)

        def build():
            reach = float(np.clip(month.get_value() - base, 0.0, 12.0))
            group = VGroup()
            if reach <= 0.02:
                return group
            run = []

            def flush():
                if len(run) > 1:
                    pts = [chart["pt"](x / 12, _theta_norm(t)) for x, t in run]
                    pts += [chart["pt"](x / 12, limit_f) for x, _t in reversed(run)]
                    group.add(Polygon(*pts, fill_color=color, fill_opacity=opacity, stroke_width=0))
                run.clear()

            for x in np.linspace(0.0, reach, max(3, int(reach * 24) + 1)):
                t = _catmull(THETA_E, x) + shift
                if t < THETA_HG:
                    run.append((float(x), t))
                else:
                    flush()
            flush()
            return group

        return always_redraw(build)

    @staticmethod
    def _heating_strips(chart, month, base):
        """🔥 The heating season marked on the time axis as far as the year cursor has come."""
        reach = float(np.clip(month.get_value() - base, 0.0, 12.0))
        group = VGroup()
        run = []

        def flush():
            if len(run) > 1:
                line = Line(chart["pt"](run[0] / 12, 0.0), chart["pt"](run[-1] / 12, 0.0), color=P_ORANGE,
                            stroke_width=7, stroke_opacity=0.85)
                group.add(line.set_cap_style(CapStyleType.ROUND))
            run.clear()

        for x in np.linspace(0.0, reach, max(3, int(reach * 12) + 1)):
            if _catmull(THETA_E, x) < THETA_HG:
                run.append(float(x))
            else:
                flush()
        flush()
        return group

    @staticmethod
    def _daily_values(chart):
        """📆 Daily outdoor temperatures scatter around the monthly mean — the basis of the real G_t."""
        rng = np.random.default_rng(11)
        xs = np.linspace(0.0, 12.0, 183)
        walk = np.cumsum(rng.normal(0.0, 1.0, xs.size))
        walk -= np.linspace(walk[0], walk[-1], xs.size)
        walk = 2.2 * walk / max(1e-6, float(np.abs(walk).max())) + rng.normal(0.0, 0.9, xs.size)
        line = VMobject(color=P_BLUE, stroke_width=1.3, stroke_opacity=0.5)
        line.set_points_as_corners([chart["pt"](x / 12, float(np.clip(_theta_norm(_catmull(THETA_E, x) + w), 0.0, 1.0)))
                                    for x, w in zip(xs, walk)])
        return line

#endregion
