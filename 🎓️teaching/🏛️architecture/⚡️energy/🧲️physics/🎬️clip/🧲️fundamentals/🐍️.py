

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))
_theme_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
semio_color = _theme_module.color

import numpy as np
from manim import *

from pathlib import Path as _Path


_cursor_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🎬️scene.🐍️")
cursor_click = _cursor_module.cursor_click

_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
centered_body_text = _manim_module.centered_body_text
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
FE_RADIATOR_METRE = 1.2
_staffage_width = _manim_module.staffage_width
room_metre = _manim_module.room_metre
P_PRIMARY = _manim_module.P_PRIMARY
section_material = _manim_module.section_material
window_frame = _manim_module.window_frame
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
lamella_faces = _manim_module.lamella_faces
P_GREEN = _manim_module.P_GREEN
SAFE_TOP = _manim_module.SAFE_TOP
NOTE_BAND_Y = _manim_module.NOTE_BAND_Y
radiation_ray = _manim_module.radiation_ray
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
animate_flows = _manim_module.animate_flows
meter = _manim_module.meter
set_meter = _manim_module.set_meter
side_labels = _manim_module.side_labels
math_text = _manim_module.math_text
math_row = _manim_module.math_row
formula_panel = _manim_module.formula_panel
highlight_param = _manim_module.highlight_param
dim_arrow = _manim_module.dim_arrow
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
tr = _manim_module.tr
label_number = _manim_module.label_number
load_vo_timing = _manim_module.load_vo_timing
begin_vo_beat = _manim_module.begin_vo_beat

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Physikalische Grundlagen"
PANEL_MAX_WIDTH = 11.6


#region Shared helpers
#region Numbers and typeset labels
def _place(mob, at, edge: str = "center"):
    """📌 Sit a typeset box on a baseline point; ``edge`` picks which side lands on ``at``."""
    at = np.array(at, dtype=float)
    x = {"left": mob.get_left()[0], "right": mob.get_right()[0], "center": mob.get_center()[0]}[edge]
    mob.shift(np.array([at[0] - x, at[1] - mob.base.get_center()[1], 0.0]))
    return mob


def _m(src: str, at=None, *, size: float = BODY_FONT_SIZE, color: str = P_WHITE, edge: str = "center"):
    """∑ Typeset label, optionally placed on a baseline point."""
    mob = math_text(src, font_size=size, color=color)
    return _place(mob, at, edge) if at is not None else mob


def _readout(fn, at, *, size: float = BODY_FONT_SIZE, color: str = P_WHITE, edge: str = "left"):
    """📟 Live label redrawn every frame from ``fn()`` — the number is read off the geometry."""
    return always_redraw(lambda: _m(fn(), at, size=size, color=color, edge=edge))


def _panel(parts, *, color: str = P_PRIMARY, size: float = FORMULA_FONT_SIZE):
    """🧮 Typeset formula in the fixed bottom panel, shrunk only when it would overrun the frame."""
    row, items = math_row(parts, font_size=size, buff=0.24)
    if row.width > PANEL_MAX_WIDTH:
        row.scale(PANEL_MAX_WIDTH / row.width)
    row, box = formula_panel(row, color=color)
    return row, box, items


def _fill_panel(scene, panel, sources, *, run_time: float = 1.2, fly: bool = True):
    """🧲 Bring source values into their empty panel slots first, then reveal the rest of the formula and its box.

    ``fly`` morphs a copy along a clear path; without it the source pulses while its slot appears in the
    same colour — used whenever a straight flight would cross the drawing.
    """
    row, box, items = panel
    targets = [items[key] for key in sources]
    rest = VGroup(*[m for m in row.submobjects if all(m is not t for t in targets)])
    if fly:
        copies = {key: src.copy() for key, src in sources.items()}
        for copy in copies.values():
            for part in copy.get_family():
                part._layout_zone = "formula"
        scene.play(*[copies[key].animate.move_to(items[key]) for key in sources], run_time=run_time * 0.7)
        scene.play(*[FadeTransform(copies[key], items[key]) for key in sources], run_time=run_time * 0.4)
    else:
        scene.play(*[Indicate(src, scale_factor=1.15) for src in sources.values()],
                   *[FadeIn(items[key], scale=1.3) for key in sources], run_time=run_time)
    scene.play(FadeIn(rest), Create(box), run_time=0.6)


def _swap_panel(scene, old, new, *, run_time: float = 0.8):
    """🔁 Replace one formula panel with the next."""
    scene.play(FadeOut(old[0]), FadeOut(old[1]), run_time=run_time / 2)
    scene.play(FadeIn(new[0]), Create(new[1]), run_time=run_time / 2)
    return new


def _reveal(scene, *mobs, run_time: float = 0.35):
    """✨ Fade readouts and labels in — nothing pops onto the screen unannounced."""
    scene.play(*[FadeIn(m, shift=UP * 0.05) for m in mobs], run_time=run_time)
#endregion


#region Charts
def _axes(origin, x_len: float, y_len: float, x_label: str, y_label: str, *, color: str = P_WHITE):
    """📐 Axis cross with arrow tips and typeset axis names."""
    o = np.array(origin, dtype=float)
    x_ax = Arrow(o, o + RIGHT * x_len, buff=0, stroke_width=2.4, color=color,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.06)
    y_ax = Arrow(o, o + UP * y_len, buff=0, stroke_width=2.4, color=color,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.08)
    xl = _m(x_label, size=LABEL_FONT_SIZE, color=color)
    xl.next_to(x_ax.get_end(), DOWN, buff=0.14).align_to(x_ax.get_end(), RIGHT)
    yl = _m(y_label, size=LABEL_FONT_SIZE, color=color)
    yl.next_to(y_ax.get_end(), RIGHT, buff=0.14)
    return {"o": o, "x": x_ax, "y": y_ax, "xl": xl, "yl": yl, "group": VGroup(x_ax, y_ax, xl, yl)}


_DECADE_WORDS = {0: "eins", 3: "Tausend", 6: "Million", 9: "Milliarde", 12: "Billion", 15: "Billiarde"}
_SI_PREFIXES = {0: ("", ""), 3: ("k", "Kilo"), 6: ("M", "Mega"), 9: ("G", "Giga"), 12: ("T", "Tera"), 15: ("P", "Peta")}
_DECADE_EN = {0: "one", 3: "thousand", 6: "million", 9: "billion", 12: "trillion", 15: "quadrillion"}
_PREFIX_EN = {"Kilo": "kilo", "Mega": "mega", "Giga": "giga", "Tera": "tera", "Peta": "peta"}


def _decade_row(e: int, *, x_right: float, y: float, unit: str, base: str, shift: int):
    """🔟 One decade rung: ``10^e unit = 1 000…`` with the zeros lit, their count, and the number word with the SI prefix."""
    groups = ([e % 3] if e % 3 else []) + [3] * (e // 3)
    parts = [("pow", rf"10^{{{e}}}\,\mathrm{{{unit}}}", P_WHITE), (None, "=", P_WHITE), ("one", "1", P_WHITE)]
    parts += [(f"z{i}", "0" * n, P_YELLOW) for i, n in enumerate(groups)]
    row, items = math_row(parts, font_size=LABEL_FONT_SIZE, buff=0.11)
    _place(row, np.array([x_right, y - 0.07, 0.0]), "right")
    symbol, name = _SI_PREFIXES[e + shift]
    count = _m(tr(rf"\textcolor{{{P_YELLOW}}}{{{e}}}\,\textcolor{{{P_YELLOW}}}{{\text{{zeros}}}}", rf"\textcolor{{{P_YELLOW}}}{{{e}}}\,\textcolor{{{P_YELLOW}}}{{\text{{Nullen}}}}"), size=17)
    names = _m(rf"\text{{{tr(_DECADE_EN[e], _DECADE_WORDS[e])}}}\,\cdot\,\textcolor{{{P_PRIMARY}}}{{\text{{{tr(_PREFIX_EN.get(name, 'unit'), name or 'Einheit')}}}}}"
               rf"\,=\,1\,\mathrm{{{symbol}{base}}}", size=17)
    gap = 0.18
    left = x_right - (count.width + gap + names.width)
    base_y = y - 0.07 - 0.36
    _place(count, np.array([left, base_y, 0.0]), "left")
    _place(names, np.array([x_right, base_y, 0.0]), "right")
    return {"e": e, "line": row, "pow": items["pow"], "one": items["one"], "zeros": [items[f"z{i}"] for i in range(len(groups))],
            "count": count, "names": names, "x_right": x_right}


def _log_ladder(lo: float, hi: float, *, x: float, y_lo: float, y_hi: float, unit: str, base: str, shift: int,
                step: int = 3):
    """📶 Vertical decade scale — every factor ten is one equal step, so a tealight and the world share one axis.

    Each major rung carries its power of ten, the written-out number with its zeros, and the SI prefix
    (``shift`` is the power of ten between ``unit`` and the unprefixed ``base`` unit: kWh → Wh is 3).
    """
    axis = Line(np.array([x, y_lo, 0.0]), np.array([x, y_hi, 0.0]), color=P_PRIMARY, stroke_width=3)

    def y_of(value: float) -> float:
        return y_lo + (np.log10(value) - lo) / (hi - lo) * (y_hi - y_lo)

    ticks, rows = VGroup(), []
    for e in range(int(np.ceil(lo)), int(np.floor(hi)) + 1):
        y = y_of(10.0 ** e)
        major = e % step == 0
        ticks.add(Line(np.array([x - (0.16 if major else 0.08), y, 0.0]), np.array([x, y, 0.0]),
                       color=P_WHITE, stroke_width=2.0 if major else 1.2))
        if major:
            rows.append(_decade_row(e, x_right=x - 0.24, y=y, unit=unit, base=base, shift=shift))
    texts = VGroup(*[VGroup(r["line"], r["count"], r["names"]) for r in rows])
    return {"axis": axis, "ticks": ticks, "rows": rows, "group": VGroup(axis, ticks, texts), "y_of": y_of, "x": x}


def _reveal_zeros(scene, ladder, *, run_time: float = 1.1):
    """0️⃣ Climb the rungs: each number grows its zeros one thousand-block at a time, then states how many there are."""
    for row in ladder["rows"]:
        scene.play(FadeIn(row["pow"], shift=RIGHT * 0.1), FadeIn(row["line"][1]), FadeIn(row["one"]),
                   run_time=run_time * 0.4)
        blocks = row["zeros"]
        if blocks:
            scene.play(LaggedStart(*[FadeIn(z, scale=1.4) for z in blocks], lag_ratio=0.5),
                       FadeIn(row["count"], shift=UP * 0.05), run_time=run_time)
        else:
            scene.play(FadeIn(row["count"], shift=UP * 0.05), run_time=run_time * 0.4)


def _reveal_prefixes(scene, ladder, *, run_time: float = 0.7):
    """🏷️ Swap the long numbers for names: the zeros step aside, each rung gets its number word and SI prefix."""
    rows = ladder["rows"]
    gone = [m for r in rows for m in (r["line"][1], r["one"], *r["zeros"])]
    scene.play(*[FadeOut(m) for m in gone],
               *[r["pow"].animate.shift(RIGHT * (r["x_right"] - r["pow"].get_right()[0])) for r in rows],
               run_time=0.8)
    for row in rows:
        scene.play(FadeIn(row["names"], shift=LEFT * 0.1), Indicate(row["count"], color=P_YELLOW, scale_factor=1.1),
                   run_time=run_time)


def _collapse_rows(scene, ladder, *, run_time: float = 0.7):
    """🧹 Clear the explanations, keep only the powers of ten, so the examples get the whole scale to themselves."""
    scene.play(*[FadeOut(m) for r in ladder["rows"] for m in (r["count"], r["names"])], run_time=run_time)


def _ladder_marks(ladder, entries, *, x_text: float, gap: float = 0.07, bottom: float = -1.7, top: float = 2.3):
    """📍 Rung dots with labels dodged apart so neighbouring values stay readable."""
    x_axis = ladder["x"]
    dots, labels, ys = [], [], []
    for value, src, color in entries:
        y = ladder["y_of"](value)
        dots.append(Dot(np.array([x_axis, y, 0.0]), radius=0.065, color=color))
        labels.append(_m(src, size=LABEL_FONT_SIZE, color=color))
        ys.append(y)
    placed = list(ys)
    order = sorted(range(len(placed)), key=lambda i: -placed[i])
    for a, b in zip(order, order[1:]):
        need = (labels[a].height + labels[b].height) / 2 + gap
        placed[b] = min(placed[b], placed[a] - need)
    under = bottom - (placed[order[-1]] - labels[order[-1]].height / 2)
    if under > 0:
        placed = [p + under for p in placed]
    over = placed[order[0]] + labels[order[0]].height / 2 - top
    if over > 0:
        placed = [p - over for p in placed]
    leaders = []
    for i, label in enumerate(labels):
        label.move_to(np.array([x_text + label.width / 2, placed[i], 0.0]))
        leaders.append(Line(np.array([x_axis + 0.09, ys[i], 0.0]), np.array([x_text - 0.08, placed[i], 0.0]),
                            color=entries[i][2], stroke_width=1.3, stroke_opacity=0.6))
    return dots, labels, leaders
#endregion


#region Particle flow with variable speed
def _gradient(stops):
    """🎨 Piecewise colour ramp over ``u`` in [0, 1] — ``stops`` are ``(u, colour)``."""
    us = [s[0] for s in stops]
    cols = [ManimColor(s[1]) for s in stops]

    def color_at(u: float):
        u = float(np.clip(u, 0.0, 1.0))
        for i in range(len(us) - 1):
            if us[i] <= u <= us[i + 1]:
                f = (u - us[i]) / max(1e-9, us[i + 1] - us[i])
                return interpolate_color(cols[i], cols[i + 1], f)
        return cols[-1]

    return color_at


def _paced_flow(scene, streams, *, run_time: float = 3.0, waves: int = 6, cycles: float = 2.0,
                radius: float = 0.055, extra=None):
    """🌀 Particles whose speed follows ``speed(i, u)`` along each path — continuity made visible.

    ``streams`` holds ``(paths, colour, speed, colour_fn, closed)``, optionally followed by a per-stream particle
    count; ``speed``, ``colour_fn`` and ``closed`` may be ``None``/``False``.
    """
    us = np.linspace(0.0, 1.0, 241)
    dots, meta = VGroup(), []
    for paths, color, speed, color_fn, closed, *count in streams:
        n = count[0] if count else waves
        for i, path in enumerate(paths):
            g = np.array([max(1e-3, speed(i, u)) if speed else 1.0 for u in us])
            tau = np.concatenate([[0.0], np.cumsum(0.5 * (1.0 / g[1:] + 1.0 / g[:-1]) * np.diff(us))])
            tau /= tau[-1]
            for w in range(n):
                dot = Dot(radius=radius, color=color, stroke_width=0)
                dot.set_fill(color, opacity=0.0)
                dot.move_to(path.point_from_proportion(0.0))
                dots.add(dot)
                meta.append((path, tau, (w / n + 0.37 * i / n) % 1.0, color, color_fn, closed))

    def update(group, alpha):
        for dot, (path, tau, offset, color, color_fn, closed) in zip(group, meta):
            t = (alpha * cycles + offset) % 1.0
            u = float(np.interp(t, tau, us))
            dot.move_to(path.point_from_proportion(u))
            fade = 1.0 if closed else min(1.0, t / 0.06, (1.0 - t) / 0.08)
            dot.set_fill(color_fn(u) if color_fn else color, opacity=max(0.0, fade))

    scene.add(dots)
    scene.play(UpdateFromAlphaFunc(dots, update), *(extra or []), run_time=run_time, rate_func=linear)
    scene.remove(dots)
#endregion


#region Light beams and wave packets
def _beam(start, end, color=P_YELLOW, opacity: float = 0.85, width: float = 2.4):
    """━ Straight light beam — sunlight arrives as parallel rays (same look as Cooling Teil 4)."""
    return Line(np.array(start, dtype=float), np.array(end, dtype=float), color=color, stroke_width=width,
                stroke_opacity=opacity)


WAVES = {"short": (0.14, 0.03, 1.6), "long": (0.55, 0.085, 2.0)}


def _wave_points(corners, kind: str):
    """🌊 Points of a sinusoidal ray along a polyline — ``short`` is a dense ripple, ``long`` a stretched one, both tapering to the exact ends."""
    wavelength, amp, _ = WAVES[kind]
    corners = [np.array(c, dtype=float) for c in corners]
    segs = [(a, b, float(np.linalg.norm(b - a))) for a, b in zip(corners[:-1], corners[1:])]
    total = sum(length for _, _, length in segs)
    pts, done = [], 0.0
    for a, b, length in segs:
        u = (b - a) / length
        n = np.array([-u[1], u[0], 0.0])
        for t in np.linspace(0.0, length, max(4, int(length / wavelength * 12)), endpoint=False):
            s_ = done + t
            taper = min(1.0, s_ / wavelength, (total - s_) / wavelength)
            pts.append(a + u * t + n * amp * taper * np.sin(TAU * s_ / wavelength))
        done += length
    pts.append(corners[-1])
    return pts


def _wave(corners, kind: str, color=P_YELLOW, opacity: float = 0.85):
    """〰️ Wavy ray from ``corners`` — short-wave sunlight is compact, long-wave heat radiation is stretched."""
    mob = VMobject(color=color, stroke_width=WAVES[kind][2], stroke_opacity=opacity)
    mob.set_points_smoothly(_wave_points(corners, kind))
    return mob


def _mirror(direction, surface_angle: float):
    """🪞 Reflect a ray direction off a flat surface tilted by ``surface_angle`` radians."""
    u = np.array([np.cos(surface_angle), np.sin(surface_angle), 0.0])
    n = np.array([-u[1], u[0], 0.0])
    return direction - 2.0 * float(np.dot(direction, n)) * n


def _sun_rays(sun_c, glass_x: float, ys, land_y: float, *, gap: float = 0.5):
    """☀️ Parallel straight sun rays aimed through a glazing at ``glass_x`` — bright outside, dimmer past the glass.

    Every ray shares the direction from the sun to the middle hit, starts ``gap`` clear of the sun's centre and
    lands on the surface at ``land_y``.
    """
    sun_c = np.array(sun_c, dtype=float)
    hits = [np.array([glass_x, y, 0.0]) for y in ys]
    aim = np.mean(hits, axis=0) - sun_c
    aim /= np.linalg.norm(aim)
    starts = [h - aim * (float(np.dot(h - sun_c, aim)) - gap) for h in hits]
    lands = [h + aim * (land_y - h[1]) / aim[1] for h in hits]
    out = VGroup(*[radiation_ray(s, h) for s, h in zip(starts, hits)])
    inside = VGroup(*[radiation_ray(h, p, stroke_width=2.0).set_stroke(opacity=0.5) for h, p in zip(hits, lands)])
    return {"out": out, "in": inside, "group": VGroup(out, inside), "starts": starts, "hits": hits, "lands": lands}


def _shine(rays, *, lag: float = 0.2):
    """🔦 Draw each ray from the sun to the glass, then on through it."""
    return LaggedStart(*[Succession(Create(o, rate_func=linear), Create(i, rate_func=linear))
                         for o, i in zip(rays["out"], rays["in"])], lag_ratio=lag)


def _ripples(centers, *, r_max: float = 0.8, rings: int = 3, cycles: float = 2.0, color=P_RED, down: bool = False):
    """🌡️ Diffuse long-wave emission — half-rings spread from warm surface points in every direction and fade.

    ``down`` emits from a ceiling into the room below instead of from a floor into the room above.
    """
    centers = [np.array(c, dtype=float) for c in centers]
    start = PI if down else 0.0
    arcs = VGroup(*[Arc(radius=0.05, start_angle=start, angle=PI, arc_center=c, color=color, stroke_width=2)
                    for c in centers for _ in range(rings)])

    def update(group, alpha):
        ramp = min(1.0, alpha * 8, (1.0 - alpha) * 8)
        for i, arc in enumerate(group):
            u = (alpha * cycles + (i % rings) / rings) % 1.0
            arc.become(Arc(radius=0.05 + r_max * u, start_angle=start, angle=PI, arc_center=centers[i // rings],
                           color=color, stroke_width=2, stroke_opacity=0.65 * (1.0 - u) * ramp))

    return UpdateFromAlphaFunc(arcs, update, remover=True, rate_func=linear)


def _beam_pulses(scene, paths, color, *, repeats: int = 2, run_time: float = 2.4, width: float = 5.0, extra=None,
                 kind: str | None = None):
    """⚡ Bright pulses running along beam lines — the light itself travels, no extra glyphs."""
    lines = []
    for corners in paths:
        line = VMobject(color=color, stroke_width=width)
        if kind:
            line.set_points_smoothly(_wave_points(corners, kind))
        else:
            line.set_points_as_corners([np.array(c, dtype=float) for c in corners])
        lines.append(line)
    flashes = [ShowPassingFlash(line.copy(), time_width=0.35) for _ in range(repeats) for line in lines]
    scene.play(LaggedStart(*flashes, lag_ratio=0.6 / len(lines)), *(extra or []), run_time=run_time)


#endregion


#region Glyphs — sun shading
BLIND_CAST = "shared-blind-lowered-mirrored"
BLIND_PORTION = 0.65


def _blind(x: float, y_top: float, y_bot: float):
    """🪟 External venetian blind of the staffage archive fitted between ``y_top`` and ``y_bot`` and centred on ``x``.

    Only the upper ``BLIND_PORTION`` of the blind is drawn (true proportions, the lower part cut away) so the lamellae read at 1/``BLIND_PORTION`` times the scale of the full blind.

    ``slats`` holds the sun-facing face of every lamella as (outer end, inner end), ``ys`` their centres, ``angle`` the lamella angle and ``group`` the drawn cast.

    @see ../../../../🧍️staffage/🗂️catalog/🧱️components/external-venetian-blind/🔣️.json — the cast the lamellae are read from
    """
    top = float(_staffage_parts(BLIND_CAST, "box")["box"][:, 1].max())
    bottom = float(_staffage_parts(BLIND_CAST, "bottom-rail")["bottom-rail"][:, 1].min())
    bottom = top - BLIND_PORTION * (top - bottom)
    unit = (y_top - y_bot) / (top - bottom)
    anchor = np.array([x + 0.075 * unit, y_bot - bottom * unit, 0.0])
    group = _staffage(BLIND_CAST, unit=unit, anchor=anchor, window=(-1.0, bottom - 1e-3, 0.0, top + 1e-3))
    parts = {key: polygon for key, polygon in _staffage_parts(BLIND_CAST, "slat-").items() if float(polygon[:, 1].min()) >= bottom - 1e-6}
    slats = [(np.append(anchor[:2] + outer * unit, 0.0), np.append(anchor[:2] + inner * unit, 0.0)) for outer, inner in lamella_faces(parts)]
    axis = slats[0][1] - slats[0][0]
    return {"ys": np.array([(a[1] + b[1]) / 2 for a, b in slats]), "slats": slats, "angle": float(np.arctan2(axis[1], axis[0])),
            "group": group.set_z_index(4)}


def _blind_hit(blind, start, direction):
    """🎯 First point where a straight ray meets a lamella of ``blind`` — rays end on the lamella faces, not on a flat line."""
    start, direction = np.array(start, dtype=float), np.array(direction, dtype=float)
    best = None
    for outer, inner in blind["slats"]:
        axis = inner - outer
        matrix = np.array([[direction[0], -axis[0]], [direction[1], -axis[1]]])
        if abs(np.linalg.det(matrix)) < 1e-9:
            continue
        along, position = np.linalg.solve(matrix, (outer - start)[:2])
        if along > 0 and 0.0 <= position <= 1.0 and (best is None or along < best):
            best = along
    if best is None:
        best = (blind["slats"][0][0][0] - start[0]) / direction[0]
    return start + direction * best
#endregion


#region Glyphs — buildings
_window = _manim_module.window_glyph


_open_window = _manim_module.open_window


_build_cross_section_house = _manim_module.house_section


_room = _manim_module.room_section


_staffage = _manim_module.staffage
_staffage_motion = _manim_module.staffage_motion
_staffage_height = _manim_module.staffage_height
_staffage_parts = _manim_module.staffage_parts
_fitted_staffage = _manim_module.fitted_staffage
HOB_UNIT = 6.0
HOB_PAN = 0.088
POT_INNER = 0.172
POT_HEIGHT = 0.10
POT_BOIL_RATE = 30.5
KETTLE_RATE = 60.0
VACUUM_RATE = 60.0
SHOWER_UNIT = 2.2
SHOWER_NOZZLE = np.array([0.23, 0.86, 0.0])
SHOWER_RATE = 40.0



#endregion


#region Glyphs — everyday objects
_build_sun = _manim_module.sun_glyph


_moon = _manim_module.moon_glyph


def _person(cast: str, foot, height: float, *, motion: bool = True):
    """🧍 Staffage figure of ``height`` standing on ``foot``; ``motion`` plays its clip, otherwise its pose."""
    return (_staffage_motion if motion else _staffage)(
        cast, unit=height / _staffage_height(cast), anchor=np.array(foot, dtype=float))


_cloud = _manim_module.cloud_glyph

_cloud_trail = _manim_module.cloud_trail

_clock = _manim_module.clock_glyph


def _vacuum(width: float = 1.8):
    """🌪️ Thermal canister vacuum cleaner in side elevation, ``width`` wide; the 25 minute run-and-park clip plays at ``VACUUM_RATE`` so the shell and the exhaust warm up while it is on screen."""
    group = _staffage_motion("physics-fundamentals-vacuum", unit=width / _staffage_width("physics-fundamentals-vacuum"), rate=VACUUM_RATE)
    return group


def _shower_head(anchor):
    """🚿 Handcrafted shower rail and head in the animated thermal view — the head warms as the water runs; ``anchor`` is the spot the water leaves."""
    return _staffage_motion("physics-fundamentals-shower", unit=SHOWER_UNIT, rate=SHOWER_RATE,
                            anchor=np.array(anchor, dtype=float) - SHOWER_NOZZLE * SHOWER_UNIT)


_thermometer = _manim_module.thermometer_glyph


_energy_tank = _manim_module.energy_tank




def _lamp(anchor, height: float = 0.8):
    """💡 Thermal pendant lamp hanging from ``anchor``; ``cable`` is the invisible feed line the current flows along."""
    group = _staffage("shared-lamp-winter", unit=height / _staffage_height("shared-lamp-winter"))
    group.shift(np.array(anchor, dtype=float) - group.get_top())
    bulb = Point(group.get_bottom() + UP * 0.2 * height)
    return {"group": group, "bulb": bulb, "cable": Line(group.get_top(), bulb.get_center() + UP * 0.1 * height)}




#endregion
#endregion


#region Beat1 – Power: how fast energy flows
class Beat1_Leistung(Scene):
    NARRATION = [
        ("rate",
         "Power tells us how quickly energy is transferred. Its symbol is P and its unit is the watt.",
         "Leistung sagt, wie schnell Energie übertragen wird. Ihr Formelzeichen ist P, ihre Einheit Watt."),
        ("unit",
         "One watt means one joule of energy per second; one kilowatt is one thousand watts.",
         "1 Watt bedeutet 1 Joule Energie pro Sekunde; 1 Kilowatt sind 1 000 Watt."),
        ("area",
         "In a power-time diagram energy is an area: the height is the power, the width the time.",
         "Im Leistung-Zeit-Diagramm ist Energie eine Fläche: Höhe = Leistung, Breite = Zeit."),
        ("vacuum",
         "A vacuum cleaner: 0.7 kilowatts high, about 1.4 hours wide — one kilowatt-hour.",
         "Ein Staubsauger: 0,7 kW hoch, rund 1,4 h breit — Fläche 1 kWh."),
        ("kettle",
         "A two-kilowatt kettle needs only half an hour for it: almost three times as high, a third as wide.",
         "Ein Wasserkocher mit 2 kW braucht dafür nur eine halbe Stunde: fast dreimal so hoch, ein Drittel so breit."),
        ("shower",
         "A warm shower: one kilowatt-hour in three minutes is about twenty kilowatts — so instantaneous heaters have 18 to 24.",
         "Eine warme Dusche: 1 kWh in drei Minuten sind rund 20 kW — darum haben Durchlauferhitzer 18 bis 24 kW."),
        ("formula",
         "So energy is power times time, and power is energy divided by time.",
         "Also: Energie ist Leistung mal Zeit, Leistung ist Energie durch Zeit."),
        ("watt",
         "The base unit: one watt is one joule per second.",
         "Die Grundeinheit: 1 Watt ist 1 Joule pro Sekunde."),
        ("joule",
         "One kilowatt-hour is a thousand watts times 3,600 seconds — 3.6 million joules.",
         "1 kWh sind 1 000 W mal 3 600 s — 3,6 Millionen Joule."),
        ("zeros",
         "Power grows in steps of a thousand: count the zeros — a thousand watts has three, a million six, a billion nine, a trillion twelve.",
         "Leistung wächst in Tausender-Schritten: 1 000 W haben 3 Nullen, eine Million 6, eine Milliarde 9, eine Billion 12."),
        ("prefix",
         "The names: kilo is a thousand watts, mega a million, giga a billion, tera a trillion.",
         "Die Namen: Kilo = 1 000 W, Mega = eine Million W, Giga = eine Milliarde W, Tera = eine Billion W."),
        ("scale1",
         "A scale of power: a tealight about forty watts, a resting person about a hundred, a vacuum cleaner about seven hundred.",
         "Eine Skala der Leistung: Teelicht rund 40 W, ruhender Mensch rund 100 W, Staubsauger rund 700 W."),
        ("scale2",
         "An insulated detached house on the coldest day about eight kilowatts, a car about a hundred.",
         "Gedämmtes Einfamilienhaus am kältesten Tag rund 8 kW, ein Auto rund 100 kW."),
        ("scale3",
         "A wind turbine about five megawatts, a large power-station unit about 1.4 gigawatts.",
         "Ein Windrad rund 5 MW, ein großer Kraftwerksblock rund 1,4 GW."),
        ("scale4",
         "All power stations in the world together: about nine terawatts of installed capacity.",
         "Alle Kraftwerke der Welt zusammen: rund 9 TW installierte Leistung."),
        ("load",
         "The design heating load is determined for defined design conditions and is not the annual energy use.",
         "Die Auslegungsheizlast wird für definierte Auslegungsbedingungen bestimmt und ist nicht der Jahresenergieverbrauch."),
        ("demand",
         "The annual heating demand in kilowatt-hours is the area under the power curve over the year.",
         "Der Jahres-Heizwärmebedarf in kWh ist die Fläche unter der Leistungskurve übers Jahr."),
        ("summer",
         "From May to September, sun and internal heat cover the losses — the heating only runs on cool days.",
         "Von Mai bis September decken Sonne und innere Wärme die Verluste — geheizt wird nur an kühlen Tagen."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Power — how fast energy flows", "Leistung — wie schnell Energie fließt"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "rate"))
        self.play(FadeIn(caption), run_time=0.3)

        power_unit = _m(r"P\;[\mathrm{W}]", size=FORMULA_FONT_SIZE, color=P_CYAN)
        self.play(FadeIn(power_unit), run_time=0.8)
        hold_for(self, N, "rate", used=BEAT_SUBTITLE_FADE + 0.3 + 0.8)
        caption = swap_caption(self, caption, subtitle_text(N, "unit"))
        unit_definition = _m(r"1\,\mathrm{W} = 1\,\frac{\mathrm{J}}{\mathrm{s}}\quad\quad 1\,\mathrm{kW} = 1\,000\,\mathrm{W}",
                             size=FORMULA_FONT_SIZE, color=P_CYAN)
        self.play(ReplacementTransform(power_unit, unit_definition), run_time=0.8)
        hold_for(self, N, "unit")
        self.play(FadeOut(unit_definition), run_time=0.4)

        O = np.array([-4.9, -0.55, 0.0])
        SX, SY = 3.4, 1.18

        def pt(t, p):
            return O + RIGHT * t * SX + UP * p * SY

        caption = swap_caption(self, caption, subtitle_text(N, "area"))
        tank = _energy_tank(np.array([-1.8, 0.4, 0.0]), height=2.2)
        tank_lbl = _m(r"1\,\mathrm{kWh}", color=P_YELLOW).next_to(tank["cap"], UP, buff=0.14)
        self.play(FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(tank_lbl), run_time=0.8)
        ax = _axes(O, 5.1, 2.95, r"t\;[\mathrm{h}]", r"P\;[\mathrm{kW}]")
        t_ticks = VGroup(*[Line(pt(t, 0) + DOWN * 0.07, pt(t, 0) + UP * 0.07, color=P_WHITE, stroke_width=2)
                           for t in (0.5, 1.0)])
        p_ticks = VGroup(*[Line(pt(0, p) + LEFT * 0.07, pt(0, p) + RIGHT * 0.07, color=P_WHITE, stroke_width=2)
                           for p in (1.0, 2.0)])
        t_lbls = VGroup(_m(tr(r"0.5", r"0{,}5"), pt(0.5, 0) + DOWN * 0.38, size=LABEL_FONT_SIZE),
                        _m(r"1", pt(1.0, 0) + DOWN * 0.38, size=LABEL_FONT_SIZE))
        p_lbls = VGroup(_m(r"1", pt(0, 1) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right"),
                        _m(r"2", pt(0, 2) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right"))
        P = ValueTracker(1.0)
        T = ValueTracker(1.0)
        locked = {"on": False}

        def t_now():
            return 1.0 / P.get_value() if locked["on"] else T.get_value()

        def area():
            p = min(P.get_value(), 2.2)
            t = max(0.002, t_now())
            return Polygon(pt(0, 0), pt(t, 0), pt(t, p), pt(0, p), stroke_color=P_YELLOW, stroke_width=2,
                           fill_color=P_YELLOW, fill_opacity=0.35)

        rect = always_redraw(area)
        target = area()
        tank_static = VGroup(tank["frame"].copy(), tank["fill"].copy())
        self.remove(tank["group"], tank["fill"])
        self.add(tank_static)
        self.play(FadeIn(ax["group"]), FadeIn(t_ticks), FadeIn(p_ticks), FadeIn(t_lbls), FadeIn(p_lbls),
                  ReplacementTransform(tank_static, target), tank_lbl.animate.move_to(pt(0.5, 0.5)), run_time=1.4)
        self.remove(target)
        self.add(rect)
        self.bring_to_front(tank_lbl)
        hold_for(self, N, "area")

        caption = swap_caption(self, caption, subtitle_text(N, "vacuum"))
        name = Text(tr("Vacuum cleaner", "Staubsauger"), font_size=BODY_FONT_SIZE, color=P_PRIMARY).move_to(np.array([2.6, 1.95, 0.0]))
        p_read = _readout(lambda: rf"P = {label_number(P.get_value(), 1)}\,\mathrm{{kW}}", np.array([1.2, 1.3, 0.0]),
                          color=P_CYAN)
        t_read = _readout(
            lambda: rf"t = {label_number(t_now(), 2)}\,\mathrm{{h}} = {label_number(t_now() * 60)}\,\mathrm{{min}}",
            np.array([1.2, 0.75, 0.0]), color=P_ORANGE)
        e_read = _readout(lambda: rf"E = P \cdot t = {label_number(P.get_value() * t_now(), 2)}\,\mathrm{{kWh}}",
                          np.array([1.2, 0.2, 0.0]), color=P_YELLOW)
        controls = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        power_control = controls.chip("Start", font_size=LABEL_FONT_SIZE).move_to(np.array([5.7, 1.95, 0.0]))
        device_frame = controls.window_frame(6.2, 0.75, color=semio_color("gray"), fill_opacity=0)
        device_frame.move_to(np.array([3.4, 1.95, 0.0]))
        device_icon = _fitted_staffage("physics-fundamentals-vacuum", width=1.0, height=0.6, rate=0.0, loop=True, window=(-0.6, -0.05, 0.95, 0.75)).move_to(np.array([4.8, 1.95, 0.0]))
        self.play(FadeOut(tank_lbl), FadeIn(device_frame), FadeIn(name), FadeIn(device_icon), FadeIn(power_control),
                  run_time=0.5)
        T.set_value(0.0)
        P.set_value(0.7)
        _reveal(self, p_read, t_read, e_read)

        def start_vacuum():
            power_control[1].become(Text("An", font_size=LABEL_FONT_SIZE, color=P_DEEP_DARK).move_to(power_control[1]))
            device_icon.rate = VACUUM_RATE
            self.play(T.animate.set_value(1.0 / 0.7), run_time=3.0, rate_func=linear)

        cursor_click(self, power_control, start_vacuum, parents=(device_frame,), role="toggle")
        hold_for(self, N, "vacuum")

        caption = swap_caption(self, caption, subtitle_text(N, "kettle"))
        locked["on"] = True
        kettle_icon = _fitted_staffage("physics-fundamentals-kettle", width=0.8, height=0.6, rate=0.0, window=(-0.12, 0.0, 0.13, 0.25)).move_to(np.array([4.8, 1.95, 0.0]))
        shower_icon = _fitted_staffage("physics-fundamentals-shower", width=0.8, height=0.6, rate=0.0, window=(-0.02, 0.5, 0.24, 0.83)).move_to(np.array([4.8, 1.95, 0.0]))
        self.play(Transform(name, Text(tr("Kettle", "Wasserkocher"), font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(name)),
                  FadeOut(device_icon), FadeIn(kettle_icon), run_time=0.5)
        cursor_click(self, power_control, lambda: (setattr(kettle_icon, "rate", KETTLE_RATE), self.play(P.animate.set_value(2.0), run_time=2.2)),
                     parents=(device_frame,), role="device")
        hold_for(self, N, "kettle")

        caption = swap_caption(self, caption, subtitle_text(N, "shower"))
        self.play(Transform(name, Text(tr("Shower (instantaneous heater)", "Dusche (Durchlauferhitzer)"), font_size=BODY_FONT_SIZE,
                                       color=P_CYAN).move_to(name)), FadeOut(kettle_icon), FadeIn(shower_icon), run_time=0.5)
        cursor_click(self, power_control,
                     lambda: (setattr(shower_icon, "rate", SHOWER_RATE), self.play(P.animate.set_value(20.0), run_time=2.6, rate_func=rate_functions.ease_in_out_sine)),
                     parents=(device_frame,), role="device")
        zigzag = VMobject(color=P_CYAN, stroke_width=2.5)
        zigzag.set_points_as_corners([pt(0.0, 2.2), pt(0.017, 2.28), pt(0.033, 2.2), pt(0.05, 2.28)])
        clip = VGroup(zigzag, _m(r"\uparrow 20\,\mathrm{kW}", pt(0.12, 1.85), size=LABEL_FONT_SIZE, color=P_CYAN,
                                 edge="left"))
        self.play(FadeIn(clip), run_time=0.5)
        hold_for(self, N, "shower")

        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        self.play(FadeOut(clip), FadeOut(name), FadeOut(device_frame), FadeOut(power_control), FadeOut(shower_icon), run_time=0.4)
        self.play(P.animate.set_value(1.0), run_time=1.0)
        h_brace = Brace(Line(pt(0, 1), pt(1, 1)), UP, color=P_ORANGE)
        h_lbl = _m("t", color=P_ORANGE).next_to(h_brace, UP, buff=0.08)
        v_brace = Brace(Line(pt(1, 0), pt(1, 1)), RIGHT, color=P_CYAN)
        v_lbl = _m("P", color=P_CYAN).next_to(v_brace, RIGHT, buff=0.1)
        row, box, items = _panel([
            ("e", "E", P_YELLOW), (None, "=", P_WHITE), ("p", "P", P_CYAN), (None, r"\cdot", P_WHITE),
            ("t", "t", P_ORANGE), (None, r"\quad\quad", P_WHITE),
            ("p2", "P", P_CYAN), (None, "=", P_WHITE),
            ("frac", r"\frac{\textcolor{semio_warning}{E}}{\textcolor{semio_tertiary}{t}}".replace("semio_warning", semio_color("warning")).replace("semio_tertiary", semio_color("tertiary")), P_WHITE),
        ])
        ring_p = highlight_param(items, "p", color=P_CYAN)
        ring_t = highlight_param(items, "t", color=P_ORANGE)
        self.play(FadeIn(row), Create(box), run_time=0.9)
        self.play(GrowFromCenter(v_brace), FadeIn(v_lbl), Create(ring_p), run_time=0.8)
        self.play(GrowFromCenter(h_brace), FadeIn(h_lbl), Create(ring_t), run_time=0.8)
        hold_for(self, N, "formula")

        caption = swap_caption(self, caption, subtitle_text(N, "watt"))
        self.play(FadeOut(ring_p), FadeOut(ring_t), FadeOut(e_read), FadeOut(t_read), FadeOut(p_read), run_time=0.4)
        new_xl = _m(r"t\;[\mathrm{s}]", size=LABEL_FONT_SIZE).move_to(ax["xl"])
        new_yl = _m(r"P\;[\mathrm{W}]", size=LABEL_FONT_SIZE).move_to(ax["yl"], aligned_edge=LEFT)
        new_t = VGroup(_m(r"1\,800", t_lbls[0].get_center(), size=LABEL_FONT_SIZE),
                       _m(r"3\,600", t_lbls[1].get_center(), size=LABEL_FONT_SIZE))
        new_p = VGroup(_m(r"1\,000", size=LABEL_FONT_SIZE).move_to(p_lbls[0], aligned_edge=RIGHT),
                       _m(r"2\,000", size=LABEL_FONT_SIZE).move_to(p_lbls[1], aligned_edge=RIGHT))
        new_h = _m(r"3\,600\,\mathrm{s}", color=P_ORANGE).next_to(h_brace, UP, buff=0.08)
        new_v = _m(r"1\,000\,\mathrm{W}", color=P_CYAN).next_to(v_brace, RIGHT, buff=0.1)
        w_panel = _panel([
            (None, r"1\,\mathrm{W}", P_CYAN), (None, "=", P_WHITE), (None, r"1\,\frac{\mathrm{J}}{\mathrm{s}}", P_YELLOW),
        ])
        self.play(ReplacementTransform(ax["xl"], new_xl), ReplacementTransform(ax["yl"], new_yl),
                  ReplacementTransform(t_lbls, new_t), ReplacementTransform(p_lbls, new_p),
                  ReplacementTransform(h_lbl, new_h), ReplacementTransform(v_lbl, new_v), run_time=1.2)
        _swap_panel(self, (row, box), w_panel[:2])
        hold_for(self, N, "watt")

        caption = swap_caption(self, caption, subtitle_text(N, "joule"))
        j_lbl = _m(tr(r"3.6 \cdot 10^{6}\,\mathrm{J}", r"3{,}6 \cdot 10^{6}\,\mathrm{J}"), pt(0.5, 0.45), color=P_YELLOW)
        j_panel = _panel([
            (None, r"1\,\mathrm{kWh}", P_YELLOW), (None, "=", P_WHITE), (None, r"1\,000\,\mathrm{W}", P_CYAN),
            (None, r"\cdot", P_WHITE), (None, r"3\,600\,\mathrm{s}", P_ORANGE), (None, "=", P_WHITE),
            (None, tr(r"3.6 \cdot 10^{6}\,\mathrm{J}", r"3{,}6 \cdot 10^{6}\,\mathrm{J}"), P_YELLOW),
        ])
        self.play(FadeIn(j_lbl, scale=0.7), run_time=0.7)
        _swap_panel(self, w_panel[:2], j_panel[:2])
        hold_for(self, N, "joule")

        caption = swap_caption(self, caption, subtitle_text(N, "zeros"))
        self.play(FadeOut(VGroup(ax["x"], ax["y"], new_xl, new_yl, t_ticks, p_ticks, new_t, new_p, h_brace, v_brace,
                                 new_h, new_v, j_lbl, j_panel[0], j_panel[1])), FadeOut(rect), run_time=0.6)
        ladder = _log_ladder(1.0, 13.3, x=-0.8, y_lo=-1.6, y_hi=2.25, unit="W", base="W", shift=0)
        entries = [
            (40, rf"\text{{{tr('Tealight', 'Teelicht')}}} \approx 40\,\mathrm{{W}}", P_ORANGE),
            (100, tr(r"\text{Person at rest} \approx 100\,\mathrm{W}", r"\text{Mensch in Ruhe} \approx 100\,\mathrm{W}"), P_RED),
            (700, rf"\text{{{tr('Vacuum cleaner', 'Staubsauger')}}} \approx 700\,\mathrm{{W}}", P_YELLOW),
            (8000, tr(r"\text{Detached house, coldest day} \approx 8\,\mathrm{kW}", r"\text{Einfamilienhaus, kältester Tag} \approx 8\,\mathrm{kW}"), P_RED),
            (1e5, tr(r"\text{Car} \approx 100\,\mathrm{kW}", r"\text{Auto} \approx 100\,\mathrm{kW}"), P_CYAN),
            (5e6, tr(r"\text{Wind turbine} \approx 5\,\mathrm{MW}", r"\text{Windrad} \approx 5\,\mathrm{MW}"), P_GREEN),
            (1.4e9, tr(r"\text{Kraftwerksblock} \approx 1.4\,\mathrm{GW}", r"\text{Kraftwerksblock} \approx 1{,}4\,\mathrm{GW}"), P_ORANGE),
            (9e12, tr(r"\text{all power plants in the world} \approx 9\,\mathrm{TW}", r"\text{alle Kraftwerke der Welt} \approx 9\,\mathrm{TW}"), P_PRIMARY),
        ]
        dots, labels, leaders = _ladder_marks(ladder, entries, x_text=-0.15)
        marker = Dot(np.array([ladder["x"], ladder["y_of"](40), 0.0]), radius=0.1, color=P_YELLOW)
        self.play(Create(ladder["axis"]), FadeIn(ladder["ticks"]), run_time=1.0)
        _reveal_zeros(self, ladder)
        hold_for(self, N, "zeros")

        caption = swap_caption(self, caption, subtitle_text(N, "prefix"))
        _reveal_prefixes(self, ladder)
        hold_for(self, N, "prefix")

        caption = swap_caption(self, caption, subtitle_text(N, "scale1"))
        _collapse_rows(self, ladder)
        self.play(FadeIn(marker), run_time=0.3)
        groups = {"scale1": (0, 1, 2), "scale2": (3, 4), "scale3": (5, 6), "scale4": (7,)}
        for key, idxs in groups.items():
            if key != "scale1":
                caption = swap_caption(self, caption, subtitle_text(N, key))
            for i in idxs:
                self.play(marker.animate.move_to(dots[i]), FadeIn(dots[i]), Create(leaders[i]),
                          FadeIn(labels[i], shift=RIGHT * 0.1), run_time=0.75)
            if key == "scale4":
                span = dim_arrow(np.array([-6.6, ladder["y_of"](40), 0.0]),
                                 np.array([-6.6, ladder["y_of"](9e12), 0.0]), color=P_YELLOW)
                span_lbl = _m(r"\times\,3 \cdot 10^{11}", color=P_YELLOW).next_to(span, UP, buff=0.1)
                span_lbl.shift(RIGHT * max(0.0, -6.85 - span_lbl.get_left()[0]))
                self.play(GrowFromCenter(span), FadeIn(span_lbl), run_time=0.9)
            hold_for(self, N, key)

        caption = swap_caption(self, caption, subtitle_text(N, "load"))
        self.play(FadeOut(VGroup(ladder["group"], marker, *dots, *labels, *leaders, span, span_lbl)), run_time=0.6)
        YO = np.array([-5.7, -1.2, 0.0])
        DX, DY = 9.6 / 365.0, 0.31
        days = np.arange(366)
        weather = np.convolve(np.random.default_rng(3).normal(0.0, 1.0, 408), np.ones(21) / 21, "same")[21:387]
        drive = 0.26 + np.cos(TAU * (days - 15) / 365.0) + 0.25 * weather / weather.std()
        raw = 0.07 * np.log1p(np.exp(drive / 0.07))
        kw_day = raw * 15000.0 / (24.0 * raw[:365].sum())
        cum = np.concatenate([[0.0], np.cumsum(kw_day[:365] * 24.0)])

        def ypt(d, p):
            return YO + RIGHT * d * DX + UP * p * DY

        yax = _axes(YO, 10.7, 3.0, "t", r"P\;[\mathrm{kW}]")
        chart_title = _m(tr(r"\text{Heating power}\; P \;\text{over the year (schematic)}", r"\text{Heizleistung}\; P \;\text{über das Jahr (schematisch)}"), size=LABEL_FONT_SIZE,
                         color=P_PRIMARY).move_to(UP * NOTE_BAND_Y)
        months = VGroup(*[
            Text(m, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(ypt(15 + 30.4 * i, 0) + DOWN * 0.24)
            for i, m in enumerate(("Jan", "Feb", tr("Mar", "Mär"), "Apr", tr("May", "Mai"), "Jun", "Jul", "Aug", "Sep", tr("Oct", "Okt"), "Nov", tr("Dec", "Dez")))
        ])
        curve = VMobject(color=P_CYAN, stroke_width=3)
        curve.set_points_smoothly([ypt(d, kw_day[d]) for d in range(0, 366, 2)])
        design = DashedLine(ypt(0, 8), ypt(365, 8), color=P_RED, stroke_width=2.5, dash_length=0.12)
        design_lbl = _m(tr(r"\text{Design heating load}\; P_{\mathrm{HL}} \approx 8\,\mathrm{kW}", r"\text{Auslegungsheizlast}\; P_{\mathrm{HL}} \approx 8\,\mathrm{kW}"), ypt(200, 8) + UP * 0.2,
                        color=P_RED, size=LABEL_FONT_SIZE, edge="left")
        p_ticks2 = VGroup(*[_m(str(p), ypt(0, p) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                            for p in (2, 4, 6, 8)])
        self.play(FadeIn(yax["group"]), FadeIn(months), FadeIn(p_ticks2), FadeIn(chart_title), run_time=0.8)
        self.play(Create(curve), run_time=1.6)
        self.play(Create(design), FadeIn(design_lbl), run_time=1.0)
        hold_for(self, N, "load")

        caption = swap_caption(self, caption, subtitle_text(N, "demand"))
        sweep = ValueTracker(0.0)

        def year_area():
            d_end = max(1, int(sweep.get_value()))
            pts = [ypt(0, 0)] + [ypt(d, kw_day[d]) for d in range(0, d_end + 1, 2)] + [ypt(d_end, 0)]
            return Polygon(*pts, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.3)

        fill = always_redraw(year_area)
        q_read = _readout(lambda: rf"Q = {label_number(cum[int(np.clip(sweep.get_value(), 0, 365))])}\,\mathrm{{kWh}}",
                          ypt(200, 6.2), color=P_CYAN, edge="left")
        self.add(fill)
        _reveal(self, q_read)
        self.play(sweep.animate.set_value(365), run_time=4.0, rate_func=linear)
        year_lbl = _m(tr(r"\text{Heating demand} \approx 15\,000\,\mathrm{kWh/a}", r"\text{Heizwärmebedarf} \approx 15\,000\,\mathrm{kWh/a}"), ypt(200, 6.2), color=P_CYAN,
                      edge="left")
        self.play(ReplacementTransform(q_read, year_lbl), run_time=0.6)
        hold_for(self, N, "demand")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        summer_tag = centered_body_text(tr("Summer: no heating\nneeded or gains\ncover the losses", "Sommer: keine Heizung\nnotwendig bzw. Gewinne\ndecken die Verluste"), font_size=LABEL_FONT_SIZE,
                                        color=P_YELLOW).move_to(ypt(197, 3.0))
        gap = Line(ypt(121, 0) + UP * 0.08, ypt(273, 0) + UP * 0.08, color=P_YELLOW, stroke_width=5)
        self.play(Create(gap), FadeIn(summer_tag, shift=UP * 0.1), run_time=1.0)
        hold_for(self, N, "summer")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Work and energy in everyday life: the kilowatt-hour
class Beat2_EnergieImAlltag(Scene):
    NARRATION = [
        ("meter",
         "Power acting over time transfers energy. The house meter counts this work in kilowatt-hours.",
         "Leistung über die Zeit überträgt Energie. Der Hauszähler zählt diese Arbeit in Kilowattstunden."),
        ("slip",
         "In everyday speech the h often gets dropped: we used four thousand kilowatts.",
         "Im Alltag fällt dabei oft das h weg: „4 000 kW verbraucht“."),
        ("hour",
         "But the h is the hour. Without it the number describes a power, not an amount.",
         "Doch das h ist die Stunde. Ohne sie beschreibt die Zahl eine Leistung, keine Menge."),
        ("product",
         "Work is transferred energy: at constant power, E equals P times t. One kilowatt for one hour is one kilowatt-hour.",
         "Arbeit ist übertragene Energie: Bei konstanter Leistung gilt E = P · t. 1 kW über 1 h ergibt 1 kWh."),
        ("vacuum",
         "What is one kilowatt-hour? A vacuum cleaner of about 700 watts runs on it for about an hour and a half.",
         "Was ist 1 kWh? Ein Staubsauger mit rund 700 W läuft damit rund anderthalb Stunden."),
        ("shower",
         "Or about three minutes of warm shower: thirty litres of water heated from ten to thirty-eight degrees.",
         "Oder rund drei Minuten warm duschen: 30 Liter Wasser, erwärmt von 10 auf 38 °C."),
        ("heat",
         "Mass times specific heat times temperature rise gives about 3,520 kilojoules.",
         "Masse mal spezifische Wärme mal Temperaturanstieg: rund 3 520 kJ."),
        ("convert1",
         "To convert: one kilowatt is one kilojoule per second, so one kilowatt-hour is 3,600 seconds of that — 3,600 kilojoules.",
         "Umrechnung: 1 kW sind 1 kJ pro Sekunde, also sind 1 kWh genau 3 600 s davon — 3 600 kJ."),
        ("convert2",
         "3,520 kilojoules divided by 3,600 kilojoules per kilowatt-hour: about one kilowatt-hour.",
         "3 520 kJ geteilt durch 3 600 kJ pro kWh: rund 1 kWh."),
        ("zeros",
         "To grasp the numbers, count the zeros: a thousand has three, a million six, a billion nine, a trillion twelve.",
         "Um die Zahlen zu begreifen, zähl die Nullen: Tausend hat 3, eine Million 6, eine Milliarde 9, eine Billion 12."),
        ("prefix",
         "Each name stands for those zeros: kilo is a thousand, mega a million, giga a billion, tera a trillion.",
         "Jeder Name steht für diese Nullen: Kilo = Tausend, Mega = Million, Giga = Milliarde, Tera = Billion."),
        ("kilo",
         "The k in kWh already means a thousand — so 1,000 kWh are 1 MWh.",
         "Das k in kWh heißt schon Tausend — 1 000 kWh sind also 1 MWh."),
        ("ladder",
         "On a scale: a tealight holds about 0.15 kilowatt-hours; a household uses about three thousand a year in electricity.",
         "Auf einer Skala: Ein Teelicht enthält rund 0,15 kWh, ein Haushalt braucht rund 3 000 kWh Strom im Jahr."),
        ("house",
         "A detached house needs around fifteen thousand kilowatt-hours of heat per year.",
         "Ein Einfamilienhaus braucht rund 15 000 kWh Heizwärme im Jahr."),
        ("world",
         "Germany uses about 2.4 trillion kilowatt-hours of final energy a year, the world about 170 trillion of primary energy.",
         "Deutschland: rund 2 400 Mrd. kWh Endenergie im Jahr. Die Welt: rund 170 000 Mrd. kWh Primärenergie."),
        ("design",
         "How many of them a house needs is decided in the design: glass areas, ceilings, openings.",
         "Wie viele kWh ein Haus braucht, entscheidet der Entwurf: Glasflächen, Decken, Öffnungen."),
        ("envelope",
         "Through walls and roof heat also flows outward — transmission.",
         "Durch Wände und Dach fließt Wärme außerdem nach außen — die Transmission."),
        ("chapters",
         "Each of these elements gets its own chapter: radiation, storage, airflow.",
         "Jedes dieser Bauteile bekommt ein eigenes Kapitel: Strahlung, Speicherung, Luftströmung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Work and energy — the kilowatt-hour", "Arbeit und Energie — die Kilowattstunde"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "meter"))
        self.play(FadeIn(caption), run_time=0.3)

        house = _build_cross_section_house(center=ORIGIN)
        hg = house["group"].scale(0.72).move_to(LEFT * 4.3 + DOWN * 0.35)
        hg.clear_updaters()
        wall_x = house["wall_right"].get_center()[0]
        meter_tag = Text(tr("Meter", "Zähler"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY)
        meter_depth = meter_tag.height + 2 * _theme_module.UI_SPACING
        meter_center = np.array([-1.45, 0.3, 0.0])
        meter_box = window_frame(
            2.5, 0.66 + meter_depth, title_width=meter_tag.width + 2 * _theme_module.UI_SPACING,
            title_depth=meter_depth, color=P_PRIMARY,
        ).move_to(meter_center + UP * meter_depth / 2)
        meter_tag.move_to([meter_box.get_left()[0] + _theme_module.UI_SPACING + meter_tag.width / 2,
                           meter_box.get_top()[1] - meter_depth / 2, 0])
        kwh = ValueTracker(0.0)
        meter_read = _readout(lambda: rf"{label_number(kwh.get_value())}\,\mathrm{{kWh}}",
                              meter_center + DOWN * 0.09, color=P_PRIMARY, edge="center")
        wire_in = Line(np.array([0.6, 0.3, 0.0]), meter_center + RIGHT * 1.25, color=P_YELLOW, stroke_width=2, stroke_opacity=0.5)
        wire_out = Line(meter_center + LEFT * 1.25, np.array([wall_x, 0.3, 0.0]), color=P_YELLOW, stroke_width=2,
                        stroke_opacity=0.5)
        grid_tag = Text(tr("Power grid", "Stromnetz"), font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(wire_in.get_start(), RIGHT,
                                                                                     buff=0.12)
        self.play(FadeIn(hg), FadeIn(meter_box), FadeIn(meter_tag), Create(wire_in), Create(wire_out), FadeIn(grid_tag),
                  run_time=1.0)
        _reveal(self, meter_read)
        animate_flows(self, [(VGroup(Line(wire_in.get_start(), wire_in.get_end()),
                                     Line(wire_out.get_start(), wire_out.get_end())), P_YELLOW)],
                      run_time=3.2, waves=4, cycles=3.0, radius=0.06, streak=False,
                      extra=[kwh.animate.set_value(4000)])
        hold_for(self, N, "meter", used=BEAT_SUBTITLE_FADE + 0.3 + 4.2)

        caption = swap_caption(self, caption, subtitle_text(N, "slip"))
        person = _person("physics-fundamentals-clerk", np.array([0.7, hg.get_bottom()[1] + 0.02, 0.0]),
                         1.75 * 0.72 * house["w_height"] / 5.0, motion=False)
        said, said_items = math_row([
            ("n", r"\text{4 000}", P_WHITE), ("kw", r"\mathrm{kW}", P_WHITE), ("v", tr(r"\text{consumed}", r"\text{verbraucht}"), P_WHITE),
        ], font_size=BODY_FONT_SIZE, buff=0.14)
        said.move_to(np.array([2.75, 0.95, 0.0]))
        cloud = _cloud(said)
        trail = _cloud_trail(cloud, person.get_top() + RIGHT * 0.12 + UP * 0.08)
        self.play(FadeIn(person, shift=UP * 0.15), FadeOut(grid_tag), run_time=0.5)
        self.play(LaggedStart(*[GrowFromCenter(p) for p in trail[::-1]], lag_ratio=0.45), run_time=0.6)
        self.play(GrowFromPoint(cloud, trail[0].get_center(), rate_func=rate_functions.ease_out_back), run_time=0.6)
        self.play(LaggedStart(*[FadeIn(said_items[k], shift=UP * 0.12) for k in ("n", "kw", "v")], lag_ratio=0.35),
                  run_time=0.8)
        self.play(cloud.animate(rate_func=there_and_back).scale(1.04), run_time=0.6)
        hold_for(self, N, "slip")

        caption = swap_caption(self, caption, subtitle_text(N, "hour"))
        clock = _clock(np.array([5.7, 0.95, 0.0]))
        clock_lbl = Text(tr("1 hour", "1 Stunde"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(clock["face"], DOWN, buff=0.14)
        h_glyph = _m(r"\mathrm{h}", clock["face"].get_top() + UP * 0.3, color=P_ORANGE)
        self.play(FadeIn(clock["group"]), FadeIn(clock_lbl), run_time=0.6)
        self.play(Rotate(clock["hand"], angle=-TAU, about_point=clock["center"]), FadeIn(h_glyph, scale=0.5),
                  run_time=1.2)
        kw = said_items["kw"]
        h_target = h_glyph.copy()
        _place(h_target, np.array([kw.get_right()[0] + 0.07, kw.base.get_center()[1], 0.0]), "left")
        shift = h_target.width + 0.09
        wide = _cloud(Rectangle(width=said.width + shift, height=said.height).move_to(said).shift(RIGHT * shift / 2))
        self.play(said_items["v"].animate.shift(RIGHT * shift), kw.animate.set_color(P_CYAN), Transform(cloud, wide),
                  run_time=0.7)
        self.play(h_glyph.animate.move_to(np.array([h_target.get_center()[0],
                                                    cloud.get_top()[1] + h_glyph.height / 2 + 0.15, 0.0])),
                  run_time=0.7)
        self.play(h_glyph.animate(rate_func=rate_functions.ease_in_quad).move_to(h_target.get_center()), run_time=0.4)
        self.play(cloud.animate(rate_func=there_and_back).scale(1.05), Flash(h_glyph, color=P_ORANGE, line_length=0.16,
                                                                             flash_radius=0.28), run_time=0.6)
        bubble = VGroup(cloud, trail)
        hold_for(self, N, "hour")

        caption = swap_caption(self, caption, subtitle_text(N, "product"))
        panel = _panel([
            (None, "1", P_WHITE), ("kw", r"\mathrm{kW}", P_CYAN), (None, r"\cdot", P_WHITE),
            (None, "1", P_WHITE), ("h", r"\mathrm{h}", P_ORANGE), (None, "=", P_WHITE),
            (None, "1", P_WHITE), ("kwh", r"\mathrm{kWh}", P_YELLOW),
        ])
        row, box, items = panel
        self.play(FadeOut(person), FadeOut(bubble), run_time=0.5)
        _fill_panel(self, panel, {"kw": kw, "h": h_glyph})
        hold_for(self, N, "product")

        caption = swap_caption(self, caption, subtitle_text(N, "vacuum"))
        tank = _energy_tank(np.array([-1.3, 0.35, 0.0]), height=2.3)
        tank_lbl = _m(r"1\,\mathrm{kWh}", color=P_YELLOW).next_to(tank["cap"], UP, buff=0.14)
        tank_read = _readout(lambda: rf"{label_number(tank['level'].get_value(), 2)}\,\mathrm{{kWh}}",
                             np.array([-1.95, 0.25, 0.0]), color=P_YELLOW, edge="right")
        floor_y = -0.98
        floor = Line(np.array([0.5, floor_y, 0.0]), np.array([4.9, floor_y, 0.0]), color=P_PRIMARY, stroke_width=3)
        vac = _vacuum()
        vac.shift(np.array([2.7 - vac.get_center()[0], floor_y + 0.02 - vac.get_bottom()[1], 0.0]))
        vac_tag = always_redraw(lambda: _m(r"P \approx 1\,\mathrm{kW}",
                                           np.array([vac.get_center()[0] + 0.45, vac.get_top()[1] + 0.3, 0.0]),
                                           color=P_PRIMARY))
        clock2 = _clock(np.array([5.5, 1.35, 0.0]))
        minutes = ValueTracker(0.0)
        min_read = _readout(lambda: rf"t = {label_number(minutes.get_value())}\,\mathrm{{min}}",
                            np.array([5.5, 0.6, 0.0]), color=P_CYAN, edge="center")
        outlet = tank["frame"].get_right() + DOWN * 0.3
        phase = ValueTracker(0.0)

        def plugged_cable():
            plug = np.array([vac.get_left()[0] + 0.36 * vac.width, vac.get_bottom()[1] + 0.26 * vac.height, 0.0])
            sag = max(floor_y + 0.04, min(outlet[1], plug[1]) - 0.25)
            wire = VMobject(color=P_YELLOW, stroke_width=2.5, stroke_opacity=0.6)
            wire.set_points_smoothly([outlet, outlet + RIGHT * 0.3,
                                      np.array([0.5 * (outlet[0] + plug[0]), sag, 0.0]), plug + LEFT * 0.3, plug])
            pulses = VGroup()
            for k in range(6):
                u = (k / 6 + phase.get_value()) % 1.0
                pulses.add(Dot(wire.point_from_proportion(u), radius=0.055, color=P_YELLOW).set_opacity(
                    min(1.0, u / 0.08, (1.0 - u) / 0.08)))
            return VGroup(wire, pulses)

        cable = always_redraw(plugged_cable)
        self.play(
            FadeOut(said), FadeOut(h_glyph), FadeOut(clock["group"]),
            FadeOut(clock_lbl), FadeOut(hg), FadeOut(meter_box), FadeOut(meter_tag), FadeOut(wire_in),
            FadeOut(wire_out), FadeOut(meter_read), run_time=0.7,
        )
        self.play(FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(tank_lbl), FadeIn(floor), FadeIn(vac),
                  FadeIn(vac_tag), FadeIn(clock2["group"]), FadeIn(cable), run_time=1.0)
        _reveal(self, tank_read, min_read)
        x0 = vac.get_center()[0]
        self.play(
            phase.animate.set_value(5.0),
            tank["level"].animate.set_value(0.0),
            minutes.animate.set_value(86),
            Rotate(clock2["hand"], angle=-TAU * 86 / 60, about_point=clock2["center"]),
            UpdateFromAlphaFunc(vac, lambda m, a: m.set_x(x0 + 0.85 * np.sin(a * TAU * 3))),
            run_time=5.5, rate_func=linear,
        )
        hold_for(self, N, "vacuum")

        caption = swap_caption(self, caption, subtitle_text(N, "shower"))
        head = _shower_head(np.array([3.0, 1.7, 0.0]))
        drop_paths = [Line(np.array([3.0 + 0.18 * dx, 1.7, 0.0]), np.array([3.0 + dx * 1.6, -0.95, 0.0])) for dx in
                      np.linspace(-0.4, 0.4, 7)]
        steam_paths = [smooth_path([np.array([3.0 + dx, -0.9, 0.0]), np.array([3.0 + dx * 1.5 + 0.5, -0.1, 0.0]),
                                    np.array([3.0 + dx + 0.8, 0.8, 0.0])]) for dx in (-0.2, 0.2, 0.6)]
        litres = ValueTracker(0.0)
        litre_read = _readout(lambda: rf"V = {label_number(litres.get_value())}\,\mathrm{{l}}",
                              np.array([1.95, -0.75, 0.0]), color=P_CYAN, edge="right")
        thermo = _thermometer(np.array([4.55, -0.85, 0.0]), height=1.7, level=10 / 50)
        temp = ValueTracker(10.0)
        thermo["level"].add_updater(lambda m: m.set_value(temp.get_value() / 50))
        temp_read = _readout(lambda: rf"{label_number(temp.get_value())}\,\mathrm{{°C}}",
                             np.array([4.3, 0.05, 0.0]), color=P_RED, edge="right")
        shower_cable = smooth_path([tank["frame"].get_right() + UP * 0.6, np.array([0.8, 1.8, 0.0]),
                                    np.array([2.1, 2.05, 0.0])])
        self.play(FadeOut(vac), FadeOut(vac_tag), FadeOut(cable), FadeIn(head), FadeIn(thermo["group"]),
                  FadeIn(thermo["column"]), tank["level"].animate.set_value(1.0), minutes.animate.set_value(0),
                  run_time=1.0)
        _reveal(self, litre_read, temp_read)
        _paced_flow(self, [
            (drop_paths, P_CYAN, None, None, False),
            (steam_paths, P_WHITE, None, None, False, 2),
            ([shower_cable], P_YELLOW, None, None, False),
        ], run_time=5.0, waves=4, cycles=5.0, radius=0.045, extra=[
            tank["level"].animate.set_value(0.0),
            minutes.animate.set_value(3),
            Rotate(clock2["hand"], angle=-TAU * 3 / 60, about_point=clock2["center"]),
            litres.animate.set_value(30),
            temp.animate.set_value(38),
        ])
        hold_for(self, N, "shower")

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        heat_panel = _panel([
            (None, "Q", P_WHITE), (None, "=", P_WHITE), ("m", r"30\,\mathrm{kg}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("c", tr(r"4.19\,\frac{\mathrm{kJ}}{\mathrm{kg\,K}}", r"4{,}19\,\frac{\mathrm{kJ}}{\mathrm{kg\,K}}"), P_WHITE), (None, r"\cdot", P_WHITE),
            ("dt", r"28\,\mathrm{K}", P_RED), (None, "=", P_WHITE), ("res", r"3\,520\,\mathrm{kJ}", P_ORANGE),
        ])
        self.play(FadeOut(row), FadeOut(box), FadeOut(floor), run_time=0.4)
        h_row, h_box, h_items = heat_panel
        _fill_panel(self, heat_panel, {"m": litre_read, "dt": temp_read})
        self.play(Indicate(h_items["res"], color=P_ORANGE), run_time=0.9)
        hold_for(self, N, "heat")

        caption = swap_caption(self, caption, subtitle_text(N, "convert1"))
        conv_panel = _panel([
            (None, r"1\,\mathrm{kWh}", P_YELLOW), (None, "=", P_WHITE), ("kw", r"1\,\mathrm{kW}", P_CYAN),
            (None, r"\cdot", P_WHITE), ("s", r"3\,600\,\mathrm{s}", P_ORANGE), (None, "=", P_WHITE),
            ("kj", r"1\,\frac{\mathrm{kJ}}{\mathrm{s}}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("s2", r"3\,600\,\mathrm{s}", P_ORANGE), (None, "=", P_WHITE), ("res", r"3\,600\,\mathrm{kJ}", P_ORANGE),
        ])
        conv_row, conv_box, conv_items = conv_panel
        _swap_panel(self, (h_row, h_box), (conv_row, conv_box))
        self.play(Indicate(conv_items["kw"], color=P_CYAN), Indicate(conv_items["kj"], color=P_CYAN), run_time=0.8)
        self.play(Indicate(conv_items["s"], color=P_ORANGE), Indicate(conv_items["s2"], color=P_ORANGE),
                  Indicate(conv_items["res"], color=P_ORANGE), run_time=0.9)
        hold_for(self, N, "convert1")

        caption = swap_caption(self, caption, subtitle_text(N, "convert2"))
        fin_panel = _panel([
            (None, "Q", P_WHITE), (None, "=", P_WHITE),
            ("a", r"\frac{3\,520\,\mathrm{kJ}}{3\,600\,\mathrm{kJ}}", P_ORANGE), (None, r"\mathrm{kWh}", P_YELLOW),
            (None, r"\approx", P_WHITE), ("res", tr(r"0.98\,\mathrm{kWh}", r"0{,}98\,\mathrm{kWh}"), P_YELLOW), (None, r"\approx", P_WHITE),
            ("one", r"1\,\mathrm{kWh}", P_YELLOW),
        ])
        fin_row, fin_box, fin_items = fin_panel
        _swap_panel(self, (conv_row, conv_box), (fin_row, fin_box))
        self.play(Indicate(fin_items["a"], color=P_ORANGE), run_time=0.8)
        self.play(Indicate(fin_items["one"], color=P_YELLOW), Indicate(tank_lbl, color=P_YELLOW), run_time=0.9)
        hold_for(self, N, "convert2")

        caption = swap_caption(self, caption, subtitle_text(N, "zeros"))
        ladder = _log_ladder(-1.5, 14.6, x=-0.8, y_lo=-1.6, y_hi=2.25, unit="kWh", base="Wh", shift=3)
        entries = [
            (0.15, rf"\text{{{tr('Tealight', 'Teelicht')}}} \approx {label_number(0.15, 2)}\,\mathrm{{kWh}}", P_ORANGE),
            (1.0, rf"\text{{{tr('Vacuum cleaner, 1.4 hours', 'Staubsauger, 1,4 Stunden')}}} \approx 1\,\mathrm{{kWh}}", P_YELLOW),
            (3000, tr(r"\text{Household electricity per year} \approx 3\,000\,\mathrm{kWh}", r"\text{Haushaltsstrom pro Jahr} \approx 3\,000\,\mathrm{kWh}"), P_CYAN),
            (15000, tr(r"\text{Space heat of a detached house per year} \approx 15\,000\,\mathrm{kWh}", r"\text{Heizwärme Einfamilienhaus pro Jahr} \approx 15\,000\,\mathrm{kWh}"), P_RED),
            (2.4e12, tr(r"\text{Deutschland pro Jahr, Endenergie} \approx 2.4 \cdot 10^{12}\,\mathrm{kWh}", r"\text{Deutschland pro Jahr, Endenergie} \approx 2{,}4 \cdot 10^{12}\,\mathrm{kWh}"), P_GREEN),
            (1.7e14, tr(r"\text{Welt pro Jahr, Primärenergie} \approx 1.7 \cdot 10^{14}\,\mathrm{kWh}", r"\text{Welt pro Jahr, Primärenergie} \approx 1{,}7 \cdot 10^{14}\,\mathrm{kWh}"), P_PRIMARY),
        ]
        dots, labels, leaders = _ladder_marks(ladder, entries, x_text=-0.15)
        thermo["level"].clear_updaters()
        self.play(
            FadeOut(fin_row), FadeOut(fin_box), FadeOut(head), FadeOut(thermo["group"]), FadeOut(thermo["column"]),
            FadeOut(litre_read), FadeOut(temp_read), FadeOut(min_read), FadeOut(clock2["group"]),
            FadeOut(tank_read), FadeOut(tank_lbl), run_time=0.6,
        )
        tank_static = VGroup(tank["frame"].copy(), tank["cap"].copy())
        self.remove(tank["group"], tank["fill"])
        self.add(tank_static)
        self.play(Create(ladder["axis"]), FadeIn(ladder["ticks"]), ReplacementTransform(tank_static, dots[1]),
                  run_time=1.2)
        _reveal_zeros(self, ladder)
        hold_for(self, N, "zeros")

        caption = swap_caption(self, caption, subtitle_text(N, "prefix"))
        _reveal_prefixes(self, ladder)
        hold_for(self, N, "prefix")

        caption = swap_caption(self, caption, subtitle_text(N, "kilo"))
        self.play(Indicate(ladder["rows"][1]["names"], color=P_PRIMARY, scale_factor=1.12), run_time=1.0)
        hold_for(self, N, "kilo")

        caption = swap_caption(self, caption, subtitle_text(N, "ladder"))
        _collapse_rows(self, ladder)
        for i in (0, 1, 2):
            self.play(FadeIn(dots[i]) if i != 1 else Indicate(dots[i], color=P_YELLOW),
                      Create(leaders[i]), FadeIn(labels[i], shift=RIGHT * 0.1), run_time=0.7)
        hold_for(self, N, "ladder")

        caption = swap_caption(self, caption, subtitle_text(N, "house"))
        self.play(FadeIn(dots[3]), Create(leaders[3]), FadeIn(labels[3], shift=RIGHT * 0.1), run_time=0.8)
        hold_for(self, N, "house")

        caption = swap_caption(self, caption, subtitle_text(N, "world"))
        for i in (4, 5):
            self.play(FadeIn(dots[i]), Create(leaders[i]), FadeIn(labels[i], shift=RIGHT * 0.1), run_time=0.8)
        span = dim_arrow(np.array([-6.6, ladder["y_of"](0.15), 0.0]), np.array([-6.6, ladder["y_of"](1.7e14), 0.0]),
                         color=P_YELLOW)
        span_lbl = _m(r"\times\,10^{15}", color=P_YELLOW).next_to(span, UP, buff=0.1)
        span_lbl.shift(RIGHT * max(0.0, -6.85 - span_lbl.get_left()[0]))
        self.play(GrowFromCenter(span), FadeIn(span_lbl), run_time=0.9)
        hold_for(self, N, "world")

        caption = swap_caption(self, caption, subtitle_text(N, "design"))
        self.play(FadeOut(VGroup(ladder["group"], *dots, *labels, *leaders, span, span_lbl)), run_time=0.6)
        house2 = _build_cross_section_house(center=np.array([-0.3, -0.45, 0.0]))
        house2["group"].clear_updaters()
        win_lo, win_hi = house2["windows"]
        sun_pos = np.array([-5.6, 1.85, 0.0])
        sun = _build_sun(sun_pos).scale(0.4)
        slab = section_material(Rectangle(width=house2["level_1"].width, height=0.18)).move_to(house2["level_1"].get_center())
        rays = _sun_rays(sun_pos, win_hi["x"], [win_hi["center"][1] + dy for dy in (0.15, 0.0, -0.15)],
                         slab.get_top()[1], gap=0.55)
        hits, lands = rays["hits"], rays["lands"]
        sun_patch = Line(lands[-1], lands[0], color=P_YELLOW, stroke_width=7, stroke_opacity=0.75)
        beams = [VMobject(color=P_YELLOW, stroke_width=5).set_points_as_corners([s, h, p])
                 for s, h, p in zip(rays["starts"], hits, lands)]

        x_w, sill, lintel = win_lo["x"], win_lo["sill"], win_lo["lintel"]
        x_r = house2["wall_right"].get_center()[0]
        floor_y, ceil_y = house2["walls"][0].get_bottom()[1], slab.get_bottom()[1]
        intake = [smooth_path([np.array(p) + UP * dy for p in (
            [x_w - 1.5, sill - 0.08, 0.0], [x_w - 0.5, sill + 0.08, 0.0], [x_w + 0.35, sill + 0.02, 0.0],
            [x_w + 1.1, floor_y + 0.2, 0.0], [x_r - 0.9, floor_y + 0.18, 0.0], [x_r - 0.3, floor_y + 0.6, 0.0],
        )]) for dy in (0.0, 0.11)]
        exhaust = [smooth_path([np.array(p) + DOWN * dy for p in (
            [x_r - 0.3, floor_y + 0.75, 0.0], [x_r - 0.55, ceil_y - 0.18, 0.0], [x_w + 1.4, ceil_y - 0.15, 0.0],
            [x_w + 0.4, lintel - 0.06, 0.0], [x_w - 0.4, lintel - 0.06, 0.0], [x_w - 1.0, lintel + 0.28, 0.0],
        )]) for dy in (0.0, 0.11)]
        guides = VGroup(flow_guides(intake[:1], P_CYAN, opacity=0.22), flow_guides(exhaust[:1], P_ORANGE, opacity=0.22))
        air = [(intake, P_CYAN, P_ORANGE), (exhaust, P_ORANGE)]

        left_tags = VGroup(
            Text(tr("Glass — radiation", "Glas — Strahlung"), font_size=LABEL_FONT_SIZE, color=P_YELLOW),
            Text(tr("Openings — air flow", "Öffnungen — Luftströmung"), font_size=LABEL_FONT_SIZE, color=P_CYAN),
        )
        left_tags[0].move_to(np.array([x_w - 0.35 - left_tags[0].width / 2, win_hi["center"][1] - 0.3, 0.0]))
        left_tags[1].move_to(np.array([x_w - 0.55 - left_tags[1].width / 2, floor_y - 0.38, 0.0]))
        left_leaders = VGroup(
            Line(left_tags[0].get_right() + RIGHT * 0.1, hits[-1] + LEFT * 0.08, color=P_YELLOW, stroke_width=1.4,
                 stroke_opacity=0.55),
            Line(left_tags[1].get_right() + RIGHT * 0.1, np.array([x_w - 0.08, sill - 0.04, 0.0]), color=P_CYAN,
                 stroke_width=1.4, stroke_opacity=0.55),
        )
        right_tags, right_leaders = side_labels([
            (slab.get_right(), tr("Ceiling — thermal mass", "Decke — Speichermasse"), P_ORANGE),
        ], x=x_r + 0.65, align="left")

        hc = house2["center"]
        top_l, top_r = hc + LEFT * 1.8 + UP * 1.2, hc + RIGHT * 1.8 + UP * 1.2
        peak = hc + UP * 2.3
        slopes = [(top_l + LEFT * 0.26, peak), (peak, top_r + RIGHT * 0.26)]

        def outward(a, b):
            u = (b - a) / np.linalg.norm(b - a)
            n = np.array([-u[1], u[0], 0.0])
            return n if n[1] > 0 else -n

        tr_lines = []
        for a, b in slopes:
            n = outward(a, b)
            m = (a + b) / 2
            tr_lines.append((m - n * 0.05, m + n * 0.65))
        wall_up, wall_dn = hc[1] + 0.7, hc[1] - 0.7
        for y in (wall_up, wall_dn):
            tr_lines.append((np.array([x_r - 0.05, y, 0.0]), np.array([x_r + 0.7, y, 0.0])))
        tr_paths = [smooth_path([a, (a + b) / 2, b]) for a, b in tr_lines]
        tr_guides = flow_guides(tr_paths, P_RED, opacity=0.3)
        tr_tag = Text(tr("Walls, roof — transmission", "Wände, Dach — Transmission"), font_size=LABEL_FONT_SIZE, color=P_RED)
        tr_tag.move_to(np.array([x_r + 0.9 + tr_tag.width / 2, wall_up, 0.0]))

        self.play(FadeIn(house2["group"]), FadeIn(sun), run_time=0.9)
        self.play(LaggedStart(*[Create(r) for r in rays["out"]], lag_ratio=0.25), FadeIn(left_tags[0]),
                  Create(left_leaders[0]), run_time=1.1)
        self.play(Indicate(win_hi["sash"], color=P_YELLOW, scale_factor=1.0),
                  LaggedStart(*[Create(r) for r in rays["in"]], lag_ratio=0.25), run_time=0.8)
        self.play(FadeIn(sun_patch, scale=0.6), run_time=0.4)
        self.play(ReplacementTransform(house2["level_1"].copy(), slab), FadeIn(right_tags[0]),
                  Create(right_leaders[0]), sun_patch.animate.set_stroke(color=P_ORANGE, opacity=0.9), run_time=1.0)
        self.play(_open_window(win_lo), FadeIn(left_tags[1]), Create(left_leaders[1]), run_time=0.9)
        self.play(FadeIn(guides), run_time=0.4)
        animate_flows(self, air, run_time=4.0, waves=5, cycles=2.0, radius=0.06,
                      extra=[LaggedStart(*[ShowPassingFlash(b.copy(), time_width=0.35) for b in beams * 3],
                                         lag_ratio=0.3)])
        hold_for(self, N, "design")

        caption = swap_caption(self, caption, subtitle_text(N, "envelope"))
        self.play(FadeIn(tr_guides), FadeIn(tr_tag), run_time=0.8)
        animate_flows(self, [(tr_paths, P_RED, P_BLUE)], run_time=3.6, waves=4, cycles=2.0, radius=0.06)
        hold_for(self, N, "envelope")

        caption = swap_caption(self, caption, subtitle_text(N, "chapters"))
        self.play(LaggedStart(
            Indicate(left_tags[0], color=P_WHITE, scale_factor=1.0),
            Indicate(right_tags[0], color=P_WHITE, scale_factor=1.0),
            Indicate(left_tags[1], color=P_WHITE, scale_factor=1.0), lag_ratio=0.5), run_time=2.0)
        hold_for(self, N, "chapters")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Energy is conserved: every watt ends up as heat
class Beat3_Energieerhaltung(Scene):
    NARRATION = [
        ("law",
         "Energy is not lost; it only changes form.",
         "Energie geht nicht verloren, sie wechselt nur die Form."),
        ("balance",
         "What goes into a room either leaves it again or stays stored in it.",
         "Was in einen Raum hineingeht, verlässt ihn wieder oder bleibt in ihm gespeichert."),
        ("losses",
         "But not everything gets in: the glass reflects 20 joules. And what is stored gives 10 joules back later.",
         "Nicht alles kommt hinein: Das Glas reflektiert 20 J. Und Gespeichertes gibt später 10 J wieder ab."),
        ("lamp",
         "A lamp draws sixty watts of electrical power.",
         "Eine Lampe nimmt 60 W elektrische Leistung auf."),
        ("split",
         "With an incandescent bulb about five percent becomes light and ninety-five percent heat straight away.",
         "Bei einer Glühlampe werden etwa 5 % Licht und 95 % sofort Wärme."),
        ("absorb",
         "The light hits walls, floor and furniture, is absorbed there — and becomes heat as well.",
         "Das Licht trifft Wände, Boden und Möbel, wird dort absorbiert — und wird ebenfalls Wärme."),
        ("total",
         "In the end all sixty watts are room heat — with LEDs too, and with every appliance.",
         "Am Ende sind alle 60 W Raumwärme. Das gilt auch für LEDs und für jedes Gerät."),
        ("person",
         "A resting person also gives off about a hundred watts.",
         "Auch ein ruhender Mensch gibt rund 100 W Wärme ab."),
        ("winter",
         "In winter this internal heat covers part of the losses and lowers the heating demand.",
         "Im Winter deckt diese innere Wärme einen Teil der Verluste und senkt den Heizwärmebedarf."),
        ("summer",
         "In summer it adds to the sun's heat and has to be removed by cooling.",
         "Im Sommer kommt sie zur Sonnenwärme hinzu und muss weggekühlt werden."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Energy is conserved — every watt becomes heat", "Energie bleibt erhalten — jedes Watt wird Wärme"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "law"))
        self.play(FadeIn(caption), run_time=0.3)

        room = _room(np.array([-2.3, 0.15, 0.0]), w=5.2, h=2.5)
        form_x0, form_w, form_y = 1.6, 4.2, 0.75

        def form_bar(parts):
            bar, x = VGroup(), form_x0
            for frac, color in parts:
                bar.add(Rectangle(width=form_w * frac, height=0.5, stroke_width=0, fill_color=color, fill_opacity=0.7)
                        .move_to(np.array([x + form_w * frac / 2, form_y, 0.0])))
                x += form_w * frac
            return bar

        forms = [form_bar([(1.0, P_CYAN)]), form_bar([(0.05, P_YELLOW), (0.95, P_RED)]), form_bar([(1.0, P_RED)])]
        bar_base = form_y - 0.09

        def in_bar_lbl(src, frac_from, frac_to):
            cx = form_x0 + form_w * (frac_from + frac_to) / 2
            return _m(src, np.array([cx, bar_base, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE)

        form_lbls = [
            VGroup(in_bar_lbl(tr(r"\text{electrical}\;100\,\mathrm{J}\;(100\,\%)", r"\text{elektrisch}\;100\,\mathrm{J}\;(100\,\%)"), 0.0, 1.0)),
            VGroup(in_bar_lbl(tr(r"\text{heat}\;95\,\mathrm{J}\;(95\,\%)", r"\text{Wärme}\;95\,\mathrm{J}\;(95\,\%)"), 0.05, 1.0),
                   _m(tr(r"\text{light}\;5\,\mathrm{J}\;(5\,\%)", r"\text{Licht}\;5\,\mathrm{J}\;(5\,\%)"), np.array([form_x0, form_y + 0.45, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_YELLOW, edge="left")),
            VGroup(in_bar_lbl(tr(r"\text{heat}\;100\,\mathrm{J}\;(100\,\%)", r"\text{Wärme}\;100\,\mathrm{J}\;(100\,\%)"), 0.0, 1.0)),
        ]
        form_brace = Brace(forms[0], DOWN, color=P_WHITE)
        form_sum = _m(tr(r"E = \text{constant}", r"E = \text{konstant}"), form_brace.get_bottom() + DOWN * 0.38, color=P_WHITE)
        law_lamp = _lamp(np.array([room["center"][0], room["y_c"], 0.0]))
        law_bulb = law_lamp["bulb"].get_center()
        law_cable = Line(law_lamp["cable"].get_start(), law_lamp["cable"].get_end())
        law_rays = VGroup(*[
            Line(law_bulb, end, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.85)
            for end in (np.array([room["x_l"] + 0.05, law_bulb[1] - 0.3, 0.0]),
                        np.array([room["center"][0] + 0.4, room["y_f"] + 0.02, 0.0]),
                        np.array([room["x_r"] - 0.05, law_bulb[1] - 0.6, 0.0]))
        ])
        law_glow = Circle(radius=0.3, stroke_width=0, fill_color=P_RED, fill_opacity=0.25).move_to(law_bulb)

        def law_heat(rt: float):
            return _ripples([law_bulb], r_max=0.8, color=P_RED, down=True, cycles=max(1.0, rt / 1.4))

        _paced_flow(self, [([law_cable], P_CYAN, None, None, False)], run_time=4.0, waves=3, cycles=2.5, radius=0.045,
                    extra=[FadeIn(room["group"]), FadeIn(law_lamp["group"]),
                           GrowFromEdge(forms[0], LEFT, rate_func=linear)])
        self.play(FadeIn(form_lbls[0]), run_time=0.7)
        self.play(GrowFromCenter(form_brace), FadeIn(form_sum), run_time=1.0)
        _paced_flow(self, [([law_cable], P_CYAN, None, None, False)], run_time=2.0, waves=3, cycles=1.5, radius=0.045)
        self.play(FadeOut(form_lbls[0]), run_time=0.5)
        self.play(ReplacementTransform(forms[0], forms[1], rate_func=linear), FadeIn(law_glow, scale=0.4),
                  LaggedStart(*[Create(r) for r in law_rays], lag_ratio=0.2), law_heat(4.0), run_time=4.0)
        self.play(FadeIn(form_lbls[1]), run_time=0.7)
        self.play(law_heat(2.5), run_time=2.5)
        self.play(FadeOut(form_lbls[1]), run_time=0.5)
        self.play(ReplacementTransform(forms[1], forms[2], rate_func=linear), law_rays.animate.set_color(P_ORANGE),
                  LaggedStart(*[Flash(r.get_end(), color=P_ORANGE, flash_radius=0.18, line_length=0.12)
                                for r in law_rays], lag_ratio=0.3),
                  room["air"].animate.set_fill(P_RED, opacity=0.1), law_heat(4.0), run_time=4.0)
        self.play(FadeIn(form_lbls[2]), FadeOut(law_rays), run_time=0.7)
        hold_for(self, N, "law", used=BEAT_SUBTITLE_FADE + 0.3 + 24.5, during=lambda rt: [law_heat(rt)])

        caption = swap_caption(self, caption, subtitle_text(N, "balance"))
        mid_y = room["center"][1]
        e_in = Arrow(np.array([-6.9, mid_y + 0.3, 0.0]), np.array([room["glass_x"] - 0.05, mid_y + 0.3, 0.0]),
                     buff=0, color=P_CYAN, stroke_width=5)
        e_out = Arrow(np.array([room["x_r"] + 0.25, mid_y - 0.3, 0.0]), np.array([1.6, mid_y - 0.3, 0.0]),
                      buff=0, color=P_RED, stroke_width=5)
        bx, bw = 2.0, 3.8

        def bar(y, x0, frac, width, color):
            return Rectangle(width=max(0.002, width * frac), height=0.42, stroke_width=0, fill_color=color,
                             fill_opacity=0.7).move_to(np.array([x0 + width * frac / 2, y, 0.0]))

        def lost_bar(y, x0, frac, width, color, ghost_opacity: float = 0.15):
            ghost = Rectangle(width=width * frac, height=0.42, stroke_width=0, fill_color=color,
                              fill_opacity=ghost_opacity)
            outline = DashedVMobject(Rectangle(width=width * frac, height=0.42, color=P_WHITE, stroke_width=2.5),
                                     num_dashes=14)
            return VGroup(ghost, outline).move_to(np.array([x0 + width * frac / 2, y, 0.0]))

        in_bar = bar(1.15, bx, 0.8, bw, P_CYAN)
        out_bar = bar(0.0, bx, 0.5, bw, P_RED)
        st_bar = bar(0.0, bx + bw * 0.5, 0.3, bw, P_ORANGE)
        refl_bar = lost_bar(1.15, bx + bw * 0.8, 0.2, bw, P_BLUE)
        st_lost = lost_bar(0.0, bx + bw * 0.7, 0.1, bw, P_ORANGE, ghost_opacity=0.0)
        in_t = _m(tr(r"\text{in}\;80\,\mathrm{J}", r"\text{hinein}\;80\,\mathrm{J}"), np.array([bx, 1.63, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN,
                  edge="left")
        out_t = _m(tr(r"\text{out}\;50\,\mathrm{J}", r"\text{hinaus}\;50\,\mathrm{J}"), np.array([bx, -0.48, 0.0]), size=LABEL_FONT_SIZE,
                   color=P_RED, edge="left")
        st_t = _m(tr(r"\text{stored}\;30\,\mathrm{J}", r"\text{gespeichert}\;30\,\mathrm{J}"), np.array([bx + bw * 0.5, -0.48, 0.0]),
                  size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        refl_t = _m(tr(r"\text{loss}\;20\,\mathrm{J}", r"\text{Verlust}\;20\,\mathrm{J}"), np.array([bx + bw, 1.63, 0.0]), size=LABEL_FONT_SIZE,
                    color=P_BLUE, edge="right")
        lost_t = _m(tr(r"10\,\mathrm{J}\;\text{released again later}", r"10\,\mathrm{J}\;\text{später wieder hinaus}"), np.array([bx + bw * 0.8, 0.5, 0.0]),
                    size=LABEL_FONT_SIZE, color=P_ORANGE, edge="right")
        lost_leader = Line(np.array([bx + bw * 0.75, 0.38, 0.0]), np.array([bx + bw * 0.75, 0.24, 0.0]),
                           color=P_ORANGE, stroke_width=1.6, stroke_opacity=0.8)
        refl_arrow = Arrow(np.array([room["glass_x"] - 0.05, mid_y + 0.3, 0.0]),
                           np.array([room["glass_x"] - 0.72, mid_y + 0.97, 0.0]), buff=0, color=P_BLUE,
                           stroke_width=4, stroke_opacity=0.9, max_tip_length_to_length_ratio=0.2)
        refl_tag = Text(tr("Reflection", "Reflexion"), font_size=LABEL_FONT_SIZE - 2, color=P_BLUE)
        refl_tag.move_to(refl_arrow.get_end() + UP * 0.33)
        row, box, items = _panel([
            ("in", r"E_{\mathrm{ein}}", P_CYAN), (None, "=", P_WHITE), ("out", r"E_{\mathrm{aus}}", P_RED),
            (None, "+", P_WHITE), ("st", r"\Delta E_{\mathrm{Speicher}}", P_ORANGE),
        ])
        self.play(FadeOut(VGroup(forms[2], form_lbls[2], form_brace, form_sum, law_lamp["group"], law_glow)),
                  room["air"].animate.set_fill(P_RED, opacity=0.0), run_time=0.6)
        self.play(FadeIn(e_in), FadeIn(in_bar), FadeIn(in_t), run_time=1.4)
        ceiling_heat = room["ceiling"].copy().set_fill(P_ORANGE, opacity=0)
        room["group"].add(ceiling_heat)
        self.add(ceiling_heat)
        self.play(FadeIn(e_out), FadeIn(out_bar), FadeIn(st_bar), FadeIn(out_t), FadeIn(st_t),
                  ceiling_heat.animate.set_fill(opacity=0.55), run_time=1.4)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        hold_for(self, N, "balance")

        caption = swap_caption(self, caption, subtitle_text(N, "losses"))
        self.play(GrowArrow(refl_arrow), FadeIn(refl_tag), FadeIn(refl_bar), FadeIn(refl_t), run_time=1.6)
        self.play(FadeIn(st_lost), FadeIn(lost_t), Create(lost_leader), run_time=1.6)
        hold_for(self, N, "losses", during=lambda rt: [
            ShowPassingFlash(refl_arrow.copy().set_stroke(width=8), time_width=0.6, run_time=rt),
            Indicate(st_lost, color=P_ORANGE, scale_factor=1.0, run_time=rt)])

        self.play(FadeOut(VGroup(e_in, e_out, in_t, out_t, st_t, row, box, refl_arrow, refl_tag, refl_bar, refl_t, st_lost,
                                 lost_t, lost_leader)), FadeOut(in_bar), FadeOut(out_bar), FadeOut(st_bar), ceiling_heat.animate.set_fill(opacity=0), run_time=0.6)
        caption = swap_caption(self, caption, subtitle_text(N, "lamp"))
        lamp = _lamp(np.array([room["center"][0], room["y_c"], 0.0]))
        p_el = ValueTracker(0.0)
        p_read = _readout(lambda: rf"P_{{\mathrm{{el}}}} = {label_number(p_el.get_value())}\,\mathrm{{W}}",
                          lamp["cable"].get_center() + RIGHT * 0.3, color=P_CYAN, edge="left")
        self.play(FadeIn(lamp["group"]), run_time=0.6)
        _reveal(self, p_read)
        _paced_flow(self, [([Line(lamp["cable"].get_start(), lamp["cable"].get_end())], P_CYAN, None, None, False)],
                    run_time=1.6, waves=3, cycles=2.0, radius=0.045, extra=[p_el.animate.set_value(60)])
        hold_for(self, N, "lamp")

        caption = swap_caption(self, caption, subtitle_text(N, "split"))
        src_x, lane_x0, lane_len, top_y, total_h = 1.5, 1.85, 2.5, 1.25, 1.9
        f_l, f_h = ValueTracker(0.0), ValueTracker(0.0)
        src = Rectangle(width=0.45, height=total_h, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.6)
        src.move_to(np.array([src_x, top_y - total_h / 2, 0.0]))
        light_h, heat_h = total_h * 0.05, total_h * 0.95
        light_lane = always_redraw(lambda: Rectangle(
            width=max(0.002, lane_len * f_l.get_value()), height=light_h, stroke_width=0, fill_color=P_YELLOW,
            fill_opacity=0.8).move_to(np.array([lane_x0 + lane_len * f_l.get_value() / 2, top_y - light_h / 2, 0.0])))
        heat_lane = always_redraw(lambda: Rectangle(
            width=max(0.002, lane_len * f_h.get_value()), height=heat_h, stroke_width=0, fill_color=P_RED,
            fill_opacity=0.45).move_to(np.array([lane_x0 + lane_len * f_h.get_value() / 2,
                                                 top_y - light_h - 0.08 - heat_h / 2, 0.0])))
        light_read = _readout(lambda: rf"\text{{{tr('Light', 'Licht')}}}\;{label_number(3 * f_l.get_value())}\,\mathrm{{W}}",
                              np.array([lane_x0, top_y + 0.22, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW)
        heat_mid_y = top_y - light_h - 0.08 - heat_h / 2

        def riding_heat_label():
            edge = lane_x0 + lane_len * f_h.get_value()
            label = _m(rf"\text{{{tr('Heat', 'Wärme')}}}\;{label_number(57 * f_h.get_value())}\,\mathrm{{W}}", size=LABEL_FONT_SIZE,
                       color=P_WHITE)
            label.move_to(np.array([edge - 0.15 - label.width / 2, heat_mid_y, 0.0]))
            if edge - lane_x0 < label.width + 0.35:
                label.set_opacity(0.0)
            return label

        heat_read = always_redraw(riding_heat_label)
        src_lbl = _m(r"60\,\mathrm{W}", np.array([src_x, top_y - total_h - 0.35, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_CYAN)
        bulb_c = lamp["bulb"].get_center()
        light_rays = VGroup(*[
            Line(bulb_c, end, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.85)
            for end in (np.array([room["x_l"] + 0.05, bulb_c[1] - 0.3, 0.0]),
                        np.array([room["x_l"] + 1.0, room["y_f"] + 0.02, 0.0]),
                        np.array([room["center"][0] + 0.4, room["y_f"] + 0.02, 0.0]),
                        np.array([room["x_r"] - 0.6, room["y_f"] + 0.02, 0.0]),
                        np.array([room["x_r"] - 0.05, bulb_c[1] - 0.6, 0.0]))
        ])
        heat_waves = Circle(radius=0.3, stroke_width=0, fill_color=P_RED, fill_opacity=0.25).move_to(bulb_c)
        self.play(FadeIn(src), FadeIn(src_lbl), run_time=0.6)
        self.add(light_lane, heat_lane)
        _reveal(self, light_read, heat_read)
        self.play(f_l.animate.set_value(1.0), f_h.animate.set_value(1.0), FadeIn(heat_waves, scale=0.4),
                  _ripples([bulb_c], r_max=0.7, color=P_RED, down=True),
                  LaggedStart(*[Create(r) for r in light_rays], lag_ratio=0.1), run_time=2.0)
        p_row, p_box, p_items = _panel([
            ("el", r"P_{\mathrm{el}}", P_CYAN), (None, "=", P_WHITE), ("l", r"P_{\mathrm{Licht}}", P_YELLOW),
            (None, "+", P_WHITE), ("w", r"P_{\mathrm{Wärme}}", P_RED), (None, "=", P_WHITE),
            ("v", r"3\,\mathrm{W} + 57\,\mathrm{W}", P_WHITE),
        ])
        self.play(FadeIn(p_row), Create(p_box), run_time=0.8)
        hold_for(self, N, "split")

        caption = swap_caption(self, caption, subtitle_text(N, "absorb"))
        flashes = [Flash(r.get_end(), color=P_ORANGE, flash_radius=0.18, line_length=0.12) for r in light_rays]
        self.play(LaggedStart(*flashes, lag_ratio=0.15), light_rays.animate.set_color(P_ORANGE), run_time=1.6)
        self.remove(light_lane)
        merged_light = Rectangle(width=lane_len, height=light_h, stroke_width=0, fill_color=P_YELLOW,
                                 fill_opacity=0.8).move_to(np.array([lane_x0 + lane_len / 2, top_y - light_h / 2, 0.0]))
        self.add(merged_light)
        light_heat = _m(rf"\text{{{tr('Light', 'Licht')}}} \to \text{{{tr('Heat', 'Wärme')}}}\;3\,\mathrm{{W}}", np.array([lane_x0, top_y + 0.22, 0.0]),
                        size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        self.play(merged_light.animate.set_fill(P_RED, opacity=0.45), FadeOut(light_rays),
                  ReplacementTransform(light_read, light_heat), run_time=1.0)
        hold_for(self, N, "absorb")

        caption = swap_caption(self, caption, subtitle_text(N, "total"))
        sink = Rectangle(width=0.45, height=total_h, stroke_width=0, fill_color=P_RED, fill_opacity=0.65)
        sink.move_to(np.array([lane_x0 + lane_len + 0.3, top_y - total_h / 2, 0.0]))
        sink_lbl = _m(rf"60\,\mathrm{{W}}\;\text{{{tr('Heat', 'Wärme')}}}", np.array([sink.get_right()[0], top_y - total_h - 0.35, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_RED, edge="right")
        tot_row, tot_box, _ = _panel([
            (None, r"P_{\mathrm{el}}", P_CYAN), (None, "=", P_WHITE), (None, r"P_{\mathrm{Wärme}}", P_RED),
            (None, "=", P_WHITE), (None, r"60\,\mathrm{W}", P_RED),
        ])
        self.play(GrowFromEdge(sink, LEFT), FadeIn(sink_lbl), run_time=0.9)
        _swap_panel(self, (p_row, p_box), (tot_row, tot_box))
        self.play(room["air"].animate.set_fill(P_RED, opacity=0.1), _ripples([bulb_c], r_max=1.1, color=P_RED, down=True),
                  run_time=1.6)
        hold_for(self, N, "total")

        caption = swap_caption(self, caption, subtitle_text(N, "person"))
        person_h = 1.75 * room_metre(room["h"])
        person = _person("shared-person-winter", np.array([room["x_r"] - 0.9, room["y_f"], 0.0]), person_h)
        head = person.get_top()
        plume = [smooth_path([head + UP * 0.08, head + UP * 0.45 + RIGHT * 0.1, head + UP * 0.8 + LEFT * 0.05,
                              np.array([head[0] + 0.1, room["y_c"] - 0.08, 0.0])])]
        person_lbl = _m(r"\approx 100\,\mathrm{W}", person.get_left() + LEFT * 0.2 + UP * 0.05, size=LABEL_FONT_SIZE,
                        color=P_ORANGE, edge="right")
        self.play(FadeIn(person), FadeIn(person_lbl), run_time=0.6)
        animate_flows(self, [(plume, P_ORANGE, P_RED)], run_time=1.8, waves=4, cycles=1.5, radius=0.055,
                      extra=[_ripples([person.get_center() + UP * 0.2], r_max=0.6, color=P_ORANGE)])
        hold_for(self, N, "person")

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        self.play(FadeOut(VGroup(src, src_lbl, merged_light, sink, sink_lbl, tot_row, tot_box, heat_waves, light_heat)),
                  FadeOut(heat_lane), FadeOut(heat_read), run_time=0.6)
        loss_arrows = VGroup(*[
            Arrow(np.array([room["x_l"] + 0.3, y, 0.0]), np.array([room["x_l"] - 1.3, y, 0.0]), buff=0,
                  color=P_BLUE, stroke_width=3, max_tip_length_to_length_ratio=0.18)
            for y in (room["y_f"] + 0.25, room["y_c"] - 0.2)
        ] + [
            Arrow(np.array([room["x_r"] - 0.2, room["center"][1], 0.0]),
                  np.array([room["x_r"] + 1.0, room["center"][1], 0.0]), buff=0, color=P_BLUE, stroke_width=3,
                  max_tip_length_to_length_ratio=0.18)
        ])
        base_y, unit = -1.15, 0.0021
        f_int = ValueTracker(0.0)
        wx = 2.2

        def winter_bars():
            gi = 200 * f_int.get_value()
            heat = 1200 - gi
            low = Rectangle(width=0.8, height=max(0.002, gi * unit), stroke_width=0, fill_color=P_ORANGE,
                            fill_opacity=0.75).move_to(np.array([wx, base_y + gi * unit / 2, 0.0]))
            high = Rectangle(width=0.8, height=heat * unit, stroke_width=0, fill_color=P_RED, fill_opacity=0.6).move_to(
                np.array([wx, base_y + gi * unit + heat * unit / 2, 0.0]))
            return VGroup(low, high)

        w_bars = always_redraw(winter_bars)
        w_frame = Rectangle(width=0.8, height=1200 * unit, color=P_BLUE, stroke_width=2).move_to(
            np.array([wx, base_y + 1200 * unit / 2, 0.0]))
        w_title = Text("Winter", font_size=BODY_FONT_SIZE, color=P_BLUE).next_to(w_frame, UP, buff=0.15)
        heat_read2 = _readout(lambda: tr(rf"\text{{Heating}}\;{label_number(1200 - 200 * f_int.get_value())}\,\mathrm{{W}}", rf"\text{{Heizung}}\;{label_number(1200 - 200 * f_int.get_value())}\,\mathrm{{W}}"),
                              np.array([wx + 0.55, base_y + 1.6, 0.0]), size=LABEL_FONT_SIZE, color=P_RED)
        int_read = _readout(lambda: tr(rf"\text{{internal}}\;{label_number(200 * f_int.get_value())}\,\mathrm{{W}}", rf"\text{{intern}}\;{label_number(200 * f_int.get_value())}\,\mathrm{{W}}"),
                            np.array([wx + 0.55, base_y + 0.15, 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        loss_lbl = _m(tr(r"\text{Losses}\;1\,200\,\mathrm{W}", r"\text{Verluste}\;1\,200\,\mathrm{W}"), np.array([wx, base_y - 0.35, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_BLUE)
        self.play(LaggedStart(*[GrowArrow(a) for a in loss_arrows], lag_ratio=0.2), FadeIn(w_frame), FadeIn(w_title),
                  FadeIn(loss_lbl), run_time=1.0)
        self.add(w_bars)
        _reveal(self, heat_read2, int_read)
        self.play(f_int.animate.set_value(1.0), Indicate(person, color=P_ORANGE), Indicate(lamp["bulb"]),
                  run_time=2.0)
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        sx = 5.1
        f_sun = ValueTracker(0.0)
        sun_c = np.array([-5.8, 2.05, 0.0])
        sun = _build_sun(sun_c).scale(0.32)

        def summer_bars():
            s = 500 * f_sun.get_value()
            low = Rectangle(width=0.8, height=200 * unit, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.75)
            low.move_to(np.array([sx, base_y + 200 * unit / 2, 0.0]))
            high = Rectangle(width=0.8, height=max(0.002, s * unit), stroke_width=0, fill_color=P_YELLOW,
                             fill_opacity=0.7).move_to(np.array([sx, base_y + 200 * unit + s * unit / 2, 0.0]))
            return VGroup(low, high)

        s_bars = always_redraw(summer_bars)
        s_title = Text(tr("Summer", "Sommer"), font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(
            np.array([sx, w_title.get_center()[1], 0.0]))
        cool_read = _readout(lambda: tr(rf"\text{{Cooling load}}\;{label_number(200 + 500 * f_sun.get_value())}\,\mathrm{{W}}", rf"\text{{Kühllast}}\;{label_number(200 + 500 * f_sun.get_value())}\,\mathrm{{W}}"),
                             np.array([sx, base_y - 0.35, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="center")
        rays = _sun_rays(sun_c, room["glass_x"], [room["win_hi"] - 0.2 - 0.35 * k for k in range(3)], room["y_f"],
                         gap=0.42)
        self.play(FadeOut(loss_arrows), FadeIn(sun), FadeIn(s_title), run_time=0.7)
        self.add(s_bars)
        _reveal(self, cool_read)
        self.play(_shine(rays), f_sun.animate.set_value(1.0), room["air"].animate.set_fill(P_RED, opacity=0.12),
                  run_time=2.0)
        hold_for(self, N, "summer")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Heat pump: lifting energy to a higher temperature level
class Beat4_Waermepumpe(Scene):
    NARRATION = [
        ("move",
         "A heat pump does not create heat. It transfers energy from the cold outdoor air to a higher temperature level, into the warm house.",
         "Eine Wärmepumpe erzeugt keine Wärme. Sie überträgt Energie aus der kalten Außenluft auf ein höheres Temperaturniveau — ins warme Haus."),
        ("evap",
         "In the evaporator a very cold refrigerant takes up heat from the outdoor air and evaporates.",
         "Im Verdampfer nimmt ein sehr kaltes Kältemittel Wärme aus der Außenluft auf und verdampft."),
        ("comp",
         "The compressor squeezes the vapour — this needs electricity. Pressure and temperature rise.",
         "Der Verdichter presst den Dampf zusammen — dafür braucht er Strom. Druck und Temperatur steigen."),
        ("cond",
         "In the condenser the hot refrigerant gives its heat to the heating water and turns liquid again.",
         "Im Verflüssiger gibt das heiße Kältemittel seine Wärme ans Heizwasser ab und wird wieder flüssig."),
        ("valve",
         "The expansion valve lowers the pressure, the refrigerant becomes very cold again, and the cycle restarts.",
         "Das Expansionsventil senkt den Druck, das Kältemittel wird wieder sehr kalt — der Kreislauf beginnt neu."),
        ("balance",
         "The balance: one kilowatt-hour of electricity plus three of ambient heat gives four of heating heat.",
         "Die Bilanz: 1 kWh Strom plus 3 kWh Umweltwärme ergeben 4 kWh Heizwärme."),
        ("cop",
         "Heat divided by electricity is the coefficient of performance, COP — here four.",
         "Heizwärme durch Strom ist die Leistungszahl COP — hier 4."),
        ("temp",
         "The colder the outdoor air, the more the compressor has to lift — the COP drops.",
         "Je kälter die Außenluft, desto mehr muss der Verdichter leisten — der COP sinkt."),
        ("jaz",
         "Averaged over a year this is the seasonal performance factor JAZ — for air-source units often around three.",
         "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl JAZ — bei Luft-Wärmepumpen oft rund 3."),
        ("final",
         "Final energy is what is delivered to the building and paid for: fuel in the tank, electricity at the meter.",
         "Endenergie ist, was ans Haus geliefert und bezahlt wird: Brennstoff im Tank, Strom am Zähler."),
        ("compare1",
         "Per kilowatt-hour of final energy: wood chips give about 0.85 kilowatt-hours of heat, oil about 0.94, gas about 0.96.",
         "Je 1 kWh Endenergie liefern Holzhackschnitzel rund 0,85 kWh Wärme, Heizöl rund 0,94, Erdgas rund 0,96."),
        ("compare2",
         "Electric heater: exactly one at the house. Fuels lose the rest with flue gas; electricity may cause flue gas elsewhere.",
         "Elektroheizstab: am Haus genau 1. Brennstoffe verlieren den Rest mit dem Abgas; Strom kann anderswo Abgas verursachen."),
        ("power",
         "But electricity is often made from fuels too: then the flue gas arises at the power station, not in the house.",
         "Auch Strom entsteht oft aus Brennstoffen: Dann entsteht das Abgas im Kraftwerk, nicht im Haus."),
        ("compare3",
         "The air-source heat pump gives about three, the ground-source about four — because it adds ambient heat.",
         "Die Luft-Wärmepumpe liefert rund 3, die Erd-Wärmepumpe rund 4 — weil sie Umweltwärme dazuholt."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Heat pump — lifting energy to a higher temperature level", "Wärmepumpe — Energie auf höheres Temperaturniveau übertragen"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "move"))
        self.play(FadeIn(caption), run_time=0.3)

        comp_c, valve_c = np.array([-1.3, 1.3, 0.0]), np.array([-1.3, -0.55, 0.0])

        def serpentine(face, sign, y_in, y_out):
            far, near = face + sign * 0.75, face + sign * 0.15
            ys = np.linspace(y_in, y_out, 4)
            xs = [(face, far), (far, near), (near, far), (far, face)]
            pts = []
            for y, (x0, x1) in zip(ys, xs):
                pts += [np.array([x0, y, 0.0]), np.array([x1, y, 0.0])]
            return [p for k, p in enumerate(pts) if k == 0 or np.linalg.norm(p - pts[k - 1]) > 1e-9]

        evap_pts, cond_pts = serpentine(-2.75, -1, -0.55, 1.3), serpentine(2.55, 1, 1.3, -0.55)
        loop_pts = [*evap_pts, comp_c, *cond_pts, valve_c, evap_pts[0]]
        loop = VMobject(stroke_width=3, color=P_WHITE, stroke_opacity=0.55)
        loop.set_points_as_corners(loop_pts)
        reach = np.concatenate([[0.0], np.cumsum([np.linalg.norm(b - a) for a, b in zip(loop_pts, loop_pts[1:])])])
        reach /= reach[-1]
        n_e, n_c = len(evap_pts), len(cond_pts)
        at = {"tl": reach[n_e - 1], "comp": reach[n_e], "tr": reach[n_e + 1], "br": reach[n_e + n_c],
              "valve": reach[n_e + n_c + 1]}
        loop_path = loop.copy()
        wall = DashedLine(np.array([0.6, -0.85, 0.0]), np.array([0.6, 2.1, 0.0]), color=P_WHITE, stroke_width=2,
                          dash_length=0.12)
        out_tag = Text(tr("outside", "außen"), font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(np.array([0.0, 2.15, 0.0]))
        in_tag = Text(tr("inside", "innen"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(np.array([1.25, 2.15, 0.0]))
        cold_bg = Rectangle(width=7.5, height=2.95, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.06).move_to(
            np.array([-3.15, 0.62, 0.0]))

        evap = _fitted_staffage("physics-fundamentals-evaporator", width=0.9, height=2.25).move_to(np.array([-3.2, 0.375, 0.0]))
        cond = _fitted_staffage("physics-fundamentals-condenser", width=0.9, height=2.25).move_to(np.array([3.0, 0.375, 0.0]))
        comp = _fitted_staffage("physics-fundamentals-compressor", height=0.9, rate=0.0, loop=True).move_to(comp_c)
        valve = _fitted_staffage("physics-fundamentals-valve", width=0.9).move_to(valve_c)
        lbl_evap = Text(tr("Evaporator", "Verdampfer"), font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(np.array([-4.6, 1.8, 0.0]))
        lbl_comp = Text(tr("Compressor", "Verdichter"), font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([-1.3, 0.72, 0.0]))
        lbl_cond = Text(tr("Condenser", "Verflüssiger"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([1.6, -0.1, 0.0]))
        lbl_valve = Text(tr("Expansion valve", "Expansionsventil"), font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(
            np.array([-1.3, -1.1, 0.0]))
        air_paths = [smooth_path([np.array([-6.6, y, 0.0]), np.array([-3.7, y, 0.0])]) for y in (-0.15, 0.4, 0.95)]
        air_rays = flow_guides(air_paths, P_GREEN, opacity=0.3)
        air_lbl = _m(tr(r"\text{Outdoor air}\;0\,\mathrm{°C}", r"\text{Außenluft}\;0\,\mathrm{°C}"), np.array([-5.1, -0.6, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_GREEN)
        power = Arrow(np.array([-1.3, 2.3, 0.0]), comp.get_top(), buff=0.02, color=P_CYAN, stroke_width=5,
                      max_tip_length_to_length_ratio=0.3)
        power_lbl = Text(tr("Electricity", "Strom"), font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(power, LEFT, buff=0.12)
        radiator = _staffage("shared-radiator-front-winter", unit=FE_RADIATOR_METRE, anchor=np.array([5.5, 0.0, 0.0]))
        feed = [np.array(q, dtype=float) for q in ((3.45, 0.75, 0.0), (4.75, 0.75, 0.0), (4.75, 0.03, 0.0), (4.9, 0.03, 0.0))]
        back = [np.array(q, dtype=float) for q in ((6.1, 0.03, 0.0), (6.25, 0.03, 0.0), (6.25, -0.3, 0.0), (3.9, -0.3, 0.0),
                                                    (3.9, 0.0, 0.0), (3.45, 0.0, 0.0))]
        hot_rays = VGroup(VMobject(stroke_width=3, color=P_RED).set_points_as_corners(feed),
                          VMobject(stroke_width=3, color=P_ORANGE).set_points_as_corners(back))
        water_path = VMobject().set_points_as_corners(feed + back + feed[:1])
        flows = {"air": False, "water": False}
        rad_lbl = _m(rf"\text{{{tr('Heating water', 'Heizwasser')}}}\;35\,\mathrm{{°C}}", np.array([5.2, -0.7, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_RED)
        refrigerant = _gradient([(0.0, P_BLUE), (at["tl"] + 0.015, P_CYAN), (at["comp"], P_CYAN),
                                 (at["comp"] + 0.04, P_RED), (at["tr"], P_RED), (at["br"] + 0.015, P_ORANGE),
                                 (at["valve"] - 0.04, P_TEAL), (at["valve"], P_BLUE), (1.0, P_BLUE)])

        def cycle(run_time=3.0, extra=None):
            streams = [([loop_path], P_CYAN, None, refrigerant, True)]
            if flows["air"]:
                streams.append((air_paths, P_GREEN, None, _gradient([(0.0, P_GREEN), (0.8, P_GREEN), (1.0, P_BLUE)]),
                                False, 4))
            if flows["water"]:
                streams.append(([water_path], P_RED, None, _gradient([(0.0, P_RED), (0.3, P_RED), (0.5, P_ORANGE),
                                                                       (1.0, P_ORANGE)]), True, 8))
            _paced_flow(self, streams, run_time=run_time, waves=22, cycles=run_time / 3.0, radius=0.07, extra=extra)

        self.play(FadeIn(cold_bg), Create(wall), FadeIn(out_tag), FadeIn(in_tag), Create(loop), FadeIn(evap),
                  FadeIn(cond), FadeIn(comp), FadeIn(valve), FadeIn(radiator), run_time=1.4)
        cycle(2.4)
        hold_for(self, N, "move", used=BEAT_SUBTITLE_FADE + 0.3 + 3.8)

        caption = swap_caption(self, caption, subtitle_text(N, "evap"))
        self.play(FadeIn(lbl_evap), FadeIn(air_lbl), FadeIn(air_rays), Indicate(evap, color=P_CYAN), run_time=1.2)
        flows["air"] = True
        cycle()
        hold_for(self, N, "evap")

        caption = swap_caption(self, caption, subtitle_text(N, "comp"))
        self.play(FadeIn(lbl_comp), GrowArrow(power), FadeIn(power_lbl), run_time=0.9)
        comp.rate = 1.0
        cycle()
        hold_for(self, N, "comp")

        caption = swap_caption(self, caption, subtitle_text(N, "cond"))
        self.play(FadeIn(lbl_cond), LaggedStart(*[Create(r) for r in hot_rays], lag_ratio=0.2), FadeIn(rad_lbl),
                  Indicate(cond, color=P_RED), run_time=1.2)
        flows["water"] = True
        cycle()
        hold_for(self, N, "cond")

        caption = swap_caption(self, caption, subtitle_text(N, "valve"))
        self.play(FadeIn(lbl_valve), Indicate(valve, color=P_BLUE), run_time=0.9)
        cycle()
        hold_for(self, N, "valve")

        caption = swap_caption(self, caption, subtitle_text(N, "balance"))
        unit, base = 0.55, -1.0
        self.play(FadeOut(radiator), FadeOut(rad_lbl), FadeOut(hot_rays), run_time=0.5)
        flows["water"] = False

        def block(x, h, color, width=0.55, y0=base):
            return Rectangle(width=width, height=h, stroke_width=1.5, color=color, fill_color=color,
                             fill_opacity=0.65).move_to(np.array([x, y0 + h / 2, 0.0]))

        stack_x = 5.9
        w_bar = block(stack_x, unit, P_CYAN, width=0.62)
        u_bar = block(stack_x, 3 * unit, P_GREEN, width=0.62, y0=base + unit)
        w_lbl = _m(r"W_{\mathrm{el}} = 1\,\mathrm{kWh}", np.array([stack_x - 0.42, base + unit / 2 - 0.07, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")
        u_lbl = _m(r"Q_{\mathrm{U}} = 3\,\mathrm{kWh}", np.array([stack_x - 0.42, base + 2.5 * unit - 0.07, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_GREEN, edge="right")
        self.play(Indicate(power, color=P_CYAN), GrowFromEdge(w_bar, DOWN), FadeIn(w_lbl), run_time=0.9)
        self.play(Indicate(air_rays, color=P_GREEN), GrowFromEdge(u_bar, DOWN), FadeIn(u_lbl), run_time=0.9)
        h_frame = Rectangle(width=0.72, height=4 * unit + 0.1, color=P_RED, stroke_width=3).move_to(
            np.array([stack_x, base + 2 * unit, 0.0]))
        h_lbl = _m(r"Q_{\mathrm{H}} = 4\,\mathrm{kWh}", np.array([stack_x - 0.42, h_frame.get_top()[1] + 0.05, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_RED, edge="right")
        self.play(Create(h_frame), FadeIn(h_lbl), run_time=0.7)
        hold_for(self, N, "balance")

        caption = swap_caption(self, caption, subtitle_text(N, "cop"))
        tower = VGroup(*[block(6.7, unit, P_CYAN, width=0.4, y0=base + k * unit) for k in range(4)])
        count = ValueTracker(0)
        count_read = _readout(lambda: rf"\times\,{int(count.get_value())}", np.array([6.7, base + 4 * unit + 0.25, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        _reveal(self, count_read)
        for k in range(4):
            self.play(TransformFromCopy(w_bar, tower[k]), count.animate.set_value(k + 1), run_time=0.45)
        row, box, items = _panel([
            (None, r"\mathrm{COP}", P_PRIMARY), (None, "=", P_WHITE),
            ("f", r"\frac{\textcolor{semio_primary}{Q_{\mathrm{H}}}}{\textcolor{semio_secondary}{W_{\mathrm{el}}}}".replace("semio_primary", semio_color("primary")).replace("semio_secondary", semio_color("secondary")), P_WHITE),
            (None, "=", P_WHITE), (None, r"\frac{4\,\mathrm{kWh}}{1\,\mathrm{kWh}}", P_WHITE), (None, "=", P_WHITE),
            ("v", "4", P_YELLOW),
        ])
        self.play(FadeIn(row), Create(box), run_time=0.9)
        hold_for(self, N, "cop")

        caption = swap_caption(self, caption, subtitle_text(N, "temp"))
        self.play(FadeOut(tower), FadeOut(count_read), FadeOut(w_bar), FadeOut(u_bar),
                  FadeOut(w_lbl), FadeOut(u_lbl), FadeOut(h_lbl), FadeOut(air_lbl), FadeOut(row), FadeOut(box),
                  run_time=0.6)
        t_out = ValueTracker(10.0)

        def cop_of(t):
            return 3.0 + 0.08 * t

        thermo = _thermometer(np.array([-5.9, -0.4, 0.0]), height=1.6, color=P_BLUE, level=0.5)
        thermo["level"].add_updater(lambda m: m.set_value((t_out.get_value() + 15) / 30))
        t_read = _readout(lambda: rf"{label_number(t_out.get_value())}\,\mathrm{{°C}}", np.array([-5.55, 0.75, 0.0]),
                          size=LABEL_FONT_SIZE, color=P_BLUE)

        def live_stack():
            w = 4.0 / cop_of(t_out.get_value())
            low = block(stack_x, w * unit, P_CYAN, width=0.62)
            high = block(stack_x, (4.0 - w) * unit, P_GREEN, width=0.62, y0=base + w * unit)
            return VGroup(low, high)

        stack = always_redraw(live_stack)
        cop_read = _readout(lambda: rf"\mathrm{{COP}} \approx {label_number(cop_of(t_out.get_value()), 1)}",
                            np.array([stack_x, base + 4 * unit + 0.3, 0.0]), color=P_YELLOW, edge="center")
        w_live = _readout(lambda: rf"W_{{\mathrm{{el}}}} = {label_number(4.0 / cop_of(t_out.get_value()), 2)}\,\mathrm{{kWh}}",
                          np.array([stack_x - 0.42, base + 0.2, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")
        self.play(FadeOut(air_rays), FadeIn(thermo["group"]), FadeIn(thermo["column"]), run_time=0.6)
        self.add(stack)
        _reveal(self, cop_read, t_read, w_live)
        self.bring_to_front(h_frame)
        self.play(t_out.animate.set_value(-10.0), run_time=3.0, rate_func=linear)
        hold_for(self, N, "temp")

        caption = swap_caption(self, caption, subtitle_text(N, "jaz"))
        self.play(t_out.animate.set_value(0.0), run_time=1.5)
        jaz = _m(tr(r"\mathrm{SPF} \approx 3 \quad \text{(Bohne 2022)}", r"\mathrm{JAZ} \approx 3 \quad \text{(Bohne 2022)}"), np.array([stack_x - 0.9, 1.95, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_PRIMARY)
        self.play(FadeIn(jaz, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, N, "jaz")

        caption = swap_caption(self, caption, subtitle_text(N, "final"))
        thermo["level"].clear_updaters()
        self.play(*[FadeOut(m) for m in self.mobjects if m not in (subtitle, caption)], run_time=0.7)
        fe_house = _build_cross_section_house(center=np.array([1.8, -0.3, 0.0]))
        fe_limit_x = -0.9
        fe_limit = DashedLine(np.array([fe_limit_x, -1.7, 0.0]), np.array([fe_limit_x, 1.9, 0.0]), color=P_WHITE,
                              stroke_width=2, dash_length=0.12)
        fe_limit_tag = Text(tr("Building boundary", "Gebäudegrenze"), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(fe_limit, UP, buff=0.1)
        fe_fuel = Arrow(np.array([-5.6, 0.6, 0.0]), np.array([fe_limit_x - 0.05, 0.6, 0.0]), buff=0, color=P_ORANGE,
                        stroke_width=5)
        fe_power = Arrow(np.array([-5.6, -0.6, 0.0]), np.array([fe_limit_x - 0.05, -0.6, 0.0]), buff=0, color=P_CYAN,
                         stroke_width=5)
        fe_fuel_lbl = Text(tr("Fuel in the tank", "Brennstoff im Tank"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(fe_fuel, UP, buff=0.12,
                                                                                                   aligned_edge=LEFT)
        fe_power_lbl = Text(tr("Electricity at the meter", "Strom am Zähler"), font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(fe_power, UP, buff=0.12,
                                                                                               aligned_edge=LEFT)
        fe_brace = Brace(VGroup(fe_fuel, fe_fuel_lbl), UP, color=P_YELLOW).stretch_to_fit_width(4.6, about_edge=RIGHT)
        fe_brace.move_to(np.array([(-5.6 + fe_limit_x) / 2, fe_fuel_lbl.get_top()[1] + 0.3, 0.0]))
        fe_lbl = Text(tr("Final energy", "Endenergie"), font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(fe_brace, UP, buff=0.12)
        fe_metre = room_metre(fe_house["w_height"] / 2)
        fe_floor = fe_house["bottom_left"][1]
        fe_heater = _staffage("shared-radiator-front-winter", unit=fe_metre, anchor=np.array([2.35, fe_floor, 0.0]))
        fe_heat_lbl = Text(tr("Useful heat", "Nutzwärme"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([1.8, 0.45, 0.0]))
        self.play(FadeIn(fe_house["group"]), Create(fe_limit), FadeIn(fe_limit_tag), run_time=1.0)
        self.play(GrowArrow(fe_fuel), FadeIn(fe_fuel_lbl), GrowArrow(fe_power), FadeIn(fe_power_lbl), run_time=1.4)
        self.play(GrowFromCenter(fe_brace), FadeIn(fe_lbl), run_time=0.8)
        self.play(FadeIn(fe_heater), FadeIn(fe_heat_lbl),
                  _ripples([fe_heater.get_top() + UP * 0.04], r_max=0.7, color=P_RED), run_time=2.0)
        hold_for(self, N, "final", during=lambda rt: [
            _ripples([fe_heater.get_top() + UP * 0.04], r_max=0.7, color=P_RED, cycles=max(1.0, rt / 1.4))])

        self.play(*[FadeOut(m) for m in self.mobjects if m not in (subtitle, caption)], run_time=0.7)
        caption = swap_caption(self, caption, subtitle_text(N, "compare1"))
        cb, cu = -1.3, 0.74
        xs = [-4.8, -2.9, -1.0, 0.9, 2.8, 4.7]
        specs = [
            (tr("Wood\nchips", "Holz-\nhackschnitzel"), 0.85, "fuel"), (tr("Fuel oil\ncondensing", "Heizöl\nBrennwert"), 0.94, "fuel"),
            (tr("Natural gas\ncondensing", "Erdgas\nBrennwert"), 0.96, "fuel"), (tr("Electric\nheating rod", "Elektro-\nheizstab"), 1.0, "el"),
            (tr("Air HP\n(SPF)", "Luft-WP\n(JAZ)"), 3.0, "hp"), (tr("Ground HP\n(SPF)", "Erd-WP\n(JAZ)"), 4.0, "hp"),
        ]
        base_line = Line(np.array([-5.3, cb, 0.0]), np.array([5.9, cb, 0.0]), color=P_WHITE, stroke_width=2)
        one_line = DashedLine(np.array([-5.3, cb + cu, 0.0]), np.array([5.9, cb + cu, 0.0]), color=P_WHITE,
                              stroke_width=1.6, stroke_opacity=0.6, dash_length=0.1)
        one_lbl = _m(r"1\,\mathrm{kWh}", np.array([-6.2, cb + cu - 0.06, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE,
                     edge="left")
        names = VGroup(*[
            centered_body_text(name, font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(
                np.array([x, cb, 0.0]), DOWN, buff=0.14)
            for x, (name, _v, _k) in zip(xs, specs)
        ])
        axis_lbl = Text(tr("Useful heat from 1 kWh final energy", "Nutzwärme aus 1 kWh Endenergie"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([-3.0, 2.15, 0.0]))
        legend = VGroup(*[
            VGroup(Square(side_length=0.2, stroke_width=0, fill_color=c, fill_opacity=0.75),
                   Text(t, font_size=LABEL_FONT_SIZE, color=c)).arrange(RIGHT, buff=0.12)
            for t, c in ((tr("Fuel", "Brennstoff"), P_ORANGE), (tr("Electricity", "Strom"), P_CYAN), (tr("Ambient heat", "Umweltwärme"), P_GREEN))
        ]).arrange(RIGHT, buff=0.4).move_to(np.array([3.0, 2.15, 0.0]))
        grow = [ValueTracker(0.0) for _ in specs]

        def bar_for(i):
            def build():
                x, (_n, value, kind) = xs[i], specs[i]
                v = value * grow[i].get_value()
                if kind == "hp":
                    el = min(v, 1.0)
                    parts = [block(x, max(0.002, el * cu), P_CYAN, width=0.9, y0=cb)]
                    if v > 1.0:
                        parts.append(block(x, (v - 1.0) * cu, P_GREEN, width=0.9, y0=cb + cu))
                    return VGroup(*parts)
                return VGroup(block(x, max(0.002, v * cu), P_ORANGE if kind == "fuel" else P_CYAN, width=0.9, y0=cb))
            return always_redraw(build)

        def value_for(i):
            x, (_n, value, _k) = xs[i], specs[i]
            return _m(label_number(value, 2 if value < 2 else 0), np.array([x, cb + value * cu - 0.26, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_WHITE)

        bars = [bar_for(i) for i in range(len(specs))]
        values = [value_for(i) for i in range(len(specs))]
        self.play(Create(base_line), Create(one_line), FadeIn(one_lbl), FadeIn(names), FadeIn(axis_lbl),
                  FadeIn(legend), run_time=0.9)
        self.add(*bars)
        self.play(*[grow[i].animate.set_value(1.0) for i in range(3)], run_time=1.6)
        self.play(*[FadeIn(values[i]) for i in range(3)], run_time=0.5)
        hold_for(self, N, "compare1")

        caption = swap_caption(self, caption, subtitle_text(N, "compare2"))
        losses = VGroup(*[
            DashedVMobject(Rectangle(width=0.9, height=(1 - specs[i][1]) * cu, color=P_ORANGE, stroke_width=1.6).move_to(
                np.array([xs[i], cb + (1 + specs[i][1]) / 2 * cu, 0.0])), num_dashes=14)
            for i in range(3)
        ])
        loss_lbl = Text(tr("Flue gas at the house", "Abgas am Haus"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(np.array([-3.4, cb + cu + 0.55, 0.0]))
        self.play(grow[3].animate.set_value(1.0), run_time=1.0)
        self.play(FadeIn(values[3]), run_time=0.4)
        self.play(Create(losses), FadeIn(loss_lbl), run_time=0.8)
        hold_for(self, N, "compare2")

        caption = swap_caption(self, caption, subtitle_text(N, "power"))
        plant_start, plant_end = np.array([xs[3], cb + cu + 0.08, 0.0]), np.array([xs[3], cb + cu + 1.05, 0.0])
        plant_arrow = Arrow(plant_start, plant_end, buff=0, color=P_ORANGE, stroke_width=4, stroke_opacity=0.75,
                            max_tip_length_to_length_ratio=0.25)
        plant_lbl = Text(tr("Exhaust at the power plant", "Abgas im Kraftwerk"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            plant_end + UP * 0.3)
        self.play(GrowArrow(plant_arrow), FadeIn(plant_lbl, shift=UP * 0.1), run_time=1.0)
        self.play(ShowPassingFlash(plant_arrow.copy().set_stroke(width=7), time_width=0.5), run_time=1.0)
        hold_for(self, N, "power")

        caption = swap_caption(self, caption, subtitle_text(N, "compare3"))
        self.play(grow[4].animate.set_value(1.0), run_time=1.4)
        self.play(FadeIn(values[4]), run_time=0.4)
        self.play(grow[5].animate.set_value(1.0), run_time=1.4)
        self.play(FadeIn(values[5]), run_time=0.4)
        hold_for(self, N, "compare3")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Radiation: short-wave in, long-wave trapped
class Beat5_Strahlung(Scene):
    NARRATION = [
        ("sun",
         "The sun radiates short-wave: visible light and near infrared. Glass lets most of it through.",
         "Die Sonne strahlt kurzwellig: sichtbares Licht und nahes Infrarot. Glas lässt das größtenteils durch."),
        ("g",
         "The share that passes is the g-value — for triple glazing about half.",
         "Welcher Anteil hindurchkommt, beschreibt der g-Wert — bei Dreifachverglasung rund die Hälfte."),
        ("gain",
         "Solar gain is g times window area times irradiance.",
         "Solarer Gewinn: g mal Fensterfläche mal Einstrahlung."),
        ("absorb",
         "Inside, the radiation hits floor and walls. The surfaces warm up.",
         "Drinnen trifft die Strahlung auf Boden und Wände. Die Oberflächen erwärmen sich."),
        ("long",
         "Warm surfaces radiate too — but long-wave, in the infrared.",
         "Warme Oberflächen strahlen selbst — aber langwellig, im Infrarot."),
        ("trap",
         "Glass hardly lets long-wave radiation through: plain glass absorbs it, a low-e coating reflects it back into the room.",
         "Langwellige Strahlung lässt Glas kaum durch: Vollglas nimmt sie auf, eine Low-E-Beschichtung wirft sie in den Raum zurück."),
        ("warm",
         "The energy stays trapped and the room warms up — the greenhouse effect.",
         "Die Energie bleibt gefangen, der Raum erwärmt sich — der Treibhauseffekt."),
        ("winter",
         "In winter this gain saves heating energy.",
         "Im Winter spart dieser Gewinn Heizwärme."),
        ("summer",
         "In summer it overheats the room — so shade outside, before the radiation reaches the glass.",
         "Im Sommer überhitzt er den Raum — darum Sonnenschutz außen, bevor die Strahlung das Glas erreicht."),
        ("inner",
         "With shading on the inside the radiation is already in the room: it warms the slats, and they radiate long-wave into the room.",
         "Innen liegender Sonnenschutz: Die Strahlung ist schon im Raum, erwärmt die Lamellen — sie strahlen langwellig in den Raum."),
        ("compare",
         "The comparison: with outside shading the room stays at 22 degrees, with inside shading it rises to 30 — outside works better.",
         "Der Vergleich: Außen bleibt der Raum bei 22 °C, innen steigt er auf 30 °C — außen verschattet wirkt besser."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Radiation — shortwave in, longwave trapped", "Strahlung — kurzwellig hinein, langwellig gefangen"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "sun"))
        self.play(FadeIn(caption), run_time=0.3)

        room = _room(np.array([1.0, 0.25, 0.0]), w=4.6, h=2.6, window=(0.34, 0.92))
        gx = room["glass_x"]
        sun_dir = np.array([1.0, -0.5, 0.0]) / np.linalg.norm([1.0, -0.5])
        sun_c = np.array([gx, (room["win_lo"] + room["win_hi"]) / 2, 0.0]) - sun_dir * 3.25
        sun = _build_sun(sun_c).scale(0.42)
        bounce_dir = np.array([-sun_dir[0], sun_dir[1], 0.0])

        def hit_point(g_pt):
            t_floor = (room["y_f"] + 0.02 - g_pt[1]) / sun_dir[1]
            t_wall = (room["x_r"] - 0.02 - g_pt[0]) / sun_dir[0]
            return g_pt + sun_dir * min(t_floor, t_wall)

        entries = [np.array([gx, y, 0.0]) for y in np.linspace(room["win_lo"] + 0.12, room["win_hi"] - 0.12, 8)]
        starts = [e - sun_dir * 2.3 for e in entries]
        hits = [hit_point(e) for e in entries]
        beams_out = VGroup(*[_wave([s, e], "short") for s, e in zip(starts, entries)])
        reflected = VGroup(*[_wave([e, e + bounce_dir * 0.55], "short", opacity=0.4) for e in entries])
        corner = [np.array([room["x_r"] - 0.02, room["y_f"] + 0.02, 0.0])] if hits[-1][0] > room["x_r"] - 0.05 else []
        shaft = Polygon(entries[0], hits[0], *corner, hits[-1], entries[-1], stroke_width=0, fill_color=P_YELLOW,
                        fill_opacity=0.0)
        patch = VMobject(color=P_YELLOW, stroke_width=6, stroke_opacity=0.85)
        patch.set_points_as_corners([hits[0], *corner, hits[-1]])
        pass_paths = [[starts[k], entries[k], hits[k]] for k in (1, 3, 5, 7)]
        split_paths = [[starts[k], entries[k], entries[k] + bounce_dir * 0.55] for k in (0, 2, 4, 6)]
        i_lbl = _m(r"I = 500\,\mathrm{W/m^{2}}", sun_c + LEFT * 0.4 + DOWN * 0.8, size=LABEL_FONT_SIZE,
                   color=P_YELLOW)
        g_lbl = _m(tr(r"g = 0.50", r"g = 0{,}50"), np.array([4.6, 1.95, 0.0]), color=P_YELLOW)
        gi_lbl = _m(r"g \cdot I \approx 250\,\mathrm{W/m^{2}}", np.array([gx + 0.3, room["y_c"] - 0.32, 0.0]),
                    size=LABEL_FONT_SIZE, color=P_YELLOW, edge="left")
        self.play(FadeIn(room["group"]), FadeIn(sun), run_time=1.0)
        self.play(LaggedStart(*[Create(b) for b in beams_out], lag_ratio=0.08), run_time=1.0)
        self.play(shaft.animate.set_fill(opacity=0.16), Create(patch), run_time=0.8)
        _beam_pulses(self, pass_paths, P_YELLOW, repeats=2, run_time=3.0, kind="short")
        hold_for(self, N, "sun", used=BEAT_SUBTITLE_FADE + 0.3 + 5.8)

        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        self.play(LaggedStart(*[Create(r) for r in reflected], lag_ratio=0.08),
                  Indicate(room["glass"], color=P_ORANGE, scale_factor=1.0), FadeIn(i_lbl), run_time=0.9)
        _beam_pulses(self, pass_paths + split_paths, P_YELLOW, repeats=1, run_time=2.6, kind="short",
                     extra=[FadeIn(g_lbl), FadeIn(gi_lbl, shift=RIGHT * 0.1)])
        norm_tag = Text("Willems et al. 2024", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).next_to(g_lbl, DOWN, buff=0.12).shift(RIGHT * 0.5)
        self.play(FadeIn(norm_tag), run_time=0.4)
        hold_for(self, N, "g")

        caption = swap_caption(self, caption, subtitle_text(N, "gain"))
        pane_c = np.array([5.6, 0.1, 0.0])
        pane_w, pane_h = 1.0, 0.96
        pane = Rectangle(width=pane_w, height=pane_h, color=P_CYAN, stroke_width=2.5).move_to(pane_c)
        a_frac = ValueTracker(0.0)
        pane_fill = always_redraw(lambda: Rectangle(
            width=pane_w, height=max(0.002, pane_h * a_frac.get_value()), stroke_width=0, fill_color=P_CYAN,
            fill_opacity=0.35).move_to(pane.get_bottom() + UP * pane_h * a_frac.get_value() / 2))
        w_dim = dim_arrow(pane.get_corner(DL) + DOWN * 0.18, pane.get_corner(DR) + DOWN * 0.18, color=P_WHITE)
        h_dim = dim_arrow(pane.get_corner(DL) + LEFT * 0.18, pane.get_corner(UL) + LEFT * 0.18, color=P_WHITE)
        w_lbl = _m(tr(r"1.25\,\mathrm{m}", r"1{,}25\,\mathrm{m}"), w_dim.get_center() + DOWN * 0.32, size=LABEL_FONT_SIZE)
        h_lbl = _m(tr(r"1.2\,\mathrm{m}", r"1{,}2\,\mathrm{m}"), size=LABEL_FONT_SIZE).next_to(h_dim, LEFT, buff=0.08)
        a_read = _readout(lambda: rf"A = {label_number(1.5 * a_frac.get_value(), 1)}\,\mathrm{{m^{{2}}}}",
                          pane.get_top() + UP * 0.3, size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        self.play(FadeIn(pane), GrowFromCenter(w_dim), GrowFromCenter(h_dim), FadeIn(w_lbl), FadeIn(h_lbl),
                  FadeOut(reflected), FadeOut(gi_lbl), FadeOut(norm_tag), run_time=0.9)
        self.add(pane_fill)
        _reveal(self, a_read)
        self.play(a_frac.animate.set_value(1.0), run_time=1.4)
        row, box, items = _panel([
            (None, r"\dot{Q}_{\mathrm{S}}", P_ORANGE), (None, "=", P_WHITE), ("g", "g", P_YELLOW),
            (None, r"\cdot", P_WHITE), ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE), ("i", "I", P_YELLOW),
            (None, "=", P_WHITE), ("gv", tr(r"0.5", r"0{,}5"), P_YELLOW), (None, r"\cdot", P_WHITE),
            ("av", tr(r"1.5\,\mathrm{m^{2}}", r"1{,}5\,\mathrm{m^{2}}"), P_CYAN), (None, r"\cdot", P_WHITE),
            ("iv", r"500\,\mathrm{W/m^{2}}", P_YELLOW), (None, "=", P_WHITE), ("r", r"375\,\mathrm{W}", P_ORANGE),
        ])
        _fill_panel(self, (row, box, items), {"gv": g_lbl, "av": a_read, "iv": i_lbl}, fly=False)
        hold_for(self, N, "gain")

        surface_heat = {key: room[key].copy().set_fill(P_ORANGE, opacity=0) for key in ("floor", "wall_lo", "wall_hi")}
        room["group"].add(*surface_heat.values())
        self.add(*surface_heat.values())
        caption = swap_caption(self, caption, subtitle_text(N, "absorb"))
        self.play(FadeOut(VGroup(pane, w_dim, h_dim, w_lbl, h_lbl)), FadeOut(pane_fill), FadeOut(a_read),
                  run_time=0.5)
        _beam_pulses(self, pass_paths, P_YELLOW, repeats=1, run_time=2.0, kind="short", extra=[
            LaggedStart(*[Flash(hits[k], color=P_ORANGE, flash_radius=0.12, line_length=0.08) for k in (1, 3, 5, 7)],
                        lag_ratio=0.2),
            patch.animate.set_stroke(color=P_ORANGE),
            surface_heat["floor"].animate.set_fill(opacity=0.4),
            surface_heat["wall_lo"].animate.set_fill(opacity=0.4),
            surface_heat["wall_hi"].animate.set_fill(opacity=0.4),
        ])
        hold_for(self, N, "absorb")

        caption = swap_caption(self, caption, subtitle_text(N, "long"))
        emitters = [interpolate(hits[0], hits[-1], f) + UP * 0.03 for f in (0.2, 0.5, 0.8)]
        ir_up, ir_back, up_paths, trap_paths = VGroup(), VGroup(), [], []
        for k, src in enumerate(emitters[:2]):
            g_in = np.array([gx + 0.1, room["win_lo"] + 0.3 + 0.9 * k, 0.0])
            bounce = _mirror((g_in - src) / np.linalg.norm(g_in - src), PI / 2)
            back_end = g_in + bounce * min(1.6, (room["y_c"] - 0.12 - g_in[1]) / max(0.05, bounce[1]))
            ir_up.add(_wave([src, g_in], "long", color=P_RED, opacity=0.45))
            ir_back.add(_wave([g_in, back_end], "long", color=P_RED, opacity=0.45))
            up_paths.append([src, g_in])
            trap_paths.append([src, g_in, back_end])
        lw_lbl = Text(tr("longwave", "langwellig"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(
            np.array([room["center"][0] + 0.6, room["y_c"] + room["slab"] + 0.22, 0.0]))
        sw_lbl = Text(tr("shortwave", "kurzwellig"), font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([-3.6, -0.4, 0.0]))
        self.play(shaft.animate.set_fill(opacity=0.07), LaggedStart(*[Create(b) for b in ir_up], lag_ratio=0.15),
                  FadeIn(lw_lbl), FadeIn(sw_lbl), run_time=1.0)
        _beam_pulses(self, up_paths, P_RED, repeats=2, run_time=2.6, kind="long", extra=[_ripples(emitters, r_max=0.85)])
        hold_for(self, N, "long")

        caption = swap_caption(self, caption, subtitle_text(N, "trap"))
        win_mid = (room["win_lo"] + room["win_hi"]) / 2
        lowe = Line(np.array([gx + 0.1, room["win_lo"], 0.0]), np.array([gx + 0.1, win_mid, 0.0]),
                    color=P_ORANGE, stroke_width=4)
        lowe_lbl = Text("Low-E", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([gx - 0.55, room["win_lo"] - 0.3, 0.0]))
        plain_lbl = Text(tr("Plain glass", "Vollglas"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([gx - 0.2, room["y_c"] + room["slab"] + 0.22, 0.0]))
        plain_leader = Line(plain_lbl.get_bottom() + DOWN * 0.04, np.array([gx, room["win_hi"] - 0.08, 0.0]),
                            color=P_CYAN, stroke_width=1.4, stroke_opacity=0.55)
        lowe_leader = Line(lowe_lbl.get_right() + RIGHT * 0.06, np.array([gx + 0.08, room["win_lo"] + 0.12, 0.0]),
                           color=P_ORANGE, stroke_width=1.4, stroke_opacity=0.55)
        self.play(Create(lowe), FadeIn(lowe_lbl), Create(lowe_leader), FadeIn(plain_lbl), Create(plain_leader),
                  Create(ir_back[0]), run_time=1.0)
        _beam_pulses(self, trap_paths[:1] + up_paths[1:], P_RED, repeats=2, run_time=3.0, kind="long",
                     extra=[_ripples(emitters, r_max=0.85)])
        hold_for(self, N, "trap")

        caption = swap_caption(self, caption, subtitle_text(N, "warm"))
        t_room = ValueTracker(20.0)
        t_read = _readout(lambda: rf"\vartheta_{{\mathrm{{Raum}}}} = {label_number(t_room.get_value())}\,\mathrm{{°C}}",
                          np.array([room["x_r"] - 0.15, room["y_c"] - 0.4, 0.0]), size=LABEL_FONT_SIZE,
                          color=P_RED, edge="right")
        _reveal(self, t_read)
        self.play(FadeOut(ir_up), FadeOut(ir_back[0]), FadeOut(lw_lbl), shaft.animate.set_fill(opacity=0.16),
                  run_time=0.8)
        plumes = [smooth_path([np.array([x, room["y_f"] + 0.06, 0.0]), np.array([x + 0.15, room["center"][1], 0.0]),
                               np.array([x - 0.1, room["y_c"] - 0.1, 0.0])])
                  for x in np.linspace(hits[0][0] + 0.4, hits[-1][0] - 0.4, 3)]
        animate_flows(self, [(plumes, P_ORANGE, P_RED)], run_time=2.4, waves=4, cycles=1.5, radius=0.06,
                      extra=[t_room.animate.set_value(35.0), room["air"].animate.set_fill(P_RED, opacity=0.14)])
        hold_for(self, N, "warm")

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        radiator = _staffage("shared-radiator-winter", unit=room_metre(room["h"]),
                             anchor=np.array([room["x_l"], room["y_f"], 0.0]), window=(0.0, -0.06, 0.25, 1.0))
        rad_top = radiator.get_top()
        rad_plume = [smooth_path([rad_top + UP * 0.05 + RIGHT * dx, rad_top + UP * 0.6 + RIGHT * (dx + 0.15),
                                  rad_top + UP * 1.2 + RIGHT * (dx + 0.05)]) for dx in (-0.15, 0.15)]
        heat_need = ValueTracker(1000.0)
        need_read = _readout(lambda: tr(rf"\text{{Heating}}\;{label_number(heat_need.get_value())}\,\mathrm{{W}}", rf"\text{{Heizung}}\;{label_number(heat_need.get_value())}\,\mathrm{{W}}"),
                             np.array([4.9, -0.2, 0.0]), size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        self.play(FadeIn(radiator), FadeIn(need_read), run_time=0.7)
        animate_flows(self, [(rad_plume, P_RED, P_ORANGE)], run_time=1.6, waves=3, cycles=1.2, radius=0.055,
                      extra=[heat_need.animate.set_value(625.0)])
        self.play(Indicate(items["r"], color=P_ORANGE), run_time=0.6)
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        out_blind = _blind(gx - 0.3, room["win_hi"] + 0.06, room["win_lo"] + 0.18)
        blind_group = out_blind["group"]
        blind_lbl = Text(tr("Venetian blind", "Raffstore"), font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([gx - 0.85, room["win_lo"] - 0.35, 0.0]))
        stopped, shade_paths = VGroup(), []
        for start in starts:
            stop = _blind_hit(out_blind, start, sun_dir)
            stopped.add(_wave([start, stop], "short"))
            bounce = _mirror(sun_dir, out_blind["angle"])
            back = stop + bounce * min(0.4, max(0.05, (room["win_hi"] + 0.04 - stop[1]) / max(bounce[1], 1e-6)))
            stopped.add(_beam(stop, back, opacity=0.4, width=1.6))
            shade_paths.append([start, stop])
        norm2 = Text("Willems et al. 2024", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(np.array([-3.7, -1.4, 0.0]))
        self.play(FadeOut(radiator), FadeOut(need_read), FadeOut(lowe_lbl), FadeOut(lowe_leader), FadeOut(plain_lbl), FadeOut(plain_leader),
                  run_time=0.4)
        self.play(FadeIn(blind_group, shift=DOWN * 0.25), FadeIn(blind_lbl), run_time=1.2)
        self.play(ReplacementTransform(beams_out, stopped), shaft.animate.set_fill(opacity=0.03), FadeOut(patch),
                  t_room.animate.set_value(22.0), room["air"].animate.set_fill(P_RED, opacity=0.04),
                  surface_heat["floor"].animate.set_fill(opacity=0),
                  surface_heat["wall_lo"].animate.set_fill(opacity=0),
                  surface_heat["wall_hi"].animate.set_fill(opacity=0), FadeIn(norm2), run_time=1.4)
        _beam_pulses(self, shade_paths, P_YELLOW, repeats=2, run_time=2.6, kind="short")
        hold_for(self, N, "summer")

        caption = swap_caption(self, caption, subtitle_text(N, "inner"))
        bar_base, bar_unit = -0.55, 0.075
        bar_x = {"out": 4.55, "in": 5.55}
        out_bar = Rectangle(width=0.62, height=(22 - 15) * bar_unit, stroke_width=0, fill_color=P_TEAL, fill_opacity=0.7)
        out_bar.move_to(np.array([bar_x["out"], bar_base + out_bar.height / 2, 0.0]))
        in_frac = ValueTracker(0.0)
        in_bar = always_redraw(lambda: Rectangle(
            width=0.62, height=max(0.002, (22 + 8 * in_frac.get_value() - 15) * bar_unit), stroke_width=0,
            fill_color=P_RED, fill_opacity=0.7).move_to(np.array(
                [bar_x["in"], bar_base + max(0.002, (22 + 8 * in_frac.get_value() - 15) * bar_unit) / 2, 0.0])))
        out_name = Text(tr("outside", "außen"), font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([bar_x["out"], bar_base - 0.28, 0.0]))
        in_name = Text(tr("inside", "innen"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([bar_x["in"], bar_base - 0.28, 0.0]))
        out_val = _m(r"22\,\mathrm{°C}", np.array([bar_x["out"], bar_base + out_bar.height + 0.3, 0.0]),
                     size=LABEL_FONT_SIZE, color=P_TEAL)
        in_val = always_redraw(lambda: _m(rf"{label_number(22 + 8 * in_frac.get_value())}\,\mathrm{{°C}}", np.array(
            [bar_x["in"], bar_base + max(0.002, (22 + 8 * in_frac.get_value() - 15) * bar_unit) + 0.3, 0.0]),
            size=LABEL_FONT_SIZE, color=P_RED))
        in_blind = _blind(gx + 0.3, room["win_hi"] + 0.06, room["win_lo"] + 0.18)
        in_group = in_blind["group"]
        in_lbl = Text(tr("Interior blind", "Raffstore innen"), font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([gx + 1.9, room["y_f"] + 0.3, 0.0]))
        in_beams, in_paths = VGroup(), []
        for start in starts:
            stop = _blind_hit(in_blind, start, sun_dir)
            in_beams.add(_wave([start, stop], "short"))
            in_paths.append([start, stop])
        slat_pts = [np.array([gx + 0.55, y, 0.0]) for y in in_blind["ys"][::3]]
        self.play(FadeOut(VGroup(blind_group, blind_lbl, norm2)), FadeOut(stopped), run_time=0.6)
        self.play(FadeIn(in_group, shift=DOWN * 0.2), FadeIn(in_lbl),
                  LaggedStart(*[Create(b) for b in in_beams], lag_ratio=0.08), run_time=1.4)
        warm_slats = interpolate_color(ManimColor(P_TEAL), ManimColor(P_RED), 0.7)
        self.play(in_group.animate.set_stroke(color=warm_slats), shaft.animate.set_fill(opacity=0.0), LaggedStart(
            *[Flash(np.array(pth[1]), color=P_ORANGE, flash_radius=0.12, line_length=0.08) for pth in in_paths],
            lag_ratio=0.1), run_time=1.0)
        _beam_pulses(self, in_paths, P_YELLOW, repeats=1, run_time=2.0, kind="short", extra=[
                         _ripples(slat_pts, r_max=0.6), t_room.animate.set_value(30.0),
                         in_frac.animate.set_value(1.0), room["air"].animate.set_fill(P_RED, opacity=0.1)])
        hold_for(self, N, "inner")

        caption = swap_caption(self, caption, subtitle_text(N, "compare"))
        self.add(in_bar, in_val)
        self.play(FadeIn(out_bar), FadeIn(out_val), FadeIn(out_name), FadeIn(in_name), run_time=0.9)
        better = Text(tr("better", "besser"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([bar_x["out"], bar_base + 1.3, 0.0]))
        self.play(Indicate(out_bar, color=P_TEAL), FadeIn(better, shift=UP * 0.1), run_time=1.0)
        hold_for(self, N, "compare")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Thermal mass: storing heat and releasing it later
class Beat6_ThermischeMasse(Scene):
    NARRATION = [
        ("store",
         "Building elements store heat. How much depends on mass, material and temperature change.",
         "Bauteile speichern Wärme. Wie viel, hängt von Masse, Material und Temperaturänderung ab."),
        ("mass",
         "One square metre of concrete ceiling, ten centimetres active depth: a tenth of a cubic metre — 240 kilograms.",
         "1 m² Betondecke mit 10 cm wirksamer Schicht: 0,1 m³ — das sind 240 kg."),
        ("heat",
         "Warmed by two kelvin it stores 480 kilojoules, about 0.13 kilowatt-hours.",
         "Um 2 K erwärmt speichert sie 480 kJ — rund 0,13 kWh."),
        ("room",
         "A twenty-square-metre ceiling holds about 2.7 kilowatt-hours — almost four hours of vacuuming.",
         "Eine Decke mit 20 m² nimmt so rund 2,7 kWh auf — fast vier Stunden Staubsaugen."),
        ("day",
         "During the day a massive ceiling absorbs heat that would otherwise warm the room air.",
         "Tagsüber nimmt eine massive Decke Wärme auf, die sonst die Raumluft aufheizen würde."),
        ("lag",
         "It releases it hours later — phase-shifted, and with a smaller swing.",
         "Sie gibt die Wärme Stunden später wieder ab — phasenverschoben und mit kleinerer Schwankung."),
        ("ceiling",
         "The ceiling has the greatest effect: it is free of cladding, warm air rises to it and cold air sinks.",
         "Die Decke wirkt am stärksten: Sie ist frei von Verkleidung, warme Luft steigt zu ihr auf und kalte Luft fällt."),
        ("view",
         "And it exchanges radiation with every surface in the room.",
         "Außerdem tauscht sie mit jeder Fläche im Raum Strahlung aus."),
        ("summer",
         "In summer it takes the peaks and is discharged at night by ventilation.",
         "Im Sommer fängt sie die Spitzen ab und wird nachts durch Lüften entladen."),
        ("winter",
         "In winter it stores the day's solar gains into the evening.",
         "Im Winter speichert sie die Sonnengewinne des Tages bis in den Abend."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Thermal mass — store and release with a delay", "Thermische Masse — speichern und zeitversetzt abgeben"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "store"))
        self.play(FadeIn(caption), run_time=0.3)

        fl = np.array([-5.0, -0.75, 0.0])
        W, D, depth = 3.8, 0.42, np.array([1.1, 0.7, 0.0])
        front = Polygon(fl, fl + RIGHT * W, fl + RIGHT * W + UP * D, fl + UP * D, color=P_WHITE, stroke_width=2.5,
                        fill_color=P_WHITE, fill_opacity=0.18)
        top = Polygon(fl + UP * D, fl + RIGHT * W + UP * D, fl + RIGHT * W + UP * D + depth, fl + UP * D + depth,
                      color=P_WHITE, stroke_width=2.5, fill_color=P_WHITE, fill_opacity=0.28)
        side = Polygon(fl + RIGHT * W, fl + RIGHT * W + depth, fl + RIGHT * W + UP * D + depth, fl + RIGHT * W + UP * D,
                       color=P_WHITE, stroke_width=2.5, fill_color=P_WHITE, fill_opacity=0.1)
        slab = VGroup(front, top, side)
        concrete = Text(tr("Concrete", "Beton"), font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(fl + RIGHT * W / 2 + UP * D / 2)
        d_w = dim_arrow(fl + DOWN * 0.24, fl + RIGHT * W + DOWN * 0.24, color=P_WHITE)
        d_w_lbl = _m(r"100\,\mathrm{cm}", d_w.get_center() + DOWN * 0.36, size=LABEL_FONT_SIZE)
        d_d = dim_arrow(fl + RIGHT * W + RIGHT * 0.25, fl + RIGHT * W + RIGHT * 0.25 + depth, color=P_WHITE)
        d_d_lbl = _m(r"100\,\mathrm{cm}", size=LABEL_FONT_SIZE).next_to(d_d, RIGHT, buff=0.06)
        d_t = dim_arrow(fl + LEFT * 0.22, fl + LEFT * 0.22 + UP * D, color=P_ORANGE)
        d_t_lbl = _m(r"10\,\mathrm{cm}", size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(d_t, LEFT, buff=0.08)
        col_x, rows = 0.7, [1.75, 1.15, 0.55, -0.05, -0.65]
        heat_rows = [1.45, 0.8, 0.15]
        dT = ValueTracker(0.0)
        v_line = _m(tr(r"V = 100\,\mathrm{cm} \cdot 100\,\mathrm{cm} \cdot 10\,\mathrm{cm} = 0.1\,\mathrm{m^{3}}", r"V = 100\,\mathrm{cm} \cdot 100\,\mathrm{cm} \cdot 10\,\mathrm{cm} = 0{,}1\,\mathrm{m^{3}}"),
                    np.array([col_x, rows[0], 0.0]), size=LABEL_FONT_SIZE, edge="left")
        m_line = _m(tr(r"m = \rho \cdot V = 2\,400\,\mathrm{kg/m^{3}} \cdot 0.1\,\mathrm{m^{3}} = 240\,\mathrm{kg}", r"m = \rho \cdot V = 2\,400\,\mathrm{kg/m^{3}} \cdot 0{,}1\,\mathrm{m^{3}} = 240\,\mathrm{kg}"),
                    np.array([col_x, rows[1], 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="left")
        dt_read = _readout(lambda: rf"\Delta T = {label_number(dT.get_value(), 1)}\,\mathrm{{K}}\quad"
                                   rf"(20 \to {label_number(20 + dT.get_value(), 1)}\,\mathrm{{°C}})",
                           np.array([col_x, heat_rows[0], 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        q_read = _readout(
            lambda: (rf"Q = 240\,\mathrm{{kg}} \cdot 1\,000\,\mathrm{{J/(kg\,K)}} \cdot {label_number(dT.get_value(), 1)}\,"
                     rf"\mathrm{{K}}"),
            np.array([col_x, heat_rows[1], 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        kwh_read = _readout(lambda: rf"=\;{label_number(240 * dT.get_value())}\,\mathrm{{kJ}} \approx "
                                    rf"{label_number(240 * dT.get_value() / 3600, 2)}\,\mathrm{{kWh}}",
                            np.array([col_x, heat_rows[2], 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW)
        heat_in = VGroup(*[Arrow(fl + depth * 0.5 + UP * (D + 1.0) + RIGHT * x, fl + depth * 0.5 + UP * (D + 0.08) + RIGHT * x,
                                 buff=0, color=P_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.25)
                           for x in np.linspace(0.5, W - 0.5, 4)])
        self.play(FadeIn(slab), FadeIn(concrete), run_time=1.0)
        hold_for(self, N, "store", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        self.play(GrowFromCenter(d_w), FadeIn(d_w_lbl), GrowFromCenter(d_d), FadeIn(d_d_lbl), GrowFromCenter(d_t),
                  FadeIn(d_t_lbl), run_time=1.0)
        self.play(FadeIn(v_line, shift=RIGHT * 0.1), run_time=0.8)
        self.play(FadeIn(m_line, shift=RIGHT * 0.1), slab.animate.set_fill(P_CYAN, opacity=0.25), run_time=0.8)
        self.wait(2.5)
        self.play(FadeOut(v_line), run_time=1.6)
        self.play(m_line.animate.move_to(np.array([m_line.get_center()[0], rows[0], 0.0])), run_time=1.2)
        hold_for(self, N, "mass")

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        row, box, items = _panel([
            (None, "Q", P_ORANGE), (None, "=", P_WHITE), ("m", "m", P_CYAN), (None, r"\cdot", P_WHITE),
            ("c", "c", P_WHITE), (None, r"\cdot", P_WHITE), ("dt", r"\Delta T", P_ORANGE),
        ])
        self.wait(1.5)
        self.play(FadeOut(VGroup(d_w, d_w_lbl, d_d, d_d_lbl, d_t, d_t_lbl)), FadeOut(m_line), run_time=1.8)
        self.play(FadeIn(row), Create(box), LaggedStart(*[GrowArrow(a) for a in heat_in], lag_ratio=0.15), run_time=0.8)
        _reveal(self, dt_read, q_read, kwh_read)
        self.play(dT.animate.set_value(2.0), slab.animate.set_fill(P_ORANGE, opacity=0.45),
                  LaggedStart(*[ShowPassingFlash(a.copy().set_stroke(width=6), time_width=0.4) for a in list(heat_in) * 3],
                              lag_ratio=0.12), run_time=2.4)
        hold_for(self, N, "heat")

        caption = swap_caption(self, caption, subtitle_text(N, "room"))
        self.play(FadeOut(VGroup(heat_in, concrete)), FadeOut(dt_read), FadeOut(q_read), run_time=0.5)
        tile = 0.5
        grid_origin = np.array([-5.4, -0.9, 0.0])
        tiles = VGroup(*[
            Square(side_length=tile * 0.94, color=P_ORANGE, stroke_width=1.6, fill_color=P_ORANGE, fill_opacity=0.45)
            .move_to(grid_origin + RIGHT * tile * (c + 0.5) + UP * tile * (r + 0.5))
            for r in range(4) for c in range(5)
        ])
        g_w = dim_arrow(grid_origin + DOWN * 0.2, grid_origin + RIGHT * 5 * tile + DOWN * 0.2, color=P_WHITE)
        g_w_lbl = _m(r"5\,\mathrm{m}", g_w.get_center() + DOWN * 0.36, size=LABEL_FONT_SIZE)
        g_h = dim_arrow(grid_origin + LEFT * 0.2, grid_origin + UP * 4 * tile + LEFT * 0.2, color=P_WHITE)
        g_h_lbl = _m(r"4\,\mathrm{m}", size=LABEL_FONT_SIZE).next_to(g_h, LEFT, buff=0.08)
        n_tiles = ValueTracker(1)
        total_read = _readout(
            lambda: rf"{int(n_tiles.get_value())}\,\mathrm{{m^{{2}}}} \cdot {label_number(0.133, 3)}\,\mathrm{{kWh}} = "
                    rf"{label_number(0.1333 * n_tiles.get_value(), 1)}\,\mathrm{{kWh}}",
            np.array([col_x, rows[1], 0.0]), color=P_YELLOW)
        self.play(ReplacementTransform(slab, tiles[0]), FadeOut(kwh_read), run_time=1.0)
        _reveal(self, total_read)
        self.play(LaggedStart(*[TransformFromCopy(tiles[0], t) for t in tiles[1:]], lag_ratio=0.08),
                  n_tiles.animate.set_value(20), GrowFromCenter(g_w), FadeIn(g_w_lbl), GrowFromCenter(g_h),
                  FadeIn(g_h_lbl), run_time=2.4)
        vac = _vacuum().scale(0.8)
        vac.move_to(np.array([col_x + vac.width / 2, rows[3], 0.0]))
        vclock = _clock(np.array([col_x + vac.width + 0.75, rows[3], 0.0]), r=0.36)
        hours = ValueTracker(0.0)
        vac_lbl = _readout(lambda: tr(rf"\approx {label_number(hours.get_value(), 1)}\,\mathrm{{h}}\;\text{{vacuuming}}", rf"\approx {label_number(hours.get_value(), 1)}\,\mathrm{{h}}\;\text{{Staubsaugen}}"),
                           np.array([col_x + vac.width + 1.35, rows[3] - 0.08, 0.0]), color=P_PRIMARY)
        self.play(FadeIn(vac), FadeIn(vclock["group"]), run_time=0.6)
        _reveal(self, vac_lbl)
        self.play(hours.animate.set_value(3.9), Rotate(vclock["hand"], angle=-TAU * 3.9, about_point=vclock["center"]),
                  run_time=1.8, rate_func=linear)
        hold_for(self, N, "room")

        caption = swap_caption(self, caption, subtitle_text(N, "day"))
        self.play(FadeOut(VGroup(tiles, vac, vclock["group"], row, box, g_w, g_w_lbl, g_h, g_h_lbl)),
                  FadeOut(total_read), FadeOut(vac_lbl), run_time=0.6)
        CO = np.array([-6.3, -1.25, 0.0])
        CX, CY = 6.4 / 24.0, 0.145

        def cpt(h, temp):
            return CO + RIGHT * h * CX + UP * (temp - 14) * CY

        def t_out(h):
            return 24 + 8 * np.sin(TAU * (h - 9) / 24)

        def t_light(h):
            return 25 + 5 * np.sin(TAU * (h - 10.5) / 24)

        def t_heavy(h):
            return 25 + 2 * np.sin(TAU * (h - 15) / 24)

        cax = _axes(CO, 7.15, 3.05, r"t\;[\mathrm{h}]", r"\vartheta\;[\mathrm{°C}]")
        h_ticks = VGroup(*[_m(str(h), cpt(h, 14) + DOWN * 0.32, size=LABEL_FONT_SIZE) for h in (0, 6, 12, 18, 24)])
        clock_h = ValueTracker(0.0)

        def curve(fn, color):
            def build():
                hs = np.linspace(0, max(0.05, clock_h.get_value()), 80)
                c = VMobject(color=color, stroke_width=3)
                c.set_points_smoothly([cpt(h, fn(h)) for h in hs])
                return c
            return always_redraw(build)

        c_out, c_light, c_heavy = curve(t_out, P_YELLOW), curve(t_light, P_ORANGE), curve(t_heavy, P_CYAN)
        legend = VGroup(*[
            VGroup(Line(LEFT * 0.2, RIGHT * 0.2, color=c, stroke_width=3), Text(t, font_size=LABEL_FONT_SIZE, color=c))
            .arrange(RIGHT, buff=0.12)
            for t, c in ((tr("outside", "außen"), P_YELLOW), (tr("Room, lightweight", "Raum, leicht"), P_ORANGE), (tr("Room, massive ceiling", "Raum, massive Decke"), P_CYAN))
        ]).arrange(RIGHT, buff=0.35)
        legend.move_to(np.array([CO[0] + legend.width / 2, 2.3, 0.0]))
        cursor = always_redraw(lambda: DashedLine(cpt(clock_h.get_value(), 14), cpt(clock_h.get_value(), 34.5),
                                                  color=P_WHITE, stroke_width=1.5, stroke_opacity=0.5))
        small = _room(np.array([4.3, CO[1] + 1.5, 0.0]), w=3.4, h=1.9, slab=0.36, window=(0.3, 0.85))

        def ceiling_heat():
            f = float(np.clip((t_heavy(clock_h.get_value()) - 23) / 4, 0, 1))
            return small["ceiling"].copy().set_fill(interpolate_color(ManimColor(P_WHITE), ManimColor(P_ORANGE), f),
                                                    opacity=0.2 + 0.55 * f)

        def ceiling_flux():
            h = clock_h.get_value()
            rate = np.cos(TAU * (h - 15) / 24)
            y_c = small["y_c"]
            arrows = VGroup()
            for x in np.linspace(small["x_l"] + 0.6, small["x_r"] - 0.6, 3):
                tail, tip = (np.array([x, y_c - 0.65, 0.0]), np.array([x, y_c - 0.08, 0.0]))
                if rate < 0:
                    tail, tip = tip, tail
                arrows.add(Arrow(tail, tip, buff=0, color=P_ORANGE, stroke_width=3,
                                 max_tip_length_to_length_ratio=0.3).set_opacity(0.15 + 0.75 * abs(rate)))
            return arrows

        ceil_live = always_redraw(ceiling_heat)
        flux_live = always_redraw(ceiling_flux)
        sky_c = np.array([small["center"][0], small["y_c"] + small["slab"] + 0.45, 0.0])
        sky_sun = _build_sun(sky_c).scale(0.22)
        sky_moon = _moon(sky_c)

        def sky():
            h = clock_h.get_value() % 24
            return (sky_sun if 6 <= h <= 19 else sky_moon).copy()

        sky_live = always_redraw(sky)
        hour_read = _readout(lambda: tr(rf"{int(clock_h.get_value()) % 24:02d}{{:}}00\,\mathrm{{h}}", rf"{int(clock_h.get_value()) % 24:02d}{{:}}00\,\mathrm{{Uhr}}"),
                             sky_c + RIGHT * 0.45 + DOWN * 0.08, size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
        self.play(FadeIn(cax["group"]), FadeIn(h_ticks), FadeIn(legend), FadeIn(small["group"]), run_time=0.9)
        self.add(c_out, c_light, c_heavy, cursor, ceil_live, flux_live, sky_live)
        _reveal(self, hour_read)
        self.play(clock_h.animate.set_value(24.0), run_time=7.0, rate_func=linear)
        hold_for(self, N, "day")

        caption = swap_caption(self, caption, subtitle_text(N, "lag"))
        p_out, p_heavy = cpt(15, 32), cpt(21, 27)
        v1 = DashedLine(p_out, cpt(15, 14), color=P_YELLOW, stroke_width=1.6, dash_length=0.08)
        v2 = DashedLine(p_heavy, cpt(21, 14), color=P_CYAN, stroke_width=1.6, dash_length=0.08)
        shift_arrow = dim_arrow(cpt(15, 33.3), cpt(21, 33.3), color=P_PRIMARY)
        phi = ValueTracker(0.0)
        phi_read = _readout(lambda: rf"\varphi \approx {label_number(phi.get_value())}\,\mathrm{{h}}", cpt(18, 33.9),
                            size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="center")
        amp_out = dim_arrow(cpt(14.5, 16), cpt(14.5, 32), color=P_YELLOW)
        amp_heavy = dim_arrow(cpt(20.5, 23), cpt(20.5, 27), color=P_CYAN)
        amp_lbl = VGroup(_m(r"16\,\mathrm{K}", cpt(14.2, 19.0), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="right"),
                         _m(r"4\,\mathrm{K}", cpt(20.2, 20.5), size=LABEL_FONT_SIZE, color=P_CYAN, edge="right"))
        self.remove(cursor)
        self.play(Create(v1), Create(v2), run_time=0.7)
        _reveal(self, phi_read)
        self.play(GrowFromCenter(shift_arrow), phi.animate.set_value(6.0), run_time=1.4)
        self.play(GrowFromCenter(amp_out), GrowFromCenter(amp_heavy), FadeIn(amp_lbl), run_time=1.0)
        hold_for(self, N, "lag")

        caption = swap_caption(self, caption, subtitle_text(N, "ceiling"))
        self.play(FadeOut(VGroup(cax["group"], h_ticks, legend, small["group"], v1, v2, shift_arrow, amp_out, amp_heavy,
                                 amp_lbl)),
                  *[FadeOut(m) for m in (c_out, c_light, c_heavy, ceil_live, flux_live, sky_live, hour_read, phi_read)],
                  run_time=0.6)
        big = _room(np.array([-1.3, 0.05, 0.0]), w=5.6, h=2.4, slab=0.34, window=(0.3, 0.85))
        rise_paths = [smooth_path([start, start + np.array([0.18, 0.6, 0.0]), start + np.array([0.05, 1.3, 0.0]),
                                   np.array([start[0] + 0.3, big["y_c"] - 0.08, 0.0]),
                                   np.array([start[0] + 1.1, big["y_c"] - 0.12, 0.0])])
                      for start in (np.array([big["center"][0] - 1.2, big["y_f"] + 0.1, 0.0]),
                                    np.array([big["center"][0] + 0.6, big["y_f"] + 0.1, 0.0]))]
        rise_guides = flow_guides(rise_paths, P_RED, opacity=0.2)
        fall_paths = [smooth_path([np.array([big["glass_x"] + 0.28 + dx, big["y_c"] - 0.15, 0.0]),
                                   np.array([big["glass_x"] + 0.34 + dx, big["center"][1], 0.0]),
                                   np.array([big["glass_x"] + 0.4 + dx, big["y_f"] + 0.25, 0.0]),
                                   np.array([big["glass_x"] + 1.3 + dx, big["y_f"] + 0.14, 0.0])]) for dx in (0.0, 0.14)]
        fall_guides = flow_guides(fall_paths[:1], P_BLUE, opacity=0.2)
        cold_tag = Text(tr("cold air sinks", "kalte Luft fällt"), font_size=LABEL_FONT_SIZE, color=P_BLUE)
        cold_tag.move_to(np.array([big["glass_x"] - 0.3 - cold_tag.width / 2, big["center"][1] - 0.1, 0.0]))
        cold_leader = Line(cold_tag.get_right() + RIGHT * 0.08, np.array([big["glass_x"] + 0.28, big["center"][1] - 0.1, 0.0]),
                           color=P_BLUE, stroke_width=1.4, stroke_opacity=0.55)
        ceil_fill = ValueTracker(0.2)
        ceiling_heat = big["ceiling"].copy().set_fill(P_ORANGE, opacity=0)
        floor_heat = big["floor"].copy().set_fill(P_ORANGE, opacity=0)
        big["group"].add(ceiling_heat, floor_heat)
        ceiling_heat.add_updater(lambda m: m.set_fill(P_ORANGE, opacity=ceil_fill.get_value()))
        tags, tag_leaders = side_labels([
            (big["ceiling"].get_right() + LEFT * 0.1, tr("free of cladding", "frei von Verkleidung"), P_ORANGE),
            (rise_paths[0].point_from_proportion(0.5), tr("warm air rises", "warme Luft steigt auf"), P_RED),
            (big["wall_r"].get_center(), tr("Radiation exchange with all surfaces", "Strahlung mit allen Flächen"), P_YELLOW),
        ], x=1.95, align="left", min_gap=0.55)
        norm = Text("Willems et al. 2024", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([3.9, -1.35, 0.0]))
        self.play(FadeIn(big["group"]), run_time=1.0)
        self.play(FadeIn(tags[0]), Create(tag_leaders[0]), Indicate(big["ceiling"], color=P_ORANGE, scale_factor=1.0),
                  run_time=1.0)
        self.play(FadeIn(tags[1]), Create(tag_leaders[1]), FadeIn(rise_guides), FadeIn(cold_tag), Create(cold_leader),
                  FadeIn(fall_guides), run_time=0.7)
        animate_flows(self, [(rise_paths, P_RED, P_ORANGE), (fall_paths, P_BLUE, P_CYAN)], run_time=2.4, waves=5,
                      cycles=2.0, radius=0.06, extra=[ceil_fill.animate.set_value(0.45)])
        hold_for(self, N, "ceiling")

        caption = swap_caption(self, caption, subtitle_text(N, "view"))
        ceil_y = big["y_c"] - 0.02
        targets = (np.array([big["glass_x"] + 0.1, big["center"][1] - 0.2, 0.0]),
                   np.array([big["x_l"] + 1.2, big["y_f"] + 0.02, 0.0]),
                   np.array([big["center"][0], big["y_f"] + 0.02, 0.0]),
                   np.array([big["x_r"] - 1.2, big["y_f"] + 0.02, 0.0]),
                   np.array([big["x_r"] - 0.02, big["center"][1] - 0.5, 0.0]))
        exchange = VGroup(*[
            DoubleArrow(np.array([0.6 * t[0] + 0.4 * big["center"][0], ceil_y, 0.0]), t, buff=0.04, color=P_YELLOW,
                        stroke_width=1.8, tip_length=0.12, max_tip_length_to_length_ratio=0.08).set_opacity(0.55)
            for t in targets
        ])
        emit = [np.array([x, ceil_y, 0.0]) for x in np.linspace(big["x_l"] + 1.0, big["x_r"] - 1.0, 3)]
        self.play(FadeIn(tags[2]), Create(tag_leaders[2]), FadeOut(rise_guides), FadeOut(fall_guides), FadeOut(cold_tag),
                  FadeOut(cold_leader),
                  LaggedStart(*[GrowFromCenter(r) for r in exchange], lag_ratio=0.12),
                  ceil_fill.animate.set_value(0.6), FadeIn(norm), run_time=1.8)
        self.play(_ripples(emit, r_max=0.9, color=P_ORANGE, down=True), run_time=2.4)
        hold_for(self, N, "view")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        self.play(FadeOut(VGroup(tags, tag_leaders, exchange)), run_time=0.5)
        sky_pos = np.array([-5.6, 1.9, 0.0])
        sun = _build_sun(sky_pos).scale(0.35)
        moon = _moon(sky_pos)
        t_ceil = ValueTracker(24.0)
        t_tag = _readout(lambda: rf"\vartheta_{{\mathrm{{Decke}}}} = {label_number(t_ceil.get_value())}\,\mathrm{{°C}}",
                         np.array([big["x_r"] + 0.45, big["y_c"] + big["slab"] / 2 - 0.08, 0.0]),
                         size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        day_flux = VGroup(*[Arrow(np.array([x, big["center"][1] - 0.35, 0.0]), np.array([x, big["y_c"] - 0.06, 0.0]),
                                  buff=0, color=P_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.2)
                            for x in np.linspace(big["x_l"] + 1.0, big["x_r"] - 1.0, 4)])
        self.play(FadeIn(sun), LaggedStart(*[GrowArrow(a) for a in day_flux], lag_ratio=0.12), run_time=0.8)
        _reveal(self, t_tag)
        self.play(LaggedStart(*[ShowPassingFlash(a.copy().set_stroke(width=6), time_width=0.4) for a in list(day_flux) * 3],
                              lag_ratio=0.1),
                  ceil_fill.animate.set_value(0.85), t_ceil.animate.set_value(27.0), run_time=2.0)
        gx, lo, hi = big["glass_x"], big["win_lo"], big["win_hi"]
        night_window = _window(gx, lo, hi, depth=(hi - lo) * 0.75, wall=big["wall"])
        big["glass"].set_opacity(0)
        self.add(night_window["group"])
        purge = [smooth_path([np.array([gx - 1.4, lo + 0.05 + dy, 0.0]), np.array([gx + 0.3, lo + 0.12 + dy, 0.0]),
                              np.array([big["center"][0], big["y_f"] + 0.3 + dy, 0.0]),
                              np.array([big["x_r"] - 0.35, big["center"][1] - dy, 0.0]),
                              np.array([big["center"][0] + 0.5, big["y_c"] - 0.15 - dy, 0.0]),
                              np.array([gx + 0.25, hi - 0.1 - dy, 0.0]), np.array([gx - 1.2, hi + 0.25 - dy, 0.0])])
                 for dy in (0.0, 0.12)]
        self.play(ReplacementTransform(sun, moon), _open_window(night_window, tilt=True), FadeOut(day_flux), run_time=0.9)
        purge_guide = flow_guides(purge[:1], P_CYAN, opacity=0.18)
        self.play(FadeIn(purge_guide), run_time=0.3)
        animate_flows(self, [(purge, P_BLUE, P_ORANGE)], run_time=3.0, waves=6, cycles=2.0, radius=0.06,
                      extra=[ceil_fill.animate.set_value(0.2), t_ceil.animate.set_value(22.0)])
        hold_for(self, N, "summer")

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        sun2 = _build_sun(sky_pos).scale(0.35)
        winter_rays = _sun_rays(sky_pos, big["glass_x"], [big["win_hi"] - 0.15 - 0.3 * k for k in range(3)],
                                big["y_f"] + 0.05, gap=0.45)
        lands = winter_rays["lands"]
        patch = Line(lands[0], lands[-1], color=P_RED, stroke_width=6, stroke_opacity=0.4)
        floor_lbl = Text(tr("The floor warms up too", "Boden erwärmt sich auch"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(
            np.array([(lands[0][0] + lands[-1][0]) / 2, big["y_f"] - big["slab"] - 0.3, 0.0]))
        self.play(ReplacementTransform(moon, sun2), _open_window(night_window, close=True), FadeOut(purge_guide),
                  run_time=0.8)
        self.play(_shine(winter_rays), run_time=1.2)
        self.play(FadeIn(patch, scale=0.6), run_time=0.3)
        self.play(_ripples([interpolate(lands[0], lands[-1], f) + UP * 0.03 for f in (0.25, 0.75)], r_max=1.2),
                  ceil_fill.animate.set_value(0.75), t_ceil.animate.set_value(24.0),
                  floor_heat.animate.set_fill(opacity=0.5), FadeIn(floor_lbl), run_time=2.0)
        evening = _moon(sky_pos)
        self.play(ReplacementTransform(sun2, evening), FadeOut(winter_rays["group"]), FadeOut(patch), FadeOut(floor_lbl),
                  floor_heat.animate.set_fill(opacity=0), run_time=0.7)
        self.play(_ripples(emit, r_max=1.1, color=P_ORANGE, down=True), ceil_fill.animate.set_value(0.3),
                  t_ceil.animate.set_value(22.0), run_time=2.4)
        ceiling_heat.clear_updaters()
        hold_for(self, N, "winter")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – Sensible and latent heat
def _x_sat(t_c: float) -> float:
    """💧 Saturation humidity ratio in g/kg (Magnus formula, 1013 hPa)."""
    p_s = 611.2 * np.exp(17.62 * t_c / (243.12 + t_c))
    return 622.0 * p_s / (101325.0 - p_s)


def _p_sat(t_c: float) -> float:
    return 611.2 * np.exp(17.62 * t_c / (243.12 + t_c))


class Beat7_SensibelLatent(Scene):
    NARRATION = [
        ("sensible",
         "Sensible heat changes the temperature — you can measure it with a thermometer.",
         "Sensible Wärme ändert die Temperatur — man kann sie mit dem Thermometer messen."),
        ("water",
         "Heating one kilogram of water from zero to a hundred degrees takes about 419 kilojoules.",
         "1 kg Wasser von 0 auf 100 °C zu erwärmen kostet rund 419 kJ."),
        ("latent",
         "At a hundred degrees the temperature stops, although energy keeps flowing in — it goes into evaporation.",
         "Bei 100 °C bleibt die Temperatur stehen, obwohl weiter Energie hineingeht — sie steckt im Verdampfen."),
        ("ratio",
         "This latent heat, about 2,257 kilojoules per kilogram, is more than five times the heating.",
         "Diese latente Wärme, rund 2 257 kJ pro kg, ist mehr als fünfmal so groß wie das Erwärmen."),
        ("air",
         "Air carries water vapour. How much it can hold at most depends strongly on temperature.",
         "Luft trägt Wasserdampf. Wie viel sie höchstens aufnehmen kann, hängt stark von der Temperatur ab."),
        ("curve",
         "At zero degrees about four grams per kilogram of air, at twenty about fifteen, at thirty about twenty-seven.",
         "Bei 0 °C sind es rund 4 g pro kg Luft, bei 20 °C rund 15 g, bei 30 °C rund 27 g."),
        ("room",
         "Room air at twenty degrees and fifty percent relative humidity holds about seven grams.",
         "Raumluft mit 20 °C und 50 % relativer Feuchte enthält rund 7 g Wasser pro kg."),
        ("dew",
         "If it cools, the relative humidity rises — at about nine degrees it reaches one hundred percent: the dew point.",
         "Kühlt sie ab, steigt die relative Feuchte — bei rund 9 °C sind es 100 %: der Taupunkt."),
        ("condense",
         "Below that, water condenses — on a cold window pane, for example.",
         "Darunter kondensiert Wasser — zum Beispiel an einer kalten Fensterscheibe."),
        ("drops",
         "On the cold surface the vapour turns into liquid droplets — that is condensation. They run down and collect as condensate.",
         "An der kalten Oberfläche wird der Dampf zu flüssigen Tropfen — das ist die Kondensation. Sie laufen ab und sammeln sich als Kondensat."),
        ("person",
         "A resting person gives off about seventy watts sensibly and thirty latently — about 45 grams of water per hour.",
         "Ein ruhender Mensch gibt rund 70 W sensibel und 30 W latent ab — etwa 45 g Wasser pro Stunde."),
        ("rel_winter",
         "In winter, condensation threatens on cold surfaces: the inside surface is colder than the dew point.",
         "Im Winter droht an kalten Flächen Tauwasser: Die Oberfläche innen ist kälter als der Taupunkt."),
        ("rel_summer",
         "In summer, cooling must also dehumidify: the cold coil takes water out of the air.",
         "Im Sommer muss die Kühlung auch entfeuchten: Das kalte Register nimmt der Luft Wasser ab."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Sensible and latent heat", "Sensible und latente Wärme"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "sensible"))
        self.play(FadeIn(caption), run_time=0.3)

        QO = np.array([-5.5, -0.95, 0.0])
        QX, QY = 7.0 / 2800.0, 2.9 / 125.0

        def qpt(q, t):
            return QO + RIGHT * q * QX + UP * t * QY

        def temp_of(q):
            return min(100.0, q / 4.19)

        qax = _axes(QO, 7.4, 3.15, r"Q\;[\mathrm{kJ}]", r"T\;[\mathrm{°C}]")
        t_ticks = VGroup(*[_m(str(t), qpt(0, t) + LEFT * 0.2 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                           for t in (50, 100)])
        Q = ValueTracker(0.0)

        def path_upto():
            q_end = max(1.0, Q.get_value())
            qs = [q for q in (0.0, 419.0) if q < q_end] + [q_end]
            c = VMobject(color=P_RED, stroke_width=3.5)
            c.set_points_as_corners([qpt(q, temp_of(q)) for q in qs])
            return c

        heat_path = always_redraw(path_upto)
        head = always_redraw(lambda: Dot(qpt(Q.get_value(), temp_of(Q.get_value())), radius=0.08, color=P_YELLOW))
        hob = _staffage_motion("physics-fundamentals-hob", unit=HOB_UNIT, anchor=np.array([4.7, QO[1], 0.0]))
        pan_y = QO[1] + HOB_PAN * HOB_UNIT
        pot_w, pot_h = POT_INNER * HOB_UNIT, POT_HEIGHT * HOB_UNIT
        pot_c = np.array([4.7, pan_y + pot_h / 2, 0.0])
        bl = pot_c + np.array([-pot_w / 2, -pot_h / 2, 0.0])
        pot = _staffage_motion("physics-fundamentals-pot", unit=HOB_UNIT, anchor=np.array([4.7, pan_y, 0.0]), rate=0.0)
        col_x = min(bl[0] - 0.35, hob.get_left()[0] - 0.3)
        q_read =_readout(lambda: rf"Q = {label_number(Q.get_value())}\,\mathrm{{kJ}}", np.array([col_x, pot_c[1] + 0.45, 0.0]),
                          color=P_ORANGE, edge="right")
        t_read = _readout(lambda: rf"T = {label_number(temp_of(Q.get_value()))}\,\mathrm{{°C}}",
                          np.array([col_x, pot_c[1] - 0.1, 0.0]), color=P_RED, edge="right")
        mass_tag = _m(r"m = 1\,\mathrm{kg}", np.array([col_x, pot_c[1] - 0.65, 0.0]), color=P_BLUE, edge="right")
        steam_paths = [smooth_path([np.array([pot_c[0] + dx, bl[1] + pot_h + 0.05, 0.0]),
                                    np.array([pot_c[0] + dx + 0.12, bl[1] + pot_h + 0.45, 0.0]),
                                    np.array([pot_c[0] + dx - 0.1, bl[1] + pot_h + 0.85, 0.0]),
                                    np.array([pot_c[0] + dx + 0.08, bl[1] + pot_h + 1.2, 0.0])])
                       for dx in (-0.45, -0.15, 0.15, 0.45)]
        bubble_paths = [Line(bl + np.array([0.2 + 0.21 * k, 0.08, 0.0]), bl + np.array([0.2 + 0.21 * k, 0.3, 0.0]))
                        for k in range(4)]
        self.play(FadeIn(qax["group"]), FadeIn(t_ticks), FadeIn(pot), FadeIn(hob), FadeIn(mass_tag), run_time=1.0)
        self.add(heat_path, head)
        _reveal(self, q_read, t_read)
        pot.rate = POT_BOIL_RATE
        self.play(Q.animate.set_value(210.0), run_time=2.0, rate_func=linear)
        hold_for(self, N, "sensible", used=BEAT_SUBTITLE_FADE + 0.3 + 3.0)

        caption = swap_caption(self, caption, subtitle_text(N, "water"))
        row, box, _ = _panel([
            (None, "Q", P_ORANGE), (None, "=", P_WHITE), (None, "m", P_BLUE), (None, r"\cdot", P_WHITE),
            (None, "c", P_WHITE), (None, r"\cdot", P_WHITE), (None, r"\Delta T", P_RED), (None, "=", P_WHITE),
            (None, tr(r"1\,\mathrm{kg} \cdot 4.19\,\mathrm{kJ/(kg\,K)} \cdot 100\,\mathrm{K}", r"1\,\mathrm{kg} \cdot 4{,}19\,\mathrm{kJ/(kg\,K)} \cdot 100\,\mathrm{K}"), P_WHITE),
            (None, r"\approx", P_WHITE), (None, r"419\,\mathrm{kJ}", P_ORANGE),
        ])
        self.play(Q.animate.set_value(419.0), run_time=1.6, rate_func=linear)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        hold_for(self, N, "water")

        caption = swap_caption(self, caption, subtitle_text(N, "latent"))
        _paced_flow(self, [(bubble_paths, P_WHITE, None, None, False), (steam_paths, semio_color("light-gray"), None, None, False)],
                    run_time=4.0, waves=3, cycles=4.0, radius=0.05,
                    extra=[Q.animate.set_value(2676.0)])
        hold_for(self, N, "latent")

        caption = swap_caption(self, caption, subtitle_text(N, "ratio"))
        sens = dim_arrow(qpt(0, 108), qpt(419, 108), color=P_ORANGE)
        lat = dim_arrow(qpt(419, 108), qpt(2676, 108), color=P_BLUE)
        sens_lbl = _m(r"419\,\mathrm{kJ}", qpt(210, 113), size=LABEL_FONT_SIZE, color=P_ORANGE)
        lat_lbl = _m(r"2\,257\,\mathrm{kJ}", qpt(1550, 113), size=LABEL_FONT_SIZE, color=P_BLUE)
        ratio = ValueTracker(0.0)
        ratio_read = _readout(lambda: rf"\frac{{2\,257}}{{419}} \approx {label_number(ratio.get_value(), 1)}",
                              qpt(1550, 70), color=P_YELLOW, edge="center")
        lat_panel = _panel([
            (None, r"Q_{\mathrm{lat}}", P_BLUE), (None, "=", P_WHITE), (None, "m", P_BLUE), (None, r"\cdot", P_WHITE),
            (None, "r", P_WHITE), (None, "=", P_WHITE),
            (None, r"1\,\mathrm{kg} \cdot 2\,257\,\mathrm{kJ/kg}", P_WHITE),
        ])
        self.play(GrowFromCenter(sens), FadeIn(sens_lbl), GrowFromCenter(lat), FadeIn(lat_lbl), run_time=1.0)
        _reveal(self, ratio_read)
        self.play(ratio.animate.set_value(2257 / 419), run_time=1.4)
        _swap_panel(self, (row, box), lat_panel[:2])
        hold_for(self, N, "ratio")

        caption = swap_caption(self, caption, subtitle_text(N, "air"))
        self.play(FadeOut(VGroup(qax["group"], t_ticks, pot, hob, mass_tag, sens, lat, sens_lbl, lat_lbl,
                                 lat_panel[0], lat_panel[1])),
                  *[FadeOut(m) for m in (heat_path, head, q_read, t_read, ratio_read)], run_time=0.6)
        AO = np.array([-5.4, -1.2, 0.0])
        AX, AY = 5.8 / 45.0, 0.1

        def apt(t, x):
            return AO + RIGHT * (t + 10) * AX + UP * x * AY

        aax = _axes(AO, 6.2, 3.25, r"\vartheta\;[\mathrm{°C}]", r"x\;[\mathrm{g/kg}]")
        t_lbls = VGroup(*[_m(str(t).replace("-", "−"), apt(t, 0) + DOWN * 0.32, size=LABEL_FONT_SIZE)
                          for t in (-10, 0, 10, 20, 30)])
        x_lbls = VGroup(*[_m(str(x), apt(-10, x) + LEFT * 0.2 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                          for x in (10, 20, 30)])
        sat = VMobject(color=P_CYAN, stroke_width=3.5)
        sat.set_points_smoothly([apt(t, _x_sat(t)) for t in np.linspace(-10, 31.5, 60)])
        sat_lbl = Text(tr("Saturation (100 %)", "Sättigung (100 %)"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(apt(17, 24))
        box_c = np.array([3.6, AO[1] + 3.25 / 2, 0.0])
        air_box = Square(side_length=1.9, color=P_WHITE, stroke_width=2.5).move_to(box_c)
        air_tag = Text(tr("1 kg air", "1 kg Luft"), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(air_box, UP, buff=0.12)
        rng = np.random.default_rng(11)
        vapour = VGroup(*[Dot(box_c + np.array([rng.uniform(-0.82, 0.82), rng.uniform(-0.82, 0.82), 0.0]), radius=0.055,
                              color=P_BLUE) for _ in range(27)])
        self.play(FadeIn(aax["group"]), FadeIn(t_lbls), FadeIn(x_lbls), FadeIn(air_box), FadeIn(air_tag), run_time=0.9)
        self.play(Create(sat), FadeIn(sat_lbl), run_time=1.8)
        hold_for(self, N, "air")

        caption = swap_caption(self, caption, subtitle_text(N, "curve"))
        marks = VGroup()
        for t, txt in ((0, r"\approx 4\,\mathrm{g}"), (20, r"\approx 15\,\mathrm{g}"), (30, r"\approx 27\,\mathrm{g}")):
            p = apt(t, _x_sat(t))
            marks.add(VGroup(Dot(p, radius=0.07, color=P_CYAN),
                             _m(txt, p + LEFT * 0.15 + UP * 0.12, size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")))
        for k, mark in enumerate(marks):
            n_show = (4, 15, 27)[k]
            self.play(FadeIn(mark), FadeIn(vapour[:n_show]), FadeOut(vapour[n_show:]), run_time=0.7)
        hold_for(self, N, "curve")

        caption = swap_caption(self, caption, subtitle_text(N, "room"))
        x_room = _x_sat(20) * 0.5 * (101325 - _p_sat(20)) / (101325 - 0.5 * _p_sat(20))
        T = ValueTracker(20.0)
        p_v = 0.5 * _p_sat(20)

        def x_now():
            t = T.get_value()
            return min(x_room, _x_sat(t))

        state = always_redraw(lambda: Dot(apt(T.get_value(), x_now()), radius=0.1, color=P_YELLOW))
        phi_gauge = meter("φ", length=air_box.height, thickness=0.36, color=P_BLUE)
        phi_gauge["group"].shift(box_c + RIGHT * (air_box.width / 2 + 0.55) - phi_gauge["track"].get_center())
        phi_gauge["label"].set_y(air_tag.get_center()[1])
        phi_gauge["fill"].add_updater(lambda m: set_meter(phi_gauge, min(1.0, p_v / _p_sat(T.get_value()))))
        phi_read = _readout(lambda: rf"\varphi = {label_number(min(100.0, 100 * p_v / _p_sat(T.get_value())))}\,\%",
                            box_c + DOWN * 1.35, color=P_BLUE, edge="center")
        st_read = _readout(lambda: rf"{label_number(T.get_value(), 1)}\,\mathrm{{°C}},\; x = {label_number(x_now(), 1)}\,\mathrm{{g/kg}}",
                           apt(-10, 0) + np.array([6.2, 3.25, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW,
                           edge="right")
        v20 = DashedLine(apt(20, 0), apt(20, _x_sat(20)), color=P_WHITE, stroke_width=1.8, stroke_opacity=0.7,
                         dash_length=0.1)
        tag100 = _m(r"100\,\%", apt(20, _x_sat(20)) + RIGHT * 0.2 + DOWN * 0.28, size=LABEL_FONT_SIZE, color=P_CYAN,
                    edge="left")
        tag50 = _m(r"50\,\%", apt(20, x_room) + RIGHT * 0.25 + DOWN * 0.07, size=LABEL_FONT_SIZE, color=P_YELLOW,
                   edge="left")
        self.play(FadeOut(marks), FadeOut(vapour[7:]), FadeIn(phi_gauge["group"]), run_time=0.7)
        _reveal(self, state, phi_read, st_read)
        self.play(Create(v20), FadeIn(tag100), FadeIn(tag50), run_time=1.0)
        hold_for(self, N, "room")

        caption = swap_caption(self, caption, subtitle_text(N, "dew"))
        cool_line = DashedLine(apt(20, x_room), apt(9.3, x_room), color=P_YELLOW, stroke_width=1.8, stroke_opacity=0.7,
                               dash_length=0.1)
        self.play(T.animate.set_value(9.3), Create(cool_line, rate_func=linear), run_time=2.6, rate_func=linear)
        dew_lbl = Text(tr("Dew point", "Taupunkt"), font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(apt(9.3, x_room), UP + LEFT,
                                                                                      buff=0.3)
        self.play(FadeIn(dew_lbl), Flash(apt(9.3, x_room), color=P_YELLOW, flash_radius=0.14, line_length=0.08),
                  run_time=0.8)
        hold_for(self, N, "dew")

        caption = swap_caption(self, caption, subtitle_text(N, "condense"))
        surf_x = box_c[0] - air_box.width / 2 - 0.1
        surf = Rectangle(width=0.14, height=air_box.height, color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN,
                         fill_opacity=0.25).move_to(np.array([surf_x, box_c[1], 0.0]))
        surf_lbl = Text(tr("cold surface", "kalte Oberfläche"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([surf_x - 0.25 - 0.5 * Text(tr("cold surface", "kalte Oberfläche"), font_size=LABEL_FONT_SIZE).width, box_c[1], 0.0]))
        trough = Rectangle(width=1.1, height=0.35, color=P_CYAN, stroke_width=2.5).move_to(
            np.array([surf_x + 0.5, box_c[1] - 1.9, 0.0]))
        trough_lvl = ValueTracker(0.0)
        trough_fill = always_redraw(lambda: Rectangle(
            width=1.04, height=max(0.002, 0.29 * trough_lvl.get_value()), stroke_width=0,
            fill_color=P_CYAN, fill_opacity=0.55).move_to(
                trough.get_bottom() + UP * (0.03 + 0.29 * trough_lvl.get_value() / 2)))
        x_cond = x_room - _x_sat(0.0)
        cond_read = _readout(lambda: tr(rf"\text{{condensate}}\;{label_number(x_cond * trough_lvl.get_value(), 1)}\,\mathrm{{g}}", rf"\text{{Kondensat}}\;{label_number(x_cond * trough_lvl.get_value(), 1)}\,\mathrm{{g}}"),
                             trough.get_right() + RIGHT * 0.2 + DOWN * 0.07, color=P_CYAN, edge="left")
        movers = [vapour[1], vapour[3], vapour[5]]
        rest = [vapour[0], vapour[2], vapour[4], vapour[6]]
        spots = [np.array([surf_x + 0.17, box_c[1] + dy, 0.0]) for dy in (0.55, 0.05, -0.4)]
        self.play(FadeOut(phi_read), FadeIn(surf), FadeIn(surf_lbl), run_time=0.6)
        self.play(T.animate.set_value(0.0), *[m.animate.move_to(sp).set_color(P_CYAN) for m, sp in zip(movers, spots)],
                  run_time=2.4, rate_func=linear)
        hold_for(self, N, "condense")

        caption = swap_caption(self, caption, subtitle_text(N, "drops"))
        def droplet(at):
            return VGroup(Circle(radius=0.075, stroke_width=1.2, color=P_WHITE, stroke_opacity=0.6, fill_color=P_CYAN,
                                 fill_opacity=0.85),
                          Dot(radius=0.02, color=P_WHITE).shift(UP * 0.03 + LEFT * 0.03)).move_to(at)

        drops = VGroup(*[droplet(sp) for sp in spots])
        drop_tag = Text(tr("Droplets", "Tropfen"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([surf_x - 0.75, box_c[1] + air_box.height / 2 + 0.3, 0.0]))
        self.play(*[ReplacementTransform(m, d) for m, d in zip(movers, drops)], FadeIn(drop_tag), FadeIn(trough),
                  FadeIn(cond_read), run_time=0.8)
        self.add(trough_fill)
        self.play(LaggedStart(*[d.animate.scale(1.9) for d in drops], lag_ratio=0.3), run_time=2.0)
        bottom_y = box_c[1] - air_box.height / 2 + 0.12
        self.play(LaggedStart(*[d.animate(rate_func=rate_functions.ease_in_quad).move_to(
            np.array([surf_x + 0.17, bottom_y, 0.0])) for d in drops], lag_ratio=0.25), run_time=3.0)
        self.play(LaggedStart(*[d.animate(rate_func=rate_functions.ease_in_quad).move_to(
            np.array([surf_x + 0.3 + 0.18 * k, trough.get_top()[1] + 0.02, 0.0])) for k, d in enumerate(drops)],
                              lag_ratio=0.25), trough_lvl.animate.set_value(1.0), run_time=3.2)
        self.play(*[FadeOut(d) for d in drops], run_time=0.4)
        hold_for(self, N, "drops")

        caption = swap_caption(self, caption, subtitle_text(N, "person"))
        phi_gauge["fill"].clear_updaters()
        self.play(FadeOut(VGroup(aax["group"], t_lbls, x_lbls, sat, sat_lbl, air_box, air_tag, *rest, dew_lbl,
                                 surf, surf_lbl, drop_tag, trough, cond_read, v20, tag100, tag50, cool_line,
                                 phi_gauge["group"])), FadeOut(trough_fill), FadeOut(state), FadeOut(st_read),
                  run_time=0.6)
        f = ValueTracker(0.0)
        bb, bu = -1.0, 0.03
        person = _person("shared-person-winter", np.array([-4.2, bb, 0.0]), 1.72)
        head = person.get_top() + DOWN * person.height * 0.07
        plume = [smooth_path([head + UP * 0.35, head + UP * 0.8 + RIGHT * 0.12, head + UP * 1.3 + LEFT * 0.05,
                              head + UP * 1.8 + RIGHT * 0.1])]
        mouth = person.get_top() + np.array([person.height * 0.075, -person.height * 0.105, 0.0])
        exhale = [smooth_path([mouth, mouth + np.array([0.45, 0.08 + dy, 0.0]), mouth + np.array([0.95, 0.3 + dy, 0.0]),
                               mouth + np.array([1.4, 0.6 + dy, 0.0])]) for dy in (-0.12, 0.08)]
        sens_tag = Text(tr("sensible: radiation, convection", "sensibel: Strahlung, Konvektion"), font_size=LABEL_FONT_SIZE, color=P_RED)
        sens_tag.next_to(plume[0].get_end(), LEFT, buff=0.2).align_to(np.array([-6.2, 0.0, 0.0]), LEFT)
        lat_tag = Text(tr("latent: water vapour", "latent: Wasserdampf"), font_size=LABEL_FONT_SIZE, color=P_BLUE)
        lat_tag.move_to(exhale[1].get_end() + UP * 0.32)

        def person_bars():
            s, l_ = 70 * f.get_value(), 30 * f.get_value()
            return VGroup(
                Rectangle(width=0.9, height=max(0.002, s * bu), stroke_width=0, fill_color=P_RED,
                          fill_opacity=0.7).move_to(np.array([0.2, bb + s * bu / 2, 0.0])),
                Rectangle(width=0.9, height=max(0.002, l_ * bu), stroke_width=0, fill_color=P_BLUE,
                          fill_opacity=0.7).move_to(np.array([1.6, bb + l_ * bu / 2, 0.0])),
            )

        bars = always_redraw(person_bars)
        s_read = _readout(lambda: tr(rf"{label_number(70 * f.get_value())}\,\mathrm{{W}}\;\text{{sensible}}", rf"{label_number(70 * f.get_value())}\,\mathrm{{W}}\;\text{{sensibel}}"),
                          np.array([0.2, bb + 2.1 + 0.25, 0.0]), size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        l_read = _readout(lambda: rf"{label_number(30 * f.get_value())}\,\mathrm{{W}}\;\text{{latent}}",
                          np.array([1.6, bb + 0.9 + 0.25, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE, edge="center")
        g_read = _readout(lambda: tr(rf"\approx {label_number(45 * f.get_value())}\,\mathrm{{g/h}}\;\text{{water}}", rf"\approx {label_number(45 * f.get_value())}\,\mathrm{{g/h}}\;\text{{Wasser}}"),
                          np.array([3.2, bb + 0.45, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        base = Line(np.array([-0.6, bb, 0.0]), np.array([2.4, bb, 0.0]), color=P_WHITE, stroke_width=2)
        self.play(FadeIn(person), Create(base), FadeIn(sens_tag), FadeIn(lat_tag), run_time=0.9)
        self.add(bars)
        _reveal(self, s_read, l_read, g_read)
        _paced_flow(self, [(plume, P_RED, None, _gradient([(0.0, P_RED), (1.0, P_ORANGE)]), False, 4),
                           (exhale, P_BLUE, None, None, False, 3)], run_time=2.4, waves=4, cycles=1.6, radius=0.05,
                    extra=[f.animate.set_value(1.0), _ripples([person.get_center() + UP * 0.3], r_max=0.7, color=P_RED)])
        hold_for(self, N, "person")

        caption = swap_caption(self, caption, subtitle_text(N, "rel_winter"))
        self.play(FadeOut(VGroup(person, sens_tag, lat_tag, base)),
                  *[FadeOut(m) for m in (bars, s_read, l_read, g_read)], run_time=0.5)
        head_y, foot_y = 2.3, -1.55
        wx, wt, w_lo, w_hi, w_bot, w_top = -4.2, 0.4, -0.45, 0.95, -1.2, 1.45
        wall_parts = VGroup(
            section_material(Rectangle(width=wt, height=w_lo - w_bot)).move_to(np.array([wx, (w_bot + w_lo) / 2, 0.0])),
            section_material(Rectangle(width=wt, height=w_top - w_hi)).move_to(np.array([wx, (w_hi + w_top) / 2, 0.0])),
        )
        pane = Line(np.array([wx, w_lo, 0.0]), np.array([wx, w_hi, 0.0]), color=P_CYAN, stroke_width=4)
        sill = Line(np.array([wx + 0.02, w_lo, 0.0]), np.array([wx + 0.38, w_lo, 0.0]), color=P_WHITE, stroke_width=3)
        room_lines = VGroup(Line(np.array([wx + wt / 2, w_bot, 0.0]), np.array([-1.6, w_bot, 0.0]), color=P_WHITE,
                                 stroke_width=2.5),
                            Line(np.array([wx + wt / 2, w_top, 0.0]), np.array([-1.6, w_top, 0.0]), color=P_WHITE,
                                 stroke_width=2.5))
        w_head = Text(tr("Winter: condensation on cold surfaces", "Winter: Tauwasser an kalten Flächen"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([-3.1, head_y, 0.0]))
        t_e = _m(r"\vartheta_{\mathrm{e}} = \text{−5}\,\mathrm{°C}", np.array([wx - 0.3, 0.3, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_BLUE, edge="right")
        t_i = _m(r"\vartheta_{\mathrm{i}} = 20\,\mathrm{°C},\; \varphi = 50\,\%", np.array([wx + 0.6, 1.1, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        w_foot = _m(r"\vartheta_{\mathrm{si}} \approx 1\,\mathrm{°C} < \vartheta_{\mathrm{Tau}} \approx 9\,\mathrm{°C}",
                    np.array([-3.6, foot_y, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN)
        rng_w = np.random.default_rng(21)
        stick_ys = np.sort(rng_w.uniform(w_lo + 0.2, w_hi - 0.1, 7))
        vapour_in = [smooth_path([np.array([rng_w.uniform(-2.5, -1.8), rng_w.uniform(-0.8, 0.8), 0.0]),
                                  np.array([rng_w.uniform(-3.6, -3.0), y + rng_w.uniform(-0.2, 0.2), 0.0]),
                                  np.array([wx + 0.06, y, 0.0])]) for y in stick_ys]
        drops = VGroup(*[Ellipse(width=0.07, height=0.1, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.9)
                         .move_to(np.array([wx + 0.06, y, 0.0])) for y in stick_ys])

        dx0, dx1, d_lo, d_hi, cx = 0.9, 6.3, -0.6, 1.15, 3.6
        duct = VGroup(Line(np.array([dx0, d_hi, 0.0]), np.array([dx1, d_hi, 0.0]), color=P_WHITE, stroke_width=3),
                      Line(np.array([dx0, d_lo, 0.0]), np.array([dx1, d_lo, 0.0]), color=P_WHITE, stroke_width=3))
        c_lo, c_hi = 0.0, d_hi - 0.08
        fins = VGroup(*[Line(np.array([cx + dx, c_lo, 0.0]), np.array([cx + dx, c_hi, 0.0]), color=P_BLUE, stroke_width=1.6)
                        for dx in np.linspace(-0.25, 0.25, 7)])
        coil = VGroup(Rectangle(width=0.7, height=c_hi - c_lo, color=P_BLUE, stroke_width=2).move_to(
            np.array([cx, (c_lo + c_hi) / 2, 0.0])), fins)
        tray = VMobject(color=P_WHITE, stroke_width=2.5).set_points_as_corners(
            [np.array([cx - 0.45, d_lo + 0.25, 0.0]), np.array([cx - 0.45, d_lo + 0.05, 0.0]),
             np.array([cx + 0.45, d_lo + 0.05, 0.0]), np.array([cx + 0.45, d_lo + 0.25, 0.0])])
        drain = Line(np.array([cx + 0.3, d_lo + 0.05, 0.0]), np.array([cx + 0.3, d_lo - 0.4, 0.0]), color=P_WHITE,
                     stroke_width=2.5)
        coil_tag = Text(tr("Cooling coil", "Kühlregister"), font_size=LABEL_FONT_SIZE, color=P_BLUE)
        coil_tag.move_to(np.array([cx - 0.6 - coil_tag.width / 2, d_lo - 0.32, 0.0]))
        cond_tag = Text(tr("Condensate", "Kondensat"), font_size=LABEL_FONT_SIZE, color=P_CYAN)
        cond_tag.move_to(np.array([cx + 0.5 + cond_tag.width / 2, d_lo - 0.55, 0.0]))
        s_head = Text(tr("Summer: cooling dehumidifies", "Sommer: Kühlung entfeuchtet"), font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(
            np.array([(dx0 + dx1) / 2, head_y, 0.0]))
        a_in = VGroup(
            _m(r"26\,\mathrm{°C} \cdot \varphi = 56\,\%", np.array([dx0 + 0.1, d_hi + 0.7, 0.0]),
               size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left"),
            _m(tr(r"x = 11.7\,\mathrm{g/kg}", r"x = 11{,}7\,\mathrm{g/kg}"), np.array([dx0 + 0.1, d_hi + 0.3, 0.0]),
               size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left"))
        a_out = VGroup(
            _m(r"14\,\mathrm{°C} \cdot \varphi = 95\,\%", np.array([dx1 - 0.1, d_hi + 0.7, 0.0]),
               size=LABEL_FONT_SIZE, color=P_BLUE, edge="right"),
            _m(tr(r"x = 9.5\,\mathrm{g/kg}", r"x = 9{,}5\,\mathrm{g/kg}"), np.array([dx1 - 0.1, d_hi + 0.3, 0.0]),
               size=LABEL_FONT_SIZE, color=P_BLUE, edge="right"))
        s_foot = _m(r"\vartheta_{\mathrm{Reg}} \approx 8\,\mathrm{°C} < \vartheta_{\mathrm{Tau}} \approx 16\,\mathrm{°C}",
                    np.array([(dx0 + dx1) / 2, foot_y, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE)
        air = [Line(np.array([dx0 - 0.3, y, 0.0]), np.array([dx1 + 0.3, y, 0.0])) for y in (0.2, 0.55, 0.9)]
        moist = [Line(np.array([dx0 - 0.3, y, 0.0]), np.array([cx - 0.36, y, 0.0])) for y in (0.08, 0.38, 0.72, 1.0)]
        drips = [Line(np.array([cx + dx, c_lo - 0.02, 0.0]), np.array([cx + dx, d_lo + 0.1, 0.0])) for dx in (-0.2, 0.05, 0.25)]
        drain_drop = [Line(np.array([cx + 0.3, d_lo - 0.4, 0.0]), np.array([cx + 0.3, d_lo - 0.75, 0.0]))]
        cool = _gradient([(0.0, P_ORANGE), (0.47, P_ORANGE), (0.53, P_BLUE), (1.0, P_BLUE)])

        self.play(FadeIn(w_head), FadeIn(wall_parts), Create(room_lines), Create(pane), Create(sill), run_time=1.6)
        self.play(FadeIn(t_e), FadeIn(t_i), run_time=1.0)
        _paced_flow(self, [(vapour_in, P_BLUE, None, None, False)], run_time=4.5, waves=1, cycles=1.0, radius=0.05,
                    extra=[LaggedStart(*[FadeIn(d, scale=0.3) for d in drops], lag_ratio=0.4)])
        self.play(LaggedStart(*[d.animate(rate_func=rate_functions.ease_in_quad).move_to(
            np.array([wx + 0.06 + 0.04 * k, w_lo + 0.05, 0.0])) for k, d in enumerate(drops)], lag_ratio=0.3),
                  run_time=3.0)
        self.play(FadeIn(w_foot), run_time=1.0)
        hold_for(self, N, "rel_winter")

        caption = swap_caption(self, caption, subtitle_text(N, "rel_summer"))
        self.play(FadeIn(s_head), run_time=1.2)
        self.play(Create(duct), run_time=1.6)
        self.play(FadeIn(coil), FadeIn(coil_tag), run_time=1.4)
        self.play(Create(tray), Create(drain), run_time=1.2)
        self.play(FadeIn(a_in), run_time=1.2)
        self.play(FadeIn(a_out), run_time=1.2)
        _paced_flow(self, [(air, P_ORANGE, None, cool, False), (moist, P_BLUE, None, None, False)],
                    run_time=6.0, waves=4, cycles=2.0, radius=0.045)
        self.play(FadeIn(cond_tag), FadeIn(s_foot), run_time=1.0)
        _paced_flow(self, [(air, P_ORANGE, None, cool, False), (moist, P_BLUE, None, None, False),
                           (drips, P_CYAN, None, None, False), (drain_drop, P_CYAN, None, None, False)],
                    run_time=6.0, waves=4, cycles=2.0, radius=0.045)
        hold_for(self, N, "rel_summer")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – Summary: seven building blocks of the video
class Beat8_Zusammenfassung(Scene):
    NARRATION = [
        ("power",
         "Power says how fast the energy flows — energy per time.",
         "Leistung sagt, wie schnell die Energie fließt — Energie pro Zeit."),
        ("energy",
         "Work is transferred energy: one kilowatt for one hour is one kilowatt-hour.",
         "Arbeit ist übertragene Energie: 1 kW über 1 Stunde ergibt 1 Kilowattstunde."),
        ("keep",
         "Energy is never lost: it leaves as heat or it is stored.",
         "Energie geht nie verloren: Sie kommt als Wärme heraus oder wird gespeichert."),
        ("pump",
         "The heat pump transfers energy to a higher temperature level — the COP counts the heat per kilowatt-hour of electricity.",
         "Die Wärmepumpe überträgt Energie auf ein höheres Temperaturniveau — der COP nennt die Wärme je kWh Strom."),
        ("sun",
         "Radiation comes through the glass short-wave and stays trapped long-wave — the g-value sets the gain.",
         "Strahlung kommt kurzwellig durchs Glas und bleibt langwellig gefangen — der g-Wert bestimmt den Gewinn."),
        ("mass",
         "Thermal mass stores heat and gives it back later.",
         "Thermische Masse speichert Wärme und gibt sie zeitversetzt wieder ab."),
        ("latent",
         "Heat sits in the temperature and in the water vapour — evaporating takes over five times as much as warming.",
         "Wärme steckt in der Temperatur und im Wasserdampf — Verdampfen braucht über fünfmal so viel wie Erwärmen."),
        ("next",
         "With these seven building blocks we now calculate heating and cooling.",
         "Mit diesen sieben Bausteinen rechnen wir nun Heizung und Kühlung."),
        ("refs",
         "To read more, two textbooks explain these topics step by step — the links are on screen.",
         "Zum Nachlesen: Zwei Lehrbücher erklären diese Themen Schritt für Schritt — die Links siehst du hier."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        subtitle = section_heading(tr("Summary — what we learned", "Zusammenfassung — was wir gelernt haben"))
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "power"))
        self.play(FadeIn(caption), run_time=0.3)

        specs = [
            ("power", tr("Power", "Leistung"), P_CYAN, r"P = E \,/\, t"),
            ("energy", tr("Work / energy", "Arbeit / Energie"), P_YELLOW, r"1\,\mathrm{kW} \cdot 1\,\mathrm{h} = 1\,\mathrm{kWh}"),
            ("keep", tr("Energy conservation", "Energieerhaltung"), P_RED, r"E_{\mathrm{ein}} = E_{\mathrm{aus}} + \Delta E_{\mathrm{Speicher}}"),
            ("pump", tr("Heat pump", "Wärmepumpe"), P_ORANGE, r"\mathrm{COP} = Q_{\mathrm{H}} \,/\, W_{\mathrm{el}}"),
            ("sun", tr("Radiation", "Strahlung"), P_YELLOW, r"\dot{Q}_{\mathrm{S}} = g \cdot A \cdot I"),
            ("mass", tr("Thermal mass", "Thermische Masse"), P_ORANGE, r"Q = m \cdot c \cdot \Delta T"),
            ("latent", tr("Sensible and latent", "Sensibel und latent"), P_BLUE, r"Q_{\mathrm{lat}} = m \cdot r"),
        ]
        top, pitch, x_bullet, x_topic, x_formula = 1.65, 0.62, -3.8, -3.5, 0.8
        rows = {}
        for k, (key, topic, color, formula) in enumerate(specs):
            y = top - k * pitch
            bullet = Square(side_length=0.2, stroke_width=0, fill_color=color, fill_opacity=0.85).move_to(
                np.array([x_bullet, y, 0.0]))
            label = Text(topic, font_size=LABEL_FONT_SIZE, color=color)
            label.move_to(np.array([x_topic + label.width / 2, y, 0.0]))
            math = _m(formula, np.array([x_formula, y - 0.08, 0.0]), size=BODY_FONT_SIZE, color=P_WHITE, edge="left")
            rows[key] = VGroup(bullet, label, math)

        for k, (key, *_rest) in enumerate(specs):
            if k:
                caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(FadeIn(rows[key], shift=RIGHT * 0.2), run_time=0.8)
            self.play(Indicate(rows[key][2], color=specs[k][2], scale_factor=1.08), run_time=0.7)
            hold_for(self, N, key, used=0.0 if k else BEAT_SUBTITLE_FADE + 0.3 + 1.5)

        caption = swap_caption(self, caption, subtitle_text(N, "next"))
        frame = SurroundingRectangle(VGroup(*rows.values()), color=P_PRIMARY, buff=0.3, stroke_width=2.5,
                                     corner_radius=0)
        self.play(Create(frame), run_time=1.0)
        hold_for(self, N, "next")

        caption = swap_caption(self, caption, subtitle_text(N, "refs"))
        books = [
            ("Willems WM, Schild K, Stricker D (2024)", "Formeln und Tabellen Bauphysik",
             "Wärmeschutz – Feuchteschutz – Klima – Akustik – Brandschutz", tr("8th edition, Wiesbaden", "8. Auflage, Wiesbaden"),
             "https://doi.org/10.1007/978-3-658-44974-2", tr("Radiation · Thermal mass · Moisture", "Strahlung · Thermische Masse · Feuchte"), P_YELLOW),
            ("Bohne D (2022)", "Gebäudetechnik und technischer Ausbau von Gebäuden",
             "", tr("12th edition, Wiesbaden", "12. Auflage, Wiesbaden"),
             "https://doi.org/10.1007/978-3-658-37488-4", tr("Heat pump · Ventilation · Building services", "Wärmepumpe · Lüftung · Anlagentechnik"), P_CYAN),
        ]
        cards = []
        for who, book, subtitle, edition, link, topics, color in books:
            lines = [Text(who, font_size=LABEL_FONT_SIZE, color=color), Text(book, font_size=BODY_FONT_SIZE, color=P_WHITE)]
            if subtitle:
                lines.append(Text(subtitle, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))
            lines += [Text(f"{edition} · {topics}", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7),
                      Text(link, font_size=LABEL_FONT_SIZE, color=P_PRIMARY)]
            cards.append(VGroup(*lines).arrange(DOWN, buff=0.1))
        VGroup(*cards).arrange(DOWN, buff=0.5).move_to(np.array([0.0, 0.1, 0.0]))
        self.play(FadeOut(VGroup(*rows.values(), frame)), run_time=0.6)
        for card in cards:
            self.play(FadeIn(card, shift=UP * 0.15), run_time=0.9)
        hold_for(self, N, "refs")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Curriculum order
# 📚 The one beat order every compose, audio and build script imports.
BEATS = [
    Beat1_Leistung,
    Beat2_EnergieImAlltag,
    Beat3_Energieerhaltung,
    Beat4_Waermepumpe,
    Beat5_Strahlung,
    Beat6_ThermischeMasse,
    Beat7_SensibelLatent,
    Beat8_Zusammenfassung,
]
#endregion
