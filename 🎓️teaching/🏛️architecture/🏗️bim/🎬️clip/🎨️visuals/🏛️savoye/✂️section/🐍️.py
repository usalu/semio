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
_plan = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")

INK, MUTED, DATA, GOOD, WARN, GLASS, BACKGROUND, CONCRETE, SERV = (_s.INK, _s.MUTED, _s.DATA, _s.GOOD, _s.WARN, _s.GLASS, _s.BACKGROUND, _s.CONCRETE, _s.SERV)
STRUCT = _d.STRUCT
P = _d.P
W_CUT, W_EDGE, W_THIN, W_HAIR = _d.W_CUT, _d.W_EDGE, _d.W_THIN, _d.W_HAIR
WALL_FILL = _d.mix(INK, BACKGROUND, 0.12)
GREEN_FILL = _d.mix(_d.GOOD, BACKGROUND, 0.42)
BEYOND = _s.BEYOND_FILL
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


def part_projection(part, plane, state):
    """✂️ Preserve true loft cuts and silhouettes while retaining concave prism intervals."""
    if state == "cut":
        return _c._model.part_sections(part,plane.value,plane.index)
    ring = _c._model.part_elevation(part,plane.index)
    return [ring] if len(ring) >= 3 else []


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
        values = sorted(g["storeys"].items(),key=lambda item:item[1])+[("top",_plan.model_vertical_bounds(g)[1])]
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
        z_low,z_high = _plan.model_vertical_bounds(g)
        hc, zc = centre if centre is not None else (hmax/2,(z_low+z_high)/2)
        view = _d.View("section", point, scale, (0.0, hc, zc) if plane.axis == "x" else (hc, 0.0, zc), axis="y" if plane.axis == "x" else "x", cut=plane.value)
        d = _s.Sheet(view, "section")
        d.z_bounds = (z_low,z_high)
        d.plane = plane.spec
        base = np.asarray(P(point), float)

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
        if use & {"architecture","structure"}:
            self._sec_solids(ctx)
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
        marks = sorted(set(g["storeys"].values())|set(_plan.model_vertical_bounds(g)))
        if "levels" in dims:
            for a, b in zip(marks[:-1], marks[1:]):
                group.add(self._dim((x, to(0, a)[1], 0), (x, to(0, b)[1], 0), self.number(b - a, 2), offset=0.0, color=MUTED))
        if "overall" in dims:
            group.add(self._dim((x, to(0, marks[0])[1], 0), (x, to(0, marks[-1])[1], 0), self.quantity(marks[-1]-marks[0], "m", 2), offset=0.7 if "levels" in dims else 0.0, color=INK))
        d.parts["dims"] = group
        d.add(group)

    def _sec_names(self, d, ctx, language):
        """🏷️ Room names inside the cut rooms where free space exists."""
        g, plane, to, look = ctx["g"], ctx["plane"], ctx["to"], ctx["look"]
        points = _s.cloud(d)
        bands = _s.storey_bands(g)
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
        """🟫 Physical regional cuts and per-region roof build-up elevations."""
        g,plane,to,look,put = ctx["g"],ctx["plane"],ctx["to"],ctx["look"],ctx["put"]
        for slab in g["slabs"]:
            item = _s.pseudo(slab["tag"],"IfcSlab","slabs",slab["guid"],("architecture","structure"))
            fill,stroke,opacity,bonus = self._pair(look,item,CONCRETE,INK)
            cells = _plan.rect_union_cells(_plan.slab_cut(slab,plane.index,plane.value))
            group = _engine.VGroup()
            for h0,h1,z0,z1 in cells:
                ring = [to(h,z) for h,z in rect(h0,h1,z0,z1)]
                group.add(_d.polygon(ring,fill,opacity))
                if (h1-h0)*ctx["scale"] > 0.1 and (z1-z0)*ctx["scale"] > 0.04:
                    group.add(_d.hatch(ring,stroke,0.05,45.0,W_HAIR,0.45*opacity))
            if cells:
                group.add(_d.segments([(to(*a),to(*b)) for a,b in _plan.cell_edges(cells)],stroke,W_CUT+bonus,opacity))
            put(2.0,0.0,slab["tag"],group)
            if not ctx["buildup"]:
                continue
            finishes = {}
            for part in slab.get("finish_parts",[]):
                finishes.setdefault(part["layer"],[]).append(part)
            for layer,parts in finishes.items():
                colour = _c.surface_color({"kind":layer})
                cells = _plan.rect_union_cells(_plan.slab_cut({"parts":parts},plane.index,plane.value))
                shapes = _engine.VGroup(*[_d.polygon([to(h,z) for h,z in rect(a,b,c,d)],colour,opacity) for a,b,c,d in cells])
                if cells:
                    shapes.add(_d.segments([(to(*a),to(*b)) for a,b in _plan.cell_edges(cells)],stroke,W_THIN,opacity))
                    put(2.1,0.0,slab["tag"]+":"+layer,shapes)

    def _beyond(self, ctx, item, ring_hz, tag, depth, fill=None, stroke=None, opacity_fill=1.0):
        """🫥 A surface beyond the cut: dark fill with a thin outline."""
        look, to = ctx["look"], ctx["to"]
        f, s, opacity, bonus = self._pair(look, item, fill or BEYOND, stroke or INK)
        group = _engine.VGroup(self._shape([to(h, z) for h, z in ring_hz], f, opacity_fill, s, W_THIN + bonus * 0.5, 0.7 * opacity))
        ctx["put"](1.0, depth, tag, group)
        return group

    def _sec_pilotis(self, ctx):
        """🏛️ Exact polygon-profile intersections and projections for every column part."""
        plane,to,look,put = ctx["plane"],ctx["to"],ctx["look"],ctx["put"]
        for item in ctx["catalogue"]:
            if item["kind"] != "pilotis":
                continue
            fill,stroke,opacity,bonus = self._pair(look,item,CONCRETE,INK)
            for part in item["data"]["parts"]:
                ring = _c._model.part_footprint(part)
                if not _c.inside(ctx["window"],ring,0.5):
                    continue
                state = plane.classify(*plane.extent(ring))
                if state == "removed":
                    continue
                for polygon in part_projection(part,plane,state):
                    points = [to(h,z) for h,z in polygon]
                    group = self._cut_poche(points,fill,stroke,opacity,bonus) if state == "cut" else _engine.VGroup(self._shape(points,BEYOND,1.0,stroke,W_THIN+bonus,0.7*opacity))
                    put(2.0 if state == "cut" else 1.0,plane.extent(ring)[1],item["tag"],group)

    def _sec_ramps(self, ctx):
        """⏫ Cut actual source-owned inclined slabs and landings through shared geometry."""
        g,plane,to,look,put = ctx["g"],ctx["plane"],ctx["to"],ctx["look"],ctx["put"]
        for ramp in g["ramp"]["ramps"]:
            for part in ramp["parts"]+ramp["child_parts"]:
                tag = part["owner"]
                cls = "IfcRampFlight" if part["role"]=="flight" else "IfcRamp"
                item = next((it for it in ctx["catalogue"] if it["tag"]==tag),None) or _s.pseudo(tag,cls,"ramp",part["owner_guid"])
                ring = _c._model.part_footprint(part)
                state = plane.classify(*plane.extent(ring))
                if state=="removed":
                    continue
                fill,stroke,opacity,bonus = self._pair(look,item,CONCRETE,INK)
                for polygon in part_projection(part,plane,state):
                    points = [to(h,z) for h,z in polygon]
                    group = self._cut_poche(points,fill,stroke,opacity,bonus) if state=="cut" else _engine.VGroup(self._shape(points,BEYOND,1.0,stroke,W_THIN,0.7*opacity))
                    put(2.0 if state=="cut" else 1.0,plane.extent(ring)[1],tag,group)

    def _wall_ring(self, wall):
        """▭ Plan rectangle of a wall (its path is the axis)."""
        a, b = np.array(wall["start"], float), np.array(wall["end"], float)
        d = (b - a) / np.linalg.norm(b - a)
        n = np.array([-d[1], d[0]]) * wall["thickness"] / 2
        return [a + n, b + n, b - n, a - n], a, d

    def _sec_walls(self, ctx):
        """⬜ Cut and projected wall cells with every opening subtracted from the wall body."""
        plane,to,look,put = ctx["plane"],ctx["to"],ctx["look"],ctx["put"]
        for wall in ctx["g"]["walls"]:
            item = next((it for it in ctx["catalogue"] if it["tag"] == wall["tag"]),None) or _s.pseudo(wall["tag"],"IfcWall","walls",wall["guid"])
            origin = np.array(wall["start"],float)
            direction = (np.array(wall["end"],float)-origin)/wall["length"]
            fill,stroke,opacity,bonus = self._pair(look,item,GREEN_FILL if wall["storey"] == "ground" else WALL_FILL,INK)
            cut_group,back = _engine.VGroup(),_engine.VGroup()
            cut_cells,back_cells = [],[]
            depth = 0.0
            for part in wall["parts"]:
                z0,z1 = part["z0"],part["z1"]
                ring = part["outline"]
                kind = plane.classify(*plane.extent(ring))
                if kind == "removed":
                    continue
                depth = plane.extent(ring)[1]
                spans = plane.intervals([ring]) if kind == "cut" else [plane.hrange(ring)]
                for h0,h1 in spans:
                    if h1-h0 <= EPS:
                        continue
                    points = [to(h,z) for h,z in rect(h0,h1,z0,z1)]
                    (cut_cells if kind == "cut" else back_cells).append((h0,h1,z0,z1))
                    (cut_group if kind == "cut" else back).add(_d.polygon(points,fill if kind == "cut" else BEYOND,opacity if kind == "cut" else 0.7*opacity))
            for group,cells,width in ((cut_group,cut_cells,W_CUT+bonus),(back,back_cells,W_THIN+bonus*0.5)):
                if cells:
                    group.add(_d.segments([(to(*p),to(*q)) for p,q in _plan.cell_edges(cells)],stroke,width,opacity))
            if len(back.submobjects):
                put(1.0,depth,wall["tag"],back)
            if len(cut_group.submobjects):
                put(2.0,0.0,wall["tag"],cut_group)
            for op in wall["openings"]:
                ring = segment_quad(op["p0"],op["p1"],wall["thickness"]/2)
                kind = plane.classify(*plane.extent(ring))
                if kind == "removed":
                    continue
                tag = op.get("fill_tag") or op.get("window_tag") or op["tag"]
                cls = op.get("fill_class")
                item = next((it for it in ctx["catalogue"] if it["tag"] == tag),None) or _s.pseudo(tag,cls or "IfcOpeningElement","openings",op["guid"])
                fill,stroke,opacity,bonus = self._pair(look,item,GLASS if cls != "IfcDoor" else CONCRETE,GLASS if cls != "IfcDoor" else INK)
                group = _engine.VGroup()
                if not cls and not op.get("window_tag"):
                    spans = plane.intervals([ring]) if kind == "cut" else [plane.hrange(ring)]
                    for h0,h1 in spans:
                        group.add(_d.segments([(to(h0,op["head_z"]),to(h1,op["head_z"])),(to(h0,op["sill_z"]),to(h1,op["sill_z"]))],INK,W_HAIR,opacity))
                filling = next((f for f in ctx["g"]["fillings"] if f["tag"]==tag),None)
                if filling:
                    for part in filling["parts"]:
                        classification = plane.classify(*plane.extent(part["outline"]))
                        if classification == "removed":
                            continue
                        spans = plane.intervals([part["outline"]]) if classification == "cut" else [plane.hrange(part["outline"])]
                        for h0,h1 in spans:
                            if h1-h0 > EPS:
                                group.add(self._shape([to(h,z) for h,z in rect(h0,h1,part["z0"],part["z1"])],GLASS if part["role"]=="glass" else INK,0.22 if part["role"]=="glass" else opacity,stroke,W_THIN+bonus,opacity))
                put(2.2 if kind == "cut" else 1.0,plane.extent(ring)[1]+0.001,tag,group,aliases=(op["tag"],) if tag != op["tag"] else ())

    def _sec_solids(self, ctx):
        """🪜 Exact section intersections and beyond projections of authored part extrusions."""
        plane,to,look,put = ctx["plane"],ctx["to"],ctx["look"],ctx["put"]
        for kind in ("stairs","details","furnishings","railings","foundations","beams"):
            for data in ctx["g"].get(kind,[]):
                if not set(data["domains"]).intersection(ctx["use"]):
                    continue
                item = next((it for it in ctx["catalogue"] if it["tag"] == data["tag"]),None) or _s.pseudo(data["tag"],data["class"],kind,data["guid"])
                groups = {}
                for part in data["parts"]:
                    fill,stroke,opacity,bonus = self._pair(look,item,_c.surface_color(data,part),INK)
                    ring = _c._model.part_footprint(part)
                    state = plane.classify(*plane.extent(ring))
                    if state == "removed":
                        continue
                    depth = plane.extent(ring)[1]
                    for polygon in part_projection(part,plane,state):
                        points = [to(h,z) for h,z in polygon]
                        shape = self._cut_poche(points,fill,stroke,opacity,bonus,hatch=False) if state == "cut" else self._shape(points,fill,0.3,stroke,W_THIN,0.65*opacity)
                        groups.setdefault((state,depth),_engine.VGroup()).add(shape)
                for (state,depth),group in groups.items():
                    put(2.1 if state == "cut" else 1.0,depth,data["tag"],group)

    def _cross_point(self, a, d, plane):
        """✖️ Point where a wall axis meets the cutting plane."""
        if abs(d[plane.index]) < 1e-9:
            return a
        t = (plane.value - a[plane.index]) / d[plane.index]
        return a + d * t

    def _sec_enclosure(self, ctx):
        """🪟 Section the actual curtain and its native hosted filling material without refilling portals."""
        plane,to,look,put=ctx["plane"],ctx["to"],ctx["look"],ctx["put"]
        items=[it for it in ctx["catalogue"] if it["kind"]=="curtain" or it["kind"] in ("doors","windows") and "parts" in it["data"]]
        for item in items:
            fill,stroke,opacity,bonus=self._pair(look,item,GLASS,GLASS)
            group=_engine.VGroup()
            for part in item["data"]["parts"]:
                ring=part["outline"]
                kind=plane.classify(*plane.extent(ring))
                if kind=="removed":
                    continue
                spans=plane.intervals([ring]) if kind=="cut" else [plane.hrange(ring)]
                for h0,h1 in spans:
                    if h1-h0>EPS:
                        glass=part["role"]=="glass"
                        group.add(_d.polygon([to(h,z) for h,z in rect(h0,h1,part["z0"],part["z1"])],GLASS if glass else INK,.15*opacity if glass else opacity,stroke,W_THIN+bonus,opacity))
            opening=item["data"].get("opening")
            put(2.1,0.,item["tag"],group,aliases=(opening,) if opening else ())

    def _sec_screens(self, ctx):
        """🌀 Solarium screens cut as hatched strips or beyond as the curved wall face."""
        g, plane, to, look, put = ctx["g"], ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        for item in ctx["catalogue"]:
            if item["kind"] != "screens":
                continue
            data = item["data"]
            fill, stroke, opacity, bonus = self._pair(look, item, WALL_FILL, INK)
            for part in data["parts"]:
                ring = _c._model.part_footprint(part)
                state = plane.classify(*plane.extent(ring))
                if state == "removed":
                    continue
                for polygon in part_projection(part,plane,state):
                    points = [to(h,z) for h,z in polygon]
                    group = self._cut_poche(points,fill,stroke,opacity,bonus) if state == "cut" else _engine.VGroup(self._shape(points,BEYOND,1.,stroke,W_HAIR+.2,.7*opacity))
                    put(2. if state == "cut" else 1.,plane.extent(ring)[1],item["tag"],group)

    def _sec_services(self, ctx):
        """🔧 Boiler, riser, pipes and radiator as thin prisms in the services colour."""
        plane, to, look, put = ctx["plane"], ctx["to"], ctx["look"], ctx["put"]
        for item in ctx["catalogue"]:
            if item["kind"] != "services":
                continue
            data = item["data"]
            fill, stroke, opacity, bonus = self._pair(look, item, SERV, SERV)
            cut_group, back = _engine.VGroup(), _engine.VGroup()
            for part in data["parts"]:
                ring = _c._model.part_footprint(part)
                z0,z1 = part["z0"],part["z1"]
                kind = plane.classify(*plane.extent(ring))
                if kind == "removed":
                    continue
                polygons = part_projection(part,plane,kind)
                for polygon in polygons:
                    shape = self._shape([to(h,z) for h,z in polygon],fill,0.35,stroke,W_EDGE+bonus,opacity)
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
        z_low,z_high = _plan.model_vertical_bounds(g)
        hc, zc = centre if centre is not None else (horizontal/2,(z_low+z_high)/2)
        cx, cy = (hc, box["width"] / 2) if face in ("south", "north") else (box["length"] / 2, hc)
        view = _d.View("elevation", point, scale, (cx, cy, zc), face=face)
        d = _s.Sheet(view, "elevation")
        d.z_bounds = (z_low,z_high)
        d.face = face
        look = _s.Look(_s.expand_names(highlight, g), _s.expand_names(ghost, g), dim, highlight_colors, colors)
        catalogue = _s.items(model)
        use = set(layers)
        faces: list = []
        base = np.asarray(P(point), float)

        def to(h, z):
            return np.array([base[0] + scale * ((h - hc) * (1 if face in ("south", "east") else -1)), base[1] + scale * (z - zc), 0.0])

        def add_face(tag, item, a, b, z0, z1, normal, fill, stroke, kind="wall", extra=None):
            facing = view.facing((normal[0], normal[1], 0.0))
            if facing <= 1e-3:
                return
            mid = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
            faces.append({"tag": tag, "item": item, "a": a, "b": b, "z0": z0, "z1": z1, "facing": facing, "fill": fill, "stroke": stroke, "kind": kind,
                          "depth": view.depth(mid[0], mid[1], (z0 + z1) / 2), "extra": extra})

        def add_loft(tag,item,part,fill,kind):
            for vertices,normal in _c.part_faces(part):
                facing = view.facing(normal)
                if facing <= 1e-3:
                    continue
                centre = np.mean(vertices,axis=0)
                faces.append({"tag":tag,"item":item,"vertices":vertices,"facing":facing,"fill":fill,"stroke":INK,"kind":kind,"depth":view.depth(*centre)})

        centre_xy = np.array([box["length"] / 2, box["width"] / 2])
        if "architecture" in use:
            curtains=[it for it in catalogue if it["kind"]=="curtain" or it["kind"] in ("doors","windows") and "parts" in it["data"]]
            for curtain in curtains:
                inside = np.mean([np.array(s["outline"]).mean(axis=0) for s in g["spaces"] if s["tag"] == "SP-HALL"], axis=0) if any(s["tag"] == "SP-HALL" for s in g["spaces"]) else centre_xy
                for part in curtain["data"]["parts"]:
                    ring = part["outline"]
                    for a,b in zip(ring,ring[1:]+ring[:1]):
                        dv = np.asarray(b)-np.asarray(a)
                        n = np.array([dv[1],-dv[0]])/max(np.linalg.norm(dv),EPS)
                        glass = part["role"]=="glass"
                        add_face(curtain["tag"],curtain,a,b,part["z0"],part["z1"],n,GLASS if glass else INK,GLASS if glass else INK,"glass" if glass else "solid",extra={"opening":curtain["data"]["opening"]} if "opening" in curtain["data"] else None)

            for item in catalogue:
                if item["kind"] == "walls":
                    wall = item["data"]
                    origin = np.array(wall["start"],float)
                    dv = (np.array(wall["end"],float)-origin)/wall["length"]
                    n = np.array([-dv[1],dv[0]])
                    if view.facing((*n,0.0)) < 0:
                        n = -n
                    half = wall["thickness"]/2
                    for part in wall["parts"]:
                        ring = part["outline"]
                        area = sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
                        for a,b in zip(ring,ring[1:]+ring[:1]):
                            delta = np.asarray(b)-np.asarray(a)
                            normal = np.array([delta[1],-delta[0]])/max(np.linalg.norm(delta),EPS)*(1 if area>=0 else -1)
                            add_face(wall["tag"],item,a,b,part["z0"],part["z1"],normal,GREEN_FILL if wall["storey"]=="ground" else WALL_FILL,INK,"solid")
                    for op in wall["openings"]:
                        tag = op.get("fill_tag") or op.get("window_tag") or op["tag"]
                        cls = op.get("fill_class")
                        opening_item = next((it for it in catalogue if it["tag"] == tag),None) or _s.pseudo(tag,cls or "IfcOpeningElement","openings",op["guid"])
                        if not cls and not op.get("window_tag"):
                            add_face(tag,opening_item,np.array(op["p0"])+n*half,np.array(op["p1"])+n*half,op["sill_z"],op["head_z"],n,BACKGROUND,INK,"opening",{"opening":op["tag"]})
                        filling = next((f for f in g["fillings"] if f["tag"]==tag),None)
                        if filling:
                            for part in filling["parts"]:
                                ring = part["outline"]
                                for a,b in zip(ring,ring[1:]+ring[:1]):
                                    delta = np.asarray(b)-np.asarray(a)
                                    normal = np.array([delta[1],-delta[0]])/max(np.linalg.norm(delta),EPS)
                                    glass = part["role"]=="glass"
                                    add_face(tag,opening_item,a,b,part["z0"],part["z1"],normal,GLASS if glass else INK,GLASS if glass else INK,"glass" if glass else "solid",{"opening":op["tag"]})
                elif item["kind"] == "screens":
                    data = item["data"]
                    for part in data["parts"]:
                        add_loft(item["tag"],item,part,WALL_FILL,"screen")

        if use & {"structure", "architecture"}:
            for slab in g["slabs"]:
                item = _s.pseudo(slab["tag"], "IfcSlab", "slabs", slab["guid"], ("architecture", "structure"))
                occupied = slab["parts"]+slab.get("finish_parts",[])
                for a,b,z0,z1,n in _plan.slab_sides(slab,occupied):
                    add_face(slab["tag"],item,a,b,z0,z1,n,CONCRETE,INK,"slab")
                finishes = {}
                for part in slab.get("finish_parts",[]):
                    finishes.setdefault(part["layer"],[]).append(part)
                for layer,parts in finishes.items():
                    for a,b,z0,z1,n in _plan.slab_sides({"parts":parts},occupied):
                        add_face(slab["tag"]+":"+layer,item,a,b,z0,z1,n,_c.surface_color({"kind":layer}),INK,"solid")
        if use & {"architecture","structure"}:
            for kind in ("stairs","details","furnishings","railings"):
                for data in g.get(kind,[]):
                    if not set(data["domains"]).intersection(use):
                        continue
                    item = next((it for it in catalogue if it["tag"] == data["tag"]),None) or _s.pseudo(data["tag"],data["class"],kind,data["guid"])
                    for part in data["parts"]:
                        if any(key in part for key in ("top_outline","section","mesh")):
                            add_loft(data["tag"],item,part,_c.surface_color(data,part),"glass" if _c.surface_role(data,part) == "glass" else "solid")
                            continue
                        ring = part["outline"]
                        area = sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
                        for a,b in zip(ring,ring[1:]+ring[:1]):
                            delta = np.array(b)-np.array(a)
                            if np.linalg.norm(delta) <= EPS:
                                continue
                            n = np.array([delta[1],-delta[0]])/np.linalg.norm(delta)*(1 if area >= 0 else -1)
                            add_face(data["tag"],item,a,b,part["z0"],part["z1"],n,_c.surface_color(data,part),INK,"glass" if _c.surface_role(data,part) == "glass" else "solid")
        if "structure" in use:
            for item in catalogue:
                if item["kind"] not in ("pilotis","foundations","beams"):
                    continue
                data = item["data"]
                for part in data["parts"]:
                    if "top_outline" in part:
                        if _c.inside(window,_c._model.part_footprint(part),0.5):
                            add_loft(data["tag"],item,part,CONCRETE,"solid")
                        continue
                    ring = part["outline"]
                    if not _c.inside(window,ring,0.5):
                        continue
                    area = sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
                    for a,b in zip(ring,ring[1:]+ring[:1]):
                        delta = np.array(b)-np.array(a)
                        if np.linalg.norm(delta) <= EPS:
                            continue
                        n = np.array([delta[1],-delta[0]])/np.linalg.norm(delta)*(1 if area >= 0 else -1)
                        add_face(data["tag"],item,a,b,part["z0"],part["z1"],n,CONCRETE,INK,"solid")
        if "services" in use:
            for item in catalogue:
                if item["kind"] == "services":
                    for part in item["data"]["parts"]:
                        add_loft(item["tag"],item,part,SERV,"solid")
        if ground:
            strip = _s.ground_strip(self, to, -1.5, horizontal + 1.5, 0.5)
            d.parts["ground"] = strip
            d.add(strip)
        members: dict = {}
        for f in sorted(faces, key=lambda row: row["depth"]):
            group = self._elev_face(f, to, view, look, scale)
            d.add(group)
            members.setdefault(f["tag"], []).append(group)
            if f.get("extra",{}):
                opening_tag = f["extra"].get("opening")
                if opening_tag:
                    members.setdefault(opening_tag,[]).append(group)
        columns = {item["tag"]:item["data"] for item in catalogue if item["kind"] == "pilotis"}
        for tag, groups in members.items():
            d.parts[tag] = _engine.VGroup(*groups)
            if tag in columns:
                column = columns[tag]
                d.anchors[tag] = view(column["x"],column["y"],(column["base"]+column["top"])/2)
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
        """🧱 Paint a visible surface with true wall voids and addressable fillings."""
        item = f["item"]
        shade = 0.45 + 0.55 * f["facing"]
        if f["kind"] == "pipe":
            fill, stroke, opacity, bonus = self._pair(look, item, SERV, SERV)
            return _engine.VGroup(_d.polyline([view(f["x"], f["y"], f["z0"]), view(f["x"], f["y"], f["z1"])], stroke, W_EDGE + bonus, opacity))
        if "vertices" in f:
            corners = [view(*p) for p in f["vertices"]]
        else:
            a,b = f["a"],f["b"]
            corners = [view(a[0],a[1],f["z0"]),view(b[0],b[1],f["z0"]),view(b[0],b[1],f["z1"]),view(a[0],a[1],f["z1"])]
        fill, stroke, opacity, bonus = self._pair(look, item, f["fill"], f["stroke"])
        if f["kind"] == "opening":
            return _engine.VGroup(_d.polyline(corners,stroke,W_HAIR+bonus,0.7*opacity,closed=True))
        if f["kind"] == "glass":
            return _engine.VGroup(_d.polygon(corners, _d.mix(BACKGROUND, GLASS, 0.18), opacity, stroke, W_THIN + bonus * 0.5, 0.8 * opacity),
                                  _d.polygon(corners, GLASS, 0.1 * opacity))
        if look.state(item) == "normal" or look.state(item) == "dim":
            fill = _d.mix(BACKGROUND, fill, shade) if look.state(item) == "normal" else fill
        if f["kind"] == "wall" and f.get("extra",{}).get("wall"):
            extra = f["extra"]
            cells = _plan.wall_cells(extra["wall"])
            origin,direction,offset = extra["origin"],extra["direction"],extra["offset"]
            def project(s,z):
                """📽️ Project a longitudinal wall station and height."""
                p = origin+direction*s+offset
                return view(p[0],p[1],z)
            group = _engine.VGroup(*[_d.polygon([project(s,z) for s,z in rect(a,b,c,d)],fill,opacity) for a,b,c,d in cells])
            group.add(_d.segments([(project(*p),project(*q)) for p,q in _plan.cell_edges(cells)],stroke,W_THIN+bonus*0.5,0.8*opacity))
            return group
        group = _engine.VGroup(_d.polygon(corners, fill, opacity, stroke, W_THIN + bonus * 0.5, 0.8 * opacity))
        return group
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            