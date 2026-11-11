"""🎬 One Nakagin capsule built parametrically in Grasshopper: sliders, a base plane, Center Box, Fillet Edge, a window cut, four bolts, the type variants, Orient onto a core plane and List Item per plane."""

import dataclasses
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
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
_sprites = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")
_keyed = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️keyed")
_scene_interop = importlib.import_module("🎓️teaching.🧰️interop.🎬️scene.🐍️")

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
NAMES = {"w": ("Casing width", "Gehäusebreite"), "l": ("Casing length", "Gehäuselänge"), "h": ("Casing height", "Gehäusehöhe"), "r": ("Radius", "Radius"), "d": ("Window Ø", "Fenster Ø"), "wu": ("Offset", "Versatz"),
         "bu": ("Bolt x", "Bolzen x"), "bvl": ("Bolt low", "Bolzen unten"), "bvh": ("Bolt high", "Bolzen oben"), "an": ("Angle", "Winkel"), "back":("Rear datum","Rückseitendatum"), "recess":("Glazing recess","Glasrücksprung"), "wv":("Window height","Fensterhöhe"), "wall":("Reveal depth","Laibungstiefe"), "idx":("Per-slot index","Index je Platz"), "types":("Proposed programs","Vorgeschlagene Programme")}
SLIDER_X = -5.0
SLIDER_AT = {key: (SLIDER_X, 2.35 - 0.8 * index) for index, key in enumerate(IDS)}
PARAMS_AT = (-0.85, 0.15)
SLIDER_W = 2.9
FULL = ("box", "fil", "cut", "cir", "cyl", "ori", "li", "rot")
PARAM_COLOR = {"w": _nkd.AXIS_X, "l": _nkd.AXIS_Y, "h": _nkd.AXIS_Z, "r": "#9B6FD6", "d": "#E08A1E", "wu": "#14A3A8"}
DIM_NAME = {"w": "width", "l": "length", "h": "height"}
RED = "#D8362B"
UNITS = {"fvp": " m³", "wall": " m"}
HEADINGS = {"fvp": ("Envelope volume", "Hüllvolumen"), "wall": ("Steel reveal depth", "Stahl-Laibungstiefe"), "pa": ("Base plane A [m]", "Grundebene A [m]"),
            "pb": ("Target plane B [m]", "Zielebene B [m]"), "pr": ("Plane B rotated [m]", "Ebene B gedreht [m]"),
            "out": ("Result", "Ergebnis")}
LEGEND = [("Parameter", "Parameter", "Number Slider", "Number Slider"), ("Dependency", "Abhängigkeit", "component + wire", "Komponente + Verbindung"),
          ("Geometry", "Geometrie", "Brep, Circle, Point", "Brep, Circle, Point"), ("Domain", "Domain", "slider limits", "Slider-Grenzen"),
          ("Plane", "Ebene", "origin + X Y Z", "Ursprung + X Y Z"), ("Orient", "Orient", "base to target plane", "Grund- auf Zielebene"),
          ("List Item", "List Item", "type by index", "Typ nach Index"), ("Index", "Index", "0, 1, 2 … wraps", "0, 1, 2 … läuft herum")]


def _reference_controls(t):
    """📏 Match reference sliders to actual fabrication bounds, separate from nominal studies."""
    c=_nk.capsule_casing(t)
    base=_nk.capsule_type(t.get('key','A')) if isinstance(t,dict) else t
    return {'w':c.width,'l':c.length,'h':c.height,'r':t.get('corner_radius',base.corner_radius) if isinstance(t,dict) else t.corner_radius,'d':t.get('window_diameter',base.window_diameter) if isinstance(t,dict) else t.window_diameter,'wu':t.get('window_u',base.window_u) if isinstance(t,dict) else t.window_u}


def _wrap_sources(slots, programs):
    """🌳 Retain each historical family while selecting proposed program variants per actual slot."""
    result={name:{'tree':{}} for name in ('types','idx','tg')}
    for i,slot in enumerate(slots):
        path='{0;'+str(i)+'}'
        result['types']['tree'][path]=[{'reference_capsule':{'type':t.key,'family':slot.layout,'interior':'study'}} for t in programs]
        result['idx']['tree'][path]=[i]
        result['tg']['tree'][path]=[{'plane':{'origin':list(slot.plane.origin),'x':list(slot.plane.x),'y':list(slot.plane.y)}}]
    return result


def _fade(group, alpha):
    """🌫️ Scale every stroke and fill opacity of a freshly built group by an alpha so partial reveals survive a redraw."""
    if alpha >= 0.999:
        return group
    for mob in group.get_family():
        mob.set_fill(opacity=float(mob.get_fill_opacity()) * alpha)
        mob.set_stroke(opacity=float(mob.get_stroke_opacity()) * alpha)
    return group


def _preview_mode(rounding, window):
    """🧊 Select the authored box, rounded shell or window preview."""
    return "window" if window > 0.5 else "rounded" if rounding > 0 else "box"


def _preview_key(p, rv, view, origin, scale=SCALE, color=None):
    """🗝️ Key only values that change visible preview geometry."""
    mode = _preview_mode(rv["round"], rv["window"])
    key = (mode, p["width"], p["length"], p["height"], p["back"], rv["lift"], tuple(view), tuple(origin), scale, color)
    if mode != "box":
        key += (p["corner_radius"] * rv["round"],)
    if mode == "window":
        key += (p["window_diameter"], p["window_u"], p["window_v"], p["glazing_y"])
    return key


class _Seq:
    """🎼 Plays and callables that one clause runs in order: a part is a callable or an ``(animations, seconds)`` pair."""

    def __init__(self, *parts):
        self.parts = parts


class Beat2_Capsule(_scene_interop.RefreshMoving, _visuals.ClipScene):
    """🧩 Build one parametric Nakagin capsule and place it on a core with explicit English and German narration."""

    TITLE = {"en": "One Capsule, Parametric", "de": "Eine Kapsel, parametrisch"}
    NARRATION = [
        ["hook", "The Nakagin tower hangs its capsules on two cores, yet a single Grasshopper definition builds every one of them.",
         "Am Nakagin-Turm hängen die Kapseln an zwei Kernen, doch eine einzige Grasshopper-Definition baut jede davon."],
        ["parametric", "Parametric means: parameters feed dependencies, dependencies build geometry. Change one value and everything follows.",
         "Parametrisch heißt: Parameter speisen Abhängigkeiten, Abhängigkeiten bauen Geometrie. Ändert sich ein Wert, folgt alles."],
        ["sliders", "Six Number Sliders drive the 2.6 by 4.2 metre reference casing: height, corner radius, window diameter and offset.",
         "Sechs Number Sliders steuern das Referenzgehäuse 2,6 mal 4,2 Meter: Höhe, Eckenradius, Fensterdurchmesser, Versatz."],
        ["plane", "Everything starts on a plane: origin plus X, Y and Z axes in Rhino's red, green and blue. This is the capsule's base plane.",
         "Alles beginnt auf einer Ebene: Ursprung plus X-, Y- und Z-Achse in Rhinos Rot, Grün und Blau. Das ist die Grundebene der Kapsel."],
        ["box", "Center Box takes the plane and the sliders for width, length and height. It centres itself on the plane, so half of it sinks below the floor.",
         "Center Box nimmt die Ebene und die Slider für Breite, Länge und Höhe. Die Box zentriert sich auf der Ebene, die Hälfte versinkt im Boden."],
        ["lift", "Plane Origin uses the physical rear datum minus 0.1 metres, half the casing length and half its height. The box now stands on the base plane.",
         "Plane Origin nutzt das Rückseitendatum minus 0,1 Meter sowie halbe Länge und Höhe. Jetzt steht die Box auf der Grundebene."],
        ["live", "Drag the length slider: the dimension line, the box and every component behind it follow in the same moment.",
         "Ziehen Sie den Längen-Slider: Maßlinie, Box und jede Komponente dahinter folgen im selben Moment."],
        ["rounded", "Fillet Edge rounds the edges with the corner radius slider: the soft shell of the capsule appears.",
         "Fillet Edge rundet die Kanten mit dem Eckenradius-Slider: Die weiche Hülle der Kapsel entsteht."],
        ["wrongplane", "A typical mistake: a Circle on the base plane lies on the floor, not on the front face. A wire does not know where you mean.",
         "Ein typischer Fehler: Ein Circle auf der Grundebene liegt am Boden, nicht an der Vorderseite. Eine Verbindung weiß nicht, wo Sie ihn meinen."],
        ["frontplane", "The fix: front datum equals rear datum plus casing length. Glazing sits at its recess; the cutter starts on the steel face.",
         "Die Lösung: Vorderdatum ist Rückdatum plus Gehäuselänge. Das Glas liegt im Rücksprung, der Zylinder startet an der Stahlfläche."],
        ["cut", "Solid Difference subtracts a cylinder of that circle from the shell: the round window is cut into the front face.",
         "Solid Difference zieht einen Zylinder dieses Kreises von der Hülle ab: Das runde Fenster entsteht an der Vorderseite."],
        ["bolts", "Four points on the back face mark the high-tension bolts that fix the capsule to the core.",
         "Vier Punkte auf der Rückseite markieren die hochfesten Schrauben, mit denen die Kapsel am Kern hängt."],
        ["domains", "Each slider domain is a design constraint; here the limits are assumptions, for example a transport envelope.",
         "Jede Slider-Domain ist eine Entwurfsgrenze; hier sind die Grenzen Annahmen, zum Beispiel eine Transporthülle."],
        ["limit", "Push the width past its domain: the slider clamps at its upper limit and the envelope turns red. The domain is a design rule.",
         "Schieben Sie die Breite über die Domain hinaus: Der Slider hält am oberen Ende an, die Hülle wird rot. Die Domain ist eine Entwurfsregel."],
        ["types", "A to D are proposed room-program studies, not historical capsule families. They share the nominal capsule envelope.",
         "A bis D sind vorgeschlagene Raumprogramm-Studien und keine historischen Kapselfamilien. Sie teilen die nominale Kapselhülle."],
        ["four", "All four studies keep the nominal centred 1.30 metre porthole. Earlier slider edits explored dimensions; these are not surveyed types.",
         "Alle vier Studien behalten das nominale mittige Bullauge von 1,30 Metern. Die vorherigen Slider-Änderungen erkundeten Maße; dies sind keine vermessenen Typen."],
        ["planes", "On the core the capsule needs a target plane on the wall; the base plane must land on it.",
         "Am Kern braucht die Kapsel eine Zielebene an der Wand; die Grundebene muss dort landen."],
        ["orient", "Orient moves the capsule in one step from its base plane A to the target plane B.",
         "Orient bewegt die Kapsel in einem Schritt von ihrer Grundebene A auf die Zielebene B."],
        ["rotate", "Rotate the target plane about the core axis and the capsule turns with it, because it follows plane B.",
         "Dreht man die Zielebene um die Kernachse, dreht die Kapsel mit, denn sie folgt der Ebene B."],
        ["listitem", "Each slot keeps its historical family. List Item picks one of four proposed variants: index 0 gives A, 1 B, 2 C, 3 D.",
         "Jeder Platz behält seine Familie. List Item wählt eine von vier Varianten: Index 0 ergibt A, 1 B, 2 C, 3 D."],
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

    def play(self, *animations, **kwargs):
        """🎼 Run a queued clause sequence part by part, everything else as a normal play."""
        if len(animations) == 1 and isinstance(animations[0], _Seq):
            for part in animations[0].parts:
                if callable(part):
                    part()
                else:
                    self.play(*(part[0]() if callable(part[0]) else part[0]), run_time=part[1])
            return None
        return super().play(*animations, **kwargs)

    def _preview_live(self, builder, key):
        """🖼️ Keep one stable camera image while preview geometry changes."""
        return _keyed.create(builder, key, self.camera)

    def _preview(self, p, rv, view, v, dims=(), *, scale=SCALE, color=None):
        """🧊 Draw and reuse parametric geometry separately from its annotations."""
        w, length, height = p["width"], p["length"], p["height"]
        mode = _preview_mode(rv["round"], rv["window"])
        key = _preview_key(p, rv, view, v.point, scale, color)
        cache = getattr(self, "_preview_cache", None)
        if cache is None:
            self._preview_cache = cache = {}
        if key not in cache:
            centre = (0.0, (p["back"] + length / 2) * rv["lift"], height / 2 * rv["lift"])
            at = v(*centre)
            if mode == "box":
                base = self.analytical_box_axon(w, length, height, at=at, scale=scale, view=view, dims=False, base=color or _canvas.PREVIEW_FILL)
                base.view = _nkd.make_view(view, at, scale, (0.0, p["back"] + length / 2, height / 2))
            else:
                projected = _nkd.make_view(view, at, scale, (0.0, p["back"] + length / 2, height / 2))
                base = _nkd.NakaginDrawing3D(projected)
                mesh = _nkd.capsule_mesh(w, length, height, max(0.001, p["corner_radius"] * rv["round"]), 4)
                vertices = mesh.V + np.array([0.0, p["back"], 0.0])
                base.preview_profile = mesh.V[mesh.faces[0]][:,(0,2)]
                towards = _nkd._basis(projected)[1]
                faces = []
                for face, normal, name in zip(mesh.faces, mesh.N, mesh.names):
                    points = vertices[face]
                    if float(normal @ towards) > 1e-9:
                        faces.append((float(points.mean(axis=0) @ towards), points, name, normal))
                angles = np.linspace(0, 2 * math.pi, 40, endpoint=False)
                for _, points, name, normal in sorted(faces, key=lambda item: item[0]):
                    color = _nkd.shade_of(color or _canvas.PREVIEW_FILL, _nkd.tone_of(normal))
                    outline = [projected(*point) for point in points]
                    if mode == "window" and name == "front":
                        ring = [(p["window_u"] + p["window_diameter"] / 2 * math.cos(a), p["window_v"] + p["window_diameter"] / 2 * math.sin(a)) for a in angles]
                        glass = [projected(x,p["glazing_y"],z) for x,z in ring]
                        hole = [projected(x,p["front"],z) for x,z in ring]
                        base.add(_draft.polygon(glass, "#5B8D9E", 0.85, "#344E59", _draft.W_EDGE))
                        base.add(_draft.region([outline,hole], color, 1.0, "#56666D", _draft.W_THIN))
                        base.preview_aperture = np.asarray(ring)
                    else:
                        base.add(_draft.polygon(outline, color, 1.0, "#56666D" if name in ("front", "back") else None, _draft.W_THIN))
            base.preview_mode = mode
            cache[key] = base
            if len(cache) > 24:
                cache.pop(next(iter(cache)))
        drawing = cache[key].copy()
        drawing.preview_mode = mode
        drawing.dims = {}
        spans = {"width": ((-w/2,p["front"],0),(w/2,p["front"],0),w),
                 "length": ((w/2,p["back"],0),(w/2,p["front"],0),length),
                 "height": ((-w/2,p["front"],0),(-w/2,p["front"],height),height)}
        for name in dims:
            centre = drawing.at(0,p["back"] + length/2,height/2)
            a,b,value = spans[name]
            dimension = self._away(drawing.at(*a),drawing.at(*b),self.quantity(value,"m",2),centre,0.4)
            dimension.measurement_value, dimension.measurement_unit = value, "m"
            dimension.world_endpoints = np.asarray([a,b])
            drawing.dims[name] = dimension
            drawing.add(dimension)
        return drawing

    def _core_preview(self, core, centre, yaw, capsule):
        """🧭 Project the physical capsule envelope at its real core pose."""
        casing = _nk.capsule_casing(capsule)
        aperture = _nk.capsule_window(capsule, "Slash")
        p = {"width":casing.width,"length":casing.length,"height":casing.height,"back":casing.back,"front":casing.front,
             "corner_radius":capsule.corner_radius,"window_diameter":2*aperture.radius,"window_u":aperture.center[0],
             "window_v":aperture.center[2],"glazing_y":aperture.center[1]}
        view = (AZIMUTH-yaw,ELEVATION)
        v = _nkd.make_view(view,core.at(*centre),SCALE_CORE,(0,0,0))
        return self._preview(p,{"round":1.0,"window":1.0,"lift":0.0},view,v,scale=SCALE_CORE)

    def _static_sprite(self, type_key, yaw, layout, scale, at, alpha):
        """🖼️ Reuse an actual detailed static capsule independently of overlay changes."""
        color = _nkd.type_color(type_key)
        key = (type_key,yaw,layout,scale,color,self.camera.pixel_width,self.camera.pixel_height)
        cache = getattr(self,"_static_capsules",None)
        if cache is None:
            self._static_capsules = cache = {}
        if key not in cache:
            vector = self.capsule_axon(_nk.capsule_type(type_key),at=(0,0,0),scale=scale,view=(AZIMUTH-yaw,ELEVATION),base=color,layout=layout,interior="study")
            cache[key] = _sprites.capture(vector,self.camera)
        sprite = cache[key].copy()
        sprite.shift(np.asarray(at,float))
        sprite.set_opacity(alpha)
        return sprite

    def _core_static_sprite(self, core, plane_origin, yaw, type_key, alpha, layout):
        """📍 Place one cached detailed capsule on its authored core plane."""
        t = _nk.capsule_type(type_key)
        casing = _nk.capsule_casing(t)
        r = math.radians(yaw)
        centre = np.asarray(plane_origin,float) + np.array([-math.sin(r),math.cos(r),0.0]) * (casing.front+casing.back)/2 + np.array([0.0,0.0,t.height/2])
        sprite = self._static_sprite(type_key,yaw,layout,SCALE_WRAP,core.at(*centre),alpha)
        normal = _nk.capsule_window(t,layout).normal
        sprite.facing = core.view.facing((math.cos(r)*normal[0]-math.sin(r)*normal[1],math.sin(r)*normal[0]+math.cos(r)*normal[1],normal[2]))
        return sprite

    def _probe(self, dims=()):
        """🔬 The capsule drawing of the live stage at the current trackers, for morph sources and targets that must sit exactly on it."""
        st = self.st
        rv = {name: tracker.get_value() for name, tracker in st.rv.items()}
        p = self._read(self._solve())
        view = (st.az.get_value(), ELEVATION)
        v = _nkd.make_view(view, ORIGIN + np.array([0.0, st.oy.get_value(), 0.0]), SCALE, (0, 0, 0))
        w, l, h = p["width"], p["length"], p["height"]
        drawing = self._preview(p, rv, view, v, dims)
        for key, name in DIM_NAME.items():
            if name in drawing.dims:
                for part in drawing.dims[name].get_family():
                    if not isinstance(part, _engine.RoundedRectangle):
                        part.set_color(PARAM_COLOR[key])
        return SimpleNamespace(v=v, p=p, drawing=drawing, view=view)

    def _capsule_at(self, glide):
        """🧊 The capsule drawing of the core viewport at a glide fraction, taken without changing the live trackers."""
        cv = self.st.cv
        keep = (cv["cap"].get_value(), cv["glide"].get_value())
        cv["cap"].set_value(1.0)
        cv["glide"].set_value(glide)
        group = self._glide(vector=True)
        cv["cap"].set_value(keep[0])
        cv["glide"].set_value(keep[1])
        return (list(group[2].submobjects) or list(group[3].submobjects))[0]

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
            self._wraps[(wrap, count)] = _gh.evaluate(self._wrap, {"wr": wrap, **{name:{"tree":dict(list(data["tree"].items())[:count])} for name,data in self._wrap_data.items()}})
        return self._wraps[(wrap, count)]

    @staticmethod
    def _read(solution):
        """📐 Capsule parameters read back from the evaluated Brep of the canvas: box domains, fillet radius, window circle."""
        brep = solution.tree("cut").items()[0]
        box, window = brep.root_box(), solution.value("cir", "Circle")
        return {"width": box.x.length, "length": box.y.length, "height": box.z.length, "corner_radius": brep.fillet_radius(),
                "window_diameter": 2 * window.radius, "window_u": window.plane.origin.x, "window_v": window.plane.origin.z,
                "back": box.plane.origin.y - box.y.length / 2, "front": box.plane.origin.y + box.y.length / 2, "glazing_y": window.plane.origin.y}

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
            dims = tuple(name for key, name in DIM_NAME.items() if rv["d" + key] > 0.02)
            drawing = self._preview(p, rv, view, v, dims)
            for key, name in DIM_NAME.items():
                if name in drawing.dims:
                    for part in drawing.dims[name].get_family():
                        if not isinstance(part, _engine.RoundedRectangle):
                            part.set_color(PARAM_COLOR[key])
                    _fade(drawing.dims[name], rv["d" + key])
            group.add(_fade(drawing, rv["body"]))
            if rv["dd"] > 0.02 and rv["window"] > 0.5:
                u, vv, d = p["window_u"], p["window_v"], p["window_diameter"]
                a, b = drawing.at(u - d / 2, p["glazing_y"], vv), drawing.at(u + d / 2, p["glazing_y"], vv)
                across = (b - a) / max(np.linalg.norm(b - a), 1e-9)
                normal = np.array([-across[1], across[0], 0.0]) * 0.07
                diameter = _engine.VGroup(_draft.segments([(a, b), (a + normal, a - normal), (b + normal, b - normal)], PARAM_COLOR["d"], 3.0),
                                          self._tag("Ø " + self.quantity(d, "m"), (a + b) / 2 + np.array([0.0, -0.4, 0.0]), color="#9A5A00"))
                group.add(_fade(diameter, rv["dd"]))
            if rv["dr"] > 0.02:
                corner = drawing.at(w / 2, p["front"], h)
                group.add(_fade(self._tag("r " + self.quantity(p["corner_radius"], "m"), corner + np.array([0.45, 0.3, 0.0]), color="#6B3FB0"), rv["dr"]))
            if rv["bolts"] > 0.02:
                dots = _engine.VGroup()
                for point in solution.tree("bolt").items():
                    centre = drawing.at(point.x, point.y, point.z)
                    dots.add(_engine.Circle(radius=0.075, stroke_color=WARN, stroke_width=3, fill_color="#FFFFFF", fill_opacity=0.9).move_to(centre), _engine.Dot(centre, radius=0.025, color=WARN))
                wall = [drawing.at(sx * (w / 2 + 0.45), -0.4, z) for sx, z in ((-1, -0.25), (1, -0.25), (1, h + 0.25), (-1, h + 0.25))]
                plate = _engine.VGroup(_draft.polygon(wall, "#9AA5AB", 0.45, "#5E6A72", _draft.W_THIN))
                for i, point in enumerate(solution.tree("bolt").items()):
                    plate.add(_draft.segments([(drawing.at(point.x, -0.4, point.z), drawing.at(point.x, p["back"], point.z))], WARN, 3.0),
                              self._tag(str(i), drawing.at(point.x, -0.4, point.z) + np.array([0.0, 0.3, 0.0]), color="#9A5A00"))
                plate.add(self._tag(self.tr("Core wall", "Kernwand"), drawing.at(0.0, -0.4, h + 0.25) + np.array([0.0, 0.3, 0.0]), color="#2B3A44"))
                group.add(_fade(plate, rv["bolts"]), _fade(dots, rv["bolts"]))
            if rv["du"] > 0.02:
                a, b = drawing.at(0, p["glazing_y"], p["window_v"]), drawing.at(p["window_u"], p["glazing_y"], p["window_v"])
                offset = _engine.VGroup(_draft.segments([(a, b)], PARAM_COLOR["wu"], 3.0), _engine.Dot(b, radius=0.04, color=PARAM_COLOR["wu"]))
                if abs(p["window_u"]) > 1e-3:
                    offset.add(self._tag("u " + self.quantity(p["window_u"], "m"), (a + b) / 2 + np.array([0.0, 0.32, 0.0]), color="#0D7377"))
                group.add(_fade(offset, rv["du"]))
        if rv["axes"] > 0.02:
            axes = self.plane_axes(_nk.Plane((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=origin, scale=SCALE, view=view, length=1.7, labels=rv["body"] < 0.5)
            group.add(_fade(axes, rv["axes"]))
        if rv["plane2"] > 0.02:
            centre = (0, (p["back"] + l / 2) * lift, h / 2 * lift)
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

    def _on_core(self, core, plane_origin, yaw, type_key, alpha=1.0, color=None, *, layout="Slash"):
        """🧱 A capsule of a type standing on a core plane: drawn at the world position of its centre, turned by the plane yaw through the view azimuth."""
        t = _nk.capsule_type(type_key)
        o = np.array(plane_origin, float)
        r = math.radians(yaw)
        centre = o + np.array([-math.sin(r), math.cos(r), 0.0]) * (_nk.capsule_casing(t).front+_nk.capsule_casing(t).back) / 2 + np.array([0.0, 0.0, t.height / 2])
        drawing = self.capsule_axon(_nk.capsule_type(type_key), at=core.at(*centre), scale=SCALE_WRAP, view=(AZIMUTH - yaw, ELEVATION), base=color or _nkd.type_color(type_key), layout=layout, interior="study")
        normal=_nk.capsule_window(t,layout).normal
        drawing.facing = core.view.facing((math.cos(r)*normal[0]-math.sin(r)*normal[1],math.sin(r)*normal[0]+math.cos(r)*normal[1],normal[2]))
        return _fade(drawing, alpha)

    def _glide(self, *, vector=False):
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
            start = np.array(self._plane_a["origin"], float) + self._cap_centre
            centre = start + (np.array(placed.center.t) - start) * cv["glide"]
            yaw = yaw_b * cv["glide"]
            drawing = self.capsule_axon(_nk.capsule_type("A"), at=self._core.at(*centre), scale=SCALE_CORE, view=(AZIMUTH - yaw, ELEVATION), base=_canvas.PREVIEW_FILL) if vector else self._core_preview(self._core, centre, yaw, _nk.capsule_type("A"))
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
        """🔁 Eight authored occupied target planes on the core with the capsule type List Item picks for each index, from the engine solution."""
        wv = {name: tracker.get_value() for name, tracker in self.st.wv.items()}
        solution = self._solve_wrap()
        picks = solution.tree("li").items()
        view = (AZIMUTH, ELEVATION)
        core = self._core_wrap
        behind, front, marks, ghost = _engine.Group(), _engine.Group(), _engine.Group(), _engine.Group()
        shown = int(round(wv["count"]))
        for index, slot in enumerate(self._wrap_slots):
            plane = slot.plane
            yaw = math.degrees(math.atan2(plane.x[1], plane.x[0]))
            pick = picks[index] if index < len(picks) else None
            if wv["planes"] > 0.02 and index < shown:
                marks.add(_fade(self.plane_axes(plane, at=core.at(*plane.origin), scale=SCALE_WRAP, view=view, length=1.2, labels=False, square=False), wv["planes"]))
                marks.add(_fade(self._tag(str(index), core.at(*plane.origin) + np.array([0.0, 0.3, 0.0]), color="#2B3A44", size=SMALL), wv["planes"]))
            if pick is not None and wv["caps"] > 0.02 and index < shown and index < wv["rev"]:
                capsule = self._core_static_sprite(core, plane.origin, yaw, pick.tag["type"], wv["caps"], pick.tag["family"])
                (front if capsule.facing >= 0 else ghost).add(capsule)
            elif pick is None and wv["caps"] > 0.5 and index < shown:
                marks.add(self._tag("!", core.at(*plane.origin) + np.array([0.3, 0.45, 0.0]), color="#B3261E", size=LABEL))
        strip = _engine.Group()
        for index in range(len(self._wrap_slots)):
            x, y = VP[0] - 1.9 + 3.8 / 7 * index, VP[1] - 2.0
            pick = picks[index] if index < len(picks) else None
            if wv["caps"] > 0.02 and index < shown and (index < wv["rev"] or pick is None):
                if pick is not None:
                    strip.add(self._static_sprite(pick.tag["type"],0.0,pick.tag["family"],0.13,(x,y,0),wv["caps"]))
                else:
                    strip.add(self._tag("!", (x, y, 0), color="#B3261E", size=LABEL))
                strip.add(self._tag(str(index), (x, y - 0.42, 0), color="#2B3A44"))
        return _engine.Group(behind, core, front, ghost, marks, strip)

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
            widget = self.gh_param(spec.outputs[0].type, at=at, label=self.tr(*NAMES[oid]) if oid in NAMES else obj.settings.get("label"))
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
            group.add(self.capsule_axon(_nk.capsule_type("A"), at=(x, y, 0), scale=0.075, base=_canvas.PREVIEW_FILL))
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
            values = [t.key, *(_reference_controls(t)[k] for k in IDS), t.rotation]
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
        casing = _nk.capsule_casing(types[0])
        self._cap_centre = np.array([(casing.left+casing.right)/2,(casing.back+casing.front)/2,(casing.bottom+casing.top)/2])
        self._mistake = _gh.define({"objects": [{"id": "d", "component": "Number Slider", "settings": {**d.obj("d").settings}}, {"id": "twow", "component": "Panel", "settings": {"lines": ["2"]}},
                                                {"id": "rad", "component": "Division"}, {"id": "xy", "component": "XY Plane"}, {"id": "cir", "component": "Circle"}],
                                    "wires": ["d.Number>rad.A", "twow.Stream>rad.B", "rad.Result>cir.Radius", "xy.Plane>cir.Plane"]})
        centre_b = np.array([tower_b.center[0], tower_b.center[1], 0.0])
        z0 = tower_b.base_elevation
        side = tower_b.core_side
        slot_b = next(s for s in slots if s.tower == 1 and s.layout == "Slash")
        self._plane_a = {"origin": (0.0, 0.0, 0.0)}
        plane_a = {"plane": {"origin": list(self._plane_a["origin"]), "x": [1, 0, 0], "y": [0, 1, 0]}}
        plane_b = {"plane": {"origin": list(slot_b.plane.origin), "x": list(slot_b.plane.x), "y": list(slot_b.plane.y)}}
        axis = {"plane": {"origin": [centre_b[0], centre_b[1], z0], "x": [1, 0, 0], "y": [0, 1, 0]}}
        self._place = _gh.define({"objects": [
            {"id": "apl", "component": "Plane Param", "settings": {"label": self.tr("Base A", "Basis A"), "data": [plane_a]}}, {"id": "tpl", "component": "Plane Param", "settings": {"label": self.tr("Target B", "Ziel B"), "data": [plane_b]}},
            {"id": "cap", "component": "Brep Param", "settings": {"label": self.tr("Capsule", "Kapsel"), "data": [{"reference_capsule": {"type": "A", "family": slot_b.layout, "interior": "reference"}}]}}, {"id": "ax", "component": "Plane Param", "settings": {"label": self.tr("Core axis", "Kernachse"), "data": [axis]}},
            {"id": "an", "component": "Number Slider", "settings": {"label": self.tr("Angle", "Winkel"), "min": -270, "max": 0, "value": 0, "rounding": "integer"}}, {"id": "rad", "component": "Radians"},
            {"id": "rot", "component": "Rotate"}, {"id": "ori", "component": "Orient"}, {"id": "pa", "component": "Panel", "settings": {"lines": [""]}},
            {"id": "pb", "component": "Panel", "settings": {"lines": [""]}}, {"id": "pr", "component": "Panel", "settings": {"lines": [""]}}],
            "wires": ["apl.Plane>pa.Stream", "tpl.Plane>pb.Stream", "tpl.Plane>rot.Geometry", "an.Number>rad.Degrees", "rad.Radians>rot.Angle", "ax.Plane>rot.Plane", "rot.Geometry>pr.Stream",
                      "cap.Brep>ori.Geometry", "apl.Plane>ori.Source", "rot.Geometry>ori.Target"]})
        self._wrap_slots = [s for s in slots if s.tower == 1][:8]
        self._wrap_data = _wrap_sources(self._wrap_slots, types)
        self._wrap = _gh.define({"objects": [
            {"id": "types", "component": "Brep Param", "settings": {"label": self.tr("Types", "Typen"), "data": self._wrap_data["types"]}},
            {"id": "idx", "component": "Integer Param", "settings": {"data": self._wrap_data["idx"]}}, {"id": "wr", "component": "Boolean Toggle", "settings": {"value": False}},
            {"id": "li", "component": "List Item"}, {"id": "tg", "component": "Plane Param", "settings": {"label": self.tr("Planes", "Ebenen"), "data": self._wrap_data["tg"]}}, {"id": "pa", "component": "Plane Param",
             "settings": {"label": self.tr("Base A", "Basis A"), "data": [plane_a]}}, {"id": "ori", "component": "Orient"}, {"id": "out", "component": "Panel", "settings": {"lines": [""]}}],
            "wires": ["types.Brep>li.List", "idx.Integer>li.Index", "wr.Boolean>li.Wrap", "li.Item>ori.Geometry", "pa.Plane>ori.Source", "tg.Plane>ori.Target", "li.Item>out.Stream"]})
        self._arc = (centre_b[0], centre_b[1], z0, math.atan2(slot_b.plane.origin[1] - centre_b[1], slot_b.plane.origin[0] - centre_b[0]),
                     math.hypot(slot_b.plane.origin[0] - centre_b[0], slot_b.plane.origin[1] - centre_b[1]))
        study_height = max(s.plane.origin[2] + types[0].height for s in self._wrap_slots) + .5
        study_core = dataclasses.replace(tower_b, storeys=4, height=study_height)
        self._core = self.core_axon(study_core, at=CORE_AT, scale=SCALE_CORE, view=(AZIMUTH, ELEVATION))
        self._core_wrap = self.core_axon(study_core, at=WRAP_AT, scale=SCALE_WRAP, view=(AZIMUTH, ELEVATION))
        sheet = self.gh_background(at=SHEET, width=SHEET_W, height=SHEET_H)
        viewport = self._viewport()
        stage = self._preview_live(lambda: self._stage(), key=lambda: (tuple(round(t.get_value(), 4) for t in vals.values()), round(st.az.get_value(), 3), round(st.oy.get_value(), 3),
                                                              tuple(round(t.get_value(), 3) for t in rv.values())))
        glide = self._preview_live(lambda: self._glide(), key=lambda: tuple(round(t.get_value(), 3) for t in cv.values()))
        wrapstage = self._preview_live(lambda: self._wrapstage(), key=lambda: tuple(round(t.get_value(), 3) for t in wv.values()))
        subtitle = self.badge(self.tr("Nakagin Capsule Tower · one capsule", "Nakagin Capsule Tower · eine Kapsel"), (-6.9, 3.35), color=MUTED, size=SMALL, filled=False, anchor="l")
        note = self._say(self.tr("Reference casing · proposed programs", "Referenzgehäuse · vorgeschlagene Programme"), (6.85, 3.45), size=SMALL, color=MUTED, edge="r")
        self.add(sheet, viewport, subtitle, note)

        slider_w = self.gh_slider(self.tr(*NAMES["w"]), value("w"), d.obj("w").settings["min"], d.obj("w").settings["max"], at=(SLIDER_X, 0.6, 0), tracker=vals["w"], width=SLIDER_W)
        center = self.gh_component("Center Box", at=(-1.7, 0.6, 0))
        wire_a = self.gh_wire(slider_w.grip_out(), center.grip_in("X"))
        outline = self.gh_group([slider_w, center], self.tr("One definition", "Eine Definition"), color="#9FD0F2")
        show_key = (*slot_b.path, slot_b.item)
        tower = self.tower_axon(**_canvas.building_fit(VP_W, VP_H, centre=VP, top=_canvas.LABEL_ROW, bottom=1.0), view="axon", lit=[show_key])
        tower_display = _snapshot.capture(tower, self.camera)
        lit = tower.capsules[show_key].get_center()
        callout = _engine.VGroup(_draft.polyline([lit, lit + np.array([-0.9, 0.55, 0.0])], DARK, 2.0, 0.9), self._tag(self.tr("one capsule", "eine Kapsel"), lit + np.array([-1.5, 0.6, 0.0])))
        counts = self.gh_panel([self.tr("Capsules ", "Kapseln ") + str(len(slots)), self.tr("Types ", "Typen ") + str(len(types)), self.tr("Definitions ", "Definitionen ") + "1"],
                               at=(VP[0] - 1.05, VP[1] - 2.2, 0), width=2.6)
        crowd = [_sprites.capture(capsule, self.camera) for capsule in tower.capsules.values()]
        chosen = tower.capsules[show_key].copy()
        count_copy = counts.copy()
        lit_back = [_sprites.capture(capsule.copy().set_color("#FFB000"), self.camera) for capsule in tower.capsules.values()]
        gather = _engine.AnimationGroup(*[_engine.FadeOut(capsule, target_position=counts, scale=.15) for capsule in crowd],
                                        _engine.FadeIn(counts, scale=.85), lag_ratio=.01, run_time=2.0)
        split = _engine.AnimationGroup(*[_engine.FadeIn(capsule, target_position=counts, scale=.15) for capsule in lit_back],
                                       _engine.FadeOut(count_copy), lag_ratio=.01, run_time=1.8)
        self.step("hook", _Seq(((_engine.FadeIn(tower_display), _engine.FadeIn(callout), _engine.FadeIn(slider_w), _engine.FadeIn(outline)), 1.0), lambda: self.add(*crowd),
                               ((gather,), 2.0), lambda: self.add(chosen),
                               ((_morph.morph(chosen, center, run_time=1.4), _engine.FadeIn(wire_a)), 1.4), lambda: self.add(count_copy),
                               ((split,), 1.8), lambda: self.remove(*lit_back)), seconds=6.2)

        rv_set = lambda **kw: _engine.AnimationGroup(*[self._to(rv[k], v, 1.2) for k, v in kw.items()])
        for name in ("grid", "lift"):
            rv[name].set_value(1.0)
        head_p = self._say(self.tr("Parameter", "Parameter"), (-5.2, 1.85), color="#E08A1E")
        head_d = self._say(self.tr("Dependency", "Abhängigkeit"), (-1.7, 1.85), color=DARK)
        head_g = self._tag(self.tr("Geometry", "Geometrie"), (VP[0], VP[1] + VP_H / 2 - 0.65), color="#1E7A3A", size=LABEL)
        wire_b = _draft.polyline([center.grip_out().get_center(), (2.45, 0.6, 0)], "#3F6FB5", 2.6, 0.95, dash=0.14)
        self.add(stage)
        knob = _morph.token(self.quantity(value("w"), "m"), kind="number", color=PARAM_COLOR["w"])
        hold = {}

        def box_ready():
            hold["solid"] = self._probe().drawing
            hold["chip"] = center.copy()
            self.add(hold["chip"])

        def box_born():
            rv["body"].set_value(1.0)
            self.remove(hold["solid"])

        def width_ready():
            hold["line"] = self._probe(("width",)).drawing.dims["width"]
            knob.move_to(slider_w.grip_out().get_center())
            self.add(knob)

        def width_done():
            rv["dw"].set_value(1.0)
            self.remove(hold["line"])

        self.step("parametric", _Seq(((_engine.AnimationGroup(*self._gone(tower_display), _engine.FadeOut(callout), _engine.FadeOut(counts), _engine.FadeIn(head_p), _engine.FadeIn(head_d),
                                                              _engine.FadeIn(head_g), _engine.FadeIn(wire_b)),), 1.2), box_ready,
                                     (lambda: (_morph.morph(hold["chip"], hold["solid"], run_time=1.6),), 1.6), box_born, width_ready,
                                     ((_morph.travel(knob, [slider_w.grip_out().get_center(), center.grip_in("X").get_center()], run_time=1.0),), 1.0), (lambda: (_morph.morph(knob, hold["line"], run_time=1.0),), 1.0), width_done,
                                     ((_engine.Succession(self._ramp(vals["w"], value("w"), 3.1, 1.6), self._ramp(vals["w"], 3.1, value("w"), 1.4)),), 3.0)), seconds=6.5)
        self.wait(0.2)

        sliders = {key: self._node(key, (*SLIDER_AT[key], 0), d, source) for key in IDS}
        dots = _engine.VGroup(*[_engine.Rectangle(width=0.09, height=0.4, stroke_width=0, fill_color=PARAM_COLOR[key], fill_opacity=1).move_to(sliders[key].view.body.get_left() + np.array([0.05, 0.0, 0.0]))
                                for key in IDS])
        card_rows = [(self.tr(*pair), key) for key, pair in (("w", ("Width", "Breite")), ("l", ("Length", "Länge")), ("h", ("Height", "Höhe")), ("r", ("Corner radius", "Eckenradius")),
                                                             ("d", ("Window diameter", "Fensterdurchmesser")), ("wu", ("Window offset", "Fensterversatz")))]
        params_card = self.live(lambda: self._params_card(card_rows, PARAMS_AT), key=lambda: tuple(round(vals[k].get_value(), 3) for k in IDS))
        def dims_ready():
            dims = self._probe(("width", "length", "height")).drawing.dims
            sheet_card = self._params_card(card_rows, PARAMS_AT)
            hold["src"] = [dims[name] for name in ("width", "length", "height")]
            hold["dst"] = [sheet_card[2 + 3 * index + 2].copy() for index in range(3)]
            self.add(*hold["src"])

        self.step("sliders", _Seq(((_engine.AnimationGroup(*self._gone(head_p, head_d, head_g, center, wire_a, wire_b, slider_w, outline), _engine.LaggedStart(*[_engine.FadeIn(n.view) for n in sliders.values()], lag_ratio=0.2),
                                                           _engine.FadeIn(dots), rv_set(dl=1.0, dh=1.0, dd=1.0, dr=1.0, du=1.0, round=1.0, window=1.0)),), 3.2), dims_ready,
                                  (lambda: (_morph.scatter(hold["src"], hold["dst"], lag=0.3, run_time=1.2, limit=2.0), _engine.FadeIn(params_card)), 2.0),
                                  lambda: self.remove(*hold["dst"])), seconds=5.2)
        self._reg.update(sliders)
        fin = lambda items: [_engine.FadeIn(m) for m in items]
        fout = lambda items: [_engine.FadeOut(m) for m in items]
        col = [n.view for n in sliders.values()] + [dots]

        xy_node = self._node("xy", (-2.0, -1.9, 0), d, source)
        origin_panel = self.gh_panel([self.tr("Origin [m]", "Ursprung [m]"), self._show(solution.value("xy").origin)], at=(0.3, -1.9, 0), width=2.9)
        wire_xy = self.gh_wire(xy_node.anchor.grip_out(), origin_panel.grip_in())
        base_axes = self.plane_axes(_nk.Plane((0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)), at=ORIGIN, scale=SCALE, view=(AZIMUTH, ELEVATION), length=1.7, labels=True)
        self.step("plane", _Seq(((_engine.AnimationGroup(_engine.FadeOut(params_card), *fin([xy_node.view]), self._to(rv["axes"], 1.0, 1.2),
                                                         rv_set(body=0.0, dw=0.0, dl=0.0, dh=0.0, dd=0.0, dr=0.0, du=0.0)),), 2.0), lambda: self.add(base_axes),
                                ((_morph.morph(base_axes, origin_panel, run_time=1.6), _engine.FadeIn(wire_xy)), 1.6)), seconds=5.0)
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
        centre_dot = _engine.Dot(ORIGIN, radius=0.07, color="#3F6FB5")

        def centre_done():
            self.remove(centre_anchor)
            self.add(centre_panel)

        self.step("box", _Seq(((_engine.AnimationGroup(*fout([origin_panel, wire_xy]), *fin([box_node.view, base_wire, *box_wires]), self._to(rv["body"], 1.0, 1.5), rv_set(dw=1.0, dl=1.0, dh=1.0)),), 3.0),
                              lambda: self.add(centre_dot), ((_morph.morph(centre_dot, centre_anchor, run_time=1.4), _engine.FadeIn(out_wire)), 1.4), centre_done), seconds=6.0)


        def pulse(wires, steps):
            found = []
            for pair, color in steps:
                for wire in wires:
                    if getattr(wire, "pair", None) == pair:
                        found.append(_engine.Indicate(wire, color=color, scale_factor=1.0, run_time=0.8))
            return _engine.LaggedStart(*found, lag_ratio=0.6)

        flow = "#3F6FB5"
        mode["engine"] = True
        lifted, lift_wires = self._rig(d, source, {"hl": (-1.6, 1.55), "hh": (-1.6, 0.55), "two": (-2.95, 1.05), "back": (-2.8,-0.3), "bc": (-0.6,-0.25), "bp": (-0.6, -1.2), "bo": (0.8, -2.1)})
        half = self._probe(("length", "height"))
        half_tokens = [_morph.token(self.quantity(half.p[name] / 2, "m"), kind="number", color=PARAM_COLOR[key]).move_to(lifted[node].view.get_center() + np.array([0.0, 0.55, 0.0]))
                       for name, key, node in (("length", "l", "hl"), ("height", "h", "hh"))]
        half_lines = [half.drawing.dims["length"], half.drawing.dims["height"]]

        def lift_start():
            self.add(*half_lines)

        def lift_done():
            self.remove(*half_tokens)

        self.step("lift", _Seq(((_engine.AnimationGroup(*fout([base_wire]), *fin([n.view for n in lifted.values()]), *fin(lift_wires)),), 1.2), lift_start,
                               ((_morph.morph(half_lines[0], half_tokens[0], path_arc=0.4, run_time=1.4), _morph.morph(half_lines[1], half_tokens[1], path_arc=-0.4, run_time=1.4)), 1.4),
                               ((_engine.AnimationGroup(self._to(rv["lift"], 1.0, 2.0), self._to(rv["plane2"], 1.0, 1.2),
                                                        _engine.Succession(_engine.Wait(1.0), pulse(lift_wires, [(("l", "hl"), PARAM_COLOR["l"]), (("h", "hh"), PARAM_COLOR["h"]), (("hl", "bc"), PARAM_COLOR["l"]), (("back", "bc"), flow), (("bc", "bp"), flow),
                                                                                                              (("hh", "bp"), PARAM_COLOR["h"]), (("bp", "bo"), flow), (("bo", "box"), flow)]))),), 4.4), lift_done),
                  seconds=8.0)

        length_range = d.obj("l").settings
        drag = length_range["max"] - 0.3
        self.step("live", _engine.Succession(self._ramp(vals["l"], value("l"), drag, 3.5), _engine.Wait(0.5)), seconds=6.0)

        helpers = [n.view for n in lifted.values()] + lift_wires + [xy_node.view, centre_panel, out_wire]
        mode["engine"] = True
        fil, fil_wires = self._rig(d, source, {"fil": (0.1, 1.1), "fvol": (1.1, -0.2), "fvp": (0.9, -1.2)}, live=("fvp",), widths={"fvp": 1.6})
        radius_token = _morph.token(self.quantity(self._read(solution)["corner_radius"], "m"), kind="number", color=PARAM_COLOR["r"])

        def radius_ready():
            around = self._probe()
            corner = around.drawing.at(around.p["width"] / 2, around.p["front"], around.p["height"])
            hold["tag"] = self._tag("r " + self.quantity(around.p["corner_radius"], "m"), corner + np.array([0.45, 0.3, 0.0]), color="#6B3FB0")
            radius_token.move_to(sliders["r"].anchor.grip_out().get_center())
            self.add(radius_token)

        def radius_done():
            rv["dr"].set_value(1.0)
            self.remove(hold["tag"])

        def shell_ready():
            hold["box"] = self._probe().drawing
            rv["round"].set_value(1.0)
            hold["shell"] = self._probe().drawing
            rv["round"].set_value(0.0)
            rv["body"].set_value(0.0)
            self.add(hold["box"])

        def shell_done():
            rv["round"].set_value(1.0)
            rv["body"].set_value(1.0)
            self.remove(hold["shell"])

        helper_nodes = [n.view for n in lifted.values()] + [xy_node.view, centre_panel]
        helper_wires = lift_wires + [out_wire]
        fil_grow = [_engine.GrowFromPoint(n.view, fil["fil"].view.get_center()) for key, n in fil.items() if key != "fil"]
        self.step("rounded", _Seq(((lambda: (_morph.gather(helper_nodes, fil["fil"].view, run_time=1.2, lag=0.1, limit=2.2),
                                              _engine.Succession(_engine.AnimationGroup(*fout(helper_wires), box_node.view.animate.move_to((-2.2, 1.0, 0)), *fil_grow, *fin(fil_wires),
                                                                                      self._to(rv["plane2"], 0.0, 1.0), rv_set(dl=0.0, dh=0.0), run_time=1.2),
                                                                 self._ramp(vals["l"], drag, value("l"), 1.2))), 2.4)), shell_ready,
                                  (lambda: (_morph.morph(hold["box"], hold["shell"], run_time=1.6),), 1.6), shell_done, radius_ready,
                                  ((_morph.travel(radius_token, [sliders["r"].anchor.grip_out().get_center(), self._grip(fil["fil"], "Radius", "in").get_center()], run_time=1.2),), 1.2),
                                  (lambda: (_morph.morph(radius_token, hold["tag"], path_arc=0.3, run_time=1.4),), 1.4), radius_done), seconds=7.0)

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
        front, front_wires = self._rig(d, source, {"back":(-2.8,1.7),"fr":(-1.5,1.65),"recess":(-2.8,.45),"gl":(-1.4,.4),"wv":(-2.8,-.75),"wpt":(-.2,-.45),"xz":(-1.7,-1.9),"wpl":(.9,-1.6)})
        bridge = [self.gh_wire(wrong["rad"].anchor.grip_out(), self._grip(wrong["cir"], "Radius", "in")), self.gh_wire(front["wpl"].anchor.grip_out(), self._grip(wrong["cir"], "Plane", "in"))]
        before = [wrong["xy"].view, *wrong_wires, bubble]
        keep_wires = [w for w in wrong_wires if w not in before]
        wu_hi = d.obj("wu").settings["max"]
        offset = min(.25, (types[0].width - types[0].window_diameter) / 2 * .8)
        floor = self._probe()
        circle = self._wrong_circle()
        floor_ring = [floor.v(*(np.array(circle.plane.origin.t) + circle.radius * (math.cos(a) * np.array(circle.plane.x.t) + math.sin(a) * np.array(circle.plane.y.t)))) for a in np.linspace(0, 2 * math.pi, 60)]
        red_ring = _draft.polyline(floor_ring, RED, 3.4, 1.0, closed=True)
        plane_token = _morph.token("", kind="plane", color=PARAM_COLOR["d"]).move_to(front["wpl"].view.get_center())
        u0, v0, d0 = floor.p["window_u"], floor.p["window_v"], floor.p["window_diameter"]
        front_ring = _draft.polyline([floor.drawing.at(u0 + d0 / 2 * math.cos(a), floor.p["glazing_y"], v0 + d0 / 2 * math.sin(a)) for a in np.linspace(0, 2 * math.pi, 48)], PARAM_COLOR["d"], 3.4, 1.0, closed=True)

        def ring_done():
            rv["front"].set_value(1.0)
            self.remove(front_ring)

        self.step("frontplane", _Seq(lambda: self.add(red_ring),
                                     ((_engine.AnimationGroup(*fout(before), *fin([n.view for n in front.values()]), *fin(front_wires), *fin(bridge), self._to(rv["wrong"], 0.0, 1.0), self._to(rv["dd"], 0.0, 0.8)),
                                       _morph.morph(red_ring, plane_token, path_arc=0.6, run_time=1.6)), 1.6),
                                     ((_morph.morph(plane_token, front_ring, run_time=1.4),), 1.4), ring_done,
                                     ((_engine.AnimationGroup(self._to(rv["du"], 1.0, 1.0), _engine.Succession(_engine.Wait(0.6), self._ramp(vals["wu"], 0.0, offset, 1.8), self._ramp(vals["wu"], offset, 0.0, 1.6)),
                                                              pulse(front_wires, [(("l", "fr"), PARAM_COLOR["l"]), (("back", "fr"),flow), (("fr", "gl"),flow), (("recess", "gl"),flow), (("gl", "wpt"),flow), (("wu", "wpt"), PARAM_COLOR["wu"]), (("wv", "wpt"), PARAM_COLOR["h"]),
                                                                                  (("wpt", "wpl"), flow), (("xz", "wpl"), flow)])),), 4.0)), seconds=7.0)

        cur = [n.view for n in wrong.values() if n is not wrong["xy"]] + [n.view for n in front.values()] + front_wires + bridge
        self._reg = dict(sliders)
        cut, cut_wires = self._rig(d, source, {"fr":(-2.8,2),"wv":(-2.8,1),"cp":(-1.4,1.5),"xz":(-2.8,-.1),"cpl":(-1.3,.1),"rad":(-2.8,-1.1),"twow":(-2.8,-2),"wall":(-1,-2),"fil":(.6,2),"cyl":(.2,-.4),"cut":(1.2,.8)},
                                   widths={"wall": 2.3})
        def cut_ready():
            seat = self._probe()
            u, vv, d_ = seat.p["window_u"], seat.p["window_v"], seat.p["window_diameter"]
            depth = float(self._solve().tree("wall").items()[0])
            angles = np.linspace(0, 2 * math.pi, 48)
            front_pts = [seat.drawing.at(u + d_ / 2 * math.cos(a), seat.p["front"], vv + d_ / 2 * math.sin(a)) for a in angles]
            back_pts = [seat.drawing.at(u + d_ / 2 * math.cos(a), seat.p["front"] - depth, vv + d_ / 2 * math.sin(a)) for a in angles]
            hold["ring"] = _draft.polyline(front_pts, PARAM_COLOR["d"], 3.4, 1.0, closed=True)
            hold["prism"] = _engine.VGroup(_draft.polygon(front_pts, PARAM_COLOR["d"], 0.35), _draft.polyline(back_pts, PARAM_COLOR["d"], 2.0, 0.8, closed=True),
                                           _draft.segments([(front_pts[i], back_pts[i]) for i in range(0, 48, 12)], PARAM_COLOR["d"], 2.0))
            hold["hole"] = _engine.VGroup(_draft.polygon(front_pts, "#1B2A33", 0.9), _draft.polyline(front_pts, "#1B2A33", 2.0, 1.0, closed=True))
            self.add(hold["ring"])

        def cut_done():
            rv["window"].set_value(1.0)
            self.remove(hold["hole"])

        cur_wires = front_wires + bridge
        cur_nodes = [view for view in cur if view not in cur_wires]
        cut_grow = [_engine.GrowFromPoint(n.view, cut["cut"].view.get_center()) for key, n in cut.items() if key != "cut"]
        self.step("cut", _Seq(((lambda: (_morph.gather(cur_nodes, cut["cut"].view, run_time=1.2, lag=0.1, limit=2.0),
                                         _engine.AnimationGroup(*fout(cur_wires), *cut_grow, *fin(cut_wires), self._to(rv["dd"], 1.0, 1.0), self._to(rv["du"], 0.0, 1.0))), 2.0)), cut_ready,
                              (lambda: (_morph.morph(hold["ring"], hold["prism"], run_time=1.4),), 1.4), (lambda: (_morph.morph(hold["prism"], hold["hole"], run_time=1.4), self._to(rv["front"], 0.0, 1.4)), 1.4), cut_done), seconds=6.5)

        cur = [n.view for n in cut.values()] + cut_wires
        bolt_sliders = {key: self._node(key, (-5.05, 1.9 - 0.8 * index, 0), d, source) for index, key in enumerate(BOLT_IDS)}
        self._reg = dict(bolt_sliders)
        bolts, bolt_wires = self._rig(d, source, {"nbu": (-2.85, 1.9), "mx": (-1.45, 1.3), "mz": (-1.45, -0.3), "back":(-2.8,-1.6), "bolt": (0.4, 0.6)})
        point_panel = self.gh_panel([self.tr("Bolt points [m]", "Schraubenpunkte [m]")] + [f"{i}  " + self._show(p_) for i, p_ in enumerate(solution.tree("bolt").items())], at=(0.15, -1.6, 0), width=3.2)
        point_wire = self.gh_wire(bolts["bolt"].anchor.grip_out(), point_panel.grip_in(), kind="list")
        def bolt_rings():
            back = self._probe().drawing
            hold["rings"] = [_engine.Circle(radius=0.075, stroke_color=WARN, stroke_width=3, fill_color="#FFFFFF", fill_opacity=0.9).move_to(back.at(pt.x, pt.y, pt.z)) for pt in solution.tree("bolt").items()]
            self.add(*hold["rings"])

        self.step("bolts", _Seq(((_engine.AnimationGroup(*fout(cur), *fout(col), *fin([n.view for n in bolt_sliders.values()]), *fin([n.view for n in bolts.values()]), *fin(bolt_wires),
                                                         self._to(st.az, -AZIMUTH, 2.5), self._to(rv["bolts"], 1.0, 1.5), rv_set(dw=0.0, dl=0.0, dh=0.0, dd=0.0)),), 3.0), bolt_rings,
                                 (lambda: (_morph.gather(hold["rings"], point_panel, run_time=1.0, lag=0.2, limit=2.4), _engine.FadeIn(point_wire)), 2.4)), seconds=7.0)

        cur = [n.view for n in bolt_sliders.values()] + [n.view for n in bolts.values()] + bolt_wires + [point_panel, point_wire]
        self._reg = dict(sliders)
        spans = {key: d.obj(key).settings for key in ("w", "l", "h")}
        card = self._card([(self.tr(*en_de), key, f"{self.quantity(spans[key]['min'], 'm')} – {self.quantity(spans[key]['max'], 'm')}") for key, en_de in
                           (("w", ("Width", "Breite")), ("l", ("Length", "Länge")), ("h", ("Height", "Höhe")))], (-1.15, 0.6))
        def range_ready():
            hi_w = d.obj("w").settings["max"]
            seat = self._probe()
            hold["range"] = card[4].copy()
            hold["edge"] = _draft.polyline([seat.v(-0.5 * hi_w, 0.0, 0.0), seat.v(0.5 * hi_w, 0.0, 0.0)], "#5E6A72", _draft.W_EDGE + 0.4, 0.95, dash=0.1)
            self.add(hold["range"])

        def range_done():
            rv["env"].set_value(1.0)
            self.remove(hold["edge"])

        self.step("domains", _Seq(((_engine.AnimationGroup(*fout(cur), *fin(col), _engine.FadeIn(card), self._to(st.az, AZIMUTH, 2.0), self._to(rv["bolts"], 0.0, 1.0), rv_set(dw=1.0, dl=1.0, dh=1.0)),), 2.2),
                                  range_ready, (lambda: (_morph.morph(hold["range"], hold["edge"], run_time=1.4),), 1.4), range_done), seconds=6.0)

        width_hi = d.obj("w").settings["max"]
        limit_badge = self.badge(self.tr("limit reached", "Grenze erreicht") + " · " + self.quantity(width_hi, "m"), (-1.15, -1.6), color=WARN, size=SMALL, filled=False)
        slider_box = sliders["w"].view.body
        red_box = _engine.Rectangle(width=slider_box.width + 0.1, height=slider_box.height + 0.1, stroke_color=RED, stroke_width=4, fill_opacity=0).move_to(slider_box.get_center())
        request = self._tag(self.tr("requested ", "angefragt ") + self.quantity(width_hi + 0.5, "m") + " → " + self.quantity(width_hi, "m"), (-1.15, -2.15, 0.0), color="#B3261E")
        self.step("limit", _engine.Succession(self._ramp(vals["w"], value("w"), width_hi, 2.4), _engine.AnimationGroup(self._to(rv["hit"], 1.0, 0.4), _engine.FadeIn(limit_badge)),
                                              _engine.AnimationGroup(self._to(rv["over"], 1.0, 0.5), _engine.FadeIn(red_box), _engine.FadeIn(request)), _engine.Wait(1.0)), seconds=7.5)

        table = self._table((-2.6, 1.25))
        minis = _engine.VGroup()
        for index, t in enumerate(types):
            x = -4.6 + 1.3 * index
            minis.add(self.capsule_axon(t, at=(x, -1.5, 0), scale=0.2, base=_nkd.type_color(t.key), cutaway=True, interior="study"), self._say(self.tr("Type ", "Typ ") + t.key, (x, -2.2), size=SMALL, color=_nkd.type_color(t.key)))
        order = [types[0], types[1], types[3], types[2]]
        ty = _engine.ValueTracker(0.0)

        def highlight():
            t = order[int(round(ty.get_value()))]
            rect = table.rows[t.key][0]
            return _engine.VGroup(_engine.Rectangle(width=rect.width, height=rect.height, stroke_color=_nkd.type_color(t.key), stroke_width=3, fill_color=_nkd.type_color(t.key),
                                                    fill_opacity=0.18).move_to(rect.get_center()))

        marker = self.live(highlight, key=lambda: int(round(ty.get_value())))
        typetag = self.live(lambda: _engine.VGroup(self._tag(self.tr("Shared envelope · program ", "Gemeinsame Hülle · Programm ") + order[int(round(ty.get_value()))].key,
                                                              (VP[0], VP[1] + VP_H / 2 - 0.75, 0.0), color=DARK, size=LABEL)), key=lambda: int(round(ty.get_value())))
        def minis_ready():
            hold["src"] = [minis[2 * index].copy() for index in range(len(types))]
            hold["dst"] = [table.rows[t.key][-1].copy() for t in types]
            self.add(*hold["src"])

        self.step("types", _Seq(((_engine.AnimationGroup(*fout([card, limit_badge, red_box, request]), self._to(rv["over"], 0.0, 0.5), self._ramp(vals["w"], width_hi, value("w"), 1.5),
                                                         self._to(rv["env"], 0.0, 1.0), self._to(rv["hit"], 0.0, 0.5)),), 1.6), ((_engine.AnimationGroup(*fout(col)),), 0.3), minis_ready,
                                (lambda: (_morph.scatter(hold["src"], hold["dst"], lag=0.3, run_time=1.2, path_arc=0.5, limit=2.4), _engine.FadeIn(minis), _engine.FadeIn(table), _engine.FadeIn(marker)), 2.4),
                                lambda: self.remove(*hold["dst"])), seconds=5.0)

        steps = [_engine.AnimationGroup(self._ramp(ty, float(i) - 1.0 if i else 0.0, float(i), 0.2), *[self._to(vals[k], v, 2.0) for k, v in _reference_controls(t).items()]) for i, t in enumerate(order)]
        self.step("four", _engine.Succession(_engine.AnimationGroup(_engine.FadeIn(typetag), self._to(rv["dd"], 1.0, 1.0), self._to(rv["du"], 1.0, 1.0)), _engine.Succession(*steps)), seconds=9.0)

        for key in IDS:
            vals[key].set_value(value(key))
        cur = [table, minis, marker, typetag]
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
        spot_a, spot_b = self._core.at(*self._plane_a["origin"]), self._core.at(goal.x, goal.y, goal.z)
        plane_a_token = _morph.token("A", kind="item", color=_nkd.type_color("A")).move_to(spot_a)
        start_capsule, end_capsule = self._capsule_at(0.0), self._capsule_at(1.0)

        def orient_done():
            cv["cap"].set_value(1.0)
            cv["glide"].set_value(1.0)
            self.remove(end_capsule)

        self.step("orient", _Seq(((_engine.AnimationGroup(*fin([n.view for n in capn.values()]), *fin(cap_wires), _engine.FadeIn(direct), _engine.FadeIn(move_panel), _engine.FadeIn(move_wire)),), 1.2),
                                 lambda: self.add(start_capsule),
                                 ((_morph.morph(start_capsule, plane_a_token, pairs=_morph.capsule_pairs(start_capsule, plane_a_token), dip=0.15, run_time=1.2), self._to(cv["arrow"], 1.0, 1.2)), 1.2),
                                 ((_morph.travel(plane_a_token, [spot_a, (spot_a + spot_b) / 2 + np.array([0.0, 0.4, 0.0]), spot_b], run_time=1.8),), 1.8),
                                 ((_morph.morph(plane_a_token, end_capsule, pairs=[(t, s) for s, t in _morph.capsule_pairs(end_capsule, plane_a_token)], dip=0.15, run_time=1.4),), 1.4), orient_done), seconds=6.0)

        self._reg = {**apl, **capn}
        spin, spin_wires = self._rig(self._place, self._solve_place, {"an": (-5.3, -1.85), "rad": (-3.3, -1.85), "ax": (-3.3, -0.2), "rot": (-1.5, -0.4), "pr": (0.15, -1.75)},
                                     live=("pr",), widths={"pr": 3.3})
        angle_token = _morph.token(self.number(-90.0, 0) + "°", kind="number", color="#3F6FB5")

        def angle_ready():
            cx, cy, cz, a0, radius = self._arc
            points = [self._core.at(cx + radius * math.cos(a0 + math.radians(t)), cy + radius * math.sin(a0 + math.radians(t)), cz) for t in np.linspace(0.0, -90.0, 36)]
            hold["arc"] = _engine.VGroup(_draft.polyline(points, "#3F6FB5", 3.2, 0.95), _engine.Dot(points[-1], radius=0.05, color="#3F6FB5"),
                                         self._tag(self.number(-90.0, 0) + "°", points[len(points) // 2] + np.array([0.45, -0.35, 0.0]), color="#25518C"))
            angle_token.move_to(spin["an"].view.get_center() + np.array([0.0, 0.45, 0.0]))
            self.add(angle_token)

        def angle_done():
            cv["angle"].set_value(-90.0)
            self.remove(hold["arc"])

        spin_grow = [_engine.GrowFromPoint(n.view, spin["pr"].view.get_center()) for key, n in spin.items() if key != "pr"]
        self.step("rotate", _Seq(((lambda: (_morph.gather([move_panel, apl["pa"].view, apl["pb"].view], spin["pr"].view, run_time=1.2, lag=0.2, limit=2.0),
                                            _engine.AnimationGroup(*fout([direct, move_wire, *apl_wires]), *spin_grow, *fin(spin_wires), self._to(cv["arrow"], 0.0, 0.6), self._to(cv["arc"], 1.0, 1.0),
                                                                   self._to(cv["glide"], 1.0, 1.2))), 2.0)), angle_ready,
                                 (lambda: (_morph.morph(angle_token, hold["arc"], path_arc=0.4, run_time=1.6),), 1.6), angle_done,
                                 ((_engine.Succession(*[self._to(cv["angle"], a, 1.8) for a in (-180.0, -270.0)]),), 3.6)), seconds=8.0)

        cur = [n.view for n in {**capn, **spin}.values()] + [apl["apl"].view, apl["tpl"].view] + cap_wires + spin_wires
        self._reg = {}
        wv["count"].set_value(4.0)
        chain, chain_wires = self._rig(self._wrap, self._solve_wrap, {"types": (-5.3, 1.8), "idx": (-5.3, -0.1), "li": (-3.0, 1.0), "ori": (-0.6, 0.7), "pa": (-3.0, 2.2), "tg": (-0.6, 2.2),
                                                                     "out": (1.0, -1.1), "wr": (-3.0, -1.9)}, live=("idx", "out", "li"), widths={"out": 1.4})
        picks = self._solve_wrap().tree("li").items()[:4]
        item_tokens = [_morph.token(pick.tag["type"], kind="item", color=_nkd.type_color(pick.tag["type"])).move_to(chain["li"].anchor.get_center() + np.array([0.9, 0.1 * index - 0.2, 0.0])) for index, pick in enumerate(picks)]
        placed = [self._on_core(self._core_wrap, slot.plane.origin, math.degrees(math.atan2(slot.plane.x[1], slot.plane.x[0])), pick.tag["type"], layout=pick.tag["family"]) for slot, pick in zip(self._wrap_slots, picks)]

        def placed_done():
            wv["caps"].set_value(1.0)
            self.remove(*placed)

        self.step("listitem", _Seq(((_engine.AnimationGroup(*fout(cur), _engine.FadeOut(glide), _engine.Succession(_engine.FadeIn(wrapstage, run_time=0.5), self._to(wv["planes"], 1.0, 1.2)),
                                                            *fin([n.view for n in chain.values()]), *fin(chain_wires)),), 2.0), lambda: self.add(*item_tokens),
                                   ((_morph.scatter(item_tokens, placed, lag=0.5, run_time=1.2, path_arc=0.5, limit=3.0, pairs_fn=_morph.capsule_pairs),), 3.0), placed_done), seconds=7.0)

        wv["rev"].set_value(4.0)
        notice = self.live(lambda: self._notice(chain["li"]), key=lambda: (self._solve_wrap().state("li"), round(wv["count"].get_value())))
        stray = _morph.token("4", kind="number", color="#E08A1E").move_to(chain["idx"].anchor.get_center() + np.array([0.0, 0.35, 0.0]))
        nothing = _morph.token("null", kind="text", color=RED).move_to(chain["out"].anchor.get_center() + np.array([0.0, -0.8, 0.0]))
        self.step("outside", _Seq(((_engine.FadeIn(notice),), 0.5), ((self._to(wv["count"], 8.0, 1.5),), 1.5), lambda: self.add(stray),
                                  ((_morph.travel(stray, [stray.get_center(), chain["li"].anchor.get_center() + np.array([-0.2, 0.0, 0.0])], run_time=1.2),), 1.2),
                                  ((_morph.morph(stray, nothing, run_time=1.0),), 1.0), ((_engine.Wait(0.5),), 0.5)), seconds=6.0)
        wrap_types = _nk.capsule_types()
        index_token = _morph.token("4", kind="number", color="#E08A1E").move_to(chain["idx"].anchor.get_center() + np.array([1.3, 0.0, 0.0]))
        again = _morph.token(wrap_types[0].key, kind="item", color=_nkd.type_color(wrap_types[0].key)).move_to(chain["li"].anchor.get_center() + np.array([-0.2, 1.0, 0.0]))

        def wrap_loop():
            self.remove(nothing)
            self.add(index_token)

        loop = _engine.CurvedArrow(index_token.get_center(), again.get_center(), angle=-2.4, color="#E08A1E", stroke_width=3.5)
        self.step("wrap", _Seq(((self._to(wv["wrap"], 1.0, 0.6), _engine.FadeIn(loop)), 0.8), wrap_loop, ((_morph.travel(index_token, loop, run_time=1.4),), 1.4),
                               ((_morph.morph(index_token, again, run_time=0.8),), 0.8), ((_engine.AnimationGroup(self._to(wv["rev"], 8.0, 3.2), _engine.FadeOut(again), _engine.FadeOut(loop)),), 3.2)), seconds=6.0)

        cur = [n.view for n in chain.values()] + chain_wires + [notice]
        legend = [self._legend_row(i, *row) for i, row in enumerate(LEGEND)]
        core = self._core_wrap
        slot = self._wrap_slots[0]
        ring = lambda point, radius=0.4: _engine.Circle(radius=radius, stroke_color="#FFB000", stroke_width=6, fill_opacity=0).move_to(point)
        strip_box = _engine.Rectangle(width=4.5, height=0.95, stroke_color="#FFB000", stroke_width=5, fill_opacity=0).move_to((VP[0], VP[1] - 2.05, 0))
        targets = [None, None, ring(core.at(*(np.array(slot.facade_plane.origin) + np.array([0.0, 2.0, 1.25]))), 0.6), None, ring(core.at(*slot.facade_plane.origin), 0.3),
                   ring(core.at(*self._wrap_slots[1].facade_plane.origin), 0.3), strip_box, ring((VP[0] - 1.9 + 3.8 / 7 * 4, VP[1] - 2.0, 0), 0.35)]
        born = {5: chain["ori"].anchor.copy(), 6: chain["li"].anchor.copy(), 7: chain["idx"].anchor.copy()}
        glow = [_engine.Succession(_engine.Wait(0.1) if i in born else _engine.FadeIn(legend[i], run_time=0.5), _engine.AnimationGroup(_engine.Indicate(legend[i], color="#FFB000", scale_factor=1.06, run_time=0.9),
                                                                                               *([_engine.Succession(_engine.FadeIn(targets[i], run_time=0.25), _engine.Wait(0.4), _engine.FadeOut(targets[i], run_time=0.25))] if targets[i] is not None else [])))
                for i in range(len(legend))]
        self.step("recap", _Seq(lambda: self.add(*born.values()),
                                ((_engine.AnimationGroup(*fout(cur)), *[born[i].animate.scale(0.35).move_to(legend[i][0].get_center()) for i in born]), 1.4),
                                ((*[_morph.morph(born[i], legend[i], run_time=1.2) for i in born],), 1.2),
                                ((_engine.Succession(*glow),), 9.0)), seconds=11.0)
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
