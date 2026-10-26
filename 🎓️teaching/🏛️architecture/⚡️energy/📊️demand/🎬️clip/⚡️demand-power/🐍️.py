

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
NOTE_BAND_Y = _manim_module.NOTE_BAND_Y
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
cross_mark = _manim_module.cross_mark
dim_arrow = _manim_module.dim_arrow
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
radiator = _manim_module.radiator
sun_glyph = _manim_module.sun_glyph
sun_rays = _manim_module.sun_rays
shine = _manim_module.shine
pulse_flashes = _manim_module.pulse_flashes
person_glyph = _manim_module.person_glyph
lamp_glyph = _manim_module.lamp_glyph
thermometer_glyph = _manim_module.thermometer_glyph
ripples = _manim_module.ripples
energy_tank = _manim_module.energy_tank

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "⏱️timing" / "🔣️.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Bedarf vs. Leistung"

POWER = P_RED
ENERGY = P_CYAN
TIME = P_YELLOW
H_T = P_ORANGE
H_V = P_BLUE
DELTA_T = P_YELLOW

PROFILE_N = 8760
PLOT_N = 365
PEAK_KW = 10.0
DESIGN_DT = 32.0
THETA_I = 20.0
THETA_E = -12.0

BASE_Y = -1.15
WIDE_ORIGIN = np.array([-4.4, -0.75, 0.0])
WIDE_W, WIDE_H = 8.8, 2.75
#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
# The whole chapter is told on one dataset: a synthetic year of heating power.
# Beat 1 reads its height and its area, Beat 2 names its peak, Beat 3 sorts it
# into a load-duration curve, Beat 4 draws candidate plant capacities across it.
# The curves use the full screen; the house appears only where its flows are the
# subject (Beat 2), and every explained object fades before the next one enters.

#region Data
def _annual_load(n: int = PROFILE_N, seed: int = 5) -> np.ndarray:
    """📉 Synthetic annual heating-power profile, normalised to its own peak.

    Shape only — a cosine season plus multi-day weather driving ``θ_i − θ_e``
    above the heating limit, one sample per hour of the year. It is deliberately
    not measured data; multiplying by ``PEAK_KW`` is what gives the beats real
    kW/kWh.
    """
    rng = np.random.default_rng(seed)
    t = np.linspace(0.0, 1.0, n)
    window = 240
    noise = np.convolve(rng.normal(0.0, 1.0, n + 2 * window), np.ones(window) / window, "same")[window:window + n]
    theta_e = 9.0 - 9.5 * np.cos(2 * np.pi * t) + 3.0 * noise / noise.std()
    load = np.maximum(0.0, 15.0 - theta_e)
    return load / load.max()


def _plot_samples(values: np.ndarray, n: int = PLOT_N) -> np.ndarray:
    """📉 Evenly spaced picks of an hourly series — what the chart actually draws."""
    return values[np.linspace(0, len(values) - 1, n).round().astype(int)]


def _cumulative(load: np.ndarray) -> np.ndarray:
    """∑ Energy in kWh after each hour — the rectangle rule with Δt = 1 h."""
    return np.concatenate([[0.0], np.cumsum(load) * PEAK_KW])


def _energy_at(cum: np.ndarray, frac: float) -> float:
    """∑ Energy collected up to a fraction of the year, in kWh."""
    return float(np.interp(frac * (len(cum) - 1), np.arange(len(cum)), cum))
#endregion


#region Stage
def _wide_chart(y_src: str = r"P\;[\mathrm{kW}]"):
    """🖼️ Full-width chart on the shared baseline — the curve is the subject, nothing shares its screen."""
    return _chart(WIDE_ORIGIN, WIDE_W, WIDE_H, r"t", y_src)


def _wide_tank(color=ENERGY):
    """🔋 Energy tank left of the wide chart, as tall as its plot area."""
    height = WIDE_H - 0.2
    return energy_tank(np.array([-6.0, WIDE_ORIGIN[1] + height / 2, 0.0]), height=height, width=0.7, color=color, level=0.0)


def _card(name: str, src: str, color: str, at):
    """🪪 Titled quantity card — the name above, the typeset symbol with its unit below."""
    inner = VGroup(body_text(name, font_size=BODY_FONT_SIZE, color=color),
                   math_label(src, size=FORMULA_FONT_SIZE, color=color)).arrange(DOWN, buff=0.25)
    frame = RoundedRectangle(width=max(inner.width + 0.8, 3.4), height=inner.height + 0.6, corner_radius=0.15,
                             color=color, stroke_width=2.4)
    return VGroup(frame, inner).move_to(at)


def _hide(scene, *mobs, run_time: float = 0.8):
    """🫥 Fade out what has been explained so the next step starts on a clean screen; live readouts stop first."""
    for mob in mobs:
        for part in mob.get_family():
            part.clear_updaters()
    scene.play(*[FadeOut(m) for m in mobs], run_time=run_time)
#endregion


#region Chart
def _chart(origin, x_len: float, y_len: float, x_src: str, y_src: str):
    """📐 Axis cross with arrow tips, typeset axis names and a unit-square mapper."""
    o = np.array(origin, dtype=float)
    x_ax = Arrow(o, o + RIGHT * x_len, buff=0, stroke_width=2.4, color=P_WHITE,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.06)
    y_ax = Arrow(o, o + UP * y_len, buff=0, stroke_width=2.4, color=P_WHITE,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.08)
    xl = math_label(x_src, size=LABEL_FONT_SIZE, color=P_WHITE).next_to(x_ax.get_end(), RIGHT, buff=0.14)
    yl = math_label(y_src, size=LABEL_FONT_SIZE, color=P_WHITE).next_to(y_ax.get_end(), UP, buff=0.10)

    def pt(fx: float, fy: float):
        return o + RIGHT * float(fx) * x_len + UP * float(fy) * y_len

    return {"o": o, "x": x_ax, "y": y_ax, "xl": xl, "yl": yl, "pt": pt, "x_len": x_len, "y_len": y_len,
            "group": VGroup(x_ax, y_ax, xl, yl)}


def _kw_ticks(chart, y_max_kw: float, values=(5, 10)):
    """📏 Power ticks on the y axis — the height of the curve read off in kW."""
    group = VGroup()
    for v in values:
        f = v / y_max_kw
        group.add(Line(chart["pt"](0, f) + LEFT * 0.07, chart["pt"](0, f) + RIGHT * 0.07, color=P_WHITE, stroke_width=2))
        group.add(math_label(rf"{v}", chart["pt"](0, f) + LEFT * 0.20 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right"))
    return group


def _month_labels(chart):
    """🗓️ Twelve month names under the year axis."""
    names = ("Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez")
    return VGroup(*[
        body_text(m, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(chart["pt"]((i + 0.5) / 12, 0) + DOWN * 0.24)
        for i, m in enumerate(names)
    ])


def _hour_labels(chart, values=(0, 2000, 4000, 6000, 8760)):
    """⏱️ Hour ticks under the sorted axis."""
    group = VGroup()
    for h in values:
        f = h / PROFILE_N
        group.add(Line(chart["pt"](f, 0) + DOWN * 0.06, chart["pt"](f, 0) + UP * 0.06, color=P_WHITE, stroke_width=2))
        group.add(math_label(de_num(h), chart["pt"](f, 0) + DOWN * 0.38, size=LABEL_FONT_SIZE - 2))
    return group


def _partial(chart, ys, frac: float, *, color=POWER, width: float = 3.0):
    """📈 The series drawn up to a fraction of the axis."""
    xs = np.linspace(0.0, 1.0, len(ys))
    k = int(np.searchsorted(xs, frac, side="right"))
    pts = [chart["pt"](x, float(y)) for x, y in zip(xs[:k], ys[:k])]
    if k < len(ys):
        pts.append(chart["pt"](frac, float(np.interp(frac, xs, ys))))
    if len(pts) < 2:
        pts = [chart["pt"](0, float(ys[0])), chart["pt"](0, float(ys[0])) + RIGHT * 0.001]
    return VMobject(color=color, stroke_width=width).set_points_as_corners(pts)


def _partial_area(chart, ys, frac: float, *, color=ENERGY, opacity: float = 0.24):
    """🟦 The area under the series up to a fraction of the axis."""
    xs = np.linspace(0.0, 1.0, len(ys))
    k = int(np.searchsorted(xs, frac, side="right"))
    pts = [chart["pt"](x, float(y)) for x, y in zip(xs[:k], ys[:k])]
    if k < len(ys):
        pts.append(chart["pt"](frac, float(np.interp(frac, xs, ys))))
    pts += [chart["pt"](max(frac, 0.0005), 0.0), chart["pt"](0.0, 0.0)]
    return Polygon(*pts, fill_color=color, fill_opacity=opacity, stroke_width=0)
#endregion


#region House and flow
def _heated_house(center, scale: float):
    """🏠 Physical-Fundamentals house with a radiator under each window and five heat-loss paths.

    Two window paths start at the radiators, two wall paths and one roof path start in the room air.
    ``spots`` are the warm points that radiate long-wave heat.
    """
    hs = house_section(center, scale)
    bl, br = hs["bottom_left"], hs["bottom_right"]
    top_y = float(hs["top_left"][1])
    storey = hs["w_height"] / 2
    floors = (float(bl[1]), float(hs["level_1"].get_center()[1]))
    rad_scale = 0.26 * storey / 0.5
    rads = VGroup()
    for fy in floors:
        r = radiator(ORIGIN, color=POWER).scale(rad_scale)
        r.move_to(np.array([bl[0] + 0.15 * scale + 0.1 + r.width / 2, fy + r.height / 2 + 0.03, 0.0]))
        rads.add(r)
    win_paths = []
    for w, r in zip(hs["windows"], rads):
        x, y = float(w["x"]), float(w["center"][1])
        start = r.get_top() + UP * 0.03
        win_paths.append(smooth_path([start, np.array([x + 0.10, y - 0.10, 0.0]), np.array([x - 0.30, y - 0.02, 0.0]),
                                      np.array([x - 0.95, y + 0.12, 0.0])]))
    wall_paths = [
        smooth_path([np.array([br[0] - 0.85 * scale - 0.2, fy + 0.4 * storey, 0.0]),
                     np.array([br[0] - 0.1, fy + 0.4 * storey + 0.02, 0.0]),
                     np.array([br[0] + 0.2, fy + 0.4 * storey + 0.12, 0.0]),
                     np.array([br[0] + 0.42, fy + 0.4 * storey + 0.52, 0.0])])
        for fy in floors
    ]
    cx = float(hs["center"][0])
    roof_x = cx + 0.8 * scale
    roof_y = top_y + 1.1 * scale * (1 - (0.8 * scale) / (1.8 * scale))
    roof_path = smooth_path([np.array([roof_x, top_y - 0.28 * scale, 0.0]), np.array([roof_x, top_y - 0.02, 0.0]),
                             np.array([roof_x + 0.03, roof_y + 0.2, 0.0]), np.array([roof_x + 0.07, roof_y + 0.5, 0.0])])
    return {
        "hs": hs, "rads": rads, "floors": floors, "storey": storey,
        "win_paths": win_paths, "wall_paths": wall_paths, "roof_path": roof_path,
        "paths": [*win_paths, *wall_paths, roof_path],
        "guides": flow_guides([*win_paths, *wall_paths, roof_path], POWER, opacity=0.22),
        "spots": [r.get_top() + UP * 0.05 for r in rads],
    }


def _stand(floor_y: float, x: float, storey: float, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the room height, feet on the floor line at ``x``."""
    height = 0.7 * storey
    return person_glyph(np.array([x, floor_y + 0.015 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _rate_stream(paths, level, rt: float, *, speed: float = 0.8, per_path: int = 8, color=POWER, color_end=P_ORANGE,
                 radius: float = 0.075):
    """💨 Particle stream whose density is the rate — ``level()`` in 0–1 decides how many particles run.

    Every particle moves at one speed, so a thicker stream means more energy per second, never a faster one.
    """
    start_c, end_c = ManimColor(color), ManimColor(color_end)
    parts, meta = VGroup(), []
    for i, path in enumerate(paths):
        for k in range(per_path):
            p = Ellipse(width=radius * 2.8, height=radius * 1.15, color=color, stroke_width=0, fill_opacity=0.0)
            p.move_to(path.point_from_proportion(0.0))
            parts.add(p)
            meta.append((path, k, (0.618 * k + 0.31 * i) % 1.0))
    cycles = speed * rt

    def update(group, alpha):
        lv = float(np.clip(level(), 0.0, 1.0)) * per_path
        for p, (path, k, off) in zip(group, meta):
            t = (alpha * cycles + off) % 1.0
            p.move_to(path.point_from_proportion(t))
            t0, t1 = max(0.0, t - 0.02), min(1.0, t + 0.02)
            tangent = path.point_from_proportion(t1) - path.point_from_proportion(t0)
            p.set_angle(float(np.arctan2(tangent[1], tangent[0])))
            fade = min(1.0, t / 0.08, (1.0 - t) / 0.10) * float(np.clip(lv - k, 0.0, 1.0))
            p.set_fill(interpolate_color(start_c, end_c, t), opacity=max(0.0, 0.92 * fade))

    return UpdateFromAlphaFunc(parts, update, remover=True, rate_func=linear)


def _heat(spots, rt: float, *, r_max: float = 0.35, color=POWER):
    """🌡️ Long-wave heat spreading from the warm radiators for ``rt`` seconds."""
    return ripples(spots, r_max=r_max, cycles=rt / 1.4, color=color)


def _life(house, level, rt: float, *, r_max: float = 0.35, paths=None, speed: float = 0.8):
    """🔥 The house at work for ``rt`` seconds — a stream per loss path plus heat ripples."""
    return [_rate_stream(house["paths"] if paths is None else paths, level, rt, speed=speed), _heat(house["spots"], rt, r_max=r_max)]


def _ramp(tracker, to: float, *, rate_func=smooth):
    """⏩ Move a tracker to ``to`` — pass ``linear`` where the movement itself must stay proportional to time."""
    start = tracker.get_value()
    return UpdateFromAlphaFunc(tracker, lambda m, a: m.set_value(start + (to - start) * a), rate_func=rate_func)
#endregion


#region Formula panel
def _snap(mob):
    """📸 Static copy of a live readout — safe to fly into a formula slot."""
    snap = mob.copy()
    snap.clear_updaters()
    for m in snap.get_family():
        m.clear_updaters()
    return snap


def _fill_panel(scene, panel, sources, *, run_time: float = 2.0):
    """🧲 Every named formula slot is fed by the drawn quantity it stands for, then the rest of the formula appears.

    ``sources`` maps a slot key to the on-screen readout it is read from; a copy of the readout morphs into its slot.
    """
    row, box, items = panel
    targets = [items[key] for key in sources]
    rest = VGroup(*[m for m in row.submobjects if all(m is not t for t in targets)])
    for part in row.get_family():
        part._layout_zone = "formula"
    scene.play(*[TransformFromCopy(VGroup(*[_snap(m) for m in src]) if isinstance(src, (list, tuple)) else _snap(src),
                                   items[key]) for key, src in sources.items()], run_time=run_time * 0.75)
    scene.play(FadeIn(rest), Create(box), run_time=1.0)


def _clear_stage(scene, keep, run_time: float = 0.6):
    """🧹 Fade out everything but the persistent title block, stopping the live updaters first."""
    leaving = [m for m in scene.mobjects if all(m is not k for k in keep)]
    for m in leaving:
        m.clear_updaters()
    scene.play(*[FadeOut(m) for m in leaving], run_time=run_time)
#endregion
#endregion


#region Beat 1 – Leistung ist eine Rate, Energie eine Menge

class Beat1_LeistungUndEnergie(Scene):
    NARRATION = [
        ("zahlen",
         "Two numbers describe every heating system.",
         "Zwei Zahlen beschreiben jede Heizung."),
        ("fehler",
         "Mixing them up is the classic beginner's mistake.",
         "Sie zu verwechseln ist der klassische Anfängerfehler."),
        ("jahr",
         "This is one year of heating power for our building.",
         "Das ist ein Jahr Heizleistung unseres Gebäudes."),
        ("saison",
         "It is high on cold days and zero all summer.",
         "An kalten Tagen ist sie hoch, den ganzen Sommer null."),
        ("hoehe",
         "Power is the height of the curve at one moment.",
         "Leistung ist die Höhe der Kurve in einem Moment."),
        ("rate",
         "Its unit is the kilowatt: how fast energy flows right now.",
         "Ihre Einheit ist das Kilowatt: wie schnell Energie gerade fließt."),
        ("summe",
         "Energy is the running sum of power over time.",
         "Energie ist die zeitliche Summe der Leistung."),
        ("kwh",
         "That is the area under the curve, and its unit is the kilowatt-hour.",
         "Das ist die Fläche unter der Kurve. Ihre Einheit ist die Kilowattstunde."),
        ("produkt",
         "Power times time gives energy.",
         "Leistung mal Zeit ergibt Energie."),
        ("trennung",
         "That single relation from the fundamentals separates the load from the demand.",
         "Diese eine Beziehung aus den Grundlagen trennt Last von Bedarf."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Leistung ist eine Rate, Energie eine Menge", title)
        din = _din_ref("Grundlagen · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "zahlen"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ys = _plot_samples(load)
        xs = np.linspace(0.0, 1.0, len(ys))
        cum = _cumulative(load)
        q_year = round(float(cum[-1]), -2)
        year = ValueTracker(0.0)

        def p_frac():
            return float(np.interp(year.get_value(), xs, ys))

        chart = _wide_chart()
        kw_ticks = _kw_ticks(chart, PEAK_KW)
        months = _month_labels(chart)
        tank = _wide_tank()
        q_tag = math_label(r"Q\;[\mathrm{kWh}]", tank["cap"].get_top() + UP * 0.28, size=LABEL_FONT_SIZE, color=ENERGY)
        p_card = _card("Leistung", r"P\;[\mathrm{kW}]", POWER, np.array([-2.8, 0.3, 0.0]))
        q_card = _card("Energie", r"Q\;[\mathrm{kWh}]", ENERGY, np.array([2.8, 0.3, 0.0]))
        neq = math_label("≠", np.array([0.0, 0.1, 0.0]), size=FORMULA_FONT_SIZE + 12, color=P_WHITE)

        curve = always_redraw(lambda: _partial(chart, ys, year.get_value()))
        knob = always_redraw(lambda: Dot(chart["pt"](year.get_value(), p_frac()), radius=0.07, color=POWER))
        p_read = math_readout(lambda: rf"P = {de_num(p_frac() * PEAK_KW, 1)}\,\mathrm{{kW}}",
                              np.array([-2.0, 1.30, 0.0]), color=POWER)
        stem = always_redraw(lambda: DashedLine(chart["pt"](year.get_value(), 0), chart["pt"](year.get_value(), p_frac()),
                                                color=P_YELLOW, stroke_width=2.4))
        peak_i = int(np.argmax(ys))
        peak_pt = chart["pt"](peak_i / (len(ys) - 1), 1.0)
        tag_cold = math_label(r"\text{kalt: hoch}", peak_pt + RIGHT * 0.95 + DOWN * 0.1, size=LABEL_FONT_SIZE, color=POWER)
        tag_warm = math_label(r"\text{Sommer: null}", chart["pt"](0.5, 0.0) + UP * 0.45, size=LABEL_FONT_SIZE, color=P_WHITE)

        self.play(FadeIn(p_card, shift=UP * 0.2), run_time=1.4)
        self.play(FadeIn(q_card, shift=UP * 0.2), run_time=1.4)
        hold_for(self, N, "zahlen", used=0.3 + 1.4 + 1.4)

        caption = swap_caption(self, caption, subtitle_text(N, "fehler"))
        self.play(FadeIn(neq, scale=1.6), run_time=1.2)
        self.play(Indicate(p_card, color=POWER, scale_factor=1.08), Indicate(q_card, color=ENERGY, scale_factor=1.08),
                  run_time=1.4)
        hold_for(self, N, "fehler")
        _hide(self, p_card, q_card, neq)

        caption = swap_caption(self, caption, subtitle_text(N, "jahr"))
        self.play(Create(chart["group"]), FadeIn(kw_ticks), FadeIn(months), run_time=1.6)
        self.add(curve, knob)
        self.play(_ramp(year, 1.0, rate_func=linear), run_time=6.0)
        hold_for(self, N, "jahr")

        caption = swap_caption(self, caption, subtitle_text(N, "saison"))
        self.play(FadeIn(tag_cold, shift=LEFT * 0.2), Indicate(knob, color=POWER, scale_factor=2.4), run_time=1.4)
        self.play(FadeIn(tag_warm, shift=UP * 0.2), run_time=1.4)
        hold_for(self, N, "saison")
        _hide(self, tag_cold, tag_warm)

        caption = swap_caption(self, caption, subtitle_text(N, "hoehe"))
        self.add(stem)
        self.play(_ramp(year, 0.06), FadeIn(p_read), run_time=2.4)
        self.play(Indicate(stem, color=P_YELLOW, scale_factor=1.0), run_time=1.2)
        self.play(_ramp(year, 0.94), run_time=3.0)
        hold_for(self, N, "hoehe")

        caption = swap_caption(self, caption, subtitle_text(N, "rate"))
        rate_path = smooth_path([np.array([-1.7, 0.15, 0.0]), np.array([-0.6, 0.22, 0.0]), np.array([0.6, 0.08, 0.0]),
                                 np.array([1.7, 0.15, 0.0])])
        rate_guides = flow_guides([rate_path], POWER, opacity=0.3)
        self.play(FadeIn(rate_guides), run_time=0.8)
        self.play(_ramp(year, 0.06), _rate_stream([rate_path], p_frac, 3.0, per_path=14, speed=0.5), run_time=3.0)
        self.play(_ramp(year, 0.94), _rate_stream([rate_path], p_frac, 3.0, per_path=14, speed=0.5), run_time=3.0)
        hold_for(self, N, "rate", during=lambda rt: [_rate_stream([rate_path], p_frac, rt, per_path=14, speed=0.5)])
        _hide(self, rate_guides, stem, p_read)

        caption = swap_caption(self, caption, subtitle_text(N, "summe"))
        area = always_redraw(lambda: _partial_area(chart, ys, year.get_value()))
        q_read = math_readout(lambda: rf"Q = {de_num(round(_energy_at(cum, year.get_value()), -2))}\,\mathrm{{kWh}}",
                              np.array([-2.0, 1.30, 0.0]), color=ENERGY)
        tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, year.get_value()) / cum[-1]))
        self.add(area, tank["level"])
        self.play(_ramp(year, 0.0), FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(q_tag), FadeIn(q_read),
                  run_time=1.6)
        self.play(_ramp(year, 1.0, rate_func=linear), run_time=7.0)
        hold_for(self, N, "summe")

        caption = swap_caption(self, caption, subtitle_text(N, "kwh"))
        q_final = math_label(rf"Q \approx {de_num(q_year)}\,\mathrm{{kWh/a}}", np.array([-2.0, 1.30, 0.0]),
                             size=BODY_FONT_SIZE, color=ENERGY, edge="left")
        self.play(ReplacementTransform(q_read, q_final), run_time=1.2)
        self.play(Indicate(q_final, color=ENERGY, scale_factor=1.15), run_time=1.4)
        hold_for(self, N, "kwh")
        _hide(self, tank["group"], tank["fill"], q_tag, knob)

        caption = swap_caption(self, caption, subtitle_text(N, "produkt"))
        cell = Rectangle(width=0.95, height=0.62, color=ENERGY, stroke_width=2, fill_color=ENERGY, fill_opacity=0.30)
        cell.move_to(np.array([2.3, 1.25, 0.0]))
        cell_h = math_label(r"1\,\mathrm{h}", cell.get_top() + UP * 0.30, size=LABEL_FONT_SIZE, color=TIME)
        cell_p = math_label(r"1\,\mathrm{kW}", cell.get_left() + LEFT * 0.12 + DOWN * 0.07, size=LABEL_FONT_SIZE,
                            color=POWER, edge="right")
        cell_q = math_label(r"1\,\mathrm{kWh}", cell.get_center() + DOWN * 0.07, size=LABEL_FONT_SIZE, color=ENERGY)
        p_now = math_readout(lambda: rf"P = {de_num(p_frac() * PEAK_KW, 1)}\,\mathrm{{kW}}", np.array([-2.0, 0.80, 0.0]),
                             color=POWER)
        self.play(FadeIn(cell), FadeIn(cell_h), run_time=1.2)
        self.play(FadeIn(cell_p), FadeIn(p_now), run_time=1.2)
        self.play(FadeIn(cell_q), run_time=1.0)
        panel = math_panel([
            ("q", "Q", ENERGY), (None, "=", P_WHITE), (None, r"\Sigma", P_WHITE), ("p", "P_{i}", POWER),
            (None, r"\cdot", P_WHITE), ("dt", r"\Delta t", TIME), (None, r"\quad", P_WHITE),
            ("u1", r"1\,\mathrm{kW}", POWER), (None, r"\cdot", P_WHITE), ("u2", r"1\,\mathrm{h}", TIME),
            (None, "=", P_WHITE), ("u3", r"1\,\mathrm{kWh}", ENERGY),
        ])
        _fill_panel(self, panel, {"q": q_final, "p": p_now, "dt": cell_h, "u1": cell_p, "u2": cell_h, "u3": cell_q})
        hold_for(self, N, "produkt")

        caption = swap_caption(self, caption, subtitle_text(N, "trennung"))
        tag_load = math_label(r"\text{Last}", peak_pt + RIGHT * 0.5 + UP * 0.1, size=LABEL_FONT_SIZE, color=POWER)
        tag_demand = math_label(r"\text{Bedarf}", chart["pt"](0.05, 0.12), size=LABEL_FONT_SIZE, color=ENERGY,
                                edge="left")
        self.play(Create(highlight_param(panel[2], "p", color=POWER)), FadeIn(tag_load), run_time=1.4)
        self.play(Create(highlight_param(panel[2], "q", color=ENERGY)), FadeIn(tag_demand), run_time=1.4)
        hold_for(self, N, "trennung")

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion


#region Beat 2 – Die Normheizlast nach DIN EN 12831-1

def _lvl(temp: float) -> float:
    """🌡️ Thermometer fill for a temperature on the −15…25 °C scale shared by both glyphs."""
    return float(np.clip((temp + 15.0) / 40.0, 0.0, 1.0))


def _signed(value: float, digits: int = 0) -> str:
    """➖ German signed number for ``math_text`` — a real minus sign for negatives."""
    return ("−" if value < 0 else "") + de_num(abs(value), digits)


def _fact(name: str, value):
    """📋 One line of the standard's identity card — a label and its typeset or plain value."""
    return VGroup(body_text(name, font_size=LABEL_FONT_SIZE, color=P_TEAL), value).arrange(RIGHT, buff=0.2)


class Beat2_Normheizlast(Scene):
    NARRATION = [
        ("spitze",
         "The peak of that curve has a name of its own: the design heat load.",
         "Die Spitze dieser Kurve hat einen eigenen Namen: die Norm-Heizlast."),
        ("regeln",
         "It comes with its own standard, its own symbol and its own rules.",
         "Dazu gehören eine eigene Norm, ein eigenes Formelzeichen, eigene Regeln."),
        ("extremfall",
         "It is computed for one artificial worst case.",
         "Sie wird für einen künstlichen Extremfall berechnet."),
        ("normtemperatur",
         "What counts is the design outdoor temperature of the site, which sets the design difference.",
         "Es gilt die Norm-Außentemperatur des Standorts — sie legt die Auslegungsdifferenz fest."),
        ("raumweise",
         "The calculation is done room by room.",
         "Gerechnet wird Raum für Raum."),
        ("verluste",
         "Crucially, only the losses under these design conditions count.",
         "Entscheidend: Nur die Verluste unter diesen Normbedingungen zählen."),
        ("solar",
         "Solar gains are not credited as reliable heating power.",
         "Solare Gewinne gelten nicht als verlässliche Heizleistung."),
        ("intern",
         "Internal gains are not credited either.",
         "Interne Gewinne werden ebenfalls nicht angerechnet."),
        ("transmission",
         "As a schematic simplification, the envelope contributes the transmission heat transfer coefficient.",
         "Schematisch vereinfacht liefert die Hülle den Transmissions-Wärmetransferkoeffizienten."),
        ("lueftung",
         "The air exchange contributes the ventilation heat transfer coefficient.",
         "Der Luftaustausch liefert den Lüftungs-Wärmetransferkoeffizienten."),
        ("summe",
         "Both together times the design difference give the design heat load.",
         "Beide zusammen mal der Auslegungsdifferenz ergeben die Norm-Heizlast."),
        ("kilowatt",
         "The result is kilowatts, not kilowatt-hours.",
         "Das Ergebnis sind Kilowatt, nicht Kilowattstunden."),
        ("auslegung",
         "It is an important basis for sizing the whole heating system.",
         "Das ist eine wichtige Grundlage für die Auslegung der Heizungsanlage."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        H_T_VAL, H_V_VAL = 225.0, 87.5

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Norm-Heizlast nach DIN EN 12831-1", title)
        din = _din_ref("DIN EN 12831-1 · DIN/TS 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "spitze"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ys = _plot_samples(load)
        chart = _wide_chart()
        kw_ticks = _kw_ticks(chart, PEAK_KW)
        months = _month_labels(chart)
        peak_i = int(np.argmax(ys))
        peak_x = peak_i / (len(ys) - 1)
        year_curve = _partial(chart, ys, 1.0)
        peak_dot = Dot(chart["pt"](peak_x, 1.0), radius=0.08, color=POWER)
        peak_name = VGroup(math_label(r"\Phi_{\mathrm{HL}}", size=BODY_FONT_SIZE, color=POWER),
                           body_text("Norm-Heizlast", font_size=LABEL_FONT_SIZE, color=POWER)).arrange(RIGHT, buff=0.18)
        peak_name.next_to(peak_dot, LEFT if peak_x > 0.5 else RIGHT, buff=0.25).shift(UP * 0.12)
        facts = VGroup(
            _fact("Norm:", body_text("DIN EN 12831-1", font_size=LABEL_FONT_SIZE, color=P_WHITE)),
            _fact("Formelzeichen:", math_label(r"\Phi_{\mathrm{HL}}", size=LABEL_FONT_SIZE, color=POWER)),
            _fact("Regeln:", body_text("Normbedingungen", font_size=LABEL_FONT_SIZE, color=P_WHITE)),
        ).arrange(DOWN, buff=0.3, aligned_edge=LEFT).move_to(np.array([0.0, 1.0, 0.0]))

        scale = 0.9
        house = _heated_house(np.array([-1.9, BASE_Y + 1.2 * scale, 0.0]), scale)
        hs, storey = house["hs"], house["storey"]
        floor0, floor1 = house["floors"]
        walls_t = [house["wall_paths"][1], house["roof_path"]]
        walls_v = house["win_paths"]
        every = [*walls_v, *walls_t]
        guides = flow_guides(every, POWER, opacity=0.22)

        theta_e = ValueTracker(8.0)
        d_theta = lambda: THETA_I - theta_e.get_value()
        phi_kw = lambda: d_theta() / DESIGN_DT * PEAK_KW
        level = lambda: d_theta() / DESIGN_DT

        bulb_y = floor0 + 0.17
        th_h = 0.9
        th_i = thermometer_glyph(np.array([hs["bottom_right"][0] - 0.38, bulb_y, 0.0]), height=th_h, color=P_RED,
                                 level=_lvl(THETA_I))
        th_e = thermometer_glyph(np.array([hs["bottom_right"][0] + 0.38, bulb_y, 0.0]), height=th_h, color=P_BLUE,
                                 level=_lvl(theta_e.get_value()))
        th_e["level"].add_updater(lambda m: m.set_value(_lvl(theta_e.get_value())))
        top = lambda temp: bulb_y + 0.05 + (th_h - 0.12) * _lvl(temp)
        arrow_x = hs["bottom_right"][0] + 0.95
        row_x = arrow_x + 0.3
        guide_i = always_redraw(lambda: DashedLine(np.array([hs["bottom_right"][0] - 0.28, top(THETA_I), 0.0]),
                                                   np.array([arrow_x, top(THETA_I), 0.0]), color=P_YELLOW,
                                                   stroke_width=1.4, stroke_opacity=0.6))
        guide_e = always_redraw(lambda: DashedLine(np.array([hs["bottom_right"][0] + 0.5, top(theta_e.get_value()), 0.0]),
                                                   np.array([arrow_x, top(theta_e.get_value()), 0.0]), color=P_YELLOW,
                                                   stroke_width=1.4, stroke_opacity=0.6))
        span = always_redraw(lambda: dim_arrow(np.array([arrow_x, top(theta_e.get_value()), 0.0]),
                                               np.array([arrow_x, top(THETA_I), 0.0]), color=DELTA_T))
        row_i = math_readout(lambda: rf"\theta_{{\mathrm{{int}}}} = {de_num(THETA_I)}\,\mathrm{{°C}}",
                             np.array([row_x, -0.12, 0.0]), size=LABEL_FONT_SIZE, color=P_RED)
        row_e = math_readout(lambda: rf"\theta_{{\mathrm{{e}}}} = {_signed(theta_e.get_value())}\,\mathrm{{°C}}",
                             np.array([row_x, -0.52, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE)
        row_d = math_readout(lambda: rf"\Delta\theta = {de_num(d_theta())}\,\mathrm{{K}}",
                             np.array([row_x, -0.97, 0.0]), size=BODY_FONT_SIZE, color=DELTA_T)
        phi_read = math_readout(lambda: rf"\Phi_{{\mathrm{{HL}}}} = {de_num(phi_kw(), 1)}\,\mathrm{{kW}}",
                                np.array([row_x, 0.55, 0.0]), size=BODY_FONT_SIZE, color=POWER)
        peak_line = always_redraw(lambda: DashedLine(chart["pt"](0, min(level(), 1.0)), chart["pt"](1, min(level(), 1.0)),
                                                     color=POWER, stroke_width=2.0, stroke_opacity=0.85))

        self.play(Create(chart["group"]), FadeIn(kw_ticks), FadeIn(months), run_time=1.4)
        self.play(Create(year_curve), run_time=2.4)
        self.play(FadeIn(peak_dot, scale=2.0), FadeIn(peak_name), Indicate(peak_dot, color=POWER, scale_factor=1.8),
                  run_time=1.4)
        hold_for(self, N, "spitze", used=0.3 + 1.4 + 2.4 + 1.4)

        caption = swap_caption(self, caption, subtitle_text(N, "regeln"))
        for fact in facts:
            self.play(FadeIn(fact, shift=RIGHT * 0.2), run_time=1.3)
        hold_for(self, N, "regeln")
        _hide(self, chart["group"], kw_ticks, months, year_curve, peak_dot, peak_name, facts)

        caption = swap_caption(self, caption, subtitle_text(N, "extremfall"))
        self.play(Create(hs["group"]), run_time=1.6)
        self.play(FadeIn(house["rads"]), FadeIn(guides), run_time=0.8)
        self.add(th_i["column"], th_e["column"])
        self.play(FadeIn(th_i["group"]), FadeIn(row_i), *_life(house, level, 1.4, paths=every), run_time=1.4)
        self.play(FadeIn(th_e["group"]), FadeIn(row_e), *_life(house, level, 1.4, paths=every), run_time=1.4)
        self.play(_ramp(theta_e, THETA_E), *_life(house, level, 4.0, paths=every, r_max=0.45), run_time=4.0)
        hold_for(self, N, "extremfall", during=lambda rt: _life(house, level, rt, paths=every, r_max=0.45))

        caption = swap_caption(self, caption, subtitle_text(N, "normtemperatur"))
        self.play(Indicate(row_e, color=P_BLUE, scale_factor=1.15), run_time=1.4)
        for live in (guide_i, guide_e, span, row_d, phi_read):
            live.update()
        self.play(FadeIn(guide_i), FadeIn(guide_e), FadeIn(span), FadeIn(row_d), run_time=1.6)
        self.play(FadeIn(phi_read), run_time=1.4)
        hold_for(self, N, "normtemperatur")

        caption = swap_caption(self, caption, subtitle_text(N, "raumweise"))
        rooms = [Rectangle(width=hs["w_width"] - 0.2, height=storey - 0.06, color=P_TEAL, stroke_width=2.6)
                 .move_to(np.array([hs["center"][0], fy + storey / 2, 0.0])) for fy in (floor0, floor1)]
        self.play(Create(rooms[0]), run_time=1.6)
        self.play(ReplacementTransform(rooms[0], rooms[1]), run_time=1.6)
        self.play(FadeOut(rooms[1]), run_time=0.6)
        hold_for(self, N, "raumweise")
        _hide(self, th_i["group"], th_i["column"], th_e["group"], th_e["column"], row_i, row_e, row_d, phi_read,
              guide_i, guide_e, span)

        caption = swap_caption(self, caption, subtitle_text(N, "verluste"))
        loss_lbl = body_text("nur Verluste zählen", font_size=LABEL_FONT_SIZE, color=P_RED)
        loss_lbl.move_to(np.array([-5.6, 1.3, 0.0]))
        self.play(FadeIn(loss_lbl), *_life(house, level, 1.4, paths=every, r_max=0.45), run_time=1.4)
        hold_for(self, N, "verluste", during=lambda rt: _life(house, level, rt, paths=every, r_max=0.45))
        _hide(self, loss_lbl)

        caption = swap_caption(self, caption, subtitle_text(N, "solar"))
        sun = sun_glyph(np.array([-5.7, 1.3, 0.0])).scale(0.6)
        up_win = hs["windows"][1]
        rays = sun_rays(sun.get_center(), float(up_win["x"]),
                        [up_win["lintel"] - 0.08, (up_win["sill"] + up_win["lintel"]) / 2, up_win["sill"] + 0.08],
                        floor1 + 0.02, gap=0.55)
        ray_paths = [[s, h, l] for s, h, l in zip(rays["starts"], rays["hits"], rays["lands"])]
        sun_lbl = VGroup(body_text("solare Gewinne", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
                         cross_mark(size=0.11)).arrange(RIGHT, buff=0.14)
        sun_lbl.move_to(np.array([-5.6, 0.2, 0.0]))
        self.play(FadeIn(sun), run_time=1.0)
        self.play(shine(rays), run_time=1.8)
        self.play(FadeIn(sun_lbl), pulse_flashes(ray_paths, P_YELLOW, repeats=2, width=4.0), run_time=1.6)
        hold_for(self, N, "solar", during=lambda rt: [pulse_flashes(ray_paths, P_YELLOW, repeats=2, width=4.0)])
        _hide(self, sun, rays["group"], sun_lbl)

        caption = swap_caption(self, caption, subtitle_text(N, "intern"))
        person = _stand(floor1, hs["bottom_right"][0] - 0.65, storey)
        lamp = lamp_glyph(np.array([hs["center"][0] - 0.1, float(hs["top_left"][1]), 0.0]), drop=0.1)
        lamp["group"].scale(0.6, about_point=np.array([hs["center"][0] - 0.1, float(hs["top_left"][1]), 0.0]))
        gain_spots = [person.get_top() + UP * 0.04]
        lamp_spot = [lamp["bulb"].get_center() + DOWN * 0.1]
        int_lbl = VGroup(body_text("interne Gewinne", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
                         cross_mark(size=0.11)).arrange(RIGHT, buff=0.14)
        int_lbl.move_to(np.array([-5.6, 0.2, 0.0]))

        def internal(rt):
            return [ripples(gain_spots, r_max=0.4, cycles=rt / 1.4, color=P_ORANGE),
                    ripples(lamp_spot, r_max=0.35, cycles=rt / 1.4, color=P_ORANGE, down=True)]

        self.play(FadeIn(person), run_time=1.2)
        self.play(FadeIn(lamp["group"]), run_time=1.2)
        self.play(FadeIn(int_lbl), *internal(1.6), run_time=1.6)
        hold_for(self, N, "intern", during=internal)
        _hide(self, person, lamp["group"], int_lbl)

        caption = swap_caption(self, caption, subtitle_text(N, "transmission"))
        t_lbl = math_label(rf"H_{{T}} = {de_num(H_T_VAL)}\,\mathrm{{W/K}}", np.array([-0.85, 1.72, 0.0]),
                           size=LABEL_FONT_SIZE, color=H_T, edge="left")
        v_lbl = math_label(rf"H_{{V}} = {de_num(H_V_VAL, 1)}\,\mathrm{{W/K}}", np.array([-6.2, -0.05, 0.0]),
                           size=LABEL_FONT_SIZE, color=H_V, edge="left")
        note = math_label(r"\text{schematisch vereinfacht — die Norm enthält weitere Terme}",
                          np.array([0.0, NOTE_BAND_Y - 0.07, 0.0]), size=LABEL_FONT_SIZE, color=P_TEAL)
        guides_t = flow_guides(walls_t, H_T, opacity=0.3)
        guides_v = flow_guides(walls_v, H_V, opacity=0.3)

        def stream_t(rt):
            return [_rate_stream(walls_t, level, rt, color=H_T, color_end=H_T), _heat(house["spots"], rt, r_max=0.4)]

        def stream_v(rt):
            return [_rate_stream(walls_v, level, rt, color=H_V, color_end=H_V), _heat(house["spots"], rt, r_max=0.4)]

        self.play(FadeOut(guides), FadeIn(guides_t), run_time=1.0)
        self.play(FadeIn(t_lbl), *stream_t(1.6), run_time=1.6)
        hold_for(self, N, "transmission", during=stream_t)

        caption = swap_caption(self, caption, subtitle_text(N, "lueftung"))
        self.play(FadeIn(guides_v), run_time=1.0)
        self.play(FadeIn(v_lbl), *stream_v(1.6), run_time=1.6)
        hold_for(self, N, "lueftung", during=stream_v)

        caption = swap_caption(self, caption, subtitle_text(N, "summe"))
        row_d = math_label(rf"\Delta\theta = {de_num(DESIGN_DT)}\,\mathrm{{K}}", np.array([row_x, -0.97, 0.0]),
                           size=BODY_FONT_SIZE, color=DELTA_T, edge="left")
        phi_read = math_label(rf"\Phi_{{\mathrm{{HL}}}} = {de_num(PEAK_KW, 1)}\,\mathrm{{kW}}", np.array([row_x, 0.55, 0.0]),
                              size=BODY_FONT_SIZE, color=POWER, edge="left")
        self.play(FadeIn(row_d), FadeIn(phi_read), FadeIn(note), run_time=1.2)
        panel = math_panel([
            (None, r"\Phi_{\mathrm{HL}}", POWER), (None, "=", P_WHITE), (None, "(", P_WHITE),
            (None, "H_{T}", H_T), (None, "+", P_WHITE), (None, "H_{V}", H_V), (None, ")", P_WHITE),
            (None, r"\cdot", P_WHITE), (None, r"\Delta\theta", DELTA_T), (None, "=", P_WHITE),
            (None, "(", P_WHITE), ("t", de_num(H_T_VAL), H_T), (None, "+", P_WHITE), ("v", de_num(H_V_VAL, 1), H_V),
            (None, ")", P_WHITE), (None, r"\mathrm{W/K}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("d", rf"{de_num(DESIGN_DT)}\,\mathrm{{K}}", DELTA_T), (None, "=", P_WHITE),
            ("phi", rf"{de_num(PEAK_KW)}\,\mathrm{{kW}}", POWER),
        ], size=FORMULA_FONT_SIZE - 5)
        _fill_panel(self, panel, {"t": t_lbl, "v": v_lbl, "d": row_d, "phi": phi_read})
        hold_for(self, N, "summe")

        caption = swap_caption(self, caption, subtitle_text(N, "kilowatt"))
        area = _partial_area(chart, ys, 1.0)
        kwh_tag = VGroup(math_label(r"\text{Fläche} = Q_h\;[\mathrm{kWh/a}]", size=LABEL_FONT_SIZE, color=ENERGY),
                         cross_mark(size=0.11)).arrange(RIGHT, buff=0.2)
        kwh_tag.move_to(chart["pt"](0.5, 0.55))
        ring = highlight_param(panel[2], "phi", color=POWER)
        _hide(self, hs["group"], house["rads"], guides_t, guides_v, t_lbl, v_lbl, note, row_d, phi_read)
        self.play(Create(chart["group"]), FadeIn(kw_ticks), Create(year_curve), run_time=1.6)
        self.play(Create(peak_line), Create(ring), run_time=1.4)
        self.play(FadeIn(area), FadeIn(kwh_tag), run_time=1.4)
        hold_for(self, N, "kilowatt")

        caption = swap_caption(self, caption, subtitle_text(N, "auslegung"))
        kw_tag = math_label(r"\text{Anlagengröße}", chart["pt"](0.5, 0.86), size=LABEL_FONT_SIZE, color=POWER)
        self.play(FadeOut(area), FadeOut(kwh_tag), run_time=0.8)
        self.play(FadeIn(kw_tag), Indicate(ring, color=POWER, scale_factor=1.1), run_time=1.4)
        hold_for(self, N, "auslegung")

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion


#region Beat 3 – Die Jahresdauerlinie

class Beat3_Jahresdauerlinie(Scene):
    NARRATION = [
        ("kurve",
         "Load and demand live on the same curve.",
         "Last und Bedarf stecken in derselben Kurve."),
        ("bild",
         "One picture shows both at once — sort the year by size.",
         "Ein Bild zeigt beides — man sortiert das Jahr nach Größe."),
        ("aufreihen",
         "Take every hour of the year and line them up from the coldest to the mildest.",
         "Man reiht jede Stunde des Jahres von der kältesten bis zur mildesten auf."),
        ("dauerlinie",
         "That is the load duration curve.",
         "Das ist die Jahresdauerlinie."),
        ("hoehe",
         "Its starting height is the design heat load.",
         "Ihre Anfangshöhe ist die Norm-Heizlast."),
        ("wenige",
         "These are the few hours the plant must still be able to cover.",
         "Das sind die wenigen Stunden, die die Anlage noch abdecken muss."),
        ("flaeche_bedarf",
         "Its area is the annual heating demand.",
         "Ihre Fläche ist der Jahres-Heizwärmebedarf."),
        ("nutzenergie",
         "It is useful energy, not the billed consumption.",
         "Es ist Nutzenergie, nicht der abgerechnete Verbrauch."),
        ("rechteck",
         "The same area fits into a rectangle as tall as the peak.",
         "Dieselbe Fläche passt in ein Rechteck, das so hoch ist wie die Spitze."),
        ("vollstunden",
         "Area divided by peak gives the full-load hours.",
         "Fläche geteilt durch Spitze ergibt die Vollbenutzungsstunden."),
        ("bruchteil",
         "A building runs at full load only a fraction of the year.",
         "Volllast läuft ein Gebäude nur einen Bruchteil des Jahres."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Jahresdauerlinie", title)
        din = _din_ref("DIN EN 12831-1 · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "kurve"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ordered = np.sort(load)[::-1]
        ys_year, ys_sorted = _plot_samples(load), _plot_samples(ordered)
        xs = np.linspace(0.0, 1.0, PLOT_N)
        cum = _cumulative(ordered)
        q_year = round(float(cum[-1]), -2)
        t_vl = q_year / PEAK_KW
        full = t_vl / PROFILE_N
        few = 0.05
        series = {"ys": ys_year}
        x = ValueTracker(0.0)

        def p_frac():
            return float(np.interp(x.get_value(), xs, series["ys"]))

        chart = _wide_chart()
        kw_ticks = _kw_ticks(chart, PEAK_KW)
        months = _month_labels(chart)
        hours = _hour_labels(chart)
        tank = _wide_tank()
        q_tag = math_label(r"Q_{h}\;[\mathrm{kWh}]", tank["cap"].get_top() + UP * 0.28, size=LABEL_FONT_SIZE, color=ENERGY)

        curve = always_redraw(lambda: _partial(chart, ys_year, x.get_value()))
        knob = always_redraw(lambda: Dot(chart["pt"](x.get_value(), p_frac()), radius=0.07, color=POWER))
        peak_i = int(np.argmax(ys_year))
        tag_load = math_label(r"\text{Last}", chart["pt"](peak_i / (PLOT_N - 1), 1.0) + RIGHT * 0.5 + UP * 0.1,
                              size=LABEL_FONT_SIZE, color=POWER)
        tag_demand = math_label(r"\text{Bedarf}", chart["pt"](0.04, 0.12), size=LABEL_FONT_SIZE, color=ENERGY,
                                edge="left")
        tag_curve = math_label(r"\text{Jahresdauerlinie}", chart["pt"](0.5, 0.45), size=BODY_FONT_SIZE, color=P_WHITE)

        self.play(Create(chart["group"]), FadeIn(kw_ticks), FadeIn(months), run_time=1.6)
        self.add(curve, knob)
        self.play(_ramp(x, 1.0, rate_func=linear), run_time=4.0)
        curve.clear_updaters()
        area = _partial_area(chart, ys_year, 1.0, opacity=0.14)
        self.play(FadeOut(knob), FadeIn(tag_load), run_time=1.2)
        self.play(FadeIn(area), FadeIn(tag_demand), run_time=1.4)
        hold_for(self, N, "kurve", used=0.3 + 1.6 + 4.0 + 1.2 + 1.4)

        caption = swap_caption(self, caption, subtitle_text(N, "bild"))
        self.play(Indicate(tag_load, color=POWER, scale_factor=1.3), run_time=1.4)
        self.play(Indicate(tag_demand, color=ENERGY, scale_factor=1.3), run_time=1.4)
        hold_for(self, N, "bild")
        _hide(self, tag_load, tag_demand)

        caption = swap_caption(self, caption, subtitle_text(N, "aufreihen"))
        sorted_curve = _partial(chart, ys_sorted, 1.0, width=3.0)
        sorted_area = _partial_area(chart, ys_sorted, 1.0, opacity=0.14)
        self.play(Transform(curve, sorted_curve), Transform(area, sorted_area), FadeOut(months), FadeIn(hours),
                  run_time=4.0)
        series["ys"] = ys_sorted
        x.set_value(0.0)
        knob = always_redraw(lambda: Dot(chart["pt"](x.get_value(), p_frac()), radius=0.07, color=POWER))
        self.add(knob)
        self.play(_ramp(x, 1.0, rate_func=linear), run_time=4.0)
        hold_for(self, N, "aufreihen")

        caption = swap_caption(self, caption, subtitle_text(N, "dauerlinie"))
        self.play(FadeIn(tag_curve, shift=UP * 0.2), _ramp(x, 0.0), run_time=1.6)
        hold_for(self, N, "dauerlinie")
        _hide(self, tag_curve)

        caption = swap_caption(self, caption, subtitle_text(N, "hoehe"))
        peak_line = DashedLine(chart["pt"](0, 1.0), chart["pt"](1, 1.0), color=POWER, stroke_width=2.2, stroke_opacity=0.8)
        phi_read = math_label(rf"\Phi_{{\mathrm{{HL}}}} = {de_num(PEAK_KW, 1)}\,\mathrm{{kW}}", np.array([0.2, 1.62, 0.0]),
                              size=BODY_FONT_SIZE, color=POWER, edge="left")
        self.play(Create(peak_line), run_time=1.6)
        self.play(FadeIn(phi_read, shift=UP * 0.2), Indicate(knob, color=POWER, scale_factor=2.4), run_time=1.4)
        hold_for(self, N, "hoehe")

        caption = swap_caption(self, caption, subtitle_text(N, "wenige"))
        sliver = _partial_area(chart, ys_sorted, few, color=P_RED, opacity=0.4)
        tag_few = math_label(r"\text{wenige Stunden}", chart["pt"](few + 0.03, 0.88), size=LABEL_FONT_SIZE, color=P_RED,
                             edge="left")
        self.play(FadeIn(sliver), run_time=1.4)
        self.play(FadeIn(tag_few, shift=LEFT * 0.2), run_time=1.4)
        hold_for(self, N, "wenige")
        _hide(self, sliver, tag_few, peak_line, knob)

        caption = swap_caption(self, caption, subtitle_text(N, "flaeche_bedarf"))
        x.set_value(0.0)
        sweep = always_redraw(lambda: _partial_area(chart, ys_sorted, x.get_value()))
        q_read = math_readout(lambda: rf"Q_{{h}} = {de_num(round(_energy_at(cum, x.get_value()), -2))}\,\mathrm{{kWh}}",
                              np.array([0.2, 1.08, 0.0]), size=BODY_FONT_SIZE, color=ENERGY)
        tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, x.get_value()) / cum[-1]))
        self.add(sweep, tank["level"])
        self.play(FadeOut(area), FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(q_tag), FadeIn(q_read), run_time=1.4)
        self.play(_ramp(x, 1.0, rate_func=linear), run_time=6.0)
        q_final = math_label(rf"Q_{{h}} \approx {de_num(q_year)}\,\mathrm{{kWh/a}}", np.array([0.2, 1.08, 0.0]),
                             size=BODY_FONT_SIZE, color=ENERGY, edge="left")
        self.play(ReplacementTransform(q_read, q_final), run_time=1.0)
        hold_for(self, N, "flaeche_bedarf")

        caption = swap_caption(self, caption, subtitle_text(N, "nutzenergie"))
        use = body_text("Nutzenergie", font_size=LABEL_FONT_SIZE, color=ENERGY)
        billed = VGroup(body_text("abgerechneter Verbrauch", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
                        cross_mark(size=0.11)).arrange(RIGHT, buff=0.2)
        rows = VGroup(use, billed).arrange(DOWN, buff=0.3, aligned_edge=LEFT).move_to(np.array([1.9, 0.35, 0.0]))
        self.play(FadeIn(use, shift=RIGHT * 0.2), run_time=1.4)
        self.play(FadeIn(billed, shift=RIGHT * 0.2), run_time=1.4)
        hold_for(self, N, "nutzenergie")
        _hide(self, rows, tank["group"], tank["fill"], q_tag)

        caption = swap_caption(self, caption, subtitle_text(N, "rechteck"))
        rect = Polygon(*[chart["pt"](full * i / (PLOT_N - 1), 1.0) for i in range(PLOT_N)], chart["pt"](full, 0), chart["pt"](0, 0),
                       fill_color=P_YELLOW, fill_opacity=0.24, stroke_color=P_YELLOW, stroke_width=2.2)
        sweep.clear_updaters()
        run = ValueTracker(0.0)
        t_read = math_readout(lambda: rf"t_{{VL}} = {de_num(min(run.get_value(), full) * PROFILE_N, 0)}\,\mathrm{{h}}",
                              np.array([0.2, 0.55, 0.0]), size=BODY_FONT_SIZE, color=TIME)
        self.add(run)
        self.play(ReplacementTransform(sweep, rect), FadeIn(phi_read), FadeIn(t_read), run_time=2.0)
        self.play(_ramp(run, 1.0, rate_func=linear), run_time=3.0)
        hold_for(self, N, "rechteck")

        caption = swap_caption(self, caption, subtitle_text(N, "vollstunden"))
        panel = math_panel([
            ("t", r"t_{VL}", TIME), (None, "=", P_WHITE),
            (None, r"\frac{Q_{h}}{\Phi_{\mathrm{HL}}}", P_WHITE), (None, "=", P_WHITE),
            ("f", rf"\frac{{\textcolor{{{ENERGY}}}{{{de_num(q_year)}\,\mathrm{{kWh/a}}}}}}"
                  rf"{{\textcolor{{{POWER}}}{{{de_num(PEAK_KW)}\,\mathrm{{kW}}}}}}", P_WHITE),
            (None, "=", P_WHITE), ("res", rf"{de_num(t_vl)}\,\mathrm{{h/a}}", TIME),
        ], size=FORMULA_FONT_SIZE - 5)
        _fill_panel(self, panel, {"f": [q_final, phi_read], "res": t_read, "t": t_read})
        hold_for(self, N, "vollstunden")

        caption = swap_caption(self, caption, subtitle_text(N, "bruchteil"))
        share = math_label(rf"\approx {de_num(100 * t_vl / PROFILE_N)}\,\%\;\text{{des Jahres}}",
                           np.array([0.2, 0.0, 0.0]), size=BODY_FONT_SIZE, color=TIME, edge="left")
        self.play(FadeIn(share, shift=UP * 0.2), Indicate(rect, color=P_YELLOW, scale_factor=1.04), run_time=1.6)
        hold_for(self, N, "bruchteil")

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion


#region Beat 4 – Auslegen auf die Spitze, nicht auf den Mittelwert

class Beat4_RichtigAuslegen(Scene):
    NARRATION = [
        ("kurve",
         "Here is our load duration curve again.",
         "Hier ist wieder unsere Jahresdauerlinie."),
        ("falsch",
         "On that same curve you can see, directly, what happens when a plant is sized wrongly.",
         "An derselben Kurve sieht man unmittelbar, was passiert, wenn eine Anlage falsch ausgelegt wird."),
        ("mittelwert",
         "Size the plant on the average load and it covers only part of the curve.",
         "Legt man auf den Mittelwert aus, deckt die Anlage nur einen Teil der Kurve."),
        ("defizit",
         "At high load a power deficit remains.",
         "Bei hoher Last bleibt ein Leistungsdefizit."),
        ("temperatur",
         "Without reserve capacity the indoor temperature can no longer be held.",
         "Ohne Reserve lässt sich die Raumtemperatur dann nicht halten."),
        ("reserve",
         "Size the plant far above the peak and a large reserve appears.",
         "Legt man die Anlage weit über die Spitze aus, entsteht eine große Reserve."),
        ("ueberdimensioniert",
         "Almost all year that reserve stays unused — the plant is oversized.",
         "Fast das ganze Jahr bleibt diese Reserve ungenutzt — die Anlage ist überdimensioniert."),
        ("takten",
         "At too high a minimum output that means more cycling.",
         "Bei zu hoher Mindestleistung führt das zu häufigerem Takten."),
        ("auslegungspunkt",
         "The design heat load gives the central sizing point.",
         "Die Norm-Heizlast liefert den zentralen Auslegungspunkt."),
        ("anlage",
         "The real plant also depends on the heat generator, controls and storage.",
         "Die reale Anlage hängt zusätzlich von Erzeuger, Regelung und Speicher ab."),
        ("bedarf",
         "The area under the curve is the useful heating demand.",
         "Die Fläche darunter ist der Heizwärmebedarf."),
        ("kosten",
         "The plant turns that into final energy, and only that creates cost.",
         "Das Heizsystem macht daraus Endenergie, erst daraus entstehen Kosten."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        PEAK_F = 0.68

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auslegen auf die Spitze, nicht auf den Mittelwert", title)
        din = _din_ref("DIN EN 12831-1 · DIN/TS 12831-1 · VDI 4645")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "kurve"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ordered = np.sort(load)[::-1]
        ys = _plot_samples(ordered) * PEAK_F
        xs = np.linspace(0.0, 1.0, PLOT_N)
        cum = _cumulative(ordered)
        q_year = round(float(cum[-1]), -2)
        mean_f = float(np.mean(ordered)) * PEAK_F
        big = PEAK_F * 1.42
        kw_of = lambda f: f / PEAK_F * PEAK_KW
        cap = ValueTracker(mean_f)
        x = ValueTracker(0.0)

        def demand():
            return float(np.interp(x.get_value(), xs, ys))

        def cycled():
            d, c = demand(), cap.get_value()
            floor_out = 0.3 * c
            if d >= floor_out:
                return d
            on = ((self.time % 1.2) / 1.2) < d / floor_out
            return floor_out if on else 0.0

        chart = _wide_chart()
        kw_ticks = _kw_ticks(chart, PEAK_KW / PEAK_F)
        hours = _hour_labels(chart)
        curve = _partial(chart, ys, 1.0, width=3.0)
        knob = always_redraw(lambda: Dot(chart["pt"](x.get_value(), demand()), radius=0.07, color=POWER))
        cap_line = always_redraw(lambda: DashedLine(chart["pt"](0.0, cap.get_value()), chart["pt"](1.0, cap.get_value()),
                                                    color=P_GREEN, stroke_width=2.6))
        cap_read = math_readout(lambda: rf"\Phi_{{\mathrm{{Anlage}}}} = {de_num(kw_of(cap.get_value()), 1)}\,\mathrm{{kW}}",
                                lambda: chart["pt"](1.0, cap.get_value()) + UP * 0.2, size=LABEL_FONT_SIZE, color=P_GREEN,
                                edge="right")
        above = [i for i, v in enumerate(ys) if v > mean_f]
        gap = Polygon(*[chart["pt"](xs[i], float(ys[i])) for i in above], *[chart["pt"](xs[i], mean_f) for i in reversed(above)],
                      fill_color=P_RED, fill_opacity=0.22, stroke_width=0)
        gap_tag = math_label(r"\text{Defizit}", chart["pt"](0.15, (float(np.interp(0.15, xs, ys)) + mean_f) / 2 - 0.02), size=LABEL_FONT_SIZE,
                             color=P_RED)

        bulb = np.array([-6.0, WIDE_ORIGIN[1] + 0.17, 0.0])
        th_h = 1.6
        th = thermometer_glyph(bulb, height=th_h, color=P_RED, level=0.8)
        th["level"].add_updater(lambda m: m.set_value(0.8 - 0.4 * float(np.clip((demand() - cap.get_value()) / PEAK_F, 0.0, 1.0))))
        soll_y = bulb[1] + 0.05 + (th_h - 0.12) * 0.8
        soll = Line(np.array([bulb[0] - 0.16, soll_y, 0.0]), np.array([bulb[0] + 0.16, soll_y, 0.0]), color=P_GREEN, stroke_width=1.8)
        soll_lbl = math_label(rf"{de_num(THETA_I)}\,\mathrm{{°C}}", np.array([bulb[0] + 0.22, soll_y - 0.07, 0.0]),
                              size=LABEL_FONT_SIZE - 2, color=P_GREEN, edge="left")
        th_lbl = body_text("Raumtemperatur", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(bulb + UP * (th_h + 0.3))

        reserve = Polygon(
            *[chart["pt"](xs[i], float(ys[i])) for i in range(PLOT_N)],
            *[chart["pt"](xs[i], big) for i in reversed(range(PLOT_N))],
            fill_color=P_ORANGE, fill_opacity=0.2, stroke_width=0)
        reserve_tag = math_label(r"\text{Reserve}", chart["pt"](0.55, 0.62), size=BODY_FONT_SIZE, color=P_ORANGE)
        floor_line = DashedLine(chart["pt"](0.0, 0.3 * big), chart["pt"](1.0, 0.3 * big), color=P_ORANGE, stroke_width=2.0)
        floor_lbl = math_label(r"\text{Mindestleistung}", chart["pt"](0.98, 0.3 * big) + UP * 0.2, size=LABEL_FONT_SIZE,
                               color=P_ORANGE, edge="right")
        sup_dot = always_redraw(lambda: Dot(chart["pt"](x.get_value(), cycled()), radius=0.09, color=P_GREEN))
        taktet = math_label(r"\text{taktet}", chart["pt"](0.5, 0.55 * big), size=BODY_FONT_SIZE, color=P_ORANGE)

        self.play(Create(chart["group"]), FadeIn(kw_ticks), FadeIn(hours), run_time=1.6)
        self.play(Create(curve), run_time=2.4)
        hold_for(self, N, "kurve", used=0.3 + 1.6 + 2.4)

        caption = swap_caption(self, caption, subtitle_text(N, "falsch"))
        self.add(knob)
        self.play(_ramp(x, 1.0, rate_func=linear), run_time=4.0)
        self.play(_ramp(x, 0.0), run_time=1.0)
        hold_for(self, N, "falsch")

        caption = swap_caption(self, caption, subtitle_text(N, "mittelwert"))
        self.add(cap_line)
        self.play(Create(cap_line), FadeIn(cap_read), run_time=1.8)
        hold_for(self, N, "mittelwert")

        caption = swap_caption(self, caption, subtitle_text(N, "defizit"))
        self.play(FadeIn(gap), FadeIn(gap_tag), run_time=1.6)
        self.play(_ramp(x, 0.3, rate_func=linear), run_time=3.0)
        hold_for(self, N, "defizit")

        caption = swap_caption(self, caption, subtitle_text(N, "temperatur"))
        self.add(th["column"], th["level"])
        self.play(_ramp(x, 0.0), FadeIn(th["group"]), FadeIn(soll), FadeIn(soll_lbl), FadeIn(th_lbl), run_time=1.6)
        self.play(_ramp(x, 0.06), run_time=2.0)
        self.play(_ramp(x, 0.25, rate_func=linear), run_time=2.0)
        hold_for(self, N, "temperatur")
        _hide(self, gap, gap_tag, th["group"], th["column"], soll, soll_lbl, th_lbl)

        caption = swap_caption(self, caption, subtitle_text(N, "reserve"))
        self.play(_ramp(cap, big), _ramp(x, 0.0), run_time=2.4)
        self.add(reserve)
        self.play(FadeIn(reserve), run_time=1.4)
        hold_for(self, N, "reserve")

        caption = swap_caption(self, caption, subtitle_text(N, "ueberdimensioniert"))
        self.play(FadeIn(reserve_tag), _ramp(x, 1.0, rate_func=linear), run_time=5.0)
        hold_for(self, N, "ueberdimensioniert")
        _hide(self, reserve_tag)

        caption = swap_caption(self, caption, subtitle_text(N, "takten"))
        self.add(sup_dot)
        self.play(_ramp(x, 0.35), FadeIn(floor_line), FadeIn(floor_lbl), run_time=1.6)
        self.play(FadeIn(taktet), _ramp(x, 0.62, rate_func=linear), run_time=5.0)
        hold_for(self, N, "takten")
        _hide(self, reserve, floor_line, floor_lbl, taktet, sup_dot, knob)
        self.add(knob)

        caption = swap_caption(self, caption, subtitle_text(N, "auslegungspunkt"))
        self.play(_ramp(cap, PEAK_F), _ramp(x, 0.0), run_time=2.0)
        self.play(_ramp(x, 1.0, rate_func=linear), run_time=4.0)
        hold_for(self, N, "auslegungspunkt")

        caption = swap_caption(self, caption, subtitle_text(N, "anlage"))
        parts = [body_text(name, font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(np.array([px, 1.75, 0.0]))
                 for name, px in (("Erzeuger", -3.2), ("Regelung", -1.0), ("Speicher", 1.2))]
        for part in parts:
            self.play(FadeIn(part, shift=UP * 0.2), run_time=1.3)
        hold_for(self, N, "anlage")
        _hide(self, *parts, cap_line, cap_read, knob)

        caption = swap_caption(self, caption, subtitle_text(N, "bedarf"))
        tank = _wide_tank()
        q_tag = math_label(r"Q_{h}\;[\mathrm{kWh}]", tank["cap"].get_top() + UP * 0.28, size=LABEL_FONT_SIZE, color=ENERGY)
        sweep = always_redraw(lambda: _partial_area(chart, ys, x.get_value()))
        q_read = math_readout(lambda: rf"Q_{{h}} = {de_num(round(_energy_at(cum, x.get_value()), -2))}\,\mathrm{{kWh}}",
                              np.array([0.2, 1.08, 0.0]), size=BODY_FONT_SIZE, color=ENERGY)
        tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, x.get_value()) / cum[-1]))
        x.set_value(0.0)
        self.add(sweep, tank["level"])
        q_read.update()
        self.play(FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(q_tag), FadeIn(q_read), run_time=1.4)
        self.play(_ramp(x, 1.0, rate_func=linear), run_time=6.0)
        q_final = math_label(rf"Q_{{h}} \approx {de_num(q_year)}\,\mathrm{{kWh/a}}", np.array([0.2, 1.08, 0.0]),
                             size=BODY_FONT_SIZE, color=ENERGY, edge="left")
        self.play(ReplacementTransform(q_read, q_final), run_time=1.0)
        hold_for(self, N, "bedarf")

        caption = swap_caption(self, caption, subtitle_text(N, "kosten"))
        _hide(self, tank["group"], tank["fill"], q_tag)
        panel = math_panel([
            ("q", r"Q_{h}", ENERGY), (None, r"\to", P_WHITE), (None, r"\text{Endenergie}", P_WHITE),
            (None, r"\to", P_WHITE), (None, r"\text{Kosten}", P_WHITE),
        ])
        _fill_panel(self, panel, {"q": q_final})
        hold_for(self, N, "kosten")

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion
