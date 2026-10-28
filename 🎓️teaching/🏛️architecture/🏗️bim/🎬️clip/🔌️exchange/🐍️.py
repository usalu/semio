"""🎬 Interfaces and Translation: piloti B3 and the first-floor slab of the Villa Savoye travel from the architecture model to a structural check."""

import importlib
import math
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN = _d.INK, _d.MUTED, _d.ARCH, _d.STRUCT, _d.DATA, _d.GOOD, _d.WARN
S_SMALL, S_LABEL = _d.S_SMALL, _d.S_LABEL
TAG = "B3"
SLAB = "SL-1"
HATCH = _d.mix(INK, _d.BACKGROUND, 0.5)
Q_SCALE = 0.05
Q_STACK = 0.09
R_SCALE = 0.004
STAR = "*"
LOAD_COLORS = (("self_weight", STRUCT), ("finishes", ARCH), ("imposed", DATA))


def _pick(excerpt, *needles):
    """🔎 A sub-excerpt with the STEP lines that contain one of the needles, in needle order (``^`` marks a line start); spans follow their lines."""
    keep = []
    for needle in needles:
        for i, line in enumerate(excerpt["lines"]):
            if i not in keep and (line.startswith(needle[1:]) if needle.startswith("^") else needle in line):
                keep.append(i)
    remap = {old: new for new, old in enumerate(keep)}
    return {"lines": [excerpt["lines"][i] for i in keep],
            "spans": [dict(span, line=remap[span["line"]]) for span in excerpt["spans"] if span["line"] in remap]}


def _run(panel, needle, role=None, nth=0):
    """🔖 The token of a step panel on the line holding ``needle``; without that role the last token of the line."""
    runs = [run for run in panel.runs if needle in run.source_text]
    chosen = [run for run in runs if run.role == role] if role else runs
    chosen = chosen or runs[-1:]
    return chosen[min(nth, len(chosen) - 1)]


def _ring(mob, color=DATA, buff=0.045):
    """⭕ A rectangle ring around a token."""
    return _engine.SurroundingRectangle(mob, color=color, buff=buff, corner_radius=0.04, stroke_width=2.4)


def _strike(mob, color=WARN):
    """➖ A strike line through a token."""
    y = mob.get_center()[1]
    return _d.segments([((mob.get_left()[0] - 0.04, y, 0), (mob.get_right()[0] + 0.04, y, 0))], color, 3.2)


def _wire(start, end, color=DATA, opacity=0.95, dot=1):
    """🧵 A smooth leader curve with horizontal tangents and a dot on one end (0 start, 1 end, None no dot)."""
    a, b = _d.P(start), _d.P(end)
    reach = np.array([0.5 * (b[0] - a[0]) if abs(b[0] - a[0]) > 0.4 else 0.0, 0.0, 0.0])
    curve = _engine.CubicBezier(a, a + reach, b - reach, b)
    curve.set_fill(opacity=0).set_stroke(color, _d.W_THIN, opacity)
    group = _engine.VGroup(curve)
    if dot is not None:
        group.add(_engine.Dot(b if dot else a, radius=0.05, color=color))
    return group


def _route(ring, panel, target, color=DATA, lane=0):
    """🛤️ A leader that leaves a token ring downward into the gap between two STEP rows, runs out of the panel along that gap and curves to the target."""
    end = _d.P(target)
    rows = [row for row in panel.rows if len(row.submobjects)]
    row = min(rows, key=lambda r: abs(r.get_center()[1] - ring.get_center()[1]))
    y = min(row.get_bottom()[1], ring.get_bottom()[1]) - 0.03 - 0.025 * lane
    x0, x1 = ring.get_center()[0], panel.frame.get_right()[0] + 0.14
    path = _d.polyline([(x0, ring.get_bottom()[1], 0), (x0, y, 0), (x1, y, 0)], color, _d.W_THIN, 0.95)
    reach = np.array([0.5 * (end[0] - x1) if end[0] - x1 > 0.4 else 0.0, 0.0, 0.0])
    start = np.array([x1, y, 0.0])
    curve = _engine.CubicBezier(start, start + reach, end - reach, end)
    curve.set_fill(opacity=0).set_stroke(color, _d.W_THIN, 0.95)
    return _engine.VGroup(path, curve, _engine.Dot(end, radius=0.05, color=color))


def _backed(label, pad=0.05):
    """🧱 A label on an opaque background plate, so lines and hatches behind it do not run through the text."""
    plate = _engine.RoundedRectangle(corner_radius=0.04, width=label.width + 2 * pad, height=label.height + 2 * pad, fill_color=_d.BACKGROUND, fill_opacity=0.92, stroke_width=0).move_to(label)
    return _engine.VGroup(plate, label)


def _hatched(points, fill, opacity=0.45, stroke=INK, width=_d.W_THIN, spacing=0.07, color=HATCH):
    """▨ A polygon with fill, outline and a 45 degree hatch."""
    return _engine.VGroup(_d.polygon(points, fill, opacity, stroke, width), _d.hatch(points, color, spacing, 45, _d.W_HAIR, 0.9))


def _inside(point, ring):
    """📍 Even-odd test of a plan point against a ring."""
    x, y, hit = point[0], point[1], False
    for (ax, ay), (bx, by) in zip(ring, ring[1:] + ring[:1]):
        if (ay > y) != (by > y) and x < ax + (y - ay) * (bx - ax) / (by - ay):
            hit = not hit
    return hit


def _volume(mesh):
    """🧊 Volume enclosed by a closed outward triangle mesh (divergence theorem)."""
    points, faces = np.asarray(mesh["vertices"], float), np.asarray(mesh["faces"], int)
    return abs(float(np.einsum("ij,ij->i", points[faces[:, 0]], np.cross(points[faces[:, 1]], points[faces[:, 2]])).sum())) / 6.0


ALLOWED_UNITS = frozenset({"m", "m²", "m³", "kN", "kN/m²", "kN/m³", "kg/m³", "°"})


def _units(node, found=None):
    """📏 The set of unit strings that occur in the model."""
    found = set() if found is None else found
    if isinstance(node, dict):
        if isinstance(node.get("unit"), str) and node["unit"]:
            found.add(node["unit"])
        for value in node.values():
            _units(value, found)
    elif isinstance(node, list):
        for value in node:
            _units(value, found)
    return found


def _seq(*stages):
    """🎞️ Stages in order; a tuple is a stage whose animations run together."""
    items = [_engine.AnimationGroup(*stage, rate_func=_engine.linear) if isinstance(stage, tuple) else stage for stage in stages]
    return _engine.Succession(*items, rate_func=_engine.linear)


def _in(mob):
    """🌫️ Fade a drawing in through a plain group, so drawings with their own ``scale`` attribute stay animatable."""
    return _engine.FadeIn(_engine.VGroup(mob))


def _to(tracker, value, seconds=1.0):
    """📈 Move a tracker to a value over some seconds."""
    return tracker.animate(run_time=seconds, rate_func=_engine.linear).set_value(value)


class Beat5_Interfaces(_visuals.ClipScene):
    """🔌 Exchange one piloti and one slab as IFC STEP, as a triangle mesh and as a structural analysis model."""

    TITLE = {"en": "Interfaces and Translation", "de": "Schnittstellen und Übersetzung"}
    NARRATION = [
        ["interface",
         "An interface defines the exchange. IFC is a standard schema, ISO 16739-1, that both sides know.",
         "Eine Schnittstelle definiert den Austausch. IFC ist ein Standardschema (ISO 16739-1), das beide Seiten kennen."],
        ["semantic",
         "Semantic exchange sends an IfcColumn at its place, with a stable GlobalId. Instance numbers (#n) are only local.",
         "Semantischer Austausch sendet eine IfcColumn am Ort mit stabiler GlobalId. Instanznummern (#n) gelten nur lokal."],
        ["properties",
         "Its properties travel along: type COLUMN, LoadBearing flag, material, circular profile and extrusion depth.",
         "Seine Eigenschaften reisen mit: Typ COLUMN, Flag LoadBearing, Material, Kreisprofil und Extrusionstiefe."],
        ["mesh",
         "A triangle mesh keeps only vertices and faces. The shape survives roughly; class, identity, properties do not.",
         "Ein Dreiecksnetz behält nur Punkte und Flächen. Die Form bleibt grob, Klasse, Identität und Eigenschaften fehlen."],
        ["roundtrip",
         "Round trip: IFC to a mesh and back. The shape returns, but class, GlobalId and LoadBearing stay lost.",
         "Rundreise: IFC zum Dreiecksnetz und zurück. Die Form kommt zurück, Klasse, GlobalId und LoadBearing nicht."],
        ["purpose",
         "To check the first-floor slab structurally, we choose an analysis schema and idealise the slab at its mid-surface.",
         "Für die statische Prüfung der Geschossdecke wählen wir ein Analyseschema und idealisieren sie als Mittelfläche."],
        ["mapping",
         "The slab becomes shell elements on the grid, each piloti a member. Every analysis item records its source GlobalId.",
         "Die Decke wird zu Schalenelementen im Raster, Stützen zu Stäben. Jedes Objekt verweist auf seine Quell-GlobalId."],
        ["slab",
         "The slab is the villa's own: the ramp opening is cut out, the salon rests on it, and the net area follows.",
         "Die Decke gehört der Villa: Rampenöffnung ausgespart, der Salon ruht darauf, daraus folgt die Nettofläche."],
        ["assumptions",
         "The analysis needs assumptions: self-weight from the thickness, finishes, imposed load, and the support conditions.",
         "Die Analyse braucht Annahmen: Eigengewicht aus der Dicke, Ausbaulasten, Nutzlast und die Auflagerbedingungen."],
        ["buildup",
         "The load is a stack: the slab's own weight, the finishes and the imposed load add up to one area load.",
         "Die Last ist ein Stapel: Eigengewicht der Decke, Ausbaulasten und Nutzlast ergeben eine Flächenlast."],
        ["cells",
         "Each piloti owns the slab up to the midlines to its neighbours. Beside the ramp opening, B3 owns a notched cell.",
         "Jeder Stütze gehört die Decke bis zu den Mittellinien zu den Nachbarn. Neben der Rampenöffnung hat B3 eine Kerbe."],
        ["reaction",
         "Each piloti is assigned its tributary area. This is a simplified estimate, not a finite element result.",
         "Jeder Stütze wird ihre Lasteinzugsfläche zugeordnet. Das ist eine grobe Schätzung, kein FE-Ergebnis."],
        ["feedback",
         "The tool sends back reactions. They return to the model as a result linked to B3; geometry never flows back.",
         "Das Werkzeug sendet Auflagerkräfte zurück. Sie kehren als verknüpftes Ergebnis zu B3 zurück, Geometrie nie."],
        ["loss",
         "Ribbon windows, infill walls and render colour do not cross over. Translating back cannot restore them.",
         "Bandfenster, Ausfachungswände und Putzfarbe gehen nicht über. Zurückübersetzen stellt sie nicht wieder her."],
        ["domain",
         "IFC has structural analysis entities, but analysis tools often use their own schemas. This is one chosen example.",
         "IFC hat Entitäten für die Tragwerksanalyse, Analysewerkzeuge nutzen oft eigene Schemata. Das ist ein Beispiel."],
        ["check",
         "Before trusting the exchange, validate what the receiver needs, for example with an IDS requirement.",
         "Bevor man dem Austausch vertraut, prüft man, was der Empfänger braucht, zum Beispiel mit einer IDS-Anforderung."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("exchange")

    def _facts(self):
        """📚 Every model value of this clip, read once."""
        m = _model.load_model()
        g = _model.building_geometry(m)
        element = _model.element(TAG, m)
        excerpt = _model.step_excerpt(TAG, m, language=self.language())
        return SimpleNamespace(
            m=m, g=g, b3=next(p for p in g["pilotis"] if p["tag"] == TAG), slab=next(s for s in g["slabs"] if s["tag"] == SLAB), element=element,
            slab_storey=_model.element(SLAB, m)["storey"], q3=_model.quantities(TAG, m), qs=_model.quantities(SLAB, m), excerpt=excerpt,
            renumbered=_model.step_excerpt(TAG, m, first_id=excerpt["first_id"] + len(excerpt["lines"]), language=self.language()),
            analysis=_model.translate_structure(m), trib=_model.tributary(m), ids=_model.ids_check(m), ids_mesh=_model.ids_check(m, source="mesh"),
            diameter=_model.information(TAG, "diameter", "en", m), thickness=_model.information(SLAB, "thickness", "en", m),
            objects=self._objects(m))

    def _objects(self, m):
        """🧮 Object counts of the partial models and of the design model that is drawn on the sender side (architecture and structure, shared objects once)."""
        tags = {key: set(_model.partial_model(key, m)["element_tags"]) for key in ("architecture", "structure")}
        return {"architecture": len(tags["architecture"]), "structure": len(tags["structure"]), "design": len(tags["architecture"] | tags["structure"])}

    def _legend(self, point, anchor="l"):
        """⭐ The legend of the assumption star."""
        return self.label(STAR + " assumed value (teaching model)", STAR + " angenommener Wert (Lehrmodell)", point, size=S_SMALL, color=WARN, anchor=anchor)

    def _count_legend(self, f, point, anchor="l"):
        """🔢 The legend of the piloti count: the model's own number, marked with the star, and the range the sources give."""
        n = len(f.g["pilotis"])
        return self.label(f"{STAR} {n} pilotis in this teaching model; sources differ: 15–{n}", f"{STAR} {n} Pilotis in diesem Lehrmodell; Quellen weichen ab: 15–{n}", point, size=S_SMALL, color=WARN, anchor=anchor)

    def _percent(self, small, large):
        """➗ The deviation of two printed volumes (rounded to four decimals as shown) in percent."""
        return self.number(100 * (round(small, 4) / round(large, 4) - 1), 1)

    def _text(self, item):
        """🏷️ The text of a model name that is a string or an en/de pair."""
        return item[self.language()] if isinstance(item, dict) else str(item)

    def _basis(self, entry):
        """🔖 The word for the basis of an authored value: sourced or assumed."""
        return self.tr("assumed", "angenommen") if entry["basis"] == "assumed" else self.tr("sourced", "belegt")

    def _lines(self, rows, point, color=INK, size=S_SMALL, pitch=0.27, anchor="l"):
        """📝 Left-aligned text lines under each other; a row is a string or a ``(string, colour)`` pair."""
        group = _engine.VGroup()
        for index, row in enumerate(rows):
            text, tone = row if isinstance(row, tuple) else (row, color)
            group.add(self.text(text, (point[0], point[1] - pitch * index, 0), size=size, color=tone, anchor=anchor))
        return group

    def present_exchange(self):
        """🎬 The whole exchange story of the clip, beat by beat."""
        f = self._facts()
        left, right = self._beat_interface(f)
        self._beat_semantic(f, left, right)
        self._beat_properties(f)
        self._beat_mesh(f)
        self._beat_roundtrip(f)
        self._beat_purpose(f)
        self._beat_mapping(f)
        self._beat_slab(f)
        self._beat_assumptions(f)
        self._beat_buildup(f)
        self._beat_cells(f)
        self._beat_reaction(f)
        self._beat_feedback(f)
        self._beat_loss(f)
        self._beat_domain(f)
        self._beat_check(f)

    def _bay(self, f, point, scale, half):
        """🔍 A crop of the ground-floor plan around B3: grid axes, ramp opening, glass wall and the piloti at its true diameter."""
        x, y, radius = f.b3["x"], f.b3["y"], f.b3["radius"]
        view = _d.View("plan", point, scale, (x, y, 0.0))
        d = _d.Drawing(view)
        low, high = (x - half, y - half), (x + half, y + half)
        d.add(_d.polygon([view(low[0], low[1]), view(high[0], low[1]), view(high[0], high[1]), view(low[0], high[1])], _d.GROUND_PLANE, 1.0, MUTED, _d.W_THIN))
        for void in f.slab["voids"]:
            x0, x1 = max(min(p[0] for p in void), low[0]), min(max(p[0] for p in void), high[0])
            y0, y1 = max(min(p[1] for p in void), low[1]), min(max(p[1] for p in void), high[1])
            if x1 > x0 and y1 > y0:
                ring = [view(x0, y0), view(x1, y0), view(x1, y1), view(x0, y1)]
                d.add(_hatched(ring, ARCH, 0.12, ARCH, _d.W_THIN, 0.09, _d.mix(ARCH, _d.BACKGROUND, 0.35)))
                d.add(self.label("ramp", "Rampe", view((x0 + x1) / 2, low[1]) + np.array([0, -0.25, 0]), size=S_SMALL, color=ARCH))
        wall = [p for p in f.g["curtain_wall"]["path"] if low[0] <= p[0] <= high[0]]
        for a, b in zip(wall, wall[1:]):
            if min(a[1], b[1]) <= high[1] and max(a[1], b[1]) >= low[1]:
                d.add(_d.segments([(view(a[0], max(min(a[1], high[1]), low[1])), view(b[0], max(min(b[1], high[1]), low[1])))], _d.GLASS, _d.W_CUT))
        d.add(_d.polyline([view(low[0], y), view(high[0], y)], MUTED, _d.W_THIN, 0.9, dash=0.14), _d.polyline([view(x, low[1]), view(x, high[1])], MUTED, _d.W_THIN, 0.9, dash=0.14))
        for text, spot in ((f.g["grid"]["labels_x"][f.g["grid"]["x"].index(x)], view(x, high[1]) + np.array([0, 0.22, 0])),
                           (f.g["grid"]["labels_y"][f.g["grid"]["y"].index(y)], view(high[0], y) + np.array([0.22, 0, 0]))):
            d.add(_engine.Circle(radius=0.14, stroke_color=INK, stroke_width=_d.W_THIN).move_to(spot), self.text(text, spot, size=S_SMALL, color=INK))
        centre = view(x, y, 0.0)
        d.add(_engine.Circle(radius=0.3, stroke_color=DATA, stroke_width=_d.W_THIN, stroke_opacity=0.8).move_to(centre),
              _engine.Circle(radius=radius * scale, stroke_color=DATA, stroke_width=_d.W_EDGE, fill_color=_d.CONCRETE, fill_opacity=1.0).move_to(centre))
        d.centre, d.radius, d.frame_corners = centre, radius * scale, [view(low[0], low[1]), view(high[0], low[1]), view(high[0], high[1]), view(low[0], high[1])]
        d.edge_right = view(high[0], y)
        return d

    def _post(self, f, grow, solid, point, scale):
        """🏛️ The piloti B3 in elevation under the first-floor slab: ground, concrete hatch, height dimension, level marks and the slab above."""
        x, y, radius, top = f.b3["x"], f.b3["y"], f.b3["radius"], f.b3["top"]
        slab_top = f.slab["top"]
        view = _d.View("section", point, scale, (x, y, top / 2), axis="x")
        d = _d.Drawing(view)
        reach = 1.4
        ground = [view(x - reach, y, 0), view(x + reach, y, 0), view(x + reach, y, -0.3), view(x - reach, y, -0.3)]
        d.add(_d.polygon(ground, _d.GROUND_PLANE, 1.0, None), _d.hatch(ground, _d.GREEN_GROUND, 0.08, 45, _d.W_HAIR, 0.9), _d.segments([(ground[0], ground[1])], INK, _d.W_EDGE))
        beam = [view(x - reach, y, top), view(x + reach, y, top), view(x + reach, y, slab_top), view(x - reach, y, slab_top)]
        d.add(_hatched(beam, _d.CONCRETE, 0.3, INK, _d.W_EDGE))
        height = top * grow
        if height > 0:
            shaft = [view(x - radius, y, 0), view(x + radius, y, 0), view(x + radius, y, height), view(x - radius, y, height)]
            d.add(_hatched(shaft, _d.CONCRETE, 0.6, DATA, _d.W_EDGE) if solid else _d.polygon(shaft, None, 0, DATA, _d.W_EDGE))
            d.add(self.dimension(view(x - radius, y, 0), view(x - radius, y, height), self.quantity(height, "m") + STAR, offset=0.32, color=DATA))
        d.add(self.level_mark(view(x - reach, y, slab_top), "+" + self.number(slab_top) + STAR, side=-1, length=0.3),
              self.level_mark(view(x + reach, y, 0), "±" + self.number(0), side=1, length=0.2))
        return d

    def _strip(self, f, c, point, scale, *, dims=True, supports=False, marks=True, ghost=True):
        """📏 Section along grid axis 3 from A to C through the first-floor slab and the pilotis A3, B3, C3; ``c`` collapses solids onto the mid-surface and axes."""
        x = f.b3["x"]
        column = sorted((p for p in f.g["pilotis"] if abs(p["x"] - x) < 1e-6 and p["y"] <= f.g["grid"]["y"][2] + 1e-6), key=lambda p: p["y"])
        top, thick = f.slab["top"], f.slab["thickness"]
        mid = top - thick / 2
        y0, y1 = 0.0, column[-1]["y"] + 0.8
        view = _d.View("section", point, scale, (x, (y0 + y1) / 2, top / 2), axis="y")
        d = _d.Drawing(view)

        def at(y, z):
            return view(x, y, z)

        ground = [at(y0 - 0.5, 0), at(y1, 0), at(y1, -0.4), at(y0 - 0.5, -0.4)]
        d.add(_d.polygon(ground, _d.GROUND_PLANE, 1.0, None), _d.hatch(ground, _d.GREEN_GROUND, 0.08, 45, _d.W_HAIR, 0.9), _d.segments([(ground[0], ground[1])], INK, _d.W_EDGE))
        lo, hi = top - thick + c * thick / 2, top - c * thick / 2
        for p in column:
            lit = p["tag"] == TAG
            colour = DATA if lit else STRUCT
            half, z_top = p["radius"] * (1 - c), (top - thick) * (1 - c) + mid * c
            if c < 0.98:
                post = [at(p["y"] - half, 0), at(p["y"] + half, 0), at(p["y"] + half, z_top), at(p["y"] - half, z_top)]
                d.add(_hatched(post, _d.CONCRETE, 0.6, colour, _d.W_EDGE, 0.05))
            else:
                d.add(_d.segments([(at(p["y"], 0), at(p["y"], mid))], colour, _d.W_CUT))
                d.add(_engine.Dot(at(p["y"], mid), radius=0.06 if lit else 0.045, color=colour))
            d.add(self.text(p["tag"], at(p["y"], -0.62), size=S_SMALL, color=colour))
            if supports:
                block = [at(p["y"] - 0.3, 0), at(p["y"] + 0.3, 0), at(p["y"] + 0.3, -0.22), at(p["y"] - 0.3, -0.22)]
                d.add(_hatched(block, colour, 0.35, colour, _d.W_THIN, 0.04, colour))
        if c < 0.98:
            body = [at(y0, lo), at(y1, lo), at(y1, hi), at(y0, hi)]
            d.add(_hatched(body, _d.CONCRETE, 0.35, INK, _d.W_EDGE, 0.05))
            if dims and c < 0.5:
                d.add(self.dimension(at(y0, lo), at(y0, hi), "t = " + self.quantity(thick, "m") + STAR, offset=0.3, color=DATA))
        else:
            d.add(_d.segments([(at(y0, mid), at(y1, mid))], DATA, _d.W_CUT))
            if ghost:
                d.add(_d.polyline([at(y0, top - thick), at(y1, top - thick), at(y1, top), at(y0, top)], HATCH, _d.W_THIN, 0.9, closed=True, dash=0.09))
                for p in column:
                    r = p["radius"]
                    d.add(_d.polyline([at(p["y"] - r, 0), at(p["y"] - r, top - thick), at(p["y"] + r, top - thick), at(p["y"] + r, 0)], HATCH, _d.W_THIN, 0.9, dash=0.07))
        if c > 0.5 and marks:
            d.add(_d.polyline([at(y0 - 0.5, mid), at(y1, mid)], DATA, _d.W_THIN, 0.8, dash=0.12))
            d.add(self.level_mark(at(y0 - 0.5, mid), "+" + self.number(mid, 3) + STAR, side=-1, length=0.3, color=DATA))
        if c <= 0.5 and marks:
            d.add(self.level_mark(at(y0 - 0.5, top), "+" + self.number(top), side=-1, length=0.3))
        d.at_y = at
        d.slab_y = at(y0, mid)[1]
        d.span = (at(y0, mid), at(y1, mid))
        return d

    def _beat_interface(self, f):
        """📨 Sender, schema and receiver with the packet B3 travelling between the two models."""
        left = self.villa_axon((-4.55, 0.0, 0), 0.11, model=f.m, highlight=(TAG,), disciplines=("architecture", "structure"), shell=0.5)
        right = self.villa_axon((4.55, 0.0, 0), 0.11, model=f.m, highlight=(TAG,), disciplines=("structure",), shell=0.5)
        names = _engine.VGroup(
            self.label("Sender · design model", "Sender · Entwurfsmodell", (-4.55, -1.95, 0), color=ARCH),
            self.label(f"{f.objects['design']} objects: architecture + structure", f"{f.objects['design']} Objekte: Architektur + Tragwerk", (-4.55, -2.3, 0), size=S_SMALL, color=MUTED),
            self.label("Receiver · structural check", "Empfänger · Tragwerksprüfung", (4.55, -1.95, 0), color=STRUCT),
            self.label(f"{f.objects['structure']} objects", f"{f.objects['structure']} Objekte", (4.55, -2.3, 0), size=S_SMALL, color=MUTED))
        schema = _engine.VGroup(
            self.badge(f.m["schema_id"], (0, 1.85, 0), color=STRUCT),
            self.text("ISO 16739-1:2024", (0, 1.4, 0), size=S_SMALL, color=MUTED),
            self.step_panel(_pick(f.excerpt, "^ISO-10303-21", "^FILE_SCHEMA"), (0, 0.55, 0), width=4.0),
            self.label("interface = shared schema", "Schnittstelle = gemeinsames Schema", (0, -0.2, 0), size=S_SMALL, color=GOOD))
        teaching = _engine.VGroup(self.badge(self.tr("Teaching model: tags and values simplified", "Lehrmodell: Bezeichnungen und Werte vereinfacht"), (0, -1.42, 0), color=MUTED, filled=False),
                                  self.label(f"pilotis: {len(f.g['pilotis'])} in this teaching model{STAR}", f"Pilotis: {len(f.g['pilotis'])} im Lehrmodell{STAR}", (0, -1.82, 0), size=S_SMALL, color=MUTED),
                                  self.label(f"{STAR} sources differ: 15–{len(f.g['pilotis'])}", f"{STAR} Quellen weichen ab: 15–{len(f.g['pilotis'])}", (0, -2.1, 0), size=S_SMALL, color=WARN))
        arrows = _engine.VGroup(_engine.Arrow((-2.9, 0.55, 0), (-1.95, 0.55, 0), buff=0, stroke_width=3, color=MUTED, max_tip_length_to_length_ratio=0.35),
                                _engine.Arrow((1.95, 0.55, 0), (2.9, 0.55, 0), buff=0, stroke_width=3, color=MUTED, max_tip_length_to_length_ratio=0.35))
        start, end = left.at(TAG), right.at(TAG)
        halos = _engine.VGroup(*[_engine.Circle(radius=0.16, color=DATA, stroke_width=3).move_to(point) for point in (start, end)])
        path = _engine.VMobject().set_points_smoothly([start, (-2.5, 0.9, 0), (0, 0.55, 0), (2.5, 0.9, 0), end])
        progress = _engine.ValueTracker(0)
        packet = self.badge(f"{TAG} · IfcColumn", start, color=DATA)
        packet.add_updater(lambda mob: mob.move_to(path.point_from_proportion(progress.get_value())))
        self.step("interface",
                  _seq((_engine.FadeIn(left), _engine.FadeIn(right), _engine.FadeIn(names), _engine.FadeIn(schema), _engine.FadeIn(arrows)),
                       (_engine.Create(halos[0]),),
                       (_engine.FadeIn(packet), _to(progress, 1, 2.6)),
                       (_engine.FadeOut(packet), _engine.Create(halos[1]), _engine.FadeIn(teaching))),
                  seconds=6.0)
        packet.clear_updaters()
        self._carry = _engine.VGroup(schema, arrows, teaching, names, halos)
        return left, right

    def _beat_semantic(self, f, left, right):
        """🧬 Class, place and stable identity: the IfcColumn line wired to the piloti in the plan crop, the site plan with its coordinates, and two exports of the same piloti."""
        x, y = f.b3["x"], f.b3["y"]
        needles = ("IFCCOLUMN(", f"IFCCARTESIANPOINT(({x:g}")
        panel_a = self.step_panel(_pick(f.excerpt, *needles), (-4.05, 1.4, 0), width=5.3)
        tag_a = self.label("file A", "Datei A", (-6.7, panel_a.frame.get_top()[1] + 0.2, 0), size=S_SMALL, color=MUTED, anchor="l")
        panel_b = self.step_panel(_pick(f.renumbered, *needles), (-4.05, -0.55, 0), width=5.3)
        tag_b = self.label("file B (next export)", "Datei B (nächster Export)", (-6.7, panel_b.frame.get_top()[1] + 0.2, 0), size=S_SMALL, color=MUTED, anchor="l")
        klass, place = _run(panel_a, "IFCCOLUMN(", "class"), _run(panel_a, "IFCCARTESIANPOINT(", "placement")
        guid_a, guid_b = _run(panel_a, "IFCCOLUMN(", "guid"), _run(panel_b, "IFCCOLUMN(", "guid")
        ref_a, ref_b = _run(panel_a, "IFCCOLUMN(", "number"), _run(panel_b, "IFCCOLUMN(", "number")
        bay = self._bay(f, (0.45, 0.4, 0), 1.0, 1.45)
        centre, rim = bay.centre, bay.radius

        def edge(angle):
            return centre + rim * np.array([math.cos(math.radians(angle)), math.sin(math.radians(angle)), 0.0])

        thumb = self.villa_plan("ground", (4.65, -0.75, 0), 0.09, model=f.m, highlight=(TAG,), thumbnail=True, halo=0.14)
        mark = thumb.at(x, y, 0)
        marker = _engine.Square(side_length=0.36, color=DATA, stroke_width=_d.W_EDGE).move_to(mark)
        callout = _d.segments(_d.dashes([bay.edge_right, mark + np.array([-0.18, 0.18, 0])], 0.1, 0.07) + _d.dashes([bay.frame_corners[1], mark + np.array([-0.18, -0.18, 0])], 0.1, 0.07), DATA, _d.W_THIN, 0.8)
        dim_x = self.dimension(thumb.at(0, 0, 0), thumb.at(x, 0, 0), self.quantity(x, "m"), offset=-0.26, color=INK)
        dim_y = self.dimension(thumb.at(f.g["box"]["length"], 0, 0), thumb.at(f.g["box"]["length"], y, 0), self.quantity(y, "m"), offset=-0.26, color=INK)
        guides = _d.segments(_d.dashes([mark, thumb.at(x, 0, 0)], 0.07, 0.05) + _d.dashes([mark, thumb.at(f.g["box"]["length"], y, 0)], 0.07, 0.05), INK, _d.W_HAIR, 0.8)
        chip_class = self.badge("IfcColumn · COLUMN", (2.45, 2.5, 0), color=ARCH, filled=False, anchor="l")
        modelling = self.label("(our modelling choice)", "(unsere Modellierung)", (2.5, 2.2, 0), size=S_SMALL, color=MUTED, anchor="l")
        chip_place = self.badge(self.tr(f"placement x = {self.number(x)} m, y = {self.number(y)} m", f"Platzierung x = {self.number(x)} m, y = {self.number(y)} m"), (2.45, 1.7, 0), color=INK, filled=False, anchor="l")
        chip_guid = self.badge(self.tr("GlobalId: stable identity", "GlobalId: stabile Identität"), (2.45, 1.1, 0), color=DATA, filled=False, anchor="l")
        local = self.label("instance number: local to this file", "Instanznummer: nur lokal in dieser Datei", (-4.05, -1.65, 0), size=S_SMALL, color=WARN)
        same = self.label("GlobalId: the same in both files", "GlobalId: in beiden Dateien gleich", (-4.05, -2.1, 0), size=S_SMALL, color=GOOD)
        rings = [_ring(klass, ARCH), _ring(place, INK), _ring(guid_a, DATA), _ring(guid_b, DATA), _ring(ref_a, WARN), _ring(ref_b, WARN)]
        wires = {"class": (_route(rings[0], panel_a, edge(125), ARCH, 0), _wire(edge(55), chip_class.get_left(), ARCH, dot=None)),
                 "place": (_route(rings[1], panel_a, edge(180), INK, 0), _wire(edge(5), chip_place.get_left(), INK, dot=None)),
                 "guid": (_route(rings[2], panel_a, edge(235), DATA, 1), _wire(edge(-50), chip_guid.get_left(), DATA, dot=None))}
        self.step("semantic",
                  _seq((_engine.FadeOut(left), _engine.FadeOut(right), _engine.FadeOut(self._carry), _in(bay), _in(thumb), _engine.FadeIn(panel_a), _engine.FadeIn(tag_a)),
                       (_engine.Create(rings[0]), _engine.Create(wires["class"][0]), _engine.FadeIn(chip_class), _engine.FadeIn(modelling), _engine.Create(wires["class"][1])),
                       (_engine.Create(rings[1]), _engine.Create(wires["place"][0]), _engine.FadeIn(chip_place), _engine.Create(wires["place"][1]),
                        _engine.Create(marker), _engine.Create(callout), _engine.Create(guides), _engine.Create(dim_x), _engine.Create(dim_y)),
                       (_engine.Create(rings[2]), _engine.Create(wires["guid"][0]), _engine.FadeIn(chip_guid), _engine.Create(wires["guid"][1])),
                       (_engine.FadeIn(panel_b), _engine.FadeIn(tag_b)),
                       (_engine.Create(rings[3]), ref_a.animate.set_color(WARN), ref_b.animate.set_color(WARN), _engine.Create(rings[4]), _engine.Create(rings[5]),
                        _engine.FadeIn(local), _engine.FadeIn(same))),
                  seconds=7.0)
        self._s = SimpleNamespace(off=[bay, thumb, marker, callout, guides, dim_x, dim_y, panel_a, tag_a, panel_b, tag_b, chip_class, modelling, chip_place, chip_guid, local, same,
                                       *rings, *[w for pair in wires.values() for w in pair]])

    def _beat_properties(self, f):
        """🧱 Type, LoadBearing flag, material, profile radius and extrusion depth, each wired from its STEP token to the drawn piloti."""
        s = self._s
        panel = self.step_panel(_pick(f.excerpt, "'LoadBearing'", "IFCPROPERTYSET(", "IFCCOLUMNTYPE(", "IFCCOLUMN(", "IFCEXTRUDEDAREASOLID(", "IFCMATERIAL(", "IFCCIRCLEPROFILEDEF("), (-4.1, 0.2, 0), width=5.3)
        thumb = self.villa_plan("ground", (6.05, 2.62, 0), 0.055, model=f.m, highlight=(TAG,), thumbnail=True, halo=0.1)
        grow, solid = _engine.ValueTracker(0), _engine.ValueTracker(0)
        scale, point = 0.85, (2.55, -0.3, 0)
        post = self.live(lambda: self._post(f, grow.get_value(), solid.get_value() > 0.5, point, scale), key=lambda: (round(grow.get_value(), 2), solid.get_value() > 0.5))
        view = _d.View("section", point, scale, (f.b3["x"], f.b3["y"], f.b3["top"] / 2), axis="x")
        x, y, radius, top = f.b3["x"], f.b3["y"], f.b3["radius"], f.b3["top"]
        tokens = {"load": _run(panel, "'LoadBearing'", "value"), "pset": _run(panel, "IFCPROPERTYSET(", "pset"), "via": _run(panel, "IFCCOLUMNTYPE(", "reference"), "type": _run(panel, "IFCCOLUMN(", "predefined"),
                  "depth": _run(panel, "IFCEXTRUDEDAREASOLID(", "depth"), "material": _run(panel, "IFCMATERIAL(", "material"), "profile": _run(panel, "IFCCIRCLEPROFILEDEF(", "profile")}
        colors = {"load": GOOD, "pset": GOOD, "via": GOOD, "type": ARCH, "depth": DATA, "material": STRUCT, "profile": INK}
        rings = {name: _ring(token, colors[name]) for name, token in tokens.items()}
        flag = f.element["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"]
        diameter = f.diameter
        centre = np.array([0.1, -1.1, 0.0])
        big = radius * 3.0
        section = _engine.VGroup(_hatched([centre + big * np.array([math.cos(t), math.sin(t), 0]) for t in np.linspace(0, 2 * math.pi, 48, endpoint=False)], _d.CONCRETE, 0.6, STRUCT, _d.W_EDGE, 0.05),
                                 _d.segments([(centre, centre + np.array([big, 0, 0]))], DATA, _d.W_EDGE), _engine.Dot(centre, radius=0.04, color=DATA))
        radius_label = self.label(f"r = {self.quantity(radius, 'm')}{STAR}", None, centre + np.array([0.0, -big - 0.75, 0]), size=S_SMALL, color=DATA)
        diameter_dim = self.dimension(centre + np.array([-big, -big, 0]), centre + np.array([big, -big, 0]), "Ø " + self.quantity(diameter["value"], "m") + STAR, offset=-0.2, color=DATA)
        arrow = _engine.Arrow(view(x, y, f.slab["top"]) + np.array([0, 0.95, 0]), view(x, y, f.slab["top"]), buff=0, stroke_width=4, color=GOOD, max_tip_length_to_length_ratio=0.3)
        key = self._legend((3.4, -2.35, 0), "c")
        slab_label = self.label("SL-1 · slab", "SL-1 · Decke", view(x + 0.45, y, f.b3["top"]) + np.array([0.0, -0.2, 0]), size=S_SMALL, color=MUTED, anchor="l")
        notes = {
            "type": _engine.VGroup(self.label("type COLUMN", "Typ COLUMN", (4.1, 0.6, 0), size=S_LABEL, color=ARCH, anchor="l"),
                                   self.label("IfcColumn · predefined type", "IfcColumn · vordefinierter Typ", (4.1, 0.3, 0), size=S_SMALL, color=MUTED, anchor="l", width=2.65)),
            "load": self._lines([(f"LoadBearing = {'TRUE' if flag else 'FALSE'}", GOOD), (self.tr("(teaching value)", "(Lehrwert)"), MUTED), (self.tr("meant to carry loads", "soll Lasten tragen"), MUTED), (self.tr("via type Piloti", "via Typ Piloti"), GOOD)], (4.1, 1.8, 0)),
            "material": _engine.VGroup(self.label(self._text(f.element["material"]["name"]), None, (4.1, -0.7, 0), size=S_LABEL, color=STRUCT, anchor="l"),
                                       self.label("IfcMaterial", None, (4.1, -1.0, 0), size=S_SMALL, color=MUTED, anchor="l"))}
        links = {"type": _route(rings["type"], panel, view(x - radius, y, 2.2), ARCH),
                 "load": _route(rings["load"], panel, arrow.get_start(), GOOD),
                 "pset": _route(rings["pset"], panel, arrow.get_start() + np.array([-0.04, 0.0, 0]), GOOD),
                 "depth": _route(rings["depth"], panel, view(x - radius - 0.32, y, top * 0.45), DATA),
                 "material": _route(rings["material"], panel, view(x - radius, y, 0.7), STRUCT),
                 "profile": _route(rings["profile"], panel, centre + np.array([-big, 0, 0]), INK)}
        right_links = {"type": _wire(view(x + radius, y, 2.2), notes["type"][0].get_left() + np.array([-0.08, 0, 0]), ARCH, dot=0),
                       "load": _wire(arrow.get_center() + np.array([0.04, 0.2, 0]), notes["load"][0].get_left() + np.array([-0.08, 0, 0]), GOOD, dot=0),
                       "material": _wire(view(x + radius, y, 0.7), notes["material"][0].get_left() + np.array([-0.08, 0, 0]), STRUCT, dot=0)}
        self.step("properties",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(panel), _in(thumb), _engine.FadeIn(post), _engine.FadeIn(slab_label), _engine.FadeIn(key)),
                       (_engine.Create(rings["type"]), _engine.Create(links["type"]), _engine.FadeIn(notes["type"]), _engine.Create(right_links["type"])),
                       (_engine.Create(rings["load"]), _engine.Create(rings["pset"]), _engine.Create(rings["via"]), _engine.Create(links["load"]), _engine.Create(links["pset"]), _engine.Create(arrow),
                        _engine.FadeIn(notes["load"]), _engine.Create(right_links["load"])),
                       (_engine.Create(rings["material"]), _engine.Create(links["material"]), _to(solid, 1, 0.4), _engine.FadeIn(notes["material"]), _engine.Create(right_links["material"])),
                       (_engine.Create(rings["profile"]), _engine.Create(links["profile"]), _engine.FadeIn(section), _engine.FadeIn(radius_label), _engine.FadeIn(diameter_dim)),
                       (_engine.Create(rings["depth"]), _engine.Create(links["depth"]), _to(grow, 1, 2.0))),
                  seconds=7.0)
        self._s = SimpleNamespace(panel=panel, off=[post, thumb, slab_label, key, section, radius_label, diameter_dim, arrow, panel, *rings.values(), *links.values(), *right_links.values(), *notes.values()])

    def _beat_mesh(self, f):
        """🔺 The same piloti as triangles: finer and finer tessellation, while class, identity, property and material are struck out."""
        s = self._s
        seg = _engine.ValueTracker(6)
        current = lambda: int(round(seg.get_value()))
        panel = self.step_panel(_pick(f.excerpt, "IFCCOLUMN(", "'LoadBearing'", "IFCMATERIAL("), (-4.1, 1.85, 0), width=5.3)
        panel.set_opacity(0.45)
        scale, centre = 1.1, (1.45, 0.3, 0)
        side = self.live(lambda: self.mesh_view(TAG, centre, scale, view="axon", segments=current(), counts=False, vertices=False, model=f.m), key=current)
        top = self.live(lambda: self.mesh_view(TAG, (4.75, 0.75, 0), 6.5, view="top", segments=current(), counts=False, model=f.m), key=current)
        axon = _d.View("axon", centre, scale, (f.b3["x"], f.b3["y"], f.b3["top"] / 2))
        height = self.dimension(axon(f.b3["x"], f.b3["y"], 0), axon(f.b3["x"], f.b3["y"], f.b3["top"]), self.quantity(f.q3["height"], "m") + STAR, offset=0.45, color=DATA)
        final = _model.mesh(TAG, 16, f.m)
        count = self.live_label(lambda: self.tr("{} vertices · {} triangles", "{} Punkte · {} Dreiecke").format(*[self.number(_model.mesh(TAG, current(), f.m)[k], 0) for k in ("vertex_count", "triangle_count")]),
                                (4.75, -0.75, 0), size=S_LABEL, color=DATA)
        volume = self.live_label(lambda: self.tr("mesh volume ", "Netzvolumen ") + self.quantity(_volume(_model.mesh(TAG, current(), f.m)), "m³", 4), (4.75, -1.2, 0), size=S_SMALL, color=INK)
        true = self.live_label(lambda: self.tr("true cylinder ", "echter Zylinder ") + self.quantity(f.q3["volume"], "m³", 4) + "  ("
                               + self._percent(_volume(_model.mesh(TAG, current(), f.m)), f.q3["volume"]) + " %)", (4.75, -1.55, 0), size=S_SMALL, color=MUTED)
        slivers = self.live(lambda: self._slivers(f, current(), (4.75, 0.75, 0), 6.5), key=current)
        sliver_note = self.label("orange: cut away from the true circle", "orange: vom echten Kreis abgeschnitten", (4.75, -1.95, 0), size=S_SMALL, color=WARN)
        triangles = self.live(lambda: self._file(current(), f), key=current)
        legend = self._lines([self.tr("v = vertex: x y z", "v = Punkt: x y z"), self.tr("f = triangle: 3 vertex numbers", "f = Dreieck: 3 Punktnummern")], (-3.45, -0.3, 0), MUTED)
        kept = self.label("kept: " + ", ".join(self._text(i) for i in final["kept"]), "bleibt: " + ", ".join(self._text(i) for i in final["kept"]), (4.75, 2.3, 0), size=S_SMALL, color=GOOD)
        lost_label = self.label("lost:", "verloren:", (-6.7, -1.65, 0), size=S_SMALL, color=WARN, anchor="l")
        lost, cursor, row = [], lost_label.get_right()[0] + 0.15, -1.65
        for item in final["lost"]:
            chip = self.badge(self._text(item), (cursor, row, 0), color=WARN, filled=False, anchor="l")
            if chip.get_right()[0] > -1.3:
                cursor, row = -6.7, row - 0.36
                chip.move_to(np.array([cursor, row, 0]) + np.array([chip.width / 2, 0, 0]))
            cursor = chip.get_right()[0] + 0.15
            lost.append(chip)
        strikes = [_strike(_run(panel, "IFCCOLUMN(", "class")), _strike(_run(panel, "IFCCOLUMN(", "guid")), _strike(_run(panel, "'LoadBearing'", "value")), _strike(_run(panel, "IFCMATERIAL(", "material"))]
        self.step("mesh",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(panel), _engine.FadeIn(side), _engine.FadeIn(top), _engine.FadeIn(slivers), _engine.FadeIn(sliver_note), _engine.FadeIn(count), _engine.FadeIn(height),
                        _engine.FadeIn(kept), _engine.FadeIn(triangles), _engine.FadeIn(legend), _engine.FadeIn(volume), _engine.FadeIn(true)),
                       (_to(seg, 16, 2.6), _engine.LaggedStart(*[_engine.Create(line) for line in strikes], lag_ratio=0.5, run_time=2.6)),
                       (_engine.FadeIn(lost_label), *[_engine.FadeIn(chip) for chip in lost])),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[panel, side, top, slivers, sliver_note, count, kept, height, triangles, legend, volume, true, lost_label, *lost, *strikes])

    def _slivers(self, f, segments, point, scale):
        """🍕 The parts of the true circle that the tessellation cuts away, as filled slivers between each chord and its arc, in the top view of ``mesh_view``."""
        data = _model.mesh(TAG, segments, f.m)
        cx, cy = data["centre"]
        radius = data["radius"]
        ring = [data["vertices"][i] for i in data["layout"]["bottom"]]
        group = _engine.VGroup()
        for a, b in zip(ring, ring[1:] + ring[:1]):
            t0, t1 = math.atan2(a[1] - cy, a[0] - cx), math.atan2(b[1] - cy, b[0] - cx)
            t1 += 2 * math.pi if t1 < t0 else 0.0
            arc = [(cx + radius * math.cos(t), cy + radius * math.sin(t)) for t in np.linspace(t0, t1, 6)]
            group.add(_d.polygon([_d.P(point) + scale * np.array([x - cx, y - cy, 0.0]) for x, y in arc], WARN, 0.9, WARN, 0.6))
        return group

    def _file(self, segments, f):
        """📄 The start of the exported mesh file: three vertices and the first two triangles, numbers only."""
        data = _model.mesh(TAG, segments, f.m)
        rows = [f"v {x:.3f} {y:.3f} {z:.3f}" for x, y, z in data["vertices"][:3]] + [f"f {a + 1} {b + 1} {c + 1}" for a, b, c in data["faces"][:2]]
        text = _engine.VGroup(*[self.mono(row, (0, 0, 0), size=S_SMALL, color=INK if row[0] in "vf" else MUTED) for row in rows]).arrange(_engine.DOWN, aligned_edge=_engine.LEFT, buff=0.06)
        text.move_to(np.array([-5.3, -0.4, 0]))
        box = _engine.SurroundingRectangle(text, color=MUTED, buff=0.12, corner_radius=0.05, stroke_width=_d.W_THIN)
        return _engine.VGroup(text, box)

    def _beat_purpose(self, f):
        """🎯 The slab in the plan, the chosen analysis target and the collapse of the section along axis 3 to the mid-surface."""
        s = self._s
        analysis = f.analysis
        plan = self.villa_plan("first", (-4.2, 1.1, 0), 0.19, model=f.m, highlight=(SLAB, TAG), layers=("structure", "architecture"), grid=False, labels=False)
        cut = _engine.VGroup(_d.polyline([plan.at(f.b3["x"], -0.4, 0), plan.at(f.b3["x"], f.g["box"]["width"] + 0.4, 0)], DATA, _d.W_THIN, 0.8, dash=0.14),
                             _d.segments([(plan.at(f.b3["x"], 0, 0), plan.at(f.b3["x"], f.g["grid"]["y"][2], 0))], DATA, _d.W_CUT))
        purpose = self.label("Purpose: structural check of the first-floor slab", "Zweck: statische Prüfung der Geschossdecke", (-6.7, -1.2, 0), size=S_SMALL, color=INK, anchor="l")
        target = _engine.VGroup(
            self.label("Target: structural analysis model", "Ziel: Tragwerksanalysemodell", (-6.7, -1.55, 0), size=S_SMALL, color=STRUCT, anchor="l"),
            self.label("IFC structural analysis domain (schema section)", "IFC-Domäne Tragwerksanalyse (Schemabereich)", (-6.7, -1.9, 0), size=S_SMALL, color=MUTED, anchor="l"))
        slab = f.slab
        facts = self.label(f"{SLAB} · {self.quantity(f.g['box']['length'], 'm', 1)} × {self.quantity(f.g['box']['width'], 'm', 1)} · net {self.quantity(f.qs['area'], 'm²', 1)}{STAR}",
                           f"{SLAB} · {self.quantity(f.g['box']['length'], 'm', 1)} × {self.quantity(f.g['box']['width'], 'm', 1)} · netto {self.quantity(f.qs['area'], 'm²', 1)}{STAR}",
                           (-6.7, -2.3, 0), size=S_SMALL, color=DATA, anchor="l")
        collapse = _engine.ValueTracker(0)
        strip = self.live(lambda: self._strip(f, collapse.get_value(), (3.45, -0.15, 0), 0.6), key=lambda: round(collapse.get_value(), 2))
        title = self.label("section along axis 3, A to C", "Schnitt entlang Achse 3, A bis C", (3.45, 2.45, 0), size=S_SMALL, color=MUTED)
        kept = self.label(f"thickness t = {self.quantity(f.thickness['value'], 'm')}{STAR} stays an attribute", f"Dicke t = {self.quantity(f.thickness['value'], 'm')}{STAR} bleibt Attribut", (3.45, 1.95, 0), size=S_SMALL, color=DATA)
        mid = self.label("mid-surface", "Mittelfläche", (4.95, 1.0, 0), size=S_SMALL, color=DATA)
        key = self._legend((3.45, -2.3, 0), "c")
        self.step("purpose",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(plan), _engine.FadeIn(cut), _engine.FadeIn(purpose), _engine.FadeIn(facts), _engine.FadeIn(strip), _engine.FadeIn(title), _engine.FadeIn(key)),
                       (_engine.FadeIn(target),),
                       (_to(collapse, 1, 2.4),),
                       (_engine.FadeIn(kept), _engine.FadeIn(mid))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[plan, cut, purpose, target, facts, strip, title, kept, mid, key])

    def _beat_mapping(self, f):
        """🗺️ The slab becomes shell elements on the grid, pilotis become members; source GlobalIds are recorded."""
        s = self._s
        analysis = f.analysis
        rows = []
        for rule in analysis["mapping"]:
            if rule["kind"] == "support":
                continue
            source = rule["source_class"] + (" " + rule["source_predefined"] if rule.get("source_predefined") else "")
            noun = self.tr("shell elements", "Schalenelemente") if rule["kind"] == "surface" else self.tr("members", "Stäbe")
            rows += [f"{source} → {rule['target_class']}", "    " + f"{rule['count']}{STAR if rule['kind'] == 'curve' else ''} {noun}"]
        counts = analysis["counts"]
        rows.append(self.tr(f"{counts['slab_nodes']} slab nodes at z = {self.number(analysis['mid_surface']['z'], 3)} m{STAR}", f"{counts['slab_nodes']} Deckenknoten bei z = {self.number(analysis['mid_surface']['z'], 3)} m{STAR}"))
        rows.append(self.tr("source GlobalId → recorded per item", "Quell-GlobalId → bei jedem Objekt vermerkt"))
        table = self.class_box(self.tr("Architecture model → analysis model", "Architekturmodell → Analysemodell"), rows, (-3.9, 1.25, 0), color=STRUCT, width=5.7)
        plan = self.villa_plan("first", (3.4, 0.5, 0), 0.26, model=f.m, layers=("structure",), grid=False, labels=False)
        view = self.analysis_view(analysis, (3.4, 0.5, 0), 0.26, supports=False, highlight=(TAG,), model=f.m)
        void = f.slab["voids"][0]
        void_label = self.label(self.quantity(analysis['slab']['void_area'], 'm²', 1) + STAR, None, view.at(8.8, 7.1, 3.375), size=S_SMALL, color=WARN)
        void_label.rotate(math.pi / 2).set_stroke(_d.BACKGROUND, 5.0, 0.85, background=True)
        hole = _engine.VGroup(_d.polyline([view.at(px, py, 3.375) for px, py in void], WARN, _d.W_EDGE, 1.0, closed=True, dash=0.1), void_label)
        chip_slab = self.guid_chip(SLAB, (-3.9, -1.7, 0), model=f.m)
        chip_b3 = self.guid_chip(TAG, (-3.9, -0.8, 0), model=f.m)
        shell = next(sh for sh in analysis["shells"] if sh["id"] == "SH-A1")
        links = _engine.VGroup(_wire(chip_b3.get_right(), view.at(TAG), DATA), _wire(chip_slab.get_right(), view.at(shell["id"]), DATA))
        note = self.label(f"{counts['shells']} shells · {counts['slab_nodes']} nodes · {counts['curve_members']}{STAR} members", f"{counts['shells']} Schalen · {counts['slab_nodes']} Knoten · {counts['curve_members']}{STAR} Stäbe",
                          (3.4, -2.15, 0), size=S_SMALL, color=MUTED)
        note_void = self.label("ramp void not meshed", "Rampenöffnung nicht vernetzt", (3.4, -2.42, 0), size=S_SMALL, color=WARN)
        key = _engine.VGroup(self._legend((-6.7, -2.22, 0)), self._count_legend(f, (-6.7, -2.47, 0)))
        self.step("mapping",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(plan), _engine.FadeIn(table)),
                       (_engine.FadeOut(plan), _engine.FadeIn(view), _engine.FadeIn(hole)),
                       (_engine.FadeIn(chip_slab), _engine.FadeIn(chip_b3), _engine.Create(links), _engine.FadeIn(note), _engine.FadeIn(note_void), _engine.FadeIn(key))),
                  seconds=6.0)
        self._s = SimpleNamespace(off=[table, view, hole, chip_slab, chip_b3, links, note, note_void, key])

    def _beat_assumptions(self, f):
        """⚖️ Self-weight from the thickness, finishes, imposed load and support conditions as stacked arrows on the idealised slab strip."""
        s = self._s
        loads = f.trib["load_breakdown"]
        support = f.analysis["supports"][0]
        level = {key: _engine.ValueTracker(0) for key, _ in LOAD_COLORS}
        weight = f.analysis["loads"]["self_weight"]
        rows = [(self.tr("Self-weight", "Eigengewicht"), weight["value"], "kN/m²", weight["basis"]),
                (self.tr("Finishes", "Ausbaulasten"), loads["finishes"]["value"], "kN/m²", loads["finishes"]["basis"]),
                (self.tr("Imposed load", "Nutzlast"), loads["imposed"]["value"], "kN/m²", loads["imposed"]["basis"]),
                (self.tr("Total q", "Summe q"), loads["total"]["value"], "kN/m²", loads["total"]["basis"]),
                (self.tr("Support", "Auflager"), self._text(support["label"]).split(" (")[0], "", support["basis"])]
        table = self.property_table(rows, (-3.75, 1.0, 0), width=5.6)
        formula = self.label(f"self-weight = {self.number(weight['density'], 0)} kN/m³ × {self.number(weight['thickness'])} m", f"Eigengewicht = {self.number(weight['density'], 0)} kN/m³ × {self.number(weight['thickness'])} m", (-6.55, -0.2, 0), size=S_SMALL, color=MUTED, anchor="l")
        swatches = _engine.VGroup(*[_engine.Square(side_length=0.14, fill_color=color, fill_opacity=1, stroke_width=0).move_to((-6.72, table.rows[i].name.get_center()[1], 0)) for i, (_, color) in enumerate(LOAD_COLORS)])
        strip = self._strip(f, 1.0, (3.45, -0.15, 0), 0.6, supports=True, marks=False, ghost=False)
        ys = np.linspace(0.4, 9.5 + 0.2, 12)
        order = [key for key, _ in LOAD_COLORS]
        base = strip.at_y(0, f.slab["top"] - f.slab["thickness"] / 2)[1] + 0.05
        arrows = _engine.VGroup(*[self.live(lambda i=i, key=key, color=color: self.load_arrows(
            [level[key].get_value()] * len(ys), [(strip.at_y(y, 0)[0], base + sum(level[k].get_value() for k in order[:i]) * Q_STACK, 0) for y in ys],
            Q_STACK, anchor="tip", unit="kN", per="m²", colors=color, labels=False, width=2.4) if level[key].get_value() > 0.001 else _engine.VGroup(), key=lambda key=key: round(level[key].get_value(), 2))
            for i, (key, color) in enumerate(LOAD_COLORS)])
        total = self.live_label(lambda: "q = " + self.quantity(sum(level[key].get_value() for key in order), "kN/m²"), (3.45, 2.5, 0), size=S_LABEL, color=DATA)
        plan = self.villa_plan("first", (-4.0, -1.45, 0), 0.085, model=f.m, highlight=(TAG,), thumbnail=True, halo=0.1)
        cut = _engine.VGroup(_d.segments([(plan.at(f.b3["x"], 0, 0), plan.at(f.b3["x"], f.g["grid"]["y"][2], 0))], DATA, _d.W_CUT),
                             self.label("A3 – C3", None, plan.at(f.b3["x"], 0, 0) + np.array([0, -0.22, 0]), size=S_SMALL, color=DATA))
        support_label = self.label(f"support at the feet: {self._text(support['label']).split(' (')[0]} ({self._basis(support)})",
                                   f"Auflager an den Fußpunkten: {self._text(support['label']).split(' (')[0]} ({self._basis(support)})", (3.45, -2.3, 0), size=S_SMALL, color=STRUCT)
        thickness = self.label(f"t = {self.quantity(f.thickness['value'], 'm')} ({self._basis(f.thickness)})", None, (4.62, strip.slab_y - 0.28, 0), size=S_SMALL, color=DATA)
        self.step("assumptions",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(strip), _engine.FadeIn(table), _engine.FadeIn(swatches), _engine.FadeIn(formula), _engine.FadeIn(arrows), _engine.FadeIn(total), _engine.FadeIn(thickness), _in(plan), _engine.FadeIn(cut)),
                       *[(_to(level[key], loads[key]["value"], 1.4),) for key in order],
                       (_engine.FadeIn(support_label),)),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[table, swatches, formula, strip, arrows, total, support_label, thickness, plan, cut])

    def _load_path(self, f, entry, k, point):
        """⬇️ One piloti with its tributary cell in the axonometric: the cell grows by ``k``, load arrows stand on it, and the reaction arrow enters the foot."""
        ring = entry.get("cell") or entry["pieces"][0]
        x, y = entry["x"], entry["y"]
        z = f.analysis["mid_surface"]["z"]
        grown = [[x + (px - x) * math.sqrt(k), y + (py - y) * math.sqrt(k)] for px, py in ring]
        view = _d.View("axon", point, 0.27, (x, y, z / 2))
        d = _d.Drawing(view)
        colour = DATA
        d.add(_d.segments([(view(x, y, 0), view(x, y, z))], INK, _d.W_CUT))
        d.add(_hatched([view(px, py, z) for px, py in grown], colour, 0.28, colour, _d.W_EDGE, 0.075, colour))
        spots = [[x + i * 1.4, y + j * 1.4] for i in range(-4, 5) for j in range(-4, 5) if _inside([x + i * 1.4, y + j * 1.4], grown)]
        q = f.trib["area_load"]
        if spots and k > 0.02:
            d.add(self.load_arrows([q] * len(spots), [view(px, py, z) for px, py in spots], Q_SCALE, anchor="tip", colors=colour, labels=False, width=2.0))
        reaction = k * entry["reaction"]
        d.add(self.load_arrows([reaction], [view(x, y, 0)], R_SCALE, direction=(0, 1), anchor="tip", colors=colour, labels=False, width=3.2))
        d.add(_engine.Dot(view(x, y, z), radius=0.05, color=INK))
        return d

    def _beat_reaction(self, f):
        """📐 Tributary areas grow around three pilotis while their load arrows and reaction arrows lengthen."""
        s = self._s
        trib = f.trib
        by_tag = {p["tag"]: p for p in trib["pilotis"]}
        mine = by_tag[TAG]
        edge = next(p for p in trib["pilotis"] if p["kind"] == "edge" and p["column"] == mine["column"])
        corner = next(p for p in trib["pilotis"] if p["kind"] == "corner")
        chosen = (TAG, edge["tag"], corner["tag"])
        view = self.tributary_view(trib, (-4.1, 0.25, 0), 0.2, highlight=chosen, labels=False, model=f.m)
        tag_at = lambda t: view.at(by_tag[t]["x"], by_tag[t]["y"], 0)
        areas = _engine.VGroup(*[self.text(t, tag_at(t) + np.array([0, -0.28, 0]), size=S_SMALL, color=DATA) for t in chosen if t != TAG],
                               self.text(TAG, np.array([tag_at(TAG)[0], 2.45, 0]), size=S_SMALL, color=DATA), _d.segments([(np.array([tag_at(TAG)[0], 2.3, 0]), tag_at(TAG) + np.array([0, 0.08, 0]))], DATA, _d.W_THIN, 0.9))
        legend = self.tributary_legend((-4.1, -2.2, 0), kinds=("interior", "edge", "corner"), size=S_SMALL, gap=0.25)
        legend_note = self.label("lit cells: areas are listed on the right", "helle Zellen: Flächen stehen rechts", (-4.1, -2.5, 0), size=S_SMALL, color=MUTED)
        k = _engine.ValueTracker(0)
        xs = (0.8, 3.2, 5.6)
        paths = [self.live(lambda t=t, x=x: self._load_path(f, by_tag[t], k.get_value(), (x, 0.45, 0)), key=lambda: round(k.get_value(), 2)) for t, x in zip(chosen, xs)]
        heads = [self.label(f"{t} · " + self.tr(*{"interior": ("interior", "innen"), "edge": ("edge", "Rand"), "corner": ("corner", "Ecke")}[by_tag[t]["kind"]]), None, (x, -1.35, 0), size=S_SMALL, color=INK)
                 for t, x in zip(chosen, xs)]
        notes = [self.live_label(lambda t=t: self.quantity(k.get_value() * by_tag[t]["area"], "m²", 2) + " × " + self.quantity(trib["area_load"], "kN/m²"), (x, -1.7, 0), size=S_SMALL, color=INK)
                 for t, x in zip(chosen, xs)]
        forces = [self.live_label(lambda t=t: f"R = {self.quantity(k.get_value() * by_tag[t]['reaction'], 'kN', 0)}", (x, -2.05, 0), size=S_SMALL, color=DATA)
                  for t, x in zip(chosen, xs)]
        forces.append(self.label("R = reaction force at the piloti foot", "R = Auflagerkraft am Stützenfuß", (3.2, -2.4, 0), size=S_SMALL, color=MUTED))
        check = self.label(f"load check (estimate): Σ R = {self.quantity(trib['total_reaction'], 'kN', 0)} = q × {self.quantity(trib['slab_area'], 'm²', 1)}",
                           f"Lastprüfung (Schätzung): Σ R = {self.quantity(trib['total_reaction'], 'kN', 0)} = q × {self.quantity(trib['slab_area'], 'm²', 1)}", (2.95, 2.75, 0), size=S_SMALL, color=GOOD)
        estimate = self.badge(self.tr("estimate, no finite element calculation", "Schätzung, keine FE-Berechnung"), (2.95, 2.4, 0), color=WARN, filled=False)
        self.step("reaction",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(view), _engine.FadeIn(areas), _engine.FadeIn(legend), _engine.FadeIn(legend_note), _engine.FadeIn(estimate), *[_engine.FadeIn(p) for p in paths], *[_engine.FadeIn(n) for n in heads + notes + forces]),
                       (_to(k, 1, 3.0),),
                       (_engine.FadeIn(check), _engine.Indicate(view.parts[TAG], color=DATA))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[view, areas, legend, legend_note, estimate, check, *paths, *heads, *notes, *forces])

    def _returned(self, f, u, point):
        """↩️ The piloti that comes back from the mesh: the classified colours fade to grey triangles as ``u`` runs from 0 to 1."""
        drawing = self.mesh_view(TAG, point, 0.8, view="axon", segments=16, counts=False, vertices=False, model=f.m)
        grey = _d.mix(MUTED, _d.BACKGROUND, 0.62)
        for shape in drawing.triangles:
            shape.set_fill(_d.mix(shape.get_fill_color().to_hex(), grey, u), 1.0)
            shape.set_stroke(_d.mix(INK, MUTED, u), _d.W_THIN, 0.5)
        return _engine.VGroup(drawing)

    def _beat_roundtrip(self, f):
        """🔁 IFC to a triangle mesh and back: the drawn piloti returns as grey triangles and class, GlobalId, LoadBearing, material and type stay lost."""
        s = self._s
        data = _model.mesh(TAG, 16, f.m)
        u = _engine.ValueTracker(0)
        cx = (-4.75, -0.15, 4.5)
        kept = [f"{self._text(item)}: {self.number(data['vertex_count' if item['id'] == 'vertices' else 'triangle_count'], 0)}" for item in data["kept"]]
        lost = [self._text(item) for item in data["lost"]]
        titles = _engine.VGroup(self.label("IFC file", "IFC-Datei", (cx[0], 2.65, 0), color=STRUCT), self.label("triangle mesh file", "Dreiecksnetz-Datei", (cx[1], 2.65, 0), color=MUTED),
                                self.label("IFC again, read from the mesh", "IFC erneut, aus dem Netz gelesen", (cx[2], 2.65, 0), color=WARN, width=3.2))
        original = self._post(f, 1.0, True, (cx[0], 0.85, 0), 0.62)
        middle = self.mesh_view(TAG, (cx[1], 0.85, 0), 0.8, view="axon", segments=16, counts=False, vertices=False, model=f.m)
        back = self.live(lambda: self._returned(f, u.get_value(), (cx[2], 0.85, 0)), key=lambda: round(u.get_value(), 2))
        rows_a = self._lines([(text, GOOD) for text in lost], (cx[0] - 1.45, -0.75, 0))
        rows_b = _engine.VGroup(self._lines([(text, GOOD) for text in kept], (cx[1] - 1.45, -0.75, 0)), self._lines([(text, MUTED) for text in lost], (cx[1] - 1.45, -0.75 - 0.27 * len(kept), 0)))
        strikes_b = _engine.VGroup(*[_strike(label) for label in rows_b[1]])

        def rows_c():
            group = _engine.VGroup(self._lines([(text, GOOD) for text in kept], (cx[2] - 1.45, -0.75, 0)))
            tail = self._lines([(text, _d.mix(GOOD, MUTED, u.get_value())) for text in lost], (cx[2] - 1.45, -0.75 - 0.27 * len(kept), 0))
            group.add(tail, *[_strike(label).set_stroke(opacity=u.get_value()) for label in tail])
            return group

        returned = self.live(rows_c, key=lambda: round(u.get_value(), 2))
        change = self._percent(_volume(data), f.q3["volume"])
        shape = self.label(f"shape returns roughly: volume {change} %", f"Form grob zurück, Volumen {change} %", (cx[2], 2.3, 0), size=S_SMALL, color=MUTED, width=3.3)
        inset = _engine.VGroup(self.mesh_view(TAG, (cx[2] + 1.4, 1.25, 0), 2.4, view="top", counts=False, vertices=False, model=f.m), self._slivers(f, 16, (cx[2] + 1.4, 1.25, 0), 2.4))
        key = self._legend((cx[0] - 1.45, -2.35, 0))
        counts = self.label(self.tr(f"{data['vertex_count']} vertices · {data['triangle_count']} triangles", f"{data['vertex_count']} Punkte · {data['triangle_count']} Dreiecke"), None, (cx[1], 2.3, 0), size=S_SMALL, color=DATA)
        export = _engine.VGroup(_engine.Arrow((-3.1, 0.85, 0), (-1.65, 0.85, 0), buff=0, stroke_width=3, color=GOOD, max_tip_length_to_length_ratio=0.2), self.label("export", "Export", (-2.37, 1.13, 0), size=S_SMALL, color=GOOD))
        reimport = _engine.VGroup(_engine.Arrow((1.6, 0.85, 0), (3.05, 0.85, 0), buff=0, stroke_width=3, color=WARN, max_tip_length_to_length_ratio=0.2), self.label("import", "Import", (2.32, 1.13, 0), size=S_SMALL, color=WARN))
        self.step("roundtrip",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(titles), _in(original), _engine.FadeIn(rows_a), _engine.FadeIn(key)),
                       (_engine.FadeIn(export), _in(middle), _engine.FadeIn(counts), _engine.FadeIn(rows_b)),
                       (_engine.Create(strikes_b), _engine.FadeIn(reimport), _engine.FadeIn(back), _engine.FadeIn(returned), _in(inset)),
                       (_to(u, 1, 2.4), _engine.FadeIn(shape))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[titles, original, middle, back, rows_a, rows_b, strikes_b, returned, shape, counts, export, reimport, inset, key])

    def _beat_slab(self, f):
        """🗺️ The first-floor slab in plan with its real ramp opening and the salon named, and the net area counted from gross minus opening."""
        s = self._s
        slab = f.analysis["slab"]
        v = _engine.ValueTracker(0)
        plan = self.villa_plan("first", (2.5, 0.05, 0), 0.22, model=f.m, highlight=(SLAB,), layers=("structure",), grid=True, labels=True)
        void = f.slab["voids"][0]
        screen = [plan.at(px, py, 0) for px, py in void]
        middle = np.mean(screen, axis=0)

        def opening():
            if v.get_value() < 0.02:
                return _engine.VGroup()
            return _hatched([middle + (p - middle) * v.get_value() for p in screen], WARN, 0.35, WARN, _d.W_EDGE, 0.07, WARN)

        shape = self.live(opening, key=lambda: round(v.get_value(), 2))
        rooms = [sp for sp in f.g["spaces"] if sp["storey"] == f.slab_storey]
        rings = {sp["tag"]: [p for p in next(m for m in f.m["spaces"] if m["tag"] == sp["tag"])["outline"] if isinstance(p, (list, tuple)) and len(p) == 2] for sp in rooms}
        rooms = [sp for sp in rooms if len(rings[sp["tag"]]) >= 3]
        tones = [GOOD, ARCH, STRUCT]
        lawn = _engine.VGroup(*[_hatched([plan.at(px, py, 0) for px, py in rings[sp["tag"]]], tones[i], 0.1, tones[i], _d.W_THIN, 0.11, tones[i]) for i, sp in enumerate(rooms)])
        reach = f.trib["cantilever"]["west"]
        width = f.g["box"]["width"]
        dim = _engine.VGroup(self.dimension(plan.at(0, width, 0), plan.at(reach, width, 0), self.quantity(reach, "m") + STAR, offset=0.2, color=INK),
                             self.dimension(plan.at(f.g["box"]["length"] - reach, width, 0), plan.at(f.g["box"]["length"], width, 0), self.quantity(reach, "m") + STAR, offset=0.2, color=INK))
        chip = self.guid_chip(SLAB, (-4.0, 2.4, 0), model=f.m)
        gross = self.label(self.tr(f"gross {self.quantity(slab['gross_area'], 'm²', 1)}", f"brutto {self.quantity(slab['gross_area'], 'm²', 1)}"), None, (-6.7, 1.55, 0), size=S_LABEL, color=INK, anchor="l")
        minus = self.live_label(lambda: "− " + self.tr("ramp opening ", "Rampenöffnung ") + self.quantity(v.get_value() * slab["void_area"], "m²", 1) + STAR, (-6.7, 1.1, 0), size=S_LABEL, color=WARN, anchor="l")
        net = self.live_label(lambda: "= " + self.tr("net ", "netto ") + self.quantity(slab["gross_area"] - v.get_value() * slab["void_area"], "m²", 1) + STAR, (-6.7, 0.65, 0), size=S_LABEL, color=DATA, anchor="l")
        names = _engine.VGroup(
            *[self.label(f"{sp['tag']} · {self._text(sp['name'])}, {self.quantity(sp['area'], 'm²', 1)}{STAR}", None, (-6.7, -0.2 - 0.36 * i, 0), size=S_SMALL, color=tones[i], anchor="l", width=4.6) for i, sp in enumerate(rooms)],
            self.label(f"{f.slab['void_tags'][0]} · " + self.tr("ramp opening in the slab", "Rampenöffnung in der Decke"), None, (-6.7, -0.2 - 0.36 * len(rooms), 0), size=S_SMALL, color=WARN, anchor="l"))
        cantilever = self.label(self.tr(f"cantilever {self.quantity(reach, 'm')}{STAR} west and east", f"Kragarm {self.quantity(reach, 'm')}{STAR} West und Ost"), None, (2.5, 2.85, 0), size=S_SMALL, color=INK)
        key = self._legend((-6.7, -0.2 - 0.36 * (len(rooms) + 1) - 0.05, 0))
        links = _engine.VGroup(*[_wire((-2.05, -0.2 - 0.36 * i, 0), np.mean([plan.at(px, py, 0) for px, py in rings[sp["tag"]]], axis=0), tones[i]) for i, sp in enumerate(rooms)],
                               _wire((-2.05, -0.2 - 0.36 * len(rooms), 0), middle + np.array([0, -0.5, 0]), WARN))
        self.step("slab",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(plan), _engine.FadeIn(lawn), _engine.FadeIn(chip), _engine.FadeIn(gross), _engine.FadeIn(minus), _engine.FadeIn(net)),
                       (_engine.FadeIn(names), _engine.FadeIn(key), _engine.Create(links), _engine.FadeIn(dim), _engine.FadeIn(cantilever)),
                       (_engine.FadeIn(shape), _to(v, 1, 2.4))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[plan, lawn, chip, gross, minus, net, names, key, links, dim, shape, cantilever])

    def _beat_buildup(self, f):
        """🥞 Section through the first-floor slab build-up: self-weight, finishes and imposed load are stacked into one area load before they meet the tributary areas."""
        s = self._s
        loads = f.analysis["loads"]
        mine = next(p for p in f.trib["pilotis"] if p["tag"] == TAG)
        order = [key for key, _ in LOAD_COLORS]
        level = {key: _engine.ValueTracker(0) for key in order}
        top, thick = f.slab["top"], f.slab["thickness"]
        view = _d.View("section", (-3.25, 0.4, 0), 2.2, (f.b3["x"], f.b3["y"], top - thick / 2), axis="y")
        y0, y1 = f.b3["y"] - 0.95, f.b3["y"] + 0.95
        skin = 0.05
        at = lambda y, z: view(f.b3["x"], y, z)
        body = _hatched([at(y0, top - thick), at(y1, top - thick), at(y1, top), at(y0, top)], _d.CONCRETE, 0.35, STRUCT, _d.W_EDGE, 0.05)
        finish = _d.polygon([at(y0, top), at(y1, top), at(y1, top + skin), at(y0, top + skin)], ARCH, 0.55, ARCH, _d.W_EDGE)
        spots = np.linspace(y0 + 0.3, y1 - 0.3, 4)
        people = _engine.VGroup(*[_engine.Circle(radius=0.05, stroke_color=DATA, stroke_width=_d.W_EDGE, fill_opacity=1, fill_color=DATA).move_to(at(y, top + skin) + np.array([0, 0.28, 0])) for y in spots])
        stems = _d.segments([(at(y, top + skin) + np.array([0, 0.03, 0]), at(y, top + skin) + np.array([0, 0.23, 0])) for y in spots], DATA, _d.W_EDGE)
        post = _hatched([at(f.b3["y"] - f.b3["radius"], top - thick - 0.4), at(f.b3["y"] + f.b3["radius"], top - thick - 0.4), at(f.b3["y"] + f.b3["radius"], top - thick), at(f.b3["y"] - f.b3["radius"], top - thick)], _d.CONCRETE, 0.6, DATA, _d.W_EDGE, 0.05)
        floor = _engine.VGroup(post, self.text(self.tr("section through SL-1 (axis 3, at B3)", "Schnitt durch SL-1 (Achse 3, bei B3)"), (-3.25, 2.2, 0), size=S_SMALL, color=MUTED), self.text(TAG, at(f.b3["y"], top - thick - 0.4) + np.array([0, -0.2, 0]), size=S_SMALL, color=DATA),
                               self.dimension(at(y0, top - thick), at(y0, top), "t = " + self.quantity(thick, "m") + STAR, offset=0.35, color=DATA),
                               _d.segments([(at(y0 - 0.3, top - thick), at(y1 + 0.3, top - thick))], INK, _d.W_THIN, 0.5))
        marks = _engine.VGroup(self.label(self.tr(f"self-weight {self.number(loads['self_weight']['density'], 0)} kN/m³ × {self.number(thick)} m", f"Eigengewicht {self.number(loads['self_weight']['density'], 0)} kN/m³ × {self.number(thick)} m"),
                                          None, (-3.25, -1.35, 0), size=S_SMALL, color=STRUCT),
                               self.label("finishes (layer thickness not modelled)", "Ausbaulasten (Schichtdicke nicht modelliert)", (-3.25, -1.75, 0), size=S_SMALL, color=ARCH),
                               self.label("imposed load: people and furniture", "Nutzlast: Personen und Möbel", (-3.25, -2.15, 0), size=S_SMALL, color=DATA))
        names = {"self_weight": self.tr("Self-weight", "Eigengewicht"), "finishes": self.tr("Finishes", "Ausbaulasten"), "imposed": self.tr("Imposed load", "Nutzlast")}
        unit, base, height = 0.5, 1.25, 0.5

        def stack():
            group, x = _engine.VGroup(), 0.9
            for key, color in LOAD_COLORS:
                width = level[key].get_value() * unit
                if width > 0.01:
                    group.add(_d.polygon([(x, base, 0), (x + width, base, 0), (x + width, base + height, 0), (x, base + height, 0)], color, 0.85, color, _d.W_EDGE))
                    if width > 0.55:
                        group.add(self.text(self.number(level[key].get_value()), (x + width / 2, base + height / 2, 0), size=S_SMALL, color=_d.BACKGROUND))
                x += width
            return group

        bar = self.live(stack, key=lambda: tuple(round(level[k].get_value(), 2) for k in order))
        total = lambda: sum(level[k].get_value() for k in order)
        equation = self.live_label(lambda: " + ".join(self.number(level[k].get_value()) for k in order) + " = " + self.quantity(total(), "kN/m²"), (3.6, 2.2, 0), size=S_LABEL, color=DATA)
        legend = _engine.VGroup(*[_engine.VGroup(_engine.Square(side_length=0.14, fill_color=color, fill_opacity=1, stroke_width=0).move_to((0.95, 0.75 - 0.35 * i, 0)),
                                                 self.text(names[key], (1.2, 0.75 - 0.35 * i, 0), size=S_SMALL, color=INK, anchor="l")) for i, (key, color) in enumerate(LOAD_COLORS)])
        times = self.live_label(lambda: self.quantity(total(), "kN/m²") + " × " + self.quantity(mine["area"], "m²", 2) + " = " + self.quantity(total() * mine["area"], "kN", 0), (3.6, -0.55, 0), size=S_LABEL, color=INK)
        key = self._legend((3.6, -1.45, 0), "c")
        who = self.label(f"{TAG}: tributary cell as in the plan", f"{TAG}: Lasteinzugsfläche wie im Plan", (3.6, -1.0, 0), size=S_SMALL, color=MUTED)
        self.step("buildup",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(body), _in(finish), _engine.FadeIn(floor), _engine.FadeIn(marks[0]), _engine.FadeIn(bar), _engine.FadeIn(equation), _engine.FadeIn(legend)),
                       (_to(level["self_weight"], loads["self_weight"]["value"], 1.4),),
                       (_engine.FadeIn(marks[1]), _to(level["finishes"], loads["finishes"]["value"], 1.0)),
                       (_engine.FadeIn(marks[2]), _engine.FadeIn(people), _engine.Create(stems), _to(level["imposed"], loads["imposed"]["value"], 1.0)),
                       (_engine.FadeIn(times), _engine.FadeIn(who), _engine.FadeIn(key))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[body, finish, floor, marks, people, stems, bar, equation, legend, times, who, key])

    def _beat_cells(self, f):
        """🔲 How the tributary cell of B3 is built: midlines to the neighbours, the notch cut by the ramp opening, the area counted by a tracker."""
        s = self._s
        mine = next(p for p in f.trib["pilotis"] if p["tag"] == TAG)
        x, y = f.b3["x"], f.b3["y"]
        grid = f.g["grid"]
        spacing = grid["spacing"]
        half = spacing / 2
        k = _engine.ValueTracker(0)
        view = _d.View("plan", (3.75, 0.2, 0), 0.46, (x, y + 0.5, 0.0))
        reach = 5.25
        lo, hi = (x - reach, y - reach + 0.5), (x + reach, y + reach + 0.5)
        frame = [view(lo[0], lo[1]), view(hi[0], lo[1]), view(hi[0], hi[1]), view(lo[0], hi[1])]
        base = _engine.VGroup(_d.polygon(frame, _d.GROUND_PLANE, 1.0, MUTED, _d.W_THIN))
        void = f.slab["voids"][0]
        vx, vy = (min(p[0] for p in void), max(p[0] for p in void)), (min(p[1] for p in void), max(p[1] for p in void))
        x0, x1 = max(vx[0], lo[0]), min(vx[1], hi[0])
        y0, y1 = max(vy[0], lo[1]), min(vy[1], hi[1])
        base.add(_hatched([view(x0, y0), view(x1, y0), view(x1, y1), view(x0, y1)], ARCH, 0.1, ARCH, _d.W_THIN, 0.09, _d.mix(ARCH, _d.BACKGROUND, 0.4)))
        base.add(_d.polyline([view(x, lo[1]), view(x, hi[1])], MUTED, _d.W_HAIR, 0.8, dash=0.12), _d.polyline([view(lo[0], y), view(hi[0], y)], MUTED, _d.W_HAIR, 0.8, dash=0.12))
        for p in f.g["pilotis"]:
            if lo[0] <= p["x"] <= hi[0] and lo[1] <= p["y"] <= hi[1]:
                lit = p["tag"] == TAG
                base.add(_engine.Circle(radius=max(p["radius"] * 0.46, 0.06), stroke_color=DATA if lit else INK, stroke_width=_d.W_EDGE, fill_color=DATA if lit else _d.CONCRETE, fill_opacity=1.0).move_to(view(p["x"], p["y"])),
                         *([] if lit else [self.text(p["tag"], view(p["x"], p["y"]) + np.array([0.2 if p["x"] <= x else -0.2, 0.15, 0]), size=S_SMALL, color=MUTED)]))
        for gx in (x - half, x + half):
            base.add(_d.polyline([view(gx, lo[1]), view(gx, hi[1])], DATA, _d.W_THIN, 0.8, dash=0.09))
        for gy in (y - half, y + half):
            base.add(_d.polyline([view(lo[0], gy), view(hi[0], gy)], DATA, _d.W_THIN, 0.8, dash=0.09))
        notch = [[max(x - half, vx[0]), max(y - half, vy[0])], [min(x + half, vx[1]), max(y - half, vy[0])], [min(x + half, vx[1]), y + half], [max(x - half, vx[0]), y + half]]

        def grown(ring):
            return [view(x + (px - x) * math.sqrt(k.get_value()), y + (py - y) * math.sqrt(k.get_value())) for px, py in ring]

        def drawing():
            if k.get_value() < 0.02:
                return _engine.VGroup()
            return _engine.VGroup(_hatched(grown(mine["cell"]), DATA, 0.3, DATA, _d.W_EDGE, 0.07, DATA), _hatched(grown(notch), WARN, 0.4, WARN, _d.W_EDGE, 0.07, WARN))

        shape = self.live(drawing, key=lambda: round(k.get_value(), 2))
        dim_x = self.dimension(view(x, y - half), view(x + half, y - half), self.quantity(half, "m", 3), offset=-0.25, color=DATA)
        dim_y = self.dimension(view(x + half, y), view(x + half, y + half), self.quantity(half, "m", 3), offset=-0.25, color=DATA)
        full_area = spacing * spacing
        cut = full_area - mine["area"]
        rows = _engine.VGroup(
            self.label(f"grid spacing {self.quantity(spacing, 'm')} → half {self.quantity(half, 'm', 3)}", f"Rasterabstand {self.quantity(spacing, 'm')} → halb {self.quantity(half, 'm', 3)}", (-6.7, 1.6, 0), size=S_SMALL, color=DATA, anchor="l"),
            self.label(f"full cell {self.quantity(spacing, 'm')} × {self.quantity(spacing, 'm')} = {self.quantity(full_area, 'm²', 2)}", f"volle Zelle {self.quantity(spacing, 'm')} × {self.quantity(spacing, 'm')} = {self.quantity(full_area, 'm²', 2)}", (-6.7, 1.15, 0), size=S_SMALL, color=INK, anchor="l"),
            self.label(f"ramp opening cuts away {self.quantity(cut, 'm²', 2)}{STAR}", f"Rampenöffnung schneidet {self.quantity(cut, 'm²', 2)}{STAR} ab", (-6.7, 0.7, 0), size=S_SMALL, color=WARN, anchor="l"))
        area = self.live_label(lambda: self.tr(f"{TAG} cell = ", f"{TAG}-Zelle = ") + self.number(k.get_value() * full_area, 2) + " − " + self.number(k.get_value() * cut, 2) + " = " + self.quantity(k.get_value() * mine["area"], "m²", 2) + STAR, (-4.1, -0.1, 0), size=S_LABEL, color=DATA)
        kind = self.label(f"{TAG}: interior piloti, but its cell is notched", f"{TAG}: Innenstütze, aber mit eingekerbter Zelle", (-4.1, -0.6, 0), size=S_SMALL, color=MUTED, width=5.2)
        method = self.label("tributary-area estimate", "Lasteinzugsflächen-Näherung", (-4.1, -1.05, 0), size=S_SMALL, color=MUTED, width=5.2)
        key = self._legend((-4.1, -1.5, 0), "c")
        self.step("cells",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(base), _engine.FadeIn(rows[0]), _engine.FadeIn(rows[1])),
                       (_engine.Create(dim_x), _engine.Create(dim_y), _engine.FadeIn(rows[2])),
                       (_engine.FadeIn(shape), _engine.FadeIn(area), _to(k, 1, 2.6)),
                       (_engine.FadeIn(kind), _engine.FadeIn(method), _engine.FadeIn(key))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[base, rows, dim_x, dim_y, shape, area, kind, method, key])

    def _beat_feedback(self, f):
        """🔄 What the analysis tool sends back: the reaction of B3 returns to the IFC model as a result linked to it, while class, geometry and GlobalId only flow forth."""
        s = self._s
        mine = next(p for p in f.trib["pilotis"] if p["tag"] == TAG)
        chosen = {t: next(p for p in f.trib["pilotis"] if p["tag"] == t)["reaction"] for t in ("A3", "A1")}
        p = _engine.ValueTracker(0)
        chip = self.guid_chip(TAG, (-4.15, 2.4, 0), model=f.m)
        left = self.label("IFC model", "IFC-Modell", (-4.15, 1.85, 0), size=S_SMALL, color=STRUCT)
        rows = [(self.tr("Reaction R", "Auflagerkraft R"), self.number(mine["reaction"], 0), "kN", "assumed"), (self.tr("Area load q", "Flächenlast q"), f.trib["area_load"], "kN/m²", "assumed"),
                (self.tr("Tributary area A", "Lasteinzugsfläche A"), self.number(mine["area"], 2), "m²", "assumed"), (self.tr("Source revision", "Quellrevision"), str(f.analysis["source_revision"]), "", "sourced")]
        table = self.property_table(rows, (-4.15, 0.5, 0), width=5.1, title=self.tr("Result linked to B3 (example)", "Ergebnis an B3 (Beispiel)"))
        note = self.label("a result set of its own, not a standard Pset", "ein eigenes Ergebnis-Set, kein Standard-Pset", (-4.15, -0.75, 0), size=S_SMALL, color=MUTED, width=5.1)
        view = self.analysis_view(f.analysis, (4.6, 0.3, 0), 0.19, highlight=(TAG,), loads=chosen, model=f.m)
        right = self.label("analysis tool", "Analysewerkzeug", (4.6, 2.65, 0), size=S_SMALL, color=STRUCT)
        own = _backed(self.text(self.quantity(mine["reaction"], "kN", 0), view.at(TAG) + np.array([0.5, -0.32, 0]), size=S_SMALL, color=GOOD), 0.03)
        view.add(own)
        forth = _engine.VGroup(_engine.Arrow((-1.5, 1.5, 0), (2.4, 1.5, 0), buff=0, stroke_width=3, color=GOOD, max_tip_length_to_length_ratio=0.1),
                               self.label("class, GlobalId, geometry, loads", "Klasse, GlobalId, Geometrie, Lasten", (0.45, 1.8, 0), size=S_SMALL, color=GOOD, width=3.8))
        route = _engine.Arrow((2.4, 0.35, 0), (-1.5, 0.35, 0), buff=0, stroke_width=3, color=DATA, max_tip_length_to_length_ratio=0.1)
        back = _engine.VGroup(route, self.label("reactions R, source revision", "Auflagerkräfte R, Quellrevision", (0.45, 0.65, 0), size=S_SMALL, color=DATA, width=3.8))
        packet = self.badge(f"R = {self.quantity(mine['reaction'], 'kN', 0)}", (2.4, 0.35, 0), color=DATA)
        packet.add_updater(lambda mob: mob.move_to((2.4 - 3.9 * p.get_value(), 0.35, 0)))
        never = _engine.VGroup(_engine.DashedLine((2.4, -0.9, 0), (-1.5, -0.9, 0), color=WARN, stroke_width=3).add_tip(tip_length=0.2), _d.segments([((0.25, -0.65, 0), (0.65, -1.15, 0)), ((0.25, -1.15, 0), (0.65, -0.65, 0))], WARN, 4),
                               self.label("geometry, class: not sent back", "Geometrie, Klasse: nicht zurück", (0.45, -1.45, 0), size=S_SMALL, color=WARN, width=3.8))
        self.step("feedback",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(chip), _engine.FadeIn(left), _engine.FadeIn(right), _engine.FadeIn(view)),
                       (_engine.FadeIn(forth),),
                       (_engine.FadeIn(back), _engine.FadeIn(packet), _to(p, 1, 2.4)),
                       (_engine.FadeOut(packet), _engine.FadeIn(table), _engine.FadeIn(note), _engine.FadeIn(never))),
                  seconds=6.5)
        packet.clear_updaters()
        self._s = SimpleNamespace(off=[chip, left, right, view, forth, back, table, note, never])

    def _beat_loss(self, f):
        """✂️ Ribbon windows, infill walls and render colour stay behind; translating back does not bring them back."""
        s = self._s
        analysis = f.analysis
        wanted = ("IfcWindow:element", "IfcOpeningElement:element", "IfcWall.STANDARD:element", "IfcWall.STANDARD:material.layer.render")
        groups = [group for key in wanted for group in analysis["loss_groups"] if group["id"] == key]
        lost_tags = tuple(dict.fromkeys(tag for group in groups for tag in group["source_tags"]))
        full = self.villa_axon((-4.2, 0.65, 0), 0.14, model=f.m, disciplines=("architecture", "structure"), highlight=lost_tags)
        ghosted = self.villa_axon((-4.2, 0.65, 0), 0.14, model=f.m, disciplines=("architecture", "structure"), ghost=lost_tags, highlight=lost_tags)
        target = self.analysis_view(analysis, (4.45, 1.0, 0), 0.2, model=f.m)
        forward = _engine.Arrow((-1.0, 1.0, 0), (1.9, 1.0, 0), buff=0, stroke_width=3, color=GOOD, max_tip_length_to_length_ratio=0.15)
        translate = self.label("translate", "übersetzen", (0.45, 1.3, 0), size=S_SMALL, color=GOOD)
        back = _engine.DashedLine((1.9, 0.2, 0), (-1.0, 0.2, 0), color=WARN, stroke_width=3).add_tip(tip_length=0.2)
        cross = _d.segments([((0.3, 0.45, 0), (0.6, -0.05, 0)), ((0.3, -0.05, 0), (0.6, 0.45, 0))], WARN, 4)
        reverse = self.label("translating back: cannot restore", "Rückübersetzung: nicht wiederherstellbar", (0.45, -0.3, 0), size=S_SMALL, color=WARN, width=3.5)
        chips = _engine.VGroup(*[self.label(f"× {group['count']} " + (self.tr("walls: render layer (white) not kept", "Wände: Putzschicht (weiß) nicht erhalten") if group["id"] == wanted[3] else self._text(group)), None, (-6.7, -1.6 - 0.36 * i, 0), size=S_SMALL, color=WARN, anchor="l")
                                 for i, group in enumerate(group for group in groups if group["id"] != wanted[1])])
        choice = self.label("teaching choice: ribbon windows on all faces", "Lehrentscheidung: Bandfenster an allen Seiten", (-4.2, 2.8, 0), size=S_SMALL, color=MUTED)
        self.step("loss",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(full), _engine.FadeIn(target), _engine.Create(forward), _engine.FadeIn(translate), _engine.FadeIn(choice)),
                       (_engine.FadeOut(full), _engine.FadeIn(ghosted), _engine.FadeIn(chips)),
                       (_engine.Create(back), _engine.Create(cross), _engine.FadeIn(reverse))),
                  seconds=6.5)
        self._s = SimpleNamespace(off=[ghosted, chips, target, forward, translate, back, cross, reverse, choice])

    def _beat_domain(self, f):
        """🧭 IFC's structural analysis entities on the drawn model, next to a tool's own input format."""
        s = self._s
        analysis = f.analysis
        sh, mb, sp = analysis["shells"][0], next(m for m in analysis["members"] if m["source_tag"] == TAG), next(u for u in analysis["supports"] if u["source_tag"] == TAG)
        view = self.analysis_view(analysis, (-3.7, 0.2, 0), 0.15, projection="axon", highlight=(TAG, sh["id"]), model=f.m)
        kinds = {"surface": ("slab shells", "Deckenschalen"), "curve": ("piloti members", "Stützen als Stäbe"), "support": ("piloti foot supports", "Stützenfüße als Lager")}
        rules = sorted(analysis["mapping"], key=lambda rule: ("surface", "curve", "support").index(rule["kind"]))
        rows = [f"{rule['target_class']} · {rule['count']}{STAR if rule['kind'] != 'surface' else ''} " + self.tr(*kinds[rule["kind"]]) for rule in rules] + [f"{analysis['model_class']} · 1 " + self.tr("model groups all items", "Modell fasst alle Objekte")]
        box = self.class_box(self.tr("IFC structural analysis entities", "IFC-Entitäten der Tragwerksanalyse"), rows, (3.75, 1.85, 0), color=STRUCT, width=5.5)
        targets = [view.at(sh["id"]), view.at(mb["source_tag"]), view.at(TAG + ".support")]
        leads = _engine.VGroup(*[_wire(row.get_left() + np.array([-0.08, 0, 0]), point, GOOD, 1) for row, point in zip(box.rows, targets)])
        own = _engine.VGroup(*[self.mono(row, (0, 0, 0), size=S_SMALL, color=INK) for row in (
            f"SHELL {sh['id']} {' '.join(sh['nodes'])} t {sh['thickness']:g} m{STAR}", f"BEAM  {mb['id']} {' '.join(mb['nodes'])} D {mb['diameter']:.2f} m{STAR}", f"FIX   {sp['node']} {sp['fixity'].upper()}{STAR}")]).arrange(_engine.DOWN, aligned_edge=_engine.LEFT, buff=0.1)
        own.move_to(np.array([3.75, -0.95, 0]))
        key = _engine.VGroup(self._legend((3.75, -2.2, 0), "c"), self._count_legend(f, (3.75, -2.47, 0), "c"))
        frame = _engine.SurroundingRectangle(own, color=MUTED, buff=0.14, corner_radius=0.05, stroke_width=_d.W_THIN)
        note = self.label("an analysis tool: its own input format (example)", "ein Analysewerkzeug: eigenes Eingabeformat (Beispiel)", (3.65, -1.85, 0), size=S_SMALL, color=MUTED, width=5.4)
        self.step("domain",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _engine.FadeIn(view), _engine.FadeIn(box)),
                       (_engine.Create(leads),),
                       (_engine.FadeIn(own), _engine.FadeIn(frame), _engine.FadeIn(note), _engine.FadeIn(key))),
                  seconds=5.5)
        self._s = SimpleNamespace(off=[view, box, leads, own, frame, note, key])

    def _beat_check(self, f):
        """✅ An IDS requirement checked over all pilotis of the semantic file and of the mesh file."""
        s = self._s
        semantic, mesh = f.ids, f.ids_mesh
        total = semantic["found"]
        card = self.class_box(self.tr("IDS requirement", "IDS-Anforderung"),
                              [self._text(semantic["applicability"]), self._text(semantic["requirement"]), self._text(semantic["cardinality"])], (-3.9, 2.0, 0), color=GOOD, width=5.7)
        known = {e["guid"] for e in f.m["elements"]}
        same = sum(1 for member in f.analysis["members"] if member["source_guid"] in known)
        units = _units(f.m) - {""}
        revision = f.analysis["source_revision"]
        checks = [(self.tr(f"GlobalIds unchanged: {same} / {len(f.analysis['members'])}", f"GlobalIds unverändert: {same} / {len(f.analysis['members'])}"), same == len(f.analysis["members"])),
                  (self.tr(f"units in the model: {', '.join(sorted(units))}", f"Einheiten im Modell: {', '.join(sorted(units))}"), units <= ALLOWED_UNITS),
                  (self.tr(f"source revision {revision} = model revision {f.m['revision']}", f"Quellrevision {revision} = Modellrevision {f.m['revision']}"), revision == f.m["revision"])]
        title = self.label("translation checks (not IDS)", "Übersetzungsprüfungen (nicht IDS)", (-6.7, -0.05, 0), size=S_SMALL, color=MUTED, anchor="l")
        rows = [self.label((self.tr("Passed: ", "Bestanden: ") if ok else self.tr("Failed: ", "Nicht bestanden: ")) + text, None, (-6.7, -0.5 - 0.4 * i, 0), size=S_SMALL, color=GOOD if ok else WARN, anchor="l") for i, (text, ok) in enumerate(checks)]
        plan = self.villa_plan("ground", (3.4, 0.55, 0), 0.26, model=f.m, layers=("structure", "architecture"), grid=False, labels=False)
        n = _engine.ValueTracker(0)
        positions = {p["tag"]: p for p in f.g["pilotis"]}
        rings = _engine.VGroup(*[_engine.Circle(radius=0.11, color=GOOD, stroke_width=3).move_to(plan.at(positions[tag]["x"], positions[tag]["y"], 0)) for tag in semantic["passed_tags"]])
        for index, ring in enumerate(rings):
            ring.add_updater(lambda mob, i=index: mob.set_stroke(opacity=1.0 if n.get_value() > i else 0.0))
        counter = self.live_label(lambda: self.tr(f"{int(n.get_value())} / {total}{STAR} IfcColumn pass", f"{int(n.get_value())} / {total}{STAR} IfcColumn bestanden"), (3.4, -2.35, 0), size=S_LABEL, color=GOOD)
        mesh_counter = self.label(f"triangle mesh export: no IfcColumn found ({mesh['found']})", f"Dreiecksnetz-Export: keine IfcColumn gefunden ({mesh['found']})",
                                  (-6.7, -2.15, 0), size=S_SMALL, color=WARN, anchor="l")
        spot = plan.at(positions[TAG]["x"], positions[TAG]["y"], 0)
        flag = f.element["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"]
        value = self.label(f"{TAG} · LoadBearing = {'TRUE' if flag else 'FALSE'}", None, (-6.7, 0.95, 0), size=S_SMALL, color=GOOD, anchor="l")
        tie = _wire(value.get_right() + np.array([0.08, 0, 0]), spot + np.array([-0.12, 0.04, 0]), GOOD)
        fan = _engine.VGroup(self.mesh_view(TAG, (-0.45, -2.15, 0), 2.4, view="top", counts=False, vertices=False, model=f.m), _engine.Circle(radius=0.34, color=WARN, stroke_width=_d.W_EDGE).move_to((-0.45, -2.15, 0)))
        key = self._count_legend(f, (-6.7, -2.5, 0))
        self.step("check",
                  _seq((*[_engine.FadeOut(m) for m in s.off], _in(plan), _engine.FadeIn(card), _engine.FadeIn(rings), _engine.FadeIn(counter), _engine.Create(tie), _engine.FadeIn(value)),
                       (_to(n, len(semantic["passed_tags"]), 2.6),),
                       (_engine.FadeIn(title), *[_engine.FadeIn(row) for row in rows]),
                       (_engine.FadeIn(mesh_counter), _in(fan), _engine.FadeIn(key))),
                  seconds=6.5)
