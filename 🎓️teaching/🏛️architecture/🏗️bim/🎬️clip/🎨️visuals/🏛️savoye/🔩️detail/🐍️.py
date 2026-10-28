"""🔩 Native physical section details of Villa Savoye source elements."""
from __future__ import annotations

import importlib

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_engine = importlib.import_module("manim")


def _part(drawing, tag, shape, anchor=None):
    """🏷️ Paint an addressable detail without repeating its geometry in the drawing family."""
    drawing.add(shape)
    drawing.part(tag, shape, anchor)


class Detail:
    """🧱 Explicit geometric details and structural idealization controls over the source model."""

    def piloti_detail(self, point=(0, 0, 0), scale: float = 0.62, *, model=None, tag: str = "B3", dims=True, grow=1.0):
        """🏛️ Show one circular piloti in elevation and section, with an authored height reveal."""
        if scale <= 0 or not 0 <= grow <= 1:
            raise ValueError("A piloti detail requires positive scale and a reveal between zero and one")
        g = _c.geometry(model)
        piloti = next(item for item in g["pilotis"] if item["tag"] == tag)
        x, y, radius = piloti["x"], piloti["y"], piloti["radius"]
        base, top = piloti["base"], piloti["top"]
        view = _d.View("section", point, scale, centre=(x, y, (base + top) / 2), axis="x")
        drawing = _d.Drawing(view)
        revealed = base + (top - base) * grow
        if revealed > base:
            elevation = _d.polygon([view(x - radius, y, base), view(x + radius, y, base),
                                    view(x + radius, y, revealed), view(x - radius, y, revealed)],
                                   _d.CONCRETE, 1, _d.STRUCT, _d.W_EDGE)
        else:
            elevation = _d.polyline([view(x - radius, y, base), view(x + radius, y, base)], _d.STRUCT, _d.W_EDGE)
        _part(drawing, "elevation", elevation)
        drawing.part(tag, elevation)
        section_centre = _d.P(point) + _engine.RIGHT * (0.85 + radius * scale)
        section = _engine.Circle(radius=radius * scale, fill_color=_d.STRUCT, fill_opacity=0.5,
                                 stroke_color=_d.STRUCT, stroke_width=_d.W_EDGE).move_to(section_centre)
        _part(drawing, "section", section, section_centre)
        drawing.add(self.text(tag + " · IfcColumn", _d.P(point) + _engine.UP * ((top - base) * scale / 2 + 0.4),
                              size=_d.S_LABEL, color=_d.DATA))
        if dims:
            drawing.add(self.dimension(section_centre + _engine.LEFT * radius * scale, section_centre + _engine.RIGHT * radius * scale,
                                       self.quantity(radius * 2, "m"), offset=-0.25, color=_d.DATA))
            if grow > 0:
                drawing.add(self.dimension(view(x - radius, y, base), view(x - radius, y, revealed),
                                           self.quantity((top - base) * grow, "m"), offset=0.4, color=_d.DATA))
        return drawing

    def slab_strip(self, point=(0, 0, 0), scale: float = 0.62, *, model=None, row: str = "B", bays=(2, 4),
                   collapse=0.0, supports=None):
        """📏 Cut a structural strip through named grid supports and collapse its slab onto the middle surface."""
        if scale <= 0 or not 0 <= collapse <= 1:
            raise ValueError("A slab strip requires positive scale and a collapse between zero and one")
        if supports not in (None, "fixed", "pinned"):
            raise ValueError("Unknown authored support fixity")
        g = _c.geometry(model)
        if row not in g["grid"]["labels_y"] or len(bays) != 2 or not 1 <= bays[0] < bays[1] <= len(g["grid"]["x"]):
            raise ValueError("A strip needs one model grid row and an ordered pair of grid columns")
        y = g["grid"]["y"][g["grid"]["labels_y"].index(row)]
        x0, x1 = (g["grid"]["x"][index - 1] for index in bays)
        slab = next(item for item in g["slabs"] if item["kind"] == "FLOOR")
        middle = slab["top"] - slab["thickness"] / 2
        view = _d.View("section", point, scale, centre=((x0 + x1) / 2, y, middle / 2), axis="x")
        drawing = _d.Drawing(view)
        thickness = slab["thickness"] * (1 - collapse)
        if thickness > 0:
            shape = _d.polygon([view(x0, y, middle - thickness / 2), view(x1, y, middle - thickness / 2),
                                view(x1, y, middle + thickness / 2), view(x0, y, middle + thickness / 2)],
                               _d.STRUCT, 0.6, _d.STRUCT, _d.W_EDGE)
        else:
            shape = _d.polyline([view(x0, y, middle), view(x1, y, middle)], _d.DATA, _d.W_CUT)
        _part(drawing, slab["tag"], shape)
        for piloti in g["pilotis"]:
            if abs(piloti["y"] - y) > 1e-6 or not x0 - 1e-6 <= piloti["x"] <= x1 + 1e-6:
                continue
            radius, x = piloti["radius"], piloti["x"]
            if collapse < 1:
                shape = _d.polygon([view(x - radius, y, piloti["base"]), view(x + radius, y, piloti["base"]),
                    view(x + radius, y, middle), view(x - radius, y, middle)], _d.CONCRETE, 0.8, _d.STRUCT, _d.W_THIN)
            else:
                shape = _d.polyline([view(x, y, piloti["base"]), view(x, y, middle)], _d.DATA, _d.W_CUT)
            _part(drawing, piloti["tag"], shape)
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
        a, along, normal, length = _c.wall_frame(wall, g)
        centre = a + along * length / 2
        view = _d.View("section", point, scale, centre=(centre[0], centre[1] + wall["thickness"] / 2, (wall["base"] + wall["top"]) / 2), axis="y")
        drawing = _d.Drawing(view)
        opening = next(item for item in wall["openings"] if item["start"] <= length / 2 <= item["end"])
        y0, y1 = centre[1], centre[1] + wall["thickness"]
        group = _engine.VGroup()
        for z0, z1 in ((wall["base"], opening["sill_z"]), (opening["head_z"], wall["top"])):
            group.add(_d.polygon([view(centre[0], y0, z0), view(centre[0], y1, z0), view(centre[0], y1, z1), view(centre[0], y0, z1)],
                                 _d.ARCH, 0.75, _d.ARCH, _d.W_CUT))
        _part(drawing, tag, group)
        glass = _d.polyline([view(centre[0], (y0 + y1) / 2, opening["sill_z"]), view(centre[0], (y0 + y1) / 2, opening["head_z"])], _d.GLASS, _d.W_EDGE)
        _part(drawing, opening["window_tag"], glass)
        if dims:
            drawing.add(self.dimension(view(centre[0], y0, wall["base"]), view(centre[0], y1, wall["base"]),
                                       self.quantity(wall["thickness"], "m"), offset=-0.3, color=_d.DATA))
        return drawing
