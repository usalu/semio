"""📍 References on the Nakagin workbook: relative, absolute, mixed, sheet, 3D and named references, and the ways they break."""

import importlib
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
_xl = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_nd = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_m = _visuals._engine
_S = {}

INK, MUTED, ARCH, SERV, DATA, GOOD, WARN, GLASS = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "SERV", "DATA", "GOOD", "WARN", "GLASS"))
BACKGROUND = _draft.BACKGROUND
LABEL, SMALL = 19.2, 16.2
SHEET = "Type A"
WIN_AT, WIN_W, WIN_H = (-2.85, 0.0, 0.0), 8.0, 5.1
RIGHT_X = 4.0
SURFACES = (("roof", "top"), ("floor", "bottom"), ("wall_left", "left"), ("wall_right", "right"), ("front", "front"), ("window", "front"), ("back", "back"))
LOSS_FORMAT = '#,##0 "W"'
KELVIN = '0.0 "K"'
WK = '0.00 "W/K"'


def _put(shape, point, edge="c"):
    """📌 Place a shape by one of its edges (c, l, r, u, d) on a scene point."""
    anchor = {"c": shape.get_center, "l": shape.get_left, "r": shape.get_right, "u": shape.get_top, "d": shape.get_bottom}[edge]()
    return shape.shift(np.array([point[0], point[1], 0.0]) - anchor)


class Beat2_References(_visuals.ClipScene):
    """📍 Teach references on the Nakagin workbook: pointer, relative fill, absolute and its mistake, mixed, sheet, 3D, names, and the deleted sheet."""

    TITLE = {"en": "Cell References", "de": "Zellbezüge"}
    NARRATION = [
        ["hook", "In this teaching model of the Nakagin tower, the width of every type A capsule sits in one cell. What follows when it changes?",
         "In diesem Lehrmodell des Nakagin-Turms steht die Breite jeder Typ-A-Kapsel in einer Zelle. Was folgt, wenn sie sich ändert?"],
        ["pointer", "A reference is an address, not a number. D3 points to B2 and B3, so a wider capsule gives a larger roof area.",
         "Ein Bezug ist eine Adresse, keine Zahl. D3 zeigt auf B2 und B3, eine breitere Kapsel ergibt also eine größere Dachfläche."],
        ["relative", "One formula per surface is tedious. Write U·A once in F3 and fill it down: the formula travels with the row.",
         "Für jedes Bauteil eine eigene Formel ist mühsam. U·A einmal in F3 schreiben und nach unten ausfüllen: Die Formel wandert mit der Zeile."],
        ["shift", "Copied one row down, D3*E3 becomes D4*E4. Relative parts shift by the offset, here one row.",
         "Eine Zeile tiefer kopiert, wird aus D3*E3 die Formel D4*E4. Relative Teile verschieben sich um den Versatz, hier eine Zeile."],
        ["lights", "Each row computes one surface. Filled down, each surface lights up on the capsule as its U·A value appears.",
         "Jede Zeile berechnet ein Bauteil. Beim Ausfüllen leuchtet jedes Bauteil an der Kapsel auf, sobald sein U·A-Wert erscheint."],
        ["back", "The back wall touches the core, so its formula also carries the factor b: a filled formula must fit every row.",
         "Die Rückwand liegt am Kern, ihre Formel trägt daher noch den Faktor b: Eine ausgefüllte Formel muss zu jeder Zeile passen."],
        ["total", "Add the column with SUM: the total equals the transmission H_T that cell B11 already holds.",
         "Die Spalte mit SUMME addieren: Die Summe entspricht dem Transmissionsverlust H_T, den Zelle B11 schon enthält."],
        ["fixed", "Heat loss per surface needs the design temperature difference, kept in one cell, D10. Every row must point to that cell.",
         "Der Wärmeverlust je Bauteil braucht die Auslegungs-Temperaturdifferenz, abgelegt in einer Zelle, D10. Jede Zeile muss auf diese Zelle zeigen."],
        ["f4", "A dollar sign before column and row makes a reference absolute. The F4 key cycles through the four forms.",
         "Ein Dollarzeichen vor Spalte und Zeile macht einen Bezug absolut. Die Taste F4 wechselt durch die vier Formen."],
        ["absolute", "Filled down, the absolute reference stays on the input in every row: each surface sees the same temperature difference.",
         "Nach unten ausgefüllt bleibt der absolute Bezug in jeder Zeile auf der Eingabe: Jedes Bauteil sieht dieselbe Temperaturdifferenz."],
        ["slide", "The typical mistake: forget the dollar sign and fill down. The reference slides off the input and the values go wrong.",
         "Der typische Fehler: Das Dollarzeichen vergessen und ausfüllen. Der Bezug rutscht von der Eingabe ab, die Werte werden falsch."],
        ["repair", "Press F4 on the reference, fill again, and each row meets the same input. Values and surfaces match again.",
         "Mit F4 den Bezug sperren und erneut ausfüllen: Jede Zeile trifft dieselbe Eingabe. Werte und Bauteile stimmen wieder."],
        ["mixed2", "A two-way table: U-values down, temperature differences across. One mixed formula, filled right and down, gives every heat flux.",
         "Eine Kreuztabelle: U-Werte links, Temperaturdifferenzen oben. Eine gemischte Formel, ausgefüllt, ergibt jede Wärmestromdichte."],
        ["mixed", "The same pattern counts capsules: types across, towers down. In B11, one reference locks the column, the other the row.",
         "Dasselbe Muster zählt Kapseln: oben Typen, links Türme. In B11 sperrt ein Bezug die Spalte, der andere die Zeile."],
        ["fill2d", "Fill right and down in one move: each cell takes its own tower and its own type, and counts those capsules on the tower.",
         "Nach rechts und unten in einem Zug ausgefüllt: Jede Zelle holt sich ihren Turm und ihren Typ und zählt diese Kapseln am Turm."],
        ["sheet", "A sheet reference names the sheet first, then an exclamation mark. A name with a space needs single quotes.",
         "Ein Blattbezug nennt zuerst das Blatt, dann ein Ausrufezeichen. Ein Name mit Leerzeichen braucht Apostrophe."],
        ["stuck", "Fill that down and only the cell part moves. The sheet name is text and stays, so the reference slides down the type A sheet.",
         "Nach unten ausgefüllt wandert nur der Zellteil. Der Blattname bleibt Text: Der Bezug rutscht auf dem Blatt Typ A nach unten."],
        ["pertype", "So edit each copy: type B reads its own sheet, types C and D theirs, and every row shows its own net floor area again.",
         "Darum jede Kopie anpassen: Typ B liest sein Blatt, Typ C und D ihre, und jede Zeile zeigt wieder ihre Nettofläche."],
        ["threed", "A 3D reference adds the same cell on all sheets between two tabs: here the heating load B13 of one capsule of each type.",
         "Ein 3D-Bezug addiert dieselbe Zelle aller Blätter zwischen zwei Blattregistern: hier die Heizlast B13 je einer Kapsel jedes Typs."],
        ["span", "The tabs between the two names define the span. Move a sheet out of it, and its value drops out of the sum.",
         "Die Register zwischen den beiden Namen legen den Bereich fest. Wird ein Blatt herausgeschoben, fällt sein Wert aus der Summe."],
        ["deleted", "The second failure: delete the sheet of type C. Direct references show an error, but the 3D sum quietly shrinks.",
         "Der zweite Fehler: Das Blatt von Typ C löschen. Direkte Bezüge zeigen einen Fehler, die 3D-Summe schrumpft dagegen still."],
        ["guard", "The dollar sign guards copies only. Delete the input row, and even an absolute reference breaks.",
         "Das Dollarzeichen schützt nur beim Kopieren. Wird die Eingabezeile gelöscht, bricht auch ein absoluter Bezug."],
        ["names", "A defined name makes an absolute reference readable: theta_i and theta_e stand for the climate cells.",
         "Ein definierter Name macht einen absoluten Bezug lesbar: theta_i und theta_e stehen für die Klimazellen."],
        ["recap", "Recap: relative moves, absolute stays, mixed locks one direction, sheet and 3D references cross tabs, a name makes it readable.",
         "Fazit: relativ wandert, absolut bleibt, gemischt sperrt eine Richtung, Blatt- und 3D-Bezüge queren Register, Namen machen es lesbar."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native Excel visual interface."""
        self.present("references")

    def _card(self, width, height, point):
        """🃏 A dark rounded card behind the right-hand explanations."""
        return _m.RoundedRectangle(corner_radius=0.12, width=width, height=height, fill_color=BACKGROUND, fill_opacity=0.92, stroke_color=MUTED, stroke_width=1.2).move_to([point[0], point[1], 0])

    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """💬 A text label placed by an edge."""
        return _put(self.text(string, (0, 0, 0), size=size, color=color, width=width), point, edge)

    def _edit(self, *events, book=None):
        """🧮 A new workbook state from authored edit events."""
        return _xl.apply(book if book is not None else _S["book"], list(events))

    def _cell(self, book, a1, sheet=SHEET, values=None):
        """🔢 The shown text of one engine cell in the active language."""
        values = values if values is not None else _xl.evaluate(book)
        return _xl.shown(book, values, sheet, a1, self.language())

    def _window(self, book, sheet, cols, rows, selected, **kwargs):
        """🪟 The Excel window at its fixed stage position."""
        return self.xl_window(book, sheet, at=WIN_AT, width=WIN_W, height=WIN_H, cols=cols, rows=rows, selected=selected, **kwargs)

    def _faces(self):
        """💊 The capsule seen from above and from below with the live state: width, highlighted surface, loss tint, hatch."""
        state, width, row = self.st, self.w.get_value(), int(round(self.row.get_value()))
        highlight = {}
        if state["mode"] == "row":
            for index, (name, face) in enumerate(SURFACES):
                if index + 3 <= row:
                    highlight[face] = GLASS if name == "window" and index + 3 == row else DATA
        elif state["mode"] == "loss":
            values = state["loss"]
            good = [abs(v) for v in values if isinstance(v, float)]
            peak = max(good) if good else 1.0
            for index, (_, face) in enumerate(SURFACES):
                if index + 3 > row:
                    break
                value = values[index]
                highlight[face] = SERV if not isinstance(value, float) else _draft.mix(ARCH, SERV, min(abs(value) / peak, 1.0))
        spec = {"key": "A", "width": width}
        views = []
        lift = -0.3 if state["card"] else 1.0
        for centre, view in (((2.75, -0.8 + lift, 0), "axon"), ((5.25, -0.8 + lift, 0), (235, -24))):
            views.append(self.capsule_axon(spec, at=centre, scale=0.3 if state["card"] else 0.44, view=view, highlight=highlight, hatch=state["hatch"], dims=state["dims"] if view == "axon" else ()))
        views.append(self._say(self.tr("from above", "von oben"), (2.75, -2.35), size=SMALL, color=INK))
        views.append(self._say(self.tr("from below", "von unten"), (5.25, -2.35), size=SMALL, color=INK))
        return _m.VGroup(*views)

    def _faces_key(self):
        """🔑 Redraw key of the capsule pair."""
        state = self.st
        return (round(self.w.get_value(), 3), int(round(self.row.get_value())), state["mode"], state["hatch"], state["dims"], state["card"], tuple(str(v) for v in state["loss"] or ()))

    def _bind_reveal(self, window, cells, book, values, tracker):
        """📈 Make filled cells appear one by one while a tracker advances over their rows."""
        for a1 in cells:
            row = int(a1[1:])
            window.bind(a1, lambda a1=a1, row=row: self._cell(book, a1, values=values) if tracker.get_value() + 0.01 >= row else "")

    def _cursor(self, window, tracker, column, color=GOOD):
        """▭ A frame that follows a tracker down one column of a window."""
        def build():
            row = int(round(tracker.get_value()))
            left, top, right, bottom = window.rect(f"{column}{row}")
            return _m.VGroup(_m.Rectangle(width=right - left, height=top - bottom, stroke_color=color, stroke_width=3.2, fill_opacity=0).move_to([(left + right) / 2, (top + bottom) / 2, 0]))
        return self.live(build, lambda: int(round(tracker.get_value())))

    def _formula_card(self, formulas, point, *, caption=None, width=5.4, colors=None):
        """🧾 A card with one or more coloured formulas below an optional caption."""
        height = 0.55 + 0.52 * len(formulas) + (0.4 if caption else 0)
        card = self._card(width, height, point)
        group = _m.VGroup(card)
        y = point[1] + height / 2 - 0.3
        if caption:
            group.add(self._say(caption, (point[0] - width / 2 + 0.25, y), size=SMALL, color=MUTED, edge="l"))
            y -= 0.42
        for formula in formulas:
            tag, text, *note = formula if isinstance(formula, tuple) else ("", formula)
            row = self.xl_formula_text(text, at=(point[0] - width / 2 + 0.25 + (max(0.85, 0.3 + 0.14 * len(tag)) if tag else 0), y - 0.1, 0), size=LABEL, colors_by_ref=colors, ink=INK)
            if tag:
                group.add(self._say(tag, (point[0] - width / 2 + 0.25, y), size=SMALL, color=DATA, edge="l"))
            group.add(row)
            if note:
                group.add(self._say(note[0], (point[0] + width / 2 - 0.25, y), size=SMALL, color=MUTED, edge="r"))
            y -= 0.52
        return group

    def _tower(self, lit=None, tint=None):
        """🗼 The Nakagin tower on the right of the stage."""
        return self.tower_axon(at=(RIGHT_X + 0.1, 0.0, 0), scale=0.1, lit=lit, tint=tint)

    def _type_slots(self, tower, key):
        """🧱 Capsule keys of one type, optionally in one tower."""
        return {(s.building, s.tower, s.storey, s.facade, s.item) for s in _nk.slots() if s.type == key and (tower is None or s.tower == tower)}

    def present_references(self):
        """🎞️ The whole animation: pointer, relative, total, absolute and its mistake, mixed, sheet, 3D, deleted, names, recap."""
        lang = self.language()
        base = _xl.workbook()
        relabel = [{"type": "set-cell", "sheet": "Program", "cell": "C3", "value": self.tr("Net area", "Nettofläche")}]
        if lang == "de":
            relabel += [{"type": "set-cell", "sheet": SHEET, "cell": a1, "value": text} for a1, text in (("C5", "Wand links"), ("C6", "Wand rechts"), ("C9", "Rückwand"))]
        base = _xl.apply(base, relabel)
        _S["book"] = base
        vals0 = _xl.evaluate(base)
        self.st = {"mode": "none", "hatch": (), "dims": (), "loss": None, "card": False}
        self.w = _m.ValueTracker(vals0[(SHEET, "B2")])
        self.row = _m.ValueTracker(3)
        width0 = self.w.get_value()
        setup = [
            {"type": "set-cell", "sheet": SHEET, "cell": "C10", "value": self.tr("Design ΔT", "ΔT Auslegung")},
            {"type": "set-cell", "sheet": SHEET, "cell": "D10", "formula": "=Climate!B2-Climate!B3", "format": KELVIN},
            {"type": "set-cell", "sheet": SHEET, "cell": "G2", "value": self.tr("Loss", "Verlust")},
        ]
        fill_g = [{"type": "copy", "sheet": SHEET, "source": "G3", "target": "G4:G9"}]
        _S["b_abs"] = self._edit(*setup, {"type": "set-cell", "sheet": SHEET, "cell": "G3", "formula": "=F3*$D$10", "format": LOSS_FORMAT}, *fill_g)
        _S["b_slide"] = self._edit(*setup, {"type": "set-cell", "sheet": SHEET, "cell": "G3", "formula": "=F3*D10", "format": LOSS_FORMAT}, *fill_g)
        _S["b_name"] = self._edit(*setup, {"type": "set-cell", "sheet": SHEET, "cell": "G3", "formula": "=F3*(theta_i-theta_e)", "format": LOSS_FORMAT}, *fill_g)
        b_total = self._edit({"type": "set-cell", "sheet": SHEET, "cell": "F10", "formula": "=SUM(F3:F9)", "format": WK})
        v_abs, v_slide, v_total = (_xl.evaluate(book) for book in (_S["b_abs"], _S["b_slide"], b_total))
        losses = lambda values: tuple(values[(SHEET, f"G{r}")] for r in range(3, 10))
        cache = {}

        def at_width(width):
            key = round(width, 3)
            if key not in cache:
                cache[key] = _xl.evaluate(self._edit({"type": "set-cell", "sheet": SHEET, "cell": "B2", "value": key}))
            return cache[key]

        faces = self.live(lambda: self._faces(), lambda: self._faces_key())

        win = self._window(base, SHEET, "A:D", (1, 10), "B2")
        win.bind("B2", lambda: _xl.display(self.w.get_value(), '0.00 "m"', lang))
        dependants = ("D3", "D4", "D7", "D9")
        for a1 in dependants:
            win.bind(a1, lambda a1=a1: _xl.display(at_width(self.w.get_value())[(SHEET, a1)], '0.0 "m²"', lang))
        dep_frames = [self.xl_ref_frame(win, a1, color=DATA, dashed=True) for a1 in dependants]
        tower = self._tower(lit=self._type_slots(None, "A"))
        self.step("hook", _m.FadeIn(win), win.select("D3"), _m.FadeIn(tower), *(_m.FadeIn(f) for f in dep_frames), self.w.animate.set_value(width0 + 0.5), seconds=3.0)
        self.w.set_value(width0)

        self.st.update(hatch=("top", "bottom", "front", "back"), dims=("width", "length"))
        win.edit("=B2*B3")
        self.step("pointer", _m.FadeOut(tower), *(_m.FadeOut(f) for f in dep_frames), _m.FadeIn(faces), self.w.animate.set_value(width0 + 0.5), seconds=3.0)
        self.w.set_value(width0)
        win.edit(None)

        self.st.update(dims=(), hatch=())
        cleared = self._edit(*({"type": "clear-cell", "sheet": SHEET, "cell": f"F{r}"} for r in range(4, 10)))
        win2 = self._window(cleared, SHEET, "C:G", (2, 12), "F3")
        self.row.set_value(3)
        self.st["mode"], self.st["card"] = "row", False
        handle = self.xl_fill_handle(win2, "F3", to="F8")
        self.step("relative", _m.FadeOut(win), _m.FadeIn(win2), _m.FadeIn(handle), seconds=1.5)
        win2.edit("=D3*E3")
        win2.select("F3")
        shift_card = self._formula_card([("F3", "=D3*E3"), ("F4", "=D4*E4", "↓ Δ " + self.tr("row", "Zeile") + " +1")], (RIGHT_X, 1.45), caption=self.tr("Copied one row down", "Eine Zeile nach unten kopiert"))
        self.st["card"] = True
        self.step("shift", _m.FadeIn(shift_card), seconds=1.8)

        win2.edit(None)
        self.st["card"] = False
        self._bind_reveal(win2, [f"F{r}" for r in range(4, 10)], base, vals0, self.row)
        cursor = self._cursor(win2, self.row, "F")
        self.row.set_value(3)
        self.st.update(mode="loss", loss=tuple(vals0[(SHEET, f"F{r}")] for r in range(3, 10)))
        self.step("lights", _m.FadeOut(shift_card), _m.FadeOut(handle), _m.FadeIn(cursor), self.row.animate.set_value(8).set_rate_func(_m.linear), seconds=4.5)
        self.row.set_value(8)

        win2.select("F9")
        win2.edit("=D9*E9*attached_factor")
        back_card = self._formula_card([("F8", "=D8*E8"), ("F9", "=D9*E9*attached_factor")], (RIGHT_X, 1.45), caption=self.tr("Back wall to the unheated core, b = ", "Rückwand zum unbeheizten Kern, b = ") + self._cell(base, "B6", "Climate", vals0))
        self.st["card"] = True
        self.row.set_value(9)
        self.step("back", _m.FadeIn(back_card), seconds=2.5)
        win2.edit(None)

        win_t = self._window(b_total, SHEET, "C:G", (2, 12), "F10")
        win_t.edit("=SUM(F3:F9)")
        total_card = self._formula_card([("F10", "=SUM(F3:F9)", self._cell(b_total, "F10", values=v_total)), ("B11", "=SUM(F3:F9)", self._cell(b_total, "B11", values=v_total))], (RIGHT_X, 1.45),
                                        caption=self.tr("Column total = transmission H_T", "Spaltensumme = Transmission H_T"))
        self.st["card"] = True
        self.step("total", _m.FadeOut(win2), _m.FadeOut(cursor), _m.FadeOut(back_card), _m.FadeIn(win_t), _m.FadeIn(total_card), seconds=2.5)
        win_t.edit(None)

        self.st.update(mode="none", loss=None, card=False)
        win3 = self._window(self._edit(*setup), SHEET, "C:G", (2, 12), "D10")
        frame = self.xl_ref_frame(win3, "D10", color=DATA)
        self.step("fixed", _m.FadeOut(win_t), _m.FadeOut(total_card), _m.FadeIn(win3), _m.FadeIn(frame), seconds=1.5)

        self.st["card"] = True
        forms, form = [], "D10"
        for _ in range(4):
            forms.append(form)
            form = _xl.cycle_reference(form)
        names = (self.tr("relative", "relativ"), self.tr("absolute", "absolut"), self.tr("row locked", "Zeile gesperrt"), self.tr("column locked", "Spalte gesperrt"))
        card = self._formula_card([(self.tr(("1st", "2nd", "3rd", "4th")[i], f"{i + 1}.") + " F4", "=F3*" + forms[i], names[i]) for i in range(4)], (RIGHT_X, 1.35),
                                  caption=self.tr("F4 cycles four forms (Mac: Fn+F4)", "F4 wechselt durch vier Formen (Mac: Fn+F4)"))
        self.step("f4", _m.FadeIn(card), seconds=2.5)

        self.st["card"] = False
        win4 = self._window(_S["b_abs"], SHEET, "C:G", (2, 12), "G3")
        self.row.set_value(3)
        self.st.update(mode="loss", loss=losses(v_abs))
        self._bind_reveal(win4, [f"G{r}" for r in range(3, 10)], _S["b_abs"], v_abs, self.row)
        frame4 = self.xl_ref_frame(win4, "D10", color=DATA)
        cursor4 = self._cursor(win4, self.row, "G")
        win4.edit("=F3*$D$10")
        self.step("absolute", _m.FadeOut(card), _m.FadeOut(win3), _m.FadeOut(frame), _m.FadeIn(win4), _m.FadeIn(frame4), _m.FadeIn(cursor4),
                  self.row.animate.set_value(9).set_rate_func(_m.linear), seconds=5.0)
        self.row.set_value(9)
        win4.edit(None)

        self.st["card"] = True
        slide = self._window(_S["b_slide"], SHEET, "C:G", (2, 12), "G4")
        slide.edit("=F4*D11")
        self.st["loss"] = losses(v_slide)

        def note(a1):
            wrong, right = self._cell(_S["b_slide"], a1, values=v_slide), self._cell(_S["b_abs"], a1, values=v_abs)
            return "= " + right if wrong == right else wrong + " ≠ " + right

        slide_card = self._formula_card([("G3", "=F3*D10", note("G3")), ("G4", "=F4*D11", note("G4")), ("G5", "=F5*D12", note("G5"))], (RIGHT_X, 1.35),
                                        caption=self.tr("The input slides down with the row", "Die Eingabe rutscht mit der Zeile nach unten"))
        slide_frames = [self.xl_ref_frame(slide, a1, color=SERV, dashed=True) for a1 in ("D10", "D11", "D12")]
        self.step("slide", _m.FadeOut(win4), _m.FadeOut(frame4), _m.FadeOut(cursor4), _m.FadeIn(slide), _m.FadeIn(slide_card), *(_m.FadeIn(f) for f in slide_frames), seconds=3.0)
        slide.edit(None)

        fixed = self._window(_S["b_abs"], SHEET, "C:G", (2, 12), "G4")
        fixed.edit("=F4*$D$10")
        self.st["loss"] = losses(v_abs)
        fix_card = self._formula_card([("G4", "=F4*D11", self.tr("before", "vorher")), ("G4", "=F4*$D$10", self.tr("after", "nachher"))], (RIGHT_X, 1.35), caption=self.tr("Lock the input", "Die Eingabe sperren"))
        self.step("repair", _m.FadeOut(slide), _m.FadeOut(slide_card), *(_m.FadeOut(f) for f in slide_frames), _m.FadeIn(fixed), _m.FadeIn(fix_card), seconds=3.0)
        fixed.edit(None)

        self._stage_two_way(fixed, fix_card, faces)
        self._stage_sheets()
        self._stage_failures()
        self._stage_names_and_recap()

    def _tower_cell(self, book, a1):
        """🗼 Capsules counted by one cell of the Program tower table: lit keys for its tower and type."""
        row, col = int(a1[1:]), a1[0]
        tower_key = self._cell(book, f"A{row}", "Program")
        type_key = self._cell(book, f"{col}10", "Program").split()[-1]
        index = next(t.index for t in _nk.towers() if t.key == tower_key)
        return self._type_slots(index, type_key)

    def _stage_two_way(self, fixed, fix_card, faces):
        """✖️ Mixed references: first U-value by temperature difference, then the tower-by-type count table."""
        base = _S["book"]
        vals = _xl.evaluate(base)
        kelvin = vals[("Climate", "B2")] - vals[("Climate", "B3")]
        rows = ((19, self.tr("Roof", "Dach"), "B11"), (20, self.tr("Wall", "Wand"), "B13"), (21, self.tr("Window", "Fenster"), "B14"))
        columns = (("C", kelvin / 2), ("D", kelvin), ("E", kelvin * 1.5))
        events = [{"type": "set-cell", "sheet": "Program", "cell": "A18", "value": "q, W/m²"}, {"type": "set-cell", "sheet": "Program", "cell": "B18", "value": "U, W/m²K"}]
        events += [{"type": "set-cell", "sheet": "Program", "cell": f"{col}18", "value": value, "format": '0 "K"'} for col, value in columns]
        for row, label, source in rows:
            events += [{"type": "set-cell", "sheet": "Program", "cell": f"A{row}", "value": label},
                       {"type": "set-cell", "sheet": "Program", "cell": f"B{row}", "value": vals[("Energy", source)], "format": "0.00"}]
        events += [{"type": "set-cell", "sheet": "Program", "cell": "C19", "formula": "=$B19*C$18", "format": "0.0"}, {"type": "copy", "sheet": "Program", "source": "C19", "target": "C19:E21"}]
        grid = self._edit(*events)
        vg = _xl.evaluate(grid)
        order = [f"{c}{r}" for r in (19, 20, 21) for c in "CDE"]
        blank = self._edit(*({"type": "clear-cell", "sheet": "Program", "cell": a1} for a1 in order), book=grid)
        flux = self._window(blank, "Program", "A:E", (18, 21), "C19")
        self.pos2 = _m.ValueTracker(-1)
        for index, a1 in enumerate(order):
            flux.bind(a1, lambda a1=a1, index=index: self._cell(grid, a1, "Program", vg) if self.pos2.get_value() + 0.01 >= index else "")
        flux.edit("=$B19*C$18")
        chart = self.xl_bar_chart([(label, vg[("Program", f"D{row}")]) for row, label, _ in rows], at=(RIGHT_X, 0.0, 0), width=5.2, height=4.4, unit="W/m²",
                                  title=self.tr("Heat flux at ", "Wärmestromdichte bei ") + self.number(kelvin, 0) + " K", decimals=1)
        self.step("mixed2", _m.FadeOut(fixed), _m.FadeOut(fix_card), _m.FadeOut(faces), _m.FadeIn(flux), _m.FadeIn(chart), self.pos2.animate.set_value(8).set_rate_func(_m.linear), seconds=6.0)
        flux.edit(None)

        cells = [f"{c}{r}" for r in (11, 12) for c in "BCDE"]
        cleared = self._edit(*({"type": "clear-cell", "sheet": "Program", "cell": a1} for a1 in cells))
        win5 = self._window(cleared, "Program", "A:E", (10, 13), "B11")
        self.pos = _m.ValueTracker(-1)
        for col in "BCDE":
            win5.bind(f"{col}13", lambda col=col: self._cell(base, f"{col}13", "Program", vals) if self.pos.get_value() + 0.01 >= 7 else "")
        for index, a1 in enumerate(cells):
            win5.bind(a1, lambda a1=a1, index=index: self._cell(base, a1, "Program", vals) if self.pos.get_value() + 0.01 >= index else "")
        self.st.update(mode="none", loss=None)
        win5.edit("=COUNTIFS($I$2:$I$141,$A11,$M$2:$M$141,B$10)")
        lit_cache = {}

        def tower():
            index = int(round(self.pos.get_value()))
            keys = lit_cache.setdefault(index, self._tower_cell(base, cells[max(index, 0)]) if index >= 0 else None)
            return self._tower(lit=keys)

        def caption():
            index = int(round(self.pos.get_value()))
            if index < 0:
                return _m.VGroup()
            a1 = cells[index]
            text = self.tr("Tower ", "Turm ") + self._cell(base, f"A{a1[1:]}", "Program", vals) + " · " + self._cell(base, f"{a1[0]}10", "Program", vals) + ": " + self._cell(base, a1, "Program", vals)
            return _m.VGroup(self._say(text, (RIGHT_X, 2.35), size=LABEL, color=DATA))

        label = self.live(caption, lambda: int(round(self.pos.get_value())))
        tower_live = _m.VGroup(self.live(tower, lambda: int(round(self.pos.get_value()))), label)
        self.step("mixed", _m.FadeOut(flux), _m.FadeOut(chart), _m.FadeIn(win5), _m.FadeIn(tower_live), seconds=2.0)
        win5.edit(None)
        self.pos.set_value(0)
        self.step("fill2d", self.pos.animate.set_value(7).set_rate_func(_m.linear), seconds=7.0)
        self.pos.set_value(7)
        self.win, self.tower_live = win5, tower_live

    def _stage_sheets(self):
        """📄 Sheet references, the sliding sheet reference and its repair, the 3D span and moving a sheet out of it."""
        base = _S["book"]
        vals = _xl.evaluate(base)
        win5, tower_live = self.win, self.tower_live
        win6 = self._window(base, "Program", "A:C", (3, 8), "C4")
        win6.edit("='Type A'!B9")
        thumb = self.xl_window(base, "Type A", at=(RIGHT_X, 0.9, 0), width=5.6, height=3.3, cols="A:B", rows=(8, 12), selected="B9")
        thumb_frame = self.xl_ref_frame(thumb, "B9", color=_visuals._workbook.REFERENCE_COLORS[0])
        plan = self.capsule_axon({"key": "A"}, at=(RIGHT_X, -1.55, 0), scale=0.3, view="plan", hatch=("top",))
        plan_label = self._say(self.tr("net floor area", "Nettogrundfläche") + " " + self._cell(base, "B9", values=vals), (RIGHT_X, -2.4), size=SMALL, color=MUTED)
        self.step("sheet", _m.FadeOut(win5), _m.FadeOut(tower_live), _m.FadeIn(win6), _m.FadeIn(thumb), _m.FadeIn(thumb_frame), _m.FadeIn(plan), _m.FadeIn(plan_label), seconds=2.0)
        win6.edit(None)

        stuck = self._edit({"type": "copy", "sheet": "Program", "source": "C4", "target": "C5:C7"})
        win7 = self._window(stuck, "Program", "A:C", (3, 7), "C5")
        win7.edit("='Type A'!B10")
        wrong = [self.xl_ref_frame(win7, a1, color=SERV, dashed=True) for a1 in ("C5", "C6", "C7")]
        slid = [self.xl_ref_frame(thumb, a1, color=SERV, dashed=True) for a1 in ("B10", "B11", "B12")]
        expected = self._say(self.tr("expected in C5:C7: ", "erwartet in C5:C7: ") + self._cell(base, "C5", "Program", vals), (RIGHT_X, -2.4), size=SMALL, color=SERV)
        self.step("stuck", _m.FadeOut(win6), _m.FadeOut(plan), _m.FadeOut(plan_label), _m.FadeIn(win7), *(_m.FadeIn(f) for f in wrong + slid), _m.FadeIn(expected), seconds=3.0)
        win7.edit(None)

        win8 = self._window(base, "Program", "A:C", (3, 8), "C5")
        win8.edit("='Type B'!B9")
        fix_rows = [(f"C{4 + i}", f"='Type {k}'!B9", self._cell(base, f"C{4 + i}", "Program", vals)) for i, k in enumerate("ABCD")]
        pertype_card = self._formula_card(fix_rows, (RIGHT_X, 0.7), caption=self.tr("Each type, its own sheet name", "Jeder Typ, sein eigener Blattname"), width=5.4)
        self.step("pertype", _m.FadeOut(win7), _m.FadeOut(thumb), _m.FadeOut(thumb_frame), *(_m.FadeOut(f) for f in wrong + slid), _m.FadeOut(expected), _m.FadeIn(win8), _m.FadeIn(pertype_card), seconds=3.0)
        win8.edit(None)

        energy = self._window(base, "Energy", "A:B", (11, 17), "B16")
        energy.edit("=SUM('Type A:Type D'!B13)")
        keys = ("A", "B", "C", "D")
        chips = self._chips(base, vals, keys, ())
        bracket = self._bracket(energy, "Type A", "Type D")
        self.step("threed", _m.FadeOut(win8), _m.FadeOut(pertype_card), _m.FadeIn(energy), _m.FadeIn(chips), _m.FadeIn(bracket), seconds=3.0)
        energy.edit(None)
        moved = self._edit({"type": "move-sheet", "sheet": "Type B", "index": 4})
        vm = _xl.evaluate(moved)
        energy2 = self._window(moved, "Energy", "A:B", (11, 17), "B16")
        chips2 = self._chips(moved, vm, keys, ("B",))
        bracket2 = self._bracket(energy2, "Type A", "Type D")
        self.step("span", _m.FadeOut(energy), _m.FadeOut(chips), _m.FadeOut(bracket), _m.FadeIn(energy2), _m.FadeIn(chips2), _m.FadeIn(bracket2), seconds=2.5)
        self.win, self.chips = _m.VGroup(energy2, bracket2), chips2

    def _bracket(self, window, first, last):
        """⎴ A bracket over the sheet tabs from the first to the last named sheet: the span of a 3D reference."""
        a, b = window.tabs[first], window.tabs[last]
        left, right, y = a.get_left()[0], b.get_right()[0], a.get_top()[1] + 0.04
        group = _m.VGroup(_m.Line([left, y, 0], [right, y, 0], stroke_color=DATA, stroke_width=4),
                          _m.Line([left, y, 0], [left, y - 0.1, 0], stroke_color=DATA, stroke_width=4), _m.Line([right, y, 0], [right, y - 0.1, 0], stroke_color=DATA, stroke_width=4))
        group.add(self._say(self.tr("3D span", "3D-Bereich"), ((left + right) / 2, y + 0.18), size=SMALL, color=DATA))
        return group

    def _chips(self, book, values, keys, outside):
        """💊 The cell B13 of every type sheet as a small capsule plus value, and the sum."""
        group = _m.VGroup()
        for index, key in enumerate(keys):
            x = RIGHT_X - 1.95 + index * 1.3
            capsule = self.capsule_axon({"key": key}, at=(x, 0.7, 0), scale=0.2, view="axon", base=_nd.type_color(key))
            tab = _xl.tab_name(book, f"Type {key}", self.language())
            group.add(capsule, self._say(tab, (x, -0.35), size=SMALL, color=MUTED if key in outside else INK),
                      self._say(_xl.shown(book, values, f"Type {key}", "B13", self.language()), (x, -0.75), size=SMALL, color=MUTED if key in outside else DATA))
            if key in outside:
                for part in capsule.get_family():
                    part.set_opacity(0.25) if hasattr(part, "set_opacity") else None
        group.add(self._say(_xl.shown(book, values, "Energy", "B16", self.language()), (RIGHT_X, -1.5), size=LABEL, color=GOOD),
                  self._say("Σ B13", (RIGHT_X, -1.1), size=SMALL, color=MUTED))
        return group

    def _stage_failures(self):
        """💥 The deleted sheet, and the deleted input row that breaks even an absolute reference."""
        lang = self.language()
        energy2, chips2 = self.win, self.chips
        base = _S["book"]
        gone = self._edit({"type": "delete-sheet", "sheet": "Type C"})
        gone = self._edit({"type": "set-cell", "sheet": "Energy", "cell": "A16", "value": self.tr("Heating load", "Heizlast")}, book=gone)
        vg, vb = _xl.evaluate(gone), _xl.evaluate(base)
        broken = self.xl_window(gone, "Program", at=(-2.45, 0, 0), width=8.8, height=WIN_H, cols="A:D", rows=(3, 8), selected="D6")
        error = self.xl_error(broken, "D6", "#REF!")
        error.tip.set_opacity(0)
        part = vb[("Type C", "B13")]
        before, after = (self.number(v, 1) for v in (vb[("Energy", "B16")], vg[("Energy", "B16")]))
        assert abs(round(vb[("Energy", "B16")], 1) - round(part, 1) - round(vg[("Energy", "B16")], 1)) < 1e-6
        small = self.xl_window(gone, "Energy", at=(4.4, 0.95, 0), width=4.8, height=3.2, cols="A:B", rows=(15, 17), selected="B15")
        small_frame = self.xl_ref_frame(small, "B16", color=GOOD)
        sum_card = self._formula_card([("", "=SUM('Type A:Type D'!B13)")], (4.4, -1.45), caption=before + " − " + self.number(part, 1) + " = " + after + " W", width=4.8)
        self.step("deleted", _m.FadeOut(energy2), _m.FadeOut(chips2), _m.FadeIn(broken), _m.FadeIn(error), _m.FadeIn(small), _m.FadeIn(small_frame), _m.FadeIn(sum_card), seconds=3.0)

        row_gone = self._edit({"type": "delete-row", "sheet": SHEET, "index": 10}, book=_S["b_abs"])
        win9 = self._window(row_gone, SHEET, "C:G", (2, 11), "G3")
        guard_card = self._formula_card([(self.tr("before", "vorher"), "=F3*$D$10"), (self.tr("after", "nachher"), "=F3*#REF!")], (RIGHT_X, 1.2), caption=self.tr("Row 10 deleted", "Zeile 10 gelöscht"), width=5.4)
        self.step("guard", _m.FadeOut(broken), _m.FadeOut(error), _m.FadeOut(small), _m.FadeOut(small_frame), _m.FadeOut(sum_card), _m.FadeIn(win9), _m.FadeIn(guard_card), seconds=2.5)
        self.keep = _m.VGroup(win9, guard_card)

    def _stage_names_and_recap(self):
        """🏷️ One short clause for defined names, then the legend of reference kinds."""
        lang = self.language()
        base0 = _S["book"]
        vv = _xl.evaluate(base0)
        named = self._window(_S["b_name"], SHEET, "C:G", (2, 12), "G3")
        named.edit("=F3*(theta_i-theta_e)")
        manager = _m.VGroup(self._card(5.4, 2.0, (RIGHT_X, -0.35)), self._say(self.tr("Name Manager", "Namens-Manager"), (RIGHT_X - 2.5, 0.5), size=SMALL, color=DATA, edge="l"))
        for x, head in ((-2.5, self.tr("Name", "Name")), (-0.6, self.tr("Refers to", "Bezieht sich auf")), (2.5, self.tr("Value", "Wert"))):
            manager.add(self._say(head, (RIGHT_X + x, 0.15), size=SMALL, color=MUTED, edge="l" if x < 2 else "r"))
        for index, name in enumerate(("theta_i", "theta_e")):
            ref = _xl.parse_ref(base0.names[name])
            y = -0.3 - index * 0.5
            manager.add(self._say(name, (RIGHT_X - 2.5, y), size=SMALL, edge="l"),
                        self._say(_xl.localize("=" + base0.names[name], lang, base0)[1:], (RIGHT_X - 0.6, y), size=SMALL, edge="l"),
                        self._say(self._cell(base0, _xl.address(ref.col, ref.row), ref.sheet, vv), (RIGHT_X + 2.5, y), size=SMALL, color=GOOD, edge="r"))
        names_card = self._formula_card([("", "=F3*(Climate!$B$2-Climate!$B$3)"), ("", "=F3*(theta_i-theta_e)")], (RIGHT_X, 1.9), caption=self.tr("Same value, readable", "Gleicher Wert, lesbar"), width=5.4)
        names_card.shift(_m.DOWN * 0.0)
        self.step("names", _m.FadeOut(self.keep), _m.FadeIn(named), _m.FadeIn(names_card), _m.FadeIn(manager), seconds=3.0)
        named.edit(None)

        rows = (
            (self.tr("Relative", "Relativ"), "=D3*E3", self.tr("column and row shift", "Spalte und Zeile wandern")),
            (self.tr("Absolute", "Absolut"), "=F3*$D$10", self.tr("nothing shifts", "nichts wandert")),
            (self.tr("Mixed", "Gemischt"), "=$B19*C$18", self.tr("only the open part shifts", "nur der offene Teil wandert")),
            (self.tr("Sheet", "Blatt"), "='Type A'!B9", self.tr("sheet name stays", "Blattname bleibt")),
            (self.tr("3D", "3D"), "=SUM('Type A:Type D'!B13)", self.tr("same cell on each tab between", "dieselbe Zelle auf jedem Blatt dazwischen")),
            (self.tr("Name", "Name"), "=F3*(theta_i-theta_e)", self.tr("acts as absolute", "wirkt absolut")),
        )
        group = _m.VGroup()
        top = 1.75
        group.add(self._say(self.tr("Kind", "Art"), (-6.6, top + 0.62), size=SMALL, color=MUTED, edge="l"), self._say(self.tr("Example", "Beispiel"), (-4.3, top + 0.62), size=SMALL, color=MUTED, edge="l"),
                  self._say(self.tr("On copy", "Beim Kopieren"), (0.9, top + 0.62), size=SMALL, color=MUTED, edge="l"))
        for index, (kind, formula, rule) in enumerate(rows):
            y = top - index * 0.72
            group.add(_m.Line([-6.7, y + 0.36, 0], [6.7, y + 0.36, 0], stroke_color=MUTED, stroke_width=1, stroke_opacity=0.5),
                      self._say(kind, (-6.6, y), color=DATA, edge="l"),
                      self.xl_formula_text(formula, at=(-4.3, y - 0.1, 0), size=SMALL, ink=INK),
                      self._say(rule, (0.9, y), size=SMALL, edge="l"))
        self.step("recap", _m.FadeOut(named), _m.FadeOut(names_card), _m.FadeOut(manager), _m.FadeIn(group), seconds=3.5)
