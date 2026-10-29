"""🎬 One Nakagin capsule built parametrically in Grasshopper: sliders, a base plane, Center Box, Fillet Edge, a window cut, four bolts, the type variants, Orient onto a core plane and List Item per plane."""

import importlib
import math
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
_canvas = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🦗️canvas.🐍️")
_gh = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_nkd = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, GOOD, WARN, DATA, SERV = (getattr(_visuals, name) for name in ("INK", "MUTED", "GOOD", "WARN", "DATA", "SERV"))
DARK = "#1B2A33"
SUBTLE = "#5E6A72"
LABEL, SMALL = 19.2, 16.2
SHEET = np.array([-2.6, 0.15, 0.0])
SHEET_W, SHEET_H = 8.8, 5.3
VP = np.array([4.5, 0.15, 0.0])
VP_W, VP_H = 5.0, 5.3
AZIMUTH, ELEVATION = 55.0, 24.0
SCALE = 0.46
SCALE_CORE = 0.25
SCALE_WRAP = 0.22
ORIGIN = VP + np.array([-0.7, -1.0, 0.0])
CORE_AT = VP + np.array([0.75, 0.45, 0.0])
WRAP_AT = VP + np.array([-0.3, 0.45, 0.0])
IDS = ("w", "l", "h", "r", "d", "wu")
BOLT_IDS = ("bu", "bvl", "bvh")
NAMES = {"w": ("Width", "Breite"), "l": ("Length", "Länge"), "h": ("Height", "Höhe"), "r": ("Radius", "Radius"), "d": ("Window Ø", "Fenster Ø"), "wu": ("Offset", "Versatz"),
         "bu": ("Bolt x", "Bolzen x"), "bvl": ("Bolt low", "Bolzen unten"), "bvh": ("Bolt high", "Bolzen oben"), "an": ("Angle", "Winkel")}
SLIDER_AT = {key: (-5.2, 2.35 - 0.8 * index) for index, key in enumerate(IDS)}
SLIDER_W = 2.9
FULL = ("box", "fil", "cut", "cir", "cyl", "ori", "li", "rot")
PARAM_COLOR = {"w": _nkd.AXIS_X, "l": _nkd.AXIS_Y, "h": _nkd.AXIS_Z, "r": "#9B6FD6", "d": "#E08A1E", "wu": "#14A3A8"}
DIM_NAME = {"w": "width", "l": "length", "h": "height"}
RED = "#D8362B"
UNITS = {"fvp": " m³", "wall": " m"}
HEADINGS = {"fvp": ("Volume", "Volumen"), "wall": ("Cylinder length", "Zylinderlänge"), "pa": ("Base plane A [m]", "Grundebene A [m]"),
            "pb": ("Target plane B [m]", "Zielebene B [m]"), "pr": ("Plane B rotated [m]", "Ebene B gedreht [m]"),
            "out": ("Result", "Ergebnis")}
LEGEND = [("Parameter", "Parameter", "Number Slider", "Number Slider"), ("Dependency", "Abhängigkeit", "component + wire", "Komponente + Verbindung"),
          ("Geometry", "Geometrie", "Brep, Circle, Point", "Brep, Circle, Point"), ("Domain", "Domain", "slider limits", "Slider-Grenzen"),
          ("Plane", "Ebene", "origin + X Y Z", "Ursprung + X Y Z"), ("Orient", "Orient", "base to target plane", "Grund- auf Zielebene"),
          ("List Item", "List Item", "type by index", "Typ nach Index"), ("Index", "Index", "0, 1, 2 … wraps", "0, 1, 2 … läuft herum")]


def _fade(group, alpha):
    """🌫️ Scale every stroke and fill opacity of a freshly built group by an alpha so partial reveals survive a redraw."""
    if alpha >= 0.999:
        return group
    for mob in group.get_family():
        mob.set_fill(opacity=float(mob.get_fill_opacity()) * alpha)
        mob.set_stroke(opacity=float(mob.get_stroke_opacity()) * alpha)
    return group


class Beat2_Capsule(_visuals.ClipScene):
    """🧩 Build one parametric Nakagin capsule and place it on a core with explicit English and German narration."""

    TITLE = {"en": "One Capsule, Parametric", "de": "Eine Kapsel, parametrisch"}
    NARRATION = [
        ["hook", "The Nakagin tower hangs 140 capsules on two cores, yet a single Grasshopper definition builds every one of them.",
         "Am Nakagin-Turm hängen 140 Kapseln an zwei Kernen, doch eine einzige Grasshopper-Definition baut jede davon."],
        ["parametric", "Parametric means: parameters feed dependencies, dependencies build geometry. Change one value and everything follows.",
         "Parametrisch heißt: Parameter speisen Abhängigkeiten, Abhängigkeiten bauen Geometrie. Ändert sich ein Wert, folgt alles."],
        ["sliders", "Six Number Sliders hold the parameters: width, length, height, corner radius, window diameter and window offset.",
         "Sechs Number Sliders halten die Parameter: Breite, Länge, Höhe, Eckenradius, Fensterdurchmesser und Fensterversatz."],
        ["plane", "Everything starts on a plane: origin plus X, Y and Z axes in Rhino's red, green and blue. This is the capsule's base plane.",
         "Alles beginnt auf einer Ebene: Ursprung plus X-, Y- und Z-Achse in Rhinos Rot, Grün und Blau. Das ist die Grundebene der Kapsel."],
        ["box", "Center Box takes the plane and the sliders for width, length and height. It centres itself on the plane, so half of it sinks below the floor.",
         "Center Box nimmt die Ebene und die Slider für Breite, Länge und Höhe. Die Box zentriert sich auf der Ebene, die Hälfte versinkt im Boden."],
        ["lift", "Plane Origin lifts the plane by half the length and half the height: now the box stands with its bottom face on the base plane.",
         "Plane Origin hebt die Ebene um die halbe Länge und die halbe Höhe: Jetzt steht die Box mit der Unterseite auf der Grundebene."],
        ["live", "Drag the length slider: the dimension line, the box and every component behind it follow in the same moment.",
         "Ziehen Sie den Längen-Slider: Maßlinie, Box und jede Komponente dahinter folgen im selben Moment."],
        ["rounded", "Fillet Edge rounds the edges with the corner radius slider: the soft shell of the capsule appears.",
         "Fillet Edge rundet die Kanten mit dem Eckenradius-Slider: Die weiche Hülle der Kapsel entsteht."],
        ["wrongplane", "A typical mistake: a Circle on the base plane lies on the floor, not on the front face. A wire does not know where you mean.",
         "Ein typischer Fehler: Ein Circle auf der Grundebene liegt am Boden, nicht an der Vorderseite. Eine Verbindung weiß nicht, wo Sie ihn meinen."],
        ["frontplane", "The fix: a plane on the front face, moved by the length and the window offset, carries the circle to the right place.",
         "Die Lösung: Eine Ebene auf der Vorderseite, verschoben um Länge und Fensterversatz, setzt den Kreis an die richtige Stelle."],
        ["cut", "Solid Difference subtracts a cylinder of that circle from the shell: the round window is cut into the front face.",
         "Solid Difference zieht einen Zylinder dieses Kreises von der Hülle ab: Das runde Fenster entsteht an der Vorderseite."],
        ["bolts", "Four points on the back face mark the high-tension bolts that fix the capsule to the core.",
         "Vier Punkte auf der Rückseite markieren die hochfesten Schrauben, mit denen die Kapsel am Kern hängt."],
        ["domains", "Each slider domain is a design constraint; here the limits are assumptions, for example a transport envelope.",
         "Jede Slider-Domain ist eine Entwurfsgrenze; hier sind die Grenzen Annahmen, zum Beispiel eine Transporthülle."],
        ["limit", "Push the width past its domain: the slider clamps at its upper limit and the envelope turns red. The domain is a design rule.",
         "Schieben Sie die Breite über die Domain hinaus: Der Slider hält am oberen Ende an, die Hülle wird rot. Die Domain ist eine Entwurfsregel."],
        ["types", "The capsule types of the teaching model are one definition with different parameter values.",
         "Die Kapseltypen des Lehrmodells sind eine Definition mit verschiedenen Parameterwerten."],
        ["four", "Feed type A to D into the same sliders: window offset and window diameter change, and the definition builds four capsules.",
         "Speisen Sie Typ A bis D in dieselben Slider ein: Fensterversatz und Fensterdurchmesser ändern sich, die Definition baut vier Kapseln."],
        ["planes", "On the core the capsule needs a target plane on the wall; the base plane must land on it.",
         "Am Kern braucht die Kapsel eine Zielebene an der Wand; die Grundebene muss dort landen."],
        ["orient", "Orient moves the capsule in one step from its base plane A to the target plane B.",
         "Orient bewegt die Kapsel in einem Schritt von ihrer Grundebene A auf die Zielebene B."],
        ["rotate", "Rotate the target plane about the core axis and the capsule turns with it, because it follows plane B.",
         "Dreht man die Zielebene um die Kernachse, dreht die Kapsel mit, denn sie folgt der Ebene B."],
        ["listitem", "List Item picks the type for each plane: index 0 gives type A, 1 gives B, 2 gives C, 3 gives D.",
         "List Item wählt den Typ je Ebene: Index 0 ergibt Typ A, 1 Typ B, 2 Typ C, 3 Typ D."],
        ["outside", "Eight planes need indices up to 7, but there are only four types: without Wrap, List Item warns from index 4.",
         "Acht Ebenen brauchen Indizes bis 7, doch es gibt nur vier Typen: Ohne Wrap warnt List Item ab Index 4."],
        ["wrap", "Switch Wrap on and the index cycles: 4 gives type A again, 5 gives B, and every plane gets a capsule.",
         "Mit Wrap läuft der Index herum: 4 ergibt wieder Typ A, 5 Typ B, und jede Ebene bekommt eine Kapsel."],
        ["recap", "Recap: parameter, dependency, geometry, domain, plane, Orient, List Item and index.",
         "Rückblick: Parameter, Abhängigkeit, Geometrie, Domain, Ebene, Orient, List Item und Index."]
    ]

    def construct(self):
        """▶️ Present this chapter through the native Grasshopper visual interface."""
        self.present("capsule")

    def _put(self, shape, point, edge="c"):
        """📌 Move a shape so its left edge, right edge or centre sits on ``point``."""
        dx = {"l": point[0] - shape.get_left()[0], "r": point[0] - shape.get_right()[0], "c": point[0] - shape.get_center()[0]}[edge]
        return shape.shift(np.array([dx, point[1] - shape.get_center()[1], 0.0]))

    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """🔤 One label aligned on its left edge, right edge or centre."""
        return self._put(self.text(string, (0, 0, 0), size=size, color=color, width=width), point, edge)

    def _ramp(self, tracker, start, stop, seconds, rate=_engine.smooth):
        """⏩ Move a tracker between two values."""
        return _engine.UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(start + (stop - start) * alpha), rate_func=rate, run_time=seconds)

    def _to(self, tracker, stop, seconds):
        """🎯 Move a tracker from where it is to a value."""
        return self._ramp(tracker, tracker.get_value(), stop, seconds)

    def _gone(self, *mobs):
        """🫥 Stop live redraws and fade the mobjects out."""
        for mob in mobs:
            for part in mob.get_family():
                part.clear_updaters()
        return [_engine.FadeOut(mob) for mob in mobs]

    def _tag(self, string, point, color=DARK, size=SMALL):
        """🏷️ A label chip for the light viewport."""
        label = self._say(string, point, size=size, color=color)
        chip = _engine.RoundedRectangle(corner_radius=0.05, width=label.width + 0.16, height=label.height + 0.1, stroke_width=0, fill_color="#FFFFFF", fill_opacity=0.85).move_to(label)
        return _engine.VGroup(chip, label)

    def _solve(self):
        """🧮 Evaluate the capsule canvas for the slider trackers, cached by their rounded values."""
        values = {key: round(tracker.get_value(), 4) for key, tracker in self.st.vals.items()}
        key = tuple(sorted(values.items()))
        if key not in self._cache:
            self._cache[key] = _gh.evaluate(self._d, values)
        return self._cache[key]

    def _solve_place(self):
        """🧮 Evaluate the Orient definition for the angle tracker."""
        angle = round(self.st.cv["angle"].get_value())
        if angle not in self._places:
            self._places[angle] = _gh.evaluate(self._place, {"an": angle})
        return self._places[angle]

    def _solve_wrap(self):
        """🧮 Evaluate the List Item definition for the Wrap toggle."""
        wrap, count = self.st.wv["wrap"].get_value() > 0.5, int(round(self.st.wv["count"].get_value()))
        if (wrap, count) not in self._wraps:
            self._wraps[(wrap, count)] = _gh.evaluate(self._wrap, {"wr": wrap, "idx": [str(i) for i in range(count)]})
        return self._wraps[(wrap, count)]

    @staticmethod
    def _read(solution):
        """📐 Capsule parameters read back from the evaluated Brep of the canvas: box domains, fillet radius, window circle."""
        brep = solution.tree("cut").items()[0]
        box, window = brep.root_box(), brep.window()
        return {"width": box.x.length, "length": box.y.length, "height": box.z.length, "corner_radius": brep.fillet_radius(),
                "window_diameter": 2 * window.radius, "window_u": window.plane.origin.x, "window_v": window.plane.origin.z}

    def _show(self, item):
        """🔢 A panel line for a datum: numbers with language punctuation, points and planes by coordinates."""
        if isinstance(item, (float, int)) and not isinstance(item, bool):
            return self.number(float(item), 2)
        if isinstance(item, str) and "." in item:
            try:
                return self.number(float(item), 2)
            except ValueError:
                return item
        if isinstance(item, (_gh.Point, _gh.Vector)):
            return "{" + self.tr(", ", "; ").join(self.number(c, 2) for c in (item.x, item.y, item.z)) + "}"
        if isinstance(item, _gh.Plane):
            return "O " + self._show(item.origin)
        if isinstance(item, _gh.Brep):
            tag = item.tag.get("type") if item.tag else None
            return "Brep " + tag if tag else "Brep"
        return "null" if item is None else str(item)

    def _panel_lines(self, items, oid):
        """📝 Panel lines of the items of an output: a plane takes four lines (origin, X, Y, Z), everything else one."""
        lines = []
        for item in items:
            if isinstance(item, _gh.Plane):
                lines += [name + " " + self._show(vector) for name, vector in (("O", item.origin), ("X", item.x), ("Y", item.y), ("Z", item.z))]
            else:
                lines.append(self._show(item) + UNITS.get(oid, ""))
        return lines

    def _viewport(self):
        """🪟 Rhino viewport frame without a fixed grid: light gradient, title tab and frame."""
        sky = _engine.Rectangle(width=VP_W, height=VP_H, stroke_width=0).set_fill([_canvas.VIEWPORT_TOP, _canvas.VIEWPORT_BOTTOM], opacity=1).set_sheen_direction(_engine.DOWN)
        frame = _engine.Rectangle(width=VP_W, height=VP_H, stroke_color="#3F6FB5", stroke_width=_canvas.W_EDGE * 1.4, fill_opacity=0)
        tab = self._say(self.tr("Perspective", "Perspektive"), (-VP_W / 2 + 0.2, VP_H / 2 - 0.22), size=SMALL, color=DARK, edge="l")
        return _engine.VGroup(sky, tab, frame).shift(VP)

    def _grid(self, view, reach=6):
        """🟦 Ground grid in the same projection as the capsule, red X and green Y axes through the origin, clipped to the viewport."""
        box = (VP[0] - VP_W / 2 + 0.03, VP[1] - VP_H / 2 + 0.03, VP[0] + VP_W / 2 - 0.03, VP[1] + VP_H / 2 - 0.4)
        clip = lambda pairs: [c for c in (_canvas.clip_segment(a, b, box) for a, b in pairs) if c]
        lines = [(view(i, -reach, 0), view(i, reach, 0)) for i in range(-reach, reach + 1) if i] + [(view(-reach, i, 0), view(reach, i, 0)) for i in range(-reach, reach + 1) if i]
        group = _engine.VGroup(_draft.segments(clip(lines), "#9C9C9C", 0.28, 0.9))
        group.add(_draft.segments(clip([(view(-reach, 0, 0), view(reach, 0, 0))]), _nkd.AXIS_X, 0.3, 0.6), _draft.segments(clip([(view(0, -reach, 0), view(0, reach, 0))]), _nkd.AXIS_Y, 0.3, 0.6))
        return group

    def _stage(self):
        """🧊 The viewport of the build phase redrawn from the engine solution and the reveal trackers: grid, planes, capsule, dimensions, envelope, window mistake and bolts."""
        st = self.st
        rv = {name: tracker.get_value() for name, tracker in st.rv.items()}
        solution = self._solve()
        p = self._read(solution)
        origin = ORIGIN + np.array([0.0, st.oy.get_value(), 0.0])
        view = (st.az.get_value(), ELEVATION)
        v = _nkd.make_view(view, origin, SCALE, (0, 0, 0))
        group = _engine.VGroup(_fade(self._grid(v), rv["grid"]))
        w, l, h = p["width"], p["length"], p["height"]
        lift = rv["lift"]
        if rv["env"] > 0.02:
            hi = {key: self._d.obj(key).settings["max"] for key in ("w", "l", "h")}
            corners = {(i, j, k): v((i - 0.5) * hi["w"], j * hi["l"], k * hi["h"]) for i in (0, 1) for j in (0, 1) for k in (0, 1)}
            edges = [(a, b) for a in corners for b in corners if a < b and sum(x != y for x, y in zip(a, b)) == 1]
            tone = RED if rv["over"] > 0.5 else "#C77700" if rv["hit"] > 0.5 else "#5E6A72"
            envelope = _engine.VGroup(*[_draft.polyline([corners[a], corners[b]], tone, _draft.W_EDGE + 0.4, 0.95, dash=0.1) for a, b in edges])
            envelope.add(self._tag(self.tr("Transport envelope (teaching assumption)", "Transporthülle (Lehrannahme)"), (VP[0], VP[1] + VP_H / 2 - 0.75, 0.0), color="#9A5A00"))
            group.add(_fade(envelope, rv["env"]))
        drawing = None
        if rv["body"] > 0.02:
            params = {"key": "A", "width": w, "length": l, "height": h, "corner_radius": p["corner_radius"] * rv["round"], "window_diameter": p["window_diameter"],
                      "window_u": p["window_u"], "window_v": p["window_v"]}
            dims = tuple(name for key, name in DIM_NAME.items() if rv["d" + key] > 0.02)
            drawing = self.capsule_axon(params, at=v(0, l / 2 * lift, h / 2 * lift), scale=SCALE, view=view, window=rv["window"] > 0.5, dims=dims, base=_canvas.PREVIEW_FILL, dim_offset=0.4)
            for key, name in DIM_NAME.items():
                if name in drawing.dims:
                    for part in drawing.dims[name].get_family():
                        if not isinstance(part, _engine.RoundedRectangle):
                            part.set_color(PARAM_COLOR[key])
                    _fade(drawing.dims[name], rv["d" + key])
            group.add(_fade(drawing, rv["body"]))
            if rv["dd"] > 0.02 and rv["window"] > 0.5:
                u, vv, d = p["window_u"], p["window_v"], p["window_diameter"]
                a, b = drawing.at(u - d / 2, l, vv), drawing.at(u + d / 2, l, vv)
                across = (b - a) / max(np.linalg.norm(b - a), 1e-9)
                normal = np.array([-across[1], across[0], 0.0]) * 0.07
                diameter = _engine.VGroup(_draft.segments([(a, b), (a + normal, a - normal), (b + normal, b - normal)], PARAM_COLOR["d"], 3.0),
                                          self._tag("Ø " + self.quantity(d, "m"), (a + b) / 2 + np.array([0.0, -0.4, 0.0]), color="#9A5A00"))
                group.add(_fade(diameter, rv["dd"]))
            if rv["dr"] > 0.02:
                corner = drawing.at(w / 2, l, h)
                group.add(_fade(self._tag("r " + self.quantity(p["corner_radius"], "m"), corner + np.array([0.45, 0.3, 0.0]), color="#6B3FB0"), rv["dr"]))
            if rv["bolts"] > 0.02:
                dots = _engine.VGroup()
                for point in solution.tree("bolt").items():
                    centre = drawing.at(point.x, point.y, point.z)
                    dots.add(_engine.Circle(radius=0.075, stroke_color=WARN, stroke_width=3, fill_color="#FFFFFF", fill_opacity=0.9).move_to(centre), _engine.Dot(centre, radius=0.025, color=WARN))
                wall = [drawing.at(sx * (w / 2 + 0.45), -0.4, z) for sx, z in ((-1, -0.25), (1, -0.25), (1, h + 0.25), (-1, h + 0.25))]
                plate = _engine.VGroup(_draft.polygon(wall, "#9AA5AB", 0.45, "#5E6A72", _draft.W_THIN))
                for i, point in enumerate(solution.tree("bolt").items()):
                    plate.add(_draft.segments([(drawing.at(point.x, -0.4, point.z), drawing.at(point.x, 0.0, point.z))], WARN, 3.0),
                              self._tag(str(i), drawing.at(point.x, -0.4, point.z) + np.array([0.0, 0.3, 0.0]), color="#9A5A00"))
                plate.add(self._tag(self.tr("Core wall", "Kernwand"), drawing.at(0.0, -0.4, h + 0.25) + np.array([0.0, 0.3, 0.0]), color="#2B3A44"))
                group.add(_fade(plate, rv["bolts"]), _fade(dots, rv["bolts"]))
            if rv["du"] > 0.02:
                a, b = drawing.at(0, l, p["window_v"]), drawing.at(p["window_u"], l, p["window_v"])
                offset = _engine.VGroup(_draft.segments([(a, b)], PARAM_COLOR["wu"], 3.0), _engine.Dot(b, radius=0.04, color=PARAM_COLOR["wu"]))
                if abs(p["window_u"]) > 1e-3:
                    offset.add(self._tag("u " + self.quantity(p["window_u"], "m"), (a + b) / 2 + np.array([0.0, 0.32, 0.0]), color="#0D7377"))
                group.add(_fade(offset, rv["du"]))
        if rv["axes"] > 0.02:
            axes = self.plane_axes(_nk.Plane((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=origin, scale=SCALE, view=view, length=1.7, labels=rv["body"] < 0.5)
            group.add(_fade(axes, rv["axes"]))
        if rv["plane2"] > 0.02:
            centre = (0, l / 2 * lift, h / 2 * lift)
            lifted = self.plane_axes(_nk.Plane(centre, (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=v(*centre), scale=SCALE, view=view, length=1.1, labels=False, square=False)
            group.add(_fade(lifted, rv["plane2"]))
        if rv["wrong"] > 0.02:
            circle = self._wrong_circle()
            ring = [v(*(np.array(circle.plane.origin.t) + circle.radius * (math.cos(a) * np.array(circle.plane.x.t) + math.sin(a) * np.array(circle.plane.y.t)))) for a in np.linspace(0, 2 * math.pi, 60)]
            flat = _engine.VGroup(_draft.polygon(ring, RED, 0.25), _draft.polyline(ring, RED, 3.4, 1.0, closed=True),
                                  self._tag(self.tr("Circle on the base plane", "Circle auf der Grundebene"), v(-2.2, -1.9, 0.0), color="#B3261E"))
            group.add(_fade(flat, rv["wrong"]))
        if rv["front"] > 0.02 and drawing is not None:
            u, vv, d = p["window_u"], p["window_v"], p["window_diameter"]
            centre = drawing.at(u, l, vv)
            plane = self.plane_axes(_nk.Plane((u, l, vv), (1, 0, 0), (0, 0, 1), (0, -1, 0)), at=centre, scale=SCALE, view=view, length=1.3, labels=True, square=False)
            ring = [drawing.at(u + d / 2 * math.cos(a), l, vv + d / 2 * math.sin(a)) for a in np.linspace(0, 2 * math.pi, 48)]
            group.add(_fade(_engine.VGroup(plane, _draft.polyline(ring, PARAM_COLOR["d"], 3.4, 1.0, closed=True)), rv["front"]))
        return group

    def _wrong_circle(self):
        """⭕ The circle of the mistaken definition (Circle wired to the XY Plane), read from its engine solution."""
        values = {"d": round(self.st.vals["d"].get_value(), 4)}
        key = ("wrong", values["d"])
        if key not in self._cache:
            self._cache[key] = _gh.evaluate(self._mistake, values).value("cir")
        return self._cache[key]

    def _on_core(self, core, plane_origin, yaw, type_key, alpha=1.0, color=None):
        """🧱 A capsule of a type standing on a core plane: drawn at the world position of its centre, turned by the plane yaw through the view azimuth."""
        t = _nk.capsule_type(type_key)
        o = np.array(plane_origin, float)
        r = math.radians(yaw)
        centre = o + np.array([-math.sin(r), math.cos(r), 0.0]) * t.length / 2 + np.array([0.0, 0.0, t.height / 2])
        drawing = self.capsule_axon({"key": type_key}, at=core.at(*centre), scale=SCALE_WRAP, view=(AZIMUTH - yaw, ELEVATION), base=color or _nkd.type_color(type_key))
        drawing.facing = core.view.facing((-math.sin(r), math.cos(r), 0.0))
        return _fade(drawing, alpha)

    def _glide(self):
        """🏗️ The core viewport of Orient: the core of tower B, base plane A, target plane B and the capsule gliding from A to B and turning with B, from the engine solution."""
        cv = {name: tracker.get_value() for name, tracker in self.st.cv.items()}
        view = (AZIMUTH, ELEVATION)
        solution = self._solve_place()
        target = solution.items("rot")[0]
        yaw_b = math.degrees(math.atan2(target.x.y, target.x.x))
        group = _engine.VGroup()
        behind = _engine.VGroup()
        front = _engine.VGroup()
        ghost = _engine.VGroup()
        if cv["cap"] > 0.02:
            placed = solution.items("ori")[0].root_box()
            start = np.array(self._plane_a["origin"], float) + np.array([0.0, self._cap_l / 2, self._cap_h / 2])
            centre = start + (np.array(placed.center.t) - start) * cv["glide"]
            yaw = yaw_b * cv["glide"]
            drawing = self.capsule_axon({"key": "A"}, at=self._core.at(*centre), scale=SCALE_CORE, view=(AZIMUTH - yaw, ELEVATION), base=_canvas.PREVIEW_FILL)
            facing = self._core.view.facing((-math.sin(math.radians(yaw)), math.cos(math.radians(yaw)), 0.0))
            if facing >= 0 or cv["glide"] < 0.5:
                front.add(_fade(drawing, cv["cap"]))
            else:
                ghost.add(_fade(drawing, cv["cap"] * 0.75))
        group.add(behind, self._core, front, ghost)
        if cv["arc"] > 0.02:
            cx, cy, cz, _, _ = self._arc
            group.add(_fade(_draft.polyline([self._core.at(cx, cy, 0.0), self._core.at(cx, cy, 11.5)], "#3F6FB5", 2.0, 0.9, dash=0.1), cv["arc"]))
        if cv["a"] > 0.02:
            plane = _nk.Plane(tuple(self._plane_a["origin"]), (1, 0, 0), (0, 1, 0), (0, 0, 1))
            group.add(_fade(self.plane_axes(plane, at=self._core.at(*plane.origin), scale=SCALE_CORE, view=view, length=1.8), cv["a"]))
        if cv["b"] > 0.02:
            plane = _nk.Plane(tuple(target.origin.t), tuple(target.x.t), tuple(target.y.t), tuple(target.z.t))
            group.add(_fade(self.plane_axes(plane, at=self._core.at(*plane.origin), scale=SCALE_CORE, view=view, length=1.8), cv["b"]))
        if cv["a"] > 0.5:
            group.add(self._tag(self.tr("Base plane A", "Grundebene A"), self._core.at(*self._plane_a["origin"]) + np.array([0.55, -0.6, 0.0])))
        if cv["b"] > 0.5:
            group.add(self._tag(self.tr("Target plane B", "Zielebene B"), self._core.at(*target.origin.t) + np.array([-0.35, 0.95, 0.0])))
        if cv["arc"] > 0.02 and abs(cv["angle"]) > 1.0:
            cx, cy, cz, a0, radius = self._arc
            steps = np.linspace(0.0, cv["angle"], 36)
            points = [self._core.at(cx + radius * math.cos(a0 + math.radians(t)), cy + radius * math.sin(a0 + math.radians(t)), cz) for t in steps]
            middle = points[len(points) // 2]
            group.add(_fade(_engine.VGroup(_draft.polyline(points, "#3F6FB5", 3.2, 0.95), _engine.Dot(points[-1], radius=0.05, color="#3F6FB5"),
                                           self._tag(self.number(cv["angle"], 0) + "°", middle + np.array([0.45, -0.35, 0.0]), color="#25518C")), cv["arc"]))
        if cv["arrow"] > 0.02:
            a = self._core.at(*(np.array(self._plane_a["origin"], float)))
            b = self._core.at(*target.origin.t)
            group.add(_fade(_draft.polyline([a, b], "#3F6FB5", 2.6, 0.95, dash=0.14), cv["arrow"]))
        return group

    def _wrapstage(self):
        """🔁 Eight target planes on the core (four facades on two storeys) with the capsule type List Item picks for each index, from the engine solution."""
        wv = {name: tracker.get_value() for name, tracker in self.st.wv.items()}
        solution = self._solve_wrap()
        picks = solution.tree("li").items()
        view = (AZIMUTH, ELEVATION)
        core = self._core_wrap
        behind, front, marks, ghost = _engine.VGroup(), _engine.VGroup(), _engine.VGroup(), _engine.VGroup()
        shown = int(round(wv["count"]))
        for index, slot in enumerate(self._wrap_slots):
            plane = slot.facade_plane
            yaw = math.degrees(math.atan2(plane.x[1], plane.x[0]))
            pick = picks[index] if index < len(picks) else None
            if wv["planes"] > 0.02 and index < shown:
                marks.add(_fade(self.plane_axes(plane, at=core.at(*plane.origin), scale=SCALE_WRAP, view=view, length=1.2, labels=False, square=False), wv["planes"]))
                marks.add(_fade(self._tag(str(index), core.at(*plane.origin) + np.array([0.0, 0.3, 0.0]), color="#2B3A44", size=SMALL), wv["planes"]))
            if pick is not None and wv["caps"] > 0.02 and index < shown and index < wv["rev"]:
                capsule = self._on_core(core, plane.origin, yaw, pick.tag["type"], wv["caps"])
                (front if capsule.facing >= 0 else ghost).add(capsule)
            elif pick is None and wv["caps"] > 0.5 and index < shown:
                marks.add(self._tag("!", core.at(*plane.origin) + np.array([0.3, 0.45, 0.0]), color="#B3261E", size=LABEL))
        strip = _engine.VGroup()
        for index in range(len(self._wrap_slots)):
            x, y = VP[0] - 1.9 + 3.8 / 7 * index, VP[1] - 2.0
            pick = picks[index] if index < len(picks) else None
            if wv["caps"] > 0.02 and index < shown and (index < wv["rev"] or pick is None):
                if pick is not None:
                    strip.add(_fade(self.capsule_axon({"key": pick.tag["type"]}, at=(x, y, 0), scale=0.13, view=view, base=_nkd.type_color(pick.tag["type"])), wv["caps"]))
                else:
                    strip.add(self._tag("!", (x, y, 0), color="#B3261E", size=LABEL))
                strip.add(self._tag(str(index), (x, y - 0.42, 0), color="#2B3A44"))
        return _engine.VGroup(behind, core, front, ghost, marks, strip)

    def _node(self, oid, at, defn, source, *, width=None, live=False, label=None):
        """🧩 One canvas object of a definition as a widget: slider, panel (live when asked), parameter, toggle or component; the result carries the mobject to add and the widget that owns the grips."""
        spec, obj = defn.spec(oid), defn.obj(oid)
        limit = lambda key: int(round(self.st.wv["count"].get_value())) if key == "idx" else 99
        if spec.kind == "slider":
            s = obj.settings
            tracker = self.st.vals[oid] if oid in self.st.vals else (self.st.cv["angle"] if oid == "an" else None)
            widget = self.gh_slider(self.tr(*NAMES[oid]) if oid in NAMES else s["label"], s["value"], s["min"], s["max"], at=at, decimals=0 if s.get("rounding") == "integer" else 2, tracker=tracker, width=SLIDER_W, unit="°" if oid == "an" else "")
            return SimpleNamespace(view=widget, anchor=widget)
        if spec.kind == "panel":
            head = [self.tr(*HEADINGS[oid])] if oid in HEADINGS else []
            lines = lambda: head + (self._panel_lines(source().tree(oid).items(), oid)[:limit(oid)] or [""])
            static = self.gh_panel(lines(), at=at, width=width)
            if not live:
                return SimpleNamespace(view=static, anchor=static)
            return SimpleNamespace(view=self.live(lambda: self.gh_panel(lines(), at=at, width=width), key=lambda: tuple(lines())), anchor=static)
        if spec.kind == "param":
            widget = self.gh_param(spec.outputs[0].type, at=at, label=obj.settings.get("label"))
            return SimpleNamespace(view=widget, anchor=widget)
        if spec.kind == "toggle":
            on = lambda: self.st.wv["wrap"].get_value() > 0.5
            static = self.gh_toggle(on(), at=at, name=label or "Wrap")
            return SimpleNamespace(view=self.live(lambda: self.gh_toggle(on(), at=at, name=label or "Wrap"), key=on), anchor=static)
        full = oid in FULL
        widget = self.gh_component(obj.component, at=at, state=source().state(oid), nickname=not full)
        if not live:
            return SimpleNamespace(view=widget, anchor=widget)
        return SimpleNamespace(view=self.live(lambda: self.gh_component(obj.component, at=at, state=source().state(oid), nickname=not full), key=lambda: source().state(oid)), anchor=widget)

    def _grip(self, node, name, side):
        """🔌 A grip of a widget by parameter name, the only one when the name is unknown."""
        try:
            return node.anchor.grip_out(name) if side == "out" else node.anchor.grip_in(name)
        except KeyError:
            return node.anchor.grip_out(0) if side == "out" else node.anchor.grip_in(0)

    def _rig(self, defn, source, layout, *, live=(), widths=None, skip=()):
        """🕸️ Widgets for the objects of a layout and the wires of the definition between them; wires of ``skip`` pairs are left out."""
        nodes = {oid: self._node(oid, (*at, 0), defn, source, live=oid in live, width=(widths or {}).get(oid)) for oid, at in layout.items()}
        known = {**self._reg, **nodes}
        wires = []
        for wire in defn.wires:
            if wire.src in known and wire.dst in known and (wire.src in nodes or wire.dst in nodes) and (wire.src, wire.dst) not in skip:
                items = source().trees.get((wire.src, wire.out))
                kind = "list" if items is not None and items.count > 1 else "item"
                wires.append(self.gh_wire(self._grip(known[wire.src], wire.out, "out"), self._grip(known[wire.dst], wire.inp, "in"), kind=kind))
                wires[-1].pair = (wire.src, wire.dst)
        return nodes, wires

    def _icon(self, index, at):
        """🖼️ A small icon for a legend term, drawn from the same widgets and colours as the clip."""
        ink = "#2F5D8C"
        x, y = at
        group = _engine.VGroup()
        if index == 0:
            group.add(_engine.RoundedRectangle(corner_radius=0.05, width=0.62, height=0.22, stroke_color=ink, stroke_width=2, fill_color="#F7F7F7", fill_opacity=1).move_to((x, y, 0)),
                      _engine.Rectangle(width=0.07, height=0.2, stroke_width=0, fill_color=ink, fill_opacity=1).move_to((x + 0.08, y, 0)))
        elif index == 1:
            group.add(_engine.Dot((x - 0.28, y - 0.1, 0), radius=0.06, color=ink), _engine.Dot((x + 0.28, y + 0.1, 0), radius=0.06, color=ink), _draft.polyline(_canvas.bezier(np.array([x - 0.28, y - 0.1, 0.0]), np.array([x + 0.28, y + 0.1, 0.0]), 20), ink, 2.4))
        elif index == 2:
            group.add(self.capsule_axon({"key": "A"}, at=(x, y, 0), scale=0.075, base=_canvas.PREVIEW_FILL))
        elif index == 3:
            group.add(_draft.segments([((x - 0.3, y, 0), (x + 0.3, y, 0)), ((x - 0.3, y - 0.12, 0), (x - 0.3, y + 0.12, 0)), ((x + 0.3, y - 0.12, 0), (x + 0.3, y + 0.12, 0))], ink, 2.6), _engine.Dot((x + 0.08, y, 0), radius=0.07, color=ink))
        elif index == 4:
            group.add(self.plane_axes(_nk.Plane((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=(x - 0.05, y - 0.12, 0), scale=0.16, length=1.5, labels=False, square=False))
        elif index == 5:
            group.add(_engine.Square(0.2, stroke_color=ink, stroke_width=2, fill_opacity=0).move_to((x - 0.3, y, 0)), _engine.Square(0.2, stroke_color=ink, stroke_width=2, fill_color=ink, fill_opacity=0.5).move_to((x + 0.3, y, 0)),
                      _engine.Arrow((x - 0.15, y, 0), (x + 0.15, y, 0), buff=0, color=ink, stroke_width=3, max_tip_length_to_length_ratio=0.4))
        elif index == 6:
            for k in range(4):
                group.add(_engine.Rectangle(width=0.5, height=0.1, stroke_color=ink, stroke_width=1.6, fill_color=ink, fill_opacity=0.9 if k == 1 else 0.0).move_to((x, y + 0.2 - 0.135 * k, 0)))
        else:
            group.add(self.mono("0 1 2", (x, y, 0), size=SMALL * 0.85, color=ink))
        return group

    def _legend_row(self, index, en, de, desc_en, desc_de):
        """📖 One legend row: icon, the term in the language of the clip and the Grasshopper meaning."""
        column, row = divmod(index, 4)
        x, y = -6.7 + column * 4.35, 1.75 - row * 1.0
        term = self._say(self.tr(en, de), (x + 0.85, y + 0.14), color=DARK, edge="l", width=3.2)
        meaning = self._say(self.tr(desc_en, desc_de), (x + 0.85, y - 0.2), size=SMALL, color=SUBTLE, edge="l", width=3.2)
        return _engine.VGroup(self._icon(index, (x + 0.35, y)), term, meaning)

    def _params_card(self, rows, at):
        """🧾 The live parameter card: every slider value with its unit, in the colour of its dimension line."""
        width, pitch = 3.9, 0.5
        frame = _engine.RoundedRectangle(corner_radius=0.08, width=width, height=pitch * (len(rows) + 1) + 0.2, stroke_color="#8A8A8A", stroke_width=_draft.W_THIN,
                                         fill_color="#FFFFFF", fill_opacity=0.95).move_to((*at, 0))
        group = _engine.VGroup(frame)
        top = at[1] + frame.height / 2 - 0.1 - pitch / 2
        group.add(self._say(self.tr("Parameters · live", "Parameter · live"), (at[0] - width / 2 + 0.2, top), color=DARK, edge="l"))
        for index, (name, key) in enumerate(rows):
            y = top - pitch * (index + 1)
            group.add(_engine.Dot((at[0] - width / 2 + 0.3, y, 0), radius=0.08, color=PARAM_COLOR[key]), self._say(name, (at[0] - width / 2 + 0.5, y), size=SMALL, color=SUBTLE, edge="l"),
                      self._say(self.quantity(self.st.vals[key].get_value(), "m"), (at[0] + width / 2 - 0.2, y), size=SMALL, color=DARK, edge="r"))
        return group

    def _table(self, at):
        """📋 The type table: one row per capsule type from the model, the parameter columns with units, the row of the live type highlighted."""
        types = _nk.capsule_types()
        columns = [("Type", "Typ", 0.7), ("w", "B", 0.62), ("l", "L", 0.62), ("h", "H", 0.62), ("r", "r", 0.62), ("Ø", "Ø", 0.62), ("u", "u", 0.66), ("α", "α", 0.5)]
        width = sum(c[2] for c in columns) + 0.3
        pitch = 0.46
        height = pitch * (len(types) + 2) + 0.2
        frame = _engine.RoundedRectangle(corner_radius=0.08, width=width, height=height, stroke_color="#8A8A8A", stroke_width=_draft.W_THIN, fill_color="#FFFFFF", fill_opacity=0.95).move_to((*at, 0))
        group = _engine.VGroup(frame)
        top = at[1] + height / 2 - 0.1 - pitch / 2
        group.rows = {}
        x = at[0] - width / 2 + 0.15
        xs = []
        for _, _, w in columns:
            xs.append(x + w)
            x += w
        for index, (en, de, _) in enumerate(columns):
            group.add(self._say(self.tr(en, de), (xs[index] - 0.08, top), size=SMALL, color=SUBTLE if index else DARK, edge="r"))
        units = ["", "m", "m", "m", "m", "m", "m", "°"]
        for index, unit in enumerate(units):
            if unit:
                group.add(self._say(unit, (xs[index] - 0.08, top - pitch * 0.62), size=SMALL, color=SUBTLE, edge="r"))
        for row, t in enumerate(types):
            y = top - pitch * (row + 1.75)
            values = [t.key, t.width, t.length, t.height, t.corner_radius, t.window_diameter, t.window_u, t.rotation]
            cells = _engine.VGroup(_engine.Rectangle(width=width - 0.2, height=pitch - 0.04, stroke_width=0, fill_color=_nkd.type_color(t.key), fill_opacity=0.0).move_to((at[0], y, 0)))
            for index, value in enumerate(values):
                text = value if isinstance(value, str) else (f"{value:.0f}" if index == 7 else self.number(value, 2))
                cells.add(self._say(text, (xs[index] - 0.08, y), size=SMALL, color=DARK, edge="r"))
            cells.add(_engine.Dot((xs[0] - 0.72, y, 0), radius=0.08, color=_nkd.type_color(t.key)))
            group.rows[t.key] = cells
            group.add(cells)
        return group

    def present_capsule(self):
        """🎞️ The whole animation: from one number to oriented capsules on a core."""
        self._d = _gh.canvas("capsule")
        self._cache, self._places, self._wraps, self._reg = {}, {}, {}, {}
        d = self._d
        types = _nk.capsule_types()
        slots = _nk.slots()
        tower_b = _nk.towers()[1]
        value = lambda key: d.obj(key).settings["value"]
        trackers = {key: _engine.ValueTracker(value(key)) for key in IDS + BOLT_IDS}
        self.st = st = SimpleNamespace(vals=trackers, az=_engine.ValueTracker(AZIMUTH), oy=_engine.ValueTracker(0.0),
                                       rv={name: _engine.ValueTracker(0.0) for name in ("grid", "env", "hit", "body", "round", "window", "dw", "dl", "dh", "dd", "du", "bolts", "axes", "plane2",
                                                                                   "lift", "wrong", "front", "dr", "over")},
                                       cv={name: _engine.ValueTracker(0.0) for name in ("a", "b", "cap", "glide", "angle", "arrow", "arc")},
                                       wv={name: _engine.ValueTracker(8.0 if name == "rev" else 0.0) for name in ("planes", "caps", "count", "wrap", "rev")})
        rv, cv, wv, vals = st.rv, st.cv, st.wv, st.vals
        solution = self._solve()
        source = self._solve
        self._cap_l, self._cap_h = types[0].length, types[0].height
        self._mistake = _gh.define({"objects": [{"id": "d", "component": "Number Slider", "settings": {**d.obj("d").settings}}, {"id": "twow", "component": "Panel", "settings": {"lines": ["2"]}},
                                                {"id": "rad", "component": "Division"}, {"id": "xy", "component": "XY Plane"}, {"id": "cir", "component": "Circle"}],
                                    "wires": ["d.Number>rad.A", "twow.Stream>rad.B", "rad.Result>cir.Radius", "xy.Plane>cir.Plane"]})
        centre_b = np.array([tower_b.center[0], tower_b.center[1], 0.0])
        z0 = tower_b.base_elevation
        side = tower_b.core_side
        slot_b = next(s for s in slots if s.tower == 1 and s.storey == 0 and s.facade == 0 and s.item == 0)
        self._plane_a = {"origin": (0.0, 0.0, 0.0)}
        plane_a = {"plane": {"origin": list(self._plane_a["origin"]), "x": [1, 0, 0], "y": [0, 1, 0]}}
        plane_b = {"plane": {"origin": list(slot_b.facade_plane.origin), "x": list(slot_b.facade_plane.x), "y": list(slot_b.facade_plane.y)}}
        axis = {"plane": {"origin": [centre_b[0], centre_b[1], z0], "x": [1, 0, 0], "y": [0, 1, 0]}}
        self._place = _gh.define({"objects": [
            {"id": "apl", "component": "Plane Param", "settings": {"label": self.tr("Base A", "Basis A"), "data": [plane_a]}}, {"id": "tpl", "component": "Plane Param", "settings": {"label": self.tr("Target B", "Ziel B"), "data": [plane_b]}},
            {"id": "cap", "component": "Brep Param", "settings": {"label": self.tr("Capsule", "Kapsel"), "data": [{"capsule": "A"}]}}, {"id": "ax", "component": "Plane Param", "settings": {"label": self.tr("Core axis", "Kernachse"), "data": [axis]}},
            {"id": "an", "component": "Number Slider", "settings": {"label": self.tr("Angle", "Winkel"), "min": -270, "max": 0, "value": 0, "rounding": "integer"}}, {"id": "rad", "component": "Radians"},
            {"id": "rot", "component": "Rotate"}, {"id": "ori", "component": "Orient"}, {"id": "pa", "component": "Panel", "settings": {"lines": [""]}},
            {"id": "pb", "component": "Panel", "settings": {"lines": [""]}}, {"id": "pr", "component": "Panel", "settings": {"lines": [""]}}],
            "wires": ["apl.Plane>pa.Stream", "tpl.Plane>pb.Stream", "tpl.Plane>rot.Geometry", "an.Number>rad.Degrees", "rad.Radians>rot.Angle", "ax.Plane>rot.Plane", "rot.Geometry>pr.Stream",
                      "cap.Brep>ori.Geometry", "apl.Plane>ori.Source", "rot.Geometry>ori.Target"]})
        self._wrap_slots = [next(s for s in slots if s.tower == 1 and s.storey == storey and s.facade == facade and s.item == 0) for storey in (0, 1) for facade in range(4)]
        planes = [{"plane": {"origin": list(s.facade_plane.origin), "x": list(s.facade_plane.x), "y": list(s.facade_plane.y)}} for s in self._wrap_slots]
        self._wrap = _gh.define({"objects": [
            {"id": "types", "component": "Brep Param", "settings": {"label": self.tr("Types", "Typen"), "data": [{"capsule": t.key} for t in types]}},
            {"id": "idx", "component": "Panel", "settings": {"lines": [str(i) for i in range(len(planes))]}}, {"id": "wr", "component": "Boolean Toggle", "settings": {"value": False}},
            {"id": "li", "component": "List Item"}, {"id": "tg", "component": "Plane Param", "settings": {"label": self.tr("Planes", "Ebenen"), "data": planes}}, {"id": "pa", "component": "Plane Param",
             "settings": {"label": self.tr("Base A", "Basis A"), "data": [plane_a]}}, {"id": "ori", "component": "Orient"}, {"id": "out", "component": "Panel", "settings": {"lines": [""]}}],
            "wires": ["types.Brep>li.List", "idx.Stream>li.Index", "wr.Boolean>li.Wrap", "li.Item>ori.Geometry", "pa.Plane>ori.Source", "tg.Plane>ori.Target", "li.Item>out.Stream"]})
        self._arc = (centre_b[0], centre_b[1], z0, math.atan2(slot_b.facade_plane.origin[1] - centre_b[1], slot_b.facade_plane.origin[0] - centre_b[0]),
                     math.hypot(slot_b.facade_plane.origin[0] - centre_b[0], slot_b.facade_plane.origin[1] - centre_b[1]))
        self._core = self.core_axon(tower_b, at=CORE_AT, scale=SCALE_CORE, view=(AZIMUTH, ELEVATION), storeys=4)
        self._core_wrap = self.core_axon(tower_b, at=WRAP_AT, scale=SCALE_WRAP, view=(AZIMUTH, ELEVATION), storeys=4)
        sheet = self.gh_background(at=SHEET, width=SHEET_W, height=SHEET_H)
        viewport = self._viewport()
        stage = self.live(lambda: self._stage(), key=lambda: (tuple(round(t.get_value(), 4) for t in vals.values()), round(st.az.get_value(), 3), round(st.oy.get_value(), 3),
                                                              tuple(round(t.get_value(), 3) for t in rv.values())))
        glide = self.live(lambda: self._glide(), key=lambda: tuple(round(t.get_value(), 3) for t in cv.values()))
        wrapstage = self.live(lambda: self._wrapstage(), key=lambda: tuple(round(t.get_value(), 3) for t in wv.values()))
        subtitle = self.badge(self.tr("Nakagin Capsule Tower · one capsule", "Nakagin Capsule Tower · eine Kapsel"), (-6.9, 3.35), color=MUTED, size=SMALL, filled=False, anchor="l")
        note = self._say(self.tr("Teaching model · simplified, rounded", "Lehrmodell · vereinfacht, gerundet"), (6.85, 3.45), size=SMALL, color=MUTED, edge="r")
        self.add(sheet, viewport, subtitle, note)

        slider_w = self.gh_slider(self.tr(*NAMES["w"]), value("w"), d.obj("w").settings["min"], d.obj("w").settings["max"], at=(-5.2, 0.6, 0), tracker=vals["w"], width=SLIDER_W)
        center = self.gh_component("Center Box", at=(-1.7, 0.6, 0))
        wire_a = self.gh_wire(slider_w.grip_out(), center.grip_in("X"))
        outline = self.gh_group([slider_w, center], self.tr("One definition", "Eine Definition"), color="#9FD0F2")
        tower = self.tower_axon(at=VP + np.array([0.0, 0.45, 0.0]), scale=0.105, view="axon", lit=[(0, 1, 4, 1, 0)])
        lit = tower.capsules[(0, 1, 4, 1, 0)].get_center()
        callout = _engine.VGroup(_draft.polyline([lit, lit + np.array([-0.9, 0.55, 0.0])], DARK, 2.0, 0.9), self._tag(self.tr("one capsule", "eine Kapsel"), lit + np.array([-1.5, 0.6, 0.0])))
        counts = self.gh_panel([self.tr("Capsules ", "Kapseln ") + str(len(slots)), self.tr("Types ", "Typen ") + str(len(types)), self.tr("Definitions ", "Definitionen ") + "1"],
                               at=(-2.6, -1.4, 0), width=2.7)
        self.step("hook", _engine.AnimationGroup(_engine.FadeIn(tower), _engine.FadeIn(callout), _engine.FadeIn(counts), _engine.FadeIn(slider_w), _engine.FadeIn(center), _engine.FadeIn(wire_a),
                                                 _engine.FadeIn(outline)), seconds=4.0)

        rv_set = lambda **kw: _engine.AnimationGroup(*[self._to(rv[k], v, 1.2) for k, v in kw.items()])
        for name in ("grid", "lift"):
            rv[name].set_value(1.0)
        head_p = self._say(self.tr("Parameter", "Parameter"), (-5.2, 1.85), color="#E08A1E")
        head_d = self._say(self.tr("Dependency", "Abhängigkeit"), (-1.7, 1.85), color=DARK)
        head_g = self._tag(self.tr("Geometry", "Geometrie"), (VP[0], VP[1] + VP_H / 2 - 0.65), color="#1E7A3A", size=LABEL)
        wire_b = _draft.polyline([center.grip_out().get_center(), (2.45, 0.6, 0)], "#3F6FB5", 2.6, 0.95, dash=0.14)
        self.add(stage)
        self.step("parametric", _engine.Succession(
            _engine.AnimationGroup(*self._gone(tower), _engine.FadeOut(callout), _engine.FadeOut(counts), _engine.FadeIn(head_p), _engine.FadeIn(head_d), _engine.FadeIn(head_g), _engine.FadeIn(wire_b),
                                   self._to(rv["body"], 1.0, 1.5), self._to(rv["dw"], 1.0, 1.5)),
            self._ramp(vals["w"], value("w"), 3.1, 1.6), self._ramp(vals["w"], 3.1, value("w"), 1.4)), seconds=5.0)
        self.wait(0.2)

        sliders = {key: self._node(key, (*SLIDER_AT[key], 0), d, source) for key in IDS}
        dots = _engine.VGroup(*[_engine.Rectangle(width=0.09, height=0.4, stroke_width=0, fill_color=PARAM_COLOR[key], fill_opacity=1).move_to(sliders[key].view.body.get_left() + np.array([0.05, 0.0, 0.0]))
                                for key in IDS])
        card_rows = [(self.tr(*pair), key) for key, pair in (("w", ("Width", "Breite")), ("l", ("Length", "Länge")), ("h", ("Height", "Höhe")), ("r", ("Corner radius", "Eckenradius")),
                                                             ("d", ("Window diameter", "Fensterdurchmesser")), ("wu", ("Window offset", "Fensterversatz")))]
        params_card = self.live(lambda: self._params_card(card_rows, (-0.95, 0.15)), key=lambda: tuple(round(vals[k].get_value(), 3) for k in IDS))
        self.step("sliders", _engine.AnimationGroup(*self._gone(head_p, head_d, head_g, center, wire_a, wire_b, slider_w, outline), _engine.LaggedStart(*[_engine.FadeIn(n.view) for n in sliders.values()], lag_ratio=0.2),
                                                    _engine.FadeIn(dots), _engine.FadeIn(params_card), rv_set(dl=1.0, dh=1.0, dd=1.0, dr=1.0, du=1.0, round=1.0, window=1.0)), seconds=5.0)
        self._reg.update(sliders)
        fin = lambda items: [_engine.FadeIn(m) for m in items]
        fout = lambda items: [_engine.FadeOut(m) for m in items]
        col = [n.view for n in sliders.values()] + [dots]

        xy_node = self._node("xy", (-2.0, -1.9, 0), d, source)
        origin_panel = self.gh_panel([self.tr("Origin [m]", "Ursprung [m]"), self._show(solution.value("xy").origin)], at=(0.3, -1.9, 0), width=2.9)
        wire_xy = self.gh_wire(xy_node.anchor.grip_out(), origin_panel.grip_in())
        self.step("plane", _engine.AnimationGroup(_engine.FadeOut(params_card), *fin([xy_node.view, origin_panel, wire_xy]), self._to(rv["axes"], 1.0, 1.2),
                                                  rv_set(body=0.0, dw=0.0, dl=0.0, dh=0.0, dd=0.0, dr=0.0, du=0.0)), seconds=5.0)
        rv["lift"].set_value(0.0)
        rv["round"].set_value(0.0)
        rv["window"].set_value(0.0)

        centred = _gh.define({"objects": [{"id": key, "component": "Number Slider", "settings": {**d.obj(key).settings}} for key in ("w", "l", "h")] + [{"id": "xy", "component": "XY Plane"},
                                                                                                                                                  {"id": "box", "component": "Center Box"}],
                              "wires": ["w.Number>box.X", "l.Number>box.Y", "h.Number>box.Z", "xy.Plane>box.Base"]})
        centred_cache = {}

        def centred_solution():
            key = tuple(round(vals[k].get_value(), 4) for k in ("w", "l", "h"))
            if key not in centred_cache:
                centred_cache[key] = _gh.evaluate(centred, dict(zip(("w", "l", "h"), key)))
            return centred_cache[key]

        mode = {"engine": False}
        box_source = lambda: self._solve() if mode["engine"] else centred_solution()
        box_node = self._node("box", (0.8, 1.1, 0), centred, box_source)
        centre_text = lambda: [self.tr("Box centre [m]", "Boxmitte [m]"), self._show(box_source().value("box").center)]
        centre_panel = self.live(lambda: self.gh_panel(centre_text(), at=(0.5, 0.05, 0), width=2.6), key=lambda: tuple(centre_text()))
        centre_anchor = self.gh_panel(centre_text(), at=(0.5, 0.05, 0), width=2.6)
        self._reg.update({"xy": xy_node, "box": box_node})
        box_wires = [self.gh_wire(sliders[k].anchor.grip_out(), self._grip(box_node, n, "in")) for k, n in (("w", "X"), ("l", "Y"), ("h", "Z"))]
        base_wire = self.gh_wire(xy_node.anchor.grip_out(), self._grip(box_node, "Base", "in"))
        out_wire = self.gh_wire(box_node.anchor.grip_out(), centre_anchor.grip_in())
        self.step("box", _engine.AnimationGroup(*fout([origin_panel, wire_xy]), *fin([box_node.view, centre_panel, base_wire, out_wire, *box_wires]),
                                                self._to(rv["body"], 1.0, 1.5), rv_set(dw=1.0, dl=1.0, dh=1.0)), seconds=6.0)


        def pulse(wires, steps):
            found = []
            for pair, color in steps:
                for wire in wires:
                    if getattr(wire, "pair", None) == pair:
                        found.append(_engine.Indicate(wire, color=color, scale_factor=1.0, run_time=0.8))
            return _engine.LaggedStart(*found, lag_ratio=0.6)

        flow = "#3F6FB5"
        mode["engine"] = True
        lifted, lift_wires = self._rig(d, source, {"hl": (-1.6, 1.55), "hh": (-1.6, 0.55), "two": (-2.95, 1.05), "bp": (-0.6, -0.95), "bo": (0.8, -1.9)})
        self.step("lift", _engine.AnimationGroup(*fout([base_wire]), *fin([n.view for n in lifted.values()]), *fin(lift_wires), self._to(rv["lift"], 1.0, 2.0), self._to(rv["plane2"], 1.0, 1.2),
                                                 _engine.Succession(_engine.Wait(1.5), pulse(lift_wires, [(("l", "hl"), PARAM_COLOR["l"]), (("h", "hh"), PARAM_COLOR["h"]), (("hl", "bp"), PARAM_COLOR["l"]),
                                                                                                       (("hh", "bp"), PARAM_COLOR["h"]), (("bp", "bo"), flow), (("bo", "box"), flow)]))),
                  seconds=8.0)

        length_range = d.obj("l").settings
        drag = length_range["max"] - 0.3
        self.step("live", _engine.Succession(self._ramp(vals["l"], value("l"), drag, 3.5), _engine.Wait(0.5)), seconds=6.0)

        helpers = [n.view for n in lifted.values()] + lift_wires + [xy_node.view, centre_panel, out_wire]
        mode["engine"] = True
        fil, fil_wires = self._rig(d, source, {"fil": (0.1, 1.1), "fvol": (1.1, -0.2), "fvp": (0.9, -1.2)}, live=("fvp",), widths={"fvp": 1.6})
        self.step("rounded", _engine.AnimationGroup(self._ramp(vals["l"], drag, value("l"), 1.2), *fout(helpers), box_node.view.animate.move_to((-2.2, 1.0, 0)), *fin([n.view for n in fil.values()]), *fin(fil_wires),
                                                    self._to(rv["round"], 1.0, 2.0), self._to(rv["plane2"], 0.0, 1.0), self._to(rv["dr"], 1.0, 1.5), rv_set(dl=0.0, dh=0.0)), seconds=7.0)

        old = [box_node.view, *box_wires, *[n.view for n in fil.values()], *fil_wires]
        self._reg = dict(sliders)
        mist_cache = {}

        def mist_source():
            key = round(vals["d"].get_value(), 4)
            if key not in mist_cache:
                mist_cache[key] = _gh.evaluate(self._mistake, {"d": key})
            return mist_cache[key]

        wrong, wrong_wires = self._rig(self._mistake, mist_source, {"xy": (-3.0, 1.5), "rad": (-3.0, -0.9), "twow": (-3.0, -1.9), "cir": (1.1, 0.4)})
        bubble = self.gh_bubble(self.tr("Plane is XY: the floor", "Ebene ist XY: der Boden"), level="warning", target=wrong["cir"].view, width=3.0)
        self.step("wrongplane", _engine.AnimationGroup(*fout(old), *fin([n.view for n in wrong.values()]), *fin(wrong_wires), _engine.FadeIn(bubble), self._to(rv["wrong"], 1.0, 1.5),
                                                       self._to(rv["dr"], 0.0, 0.8), self._to(rv["dd"], 1.0, 1.0)), seconds=6.5)

        self._reg.update(wrong)
        front, front_wires = self._rig(d, source, {"hh": (-1.6, 0.55), "two": (-2.95, 1.05), "wpt": (-0.5, -0.7), "xz": (-1.9, -1.9), "wpl": (0.9, -1.5)})
        bridge = [self.gh_wire(wrong["rad"].anchor.grip_out(), self._grip(wrong["cir"], "Radius", "in")), self.gh_wire(front["wpl"].anchor.grip_out(), self._grip(wrong["cir"], "Plane", "in"))]
        before = [wrong["xy"].view, *wrong_wires, bubble]
        keep_wires = [w for w in wrong_wires if w not in before]
        wu_hi = d.obj("wu").settings["max"]
        offset = types[3].window_u
        self.step("frontplane", _engine.AnimationGroup(*fout(before), *fin([n.view for n in front.values()]), *fin(front_wires), *fin(bridge), self._to(rv["wrong"], 0.0, 1.0),
                                                       self._to(rv["front"], 1.0, 1.5), self._to(rv["du"], 1.0, 1.5), self._to(rv["dd"], 0.0, 0.8),
                                                       _engine.Succession(_engine.Wait(1.6), self._ramp(vals["wu"], 0.0, offset, 1.8), self._ramp(vals["wu"], offset, 0.0, 1.6)),
                                                       _engine.Succession(_engine.Wait(1.2), pulse(front_wires, [(("l", "wpt"), PARAM_COLOR["l"]), (("wu", "wpt"), PARAM_COLOR["wu"]), (("hh", "wpt"), PARAM_COLOR["h"]),
                                                                                                                  (("wpt", "wpl"), flow), (("xz", "wpl"), flow)]))), seconds=8.0)

        cur = [n.view for n in wrong.values() if n is not wrong["xy"]] + [n.view for n in front.values()] + front_wires + bridge
        self._reg = dict(sliders)
        cut, cut_wires = self._rig(d, source, {"wpl": (-2.7, 1.8), "rad": (-2.7, 0.5), "twow": (-2.7, -0.7), "wall": (-2.2, -1.8), "fil": (-0.7, 2.0), "cyl": (-0.7, 0.1), "cut": (0.7, 1.0)},
                                   widths={"wall": 2.3})
        self.step("cut", _engine.AnimationGroup(*fout(cur), *fin([n.view for n in cut.values()]), *fin(cut_wires), self._to(rv["window"], 1.0, 1.5), self._to(rv["front"], 0.0, 1.0),
                                                self._to(rv["dd"], 1.0, 1.0), self._to(rv["du"], 0.0, 1.0)), seconds=6.5)

        cur = [n.view for n in cut.values()] + cut_wires
        bolt_sliders = {key: self._node(key, (-5.05, 1.9 - 0.8 * index, 0), d, source) for index, key in enumerate(BOLT_IDS)}
        self._reg = dict(bolt_sliders)
        bolts, bolt_wires = self._rig(d, source, {"nbu": (-2.85, 1.9), "mx": (-1.45, 1.3), "mz": (-1.45, -0.3), "bolt": (0.4, 0.6)})
        point_panel = self.gh_panel([self.tr("Bolt points [m]", "Schraubenpunkte [m]")] + [f"{i}  " + self._show(p_) for i, p_ in enumerate(solution.tree("bolt").items())], at=(0.15, -1.6, 0), width=3.2)
        point_wire = self.gh_wire(bolts["bolt"].anchor.grip_out(), point_panel.grip_in(), kind="list")
        self.step("bolts", _engine.AnimationGroup(*fout(cur), *fout(col), *fin([n.view for n in bolt_sliders.values()]), *fin([n.view for n in bolts.values()]), *fin(bolt_wires),
                                                  *fin([point_panel, point_wire]), self._to(st.az, -AZIMUTH, 2.5), self._to(rv["bolts"], 1.0, 1.5), rv_set(dw=0.0, dl=0.0, dh=0.0, dd=0.0)),
                  seconds=7.0)

        cur = [n.view for n in bolt_sliders.values()] + [n.view for n in bolts.values()] + bolt_wires + [point_panel, point_wire]
        self._reg = dict(sliders)
        spans = {key: d.obj(key).settings for key in ("w", "l", "h")}
        card = self._card([(self.tr(*en_de), key, f"{self.quantity(spans[key]['min'], 'm')} – {self.quantity(spans[key]['max'], 'm')}") for key, en_de in
                           (("w", ("Width", "Breite")), ("l", ("Length", "Länge")), ("h", ("Height", "Höhe")))], (-1.15, 0.6))
        self.step("domains", _engine.AnimationGroup(*fout(cur), *fin(col), _engine.FadeIn(card), self._to(st.az, AZIMUTH, 2.0), self._to(rv["env"], 1.0, 1.5), self._to(rv["bolts"], 0.0, 1.0),
                                                    rv_set(dw=1.0, dl=1.0, dh=1.0)), seconds=6.0)

        width_hi = d.obj("w").settings["max"]
        limit_badge = self.badge(self.tr("limit reached", "Grenze erreicht") + " · " + self.quantity(width_hi, "m"), (-1.15, -1.6), color=WARN, size=SMALL, filled=False)
        slider_box = sliders["w"].view.body
        red_box = _engine.Rectangle(width=slider_box.width + 0.1, height=slider_box.height + 0.1, stroke_color=RED, stroke_width=4, fill_opacity=0).move_to(slider_box.get_center())
        request = self._tag(self.tr("requested ", "angefragt ") + self.quantity(width_hi + 0.5, "m") + " → " + self.quantity(width_hi, "m"), (-1.15, -2.15, 0.0), color="#B3261E")
        self.step("limit", _engine.Succession(self._ramp(vals["w"], value("w"), width_hi, 2.4), _engine.AnimationGroup(self._to(rv["hit"], 1.0, 0.4), _engine.FadeIn(limit_badge)),
                                              _engine.AnimationGroup(self._to(rv["over"], 1.0, 0.5), _engine.FadeIn(red_box), _engine.FadeIn(request)), _engine.Wait(1.0)), seconds=7.5)

        table = self._table((-0.85, 1.25))
        minis = _engine.VGroup()
        for index, t in enumerate(types):
            x = -2.85 + 1.3 * index
            minis.add(self.capsule_axon({"key": t.key}, at=(x, -1.5, 0), scale=0.2, base=_nkd.type_color(t.key)), self._say(self.tr("Type ", "Typ ") + t.key, (x, -2.2), size=SMALL, color=_nkd.type_color(t.key)))
        order = [types[0], types[1], types[3], types[2]]
        ty = _engine.ValueTracker(0.0)

        def highlight():
            t = order[int(round(ty.get_value()))]
            rect = table.rows[t.key][0]
            return _engine.VGroup(_engine.Rectangle(width=rect.width, height=rect.height, stroke_color=_nkd.type_color(t.key), stroke_width=3, fill_color=_nkd.type_color(t.key),
                                                    fill_opacity=0.18).move_to(rect.get_center()))

        marker = self.live(highlight, key=lambda: int(round(ty.get_value())))
        typetag = self.live(lambda: _engine.VGroup(self._tag(self.tr("Same definition · type ", "Dieselbe Definition · Typ ") + order[int(round(ty.get_value()))].key,
                                                              (VP[0], VP[1] + VP_H / 2 - 0.75, 0.0), color=DARK, size=LABEL)), key=lambda: int(round(ty.get_value())))
        self.step("types", _engine.AnimationGroup(*fout([card, limit_badge, red_box, request]), self._to(rv["over"], 0.0, 0.5), _engine.FadeIn(table), _engine.FadeIn(minis), _engine.FadeIn(marker), self._ramp(vals["w"], width_hi, value("w"), 1.5), self._to(rv["env"], 0.0, 1.0),
                                                  self._to(rv["hit"], 0.0, 0.5)), seconds=5.0)

        steps = [_engine.AnimationGroup(self._ramp(ty, float(i) - 1.0 if i else 0.0, float(i), 0.2), *[self._to(vals[k], v, 2.0) for k, v in (
            ("w", t.width), ("l", t.length), ("h", t.height), ("r", t.corner_radius), ("d", t.window_diameter), ("wu", t.window_u))]) for i, t in enumerate(order)]
        self.step("four", _engine.AnimationGroup(_engine.FadeIn(typetag), self._to(rv["dd"], 1.0, 1.0), self._to(rv["du"], 1.0, 1.0), _engine.Succession(*steps)), seconds=9.0)

        for key in IDS:
            vals[key].set_value(value(key))
        cur = col + [table, minis, marker, typetag]
        self._reg = {}
        apl, apl_wires = self._rig(self._place, self._solve_place, {"apl": (-5.3, 1.95), "pa": (-2.8, 1.95), "tpl": (-5.3, 0.35), "pb": (-2.8, 0.4)}, widths={"pa": 3.2, "pb": 3.2})
        cv["a"].set_value(1.0)
        cv["b"].set_value(1.0)
        self.step("planes", _engine.AnimationGroup(*fout(cur), _engine.FadeOut(stage), _engine.FadeIn(glide), *fin([n.view for n in apl.values()]), *fin(apl_wires)), seconds=6.0)

        self._reg = dict(apl)
        capn, cap_wires = self._rig(self._place, self._solve_place, {"cap": (-5.3, -0.85), "ori": (0.45, 0.9)})
        direct = self.gh_wire(apl["tpl"].anchor.grip_out(), self._grip(capn["ori"], "Target", "in"))
        goal = self._solve_place().items("rot")[0].origin
        move_vector = _gh.Vector(goal.x - self._plane_a["origin"][0], goal.y - self._plane_a["origin"][1], goal.z - self._plane_a["origin"][2])
        move_panel = self.gh_panel([self.tr("Translation [m]", "Verschiebung [m]"), self._show(move_vector)], at=(0.3, -1.0, 0), width=3.0)
        move_wire = self.gh_wire(self._grip(capn["ori"], "X", "out"), move_panel.grip_in())
        self.step("orient", _engine.AnimationGroup(*fin([n.view for n in capn.values()]), *fin(cap_wires), _engine.FadeIn(direct),
                                                   _engine.FadeIn(move_panel), _engine.FadeIn(move_wire),
                                                   _engine.Succession(self._to(cv["cap"], 1.0, 1.0), self._to(cv["arrow"], 1.0, 0.8), self._to(cv["glide"], 0.55, 2.4))), seconds=6.0)

        self._reg = {**apl, **capn}
        spin, spin_wires = self._rig(self._place, self._solve_place, {"an": (-5.3, -1.85), "rad": (-3.3, -1.85), "ax": (-3.3, -0.2), "rot": (-1.5, -0.4), "pr": (0.15, -1.75)},
                                     live=("pr",), widths={"pr": 3.3})
        self.step("rotate", _engine.AnimationGroup(*fout([direct, move_panel, move_wire, apl["pa"].view, apl["pb"].view, *apl_wires]), *fin([n.view for n in spin.values()]), *fin(spin_wires), self._to(cv["arrow"], 0.0, 0.6), self._to(cv["arc"], 1.0, 1.0),
                                                   _engine.Succession(self._to(cv["glide"], 1.0, 1.2), *[self._to(cv["angle"], a, 1.8) for a in (-90.0, -180.0, -270.0)])), seconds=9.0)

        cur = [n.view for n in {**capn, **spin}.values()] + [apl["apl"].view, apl["tpl"].view] + cap_wires + spin_wires
        self._reg = {}
        wv["count"].set_value(4.0)
        chain, chain_wires = self._rig(self._wrap, self._solve_wrap, {"types": (-5.3, 1.8), "idx": (-5.3, -0.1), "li": (-3.0, 1.0), "ori": (-0.6, 0.7), "pa": (-3.0, 2.2), "tg": (-0.6, 2.2),
                                                                     "out": (1.0, -1.1), "wr": (-3.0, -1.9)}, live=("idx", "out", "li"), widths={"out": 1.4})
        self.step("listitem", _engine.AnimationGroup(*fout(cur), _engine.FadeOut(glide), _engine.FadeIn(wrapstage), *fin([n.view for n in chain.values()]), *fin(chain_wires),
                                                     self._to(wv["planes"], 1.0, 1.2), _engine.Succession(_engine.Wait(1.0), self._to(wv["caps"], 1.0, 2.0))), seconds=7.0)

        wv["rev"].set_value(4.0)
        notice = self.live(lambda: self._notice(chain["li"]), key=lambda: (self._solve_wrap().state("li"), round(wv["count"].get_value())))
        self.step("outside", _engine.AnimationGroup(self._to(wv["count"], 8.0, 1.5), _engine.FadeIn(notice)), seconds=6.0)
        self.step("wrap", _engine.AnimationGroup(self._to(wv["wrap"], 1.0, 0.4), _engine.Succession(_engine.Wait(0.6), self._to(wv["rev"], 8.0, 3.2))), seconds=6.0)

        cur = [n.view for n in chain.values()] + chain_wires + [notice]
        legend = [self._legend_row(i, *row) for i, row in enumerate(LEGEND)]
        core = self._core_wrap
        slot = self._wrap_slots[0]
        ring = lambda point, radius=0.4: _engine.Circle(radius=radius, stroke_color="#FFB000", stroke_width=6, fill_opacity=0).move_to(point)
        strip_box = _engine.Rectangle(width=4.5, height=0.95, stroke_color="#FFB000", stroke_width=5, fill_opacity=0).move_to((VP[0], VP[1] - 2.05, 0))
        targets = [None, None, ring(core.at(*(np.array(slot.facade_plane.origin) + np.array([0.0, 2.0, 1.25]))), 0.6), None, ring(core.at(*slot.facade_plane.origin), 0.3),
                   ring(core.at(*self._wrap_slots[1].facade_plane.origin), 0.3), strip_box, ring((VP[0] - 1.9 + 3.8 / 7 * 4, VP[1] - 2.0, 0), 0.35)]
        glow = [_engine.Succession(_engine.FadeIn(legend[i], run_time=0.5), _engine.AnimationGroup(_engine.Indicate(legend[i], color="#FFB000", scale_factor=1.06, run_time=0.9),
                                                                                               *([_engine.Succession(_engine.FadeIn(targets[i], run_time=0.25), _engine.Wait(0.4), _engine.FadeOut(targets[i], run_time=0.25))] if targets[i] is not None else [])))
                for i in range(len(legend))]
        self.step("recap", _engine.AnimationGroup(*fout(cur), _engine.Succession(*glow)), seconds=11.0)
        self.wait(0.5)

    def _notice(self, node):
        """⚠️ The List Item message from the engine solution as a bubble over the component while it warns; empty when it is fine."""
        solution = self._solve_wrap()
        messages = solution.msgs("li")
        if not messages:
            return _engine.VGroup()
        return _engine.VGroup(self.gh_bubble(messages[0][self.language()], level="warning", target=node.anchor, side="below", width=3.4))

    def _card(self, rows, at):
        """🧾 The constraint card: one row per parameter with its colour, name and domain from the model."""
        width, pitch = 4.9, 0.55
        frame = _engine.RoundedRectangle(corner_radius=0.08, width=width, height=pitch * (len(rows) + 1) + 0.25, stroke_color="#8A8A8A", stroke_width=_draft.W_THIN,
                                         fill_color="#FFFFFF", fill_opacity=0.95).move_to((*at, 0))
        group = _engine.VGroup(frame)
        top = at[1] + frame.height / 2 - 0.1 - pitch / 2
        group.add(self._say(self.tr("Slider domain = design limit (teaching assumption)", "Slider-Domain = Entwurfsgrenze (Lehrannahme)"), (at[0] - width / 2 + 0.2, top), color=DARK, edge="l", width=width - 0.4))
        for index, (name, key, text) in enumerate(rows):
            y = top - pitch * (index + 1)
            group.add(_engine.Dot((at[0] - width / 2 + 0.3, y, 0), radius=0.09, color=PARAM_COLOR[key]), self._say(name, (at[0] - width / 2 + 0.55, y), size=SMALL, color=SUBTLE, edge="l"),
                      self._say(text, (at[0] + width / 2 - 0.2, y), size=SMALL, color=DARK, edge="r"))
        return group
