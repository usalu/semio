

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


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
body_text = _manim_module.body_text
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
room_metre = _manim_module.room_metre
P_PRIMARY = _manim_module.P_PRIMARY
section_material = _manim_module.section_material
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
note_line = _manim_module.note_line
stacked_bar = _manim_module.stacked_bar
dim_arrow = _manim_module.dim_arrow
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
label_number = _manim_module.label_number
caption_bar = _manim_module.caption_bar
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
tr = _manim_module.tr
load_vo_timing = _manim_module.load_vo_timing
room_section = _manim_module.room_section
staffage = _manim_module.staffage
staffage_motion = _manim_module.staffage_motion
staffage_height = _manim_module.staffage_height
fitted_staffage = _manim_module.fitted_staffage
ripples = _manim_module.ripples
droplets = _manim_module.droplets

set_vo_language("de")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Von der Nutzenergie zur Endenergie"

NUTZ_BLUE = semio_color("secondary")
UEBER_RED = semio_color("primary")
VERT_ORANGE = semio_color("tertiary")
SPEI_YELLOW = semio_color("warning")
ERZ_AMBER = semio_color("tertiary")
END_RED = semio_color("danger")
AMBIENT_GREEN = semio_color("success")
BOILER_RATE = 60.0
TANK_CHARGE_S = 900.0

# The one worked example the rest of the series keeps quoting: 100 kWh/(m²·a)
# of useful heat, delivered once by a gas boiler and once by a heat pump. Every
# number below is a schematic teaching example, not a measured or normed value.
Q_NUTZ = 100.0

# —— The Anlagenaufwandszahl chain (Beats 2–4) — one set of factors used
# everywhere, so the multiplication a viewer reproduces by hand always matches
# what is on screen. Erzeugung is kept separate for the plain and the
# condensing boiler, since Beat 3 shows both and they must not share a label.
E_UEBERGABE = 1.04
E_VERTEILUNG = 1.08
E_SPEICHERUNG = 1.06
E_ERZEUGUNG_GAS = 1.18            # Gaskessel: 118 kWh Erdgas → 100 kWh Wärme
E_ERZEUGUNG_GAS_BRENNWERT = 1.07  # nach Kondensation: 107 kWh → 100 kWh
E_ERZEUGUNG_WP = 0.26             # Wärmepumpe: 26 kWh Strom → 100 kWh Wärme

E_P_GAS = E_UEBERGABE * E_VERTEILUNG * E_SPEICHERUNG * E_ERZEUGUNG_GAS  # ≈ 1.40
E_P_WP = E_UEBERGABE * E_VERTEILUNG * E_SPEICHERUNG * E_ERZEUGUNG_WP    # ≈ 0.31
Q_E_GAS = Q_NUTZ * E_P_GAS  # ≈ 140 kWh/(m²·a)
Q_E_WP = Q_NUTZ * E_P_WP    # ≈ 31 kWh/(m²·a)


def _cascade(factors) -> tuple[float, ...]:
    """🪜 Energy each stage adds when the useful heat is multiplied up the chain."""
    added, level = [], Q_NUTZ
    for factor in factors:
        added.append(level * factor - level)
        level *= factor
    return tuple(added)


# Beat 1's staircase is the same multiplication Beat 4 shows as e_p, so its
# segments are derived from the factors rather than typed — typed values once
# summed to 135 while Beat 4 printed 140 for the very same boiler.
STAGES = tuple(zip(
    ("Übergabe", "Verteilung", "Speicherung", "Erzeugung"),
    _cascade((E_UEBERGABE, E_VERTEILUNG, E_SPEICHERUNG, E_ERZEUGUNG_GAS)),
    (UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER),
))  # ≈ +4, +8, +7, +21 kWh → Q_E_GAS

LOSS = tuple(value for _name, value, _colour in STAGES)
LEVELS = tuple(Q_NUTZ + sum(LOSS[:k]) for k in range(5))  # 100, 104, 112, 119, 140 kWh


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

FLOW_UNIT = 0.0050
LANE_PITCH = 0.042
PARTICLE_GAP = 0.5
PARTICLE_R = 0.034
_KAPPA = 0.5523
_UNIT_CIRCLE = np.array([
    [1, 0, 0], [1, _KAPPA, 0], [_KAPPA, 1, 0], [0, 1, 0], [0, 1, 0], [-_KAPPA, 1, 0], [-1, _KAPPA, 0], [-1, 0, 0],
    [-1, 0, 0], [-1, -_KAPPA, 0], [-_KAPPA, -1, 0], [0, -1, 0], [0, -1, 0], [_KAPPA, -1, 0], [1, -_KAPPA, 0], [1, 0, 0],
], dtype=float)
FLOW_PACE = 0.9
_FLOW_CLOCK = [0.0]

ROOM_C = np.array([-3.55, 0.97, 0.0])
ROOM_W, ROOM_H = 5.4, 1.6
GROUND_Y = -1.40
PIPE_Y = -0.58
APPLIANCE_Y = GROUND_Y + 0.5
RAD_X, TANK_X, BOILER_X, METER_X, PERSON_X = -5.45, -3.9, -1.9, 0.30, -1.10
WP_STROM = LEVELS[3] * E_ERZEUGUNG_WP
WP_UMWELT = LEVELS[3] - WP_STROM


def _rounded_polyline(points, radius: float = 0.7, per_corner: int = 9) -> np.ndarray:
    """📐 Dense polyline through ``points`` whose corners are rounded by quadratic Béziers."""
    pts = [np.array(p, dtype=float) for p in points]
    out = [pts[0]]
    for a, b, c in zip(pts, pts[1:], pts[2:]):
        u, v = a - b, c - b
        lu, lv = np.linalg.norm(u), np.linalg.norm(v)
        r = min(radius, 0.48 * lu, 0.48 * lv)
        p0, p2 = b + u / lu * r, b + v / lv * r
        out.extend((1 - t) ** 2 * p0 + 2 * (1 - t) * t * b + t ** 2 * p2 for t in np.linspace(0.0, 1.0, per_corner))
    out.append(pts[-1])
    return np.array(out)


def _resample(points: np.ndarray, n: int = 110):
    """📏 Points at equal arc length with unit normals (the left of the travel direction) and the path length."""
    seg = np.linalg.norm(np.diff(points, axis=0), axis=1)
    arc = np.concatenate([[0.0], np.cumsum(seg)])
    samples = np.linspace(0.0, arc[-1], n)
    pts = np.stack([np.interp(samples, arc, points[:, k]) for k in range(3)], axis=1)
    tangent = np.gradient(pts, axis=0)
    tangent /= np.linalg.norm(tangent, axis=1)[:, None]
    normals = np.stack([-tangent[:, 1], tangent[:, 0], np.zeros(n)], axis=1)
    return pts, normals, float(arc[-1])


class _Stream:
    """🌊 Energy stream — layered ribbon whose width per layer is kWh, carrying particles of the same width.

    ``layers`` are ``(tracker, colour, taper)``; a layer's width is its tracker value times ``FLOW_UNIT`` and a
    tapered layer shrinks linearly to nothing along the path. Layers stack on the left of the travel direction
    from offset ``start``, so every ribbon in a chain keeps one common base line.
    """

    def __init__(self, points, layers, *, start: float, cap: float, seed: int, radius: float = 0.7):
        self.pts, self.nrm, self.length = _resample(_rounded_polyline(points, radius))
        self.layers, self.start = layers, start
        self.grid = np.linspace(0.0, 1.0, len(self.pts))
        self.lanes = int(np.ceil(cap * FLOW_UNIT / LANE_PITCH)) + 1
        self.per = max(2, int(round(self.length / PARTICLE_GAP)))
        rng = np.random.default_rng(seed)
        self.phase = (np.arange(self.per)[None, :] + rng.uniform(0.0, 1.0, (self.lanes, 1))) / self.per
        self.offset = start + (np.repeat(np.arange(self.lanes), self.per) + 0.5) * LANE_PITCH
        self._seen = [None] * len(layers)
        self.ribbons = VGroup(*[VMobject(stroke_width=0) for _ in layers])
        for i, ribbon in enumerate(self.ribbons):
            ribbon.add_updater(lambda m, i=i: self._shape(i, m))
            self._shape(i, ribbon)

    def edges(self, s: np.ndarray) -> np.ndarray:
        """🧱 Cumulative layer edges at path fractions ``s`` — row ``i`` is the lower edge of layer ``i``."""
        widths = [max(tr.get_value(), 0.0) * FLOW_UNIT * ((1.0 - s) if taper else np.ones_like(s))
                  for tr, _c, taper in self.layers]
        return self.start + np.vstack([np.zeros_like(s), np.cumsum(widths, axis=0)])

    def total(self) -> float:
        """🧮 Stream width in kWh at its start."""
        return float(sum(max(tr.get_value(), 0.0) for tr, _c, _t in self.layers))

    def colour(self, i: int):
        """🎨 Colour of layer ``i`` — layers may follow a tracker through a callable."""
        c = self.layers[i][1]
        return c() if callable(c) else c

    def _shape(self, i: int, mob) -> None:
        signature = (tuple(round(tr.get_value(), 3) for tr, _c, _t in self.layers), self.colour(i))
        if self._seen[i] == signature:
            return
        self._seen[i] = signature
        edges = self.edges(self.grid)
        lo, hi = edges[i], edges[i + 1]
        if float((hi - lo).max()) < 1e-3:
            mob.clear_points()
            return
        a = self.pts + self.nrm * lo[:, None]
        b = self.pts + self.nrm * hi[:, None]
        mob.set_points_as_corners(np.concatenate([a, b[::-1], a[:1]]))
        mob.set_fill(self.colour(i), opacity=0.45)
        mob.set_stroke(width=0)

    def advance(self, clock: float, holders) -> None:
        """🏃 Place every particle for stream time ``clock``; lanes beyond the current width shrink to nothing."""
        t = ((self.phase + FLOW_PACE * clock / self.length) % 1.0).ravel()
        edges = self.edges(t)
        weight = np.clip((edges[-1] - self.offset) / LANE_PITCH + 0.5, 0.0, 1.0)
        layer = np.minimum((self.offset[None, :] >= edges[1:-1]).sum(axis=0), len(self.layers) - 1)
        f = t * (len(self.pts) - 1)
        i0 = np.minimum(f.astype(int), len(self.pts) - 2)
        u = (f - i0)[:, None]
        centre = self.pts[i0] * (1 - u) + self.pts[i0 + 1] * u
        normal = self.nrm[i0] * (1 - u) + self.nrm[i0 + 1] * u
        pos = centre + normal * self.offset[:, None]
        size = weight * np.clip(np.minimum(t / 0.05, (1.0 - t) / 0.07), 0.0, 1.0)
        for i, mob in enumerate(holders):
            pick = (layer == i) & (size > 0.05)
            if not pick.any():
                mob.clear_points()
                continue
            mob.set_points((pos[pick][:, None, :] + (PARTICLE_R * np.sqrt(size[pick]))[:, None, None]
                            * _UNIT_CIRCLE[None]).reshape(-1, 3))
            mob.set_fill(self.colour(i), opacity=0.95)
            mob.set_stroke(width=0)


def _flow(streams, rt: float):
    """💨 The particles of several ``_Stream``s as one animation; stream time runs on across plays so nothing jumps."""
    holders = [(stream, [VMobject(stroke_width=0) for _ in stream.layers]) for stream in streams]
    group = VGroup(*[mob for _stream, mobs in holders for mob in mobs])
    base = _FLOW_CLOCK[0]
    _FLOW_CLOCK[0] += rt

    def update(_mob, alpha):
        for stream, mobs in holders:
            stream.advance(base + alpha * rt, mobs)

    return UpdateFromAlphaFunc(group, update, remover=True, rate_func=linear)


def _readout(fn, at, *, size: float | None = None, color: str = P_WHITE, edge: str = "left"):
    """📟 Live typeset label that is rebuilt only when its text changes — ``math_readout`` redraws every frame."""
    holder = VGroup()
    state = {"text": None}

    def refresh(mob):
        text = fn()
        if text == state["text"]:
            return
        state["text"] = text
        place = np.array(at() if callable(at) else at, dtype=float)
        fresh = math_label(text, place, size=size, color=color, edge=edge) if text is not None else VGroup()
        if mob.submobjects:
            mob.submobjects[0].become(fresh)
        else:
            mob.add(fresh)

    holder.add_updater(refresh)
    refresh(holder)
    return holder


def _still(mob):
    """🧊 Frozen copy of a ribbon's current shape — the piece that lifts off a stream and morphs into a bar."""
    frozen = VMobject(stroke_width=0)
    frozen.set_points(mob.points.copy())
    frozen.set_fill(mob.get_fill_color(), opacity=mob.get_fill_opacity())
    return frozen


def _stand(room, x: float):
    """🧍 Breathing thermal occupant at true scale — 70 % of the clear room height, feet on the floor slab's top edge."""
    cast = "shared-person-winter"
    return staffage_motion(cast, unit=room_metre(room["h"]), anchor=np.array([x, room["y_f"], 0.0]))


def _boiler(center, w: float = 0.80, h: float = 1.0):
    """♨️ Gas boiler in elevation, fitted into the ``w`` × ``h`` box around ``center``; its heating cycle plays through scene time."""
    return fitted_staffage("shared-boiler-heating-cycle", width=w, height=h, rate=BOILER_RATE).move_to(np.array(center, dtype=float))


def _heat_pump(center, w: float = 1.0, h: float = 1.0):
    """🌬️ Air-source heat pump outdoor unit in winter air, fitted into the ``w`` × ``h`` box around ``center``; its clip plays through scene time at ``rate``."""
    return fitted_staffage("shared-heat-pump-heating-sweep", width=w, height=h, rate=1.0).move_to(np.array(center, dtype=float))


def _tank(center, w: float = 0.80, h: float = 1.0, level: float = 1.0):
    """🛢️ Hot-water cylinder whose charge is the heat still stored — ``level`` is a 0–1 tracker that seeks the charge-up clip."""
    tracker = ValueTracker(level)
    group = fitted_staffage("shared-tank-charge-up", width=w, height=h, rate=0.0).move_to(np.array(center, dtype=float))
    group.add_updater(lambda m: m.seek(float(np.clip(tracker.get_value(), 0.0, 1.0)) * TANK_CHARGE_S))
    return {"group": group, "fill": VGroup(), "level": tracker}


def _chimney(x: float, y0: float, y1: float):
    """🏭 Flue stack outside the right wall."""
    return VGroup(Line(np.array([x - 0.13, y0, 0.0]), np.array([x - 0.13, y1, 0.0]), color=P_WHITE, stroke_width=2),
                  Line(np.array([x + 0.13, y0, 0.0]), np.array([x + 0.13, y1, 0.0]), color=P_WHITE, stroke_width=2))


def _floor_loop(x0: float, x1: float, y: float, color=P_CYAN):
    """➰ Underfloor heating loop embedded in the floor slab."""
    xs = np.linspace(x0, x1, 44)
    loop = VMobject(color=color, stroke_width=3.2)
    loop.set_points_smoothly([np.array([x, y + 0.045 * np.sin((x - x0) * 9.0), 0.0]) for x in xs])
    return loop


def _ratio_row(name: str, symbol: str, color: str, y: float, *, out: float, value, shown=lambda: True,
               x0: float = 0.75, length: float = 2.2, top: float = 1.45, name_w: float = 1.55,
               tick_label: bool = False):
    """📊 One factor gauge — track 0 … ``top``, a tick at 1 and a fill that follows ``value()``.

    Up to 1 the fill is the useful heat (blue, one unit comes out); whatever lies beyond 1 is the extra energy
    the stage swallows, in its own colour — so a factor above one reads as an overshoot and the heat pump's 0,26
    as a short bar. The readout counts ``in / out = factor`` from the live value, ``out`` being the kWh that leave
    the stage.
    """
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=color).move_to(np.array([x0, y, 0.0]), aligned_edge=LEFT)
    sym = math_label(symbol, np.array([x0 + name_w, y, 0.0]), size=BODY_FONT_SIZE, color=color, edge="left")
    left = x0 + name_w + 0.58
    unit = length / top
    track = Rectangle(width=length, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK,
                      fill_opacity=1.0).move_to(np.array([left + length / 2, y, 0.0]))
    base = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=NUTZ_BLUE, fill_opacity=0.9)
    over = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=color, fill_opacity=0.95)

    def shape(_m=None):
        e = value()
        b, o = min(e, 1.0) * unit, max(e - 1.0, 0.0) * unit
        base.stretch_to_fit_width(max(b, 1e-3)).move_to(np.array([left + b / 2, y, 0.0]))
        over.stretch_to_fit_width(max(o, 1e-3)).move_to(np.array([left + unit + o / 2, y, 0.0]))
        over.set_fill(opacity=0.95 if o > 1e-3 else 0.0)

    shape()
    base.add_updater(shape)
    tick = Line(np.array([left + unit, y - 0.17, 0.0]), np.array([left + unit, y + 0.17, 0.0]), color=P_WHITE,
                stroke_width=1.8)
    group = VGroup(label, sym, track, base, over, tick)
    if tick_label:
        group.add(math_label("1", np.array([left + unit, y + 0.33, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE))
    readout = _readout(
        lambda: (rf"=\frac{{{label_number(round(value() * out))}}}{{{label_number(round(out))}}}={label_number(value(), 2)}"
                 if shown() else None),
        np.array([left + length + 0.2, y, 0.0]), size=LABEL_FONT_SIZE, color=color, edge="left")
    return {"group": group, "label": label, "symbol": sym, "track": track, "readout": readout, "y": y,
            "color": color, "tick_x": left + unit, "value": value}


def _stage_row(name: str, symbol: str, color: str, y: float, loss, out: float, **kwargs):
    """🎚️ ``_ratio_row`` for one stage: the ``unit`` tracker lifts the useful unit to 1, the stage's loss tracker adds the rest."""
    unit = ValueTracker(0.0)
    row = _ratio_row(name, symbol, color, y, out=out, value=lambda: unit.get_value() + loss.get_value() / out,
                     shown=lambda: unit.get_value() > 0.999 and loss.get_value() > 0.05, **kwargs)
    row["unit"] = unit
    return row


def _product_row(name: str, symbol: str, y: float, factors, *, lift=lambda: 1.0, out: float = Q_NUTZ,
                 x0: float = 0.75, length: float = 2.2, top: float = 1.45, name_w: float = 1.55):
    """✖️ Gauge of the product of several stage factors — each stage's share of the overshoot stacks in its own colour.

    ``factors`` is a list of ``(value(), colour)``; ``lift()`` scales the useful unit from 0 to 1 first. The bar's
    end is the running product, so a factor below one cuts the stack back down and a product below one leaves only
    the blue useful part.
    """
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([x0, y, 0.0]), aligned_edge=LEFT)
    sym = math_label(symbol, np.array([x0 + name_w, y, 0.0]), size=BODY_FONT_SIZE, color=P_WHITE, edge="left")
    left = x0 + name_w + 0.58
    unit = length / top
    track = Rectangle(width=length, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK,
                      fill_opacity=1.0).move_to(np.array([left + length / 2, y, 0.0]))
    base = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=NUTZ_BLUE, fill_opacity=0.9)
    parts = VGroup(*[Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=c, fill_opacity=0.95)
                     for _v, c in factors])

    def product() -> float:
        return float(lift() * np.prod([f() for f, _c in factors]))

    def shape(_m=None):
        b = min(product(), 1.0) * unit
        base.stretch_to_fit_width(max(b, 1e-3)).move_to(np.array([left + b / 2, y, 0.0]))
        level = lift()
        for part, (f, _c) in zip(parts, factors):
            nxt = level * f()
            lo, hi = max(1.0, min(level, nxt)), max(1.0, max(level, nxt))
            width = (hi - lo) * unit
            part.stretch_to_fit_width(max(width, 1e-3)).move_to(np.array([left + unit * (lo + hi) / 2, y, 0.0]))
            part.set_fill(opacity=0.95 if width > 1e-3 else 0.0)
            level = nxt

    shape()
    base.add_updater(shape)
    tick = Line(np.array([left + unit, y - 0.17, 0.0]), np.array([left + unit, y + 0.17, 0.0]), color=P_WHITE,
                stroke_width=1.8)
    readout = _readout(
        lambda: rf"=\frac{{{label_number(round(product() * out))}}}{{{label_number(round(out))}}}={label_number(product(), 2)}",
        np.array([left + length + 0.2, y, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
    return {"group": VGroup(label, sym, track, base, parts, tick), "label": label, "symbol": sym, "readout": readout,
            "product": product, "y": y, "tick_x": left + unit}


def _swap(scene, old, text_de: str, live, run_time: float = 0.35):
    """💬 ``swap_caption`` that keeps particles and heat ripples running through the two fades."""
    new = caption_bar(text_de)
    half = run_time / 2
    scene.play(FadeOut(old), *live(half), run_time=half)
    scene.play(FadeIn(new), *live(half), run_time=half)
    scene._vo_caption_t0 = float(getattr(scene.renderer, "time", 0.0) or 0.0)
    return new


def _plant(*, with_person: bool = True):
    """🏠 The building section every beat of the chapter draws — room over cellar, plant parts and their streams.

    The heat path runs gas meter → boiler → tank → distribution → radiator → room; each segment is a
    ``_Stream`` whose layers are the useful heat plus every loss still to come, so the ribbon widens by
    exactly one stage's loss at each node walking upstream.
    """
    room = room_section(ROOM_C, w=ROOM_W, h=ROOM_H)
    under_slab = room["y_f"] - room["slab"]
    cellar_h = under_slab - GROUND_Y
    cellar_walls = VGroup(*[section_material(Rectangle(width=0.2, height=cellar_h)).move_to(
        np.array([x, under_slab - cellar_h / 2, 0.0])) for x in (room["x_l"] - 0.1, room["x_r"] + 0.1)])
    cellar_air = Rectangle(width=ROOM_W, height=cellar_h, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.05).move_to(
        np.array([ROOM_C[0], under_slab - cellar_h / 2, 0.0]))
    ground = Line(np.array([room["x_l"] - 0.5, GROUND_Y, 0.0]), np.array([1.3, GROUND_Y, 0.0]), color=P_PRIMARY,
                  stroke_width=2.2)
    hatches = VGroup(*[Line(ground.point_from_proportion(u), ground.point_from_proportion(u) + np.array([-0.07, -0.1, 0.0]),
                            color=P_PRIMARY, stroke_width=1.1) for u in np.linspace(0.03, 0.97, 24)])
    shell = VGroup(room["group"], cellar_air, cellar_walls, ground, hatches)

    metre = room_metre(room["h"])
    rad_c = np.array([RAD_X, room["y_f"] + (0.12 + 0.65 / 2) * metre, 0.0])
    rad = staffage("shared-radiator-winter", unit=metre, anchor=np.array([RAD_X, room["y_f"], 0.0]))
    rad.add(_manim_module.radiator_wall(np.array([RAD_X, room["y_f"], 0.0]), metre))
    person = _stand(room, PERSON_X) if with_person else None
    tank = _tank(np.array([TANK_X, APPLIANCE_Y, 0.0]))
    boiler = _boiler(np.array([BOILER_X, APPLIANCE_Y, 0.0]))
    meter = fitted_staffage("shared-gas-meter-run", width=0.54, height=0.6, rate=1.0, loop=True).move_to(np.array([METER_X, PIPE_Y, 0.0]))
    chimney = _chimney(-0.30, -0.34, 1.55)

    v = {name: ValueTracker(0.0) for name in
         ("z0n", "z2n", "z2u", "z2v", "z3n", "z3u", "z3v", "z3s", "z4n", "z4u", "z4v", "z4s", "z4e", "z4w", "flue", "env")}
    target = {"z0n": Q_NUTZ, "z2n": Q_NUTZ, "z2u": LOSS[0], "z2v": LOSS[1], "z3n": Q_NUTZ, "z3u": LOSS[0],
              "z3v": LOSS[1], "z3s": LOSS[2], "z4n": Q_NUTZ, "z4u": LOSS[0], "z4v": LOSS[1], "z4s": LOSS[2],
              "z4e": LOSS[3], "z4w": WP_STROM, "flue": LOSS[3], "env": WP_UMWELT}
    half = 0.5 * Q_NUTZ * FLOW_UNIT
    cols = (NUTZ_BLUE, UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER)
    rad_out = rad.get_right()
    z0 = _Stream([rad_out, np.array([-4.2, rad_out[1], 0.0]), np.array([-3.1, rad_out[1], 0.0])],
                 [(v["z0n"], cols[0], False)], start=-half, cap=Q_NUTZ, seed=1)
    z2 = _Stream([np.array([TANK_X - 0.40, PIPE_Y, 0.0]), np.array([RAD_X, PIPE_Y, 0.0]),
                  rad.get_bottom()],
                 [(v["z2n"], cols[0], False), (v["z2u"], cols[1], False), (v["z2v"], cols[2], True)],
                 start=-half, cap=LEVELS[2], seed=2)
    z3 = _Stream([np.array([BOILER_X - 0.40, PIPE_Y, 0.0]), np.array([TANK_X + 0.40, PIPE_Y, 0.0])],
                 [(v["z3n"], cols[0], False), (v["z3u"], cols[1], False), (v["z3v"], cols[2], False),
                  (v["z3s"], cols[3], False)], start=-half, cap=LEVELS[3], seed=3)
    z4 = _Stream([np.array([METER_X - 0.27, PIPE_Y, 0.0]), np.array([BOILER_X + 0.40, PIPE_Y, 0.0])],
                 [(v["z4n"], cols[0], False), (v["z4u"], cols[1], False), (v["z4v"], cols[2], False),
                  (v["z4s"], cols[3], False), (v["z4e"], cols[4], False), (v["z4w"], P_YELLOW, False)],
                 start=-half, cap=LEVELS[4], seed=4)
    flue_y = under_slab - 0.12
    flue = _Stream([np.array([BOILER_X + 0.15, APPLIANCE_Y + 0.46, 0.0]), np.array([BOILER_X + 0.15, flue_y, 0.0]),
                    np.array([-0.30, flue_y, 0.0]), np.array([-0.30, 1.50, 0.0])],
                   [(v["flue"], P_RED, False)], start=-0.5 * LOSS[3] * FLOW_UNIT, cap=LOSS[3], seed=5, radius=0.3)

    env = _Stream([np.array([room["x_r"] + 0.30, PIPE_Y - 0.27, 0.0]), np.array([BOILER_X + 0.40, PIPE_Y - 0.27, 0.0])],
                  [(v["env"], AMBIENT_GREEN, False)], start=-0.5 * WP_UMWELT * FLOW_UNIT, cap=WP_UMWELT, seed=6)
    power_meter = fitted_staffage("shared-electricity-meter-run", width=0.54, height=0.6, rate=1.0, loop=True).move_to(np.array([METER_X, PIPE_Y, 0.0]))

    def heat(name: str, rt: float, r_max: float = 0.4):
        """🌡️ Ripples for the named warm place, cycling for ``rt`` seconds."""
        cyc = max(1.0, rt / 1.4)
        top = rad_c + UP * 0.30
        spots = {
            "rad": ripples([top], r_max=r_max, color=P_ORANGE, cycles=cyc),
            "ue": ripples([rad_c + LEFT * 0.47], facing=PI, r_max=0.32, color=UEBER_RED, cycles=cyc),
            "ve": ripples([np.array([-4.75, PIPE_Y - 0.43, 0.0])], down=True, r_max=0.30, color=VERT_ORANGE, cycles=cyc),
            "gain": ripples([np.array([RAD_X + 0.42, room["y_f"] + 0.02, 0.0])], facing=PI / 2, r_max=0.22,
                            color=VERT_ORANGE, cycles=cyc),
            "sp": ripples([np.array([TANK_X, APPLIANCE_Y + 0.5, 0.0])], r_max=0.30, color=SPEI_YELLOW, cycles=cyc),
        }
        if person is not None:
            spots["person"] = ripples([person.get_top() + UP * 0.05], r_max=0.36, color=P_ORANGE, cycles=cyc)
        return spots[name]

    return {"room": room, "shell": shell, "radiator": rad, "person": person, "tank": tank, "boiler": boiler,
            "meter": meter, "chimney": chimney, "v": v, "target": target, "z0": z0, "z2": z2, "z3": z3, "z4": z4,
            "flue": flue, "env": env, "power_meter": power_meter,
            "streams": (z0, z2, z3, z4, flue, env), "heat": heat, "rad_c": rad_c, "under_slab": under_slab}


def _grow(plant, names, run_time: float = 1.4, rate=smooth):
    """📈 Animations that swell the named stream layers from zero to their kWh values together."""
    return [plant["v"][n].animate(run_time=run_time, rate_func=rate).set_value(plant["target"][n]) for n in names]
#endregion


#region Beat 2 – Übergabe, Verteilung, Speicherung

class Beat2_UebergabeVerteilungSpeicherung(Scene):
    NARRATION = [
        ("intro",
         "Walk the chain backwards through a real building, starting in the room itself.",
         "Gehen wir die Kette rückwärts durch ein echtes Gebäude — beginnend im Raum selbst."),
        ("uebergabe",
         "Emission: how the surface hands heat to the air.",
         "Übergabe: wie die Fläche Wärme an die Luft abgibt."),
        ("flaeche",
         "A radiator needs hot water, underfloor heating needs far less.",
         "Ein Heizkörper braucht heißes Wasser, eine Fußbodenheizung viel weniger."),
        ("verteilung",
         "Distribution: every pipe metre loses heat.",
         "Verteilung: Jeder Rohrmeter verliert Wärme."),
        ("huelle",
         "Outside the envelope that heat is lost for good — inside it partly comes back as a gain.",
         "Außerhalb der Hülle ist sie weg — innerhalb kommt sie teilweise als Gewinn zurück."),
        ("speicher",
         "Storage: a tank kept warm around the clock loses heat even without any draw-off.",
         "Speicherung: Ein warm gehaltener Speicher verliert auch ohne Zapfung Wärme."),
        ("woche",
         "That goes on for as long as a whole week.",
         "Das geht selbst eine ganze Woche lang so."),
        ("kennzahl",
         "Each stage gets one expense figure.",
         "Jede Stufe bekommt eine Aufwandszahl."),
        ("einheit",
         "It tells how much has to go in so that one unit comes out.",
         "Sie sagt, wie viel hinein muss, damit eine Einheit herauskommt."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("Emission, distribution, storage", "Übergabe, Verteilung, Speicherung"))
        din = _din_ref("DIN V 18599-5:2018-09 · -6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        active: list = []
        extra: list = []

        def live(rt, *heat):
            return [_flow(active, rt), *[fn(rt) for fn in extra], *[P["heat"](n, rt) for n in heat]]

        def step(caption, key, *heat):
            return _swap(self, caption, subtitle_text(self.NARRATION, key), lambda rt: live(rt, *heat))

        envelope = DashedVMobject(
            Rectangle(width=ROOM_W + 0.64, height=ROOM_H + 0.26 + 0.12 + 0.26, color=P_PRIMARY, stroke_width=2.4).move_to(
                np.array([ROOM_C[0], P["under_slab"] + (ROOM_H + 0.64) / 2, 0.0])), num_dashes=70)
        shell_tag = body_text(tr("heated envelope", "beheizte Hülle"), font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY).move_to(
            np.array([0.45, 1.80, 0.0]), aligned_edge=LEFT)
        cellar_tag = body_text(tr("unheated basement", "unbeheizter Keller"), font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(
            np.array([ROOM_C[0], GROUND_Y - 0.30, 0.0]))
        cellar_tag.set_opacity(0.7)

        specs = {
            "ue": (tr("Emission", "Übergabe"), r"e_{ü}", UEBER_RED, v["z2u"], LEVELS[0]),
            "ve": (tr("Distribution", "Verteilung"), r"e_{d}", VERT_ORANGE, v["z2v"], LEVELS[1]),
            "sp": (tr("Storage", "Speicherung"), r"e_{s}", SPEI_YELLOW, v["z3s"], LEVELS[2]),
        }
        rows = {k: _stage_row(name, sym, colour, 0.95, loss, out, tick_label=True)
                for k, (name, sym, colour, loss, out) in specs.items()}
        floor_heat = lambda rt: ripples([np.array([x, room["y_f"] + 0.02, 0.0]) for x in (-2.7, -2.35, -2.0)],
                                        r_max=0.30, color=P_CYAN, cycles=max(1.0, rt / 1.4))

        def show_row(key, rt):
            row = rows[key]
            self.add(row["readout"])
            return [FadeIn(row["group"]), row["unit"].animate(run_time=rt).set_value(1.0)]

        def hide_row(key):
            return [FadeOut(rows[key]["group"]), FadeOut(rows[key]["readout"])]

        self.play(Create(P["shell"]), run_time=1.5)
        self.play(FadeIn(P["person"]), FadeIn(P["radiator"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(1.2, "person"), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 1.2,
                 during=lambda rt: live(rt, "person"))

        caption = step(caption, "uebergabe", "person")
        self.add(P["z0"].ribbons, P["z2"].ribbons)
        active[:] = [P["z0"], P["z2"]]
        self.play(*show_row("ue", 1.4), *_grow(P, ["z0n"], 1.4), *live(1.4, "rad"), run_time=1.4)
        self.play(Circumscribe(P["radiator"], color=UEBER_RED, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z2n", "z2u"], 2.0), *live(2.0, "rad", "ue"), run_time=2.0)
        hold_for(self, self.NARRATION, "uebergabe", during=lambda rt: live(rt, "rad", "ue"))

        caption = step(caption, "flaeche", "rad", "ue")
        rad_tag = math_label(tr(r"\text{Radiator}\,\cdot\,55\,\mathrm{°C}", r"\text{Heizkörper}\,\cdot\,55\,\mathrm{°C}"), np.array([-4.95, 1.52, 0.0]),
                             size=LABEL_FONT_SIZE, color=UEBER_RED)
        loop = _floor_loop(-2.95, -1.75, room["y_f"] - 0.07)
        loop_tag = math_label(tr(r"\text{Underfloor heating}\,\cdot\,35\,\mathrm{°C}", r"\text{Fußbodenheizung}\,\cdot\,35\,\mathrm{°C}"), np.array([-3.00, 1.10, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_CYAN)
        flow_note = note_line(tr("low flow temperature → better generator efficiency", "niedrige Vorlauftemperatur → bessere Erzeugereffizienz"), color=P_CYAN)
        extra[:] = [floor_heat]
        self.play(FadeIn(rad_tag), run_time=1.0)
        self.play(Create(loop), FadeIn(loop_tag), FadeIn(flow_note), *live(1.6, "rad", "ue"), run_time=1.6)
        hold_for(self, self.NARRATION, "flaeche", during=lambda rt: live(rt, "rad", "ue"))

        caption = step(caption, "verteilung", "rad", "ue")
        extra[:] = []
        active[:] = [P["z2"]]
        self.play(FadeOut(flow_note), FadeOut(rad_tag), FadeOut(loop_tag), FadeOut(loop), *hide_row("ue"),
                  run_time=0.8)
        self.play(*show_row("ve", 1.2), *live(1.2), run_time=1.2)
        self.play(Circumscribe(P["z2"].ribbons, color=VERT_ORANGE, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z2v"], 2.0), *live(2.0, "ve"), run_time=2.0)
        hold_for(self, self.NARRATION, "verteilung", during=lambda rt: live(rt, "ve"))

        caption = step(caption, "huelle", "ve")
        inside_note = note_line(tr("Losses inside the envelope are credited as gains", "Verluste innerhalb der Hülle werden als Gewinn gutgeschrieben"), color=VERT_ORANGE)
        self.play(Create(envelope), FadeIn(shell_tag), FadeIn(cellar_tag), *live(1.6, "ve"), run_time=1.6)
        self.play(FadeIn(inside_note), *live(1.2, "ve", "gain"), run_time=1.2)
        hold_for(self, self.NARRATION, "huelle", during=lambda rt: live(rt, "ve", "gain"))

        caption = step(caption, "speicher", "ve", "gain")
        tank_note = body_text(tr("7 days without draw-off", "7 Tage ohne Zapfung"), font_size=LABEL_FONT_SIZE - 3, color=SPEI_YELLOW).move_to(
            np.array([TANK_X, GROUND_Y - 0.62, 0.0]))
        self.add(P["z3"].ribbons, P["tank"]["fill"])
        active[:] = [P["z3"]]
        self.play(FadeOut(inside_note), FadeOut(envelope), FadeOut(shell_tag), FadeOut(cellar_tag), *hide_row("ve"),
                  run_time=0.8)
        self.play(*show_row("sp", 1.2), FadeIn(P["tank"]["group"]), FadeIn(P["boiler"]), *live(1.2), run_time=1.2)
        self.play(Circumscribe(P["tank"]["group"], color=SPEI_YELLOW, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z3n", "z3u", "z3v", "z3s"], 2.0), *live(2.0, "sp"), run_time=2.0)
        hold_for(self, self.NARRATION, "speicher", during=lambda rt: live(rt, "sp"))

        caption = step(caption, "woche", "sp")
        self.play(FadeIn(tank_note), P["tank"]["level"].animate(run_time=3.0).set_value(0.72), *live(3.0, "sp"),
                  run_time=3.0)
        hold_for(self, self.NARRATION, "woche", during=lambda rt: live(rt, "sp"))

        caption = step(caption, "kennzahl", "sp")
        keep = (subtitle, din, caption)
        self.play(*[FadeOut(m) for m in list(self.mobjects) if all(m is not k for k in keep)], run_time=1.2)
        active[:] = []
        final = {k: _stage_row(name, sym, colour, y, loss, out, x0=-3.2, tick_label=True)
                 for (k, (name, sym, colour, loss, out)), y in zip(specs.items(), (1.5, 0.6, -0.3))}
        for row in final.values():
            row["unit"].set_value(1.0)
            row["group"].update()
            row["readout"].update()
        self.play(*[FadeIn(part) for row in final.values() for part in (row["group"], row["readout"])], run_time=1.6)
        hold_for(self, self.NARRATION, "kennzahl")

        caption = step(caption, "einheit")
        gt_note = note_line(tr("all expenditure factors > 1", "alle Aufwandszahlen > 1"), color=P_PRIMARY)
        eq, eq_box, items = math_panel([
            ("eu", tr(r"e_{ü}\approx 1.04", r"e_{ü}\approx 1{,}04"), UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", tr(r"e_{d}\approx 1.08", r"e_{d}\approx 1{,}08"), VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", tr(r"e_{s}\approx 1.06", r"e_{s}\approx 1{,}06"), SPEI_YELLOW),
        ])
        self.play(Create(eq_box), FadeIn(gt_note),
                  *[stage.transition(f"demand.system-losses.expense.{k}-symbol-into-product", source=final[k]["symbol"], target=items[i])
                    for k, i in (("ue", "eu"), ("ve", "ed"), ("sp", "es"))],
                  FadeIn(VGroup(*[m for m in eq if all(m is not items[i] for i in ("eu", "ed", "es"))])),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "einheit")

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 3 – Erzeugung: Wirkungsgrad oder Arbeitszahl

class Beat3_Erzeugung(Scene):
    NARRATION = [
        ("intro",
         "The last stage is the biggest.",
         "Die letzte Stufe ist die größte."),
        ("unterschied",
         "Here a combustion boiler and a heat pump differ fundamentally.",
         "Hier unterscheiden sich Verbrennungskessel und Wärmepumpe grundlegend."),
        ("kessel",
         "A boiler burns fuel.",
         "Ein Kessel verbrennt Brennstoff."),
        ("abgas",
         "Part of the heat leaves through the flue.",
         "Ein Teil der Wärme geht durch den Schornstein."),
        ("mehr",
         "So more always goes in than comes out.",
         "Es muss also immer mehr hinein als heraus."),
        ("brennwert",
         "A condensing boiler cools the flue gas until its water vapour condenses.",
         "Ein Brennwertkessel kühlt das Abgas, bis Wasserdampf darin kondensiert."),
        ("latent",
         "The latent heat from the fundamentals video becomes usable.",
         "Die latente Wärme aus dem Grundlagen-Video wird nutzbar."),
        ("waermepumpe",
         "A heat pump burns nothing.",
         "Eine Wärmepumpe verbrennt nichts."),
        ("umwelt",
         "It lifts ambient heat.",
         "Sie hebt Umweltwärme an."),
        ("zukauf",
         "So three quarters of what it delivers is not purchased final energy.",
         "Drei Viertel der Lieferung sind daher keine zugekaufte Endenergie."),
        ("jaz",
         "Its seasonal performance factor is the ratio of heat delivered over the year to the electricity that drove it.",
         "Ihre Jahresarbeitszahl ist das Verhältnis von Wärme zu eingesetztem Strom im Jahr."),
        ("zahl",
         "Here that is a hundred to twenty-six.",
         "Hier sind das hundert zu sechsundzwanzig."),
        ("kehrwert",
         "The expense figure of the generator is its inverse: about zero point two six.",
         "Die Aufwandszahl der Erzeugung ist ihr Kehrwert: etwa null Komma zwei sechs."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("Generation: efficiency or coefficient of performance", "Erzeugung: Wirkungsgrad oder Arbeitszahl"))
        din = _din_ref("DIN V 18599-5 · VDI 4650-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        t = {k: ValueTracker(0.0) for k in ("heat", "gas", "loss", "recov", "el", "env", "out_env", "out_el", "split")}
        Y, half, BX, PX, ROW_Y = 0.55, 0.5 * Q_NUTZ * FLOW_UNIT, 3.2, -3.7, -0.85
        at = lambda x, y: np.array([x, y, 0.0])
        boiler = _boiler(at(-3.2, Y), w=0.9, h=1.2)
        pump = _heat_pump(at(3.7, Y), w=1.0, h=1.2)
        floor_line = Line(at(-4.6, Y - 0.6), at(4.6, Y - 0.6), color=P_PRIMARY, stroke_width=2.2)
        floor_hatch = VGroup(*[Line(p := floor_line.point_from_proportion(u), p + at(-0.07, -0.1), color=P_PRIMARY,
                                    stroke_width=1.1) for u in np.linspace(0.02, 0.98, 52)])
        floor = VGroup(floor_line, floor_hatch)
        divider = DashedLine(at(0.0, -1.0), at(0.0, 1.4), color=P_WHITE, stroke_width=1.6, stroke_opacity=0.3)
        stage_tag = body_text(tr("Stage 4 · Generation", "Stufe 4 · Erzeugung"), font_size=BODY_FONT_SIZE, color=ERZ_AMBER).move_to(at(0.0, 1.75))
        boiler_name = body_text(tr("Combustion boiler", "Verbrennungskessel"), font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(at(-3.2, Y - 0.95))
        pump_name = body_text(tr("Heat pump", "Wärmepumpe"), font_size=LABEL_FONT_SIZE, color=AMBIENT_GREEN).move_to(at(3.7, Y - 0.95))
        out_env = lambda: interpolate_color(ManimColor(NUTZ_BLUE), ManimColor(AMBIENT_GREEN), t["split"].get_value())
        out_el = lambda: interpolate_color(ManimColor(NUTZ_BLUE), ManimColor(P_YELLOW), t["split"].get_value())

        gas_in = _Stream([at(-3.1, Y), at(-0.45, Y)], [(t["gas"], ERZ_AMBER, False), (t["loss"], P_RED, False)],
                         start=-half, cap=118, seed=11)
        heat_l = _Stream([at(0.45, Y), at(2.6, Y)], [(t["heat"], NUTZ_BLUE, False)], start=-half, cap=100, seed=12)
        flue = _Stream([at(-0.2, Y + 0.58), at(-0.2, 1.95)], [(t["loss"], P_RED, False)],
                       start=-0.5 * 18 * FLOW_UNIT, cap=18, seed=13)
        recov = _Stream([at(-0.2, 1.62), at(0.35, 1.62), at(0.35, Y + 0.58)], [(t["recov"], P_CYAN, False)],
                        start=-0.5 * 11 * FLOW_UNIT, cap=11, seed=14, radius=0.25)
        env_in = _Stream([at(-2.8, Y - 0.25), at(-0.55, Y - 0.25)], [(t["env"], AMBIENT_GREEN, False)],
                         start=-0.5 * 74 * FLOW_UNIT, cap=74, seed=15)
        el_in = _Stream([at(-2.8, Y + 0.42), at(-0.55, Y + 0.42)], [(t["el"], P_YELLOW, False)],
                        start=-0.5 * 26 * FLOW_UNIT, cap=26, seed=16)
        hp_out = _Stream([at(0.55, Y), at(2.3, Y)], [(t["out_env"], out_env, False), (t["out_el"], out_el, False)],
                         start=-half, cap=100, seed=17)
        active: list = []
        pump_on = [False]

        def live(rt):
            pump.rate = 1.0 if pump_on[0] else 0.0
            return [_flow(active, rt)]

        def step(caption, key):
            return _swap(self, caption, subtitle_text(self.NARRATION, key), live)

        def label(src, pos, color, edge="left"):
            return _readout(src, pos, size=LABEL_FONT_SIZE, color=color, edge=edge)

        gas_lbl = label(lambda: tr(rf"\text{{Natural gas}}\;{label_number(t['gas'].get_value() + t['loss'].get_value())}\,\mathrm{{kWh}}", rf"\text{{Erdgas}}\;{label_number(t['gas'].get_value() + t['loss'].get_value())}\,\mathrm{{kWh}}"),
                        at(-3.1, Y + 0.75), ERZ_AMBER)
        heat_lbl = label(lambda: tr(rf"\text{{Heat}}\;{label_number(t['heat'].get_value())}\,\mathrm{{kWh}}", rf"\text{{Wärme}}\;{label_number(t['heat'].get_value())}\,\mathrm{{kWh}}"),
                         at(2.6, Y + 0.50), NUTZ_BLUE, "right")
        pump_lbl = label(lambda: rf"{label_number(t['out_env'].get_value() + t['out_el'].get_value())}\,\mathrm{{kWh}}",
                         at(2.3, Y + 0.50), NUTZ_BLUE, "right")
        el_lbl = label(lambda: tr(rf"\text{{Electricity}}\;{label_number(t['el'].get_value())}\,\mathrm{{kWh}}", rf"\text{{Strom}}\;{label_number(t['el'].get_value())}\,\mathrm{{kWh}}"),
                       at(-2.8, Y + 0.75), P_YELLOW)
        env_lbl = label(lambda: tr(rf"\text{{Ambient}}\;{label_number(t['env'].get_value())}\,\mathrm{{kWh}}", rf"\text{{Umwelt}}\;{label_number(t['env'].get_value())}\,\mathrm{{kWh}}"),
                        at(-2.8, Y - 0.82), AMBIENT_GREEN)

        gas_row = _ratio_row(tr("Gas boiler", "Gaskessel"), r"e_{g}", ERZ_AMBER, ROW_Y, out=Q_NUTZ,
                             value=lambda: (t["gas"].get_value() + t["loss"].get_value()) / Q_NUTZ,
                             shown=lambda: t["gas"].get_value() > 99.0 and t["heat"].get_value() > 99.0,
                             x0=-3.75, name_w=2.15, length=2.0)
        wp_row = _ratio_row(tr("Heat pump", "Wärmepumpe"), r"e_{g}", AMBIENT_GREEN, ROW_Y, out=Q_NUTZ,
                            value=lambda: t["el"].get_value() / Q_NUTZ,
                            shown=lambda: t["el"].get_value() > 25.0, x0=-3.35, name_w=1.85, length=2.0)
        cond_label = body_text(tr("Condensing boiler", "Brennwertkessel"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            gas_row["label"].get_left(), aligned_edge=LEFT)
        flue_lbl = body_text(tr("Flue gas", "Abgas"), font_size=LABEL_FONT_SIZE - 3, color=P_RED).move_to(at(0.2, 1.95))
        drops = droplets(at(0.08, 1.40), n=4, spread=(0.10, 0.08), color=P_CYAN)

        self.play(FadeIn(stage_tag), FadeIn(floor), FadeIn(boiler), FadeIn(pump), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2)

        pump_on[0] = True
        caption = step(caption, "unterschied")
        self.play(Create(divider), FadeIn(boiler_name), FadeIn(pump_name), *live(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "unterschied", during=live)

        caption = step(caption, "kessel")
        pump_on[0] = False
        self.play(FadeOut(pump), FadeOut(pump_name), FadeOut(divider), FadeOut(stage_tag), FadeOut(boiler_name),
                  boiler.animate.shift(RIGHT * BX), run_time=1.4)
        self.add(gas_in.ribbons, gas_lbl)
        active[:] = [gas_in]
        self.play(t["gas"].animate.set_value(Q_NUTZ), Circumscribe(boiler, color=ERZ_AMBER, buff=0.1, stroke_width=3,
                                                                    fade_out=True), *live(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "kessel", during=live)

        caption = step(caption, "abgas")
        self.add(flue.ribbons)
        active[:] = [gas_in, flue]
        self.play(FadeIn(flue_lbl), run_time=0.8)
        self.play(t["loss"].animate.set_value(18.0), *live(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "abgas", during=live)

        caption = step(caption, "mehr")
        self.add(heat_l.ribbons, heat_lbl, gas_row["readout"])
        active[:] = [gas_in, flue, heat_l]
        self.play(FadeOut(flue_lbl), FadeIn(gas_row["group"]), t["heat"].animate.set_value(Q_NUTZ), *live(2.0),
                  run_time=2.0)
        hold_for(self, self.NARRATION, "mehr", during=live)

        caption = step(caption, "brennwert")
        cond_note = note_line(tr("Condensing: water vapour in the flue gas condenses → latent heat becomes usable", "Brennwert: Wasserdampf im Abgas kondensiert → latente Wärme wird nutzbar"), color=P_CYAN)
        self.play(FadeIn(cond_note), FadeIn(drops), *live(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "brennwert", during=live)

        caption = step(caption, "latent")
        self.add(recov.ribbons)
        active[:] = [gas_in, flue, heat_l, recov]
        self.play(stage.transition("demand.system-losses.generation.gas-label-to-condensing", source=gas_row["label"], target=cond_label), t["loss"].animate.set_value(7.0),
                  t["recov"].animate.set_value(11.0), *live(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "latent", during=live)

        caption = step(caption, "waermepumpe")
        boiler_side = [boiler, cond_label, *[m for m in gas_row["group"] if m is not gas_row["label"]],
                       gas_row["readout"], gas_lbl, heat_lbl, drops, cond_note, gas_in.ribbons, flue.ribbons,
                       heat_l.ribbons, recov.ribbons]
        active[:] = []
        self.play(*[FadeOut(m) for m in boiler_side], pump.animate.shift(RIGHT * PX), run_time=1.6)
        pump_on[0] = True
        hold_for(self, self.NARRATION, "waermepumpe", during=live)

        caption = step(caption, "umwelt")
        self.add(env_in.ribbons, env_lbl)
        active[:] = [env_in]
        self.play(t["env"].animate.set_value(74.0), Circumscribe(pump, color=AMBIENT_GREEN, buff=0.1, stroke_width=3,
                                                                  fade_out=True), *live(2.2), run_time=2.2)
        hold_for(self, self.NARRATION, "umwelt", during=live)

        caption = step(caption, "zukauf")
        bought = body_text(tr("purchased", "zugekauft"), font_size=LABEL_FONT_SIZE - 2, color=P_YELLOW).move_to(at(1.25, 1.50))
        free = body_text(tr("not purchased", "nicht zugekauft"), font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN).move_to(at(1.5, 0.02))
        bought_leader = Line(bought.get_bottom() + DOWN * 0.04, at(1.25, Y + 0.185), color=P_YELLOW, stroke_width=1.4,
                             stroke_opacity=0.6)
        free_leader = Line(at(1.25, free.get_top()[1] + 0.04), at(1.25, Y - 0.065), color=AMBIENT_GREEN,
                           stroke_width=1.4, stroke_opacity=0.6)
        self.add(el_in.ribbons, hp_out.ribbons, el_lbl, pump_lbl)
        active[:] = [env_in, el_in, hp_out]
        self.play(t["el"].animate.set_value(26.0), t["out_env"].animate.set_value(74.0),
                  t["out_el"].animate.set_value(26.0), *live(2.2), run_time=2.2)
        self.play(t["split"].animate.set_value(1.0), FadeIn(bought), FadeIn(free), Create(bought_leader),
                  Create(free_leader), *live(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "zukauf", during=live)

        caption = step(caption, "jaz")
        eq, eq_box, items = math_panel([
            ("jaz", r"\mathrm{JAZ}", AMBIENT_GREEN), (None, "=", P_WHITE),
            ("frac", tr(rf"\frac{{\textcolor{{{NUTZ_BLUE}}}{{\text{{heat out}}}}}}{{\textcolor{{{P_YELLOW}}}{{\text{{power in}}}}}}", rf"\frac{{\textcolor{{{NUTZ_BLUE}}}{{\text{{Wärme ab}}}}}}{{\textcolor{{{P_YELLOW}}}{{\text{{Strom zu}}}}}}"),
             P_WHITE),
            ("num", tr(r"=\frac{100}{26}\approx 3.8", r"=\frac{100}{26}\approx 3{,}8"), P_WHITE),
            ("eg", tr(r"\;\Rightarrow\;e_{g}=\frac{1}{\mathrm{SPF}}", r"\;\Rightarrow\;e_{g}=\frac{1}{\mathrm{JAZ}}"), AMBIENT_GREEN),
        ])
        pump_lbl.clear_updaters()
        el_lbl.clear_updaters()
        sources = VGroup(pump_lbl[0].copy(), el_lbl[0].copy())
        active[:] = [el_in, hp_out]
        self.play(FadeOut(env_in.ribbons), FadeOut(env_lbl), FadeOut(bought), FadeOut(free), FadeOut(bought_leader),
                  FadeOut(free_leader), run_time=0.8)
        self.play(Create(eq_box), FadeIn(items["jaz"]), FadeIn(eq[1]),
                  stage.transition("demand.system-losses.generation.parts-into-spf", source=sources, target=items["frac"]),
                  Circumscribe(hp_out.ribbons, color=NUTZ_BLUE, buff=0.06, stroke_width=3, fade_out=True),
                  Circumscribe(el_in.ribbons, color=P_YELLOW, buff=0.06, stroke_width=3, fade_out=True),
                  *live(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "jaz", during=live)

        caption = step(caption, "zahl")
        self.play(FadeIn(items["num"]), *live(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "zahl", during=live)

        caption = step(caption, "kehrwert")
        self.add(wp_row["readout"])
        self.play(FadeIn(items["eg"]), FadeIn(wp_row["group"]), *live(1.6), run_time=1.6)
        self.play(Circumscribe(wp_row["group"], color=AMBIENT_GREEN, buff=0.08, stroke_width=3, fade_out=True),
                  *live(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "kehrwert", during=live)

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 4 – Die Anlagenaufwandszahl e_p

class Beat4_Anlagenaufwandszahl(Scene):
    NARRATION = [
        ("intro",
         "Multiply the four stages together and the whole plant collapses into a single number.",
         "Multipliziert man die vier Stufen, schrumpft die ganze Anlage auf eine einzige Zahl."),
        ("faktoren",
         "Each of the four stages has its own expense figure.",
         "Jede der vier Stufen hat ihre eigene Aufwandszahl."),
        ("produkt",
         "Multiplied together, the four figures stack up into one bar.",
         "Miteinander multipliziert, stapeln sich die vier Zahlen zu einem Balken."),
        ("kette",
         "That is the plant expense figure.",
         "Das ist die Anlagenaufwandszahl."),
        ("einheit",
         "It tells how much final energy is required per unit of useful heat.",
         "Sie sagt, wie viel Endenergie je Einheit Nutzenergie erforderlich ist."),
        ("gas",
         "For the gas boiler the figure lands at one point four.",
         "Beim Gaskessel liegt sie bei eins Komma vier."),
        ("gas_kwh",
         "A hundred kilowatt-hours of useful heat become about a hundred and forty.",
         "Aus hundert Kilowattstunden Nutzenergie werden etwa hundertvierzig."),
        ("wp",
         "For the heat pump it drops below one.",
         "Bei der Wärmepumpe fällt sie unter eins."),
        ("umwelt",
         "Three quarters of the delivered heat came from the environment.",
         "Drei Viertel der gelieferten Wärme kamen aus der Umwelt."),
        ("brueck",
         "But thirty-one kilowatt-hours of electricity and a hundred and forty of gas are still not comparable.",
         "Doch einunddreißig Kilowattstunden Strom und hundertvierzig Gas sind noch nicht vergleichbar."),
        ("weiter",
         "That is the next chapter.",
         "Das ist das nächste Kapitel."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The system expenditure factor e_p", "Die Anlagenaufwandszahl e_p"))
        din = _din_ref("DIN V 18599-5:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        mix, lift, grow_p = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        active: list = []
        heat_names = ("rad", "person", "ue", "ve", "sp")

        def live(rt):
            return [_flow(active, rt), *[P["heat"](n, rt) for n in heat_names]]

        def step(caption, key, during=None):
            return _swap(self, caption, subtitle_text(self.NARRATION, key), during or (lambda rt: []))

        q_h = _readout(lambda: rf"Q_{{h}}={label_number(v['z0n'].get_value())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                       np.array([ROOM_C[0], 1.42, 0.0]), size=BODY_FONT_SIZE, color=NUTZ_BLUE, edge="center")
        q_e = _readout(lambda: rf"Q_{{E}}={label_number(P['z4'].total())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                       np.array([1.55, -1.22, 0.0]), size=BODY_FONT_SIZE, color=END_RED, edge="left")

        X0 = -4.4
        rows = {
            "ue": _stage_row(tr("Emission", "Übergabe"), r"e_{ü}", UEBER_RED, 1.50, v["z2u"], LEVELS[0], name_w=1.85, x0=X0,
                             tick_label=True),
            "ve": _stage_row(tr("Distribution", "Verteilung"), r"e_{d}", VERT_ORANGE, 0.95, v["z2v"], LEVELS[1], name_w=1.85, x0=X0),
            "sp": _stage_row(tr("Storage", "Speicherung"), r"e_{s}", SPEI_YELLOW, 0.40, v["z3s"], LEVELS[2], name_w=1.85, x0=X0),
        }
        eg_unit = ValueTracker(0.0)

        def eg_value():
            m = mix.get_value()
            return ((1 - m) * (eg_unit.get_value() + v["z4e"].get_value() / LEVELS[3])
                    + m * v["z4w"].get_value() / LEVELS[3])

        rows["eg"] = _ratio_row(tr("Generation", "Erzeugung"), r"e_{g}", ERZ_AMBER, -0.15, out=LEVELS[3], value=eg_value, name_w=1.85,
                                x0=X0, shown=lambda: eg_unit.get_value() > 0.999 and (v["z4e"].get_value() > 0.05
                                                                                      or v["z4w"].get_value() > 0.05))
        keys = ("ue", "ve", "sp", "eg")
        colours = (UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER)
        units = {"ue": rows["ue"]["unit"], "ve": rows["ve"]["unit"], "sp": rows["sp"]["unit"], "eg": eg_unit}
        product = _product_row(
            tr("System", "Anlage"), r"e_{p}", -0.90, name_w=1.85, x0=X0, factors=
            [((lambda f=rows[k]["value"]: 1.0 + grow_p.get_value() * (f() - 1.0)), c) for k, c in zip(keys, colours)],
            lift=lambda: lift.get_value())

        self.play(Create(P["shell"]), run_time=1.2)
        self.add(P["z0"].ribbons, P["z2"].ribbons, P["z3"].ribbons, P["z4"].ribbons, P["flue"].ribbons,
                 P["tank"]["fill"], q_h, q_e)
        active.extend([P["z0"], P["z2"], P["z3"], P["z4"], P["flue"]])
        self.play(FadeIn(P["person"]), FadeIn(P["radiator"]), FadeIn(P["tank"]["group"]), FadeIn(P["boiler"]),
                  FadeIn(P["meter"]), FadeIn(P["chimney"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(0.8), run_time=0.8)
        self.play(*_grow(P, ["z0n", "z2n", "z2u", "z2v", "z3n", "z3u", "z3v", "z3s", "z4n", "z4u", "z4v", "z4s",
                             "z4e", "flue"], 2.4), *live(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 0.8 + 2.4, during=live)

        caption = step(caption, "faktoren", live)
        keep = (subtitle, din, caption)
        self.play(*[FadeOut(m) for m in list(self.mobjects) if all(m is not k for k in keep)], run_time=1.2)
        active[:] = []
        for k in keys:
            self.add(rows[k]["readout"])
            self.play(FadeIn(rows[k]["group"]), units[k].animate(run_time=1.4).set_value(1.0), run_time=1.4)
        hold_for(self, self.NARRATION, "faktoren")

        caption = step(caption, "produkt")
        schematic_note = note_line(tr("Schematic view — DIN/TS 18599-5 calculates in far more detail", "Schematische Darstellung — DIN/TS 18599-5 rechnet deutlich detaillierter"),
                                   color=P_PRIMARY)
        self.play(FadeIn(product["group"]), FadeIn(schematic_note), lift.animate(run_time=1.4).set_value(1.0),
                  run_time=1.4)
        self.add(product["readout"])
        self.play(grow_p.animate(run_time=2.4).set_value(1.0), run_time=2.4)
        hold_for(self, self.NARRATION, "produkt")

        caption = step(caption, "kette")
        eq, eq_box, items = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, "=", P_WHITE),
            ("eu", r"e_{ü}", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"e_{d}", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"e_{s}", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"e_{g}", ERZ_AMBER),
        ])
        self.play(Create(eq_box),
                  stage.transition("demand.system-losses.chain.product-symbol-into-ep", source=product["symbol"], target=items["ep"]),
                  *[stage.transition(f"demand.system-losses.chain.{k}-symbol-into-term", source=rows[k]["symbol"], target=items[i])
                    for k, i in zip(keys, ("eu", "ed", "es", "eg"))],
                  FadeIn(VGroup(*[m for m in eq if all(m is not items[i] for i in ("ep", "eu", "ed", "es", "eg"))])),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "kette")

        caption = step(caption, "einheit")
        self.play(Indicate(product["readout"], color=P_WHITE), Circumscribe(product["group"], color=P_WHITE, buff=0.08,
                                                                             stroke_width=3, fade_out=True),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "einheit")

        caption = step(caption, "gas")
        gas_label = body_text(tr("Gas boiler", "Gaskessel"), font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(
            rows["eg"]["label"].get_left(), aligned_edge=LEFT)
        eq2, eq2_box, items2 = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, r"\approx", P_WHITE),
            ("eu", tr(r"1.04", r"1{,}04"), UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", tr(r"1.08", r"1{,}08"), VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", tr(r"1.06", r"1{,}06"), SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", tr(r"1.18", r"1{,}18"), ERZ_AMBER), (None, tr(r"\approx 1.40", r"\approx 1{,}40"), P_WHITE),
        ])
        self.play(FadeOut(schematic_note), stage.transition("demand.system-losses.chain.generator-label-to-gas-boiler", source=rows["eg"]["label"], target=gas_label),
                  stage.transition("demand.system-losses.chain.formula-box-to-gas-numbers", source=eq_box, target=eq2_box),
                  stage.transition("demand.system-losses.chain.formula-to-gas-numbers", source=eq, target=eq2), run_time=1.8)
        hold_for(self, self.NARRATION, "gas")

        caption = step(caption, "gas_kwh")
        unit_tex = r"\,\mathrm{kWh/(m^{2}\cdot a)}"
        out_h = math_label(rf"Q_{{h}}={label_number(Q_NUTZ)}{unit_tex}", np.array([5.1, 1.5, 0.0]), size=LABEL_FONT_SIZE,
                           color=NUTZ_BLUE)
        out_e = math_label(rf"Q_{{E}}\approx {label_number(round(Q_E_GAS))}{unit_tex}", np.array([5.1, 0.3, 0.0]),
                           size=LABEL_FONT_SIZE, color=END_RED)
        out_arrow = Arrow(np.array([4.2, 1.2, 0.0]), np.array([4.2, 0.6, 0.0]), buff=0, color=P_WHITE, stroke_width=3,
                          max_tip_length_to_length_ratio=0.3)
        out_x = math_label(tr(r"\cdot\,e_{p}\approx 1.40", r"\cdot\,e_{p}\approx 1{,}40"), np.array([4.5, 0.9, 0.0]), size=LABEL_FONT_SIZE,
                           color=P_WHITE, edge="left")
        self.play(FadeIn(out_h), run_time=1.0)
        self.play(Create(out_arrow), FadeIn(out_x), run_time=1.2)
        self.play(FadeIn(out_e), run_time=1.2)
        hold_for(self, self.NARRATION, "gas_kwh")

        caption = step(caption, "wp")
        pump_label = body_text(tr("Heat pump", "Wärmepumpe"), font_size=LABEL_FONT_SIZE, color=AMBIENT_GREEN).move_to(
            gas_label.get_left(), aligned_edge=LEFT)
        eq3, eq3_box, items3 = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, r"\approx", P_WHITE),
            ("eu", tr(r"1.04", r"1{,}04"), UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", tr(r"1.08", r"1{,}08"), VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", tr(r"1.06", r"1{,}06"), SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", tr(r"0.26", r"0{,}26"), AMBIENT_GREEN), (None, tr(r"\approx 0.31", r"\approx 0{,}31"), P_WHITE),
        ])
        out_e_wp = math_label(rf"Q_{{E}}\approx {label_number(round(Q_E_WP))}{unit_tex}", np.array([5.1, 0.3, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_YELLOW)
        out_x_wp = math_label(tr(r"\cdot\,e_{p}\approx 0.31", r"\cdot\,e_{p}\approx 0{,}31"), np.array([4.5, 0.9, 0.0]), size=LABEL_FONT_SIZE,
                              color=P_WHITE, edge="left")
        self.play(
            stage.transition("demand.system-losses.chain.gas-label-to-heat-pump-label", source=gas_label, target=pump_label),
            *[v[n].animate(run_time=3.0).set_value(0.0) for n in ("z4n", "z4u", "z4v", "z4s", "z4e", "flue")],
            v["z4w"].animate(run_time=3.0).set_value(WP_STROM), v["env"].animate(run_time=3.0).set_value(WP_UMWELT),
            mix.animate(run_time=3.0).set_value(1.0),
            stage.transition("demand.system-losses.chain.formula-box-to-heat-pump", source=eq2_box, target=eq3_box),
            stage.transition("demand.system-losses.chain.formula-to-heat-pump-numbers", source=eq2, target=eq3),
            stage.transition("demand.system-losses.chain.final-energy-gas-to-heat-pump", source=out_e, target=out_e_wp),
            stage.transition("demand.system-losses.chain.primary-factor-gas-to-heat-pump", source=out_x, target=out_x_wp), run_time=3.0)
        hold_for(self, self.NARRATION, "wp")

        caption = step(caption, "umwelt")
        why = note_line(tr("e_p < 1 only because ambient heat comes in without purchased final energy", "e_p < 1 nur, weil Umweltwärme ohne zugekaufte Endenergie dazukommt"), color=AMBIENT_GREEN)
        gen = stacked_bar([(tr("Ambient", "Umwelt"), WP_UMWELT, AMBIENT_GREEN), (tr("Electricity", "Strom"), WP_STROM, P_YELLOW)],
                          base=np.array([6.0, -1.0, 0.0]), width=0.8, unit=0.0165)
        env_tag = body_text(tr(f"Ambient  {round(WP_UMWELT)} kWh", f"Umwelt  {round(WP_UMWELT)} kWh"), font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN
                            ).move_to(np.array([5.4, gen["anchors"][0][1], 0.0]), aligned_edge=RIGHT)
        el_tag = body_text(tr(f"Electricity  {round(WP_STROM)} kWh", f"Strom  {round(WP_STROM)} kWh"), font_size=LABEL_FONT_SIZE - 2, color=P_YELLOW
                           ).move_to(np.array([5.4, gen["anchors"][1][1], 0.0]), aligned_edge=RIGHT)
        self.play(*[FadeOut(m) for m in (out_h, out_arrow, out_x_wp, out_e_wp)], run_time=0.8)
        self.play(FadeIn(why), GrowFromEdge(gen["bars"][0], DOWN), FadeIn(env_tag), run_time=1.8)
        self.play(GrowFromEdge(gen["bars"][1], DOWN), FadeIn(el_tag), run_time=1.4)
        hold_for(self, self.NARRATION, "umwelt")

        caption = step(caption, "brueck")
        bridge, bridge_box, bitems = math_panel([
            ("qe", r"Q_{E}", END_RED), (None, "=", P_WHITE),
            ("qh", r"Q_{h}", NUTZ_BLUE), (None, r"\cdot", P_WHITE),
            ("ep", r"e_{p}", P_WHITE),
        ])
        gx, px = -2.4, 2.4
        gas_meter = fitted_staffage("shared-gas-meter-run", width=0.8, height=0.9, rate=1.0, loop=True).move_to(np.array([gx, 1.2, 0.0]))
        power_meter = fitted_staffage("shared-electricity-meter-run", width=0.8, height=0.9, rate=1.0, loop=True).move_to(np.array([px, 1.2, 0.0]))
        gas_val = math_label(rf"{label_number(round(Q_E_GAS))}{unit_tex}", np.array([gx, 0.4, 0.0]), size=BODY_FONT_SIZE,
                             color=END_RED)
        power_val = math_label(rf"{label_number(round(Q_E_WP))}{unit_tex}", np.array([px, 0.4, 0.0]), size=BODY_FONT_SIZE,
                               color=P_YELLOW)
        not_equal = VGroup(*[Line(np.array([-0.16, 1.2 + dy, 0.0]), np.array([0.16, 1.2 + dy, 0.0]), color=P_WHITE,
                                  stroke_width=3) for dy in (0.07, -0.07)],
                           Line(np.array([-0.1, 1.0, 0.0]), np.array([0.1, 1.4, 0.0]), color=P_WHITE, stroke_width=3))
        gas_name = body_text("Gas", font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(np.array([gx, 1.85, 0.0]))
        power_name = body_text(tr("Electricity", "Strom"), font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([px, 1.85, 0.0]))
        gone = [*[rows[k]["group"] for k in keys], *[rows[k]["readout"] for k in keys], product["group"],
                product["readout"], pump_label, why, gen["bars"], env_tag,
                el_tag]
        self.play(*[FadeOut(m) for m in gone], run_time=1.2)
        self.play(stage.transition("demand.system-losses.chain.formula-box-to-bridge", source=eq3_box, target=bridge_box),
                  stage.transition("demand.system-losses.chain.formula-to-bridge", source=eq3, target=bridge),
                  FadeIn(gas_meter), FadeIn(gas_name), FadeIn(gas_val), FadeIn(power_meter), FadeIn(power_name),
                  FadeIn(power_val), FadeIn(not_equal), run_time=1.8)
        hold_for(self, self.NARRATION, "brueck")

        caption = step(caption, "weiter")
        bridge_note = note_line(tr("1 kWh electricity ≠ 1 kWh gas", "1 kWh Strom ≠ 1 kWh Gas"), color=P_PRIMARY)
        self.play(FadeIn(bridge_note), run_time=1.0)
        hold_for(self, self.NARRATION, "weiter")

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion
