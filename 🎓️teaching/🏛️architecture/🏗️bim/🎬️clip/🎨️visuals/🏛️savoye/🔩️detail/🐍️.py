"""🔩 Native physical section details of Villa Savoye source elements."""
from __future__ import annotations

import importlib

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_engine = importlib.import_module("manim")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_plan = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")


def _part(drawing, tag, shape, anchor=None):
    """🏷️ Paint an addressable detail without repeating its geometry in the drawing family."""
    drawing.add(shape)
    drawing.part(tag, shape, anchor)


class Detail:
    """🧱 Explicit geometric details and structural idealization controls over the source model."""

    def piloti_detail(self, point=(0, 0, 0), scale: float = 0.62, *, model=None, tag: str | None = None, dims=True, grow=1.0):
        """🏛️ Show authored column contours, part heights and two profile dimensions."""
        if scale <= 0 or not 0 <= grow <= 1:
            raise ValueError("A piloti detail requires positive scale and a reveal between zero and one")
        g = _c.geometry(model)
        tag = tag or _model.role("column_example",model)
        piloti = next(item for item in g["pilotis"] if item["tag"] == tag)
        x,y = piloti["x"],piloti["y"]
        base,top = piloti["base"],piloti["top"]
        view = _d.View("section",point,scale,centre=(x,y,(base+top)/2),axis="x")
        drawing = _d.Drawing(view)
        revealed = base+(top-base)*grow
        elevation = _engine.VGroup()
        for part in piloti["parts"]:
            z0,z1 = part["z0"],min(part["z1"],revealed)
            if z1 <= z0:
                continue
            if "top_outline" in part:
                visible = {**part,"z1":z1,"top_outline":_model.part_outline_at(part,z1)}
                points = [view(h,y,z) for h,z in _model.part_elevation(visible)]
            else:
                xs = [p[0] for p in part["outline"]]
                points = [view(min(xs),y,z0),view(max(xs),y,z0),view(max(xs),y,z1),view(min(xs),y,z1)]
            elevation.add(_d.polygon(points,_d.CONCRETE,1,_d.STRUCT,_d.W_EDGE))
        _part(drawing,"elevation",elevation)
        drawing.part(tag,elevation)
        section_z = (base+revealed)/2
        active = next(part for part in piloti["parts"] if part["z0"] <= section_z <= part["z1"])
        outline = _model.part_outline_at(active,section_z)
        drawing.section_z = section_z
        xs,ys = [p[0] for p in outline],[p[1] for p in outline]
        width,depth = max(xs)-min(xs),max(ys)-min(ys)
        section_centre = np.asarray(_d.P(point), float)+_engine.RIGHT*(0.85+width*scale/2)
        ring = [section_centre+_engine.RIGHT*((p[0]-x)*scale)+_engine.UP*((p[1]-y)*scale) for p in outline]
        holes = [[section_centre+_engine.RIGHT*((p[0]-x)*scale)+_engine.UP*((p[1]-y)*scale) for p in h] for h in active.get("holes",())]
        section = _d.region([ring,*holes],_d.STRUCT,0.5,_d.STRUCT,_d.W_EDGE) if holes else _d.polygon(ring,_d.STRUCT,0.5,_d.STRUCT,_d.W_EDGE)
        _part(drawing,"section",section,section_centre)
        drawing.add(self.text(tag+" · IfcColumn",np.asarray(_d.P(point), float)+_engine.UP*((top-base)*scale/2+0.4),size=_d.S_LABEL,color=_d.DATA))
        if dims:
            drawing.add(self.dimension(section_centre+_engine.RIGHT*((min(xs)-x)*scale),section_centre+_engine.RIGHT*((max(xs)-x)*scale),self.quantity(width,"m"),offset=-0.3,color=_d.DATA))
            drawing.add(self.dimension(section_centre+_engine.UP*((min(ys)-y)*scale),section_centre+_engine.UP*((max(ys)-y)*scale),self.quantity(depth,"m"),offset=width*scale/2+0.3,color=_d.DATA))
            if grow > 0:
                drawing.add(self.dimension(view(min(xs),y,base),view(min(xs),y,revealed),self.quantity((top-base)*grow,"m"),offset=0.4,color=_d.DATA))
        return drawing

    def slab_strip(self, point=(0, 0, 0), scale: float = 0.62, *, model=None, row: str | float | None = None, bays=None, tag: str | None = None,
                   collapse=0.0, supports=None):
        """📏 Cut a structural strip through named grid supports and collapse its slab onto the middle surface."""
        if scale <= 0 or not 0 <= collapse <= 1:
            raise ValueError("A slab strip requires positive scale and a collapse between zero and one")
        if supports not in (None, "fixed", "pinned"):
            raise ValueError("Unknown authored support fixity")
        g = _c.geometry(model)
        reference = next(item for item in g["pilotis"] if item["tag"] == (tag or _model.role("column_example",model)))
        slab = next(item for item in g["slabs"] if item["tag"] == _model.role("floor",model))
        if row is None:
            y = reference["y"]
        elif isinstance(row,(int,float)):
            y = row
        else:
            y = g["grid"]["y"][g["grid"]["labels_y"].index(row)]
        if bays is None:
            spans = _plan.profile_cut(slab["outline"],1,y)
            if not spans:
                raise ValueError("The strip plane misses the authored floor")
            x0,x1 = min(a for a,b in spans),max(b for a,b in spans)
        else:
            if len(bays) != 2 or not 1 <= bays[0] < bays[1] <= len(g["grid"]["x"]):
                raise ValueError("A strip needs an ordered pair of reference-grid columns")
            x0,x1 = (g["grid"]["x"][index-1] for index in bays)
        cells = _plan.slab_cut(slab,1,y,(x0,x1))
        if not cells:
            raise ValueError("The strip plane misses every occupied slab region")
        def station_cell(x):
            """📍 Physical regional interval nearest an authored support station."""
            nearby = [cell for cell in cells if cell[0]-1e-9 <= x <= cell[1]+1e-9]
            return min(nearby or cells,key=lambda cell:abs((cell[0]+cell[1])/2-x))
        def station_middle(x):
            """📏 Local regional middle surface at an authored support station."""
            cell = station_cell(x)
            return (cell[2]+cell[3])/2
        middle = station_middle(reference["x"])
        view = _d.View("section",point,scale,centre=((x0+x1)/2,y,middle/2),axis="x")
        drawing = _d.Drawing(view)
        shape = _engine.VGroup()
        for a,b,z0,z1 in cells:
            centre = (z0+z1)/2
            half = (z1-z0)*(1-collapse)/2
            if half > 0:
                shape.add(_d.polygon([view(a,y,centre-half),view(b,y,centre-half),view(b,y,centre+half),view(a,y,centre+half)],_d.STRUCT,0.6,_d.STRUCT,_d.W_EDGE))
            else:
                shape.add(_d.polyline([view(a,y,centre),view(b,y,centre)],_d.DATA,_d.W_CUT))
        _part(drawing,slab["tag"],shape)
        for piloti in g["pilotis"]:
            if piloti["base"] >= station_cell(piloti["x"])[3] or piloti["top"] <= g["storeys"]["ground"] or not x0-1e-6 <= piloti["x"] <= x1+1e-6:
                continue
            x = piloti["x"]
            group = _engine.VGroup()
            if collapse < 1:
                for part in piloti["parts"]:
                    if part["z0"] >= station_cell(x)[3] or part["z1"] <= g["storeys"]["ground"]:
                        continue
                    support_top = part["z1"]+(station_middle(x)-part["z1"])*collapse
                    if "top_outline" in part:
                        base = max(part["z0"],g["storeys"]["ground"])
                        visible = {**part,"outline":_model.part_outline_at(part,base),"z0":base}
                        cut = _model.part_section(visible,y)
                        points = [view(h,y,base+(z-base)*(support_top-base)/(part["z1"]-base)) for h,z in cut]
                        if points:
                            group.add(_d.polygon(points,_d.CONCRETE,0.8,_d.STRUCT,_d.W_THIN))
                    else:
                        for h0,h1 in _model.part_intervals(part,y):
                            group.add(_d.polygon([view(h0,y,max(part["z0"],g["storeys"]["ground"])),view(h1,y,max(part["z0"],g["storeys"]["ground"])),view(h1,y,support_top),view(h0,y,support_top)],_d.CONCRETE,0.8,_d.STRUCT,_d.W_THIN))
            else:
                group.add(_d.polyline([view(x,y,piloti["base"]),view(x,y,station_middle(x))],_d.DATA,_d.W_CUT))
            if not len(group.submobjects):
                continue
            _part(drawing,piloti["tag"],group)
            if supports:
                foot = view(x, y, piloti["base"])
                if supports == "fixed":
                    mark = _d.segments([(foot + _engine.LEFT * 0.18, foot + _engine.RIGHT * 0.18),
                        (foot + _engine.LEFT * 0.18, foot + _engine.LEFT * 0.28 + _engine.DOWN * 0.12),
                        (foot, foot + _engine.LEFT * 0.1 + _engine.DOWN * 0.12),
                        (foot + _engine.RIGHT * 0.18, foot + _engine.RIGHT * 0.08 + _engine.DOWN * 0.12)], _d.STRUCT, _d.W_EDGE)
                else:
                    mark = _d.polygon([foot, foot + _engine.DOWN * 0.2 + _engine.LEFT * 0.16,
                                       foot + _engine.DOWN * 0.2 + _engine.RIGHT * 0.16], _d.STRUCT, 0.3, _d.STRUCT, _d.W_THIN)
                _part(drawing, "support:" + piloti["tag"], mark, foot)
        return drawing

    def wall_detail(self, point=(0, 0, 0), scale: float = 0.62, *, model=None, tag: str = "W-S", dims=True):
        """🪟 Cut the source wall through its ribbon opening, retaining wall and window identities."""
        if scale <= 0:
            raise ValueError("A wall detail requires positive scale")
        g = _c.geometry(model)
        wall = next(item for item in g["walls"] if item["tag"] == tag)
        length = wall["length"]
        openings = wall["openings"]
        selected = min(openings,key=lambda op:abs((op["start"]+op["end"])/2-length/2)) if openings else None
        station = (selected["start"]+selected["end"])/2 if selected else length/2
        thickness = wall["thickness"]
        view = _d.View("section",point,scale,centre=(0.0,0.0,(wall["base"]+wall["top"])/2),axis="y")
        drawing = _d.Drawing(view)
        y0,y1 = -thickness/2,thickness/2
        group = _engine.VGroup()
        for s,e,z0,z1 in _plan.wall_cells(wall):
            if s <= station < e:
                group.add(_d.polygon([view(0,y0,z0),view(0,y1,z0),view(0,y1,z1),view(0,y0,z1)],_d.ARCH,0.75,_d.ARCH,_d.W_CUT))
        _part(drawing,tag,group)
        for opening in openings:
            if not opening["start"] <= station <= opening["end"]:
                continue
            tag = opening.get("fill_tag") or opening.get("window_tag")
            shape = _engine.VGroup()
            if tag:
                colour = _d.ARCH if opening.get("fill_class") == "IfcDoor" else _d.GLASS
                shape.add(_d.polyline([view(0,0,opening["sill_z"]),view(0,0,opening["head_z"])],colour,_d.W_EDGE))
                _part(drawing,tag,shape)
            drawing.part(opening["tag"],shape)
        if dims:
            drawing.add(self.dimension(view(0,y0,wall["base"]),view(0,y1,wall["base"]),self.quantity(thickness,"m"),offset=-0.3,color=_d.DATA))
        return drawing
     
    def solarium_detail(self, point=(0, 0, 0), scale: float = 1.5, *, model=None, tag: str | None = None, dims=True):
        """🌀 Cut the native landscape aperture, reinforced shelf/head and sloping wall crest."""
        if scale <= 0:
            raise ValueError("A solarium detail requires positive scale")
        g = _c.geometry(model)
        wall = next(item for item in g["screens"] if tag is None or item["tag"]==tag)
        if not wall["openings"]:
            raise ValueError("A solarium detail requires a native landscape aperture")
        opening = wall["openings"][0]
        cut = (opening["p0"][0]+opening["p1"][0])/2
        middle = (opening["p0"][1]+opening["p1"][1])/2
        view = _d.View("section",point,scale,centre=(cut,middle,(wall["base"]+wall["top"])/2),axis="y")
        drawing = _d.Drawing(view)
        sources = [(wall["tag"],wall["parts"])]+[(item["tag"],item["parts"]) for item in g["beams"] if item["storey"]=="roof"]
        for owner,parts in sources:
            group = _engine.VGroup()
            for part in parts:
                if "top_outline" in part:
                    rings = [_model.part_section(part,cut,0)]
                else:
                    rings = [[[lo,part["z0"]],[hi,part["z0"]],[hi,part["z1"]],[lo,part["z1"]]] for lo,hi in _model.part_intervals(part,cut,0)]
                for ring in rings:
                    if ring and any(abs(p[0]-middle)<.8 for p in ring):
                        group.add(_d.polygon([view(cut,h,z) for h,z in ring],_d.CONCRETE,.7,_d.INK,_d.W_THIN))
            if group.submobjects:
                _part(drawing,owner,group)
        drawing.part(opening["tag"],_engine.VGroup())
        if dims:
            for z in (wall["base"],opening["sill_z"],opening["head_z"],wall["body_top"],wall["top"]):
                drawing.add(_d.level_mark(view(cut,middle+.6,z),self.quantity(z,"m"),color=_d.DATA))
        return drawing
