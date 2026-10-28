"""📋 Orthographic plans of the Villa Savoye: cut plans of the ground floor, first floor and roof from one model-driven drawing."""

from __future__ import annotations

import importlib

import numpy as np

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye."
_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_s = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, GOOD, GLASS, BACKGROUND, CONCRETE, SERV = _s.INK, _s.MUTED, _s.GOOD, _s.GLASS, _s.BACKGROUND, _s.CONCRETE, _s.SERV
W_CUT, W_EDGE, W_THIN, W_HAIR = _d.W_CUT, _d.W_EDGE, _d.W_THIN, _d.W_HAIR
WALL_FILL = _d.mix(INK, BACKGROUND, 0.12)
GROUND_WALL_FILL = _d.mix(GOOD, BACKGROUND, 0.42)
BUBBLE_GAP = 0.38
BUBBLE_R = 0.13
LEVEL_CUT = {"ground": 1.0, "roof": 1.5}


def quad(a, b, half: float) -> list:
    """▭ Plan rectangle of a wall piece from ``a`` to ``b`` with ``half`` metres to each side."""
    a, b = np.asarray(a, float), np.asarray(b, float)
    d = (b - a) / max(np.linalg.norm(b - a), 1e-9)
    n = np.array([-d[1], d[0]]) * half
    return [a + n, b + n, b - n, a - n]


class Plan(_s.SheetMixin):
    """📋 Scene mixin: ``villa_plan``."""

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

    def villa_plan(self, level: str = "ground", point=(0, 0, 0), scale: float = 0.26, *, model=None, highlight=(), ghost=(), highlight_colors=None,
                   dims=(), grid: bool = True, labels: bool = True, layers=("structure", "architecture"), names=None, ramp_arrow: bool = True,
                   label_sides=("bottom", "left"), below: bool = True, north: bool = False, scalebar: bool = False, cuts=(), colors: str = "material",
                   dim: float = 0.45, halo: float = 0.2, window=None, centre=None, thumbnail: bool = False):
        """📋 Orthographic plan of one storey: poché walls and pilotis, thin glass lines, hidden dashes below the cut, grid, dimensions.

        ``level`` is ``"ground"``, ``"first"`` or ``"roof"``; ``point`` is the screen position of the box centre (or of ``centre``, model metres), ``scale`` screen units per metre.
        ``layers`` chooses ``"structure"`` (pilotis, slabs, ramp), ``"architecture"`` (walls, windows, glass, rooms) and ``"services"`` (boiler, radiator, pipes, drains).
        ``highlight`` / ``ghost`` take tags, IFC classes or kinds; ``highlight_colors`` maps a tag to its colour (default DATA); ``halo`` is the screen radius of the ring around lit pilotis.
        ``dims`` may contain ``"grid"`` and ``"overall"`` (chains on the top and right side); ``cuts`` is a list of section marks (``"salon"``, ``"ramp"``, ``"row:3"`` …).
        ``label_sides`` places grid bubbles; ``thumbnail`` drops labels, grid, dimensions and names. Returns a ``Sheet`` with ``parts[tag]``, ``at(x, y)``, ``to_screen(x, y)``."""
        g = _c.geometry(model)
        if thumbnail:
            labels = grid = north = scalebar = ramp_arrow = False
            dims, cuts, names = (), (), False
        names = bool(names)
        language = self.language()
        box = g["box"]
        cx, cy = centre if centre is not None else (box["length"] / 2, box["width"] / 2)
        view = _d.View("plan", point, scale, (cx, cy, 0.0))
        d = _s.Sheet(view, "plan")
        d.level = level
        look = _s.Look(_s.expand_names(highlight, g), _s.expand_names(ghost, g), dim, highlight_colors, colors)
        bands = _s.storey_bands(g)
        elev, top = bands[level]
        opening = g["walls"][0]["openings"][0]
        d.cut_z = {"ground": elev + LEVEL_CUT["ground"], "first": (opening["sill_z"] + opening["head_z"]) / 2, "roof": elev + LEVEL_CUT["roof"]}[level]
        catalogue = _s.items(model)
        entries: list = []
        use = set(layers)
        w = 0.7 if thumbnail else 1.0
        ctx = {"g": g, "view": view, "look": look, "level": level, "scale": scale, "w": w, "window": window, "bands": bands, "catalogue": catalogue,
               "put": lambda rank, tag, group, anchor=None, aliases=(): entries.append((rank, len(entries), tag, group, anchor, aliases)),
               "screen": lambda ring: [view(p[0], p[1], 0.0) for p in ring]}
        if use & {"architecture", "structure"}:
            self._plan_slabs(ctx)
        if "structure" in use:
            self._plan_pilotis(ctx, halo, below, thumbnail)
        if "architecture" in use:
            if level == "ground":
                self._plan_ground_walls(ctx, hidden=False)
                self._plan_curtain(ctx, hidden=False)
            elif level == "first":
                self._plan_walls(ctx)
                if below:
                    self._plan_ground_walls(ctx, hidden=True)
                    self._plan_curtain(ctx, hidden=True)
            else:
                self._plan_screens(ctx)
            self._plan_spaces(ctx)
        if use & {"architecture", "structure"}:
            self._plan_ramps(ctx, ramp_arrow)
        if "services" in use:
            self._plan_services(ctx)
        if cuts:
            self._plan_cuts(ctx, (cuts,) if isinstance(cuts, str) else cuts)
        for rank, _, tag, group, anchor, aliases in sorted(entries, key=lambda row: (row[0], row[1])):
            d.register(tag, group, anchor)
            for name in aliases:
                d.alias(name, tag)
        for name, tag in (("curtain_wall", "CW-1"), ("slab:first", "SL-1"), ("slab:roof", "SL-R")):
            if tag in d.parts:
                d.alias(name, tag)
        for ramp in g["ramp"]["ramps"]:
            members = [d.parts[f["tag"]] for f in ramp["flights"] if f["tag"] in d.parts]
            if members:
                d.parts[ramp["tag"]] = _engine.VGroup(*members)
        if grid:
            self._plan_grid(d, g, view, label_sides)
        if dims:
            self._plan_dims(d, g, view, dims, label_sides)
        if north:
            d.parts["north"] = self.north_arrow(view(0.0, box["width"], 0.0) + np.array([-0.5, 0.65, 0.0]), 0.42)
            d.add(d.parts["north"])
        if scalebar:
            metres = 5.0
            d.parts["scale_bar"] = self.scale_bar(view(box["length"], 0.0, 0.0) + np.array([-metres * scale, -0.85, 0.0]), scale, metres, steps=2)
            d.add(d.parts["scale_bar"])
        if names and "architecture" in use:
            self._plan_names(d, ctx, language)
        return d

    def _plan_slabs(self, ctx):
        """🟫 Slab regions: dashed voids above the cut, faint fill and solid voids below it; box outline; planted roof and solarium zone."""
        g, look, level, put, screen = ctx["g"], ctx["look"], ctx["level"], ctx["put"], ctx["screen"]
        box = g["box"]
        outline = [[0.0, 0.0], [box["length"], 0.0], [box["length"], box["width"]], [0.0, box["width"]]]
        for slab in g["slabs"]:
            item = _s.pseudo(slab["tag"], "IfcSlab", "slabs", slab["guid"], ("architecture", "structure"))
            is_roof = slab["kind"] == "ROOF"
            above = (level == "ground" and not is_roof) or (level == "first" and is_roof)
            under = (level == "first" and not is_roof) or (level == "roof" and is_roof)
            if not (above or under):
                continue
            fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
            rings = [screen(slab["outline"])] + [screen(v) for v in slab["voids"]]
            group = _engine.VGroup(_d.region(rings, fill, (0.06 if above else 0.1) * opacity, None, 0))
            for ring in rings[1:]:
                if above:
                    group.add(_d.polyline(ring, stroke, W_HAIR + 0.3 + bonus, 0.55 * opacity, closed=True, dash=0.06))
                else:
                    group.add(_d.polyline(ring, stroke, W_THIN + bonus, 0.8 * opacity, closed=True))
            if under and level == "roof":
                group.add(_d.polyline(rings[0], stroke, W_EDGE + bonus, 0.9 * opacity, closed=True))
            put(0.2, slab["tag"], group)
        if level == "ground":
            ctx["put"](0.1, "box", _engine.VGroup(_d.polyline(screen(outline), INK, W_HAIR + 0.3, 0.5, closed=True, dash=0.08)))
        if level == "roof":
            roof = next(s for s in g["slabs"] if s["kind"] == "ROOF")
            zone = roof.get("zones", {}).get("solarium")
            rings = [screen(roof["outline"])] + [screen(v) for v in roof["voids"]] + ([screen(zone)] if zone else [])
            put(0.3, "roof:planted", _engine.VGroup(_d.hatch(rings, GOOD, 0.11, 45.0, W_HAIR, 0.55)))
            if zone:
                put(0.3, "roof:solarium", _engine.VGroup(_d.polygon(screen(zone), CONCRETE, 0.1)))

    def _plan_pilotis(self, ctx, halo, below, thumbnail):
        """⚪ Pilotis: poché circles at the ground floor, dashed circles below the cut of the upper storeys; lit ones get a halo ring."""
        view, look, level, scale, w = ctx["view"], ctx["look"], ctx["level"], ctx["scale"], ctx["w"]
        for item in ctx["catalogue"]:
            if item["kind"] != "pilotis":
                continue
            data = item["data"]
            x, y = data["x"], data["y"]
            if not _c.inside(ctx["window"], [(x, y)], 0.5):
                continue
            centre = view(x, y, 0.0)
            radius = max(data["radius"] * scale, _s.MIN_PILOTI)
            fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
            group = _engine.VGroup()
            if level == "ground":
                group.add(_d.polygon(_s.circle(centre, radius, 20), fill, 0.9 * opacity, stroke, (W_EDGE + bonus) * w, opacity))
            elif below:
                group.add(_d.polyline(_s.circle(centre, radius, 20), stroke, (W_HAIR + 0.3 + bonus) * w, 0.7 * opacity, closed=True, dash=0.045))
            else:
                continue
            lit = look.state(item) == "highlight"
            if lit and halo > 0 and not thumbnail:
                colour = look.highlight_color(item)
                group.add(_d.polygon(_s.circle(centre, halo, 40), colour, 0.12), _d.polyline(_s.circle(centre, halo, 40), colour, W_THIN + 0.4, 1.0, closed=True))
            ctx["put"](6.5 if lit else 6.0, item["tag"], group, centre)

    def _plan_ground_walls(self, ctx, hidden: bool):
        """🟩 Set-back ground-floor walls derived from the ground spaces, one poché band per edge; dashed hairlines when hidden below a cut."""
        g, view, look, w = ctx["g"], ctx["view"], ctx["look"], ctx["w"]
        walls = _s.ground_walls(g)
        owners: dict = {}
        for a, b, tag in walls["outer"] + walls["inner"]:
            if not _c.inside(ctx["window"], [a, b], 0.5):
                continue
            if hidden:
                owners.setdefault("ground_walls", _engine.VGroup()).add(_d.polyline([view(*a, 0.0), view(*b, 0.0)], MUTED, W_HAIR + 0.3, 0.55, dash=0.06))
                continue
            item = _s.pseudo(tag, "IfcSpace", "spaces", tag)
            fill, stroke, opacity, bonus = self._pair(look, item, GROUND_WALL_FILL, INK)
            band = [view(p[0], p[1], 0.0) for p in quad(a, b, walls["thickness"] / 2)]
            owners.setdefault(tag, _engine.VGroup()).add(_d.polygon(band, fill, 0.95 * opacity, stroke, 0.7 * w, 0.8 * opacity))
        for tag, group in owners.items():
            ctx["put"](1.4 if hidden else 4.0, tag, group)

    def _plan_curtain(self, ctx, hidden: bool):
        """🪟 The curved glass wall: two thin parallel lines with mullion ticks; one dashed line when hidden below the cut."""
        g, view, look, scale, w = ctx["g"], ctx["view"], ctx["look"], ctx["scale"], ctx["w"]
        item = next((it for it in ctx["catalogue"] if it["kind"] == "curtain"), None)
        if item is None:
            return
        path = item["data"]["path"]
        fill, stroke, opacity, bonus = self._pair(look, item, GLASS, GLASS)
        group = _engine.VGroup()
        if hidden:
            group.add(_d.polyline([view(p[0], p[1], 0.0) for p in path], stroke, W_HAIR + 0.3, 0.5 * opacity, dash=0.06))
        else:
            left, right = _s.offset_path(path, max(0.07, 0.015 / scale))
            for line in (left, right):
                group.add(_d.polyline([view(p[0], p[1], 0.0) for p in line], stroke, (W_THIN + bonus) * w, opacity))
            for end in (0, -1):
                group.add(_d.polyline([view(*left[end], 0.0), view(*right[end], 0.0)], stroke, W_THIN * w, opacity))
            group.add(_d.segments([(view(*left[i], 0.0), view(*right[i], 0.0)) for i in range(0, len(path), 3)], stroke, W_HAIR, 0.8 * opacity))
        ctx["put"](1.5 if hidden else 5.0, "CW-1", group)

    def _plan_walls(self, ctx):
        """⬜ First-floor walls cut in the window zone: poché corner piers and glass double lines in the ribbon opening."""
        view, look, w = ctx["view"], ctx["look"], ctx["w"]
        for item in ctx["catalogue"]:
            if item["kind"] != "walls":
                continue
            wall = item["data"]
            a, b = np.array(wall["start"], float), np.array(wall["end"], float)
            d = (b - a) / np.linalg.norm(b - a)
            n = np.array([-d[1], d[0]])
            half = wall["thickness"] / 2
            fill, stroke, opacity, bonus = self._pair(look, item, WALL_FILL, INK)
            group = _engine.VGroup()
            cursor = a
            for opening in wall["openings"]:
                p0, p1 = np.array(opening["p0"], float), np.array(opening["p1"], float)
                if np.linalg.norm(p0 - cursor) > 1e-6:
                    group.add(self._wall_piece(view, cursor, p0, half, fill, stroke, opacity, bonus, w))
                sill = [view(q[0], q[1], 0.0) for q in quad(p0, p1, half)]
                group.add(_d.polygon(sill, fill, 0.28 * opacity, stroke, W_THIN * w, 0.85 * opacity))
                cursor = p1
            if np.linalg.norm(b - cursor) > 1e-6:
                group.add(self._wall_piece(view, cursor, b, half, fill, stroke, opacity, bonus, w))
            ctx["put"](4.0, wall["tag"], group)
            for opening in wall["openings"]:
                p0, p1 = np.array(opening["p0"], float), np.array(opening["p1"], float)
                window = next(it for it in ctx["catalogue"] if it["tag"] == opening["window_tag"])
                g_fill, g_stroke, g_opacity, g_bonus = self._pair(look, window, GLASS, GLASS)
                shape = _engine.VGroup()
                for sign in (1, -1):
                    shape.add(_d.polyline([view(*(p0 + n * half * 0.55 * sign), 0.0), view(*(p1 + n * half * 0.55 * sign), 0.0)], g_stroke, (W_THIN + g_bonus) * w, g_opacity))
                jamb = INK if look.state(window) == "normal" else g_stroke
                shape.add(_d.segments([(view(*(p0 + n * half), 0.0), view(*(p0 - n * half), 0.0)), (view(*(p1 + n * half), 0.0), view(*(p1 - n * half), 0.0))], jamb, W_THIN, g_opacity))
                ctx["put"](5.0, opening["window_tag"], shape, aliases=(opening["tag"],))

    def _wall_piece(self, view, a, b, half, fill, stroke, opacity, bonus, w):
        """⬛ One poché piece of a wall between two plan points."""
        ring = [view(p[0], p[1], 0.0) for p in quad(a, b, half)]
        return _s.poche([ring], fill, hatch=False, stroke=stroke, width=(W_CUT * 0.7 + bonus) * w, opacity=opacity)

    def _plan_screens(self, ctx):
        """🌀 Solarium screen walls cut above the roof: curved poché bands."""
        view, look, w = ctx["view"], ctx["look"], ctx["w"]
        for item in ctx["catalogue"]:
            if item["kind"] != "screens":
                continue
            data = item["data"]
            fill, stroke, opacity, bonus = self._pair(look, item, WALL_FILL, INK)
            left, right = _s.offset_path(data["path"], data["thickness"] / 2)
            ring = [view(*p, 0.0) for p in list(left) + list(right[::-1])]
            ctx["put"](4.0, item["tag"], _s.poche([ring], fill, hatch=False, stroke=stroke, width=(W_CUT * 0.7 + bonus) * w, opacity=opacity))

    def _plan_ramps(self, ctx, ramp_arrow):
        """⏫ Ramp flights of the storey: outlined strips with an arrow in the rising direction and a dot at the low end; landings as outlines."""
        g, view, look, level, w = ctx["g"], ctx["view"], ctx["look"], ctx["level"], ctx["w"]
        elev, top = ctx["bands"][level]
        for ramp in g["ramp"]["ramps"]:
            item = next((it for it in ctx["catalogue"] if it["tag"] == ramp["tag"]), None) or _s.pseudo(ramp["tag"], "IfcRamp", "ramp", ramp["guid"])
            fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
            for flight in ramp["flights"]:
                if flight["z1"] <= elev + 1e-9 or flight["z0"] >= top - 1e-9:
                    continue
                s, e = np.array(flight["start"], float), np.array(flight["end"], float)
                d = (e - s) / np.linalg.norm(e - s)
                n = np.array([-d[1], d[0]])
                half = flight["width"] / 2
                strip = [view(*p, 0.0) for p in (s + n * half, e + n * half, e - n * half, s - n * half)]
                shape = _engine.VGroup(_d.polygon(strip, fill, 0.1 * opacity, stroke, (W_THIN + bonus) * w, 0.9 * opacity))
                if ramp_arrow:
                    a, b = view(*s, 0.0), view(*e, 0.0)
                    shape.add(_d.polyline([a, b], stroke, W_THIN * w, 0.9 * opacity))
                    direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
                    side = np.array([-direction[1], direction[0], 0.0])
                    shape.add(_d.polygon([b, b - direction * 0.16 + side * 0.06, b - direction * 0.16 - side * 0.06], stroke, opacity),
                              _engine.Dot(a, radius=0.035, color=stroke).set_opacity(opacity))
                ctx["put"](3.0, flight["tag"], shape)
            landings = _engine.VGroup()
            for landing in ramp["landings"]:
                if elev - 1e-9 <= landing["z"] < top - 1e-9:
                    landings.add(_d.polyline([view(*p, 0.0) for p in landing["outline"]], stroke, W_THIN * w, 0.7 * opacity, closed=True))
            if len(landings.submobjects):
                ctx["put"](3.0, ramp["tag"] + ":landings", landings)

    def _plan_services(self, ctx):
        """🔧 Boiler, radiator, pipes, drains and downpipes of the storey in the services colour."""
        view, look, level, scale = ctx["view"], ctx["look"], ctx["level"], ctx["scale"]
        elev, top = ctx["bands"][level]
        for item in ctx["catalogue"]:
            if item["kind"] != "services":
                continue
            data = item["data"]
            zs = [p[2] for p in data["points"]]
            if max(zs) < elev - 1e-9 or min(zs) >= top - 1e-9 or not _c.inside(ctx["window"], [p[:2] for p in data["points"]], 0.5):
                continue
            fill, stroke, opacity, bonus = self._pair(look, item, SERV, SERV)
            group = _engine.VGroup()
            kind = data["kind"]
            points = np.array([p[:2] for p in data["points"]], float)
            if kind in ("boiler", "radiator"):
                size = data.get("size") or [0.6, 0.6, 1.0]
                centre = points.mean(axis=0)
                half = np.maximum(np.abs(points[-1] - points[0]) / 2, np.array(size[:2]) / 2)
                ring = [view(centre[0] + sx * half[0], centre[1] + sy * half[1], 0.0) for sx, sy in ((-1, -1), (1, -1), (1, 1), (-1, 1))]
                group.add(_d.polygon(ring, fill, 0.25 * opacity, stroke, W_EDGE + bonus, opacity))
                if kind == "boiler":
                    group.add(_d.segments([(ring[0], ring[2]), (ring[1], ring[3])], stroke, W_THIN, opacity))
            elif kind in ("drain", "downpipe"):
                radius = max(0.045, (data.get("diameter") or 0.1) / 2 * scale)
                centre = view(points[0][0], points[0][1], 0.0)
                group.add(_d.polygon(_s.circle(centre, radius, 14), fill, 0.35 * opacity, stroke, W_EDGE + bonus, opacity),
                          _d.segments([(centre + (-radius, 0, 0), centre + (radius, 0, 0)), (centre + (0, -radius, 0), centre + (0, radius, 0))], stroke, W_HAIR, opacity))
            elif np.linalg.norm(points[-1] - points[0]) < 1e-6:
                centre = view(points[0][0], points[0][1], 0.0)
                group.add(_d.polygon(_s.circle(centre, 0.04, 12), stroke, opacity, stroke, W_THIN, opacity))
            else:
                group.add(_d.polyline([view(p[0], p[1], 0.0) for p in points], stroke, W_EDGE + 0.8 + bonus, opacity))
            ctx["put"](7.0, item["tag"], group)

    def _plan_spaces(self, ctx):
        """🌿 Hatched hanging garden."""
        g, view, look, level, scale = ctx["g"], ctx["view"], ctx["look"], ctx["level"], ctx["scale"]
        for space in g["spaces"]:
            here = space["storey"] == level
            if not here and not (level == "roof" and space["tag"] == "SP-TERRACE"):
                continue
            item = _s.pseudo(space["tag"], "IfcSpace", "spaces", space["guid"])
            fill, stroke, opacity, bonus = self._pair(look, item, MUTED, MUTED)
            group = _engine.VGroup()
            if space["tag"] == "SP-TERRACE":
                ring = ctx["screen"](space["outline"])
                group.add(_d.polygon(ring, GOOD, 0.12 * opacity), _d.hatch(ring, GOOD, 0.1, 45.0, W_HAIR, 0.6 * opacity))
            if len(group.submobjects):
                ctx["put"](2.0, space["tag"], group)

    def _plan_names(self, d, ctx, language):
        """🏷️ Short room names placed on the free spot nearest to each room centroid; rooms without a free spot stay unnamed."""
        g, view, look, level = ctx["g"], ctx["view"], ctx["look"], ctx["level"]
        points = _s.cloud(d)
        for space in g["spaces"]:
            if space["storey"] != level:
                continue
            item = _s.pseudo(space["tag"], "IfcSpace", "spaces", space["guid"])
            colour = _d.mix(INK, BACKGROUND, 0.2 if look.state(item) != "dim" else look.dim)
            label = self._t(_s.short_name(space["name"], language), (0, 0, 0), _d.S_SMALL, colour)
            ring = ctx["screen"](space["outline"])
            centre = _s.centroid(space["outline"])
            spot = _s.find_spot(points, label.width, label.height, ring, view(centre[0], centre[1], 0.0))
            if spot is None:
                continue
            label.move_to(np.array([spot[0], spot[1], 0.0]))
            points = np.vstack([points, _s.cloud(label)])
            d.register(space["tag"], _engine.VGroup(label))

    def _plan_grid(self, d, g, view, sides):
        """🔢 Dash-dot axes with bubbles 1…5 and A…E on the chosen sides."""
        gx, gy = g["grid"]["x"], g["grid"]["y"]
        lx, ly = g["box"]["length"], g["box"]["width"]
        bottom, top, left, right = ("bottom" in sides, "top" in sides, "left" in sides, "right" in sides)
        origin, far = view(0.0, 0.0, 0.0), view(lx, ly, 0.0)
        reach = BUBBLE_GAP - BUBBLE_R
        y0, y1 = origin[1] - (reach if bottom else 0.2), far[1] + (reach if top else 0.2)
        x0, x1 = origin[0] - (reach if left else 0.2), far[0] + (reach if right else 0.2)
        lines, marks = _engine.VGroup(), _engine.VGroup()
        for x, label in zip(gx, g["grid"]["labels_x"]):
            sx = view(x, 0.0, 0.0)[0]
            axis = _d.segments(_s.dash_dot((sx, y0, 0), (sx, y1, 0)), MUTED, W_HAIR, 0.55)
            d.parts["axis_x:" + label] = _engine.VGroup(axis)
            lines.add(axis)
            for flag, y in ((bottom, origin[1] - BUBBLE_GAP), (top, far[1] + BUBBLE_GAP)):
                if flag:
                    bub = _s.bubble(self, label, np.array([sx, y, 0.0]), radius=BUBBLE_R)
                    d.parts.setdefault("label_x:" + label, _engine.VGroup()).add(bub)
                    marks.add(bub)
        for y, label in zip(gy, g["grid"]["labels_y"]):
            sy = view(0.0, y, 0.0)[1]
            axis = _d.segments(_s.dash_dot((x0, sy, 0), (x1, sy, 0)), MUTED, W_HAIR, 0.55)
            d.parts["axis_y:" + label] = _engine.VGroup(axis)
            lines.add(axis)
            for flag, x in ((left, origin[0] - BUBBLE_GAP), (right, far[0] + BUBBLE_GAP)):
                if flag:
                    bub = _s.bubble(self, label, np.array([x, sy, 0.0]), radius=BUBBLE_R)
                    d.parts.setdefault("label_y:" + label, _engine.VGroup()).add(bub)
                    marks.add(bub)
        d.add(lines)
        d.add_to_back(lines)
        d.add(marks)
        d.parts["grid"] = _engine.VGroup(lines, marks)

    def _plan_dims(self, d, g, view, dims, sides):
        """↔️ Grid chain and overall dimensions above and to the right of the plan."""
        lx, ly = g["box"]["length"], g["box"]["width"]
        pad = 0.5 + (0.4 if "top" in sides else 0.0)
        padr = 0.5 + (0.4 if "right" in sides else 0.0)
        group = _engine.VGroup()
        if "grid" in dims:
            xs = sorted({0.0, lx, *g["grid"]["x"]})
            ys = sorted({0.0, ly, *g["grid"]["y"]})
            for a, b in zip(xs[:-1], xs[1:]):
                group.add(self._dim(view(a, ly, 0.0), view(b, ly, 0.0), self.number(b - a, 2), offset=pad, color=MUTED))
            for a, b in zip(ys[:-1], ys[1:]):
                group.add(self._dim(view(lx, a, 0.0), view(lx, b, 0.0), self.number(b - a, 2), offset=-padr, color=MUTED))
        if "overall" in dims:
            step = 0.5 if "grid" in dims else 0.0
            group.add(self._dim(view(0.0, ly, 0.0), view(lx, ly, 0.0), self.quantity(lx, "m", 2), offset=pad + step, color=INK))
            group.add(self._dim(view(lx, 0.0, 0.0), view(lx, ly, 0.0), self.quantity(ly, "m", 2), offset=-(padr + step), color=INK))
        d.parts["dims"] = group
        d.add(group)

    def _plan_cuts(self, ctx, cuts):
        """✂️ Section marks: dash-dot line through the plan with bold ends, view arrows and a letter."""
        g, view, put = ctx["g"], ctx["view"], ctx["put"]
        for index, spec in enumerate(cuts):
            plane = _s.parse_cut(spec, g)
            lx, ly = g["box"]["length"], g["box"]["width"]
            if plane.axis == "x":
                a, b, direction = view(plane.value, 0.0, 0.0), view(plane.value, ly, 0.0), np.array([-1.0, 0.0, 0.0])
            else:
                a, b, direction = view(0.0, plane.value, 0.0), view(lx, plane.value, 0.0), np.array([0.0, 1.0, 0.0])
            along = (b - a) / np.linalg.norm(b - a)
            side = np.array([-direction[1], direction[0], 0.0])
            group = _engine.VGroup(_d.segments(_s.dash_dot(a - along * 0.2, b + along * 0.2), INK, W_THIN, 0.8),
                                   _d.segments([(a - along * 0.62, a - along * 0.98), (b + along * 0.62, b + along * 0.98)], INK, W_CUT))
            for end, out in ((a - along * 0.98, -along), (b + along * 0.98, along)):
                tip = end + direction * 0.28
                group.add(_d.segments([(end, tip)], INK, W_EDGE),
                          _d.polygon([tip + direction * 0.1, tip - direction * 0.03 + side * 0.06, tip - direction * 0.03 - side * 0.06], INK),
                          self._t("ABCDEFG"[index], end + out * 0.17 + direction * 0.05, _d.S_SMALL, INK))
            put(8.0, "cut:" + (spec if isinstance(spec, str) else str(spec)), group)
