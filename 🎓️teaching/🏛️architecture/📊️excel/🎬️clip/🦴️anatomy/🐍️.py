"""🦴 The anatomy of the Nakagin workbook: workbook, worksheet, cell, range, formula, function, recalculation and the four classic errors, each drawn on the capsule it computes."""

import importlib
import sys
from functools import lru_cache
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
_book = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.📗️workbook.🐍️")
_xl = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN = _visuals.INK, _visuals.MUTED, _visuals.ARCH, _visuals.STRUCT, _visuals.SERV, _visuals.DATA, _visuals.GOOD, _visuals.WARN
LABEL, SMALL = 19.2, 16.2
SHEET = "Type A"
WIN_AT = np.array([-2.38, -0.05, 0.0])
WIN_W, WIN_H = 8.85, 5.0
CAP_AT = np.array([4.4, 0.35, 0.0])
CAP_SCALE = 0.62
BLUE, RED = _book.REFERENCE_COLORS[0], _book.REFERENCE_COLORS[1]
DIM = {"width": BLUE, "length": RED, "height": STRUCT}
FITTING_TONES = (ARCH, WARN, STRUCT, GOOD)
AREA_FORMAT, VOLUME_FORMAT = '0.0 "m²"', '0.0 "m³"'


@lru_cache(maxsize=256)
def _interior(width: float) -> dict:
    """♻️ Recalculated cells of the Type A sheet for one interior width, cached by value."""
    book = _xl.apply(_xl.workbook(), [{"type": "set-cell", "sheet": SHEET, "cell": "B5", "value": width}])
    values = _xl.evaluate(book)
    return {a1: values[(SHEET, a1)] for a1 in ("B5", "B9", "B10")}


class Beat1_Anatomy(_visuals.ClipScene):
    """🦴 Teach the parts of a workbook on the Type A sheet of the Nakagin workbook next to the capsule it describes, in English and German."""

    TITLE = {"en": "The Anatomy of a Workbook", "de": "Aufbau einer Arbeitsmappe"}
    NARRATION = [
        ["hook", "The Nakagin tower in our teaching model: capsules in four types. How much floor does one give, and what if its width changes?",
         "Der Nakagin-Turm im Lehrmodell: Kapseln in vier Typen. Wie viel Fläche bietet eine, und was passiert, wenn sich ihre Breite ändert?"],
        ["workbook", "A workbook is the whole file: Nakagin.xlsx holds the calculation of the entire building.",
         "Eine Arbeitsmappe ist die ganze Datei: Nakagin.xlsx enthält die Berechnung des ganzen Gebäudes."],
        ["sheets", "Worksheets are the tabs inside it: Program, one sheet per capsule type, Climate and Energy.",
         "Arbeitsblätter sind die Register darin: Programm, je Kapseltyp ein Blatt, Klima und Energie."],
        ["grid", "A worksheet is a grid: columns have letters, rows have numbers. Type A describes one capsule.",
         "Ein Arbeitsblatt ist ein Raster: Spalten haben Buchstaben, Zeilen Nummern. Typ A beschreibt eine Kapsel."],
        ["address", "A cell is named by column and row. B2 is the width of the capsule, shown with its unit.",
         "Eine Zelle heißt nach Spalte und Zeile. B2 ist die Breite der Kapsel, angezeigt mit Einheit."],
        ["range", "A range joins cells: B2:B4 holds width, length and height.",
         "Ein Bereich fasst Zellen zusammen: B2:B4 enthält Breite, Länge und Höhe."],
        ["bars", "The Name Box shows the address, the formula bar shows what the cell contains.",
         "Das Namenfeld zeigt die Adresse, die Bearbeitungsleiste zeigt den Inhalt der Zelle."],
        ["constant", "Typed numbers are constants, and so is text such as Width: Excel stores both as they are.",
         "Eingetippte Zahlen sind Konstanten, ebenso Text wie Breite: Excel speichert beides unverändert."],
        ["display", "The cell shows a rounded number with a unit, but it stores the exact number. The format changes the display, not the value.",
         "Die Zelle zeigt eine gerundete Zahl mit Einheit, speichert aber die genaue Zahl. Das Format ändert die Anzeige, nicht den Wert."],
        ["formula", "A formula starts with an equals sign. The floor area is interior width times interior length: B5 times B6.",
         "Eine Formel beginnt mit einem Gleichheitszeichen. Die Fläche ist Innenbreite mal Innenlänge: B5 mal B6."],
        ["frames", "Excel colours every reference and frames its cell: the formula points at its sources, and the same colours mark width and length on the floor.",
         "Excel färbt jeden Bezug und umrahmt seine Zelle: Die Formel zeigt auf ihre Quellen, dieselben Farben markieren Breite und Länge im Boden."],
        ["hatch", "The result is the floor of the capsule, drawn as a hatched surface.",
         "Das Ergebnis ist der Boden der Kapsel, als schraffierte Fläche gezeichnet."],
        ["precedence", "A typical mistake: without brackets Excel multiplies first and adds after, so one side of the perimeter is missing. Brackets fix it.",
         "Typischer Fehler: Ohne Klammern rechnet Excel Punkt vor Strich, eine Seite des Umfangs fehlt. Mit Klammern stimmt es."],
        ["function", "A function is a named calculation. SUM adds a range: the fittings give their total area.",
         "Eine Funktion ist eine benannte Berechnung. SUMME addiert einen Bereich: Die Einbauten ergeben ihre Gesamtfläche."],
        ["locale", "German Excel writes SUMME, separates arguments with a semicolon and decimals with a comma.",
         "In der deutschen Version heißt es SUMME, Argumente trennt ein Semikolon, Dezimalstellen ein Komma."],
        ["recalc", "Widen the interior up to the outer wall: every dependent cell recalculates and the floor follows.",
         "Weiten wir den Innenraum bis zur Außenwand: Alle abhängigen Zellen rechnen neu, und der Boden folgt."],
        ["trace", "Trace arrows show precedents, the cells a formula uses, and dependents, the cells that use this one.",
         "Spurpfeile zeigen Vorgänger, die Zellen, die eine Formel nutzt, und Nachfolger, die Zellen, die diese nutzen."],
        ["div0", "Typical errors have causes. An empty interior length leaves a floor area of zero, and dividing by it gives #DIV/0!.",
         "Typische Fehler haben Ursachen. Eine leere Innenlänge ergibt die Fläche null, und die Division durch null ergibt #DIV/0!."],
        ["name", "A misspelled function gives #NAME?: SUMM does not exist.",
         "Eine falsch geschriebene Funktion ergibt #NAME?: SUMM gibt es nicht."],
        ["ref", "Delete a source row and the reference has nothing to point to: #REF! in English, #BEZUG! in German Excel.",
         "Wird eine Quellzeile gelöscht, hat der Bezug kein Ziel mehr: #REF! im englischen, #BEZUG! im deutschen Excel."],
        ["value", "Text where a number is expected gives #VALUE! in English and #WERT! in German: here the interior length was typed with its unit.",
         "Text, obwohl eine Zahl erwartet wird, ergibt #WERT! und im Englischen #VALUE!: Hier wurde die Innenlänge mit Einheit getippt."],
        ["repair", "Type the number again and the numbers return: floor area, volume and fittings.",
         "Tippen wir die Zahl neu, kehren die Werte zurück: Fläche, Volumen und Einbauten."],
        ["legend", "Remember the terms: workbook, worksheet, cell, reference, formula, function, Name Box, formula bar. Next video: references.",
         "Merken wir uns: Arbeitsmappe, Arbeitsblatt, Zelle, Bezug, Formel, Funktion, Namenfeld, Bearbeitungsleiste. Nächstes Video: Bezüge."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native Excel visual interface."""
        self.present("anatomy")

    def _put(self, shape, point, edge="c"):
        """📌 Move a shape so its left edge, right edge or centre sits on ``point``."""
        dx = {"l": point[0] - shape.get_left()[0], "r": point[0] - shape.get_right()[0], "c": point[0] - shape.get_center()[0]}[edge]
        return shape.shift(np.array([dx, point[1] - shape.get_center()[1], 0.0]))

    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """🔤 One label aligned on its left edge, right edge or centre."""
        return self._put(self.text(string, (0, 0, 0), size=size, color=color, width=width), point, edge)

    def xl_display(self, value, number_format="General", language=None):
        """🔢 The cell text of a value; the status bar takes the number format of the first selected cell while the range is shown."""
        if self.s.status and number_format == "General" and isinstance(value, float):
            number_format = self.s.status
        return super().xl_display(value, number_format, language)

    def _window(self, book, *, selected="A1", cols="A:C", rows=(1, 10), **kwargs):
        """🪟 The Type A window over a workbook state."""
        return self.xl_window(book, SHEET, at=WIN_AT, width=WIN_W, height=WIN_H, cols=cols, rows=rows, selected=selected, **kwargs)

    def _hop(self, window, cells, seconds):
        """🦘 Move the selection through ``cells`` while the clause runs, so the formula bar follows each stop."""
        seen = {"index": -1}

        def tick(_, alpha):
            index = min(int(alpha * len(cells)), len(cells) - 1)
            if index != seen["index"]:
                seen["index"] = index
                move = window.select(cells[index])
                move.begin()
                move.interpolate(1.0)
                move.finish()

        return _engine.UpdateFromAlphaFunc(_engine.ValueTracker(0.0), tick, run_time=seconds, rate_func=_engine.linear)

    def _state(self, **kwargs):
        """🎛️ Update the drawing state; the live capsule redraws on the next frame."""
        for key, value in kwargs.items():
            setattr(self.s, key, value)

    def _key(self):
        """🔑 Redraw key of the live capsule."""
        s = self.s
        return (round(s.iw.get_value(), 3), s.il, s.dims, s.floor, s.floor_color, s.interior, s.err, s.fit, s.chips, s.perim, s.area, s.delta, s.fixchip, s.show)

    def _live_capsule(self):
        """♻️ The capsule redrawn whenever its state changes."""
        return self.live(lambda: self._drawing() if self.s.show else _engine.VGroup(), self._key)

    def _drawing(self):
        """💊 The capsule at the outer size of B2:B4 with the interior floor, its hatch, the fittings and the perimeter, all from the state dictionary and the engine values."""
        s, lang = self.s, self.language()
        iw = s.iw.get_value()
        il = self._base_il if s.il is None else s.il
        t = {"key": "A", **self._outer}
        cap = self.capsule_axon(t, at=CAP_AT, scale=CAP_SCALE, dims=s.dims)
        for name in s.dims:
            cap.dims[name].set_color(DIM[name])
        if s.floor or s.err:
            cap.body.set_fill(opacity=0.3)
            cap.body.set_stroke(opacity=0.3)
        group = _engine.VGroup(cap)
        w, l = self._outer["width"], self._outer["length"]
        y0, z = (l - il) / 2, 0.12
        ring = [cap.at(x, y, z)[:2] for x, y in ((-iw / 2, y0), (iw / 2, y0), (iw / 2, y0 + il), (-iw / 2, y0 + il))]
        if s.err:
            group.add(_draft.polyline(ring, SERV, 3.0, 0.95, closed=True, dash=0.1))
        elif s.floor:
            group.add(_draft.polygon(ring, s.floor_color, 0.16, s.floor_color, _draft.W_THIN), self.hatch([ring], s.floor_color, 0.09, 45, _draft.W_HAIR, 0.9))
        if s.interior or s.err:
            front = (cap.at(-iw / 2, y0 + il, z), cap.at(iw / 2, y0 + il, z))
            side = (cap.at(iw / 2, y0, z), cap.at(iw / 2, y0 + il, z))
            group.add(self.dimension(front[0], front[1], self.quantity(iw, "m", 2), offset=0.32, color=BLUE, size=SMALL),
                      self.dimension(side[0], side[1], "?" if s.err else self.quantity(il, "m", 2), offset=-0.32, color=SERV if s.err else RED, size=SMALL))
        if s.area and s.floor:
            text = _xl.display(_interior(round(iw, 3))["B9"], AREA_FORMAT, lang)
            group.add(self.badge(text, tuple(cap.at(0, y0 + il / 2, z)[:2]) + (0.0,), color=GOOD, size=SMALL, filled=True))
        for index in range(s.fit):
            fit = self._fittings[index]
            x0 = -iw / 2 + 0.08 if index % 2 == 0 else iw / 2 - 0.08 - fit["width"]
            y1 = y0 + 0.08 + (self._fittings[0]["length"] + 0.1 if index == 2 else 0.0)
            box = [cap.at(x0 + dx, y1 + dy, z + 0.02)[:2] for dx, dy in ((0, 0), (fit["width"], 0), (fit["width"], fit["length"]), (0, fit["length"]))]
            tone = FITTING_TONES[index % 4]
            group.add(_draft.polygon(box, tone, 0.2, tone, _draft.W_EDGE), self.hatch([box], tone, 0.07, 135, _draft.W_HAIR, 0.9))
        if s.chips:
            for i in range(len(self._fittings)):
                group.add(self._say(f"{self._fittings[i]['name']} {self._fittings[i]['area']}", (2.5, -1.55 - 0.36 * i), size=SMALL, color=FITTING_TONES[i % 4], edge="l"))
            group.add(self.badge("Σ " + self._fitting_total, (6.0, -1.9, 0.0), color=GOOD, size=SMALL, filled=True))
        if s.perim:
            colour = SERV if s.perim == "wrong" else GOOD
            corners = [cap.at(*p, 0.0)[:2] for p in ((-w / 2, 0), (w / 2, 0), (w / 2, l), (-w / 2, l))]
            for i, tone in enumerate((BLUE, RED, BLUE, RED)):
                edge = [corners[i], corners[(i + 1) % 4]]
                group.add(_draft.polyline(edge, MUTED, 3.0, 0.9, dash=0.1) if i == 3 and s.perim == "wrong" else _draft.polyline(edge, tone, 5.5, 0.95))
            note = self.tr("without brackets: one side missing", "ohne Klammern: eine Seite fehlt") if s.perim == "wrong" else self.tr("with brackets: all four sides", "mit Klammern: alle vier Seiten")
            group.add(self._say(note, (CAP_AT[0], -1.78), size=SMALL, color=colour))
            group.add(self.mono(f"=2*B2+B3 = {self._perims[0]}", (CAP_AT[0], -2.06, 0), size=SMALL, color=SERV if s.perim == "wrong" else MUTED))
            if s.perim == "right":
                group.add(self.mono(f"=2*(B2+B3) = {self._perims[1]}", (CAP_AT[0], -2.34, 0), size=SMALL, color=GOOD))
        if s.delta:
            for i, line in enumerate(self._delta):
                group.add(self._say(line, (2.45, -1.42 - 0.3 * i), size=SMALL, color=GOOD if i < 3 else DATA, edge="l", width=4.4))
        if s.fixchip:
            group.add(self.badge(s.fixchip, (CAP_AT[0], -1.62, 0.0), color=GOOD, size=SMALL, filled=False))
        return group

    def _tower(self, lit=None, at=(0.0, 0.0, 0.0), scale=0.115):
        """🗼 The tower with the capsule types in their colours."""
        return self.tower_axon(at=at, scale=scale, lit=lit)

    def _chip(self, text, point, color=DATA):
        """🔖 An outlined chip."""
        return self.badge(text, point, color=color, size=SMALL, filled=False, anchor="c")

    def _term(self, en, de):
        """📖 The term in the language of the narration first, the other language after the slash."""
        return f"{en} / {de}" if self.language() == "en" else f"{de} / {en}"

    def _tip_aside(self, error):
        """💬 Move the error tooltip under the capsule so it never covers a taught value of the window."""
        tip = error.tip
        if tip.width > 4.7:
            tip.scale_to_fit_width(4.7)
        tip.move_to((CAP_AT[0], -2.15, 0.0))
        return error

    def present_anatomy(self):
        """🎞️ The whole animation: file, sheets, cell, range, formula, function, recalculation, tracing, errors and the term legend."""
        lang = self.language()
        book = _xl.workbook(language=lang)
        values = _xl.evaluate(book)
        self._values = values
        ctype = _nk.capsule_type("A")
        self._outer = {"width": values[(SHEET, "B2")], "length": values[(SHEET, "B3")], "height": values[(SHEET, "B4")]}
        base_iw, self._base_il = values[(SHEET, "B5")], values[(SHEET, "B6")]
        self._fittings = [{"width": f["width"], "length": f["length"], "name": _xl.shown(book, values, SHEET, f"C{12 + i}", lang), "area": _xl.shown(book, values, SHEET, f"G{12 + i}", lang)}
                          for i, f in enumerate(ctype.fittings)]
        self._fitting_total = _xl.shown(book, values, SHEET, "G16", lang)
        after = _interior(round(self._outer["width"], 3))
        before = _interior(round(base_iw, 3))
        fmt = {"B5": '0.00 "m"', "B9": AREA_FORMAT, "B10": VOLUME_FORMAT}
        self._delta = [f"{a1}: {_xl.display(before[a1], fmt[a1], lang)} → {_xl.display(after[a1], fmt[a1], lang)}" for a1 in ("B5", "B9", "B10")]
        self._delta.append("Δ " + _xl.display(after["B9"] - before["B9"], '+0.0 "m²"', lang))
        typemap = {(s.tower, s.storey, s.facade, s.item): s.type for s in _nk.slots()}
        self.s = SimpleNamespace(iw=_engine.ValueTracker(base_iw), il=self._base_il, dims=(), floor=False, floor_color=GOOD, interior=False, err=False, fit=0, chips=False, perim=None,
                                 area=False, delta=False, fixchip=None, show=False, status=None)

        one = next(slot for slot in _nk.slots() if slot.type == "A")
        self._one = one.path + (one.item,)
        lit_one = lambda key: key == self._one
        tower = self.tower_axon(at=(0.0, 0.65, 0.0), scale=0.105, lit=lit_one)
        chip = self._chip(self.tr("capsules in the building: ", "Kapseln im Gebäude: ") + self.number(len(_nk.slots()), 0), (0.0, -2.42, 0.0))
        mark = _engine.VGroup(self.leader(tower.capsules[self._one].get_center(), (2.3, -1.5, 0.0), None, color=DATA), self._say(self.tr("one Type A capsule", "eine Kapsel Typ A"), (2.45, -1.5), size=SMALL, color=DATA, edge="l"))
        self.step("hook", _engine.FadeIn(tower), _engine.FadeIn(chip), _engine.FadeIn(mark), seconds=2.0)

        window = self._window(book, selected="A1")
        small = self.tower_axon(at=(4.2, -0.1, 0.0), scale=0.085, lit=lit_one)
        self.step("workbook", _engine.FadeOut(chip), _engine.FadeOut(mark), _engine.FadeOut(tower), _engine.FadeIn(small), _engine.FadeIn(window), seconds=2.0)
        self.play(_engine.Circumscribe(window.title, color=GOOD), run_time=0.5)
        self.step("sheets", _engine.LaggedStart(*[_engine.Indicate(tab) for tab in window.tabs.values()], lag_ratio=0.25), seconds=3.0)
        live_tower = small
        self.s.show = True
        capsule = self._live_capsule()
        self.add(capsule)
        self.play(_engine.FadeOut(live_tower), run_time=0.01)
        self.step("grid", _engine.Circumscribe(window.headers, color=GOOD), seconds=1.5)

        self._state(dims=("width",))
        frame = self.xl_ref_frame(window, "B2", color=BLUE)
        self.step("address", window.select("B2"), _engine.FadeIn(frame), _engine.Indicate(window.cell("B2"), scale_factor=1.25), seconds=1.5)
        self._state(dims=("width", "length", "height"))
        self.s.status = window.view.number_format(SHEET, "B2")
        self.step("range", window.select("B2:B4"), seconds=1.5)
        self.s.status = None
        self._state(dims=("length",))
        top = window.top + 0.3
        callouts = _engine.VGroup(self._say(self.tr("Name Box", "Namenfeld"), (window.x0 + 0.1, top), size=SMALL, color=GOOD, edge="l"),
                                  self._say(self.tr("formula bar", "Bearbeitungsleiste"), (window.x0 + 3.2, top), size=SMALL, color=GOOD, edge="l"),
                                  _draft.segments([((window.x0 + 0.6, top - 0.14, 0), (window.x0 + 0.6, window.name_box.get_center()[1] + 0.16, 0))], GOOD, _draft.W_THIN),
                                  _draft.segments([((window.x0 + 3.9, top - 0.14, 0), (window.x0 + 3.9, window.formula_bar.get_center()[1] + 0.16, 0))], GOOD, _draft.W_THIN))
        self.step("bars", _engine.FadeOut(frame), window.select("B3"), _engine.FadeIn(callouts), seconds=1.5)
        self.play(_engine.FadeOut(callouts), run_time=0.01)
        self._state(dims=("width",))
        self.step("constant", self._hop(window, ["B2", "A2"], 2.0), seconds=1.0)
        pop = self.xl_cell_pop(window, "B9", at=(4.6, 0.55, 0.0), width=3.8)
        pop.remove(pop.leader)
        if lang == "de":
            row = pop.rows["format"]
            plain = row[1]
            row.remove(plain)
            row.add(self._say(window.view.number_format(SHEET, "B9").replace(".", ","), (plain.get_left()[0], plain.get_center()[1]), size=LABEL, color=_book.CELL_INK, edge="l"))
        self._state(show=False)
        self.step("display", window.select("B9"), _engine.FadeIn(pop), seconds=2.0)
        self.play(_engine.FadeOut(pop), run_time=0.01)
        self._state(show=True, dims=(), floor=True, floor_color=MUTED, interior=True)
        window.edit("=B5*B6")
        self.step("formula", window.select("B9"), seconds=2.0)
        frames = _engine.VGroup(self.xl_ref_frame(window, "B5", color=BLUE), self.xl_ref_frame(window, "B6", color=RED))
        self.step("frames", _engine.FadeIn(frames), seconds=2.0)
        window.edit(None)
        self._state(floor_color=GOOD, area=True)
        self.step("hatch", _engine.FadeOut(frames), _engine.Indicate(window.cell("B9")), seconds=2.0)
        self._state(area=False, interior=False)

        extra = _xl.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "A18", "value": self.tr("Perimeter, wrong", "Umfang, falsch")},
                                 {"type": "set-cell", "sheet": SHEET, "cell": "B18", "formula": "=2*B2+B3", "format": '0.00 "m"'},
                                 {"type": "set-cell", "sheet": SHEET, "cell": "A19", "value": self.tr("Perimeter, right", "Umfang, richtig")},
                                 {"type": "set-cell", "sheet": SHEET, "cell": "B19", "formula": "=2*(B2+B3)", "format": '0.00 "m"'}])
        shown = _xl.evaluate(extra)
        self._perims = [_xl.shown(extra, shown, SHEET, a1, lang) for a1 in ("B18", "B19")]
        window2 = self._window(extra, selected="B18", cols="A:B", rows=(14, 20), ribbon="Formulas")
        self._state(perim="wrong", floor=False, dims=("width", "length"))
        flip = _engine.UpdateFromAlphaFunc(_engine.ValueTracker(0.0), lambda mob, alpha: self._state(perim="wrong" if alpha < 0.5 else "right"), run_time=3.0, rate_func=_engine.linear)
        self.step("precedence", _engine.FadeOut(window), _engine.FadeIn(window2), flip, self._hop(window2, ["B18", "B19"], 3.0), seconds=3.0)
        self._state(perim=None, dims=(), floor=True, floor_color=GOOD)

        window3 = self._window(book, selected="G16", cols="C:G", rows=(10, 16), ribbon="Formulas")
        self._state(fit=len(self._fittings), chips=True)
        self.step("function", _engine.FadeOut(window2), _engine.FadeIn(window3), seconds=2.5)
        self._state(chips=False)
        formulas = _engine.VGroup(self.xl_formula_text("=SUM(G12:G15,1.5)", language="en", at=(2.9, -1.85, 0.0), ink=INK), self.xl_formula_text("=SUM(G12:G15,1.5)", language="de", at=(2.9, -2.25, 0.0), ink=INK))
        tags = _engine.VGroup(self._say("EN", (2.75, -1.78), size=SMALL, color=MUTED, edge="r"), self._say("DE", (2.75, -2.18), size=SMALL, color=MUTED, edge="r"))
        self.step("locale", _engine.FadeIn(formulas), _engine.FadeIn(tags), seconds=2.5)
        self.play(_engine.FadeOut(formulas), _engine.FadeOut(tags), run_time=0.01)

        window4 = self._window(book, selected="B5")
        self._state(fit=0, interior=True, floor_color=GOOD, area=True)
        window4.bind("B5", lambda: self.xl_display(_interior(round(self.s.iw.get_value(), 3))["B5"], '0.00 "m"', lang))
        window4.bind("B9", lambda: self.xl_display(_interior(round(self.s.iw.get_value(), 3))["B9"], AREA_FORMAT, lang))
        window4.bind("B10", lambda: self.xl_display(_interior(round(self.s.iw.get_value(), 3))["B10"], VOLUME_FORMAT, lang))
        self._state(delta=True)
        self.step("recalc", _engine.FadeOut(window3), _engine.FadeIn(window4), window4.select("B9"), _ramp(self.s.iw, base_iw, self._outer["width"], 3.0), seconds=3.0)
        self.s.iw.set_value(base_iw)
        self._state(area=False, interior=False, delta=False)

        window5 = self._window(book, selected="B9", rows=(4, 16))
        sources, users = sorted(a for s, a in _xl.precedents(book, (SHEET, "B9")) if s == SHEET and window5.visible(a)), sorted(a for s, a in _xl.dependents(book, (SHEET, "B9")) if s == SHEET and window5.visible(a))
        arrows = _engine.VGroup(self.xl_trace(window5, f"{sources[0]}:{sources[-1]}", "B9"), *[self.xl_trace(window5, "B9", addr, kind="dependent", color=WARN) for addr in users])
        self.step("trace", _engine.FadeOut(window4), _engine.FadeIn(window5), _engine.FadeIn(arrows), seconds=2.5)
        self.play(_engine.FadeOut(arrows), run_time=0.01)

        window6 = self._window(_xl.apply(book, [{"type": "clear-cell", "sheet": SHEET, "cell": "B6"}]), selected="B15", rows=(6, 15))
        self._state(err=True, floor=False, interior=True)
        self._state(fixchip=self.tr("fix: type the length into B6", "Behebung: Länge in B6 eintippen"))
        error = self._tip_aside(self.xl_error(window6, "B15", "#DIV/0!"))
        self.step("div0", _engine.FadeOut(window5), _engine.FadeIn(window6), _engine.FadeIn(error), seconds=2.5)
        self.play(_engine.FadeOut(error), run_time=0.01)

        window7 = self._window(_xl.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "G16", "formula": "=SUMM(G12:G15)"}]), selected="G16", cols="C:G", rows=(10, 16))
        self._state(err=False, floor=True, floor_color=MUTED, interior=False, fit=len(self._fittings))
        self._state(fixchip=self.tr("fix: write SUM correctly", "Behebung: SUMME richtig schreiben"))
        error = self._tip_aside(self.xl_error(window7, "G16", "#NAME?"))
        self.step("name", _engine.FadeOut(window6), _engine.FadeIn(window7), _engine.FadeIn(error), seconds=2.5)
        self.play(_engine.FadeOut(error), run_time=0.01)

        window8 = self._window(_xl.apply(book, [{"type": "delete-row", "sheet": SHEET, "index": 6}]), selected="B8", rows=(5, 14))
        self._state(err=True, floor=False, interior=True, fit=0)
        self._state(fixchip=self.tr("deleted: ", "gelöscht: ") + _xl.shown(book, values, SHEET, "A6", lang) + self.tr(" · undo restores it", " · Rückgängig stellt sie her"))
        error = self._tip_aside(self.xl_error(window8, "B8", "#REF!"))
        self.step("ref", _engine.FadeOut(window7), _engine.FadeIn(window8), _engine.FadeIn(error), seconds=2.5)
        self.play(_engine.FadeOut(error), run_time=0.01)

        broken = _xl.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "B6", "value": f"{self.number(self._base_il, 1)} m"}])
        window9 = self._window(broken, selected="B9", rows=(4, 13))
        self._state(fixchip=self.tr("fix: type the number without the unit", "Behebung: Zahl ohne Einheit eintippen"))
        error = self._tip_aside(self.xl_error(window9, "B9", "#VALUE!"))
        self.step("value", _engine.FadeOut(window8), _engine.FadeIn(window9), _engine.FadeIn(error), seconds=2.5)
        self.play(_engine.FadeOut(error), run_time=0.01)

        self._state(err=False, floor=True, floor_color=GOOD, interior=True, fit=len(self._fittings), fixchip=None)
        self.step("repair", window9.recalculate(book), seconds=2.5)

        self._state(show=False)
        panel, pulse = self._legend(window9, lang)
        self.step("legend", pulse, seconds=3.5)

    def _legend(self, window, lang):
        """📖 Eight terms in one language, each row lit together with a coloured frame around the part of the window it names."""
        bar_y, field = window.name_box.get_center()[1], window.field_left
        around = lambda mob, tone: _engine.SurroundingRectangle(mob, color=tone, buff=0.04, stroke_width=3)
        box = lambda left, right, y, tone: _draft.polyline([(left, y + 0.17), (right, y + 0.17), (right, y - 0.17), (left, y - 0.17)], tone, 3.0, closed=True)
        function = self.xl_localize("=SUM(F3:F9)", lang)
        tones = (GOOD, BLUE, WARN, RED, ARCH, STRUCT, DATA, SERV)
        targets = (around(window.title, tones[0]), around(window.tabs[SHEET], tones[1]), around(window.cell("B9"), tones[2]), around(window.cell("B5"), tones[3]),
                   box(field - 0.05, field + 1.3, bar_y, tones[4]), around(window.cell("B11"), tones[5]), box(window.x0 + 0.1, window.x0 + 1.55, bar_y, tones[6]),
                   box(field - 0.12, window.right - 0.05, bar_y, tones[7]))
        names = (self.tr("workbook", "Arbeitsmappe"), self.tr("worksheet", "Arbeitsblatt"), self.tr("cell", "Zelle"), self.tr("reference", "Bezug"), self.tr("formula", "Formel"),
                 self.tr("function", "Funktion"), self.tr("Name Box", "Namenfeld"), self.tr("formula bar", "Bearbeitungsleiste"))
        rows = _engine.VGroup()
        for index, (name, tone) in enumerate(zip(names, tones)):
            y = 2.1 - 0.5 * index
            rows.add(_engine.VGroup(_engine.Square(0.2, stroke_width=0, fill_color=tone, fill_opacity=1.0).move_to((2.3, y, 0)), self._say(name, (2.6, y), size=SMALL, edge="l", width=4.2)))
        rows[5].add(self.xl_formula_text("=SUM(F3:F9)", language=lang, at=(4.1, 2.1 - 0.5 * 5 - 0.08, 0.0), size=SMALL, ink=INK))
        pulse = _engine.LaggedStart(*[_engine.AnimationGroup(_engine.FadeIn(row), _engine.FadeIn(target)) for row, target in zip(rows, targets)], lag_ratio=0.5, run_time=3.5)
        return _engine.VGroup(rows, *targets), pulse


def _ramp(tracker, start, stop, seconds):
    """⏩ Move a tracker between two values."""
    return _engine.UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(start + (stop - start) * alpha), rate_func=_engine.smooth, run_time=seconds)
