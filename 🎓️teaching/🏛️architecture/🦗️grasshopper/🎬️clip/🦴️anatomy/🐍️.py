"""🦴 Anatomy of a Grasshopper script, built on the Nakagin capsule: definition, canvas, component, parameter, inputs, panel, states, plugin and item, list, tree."""

import functools
import importlib
import re
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
_gh = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_drawing = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_canvas = _visuals._canvas
_art = _visuals._art
_engine = _visuals._engine

INK, MUTED = _visuals.INK, _visuals.MUTED
DARK, SOFT, SMALL = _canvas.TEXT, _canvas.SUBTLE, _canvas.S_SMALL
DIMCOL = {"width": "#C28F00", "length": "#0B8F86", "height": "#3E9B45"}
AMBER, RED, GREEN = "#E0A000", "#D8362B", "#2E8B2E"
SHEET, SHEET_W, SHEET_H = np.array([-2.25, -0.76, 0.0]), 8.9, 3.64
VIEW, VIEW_W, VIEW_H = np.array([4.6, -0.76, 0.0]), 4.4, 3.64
RIBBON_AT = (0.0, 1.78, 0.0)
SL_X, SL_W = -2.6, 3.3
CB_AT = (0.5, 0.2)
WINDOW_AT = VIEW + np.array([0.4, -0.05, 0.0])
LABELS = {"Width": ("Width", "Breite"), "Length": ("Length", "Länge"), "Height": ("Height", "Höhe"), "Storey height": ("Storey height", "Geschosshöhe"), "Window": ("Window", "Fenster")}


NB, NA = (next(t.capsule_storeys for t in _nk.towers() if t.index == index) for index in (1, 0))


def _figures(rows):
    """🔢 Fill the model figures (slider values, domain, volume, window, storey step, capsule levels) into the narration, decimal comma in German."""
    objects = _gh.canvas("anatomy").objects
    w, bl, bh, st = (objects[key].settings for key in ("w", "bl", "bh", "st"))
    figures = {"wmin": f"{w['min']:.1f}", "wmax": f"{w['max']:.1f}", "w": f"{w['value']:.1f}", "w1": f"{w['value'] + 0.5:.1f}", "l1": f"{bl['value'] + 1.0:.1f}", "h1": f"{bh['value'] + 0.5:.1f}",
               "vol": f"{w['value'] * bl['value'] * bh['value']:.0f}", "dia": f"{_nk.capsule_type('A').window_diameter:.1f}", "step": f"{st['value']:g}", "nb": str(NB), "na": str(NA), "nbm": str(NB - 1)}

    def fill(text, comma):
        for key, value in figures.items():
            text = text.replace("{" + key + "}", value.replace(".", ",") if comma else value)
        return text

    return [[key, fill(en, False), fill(de, True)] for key, en, de in rows]


@functools.lru_cache(maxsize=None)
def _fit(towers, podium):
    """🧲 ``at`` and ``scale`` of the tower drawing that fits the Rhino viewport below its label row (capitals off, as in every viewport tower here)."""
    return _canvas.building_fit(VIEW_W, VIEW_H, centre=VIEW, top=_canvas.LABEL_ROW, towers=towers, podium=podium, capital=False)


def _at(x, y):
    """📍 A point on the canvas sheet from offsets to its centre."""
    return SHEET + np.array([x, y, 0.0])


class Beat1_Anatomy(_visuals.ClipScene):
    """🦴 Teach the parts of a Grasshopper script on the parametric Nakagin capsule, then data from item to list to tree."""

    TITLE = {"en": "Anatomy of a Grasshopper Script", "de": "Aufbau eines Grasshopper-Skripts"}
    NARRATION = _figures([
        ["hook", "This is one capsule of the Nakagin Capsule Tower, a simplified teaching model. How do we hand it to a computer?",
         "Das ist eine Kapsel des Nakagin Capsule Tower, ein vereinfachtes Lehrmodell. Wie geben wir sie dem Computer?"],
        ["script", "We describe it as a script: a Grasshopper definition, saved as a .gh file. Its drawing area is the canvas.",
         "Wir beschreiben sie als Skript: eine Grasshopper-Definition, gespeichert als .gh-Datei. Ihre Arbeitsfläche ist der Canvas."],
        ["slider", "A Number Slider is the first input: a name, a domain from {wmin} to {wmax}, one decimal and a value, {w}.",
         "Ein Number Slider ist die erste Eingabe: ein Name, ein Bereich von {wmin} bis {wmax}, eine Dezimalstelle und ein Wert, {w}."],
        ["box", "A component turns inputs into an output. Center Box, nickname Box, takes width, length and height and builds a box; the viewport shows it.",
         "Eine Komponente macht aus Eingaben eine Ausgabe: Center Box, Kurzname Box, baut aus Breite, Länge und Höhe eine Box."],
        ["parts", "Every component has a name and a nickname, and the canvas shows the nickname. Inputs sit on the left, outputs on the right, each with a grip.",
         "Jede Komponente hat Namen und Kurznamen; der Canvas zeigt den Kurznamen. Eingänge liegen links, Ausgänge rechts, je mit einem Griff."],
        ["flow", "Wires carry data from an output grip to an input grip and normally flow from left to right.",
         "Verbindungen tragen Daten von einem Ausgangsgriff zu einem Eingangsgriff und laufen normalerweise von links nach rechts."],
        ["sliders", "Each slider is one dimension: wire and dimension line share its colour. Length {l1} and height {h1} metres make the box grow.",
         "Jeder Schieberegler ist ein Maß: Verbindung und Maßlinie teilen seine Farbe. Länge {l1} und Höhe {h1} Meter lassen die Box wachsen."],
        ["live", "Drag a slider and the result follows. The width goes from {w} to {w1} metres and back, and the box changes with it.",
         "Zieht man einen Schieberegler, folgt das Ergebnis. Die Breite geht von {w} auf {w1} Meter und zurück, und die Box ändert sich mit."],
        ["preview", "The preview eye hides only the drawing, not the data: the box disappears, the volume still reads {vol} cubic metres.",
         "Das Vorschau-Auge blendet nur die Zeichnung aus, nicht die Daten: Die Box verschwindet, das Volumen zeigt weiter {vol} Kubikmeter."],
        ["warning", "Grey is fine. A number and a text in Z turn it orange, a warning: one box, one null. Replace the text by 2.5 and it is grey again.",
         "Grau ist in Ordnung. Zahl und Text in Z machen sie orange, eine Warnung: eine Box, ein null. Mit 2.5 statt tall ist sie grau."],
        ["error", "With nothing usable it turns red, an error: a wire cannot make the text tall a height. Give Z the number 2.5 and the box returns.",
         "Ohne Brauchbares wird sie rot, ein Fehler: Aus dem Text tall macht keine Verbindung eine Höhe. Mit dem Text 2.5 in Z kehrt die Box zurück."],
        ["panel", "A Panel shows what a wire carries: path {0}, index 0, value {w}. It makes a hidden number readable.",
         "Ein Panel zeigt, was eine Verbindung trägt: Pfad {0}, Index 0, Wert {w}. Es macht eine verborgene Zahl lesbar."],
        ["params", "A parameter holds typed data. Floating it stands alone; as input X of a component it belongs to a port. Both hold the Number {w}.",
         "Ein Parameter hält Daten eines Typs. Frei steht er allein; als Eingang X einer Komponente gehört er zu einem Port. Beide halten die Zahl {w}."],
        ["toggle", "A Boolean Toggle has two states. True gives the capsule its round window, {dia} metres across, in the front face.",
         "Ein Boolean Toggle hat zwei Zustände. True gibt der Kapsel ihr rundes Fenster, {dia} Meter im Durchmesser, in der Stirnseite."],
        ["valuelist", "A Value List offers fixed choices: the proposed room-program studies A to D. They share the nominal 1.30 metre porthole.",
         "Eine Werteliste bietet feste Auswahlmöglichkeiten: die Raumprogramm-Studien A bis D. Alle teilen das Bullauge von 1,30 Metern."],
        ["plugin", "A plugin, a .gha file, adds a tab to the ribbon. Ladybug Tools studies the sun on the capsule window: more sun, warmer tint.",
         "Ein Plugin, eine .gha-Datei, fügt dem Ribbon einen Reiter hinzu. Ladybug Tools untersucht die Sonne am Fenster: mehr Sonne, wärmere Tönung."],
        ["item", "Now the data. One value is an item: the elevation offset zero metres for the first capsule level of Tower B.",
         "Nun die Daten. Ein Wert ist ein Element: der Höhenversatz null Meter für die erste Kapselebene von Turm B."],
        ["series", "A Series with step {step} metres gives {nb} offsets in one branch, indices 0 to {nbm}: one capsule per level of Tower B.",
         "Eine Series mit Schrittweite {step} Meter liefert {nb} Versätze in einem Ast, Indizes 0 bis {nbm}: eine Kapsel je Ebene von Turm B."],
        ["count", "Change Count from {nb} to {na} and the list grows. Tower A has {na} capsule levels.",
         "Ändert man Anzahl von {nb} auf {na}, wächst die Liste. Turm A hat {na} Kapselebenen."],
        ["tree", "Many lists form a data tree. Each occupied facade group is a branch with its displayed address and a variable number of capsule items.",
         "Viele Listen bilden einen Datenbaum. Jede belegte Fassadengruppe ist ein Ast mit angezeigter Adresse und einer variablen Anzahl von Kapseln."],
        ["mistake", "A common mistake is to read a tree as one long list. Index one exists in many branches; a singleton branch has no item at that index.",
         "Typischer Fehler: einen Datenbaum wie eine lange Liste lesen. Index eins gibt es in vielen Ästen, nicht in jedem."],
        ["address", "Only path plus index identifies exactly one capsule. The Panel shows the actual selected address and index one.",
         "Erst Pfad plus Index identifiziert genau eine Kapsel. Das Panel zeigt die tatsächlich gewählte Adresse und Index eins."],
        ["legend_tool", "The canvas terms, row by row in English and German: script, Skript; component, Komponente; and six more.",
         "Die Begriffe des Canvas: Skript, Komponente, Parameter, Eingang, Ausgang, Schieberegler, Panel und Plugin. Englisch steht daneben."],
        ["legend_data", "The data terms in English and German: item, Element; list, Liste; tree, Datenbaum; branch, Ast; path, Pfad; index.",
         "Die Begriffe der Daten: Element, Liste, Datenbaum, Ast, Pfad und Index. Die englischen Wörter stehen daneben."],
    ])

    TOOL_TERMS = (("Script", "Skript"), ("Component", "Komponente"), ("Parameter", "Parameter"), ("Input", "Eingang"), ("Output", "Ausgang"),
                  ("Slider", "Schieberegler"), ("Panel", "Panel"), ("Plugin", "Plugin"))
    DATA_TERMS = (("Item", "Element"), ("List", "Liste"), ("Tree", "Datenbaum"), ("Branch", "Ast"), ("Path", "Pfad"), ("Index", "Index"))
    TOOL_ORDER, DATA_ORDER = tuple(range(1, len(TOOL_TERMS) + 1)), (1, 2, 4, 3, 5, 6)

    def snapshot(self, tag):
        """📸 Keyframe hook for beats that continue after their clause; the capture script saves a frame here."""
        return None

    def construct(self):
        """▶️ Present this chapter through the Grasshopper visual interface."""
        self.present("anatomy")

    def _nice(self, text):
        """🔢 Decimal comma in German for plain numbers inside panel text."""
        return re.sub(r"(?<=\d)\.(?=\d)", ",", text) if self.language() == "de" else text

    def _rows(self, tree, unit=""):
        """📋 Panel rows ``(kind, index, text)`` from an engine tree, localized, one path header per branch."""
        out, last = [], None
        for path, index, text in _gh.panel_rows(tree, True):
            if path != last:
                out.append(("path", "", path))
                last = path
            out.append(("item", str(index), self._nice(text) + unit))
        return out

    def _label(self, name):
        """🏷️ Localized user name of a slider."""
        return self.tr(*LABELS.get(name, (name, name)))

    def _sub(self, ids, wires):
        """🧪 A sub-definition of the anatomy canvas with only the given objects and wires."""
        full = _gh.canvas("anatomy")
        objects = [{"id": i, "component": full.objects[i].component, "at": list(full.objects[i].at), "settings": full.objects[i].settings} for i in ids]
        return _gh.define({"objects": objects, "wires": list(wires)}, "anatomy")

    def _note(self, text, point, *, color=DARK, anchor="c"):
        """🔤 A label on the light canvas sheet or viewport."""
        return self.text(text, point, size=SMALL, color=color, anchor=anchor)

    def _bit(self, label, point, color, kind="number", size=1.0):
        """🪙 An abstract token of the morph toolkit placed at a point."""
        return _morph.token(label, kind=kind, color=color, size=size).move_to(point).set_z_index(8)

    def _hop(self, label, start, stop, color, kind="number", ghost=0.0, first=None, size=1.0):
        """🔀 A token born at ``start`` that morphs into its twin at ``stop``; returns the animation and the arriving token."""
        source, target = self._bit(label if first is None else first, start, color, kind, size), self._bit(label, stop, color, kind, size)
        self.add(source)
        return _morph.morph(source, target, ghost=ghost, path_arc=0.5, run_time=1.2, dip=0.12), target

    def _cap(self, key, towers):
        """🏗️ A copy of one capsule of the static tower drawing, in place, as a morph source or target."""
        self._spot(key, towers)
        return self._towers[towers].capsules[key].copy().set_z_index(8)

    def _cap_to(self, key, towers, token):
        """🔀 A capsule copy that flies part-wise into a token (shell to ring, window to dot); returns the animation."""
        source = self._cap(key, towers)
        self.add(source)
        return _morph.morph(source, token, pairs=_morph.capsule_pairs(source, token), path_arc=0.5, run_time=1.2, dip=0.15)

    def _to_cap(self, token, key, towers, run_time=1.2, arc=0.5):
        """🔀 A token that flies part-wise into a capsule copy; returns the animation and the arriving capsule."""
        target = self._cap(key, towers)
        return _morph.morph(token, target, pairs=[(b, a) for a, b in _morph.capsule_pairs(target, token)], path_arc=arc, run_time=run_time, dip=0.15), target

    @staticmethod
    def _disc(marker):
        """⚫ The numbered disc of a legend marker."""
        return marker if isinstance(marker[0], _engine.Circle) else marker[-1]

    def _fly_discs(self, markers, table, order):
        """🔢 The numbered markers on the canvas fly into the number discs of their legend rows."""
        sources = []
        for marker in markers:
            original = self._disc(marker)
            disc = original.copy()
            original.set_opacity(0.0)
            self.add(disc)
            sources.append(disc)
        targets = [table[number - 1][0] for number in order]
        return _morph.scatter(sources, targets, lag=0.15, path_arc=0.8, run_time=1.3, limit=3.2, pairs_fn=lambda a, b: [(a[0], b[0]), (a[1], b[1])])

    def _rest(self, table):
        """📖 The legend rows without their number discs."""
        return _engine.VGroup(*[_engine.VGroup(*list(row)[1:]) for row in table])

    def _spot(self, key, towers):
        """📍 Screen point of a capsule on the static tower drawing that matches the live viewport tower."""
        cache = self.__dict__.setdefault("_towers", {})
        if towers not in cache:
            cache[towers] = self.tower_axon(**_fit(towers, False), lit=None, segments=1, ground=False, types=False, capital=False, towers=towers, podium=False)
        slot = next(s for s in _nk.slots() if (*s.path, s.item) == key)
        point = np.array(cache[towers].at(*slot.plane.origin), dtype=float)
        point[2] = 0.0
        return point

    def _tag(self, text, point, color=DARK, anchor="c"):
        """🔖 An outlined chip on the light canvas."""
        return self.badge(text, point, color=color, size=SMALL, filled=False, anchor=anchor)

    def _reach(self, a, b, text, color=DARK):
        """➖ A thin leader from a label to a point."""
        return self.leader(a, b, text, color=color, dot=True)

    def _slider(self, key, at, tracker=None, width=SL_W, color=None):
        """🎚️ A Number Slider from the engine settings, optionally bound to a tracker and ringed in its dimension colour."""
        cfg = _gh.canvas("anatomy").objects[key].settings
        integer = cfg.get("rounding") == "integer"
        slider = self.gh_slider(self._label(cfg["label"]), cfg["value"], cfg["min"], cfg["max"], at=at, decimals=0 if integer else cfg.get("decimals", 2), tracker=tracker, width=width)
        if color:
            slider.add(_engine.SurroundingRectangle(slider.body, buff=0.04, color=color, stroke_width=4, corner_radius=0.09))
        return slider

    def _eye(self, widget, on=True):
        """👁️ The preview eye above a component, struck through when the preview is off."""
        eye = _engine.VGroup(_engine.Ellipse(width=0.42, height=0.22, stroke_color=DARK, stroke_width=2, fill_color="#FFFFFF", fill_opacity=1), _engine.Dot(radius=0.06, color=DARK))
        if not on:
            eye.add(_engine.Line(np.array([-0.26, -0.15, 0]), np.array([0.26, 0.15, 0]), color=RED, stroke_width=3))
        return eye.move_to(widget.get_top() + np.array([0.0, 0.2, 0.0]))

    def _pulse(self, out_grip, in_grip, color, seconds=0.9):
        """⚪ A dot travelling along a wire from the output grip to the input grip."""
        curve = _canvas.bezier(np.array(out_grip.get_center()), np.array(in_grip.get_center()))
        dot = _engine.Dot(curve[0], radius=0.09, color=color).set_z_index(5)
        return dot, _engine.UpdateFromAlphaFunc(dot, lambda mob, alpha: mob.move_to(curve[min(int(alpha * (len(curve) - 1)), len(curve) - 1)]), run_time=seconds, rate_func=_engine.linear)

    def _ramp(self, tracker, start, stop, seconds):
        """⏩ Move a tracker between two values."""
        return _engine.UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(start + (stop - start) * alpha), rate_func=_engine.smooth, run_time=seconds)

    def _ramp_index(self, tracker, sequence, seconds):
        """⏩ Step an integer tracker through ``sequence`` in equal time slices."""
        return _engine.UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(sequence[min(int(alpha * len(sequence)), len(sequence) - 1)]), rate_func=_engine.linear, run_time=seconds)

    def _type_follow(self, seconds):
        """🔁 Make the viewport capsule follow the value list index."""
        return _engine.UpdateFromAlphaFunc(_engine.Mobject(), lambda mob, alpha: self.view_state.update(type="ABCD"[int(self.vl_index.get_value())]), run_time=seconds, rate_func=_engine.linear)

    def _dims_of(self, sol, oid="cb"):
        """📐 Box sizes ``(x, y, z)`` from the first Box of a Center Box output."""
        box = sol.items(oid, "Box")[0]
        return tuple(round(d.hi - d.lo, 6) for d in (box.x, box.y, box.z))

    def _preview(self):
        """🟩 The live capsule in the Rhino viewport: a plain box from the engine sizes, or a finished capsule type."""
        state = self.view_state

        def build():
            if not state["visible"]:
                return _engine.VGroup()
            if state["mode"] == "box":
                over = {"w": round(self.W.get_value(), 3), "bl": round(self.L.get_value(), 3), "bh": round(self.H.get_value(), 3)}
                x, y, z = self._dims_of(_gh.evaluate(state["definition"], {**{k: v for k, v in over.items() if k in state["definition"].objects}, **state["overrides"]}), state["box"])
                shape, diameter = {"dimensional_basis": "physical", "width": x, "length": y, "height": z, "corner_radius": 0.0, "key": "A"}, _nk.capsule_type("A").window_diameter
            else:
                shape = _nk.capsule_type(state["type"])
                diameter = shape.window_diameter
            drawing = self.capsule_axon(shape, at=VIEW + np.array([-0.2, 0.12, 0.0]), scale=state["scale"], base=_canvas.PREVIEW_FILL, window=state["window"],
                                        dims=tuple(d for d in state["dims"] if d != "window"), dim_offset=0.4)
            for name, group in drawing.dims.items():
                group.set_color(DIMCOL[name])
            if state["window"] and "window" in state["dims"]:
                centre, radius = drawing.window.get_center(), drawing.window.width / 2
                chip = self.badge(f"Ø {self.number(diameter, 2)} m", centre + np.array([-0.55, radius + 0.55, 0.0]), color=AMBER, size=SMALL, filled=True)
                drawing.add(chip, self.leader(chip, centre + np.array([0.0, radius * 0.9, 0.0]), None, color=DARK, dot=True))
            if state["sun"]:
                centre, radius = drawing.window.get_center(), drawing.window.width / 2
                drawing.add(_engine.Circle(radius=radius * 1.12, stroke_width=0, fill_color="#F08A00", fill_opacity=0.62).move_to(centre))
            return drawing

        key = lambda: (state["visible"], state["mode"], state["type"], state["window"], state["dims"], state["sun"], state["scale"], state["box"], tuple(sorted(state["overrides"])),
                       round(self.W.get_value(), 3), round(self.L.get_value(), 3), round(self.H.get_value(), 3))
        return self.live(build, key)

    def _tower(self):
        """🗼 The live tower in the Rhino viewport with the capsules chosen by ``tower_state["lit"]`` outlined."""
        state = self.tower_state
        slots = {(c.building, c.tower, c.storey, c.facade, c.item): c for c in _nk.slots()}

        def build():
            drawing = self.tower_axon(**_fit(state["towers"], state["podium"]), lit=state["lit"](), segments=1, ground=False, types=False,
                                      towers=state["towers"], podium=state["podium"], capital=False)
            for text, key, dx, dy in state["marks"]():
                spot = np.array(drawing.at(*slots[key].plane.origin), dtype=float)
                spot[2] = 0.0
                label = self._note(text, spot + np.array([dx, dy, 0.0]), anchor="l" if dx > 0 else "r")
                drawing.add(self.leader(spot + np.array([dx - 0.05 * np.sign(dx), dy, 0.0]), spot, None, color=DARK, dot=True), label)
            return drawing

        return self.live(build, lambda: (state["key"](), state["towers"], tuple(m[0] for m in state["marks"]())))

    def _frame_rows(self, widget, rows, color):
        """🟨 A translucent band behind the given panel row indices."""
        bands = _engine.VGroup()
        for row in rows:
            line = widget.parts["rows"][row]
            bands.add(_engine.Rectangle(width=widget.body.width - 0.06, height=0.27, fill_color=color, fill_opacity=0.32, stroke_width=0).move_to([widget.body.get_center()[0], line.get_center()[1], 0]).set_z_index(1))
        return bands

    def _type_note(self):
        """🔤 Capsule type, name and window in two lines at the bottom right of the viewport, clear of the header and the axes."""
        state = self.view_state

        def build():
            t = _nk.capsule_type(state["type"])
            corner = VIEW + np.array([VIEW_W / 2 - 0.12, -VIEW_H / 2 + 0.2, 0.0])
            return _engine.VGroup(self._note(f"{self.tr('Type', 'Typ')} {t.key} · {t.name[self.language()]}", corner + np.array([0.0, 0.27, 0.0]), anchor="r"),
                                  self._note(f"Ø {self.number(t.window_diameter, 1)} m", corner, anchor="r"))

        return self.live(build, lambda: (state["type"], state["mode"]))

    def _tower_rows(self, branches):
        """📋 Tree rows for the given aggregated branches, items named by capsule type."""
        tree = _gh.aggregate_tree()
        rows = []
        for path in branches:
            rows.append(("path", "", _gh.format_path(path)))
            rows.extend(("item", str(i), self.tr("Capsule ", "Kapsel ") + item.tag["type"]) for i, item in enumerate(tree.branch(path)))
        return rows

    def _num(self, number, point):
        """🔢 A numbered marker that links a legend row to its object."""
        disc = _engine.Circle(radius=0.16, stroke_width=0, fill_color="#2B2B2B", fill_opacity=1)
        mark = self.text(str(number), (0, 0, 0), size=SMALL - 1, color="#FFFFFF")
        return _engine.VGroup(disc, mark.move_to(disc)).move_to(point).set_z_index(6)

    def _tie(self, number, badge_at, target):
        """📌 A numbered marker with a thin line to its target."""
        return _engine.VGroup(_engine.Line(np.array(badge_at), np.array(target), color="#2B2B2B", stroke_width=1.6), _engine.Dot(np.array(target), radius=0.04, color="#2B2B2B"),
                              self._num(number, badge_at))

    def _bracket(self, number, x, y0, y1, side):
        """⎱ A bracket along panel rows with its marker on the outside."""
        d = 0.09 * side
        line = _engine.VGroup(*[_engine.Line(np.array(a), np.array(b), color="#2B2B2B", stroke_width=1.6) for a, b in
                                (((x, y0, 0), (x + d, y0, 0)), ((x + d, y0, 0), (x + d, y1, 0)), ((x + d, y1, 0), (x, y1, 0)))])
        return _engine.VGroup(line, self._num(number, (x + d + 0.3 * side, (y0 + y1) / 2, 0)))

    def _table(self, terms, at, first=0):
        """📖 A numbered legend: the term of the spoken language first, the other language second."""
        pitch = 0.36
        rows = _engine.VGroup()
        german = self.language() == "de"
        for index, pair in enumerate(terms):
            y = pitch * ((len(terms) - 1) / 2 - index)
            main, other = (pair[1], pair[0]) if german else pair
            rows.add(_engine.VGroup(self._num(first + index + 1, at + np.array([-1.45, y, 0.0])), self._note(main, at + np.array([-0.35, y, 0.0]), color=DARK),
                                    self._note(other, at + np.array([1.1, y, 0.0]), color="#1F6F3F" if not german else SOFT),
                                    _engine.Line(at + np.array([-1.7, y - pitch / 2, 0.0]), at + np.array([2.0, y - pitch / 2, 0.0]), color="#BDBDBD", stroke_width=1)))
        return rows

    def _states(self, kind, sol, lines=None):
        """🚦 The orange or red Center Box case: two sliders, the text panel, the component in its solved state, its bubble and its output panel."""
        w_slider = self._slider("w", _at(SL_X, 0.85), color=DIMCOL["width"])
        l_slider = self._slider("bl", _at(SL_X, 0.05), color=DIMCOL["length"])
        key, out = ("cbw", "cbwp") if kind == "warning" else ("cbe", "cbep")
        src = "wn" if kind == "warning" else "er"
        lines = lines or _gh.canvas("anatomy").objects[src].settings["lines"]
        text = self.gh_panel(None, rows=[("line", "", line) for line in lines], at=_at(SL_X, -0.85), max_lines=4)
        box = self.gh_component("Center Box", at=_at(1.1, -0.2), state=sol.state(key), nickname=True)
        results = sol.tree(out)
        panel = self.gh_panel(None, rows=self._rows(results) if results.count else [("line", "", "")], at=_at(3.1, -0.2), max_lines=4)
        notes = sol.msgs(key)
        bubble = self.gh_bubble((notes[-1] if kind == "error" else notes[0])["en"], level="warning" if kind == "warning" else "error", target=box, width=3.6) if notes else _engine.VGroup()
        many = "list" if len(lines) > 1 else "item"
        wires = [self.gh_wire(w_slider.grip_out(), box.grip_in("X"), color=DIMCOL["width"]), self.gh_wire(l_slider.grip_out(), box.grip_in("Y"), color=DIMCOL["length"]),
                 self.gh_wire(text.grip_out(), box.grip_in("Z"), kind=many), self.gh_wire(box.grip_out(), panel.grip_in(), kind=many)]
        gloss = _engine.VGroup()
        if notes and self.language() == "de":
            german = "Umwandlung Text → Zahl fehlgeschlagen" if kind == "warning" else "Eingabeparameter Z ohne Daten"
            gloss = _engine.VGroup(self._note("Grasshopper-Meldung (englisch)", _at(2.2, -1.2), color=SOFT), self._note(german, _at(2.2, -1.5)))
        group = _engine.VGroup(w_slider, l_slider, text, box, panel, bubble, gloss, *wires)
        group.text_panel, group.box, group.panel, group.bubble, group.gloss = text, box, panel, bubble, gloss
        return group

    def _repair(self, old, new, index, extra=()):
        """🔧 The repair as an event: the text in the Panel morphs in place into 2.5, then the component, its bubble and its output turn into the repaired state."""
        row, fixed = old.text_panel.parts["rows"][index], new.text_panel.parts["rows"][index]
        self.play(_morph.retext(row, fixed, run_time=1.2), run_time=1.2)
        self.play(_engine.Transform(old.box, new.box), _engine.FadeOut(old.bubble), _engine.FadeOut(old.gloss), _engine.Transform(old.panel, new.panel), *[_engine.FadeOut(m) for m in extra], run_time=1.2)
        self._sweep(old)
        self.remove(fixed)
        self.add(new)

    def _sweep(self, group):
        """🧹 Remove a staged group and every top-level mobject that is one of its parts."""
        family = {id(m) for m in group.get_family()}
        self.remove(group, *[m for m in self.mobjects if id(m) in family or any(id(x) in family for x in m.get_family())])

    def present_anatomy(self):
        """🎞️ Run the clauses: script, slider, component, wires, states, panel, parameters, inputs, plugin, then item, list and tree."""
        defn = _gh.canvas("anatomy")
        full = _gh.evaluate(defn)
        settings = {k: defn.objects[k].settings for k in ("w", "bl", "bh", "st", "cnt")}
        self.W, self.L, self.H = (_engine.ValueTracker(settings[k]["value"]) for k in ("w", "bl", "bh"))
        self.three = self._sub(("w", "bl", "bh", "xy", "cb", "vol", "vp"), ("w.Number>cb.X", "bl.Number>cb.Y", "bh.Number>cb.Z", "xy.Plane>cb.Base", "cb.Box>vol.Geometry", "vol.Volume>vp.Stream"))
        self.view_state = {"visible": True, "mode": "type", "type": "A", "window": True, "dims": ("width", "length", "height"), "sun": False, "scale": 0.42,
                           "definition": self.three, "box": "cb", "overrides": {}}
        self.tower_state = {"lit": lambda: None, "key": lambda: None, "towers": (0,), "podium": False, "marks": lambda: []}
        self.cnt = _engine.ValueTracker(1)
        self.vl_index = _engine.ValueTracker(0)
        cw, cl, ch = DIMCOL["width"], DIMCOL["length"], DIMCOL["height"]

        sheet = self.gh_background(at=SHEET, width=SHEET_W, height=SHEET_H)
        ribbon = self.gh_ribbon(tabs=_canvas.RIBBON_TABS, active="Params", at=RIBBON_AT, width=13.4,
                                groups=[("Input", ["Panel", "Number Slider", "Boolean Toggle", "Value List"]), ("Geometry", ["Point", "Curve", "Brep", "Box"]), ("Util", ["Param Viewer"])])
        viewport = self.gh_viewport(at=VIEW, width=VIEW_W, height=VIEW_H, label=self.tr("Perspective", "Perspektive"))
        preview = self._preview()
        model_note = self.text(self.tr("Shared reconstruction · documented counts", "Gemeinsame Rekonstruktion · belegte Anzahlen"), (6.85, 3.62, 0), size=SMALL, color=MUTED, anchor="r")

        authored_slots = _nk.slots()
        first_key = tuple((*authored_slots[0].path, authored_slots[0].item))
        slots_n = len(authored_slots)
        whole = self.tower_axon(**self.gh_building_fit(viewport, towers=(0, 1), podium=True, capital=True), lit=lambda k: k == first_key, segments=1, ground=False, types=False, towers=(0, 1), podium=True)
        whole_note = self._note(self.tr(f"1 of {slots_n} capsules", f"1 von {slots_n} Kapseln"), VIEW + np.array([VIEW_W / 2 - 0.12, VIEW_H / 2 - 0.24, 0.0]), anchor="r")
        self.step("hook", _engine.FadeIn(sheet), _engine.FadeIn(ribbon), _engine.FadeIn(viewport), _engine.FadeIn(whole), _engine.FadeIn(whole_note), _engine.FadeIn(model_note), seconds=1.4)

        chip = self._tag("Nakagin_Capsule.gh", _at(-3.0, 1.45))
        canvas_tag = self._tag("Canvas", _at(3.6, 1.45))
        frame = _engine.SurroundingRectangle(sheet, buff=0.0, color=_art.P_PRIMARY, stroke_width=3)
        kinds = _nk.capsule_types()
        strip = _engine.VGroup(*[self.capsule_axon(t, at=_at(-3.0 + 2.0 * i, 0.2), scale=0.2, base=_canvas.PREVIEW_FILL, window=True) for i, t in enumerate(kinds)],
                               *[self._note(t.key, _at(-3.0 + 2.0 * i, -0.45)) for i, t in enumerate(kinds)],
                               self._note(self.tr(f"one definition: {len(kinds)} types, {slots_n} placements", f"eine Definition: {len(kinds)} Typen, {slots_n} Plätze"), _at(0.0, -1.0), color=SOFT))
        lit_capsule = whole.capsules[first_key].copy().set_z_index(8)
        self.add(lit_capsule)
        self.step("script", _engine.FadeOut(whole), _engine.FadeOut(whole_note), _engine.FadeIn(preview), lit_capsule.animate.scale(4.0).move_to(WINDOW_AT).set_stroke(color="#F2C200", width=2.5), _engine.Create(frame), _engine.FadeIn(canvas_tag), _engine.FadeIn(strip), _engine.FadeOut(model_note), seconds=1.2)
        self.play(_morph.morph(lit_capsule, chip, pairs=_morph.capsule_pairs(lit_capsule, chip), path_arc=0.4, run_time=1.4, dip=0.15), run_time=1.4)
        self.play(_engine.FadeOut(frame), run_time=0.3)

        w_slider = self._slider("w", _at(SL_X, 1.0), self.W, color=cw)
        notes = _engine.VGroup(self._note(self.tr("Name", "Name"), _at(-3.95, 1.4)), self._note(self.tr("Value", "Wert"), _at(-2.95, 1.4)),
                               self._note(self.tr("Domain 1.0 to 5.0 · one decimal", "Bereich 1,0 bis 5,0 · 1 Dezimalstelle"), _at(SL_X, 0.1)))
        self.view_state["dims"] = ("width",)
        self.W.set_value(settings["w"]["min"])
        hop_w, hop_w_end = self._hop(self.number(settings["w"]["value"], 1), _at(SL_X + 1.3, 1.0), VIEW + np.array([-0.2, -1.1, 0.0]), cw)
        self.step("slider", _engine.FadeOut(chip), _engine.FadeOut(canvas_tag), _engine.FadeOut(strip), _engine.FadeIn(w_slider), _engine.FadeIn(notes), self._ramp(self.W, settings["w"]["min"], settings["w"]["value"], 1.4), hop_w, seconds=1.4)
        self.play(_engine.FadeOut(hop_w_end), run_time=0.3)

        l_slider, h_slider = self._slider("bl", _at(SL_X, 0.2), self.L, color=cl), self._slider("bh", _at(SL_X, -0.6), self.H, color=ch)
        cb_full = self.gh_component("Center Box", at=_at(*CB_AT))
        full_wires = [self.gh_wire(w_slider.grip_out(), cb_full.grip_in("X"), color=cw), self.gh_wire(l_slider.grip_out(), cb_full.grip_in("Y"), color=cl),
                      self.gh_wire(h_slider.grip_out(), cb_full.grip_in("Z"), color=ch)]
        self.view_state.update(mode="box", window=False, dims=("width", "length", "height"), scale=0.46)
        wire_paths = [_canvas.bezier(np.array(slider.grip_out().get_center()), np.array(cb_full.grip_in(port).get_center())) for slider, port in ((w_slider, "X"), (l_slider, "Y"), (h_slider, "Z"))]
        hops = [self._hop(self.number(settings[k]["value"], 1), VIEW + np.array([-1.3 + 1.3 * i, -1.1, 0.0]), path[len(path) // 2], color, size=0.8)
                for i, ((k, color), path) in enumerate(zip((("w", cw), ("bl", cl), ("bh", ch)), wire_paths))]
        dimension_lines = [_engine.Line(VIEW + np.array([-1.75 + 1.3 * i, -1.35, 0.0]), VIEW + np.array([-0.85 + 1.3 * i, -1.35, 0.0]), color=color, stroke_width=4).set_z_index(7)
                           for i, color in enumerate((cw, cl, ch))]
        wire_curves = [_engine.VMobject(stroke_color=color, stroke_width=3).set_points_smoothly(path) for path, color in zip(wire_paths, (cw, cl, ch))]
        self.add(*dimension_lines)
        self.step("box", _engine.FadeOut(notes), _engine.FadeIn(l_slider), _engine.FadeIn(h_slider), _engine.FadeIn(cb_full), *[a for a, _ in hops],
                  *[_morph.morph(line, curve, path_arc=0.4, run_time=1.4, dip=0.1) for line, curve in zip(dimension_lines, wire_curves)], seconds=1.4)
        self.remove(*wire_curves)
        self.add(*full_wires)
        self.play(*[_engine.FadeOut(t) for _, t in hops], run_time=0.3)
        self.view_state["dims"] = ()

        cb = self.gh_component("Center Box", at=_at(*CB_AT), nickname=True)
        wires = [self.gh_wire(w_slider.grip_out(), cb.grip_in("X"), color=cw), self.gh_wire(l_slider.grip_out(), cb.grip_in("Y"), color=cl), self.gh_wire(h_slider.grip_out(), cb.grip_in("Z"), color=ch)]
        call = _engine.VGroup(
            self._reach(_at(0.2, 1.45), _at(CB_AT[0] - 0.3, CB_AT[1] + 0.8), self.tr("Name: Center Box", "Name: Center Box")),
            self._reach(_at(2.7, 1.45), cb.get_top() + np.array([0.15, 0.0, 0.0]), self.tr("Nickname: Box", "Kurzname: Box")),
            self._reach(_at(-0.7, -1.5), cb.grip_in("Y").get_center(), self.tr("Inputs", "Eingänge")),
            self._reach(_at(2.6, -1.5), cb.grip_out().get_center(), self.tr("Output", "Ausgang")))
        self.step("parts", *[_engine.FadeOut(m) for m in (cb_full, *full_wires)], _engine.FadeIn(cb), *[_engine.FadeIn(w) for w in wires], _engine.FadeIn(call), seconds=1.4)

        heads = _engine.VGroup()
        for slider, port, color in ((w_slider, "X", cw), (l_slider, "Y", cl), (h_slider, "Z", ch)):
            curve = _canvas.bezier(np.array(slider.grip_out().get_center()), np.array(cb.grip_in(port).get_center()))
            mid, ahead = len(curve) // 2, curve[len(curve) // 2 + 2] - curve[len(curve) // 2 - 2]
            heads.add(_engine.Triangle(fill_color=color, fill_opacity=1, stroke_width=0).scale(0.1).rotate(float(np.arctan2(ahead[1], ahead[0])) - np.pi / 2).move_to(curve[mid]).set_z_index(4))
        pulses = [self._pulse(slider.grip_out(), cb.grip_in(port), color) for slider, port, color in ((w_slider, "X", cw), (l_slider, "Y", cl), (h_slider, "Z", ch))]
        self.add(*[dot for dot, _ in pulses])
        self.step("flow", _engine.FadeOut(call), _engine.FadeIn(heads), _engine.Succession(*[move for _, move in pulses]), seconds=2.7)
        self.play(*[_engine.FadeOut(dot) for dot, _ in pulses], run_time=0.3)
        out_token = self._bit("Box", cb.grip_out().get_center() + np.array([0.5, 0.0, 0.0]), _canvas.PREVIEW_FILL, size=1.25)
        out_target = self._bit("Box", VIEW + np.array([-0.2, 1.3, 0.0]), _canvas.PREVIEW_FILL, size=1.25)
        self.add(out_token)
        self.play(_morph.morph(out_token, out_target, path_arc=0.5, run_time=1.4, dip=0.1), run_time=1.4)
        self.play(_engine.FadeOut(out_target), run_time=0.3)

        l0, h0 = settings["bl"]["value"], settings["bh"]["value"]
        self.view_state["dims"] = ("width", "length", "height")
        l_hop, l_end = self._hop(self.number(l0 + 1.0, 1), l_slider.get_right() + np.array([0.3, 0.0, 0.0]), VIEW + np.array([-1.6, -0.9, 0.0]), cl)
        h_hop, h_end = self._hop(self.number(h0 + 0.5, 1), h_slider.get_right() + np.array([0.3, 0.0, 0.0]), VIEW + np.array([1.5, 0.1, 0.0]), ch)
        self.step("sliders", self._ramp(self.L, l0, l0 + 1.0, 2.0), self._ramp(self.H, h0, h0 + 0.5, 2.0), l_hop, h_hop, seconds=2.0)
        self.play(_engine.FadeOut(l_end), _engine.FadeOut(h_end), run_time=0.3)
        self.play(self._ramp(self.L, l0 + 1.0, l0, 1.0), self._ramp(self.H, h0 + 0.5, h0, 1.0), run_time=1.0)

        w0, w1 = settings["w"]["value"], settings["w"]["value"] + 0.5
        readout = self.live_label(lambda: f"X = {self.number(self.W.get_value(), 1)} m", _at(0.5, 1.5), size=SMALL, color=cw)
        self.step("live", _engine.FadeIn(readout), self._ramp(self.W, w0, w1, 2.0), seconds=2.0)
        self.play(self._ramp(self.W, w1, w0, 1.2), run_time=1.2)

        vol = self.gh_component("Volume", at=_at(2.0, 0.35))
        vp_rows = self._rows(_gh.evaluate(self.three).tree("vp"), " m³")
        vp = self.gh_panel(None, rows=vp_rows, at=_at(3.7, 0.35), width=1.5)
        eye = self._eye(cb)
        vol_wire, vp_wire = self.gh_wire(cb.grip_out(), vol.grip_in()), self.gh_wire(vol.grip_out("V"), vp.grip_in())
        self.play(_engine.FadeOut(readout), _engine.FadeIn(vol), _engine.FadeIn(vp), _engine.FadeIn(vol_wire), _engine.FadeIn(vp_wire), _engine.FadeIn(eye), run_time=0.8)
        volume_hop, volume_end = self._hop(vp_rows[-1][2], VIEW + np.array([-0.2, 0.1, 0.0]), vp.get_top() + np.array([0.0, 0.4, 0.0]), GREEN)
        self.view_state["visible"] = False
        self.step("preview", _engine.Transform(eye, self._eye(cb, on=False)), volume_hop, seconds=1.2)
        volume_ghost = _engine.VGroup(_engine.Rectangle(width=1.7, height=1.1, stroke_width=0, fill_color=GREEN, fill_opacity=0.22),
                                      _engine.DashedVMobject(_engine.Rectangle(width=1.7, height=1.1, stroke_color=GREEN, stroke_width=3), num_dashes=28)).move_to(WINDOW_AT + np.array([-0.4, 0.0, 0.0])).set_z_index(6)
        self.play(_morph.morph(volume_end, volume_ghost, path_arc=0.7, run_time=1.4), run_time=1.4)

        main = _engine.VGroup(w_slider, l_slider, h_slider, cb, *wires, heads)
        extra = _engine.VGroup(vol, vp, vol_wire, vp_wire, eye)
        warn_sol, error_sol = _gh.evaluate(defn), _gh.evaluate(defn)
        warning, error = self._states("warning", warn_sol), self._states("error", error_sol)
        self.view_state.update(visible=True, definition=defn, box="cbw")
        self.step("warning", _engine.FadeOut(main), _engine.FadeOut(extra), _engine.FadeOut(volume_ghost), _engine.FadeIn(warning), seconds=1.4)
        fixed_w = self._states("warning", _gh.evaluate(defn, {"wn": ["2.5", "2.5"]}), lines=["2.5", "2.5"])
        self._repair(warning, fixed_w, 1)
        self.snapshot("warning_fixed")

        culprit = error.text_panel
        first = _engine.VGroup(_engine.SurroundingRectangle(culprit, buff=0.08, color=RED, stroke_width=3),
                               self._note(self.tr("first problem", "erstes Problem"), culprit.get_right() + np.array([1.0, -0.3, 0.0]), color=RED))
        self.view_state["visible"] = False
        self.step("error", _engine.FadeOut(fixed_w), _engine.FadeIn(error), _engine.FadeIn(first), seconds=1.4)
        self._sweep(fixed_w)
        fixed_e = self._states("error", _gh.evaluate(defn, {"er": ["2.5"]}), lines=["2.5"])
        self.view_state.update(visible=True, box="cbe", overrides={"er": ["2.5"]})
        self._repair(error, fixed_e, 0, extra=(first,))
        self.snapshot("error_fixed")
        self.view_state.update(definition=self.three, box="cb", overrides={})

        wp = self.gh_panel(None, rows=self._rows(full.tree("wp")), at=_at(1.2, 1.38))
        wp_wire = self.gh_wire(w_slider.grip_out(), wp.grip_in(), color=cw)
        mark = _engine.SurroundingRectangle(wp, buff=0.04, color=GREEN, stroke_width=3)
        panel_hop, panel_end = self._hop(self.number(settings["w"]["value"], 1), VIEW + np.array([-0.2, -1.1, 0.0]), wp.get_left() + np.array([-0.5, 0.0, 0.0]), cw)
        self.step("panel", _engine.FadeOut(fixed_e), _engine.FadeIn(main), _engine.FadeIn(wp), _engine.FadeIn(wp_wire), _engine.Create(mark), panel_hop, seconds=1.2)
        self.play(_engine.FadeOut(panel_end), run_time=0.3)
        self._sweep(fixed_e)

        fl = self.gh_param("Number", at=_at(0.9, 1.5), label=self.tr("Width", "Breite"))
        flp = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(2.7, 1.5))
        fl_in, fl_out = self.gh_wire(w_slider.grip_out(), fl.grip_in(), color=cw), self.gh_wire(fl.grip_out(), flp.grip_in(), color=cw)
        ring = _engine.SurroundingRectangle(cb.parts["inputs"]["X"], buff=0.07, color=RED, stroke_width=3)
        x_tag = self._tag(self.tr("input X · Number", "Eingang X · Number"), _at(-1.3, 1.5), color=RED)
        x_lead = self._reach(x_tag, cb.grip_in("X").get_center(), None, color=RED)
        param_hop, param_end = self._hop(self.number(settings["w"]["value"], 1), wp.get_left() + np.array([-0.5, 0.0, 0.0]), fl.get_center(), cw)
        self.step("params", param_hop, _engine.FadeOut(wp), _engine.FadeOut(wp_wire), _engine.FadeOut(mark), _engine.FadeIn(fl), _engine.FadeIn(flp), _engine.FadeIn(fl_in), _engine.FadeIn(fl_out),
                  _engine.Create(ring), _engine.FadeIn(x_tag), _engine.FadeIn(x_lead), seconds=1.4)
        self.play(_engine.FadeOut(param_end), run_time=0.3)

        toggle_off, toggle_on = self.gh_toggle(False, at=_at(-1.9, 0.3), name=self._label("Window")), self.gh_toggle(True, at=_at(-1.9, 0.3), name=self._label("Window"))
        tg_panel = self.gh_panel(None, rows=self._rows(full.tree("tgp")), at=_at(0.2, 0.3))
        tg_wire = self.gh_wire(toggle_on.grip_out(), tg_panel.grip_in())
        self.view_state.update(mode="type", window=False, dims=(), scale=0.5)
        self.play(*[_engine.FadeOut(m) for m in (main, fl, flp, fl_in, fl_out, ring, x_tag, x_lead, extra)], _engine.FadeIn(toggle_off), run_time=0.5)
        self.view_state.update(window=True, dims=("window",))
        window_ring = _engine.Circle(radius=0.3, stroke_color=AMBER, stroke_width=4).move_to(WINDOW_AT).set_z_index(8)
        true_token = self._bit("True", _at(-1.9, 0.3) + np.array([0.5, 0.0, 0.0]), GREEN)
        self.add(true_token)
        self.step("toggle", _engine.FadeOut(toggle_off), _engine.FadeIn(toggle_on), _engine.FadeIn(tg_panel), _engine.FadeIn(tg_wire), _morph.morph(true_token, window_ring, path_arc=0.5), seconds=1.4)
        self.play(_engine.FadeOut(window_ring), run_time=0.3)

        names = [f"{self.tr('Type', 'Typ')} {t.key}" for t in _nk.capsule_types()]
        ref = self.gh_value_list(names, 0, at=_at(-1.9, 0.0), name="Value List", is_open=True)
        vlist = self.live(lambda: self.gh_value_list(names, int(self.vl_index.get_value()), at=_at(-1.9, 0.0), name="Value List", is_open=True), lambda: int(self.vl_index.get_value()))
        vlp = self.live(lambda: self.gh_panel(None, rows=self._rows(_gh.evaluate(defn, {"vl": int(self.vl_index.get_value())}).tree("vlp")), at=_at(0.4, 0.0)), lambda: int(self.vl_index.get_value()))
        vl_wire = self.gh_wire(ref.grip_out().get_center(), _at(-0.1, 0.0))
        vl_note = self.live(lambda: _engine.VGroup(self._note(f"{self.tr('index', 'Index')} {int(self.vl_index.get_value())} → {self.tr('Type', 'Typ')} {'ABCD'[int(self.vl_index.get_value())]}", _at(0.4, -0.6), color=SOFT)),
                            lambda: int(self.vl_index.get_value()))
        type_note = self._type_note()
        self.vl_index.set_value(0)
        self.view_state.update(window=True, dims=("window",), scale=0.5)
        pick_hop, pick_end = self._hop("C", _at(-1.9, 0.0) + np.array([0.4, 0.0, 0.0]), WINDOW_AT + np.array([-0.2, 0.0, 0.0]), _drawing.type_color("C"), kind="item")
        self.step("valuelist", pick_hop, _engine.FadeOut(toggle_on), _engine.FadeOut(tg_panel), _engine.FadeOut(tg_wire), _engine.FadeIn(vlist), _engine.FadeIn(vlp), _engine.FadeIn(vl_wire), _engine.FadeIn(vl_note),
                  _engine.FadeIn(type_note), self._ramp_index(self.vl_index, (0, 1, 3, 2), 2.4), self._type_follow(2.4), seconds=2.4)
        self.play(_engine.FadeOut(pick_end), run_time=0.3)
        self.view_state.update(window=True, dims=())

        sun = self.gh_ribbon(tabs=_canvas.RIBBON_TABS + ("Ladybug",), active="Ladybug", at=RIBBON_AT, width=13.4,
                             groups=[("Sun", ["LB Sunpath", "LB Direct Sun Hours"])], highlight="LB Direct Sun Hours")
        steps = _engine.VGroup(self._tag(self.tr("Package Manager", "Paketverwaltung"), _at(-2.5, 1.2)), self._tag("Food4Rhino", _at(0.0, 1.2)), self._tag("Ladybug.gha", _at(2.4, 1.2)))
        arrows = _engine.VGroup(*[_engine.Arrow(_at(a, 1.2), _at(b, 1.2), buff=0.0, color=DARK, stroke_width=3, max_tip_length_to_length_ratio=0.3) for a, b in ((-1.4, -0.95), (0.95, 1.55))])
        sun_spec = {"name": "LB Direct Sun Hours", "nickname": "Sun", "inputs": [{"name": "Geometry", "nickname": "G", "type": "Geometry"}, {"name": "Sun vectors", "nickname": "V", "type": "Vector"}],
                    "outputs": [{"name": "Hours", "nickname": "H", "type": "Number"}]}
        sun_param = self.gh_param("Brep", at=_at(-3.4, -0.45), label=self.tr("Window", "Fenster"))
        sun_comp = self.gh_component(sun_spec, at=_at(-1.2, -0.45))
        bar = _engine.Rectangle(width=2.0, height=0.3, stroke_color=DARK, stroke_width=2).set_fill(color=["#FFF2B0", "#F08A00"], opacity=1).set_sheen_direction(_engine.RIGHT).move_to(_at(2.5, -0.45))
        fine = _engine.VGroup(self._note(self.tr("install", "installieren"), _at(-2.5, 0.85)), self._note(self.tr("download", "herunterladen"), _at(0.0, 0.85)), self._note(self.tr("plugin file", "Plugin-Datei"), _at(2.4, 0.85)),
                              sun_comp, bar, self.gh_wire(sun_param.grip_out(), sun_comp.grip_in("G")), self.gh_wire(sun_comp.grip_out(), _at(1.45, -0.45)),
                              self._note(self.tr("less sun", "weniger Sonne"), _at(1.7, -0.9)), self._note(self.tr("more sun", "mehr Sonne"), _at(3.3, -0.9)),
                              self._note(self.tr("qualitative, no sun-hour values", "qualitativ, keine Sonnenstunden-Werte"), _at(0.5, -1.4), color=SOFT))
        self.view_state.update(type="A", window=True, dims=(), sun=True)
        sun_ring = _engine.Circle(radius=0.3, stroke_color=AMBER, stroke_width=4).move_to(WINDOW_AT).set_z_index(8)
        sun_token = self._bit("Window", sun_param.get_center(), AMBER, "cell")
        self.add(sun_ring)
        self.step("plugin", _morph.morph(sun_ring, sun_token, pairs=[(sun_ring, sun_token[0])], path_arc=0.5, dip=0.12), _engine.FadeOut(vlist), _engine.FadeOut(vlp), _engine.FadeOut(vl_wire), _engine.FadeOut(vl_note), _engine.FadeOut(type_note),
                  _engine.FadeOut(ribbon), _engine.FadeIn(sun), _engine.FadeIn(steps), _engine.FadeIn(arrows), _engine.FadeIn(fine), seconds=1.6)
        self.play(_engine.FadeOut(sun_token), run_time=0.4)

        self.cnt.set_value(1)
        self.view_state["sun"] = False
        series_ribbon = self.gh_ribbon(tabs=_canvas.RIBBON_TABS + ("Ladybug",), active="Sets", at=RIBBON_AT, width=13.4, groups=[("Sequence", ["Series", "Range"])], highlight="Series")
        st_slider = self._slider("st", _at(-2.3, 0.8), width=3.6)
        cnt_slider = self.gh_slider(self.tr("Count", "Anzahl"), 1, settings["cnt"]["min"], settings["cnt"]["max"], at=_at(-2.3, -0.2), decimals=0, tracker=self.cnt, width=3.6)
        series = self.gh_component("Series", at=_at(0.7, 0.3))
        series_defn = self._sub(("st", "cnt", "se", "sp"), ("st.Number>se.Step", "cnt.Number>se.Count", "se.Series>sp.Stream"))
        count = lambda: int(round(self.cnt.get_value()))
        list_panel = self.live(lambda: self.gh_panel(None, rows=self._rows(_gh.evaluate(series_defn, {"cnt": count()}).tree("sp"), " m"), at=_at(3.5, 0.0), scale=0.9), count)
        wires2 = [self.gh_wire(st_slider.grip_out(), series.grip_in("N")), self.gh_wire(cnt_slider.grip_out(), series.grip_in("C")),
                  self.gh_wire(series.grip_out(), _at(2.95, 0.0), kind="list"),
                  self._note(self.tr("S start 0 · N step · C count", "S Start 0 · N Schritt · C Anzahl"), _at(1.0, -0.55))]

        representatives = {tower.index: {level: next(slot for slot in authored_slots if slot.tower == tower.index and slot.storey == level) for level in range(tower.capsule_storeys)} for tower in _nk.towers()}

        def series_marks(tower_index):
            n = min(count(), len(representatives[tower_index]))
            values = _gh.evaluate(series_defn, {"cnt": n}).items("se", "Series")
            return [(f"ΔZ {i} → {self.number(values[i], 1)} m", (*representatives[tower_index][i].path, representatives[tower_index][i].item), -.6, 0) for i in sorted({0, n - 1})]

        def series_keys(tower_index):
            return {(*slot.path, slot.item) for level, slot in representatives[tower_index].items() if level < count()}

        self.tower_state.update(towers=(1,), marks=lambda: series_marks(1), lit=lambda: (lambda key: key in series_keys(1)), key=lambda: count())
        tower = self._tower()

        def list_points(n):
            panel = self.gh_panel(None, rows=self._rows(_gh.evaluate(series_defn, {"cnt": n}).tree("sp"), " m"), at=_at(3.5, 0.0), scale=0.9)
            return [row.get_center() for row in panel.parts["rows"]][1:], [text for kind, _, text in self._rows(_gh.evaluate(series_defn, {"cnt": n}).tree("sp"), " m") if kind == "item"]

        def level_key(tower_index, level):
            return (*representatives[tower_index][level].path, representatives[tower_index][level].item)

        def level_color(tower_index, level):
            return _drawing.type_color(representatives[tower_index][level].type)

        first_points, first_texts = list_points(1)
        item_end = self._bit(first_texts[0], first_points[0], level_color(1, 0), "item")
        item_hop = self._cap_to(level_key(1, 0), (1,), item_end)
        self.step("item", _engine.FadeOut(steps), _engine.FadeOut(arrows), _engine.FadeOut(fine), _engine.FadeOut(sun), _engine.FadeIn(series_ribbon), _engine.FadeIn(st_slider),
                  _engine.FadeIn(cnt_slider), _engine.FadeIn(series), _engine.FadeIn(list_panel), *[_engine.FadeIn(w) for w in wires2], _engine.FadeOut(preview), _engine.FadeIn(tower), seconds=1.4)
        self.play(item_hop, run_time=1.2)
        self.play(_engine.FadeOut(item_end), run_time=0.3)

        self.cnt.set_value(NB)
        self.step("series", seconds=0.5)
        series_points, series_texts = list_points(NB)
        series_src = [self._cap(level_key(1, i), (1,)) for i in range(NB)]
        series_dst = [self._bit(series_texts[i], series_points[i], level_color(1, i), "item") for i in range(NB)]
        self.add(*series_src)
        self.play(_morph.scatter(series_src, series_dst, lag=0.1, path_arc=0.5, run_time=1.0, limit=2.2, dip=0.12, pairs_fn=_morph.capsule_pairs))
        self.play(*[_engine.FadeOut(t) for t in series_dst], run_time=0.3)

        self.tower_state.update(towers=(0,), marks=lambda: series_marks(0), lit=lambda: (lambda key: key in series_keys(0)))
        self.step("count", self._ramp(self.cnt, NB, NA, 1.6), seconds=1.6)
        count_points, count_texts = list_points(NA)
        count_hops = [self._hop(count_texts[i], count_points[i], self._spot(level_key(0, i), (0,)) + np.array([0.95, 0.4 * (i - (NB + NA - 1) / 2), 0.0]), level_color(0, i)) for i in range(NB, NA)]
        self.play(*[a for a, _ in count_hops], run_time=1.6)
        self.play(*[_engine.FadeOut(t) for _, t in count_hops], run_time=0.4)

        t_param = self.gh_param("Brep", at=_at(-3.2, 0.0), label=self.tr("Capsules", "Kapseln"))
        aggregate = _gh.aggregate_tree()
        tower_branches = [path for path in aggregate.paths() if path[:2] == (0, 0)]
        chosen_path = next(path for path in tower_branches if len(aggregate.branch(path)) >= 2)
        chosen_key = tuple(aggregate.branch(chosen_path)[1].tag["slot"])
        branches = [chosen_path, *[path for path in tower_branches if path != chosen_path][:2]]
        row_indices, cursor = {}, 0
        for path in branches:
            row_indices[path] = tuple(range(cursor + 1, cursor + 1 + len(aggregate.branch(path))))
            cursor += 1 + len(aggregate.branch(path))
        tree_panel = self.gh_panel(None, rows=self._tower_rows(branches), at=_at(-0.9, 0.0), width=2.4, max_lines=32)
        t_wire = self.gh_wire(t_param.grip_out(), tree_panel.grip_in(), kind="tree")
        explain = _engine.VGroup(self._note(self.tr("branch = facade group", "Ast = Fassadengruppe"), _at(0.7, 0.35), anchor="l"),
                                 self._note(self.tr("path = its address", "Pfad = seine Adresse"), _at(0.7, -0.05), anchor="l"),
                                 self._note(self.tr("index = place in branch", "Index = Platz im Ast"), _at(0.7, -0.45), anchor="l"),
                                 self._note(self.tr("a list: {0} · a tree: deeper", "Liste: {0} · Baum: tiefer"), _at(0.7, -0.85), anchor="l", color=SOFT))
        band = self._frame_rows(tree_panel, row_indices[chosen_path], "#97D697")
        path_text = _gh.format_path(chosen_path)
        path_marks = lambda: [(path_text, chosen_key, -.6, .22), (self.tr("facade group", "Fassadengruppe"), chosen_key, -.6, -.22)]
        self.tower_state.update(towers=(0,), marks=path_marks, lit=lambda: (lambda key: key[:4] == chosen_path), key=lambda: "branch")
        chosen_items = aggregate.branch(chosen_path)
        leaf_hops = []
        for i, item in enumerate(chosen_items):
            row = tree_panel.parts["rows"][row_indices[chosen_path][i]]
            target = self._bit("", row.get_left() + np.array([-0.14, 0.0, 0.0]), _drawing.type_color(item.tag["type"]), "item")
            leaf_hops.append((self._cap_to(tuple(item.tag["slot"]), (0,), target), target))
        self.step("tree", _engine.FadeIn(ribbon), *[_engine.FadeOut(m) for m in (st_slider, cnt_slider, series, list_panel, series_ribbon, *wires2)], _engine.FadeIn(t_param), _engine.FadeIn(tree_panel),
                  _engine.FadeIn(t_wire), _engine.FadeIn(explain), _engine.FadeIn(band), seconds=1.6)
        self.play(*[a for a, _ in leaf_hops], run_time=1.2)
        self.play(*[_engine.FadeOut(t) for _, t in leaf_hops], run_time=0.3)
        segment_at = self._spot(chosen_key, (0,)) + np.array([-1.0, 0.0, 0.0])
        segments = [self._bit(str(part), segment_at + np.array([0.0, 0.5 - 0.33 * i, 0.0]), _art.P_PRIMARY) for i, part in enumerate(chosen_path)]
        path_chip = self._bit(path_text, _at(2.6, 1.55), _art.P_PRIMARY, "path")
        self.add(*segments)
        self.play(_morph.gather(segments, path_chip, lag=0.2, run_time=1.0))
        levels = self.tr("BUILDING;TOWER;STORY;FACADE", "GEBÄUDE;TURM;GESCHOSS;FASSADE").split(";")
        path_levels = _engine.VGroup(*[self._note(name + (";" if k < 3 else ""), (0, 0, 0), color=_art.P_PRIMARY) for k, name in enumerate(levels)]).arrange_in_grid(rows=2, cols=2, buff=(0.15, 0.06), col_alignments="ll").next_to(path_chip, _engine.DOWN, buff=0.15)
        self.play(_engine.LaggedStart(*[_engine.FadeIn(level, shift=0.1 * _engine.DOWN) for level in path_levels], lag_ratio=0.5), run_time=1.6)

        every = self._frame_rows(tree_panel, [row_indices[path][1] for path in branches if len(row_indices[path]) > 1], "#F4A640")
        shown = self._note(self.tr(f"3 of {len(tower_branches)} branches shown", f"3 von {len(tower_branches)} Ästen gezeigt"), tree_panel.get_bottom() + np.array([0.0, -0.22, 0.0]), color=SOFT)
        ambiguous = {tuple(aggregate.branch(path)[1].tag["slot"]) for path in tower_branches if len(aggregate.branch(path)) > 1}
        lit_count = len(ambiguous)
        bad = self._tag(self.tr(f"[1] exists in {lit_count} branches · {lit_count} capsules", f"[1] existiert in {lit_count} Ästen · {lit_count} Kapseln"), _at(2.3, -1.4), color=RED)
        expected = self._tag(self.tr("expected: 1 capsule", "erwartet: 1 Kapsel"), _at(2.3, -0.4), color=SOFT)
        self.step("mistake", _engine.FadeOut(band), _engine.FadeOut(explain), _engine.FadeIn(every), seconds=1.4)
        index_row = tree_panel.parts["rows"][row_indices[chosen_path][1]].get_right() + np.array([0.4, 0.0, 0.0])
        index_token = self._bit("[1]", index_row, RED)
        self.add(index_token)
        flock = [_engine.Circle(radius=0.07, stroke_color=RED, stroke_width=2.5, fill_opacity=0).move_to(self._spot(key, (0,))).set_z_index(8) for key in sorted(ambiguous)]
        self.play(_morph.split(index_token, flock, lag=0.04, path_arc=0.3, run_time=0.9, limit=2.4))
        self.tower_state.update(marks=lambda: [], lit=lambda: (lambda key: key in ambiguous), key=lambda: "all")
        self.play(_engine.FadeIn(bad), _engine.FadeIn(expected), _engine.FadeIn(shown), run_time=0.6)

        one = self._frame_rows(tree_panel, (row_indices[chosen_path][1],), "#97D697")
        good = self._tag(self.tr(f"{path_text}, index 1: one capsule", f"{path_text}, Index 1: eine Kapsel"), _at(2.3, -0.4), color=GREEN)
        strike = _engine.Line(bad.get_left(), bad.get_right(), color=RED, stroke_width=4)
        self.tower_state.update(marks=lambda: [], lit=lambda: (lambda key: key == chosen_key), key=lambda: "one")
        final_tag = self._bit(path_text + " [1]", self._spot(chosen_key, (0,)) + np.array([-1.0, 0.0, 0.0]), GREEN, "path")
        self.step("address", _engine.FadeOut(path_chip), _engine.FadeOut(path_levels), _engine.FadeOut(every), _engine.FadeOut(expected), bad.animate.set_opacity(0.45), _engine.Create(strike), _engine.FadeOut(shown), _engine.FadeIn(one), _engine.FadeIn(good), seconds=1.4)
        self.play(_engine.LaggedStart(*[ring.animate(path_arc=0.9).move_to(final_tag.get_center()).set_stroke(opacity=0.0) for ring in flock], lag_ratio=0.05, run_time=2.4))
        self.remove(*flock)
        self.play(_engine.FadeIn(final_tag, scale=0.85), run_time=0.5)

        mini_slider = self._slider("w", _at(-3.2, 0.75), width=2.3, color=cw)
        mini_box = self.gh_component("Center Box", at=_at(-0.45, 0.75), nickname=True)
        mini_param = self.gh_param("Number", at=_at(-1.5, -0.65), label=self.tr("Width", "Breite"))
        mini_panel = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(0.1, -0.65))
        mini_wires = [self.gh_wire(mini_slider.grip_out(), mini_box.grip_in("X"), color=cw), self.gh_wire(mini_slider.grip_out(), mini_param.grip_in(), color=cw),
                      self.gh_wire(mini_param.grip_out(), mini_panel.grip_in(), color=cw)]
        mini_chip = self._tag("Nakagin_Capsule.gh", _at(-3.0, 1.5))
        final_ribbon = self.gh_ribbon(tabs=_canvas.RIBBON_TABS + ("Ladybug",), active="Ladybug", at=RIBBON_AT, width=13.4, groups=[("Sun", ["LB Sunpath", "LB Direct Sun Hours"])], highlight="LB Direct Sun Hours")
        marks = _engine.VGroup(
            self._tie(1, mini_chip.get_right() + np.array([0.3, 0.0, 0.0]), mini_chip.get_right()),
            self._num(2, mini_box.get_top() + np.array([0.0, 0.3, 0.0])), self._num(3, mini_param.get_left() + np.array([-0.4, 0.0, 0.0])),
            self._tie(4, mini_box.grip_in("Y").get_center() + np.array([-0.45, -0.55, 0.0]), mini_box.grip_in("Y").get_center()),
            self._tie(5, mini_box.grip_out().get_center() + np.array([0.45, -0.55, 0.0]), mini_box.grip_out().get_center()),
            self._num(6, mini_slider.get_left() + np.array([0.0, 0.5, 0.0])), self._num(7, mini_panel.get_bottom() + np.array([0.0, -0.3, 0.0])),
            self._num(8, final_ribbon.parts["tiles"]["LB Direct Sun Hours"].get_right() + np.array([0.35, 0.0, 0.0])))
        table = self._table(self.TOOL_TERMS, _at(2.55, -0.05))
        self.tower_state.update(marks=lambda: [], lit=lambda: None, key=lambda: "none")
        table_rest = self._rest(table)
        self.step("legend_tool", *[_engine.FadeOut(m) for m in (t_param, tree_panel, t_wire, one, good, bad, strike, ribbon, final_tag)], _engine.FadeIn(final_ribbon), _engine.FadeIn(mini_chip), _engine.FadeIn(mini_slider),
                  _engine.FadeIn(mini_box), _engine.FadeIn(mini_param), _engine.FadeIn(mini_panel), *[_engine.FadeIn(w) for w in mini_wires], _engine.FadeIn(marks),
                  _engine.LaggedStart(*[_engine.FadeIn(r) for r in table_rest], lag_ratio=0.15), seconds=1.4)
        self.play(self._fly_discs(marks, table, self.TOOL_ORDER))

        data_panel = self.gh_panel(None, rows=self._tower_rows(branches), at=_at(-2.6, -.05), width=2.2, max_lines=32)
        rows = data_panel.parts["rows"]
        left, right = data_panel.body.get_left()[0], data_panel.body.get_right()[0]
        y = lambda index: rows[index].get_center()[1]
        selected_rows = row_indices[chosen_path]
        last_rows = row_indices[branches[-1]]
        header = last_rows[0] - 1
        data_marks = _engine.VGroup(
            self._tie(1, (right + .5, y(selected_rows[0]) + .3, 0), (right, y(selected_rows[0]), 0)),
            self._bracket(2, right, y(selected_rows[0]) + .12, y(selected_rows[-1]) - .12, 1),
            self._bracket(4, right, y(header) + .12, y(last_rows[-1]) - .12, 1),
            self._bracket(3, left, y(0) + .12, y(len(rows) - 1) - .12, -1),
            self._tie(5, (left - .45, y(header), 0), (left, y(header), 0)),
            self._tie(6, (left - .5, y(selected_rows[1]) + .3, 0), (left, y(selected_rows[1]), 0)))
        data_table = self._table(self.DATA_TERMS, _at(2.4, -0.05))
        self.tower_state.update(marks=lambda: [], lit=lambda: (lambda k: k[:4] == chosen_path), key=lambda: "branch2")
        self.step("legend_data", *[_engine.FadeOut(m) for m in (mini_chip, mini_slider, mini_box, mini_param, mini_panel, *mini_wires, marks, table, table_rest)], _engine.FadeIn(data_panel), _engine.FadeIn(data_marks),
                  _engine.LaggedStart(*[_engine.FadeIn(r) for r in self._rest(data_table)], lag_ratio=0.15), seconds=1.2)
        self.play(self._fly_discs(data_marks, data_table, self.DATA_ORDER))
