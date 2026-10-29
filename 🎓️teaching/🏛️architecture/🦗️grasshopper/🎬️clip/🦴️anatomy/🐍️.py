"""🦴 Anatomy of a Grasshopper script, built on the Nakagin capsule: definition, canvas, component, parameter, inputs, panel, states, plugin and item, list, tree."""

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
LABELS = {"Width": ("Width", "Breite"), "Length": ("Length", "Länge"), "Height": ("Height", "Höhe"), "Storey height": ("Storey height", "Geschosshöhe"), "Window": ("Window", "Fenster")}


def _at(x, y):
    """📍 A point on the canvas sheet from offsets to its centre."""
    return SHEET + np.array([x, y, 0.0])


class Beat1_Anatomy(_visuals.ClipScene):
    """🦴 Teach the parts of a Grasshopper script on the parametric Nakagin capsule, then data from item to list to tree."""

    TITLE = {"en": "Anatomy of a Grasshopper Script", "de": "Aufbau eines Grasshopper-Skripts"}
    NARRATION = [
        ["hook", "This is one capsule of the Nakagin Capsule Tower, a simplified teaching model. How do we hand it to a computer?",
         "Das ist eine Kapsel des Nakagin Capsule Tower, ein vereinfachtes Lehrmodell. Wie geben wir sie dem Computer?"],
        ["script", "We describe it as a script: a Grasshopper definition, saved as a .gh file. Its drawing area is the canvas.",
         "Wir beschreiben sie als Skript: eine Grasshopper-Definition, gespeichert als .gh-Datei. Ihre Arbeitsfläche ist der Canvas."],
        ["slider", "A Number Slider is the first input: a name, a domain from 1.0 to 5.0, one decimal and a value, 2.5.",
         "Ein Number Slider ist die erste Eingabe: ein Name, ein Bereich von 1,0 bis 5,0, eine Dezimalstelle und ein Wert, 2,5."],
        ["box", "A component turns inputs into an output. Center Box, nickname Box, takes width, length and height and builds a box; the viewport shows it.",
         "Eine Komponente macht aus Eingaben eine Ausgabe: Center Box, Kurzname Box, baut aus Breite, Länge und Höhe eine Box."],
        ["parts", "Every component has a name and a nickname, and the canvas shows the nickname. Inputs sit on the left, outputs on the right, each with a grip.",
         "Jede Komponente hat Namen und Kurznamen; der Canvas zeigt den Kurznamen. Eingänge liegen links, Ausgänge rechts, je mit einem Griff."],
        ["flow", "Wires carry data from an output grip to an input grip and normally flow from left to right.",
         "Verbindungen tragen Daten von einem Ausgangsgriff zu einem Eingangsgriff und laufen normalerweise von links nach rechts."],
        ["sliders", "Each slider is one dimension: wire and dimension line share its colour. Length 5.0 and height 3.0 metres make the box grow.",
         "Jeder Schieberegler ist ein Maß: Verbindung und Maßlinie teilen seine Farbe. Länge 5,0 und Höhe 3,0 Meter lassen die Box wachsen."],
        ["live", "Drag a slider and the result follows. The width goes from 2.5 to 3.0 metres and back, and the box changes with it.",
         "Zieht man einen Schieberegler, folgt das Ergebnis. Die Breite geht von 2,5 auf 3,0 Meter und zurück, und die Box ändert sich mit."],
        ["preview", "The preview eye hides only the drawing, not the data: the box disappears, the volume still reads 25 cubic metres.",
         "Das Vorschau-Auge blendet nur die Zeichnung aus, nicht die Daten: Die Box verschwindet, das Volumen zeigt weiter 25 Kubikmeter."],
        ["warning", "Grey is fine. A number and a text in Z turn it orange, a warning: one box, one null. Replace the text by 2.5 and it is grey again.",
         "Grau ist in Ordnung. Zahl und Text in Z machen sie orange, eine Warnung: eine Box, ein null. Mit 2.5 statt tall ist sie grau."],
        ["error", "With nothing usable it turns red, an error: a wire cannot make the text tall a height. Give Z the number 2.5 and the box returns.",
         "Ohne Brauchbares wird sie rot, ein Fehler: Aus dem Text tall macht keine Verbindung eine Höhe. Mit dem Text 2.5 in Z kehrt die Box zurück."],
        ["panel", "A Panel shows what a wire carries: path {0}, index 0, value 2.5. It makes a hidden number readable.",
         "Ein Panel zeigt, was eine Verbindung trägt: Pfad {0}, Index 0, Wert 2,5. Es macht eine verborgene Zahl lesbar."],
        ["params", "A parameter holds typed data. Floating it stands alone; as input X of a component it belongs to a port. Both hold the Number 2.5.",
         "Ein Parameter hält Daten eines Typs. Frei steht er allein; als Eingang X einer Komponente gehört er zu einem Port. Beide halten die Zahl 2,5."],
        ["toggle", "A Boolean Toggle has two states. True gives the capsule its round window, 1.3 metres across, in the front face.",
         "Ein Boolean Toggle hat zwei Zustände. True gibt der Kapsel ihr rundes Fenster, 1,3 Meter im Durchmesser, in der Stirnseite."],
        ["valuelist", "A Value List offers fixed choices: the capsule types A to D. Pick one and the capsule takes that type's window.",
         "Eine Werteliste bietet feste Auswahlmöglichkeiten: die Kapseltypen A bis D. Wählt man einen, übernimmt die Kapsel dessen Fenster."],
        ["plugin", "A plugin, a .gha file, adds a tab to the ribbon. Ladybug Tools studies the sun on the capsule window: more sun, warmer tint.",
         "Ein Plugin, eine .gha-Datei, fügt dem Ribbon einen Reiter hinzu. Ladybug Tools untersucht die Sonne am Fenster: mehr Sonne, wärmere Tönung."],
        ["item", "Now the data. One value is an item: the height 0 m of the first storey, one capsule on Tower A.",
         "Nun die Daten. Ein Wert ist ein Element: die Höhe 0 m des ersten Geschosses, eine Kapsel an Turm A."],
        ["series", "A Series with step 2.75 metres makes a list: nine heights in one branch {0}, indices 0 to 8, one capsule on each of nine storeys.",
         "Eine Series mit Schrittweite 2,75 Meter erzeugt eine Liste: neun Höhen in einem Ast {0}, Indizes 0 bis 8, je eine Kapsel pro Geschoss."],
        ["count", "Change Count from 9 to 11 and the list grows by two: Tower B has eleven capsule storeys.",
         "Ändert man den Schieberegler Anzahl von 9 auf 11, wächst die Liste um zwei: Turm B hat elf Kapselgeschosse."],
        ["tree", "Many lists form a data tree. Each facade of a storey is one branch with its own path, here {0;0;3;1}, holding two capsule items.",
         "Viele Listen bilden einen Datenbaum. Jede Fassade eines Geschosses ist ein Ast mit eigenem Pfad, hier {0;0;3;1}, und enthält zwei Kapseln."],
        ["mistake", "A common mistake is to read a tree as one long list. The index 1 alone fits all 27 branches of Tower A, so it names 27 capsules.",
         "Typischer Fehler: einen Datenbaum wie eine lange Liste lesen. Der Index 1 allein passt zu allen 27 Ästen von Turm A: 27 Kapseln."],
        ["address", "Only path plus index names exactly one capsule: path {0;0;3;1}, index 1. The Panel shows both.",
         "Erst Pfad plus Index nennt genau eine Kapsel: Pfad {0;0;3;1}, Index 1. Das Panel zeigt beides."],
        ["legend_tool", "The canvas terms, row by row in English and German: script, Skript; component, Komponente; and six more.",
         "Die Begriffe des Canvas: Skript, Komponente, Parameter, Eingang, Ausgang, Schieberegler, Panel und Plugin. Englisch steht daneben."],
        ["legend_data", "The data terms in English and German: item, Element; list, Liste; tree, Datenbaum; branch, Ast; path, Pfad; index.",
         "Die Begriffe der Daten: Element, Liste, Datenbaum, Ast, Pfad und Index. Die englischen Wörter stehen daneben."],
    ]

    TOOL_TERMS = (("Script", "Skript"), ("Component", "Komponente"), ("Parameter", "Parameter"), ("Input", "Eingang"), ("Output", "Ausgang"),
                  ("Slider", "Schieberegler"), ("Panel", "Panel"), ("Plugin", "Plugin"))
    DATA_TERMS = (("Item", "Element"), ("List", "Liste"), ("Tree", "Datenbaum"), ("Branch", "Ast"), ("Path", "Pfad"), ("Index", "Index"))

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
                shape, diameter = {"width": x, "length": y, "height": z, "corner_radius": 0.0, "key": "A"}, _nk.capsule_type("A").window_diameter
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
            drawing = self.tower_axon(at=VIEW + np.array([0.0, state["dy"], 0.0]), scale=state["scale"], lit=state["lit"](), segments=1, ground=False, types=False,
                                      towers=state["towers"], podium=state["podium"])
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
        group.text_panel = text
        return group

    def present_anatomy(self):
        """🎞️ Run the clauses: script, slider, component, wires, states, panel, parameters, inputs, plugin, then item, list and tree."""
        defn = _gh.canvas("anatomy")
        full = _gh.evaluate(defn)
        settings = {k: defn.objects[k].settings for k in ("w", "bl", "bh", "st", "cnt")}
        self.W, self.L, self.H = (_engine.ValueTracker(settings[k]["value"]) for k in ("w", "bl", "bh"))
        self.three = self._sub(("w", "bl", "bh", "xy", "cb", "vol", "vp"), ("w.Number>cb.X", "bl.Number>cb.Y", "bh.Number>cb.Z", "xy.Plane>cb.Base", "cb.Box>vol.Geometry", "vol.Volume>vp.Stream"))
        self.view_state = {"visible": True, "mode": "type", "type": "A", "window": True, "dims": ("width", "length", "height"), "sun": False, "scale": 0.42,
                           "definition": self.three, "box": "cb", "overrides": {}}
        self.tower_state = {"lit": lambda: None, "key": lambda: None, "towers": (0,), "scale": 0.092, "dy": -0.15, "podium": False, "marks": lambda: []}
        self.cnt = _engine.ValueTracker(1)
        self.vl_index = _engine.ValueTracker(0)
        cw, cl, ch = DIMCOL["width"], DIMCOL["length"], DIMCOL["height"]

        sheet = self.gh_background(at=SHEET, width=SHEET_W, height=SHEET_H)
        ribbon = self.gh_ribbon(tabs=_canvas.RIBBON_TABS, active="Params", at=RIBBON_AT, width=13.4,
                                groups=[("Input", ["Panel", "Number Slider", "Boolean Toggle", "Value List"]), ("Geometry", ["Point", "Curve", "Brep", "Box"]), ("Util", ["Param Viewer"])])
        viewport = self.gh_viewport(at=VIEW, width=VIEW_W, height=VIEW_H, label=self.tr("Perspective", "Perspektive"))
        preview = self._preview()
        model_note = self.text(self.tr("Teaching model · simplified, rounded", "Lehrmodell · vereinfacht, gerundet"), (6.85, 3.62, 0), size=SMALL, color=MUTED, anchor="r")

        slots_n = len(_nk.slots())
        whole = self.tower_axon(at=VIEW + np.array([0.0, -0.05, 0.0]), scale=0.065, lit=lambda k: k == (0, 0, 3, 1, 1), segments=1, ground=False, types=False, towers=(0, 1), podium=True)
        whole_note = self._note(self.tr(f"1 of {slots_n} capsules", f"1 von {slots_n} Kapseln"), VIEW + np.array([VIEW_W / 2 - 0.12, VIEW_H / 2 - 0.24, 0.0]), anchor="r")
        self.step("hook", _engine.FadeIn(sheet), _engine.FadeIn(ribbon), _engine.FadeIn(viewport), _engine.FadeIn(whole), _engine.FadeIn(whole_note), _engine.FadeIn(model_note), seconds=1.4)

        chip = self._tag("Nakagin_Capsule.gh", _at(-3.0, 1.45))
        canvas_tag = self._tag("Canvas", _at(3.6, 1.45))
        frame = _engine.SurroundingRectangle(sheet, buff=0.0, color=_art.P_PRIMARY, stroke_width=3)
        kinds = _nk.capsule_types()
        strip = _engine.VGroup(*[self.capsule_axon(t, at=_at(-3.0 + 2.0 * i, 0.2), scale=0.2, base=_canvas.PREVIEW_FILL, window=True) for i, t in enumerate(kinds)],
                               *[self._note(t.key, _at(-3.0 + 2.0 * i, -0.45)) for i, t in enumerate(kinds)],
                               self._note(self.tr(f"one definition: {len(kinds)} types, {slots_n} placements", f"eine Definition: {len(kinds)} Typen, {slots_n} Plätze"), _at(0.0, -1.0), color=SOFT))
        self.step("script", _engine.FadeOut(whole), _engine.FadeOut(whole_note), _engine.FadeIn(preview), _engine.FadeIn(chip), _engine.Create(frame), _engine.FadeIn(canvas_tag), _engine.FadeIn(strip), _engine.FadeOut(model_note), seconds=1.2)
        self.play(_engine.FadeOut(frame), run_time=0.3)

        w_slider = self._slider("w", _at(SL_X, 1.0), self.W, color=cw)
        notes = _engine.VGroup(self._note(self.tr("Name", "Name"), _at(-3.95, 1.4)), self._note(self.tr("Value", "Wert"), _at(-2.95, 1.4)),
                               self._note(self.tr("Domain 1.0 to 5.0 · one decimal", "Bereich 1,0 bis 5,0 · 1 Dezimalstelle"), _at(SL_X, 0.1)))
        self.view_state["dims"] = ("width",)
        self.W.set_value(settings["w"]["min"])
        self.step("slider", _engine.FadeOut(chip), _engine.FadeOut(canvas_tag), _engine.FadeOut(strip), _engine.FadeIn(w_slider), _engine.FadeIn(notes), self._ramp(self.W, settings["w"]["min"], settings["w"]["value"], 1.4), seconds=1.4)

        l_slider, h_slider = self._slider("bl", _at(SL_X, 0.2), self.L, color=cl), self._slider("bh", _at(SL_X, -0.6), self.H, color=ch)
        cb_full = self.gh_component("Center Box", at=_at(*CB_AT))
        full_wires = [self.gh_wire(w_slider.grip_out(), cb_full.grip_in("X"), color=cw), self.gh_wire(l_slider.grip_out(), cb_full.grip_in("Y"), color=cl),
                      self.gh_wire(h_slider.grip_out(), cb_full.grip_in("Z"), color=ch)]
        self.view_state.update(mode="box", window=False, dims=("width", "length", "height"), scale=0.46)
        self.step("box", _engine.FadeOut(notes), _engine.FadeIn(l_slider), _engine.FadeIn(h_slider), _engine.FadeIn(cb_full), *[_engine.FadeIn(w) for w in full_wires], seconds=1.4)

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

        l0, h0 = settings["bl"]["value"], settings["bh"]["value"]
        self.step("sliders", self._ramp(self.L, l0, l0 + 1.0, 2.0), self._ramp(self.H, h0, h0 + 0.5, 2.0), seconds=2.0)
        self.play(self._ramp(self.L, l0 + 1.0, l0, 1.0), self._ramp(self.H, h0 + 0.5, h0, 1.0), run_time=1.0)

        w0, w1 = settings["w"]["value"], settings["w"]["value"] + 0.5
        readout = self.live_label(lambda: f"X = {self.number(self.W.get_value(), 1)} m", _at(0.5, 1.5), size=SMALL, color=cw)
        self.step("live", _engine.FadeIn(readout), self._ramp(self.W, w0, w1, 2.0), seconds=2.0)
        self.play(self._ramp(self.W, w1, w0, 1.2), run_time=1.2)

        vol = self.gh_component("Volume", at=_at(2.0, 0.35))
        vp = self.gh_panel(None, rows=self._rows(_gh.evaluate(self.three).tree("vp"), " m³"), at=_at(3.7, 0.35), width=1.5)
        eye = self._eye(cb)
        vol_wire, vp_wire = self.gh_wire(cb.grip_out(), vol.grip_in()), self.gh_wire(vol.grip_out("V"), vp.grip_in())
        self.play(_engine.FadeOut(readout), _engine.FadeIn(vol), _engine.FadeIn(vp), _engine.FadeIn(vol_wire), _engine.FadeIn(vp_wire), _engine.FadeIn(eye), run_time=0.8)
        self.view_state["visible"] = False
        self.step("preview", _engine.Transform(eye, self._eye(cb, on=False)), seconds=1.2)

        main = _engine.VGroup(w_slider, l_slider, h_slider, cb, *wires, heads)
        extra = _engine.VGroup(vol, vp, vol_wire, vp_wire, eye)
        warn_sol, error_sol = _gh.evaluate(defn), _gh.evaluate(defn)
        warning, error = self._states("warning", warn_sol), self._states("error", error_sol)
        self.view_state.update(visible=True, definition=defn, box="cbw")
        self.step("warning", _engine.FadeOut(main), _engine.FadeOut(extra), _engine.FadeIn(warning), seconds=1.4)
        fixed_w = self._states("warning", _gh.evaluate(defn, {"wn": ["2.5"]}), lines=["2.5"])
        self.play(_engine.FadeOut(warning), _engine.FadeIn(fixed_w), run_time=1.0)
        self.snapshot("warning_fixed")

        culprit = error.text_panel
        first = _engine.VGroup(_engine.SurroundingRectangle(culprit, buff=0.08, color=RED, stroke_width=3),
                               self._note(self.tr("first problem", "erstes Problem"), culprit.get_right() + np.array([1.0, -0.3, 0.0]), color=RED))
        self.view_state["visible"] = False
        self.step("error", _engine.FadeOut(fixed_w), _engine.FadeIn(error), _engine.FadeIn(first), seconds=1.4)
        fixed_e = self._states("error", _gh.evaluate(defn, {"er": ["2.5"]}), lines=["2.5"])
        self.view_state.update(visible=True, box="cbe", overrides={"er": ["2.5"]})
        self.play(_engine.FadeOut(error), _engine.FadeOut(first), _engine.FadeIn(fixed_e), run_time=1.0)
        self.snapshot("error_fixed")
        self.view_state.update(definition=self.three, box="cb", overrides={})

        wp = self.gh_panel(None, rows=self._rows(full.tree("wp")), at=_at(1.2, 1.38))
        wp_wire = self.gh_wire(w_slider.grip_out(), wp.grip_in(), color=cw)
        mark = _engine.SurroundingRectangle(wp, buff=0.04, color=GREEN, stroke_width=3)
        self.step("panel", _engine.FadeOut(fixed_e), _engine.FadeIn(main), _engine.FadeIn(wp), _engine.FadeIn(wp_wire), _engine.Create(mark), seconds=1.2)

        fl = self.gh_param("Number", at=_at(0.9, 1.5), label=self.tr("Width", "Breite"))
        flp = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(2.7, 1.5))
        fl_in, fl_out = self.gh_wire(w_slider.grip_out(), fl.grip_in(), color=cw), self.gh_wire(fl.grip_out(), flp.grip_in(), color=cw)
        ring = _engine.SurroundingRectangle(cb.parts["inputs"]["X"], buff=0.07, color=RED, stroke_width=3)
        x_tag = self._tag(self.tr("input X · Number", "Eingang X · Number"), _at(-1.3, 1.5), color=RED)
        x_lead = self._reach(x_tag, cb.grip_in("X").get_center(), None, color=RED)
        self.step("params", _engine.FadeOut(wp), _engine.FadeOut(wp_wire), _engine.FadeOut(mark), _engine.FadeIn(fl), _engine.FadeIn(flp), _engine.FadeIn(fl_in), _engine.FadeIn(fl_out),
                  _engine.Create(ring), _engine.FadeIn(x_tag), _engine.FadeIn(x_lead), seconds=1.4)

        toggle_off, toggle_on = self.gh_toggle(False, at=_at(-1.9, 0.3), name=self._label("Window")), self.gh_toggle(True, at=_at(-1.9, 0.3), name=self._label("Window"))
        tg_panel = self.gh_panel(None, rows=self._rows(full.tree("tgp")), at=_at(0.2, 0.3))
        tg_wire = self.gh_wire(toggle_on.grip_out(), tg_panel.grip_in())
        self.view_state.update(mode="type", window=False, dims=(), scale=0.5)
        self.play(*[_engine.FadeOut(m) for m in (main, fl, flp, fl_in, fl_out, ring, x_tag, x_lead, extra)], _engine.FadeIn(toggle_off), run_time=0.5)
        self.view_state.update(window=True, dims=("window",))
        self.step("toggle", _engine.FadeOut(toggle_off), _engine.FadeIn(toggle_on), _engine.FadeIn(tg_panel), _engine.FadeIn(tg_wire), seconds=1.4)

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
        self.step("valuelist", _engine.FadeOut(toggle_on), _engine.FadeOut(tg_panel), _engine.FadeOut(tg_wire), _engine.FadeIn(vlist), _engine.FadeIn(vlp), _engine.FadeIn(vl_wire), _engine.FadeIn(vl_note),
                  _engine.FadeIn(type_note), self._ramp_index(self.vl_index, (0, 1, 3, 2), 2.4), self._type_follow(2.4), seconds=2.4)
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
                              sun_param, sun_comp, bar, self.gh_wire(sun_param.grip_out(), sun_comp.grip_in("G")), self.gh_wire(sun_comp.grip_out(), _at(1.45, -0.45)),
                              self._note(self.tr("less sun", "weniger Sonne"), _at(1.7, -0.9)), self._note(self.tr("more sun", "mehr Sonne"), _at(3.3, -0.9)),
                              self._note(self.tr("qualitative, no sun-hour values", "qualitativ, keine Sonnenstunden-Werte"), _at(0.5, -1.4), color=SOFT))
        self.view_state.update(type="A", window=True, dims=(), sun=True)
        self.step("plugin", _engine.FadeOut(vlist), _engine.FadeOut(vlp), _engine.FadeOut(vl_wire), _engine.FadeOut(vl_note), _engine.FadeOut(type_note),
                  _engine.FadeOut(ribbon), _engine.FadeIn(sun), _engine.FadeIn(steps), _engine.FadeIn(arrows), _engine.FadeIn(fine), seconds=1.6)

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

        def series_marks(tower_index):
            n = count()
            values = _gh.evaluate(series_defn, {"cnt": n}).items("se", "Series")
            return [(f"{i} → {self.number(values[i], 1)} m", (0, tower_index, i, 1, 0), -0.6, 0.0) for i in sorted({0, n - 1})]

        self.tower_state.update(marks=lambda: series_marks(0), lit=lambda: (lambda k: k[0] == 0 and k[1] == 0 and k[3] == 1 and k[4] == 0 and k[2] < count()), key=lambda: count())
        tower = self._tower()
        self.step("item", _engine.FadeOut(steps), _engine.FadeOut(arrows), _engine.FadeOut(fine), _engine.FadeOut(sun), _engine.FadeIn(series_ribbon), _engine.FadeIn(st_slider),
                  _engine.FadeIn(cnt_slider), _engine.FadeIn(series), _engine.FadeIn(list_panel), *[_engine.FadeIn(w) for w in wires2], _engine.FadeOut(preview), _engine.FadeIn(tower), seconds=1.4)

        self.step("series", self._ramp(self.cnt, 1, 9, 2.0), seconds=2.0)

        self.tower_state.update(towers=(1,), scale=0.075, marks=lambda: series_marks(1), lit=lambda: (lambda k: k[0] == 0 and k[1] == 1 and k[3] == 1 and k[4] == 0 and k[2] < count()))
        self.step("count", self._ramp(self.cnt, 9, 11, 1.6), seconds=1.6)

        t_param = self.gh_param("Brep", at=_at(-3.2, 0.0), label=self.tr("Capsules", "Kapseln"))
        branches = [(0, 0, 3, f) for f in (0, 1, 2)]
        tree_panel = self.gh_panel(None, rows=self._tower_rows(branches), at=_at(-0.9, 0.0), width=2.4)
        t_wire = self.gh_wire(t_param.grip_out(), tree_panel.grip_in(), kind="tree")
        explain = _engine.VGroup(self._note(self.tr("branch = one facade", "Ast = eine Fassade"), _at(1.1, 0.7), anchor="l"),
                                 self._note(self.tr("path = its address", "Pfad = seine Adresse"), _at(1.1, 0.25), anchor="l"),
                                 self._note(self.tr("index = place in branch", "Index = Platz im Ast"), _at(1.1, -0.2), anchor="l"),
                                 self._note(self.tr("a list: {0} · a tree: deeper", "Liste: {0} · Baum: tiefer"), _at(1.1, -0.65), anchor="l", color=SOFT))
        band = self._frame_rows(tree_panel, (4, 5), "#97D697")
        path_marks = lambda: [(self.tr("storey 3", "Geschoss 3"), (0, 0, 3, 1, 0), -0.6, 0.22), (self.tr("facade 1 (E)", "Fassade 1 (O)"), (0, 0, 3, 1, 0), -0.6, -0.22)]
        self.tower_state.update(towers=(0,), scale=0.092, marks=path_marks, lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch")
        self.step("tree", _engine.FadeIn(ribbon), *[_engine.FadeOut(m) for m in (st_slider, cnt_slider, series, list_panel, series_ribbon, *wires2)], _engine.FadeIn(t_param), _engine.FadeIn(tree_panel),
                  _engine.FadeIn(t_wire), _engine.FadeIn(explain), _engine.FadeIn(band), seconds=1.6)

        tower_branches = [p for p in _gh.aggregate_tree().paths() if p[:2] == (0, 0)]
        every = self._frame_rows(tree_panel, (2, 5, 8), "#F4A640")
        shown = self._note(self.tr(f"3 of {len(tower_branches)} branches shown", f"3 von {len(tower_branches)} Ästen gezeigt"), tree_panel.get_bottom() + np.array([0.0, -0.22, 0.0]), color=SOFT)
        lit_count = len(tower_branches)
        bad = self._tag(self.tr(f"[1] fits {len(tower_branches)} branches · {lit_count} capsules", f"[1] passt zu {len(tower_branches)} Ästen · {lit_count} Kapseln"), _at(2.3, -0.9), color=RED)
        self.tower_state.update(marks=lambda: [], lit=lambda: (lambda k: k[0] == 0 and k[1] == 0 and k[4] == 1), key=lambda: "all")
        expected = self._tag(self.tr("expected: 1 capsule", "erwartet: 1 Kapsel"), _at(2.3, -0.4), color=SOFT)
        self.step("mistake", _engine.FadeOut(band), _engine.FadeOut(explain), _engine.FadeIn(every), _engine.FadeIn(bad), _engine.FadeIn(expected), _engine.FadeIn(shown), seconds=1.4)

        one = self._frame_rows(tree_panel, (5,), "#97D697")
        good = self._tag(self.tr("{0;0;3;1}, index 1: one capsule", "{0;0;3;1}, Index 1: eine Kapsel"), _at(2.3, -0.4), color=GREEN)
        strike = _engine.Line(bad.get_left(), bad.get_right(), color=RED, stroke_width=4)
        self.tower_state.update(marks=lambda: [("{0;0;3;1} [1]", (0, 0, 3, 1, 1), -0.6, 0.0)], lit=lambda: (lambda k: k == (0, 0, 3, 1, 1)), key=lambda: "one")
        self.step("address", _engine.FadeOut(every), _engine.FadeOut(expected), bad.animate.set_opacity(0.45), _engine.Create(strike), _engine.FadeOut(shown), _engine.FadeIn(one), _engine.FadeIn(good), seconds=1.4)

        mini_slider = self._slider("w", _at(-3.2, 0.75), width=2.3, color=cw)
        mini_box = self.gh_component("Center Box", at=_at(-0.7, 0.75), nickname=True)
        mini_param = self.gh_param("Number", at=_at(-1.5, -0.65), label=self.tr("Width", "Breite"))
        mini_panel = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(0.1, -0.65))
        mini_wires = [self.gh_wire(mini_slider.grip_out(), mini_box.grip_in("X"), color=cw), self.gh_wire(mini_slider.grip_out(), mini_param.grip_in(), color=cw),
                      self.gh_wire(mini_param.grip_out(), mini_panel.grip_in(), color=cw)]
        mini_chip = self._tag("Nakagin_Capsule.gh", _at(-3.0, 1.5))
        final_ribbon = self.gh_ribbon(tabs=_canvas.RIBBON_TABS + ("Ladybug",), active="Ladybug", at=RIBBON_AT, width=13.4, groups=[("Sun", ["LB Sunpath", "LB Direct Sun Hours"])], highlight="LB Direct Sun Hours")
        marks = _engine.VGroup(
            self._tie(1, mini_chip.get_right() + np.array([0.55, 0.0, 0.0]), mini_chip.get_right()),
            self._num(2, mini_box.get_top() + np.array([0.0, 0.3, 0.0])), self._num(3, mini_param.get_left() + np.array([-0.4, 0.0, 0.0])),
            self._tie(4, mini_box.grip_in("Y").get_center() + np.array([-0.45, -0.55, 0.0]), mini_box.grip_in("Y").get_center()),
            self._tie(5, mini_box.grip_out().get_center() + np.array([0.45, -0.55, 0.0]), mini_box.grip_out().get_center()),
            self._num(6, mini_slider.get_left() + np.array([0.0, 0.5, 0.0])), self._num(7, mini_panel.get_bottom() + np.array([0.0, -0.3, 0.0])),
            self._num(8, final_ribbon.parts["tiles"]["LB Direct Sun Hours"].get_right() + np.array([0.35, 0.0, 0.0])))
        table = self._table(self.TOOL_TERMS, _at(2.55, -0.05))
        self.tower_state.update(marks=lambda: [], lit=lambda: None, key=lambda: "none")
        self.step("legend_tool", *[_engine.FadeOut(m) for m in (t_param, tree_panel, t_wire, one, good, bad, strike, ribbon)], _engine.FadeIn(final_ribbon), _engine.FadeIn(mini_chip), _engine.FadeIn(mini_slider),
                  _engine.FadeIn(mini_box), _engine.FadeIn(mini_param), _engine.FadeIn(mini_panel), *[_engine.FadeIn(w) for w in mini_wires], _engine.FadeIn(marks),
                  _engine.LaggedStart(*[_engine.FadeIn(r) for r in table], lag_ratio=0.15), seconds=2.4)

        data_panel = self.gh_panel(None, rows=self._tower_rows(branches), at=_at(-2.6, -0.05), width=2.2)
        rows = data_panel.parts["rows"]
        left, right = data_panel.body.get_left()[0], data_panel.body.get_right()[0]
        y = lambda i: rows[i].get_center()[1]
        data_marks = _engine.VGroup(
            self._tie(1, (right + 0.45, y(1), 0), (right, y(1), 0)), self._bracket(2, right, y(4) + 0.12, y(5) - 0.12, 1), self._bracket(4, right, y(6) + 0.12, y(8) - 0.12, 1),
            self._bracket(3, left, y(0) + 0.12, y(8) - 0.12, -1), self._tie(5, (left - 0.45, y(6), 0), (left, y(6), 0)), self._tie(6, (left - 0.45, y(2), 0), (left, y(2), 0)))
        data_table = self._table(self.DATA_TERMS, _at(2.4, -0.05))
        self.tower_state.update(marks=lambda: [], lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch2")
        self.step("legend_data", *[_engine.FadeOut(m) for m in (mini_chip, mini_slider, mini_box, mini_param, mini_panel, *mini_wires, marks, table)], _engine.FadeIn(data_panel), _engine.FadeIn(data_marks),
                  _engine.LaggedStart(*[_engine.FadeIn(r) for r in data_table], lag_ratio=0.15), seconds=2.0)
