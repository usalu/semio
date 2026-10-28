"""🏠 Drawing vocabulary of the seminar-room design study: section, elevation, plan, bars, thumbnails and heat maps."""

from __future__ import annotations

import importlib
import math
from dataclasses import dataclass
from functools import lru_cache

import numpy as np
from PIL import Image

_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_engine = importlib.import_module("manim")

INK, GRAY = _art.P_WHITE, _art.PASTEL_GREY
HEAT, COOL, LIGHT, SUN = _art.P_RED, _art.P_BLUE, _art.P_YELLOW, _art.P_YELLOW
GLASS, INSULATION, GOOD, WARN = _art.P_CYAN, _art.P_GREEN, _art.P_GREEN, _art.P_ORANGE
KEYS = ("orientation", "wwr", "overhang", "insulation")
WIDTH, DEPTH, HEIGHT, SLAB, MASONRY, HEAD = 8.0, 6.0, 3.0, 0.3, 0.2, 2.7
RAMP_STOPS = ("#0b4f58", _art.P_TEAL, _art.P_WHITE)
REFERENCE_SIZE = 28
DIGITS = "0123456789,.−"
WORK = 0.85
DAYLIGHT = _engine.interpolate_color(_engine.ManimColor(SUN), _engine.ManimColor(INK), .55).to_hex()
SKY = _engine.interpolate_color(_engine.ManimColor(_art.P_CYAN), _engine.ManimColor(INK), .6).to_hex()
DEMAND_EDGE, DIMENSION_SIZE, DEMAND_FULL_SCALE = 6.95, 14, 0.34
SKY_ELEVATIONS = (15, 35, 55, 75)
INTENSITY_UNIT = "kWh/m²a"


class Number(str):
    """🔢 A part of a label that changes while it is on screen: it is assembled from cached digit glyphs."""


@dataclass(frozen=True)
class Live:
    """📟 One live number of a formula row: ``source`` is a model key, a constant or ``fn(design, found) -> float``, shown ``digits`` decimals × ``scale`` in ``color`` with ``unit``.

    ``parts_of`` (model keys, ``source`` among them) shows the largest-remainder rounding of those keys, so printed parts add up to the printed total.
    """

    source: object
    color: str = INK
    digits: int = 0
    unit: str = ""
    scale: float = 1.0
    parts_of: tuple = ()


@lru_cache(maxsize=None)
def _proto(string: str):
    return _fonts.body_text(string, font_size=REFERENCE_SIZE, color=INK)


@lru_cache(maxsize=1)
def _glyphs():
    pair = _proto("00")
    advance = pair[1].get_left()[0] - pair[0].get_left()[0]
    zero = pair[0]
    table = {"0": (zero.points.copy() - np.array([zero.get_left()[0], zero.get_bottom()[1], 0.0]), 0.0, 0.0, advance)}
    for char in DIGITS[1:]:
        before, glyph, after = _proto("0" + char + "0").submobjects
        table[char] = (glyph.points.copy() - np.array([glyph.get_left()[0], glyph.get_bottom()[1], 0.0]),
                       glyph.get_left()[0] - before.get_left()[0] - advance, glyph.get_bottom()[1] - before.get_bottom()[1],
                       after.get_left()[0] - before.get_left()[0] - advance)
    spaced = _proto("0 0")
    return table, spaced[2].get_left()[0] - spaced[0].get_left()[0] - advance, advance


@lru_cache(maxsize=None)
def _word(word: str):
    proto = _proto("0" + word)
    zero, rest = proto.submobjects[0], proto.submobjects[1:]
    origin = np.array([zero.get_left()[0] + _glyphs()[2], zero.get_bottom()[1], 0.0])
    return [glyph.points.copy() - origin for glyph in rest if len(glyph.points)], rest[-1].get_right()[0] - origin[0] + 0.05


class LiveText(_engine.Text):
    """🏷️ A label drawn as one shape from cached glyph outlines; it is a ``Text`` with fill opacity 1, which is what ``layout_conflicts`` needs to see it at all (it skips invisible texts)."""

    def __init__(self, points, color, text=""):
        _engine.VMobject.__init__(self, fill_color=color, fill_opacity=1, stroke_width=0)
        shape = _engine.VMobject(fill_color=color, fill_opacity=1, stroke_width=0)
        shape.points = points
        self.add(shape)
        self.text = self.original_text = text


class RasterCamera(_engine.Camera):
    """🖼️ Camera that composites image mobjects on whole pixels: Manim's own path fails with the installed Pillow and blurs seams between neighbours."""

    def display_image_mobject(self, image_mobject, pixel_array):
        scale = np.array([self.pixel_width / self.frame_width, -self.pixel_height / self.frame_height])
        corners = (image_mobject.points[:, :2] - self.frame_center[:2]) * scale + np.array([self.pixel_width, self.pixel_height]) / 2
        left, top = (int(v) for v in np.rint(corners.min(axis=0)))
        right, bottom = (int(v) for v in np.rint(corners.max(axis=0)))
        x0, y0, x1, y1 = max(left, 0), max(top, 0), min(right, self.pixel_width), min(bottom, self.pixel_height)
        if right <= left or bottom <= top or x0 >= x1 or y0 >= y1:
            return
        source = Image.fromarray(image_mobject.get_pixel_array())
        if source.size != (right - left, bottom - top):
            source = source.resize((right - left, bottom - top), image_mobject.resampling_algorithm)
        source = source.crop((x0 - left, y0 - top, x1 - left, y1 - top))
        region = Image.fromarray(np.ascontiguousarray(pixel_array[y0:y1, x0:x1]))
        pixel_array[y0:y1, x0:x1] = np.asarray(Image.alpha_composite(region, source), dtype="uint8")


class HeatStrip(_engine.ImageMobject):
    """🧱 A raster strip that answers the vector opacity calls (``set_fill``, ``set_stroke``, ``get_fill_opacity``) the scenes use for fades."""

    def set_opacity(self, alpha, family=True):
        super().set_opacity(alpha)
        self.fill_opacity = alpha
        return self

    def set_fill(self, color=None, opacity=None, family=True):
        return self if opacity is None else self.set_opacity(opacity)

    def set_stroke(self, color=None, width=None, opacity=None, background=False, family=True):
        return self

    def get_fill_opacity(self):
        return self.fill_opacity

    def get_stroke_opacity(self):
        return self.stroke_opacity


_STALE = object()


class LiveGroup(_engine.VGroup):
    """♻️ A widget rebuilt from its builder every frame, or only when its ``key`` snapshot changes; safe for every animation.

    While an animation runs on it the children stay frozen; the rebuild after the animation is deferred to the next frame, so ``FadeOut`` cleanup and
    every ``interpolate`` see the structure they started with. Copies (used by ``FadeIn``, ``FadeOut``, ``Transform``, ``Indicate``) are fresh static snapshots.
    With a ``key`` (a cheap callable returning a hashable snapshot of everything the builder reads) unchanged frames reuse the children; every finished
    animation invalidates the snapshot, so a widget that was transformed is rebuilt exactly once afterwards.
    """

    def __init__(self, builder, key=None):
        super().__init__(*builder().submobjects)
        self._builder, self._deferred, self._key, self._seen = builder, False, None, _STALE
        self.add_updater(lambda mob: mob.refresh())
        if key is not None:
            self.memoize(key)

    def memoize(self, key):
        """🧠 Rebuild only when ``key()`` differs from the snapshot of the last build."""
        self._key, self._seen = key, key()
        return self

    def refresh(self):
        """🔄 Rebuild the children from the current inputs unless nothing changed or an animation has just finished."""
        if self._builder is None or self._deferred:
            return self
        if self._key is not None:
            seen = self._key()
            if seen == self._seen:
                return self
            self._seen = seen
        self.submobjects = self._builder().submobjects
        return self

    def resume_updating(self, recursive=True):
        """▶️ Resume without rebuilding in the middle of an animation's finish."""
        self._deferred = True
        try:
            return super().resume_updating(recursive)
        finally:
            self._deferred = False
            self._seen = _STALE

    def __deepcopy__(self, memo):
        self.refresh()
        twin = super().__deepcopy__(memo)
        twin.updaters, twin._builder, twin._key = [], None, None
        return twin


def clone(mob):
    """🪞 A cheap structural copy of a static glyph, so redraws never share mutable objects with an animation."""
    twin = _engine.VMobject()
    twin.points = mob.points.copy()
    twin.match_style(mob)
    twin.add(*[clone(sub) for sub in mob.submobjects])
    return twin


@lru_cache(maxsize=8192)
def _label_points(size, key):
    table, gap, advance = _glyphs()
    factor = size / REFERENCE_SIZE
    outlines, cursor, tight = [], 0.0, True
    for live, part in key:
        if live:
            if not tight:
                cursor += gap
            for char in part:
                points, dx, dy, step = table[char]
                outlines.append(points + np.array([cursor + dx, dy, 0.0]))
                cursor += step
        else:
            joined = part.startswith("~")
            word = part[1:] if joined else part
            if not tight and not joined:
                cursor += gap
            shapes, length = _word(word)
            outlines += [points + np.array([cursor, 0.0, 0.0]) for points in shapes]
            cursor += length
        tight = False
    return (np.concatenate(outlines) * factor) if outlines else np.zeros((0, 3))


def row(size, color, *parts):
    """🏷️ One label from static words and live numbers, joined by a space; a leading ``~`` joins a word tightly to the previous part."""
    key = tuple((isinstance(part, Number), str(part).replace("kWh/(m²·a)", INTENSITY_UNIT)) for part in parts if part)
    return LiveText(_label_points(size, key).copy(), color, " ".join(part for _, part in key))


def text(value, point=(0, 0, 0), *, size=16, color=INK, width=None):
    """🔤 Place one label, shrunk to ``width`` when it would overrun."""
    label = row(size, color, *(value if isinstance(value, (list, tuple)) else [value]))
    if width is not None and label.width > width:
        label.scale_to_fit_width(width)
    return label.move_to(point)


@lru_cache(maxsize=2048)
def _frame(width, height, color):
    return _art.window_frame(width, height, color=color)


def chip(value, point=(0, 0, 0), *, size=16, color=INK):
    """🔖 Place one label in the shared square chip chrome (frames are cached by size)."""
    label = text(value, size=size, color=color)
    frame = clone(_frame(round(label.width + 2 * _art.UI_SPACING, 2), round(label.height + 2 * _art.UI_SPACING, 2), color)).move_to(label.get_center())
    return _engine.VGroup(frame, label).move_to(point)


def decimals(value: float) -> int:
    """🔬 Fewest decimals (1 or 2) that show ``value`` exactly to 0.01: 0.6 → 1, 0.99 → 2, 1.0 → 1."""
    return 1 if abs(round(value, 1) - value) < 0.005 else 2


def muted(colour: str, amount: float = 0.45) -> str:
    """🌫️ A colour pulled toward grey: the quieter partner of a saturated result colour."""
    return _engine.interpolate_color(_engine.ManimColor(colour), _engine.ManimColor(GRAY), amount).to_hex()


def place(item, point, edge="center"):
    """📌 Move an item so its left, right or centre edge lands on ``point``."""
    point = np.array(point, dtype=float)
    anchor = {"left": item.get_left(), "right": item.get_right(), "center": item.get_center()}[edge]
    return item.shift(np.array([point[0] - anchor[0], point[1] - item.get_center()[1], 0.0]))


def column(items, *, x, top, bottom, size=15, min_gap=0.34, pad=0.12):
    """🏷️ Labels in a column at ``x``, dodged apart top-down; ``items`` are ``(anchor, parts, color)``; returns labels and leader lines."""
    labels = [text(parts, size=size, color=color) for _, parts, color in items]
    ys = [float(anchor[1]) for anchor, _, _ in items]
    order = sorted(range(len(labels)), key=lambda i: -ys[i])
    for rank in range(1, len(order)):
        above, current = order[rank - 1], order[rank]
        ys[current] = min(ys[current], ys[above] - max(min_gap, (labels[above].height + labels[current].height) / 2 + pad))
    if order:
        under = bottom + labels[order[-1]].height / 2 - ys[order[-1]]
        ys = [y + max(0.0, under) for y in ys]
        over = ys[order[0]] - (top - labels[order[0]].height / 2)
        ys = [y - max(0.0, over) for y in ys]
    group = _engine.VGroup()
    for label, y, (anchor, _, color) in zip(labels, ys, items):
        place(label, (x, y, 0), "left")
        group.add(label, _engine.Line(label.get_left() + np.array([-0.1, 0, 0]), np.array(anchor, dtype=float), color=color, stroke_width=1.4, stroke_opacity=.55))
    return group


def values(trackers) -> dict:
    """🎚️ Read the live design out of the shared trackers."""
    return {key: float(trackers[key].get_value()) for key in KEYS}


def design_key(trackers):
    """🔑 A ``LiveGroup`` key that changes exactly when one of the design trackers moves."""
    return lambda: tuple(float(trackers[key].get_value()) for key in KEYS)


@lru_cache(maxsize=1)
def baseline() -> dict:
    """🏁 The study baseline design."""
    return _model.load_study()["baseline"]


@lru_cache(maxsize=8192)
def _evaluated(orientation, wwr, overhang, insulation):
    return _model.evaluate(dict(zip(KEYS, (orientation, wwr, overhang, insulation))))


def result(design: dict) -> dict:
    """⚙️ Evaluate a design through the model with rounded, cached inputs."""
    design = {**baseline(), **design}
    return _evaluated(*(round(float(design[key]), 5) for key in KEYS))


@lru_cache(maxsize=1)
def reference() -> dict:
    """📏 Largest model responses over a coarse design grid: the shared full scales of bars and arrows."""
    best = {"heating_demand": 0.0, "cooling_demand": 0.0, "electricity": 0.0}
    for orientation in range(90, 451, 30):
        for wwr in (.1, .45, .8):
            for overhang in (0, .6, 1.2):
                for insulation in (.1, .3):
                    found = _evaluated(float(orientation), wwr, float(overhang), insulation)
                    best = {key: max(value, found[key]) for key, value in best.items()}
    return best


@lru_cache(maxsize=1)
def systems() -> dict:
    """🔌 Seasonal heat-pump COP and chiller EER of the study: the conversion from useful heat or cold to electricity."""
    found = _model.load_study()["parameters"]["systems"]
    return {key: float(found[key]["value"]) for key in ("cop", "eer")}


@lru_cache(maxsize=None)
def constant(section: str, name: str) -> float:
    """📚 One coefficient of the study JSON, e.g. ``constant("lighting", "hours")``."""
    return float(_model.load_study()["parameters"][section][name]["value"])


@lru_cache(maxsize=1)
def electricity_range() -> tuple[float, float]:
    """🌡️ Lowest and highest electricity of the south wall surface: the default colour range."""
    grid = _model.surface(41)["electricity"]
    flat = [value for row_ in grid for value in row_]
    return min(flat), max(flat)


@lru_cache(maxsize=1)
def autonomy_range() -> tuple[float, float]:
    """💡 Lowest and highest daylight autonomy over the south wall surface and the Pareto designs: the full off-to-on scale of the lamps."""
    south = [_evaluated(float(baseline()["orientation"]), wwr, overhang, baseline()["insulation"])["autonomy"]
             for wwr in _model.surface(41)["wwr"] for overhang in _model.surface(41)["overhang"]]
    found = south + [result(item["design"])["autonomy"] for item in _model.pareto()["designs"]]
    return min(found), max(found)


@lru_cache(maxsize=1)
def daylight_threshold() -> float:
    """🌤️ The daylight factor in % that counts as daylit: the model's reference factor (two thirds of the maximum autonomy), 2 % by the usual rule of thumb."""
    return float(_model.load_study()["parameters"]["daylight"]["reference_factor"]["value"])


def daylit_depth(mean_factor: float) -> float:
    """📐 Depth in m from the window to where the daylight factor falls below the threshold, for a factor falling linearly from the window to zero at the back wall.

    With mean ``DF̄`` over the room depth ``D`` the factor is ``DF(x) = 2·DF̄·(1 − x/D)``, so ``DF(x) ≥ DF₀`` holds up to ``D·(1 − DF₀ / (2·DF̄))``.
    See https://en.wikipedia.org/wiki/Daylight_factor
    """
    return DEPTH * min(1.0, max(0.0, 1 - daylight_threshold() / (2 * max(mean_factor, 1e-9))))


def ramp(low=None, high=None):
    """🎨 Colour function from low (dark teal, good) to high (cream, bad) electricity."""
    if low is None or high is None:
        low, high = electricity_range()
    stops = [_engine.ManimColor(color) for color in RAMP_STOPS]

    def colour(value: float) -> str:
        position = min(1.0, max(0.0, (value - low) / (high - low) if high > low else 0.0)) * (len(stops) - 1)
        index = min(len(stops) - 2, int(position))
        return _engine.interpolate_color(stops[index], stops[index + 1], position - index).to_hex()

    colour.low, colour.high = low, high
    return colour


@lru_cache(maxsize=4)
def _sun_track(season: str):
    sun = _model.load_study()["sun"]
    phi, delta = math.radians(sun["latitude"]), math.radians(sun["declination"] if season == "summer" else -sun["declination"])
    track = []
    for step in range(-900, 901):
        hour = math.radians(step / 5)
        altitude = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(hour)))
        if altitude >= sun["minimum_altitude"]:
            track.append(((180 + math.degrees(math.atan2(math.sin(hour), math.cos(hour) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360, altitude))
    return track


def sun_arc(season: str) -> tuple[float, float, float]:
    """☀️ Bearing range and noon altitude of the sun at the solstice."""
    track = _sun_track(season)
    bearings = [bearing for bearing, _ in track]
    return min(bearings), max(bearings), max(altitude for _, altitude in track)


@lru_cache(maxsize=4096)
def _sun(orientation: float, season: str) -> dict:
    return _model.sun_state(orientation, season)


def sun_state(orientation: float, season: str) -> dict:
    """🌞 The model's sun-path point that meets the façade best (cached per quarter degree; do not mutate)."""
    return _sun(round(orientation * 4) / 4, season)


def transmission(found: dict) -> dict:
    """🌡️ Transmission conductance U·A in W/K of the glazing and of the opaque façade wall of a design; only these two lose heat to the outside in the study (rear, side walls, floor and roof are adiabatic apart from the model's fixed ``h_other``)."""
    return {"window": constant("envelope", "u_window") * found["window_area"], "wall": found["u_wall"] * found["wall_area"]}


@lru_cache(maxsize=4)
def max_sun_hours(season: str) -> int:
    """🕘 Most hours of direct solstice sun any bearing receives: the full scale of the direct-sun share (``hours / max_sun_hours``)."""
    return max(1, max(len(_model.sun_samples(bearing, season)) for bearing in range(0, 361, 15)))


def cardinal(orientation: float, german: bool) -> str:
    """🧭 Eight-point compass name of a bearing."""
    names = ("N", "NO" if german else "NE", "O" if german else "E", "SO" if german else "SE", "S", "SW", "W", "NW")
    return names[int(((orientation % 360) + 22.5) // 45) % 8]


def _hit(origin, direction, boxes):
    best, which = math.inf, None
    for index, (x0, y0, x1, y1) in enumerate(boxes):
        low, high = -math.inf, math.inf
        for o, d, a, b in ((origin[0], direction[0], x0, x1), (origin[1], direction[1], y0, y1)):
            if abs(d) < 1e-9:
                if o < a or o > b:
                    low, high = math.inf, -math.inf
                continue
            first, second = (a - o) / d, (b - o) / d
            low, high = max(low, min(first, second)), min(high, max(first, second))
        if low <= high and high > 0 and max(low, 0) < best:
            best, which = max(low, 0), index
    return best, which


def number(value: float, digits: int, german: bool) -> str:
    """🔢 Group thousands and punctuate decimals per language."""
    text_value = f"{round(value, digits):,.{digits}f}".replace("-", "−")
    return text_value.replace(",", "\0").replace(".", ",").replace("\0", ".") if german else text_value


def unit_text(value: float, unit: str, digits: int, german: bool) -> str:
    """📐 Number plus unit, joined by a narrow no-break space unless the unit hugs the number."""
    return number(value, digits, german) + ("" if unit in {"°", ""} else " ") + unit.replace("kWh/(m²·a)", INTENSITY_UNIT)


def parts(value: float, unit: str, digits: int, german: bool) -> list:
    """📐 Label parts for a live number and its unit."""
    return [Number(number(value, digits, german)), ("~" + unit) if unit == "°" else unit]


def apportion(values, digits: int = 0, total=None) -> list[float]:
    """⚖️ Largest-remainder rounding: ``values`` rounded to ``digits`` so that they add up exactly to ``total`` (default their sum) rounded to ``digits``.

    Every label that prints parts next to their total must print these, never ``round`` each part: 34.41 + 424.24 + 139.89 = 598.54 prints as 35 + 424 + 140 = 599, while rounding each part gives 34 + 424 + 140 = 598.
    See https://en.wikipedia.org/wiki/Largest_remainder_method
    """
    scale = 10 ** digits
    values = [float(value) for value in values]
    goal = int(round(round(sum(values) if total is None else float(total), digits) * scale))
    exact = [value * scale for value in values]
    floors = [math.floor(value + 1e-9) for value in exact]
    order = sorted(range(len(values)), key=lambda index: (-(exact[index] - floors[index]), -abs(values[index]), index))
    gap = goal - sum(floors)
    for step in range(abs(gap)):
        floors[order[step % len(order)] if gap > 0 else order[-1 - step % len(order)]] += 1 if gap > 0 else -1
    return [value / scale for value in floors]


def dashed(a, b, color=GRAY, stroke_width=1.6, dash=0.09, gap=0.07, opacity=1.0):
    """┄ A dashed line drawn as one shape (``DashedLine`` builds one mobject per dash and is far slower)."""
    a, b = np.array(a, dtype=float), np.array(b, dtype=float)
    length = float(np.linalg.norm(b - a))
    line = _engine.VMobject(stroke_color=color, stroke_width=stroke_width, stroke_opacity=opacity, fill_opacity=0)
    if length < 1e-6:
        return line
    unit = (b - a) / length
    position = 0.0
    while position < length:
        stop = min(position + dash, length)
        line.start_new_path(a + unit * position)
        line.add_line_to(a + unit * stop)
        position += dash + gap
    return line


def _dimension(a, b, color=GRAY, tick=0.05, width=1.4):
    a, b = np.array(a, dtype=float), np.array(b, dtype=float)
    direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
    normal = np.array([-direction[1], direction[0], 0.0]) * tick
    return strokes([(a, b), (a - normal, a + normal), (b - normal, b + normal)], color, width)


def _ring_polygon(centre, inner, outer, start, end, **style):
    steps = max(4, int(abs(end - start) / 4))
    bearings = [start + (end - start) * k / steps for k in range(steps + 1)]
    unit = lambda b: np.array([math.sin(math.radians(b)), math.cos(math.radians(b)), 0.0])
    points = [centre + unit(b) * outer for b in bearings] + [centre + unit(b) * inner for b in reversed(bearings)]
    return _engine.Polygon(*points, **style)


def _arc_points(centre, radius, start, end):
    steps = max(4, int(abs(end - start) / 3))
    return [centre + radius * np.array([math.sin(math.radians(start + (end - start) * k / steps)),
                                        math.cos(math.radians(start + (end - start) * k / steps)), 0.0]) for k in range(steps + 1)]


def _marching(grid_x, grid_y, grid_z, level):
    segments = []
    for i in range(len(grid_x) - 1):
        for j in range(len(grid_y) - 1):
            corners = [(grid_x[i], grid_y[j], grid_z[i][j]), (grid_x[i + 1], grid_y[j], grid_z[i + 1][j]),
                       (grid_x[i + 1], grid_y[j + 1], grid_z[i + 1][j + 1]), (grid_x[i], grid_y[j + 1], grid_z[i][j + 1])]
            crossings = []
            for (xa, ya, za), (xb, yb, zb) in zip(corners, corners[1:] + corners[:1]):
                if (za < level) != (zb < level):
                    t = (level - za) / (zb - za)
                    crossings.append((xa + (xb - xa) * t, ya + (yb - ya) * t))
            if len(crossings) == 2:
                segments.append(tuple(crossings))
            elif len(crossings) == 4:
                segments += [(crossings[0], crossings[1]), (crossings[2], crossings[3])]
    return segments


def nice_levels(low: float, high: float, count: int) -> list[float]:
    """📶 Round contour levels strictly inside the range."""
    raw = (high - low) / (count + 1)
    step = 10 ** math.floor(math.log10(raw))
    step *= min((1, 2, 2.5, 5, 10), key=lambda m: abs(m * step - raw)) if raw else 1
    first = math.ceil(low / step + 1e-9) * step
    return [first + k * step for k in range(int((high - first) // step) + 1) if first + k * step < high]


class RoomVisuals:
    """🏠 Draw one design, live from shared trackers, as section, elevation, plan, bars, thumbnails and heat maps."""

    def _german(self) -> bool:
        return self.language() == "de"

    def __init__(self, *args, **kwargs):
        kwargs.setdefault("camera_class", RasterCamera)
        super().__init__(*args, **kwargs)

    def _live(self, builder, key=None):
        """♻️ A live group; pass ``key`` (a cheap callable returning a hashable snapshot of every input of ``builder``) to rebuild only when it changes."""
        return LiveGroup(builder, key)

    def _widget(self, builder, key):
        """🧩 A live group memoized on ``key``; goes through ``_live`` so scenes can still swap the group class."""
        return self._live(builder).memoize(key)

    def study_trackers(self, design: dict | None = None) -> dict:
        """🎚️ Create the four shared design trackers, filled up from the study baseline."""
        base = {**baseline(), **(design or {})}
        return {key: _engine.ValueTracker(float(base[key])) for key in KEYS}

    def set_design(self, trackers: dict, design: dict, run_time: float = 1.0):
        """🎬 Move every named tracker to a design together, so drawings and numbers stay in step."""
        moves = [trackers[key].animate(run_time=run_time).set_value(float(value)) for key, value in design.items() if key in trackers]
        return _engine.AnimationGroup(*moves)

    def room_section(self, trackers, point=(0, 0, 0), scale=0.5, season="winter", *, labels=True, demand=True, person=True, daylight=True, light_label=False, sky="direct"):
        """🏠 Vertical section through the room: wall, window, overhang, model-driven sun rays, floor patch, sky light, daylight zone, lamps and heating or cooling.

        ``daylight`` draws the diffuse sky light (cream rays from four sky elevations enter the window, the overhang stops the high ones), the daylight-factor gradient along the floor,
        the daylit depth on the work plane and ceiling lamps whose glow and light cone follow ``1 − autonomy``. ``light_label`` (default off, it needs ≈ 0.3 units above the roof and
        under the floor) names them: the share of the hours the lamps burn above the roof, the daylit depth under the floor. The model has only a mean daylight factor, no profile
        function, so the depth is the toolkit's rule of thumb (factor falling linearly to zero at the back wall, ``daylit_depth``) and the label says so.
        ``sky="diffuse"`` draws no direct sun at all (disc, rays, wedge, patches) for scenes about the diffuse-sky metric: only the sky-light arrows enter. Winter with ``demand``
        draws the transmission losses through the façade glass and the opaque façade wall, arrow weight ∝ U·A of each (``transmission``); the rear and side walls carry no arrows.
        """
        mn, s, german = _engine, scale, self._german()
        origin = np.array(point, dtype=float) - s * np.array([3.0, 1.5, 0.0])
        P = lambda x, y: origin + s * np.array([x, y, 0.0])
        full = reference()
        sun_proto = _art.sun_glyph((0, 0, 0)).scale(0.3 * s / 0.6)
        lamp_protos = [_art.lamp_glyph(P(x, HEIGHT), drop=0.1 * s / 0.5, scale=s / 0.6) for x in (1.2, 2.6, 4.0)]
        figure_proto = _art.person_glyph(P(3.7, 0.875))
        figure_proto.scale_to_fit_height(1.75 * s).move_to(P(3.7, 0.875))
        radiator_proto = _art.radiator((0, 0, 0), HEAT, height=0.7 * s, width=0.8 * s)
        word_sun, word_behind = self.tr("Sun altitude", "Sonnenhöhe"), self.tr("Sun behind", "Sonne dahinter")
        word_diffuse = self.tr("Diffuse light only", "Nur diffuses Licht") if sky != "diffuse" else self.tr("Diffuse sky light", "Diffuses Himmelslicht")
        word_demand = self.tr("Heating", "Heizung") if season == "winter" else self.tr("Cooling", "Kühlung")
        word_need = self.tr("Heating demand", "Heizbedarf") if season == "winter" else self.tr("Cooling demand", "Kühlbedarf")
        word_heat, word_power = self.tr("heat", "Wärme"), self.tr("electricity", "Strom")
        word_lamps, word_hours = self.tr("Lamps on", "Lampen an"), self.tr("of hours", "der Zeit")
        word_daylit, word_to = self.tr("Daylit ≥", "Tageslicht ≥"), self.tr("to", "bis")
        word_rule = self.tr("rule of thumb: linear fall-off", "Faustregel: linear abfallend")
        size = label_size(s)

        def rect(x0, y0, x1, y1):
            return mn.Rectangle(width=max(s * (x1 - x0), 1e-4), height=max(s * (y1 - y0), 1e-4)).move_to(P((x0 + x1) / 2, (y0 + y1) / 2))

        def solid(x0, y0, x1, y1):
            return _art.section_material(rect(x0, y0, x1, y1))

        def build():
            design = values(trackers)
            found = result(design)
            t, d = design["insulation"], max(design["overhang"], 0.0)
            window = min(found["window_area"] / WIDTH, HEAD - 0.05)
            sill, x_o = HEAD - window, -(MASONRY + t)
            x_tip = x_o - d
            sun = sun_state(design["orientation"], season)
            hours = len(_model.sun_samples(design["orientation"], season))
            direct = hours > 0 and sky != "diffuse"
            strength = hours / max_sun_hours(season)
            blocked = round(7 * found["shading_" + season])
            front = sun["front"]
            group = mn.VGroup()
            ground = mn.VGroup(strokes([(P(x_o - 2.4, 0), P(x_o, 0))], GRAY, 2),
                               strokes([(P(x_o - 0.35 * k, 0), P(x_o - 0.35 * k - 0.25, -0.25)) for k in range(1, 8)], GRAY, 1, .6))
            roof = solid(x_o, HEIGHT, DEPTH + MASONRY, HEIGHT + SLAB)
            overhang = _art.section_material(rect(x_tip, HEIGHT, x_o, HEIGHT + SLAB)).set_fill(WARN, opacity=.95).set_stroke(INK, width=1.5, opacity=1) if d > 0.01 else mn.VGroup()
            floor = solid(x_o, -SLAB, DEPTH + MASONRY, 0)
            back = solid(DEPTH, 0, DEPTH + MASONRY, HEIGHT)
            masonry = mn.VGroup(solid(-MASONRY, 0, 0, sill), solid(-MASONRY, HEAD, 0, HEIGHT))
            insulation = mn.VGroup()
            for lo, hi in ((0, sill), (HEAD, HEIGHT)):
                insulation.add(rect(x_o, lo, -MASONRY, hi).set_fill(INSULATION, opacity=.55).set_stroke(INSULATION, width=1.5),
                               _hatch((x_o, lo, -MASONRY, hi), 0.16, P, INSULATION, 1.3, .9))
            glass = rect(x_o, sill, x_o + 0.2, HEAD).set_fill(GLASS, opacity=.8).set_stroke(INK, width=2.5)
            alpha = math.radians(sun["profile"])
            if front and d > 0.02 and blocked < 7:
                alpha = min(alpha, math.atan(0.95 * (HEIGHT - (HEAD - (blocked + .5) / 7 * window)) / d))
            direction = np.array([math.cos(alpha) * (1 if front else -1), -math.sin(alpha)])
            centre = np.array([x_o if front else DEPTH + MASONRY, 1.6])
            sun_at = centre - direction * 4.4 if front else np.array([DEPTH + 1.8, HEIGHT + 1.7])
            walls = [(x_o, -SLAB, 0, sill), (x_o, HEAD, 0, HEIGHT), (x_o, HEIGHT, DEPTH + MASONRY, HEIGHT + SLAB),
                     (x_o, -SLAB, DEPTH + MASONRY, 0), (DEPTH, 0, DEPTH + MASONRY, HEIGHT + SLAB), (x_o - 60, -2, x_o, 0)]
            rays, marks = mn.VGroup(), mn.VGroup()
            corridor = []
            opacity = 0.55 + 0.4 * strength

            def ray(start, end, width=2.6, fade=1.0, soft=True):
                line = _art.radiation_ray(P(*start), P(*end), color=SUN, stroke_width=width)
                return line.set_stroke(opacity=[0.0, opacity * fade, opacity * fade] if soft else opacity * fade)

            def start_of(hit):
                return hit - direction * (float((hit - sun_at) @ direction) - 0.35)

            def trace(y):
                hit = np.array([x_o, y])
                return hit + direction * _hit(hit - direction * 1e-6, direction, walls)[0]

            patch_floor, patch_back, wedge = [], [], None
            if front and direct:
                for slot in range(7):
                    y = HEAD - (slot + .5) / 7 * window
                    hit = np.array([x_o, y])
                    start = start_of(hit)
                    corridor.append((start, hit))
                    if slot < blocked:
                        end = hit + direction * ((x_tip - x_o) / direction[0])
                        if end[1] > HEIGHT + SLAB:
                            end = hit + direction * ((HEIGHT + SLAB - y) / direction[1])
                        rays.add(ray(start, end))
                        marks.add(_art.cross_mark(SUN, 0.05).scale(s / 0.5).move_to(P(*end)).set_stroke(opacity=opacity))
                    else:
                        rays.add(ray(start, hit), ray(hit, trace(y), width=2.4, fade=.9, soft=False))
                lit_top = HEAD - blocked / 7 * window
                if blocked < 7:
                    edges = [trace(lit_top), trace(sill)]
                    for end in edges:
                        if end[0] > 0 and abs(end[1]) < 1e-6:
                            patch_floor.append(end[0])
                        elif end[0] > DEPTH - 1e-6:
                            patch_back.append(end[1])
                    top_end, bottom_end = edges
                    corner = [np.array([DEPTH, 0.0])] if (top_end[0] > DEPTH - 1e-6) != (bottom_end[0] > DEPTH - 1e-6) else []
                    wedge = mn.Polygon(P(*start_of(np.array([x_o, lit_top]))), P(*top_end), *[P(*c) for c in corner], P(*bottom_end), P(*start_of(np.array([x_o, sill]))),
                                       stroke_width=0, fill_color=_engine.interpolate_color(_engine.ManimColor(SUN), _engine.ManimColor(INK), .45), fill_opacity=0.12 + 0.12 * strength)
            group.add(ground)
            reach = daylit_depth(found["daylight"]) if daylight else 0.0
            if daylight:
                steps = 24
                group.add(mn.VGroup(*[rect(DEPTH * k / steps, 0, DEPTH * (k + 1) / steps, 0.2).set_fill(
                    DAYLIGHT, opacity=min(1.0, 2 * found["daylight"] * (1 - (k + .5) / steps) / 8.0)).set_stroke(width=0) for k in range(steps)]))
            if reach > 0.05:
                steps = 16
                group.add(mn.VGroup(*[rect(reach * k / steps, 0, reach * (k + 1) / steps, HEIGHT).set_fill(DAYLIGHT, opacity=.62 * (1 - (k + .5) / steps) ** 1.1).set_stroke(width=0)
                                      for k in range(steps)]),
                          strokes([(P(0, WORK), P(reach, WORK)), (P(reach, WORK - .12), P(reach, WORK + .12))], DAYLIGHT, 3, .9))
            if wedge is not None:
                group.add(wedge)
            group.add(insulation, masonry, floor, back, roof, overhang, glass, rays, marks)
            if daylight:
                window_mid = (sill + HEAD) / 2
                bright = 0.5 + 0.5 * min(1.0, found["daylight"] / 5.0)
                for elevation in SKY_ELEVATIONS:
                    angle = math.radians(elevation)
                    way = np.array([math.cos(angle), -math.sin(angle)])
                    thick = 2.4 + 2.0 * (1 + 2 * math.sin(angle)) / 3
                    hit = np.array([x_o, window_mid])
                    arrow = lambda a, b: mn.Arrow(P(*a), P(*b), buff=0, color=SKY, stroke_width=thick, stroke_opacity=0.9 * bright, tip_length=0.13 * s / 0.5,
                                                  max_tip_length_to_length_ratio=.4, max_stroke_width_to_length_ratio=14)
                    if d > 0.02 and (HEIGHT - window_mid) / math.tan(angle) <= d:
                        far = hit - way * 6
                        stop = far + way * _hit(far, way, [(x_tip, HEIGHT, x_o, HEIGHT + SLAB)])[0]
                        corridor.append((stop - way * 0.75, stop))
                        group.add(_art.radiation_ray(P(*(stop - way * 0.75)), P(*stop), color=SKY, stroke_width=thick).set_stroke(opacity=[0.0, 0.85 * bright, 0.85 * bright]),
                                  _art.cross_mark(SKY, 0.05).scale(s / 0.5).move_to(P(*stop)).set_stroke(opacity=0.9 * bright))
                    else:
                        corridor.append((hit - way * 0.75, hit))
                        group.add(arrow(hit - way * 0.75, hit),
                                  _art.radiation_ray(P(*hit), P(*(hit + way * 1.4)), color=SKY, stroke_width=thick).set_stroke(opacity=[0.85 * bright, 0.0]))
            if patch_floor:
                low, high = min(patch_floor), max(patch_floor)
                group.add(rect(low, 0, high, 0.32).set_fill(SUN, opacity=.25).set_stroke(width=0), rect(low, 0, high, 0.1).set_fill(SUN, opacity=.95).set_stroke(width=0))
            if patch_back:
                low, high = min(patch_back), max(patch_back)
                group.add(rect(DEPTH - 0.32, low, DEPTH, high).set_fill(SUN, opacity=.25).set_stroke(width=0), rect(DEPTH - 0.1, low, DEPTH, high).set_fill(SUN, opacity=.95).set_stroke(width=0))
            if direct:
                group.add(clone(sun_proto).move_to(P(*sun_at)).set_opacity(0.45 + 0.55 * min(1.0, strength * 2)))
            low_autonomy, high_autonomy = autonomy_range()
            brightness = 1 - min(1.0, max(0.0, (found["autonomy"] - low_autonomy) / max(high_autonomy - low_autonomy, 1e-9)))
            for lamp in lamp_protos:
                bulb = clone(lamp["bulb"]).set_fill(opacity=0.08 + 0.92 * brightness)
                halo = mn.Circle(radius=0.28 * s / 0.6 * (.4 + .6 * brightness), stroke_width=0, fill_color=LIGHT, fill_opacity=.35 * brightness)
                if daylight and brightness > 0.02:
                    apex, floor_y = bulb.get_center(), P(0, WORK)[1]
                    group.add(mn.Polygon(apex, np.array([apex[0] - 0.9 * s, floor_y, 0.0]), np.array([apex[0] + 0.9 * s, floor_y, 0.0]),
                                         stroke_width=0, fill_color=LIGHT, fill_opacity=.3 * brightness))
                group.add(halo.move_to(bulb.get_center()), clone(lamp["cable"]), clone(lamp["shade"]), bulb)
            if person:
                group.add(clone(figure_proto))
            loss = []
            if demand and season == "winter":
                flows = transmission(found)
                peak = max(flows.values()) or 1.0
                for name, y in (("window", (sill + HEAD) / 2), ("wall", sill / 2 if sill > 0.3 else (HEAD + HEIGHT) / 2)):
                    loss.append((y, 0.5 + 0.5 * flows[name] / peak, flows[name] / peak))
                    corridor.append((np.array([x_o, y]), np.array([x_o - loss[-1][1], y])))
            placed = []
            if labels:
                column = x_o - 0.8
                level = min(HEAD, (sill + HEAD) / 2)
                notes = [(chip([self.tr("d", "Vordach"), *parts(d, "m", decimals(d), german)], size=size - 1, color=WARN), HEIGHT + SLAB + 0.45, min(x_tip, x_o)),
                         (chip([self.tr("WWR", "Fenster"), *parts(design["wwr"] * 100, "%", 0, german)], size=size - 1, color=GLASS), level, x_o - 0.35),
                         (chip([self.tr("t", "Dämmung"), *parts(t * 100, "cm", 0, german)], size=size - 1, color=INSULATION), -0.62, x_o)]
                samples = [a + (b - a) * k / 24 for a, b in corridor for k in range(25)]
                boxes_m = []

                def crowding(right, y, item):
                    low, high = y - item.height / s / 2 - 0.1, y + item.height / s / 2 + 0.1
                    left = right - item.width / s - 0.1
                    chips = sum(1 for l, r, b, t in boxes_m if left < r + 0.1 and right > l - 0.1 and low < t and high > b)
                    return chips, sum(1 for p in samples if left < p[0] < right + 0.1 and low < p[1] < high)

                tries = [(0, 0), (0, -.5), (0, .5), (-.6, 0), (0, -1), (0, 1), (-.6, -.5), (-.6, .5), (0, -1.5), (0, 1.5), (-1.2, 0), (-1.2, -.7), (-1.2, .7), (0, -2), (0, 2)]
                for item, y, end in notes:
                    right0 = min(column, x_tip - 0.35) if end == min(x_tip, x_o) else column
                    options = [(right0 + dx, y + dy) for dx, dy in tries if -0.9 <= y + dy <= HEIGHT + SLAB + 1.3 and (y + dy - item.height / s / 2 >= 0.1 or y + dy + item.height / s / 2 <= -0.2)] or [(right0, y)]
                    right, drawn = min(options, key=lambda o: (crowding(*o, item), options.index(o)))
                    place(item, P(right, drawn), "right")
                    boxes_m.append((right - item.width / s, right, drawn - item.height / s / 2, drawn + item.height / s / 2))
                    backing = mn.Rectangle(width=item.width, height=item.height, stroke_width=0, fill_color=_art.P_DEEP_DARK, fill_opacity=1).move_to(item.get_center())
                    group.add(backing, item, _leader(item, P(end, y), item[0].get_stroke_color()))
                    placed.append(item)
                group.add(_dimension(P(x_o - 0.35, sill), P(x_o - 0.35, HEAD), color=GLASS))
                if d > 0.05:
                    group.add(_dimension(P(x_tip, HEIGHT + SLAB + 0.45), P(x_o, HEIGHT + SLAB + 0.45), color=WARN))
                group.add(_dimension(P(x_o, -0.62), P(-MASONRY, -0.62), color=INSULATION))
            if demand:
                heating = season == "winter"
                key = "heating_demand" if heating else "cooling_demand"
                share = found[key] / full[key]
                length, colour = 0.3 + 1.8 * share, HEAT if heating else COOL
                if heating:
                    for y, reach_m, weight in loss:
                        group.add(mn.Arrow(P(x_o + 0.02, y), P(x_o - reach_m, y), buff=0, color=colour, stroke_width=2.5 + 4.5 * weight,
                                           max_tip_length_to_length_ratio=.4, tip_length=0.13 * s / 0.5, max_stroke_width_to_length_ratio=16))
                else:
                    for y in (1.1, 1.6, 2.1):
                        group.add(mn.Arrow(P(DEPTH - 0.5, y), P(DEPTH + MASONRY + 0.4 + length, y), buff=0, color=colour, stroke_width=3 + 3 * share,
                                           max_tip_length_to_length_ratio=.35, tip_length=0.13, max_stroke_width_to_length_ratio=12))
                group.add(clone(radiator_proto).move_to(P(DEPTH - 0.6, 0.45)) if heating else _cooler(P(DEPTH - 0.9, 2.55), s, colour))
                if labels:
                    electricity = parts(found["heating" if heating else "cooling"], "kWh/a", 0, german)
                    efficiency = ["COP" if heating else "EER", Number(number(systems()["cop" if heating else "eer"], 1, german))]
                    heat_line = ([word_need, *parts(found[key], "kWh/a", 0, german), word_heat], _engine.interpolate_color(_engine.ManimColor(colour), _engine.ManimColor(GRAY), .45))
                    power_line = (["÷", *efficiency, "=", *electricity, word_power], colour)
                    lines = (heat_line, power_line) if s >= DEMAND_FULL_SCALE else (([word_demand, "→", *electricity, word_power], colour),)
                    drawn = [text(words, size=size, color=tint) for words, tint in lines]
                    anchor = P(DEPTH + MASONRY + 0.5, 3.35)
                    edge, left, top = "left", anchor[0], anchor[1]
                    if left + max(item.width for item in drawn) > DEMAND_EDGE:
                        edge, left, top = "right", DEMAND_EDGE, P(0, HEIGHT + SLAB + (0.9 if len(drawn) > 1 else 0.45))[1]
                    for index, item in enumerate(drawn):
                        place(item, (left, top if index == 0 else drawn[index - 1].get_bottom()[1] - item.height / 2 - 0.04, 0), edge)
                    group.add(*drawn)
                    placed.extend(drawn)
            depth_y = min(-1.3, -0.5 - 0.55 / s) if light_label and daylight else -1.3
            for words, x0, x1 in (("6 m", 0, DEPTH),):
                note = text(words, size=DIMENSION_SIZE, color=GRAY).move_to(P((x0 + x1) / 2, depth_y))
                group.add(_dimension(P(x0, depth_y), P(x1, depth_y)), note.shift(np.array([0, -0.08 - note.height / 2, 0])))
            if labels:
                words = [word_sun, *parts(sun["profile"], "°", 0, german)] if front and direct else [word_diffuse if not direct else word_behind]
                if direct:
                    for dx, dy in ((0, 0.55), (0, -0.55), (-1.3, 0), (1.3, 0), (0, 0.9), (0, -0.9)):
                        label = text(words, P(*sun_at) + np.array([dx, dy, 0]) * s * 2, size=size - 1, color=SUN)
                        if not any(_overlap(label, other) for other in placed):
                            break
                else:
                    label = place(text(words, size=size - 1, color=SUN), P(0.3, HEIGHT + SLAB + 0.35) + np.array([0.0, 0.1, 0.0]), "left")
                    for _ in range(4):
                        if not any(_overlap(label, other) for other in placed):
                            break
                        label.shift(np.array([0.0, 0.2, 0.0]))
                    placed.append(label)
                group.add(label)
            if light_label and daylight:
                lamps = text([word_lamps, *parts(100 * (1 - found["autonomy"]), "%", 0, german), word_hours], size=size - 1, color=LIGHT)
                place(lamps, P(0.3, HEIGHT + SLAB + 0.35) + np.array([0.0, lamps.height / 2, 0.0]), "left")
                for _ in range(4):
                    if not any(_overlap(lamps, other) for other in placed):
                        break
                    lamps.shift(np.array([0.0, 0.2, 0.0]))
                group.add(lamps)
                if reach > 0.05:
                    group.add(_dimension(P(0, -0.5), P(reach, -0.5), color=DAYLIGHT))
                    deep = text([word_daylit, *parts(daylight_threshold(), "%", 0, german), word_to, *parts(reach, "m", 1, german)], size=size - 1, color=DAYLIGHT)
                    group.add(place(deep, P(0, -0.5) + np.array([0.0, -0.05 - deep.height / 2, 0.0]), "left"))
                    rule = text(word_rule, size=size - 1, color=GRAY)
                    group.add(place(rule, P(0, -0.5) + np.array([0.0, -0.1 - deep.height - rule.height / 2, 0.0]), "left"))
            return group

        return self._widget(build, design_key(trackers))

    def facade(self, trackers, point=(0, 0, 0), scale=0.5, season="summer", *, labels=True):
        """🪟 Elevation of the 8 × 3 m façade: window opening, overhang edge and the overhang's shadow band."""
        mn, s, german = _engine, scale, self._german()
        origin = np.array(point, dtype=float) - s * np.array([WIDTH / 2, HEIGHT / 2, 0.0])
        P = lambda x, y: origin + s * np.array([x, y, 0.0])

        def build():
            design = values(trackers)
            found = result(design)
            window = min(found["window_area"] / WIDTH, HEAD - 0.05)
            sill = HEAD - window
            wall = mn.Rectangle(width=s * WIDTH, height=s * HEIGHT, color=INK, stroke_width=2, fill_color=INK, fill_opacity=.12).move_to(P(WIDTH / 2, HEIGHT / 2))
            glass = mn.Rectangle(width=s * WIDTH, height=s * window, color=GLASS, stroke_width=2, fill_color=GLASS, fill_opacity=.4).move_to(P(WIDTH / 2, (sill + HEAD) / 2))
            group = mn.VGroup(wall, glass)
            for x in (2, 4, 6):
                group.add(mn.Line(P(x, sill), P(x, HEAD), color=GLASS, stroke_width=1.5))
            shaded = found["shading_" + season]
            band = (HEIGHT - HEAD) + shaded * window if shaded > 0.005 else 0.0
            if band > 0.01:
                group.add(mn.Rectangle(width=s * WIDTH, height=s * band, stroke_width=0, fill_color=_art.P_DEEP_DARK, fill_opacity=.42).move_to(P(WIDTH / 2, HEIGHT - band / 2)))
                group.add(mn.Line(P(0, HEIGHT - band), P(WIDTH, HEIGHT - band), color=WARN, stroke_width=2))
            group.add(_art.section_material(mn.Rectangle(width=s * (WIDTH + .3), height=s * SLAB)).move_to(P(WIDTH / 2, HEIGHT + SLAB / 2)).set_fill(WARN, opacity=.95).set_stroke(INK, width=1.5, opacity=1))
            size = label_size(s)
            if s >= 0.3:
                group.add(_dimension(P(-.35, 0), P(-.35, HEIGHT)), text("3 m", P(-.35, HEIGHT / 2) + np.array([-0.3 - size * 0.012, 0, 0]), size=size - 2, color=GRAY))
                group.add(_dimension(P(0, -.35), P(WIDTH, -.35)), text("8 m", P(WIDTH / 2, -.35) + np.array([0, -0.2 - size * 0.012, 0]), size=size - 2, color=GRAY))
            else:
                group.add(_dimension(P(-.35, 0), P(-.35, HEIGHT)), _dimension(P(0, -.35), P(WIDTH, -.35)))
            if labels and s >= 0.22:
                full = chip([self.tr("WWR", "Fensteranteil"), *parts(design["wwr"] * 100, "%", 0, german), "·", *parts(found["window_area"], "m²", 1, german)], size=size - 1, color=GLASS)
                short = chip([self.tr("WWR", "Fensteranteil"), *parts(design["wwr"] * 100, "%", 0, german)], size=size - 1, color=GLASS)
                depth_chip = chip([self.tr("d", "Vordach"), *parts(design["overhang"], "m", decimals(design["overhang"]), german)], size=size - 1, color=WARN)
                room_wide = s * WIDTH + 0.3
                top = P(0, HEIGHT + SLAB + 0.5)
                first = full if full.width + depth_chip.width + 0.15 <= room_wide else short
                if first.width + depth_chip.width + 0.15 <= room_wide:
                    group.add(place(first, top, "left"), place(depth_chip, P(WIDTH, HEIGHT + SLAB + 0.5), "right"))
                else:
                    group.add(place(short, (origin[0] + s * WIDTH / 2 + short.width / 2 * 0, top[1] + depth_chip.height + 0.08, 0), "center"),
                              place(depth_chip, (origin[0] + s * WIDTH / 2, top[1], 0), "center"))
            return group

        return self._widget(build, design_key(trackers))

    def plan(self, trackers, point=(0, 0, 0), scale=0.28, *, ring_radius=6.0, arcs=False, sector=None, site=None, ghost=None, dimensions=False, season_labels=True):
        """🧭 Plan of the room turning inside a fixed compass; extras: sun arcs, shaded bearing band, site labels, ghost room, dimensions."""
        mn, s, german = _engine, scale, self._german()
        centre = np.array(point, dtype=float)
        R = ring_radius
        unit = lambda bearing: np.array([math.sin(math.radians(bearing)), math.cos(math.radians(bearing)), 0.0])
        static = mn.VGroup(mn.Circle(radius=s * R, color=GRAY, stroke_width=1.5).move_to(centre))
        for bearing in range(0, 360, 45):
            static.add(mn.Line(centre + unit(bearing) * s * (R - .25), centre + unit(bearing) * s * R, color=GRAY, stroke_width=1.5))
        for bearing, name in ((0, "N"), (90, "O" if german else "E"), (180, "S"), (270, "W")):
            static.add(text(name, centre + unit(bearing) * s * (R + .85), size=17 if name == "N" else 15, color=INK if name == "N" else GRAY))
        if sector is not None:
            low, high, colour = sector
            static.add(_ring_polygon(centre, s * (R - 1.0), s * R, low, high, stroke_width=0, fill_color=colour, fill_opacity=.28))
            for bearing in (low, high):
                static.add(mn.Line(centre + unit(bearing) * s * (R - 1.0), centre + unit(bearing) * s * (R + .1), color=colour, stroke_width=2))
        if arcs:
            for season, radius, colour in (("winter", R + 1.7, WARN), ("summer", R + 2.5, SUN)):
                low, high, noon = sun_arc(season)
                arc = mn.VMobject(color=colour, stroke_width=2, stroke_opacity=.85)
                arc.set_points_as_corners(_arc_points(centre, s * radius, low, high))
                static.add(mn.DashedVMobject(arc, num_dashes=40))
                if season_labels:
                    outward = unit(high + 4)[0] < 0
                    static.add(place(text([self.tr("winter", "Winter") if season == "winter" else self.tr("summer", "Sommer"), *parts(noon, "°", 0, german)], size=14, color=colour),
                                     centre + unit(high + 4) * s * radius + np.array([-0.06 if outward else 0.06, 0, 0]), "right" if outward else "left"))
        if site is not None:
            street, quiet = site
            static.add(text(self.tr("Street", "Straße"), centre + unit(street) * s * (R + (3.9 if arcs else 2.0)), size=15, color=GRAY))
            static.add(text(self.tr("Courtyard", "Innenhof"), centre + unit(quiet) * s * (R + (3.9 if arcs else 2.0)), size=15, color=GOOD))
        if ghost is not None:
            normal, tangent = unit(ghost), unit(ghost + 90)
            corners = [normal * 3 + tangent * 4, normal * 3 - tangent * 4, -normal * 3 - tangent * 4, -normal * 3 + tangent * 4]
            static.add(mn.DashedVMobject(mn.Polygon(*[centre + s * c for c in corners], color=GRAY, stroke_width=1.8), num_dashes=40))

        def build():
            design = values(trackers)
            found = result(design)
            theta = design["orientation"]
            normal, tangent = unit(theta), unit(theta + 90)
            corners = [normal * 3 + tangent * 4, normal * 3 - tangent * 4, -normal * 3 - tangent * 4, -normal * 3 + tangent * 4]
            group = mn.VGroup(mn.Polygon(*[centre + s * c for c in corners], color=INK, stroke_width=2, fill_color=INK, fill_opacity=.1))
            group.add(mn.Line(centre + s * (normal * 3 + tangent * 4), centre + s * (normal * 3 - tangent * 4), color=GLASS, stroke_width=3 + 8 * design["wwr"]))
            group.add(mn.Arrow(centre + s * normal * 3.1, centre + s * normal * 4.6, buff=0, color=GLASS, stroke_width=3, max_tip_length_to_length_ratio=.4, tip_length=.12))
            if dimensions:
                group.add(text("8 m", centre + s * (normal * 2.2 - tangent * 2.6), size=13, color=GRAY), text("6 m", centre + s * (tangent * 3.2 - normal * 0.2), size=13, color=GRAY))
            if arcs:
                for season, radius in (("winter", R + 1.7), ("summer", R + 2.5)):
                    state = sun_state(theta, season)
                    strength = found["sun_" + season]
                    spot = centre + unit(state["bearing"]) * s * radius
                    group.add(mn.Line(spot, centre + s * normal * 3.0, color=SUN if season == "summer" else WARN, stroke_width=1.6, stroke_opacity=.15 + .6 * strength))
                    group.add(sun_marker(spot, .16 * s / .28, .35 + .65 * min(1, strength * 2)))
            group.add(text([Number(number(theta, 0, german)), "~°", cardinal(theta, german)], centre, size=16, color=INK))
            return group

        return mn.VGroup(static, self._widget(build, design_key(trackers)))

    def energy_bars(self, trackers, point=(0, 0, 0), height=3.0, maximum=None, *, width=0.8, labels=True, compact=False, reference=None, intensity=True, size=15, digits=0, total=None, rule=True):
        """📊 Stacked heating, cooling and lighting electricity with live kWh/a; fixed scale via ``maximum``; ``rule=False`` omits the vertical scale rule; ``reference`` adds a ghost bar; ``digits`` decimals of every printed kWh/a and of the intensity; ``total`` (``fn(design, found) -> kWh/a``) replaces the printed total when a scene derives it from its own printed parts."""
        mn, german = _engine, self._german()
        base = np.array(point, dtype=float)
        top_value = maximum or reference_scale()
        unit = height / top_value
        names = (("heating", self.tr("Heating", "Heizung"), HEAT), ("cooling", self.tr("Cooling", "Kühlung"), COOL), ("lighting", self.tr("Lighting", "Beleuchtung"), LIGHT))
        ghost_x = base[0] - width * 1.6 if reference is not None else base[0]
        static = mn.VGroup(mn.Line(base + np.array([-width * .75 - (width * 1.6 if reference is not None else 0), 0, 0]), base + np.array([width * .75, 0, 0]), color=GRAY, stroke_width=2),
                           *([mn.Line(base + np.array([-width / 2 - .3, 0, 0]), base + np.array([-width / 2 - .3, height, 0]), color=GRAY, stroke_width=1.2, stroke_opacity=.6),
                              mn.Line(base + np.array([-width / 2 - .3, height, 0]), base + np.array([-width / 2 - .2, height, 0]), color=GRAY, stroke_width=1.2, stroke_opacity=.6)] if rule else []))
        if reference is not None:
            ref = result(reference)
            cursor = 0.0
            for key, name, colour in names:
                span = ref[key] * unit
                static.add(mn.Rectangle(width=width, height=max(span, 1e-3), color=colour, stroke_width=1.6, fill_color=colour, fill_opacity=.12).move_to(
                    np.array([ghost_x, base[1] + cursor + span / 2, 0])))
                cursor += span
            static.add(text(parts(sum(apportion([ref[key] for key, _, _ in names], digits)), "kWh/a", digits, german), np.array([ghost_x, base[1] + cursor + .25, 0]), size=size - 1, color=GRAY))
            static.add(text(self.tr("reference", "Referenz"), np.array([ghost_x, base[1] - .3, 0]), size=size - 1, color=GRAY))

        def build():
            found = result(values(trackers))
            shown = apportion([found[key] for key, _, _ in names], digits)
            group = mn.VGroup()
            cursor, anchors = 0.0, []
            for (key, name, colour), printed in zip(names, shown):
                span = found[key] * unit
                if span > 1e-3:
                    group.add(mn.Rectangle(width=width, height=span, color=colour, stroke_width=1.2, fill_color=colour, fill_opacity=.78).move_to(
                        np.array([base[0], base[1] + cursor + span / 2, 0])))
                anchors.append((np.array([base[0] + width / 2, base[1] + cursor + span / 2, 0]), [name, *parts(printed, "kWh/a", digits, german)], colour))
                cursor += span
            printed_total = sum(shown) if total is None else total(values(trackers), found)
            group.add(text(parts(printed_total, "kWh/a", digits, german), np.array([base[0], base[1] + cursor + .3, 0]), size=size + 3, color=INK))
            if labels and not compact:
                group.add(column(anchors, x=base[0] + width / 2 + .35, top=base[1] + height + .9, bottom=base[1] + .1, size=size))
            if intensity:
                group.add(text(parts(found["intensity"], INTENSITY_UNIT, digits, german), np.array([base[0], base[1] - .3, 0]), size=size - 1, color=INK))
            return group

        return mn.VGroup(static, self._widget(build, design_key(trackers)))

    def formula(self, trackers, rows, point=(0, 0, 0), *, size=20, align="center", columns=True, frame=None, width=11.6, gap=0.12):
        """🧮 Live substituted formula in the energy-clip style: rows of words and numbers redrawn from the design trackers.

        ``rows`` is a list of rows, or ``fn(design, found) -> rows``; a row is a list of items: a string (operator or word, ink), ``(string, colour)`` or ``Live(...)``.
        With ``columns`` and equally long rows the i-th item of every row shares one column (words above their numbers). ``frame`` is an outline colour
        (``None``: no frame), ``align`` anchors ``point`` at the ``left``, ``center`` or ``right`` of the widest row, ``width`` is the largest width in screen units.
        """
        mn, german = _engine, self._german()
        spacing = _glyphs()[1] * size / REFERENCE_SIZE * 1.8
        pitch = size * 0.0165 + gap

        def build():
            design = values(trackers)
            found = result(design)
            table = rows(design, found) if callable(rows) else rows
            shown = {}
            for items in table:
                for item in items:
                    if isinstance(item, Live) and item.parts_of and (item.parts_of, item.digits, item.scale) not in shown:
                        shown[(item.parts_of, item.digits, item.scale)] = dict(zip(item.parts_of, apportion([found[key] * item.scale for key in item.parts_of], item.digits)))

            def token(item):
                if isinstance(item, Live):
                    source = item.source
                    value = shown[(item.parts_of, item.digits, item.scale)][source] if item.parts_of else (
                        source(design, found) if callable(source) else found[source] if isinstance(source, str) else float(source)) * item.scale
                    return row(size, item.color, *parts(value, item.unit, item.digits, german))
                string, colour = item if isinstance(item, tuple) else (item, INK)
                return row(size, colour, string)

            tokens = [[token(item) for item in items] for items in table]
            group = mn.VGroup()
            blocks, start = [], 0
            for index in range(1, len(tokens) + 1):
                if index == len(tokens) or not columns or len(tokens[index]) != len(tokens[start]):
                    blocks.append(range(start, index))
                    start = index
            layout, extent = [], 0.0
            for block in blocks:
                size_ = len(tokens[block[0]])
                if columns and len(block) > 1:
                    widths = [max(tokens[row_][column_].width for row_ in block) for column_ in range(size_)]
                    lefts = [sum(widths[:column_]) + spacing * column_ for column_ in range(size_)]
                    span = sum(widths) + spacing * (size_ - 1)
                else:
                    widths = lefts = None
                    span = max(sum(label.width for label in tokens[row_]) + spacing * (len(tokens[row_]) - 1) for row_ in block)
                layout.append((block, widths, lefts, span))
                extent = max(extent, span)
            for block, widths, lefts, span in layout:
                shift = {"left": 0.0, "center": (extent - span) / 2, "right": extent - span}[align]
                for row_ in block:
                    cursor = shift
                    for column_, label in enumerate(tokens[row_]):
                        left = shift + lefts[column_] + (widths[column_] - label.width) / 2 if widths else cursor
                        group.add(label.shift(np.array([left - label.get_left()[0], -row_ * pitch, 0.0])))
                        cursor = left + label.width + spacing
            if group.width > width:
                group.scale_to_fit_width(width)
            anchor = {"left": group.get_left()[0], "right": group.get_right()[0], "center": group.get_center()[0]}[align]
            group.shift(np.array([point[0] - anchor, point[1] - group.get_center()[1], 0.0]))
            if frame is not None:
                pad = _art.UI_SPACING
                group.add(clone(_frame(round(group.width + 2 * pad, 2), round(group.height + 2 * pad, 2), frame)).move_to(group.get_center()).set_z_index(-1))
            return group

        return self._widget(build, design_key(trackers))

    def sun_note(self, trackers, season, point=(0, 0, 0), *, size=14, color=SUN, align="left", width=3.2):
        """🔆 Two-line live note on the direct sun of a solstice: "direct sun 9 h" (the hours the sun shines on the façade, the number the section draws its rays from) and
        "44 % of the sunniest façade" (the model's `sun_<season>`: seasonal irradiance relative to the best bearing), so the percentage always names its reference."""
        key = "sun_" + season
        hours = lambda design, found: len(_model.sun_samples(design["orientation"], season))
        rows = [[(self.tr("direct sun", "direkte Sonne"), color), self.live(hours, color, 0, "h")],
                [self.live(lambda design, found: found[key], color, 0, "%", scale=100.0), (self.tr("of the sunniest façade", "der sonnigsten Fassade"), color)]]
        return self.formula(trackers, rows, point, size=size, align=align, columns=False, width=width, gap=0.04)

    def live(self, source, color=INK, digits=0, unit="", *, scale=1.0, parts_of=()):
        """🔟 One live number for ``formula``: ``source`` is a model key, a constant or ``fn(design, found)``; ``parts_of`` rounds with the largest remainder."""
        return Live(source, color, digits, unit, scale, tuple(parts_of))

    def electricity_formula(self, trackers, point=(0, 0, 0), *, size=20, words=True, frame=None, align="center", width=11.6, digits=0):
        """⚡ ``Heating + Cooling + Lighting = Total`` with live kWh/a in the bar colours; the printed parts always add up to the printed total (``digits`` decimals; use 1 next to ``derivation``, which prints kWh/a with one decimal)."""
        group = ("heating", "cooling", "lighting")
        colours = (HEAT, COOL, LIGHT)
        names = (self.tr("Heating", "Heizung"), self.tr("Cooling", "Kühlung"), self.tr("Lighting", "Beleuchtung"))
        numbers = [item for key, colour in zip(group, colours) for item in (self.live(key, colour, digits, parts_of=group), "+")][:-1]
        table = [[item for name, colour in zip(names, colours) for item in ((name, colour), "+")][:-1] + ["=", (self.tr("Total", "Summe"), INK)]] if words else []
        table.append([*numbers, "=", self.live("electricity", INK, digits, "kWh/a")])
        return self.formula(trackers, table, point, size=size, align=align, frame=frame, width=width)

    def derivation(self, trackers, part, point=(0, 0, 0), *, size=20, words=True, frame=None, align="center", width=11.6, digits=1):
        """📜 How one number of the bar comes about, read from the model's balance terms in ``evaluate`` (nothing is re-derived here): ``part`` is
        ``heating`` (heat losses − utilisation × gains = demand, then demand ÷ COP = electricity), ``cooling`` (gains − utilisation × removable heat = load, then ÷ EER),
        ``lighting`` (installed power × hours × share of the hours the lamps burn), ``daylight`` (mean daylight factor = transmittance × window area × sky angle ÷ effective
        room surfaces, from the study coefficients: the model returns only the factor) or ``bill`` (electricity × tariff).
        Precision rule: kWh/a at ``digits`` decimals (1), utilisation, share and COP with the fewest decimals for which the printed line holds exactly as printed; where no
        precision does, "=" turns into "≈". Electricity results are the largest-remainder parts of the sum formula, so they equal ``electricity_formula(digits=digits)``.
        See https://en.wikipedia.org/wiki/Daylight_factor"""
        tr = self.tr
        tariff = constant("economics", "tariff")

        def half_up(value, places=0):
            return math.floor(value * 10 ** places + 0.5) / 10 ** places

        def fit(options, holds):
            return next(((places, "=") for places in options if holds(places)), (options[-1], "≈"))

        def conversion(found, colour, soft, demand_word, factor_word, factor, result_word, index, shown_demand):
            places = 1 if abs(factor - half_up(factor, 1)) < 0.005 else 2
            printed = half_up(factor, places)
            result = apportion([found["heating"], found["cooling"], found["lighting"]], digits)[index]
            sign = "=" if half_up(shown_demand / printed, digits) == result else "≈"
            return [[(demand_word, soft), "÷", (factor_word, INK), sign, (result_word, colour)],
                    [self.live(shown_demand, soft, digits, "kWh/a"), "÷", self.live(printed, INK, places), sign, self.live(result, colour, digits, "kWh/a")]]

        def balance(found, key):
            heating = key == "heating"
            colour, first, second = (HEAT, "heating_losses", "heating_gains") if heating else (COOL, "cooling_gains", "cooling_removable")
            soft = muted(colour)
            one, two, shown_demand = half_up(found[first], digits), half_up(found[second], digits), half_up(found[key + "_demand"], digits)
            places, sign = fit((2, 3, 4), lambda p: half_up(one - half_up(found[key + "_eta"], p) * two, digits) == shown_demand)
            names = ((tr("Heat losses", "Wärmeverlust"), tr("Gains", "Gewinne"), tr("Heating demand", "Heizbedarf")) if heating else
                     (tr("Heat gains", "Wärmegewinn"), tr("Removable heat", "Abführbare Wärme"), tr("Cooling load", "Kühllast")))
            head = [(names[0], soft), "−", (tr("Utilisation", "Ausnutzung"), INK), "×", (names[1], soft), sign, (names[2], soft)]
            line = [self.live(one, soft, digits, "kWh/a"), "−", self.live(half_up(found[key + "_eta"], places), INK, places), "×", self.live(two, soft, digits, "kWh/a"),
                    sign, self.live(shown_demand, soft, digits, "kWh/a")]
            head2, line2 = conversion(found, colour, soft, names[2], "COP" if heating else "EER", found["cop" if heating else "eer"],
                                      tr("Heating", "Heizung") if heating else tr("Cooling", "Kühlung"), 0 if heating else 1, shown_demand)
            return ([head] if words else []) + [line] + ([head2] if words else []) + [line2]

        def table(design, found):
            if part in ("heating", "cooling"):
                return balance(found, part)
            if part == "daylight":
                glow = muted(DAYLIGHT, .2)
                depth = max(design["overhang"], 0.0)
                angle = constant("daylight", "sky_angle") * (1 - constant("daylight", "sky_angle_loss") * depth ** 2 / (depth ** 2 + constant("daylight", "overhang_half_depth") ** 2))
                effective = constant("daylight", "surface_area") * (1 - constant("daylight", "reflectance") ** 2)
                tau = constant("daylight", "transmittance")
                sign = "=" if round(half_up(tau, 2) * half_up(found["window_area"], 1) * half_up(angle) / half_up(effective), 1) == half_up(found["daylight"], 1) else "≈"
                head = [(tr("Glass transmittance", "Glasdurchlässigkeit"), INK), "×", (tr("Window area", "Fensterfläche"), GLASS), "×", (tr("Sky angle", "Himmelswinkel"), glow), "÷",
                        (tr("Room surfaces", "Raumflächen"), INK), sign, (tr("Daylight factor", "Tageslichtquotient"), DAYLIGHT)]
                return ([head] if words else []) + [[self.live(tau, INK, 2), "×", self.live("window_area", GLASS, 1, "m²"), "×", self.live(angle, glow, 0, "°"), "÷",
                                                       self.live(effective, INK, 0, "m²"), sign, self.live("daylight", DAYLIGHT, 1, "%")]]
            shown = apportion([found["heating"], found["cooling"], found["lighting"]], digits)
            if part == "lighting":
                power, hours = half_up(found["lighting_power"], 3), found["lighting_hours"]
                places, sign = fit((1, 2, 3), lambda p: half_up(power * hours * half_up(100 * found["lamps_on"], p) / 100, digits) == shown[2])
                head = [(tr("Installed power", "Installierte Leistung"), INK), "×", (tr("Hours", "Stunden"), INK), "×", (tr("Lamps on", "Lampen an"), LIGHT), sign, (tr("Lighting", "Beleuchtung"), LIGHT)]
                return ([head] if words else []) + [[self.live(power, INK, 3, "kW"), "×", self.live(hours, INK, 0, "h/a"), "×",
                                                       self.live(half_up(100 * found["lamps_on"], places), LIGHT, places, "%"), sign, self.live(shown[2], LIGHT, digits, "kWh/a")]]
            total = sum(shown)
            head = [(tr("Electricity", "Strom"), INK), "×", (tr("Tariff", "Tarif"), INK), "=", (tr("Energy cost", "Energiekosten"), INK)]
            return ([head] if words else []) + [[self.live(total, INK, digits, "kWh/a"), "×", self.live(tariff, INK, 2, "CHF/kWh"), "=",
                                                   self.live(half_up(total * tariff, digits), INK, digits, "CHF/a")]]

        return self.formula(trackers, table, point, size=size, align=align, frame=frame, width=width)

    def thumbnail(self, design, point=(0, 0, 0), scale=0.12, *, state="filled", ramp=None, value=None, ring=None):
        """🏘️ Section pictogram of one façade: glass opening ∝ WWR, overhang ∝ d, insulation ∝ t; ghost = grey outline, filled = ramp colour."""
        mn = _engine
        design = {**baseline(), **design}
        found = result(design)
        s, centre = scale, np.array(point, dtype=float)
        P = lambda x, y: centre + s * np.array([x - 0.4, y - 1.6, 0.0])
        window = min(found["window_area"] / WIDTH, HEAD - 0.05)
        sill, t, d = HEAD - window, design["insulation"] * 2.0, design["overhang"]
        ghost = state == "ghost"
        ink = GRAY if ghost else INK
        group = mn.VGroup()
        tile = mn.Rectangle(width=s * 4.8, height=s * 4.0).move_to(P(0.4, 1.6))
        if ghost:
            group.add(mn.DashedVMobject(tile.set_stroke(GRAY, width=1.4), num_dashes=24))
        else:
            colour = (ramp or globals()["ramp"]())(found["electricity"] if value is None else value)
            group.add(tile.set_fill(colour, opacity=.9).set_stroke(ring or INK, width=2.2 if ring else 1.2))
        for x0, y0, x1, y1 in ((-MASONRY - t, 0, 0, sill), (-MASONRY - t, HEAD, 0, HEIGHT)):
            group.add(mn.Rectangle(width=s * (x1 - x0), height=s * (y1 - y0), stroke_width=0, fill_color=INSULATION if not ghost else GRAY, fill_opacity=.9).move_to(P((x0 + x1) / 2, (y0 + y1) / 2)))
            group.add(mn.Rectangle(width=s * MASONRY, height=s * (y1 - y0), stroke_width=0, fill_color=ink, fill_opacity=.95).move_to(P(-MASONRY / 2, (y0 + y1) / 2)))
        roof = DEPTH * 0.45 + MASONRY + t
        group.add(mn.Rectangle(width=s * roof, height=s * 0.3, stroke_width=0, fill_color=ink, fill_opacity=.95).move_to(P(-MASONRY - t + roof / 2, HEIGHT + .15)))
        if d > 0.01:
            group.add(mn.Rectangle(width=s * d, height=s * 0.3, stroke_width=0, fill_color=GRAY if ghost else WARN, fill_opacity=.95).move_to(P(-MASONRY - t - d / 2, HEIGHT + .15)))
        group.add(mn.Rectangle(width=s * (DEPTH * 0.45 + MASONRY + t), height=s * 0.3, stroke_width=0, fill_color=ink, fill_opacity=.95).move_to(
            P(-MASONRY - t + (DEPTH * 0.45 + MASONRY + t) / 2, -.15)))
        group.add(mn.Rectangle(width=s * 0.22, height=s * window, stroke_width=0, fill_color=GRAY if ghost else GLASS, fill_opacity=.95).move_to(P(-MASONRY - t, (sill + HEAD) / 2)))
        return group

    def reveal(self, *widgets, run_time: float = 0.5):
        """🌅 Fade live widgets in safely: they are refreshed to the current trackers first, so fading in after a fade-out works."""
        for widget in widgets:
            widget.update(0)
        return _engine.AnimationGroup(*[_engine.FadeIn(widget, run_time=run_time) for widget in widgets])

    def dismiss(self, *widgets, run_time: float = 0.5):
        """🌇 Fade widgets out; ``reveal`` brings them back."""
        return _engine.AnimationGroup(*[_engine.FadeOut(widget, run_time=run_time) for widget in widgets])

    def pulse(self, widget, *, color=INK, run_time: float = 1.0, buff: float = 0.12):
        """💓 Emphasize any widget, live or static, by flashing a static outline around its current extent (``Indicate`` on live widgets breaks)."""
        outline = _engine.SurroundingRectangle(widget, color=color, buff=buff, stroke_width=3)
        return _engine.Succession(_engine.FadeIn(outline, run_time=0.2 * run_time), _engine.Indicate(outline, color=color, scale_factor=1.05, run_time=0.6 * run_time),
                                  _engine.FadeOut(outline, run_time=0.2 * run_time))

    def ramp(self, low=None, high=None):
        """🎨 Colour function from the lowest (dark teal) to the highest (cream) electricity of the clip."""
        return ramp(low, high)

    def ramp_strip(self, colour, point=(0, 0, 0), width=3.5, height=0.16, *, unit="kWh/a", size=15, ticks=(), title=None):
        """🌈 Horizontal legend of a ramp with its end values and the unit; ``ticks`` (values inside the range) add marks with numbers, the unit then sits right of the strip."""
        mn, german = _engine, self._german()
        count = 48
        centre = np.array(point, dtype=float)
        strip = mn.VGroup(*[mn.Rectangle(width=width / count, height=height, stroke_width=0, fill_opacity=1,
                                         fill_color=colour(colour.low + (colour.high - colour.low) * (k + .5) / count)).move_to(
            centre + np.array([-width / 2 + width * (k + .5) / count, 0, 0])) for k in range(count)])
        strip.add(mn.Rectangle(width=width, height=height, color=INK, stroke_width=1).move_to(point))
        strip.add(place(text([Number(number(colour.low, 0, german))], size=size, color=INK), centre + np.array([-width / 2, -height - .12, 0]), "left"))
        strip.add(place(text([Number(number(colour.high, 0, german))], size=size, color=INK), centre + np.array([width / 2, -height - .12, 0]), "right"))
        span = colour.high - colour.low
        marks = [value for value in ticks if colour.low + 0.18 * span < value < colour.high - 0.18 * span]
        for value in marks:
            x = -width / 2 + width * (value - colour.low) / span
            strip.add(strokes([(centre + np.array([x, -height / 2, 0]), centre + np.array([x, -height / 2 - .07, 0]))], INK, 1.4),
                      text([Number(number(value, 0, german))], centre + np.array([x, -height - .12, 0]), size=size, color=INK))
        if marks:
            strip.add(place(text(unit, size=size, color=INK), centre + np.array([width / 2 + .12, 0, 0]), "left"))
        else:
            strip.add(text(unit, centre + np.array([0, -height - .12, 0]), size=size, color=INK))
        if title:
            heading = place(text(title, size=size, color=INK), centre + np.array([width / 2, height / 2 + .2, 0]), "right")
            strip.add(mn.Rectangle(width=heading.width + .12, height=heading.height + .1, stroke_width=0, fill_color=_art.P_DEEP_DARK, fill_opacity=1).move_to(heading.get_center()), heading)
        return strip

    def heat_map(self, axes, surface=None, *, colour=None, opacity=0.6, levels=0, labels=True, star=True, legend=None, legend_width=3.5, x_scale=100.0, columns=True, avoid=(), ticks=(), legend_title=None):
        """🗺️ Electricity over WWR × overhang in ``axes`` (x in % when ``x_scale`` is 100) as a bilinearly smoothed raster registered to whole pixels (``image`` is a group of one ``ImageMobject`` strip per WWR column, or with ``columns=False`` a single one), with contours, optimum star and a kWh/a strip.

        Contour labels stay inside the axes, never overlap each other and keep clear of the star and of every point in ``avoid`` (screen points, e.g. markers);
        ``ticks`` (kWh/a) are passed to the legend strip."""
        mn, german = _engine, self._german()
        surface = surface or _model.surface(41)
        colour = colour or ramp()
        xs, ys, grid = [x * x_scale for x in surface["wwr"]], surface["overhang"], surface["electricity"]
        floor, ceiling = colour.low, colour.high
        camera = self.camera
        to_x = lambda x: (axes.c2p(x, ys[0])[0] - camera.frame_center[0]) * camera.pixel_width / camera.frame_width + camera.pixel_width / 2
        to_y = lambda y: camera.pixel_height / 2 - (axes.c2p(xs[0], y)[1] - camera.frame_center[1]) * camera.pixel_height / camera.frame_height
        edges = [xs[0], *[(a + b) / 2 for a, b in zip(xs, xs[1:])], xs[-1]]
        cuts = [round(to_x(x)) for x in edges]
        top, bottom = round(to_y(ys[-1])), round(to_y(ys[0]))
        width, height = max(cuts[-1] - cuts[0], 1), max(bottom - top, 1)
        ix = np.interp(xs[0] + (np.arange(width) + .5) / width * (xs[-1] - xs[0]), xs, np.arange(len(xs)))
        iy = np.interp(ys[-1] - (np.arange(height) + .5) / height * (ys[-1] - ys[0]), ys, np.arange(len(ys)))
        i0, j0 = np.minimum(ix.astype(int), len(xs) - 2), np.minimum(iy.astype(int), len(ys) - 2)
        tx, ty = (ix - i0)[None, :], (iy - j0)[:, None]
        table = np.asarray(grid, dtype=float).T
        level = (table[j0][:, i0] * (1 - tx) * (1 - ty) + table[j0][:, i0 + 1] * tx * (1 - ty) + table[j0 + 1][:, i0] * (1 - tx) * ty + table[j0 + 1][:, i0 + 1] * tx * ty)
        lut = np.array([mn.ManimColor(colour(floor + (ceiling - floor) * k / 255)).to_rgb() for k in range(256)]) * 255
        pixels = np.empty((height, width, 4), dtype=np.uint8)
        pixels[..., :3] = np.rint(lut[np.clip(np.rint((level - floor) / (ceiling - floor) * 255), 0, 255).astype(int)])
        pixels[..., 3] = round(opacity * 255)
        slices = list(zip(edges, edges[1:], cuts, cuts[1:])) if columns else [(edges[0], edges[-1], cuts[0], cuts[-1])]
        image = mn.Group()
        for xa, xb, ca, cb in slices:
            first = min(ca - cuts[0], width - 1)
            strip = HeatStrip(np.ascontiguousarray(pixels[:, first:max(cb - cuts[0], first + 1)]), resampling_algorithm=mn.RESAMPLING_ALGORITHMS["linear"])
            low, high = axes.c2p(xa, ys[0]), axes.c2p(xb, ys[-1])
            image.add(strip.stretch_to_fit_width(high[0] - low[0]).stretch_to_fit_height(high[1] - low[1]).move_to((low + high) / 2))
        contours = mn.VGroup()
        optimum_at = axes.c2p(surface["minimum"]["design"]["wwr"] * x_scale, surface["minimum"]["design"]["overhang"])
        kept = [np.array(point, dtype=float) for point in avoid] + ([optimum_at] if star else [])
        boxes = []
        flat = [value for row_ in grid for value in row_]
        wanted = list(levels) if isinstance(levels, (list, tuple)) else nice_levels(min(flat), max(flat), levels) if levels else []
        for level in wanted:
            path = mn.VMobject(color=INK, stroke_width=1.4, stroke_opacity=.75)
            segments = _marching(xs, ys, grid, level)
            for (xa, ya), (xb, yb) in segments:
                path.start_new_path(axes.c2p(xa, ya))
                path.add_line_to(axes.c2p(xb, yb))
            contours.add(path)
            if labels and segments:
                share = wanted.index(level) / max(1, len(wanted) - 1)
                goal_x, goal_y = xs[0] + (xs[-1] - xs[0]) * (0.12 + 0.76 * share), ys[0] + (ys[-1] - ys[0]) * (0.85 - 0.7 * share)
                label = chip(parts(level, "kWh/a", 0, german), size=13, color=INK)
                half_w, half_h = label.width / 2, label.height / 2
                inside_l, inside_r = axes.c2p(xs[0], ys[0])[0] + half_w, axes.c2p(xs[-1], ys[0])[0] - half_w
                inside_b, inside_t = axes.c2p(xs[0], ys[0])[1] + half_h, axes.c2p(xs[0], ys[-1])[1] - half_h
                spot = lambda seg: np.array([min(max(axes.c2p((seg[0][0] + seg[1][0]) / 2, 0)[0], inside_l), inside_r),
                                             min(max(axes.c2p(0, (seg[0][1] + seg[1][1]) / 2)[1], inside_b), inside_t), 0.0])
                free = lambda at: all(not (abs(at[0] - other[0]) < half_w + other[2] + .1 and abs(at[1] - other[1]) < half_h + other[3] + .1) for other in boxes) and all(
                    not (abs(at[0] - p[0]) < half_w + .2 and abs(at[1] - p[1]) < half_h + .2) for p in kept)
                ranked = sorted(segments, key=lambda seg: abs((seg[0][0] + seg[1][0]) / 2 - goal_x) / (xs[-1] - xs[0]) + abs((seg[0][1] + seg[1][1]) / 2 - goal_y) / (ys[-1] - ys[0]))
                chosen = spot(next((seg for seg in ranked if free(spot(seg))), ranked[0]))
                boxes.append((chosen[0], chosen[1], half_w, half_h))
                contours.add(label.move_to(chosen))
        found = {"image": image, "contours": contours}
        if star:
            optimum = surface["minimum"]["design"]
            found["star"] = mn.Star(n=5, outer_radius=.14, inner_radius=.06, color=INK, stroke_width=1.5, fill_color=GOOD, fill_opacity=1).move_to(
                axes.c2p(optimum["wwr"] * x_scale, optimum["overhang"]))
        if legend is not None:
            if legend is True:
                legend_width = min(legend_width, axes.width * .45)
                legend = np.array([axes.get_right()[0] - legend_width / 2, axes.get_top()[1] + .5, 0])
            found["legend"] = self.ramp_strip(colour, legend, legend_width, ticks=ticks, title=legend_title)
        found["group"] = mn.Group(*found.values())
        found["ramp"] = colour
        return found


@lru_cache(maxsize=1)
def _SUN_PROTO():
    return _art.sun_glyph((0, 0, 0))


def strokes(segments, color=GRAY, width=1.4, opacity=1.0):
    """〰️ Many separate line segments drawn as one shape."""
    shape = _engine.VMobject(stroke_color=color, stroke_width=width, stroke_opacity=opacity, fill_opacity=0)
    for a, b in segments:
        shape.start_new_path(np.array(a, dtype=float))
        shape.add_line_to(np.array(b, dtype=float))
    return shape


def _hatch(box, spacing, project, color, width=1.2, opacity=.7):
    x0, y0, x1, y1 = box
    segments, c = [], x0 - y1
    while c < x1 - y0:
        low, high = max(x0, y0 + c), min(x1, y1 + c)
        if high - low > 1e-3:
            segments.append((project(low, low - c), project(high, high - c)))
        c += spacing
    return strokes(segments, color, width, opacity)


def label_size(scale: float) -> int:
    """🔠 Label font size that follows the drawing scale between 14 and 17 points (never smaller than 14 px at 1280 px width)."""
    return min(17, max(14, round(28 * scale + 3)))


def _overlap(first, second, margin=0.05):
    return (first.get_left()[0] < second.get_right()[0] + margin and first.get_right()[0] > second.get_left()[0] - margin
            and first.get_bottom()[1] < second.get_top()[1] + margin and first.get_top()[1] > second.get_bottom()[1] - margin)


def _leader(item, target, color):
    return _engine.Line(item.get_right() + np.array([0.02, 0, 0]), np.array(target, dtype=float), color=color, stroke_width=1.4, stroke_opacity=.8)


def _cooler(centre, scale, color):
    box = _engine.Rectangle(width=1.0 * scale, height=0.4 * scale, color=color, stroke_width=2, fill_color=_art.P_DEEP_DARK, fill_opacity=1).move_to(centre)
    flake = _engine.VGroup(*[_engine.Line(centre + 0.13 * scale * np.array([math.cos(a), math.sin(a), 0]), centre - 0.13 * scale * np.array([math.cos(a), math.sin(a), 0]),
                                          color=color, stroke_width=2) for a in (0, math.pi / 3, 2 * math.pi / 3)])
    return _engine.VGroup(box, flake)


def sun_marker(point, scale, opacity):
    """☀️ A small sun disc for plan arcs."""
    return clone(_SUN_PROTO()).scale(scale).move_to(point).set_opacity(opacity)


@lru_cache(maxsize=1)
def reference_scale() -> float:
    """📏 Default full scale of the electricity bars: the largest electricity of the reference grid."""
    return reference()["electricity"]
