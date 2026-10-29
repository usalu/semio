"""📋 Orthographic plans of every authored Villa Savoye storey."""

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


def quad(a, b, half: float) -> list:
    """▭ Plan rectangle of a wall piece from ``a`` to ``b`` with ``half`` metres to each side."""
    a, b = np.asarray(a, float), np.asarray(b, float)
    d = (b - a) / max(np.linalg.norm(b - a), 1e-9)
    n = np.array([-d[1], d[0]]) * half
    return [a + n, b + n, b - n, a - n]


def wall_cells(wall):
    """🧩 Disjoint longitudinal-height cells after subtracting every authored opening."""
    length, base, top = wall["length"], wall["base"], wall["top"]
    openings = wall["openings"]
    xs = sorted({0.0, length, *(max(0.0, min(length, op[key])) for op in openings for key in ("start", "end"))})
    zs = sorted({base, top, *(max(base, min(top, op[key])) for op in openings for key in ("sill_z", "head_z"))})
    return [(a, b, c, d) for a, b in zip(xs, xs[1:]) for c, d in zip(zs, zs[1:])
            if b > a and d > c and not any(op["start"] < (a+b)/2 < op["end"] and op["sill_z"] < (c+d)/2 < op["head_z"] for op in openings)]


def cell_edges(cells):
    """🧵 Boundary segments of disjoint rectangles, removing every shared cell edge."""
    edges = {}
    for a,b,c,d in cells:
        ring = [(a,c),(b,c),(b,d),(a,d)]
        for p,q in zip(ring,ring[1:]+ring[:1]):
            key = tuple(sorted((tuple(round(x,9) for x in p),tuple(round(x,9) for x in q))))
            if key in edges:
                del edges[key]
            else:
                edges[key] = (p,q)
    return list(edges.values())


def profile_cut(outline, axis, value):
    """✂️ Exact polygon scanline intervals, independent of a profile's descriptive kind."""
    hits = []
    for a,b in zip(outline,outline[1:]+outline[:1]):
        if (a[axis] <= value < b[axis]) or (b[axis] <= value < a[axis]):
            t = (value-a[axis])/(b[axis]-a[axis])
            hits.append(a[1-axis]+t*(b[1-axis]-a[1-axis]))
    hits.sort()
    return [(a,b) for a,b in zip(hits[::2],hits[1::2]) if b-a > 1e-9]


def model_vertical_bounds(g):
    """🏢 Physical drawing range including cellar, foundations and all authored solid parts."""
    levels = list(g["storeys"].values())+[g["box"]["height"]]
    for slab in g["slabs"]:
        for part in slab["parts"]+slab.get("finish_parts",[]):
            levels.extend((part["z0"],part["z1"]))
    for kind in ("pilotis","stairs","details","furnishings","railings","foundations","beams","services"):
        for item in g.get(kind,[]):
            for part in item["parts"]:
                levels.extend((part["z0"],part["z1"]))
    return min(levels),max(levels)


def plan_cut_z(g, level):
    """📏 Consistent model-relative plan cut for every authored storey."""
    return g["storeys"][level]+1.2


def adjacent_slabs(g, cut):
    """🟫 Adjacent slab assemblies selected by actual regional bounds."""
    bounds = [(slab,min(p["z0"] for p in slab["parts"]),max(p["z1"] for p in slab["parts"])) for slab in g["slabs"]]
    lower = [top for slab,base,top in bounds if top <= cut]
    upper = [base for slab,base,top in bounds if base > cut]
    return [(slab,base > cut,top <= cut) for slab,base,top in bounds
            if base <= cut < top or lower and abs(top-max(lower)) < 1e-9 or upper and abs(base-min(upper)) < 1e-9]


def part_plan_outline(part, cut):
    """🔻 Read the true cut ring, visible upper cap below, or underside above the plan plane."""
    if "section" in part or "mesh" in part:
        return _c._model.part_outline_at(part,cut) if part["z0"]-1e-10 <= cut <= part["z1"]+1e-10 else _c._model.part_footprint(part)
    return _c._model.part_outline_at(part,cut) if "top_outline" in part else part["outline"]


def plan_part_rank(data):
    """🪵 Source floor finishes sit beneath fixtures, walls and ramps."""
    return 1.0 if data["kind"] in ("parquet","tile","paving","grass_joint") else 3.5


def plan_part_visible(data, part, level, band, below, structural=False):
    """🧱 Keep capped finish layers and buried structure only on their authored storey."""
    elev,top = band
    if part["z0"] >= top-1e-6:
        return False
    if plan_part_rank(data) == 1.0:
        return data["storey"] == level
    return part["z1"] >= elev-1e-6 or structural and below and data["storey"] == level


def slab_cut(slab, axis, value, clip=None):
    """🟫 Physical regional slab rectangles at a plane, already respecting source voids."""
    cells = []
    for part in slab["parts"]:
        for a,b in profile_cut(part["outline"],axis,value):
            if clip is not None:
                a,b = max(a,clip[0]),min(b,clip[1])
            if b-a > 1e-9:
                cells.append((a,b,part["z0"],part["z1"]))
    return cells


def rect_union_cells(rectangles):
    """🧩 Nonoverlapping rectangular union cells for seam-free section boundaries."""
    xs = sorted({v for a,b,c,d in rectangles for v in (a,b)})
    zs = sorted({v for a,b,c,d in rectangles for v in (c,d)})
    return [(a,b,c,d) for a,b in zip(xs,xs[1:]) for c,d in zip(zs,zs[1:])
            if any(x0 < (a+b)/2 < x1 and z0 < (c+d)/2 < z1 for x0,x1,z0,z1 in rectangles)]


def point_in_ring(point, ring):
    """📍 Even-odd containment for a point displaced from physical part boundaries."""
    x,y = point
    inside = False
    for a,b in zip(ring,ring[1:]+ring[:1]):
        if (a[1] > y) != (b[1] > y) and x < a[0]+(y-a[1])*(b[0]-a[0])/(b[1]-a[1]):
            inside = not inside
    return inside


def slab_sides(slab, occluders=None):
    """🧱 Exposed regional side faces, subtracting neighboring occupied vertical intervals."""
    faces = []
    occupied = slab["parts"] if occluders is None else occluders
    for part in slab["parts"]:
        ring = part["outline"]
        area = sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
        for a,b in zip(ring,ring[1:]+ring[:1]):
            dx,dy = b[0]-a[0],b[1]-a[1]
            squared = dx*dx+dy*dy
            if squared < 1e-18:
                continue
            length = squared**0.5
            normal = (dy/length,-dx/length) if area > 0 else (-dy/length,dx/length)
            ts = {0.0,1.0}
            for other in occupied:
                for p in other["outline"]:
                    if abs((p[0]-a[0])*dy-(p[1]-a[1])*dx) < 1e-8*length:
                        t = ((p[0]-a[0])*dx+(p[1]-a[1])*dy)/squared
                        if 0 < t < 1:
                            ts.add(t)
            values = sorted(ts)
            for t0,t1 in zip(values,values[1:]):
                mid = (t0+t1)/2
                probe = (a[0]+dx*mid+normal[0]*1e-7,a[1]+dy*mid+normal[1]*1e-7)
                neighbors = [p for p in occupied if p is not part and point_in_ring(probe,p["outline"])]
                zs = sorted({part["z0"],part["z1"],*(max(part["z0"],min(part["z1"],p[key])) for p in neighbors for key in ("z0","z1"))})
                for z0,z1 in zip(zs,zs[1:]):
                    if z1-z0 > 1e-9 and not any(p["z0"] < (z0+z1)/2 < p["z1"] for p in neighbors):
                        faces.append(((a[0]+dx*t0,a[1]+dy*t0),(a[0]+dx*t1,a[1]+dy*t1),z0,z1,normal))
    return faces


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

        ``level`` is an authored storey key, including ``"cellar"``; ``point`` is the screen position of the box centre (or of ``centre``, model metres), ``scale`` screen units per metre.
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
        d.cut_z = plan_cut_z(g,level)
        catalogue = _s.items(model)
        entries: list = []
        use = set(layers)
        w = 0.7 if thumbnail else 1.0
        ctx = {"g": g, "view": view, "look": look, "level": level, "scale": scale, "w": w, "window": window, "bands": bands, "catalogue": catalogue, "cut_z": d.cut_z, "use": use, "below": below,
               "put": lambda rank, tag, group, anchor=None, aliases=(): entries.append((rank, len(entries), tag, group, anchor, aliases)),
               "screen": lambda ring: [view(p[0], p[1], 0.0) for p in ring]}
        if use & {"architecture", "structure"}:
            self._plan_slabs(ctx)
        if "structure" in use:
            self._plan_pilotis(ctx, halo, below, thumbnail)
        if "architecture" in use:
            self._plan_walls(ctx)
            if below:
                self._plan_below_walls(ctx,hidden=True)
            curtain = g.get("curtain_wall")
            lower_levels = [value for value in g["storeys"].values() if value < elev]
            if curtain and curtain["base"] <= d.cut_z <= curtain["top"]:
                self._plan_curtain(ctx,hidden=False)
            elif curtain and below and lower_levels and max(lower_levels) < curtain["top"] < elev:
                self._plan_curtain(ctx,hidden=True)
            self._plan_screens(ctx)
            self._plan_spaces(ctx)
        if use & {"architecture", "structure"}:
            self._plan_solids(ctx)
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
            position = view(0.0, box["width"], 0.0) + np.array([-0.5, 0.65, 0.0])
            direction = np.asarray(g["north"],float)
            angle = np.arctan2(direction[1],direction[0])-np.pi/2
            compass = self.north_arrow(position,0.42)
            for part in compass.submobjects[:2]:
                part.rotate(angle,about_point=position)
            compass.submobjects[2].move_to(position+np.array([direction[0],direction[1],0.0])*0.44)
            d.parts["north"] = compass
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
        for slab,above,under in adjacent_slabs(g,ctx["cut_z"]):
            item = _s.pseudo(slab["tag"], "IfcSlab", "slabs", slab["guid"], ("architecture", "structure"))
            fill, stroke, opacity, bonus = self._pair(look, item, CONCRETE, INK)
            rings = [screen(slab["outline"])]+[screen(v) for v in slab["voids"]]
            group = _engine.VGroup()
            for part in slab["parts"]:
                crossed = part["z0"] <= ctx["cut_z"] < part["z1"]
                group.add(_d.region([screen(part["outline"])],fill,(0.35 if crossed else 0.06 if above else 0.1)*opacity,None,0))
            for ring in rings[1:]:
                if above:
                    group.add(_d.polyline(ring, stroke, W_HAIR + 0.3 + bonus, 0.55 * opacity, closed=True, dash=0.06))
                else:
                    group.add(_d.polyline(ring, stroke, W_THIN + bonus, 0.8 * opacity, closed=True))
            if under:
                group.add(_d.polyline(rings[0], stroke, W_EDGE + bonus, 0.9 * opacity, closed=True))
            put(0.2, slab["tag"], group)
            if not above:
                finishes = {}
                for part in slab.get("finish_parts",[]):
                    colour = _c.surface_color({"kind":part["layer"]})
                    finishes.setdefault(part["layer"],_engine.VGroup()).add(_d.region([screen(part["outline"])],colour,0.3*opacity,None,0))
                for layer,shapes in finishes.items():
                    put(1.0,slab["tag"]+":"+layer,shapes)
        if level == "ground":
            ctx["put"](0.1, "box", _engine.VGroup(_d.polyline(screen(outline), INK, W_HAIR + 0.3, 0.5, closed=True, dash=0.08)))

    def _plan_pilotis(self, ctx, halo, below, thumbnail):
        """🏛️ Authored column contours at the cut, with circular selection halos only."""
        view,look,w,cut = ctx["view"],ctx["look"],ctx["w"],ctx["cut_z"]
        for item in ctx["catalogue"]:
            if item["kind"] != "pilotis":
                continue
            data = item["data"]
            if data["top"] <= ctx["bands"][ctx["level"]][0] or data["base"] > cut:
                continue
            fill,stroke,opacity,bonus = self._pair(look,item,CONCRETE,INK)
            group = _engine.VGroup()
            for part in data["parts"]:
                if not _c.inside(ctx["window"],_c._model.part_footprint(part),0.5):
                    continue
                ring = ctx["screen"](part_plan_outline(part,cut))
                if part["z0"] <= cut < part["z1"]:
                    group.add(_d.region([ring,*[ctx["screen"](h) for h in part["holes"]]],fill,0.9*opacity,stroke,(W_EDGE+bonus)*w,opacity) if part.get("holes") else _d.polygon(ring,fill,0.9*opacity,stroke,(W_EDGE+bonus)*w,opacity))
                elif below and part["z1"] <= cut and part["z1"] > ctx["bands"][ctx["level"]][0]:
                    group.add(_d.polyline(ring,stroke,(W_HAIR+bonus)*w,0.55*opacity,closed=True,dash=0.045))
            if not len(group.submobjects):
                continue
            centre = view(data["x"],data["y"],0.0)
            lit = look.state(item) == "highlight"
            if lit and halo > 0 and not thumbnail:
                colour = look.highlight_color(item)
                radius = max(halo,data["bounding_radius"]*ctx["scale"]+0.05)
                group.add(_d.polygon(_s.circle(centre,radius,40),colour,0.1),_d.polyline(_s.circle(centre,radius,40),colour,W_THIN+0.4,1.0,closed=True))
            ctx["put"](6.5 if lit else 6.0,item["tag"],group,centre)

    def _plan_below_walls(self, ctx, hidden: bool):
        """🟩 Explicit walls on the immediately preceding authored storey."""
        if not hidden:
            self._plan_walls(ctx)
            return
        group = _engine.VGroup()
        elev = ctx["bands"][ctx["level"]][0]
        lower = [(value,name) for name,value in ctx["g"]["storeys"].items() if value < elev]
        previous = max(lower)[1] if lower else None
        for wall in ctx["g"]["walls"]:
            if wall["storey"] == previous:
                group.add(_d.polyline(ctx["screen"]([wall["start"], wall["end"]]), MUTED, W_HAIR, 0.3, dash=0.06))
        ctx["put"](1.4,"below_walls",group)

    def _plan_curtain(self, ctx, hidden: bool):
        """🪟 Draw actual curtain members and its native hosted door solids from shared physical parts."""
        view,look,w=ctx["view"],ctx["look"],ctx["w"]
        items=[it for it in ctx["catalogue"] if it["kind"]=="curtain" or it["kind"] in ("doors","windows") and "parts" in it["data"]]
        for item in items:
            fill,stroke,opacity,bonus=self._pair(look,item,GLASS,GLASS)
            group=_engine.VGroup()
            for part in item["data"]["parts"]:
                if hidden:
                    group.add(_d.polyline(ctx["screen"](part["outline"]+[part["outline"][0]]),stroke,W_HAIR+.3,.5*opacity,dash=.06))
                elif part["z0"]<=ctx["cut_z"]<part["z1"]:
                    group.add(_d.polygon([view(*p) for p in part["outline"]],GLASS if part["role"]=="glass" else INK,opacity,stroke,(W_THIN+bonus)*w))
            opening=item["data"].get("opening")
            ctx["put"](1.5 if hidden else 5.,item["tag"],group,aliases=(opening,) if opening else ())

    def _plan_walls(self, ctx):
        """⬜ Explicit storey walls with cut openings, glazing and diagrammatic door swings."""
        view, look, w, cut = ctx["view"], ctx["look"], ctx["w"], ctx["cut_z"]
        for wall in ctx["g"]["walls"]:
            if wall["storey"] != ctx["level"] or wall["base"] > cut:
                continue
            if not _c.inside(ctx["window"], [wall["start"], wall["end"]], 0.5):
                continue
            item = next((it for it in ctx["catalogue"] if it["tag"] == wall["tag"]), None) or _s.pseudo(wall["tag"], "IfcWall", "walls", wall["guid"])
            a, b = np.array(wall["start"], float), np.array(wall["end"], float)
            direction = (b-a)/wall["length"]
            n = np.array([-direction[1], direction[0]])
            half = wall["thickness"]/2
            fill, stroke, opacity, bonus = self._pair(look, item, GROUND_WALL_FILL if wall["storey"] == "ground" else WALL_FILL, INK)
            group = _engine.VGroup()
            for part in wall["parts"]:
                if part["z0"] <= cut < part["z1"] or wall["top"] <= cut:
                    group.add(_d.polygon([view(*p) for p in part["outline"]],fill,opacity,stroke,w+bonus))
            ctx["put"](4.0, wall["tag"], group)
            for op in wall["openings"]:
                if not op["sill_z"] <= cut <= op["head_z"]:
                    continue
                p0,p1 = np.array(op["p0"],float),np.array(op["p1"],float)
                tag = op.get("fill_tag") or op.get("window_tag")
                cls = op.get("fill_class")
                item = next((it for it in ctx["catalogue"] if it["tag"] == tag),None) or _s.pseudo(tag or op["tag"],cls or "IfcOpeningElement","openings",op["guid"])
                gfill,gs,go,gb = self._pair(look,item,GLASS if cls != "IfcDoor" else CONCRETE,GLASS if cls != "IfcDoor" else INK)
                shape = _engine.VGroup()
                filling = next((f for f in ctx["g"]["fillings"] if f["tag"]==tag),None)
                if filling:
                    for part in filling["parts"]:
                        if part["z0"] <= cut < part["z1"]:
                            shape.add(_d.polygon([view(*p) for p in part["outline"]],GLASS if part["role"]=="glass" else INK,go,gs,(W_THIN+gb)*w))
                    if cls == "IfcDoor" and filling["operation"]=="swing" and len({p.get("panel_index") for p in filling["parts"] if p["role"] in ("glass","panel")})==1:
                        hinge = a+direction*filling["clear_start"]
                        leaf_width = filling["clear_width"]
                        closed,opened = direction*leaf_width,n*leaf_width
                        shape.add(_d.polyline([view(*hinge),view(*(hinge+opened))],gs,(W_THIN+gb)*w,go))
                        arc = [hinge+closed*np.cos(t)+opened*np.sin(t) for t in np.linspace(0,np.pi/2,17)]
                        shape.add(_d.polyline(ctx["screen"](arc),gs,W_HAIR*w,0.7*go))
                ctx["put"](5.0, tag or op["tag"], shape, aliases=(op["tag"],) if tag else ())

    def _plan_solids(self, ctx):
        """🪜 Authored treads, landings, fixtures, rooflights, parapets and guard solids."""
        elev, top = ctx["bands"][ctx["level"]]
        for kind in ("stairs", "details", "furnishings", "railings", "foundations", "beams"):
            for data in ctx["g"].get(kind, []):
                structural = "structure" in data["domains"]
                if not set(data["domains"]).intersection(ctx["use"]):
                    continue
                if not structural and kind != "stairs" and data["storey"] != ctx["level"]:
                    continue
                item = next((it for it in ctx["catalogue"] if it["tag"] == data["tag"]),None) or _s.pseudo(data["tag"],data["class"],kind,data["guid"])
                group = _engine.VGroup()
                for part in data["parts"]:
                    fill,stroke,opacity,bonus = self._pair(ctx["look"],item,_c.surface_color(data,part),INK)
                    if kind == "foundations" and "architecture" in ctx["use"] and ctx["level"] == "ground" and part["z1"] < ctx["g"]["storeys"]["ground"]-1e-9 and ctx["look"].state(item) != "highlight":
                        continue
                    if not plan_part_visible(data,part,ctx["level"],(elev,top),ctx["below"],structural):
                        continue
                    ring = ctx["screen"](part_plan_outline(part,ctx["cut_z"]))
                    above = part["z0"] > ctx["cut_z"] or structural and part["z1"] < elev
                    group.add(_d.polygon(ring,fill,0.08*opacity if above else 0.3*opacity),_d.polyline(ring,stroke,(W_HAIR if above else W_THIN)+bonus,0.45*opacity if above else opacity,closed=True,dash=0.05 if above else None))
                if len(group.submobjects):
                    ctx["put"](plan_part_rank(data),data["tag"],group)

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
            if data["base"] > ctx["cut_z"] or data["top"] <= ctx["bands"][ctx["level"]][0]:
                continue
            fill, stroke, opacity, bonus = self._pair(look, item, WALL_FILL, INK)
            group = _engine.VGroup()
            for part in data["parts"]:
                if not part["z0"] <= ctx["cut_z"] < part["z1"]:
                    continue
                ring = [view(*p,0.) for p in _c._model.part_outline_at(part,ctx["cut_z"])]
                group.add(_s.poche([ring],fill,hatch=False,stroke=stroke,width=(W_CUT*.7+bonus)*w,opacity=opacity))
            ctx["put"](4.,item["tag"],group)

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
                strip = [view(*p,0.0) for p in _c._model.part_footprint(flight["parts"][0])]
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
                if landing["owner"]!=ramp["tag"]:
                    continue
                if elev - 1e-9 <= landing["z"] < top - 1e-9:
                    landings.add(_d.polyline([view(*p, 0.0) for p in landing["outline"]], stroke, W_THIN * w, 0.7 * opacity, closed=True))
            if len(landings.submobjects):
                ctx["put"](3.0, ramp["tag"] + ":landings", landings)

    def _plan_services(self, ctx):
        """🔧 Canonical equipment and pipe footprints with model-authored vertical bounds."""
        look = ctx["look"]
        elev,top = ctx["bands"][ctx["level"]]
        for item in ctx["catalogue"]:
            if item["kind"] != "services":
                continue
            data = item["data"]
            fill,stroke,opacity,bonus = self._pair(look,item,SERV,SERV)
            group = _engine.VGroup()
            for part in data["parts"]:
                if part["z1"] < elev or part["z0"] >= top or not _c.inside(ctx["window"],_c._model.part_footprint(part),0.5):
                    continue
                if "size" not in data and not part["z0"]-1e-10 <= ctx["cut_z"] <= part["z1"]+1e-10:
                    group.add(_d.polyline(ctx["screen"](_c._model.part_footprint(part)),stroke,W_HAIR,.5*opacity,closed=True))
                    continue
                outline = part_plan_outline(part,ctx["cut_z"])
                if len(outline)<3:
                    continue
                ring = ctx["screen"](outline)
                group.add(_d.polygon(ring,fill,0.25*opacity,stroke,W_THIN+bonus,opacity))
                if data["kind"] in ("boiler","drain") and "size" in data:
                    group.add(_d.segments([(ring[0],ring[2]),(ring[1],ring[3])],stroke,W_HAIR,opacity))
            if len(group.submobjects):
                ctx["put"](7.0,item["tag"],group)

    def _plan_spaces(self, ctx):
        """🏷️ Selected room outlines without inferring a floor finish from a room name."""
        for space in ctx["g"]["spaces"]:
            if space["storey"] != ctx["level"]:
                continue
            item = _s.pseudo(space["tag"],"IfcSpace","spaces",space["guid"])
            if ctx["look"].state(item) == "highlight":
                colour = ctx["look"].highlight_color(item)
                ctx["put"](2.0,space["tag"],_engine.VGroup(_d.polyline(ctx["screen"](space["outline"]),colour,W_THIN,0.8,closed=True)))

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
                                                                                                                                                                                                                                                                                    