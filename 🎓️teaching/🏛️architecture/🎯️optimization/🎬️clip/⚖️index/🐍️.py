"""🎬 Multiple Criteria and a Normalized Index: five façade variants, physical rulers, weights and a ranked index."""

import importlib
import math
import re
import sys
from functools import lru_cache
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
mn = importlib.import_module("manim")

INK, MUTED, GOOD, WARN, BAD = _art.P_WHITE, _art.PASTEL_GREY, _art.P_GREEN, _art.P_ORANGE, _art.P_PRIMARY
COLORS = {"electricity": _art.PASTEL_PURPLE, "cost": _art.P_ORANGE, "daylight": _art.P_YELLOW, "carbon": _art.PASTEL_GREY}
GLYPHS = {"electricity": "bulb", "daylight": "sun", "carbon": "factory"}
ORDER = [item["id"] for item in _model.criteria()]
CRITERIA = {item["id"]: item for item in _model.criteria()}
DIGITS = {"electricity": 2, "cost": 0, "daylight": 2, "carbon": 0}
SHORT = {"electricity": ("Electricity", "Strombedarf"), "cost": ("Cost", "Kosten"), "daylight": ("Daylight", "Tageslicht"), "carbon": ("Carbon", "Graue Emissionen")}
TITLES = {"electricity": ("Electricity intensity", "Strombedarf pro m²")}
S0, S1, S2 = 15, _fonts.LABEL_FONT_SIZE - 2, _fonts.BODY_FONT_SIZE
FLOOR_AREA = 48
BUDGET_CHF = _model.brief()["budget_chf"]
ROOM_SCALE, ROOM_LEFT, FLOOR_Y = 0.72, 2.0, -0.7
SECTION_POINT = (ROOM_LEFT + 3 * ROOM_SCALE, FLOOR_Y + 1.5 * ROOM_SCALE, 0)
LEFT, RIGHT = -6.95, -0.2
FRAME_X0, FRAME_X1 = -7.08, 0.1
WIDTH, DEPTH, HEIGHT, SLAB, MASONRY, HEAD = _room.WIDTH, _room.DEPTH, _room.HEIGHT, _room.SLAB, _room.MASONRY, _room.HEAD
LAYER = {"fixed": MUTED, "wall": INK, "insulation": _room.INSULATION, "window": _room.GLASS, "overhang": WARN}
VARIANT_COLOR = {"glass_shaded": _art.P_CYAN, "ribbon": _art.P_GREEN, "punched": _art.PASTEL_PURPLE, "punched_insulated": _art.P_YELLOW, "glass_bare": _art.P_ORANGE, "glass_premium": _art.P_RED}
ENERGY = (("heating", _room.HEAT), ("cooling", _room.COOL), ("lighting", _room.LIGHT))


def source_files() -> tuple[Path, ...]:
    """🧾 Include this scene in the shared render provenance."""
    return (Path(__file__),)


def P(x, y):
    """📍 Screen point of an interior coordinate (metres across the room depth and above the floor) of the room panel."""
    return np.array([ROOM_LEFT + ROOM_SCALE * x, FLOOR_Y + ROOM_SCALE * y, 0.0])


@lru_cache(maxsize=None)
def variants(extra=False):
    """🏘️ The five (or six) compared façade variants of the model."""
    return tuple(_model.variants(extra=extra))


def by_id(identifier):
    """🔎 One variant (including the extra one) by its id."""
    return next(item for item in variants(True) if item["id"] == identifier)


def ids(extra=False):
    """🔖 Ids of the compared variants in model order."""
    return [item["id"] for item in variants(extra)]


@lru_cache(maxsize=None)
def table(weights, extra=False, pool=None):
    """📋 Model index rows by variant id for hashable weights (a tuple in criterion order) and an optional tuple of compared ids."""
    return {row["id"]: row for row in _model.index(dict(zip(ORDER, weights)), list(pool) if pool else ids(extra))}


def raw(identifier, criterion):
    """📏 The model's raw value of one variant and criterion."""
    return by_id(identifier)[CRITERIA[criterion]["key"]]


def best_id(criterion, pool):
    """🏅 The variant with the best raw value of a criterion in a pool."""
    sign = 1 if CRITERIA[criterion]["direction"] == "min" else -1
    return min(pool, key=lambda i: sign * raw(i, criterion))


def winner_of(weights, extra=False, pool=None):
    """🥇 The id with the lowest index under weights."""
    return min(table(weights, extra, pool).values(), key=lambda row: row["score"])["id"]


def text(value, point=(0, 0, 0), *, size=S1, color=INK, width=None):
    """🔤 Static label in the canonical face, shrunk to a width when needed."""
    label = _fonts.body_text(str(value).replace("kWh/(m²·a)", _room.INTENSITY_UNIT), font_size=size, color=color)
    if width is not None and label.width > width:
        label.scale_to_fit_width(width)
    return label.move_to(point)


def lines(value, limit=15):
    """✂️ Break a long name into two balanced lines, at spaces or after hyphens."""
    if len(value) <= limit:
        return value
    tokens = re.findall(r"[^\s-]+-?", value)
    if len(tokens) < 2:
        return value

    def join(parts):
        return "".join(part + ("" if part.endswith("-") else " ") for part in parts).strip()
    cut = min(range(1, len(tokens)), key=lambda k: max(len(join(tokens[:k])), len(join(tokens[k:]))))
    return join(tokens[:cut]) + "\n" + join(tokens[cut:])


def stack(value):
    """📚 A name with one word per line (hyphenated words break after the hyphen)."""
    return re.findall(r"[^\s-]+-?", value)


def block(value, point=(0, 0, 0), *, size=S0, color=INK, width=None):
    """📝 One or two centred lines of text."""
    group = _fonts.centered_body_text(value, font_size=size, color=color)
    if width is not None and group.width > width:
        group.scale_to_fit_width(width)
    return group.move_to(point)


def glyph(criterion, point=(0, 0, 0), width=0.34, color=None):
    """🔣 The criterion's pictogram: bulb, coin, sun or factory."""
    color = color or COLORS[criterion]
    if criterion == "cost":
        return mn.VGroup(mn.Circle(radius=width / 2, color=color, stroke_width=2.2, fill_color=color, fill_opacity=.18),
                         mn.Circle(radius=width / 3.4, color=color, stroke_width=1.4)).move_to(point)
    return _art.object_silhouette(GLYPHS[criterion], color=color, width=width).move_to(point)


def arrow(criterion, point=(0, 0, 0), length=0.3):
    """⬇️ Direction arrow: down where less is better, up where more is better."""
    sign = 1 if CRITERIA[criterion]["direction"] == "max" else -1
    start = np.array(point, dtype=float) - np.array([0, sign * length / 2, 0])
    return mn.Arrow(start, start + np.array([0, sign * length, 0]), buff=0, color=COLORS[criterion], stroke_width=3,
                    tip_length=.11, max_tip_length_to_length_ratio=.5)


def live(builder, key=None):
    """🔁 The toolkit's animation-safe live group: rebuilt every frame, or only when ``key()`` changes; copies made by animations are static."""
    return _room.LiveGroup(builder, key)


def nice_step(span, target):
    """📶 A round tick step giving about ``target`` ticks over a span."""
    raw_step = span / target
    base = 10 ** math.floor(math.log10(raw_step))
    return next(m * base for m in (1, 2, 2.5, 5, 10) if m * base >= raw_step)


def lerp(a, b, t):
    """➗ Linear interpolation."""
    return a + (b - a) * t


def reproduce(function, inputs, target, out_digits, limit=3):
    """🧮 Fewest decimals of the printed inputs for which the printed result reproduces; ``≈`` when no precision does."""
    for digits in range(limit + 1):
        if round(function(*[round(value, digits) for value in inputs]), out_digits) == round(target, out_digits):
            return digits, "="
    return limit, "≈"


def energy_digits(found):
    """⚡ Decimals of the electricity parts for which parts add up to the total and total ÷ floor area reproduces the printed intensity."""
    for digits in (0, 1, 2):
        shown = _room.apportion([found[key] for key, _ in ENERGY], digits)
        if round(round(sum(shown), digits) / FLOOR_AREA, 2) == round(found["intensity"], 2):
            return digits
    return 2


class Retire(mn.Animation):
    """🧊 Fade a live group out through a static twin made when the animation starts, so shared trackers and rebuilds never race the fade."""

    def __init__(self, group, **kwargs):
        super().__init__(group, **kwargs)
        self.scene = self.twin = self.inner = None

    def _setup_scene(self, scene):
        self.scene = scene

    def begin(self):
        self.twin = self.mobject.copy()
        self.scene.remove(self.mobject)
        self.scene.add(self.twin)
        self.inner = mn.FadeOut(self.twin, run_time=self.run_time)
        self.inner._setup_scene(self.scene)
        self.inner.begin()
        self.starting_mobject = self.inner.starting_mobject

    def interpolate(self, alpha):
        self.inner.interpolate(alpha)

    def finish(self):
        self.inner.finish()

    def clean_up_from_scene(self, scene):
        self.inner.clean_up_from_scene(scene)
        scene.remove(self.twin, self.mobject)

    def get_all_mobjects(self):
        return self.inner.get_all_mobjects()

    def update_mobjects(self, dt):
        self.inner.update_mobjects(dt)


def runs_of(items):
    """🏃 Maximal runs of equal consecutive items as (item, first index, last index)."""
    runs, start = [], 0
    for k in range(1, len(items) + 1):
        if k == len(items) or items[k] != items[start]:
            runs.append((items[start], start, k - 1))
            start = k
    return runs


def sorted_index(pool, values, identifier):
    """🔢 Position of an id when the pool is sorted by value (ties by pool order)."""
    ordered = sorted(pool, key=lambda i: (values[i], list(pool).index(i)))
    return ordered.index(identifier)


class Ruler:
    """📏 A criterion ruler: dots at the real values, thumbnails in slots, driven by offset, scale and mirror trackers."""

    def __init__(self, scene, criterion, pool, compared, *, x0=-6.6, length=6.0, line_y=.85, state="physical", name_size=S0, rings=True):
        self.name_size, self.rings = name_size, rings
        self.scene, self.criterion, self.pool, self.compared = scene, criterion, list(pool), list(compared)
        self.x0, self.length, self.line_y, self.center = x0, length, line_y, x0 + length / 2
        self.color = COLORS[criterion]
        self.digits_tag = 3 if criterion == "electricity" else DIGITS[criterion]
        self.sign = 1 if CRITERIA[criterion]["direction"] == "min" else -1
        self.values = {i: raw(i, criterion) for i in self.pool}
        inside = [self.values[i] for i in self.compared]
        self.low, self.high = min(inside), max(inside)
        self.best = self.low if self.sign > 0 else self.high
        every = list(self.values.values())
        pad = (max(every) - min(every)) * .06
        self.step_abs = nice_step(max(every) - min(every) + 2 * pad, 6)
        self.digits_abs = 0 if self.step_abs >= 1 else (1 if self.step_abs >= .1 else 2)
        self.lo = math.floor((min(every) - pad) / self.step_abs) * self.step_abs
        self.hi = math.ceil((max(every) + pad) / self.step_abs) * self.step_abs
        self.k0 = length / (self.hi - self.lo)
        self.step_off = nice_step(self.high - self.low, 4)
        self.digits_off = 0 if self.step_off >= 1 else (1 if self.step_off >= .1 else 2)
        self.xb = mn.ValueTracker(x0 + (self.best - self.lo) * self.k0)
        self.g = mn.ValueTracker(self.k0)
        self.sigma = mn.ValueTracker(1.0)
        self.o_abs, self.o_off, self.o_norm, self.o_val, self.o_ring = (mn.ValueTracker(0.0) for _ in range(5))
        self.focus = mn.ValueTracker(0.0)
        gap = (length - 1.5) / max(len(self.pool) - 1, 1)
        order = sorted(self.pool, key=lambda i: (self.values[i], self.pool.index(i)))
        self.gap = gap
        self.slot_x = {i: x0 + .75 + n * gap for n, i in enumerate(order)}
        self.scan = sorted(self.compared, key=lambda i: (self.sign * self.values[i], self.pool.index(i)))
        norm = {row["id"]: row["normalized"][criterion] for row in _model.index(dict(zip(ORDER, (.25,) * 4)), self.compared)}
        self.norm = norm
        self.members, self.ticks = {}, mn.VGroup()
        self.line = mn.Line([x0 - .15, line_y, 0], [x0 + length + .15, line_y, 0], color=INK, stroke_width=2.4)
        self.ring = mn.Rectangle(width=.95, height=.78, color=INK, stroke_width=2.4)
        self.ring.add_updater(lambda m: self._ring_update(m))
        self._ticks()
        for identifier in self.pool:
            self.members[identifier] = self._member(identifier)
        if state == "normalized":
            self.g.set_value(length / (self.high - self.low))
            self.xb.set_value(x0 if self.sign > 0 else x0 + length)
            self.sigma.set_value(self.sign)
            self.o_norm.set_value(1.0)
            self.o_val.set_value(1.0)

    def x_of(self, value):
        """📍 Screen x of a physical value under the live offset, scale and mirror."""
        pre = self.xb.get_value() + (value - self.best) * self.g.get_value()
        return self.center + self.sigma.get_value() * (pre - self.center)

    def fraction(self, identifier):
        """🔢 Position of a variant along the ruler frame, 0 at the left end and 1 at the right."""
        return (self.x_of(self.values[identifier]) - self.x0) / self.length

    def clamp(self, x):
        """📎 Keep a dot near the ruler frame; off-scale values wait just beyond the right end."""
        return min(max(x, self.x0 - .2), self.x0 + self.length + .3)

    def _slot_x(self, identifier):
        return self.center + self.sigma.get_value() * (self.slot_x[identifier] - self.center)

    def _ticks(self):
        s = self.scene
        specs, value = [], self.lo
        while value <= self.hi + 1e-9:
            specs.append(("abs", value, s.number(value, self.digits_abs)))
            value += self.step_abs
        offset = 0.0
        while offset <= (self.high - self.low) * 1.7 + 1e-9:
            label = "0" if offset == 0 else ("+" if self.sign > 0 else "−") + s.number(offset, self.digits_off) + (" " + CRITERIA[self.criterion]["unit"] if abs(offset - self.step_off) < 1e-9 else "")
            specs.append(("off", offset, label))
            offset += self.step_off
        for fraction in (0, .25, .5, .75, 1):
            specs.append(("norm", fraction, {0: "0", 1: "1"}.get(fraction, s.number(fraction, 2))))
        for kind, value, label in specs:
            tick = mn.VGroup(mn.Line([0, self.line_y - .07, 0], [0, self.line_y + .07, 0], color=INK, stroke_width=2),
                             text(label, [0, self.line_y - .3, 0], size=S0, color=INK))
            tick.kind, tick.value = kind, value
            tick.add_updater(lambda m: self._tick_update(m))
            self.ticks.add(tick)

    def _tick_update(self, tick):
        if tick.kind == "abs":
            x, level = self.x_of(tick.value), self.o_abs.get_value()
        elif tick.kind == "off":
            pre = self.xb.get_value() + self.sign * tick.value * self.g.get_value()
            x, level = self.center + self.sigma.get_value() * (pre - self.center), self.o_off.get_value()
        else:
            x, level = self.x0 + tick.value * self.length, self.o_norm.get_value()
        tick.move_to([x, tick.get_center()[1], 0])
        tick.set_opacity(level if self.x0 - .03 <= x <= self.x0 + self.length + .03 else 0.0)

    def _member(self, identifier):
        s = self.scene
        item = by_id(identifier)
        norm = self.norm.get(identifier)
        ring = (GOOD if norm == 0 else (WARN if norm == 1 else None)) if self.rings else None
        shade = (GOOD if norm == 0 else (WARN if norm == 1 else INK)) if self.rings else INK
        row = sorted_index(self.pool, self.values, identifier) % 2
        ly = self.line_y
        thumb = s.thumbnail(item["design"], (0, ly + .85, 0), .15, ramp=s._ramp, value=s._ramp.low, ring=ring)
        name = block(lines(item["title"][s.language()], 14), (0, ly + 1.5 + .5 * row, 0), size=self.name_size, color=shade, width=1.9)
        parts = [thumb, name]
        if row:
            parts.append(mn.Line([0, ly + 1.18, 0], [0, ly + 1.62, 0], color=MUTED, stroke_width=1.2, stroke_opacity=.6))
        slot = mn.VGroup(*parts)
        slot.identifier = identifier
        slot.add_updater(lambda m: m.move_to([self._slot_x(m.identifier), m.get_center()[1], 0]))
        dot = mn.Dot(radius=.075, color=self.color)
        dot.identifier = identifier
        dot.add_updater(lambda m: m.move_to([self.clamp(self.x_of(self.values[m.identifier])), self.line_y, 0]))
        leader = mn.Line([0, 0, 0], [1, 0, 0], color=self.color, stroke_width=1.6, stroke_opacity=.7)
        leader.identifier = identifier
        leader.add_updater(lambda m: m.put_start_and_end_on([self._slot_x(m.identifier), self.line_y + .5, 0],
                                                              [self.clamp(self.x_of(self.values[m.identifier])), self.line_y + .08, 0]))
        tag = text(s.number(self.values[identifier], self.digits_tag), (0, 0, 0), size=S0, color=shade)
        tag.identifier, tag.row = identifier, row
        tag.add_updater(lambda m: (m.move_to([self.clamp(self.x_of(self.values[m.identifier])), self.line_y - .62 - .3 * m.row, 0]),
                                   m.set_opacity(self.o_val.get_value() * (1 - self.o_norm.get_value()))))
        number = live(lambda: self._norm_tag(identifier))
        for part in (slot, dot, leader, tag):
            part.update()
        return {"slot": slot, "dot": dot, "leader": leader, "tag": tag, "norm": number}

    def _norm_tag(self, identifier):
        fraction = min(max(self.fraction(identifier), 0), 9.99)
        label = _room.text([_room.Number(self.scene.number(fraction, 2))], size=S1, color=INK)
        row = sorted_index(self.pool, self.values, identifier) % 2
        label.move_to([self.clamp(self.x_of(self.values[identifier])), self.line_y - .62 - .3 * row, 0])
        label.set_opacity(self.o_norm.get_value())
        return mn.VGroup(label)

    def _ring_update(self, ring):
        position = min(max(self.focus.get_value(), 0), len(self.scan) - 1)
        low, high = int(math.floor(position)), int(math.ceil(position))
        a, b = self.members[self.scan[low]]["slot"][0].get_center(), self.members[self.scan[high]]["slot"][0].get_center()
        ring.move_to(a + (b - a) * (position - low))
        ring.set_stroke(opacity=self.o_ring.get_value())

    def mobjects(self, only=None):
        """🧩 Every mobject of the ruler (or of the given variants) in one group, ticks and ring included."""
        group = mn.VGroup(self.line, self.ticks) if only is None else mn.VGroup()
        for identifier in (self.compared if only is None else only):
            group.add(*self.members[identifier].values())
        if only is None:
            group.add(self.ring)
        return group

    def current(self):
        """📌 The id the focus ring points at."""
        return self.scan[int(round(min(max(self.focus.get_value(), 0), len(self.scan) - 1)))]

    def tween(self, run_time, **targets):
        """🎚️ Animate named trackers together."""
        return mn.AnimationGroup(*[getattr(self, name).animate(run_time=run_time).set_value(value) for name, value in targets.items()])

    def appear(self, run_time=1.2):
        """✨ Show the physical ruler: ticks, then value tags."""
        return self.tween(run_time, o_abs=1.0, o_val=1.0)

    def pan(self, run_time=2.0):
        """➖ Step one: move the best value to the origin."""
        return self.tween(run_time, xb=self.x0 if self.sign > 0 else self.x0 + self.length, o_abs=0.0, o_off=1.0)

    def stretch(self, run_time=2.0, span=None):
        """➗ Step two: divide by the span so the worst value reaches one."""
        return self.tween(run_time, g=self.length / (span or (self.high - self.low)), o_off=0.0, o_norm=1.0)

    def flip(self, run_time=2.0):
        """🔄 Mirror the ruler so that the best value is on the left."""
        return self.tween(run_time, sigma=-1.0)

    def visit(self, positions, hold=.5, travel=.7, move_room=True):
        """🧭 Move the focus ring (and the room) along scan positions."""
        steps = []
        for position in positions:
            moves = [self.focus.animate(run_time=travel).set_value(position)]
            if move_room:
                moves.append(self.scene.set_focus(self.scan[position], travel))
            steps += [mn.AnimationGroup(*moves), mn.Wait(hold)]
        return mn.Succession(*steps)


@lru_cache(maxsize=None)
def glyph_proto(criterion):
    """🧊 A cached small pictogram to clone from."""
    return glyph(criterion, width=.26)


def head_content(scene, criterion):
    """🏷️ Column head content: pictogram, name (one word per line when long) and unit with its direction arrow, in the criterion colour."""
    name = SHORT[criterion][1 if scene.language() == "de" else 0]
    rows = [_room.text([word], size=13, color=COLORS[criterion]) for word in (name.split() if len(name) > 12 else [name])]
    rows.append(_room.text([*CRITERIA[criterion]["unit"].split(), "~↓" if CRITERIA[criterion]["direction"] == "min" else "~↑"], size=13, color=COLORS[criterion]))
    return mn.VGroup(_room.clone(glyph_proto(criterion)), *rows).arrange(mn.DOWN, buff=.05)


class Board:
    """📊 The weighted table: columns as wide as the weights, cells filled to the normalized value, rows ranked by index."""

    def __init__(self, scene, pool, extra=(), *, y0=1.7, pitch=.76, xl=-4.45, width=4.2, weights=None, order=None, name_size=14):
        self.scene, self.pool, self.extra = scene, list(pool), list(extra)
        self.all = self.pool + self.extra
        self.y0, self.pitch, self.xl, self.width = y0, pitch, xl, width
        start = weights or (.25,) * 4
        self.w = {c: mn.ValueTracker(v) for c, v in zip(ORDER, start)}
        positions = order or self.all
        self.r = {i: mn.ValueTracker(positions.index(i) if i in positions else len(positions)) for i in self.all}
        self.row_o = {i: mn.ValueTracker(1.0 if i in self.pool else 0.0) for i in self.all}
        self.t, self.fill, self.compact, self.numbers, self.scores, self.badges, self.header, self.win, self.old, self.mode, self.cur, self.focus, self.seg, self.seg_o = (mn.ValueTracker(0.0) for _ in range(14))
        self.header.set_value(1.0)
        self.numbers.set_value(1.0)
        self.old_id = None
        self.scan = list(self.pool)
        self.labels = {}
        for identifier in self.all:
            item = by_id(identifier)
            thumb = scene.thumbnail(item["design"], (-6.45, 0, 0), .095, ramp=scene._ramp, value=scene._ramp.low)
            name = block(lines(item["title"][scene.language()], 15), (-5.3, 0, 0), size=name_size, width=1.55)
            label = mn.VGroup(thumb, name)
            label.identifier = identifier
            label.add_updater(lambda m: m.move_to([m.get_center()[0], self.y(m.identifier), 0]))
            label.update()
            self.labels[identifier] = label
        self.live = live(lambda: self._build(), lambda: self._key())

    def _key(self):
        trackers = (*self.w.values(), *self.r.values(), *self.row_o.values(), self.t, self.fill, self.compact, self.numbers, self.scores, self.badges,
                    self.header, self.win, self.old, self.mode, self.cur, self.focus, self.seg, self.seg_o)
        return tuple(tracker.get_value() for tracker in trackers) + (self.old_id,)

    def segments(self, travel=.55, dwell=.35):
        """🔦 Point at the four segments of the focus row one after the other; the room's value strip names the raw value behind each."""
        active = self.scene.active
        steps = [mn.AnimationGroup(self.seg.animate(run_time=.01).set_value(0), active.animate(run_time=.01).set_value(0)),
                 self.seg_o.animate(run_time=.3).set_value(1.0), mn.Wait(dwell)]
        for j in range(1, 4):
            steps += [mn.AnimationGroup(self.seg.animate(run_time=travel).set_value(j), active.animate(run_time=travel).set_value(j)), mn.Wait(dwell)]
        return mn.Succession(*steps, mn.AnimationGroup(self.seg_o.animate(run_time=.3).set_value(0.0), active.animate(run_time=.3).set_value(-1)))

    def cursor(self):
        """🎯 The id the focus row sits on and the screen height of the focus frame; the frame follows its row while rows re-rank."""
        position = min(max(self.focus.get_value(), 0), len(self.scan) - 1)
        low, high = int(math.floor(position)), int(math.ceil(position))
        a, b = self.scan[low], self.scan[high]
        return self.scan[int(round(position))], lerp(self.y(a), self.y(b), position - low)

    def visit(self, positions, travel=.3, move_room=True):
        """🧭 Move the focus row (and the room) along scan positions."""
        steps = []
        for position in positions:
            moves = [self.focus.animate(run_time=travel).set_value(position)]
            if move_room:
                moves.append(self.scene.set_focus(self.scan[position], travel))
            steps.append(mn.AnimationGroup(*moves))
        return mn.Succession(*steps)

    def weights(self):
        """⚖️ The live weights, normalized to sum one, in criterion order."""
        values = [self.w[c].get_value() for c in ORDER]
        total = sum(values) or 1.0
        return tuple(v / total for v in values)

    def y(self, identifier):
        """↕️ Live row height of a variant."""
        return self.y0 - self.r[identifier].get_value() * self.pitch

    def state(self):
        """📋 Live normalized values and scores, blended towards the six-variant normalization by ``t``."""
        key = tuple(round(v, 6) for v in self.weights())
        five, t = table(key, False), self.t.get_value()
        six = table(key, True) if self.extra else None
        out = {}
        for identifier in self.all:
            if identifier in five and (six is None or t <= 0):
                values = [five[identifier]["normalized"][c] for c in ORDER]
            elif identifier in five:
                values = [lerp(five[identifier]["normalized"][c], six[identifier]["normalized"][c], t) for c in ORDER]
            else:
                values = [six[identifier]["normalized"][c] for c in ORDER]
            out[identifier] = {"n": values, "score": sum(w * v for w, v in zip(key, values))}
        return out

    def winner(self):
        """🥇 The id with the lowest live score among visible rows."""
        state = self.state()
        return min((i for i in self.all if self.row_o[i].get_value() > .5), key=lambda i: state[i]["score"])

    def _build(self):
        s, german = self.scene, self.scene.language() == "de"
        group = mn.VGroup()
        ws = self.weights()
        state = self.state()
        W, xl = self.width, self.xl
        lefts = [xl + W * sum(ws[:j]) for j in range(4)]
        compact, fill = self.compact.get_value(), self.fill.get_value()
        header, mode = self.header.get_value(), self.mode.get_value()
        strip_y = self.y0 + .5
        if header > .02:
            for j, c in enumerate(ORDER):
                x = xl + W / 2 + (j - 1.5) * 1.18
                chip = _art.window_chip(head_content(s, c), color=COLORS[c])
                chip.move_to([x, strip_y + .26 + chip.height / 2, 0]).set_opacity(header)
                group.add(chip)
                if mode > .02:
                    split = mn.Rectangle(width=max(ws[j] * W, 1e-3), height=.26, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85 * header * mode)
                    split.move_to([lefts[j] + ws[j] * W / 2, strip_y, 0])
                    link = mn.Line([x, chip.get_bottom()[1], 0], [lefts[j] + ws[j] * W / 2, strip_y + .13, 0], color=COLORS[c], stroke_width=1.4, stroke_opacity=.6 * header * mode)
                    decimals = 1 if any(abs(w * 100 - round(w * 100)) > .05 for w in ws) else 0
                    percent = [_room.Number(s.number(round(ws[j] * 100, decimals), decimals)), "%"]
                    if ws[j] * W < .44:
                        share = _room.place(_room.text(percent, size=14, color=COLORS[c]), (lefts[j] + ws[j] * W + .08, strip_y, 0), "left").set_opacity(header * mode)
                    else:
                        share = _room.text(percent, (lefts[j] + ws[j] * W / 2, strip_y, 0), size=14 if ws[j] * W > .7 else 13, color="#06121a").set_opacity(header * mode)
                    group.add(split, link, share)
            if mode > .02:
                word = s.tr("equal weights", "gleiche Gewichte") if max(ws) - min(ws) < 1e-3 else s.tr("weights", "Gewichte")
                group.add(_room.place(_room.text(word.split(), size=S0, color=MUTED), (xl - .12, strip_y, 0), "right").set_opacity(header * mode))
        numbers = self.numbers.get_value()
        tall, drop = self.pitch - .02 + .12 * numbers, .08 + .08 * numbers
        frame_w, frame_x = FRAME_X1 - FRAME_X0, (FRAME_X1 + FRAME_X0) / 2
        if self.cur.get_value() > .02:
            group.add(mn.Rectangle(width=frame_w, height=tall, color=INK, stroke_width=2.4, stroke_opacity=self.cur.get_value()).move_to([frame_x, self.cursor()[1] - drop, 0]))
        if self.win.get_value() > .02:
            group.add(mn.Rectangle(width=frame_w, height=tall, color=GOOD, stroke_width=3.2, stroke_opacity=self.win.get_value()).move_to([frame_x, self.y0 - drop, 0]))
        if self.old.get_value() > .02 and self.old_id in self.all:
            for edge in (tall / 2, -tall / 2):
                group.add(mn.DashedVMobject(mn.Line([frame_x - frame_w / 2, self.y(self.old_id) - drop + edge, 0], [frame_x + frame_w / 2, self.y(self.old_id) - drop + edge, 0], color=MUTED, stroke_width=2.2,
                                                    stroke_opacity=self.old.get_value()), num_dashes=40))
        if self.seg_o.get_value() > .02:
            ident, j = self.cursor()[0], int(round(min(max(self.seg.get_value(), 0), 3)))
            part = state[ident]["n"]
            start = xl + W * sum(ws[k] * part[k] * fill for k in range(j))
            left, span = lerp(lefts[j], start, compact), max(lerp(ws[j] * W, ws[j] * W * part[j] * fill, compact), .1)
            group.add(mn.Rectangle(width=span + .1, height=.44, color=INK, stroke_width=3.6, stroke_opacity=self.seg_o.get_value()).move_to([left + span / 2, self.y(ident), 0]))
        height = lerp(.34 if len(self.all) <= 5 else .32, .26, numbers)
        for identifier in self.all:
            level = self.row_o[identifier].get_value()
            if level < .02:
                continue
            y = self.y(identifier)
            values = state[identifier]["n"]
            cursor = xl
            for j, c in enumerate(ORDER):
                slot = ws[j] * W
                width = slot * values[j] * fill
                left = lerp(lefts[j], cursor, compact)
                if (1 - compact) * level > .02:
                    group.add(mn.Rectangle(width=max(slot, 1e-3), height=height, color=COLORS[c], stroke_width=1.4, stroke_opacity=.55 * (1 - compact) * level,
                                           fill_opacity=0).move_to([lefts[j] + slot / 2, y, 0]))
                if width > .004:
                    group.add(mn.Rectangle(width=width, height=height, stroke_width=0, fill_color=COLORS[c], fill_opacity=.8 * level).move_to([left + width / 2, y, 0]))
                if (1 - compact) * self.numbers.get_value() * level > .05:
                    shown = (1 - compact) * self.numbers.get_value() * level
                    group.add(_room.text([_room.Number(s.number(values[j], 2))], (lefts[j] + slot / 2, y - .26, 0), size=14, color=INK, width=max(slot + .2, .55)).set_opacity(shown))
                    if slot >= .6:
                        group.add(_room.text([_room.Number(s.number(raw(identifier, c), DIGITS[c]))], (lefts[j] + slot / 2, y - .46, 0), size=13, color=MUTED, width=slot + .2).set_opacity(shown))
                cursor += slot * values[j] * fill
            if self.scores.get_value() * level > .02:
                group.add(_room.place(_room.text([_room.Number(s.number(state[identifier]["score"], 2))], size=S1, color=INK), (xl + W - .1, y, 0), "right")
                          .set_opacity(self.scores.get_value() * level))
            if self.badges.get_value() * level > .02:
                rank = int(round(self.r[identifier].get_value())) + 1
                color = GOOD if rank == 1 else INK
                group.add(mn.Circle(radius=.15, color=color, stroke_width=2, stroke_opacity=self.badges.get_value() * level).move_to([-6.9, y, 0]),
                          _room.text([_room.Number(str(rank))], (-6.9, y, 0), size=S0, color=color).set_opacity(self.badges.get_value() * level))
        return group

    def mobjects(self):
        """🧩 Labels and the live table in one group."""
        return mn.VGroup(*self.labels.values(), self.live)

    def rank_to(self, ranks, run_time=2.0):
        """🔀 Move every row to its rank position."""
        return mn.AnimationGroup(*[self.r[i].animate(run_time=run_time).set_value(rank - 1) for i, rank in ranks.items()])

    def weights_to(self, weights, run_time=2.5):
        """🎚️ Tween the weights together."""
        return mn.AnimationGroup(*[self.w[c].animate(run_time=run_time).set_value(v) for c, v in zip(ORDER, weights)])


class Beat4_NormalizedIndex(_visuals.ClipScene):
    """📚 Present authored bilingual narration with model-bound animation."""

    TITLE = {"en": "Multiple Criteria and a Normalized Index", "de": "Mehrere Kriterien und ein normierter Index"}
    NARRATION = [
        ["variants", "Which of five façade designs suits this seminar room best?", "Welcher von fünf Fassadenentwürfen passt hier am besten?"],
        ["variants_2", "Only window share, overhang and insulation are compared.", "Verglichen werden nur Fensteranteil, Vordach und Dämmung."],
        ["criteria", "Electricity intensity and cost: lower is better.", "Strombedarf pro m² und Fassadenkosten: kleiner ist besser."],
        ["criteria_2", "Daylight factor: higher is better. Embodied carbon: lower is better.", "Tageslichtquotient: mehr ist besser. Graue Emissionen: kleiner ist besser."],
        ["best", "No design is best everywhere.", "Kein Entwurf ist überall der beste."],
        ["units", "Kilowatt-hours per m² and year plus francs? That sum has no unit.", "Kilowattstunden pro m² und Jahr plus Franken? Keine Einheit."],
        ["units_2", "Added anyway, the biggest numbers decide, not the importance.", "Trotzdem addiert, entscheiden die größten Zahlen, nicht die Wichtigkeit."],
        ["units_3", "Cost in kCHF, carbon in tonnes: the ranking changes.", "Kosten in kCHF, Graue Emissionen in Tonnen: Rangfolge ändert sich."],
        ["ruler", "Give each criterion its own ruler, starting with electricity.", "Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strombedarf pro m²."],
        ["ruler_2", "Each design sits at its real value; the room shows what the value means.", "Jeder Entwurf liegt auf seinem Wert; der Raum zeigt dessen Sinn."],
        ["ruler_3", "Sun becomes cooling demand; the chiller divides it by its efficiency.", "Sonne wird zum Kühlbedarf; die Kältemaschine teilt ihn durch EER."],
        ["ruler_4", "In winter the room loses heat; the pump needs less power than heat.", "Im Winter verliert der Raum Wärme; die Wärmepumpe braucht weniger Strom als Wärme."],
        ["ruler_5", "Lamps add the rest, less where daylight helps; the sum is the electricity.", "Lampen kommen dazu, weniger bei Tageslicht; Summe: der Strombedarf."],
        ["shift", "Step one: subtract the best of these five values.", "Schritt eins: Ziehe den besten dieser fünf Werte ab."],
        ["shift_2", "Step two: divide by the span from best to worst.", "Schritt zwei: Teile durch die Spanne vom besten zum schlechtesten Wert."],
        ["mapped", "Zero is the best of these five, one the worst of these five.", "Null ist die beste dieser fünf, eins die schlechteste dieser fünf."],
        ["cost", "Façade cost gets the same ruler: the cheapest façade becomes zero.", "Die Fassadenkosten erhalten dasselbe Lineal: Die günstigste Fassade wird null."],
        ["cost_2", "The price is the sum of its parts: fixed items, wall, insulation, windows and overhang.", "Der Preis ist die Summe seiner Teile: Fixposten, Wand, Dämmung, Fenster und Vordach."],
        ["daylight", "Daylight factor: more is better, so the ruler is mirrored.", "Tageslichtquotient: mehr ist besser, also wird das Lineal gespiegelt."],
        ["carbon", "Embodied carbon is the sum of the façade's materials, per layer.", "Graue Emissionen sind die Summe der Fassadenmaterialien, in Kilogramm je Schicht."],
        ["matrix", "Embodied carbon follows the same rule: its five values now lie between zero and one.", "Graue Emissionen folgen derselben Regel: ihre fünf Werte liegen nun zwischen null und eins."],
        ["weights", "Weights say how much each criterion matters; they add up to 100 %.", "Gewichte sagen, wie wichtig jedes Kriterium ist; zusammen 100 %."],
        ["weights_2", "Here: electricity intensity 40 %, cost 30 %, daylight factor 20 %, embodied carbon 10 %.", "Hier: Strombedarf pro m² 40 %, Kosten 30 %, Tageslichtquotient 20 %, Graue Emissionen 10 %."],
        ["weights_3", "A wider column means a stronger pull on the result.", "Eine breitere Spalte zieht das Ergebnis stärker an."],
        ["index", "Slide the filled parts together: their length is the index.", "Schiebe die gefüllten Teile zusammen: ihre Länge ist der Index."],
        ["index_2", "The shortest bar has the lowest index and ranks first.", "Der kürzeste Balken hat den kleinsten Index und steht vorn."],
        ["index_3", "Weight times normalized value, summed over four criteria: the index.", "Gewicht mal normierter Wert, über vier Kriterien summiert: der Index."],
        ["winner", "Its room: not best everywhere, but best overall.", "Sein Raum: nicht überall der beste, aber insgesamt der beste."],
        ["winner_2", "Strengths elsewhere outweigh its weakest criterion.", "Stärken anderswo gleichen sein schwächstes Kriterium aus."],
        ["sensitivity", "A cost-driven client puts more weight on cost.", "Ein kostenorientierter Bauherr gewichtet die Kosten stärker."],
        ["sensitivity_2", "The bars change and the ranking reorders.", "Die Balken ändern sich, und die Rangfolge ordnet sich neu."],
        ["sensitivity_3", "Another façade wins, though no design changed.", "Eine andere Fassade gewinnt, obwohl sich kein Entwurf geändert hat."],
        ["sweep", "Sweep the cost weight to see where the winner switches.", "Verschiebe das Kostengewicht und sieh, wo der Sieger wechselt."],
        ["sweep_2", "Close calls are fragile: test the ranking before you decide.", "Knappe Entscheidungen sind fragil: Rangfolge vorher prüfen."],
        ["range", "The rulers depend on the designs compared: add a sixth façade.", "Die Lineale hängen von der Vergleichsmenge ab: Füge eine sechste Fassade hinzu."],
        ["range_2", "Premium is dearest and most frugal: costs shrink, electricity values shift a little.", "Premium ist am teuersten und sparsamsten: Kosten schrumpfen, Stromwerte verschieben sich."],
        ["range_3", "The order of the original five can flip, though they did not change.", "Die Reihenfolge der ersten fünf kippt, obwohl sie gleich blieben."],
        ["record", "Record weights, designs and value ranges with every decision.", "Dokumentiere Gewichte, verglichene Entwürfe und Wertebereiche bei jeder Entscheidung."],
        ["limits", "Client budget 21,000 CHF: a mandatory limit, applied before weights.", "Das Budget des Bauherrn beträgt 21.000 CHF: eine zwingende Grenze, vor jeder Gewichtung."],
        ["limits_2", "Within the budget, strengths still offset weaknesses; the limit itself is never traded off.", "Im Budget gleichen Stärken noch Schwächen aus; die Grenze selbst wird nie verrechnet."],
        ["pointer", "To see trade-offs before weighting, look at Pareto fronts.", "Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten."],
        ["next", "For this room at these weights, the winner has the lowest index.", "Für diesen Raum gewinnt bei diesen Gewichten der kleinste Index."]
    ]

    def construct(self):
        """▶️ Present this independent architecture optimization chapter."""
        self.present("index")

    def prepare(self):
        """🧰 Clip-wide colour ramp, weight sets, the shared design trackers and the persistent room panel."""
        every = [_room.result(item["design"])["electricity"] / FLOOR_AREA for item in variants(True)]
        self._ramp = self.ramp(min(every), max(every))
        study = _model.load_study()["weights"]
        self._weights = tuple(study["balanced"][c] for c in ORDER)
        self._heavy = tuple(_model.reweigh(study["balanced"], "cost", .6)[c] for c in ORDER)
        self._maximum = max(_room.result(v["design"])["electricity"] for v in variants(True)) * 1.1
        self._rt = self.study_trackers(variants()[0]["design"])
        self.focus, self.active, self.count, self.detail, self.row_hi = mn.ValueTracker(0.0), mn.ValueTracker(-1.0), mn.ValueTracker(0.0), mn.ValueTracker(1.0), mn.ValueTracker(-1.0)
        self.roster_pos, self.roster_o = mn.ValueTracker(0.0), mn.ValueTracker(0.0)
        self.panel = self.build_panel()
        self.strip = self.value_strip()
        self.footer = _room.place(_room.text(self.tr("Simplified annual teaching model · degree-hours method · EN ISO 13790 style",
                                                      "Vereinfachtes Jahres-Lehrmodell · Gradstundenverfahren · Stil EN ISO 13790").split(), size=13, color=MUTED), (LEFT + .2, 3.68, 0), "left")
        self.season = "summer"

    def present_index(self):
        """🎞️ Cards, units, rulers, the weighted table, sensitivity, range dependence, the decision record and its limit."""
        self.prepare()
        self.screen_cards()
        self.screen_units()
        self.screen_rulers()
        self.screen_board()
        self.screen_sweep()
        self.screen_range()
        self.screen_record()

    def clear_stage(self):
        """🧹 Fade out everything but the caption and the persistent room panel; live widgets go through static twins."""
        kept = (self._caption, getattr(self, "panel", None), getattr(self, "strip", None), getattr(self, "footer", None), getattr(self, getattr(self, "season", "summer"), None))
        content = [item for item in self.mobjects if not any(item is other for other in kept)]
        if content:
            self.play(*[Retire(item) if any(isinstance(member, _room.LiveGroup) for member in item.get_family()) else mn.FadeOut(item) for item in content], run_time=.5)

    def n(self, value, digits=0):
        """🔢 A live number part of a label."""
        return _room.Number(self.number(value, digits))

    def words(self, value):
        """💬 The words of a phrase as label parts."""
        return str(value).split()

    def gap(self, size):
        """↔️ The space between two runs of a row."""
        return _room._glyphs()[1] * size / _room.REFERENCE_SIZE * 1.8

    def equation(self, items, x, y, *, size=14, edge="left", width=None):
        """🧾 One row of coloured runs on a common baseline: ``items`` are ``(parts, colour)``; ``x`` anchors the left, centre or right edge."""
        labels, cursor = [], 0.0
        for parts, colour in items:
            label = _room.row(size, colour, *parts)
            label.shift(np.array([cursor - label.get_left()[0], 0.0, 0.0]))
            labels.append(label)
            cursor = label.get_right()[0] + self.gap(size)
        span = labels[-1].get_right()[0] - labels[0].get_left()[0]
        start = {"left": x, "center": x - span / 2, "right": x - span}[edge]
        group = mn.VGroup(*labels)
        group.shift(np.array([start - labels[0].get_left()[0], y, 0.0]))
        if width is not None and span > width:
            group.scale(width / span, about_point=np.array([start, y, 0.0]) if edge == "left" else None)
        return group

    def name(self, identifier, size=S0, width=1.5, color=INK, limit=14):
        """🏷️ The variant's model title in the scene language, on one or two lines."""
        return block(lines(by_id(identifier)["title"][self.language()], limit), size=size, color=color, width=width)

    def value_text(self, criterion, value, digits=None):
        """🔢 A raw value with the model's digits for its criterion."""
        return self.number(value, DIGITS[criterion] if digits is None else digits)

    def unit_label(self, criterion):
        """🏷️ The unit of a criterion as the model gives it."""
        return CRITERIA[criterion]["unit"].replace("kWh/(m²·a)", _room.INTENSITY_UNIT)

    def focus_id(self):
        """📌 The variant the room panel currently shows."""
        return ids(True)[int(round(min(max(self.focus.get_value(), 0), len(ids(True)) - 1)))]

    def set_focus(self, identifier, run_time=1.0):
        """🎯 Move the room panel to a variant: the design trackers and the focus index move together."""
        return mn.AnimationGroup(self.set_design(self._rt, by_id(identifier)["design"], run_time), self.focus.animate(run_time=run_time).set_value(ids(True).index(identifier)))

    def build_panel(self):
        """🏠 The persistent room panel: title, section with sun, sky light, daylight zone and lamps, spec chips, overhang dimension and the heat arrows of the season."""
        rt, language = self._rt, self.language()
        key, focus_key = _room.design_key(rt), (lambda: round(self.focus.get_value()))
        title = live(lambda: mn.VGroup(_room.place(_room.text(self.words(by_id(self.focus_id())["title"][language]), size=21, color=INK), (6.95, 3.3, 0), "right")), focus_key)

        def specs():
            design = _room.values(rt)
            found = _room.result(design)
            chips = [_room.chip([self.tr("Window", "Fenster"), self.n(design["wwr"] * 100), "%", "·", self.n(found["window_area"], 1), "m²"], size=14, color=_room.GLASS),
                     _room.chip([self.tr("Overhang", "Vordach"), self.n(design["overhang"], 1), "m"], size=14, color=WARN),
                     _room.chip([self.tr("Insulation", "Dämmung"), self.n(design["insulation"] * 100), "cm"], size=14, color=_room.INSULATION)]
            group = mn.VGroup()
            for k, chip in enumerate(chips):
                group.add(_room.place(chip, (6.95, 2.8 - .42 * k, 0), "right"))
            return group

        def dimension():
            design = _room.values(rt)
            depth, thickness = design["overhang"], design["insulation"]
            x_o = -(MASONRY + thickness)
            a, b = P(x_o - depth, HEIGHT + SLAB + .35), P(x_o, HEIGHT + SLAB + .35)
            if depth < .01:
                label = _room.text(self.words(self.tr("no overhang", "kein Vordach")), P(2.4, HEIGHT + SLAB + .6), size=14, color=WARN)
                return mn.VGroup(label, mn.Line(label.get_left() + np.array([-.05, 0, 0]), P(x_o + .05, HEIGHT + SLAB + .03), color=WARN, stroke_width=1.4, stroke_opacity=.8)).set_z_index(8)
            return mn.VGroup(_room._dimension(a, b, color=WARN, tick=.07), _room.chip(["d", self.n(depth, 1), "m"], (a + b) / 2 + np.array([0, .3, 0]), size=14, color=WARN)).set_z_index(8)

        section = self.room_section(rt, SECTION_POINT, ROOM_SCALE, "summer", labels=False, demand=False)
        self.summer = mn.VGroup(section, self.demand_arrows("summer"))
        return mn.VGroup(live(dimension, key), live(specs, key), title)

    def demand_arrows(self, season):
        """➡️ Heat arrows leaving through the back wall: red for the winter heating demand, blue for the summer cooling demand; length and weight follow the model."""
        rt, heating = self._rt, season == "winter"

        def build():
            found = _room.result(_room.values(rt))
            key = "heating_demand" if heating else "cooling_demand"
            share = found[key] / _room.reference()[key]
            wall = P(DEPTH + MASONRY, 0)[0]
            group = mn.VGroup()
            for y in (1.1, 1.6, 2.1):
                start = P(DEPTH - .5, y)
                group.add(mn.Arrow(start, np.array([wall + .12 + .36 * share, start[1], 0.0]), buff=0, color=_room.HEAT if heating else _room.COOL, stroke_width=3 + 3 * share,
                                   max_tip_length_to_length_ratio=.35, tip_length=.12, max_stroke_width_to_length_ratio=12))
            return group
        return live(build, _room.design_key(rt))

    def heat_loss_arrows(self):
        """🔴 Winter heat loss: red arrows leave through the opaque wall below the window and through the glass, weight and length bound to the heating demand."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            share = found["heating_demand"] / _room.reference()["heating_demand"]
            window = min(found["window_area"] / WIDTH, HEAD - .05)
            sill, x_o = HEAD - window, -(MASONRY + design["insulation"])
            group = mn.VGroup()
            for y in (sill * .3, sill * .65, (sill + HEAD) / 2):
                start = P(.3, y)
                group.add(mn.Arrow(start, np.array([P(x_o, 0)[0] - .12 - .5 * share, start[1], 0.0]), buff=0, color=_room.HEAT, stroke_width=3 + 3 * share, max_tip_length_to_length_ratio=.25,
                                   tip_length=.12, max_stroke_width_to_length_ratio=12))
            return group
        return live(build, _room.design_key(rt))

    def swap_season(self, season, run_time=.6):
        """🍂 Cross-fade the room panel's section between its summer and winter picture."""
        if season == self.season:
            return mn.AnimationGroup()
        if season == "winter":
            self.winter = mn.VGroup(self.room_section(self._rt, SECTION_POINT, ROOM_SCALE, "winter", labels=False, demand=False), self.heat_loss_arrows())
        old, new = (self.summer, self.winter) if season == "winter" else (self.winter, self.summer)
        self.season = season
        return mn.AnimationGroup(Retire(old, run_time=run_time), mn.FadeIn(new, run_time=run_time))

    def cost_raws(self, design, found):
        """💰 The five cost parts in CHF: fixed items, wall, insulation, windows, overhang."""
        price = lambda name: _room.constant("prices", name)
        return [price("fixed"), found["wall_area"] * price("wall"), found["wall_area"] * design["insulation"] * 100 * price("insulation_per_cm"), found["window_area"] * price("window"),
                WIDTH * design["overhang"] * price("overhang")]

    def carbon_raws(self, design, found):
        """🏭 The four carbon layers in kg CO₂e: masonry, insulation, glass, overhang."""
        factor = lambda name: _room.constant("carbon", name)
        return [found["wall_area"] * factor("wall"), found["wall_area"] * design["insulation"] * 100 * factor("insulation_per_cm"), found["window_area"] * factor("window"),
                WIDTH * design["overhang"] * factor("overhang")]

    def value_strip(self):
        """🔢 The four raw values of the room on show, each with its pictogram and unit; the active criterion is framed."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            shown, active = int(round(self.count.get_value())), int(round(self.active.get_value()))
            parts = {"electricity": ([found[key] for key, _ in ENERGY], [colour for _, colour in ENERGY], 760),
                     "cost": (self.cost_raws(design, found), [LAYER[k] for k in ("fixed", "wall", "insulation", "window", "overhang")], 22500),
                     "daylight": ([found["daylight"]], [_room.LIGHT], 6),
                     "carbon": (self.carbon_raws(design, found), [LAYER[k] for k in ("wall", "insulation", "window", "overhang")], 5600)}
            group = mn.VGroup()
            for k, c in enumerate(ORDER[:shown]):
                x, y = 2.1 + 2.45 * (k % 2), -2.05 - .38 * (k // 2)
                label = _room.text([self.n(found[CRITERIA[c]["key"]], DIGITS[c]), *self.words(self.unit_label(c))], size=14, color=COLORS[c])
                _room.place(label, (x + .42, y, 0), "left")
                group.add(_room.clone(glyph(c, width=.26)).move_to([x + .17, y, 0]), label)
                values, colours, full = parts[c]
                cursor = x + .42
                for value, colour in zip(values, colours):
                    width = max(value / full * 1.9, 1e-3)
                    group.add(mn.Rectangle(width=width, height=.08, stroke_width=0, fill_color=colour, fill_opacity=.9).move_to([cursor + width / 2, y - .17, 0]))
                    cursor += width
                if c == "daylight":
                    group.add(mn.Line([x + .42 + 2 / 6 * 1.9, y - .22, 0], [x + .42 + 2 / 6 * 1.9, y - .12, 0], color=INK, stroke_width=1.6))
                if k == active:
                    group.add(mn.Rectangle(width=2.35, height=.42, color=COLORS[c], stroke_width=2.4).move_to([x + 1.12, y - .03, 0]))
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.count.get_value()), round(self.active.get_value())))

    def materials_overlay(self, layers):
        """🧱 Tint the façade layers of the room section: insulation green, masonry cream, glass cyan and the overhang slab orange, each in its layer colour."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            depth, thickness = max(design["overhang"], 0.0), design["insulation"]
            window = min(found["window_area"] / WIDTH, HEAD - .05)
            sill, x_o = HEAD - window, -(MASONRY + thickness)
            boxes = {"insulation": [(x_o, 0, -MASONRY, sill), (x_o, HEAD, -MASONRY, HEIGHT)], "wall": [(-MASONRY, 0, 0, sill), (-MASONRY, HEAD, 0, HEIGHT)],
                     "window": [(x_o, sill, x_o + .2, HEAD)], "overhang": [(x_o - depth, HEIGHT, x_o, HEIGHT + SLAB)] if depth > .01 else []}
            group = mn.VGroup()
            for layer in layers:
                for x0, y0, x1, y1 in boxes[layer]:
                    a, b = P(x0, y0), P(x1, y1)
                    group.add(mn.Rectangle(width=max(b[0] - a[0], 1e-3), height=max(b[1] - a[1], 1e-3), color=LAYER[layer], stroke_width=3, fill_color=LAYER[layer],
                                           fill_opacity=.5).move_to((a + b) / 2))
            return group
        return live(build, _room.design_key(rt))

    def marker(self, colour, y):
        """⏹️ A small square in front of a row of a build-up."""
        return mn.Square(.14, stroke_width=0, fill_color=colour, fill_opacity=.9).move_to([LEFT + .12, y, 0])

    def lighting_row(self, found, digits, shown, y):
        """💡 Installed power · hours · share of the hours the lamps burn = lighting electricity, with the share printed as precisely as the result needs."""
        power = _room.constant("lighting", "power_density") * FLOOR_AREA
        hours = _room.constant("lighting", "hours")
        on = 100 * (1 - found["autonomy"])
        precision, sign = reproduce(lambda value: power * hours / 1000 * value / 100, [on], shown, digits)
        items = [([self.tr("Lighting", "Beleuchtung")], _room.LIGHT),
                 ([self.n(power), "W", "·", self.n(hours), "h/a", "·", *self.words(self.tr("lamps on", "Lampen an")), self.n(on, precision), "%", sign], INK),
                 ([self.n(shown, digits), "kWh/a"], _room.LIGHT)]
        return self.equation(items, LEFT + .4, y - .07, size=14, width=6.6)

    def bar_row(self, group, parts, total_parts, unit, colour_of):
        """📊 A horizontal stacked bar of the parts (length ∝ value) with the total in front of its end."""
        cursor = LEFT + .4
        for value, colour in zip(parts, colour_of):
            width = max(value * unit, 1e-3)
            group.add(mn.Rectangle(width=width, height=.22, stroke_width=0, fill_color=colour, fill_opacity=.88).move_to([cursor + width / 2, -.62, 0]))
            cursor += width
        group.add(self.equation([(total_parts, INK)], cursor + .15, -.62 - .07, size=15))

    def highlight(self, group, k, y):
        """🔦 Frame row ``k`` of a build-up while the clause talks about it."""
        if int(round(self.row_hi.get_value())) == k:
            group.add(mn.Rectangle(width=6.8, height=.31, color=INK, stroke_width=2, stroke_opacity=.8).move_to([(LEFT + RIGHT) / 2, y, 0]))

    def block_electricity(self):
        """⚡ Heating, cooling and lighting electricity of the room on show: stacked bar and the arithmetic behind every part, down to kWh/(m²·a)."""
        rt = self._rt

        def build():
            found = _room.result(_room.values(rt))
            digits = energy_digits(found)
            shown = _room.apportion([found[key] for key, _ in ENERGY], digits)
            total = round(sum(shown), digits)
            group, detail = mn.VGroup(), self.detail.get_value() > .5
            self.bar_row(group, [found[key] for key, _ in ENERGY], [self.n(total, digits), "kWh/a"], 4.4 / 760, [colour for _, colour in ENERGY])
            ys = [-1.0 - .32 * k for k in range(5)]
            names = (self.tr("Heating", "Heizung"), self.tr("Cooling", "Kühlung"))
            if detail:
                for k, (key, colour) in enumerate(ENERGY[:2]):
                    factor = _room.systems()["cop" if k == 0 else "eer"]
                    precision, sign = reproduce(lambda value: value / factor, [found[key + "_demand"]], shown[k], digits)
                    items = [([names[k]], colour), ([self.n(found[key + "_demand"], precision), "kWh/a", *self.words(self.tr("heat" if k == 0 else "cold", "Wärme" if k == 0 else "Kälte")), "÷", "COP" if k == 0 else "EER",
                                                     self.n(factor, 1), sign], INK), ([self.n(shown[k], digits), "kWh/a"], colour)]
                    group.add(self.marker(colour, ys[k]), self.equation(items, LEFT + .4, ys[k] - .07, size=14, width=6.6))
                group.add(self.marker(_room.LIGHT, ys[2]), self.lighting_row(found, digits, shown[2], ys[2]))
                for k in range(3):
                    self.highlight(group, k, ys[k])
            sums = [([self.tr("Total", "Summe")], INK)] + [item for k, (_, colour) in enumerate(ENERGY) for item in (([self.n(shown[k], digits)], colour), (["+" if k < 2 else "="], INK))]
            group.add(self.equation(sums + [([self.n(total, digits), "kWh/a"], INK)], LEFT + .4, ys[3] - .07, size=14, width=6.6))
            group.add(self.equation([([self.n(total, digits), "kWh/a", "÷", self.n(FLOOR_AREA), "m²", "="], INK), ([self.n(found["intensity"], 2), _room.INTENSITY_UNIT], COLORS["electricity"])],
                                    LEFT + .4, ys[4] - .07, size=14, width=6.6))
            for k in (3, 4):
                self.highlight(group, k, ys[k])
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.detail.get_value()), round(self.row_hi.get_value())))

    def block_cost(self):
        """💰 The façade price of the room on show as its parts: fixed items, wall, insulation, windows and overhang, each with area × price."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            price = lambda name: _room.constant("prices", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = self.cost_raws(design, found)
            shown = _room.apportion(raws, 0)
            colours = [LAYER[k] for k in ("fixed", "wall", "insulation", "window", "overhang")]
            group = mn.VGroup()
            self.bar_row(group, raws, [self.n(sum(shown)), "CHF"], 4.6 / 22500, colours)
            cost = lambda name, rest: ([self.tr(*name)], colours[("fixed", "wall", "insulation", "window", "overhang").index(name[2])])
            rows = [(("Fixed items", "Fixposten", "fixed"), []),
                    (("Wall", "Wand", "wall"), [self.n(wall, 1), "m²", "×", self.n(price("wall")), "CHF/m²"]),
                    (("Insulation", "Dämmung", "insulation"), [self.n(wall, 1), "m²", "×", self.n(thickness), "cm", "×", self.n(price("insulation_per_cm")), "CHF/m²/cm"]),
                    (("Windows", "Fenster", "window"), [self.n(window, 1), "m²", "×", self.n(price("window")), "CHF/m²"]),
                    (("Overhang", "Vordach", "overhang"), [self.n(WIDTH), "m", "×", self.n(depth, 1), "m", "×", self.n(price("overhang")), "CHF/m²"])]
            for k, ((en, de, layer), formula) in enumerate(rows):
                y = -1.0 - .32 * k
                items = [([self.tr(en, de)], LAYER[layer])] + ([(formula + ["="], INK)] if formula else []) + [([self.n(shown[k]), "CHF"], LAYER[layer])]
                group.add(self.marker(LAYER[layer], y), self.equation(items, LEFT + .4, y - .07, size=14, width=6.6))
                self.highlight(group, k, y)
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.row_hi.get_value())))

    def block_carbon(self):
        """🏭 The embodied carbon of the room on show as material layers: masonry, insulation, glass and the concrete overhang, in kg CO₂e per layer."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            factor = lambda name: _room.constant("carbon", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = self.carbon_raws(design, found)
            shown = _room.apportion(raws, 0)
            layers = ("wall", "insulation", "window", "overhang")
            group = mn.VGroup()
            self.bar_row(group, raws, [self.n(sum(shown)), "kg", "CO₂e"], 4.6 / 5600, [LAYER[k] for k in layers])
            rows = [(("Masonry", "Mauerwerk"), [self.n(wall, 1), "m²", "×", self.n(factor("wall")), "kg", "CO₂e/m²"]),
                    (("Insulation", "Dämmung"), [self.n(wall, 1), "m²", "×", self.n(thickness), "cm", "×", self.n(factor("insulation_per_cm"), 1), "kg", "CO₂e/m²/cm"]),
                    (("Glass", "Verglasung"), [self.n(window, 1), "m²", "×", self.n(factor("window")), "kg", "CO₂e/m²"]),
                    (("Concrete overhang", "Vordach aus Beton"), [self.n(WIDTH), "m", "×", self.n(depth, 1), "m", "×", self.n(factor("overhang")), "kg", "CO₂e/m²"])]
            for k, ((en, de), formula) in enumerate(rows):
                y = -1.0 - .32 * k
                items = [([self.tr(en, de)], LAYER[layers[k]]), (formula + ["="], INK), ([self.n(shown[k]), "kg", "CO₂e"], LAYER[layers[k]])]
                group.add(self.marker(LAYER[layers[k]], y), self.equation(items, LEFT + .4, y - .07, size=14, width=6.6))
                self.highlight(group, k, y)
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.row_hi.get_value())))

    def block_daylight(self):
        """☀️ The daylight factor of the room on show: gauge against the 2 % daylit limit, the sky angle behind the overhang, the substituted formula and the lamps it saves."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            const = lambda name: _room.constant("daylight", name)
            depth = design["overhang"]
            theta = const("sky_angle") * (1 - const("sky_angle_loss") * depth ** 2 / (depth ** 2 + const("overhang_half_depth") ** 2))
            factor = found["daylight"]
            group, x0, scale = mn.VGroup(), LEFT + .4, 4.8 / 6
            group.add(mn.Rectangle(width=max(factor * scale, 1e-3), height=.22, stroke_width=0, fill_color=_room.LIGHT, fill_opacity=.88).move_to([x0 + factor * scale / 2, -.62, 0]),
                      mn.Line([x0, -.62 - .16, 0], [x0 + 6 * scale, -.62 - .16, 0], color=MUTED, stroke_width=1.4))
            for mark in (0, 2, 4, 6):
                group.add(mn.Line([x0 + mark * scale, -.62 - .16, 0], [x0 + mark * scale, -.62 - .22, 0], color=MUTED, stroke_width=1.4),
                          _room.text([self.n(mark), "%"], (x0 + mark * scale, -.62 - .36, 0), size=13, color=MUTED))
            group.add(_room.dashed([x0 + 2 * scale, -.62 + .15, 0], [x0 + 2 * scale, -.62 - .16, 0], color=INK, stroke_width=2),
                      self.equation([([self.n(factor, 2), "%"], _room.LIGHT)], x0 + 6 * scale + .2, -.62 - .07, size=15))
            ys = [-1.4 - .32 * k for k in range(4)]
            sky = [([self.tr("Sky angle", "Himmelswinkel"), "θ", "=", self.n(theta, 1), "~°"], _room.LIGHT)]
            sky.append(([*self.words(self.tr("with the overhang; 75° without", "mit Vordach; 75° ohne"))], MUTED) if depth > .01 else ([*self.words(self.tr("no overhang: full sky view", "kein Vordach: voller Himmel"))], MUTED))
            group.add(self.marker(_room.LIGHT, ys[0]), self.equation(sky, LEFT + .4, ys[0] - .07, size=14, width=6.6))
            area = round(found["window_area"], 1)
            precision, sign = reproduce(lambda angle: const("transmittance") * area * angle / (const("surface_area") * (1 - const("reflectance") ** 2)), [theta], factor, 2)
            group.add(self.marker(_room.LIGHT, ys[1]), self.equation([([self.tr("DF", "TQ"), "="], _room.LIGHT),
                                                                      ([self.n(const("transmittance"), 1), "·", self.n(area, 1), "m²", "·", self.n(theta, precision), "~°", "÷", "(" + self.number(const("surface_area")), "m²", "·",
                                                                        self.number(1 - const("reflectance") ** 2, 2) + ")", sign], INK), ([self.n(factor, 2), "%"], _room.LIGHT)], LEFT + .4, ys[1] - .07, size=14, width=6.6))
            reach = _room.daylit_depth(factor)
            group.add(self.marker(_room.DAYLIGHT, ys[2]), self.equation([([*self.words(self.tr("Daylit ≥ 2 % to", "Tageslichtversorgt ≥ 2 % bis")), self.n(reach, 1), "m", *self.words(self.tr("of 6 m (linear falloff)", "von 6 m (lineares Abklingen)"))], INK)],
                                                                      LEFT + .4, ys[2] - .07, size=14, width=6.6))
            digits = energy_digits(found)
            shown = _room.apportion([found[key] for key, _ in ENERGY], digits)
            group.add(self.marker(_room.LIGHT, ys[3]), self.lighting_row(found, digits, shown[2], ys[3]))
            return group
        return live(build, _room.design_key(rt))

    def index_terms(self, identifier, weights):
        """🧮 Weights, normalized values and the four weighted parts as printed: the parts add up to the printed index and every product holds as printed."""
        row = table(tuple(round(w, 6) for w in weights))[identifier]
        for digits in (2, 3, 4):
            printed_w = [round(w, digits) for w in weights]
            norm = [round(row["normalized"][c], digits) for c in ORDER]
            if round(sum(w * v for w, v in zip(printed_w, norm)), 2) == round(row["score"], 2) and abs(sum(printed_w) - 1) < 1e-9:
                break
        parts = _room.apportion([w * v for w, v in zip(printed_w, norm)], 2, total=row["score"])
        return printed_w, norm, parts, digits

    def block_index(self, getter, weights, *, full=False, y=-2.2):
        """⚖️ The index of one variant written out: ``w × n`` per criterion, the four parts and their sum; ``full`` adds the raw value behind every normalized value."""
        def build():
            ws = weights()
            identifier = getter()
            printed_w, norm, parts, digits = self.index_terms(identifier, ws)
            group = mn.VGroup()
            if not full:
                items = [([self.tr("Index", "Index"), "="], INK)]
                for k, c in enumerate(ORDER):
                    items += [([self.n(printed_w[k], digits), "×", self.n(norm[k], digits)], COLORS[c]), (["+" if k < 3 else "="], INK)]
                items.append(([self.n(sum(parts), 2)], GOOD))
                group.add(self.equation(items, LEFT + .1, y - .07, size=14, width=6.8))
                return group
            exact = table(tuple(round(w, 6) for w in weights()))[identifier]
            norm3 = [round(exact["normalized"][c], 3) for c in ORDER]
            terms = [round(w * v, 4) for w, v in zip(printed_w, norm3)]
            for k, c in enumerate(ORDER):
                yy = 2.45 - .55 * k
                items = [([self.n(raw(identifier, c), DIGITS[c]), *self.words(self.unit_label(c))], COLORS[c]), (["→"], MUTED),
                         ([self.tr("normalized", "normiert"), self.n(norm3[k], 3), "×", self.n(printed_w[k] * 100, digits - 2), "%", "="], INK), ([self.n(terms[k], 4)], COLORS[c])]
                group.add(_room.clone(glyph(c, width=.26)).move_to([LEFT + .15, yy, 0]), self.equation(items, LEFT + .45, yy - .07, size=14, width=6.4))
            group.add(self.equation([([self.tr("Index", "Index"), "="], INK), ([self.n(terms[0], 4), "+", self.n(terms[1], 4), "+", self.n(terms[2], 4), "+", self.n(terms[3], 4), "="], INK),
                                     ([self.n(sum(terms), 4)], GOOD)], LEFT + .45, 2.45 - .55 * 4 - .07, size=16, width=6.4))
            return group
        return live(build, lambda: (getter(), tuple(round(w, 4) for w in weights())))

    def criterion_chip(self, criterion, x, bottom):
        """🔖 The head of a table column: pictogram, name, unit and direction in a chip whose bottom edge sits at ``bottom``."""
        chip = _art.window_chip(head_content(self, criterion), color=COLORS[criterion])
        return chip.move_to([x, bottom + chip.height / 2, 0])

    def screen_cards(self):
        """🪪 The five variants as rows of a table next to the room: a cursor walks through them, spec columns, then the four criteria columns and the best value of each."""
        pool = variants()
        Y = lambda k: 1.4 - .8 * k
        CX = [-3.9 + 1.18 * j for j in range(4)]
        CS = [-3.65 + 1.3 * j for j in range(3)]
        base = mn.VGroup()
        for k, item in enumerate(pool):
            base.add(self.thumbnail(item["design"], (-6.3, Y(k), 0), .15, ramp=self._ramp, value=self._ramp.low), self.name(item["id"], size=S0, width=1.5).move_to([-5.15, Y(k), 0]))
        cursor = mn.Rectangle(width=FRAME_X1 - FRAME_X0, height=.7, color=INK, stroke_width=2.4)
        cursor.add_updater(lambda m: (m.move_to([(FRAME_X0 + FRAME_X1) / 2, 1.4 - .8 * self.roster_pos.get_value(), 0]), m.set_stroke(opacity=self.roster_o.get_value())))
        cursor.update()
        spec = mn.VGroup()
        for j, (en, de, unit, colour, key, digits, scale) in enumerate((("Window", "Fenster", "%", _room.GLASS, "wwr", 0, 100), ("Overhang", "Vordach", "m", WARN, "overhang", 1, 1),
                                                                         ("Insulation", "Dämmung", "cm", _room.INSULATION, "insulation", 0, 100))):
            head = mn.VGroup(_room.text([self.tr(en, de)], size=13, color=colour), _room.text([unit], size=13, color=colour)).arrange(mn.DOWN, buff=.05)
            chip = _art.window_chip(head, color=colour)
            spec.add(chip.move_to([CS[j], 2.0 + chip.height / 2, 0]))
            for k, item in enumerate(pool):
                spec.add(text(self.number(item["design"][key] * scale, digits), (CS[j], Y(k), 0), size=19, color=colour))
        cols = {}
        for j, c in enumerate(ORDER):
            group = mn.VGroup(self.criterion_chip(c, CX[j], 2.0))
            for k, item in enumerate(pool):
                group.add(text(self.value_text(c, raw(item["id"], c)), (CX[j], Y(k), 0), size=18, color=COLORS[c], width=.98))
            cols[c] = group
        frames = []
        for j, c in enumerate(ORDER):
            winner = best_id(c, ids())
            frames.append(mn.Rectangle(width=.86, height=.5, color=GOOD, stroke_width=3).move_to([CX[j], Y(ids().index(winner)), 0]))
        self.add(cursor)
        walk = []
        for position in (0, 1, 2, 3, 4, 0):
            walk += [mn.AnimationGroup(self.roster_pos.animate(run_time=.35).set_value(position), self.set_focus(ids()[position], .35)), mn.Wait(.3)]
        self.step("variants", mn.FadeIn(self.panel), mn.FadeIn(self.summer), mn.FadeIn(self.footer), mn.LaggedStart(*[mn.FadeIn(part) for part in base], lag_ratio=.12),
                  mn.Succession(mn.Wait(.8), self.roster_o.animate(run_time=.3).set_value(1.0), *walk), seconds=4.4)
        self.step("variants_2", mn.FadeIn(spec), seconds=1.8)
        self.step("criteria", mn.FadeOut(spec), mn.FadeIn(cols["electricity"]), mn.FadeIn(cols["cost"]), mn.FadeIn(self.strip), self.count.animate(run_time=.3).set_value(2), seconds=1.8)
        self.step("criteria_2", mn.FadeIn(cols["daylight"]), mn.FadeIn(cols["carbon"]), self.count.animate(run_time=.3).set_value(4), seconds=1.8)
        visits = []
        for j, c in enumerate(ORDER):
            winner = best_id(c, ids())
            visits += [mn.AnimationGroup(self.roster_pos.animate(run_time=.4).set_value(ids().index(winner)), self.set_focus(winner, .4), self.active.animate(run_time=.1).set_value(j),
                                         mn.Create(frames[j], run_time=.4)), mn.Wait(.3)]
        self.step("best", mn.Succession(*visits), seconds=3.4)
        self.a_items = [base, cursor, *cols.values(), *frames]

    def sums(self, scale):
        """➕ The naive sum of the printed raw numbers with cost and carbon divided by a scale."""
        return {i: sum(raw(i, c) / (scale if c in ("cost", "carbon") else 1) for c in ORDER) for i in ids()}

    def naive_rows(self, scale):
        """📶 One row per variant: thumbnail, name, a bar of four coloured segments and the sum, sorted by the sum."""
        sums = self.sums(scale)
        order = sorted(ids(), key=sums.get)
        longest = max(sums.values())
        rows = {}
        for rank, identifier in enumerate(order):
            y = 1.9 - .72 * rank
            thumb = self.thumbnail(by_id(identifier)["design"], (-6.35, y, 0), .11, ramp=self._ramp, value=self._ramp.low)
            name = self.name(identifier, size=S0, width=1.5).move_to([-5.25, y, 0])
            cursor, segments = -4.2, []
            for c in ORDER:
                value = raw(identifier, c) / (scale if c in ("cost", "carbon") else 1)
                width = max(value / longest * 3.0, .012)
                segments.append(mn.Rectangle(width=width, height=.4, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85).move_to([cursor + width / 2, y, 0]))
                cursor += width
            total = text(self.number(sums[identifier], 0 if scale == 1 else 1), (cursor + .6, y, 0), size=S1)
            total.move_to([cursor + .12 + total.width / 2, y, 0])
            badge = mn.Circle(radius=.15, color=GOOD if rank == 0 else INK, stroke_width=2).move_to([-6.9, y, 0])
            number = text(str(rank + 1), badge.get_center(), size=S0, color=GOOD if rank == 0 else INK)
            rows[identifier] = mn.VGroup(thumb, name, *segments, total, badge, number)
        return rows, order

    def legend(self, scale):
        """🏷️ Four coloured pictograms with the unit each segment is counted in."""
        group = mn.VGroup()
        for c in ORDER:
            unit = self.unit_label(c)
            if scale != 1 and c == "cost":
                unit = "kCHF"
            if scale != 1 and c == "carbon":
                unit = "t CO₂e"
            group.add(mn.VGroup(glyph(c, width=.28), text(unit, size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.1))
        return group.arrange(mn.RIGHT, buff=.4).move_to([-3.5, -1.95, 0])

    def screen_units(self):
        """➕ Why kWh/(m²·a) and CHF cannot be added, and why a naive sum depends on the unit."""
        first = ids()[0]
        item = by_id(first)
        chips = []
        for c in ("electricity", "cost"):
            content = mn.VGroup(glyph(c, width=.34), text(self.value_text(c, raw(first, c)), size=S2 + 2), text(self.unit_label(c), size=S1, color=COLORS[c])).arrange(mn.RIGHT, buff=.16)
            chips.append(_art.window_chip(content, color=COLORS[c]))
        plus = text("+", size=S2 + 6)
        tail = mn.VGroup(text("=", size=S2 + 6), text("?", size=S2 + 6)).arrange(mn.RIGHT, buff=.25)
        equation = mn.VGroup(chips[0], plus, chips[1], tail).arrange(mn.DOWN, buff=.18).move_to([-3.5, .5, 0])
        cross = _art.cross_mark(color=BAD, size=.28).next_to(tail, mn.RIGHT, buff=.3)
        verdict = _art.window_chip(text(self.tr("no unit", "keine Einheit"), size=S2, color=BAD), color=BAD).move_to([-3.5, -1.75, 0])
        subject = mn.VGroup(self.thumbnail(item["design"], (0, 0, 0), .26, ramp=self._ramp, value=self._ramp.low), self.name(first, size=S1, width=3.2)).arrange(mn.RIGHT, buff=.3).move_to([-3.5, 2.6, 0])
        self.step("units", *[mn.FadeOut(part) for part in self.a_items], *[mn.FadeIn(chip) for chip in chips], mn.FadeIn(subject), mn.FadeIn(plus), mn.FadeIn(tail),
                  mn.Create(cross), mn.FadeIn(verdict), self.set_focus(first, .8), self.active.animate(run_time=.1).set_value(-1), seconds=2.4)
        rows, order = self.naive_rows(1)
        legend = self.legend(1)
        head = _room.place(text(self.tr("Sum of the printed numbers", "Summe der gedruckten Zahlen"), size=S1, color=MUTED), (LEFT, 3.1, 0), "left")
        unit_chip = _art.window_chip(text(self.tr("Units: CHF · kg", "Einheiten: CHF · kg"), size=S1, color=WARN), color=WARN)
        _room.place(unit_chip, (RIGHT, 2.5, 0), "right")
        self.rows = rows
        self.step("units_2", mn.FadeOut(mn.VGroup(*chips, plus, tail)), mn.FadeOut(cross), mn.FadeOut(verdict), mn.FadeOut(subject),
                  *[mn.FadeIn(r) for r in rows.values()], mn.FadeIn(legend), mn.FadeIn(head), mn.FadeIn(unit_chip), self.set_focus(order[0], .8), seconds=2.2)
        after, order2 = self.naive_rows(1000)
        legend2 = self.legend(1000)
        head2 = _room.place(text(self.tr("Still a sum of unlike units", "Immer noch eine Summe verschiedener Einheiten"), size=S1, color=MUTED), (LEFT, 3.1, 0), "left")
        unit_chip2 = _art.window_chip(text(self.tr("Units: kCHF · t", "Einheiten: kCHF · t"), size=S1, color=GOOD), color=GOOD)
        _room.place(unit_chip2, (RIGHT, 2.5, 0), "right")
        self.step("units_3", *[mn.ReplacementTransform(rows[i], after[i]) for i in ids()], mn.ReplacementTransform(legend, legend2),
                  mn.ReplacementTransform(unit_chip, unit_chip2), mn.ReplacementTransform(head, head2), self.set_focus(order2[0], .8), seconds=2.8)
        self.naive_order = (order, order2)

    def ruler_away(self, ruler, header, away, run_time=.6):
        """🫥 Fade the ruler out while the winter picture needs the whole width, and back in afterwards."""
        level = 0.0 if away else 1.0
        parts = [ruler.line, header, *[ruler.members[i][k] for i in ruler.pool for k in ("slot", "dot", "leader")]]
        return mn.AnimationGroup(ruler.tween(run_time, o_abs=level, o_val=level, o_ring=level), *[(mn.FadeOut(part) if away else mn.FadeIn(part)) for part in parts], run_time=run_time)

    def screen_rulers(self):
        """📏 One ruler per criterion: real values, subtract the best, divide by the span; then the table."""
        pool = ids()
        self.clear_stage()
        ruler = Ruler(self, "electricity", pool, pool)
        header = self.ruler_header(ruler)
        block = self.block_electricity()
        self.add(ruler.ticks)
        first = ruler.scan[0]
        self.step("ruler", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(pool)), mn.FadeIn(block), ruler.appear(1.2), self.set_focus(first, .8),
                  self.active.animate(run_time=.3).set_value(0), seconds=2.4)
        self.add(ruler.ring)
        last = len(ruler.scan) - 1
        self.step("ruler_2", mn.Succession(ruler.tween(.3, o_ring=1.0), ruler.visit(list(range(1, last + 1)) + [0], hold=.4, travel=.6)), seconds=5.0)
        bare = ruler.scan.index("glass_bare")
        self.step("ruler_3", ruler.focus.animate(run_time=.8).set_value(bare), self.set_focus("glass_bare", .8), self.row_hi.animate(run_time=.1).set_value(1), seconds=3.6)
        winter = mn.VGroup(self.winter_note(), self.derivation(self._rt, "heating", (LEFT + .4, .4, 0), size=18, words=False, align="left", width=5.2))
        self.step("ruler_4", self.ruler_away(ruler, header, True), self.set_focus("punched", .8), self.swap_season("winter"), self.row_hi.animate(run_time=.1).set_value(0), mn.FadeIn(winter), seconds=3.6)
        self.step("ruler_5", self.ruler_away(ruler, header, False), mn.FadeOut(winter), self.swap_season("summer"), ruler.focus.animate(run_time=.1).set_value(ruler.scan.index("punched")),
                  mn.Succession(self.row_hi.animate(run_time=.1).set_value(2), mn.Wait(1.2), self.row_hi.animate(run_time=.1).set_value(3), mn.Wait(.9),
                                self.row_hi.animate(run_time=.1).set_value(4)), seconds=4.0)
        note = self.step_note(ruler, "−", ruler.best)
        self.step("shift", ruler.pan(2.0), mn.FadeIn(note), ruler.focus.animate(run_time=1.0).set_value(0), self.set_focus(ruler.scan[0], 1.0), self.row_hi.animate(run_time=.1).set_value(-1),
                  self.detail.animate(run_time=.1).set_value(0), seconds=2.2)
        note2 = self.step_note(ruler, "÷", round(ruler.high, ruler.digits_tag) - round(ruler.low, ruler.digits_tag))
        zero_one = _room.place(text(self.tr(f"0 = best, 1 = worst of the {len(pool)} compared variants", f"0 = Bester, 1 = Schlechtester der {len(pool)} verglichenen Varianten"), size=S0, color=MUTED),
                               (LEFT + .4, -.36, 0), "left")
        self.step("shift_2", ruler.stretch(2.0), mn.ReplacementTransform(note, note2), mn.FadeIn(zero_one), seconds=2.2)
        self.step("mapped", mn.FadeOut(note2), self.detail.animate(run_time=.1).set_value(1), ruler.visit([0, last], hold=.6, travel=.8), seconds=2.8)
        for key in ("cost", "daylight"):
            new, new_header, new_block, run = self.next_ruler(key, ruler, header, block, [zero_one] if zero_one is not None else [])
            sky = mn.VGroup(_room.place(_room.text(self.words(self.tr("sky light · in DF", "Himmelslicht · im TQ")), size=14, color=_room.DAYLIGHT), (2.15, 2.75, 0), "left"),
                            _room.place(_room.text(self.words(self.tr("direct sun · not in DF", "Direktsonne · nicht im TQ")), size=14, color=_room.SUN), (2.15, 2.45, 0), "left"))
            self.step(key, mn.Succession(*run), *([mn.FadeIn(sky)] if key == "daylight" else []), seconds=3.6)
            self.add(new.ring)
            ruler, header, block, zero_one = new, new_header, new_block, None
            if key == "cost":
                overlay = self.materials_overlay(("wall", "insulation", "window", "overhang"))
                self.step("cost_2", mn.FadeIn(overlay), ruler.focus.animate(run_time=.8).set_value(ruler.scan.index("ribbon")), self.set_focus("ribbon", .8),
                          mn.Succession(*[item for k in range(5) for item in (self.row_hi.animate(run_time=.1).set_value(k), mn.Wait(.7))]), seconds=4.4)
                self.row_hi.set_value(-1)
                self.overlay = overlay
        new, new_header, new_block, run = self.next_ruler("carbon", ruler, header, block, [sky], retire=[self.overlay])
        overlay = self.materials_overlay(("wall", "insulation", "window", "overhang"))
        self.step("carbon", mn.Succession(*run), mn.Succession(mn.Wait(1.2), mn.FadeIn(overlay)), seconds=4.4)
        board = Board(self, pool)
        board.cur.set_value(1.0)
        legend = self.table_legend()
        self._legend, self.board = legend, board
        moves, swaps = self.glide([new.members[i]["slot"] for i in pool], [board.labels[i] for i in pool])
        fold = mn.Succession(mn.AnimationGroup(mn.FadeOut(new.line), mn.FadeOut(new.ticks), mn.FadeOut(new_header), Retire(new_block), Retire(overlay), mn.FadeOut(new.ring),
                                               *[mn.FadeOut(new.members[i][k]) for i in pool for k in ("dot", "leader", "tag", "norm")], *moves,
                                               self.set_focus(board.scan[0], 1.2), self.active.animate(run_time=.1).set_value(-1)),
                             mn.AnimationGroup(*swaps, mn.FadeIn(board.live), mn.FadeIn(legend), board.fill.animate(run_time=2.0).set_value(1.0)))
        self.step("matrix", fold, seconds=5.0)
        self.ruler = new

    def winter_note(self):
        """❄️ What the winter picture means: conductance of wall and window, heat loss over the degree hours, the free heat that offsets it, and (toolkit derivation) demand ÷ COP."""
        rt = self._rt
        constant = _room.constant

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            conductance = found["u_wall"] * found["wall_area"] + constant("envelope", "u_window") * found["window_area"] + constant("envelope", "h_other")
            hours = constant("climate", "heating_degree_hours") / 1000
            loss = conductance * hours
            digits, _ = reproduce(lambda value: value * hours, [conductance], loss, 0, limit=3)
            parts = _room.apportion([found["u_wall"] * found["wall_area"], constant("envelope", "u_window") * found["window_area"], constant("envelope", "h_other")], digits, total=conductance)
            free = round(loss) - round(found["heating_demand"])
            group = mn.VGroup(self.equation([([self.tr("Winter (Zurich-like climate): sun low, heat leaves through wall and window", "Winter (Zürcher Klima): Sonne tief, Wärme geht durch Wand und Fenster verloren")], _room.HEAT)],
                                            LEFT + .4, 3.0, size=16, width=6.4),
                              self.equation([([self.tr("Conductance", "Leitwert"), "H", "=", self.n(parts[0], digits), "+", self.n(parts[1], digits), "+", self.n(parts[2], digits), "=", self.n(sum(parts), digits), "W/K"], INK)],
                                            LEFT + .4, 2.6, size=15, width=6.4),
                              self.equation([([self.tr("wall U", "Wand U"), self.n(found["u_wall"], 2), "·", self.tr("window U", "Fenster U"), self.n(constant("envelope", "u_window"), 1), "W/m²K", "·",
                                               self.tr("other", "Rest"), self.n(constant("envelope", "h_other")), "W/K"], MUTED)], LEFT + .4, 2.3, size=14, width=6.4),
                              self.equation([([self.tr("heat losses − used free heat = heating demand", "Wärmeverlust − genutzte freie Wärme = Heizbedarf")], MUTED)], LEFT + .4, 1.3, size=14, width=5.2),
                              self.equation([([self.tr("heating demand ÷ COP = heating", "Heizbedarf ÷ COP = Heizung")], MUTED)], LEFT + .4, 1.02, size=14, width=5.2))
            return group
        return live(build, _room.design_key(rt))

    def step_note(self, ruler, sign, value):
        """🧾 The operation of a normalization step, e.g. − best value with its unit."""
        number = _room.Number(sign + self.number(value, ruler.digits_tag)) if sign == "−" else self.n(value, ruler.digits_tag)
        label = _room.text([*([] if sign == "−" else [sign]), number, self.unit_label(ruler.criterion)], (ruler.center, -1.15, 0), size=S1, color=ruler.color)
        return label

    def ruler_header(self, ruler):
        """🏷️ Criterion pictogram, title, unit and direction."""
        c = ruler.criterion
        direction = self.tr("lower is better ↓", "weniger ist besser ↓") if CRITERIA[c]["direction"] == "min" else self.tr("higher is better ↑", "mehr ist besser ↑")
        title = TITLES[c][self.language() == "de"] if c in TITLES else CRITERIA[c]["title"][self.language()]
        items = mn.VGroup(glyph(c, width=.36), text(f"{title}, {self.unit_label(c)}", size=S1, color=ruler.color), text(direction, size=S1)).arrange(mn.RIGHT, buff=.22)
        return items.move_to([LEFT + items.width / 2, 3.34, 0])

    def next_ruler(self, criterion, old, header, block, extras, retire=()):
        """🔁 Replace the ruler by the next criterion; returns the new parts and the animations (flip first for daylight)."""
        pool = ids()
        new = Ruler(self, criterion, pool, pool)
        new_header = self.ruler_header(new)
        new_block = {"cost": self.block_cost, "daylight": self.block_daylight, "carbon": self.block_carbon}[criterion]()
        for member in old.members.values():
            member["slot"].clear_updaters()
        keep = ("dot", "leader", "tag", "norm")
        index = ORDER.index(criterion)
        swap = mn.AnimationGroup(
            *[mn.ReplacementTransform(old.members[i]["slot"], new.members[i]["slot"]) for i in pool],
            mn.FadeOut(old.line), mn.FadeOut(old.ticks), mn.FadeOut(old.ring), *[mn.FadeOut(e) for e in extras], *[Retire(r) for r in retire],
            *[mn.FadeOut(old.members[i][k]) for i in pool for k in keep], mn.ReplacementTransform(header, new_header),
            Retire(block), mn.FadeIn(new_block), mn.FadeIn(new.line), *[mn.FadeIn(new.members[i][k]) for i in pool for k in keep],
            new.appear(1.0), self.active.animate(run_time=.3).set_value(index), self.row_hi.animate(run_time=.1).set_value(-1))
        self.add(new.ticks)
        target = {"cost": new.scan[0], "daylight": new.scan[0], "carbon": "ribbon"}[criterion]
        run = [swap, mn.AnimationGroup(new.pan(1.0), self.set_focus(target, 1.0), new.focus.animate(run_time=1.0).set_value(new.scan.index(target))), new.stretch(1.0)]
        if CRITERIA[criterion]["direction"] == "max":
            run.insert(1, new.flip(1.6))
        return new, new_header, new_block, run

    def glide(self, slots, labels):
        """🛫 Move ruler slots onto their table rows, then cross-fade them into the row labels."""
        moves, swaps = [], []
        for slot, label in zip(slots, labels):
            slot.clear_updaters()
            moves.append(slot.animate(run_time=1.2).scale(.5).move_to(label.get_center()))
            swaps += [mn.FadeOut(slot), mn.FadeIn(label)]
        return moves, swaps

    def table_legend(self):
        """🧭 How to read the table: 0 is the best and 1 the worst of the compared set; the arrows of the column heads give the direction."""
        scale = mn.VGroup(text(self.tr("Normalized value:", "Normierter Wert:"), size=S0),
                          text(self.tr("0 = best", "0 = Bester"), size=S0, color=GOOD),
                          text(self.tr("1 = worst", "1 = Schlechtester"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.3)
        under = text(self.tr("Under each bar: normalized value, then physical value", "Unter jedem Balken: normiert, darunter der physikalische Wert"),
                     size=14, color=MUTED, width=6.8)
        return mn.VGroup(scale, under).arrange(mn.DOWN, buff=.1).move_to([-3.55, -2.3, 0])

    def screen_board(self):
        """⚖️ Weights, the index bar, its arithmetic, the winner's room and a cost-driven re-weighting; the focus row moves the room."""
        board = self.board
        ws = self._weights
        self.step("weights", board.mode.animate(run_time=1.2).set_value(1.0), seconds=1.4)
        self.step("weights_2", board.weights_to(ws, 2.5), seconds=2.6)
        wide, narrow = max(range(4), key=lambda j: ws[j]), min(range(4), key=lambda j: ws[j])
        outlines, tags = mn.VGroup(), mn.VGroup()
        for j, (shape, colour) in ((wide, (lambda r: r, GOOD)), (narrow, (lambda r: mn.DashedVMobject(r, num_dashes=40), WARN))):
            centre = board.xl + board.width * (sum(ws[:j]) + ws[j] / 2)
            top, bottom = board.y0 + .3, board.y0 - board.pitch * 4 - .7
            outlines.add(shape(mn.Rectangle(width=ws[j] * board.width + .1, height=top - bottom, color=colour, stroke_width=3).move_to([centre, (top + bottom) / 2, 0])))
            words = [self.tr("strongest pull", "stärkster Einfluss") if j == wide else self.tr("weakest pull", "schwächster Einfluss"), self.n(ws[j] * 100), "%"]
            chip = _room.chip(words, (centre, -2.5, 0), size=14, color=colour)
            chip.shift(np.array([(LEFT + .3 - chip.get_left()[0]) if j == wide else (FRAME_X1 - chip.get_right()[0]), 0.0, 0.0]))
            tags.add(chip)
        self.step("weights_3", mn.Create(outlines), mn.FadeOut(self._legend), mn.FadeIn(tags), board.visit(list(range(1, len(board.scan))) + [0], .3), seconds=2.6)
        axis = self.index_axis(board)
        self.step("index", mn.Succession(mn.AnimationGroup(mn.FadeOut(outlines), mn.FadeOut(tags), board.compact.animate(run_time=2.0).set_value(1.0), board.numbers.animate(run_time=1.0).set_value(0.0),
                                                           board.scores.animate(run_time=1.4).set_value(1.0), mn.FadeIn(axis)),
                                         board.segments()), seconds=4.8)
        key = tuple(round(v, 6) for v in board.weights())
        ranks = {i: row["rank"] for i, row in table(key).items()}
        winner = winner_of(key)
        self.step("index_2", board.rank_to(ranks, 2.0), board.badges.animate(run_time=1.2).set_value(1.0), board.win.animate(run_time=1.2).set_value(1.0),
                  board.cur.animate(run_time=1.6).set_value(0.0), board.visit([board.scan.index(winner)], 2.0), seconds=2.4)
        formula = self.block_index(lambda: board.cursor()[0], board.weights, y=-2.2)
        self.step("index_3", mn.FadeIn(formula), seconds=2.6)
        self.step("winner", self.pulse(getattr(self, self.season)[0], color=GOOD, run_time=1.4),
                  mn.Succession(mn.Succession(board.win.animate(run_time=.4).set_value(.25), board.win.animate(run_time=.6).set_value(1.0)), board.segments(.5, .3)), seconds=4.4)
        state = table(key)[winner]["normalized"]
        weakest = max(range(4), key=lambda j: state[ORDER[j]])
        cursor = board.xl + board.width * sum(ws[j] * state[ORDER[j]] for j in range(weakest))
        width = board.width * ws[weakest] * state[ORDER[weakest]]
        mark = mn.Rectangle(width=max(width, .06) + .08, height=.46, color=WARN, stroke_width=3).move_to([cursor + width / 2, board.y0, 0])
        name = SHORT[ORDER[weakest]][1 if self.language() == "de" else 0]
        note = mn.VGroup(glyph(ORDER[weakest], width=.28, color=WARN), text(self.tr(f"weakest = highest normalized value: {name} {self.number(state[ORDER[weakest]], 2)}",
                                                                                      f"schwächstes = höchster normierter Wert: {name} {self.number(state[ORDER[weakest]], 2)}"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.15).move_to([-3.4, -2.5, 0])
        self.step("winner_2", mn.Create(mark), mn.FadeIn(note), seconds=1.8)
        persona = mn.VGroup(glyph("cost", width=.24, color=WARN), text(self.tr("cost-driven client", "kostenorientierter Bauherr"), size=S1, color=WARN)).arrange(mn.RIGHT, buff=.2).move_to([-3.4, -2.48, 0])
        board.old_id = winner
        heavy_key = tuple(round(v, 6) for v in self._heavy)
        heavy_ranks = {i: row["rank"] for i, row in table(heavy_key).items()}
        new = winner_of(heavy_key)
        was = _room.chip(self.words(self.tr("was 1st", "war 1.")), (-1.4, board.y0 - board.pitch - .12, 0), size=S0, color=WARN)
        self.step("sensitivity", mn.FadeOut(mark), mn.FadeOut(note), mn.FadeIn(persona), board.weights_to(self._heavy, 3.0),
                  mn.Succession(mn.Wait(1.0), board.rank_to(heavy_ranks, 2.0)), board.win.animate(run_time=1.0).set_value(0.0), seconds=3.2)
        self.step("sensitivity_2", board.old.animate(run_time=1.0).set_value(1.0), mn.FadeIn(was), seconds=1.8)
        self.step("sensitivity_3", board.win.animate(run_time=1.0).set_value(1.0), mn.FadeOut(was), board.visit([board.scan.index(new)], 1.5), seconds=1.8)
        self.persona = persona

    def index_axis(self, board):
        """📐 Scale under the bars: 0 is the best and 1 the worst of the compared set in every criterion."""
        y = board.y0 - (len(board.all) - 1) * board.pitch - .36
        line = mn.Line([board.xl, y, 0], [board.xl + board.width, y, 0], color=INK, stroke_width=2)
        ticks = mn.VGroup(*[mn.VGroup(mn.Line([board.xl + board.width * f, y - .06, 0], [board.xl + board.width * f, y + .06, 0], color=INK, stroke_width=2),
                                      text({0: "0", 1: "1"}.get(f, self.number(f, 1)), [board.xl + board.width * f, y - .22, 0], size=S0)) for f in (0, .5, 1)])
        return mn.VGroup(line, ticks, _room.place(text(self.tr("Index", "Index"), size=S1, color=INK), (board.xl - .15, y, 0), "right"))

    def switch_shares(self, base):
        """🔀 The cost weights (in percent) at which the winner switches, found by bisection on the model's index."""
        champion = lambda share: winner_of(tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER))
        found, previous = [], champion(0)
        for k in range(1, 401):
            current = champion(k * .25)
            if current != previous:
                low, high = (k - 1) * .25, k * .25
                for _ in range(24):
                    middle = (low + high) / 2
                    low, high = (middle, high) if champion(middle) == previous else (low, middle)
                found.append(high)
                previous = current
        return found

    def close_bands(self, base, limit=.03):
        """🤏 The cost weights (in percent) between which the top two index values lie closer than ``limit``: exact edges by bisection on the gap."""
        def gap(share):
            scores = sorted(row["score"] for row in table(tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER)).values())
            return scores[1] - scores[0] - limit
        grid = [k * .25 for k in range(401)]
        flags = [gap(share) < 0 for share in grid]
        bands, start = [], None
        for k, flag in enumerate(flags + [False]):
            if flag and start is None:
                start = k
            if not flag and start is not None:
                low, high = grid[start], grid[k - 1]
                if start > 0:
                    a, b = grid[start - 1], grid[start]
                    for _ in range(24):
                        middle = (a + b) / 2
                        a, b = (a, middle) if gap(middle) < 0 else (middle, b)
                    low = (a + b) / 2
                if k <= 400:
                    a, b = grid[k - 1], grid[k]
                    for _ in range(24):
                        middle = (a + b) / 2
                        a, b = (middle, b) if gap(middle) < 0 else (a, middle)
                    high = (a + b) / 2
                bands.append((low, high))
                start = None
        return bands

    def screen_sweep(self):
        """🔀 Rank of every variant against the cost weight, with the winner strip, the switch shares and the room of the current winner."""
        self.clear_stage()
        pool = ids()
        shares = [2.5 * k for k in range(41)]
        base = _model.load_study()["weights"]["balanced"]
        ranks = {i: [] for i in pool}
        for share in shares:
            weights = tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER)
            rows = table(weights)
            for i in pool:
                ranks[i].append(rows[i]["rank"])
        x0, x1, top, step = -4.8, -1.9, 1.7, .66
        X = lambda share: x0 + (x1 - x0) * share / 100
        Y = lambda rank: top - (rank - 1) * step
        static = mn.VGroup()
        for rank in range(1, 6):
            static.add(mn.Line([x0, Y(rank), 0], [x1, Y(rank), 0], color=MUTED, stroke_width=1, stroke_opacity=.35), text(str(rank), (x0 - .2, Y(rank), 0), size=S0, color=MUTED))
        static.add(mn.Line([x0, Y(5) - .4, 0], [x1, Y(5) - .4, 0], color=INK, stroke_width=2))
        for share in (0, 25, 50, 75, 100):
            static.add(mn.Line([X(share), Y(5) - .46, 0], [X(share), Y(5) - .34, 0], color=INK, stroke_width=2), text(str(share), (X(share), Y(5) - .64, 0), size=S0))
        static.add(text(self.tr("rank", "Rang"), (x0 - .55, Y(5) - .64, 0), size=S0, color=MUTED),
                   _room.place(text(self.tr("cost weight (%)", "Kostengewicht (%)"), size=S0, color=MUTED, width=1.5), (x1 + .3, Y(5) - .64, 0), "left"),
                   _room.place(text(self.tr("at 0 %", "bei 0 %"), size=S0, color=MUTED), (x0 - .45, Y(1) + .75, 0), "right"),
                   _room.place(text(self.tr("at 100 %", "bei 100 %"), size=S0, color=MUTED), (x1 + .3, Y(1) + .75, 0), "left"))
        lines_ = mn.VGroup(*[mn.VMobject(color=INK, stroke_width=2.4, stroke_opacity=.55).set_points_as_corners([[X(s), Y(r), 0] for s, r in zip(shares, ranks[i])]) for i in pool])
        names = mn.VGroup()
        for i in pool:
            names.add(_room.place(self.name(i, size=14, width=1.7, color=VARIANT_COLOR[i] if ranks[i][0] == 1 else INK), (x0 - .4, Y(ranks[i][0]), 0), "right"),
                      _room.place(self.name(i, size=14, width=1.55, color=VARIANT_COLOR[i] if ranks[i][-1] == 1 else INK), (x1 + .3, Y(ranks[i][-1]), 0), "left"))
        strip, winners = mn.VGroup(), [next(i for i in pool if ranks[i][k] == 1) for k in range(len(shares))]
        half = (x1 - x0) / (len(shares) - 1) / 2
        for winner, first, last in runs_of(winners):
            a, b = max(x0, X(shares[first]) - half), min(x1, X(shares[last]) + half)
            strip.add(mn.Line([a + .02, Y(1) + .16, 0], [b - .02, Y(1) + .16, 0], color=VARIANT_COLOR[winner], stroke_width=6),
                      self.thumbnail(by_id(winner)["design"], ((a + b) / 2, Y(1) + .52, 0), .075, ramp=self._ramp, value=self._ramp.low, ring=VARIANT_COLOR[winner]))
        for crossing in self.switch_shares(base):
            strip.add(mn.DashedLine([X(crossing), Y(1) + .16, 0], [X(crossing), Y(5) - .4, 0], color=GOOD, stroke_width=2, stroke_opacity=.7),
                      _room.chip([_room.Number(self.number(crossing, 0)), "%"], (X(crossing), Y(1) + .88, 0), size=S0, color=GOOD))
        close = mn.VGroup()
        bands = self.close_bands(base)
        for low, high in bands:
            close.add(mn.Rectangle(width=X(high) - X(low), height=Y(1) - Y(5) + .5, stroke_width=0, fill_color=WARN, fill_opacity=.2).move_to([(X(low) + X(high)) / 2, (Y(1) + Y(5)) / 2, 0]))
        key = mn.VGroup(mn.Rectangle(width=.3, height=.22, stroke_width=1.4, color=WARN, fill_color=WARN, fill_opacity=.3),
                        text(self.tr("close call: index gap below 0.03", "knapp: Indexabstand unter 0,03"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.15)
        key.move_to([-3.4, -2.35, 0])
        close.add(key)
        close_share = (bands[-1][0] + bands[-1][1]) / 2 if bands else 50
        share = mn.ValueTracker(self._heavy[ORDER.index("cost")] * 100)

        def champion():
            weights = tuple(_model.reweigh(base, "cost", share.get_value() / 100)[c] for c in ORDER)
            return winner_of(weights)
        for name in ("orientation", "wwr", "overhang", "insulation"):
            self._rt[name].add_updater(lambda m, name=name: m.set_value(by_id(champion())["design"][name]))
        self.focus.add_updater(lambda m: m.set_value(ids(True).index(champion())))
        self.add(share, *self._rt.values(), self.focus)

        def marker():
            weights = tuple(_model.reweigh(base, "cost", share.get_value() / 100)[c] for c in ORDER)
            rows = table(weights)
            x = X(share.get_value())
            group = mn.VGroup(mn.Line([x, Y(1) + .3, 0], [x, Y(5) - .4, 0], color=INK, stroke_width=2.4))
            for i in pool:
                group.add(mn.Dot([x, Y(rows[i]["rank"]), 0], radius=.09, color=GOOD if rows[i]["rank"] == 1 else INK))
            chip = _room.chip([self.tr("cost weight", "Kostengewicht"), _room.Number(self.number(share.get_value(), 0)), "%"], (x, Y(5) - .98, 0), size=S0, color=COLORS["cost"])
            chip.shift(mn.RIGHT * (max(0, x0 - .6 - chip.get_left()[0]) - max(0, chip.get_right()[0] - (x1 + .6))))
            group.add(chip)
            return group
        self.step("sweep", mn.FadeIn(static), mn.Create(lines_), mn.FadeIn(names), mn.FadeIn(strip), mn.FadeIn(live(marker)),
                  mn.Succession(mn.Wait(.8), share.animate(run_time=1.4).set_value(0), share.animate(run_time=2.0).set_value(100), share.animate(run_time=1.0).set_value(self._heavy[ORDER.index("cost")] * 100)), seconds=5.4)
        self.step("sweep_2", mn.FadeIn(close), share.animate(run_time=1.0).set_value(close_share), seconds=1.4)
        self.sweep_share = share

    def screen_range(self):
        """➕ A sixth variant stretches the cost ruler, squeezes the others and flips the order of two of them."""
        self.clear_stage()
        for tracker in (*self._rt.values(), self.focus):
            tracker.clear_updaters()
        five, six = ids(), ids(True)
        extra = six[-1]
        ruler = Ruler(self, "cost", six, five, state="normalized", name_size=13, rings=False)
        header = self.ruler_header(ruler)
        block = self.block_cost()
        ruler.scan = ruler.scan + [extra]
        ruler.focus.set_value(len(ruler.scan) - 1)
        worst_old = max(five, key=lambda i: ruler.values[i])
        frames = {"best": self.slot_frame(ruler, ruler.scan[0], GOOD), "old": self.slot_frame(ruler, worst_old, WARN), "new": self.slot_frame(ruler, extra, WARN)}
        ghosts = mn.VGroup(*[mn.DashedVMobject(mn.Circle(radius=.09, color=MUTED, stroke_width=2).move_to([ruler.x_of(ruler.values[i]), ruler.line_y, 0]), num_dashes=10) for i in five])
        hint = text(self.tr(f"{self.number(ruler.fraction(extra), 2)} > 1: beyond the old range", f"{self.number(ruler.fraction(extra), 2)} > 1: außerhalb des alten Bereichs"),
                    (ruler.x0 + ruler.length - 1.6, -.35, 0), size=S0, color=WARN)
        ruler.ring.update()
        self.add(ruler.ticks, ruler.ring, *frames.values())
        self.step("range", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(five)), mn.FadeIn(ghosts), frames["best"].level.animate.set_value(1.0), frames["old"].level.animate.set_value(1.0),
                  self.set_focus(extra, .8), self.active.animate(run_time=.3).set_value(1),
                  mn.Succession(mn.Wait(.8), mn.FadeIn(ruler.mobjects([extra])), mn.AnimationGroup(mn.FadeIn(block), mn.FadeIn(hint), ruler.tween(.6, o_ring=1.0))), seconds=3.0)
        span = max(ruler.values[i] for i in six) - min(ruler.values[i] for i in six)
        elec = Ruler(self, "electricity", six, six, state="normalized", name_size=13)
        elec_header = self.ruler_header(elec)
        self.add(elec.ticks)
        to_elec = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(ghosts), mn.FadeOut(header),
                                    *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("slot", "dot", "leader", "tag", "norm")], *[frame.level.animate(run_time=.5).set_value(0.0) for frame in frames.values()],
                                    mn.FadeIn(elec.line), mn.FadeIn(elec_header), *[mn.FadeIn(elec.members[i][k]) for i in six for k in ("slot", "dot", "leader", "norm")],
                                    self.active.animate(run_time=.3).set_value(0))
        self.step("range_2", mn.Succession(mn.AnimationGroup(ruler.stretch(2.5, span=span), frames["old"].level.animate(run_time=1.2).set_value(0.0), frames["new"].level.animate(run_time=1.2).set_value(1.0), mn.FadeOut(hint)),
                                           mn.Wait(.5), to_elec), seconds=6.0)
        ruler, header = elec, elec_header
        self.ruler = ruler
        order = sorted(five, key=lambda i: table(self._weights)[i]["rank"])
        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.54, y0=2.2, name_size=13)
        board.header.set_value(0.0)
        for tracker in (board.fill, board.compact, board.scores, board.badges, board.win, board.mode):
            tracker.set_value(1.0)
        board.numbers.set_value(0.0)
        new_ranks = {i: row["rank"] for i, row in table(self._weights, True).items()}
        old_ranks = {i: row["rank"] for i, row in table(self._weights).items()}
        axis = self.index_axis(board)
        notes = mn.VGroup()
        for i in six:
            before = str(old_ranks[i]) if i in old_ranks else self.tr("new", "neu")
            note = text(f"{before} → {new_ranks[i]}", (-1.6, 0, 0), size=S1, color=WARN if i in old_ranks and old_ranks[i] != new_ranks[i] else INK)
            note.identifier = i
            note.add_updater(lambda m: (m.move_to([-.98 - m.width / 2, board.y(m.identifier), 0]), m.set_opacity(board.row_o[m.identifier].get_value())))
            notes.add(note)
        flipped = [i for i in five if (sorted(five, key=old_ranks.get).index(i) != sorted([f for f in six if f in five], key=new_ranks.get).index(i))]
        heading = _room.place(text(self.tr("rank old → new (orange: changed)", "Rang alt → neu (orange: geändert)"), size=S0, color=MUTED), (LEFT + .4, -1.35, 0), "left")
        link = None
        board_parts = mn.VGroup(board.live, axis, notes, heading)
        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(header), Retire(block), mn.FadeOut(ruler.members[extra]["slot"]),
                                *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag", "norm")], self.active.animate(run_time=.3).set_value(0))
        moves, swaps = self.glide([ruler.members[i]["slot"] for i in five], [board.labels[i] for i in five])
        fold = mn.Succession(mn.AnimationGroup(*moves), mn.AnimationGroup(*swaps, mn.FadeIn(board_parts)))
        reorder = mn.AnimationGroup(board.t.animate(run_time=1.6).set_value(1.0), board.row_o[extra].animate(run_time=1.6).set_value(1.0),
                                    board.rank_to(new_ranks, 2.0), mn.FadeIn(board.labels[extra]))
        steps = [out, fold, reorder] + ([mn.Create(link, run_time=.7)] if link is not None else [])
        self.step("range_3", mn.Succession(*steps), seconds=5.4)
        self.final_board = board

    def slot_frame(self, ruler, identifier, color):
        """🔲 A frame that follows a thumbnail on the ruler; its opacity is the ``level`` tracker."""
        frame = mn.Rectangle(width=.95, height=.78, color=color, stroke_width=2.4)
        frame.level = mn.ValueTracker(0.0)
        frame.add_updater(lambda m: (m.move_to(ruler.members[identifier]["slot"][0].get_center()), m.set_stroke(opacity=m.level.get_value())))
        frame.update()
        return frame

    def screen_record(self):
        """🗂️ The decision record, the budget limit, the bridge to Pareto fronts and the answer."""
        self.clear_stage()
        pool, key = ids(), self._weights
        rows = table(key)
        winner = winner_of(key)
        left = lambda item, x, y: _room.place(item, (x, y, 0), "left")
        card = _art.window_frame(.1 - LEFT, 5.75, color=INK).move_to([(LEFT + .1) / 2, .4, 0])
        parts = mn.VGroup()
        heading = lambda label, x, y: left(text(label, size=S1, color=MUTED), x, y)
        parts.add(heading(self.tr("Weights", "Gewichte"), LEFT + .3, 3.0))
        for k, c in enumerate(ORDER):
            line = mn.VGroup(glyph(c, width=.26), text(f"{SHORT[c][1 if self.language() == 'de' else 0]} {self.number(key[k] * 100)} %", size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.12)
            parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, 2.6 - .38 * (k // 2)))
        parts.add(heading(self.tr("Compared variants", "Verglichene Varianten"), LEFT + .3, 1.65))
        for k, i in enumerate(pool):
            x = LEFT + .72 + 1.36 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, 1.0, 0), .11, ramp=self._ramp, value=self._ramp.low), block("\n".join(stack(by_id(i)["title"][self.language()])), size=13, width=1.3).move_to([x, .3, 0]))
        parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), LEFT + .3, -.45))
        for k, c in enumerate(ORDER):
            best = best_id(c, pool)
            worst = max(pool, key=lambda i: raw(i, c) * (1 if CRITERIA[c]["direction"] == "min" else -1))
            line = mn.VGroup(glyph(c, width=.26), text(f"{self.value_text(c, raw(best, c))} … {self.value_text(c, raw(worst, c))} {self.unit_label(c)}", size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.12)
            parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, -.9 - .4 * (k // 2)))
        parts.add(heading(self.tr("Result", "Ergebnis"), LEFT + .3, -1.75),
                  self.thumbnail(by_id(winner)["design"], (LEFT + 1.0, -2.15, 0), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), LEFT + 1.5, -2.15),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), LEFT + 4.3, -2.15))
        record = mn.VGroup(card, parts)
        self.step("record", mn.FadeIn(record), self.set_focus(winner, .8), self.active.animate(run_time=.3).set_value(-1), seconds=2.5)
        self.screen_limits(record, winner)
        self.clear_stage()
        pool = ids()
        costs, powers = [raw(i, "cost") for i in pool], [raw(i, "electricity") for i in pool]
        xs, ys = 500, nice_step(max(powers) - min(powers), 4)
        x_range = (math.floor(min(costs) / 1000) * 1000, math.ceil(max(costs) / 1000) * 1000 + 500, xs)
        y_range = (math.floor(min(powers) / ys - .3) * ys, math.ceil(max(powers) / ys + .3) * ys, ys)
        axes, labels = self.plot_axes(x_range=x_range, y_range=y_range, x_label=self.tr("Façade cost, CHF", "Fassadenkosten, CHF"),
                                      y_label=self.tr(f"Electricity intensity, {_room.INTENSITY_UNIT}", f"Strombedarf pro m², {_room.INTENSITY_UNIT}"), point=(-3.5, .35, 0), width=5.6, height=3.4,
                                      y_digits=0 if ys >= 1 else 1, x_ticks=[19000, 20000, 21000, 22000])
        free = [i for i in pool if not any(raw(j, "cost") <= raw(i, "cost") and raw(j, "electricity") <= raw(i, "electricity") and j != i for j in pool)]
        marks = mn.VGroup(*[self.thumbnail(by_id(i)["design"], axes.c2p(raw(i, "cost"), raw(i, "electricity")), .1, ramp=self._ramp, value=self._ramp.low, ring=GOOD if i in free else None) for i in pool])
        tags = mn.VGroup()
        dominator = by_id(winner)["title"][self.language()]
        for i in pool:
            spot = axes.c2p(raw(i, "cost"), raw(i, "electricity"))
            if i in free:
                tags.add(self.name(i, size=13, width=1.5, limit=14).move_to(spot + mn.DOWN * .55))
                continue
            tags.add(_room.place(self.name(i, size=13, width=1.5, limit=14, color=MUTED).move_to(spot), spot + mn.RIGHT * .45, "left"))
        note = _room.place(_room.text(self.words(self.tr("grey = dominated by", "grau = dominiert von")) + self.words(dominator), size=14, color=WARN), (LEFT + .3, 3.0, 0), "left")
        front = sorted((raw(i, "cost"), raw(i, "electricity")) for i in free)
        outline = mn.DashedVMobject(mn.VMobject(color=GOOD, stroke_width=3).set_points_as_corners([axes.c2p(*p) for p in front]), num_dashes=24)
        chip = _art.window_chip(text(self.tr("Pareto front", "Pareto-Front"), size=S2, color=GOOD), color=GOOD).move_to([-1.2, 3.0, 0])
        self.step("pointer", mn.FadeIn(axes), mn.FadeIn(labels), mn.FadeIn(marks), mn.FadeIn(tags), mn.Create(outline), mn.FadeIn(chip), mn.FadeIn(note), self.active.animate(run_time=.3).set_value(0), seconds=2.6)
        self.screen_answer(winner)

    def screen_answer(self, winner):
        """🏁 The answer: the winner's index written out criterion by criterion, and the electricity behind its first value, next to its room."""
        self.clear_stage()
        title = mn.VGroup(self.thumbnail(by_id(winner)["design"], (0, 0, 0), .2, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                          self.name(winner, size=S2, width=3.4, color=GOOD, limit=40),
                          _art.window_chip(text(self.tr("lowest index of five", "kleinster Index von fünf"), size=S1, color=GOOD), color=GOOD)).arrange(mn.RIGHT, buff=.3).move_to([-3.5, 3.15, 0])
        table_ = self.block_index(lambda: winner, lambda: self._weights, full=True)
        self.detail.set_value(1.0)
        loss = self.heat_loss_arrows()
        legend = mn.VGroup(_room.place(_room.text([self.tr("winter heating demand", "Winter: Heizbedarf"), self.n(_room.result(by_id(winner)["design"])["heating_demand"]), "kWh/a"], size=13, color=_room.HEAT), (2.05, 2.5, 0), "left"),
                           _room.place(_room.text([self.tr("summer cooling", "Sommer: Kühllast"), self.n(_room.result(by_id(winner)["design"])["cooling_demand"]), "kWh/a"], size=13, color=_room.COOL), (2.05, 2.2, 0), "left"))
        self.step("next", mn.FadeIn(title), mn.FadeIn(table_), mn.FadeIn(self.block_electricity()), mn.FadeIn(loss), mn.FadeIn(legend), self.active.animate(run_time=.3).set_value(0), seconds=3.0)

    def screen_limits(self, record, winner):
        """🚧 The client's budget is a mandatory limit: it removes variants before any weight is applied; the allowed ones are normalized and ranked again."""
        pool = ids()
        costs = {i: raw(i, "cost") for i in pool}
        over = [i for i in pool if costs[i] > BUDGET_CHF]
        allowed = tuple(i for i in pool if i not in over)
        rows = table(self._weights, pool=allowed)
        low, high = math.floor(min(costs.values()) / 1000) * 1000, math.ceil(max(costs.values()) / 1000) * 1000
        xl, xr, y_axis = -6.0, -.7, .7
        X = lambda value: xl + (value - low) / (high - low) * (xr - xl)
        flow = mn.VGroup(_art.window_chip(text(self.tr("Constraints: filter", "Nebenbedingungen: filtern"), size=S1, color=WARN), color=WARN), mn.Arrow(mn.LEFT * .35, mn.RIGHT * .35, buff=0, color=INK),
                         _art.window_chip(text(self.tr("Weights: rank", "Gewichte: ordnen"), size=S1, color=GOOD), color=GOOD)).arrange(mn.RIGHT, buff=.3)
        flow.move_to([-3.5, 3.25, 0])
        axis = mn.VGroup(mn.Line([xl - .15, y_axis, 0], [xr + .15, y_axis, 0], color=INK, stroke_width=2.4), text("CHF", (xr + .5, y_axis, 0), size=S0, color=COLORS["cost"]))
        for value in range(low, high + 1, 1000):
            axis.add(mn.Line([X(value), y_axis - .07, 0], [X(value), y_axis + .07, 0], color=INK, stroke_width=2))
            if value != BUDGET_CHF:
                axis.add(text(self.number(value), (X(value), y_axis + .3, 0), size=S0))
        order = sorted(pool, key=costs.get)
        gap = (xr - xl + .4) / (len(order) - 1)
        slots = {i: xl - .2 + k * gap for k, i in enumerate(order)}
        dots, groups = {}, mn.VGroup()
        for k, i in enumerate(order):
            thumb = self.thumbnail(by_id(i)["design"], (slots[i], 2.0, 0), .11, ramp=self._ramp, value=self._ramp.low)
            name = self.name(i, size=13, width=1.6, limit=14).move_to([slots[i], 2.7, 0])
            dots[i] = mn.Dot([X(costs[i]), y_axis, 0], radius=.075, color=COLORS["cost"])
            leader = mn.Line([slots[i], 1.68, 0], [X(costs[i]), y_axis + .08, 0], color=COLORS["cost"], stroke_width=1.6, stroke_opacity=.7)
            tag = text(self.number(costs[i]), (X(costs[i]), y_axis - .3 - .28 * (k % 2), 0), size=S0, color=INK)
            groups.add(thumb, name, dots[i], leader, tag)
        zone_x = X(BUDGET_CHF)
        zone = mn.Rectangle(width=xr + .15 - zone_x, height=1.15, stroke_width=0, fill_color=WARN, fill_opacity=.16).move_to([(zone_x + xr + .15) / 2, y_axis - .15, 0])
        limit = mn.VGroup(mn.DashedLine([zone_x, y_axis + .45, 0], [zone_x, y_axis - .16, 0], color=WARN, stroke_width=3),
                          _room.chip([self.tr("client budget", "Budget des Bauherrn"), _room.Number(self.number(BUDGET_CHF)), "CHF"], (zone_x, y_axis - 1.1, 0), size=14, color=WARN),
                          text(self.tr("not allowed", "nicht erlaubt"), ((zone_x + xr + .15) / 2 + .35, y_axis - .62, 0), size=S0, color=WARN))
        crosses = mn.VGroup(*[_art.cross_mark(color=BAD, size=.2).move_to([slots[i], 2.0, 0]) for i in over])
        note = _room.place(text(self.tr("normalized over the allowed designs", "normiert über die erlaubten Entwürfe"), size=14, color=MUTED), (LEFT, -1.0, 0), "left")
        rank_rows = mn.VGroup(note)
        passing = sorted(allowed, key=lambda i: rows[i]["score"])
        for k, i in enumerate(passing):
            y = -1.4 - .42 * k
            thumb = self.thumbnail(by_id(i)["design"], (-6.5, y, 0), .08, ramp=self._ramp, value=self._ramp.low)
            name = self.name(i, size=13, width=1.6, limit=18).move_to([-5.45, y, 0])
            cursor, bar = -4.2, mn.VGroup()
            for j, c in enumerate(ORDER):
                part = 1.9 * self._weights[j] * rows[i]["normalized"][c]
                bar.add(mn.Rectangle(width=max(part, 1e-3), height=.28, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85).move_to([cursor + part / 2, y, 0]))
                cursor += part
            score = _room.place(text(self.number(rows[i]["score"], 2), size=S1), (cursor + .12, y, 0), "left")
            price = _room.place(text(f"{self.number(costs[i])} CHF", size=S0, color=COLORS["cost"]), (-1.55, y, 0), "left")
            rank_rows.add(thumb, name, bar, score, price)
        frame = mn.Rectangle(width=RIGHT - LEFT, height=.4, color=GOOD, stroke_width=3).move_to([(LEFT + RIGHT) / 2, -1.4, 0])
        verdict = _room.chip([_room.Number(self.number(costs[winner])), "CHF", "<", _room.Number(self.number(BUDGET_CHF)), "CHF"], (RIGHT - 1.4, -1.0, 0), size=14, color=GOOD)
        self.step("limits", mn.FadeOut(record), mn.Succession(
            mn.AnimationGroup(mn.FadeIn(flow), mn.FadeIn(axis), mn.FadeIn(groups), self.active.animate(run_time=.3).set_value(1)),
            mn.AnimationGroup(mn.FadeIn(zone), mn.Create(limit[0]), mn.FadeIn(limit[1]), mn.FadeIn(limit[2])),
            mn.AnimationGroup(mn.Create(crosses), *[dots[i].animate.set_color(BAD) for i in over])), seconds=4.4)
        self.step("limits_2", mn.AnimationGroup(mn.FadeIn(rank_rows), mn.Create(frame), mn.FadeIn(verdict)), seconds=2.8)
