"""✂️ Orthographic sections and elevations of the Villa Savoye plus the roof build-up and slab strip details."""

from __future__ import annotations

import importlib
import math

import numpy as np

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye."
_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_s = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, DATA, GOOD, WARN, GLASS, BACKGROUND, CONCRETE, SERV = (_s.INK, _s.MUTED, _s.DATA, _s.GOOD, _s.WARN, _s.GLASS, _s.BACKGROUND, _s.CONCRETE, _s.SERV)
STRUCT = _d.STRUCT
P = _d.P
W_CUT, W_EDGE, W_THIN, W_HAIR = _d.W_CUT, _d.W_EDGE, _d.W_THIN, _d.W_HAIR
WALL_FILL = _d.mix(INK, BACKGROUND, 0.12)
GREEN_FILL = _d.mix(_d.GOOD, BACKGROUND, 0.42)
BEYOND = _s.BEYOND_FILL
MIN_LAYER = 0.014
EPS = 1e-9


def rect(h0, h1, z0, z1) -> list:
    """▭ Corner list ``(h, z)`` of a rectangle."""
    return [(h0, z0), (h1, z0), (h1, z1), (h0, z1)]


def segment_quad(first, last, half_width: float) -> list:
    """▰ Four model-space corners of a finite-width segment, with no degenerate band."""
    a, b = np.asarray(first, dtype=float)[:2], np.asarray(last, dtype=float)[:2]
    delta = b - a
    length = float(np.linalg.norm(delta))
    if length <= EPS or half_width <= 0:
        raise ValueError("A band requires a nonzero segment and a positive half-width")
    normal = np.array([-delta[1], delta[0]]) * (half_width / length)
    return [point.tolist() for point in (a + normal, b + normal, b - normal, a - normal)]


class Section(_s.SheetMixin):
    """✂️ Scene mixin: ``villa_section``, ``villa_elevation``, ``roof_detail``, ``slab_strip``."""

    def _pair(self, look, item, fill: str, stroke: str) -> tuple:
        """🎨 ``(fill, stroke, opacity, extra width)`` of an element for its selection state."""
        state = look.state(item)
        if state == "highlight":
            colour = look.highlight_color(item)
            return colour, colour, 1.0, 0.8
        if state == "ghost":
            return fill, stroke, 0.22, 0.0
        if state == "dim":
            return _d.mix(fill, BACKGROUND, look.dim), _d.mix(stroke, BACKGROUND, look.dim), 1.0, 0.0
        return fill, stroke, 1.0, 0.0

    def _shape(self, ring, fill, fill_opacity, stroke, width, opacity=1.0):
        """⬟ A filled, outlined polygon with an overall opacity."""
        return _d.polygon(ring, fill, fill_opacity * opacity, stroke, width, opacity)

    def _levels(self, d, to, g, h0, h1, hatch_h, *, lines: bool = True, thumbnail: bool = False):
        """🔻 Level marks ±0.00 / +3.50 / +6.70 / +9.40 right of the drawing with faint level lines across it."""
        group = _engine.VGroup()
        x_mark = to(h1, 0.0)[0] + 0.55
        values = [("ground", g["storeys"]["ground"]), ("first", g["storeys"]["first"]), ("roof", g["storeys"]["roof"]), ("top", g["box"]["height"])]
        for tag, value in values:
            y = to(h0, value)[1]
            if lines:
                group.add(_d.segments(_s.dash_dot((to(h0, value)[0] - 0.2, y, 0), (x_mark, y, 0)), MUTED, W_HAIR, 0.45))
            mark = self._level((x_mark, y, 0), value)
            group.add(mark)
            d.parts["level:" + tag] = mark
        d.parts["levels"] = group
        d.add(group)

    def _section_ground(self, d, to, h0, h1):
        """🌱 Ground strip with hatch and ground line."""
        strip = _s.ground_strip(self, to, h0, h1, 0.5)
        d.parts["ground"] = strip
        d.add(strip)

    def _section_plane_items(self, ctx):
        """🧭 Section helpers bound to a plane and a projection."""

    def villa_section(self, point=(0, 0, 0), scale: float = 0.3, *, model=None, cut="salon", highlight=(), ghost=(), highlight_colors=None,
                      layers=("structure", "architecture"), buildup: bool = True, labels: bool = True, names=None, levels: bool = True, level_lines: bool = True,
                      dims=(), colors: str = "material", dim: float = 0.45, centre=None, window=None, thumbnail: bool = False):
        """✂️ Orthographic section: hatched concrete cuts, roof build-up layers live from the model, ribbon-window glass, pilotis cut vs beyond, ground enclosure, level marks.

        ``cut`` is ``"salon"`` (through piloti axis 2, the boiler, the salon and the solarium), ``"ramp"`` (through a ramp flight), ``"terrace"`` (axis 4), ``"row:3"`` / ``"row:C"``, a number (x) or ``("x"|"y", value)``.
        An x-cut looks west with south at the left; a y-cut looks north with west at the left. ``point`` is the screen position of the drawing centre (or of ``centre`` = ``(h, z)`` in metres).
        ``layers`` picks ``"structure"``, ``"architecture"``, ``"services"``; ``buildup`` draws the roof layers (waterproofing, screed, soil in the planted zone) with a minimum visible thickness.
        ``highlight`` / ``ghost`` take tags, classes or kinds; ``highlight_colors`` one colour per tag. ``at(h, z)`` maps drawing coordinates, ``at(x, y, z)`` model points.
        Returns a ``Sheet`` with ``parts[tag]`` (pilotis, ``W-*``, ``RW-*``, ``SL-1``, ``SL-R``, ``SL-R:<layer>``, ``SC-1``, ``CW-1``, space tags), ``parts["levels"]``, ``parts["ground"]``."""
        g = _c.geometry(model)
        if thumbnail:
            labels = levels = level_lines = False
            dims, names = (), False
        names = bool(names)
        language = self.language()
        plane = _s.parse_cut(cut, g)
        box = g["box"]
        hmax = box["width"] if plane.axis == "x" else box["length"]
        hc, zc = centre if centre is not None else (hmax / 2, box["height"] / 2)
        view = _d.View("section", point, scale, (0.0, hc, zc) if plane.axis == "x" else (hc, 0.0, zc), axis="y" if plane.axis == "x" else "x", cut=plane.value)
        d = _s.Sheet(view, "section")
        d.plane = plane.spec
        base = P(point)

        def to(h, z):
            return np.array([base[0] + scale * (h - hc), base[1] + scale * (z - zc), 0.0])

        look = _s.Look(_s.expand_names(highlight, g), _s.expand_names(ghost, g), dim, highlight_colors, colors)
        catalogue = _s.items(model)
        use = set(layers) if not isinstance(layers, bool) else {"structure", "architecture"}
        if layers is True:
            buildup = True
        entries: list = []

        def put(stage: float, depth: float, tag: str, group, aliases=()):
            entries.append((stage, depth, len(entries), tag, group, aliases))

        ctx = {"g": g, "plane": plane, "to": to, "look": look, "put": put, "catalogue": catalogue, "scale": scale, "window": window, "buildup": buildup, "use": use,
               "h0": -1.5, "h1": hmax + 1.5}
        if use & {"structure", "architecture"}:
            self._sec_slabs(ctx)
        if "structure" in use:
            self._sec_pilotis(ctx)
            self._sec_ramps(ctx)
        if "architecture" in use:
            self._sec_walls(ctx)
            self._sec_enclosure(ctx)
            self._sec_screens(ctx)
        if "services" in use:
            self._sec_services(ctx)
        self._section_ground(d, to, ctx["h0"], ctx["h1"])
        for stage, depth, _, tag, group, aliases in sorted(entries, key=lambda row: (row[0], row[1], row[2])):
            d.register(tag, group)
            for name in aliases:
                d.alias(name, tag)
        if "SL-R" in d.parts:
            d.alias("slab:roof", "SL-R")
        if "SL-1" in d.parts:
            d.alias("slab:first", "SL-1")
        layer_parts = [v for k, v in d.parts.items() if k.startswith("SL-R:")]
        if layer_parts:
            d.parts["roof_layers"] = _engine.VGroup(*layer_parts)
        if levels:
            self._levels(d, to, g, ctx["h0"], hmax, 0.0, lines=level_lines)
        if dims:
            self._sec_dims(d, to, g, dims, ctx)
        if names:
            self._sec_names(d, ctx, language)
        return d

    def _sec_dims(self, d, to, g, dims, ctx):
        """↔️ Storey-height chain and overall height left of the section."""
        group = _engine.VGroup()
        x = to(ctx["h0"], 0.0)[0] - 0.35
        marks = [0.0, g["storeys"]["first"], g["storeys"]["roof"], g["box"]["height"]]
        if "levels" in dims:
            for a, b in zip(marks[:-1], marks[1:]):
                group.add(self._dim((x, to(0, a)[1], 0), (x, to(0, b)[1], 0), self.number(b - a, 2), offset=0.0, color=MUTED))
        if "overall" in dims:
            group.add(self._dim((x, to(0, 0.0)[1], 0), (x, to(0, marks[-1])[1], 0), self.quantity(marks[-1], "m", 2), offset=0.7 if "levels" in dims else 0.0, color=INK))
        d.parts["dims"] = group
        d.add(group)

    def _sec_names(self, d, ctx, language):
        """🏷️ Room names inside the cut rooms where free space exists."""
        g, plane, to, look = ctx["g"], ctx["plane"], ctx["to"], ctx["look"]
        points = _s.cloud(d)
        bands = {"ground": (0.0, g["slabs"][0]["top"] - g["slabs"][0]["thickness"]), "first": (g["storeys"]["first"], g["walls"][0]["top"]), "roof": (g["storeys"]["roof"], g["box"]["height"])}
        for space in g["spaces"]:
            z0, z1 = bands[space["storey"]]
            for h0, h1 in plane.intervals([space["outline"]]):
                item = _s.pseudo(space["tag"], "IfcSpace", "spaces", space["guid"])
                colour = _d.mix(INK, BACKGROUND, 0.2 if look.state(item) != "dim" else look.dim)
                label = self._t(_s.short_name(space["name"], language), (0, 0, 0), _d.S_SMALL, colour)
                ring = [to(h0, z0), to(h1, z0), to(h1, z1), to(h0, z1)]
                spot = _s.find_spot(points, label.width, label.height, ring, (to((h0 + h1) / 2, z0 + (z1 - z0) * 0.62)))
                if spot is None:
                    continue
                label.move_to(np.array([spot[0], spot[1], 0.0]))
                points = np.vstack([points, _s.cloud(label)])
                d.register(space["tag"], _engine.VGroup(label))
                break

    def _cut_poche(self, ring, fill, stroke, opacity, bonus, hatch=True, cross=False):
        """⬛ A cut area: hatched when large enough, solid when thin."""
        return _s.poche([ring], fill, hatch=hatch, cross=cross, stroke=stroke, width=W_CUT + bonus, opacity=opacity)

    def _sec_slabs(self, ctx):
        """🟫 First-floor slab and roof slab cut with hatched concrete; roof build-up layers by zone."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        for slab in g["slabs"]:
            item = _s.pseudo(slab["tag"], "IfcSlab", "slabs", slab["guid"], ("architecture", "structure"))
            fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
            rings = [slab["outline"]] + list(slab["voids"])
            spans = plane.intervals(rings)
            ctx["h0"], ctx["h1"] = min(ctx["h0"], -1.5), max(ctx["h1"], 0.0)
            z0 = slab["top"] - slab["thickness"]
            group = _engine.VGroup()
            for h0, h1 in spans:
                group.add(self._cut_poche([to(h, z) for h, z in rect(h0, h1, z0, slab["top"])], fill, stroke, opacity, bonus))
            put(2.0, 0.0, slab["tag"], group)
            if slab["kind"] != "ROOF" or not ctx["buildup"] or "layers" not in slab:
                continue
            solarium = plane.intervals([slab["zones"]["solarium"]]) if slab.get("zones", {}).get("solarium") else []
            z = slab["top"]
            for layer in slab["layers"][1:]:
                key = layer["id"]
                thickness = max(layer["thickness"], MIN_LAYER / ctx["scale"])
                zones = set(layer.get("zones", []))
                if zones == {"planted"}:
                    ranges = _s.subtract(spans, solarium)
                elif zones == {"solarium"}:
                    ranges = _s.intersect(spans, solarium)
                else:
                    ranges = spans
                colour = _c.layer_color(key)
                lg = _engine.VGroup()
                for h0, h1 in ranges:
                    ring = [to(h, zz) for h, zz in rect(h0, h1, z, z + thickness)]
                    lg.add(_d.polygon(ring, _d.mix(colour, BACKGROUND, 0.1 if key != "waterproofing" else 0.0), 0.9 * opacity, stroke if look.state(item) == "highlight" else colour, W_THIN, opacity))
                    if key == "soil":
                        lg.add(_d.hatch(ring, GOOD, 0.045, 30.0, W_HAIR, 0.8 * opacity))
                if len(lg.submobjects):
                    put(2.1, 0.0, "SL-R:" + key, lg)
                z += thickness

    def _beyond(self, ctx, item, ring_hz, tag, depth, fill=None, stroke=None, opacity_fill=1.0):
        """🫥 A surface beyond the cut: dark fill with a thin outline."""
        look, to = ctx["look"], ctx["to"]
        f, s, opacity, bonus = self._pair(look, item, fill or BEYOND, stroke or INK)
        group = _engine.VGroup(self._shape([to(h, z) for h, z in ring_hz], f, opacity_fill, s, W_THIN + bonus * 0.5, 0.7 * opacity))
        ctx["put"](1.0, depth, tag, group)
        return group

    def _sec_pilotis(self, ctx):
        """⚪ Pilotis of the cut row as hatched rectangles, pilotis beyond as thin outlines."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        cut_h = []
        for item in ctx["catalogue"]:
            if item["kind"] != "pilotis":
                continue
            data = item["data"]
            p = (data["x"], data["y"])
            s, r, h = plane.s(p), data["radius"], plane.h(p)
            kind = plane.classify(s - r, s + r)
            if kind == "cut":
                half = math.sqrt(max(r * r - s * s, 0.0))
                cut_h.append(h)
                fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
                ring = [to(a, b) for a, b in rect(h - half, h + half, data["base"], data["top"])]
                put(2.0, 0.0, item["tag"], self._cut_poche(ring, fill, stroke, opacity, bonus))
        for item in ctx["catalogue"]:
            if item["kind"] != "pilotis":
                continue
            data = item["data"]
            p = (data["x"], data["y"])
            s, r, h = plane.s(p), data["radius"], plane.h(p)
            if plane.classify(s - r, s + r) == "beyond" and not any(abs(h - c) < r * 0.9 for c in cut_h):
                self._beyond(ctx, item, rect(h - r, h + r, data["base"], data["top"]), item["tag"], s)

    def _sec_ramps(self, ctx):
        """⏫ Ramp flights cut along their slope or seen beyond, landings as slabs."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        thickness = g["slabs"][0]["thickness"]
        for ramp in g["ramp"]["ramps"]:
            item = next((it for it in ctx["catalogue"] if it["tag"] == ramp["tag"]), None) or _s.pseudo(ramp["tag"], "IfcRamp", "ramp", ramp["guid"])
            for flight in ramp["flights"]:
                s, e = np.array(flight["start"], float), np.array(flight["end"], float)
                d = (e - s) / np.linalg.norm(e - s)
                n = np.array([-d[1], d[0]])
                half = flight["width"] / 2
                ring = [s + n * half, e + n * half, e - n * half, s - n * half]
                kind = plane.classify(*plane.extent(ring))
                if kind == "removed":
                    continue
                hs, he = plane.h(s), plane.h(e)
                fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
                z0, z1 = flight["z0"], flight["z1"]
                if abs(he - hs) > abs(s[0] - e[0]) * 0 + 1e-6 and abs(he - hs) > 0.5 * float(np.linalg.norm(e - s)):
                    band = [to(hs, z0), to(he, z1), to(he, z1 - thickness), to(hs, z0 - thickness)]
                    if kind == "cut":
                        put(2.0, 0.0, flight["tag"], self._cut_poche(band, fill, stroke, opacity, bonus))
                    else:
                        put(1.0, plane.extent(ring)[1], flight["tag"], _engine.VGroup(self._shape(band, BEYOND, 1.0, stroke, W_THIN, 0.7 * opacity)))
                elif kind == "cut":
                    t = (plane.value - s[0]) / (e[0] - s[0]) if plane.index == 0 else (plane.value - s[1]) / (e[1] - s[1])
                    z = z0 + (z1 - z0) * min(max(t, 0.0), 1.0)
                    h0, h1 = plane.hrange(ring)
                    put(2.0, 0.0, flight["tag"], self._cut_poche([to(h, zz) for h, zz in rect(h0, h1, z - thickness, z)], fill, stroke, opacity, bonus))
                else:
                    h0, h1 = plane.hrange(ring)
                    put(1.0, plane.extent(ring)[1], flight["tag"], _engine.VGroup(self._shape([to(h, zz) for h, zz in rect(h0, h1, z0 - thickness, z1)], BEYOND, 1.0, stroke, W_THIN, 0.7 * opacity)))
            for landing in ramp["landings"]:
                ring = landing["outline"]
                kind = plane.classify(*plane.extent(ring))
                h0, h1 = plane.hrange(ring)
                if kind == "cut":
                    fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
                    put(2.0, 0.0, ramp["tag"] + ":landing", self._cut_poche([to(h, zz) for h, zz in rect(*[min(plane.intervals([ring]) or [(h0, h1)])[0], max(plane.intervals([ring]) or [(h0, h1)])[1]], landing["z"] - thickness, landing["z"])], fill, stroke, opacity, bonus))

    def _wall_ring(self, wall):
        """▭ Plan rectangle of a wall (its path is the axis)."""
        a, b = np.array(wall["start"], float), np.array(wall["end"], float)
        d = (b - a) / np.linalg.norm(b - a)
        n = np.array([-d[1], d[0]]) * wall["thickness"] / 2
        return [a + n, b + n, b - n, a - n], a, d

    def _sec_walls(self, ctx):
        """⬜ First-floor walls cut with a window gap and glass lines, or seen beyond as interior elevations with the ribbon window band."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        for item in ctx["catalogue"]:
            if item["kind"] != "walls":
                continue
            wall = item["data"]
            ring, a, d = self._wall_ring(wall)
            kind = plane.classify(*plane.extent(ring))
            if kind == "removed":
                continue
            fill, stroke, opacity, bonus = self._pair(look, item, WALL_FILL, INK)
            window = next(it for it in ctx["catalogue"] if it["kind"] == "windows" and it["data"]["wall"] == wall["tag"])
            op = window["data"]
            if kind == "cut":
                spans = plane.intervals([ring])
                group = _engine.VGroup()
                glass = _engine.VGroup()
                for h0, h1 in spans:
                    t = float((self._cross_point(a, d, plane) - a) @ d)
                    gap = op["start"] <= t <= op["end"]
                    pieces = [(wall["base"], op["sill_z"]), (op["head_z"], wall["top"])] if gap else [(wall["base"], wall["top"])]
                    for z0, z1 in pieces:
                        group.add(self._cut_poche([to(h, z) for h, z in rect(h0, h1, z0, z1)], fill, stroke, opacity, bonus, hatch=True, cross=True))
                    if gap:
                        g_fill, g_stroke, g_opacity, g_bonus = self._pair(look, window, GLASS, GLASS)
                        for h in (h0 + (h1 - h0) * 0.3, h0 + (h1 - h0) * 0.7):
                            glass.add(_d.polyline([to(h, op["sill_z"]), to(h, op["head_z"])], g_stroke, W_EDGE + g_bonus, g_opacity))
                        glass.add(_d.segments([(to(h0, op["sill_z"]), to(h1, op["sill_z"])), (to(h0, op["head_z"]), to(h1, op["head_z"]))], g_stroke, W_THIN, g_opacity))
                put(2.0, 0.0, wall["tag"], group)
                if len(glass.submobjects):
                    put(2.2, 0.0, window["tag"], glass, aliases=(op["tag"],))
            else:
                h0, h1 = plane.hrange(ring)
                depth = plane.extent(ring)[1]
                self._beyond(ctx, item, rect(h0, h1, wall["base"], wall["top"]), wall["tag"], depth)
                ph = [plane.h(op["p0"]), plane.h(op["p1"])]
                g_fill, g_stroke, g_opacity, g_bonus = self._pair(look, window, GLASS, GLASS)
                band = [to(h, z) for h, z in rect(min(ph), max(ph), op["sill_z"], op["head_z"])]
                glass = _engine.VGroup(self._shape(band, _d.mix(GLASS, BACKGROUND, 0.82), 1.0, g_stroke, W_THIN + g_bonus * 0.5, g_opacity))
                put(1.0, depth + 1e-3, window["tag"], glass, aliases=(op["tag"],))

    def _cross_point(self, a, d, plane):
        """✖️ Point where a wall axis meets the cutting plane."""
        if abs(d[plane.index]) < 1e-9:
            return a
        t = (plane.value - a[plane.index]) / d[plane.index]
        return a + d * t

    def _sec_enclosure(self, ctx):
        """🟩 Dark-green set-back ground-floor walls cut or beyond, and the curved glass wall as thin lines."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        walls = _s.ground_walls(g)
        height = walls["height"]
        owners: dict = {}
        for a, b, tag in walls["outer"] + walls["inner"]:
            ring = segment_quad(a, b, walls["thickness"] / 2)
            kind = plane.classify(*plane.extent(ring))
            if kind == "removed":
                continue
            item = _s.pseudo(tag, "IfcSpace", "spaces", tag)
            fill, stroke, opacity, bonus = self._pair(look, item, GREEN_FILL, INK)
            if kind == "cut":
                for h0, h1 in plane.intervals([ring]):
                    owners.setdefault((2.0, tag), _engine.VGroup()).add(self._cut_poche([to(h, z) for h, z in rect(h0, h1, 0.0, height)], fill, stroke, opacity, bonus, hatch=False))
            else:
                h0, h1 = plane.hrange(ring)
                owners.setdefault((1.0, tag, plane.extent(ring)[1]), _engine.VGroup()).add(self._shape([to(h, z) for h, z in rect(h0, h1, 0.0, height)], _d.mix(fill, BACKGROUND, 0.45), 1.0, stroke, W_THIN, 0.55 * opacity))
        for key, group in owners.items():
            put(key[0], key[2] if len(key) > 2 else 0.0, key[1], group)
        item = next((it for it in ctx["catalogue"] if it["kind"] == "curtain"), None)
        if item is None:
            return
        fill, stroke, opacity, bonus = self._pair(look, item, GLASS, GLASS)
        cut_group, back = _engine.VGroup(), []
        path = item["data"]["path"]
        for a, b in zip(path[:-1], path[1:]):
            ring = segment_quad(a, b, 0.04)
            kind = plane.classify(*plane.extent(ring))
            if kind == "removed":
                continue
            if kind == "cut":
                for h0, h1 in plane.intervals([ring]):
                    cut_group.add(_d.polyline([to(h0, 0.0), to(h0, item["data"]["top"])], stroke, (W_EDGE + bonus), opacity), _d.polyline([to(h1, 0.0), to(h1, item["data"]["top"])], stroke, W_THIN + bonus, opacity))
            else:
                h0, h1 = plane.hrange(ring)
                back.append((plane.extent(ring)[1], h0, h1))
        if back:
            group = _engine.VGroup()
            for depth, h0, h1 in sorted(back):
                group.add(_d.polygon([to(h, z) for h, z in rect(h0, h1, 0.0, item["data"]["top"])], GLASS, 0.1 * opacity, stroke, W_HAIR + 0.2, 0.55 * opacity))
            put(1.0, -0.001, "CW-1:beyond", group)
        if len(cut_group.submobjects):
            put(2.1, 0.0, "CW-1", cut_group)
        elif back:
            ctx["put"](1.0, -0.001, "CW-1", _engine.VGroup())

    def _sec_screens(self, ctx):
        """🌀 Solarium screens cut as hatched strips or beyond as the curved wall face."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        for item in ctx["catalogue"]:
            if item["kind"] != "screens":
                continue
            data = item["data"]
            fill, stroke, opacity, bonus = self._pair(look, item, WALL_FILL, INK)
            cut_group, back = _engine.VGroup(), []
            path = data["path"]
            for a, b in zip(path[:-1], path[1:]):
                ring = segment_quad(a, b, data["thickness"] / 2)
                kind = plane.classify(*plane.extent(ring))
                if kind == "removed":
                    continue
                if kind == "cut":
                    for h0, h1 in plane.intervals([ring]):
                        cut_group.add(self._cut_poche([to(h, z) for h, z in rect(h0, h1, data["base"], data["top"])], fill, stroke, opacity, bonus))
                else:
                    back.append((plane.extent(ring)[1], *plane.hrange(ring)))
            group = _engine.VGroup()
            for depth, h0, h1 in sorted(back):
                group.add(self._shape([to(h, z) for h, z in rect(h0, h1, data["base"], data["top"])], BEYOND, 1.0, stroke, W_HAIR + 0.2, 0.7 * opacity))
            if len(group.submobjects):
                put(1.0, min(row[0] for row in back), item["tag"] + ":beyond", group)
            if len(cut_group.submobjects):
                put(2.0, 0.0, item["tag"], cut_group)
            elif len(group.submobjects):
                ctx["put"](1.0, 0.0, item["tag"], _engine.VGroup())

    def _sec_services(self, ctx):
        """🔧 Boiler, riser, pipes and radiator as thin prisms in the services colour."""
        plane, to, look, put = ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        for item in ctx["catalogue"]:
            if item["kind"] != "services":
                continue
            data = item["data"]
            fill, stroke, opacity, bonus = self._pair(look, item, SERV, SERV)
            cut_group, back = _engine.VGroup(), _engine.VGroup()
            for ring, z0, z1 in service_prisms(data):
                kind = plane.classify(*plane.extent(ring))
                if kind == "removed":
                    continue
                h0, h1 = plane.hrange(ring)
                shape = self._shape([to(h, z) for h, z in rect(h0, h1, z0, z1)], fill, 0.35, stroke, W_EDGE + bonus, opacity)
                (cut_group if kind == "cut" else back).add(shape)
            if len(back.submobjects):
                put(1.0, 0.0, item["tag"], back)
            if len(cut_group.submobjects):
                put(2.3, 0.0, item["tag"], cut_group)

    def villa_elevation(self, face: str = "south", point=(0, 0, 0), scale: float = 0.33, *, model=None, highlight=(), ghost=(), highlight_colors=None,
                        layers=("structure", "architecture"), levels: bool = True, dims=(), colors: str = "material", dim: float = 0.45, centre=None,
                        window=None, thumbnail: bool = False, ground: bool = True):
        """🏠 Orthographic elevation: dark-green set-back ground floor with curved glass, pilotis, white box with the ribbon window band, roof edge, solarium screens, ground line, level marks.

        ``face`` is ``"south"``, ``"north"``, ``"east"`` or ``"west"``; ``point`` is the screen position of the box centre (or ``centre`` = ``(h, z)`` in metres).
        ``highlight`` / ``ghost`` take tags, classes or kinds; ``highlight_colors`` one colour per tag. Returns a ``Sheet`` with ``parts[tag]`` for pilotis, ``W-*``, ``RW-*`` (alias ``OP-*``), ``SL-1``, ``SL-R``, ``SC-1``, ``CW-1``, space tags, ``parts["levels"]``, ``parts["ground"]``."""
        g = _c.geometry(model)
        if thumbnail:
            levels = False
            dims = ()
        box = g["box"]
        horizontal = box["length"] if face in ("south", "north") else box["width"]
        hc, zc = centre if centre is not None else (horizontal / 2, box["height"] / 2)
        cx, cy = (hc, box["width"] / 2) if face in ("south", "north") else (box["length"] / 2, hc)
        view = _d.View("elevation", point, scale, (cx, cy, zc), face=face)
        d = _s.Sheet(view, "elevation")
        d.face = face
        look = _s.Look(_s.expand_names(highlight, g), _s.expand_names(ghost, g), dim, highlight_colors, colors)
        catalogue = _s.items(model)
        use = set(layers)
        faces: list = []
        base = P(point)

        def to(h, z):
            return np.array([base[0] + scale * ((h - hc) * (1 if face in ("south", "east") else -1)), base[1] + scale * (z - zc), 0.0])

        def add_face(tag, item, a, b, z0, z1, normal, fill, stroke, kind="wall", extra=None):
            facing = view.facing((normal[0], normal[1], 0.0))
            if facing <= 1e-3:
                return
            mid = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
            faces.append({"tag": tag, "item": item, "a": a, "b": b, "z0": z0, "z1": z1, "facing": facing, "fill": fill, "stroke": stroke, "kind": kind,
                          "depth": view.depth(mid[0], mid[1], (z0 + z1) / 2), "extra": extra})

        centre_xy = np.array([box["length"] / 2, box["width"] / 2])
        if "architecture" in use:
            walls = _s.ground_walls(g)
            for a, b, tag in walls["outer"]:
                dv = np.array(b) - np.array(a)
                n = np.array([dv[1], -dv[0]]) / np.linalg.norm(dv)
                add_face(tag, _s.pseudo(tag, "IfcSpace", "spaces", tag), a, b, 0.0, walls["height"], n, GREEN_FILL, INK)
            curtain = next((it for it in catalogue if it["kind"] == "curtain"), None)
            if curtain is not None:
                path = curtain["data"]["path"]
                inside = np.mean([np.array(s["outline"]).mean(axis=0) for s in g["spaces"] if s["tag"] == "SP-HALL"], axis=0) if any(s["tag"] == "SP-HALL" for s in g["spaces"]) else centre_xy
                for a, b in zip(path[:-1], path[1:]):
                    dv = np.array(b) - np.array(a)
                    n = np.array([dv[1], -dv[0]]) / np.linalg.norm(dv)
                    if (np.array(a) + np.array(b)) / 2 @ n - inside @ n < 0:
                        n = -n
                    add_face("CW-1", curtain, a, b, 0.0, curtain["data"]["top"], n, GLASS, GLASS, "glass")
            for item in catalogue:
                if item["kind"] == "walls":
                    wall = item["data"]
                    ring, a, dv = self._wall_ring(wall)
                    mid = (np.array(wall["start"]) + np.array(wall["end"])) / 2
                    n = mid - centre_xy
                    n = n / np.linalg.norm(n)
                    n = np.array([round(n[0]), round(n[1])], float) if abs(n[0]) != abs(n[1]) else n
                    half = wall["thickness"] / 2
                    a_o, b_o = np.array(wall["start"]) + n * half, np.array(wall["end"]) + n * half
                    window_item = next(it for it in catalogue if it["kind"] == "windows" and it["data"]["wall"] == wall["tag"])
                    add_face(wall["tag"], item, a_o, b_o, wall["base"], wall["top"], n, WALL_FILL, INK, "wall", {"window": window_item, "n": n, "half": half})
                elif item["kind"] == "screens":
                    data = item["data"]
                    inner = np.array(data["path"]).mean(axis=0)
                    for a, b in zip(data["path"][:-1], data["path"][1:]):
                        dv = np.array(b) - np.array(a)
                        n = np.array([dv[1], -dv[0]]) / np.linalg.norm(dv)
                        if ((np.array(a) + np.array(b)) / 2 - inner) @ n < 0:
                            n = -n
                        facing = view.facing((n[0], n[1], 0.0))
                        if facing < 0:
                            n = -n
                        add_face(item["tag"], item, a, b, data["base"], data["top"], n, WALL_FILL, INK, "screen")
        if use & {"structure", "architecture"}:
            for slab in g["slabs"]:
                item = _s.pseudo(slab["tag"], "IfcSlab", "slabs", slab["guid"], ("architecture", "structure"))
                ring = slab["outline"]
                for a, b in zip(ring, ring[1:] + ring[:1]):
                    dv = np.array(b) - np.array(a)
                    n = np.array([dv[1], -dv[0]]) / np.linalg.norm(dv)
                    add_face(slab["tag"], item, a, b, slab["top"] - slab["thickness"], slab["top"], n, CONCRETE, INK, "slab")
        if "structure" in use:
            for item in catalogue:
                if item["kind"] == "pilotis":
                    data = item["data"]
                    if not _c.inside(window, [(data["x"], data["y"])], 0.5):
                        continue
                    faces.append({"tag": item["tag"], "item": item, "x": data["x"], "y": data["y"], "r": data["radius"], "z0": data["base"], "z1": data["top"],
                                  "kind": "piloti", "depth": view.depth(data["x"], data["y"], 0.0), "facing": 1.0})
        if "services" in use:
            for item in catalogue:
                if item["kind"] == "services" and item["data"]["kind"] == "downpipe":
                    data = item["data"]
                    x, y = data["points"][0][0], data["points"][0][1]
                    faces.append({"tag": item["tag"], "item": item, "x": x, "y": y, "z0": min(p[2] for p in data["points"]), "z1": max(p[2] for p in data["points"]),
                                  "kind": "pipe", "depth": view.depth(x, y, 0.0) + 0.05, "facing": 1.0})
        if ground:
            strip = _s.ground_strip(self, to, -1.5, horizontal + 1.5, 0.5)
            d.parts["ground"] = strip
            d.add(strip)
        members: dict = {}
        for f in sorted(faces, key=lambda row: row["depth"]):
            group = self._elev_face(f, to, view, look, scale)
            d.add(group)
            members.setdefault(f["tag"], []).append(group)
            if f.get("window_group") is not None:
                d.add(f["window_group"])
                window = f["extra"]["window"]
                members.setdefault(window["tag"], []).append(f["window_group"])
                members.setdefault(window["opening"], []).append(f["window_group"])
            if f["kind"] == "piloti":
                d.anchors[f["tag"]] = view(f["x"], f["y"], (f["z0"] + f["z1"]) / 2)
        for tag, groups in members.items():
            d.parts[tag] = _engine.VGroup(*groups)
        if "CW-1" in d.parts:
            d.alias("curtain_wall", "CW-1")
        if levels:
            self._levels(d, to, g, -1.5, horizontal, 0.0, lines=False)
        if dims:
            group = _engine.VGroup()
            y = to(0, -0.5)[1] - 0.15
            if "overall" in dims:
                group.add(self._dim((to(0, 0)[0], y, 0), (to(horizontal, 0)[0], y, 0), self.quantity(horizontal, "m", 2), offset=-0.2, color=INK))
            d.parts["dims"] = group
            d.add(group)
        return d

    def _elev_face(self, f, to, view, look, scale):
        """🧱 One painted elevation face; sets ``window_group`` on the face record for walls with a ribbon window."""
        item = f["item"]
        shade = 0.45 + 0.55 * f["facing"]
        if f["kind"] == "piloti":
            x, y, r = f["x"], f["y"], f["r"]
            b, t = view(x, y, f["z0"]), view(x, y, f["z1"])
            w = max(r * scale, 0.02)
            fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
            body = _d.polygon([b + (-w, 0, 0), b + (w, 0, 0), t + (w, 0, 0), t + (-w, 0, 0)], _d.mix(BACKGROUND, fill, 0.7), opacity, stroke, W_THIN + bonus * 0.5, opacity)
            lit = _d.polygon([b + (w * 0.2, 0, 0), b + (w, 0, 0), t + (w, 0, 0), t + (w * 0.2, 0, 0)], _d.mix(BACKGROUND, fill, 0.42), opacity, None)
            return _engine.VGroup(body, lit)
        if f["kind"] == "pipe":
            fill, stroke, opacity, bonus = self._pair(look, item, SERV, SERV)
            return _engine.VGroup(_d.polyline([view(f["x"], f["y"], f["z0"]), view(f["x"], f["y"], f["z1"])], stroke, W_EDGE + bonus, opacity))
        a, b = f["a"], f["b"]
        corners = [view(a[0], a[1], f["z0"]), view(b[0], b[1], f["z0"]), view(b[0], b[1], f["z1"]), view(a[0], a[1], f["z1"])]
        fill, stroke, opacity, bonus = self._pair(look, item, f["fill"], f["stroke"])
        if f["kind"] == "glass":
            return _engine.VGroup(_d.polygon(corners, _d.mix(BACKGROUND, GLASS, 0.18), opacity, stroke, W_THIN + bonus * 0.5, 0.8 * opacity),
                                  _d.polygon(corners, GLASS, 0.1 * opacity))
        if look.state(item) == "normal" or look.state(item) == "dim":
            fill = _d.mix(BACKGROUND, fill, shade) if look.state(item) == "normal" else fill
        group = _engine.VGroup(_d.polygon(corners, fill, opacity, stroke, W_THIN + bonus * 0.5, 0.8 * opacity))
        if f["kind"] == "wall" and f.get("extra"):
            window = f["extra"]["window"]
            op = window["data"]
            n, half = f["extra"]["n"], f["extra"]["half"]
            p0 = np.array(op["p0"]) + n * half
            p1 = np.array(op["p1"]) + n * half
            gc = [view(p0[0], p0[1], op["sill_z"]), view(p1[0], p1[1], op["sill_z"]), view(p1[0], p1[1], op["head_z"]), view(p0[0], p0[1], op["head_z"])]
            g_fill, g_stroke, g_opacity, g_bonus = self._pair(look, window, GLASS, GLASS)
            panes = max(2, round(float(np.linalg.norm(p1 - p0)) / 1.5))
            wgroup = _engine.VGroup(_d.polygon(gc, _d.mix(BACKGROUND, GLASS, 0.3 if look.state(window) != "highlight" else 0.0) if look.state(window) != "highlight" else g_fill,
                                               g_opacity if look.state(window) == "highlight" else g_opacity, g_stroke, W_EDGE + g_bonus, g_opacity))
            mullions = [(view(*(p0 + (p1 - p0) * k / panes), op["sill_z"]), view(*(p0 + (p1 - p0) * k / panes), op["head_z"])) for k in range(1, panes)]
            wgroup.add(_d.segments(mullions, g_stroke, W_THIN, g_opacity))
            f["window_group"] = wgroup
        return group
