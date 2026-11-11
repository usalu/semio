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
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
_keyed = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️keyed")

INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN = _visuals.INK, _visuals.MUTED, _visuals.ARCH, _visuals.STRUCT, _visuals.SERV, _visuals.DATA, _visuals.GOOD, _visuals.WARN
LABEL, SMALL = 19.2, 16.2
SHEET = "Type A"
WIN_AT = np.array([-2.38, -0.05, 0.0])
WIN_W, WIN_H = 8.85, 5.0
CAP_AT = np.array([4.4, 0.35, 0.0])
CAP_SCALE = 0.62
TOWER_FIT = _visuals.building_fit(5.0, 4.9, centre=(0.0, 0.45, 0.0))
SIDE_FIT = _visuals.building_fit(5.0, 4.8, centre=(4.2, -0.1, 0.0))
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
        ["hook", "Four proposed A–D studies use nominal inputs; the reference casing is 2.6 by 4.2 metres. How much usable floor does one give?",
         "Vier vorgeschlagene A–D-Studien nutzen nominale Eingaben; Referenzgehäuse: 2,6 mal 4,2 Meter. Wie viel Nutzfläche bietet eine?"],
        ["workbook", "A workbook is the whole file: Nakagin.xlsx holds the calculation of the entire building.",
         "Eine Arbeitsmappe ist die ganze Datei: Nakagin.xlsx enthält die Berechnung des ganzen Gebäudes."],
        ["sheets", "Worksheets are the tabs inside it: Program, one sheet per capsule type, Climate and Energy.",
         "Arbeitsblätter sind die Register darin: Programm, je Kapseltyp ein Blatt, Klima und Energie."],
        ["grid", "A worksheet is a grid: columns have letters, rows have numbers. Type A describes one capsule.",
         "Ein Arbeitsblatt ist ein Raster: Spalten haben Buchstaben, Zeilen Nummern. Typ A beschreibt eine Kapsel."],
        ["address", "A cell is named by column and row. B2 is the nominal study width, shown with its unit.",
         "Eine Zelle heißt nach Spalte und Zeile. B2 ist die nominale Studienbreite, angezeigt mit Einheit."],
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
        return _keyed.create(lambda: self._drawing() if self.s.show else _engine.VGroup(), self._key, self.camera)

    def _drawing(self):
        """💊 The capsule at the outer size of B2:B4 with the interior floor, its hatch, the fittings and the perimeter, all from the state dictionary and the engine values."""
        s, lang = self.s, self.language()
        iw = s.iw.get_value()
        il = self._base_il if s.il is None else s.il
        t = {"dimensional_basis": "nominal", "key": "A", **self._outer, "interior_width": iw, "interior_length": il}
        cap = self.capsule_axon(t, at=CAP_AT, scale=CAP_SCALE, dims=s.dims)
        for name in s.dims:
            cap.dims[name].set_color(DIM[name])
        if s.floor or s.err:
            cap.body.set_fill(opacity=0.3)
            cap.body.set_stroke(opacity=0.3)
        group = _engine.VGroup(cap)
        w, l = self._outer["width"], self._outer["length"]
        y0, z = (l - il) / 2, _nk.capsule_interior(t).bottom
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

    def _chip(self, text, point, color=DATA):
        """🔖 An outlined chip."""
        return self.badge(text, point, color=color, size=SMALL, filled=False, anchor="c")

    def _term(self, en, de):
        """📖 The term in the language of the narration first, the other language after the slash."""
        return f"{en} / {de}" if self.language() == "en" else f"{de} / {en}"

    def _clause(self, key, *plays):
        """🎬 One clause whose animations play one after another (morph swarms play alone) before the narration hold."""
        row = next(row for row in self.NARRATION if row[0] == key)
        caption = row[1] if self.language() == "en" else row[2]
        if self._caption is None:
            self._caption = _art.caption_bar(caption)
            self.play(_engine.FadeIn(self._caption), run_time=0.3)
            self._vo_caption_t0 = self.renderer.time
        else:
            self._caption = _art.swap_caption(self, self._caption, caption)
        for play in plays:
            if callable(play):
                play()
            else:
                self.play(*play) if isinstance(play, (list, tuple)) else self.play(play)
        localized = [(code, en, en if self.language() == "en" else de) for code, en, de in self.NARRATION]
        _art.hold_for(self, localized, key)
        gone = list(self._ghosts + self._after)
        self._ghosts.clear()
        self._after.clear()
        gone and self.play(_engine.FadeOut(*gone), run_time=0.3)
        gone and self.remove(*gone)

    def _handoff(self, window, capsule, update):
        """🔄 Hide the old workbook and capsule before changing their state."""
        self.play(_engine.FadeOut(window), _engine.FadeOut(capsule))
        update()
        capsule.update(0)

    def _repair_window(self, window, book, line, capsule):
        """🩹 Finish workbook recalculation before restoring the capsule."""
        self.play(window.recalculate(book), _engine.FadeOut(line))
        self._state(err=False, floor=True, floor_color=GOOD, interior=True, fit=len(self._fittings), fixchip=None)
        capsule.update(0)

    def _fly(self, src, dst, ghosted=True, **kwargs):
        """🕊️ A token flies to its partner with chip and text travelling together, so it stays readable mid-flight."""
        move = _engine.ReplacementTransform(src, dst, **kwargs)
        if not ghosted:
            return move
        ghost = src.copy().set_opacity(0.22)
        self._ghosts.append(ghost)
        return _engine.AnimationGroup(_engine.FadeIn(ghost, run_time=0.3), move, lag_ratio=0.0)

    def _swarm(self, srcs, dsts, **kwargs):
        """🎆 Tokens fly to their partners in order, chip and text together."""
        run = kwargs.pop("run_time", 1.2)
        lag = kwargs.pop("lag", 0.3)
        return _engine.LaggedStart(*[self._fly(a, b, run_time=run, **kwargs) for a, b in zip(srcs, dsts)], lag_ratio=lag / run)

    def _strip_points(self):
        """🟩 Screen corners of the floor strip that the widening adds, at the outer width."""
        iw = self.s.iw
        saved = iw.get_value()
        iw.set_value(self._outer["width"])
        points = self._lines().strip
        iw.set_value(saved)
        return points

    def _tok(self, text, point, color, kind="cell"):
        """🔘 An abstract token of a value at a screen point."""
        chip = _morph.token(text, kind=kind, color=color)
        chip[0].set_fill(color=color if kind == "number" else "#10222B", opacity=0.96)
        return chip.move_to(point)

    def _gauge(self, window, a1):
        """📍 The window text of one cell."""
        return window.text_of(a1)[0]

    def _lines(self):
        """📏 Screen points of the outer and interior dimension lines of the capsule at its current state."""
        iw, il = self.s.iw.get_value(), self._base_il if self.s.il is None else self.s.il
        cap = self.capsule_axon({"dimensional_basis": "nominal", "key": "A", **self._outer}, at=CAP_AT, scale=CAP_SCALE, dims=("width", "length", "height"))
        y0 = (self._outer["length"] - il) / 2
        return SimpleNamespace(width=cap.dims["width"].get_center(), length=cap.dims["length"].get_center(), height=cap.dims["height"].get_center(),
                               front=cap.at(0, y0 + il, 0.12) + np.array([0.0, -0.32, 0.0]), side=cap.at(iw / 2, y0 + il / 2, 0.12) + np.array([0.45, 0.0, 0.0]),
                               floor=cap.at(0, y0 + il / 2, 0.12), front_edge=cap.at(0, self._outer["length"], 0.0), side_edge=cap.at(self._outer["width"] / 2, self._outer["length"] / 2, 0.0),
                               back_edge=cap.at(-self._outer["width"] / 2, self._outer["length"] / 2, 0.0),
                               strip=[cap.at(x, y, 0.12)[:2] for x, y in ((self._base_iw / 2, y0), (iw / 2, y0), (iw / 2, y0 + il), (self._base_iw / 2, y0 + il))])

    def _ship(self, window, a1, code):
        """🚚 The error code leaves its cell and lands on the capsule; returns the plays of one clause."""
        text = _xl.localize_error(code, self.language())
        src = self._tok(text, window.cell(a1).get_center(), SERV, "number")
        dst = self._tok(text, np.array([CAP_AT[0], 0.95, 0.0]), SERV, "number")
        return [lambda: self.add(src), [self._fly(src, dst, path_arc=0.5, run_time=1.1)], lambda: self._after.append(dst)]

    def _veil(self, window, *cells, on):
        """🫥 Hide or show the text of cells while a flying token stands in for it."""
        for a1 in cells:
            label = window.cells[a1].text
            label is not None and label.set_opacity(0.0 if on else 1.0)

    def _tip_aside(self, error):
        """💬 Move the error tooltip under the capsule so it never covers a taught value of the window."""
        tip = error.tip
        if tip.width > 4.2:
            tip.scale_to_fit_width(4.2)
        tip.move_to((CAP_AT[0] + 0.3, -2.15, 0.0))
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
        self._delta_chip = "Δ " + _xl.display(after["B9"] - before["B9"], '+0.0 "m²"', lang)
        typemap = {(s.tower, s.storey, s.facade, s.item): s.type for s in _nk.slots()}
        self._ghosts, self._after, self._base_iw = [], [], base_iw
        self.s = SimpleNamespace(iw=_engine.ValueTracker(base_iw), il=self._base_il, dims=(), floor=False, floor_color=GOOD, interior=False, err=False, fit=0, chips=False, perim=None,
                                 area=False, delta=False, fixchip=None, show=False, status=None)

        one = next(slot for slot in _nk.slots() if slot.type == "A")
        self._one = one.path + (one.item,)
        lit_one = lambda key: key == self._one
        tower = self.tower_axon(**TOWER_FIT, lit=lit_one)
        tower_display = _snapshot.capture(tower, self.camera)
        chip = self._chip(self.tr("capsules in the building: ", "Kapseln im Gebäude: ") + self.number(len(_nk.slots()), 0), (0.0, -2.42, 0.0))
        mark = _engine.VGroup(self.leader(tower.capsules[self._one].get_center(), (2.3, -1.5, 0.0), None, color=DATA), self._say(self.tr("one Type A capsule", "eine Kapsel Typ A"), (2.45, -1.5), size=SMALL, color=DATA, edge="l"))
        self.step("hook", _engine.FadeIn(tower_display), _engine.FadeIn(chip), _engine.FadeIn(mark), seconds=2.0)

        window = self._window(book, selected="A1")
        small = self.tower_axon(**SIDE_FIT, lit=lit_one)
        small_display = _snapshot.capture(small, self.camera)
        body = _morph.silhouette(tower)
        stripes = _engine.VGroup(*[_engine.Square(0.14, fill_color=_draw.type_color(letter), fill_opacity=1.0, stroke_width=0) for letter in "ABCD"]).arrange(_engine.RIGHT, buff=0.06)
        stripes.move_to(window.title.get_center() + np.array([1.9, 0.0, 0.0]))
        filename = getattr(window.view, "file_name", "Nakagin.xlsx")
        filetok = self._tok(filename, window.title.get_center(), DATA)
        count = self._tok(self.number(len(_nk.slots()), 0) + " " + self.tr("capsules", "Kapseln"), window.title.get_center() + np.array([2.6, 0.0, 0.0]), DATA, "number")
        self._clause("workbook", [_engine.FadeOut(chip), _engine.FadeOut(mark), _engine.FadeIn(window)],
                     lambda: (self.add(body), self.remove(tower_display)), [_morph.morph(body, filetok, path_arc=0.4, run_time=1.6)],
                     [_engine.TransformFromCopy(filetok, count)], [_engine.FadeOut(filetok), _engine.FadeIn(small_display), _engine.FadeIn(stripes)])
        kinds = [(letter, next(s for s in _nk.slots() if s.type == letter)) for letter in "ABCD"]
        srcs = [small.capsules[slot.path + (slot.item,)].copy() for _, slot in kinds]
        dsts = [_morph.token("", kind="item", color=_draw.type_color(letter)).move_to(window.tabs[f"Type {letter}"].get_center()) for letter, _ in kinds]
        self.add(*srcs)
        self._clause("sheets", [_morph.scatter(srcs, dsts, lag=0.25, path_arc=0.5, run_time=2.0, pairs_fn=_morph.capsule_pairs)],
                     [_engine.FadeOut(*dsts), _engine.FadeOut(count), _engine.FadeOut(stripes), _engine.LaggedStart(*[_engine.Indicate(tab) for tab in window.tabs.values()], lag_ratio=0.25)])
        live_tower = small_display
        self.s.show = True
        capsule = self._live_capsule()
        self.add(capsule)
        self.play(_engine.FadeOut(live_tower), run_time=0.01)
        self.step("grid", _engine.Circumscribe(window.headers, color=GOOD), seconds=1.5)

        self._state(dims=("width",))
        frame = self.xl_ref_frame(window, "B2", color=BLUE)
        lines = self._lines()
        src, dst = self._tok(self._gauge(window, "B2"), lines.width, BLUE), self._tok(self._gauge(window, "B2"), window.cell("B2").get_center(), BLUE)
        self.add(src)
        self._clause("address", lambda: self._veil(window, "B2", on=True), [self._fly(src, dst, path_arc=0.5, run_time=1.3)], lambda: (self._veil(window, "B2", on=False), self.remove(dst)),
                     [self._hop(window, ["B2"], 1.0), _engine.FadeIn(frame)])
        self._state(dims=("width", "length", "height"))
        self.s.status = window.view.number_format(SHEET, "B2")
        tones = (BLUE, RED, DIM["height"])
        srcs = [self._tok(self._gauge(window, a1), point, tone) for a1, point, tone in zip(("B2", "B3", "B4"), (lines.width, lines.length, lines.height), tones)]
        dsts = [self._tok(self._gauge(window, a1), window.cell(a1).get_center(), tone) for a1, tone in zip(("B2", "B3", "B4"), tones)]
        self.add(*srcs)
        self._clause("range", lambda: self._veil(window, "B2", "B3", "B4", on=True), [self._swarm(srcs, dsts, lag=0.3, path_arc=0.5, run_time=1.6)],
                     lambda: (self._veil(window, "B2", "B3", "B4", on=False), self.remove(*dsts)), [self._hop(window, ["B2:B4"], 1.0)])
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
        shown, stored = self._gauge(window, "B9"), self.number(values[(SHEET, "B9")], 2)
        spot = pop.rows["stored"][-1].get_center()
        there = self._tok(shown, window.cell("B9").get_center(), GOOD, "number")
        card = self._tok(stored, spot, GOOD, "number")
        back = self._tok(shown, window.cell("B9").get_center(), GOOD, "number")
        self._clause("display", [window.select("B9"), _engine.FadeIn(pop)], lambda: (self.add(there), self._veil(window, "B9", on=True)),
                     [_morph.retext(there, card, run_time=1.4, path_arc=0.4)], [_morph.retext(card, back, run_time=1.4, path_arc=0.4)],
                     lambda: (self._veil(window, "B9", on=False), self.remove(back)))
        self.play(_engine.FadeOut(pop), run_time=0.01)
        self._state(show=True, dims=(), floor=True, floor_color=MUTED, interior=True)
        window.edit("=B5*B6")
        self.step("formula", window.select("B9"), seconds=2.0)
        frames = _engine.VGroup(self.xl_ref_frame(window, "B5", color=BLUE), self.xl_ref_frame(window, "B6", color=RED))
        lines = self._lines()
        srcs = [self._tok(self._gauge(window, a1), point, tone) for a1, point, tone in (("B5", lines.front, BLUE), ("B6", lines.side, RED))]
        dsts = [self._tok(self._gauge(window, a1), window.cell(a1).get_center(), tone) for a1, tone in (("B5", BLUE), ("B6", RED))]
        self.add(*srcs)
        self._clause("frames", lambda: self._veil(window, "B5", "B6", on=True), [self._swarm(srcs, dsts, lag=0.4, path_arc=0.5, run_time=1.6)],
                     lambda: (self._veil(window, "B5", "B6", on=False), self.remove(*dsts)), [_engine.FadeIn(frames, scale=1.4)])
        window.edit(None)
        self._state(floor_color=GOOD)
        total = self._gauge(window, "B9")
        src, dst = self._tok(total, window.cell("B9").get_center(), GOOD, "number"), self._tok(total, lines.floor, GOOD, "number")
        self.add(src)
        self._clause("hatch", [_engine.FadeOut(frames)], lambda: self._veil(window, "B9", on=True), [self._fly(src, dst, ghosted=False, path_arc=0.5, run_time=1.5)],
                     lambda: (self._state(area=True), self.remove(dst), self._veil(window, "B9", on=False)))
        self._state(area=False, interior=False)

        extra = _xl.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "A18", "value": self.tr("Perimeter, wrong", "Umfang, falsch")},
                                 {"type": "set-cell", "sheet": SHEET, "cell": "B18", "formula": "=2*B2+B3", "format": '0.00 "m"'},
                                 {"type": "set-cell", "sheet": SHEET, "cell": "A19", "value": self.tr("Perimeter, right", "Umfang, richtig")},
                                 {"type": "set-cell", "sheet": SHEET, "cell": "B19", "formula": "=2*(B2+B3)", "format": '0.00 "m"'}])
        shown = _xl.evaluate(extra)
        self._perims = [_xl.shown(extra, shown, SHEET, a1, lang) for a1 in ("B18", "B19")]
        window2 = self._window(extra, selected="B18", cols="A:B", rows=(14, 20), ribbon="Formulas")
        self._state(perim="wrong", floor=False, dims=("width", "length"))
        lines = self._lines()
        bar = window2.formula_bar.get_center()
        names = [self._tok(name, bar + np.array([dx, 0.0, 0.0]), tone) for name, dx, tone in (("B2", -0.5, BLUE), ("B3", 0.5, RED))]
        edges = [self._tok(name, point, tone) for name, point, tone in (("B2", lines.front_edge, BLUE), ("B3", lines.side_edge, RED))]
        glyph = self.xl_formula_text("(  )", language=lang, at=(0.0, -0.1, 0.0), ink=GOOD).scale(2.4)
        glyph.set_stroke(color=GOOD, width=2.5, opacity=1.0).set_fill(GOOD, opacity=1.0)
        plate = _engine.RoundedRectangle(corner_radius=0.12, width=glyph.width + 0.4, height=glyph.height + 0.3, stroke_color=GOOD, stroke_width=3, fill_color="#10222B", fill_opacity=1.0).move_to(glyph)
        bracket = _engine.VGroup(plate, glyph).move_to(bar + np.array([1.5, 0.0, 0.0]))
        landed = bracket.copy().move_to(lines.back_edge)
        self._clause("precedence", [_engine.FadeOut(window), _engine.FadeIn(window2)], lambda: self.add(*names, bracket),
                     [self._swarm(names, edges, lag=0.5, path_arc=0.5, run_time=1.5)], [_engine.FadeOut(*edges)],
                     [self._fly(bracket, landed, path_arc=0.5, run_time=1.4)], lambda: self._state(perim="right"), [self._hop(window2, ["B19"], 1.0), _engine.FadeOut(landed)])
        self._state(perim=None, dims=(), floor=True, floor_color=GOOD)

        window3 = self._window(book, selected="G16", cols="C:G", rows=(10, 16), ribbon="Formulas")
        self._state(fit=len(self._fittings), chips=True)
        srcs = [self._tok(fit["area"], (3.5, -1.55 - 0.36 * i, 0.0), FITTING_TONES[i % 4], "number") for i, fit in enumerate(self._fittings)]
        sigma = self._tok(self._fitting_total, window3.cell("G16").get_center(), GOOD, "number")
        self._clause("function", [_engine.FadeOut(window2), _engine.FadeIn(window3)], lambda: (self.add(*srcs), self._veil(window3, "G16", on=True)),
                     [_morph.gather(srcs, sigma, lag=0.25, run_time=1.6)], lambda: (self._veil(window3, "G16", on=False), self.remove(sigma)))
        self._state(chips=False)
        formula = "=SUM(G12:G15,1.5)"
        left, bar_y = window3.field_left + 0.12, window3.formula_bar.get_center()[1]
        wide = window3.right - window3.field_left - 0.5
        cover = _engine.Rectangle(width=wide, height=0.34, fill_color="#FFFFFF", fill_opacity=1.0, stroke_width=0).move_to((left + wide / 2 - 0.1, bar_y, 0.0))
        english, german = (self.xl_formula_text(formula, language=code, at=(left, bar_y - 0.1, 0.0), ink=_book.CELL_INK) for code in ("en", "de"))
        self._clause("locale", [_engine.FadeIn(cover), _engine.FadeIn(english)], [_morph.retext(english, german, run_time=1.8)])
        self.play(_engine.FadeOut(german), _engine.FadeOut(cover), run_time=0.3)

        window4 = self._window(book, selected="B5")
        self._state(fit=0, interior=True, floor_color=GOOD, area=True)
        window4.bind("B5", lambda: self.xl_display(_interior(round(self.s.iw.get_value(), 3))["B5"], '0.00 "m"', lang))
        window4.bind("B9", lambda: self.xl_display(_interior(round(self.s.iw.get_value(), 3))["B9"], AREA_FORMAT, lang))
        window4.bind("B10", lambda: self.xl_display(_interior(round(self.s.iw.get_value(), 3))["B10"], VOLUME_FORMAT, lang))
        self._state(delta=True)
        strip = _draft.polygon(self._strip_points(), GOOD, 0.55, GOOD, _draft.W_THIN)
        change = self._tok(self._delta_chip, np.array([6.1, -1.0, 0.0]), GOOD, "number")
        window_frame = {}
        def freeze_window():
            window_frame["display"] = _keyed.create(lambda: (window4.update(0), window4)[1], lambda: round(self.s.iw.get_value(), 3), self.camera)
            self.remove(window4, *window4.get_family())
            self.add(window_frame["display"])
        def restore_window():
            self.remove(window_frame.pop("display"))
            window4.update(0)
            self.add(window4)
        self._clause("recalc", [_engine.FadeOut(window3), _engine.FadeIn(window4), window4.select("B9")], freeze_window,
                     [_ramp(self.s.iw, base_iw, self._outer["width"], 3.0)], restore_window,
                     lambda: self.add(strip), [_morph.morph(strip, change, pairs=[(strip, change[0])], path_arc=0.5, run_time=1.5)], lambda: self._after.append(change))
        self._handoff(window4, capsule, lambda: (self.s.iw.set_value(base_iw), self._state(area=False, interior=False, delta=False)))

        window5 = self._window(book, selected="B9", rows=(4, 16))
        sources, users = sorted(a for s, a in _xl.precedents(book, (SHEET, "B9")) if s == SHEET and window5.visible(a)), sorted(a for s, a in _xl.dependents(book, (SHEET, "B9")) if s == SHEET and window5.visible(a))
        arrows = _engine.VGroup(self.xl_trace(window5, f"{sources[0]}:{sources[-1]}", "B9"), *[self.xl_trace(window5, "B9", addr, kind="dependent", color=WARN) for addr in users])
        centre = lambda a1: window5.cell(a1).get_center()
        used = [self._tok(self._gauge(window5, a1), centre(a1), tone) for a1, tone in (("B5", BLUE), ("B6", RED))]
        result = [self._tok(self._gauge(window5, "B9"), centre("B9"), GOOD, "number") for _ in users]
        self._clause("trace", [_engine.FadeIn(capsule), _engine.FadeIn(window5), _engine.FadeIn(arrows)], lambda: (self.add(*used), self._veil(window5, "B9", on=True)),
                     [_morph.travel(tok, _engine.Line(centre(a1), centre("B9")), run_time=1.2) for tok, a1 in zip(used, ("B5", "B6"))], [_engine.FadeOut(*used)],
                     lambda: (self._veil(window5, "B9", on=False), self.add(*result), self._veil(window5, *users, on=True)), [_morph.travel(tok, _engine.Line(centre("B9"), centre(a1)), run_time=1.2) for tok, a1 in zip(result, users)], [_engine.FadeOut(*result)], lambda: self._veil(window5, *users, on=False))
        self.play(_engine.FadeOut(arrows), run_time=0.01)

        window6 = self._window(_xl.apply(book, [{"type": "clear-cell", "sheet": SHEET, "cell": "B6"}]), selected="B15", rows=(6, 15))
        self._handoff(window5, capsule, lambda: self._state(err=True, floor=False, interior=True, fixchip=self.tr("fix: type the length into B6", "Behebung: Länge in B6 eintippen")))
        error = self._tip_aside(self.xl_error(window6, "B15", "#DIV/0!"))
        self._clause("div0", [_engine.FadeIn(capsule), _engine.FadeIn(window6), _engine.FadeIn(error)], *self._ship(window6, "B15", "#DIV/0!"))
        self.play(_engine.FadeOut(error), run_time=0.01)

        window7 = self._window(_xl.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "G16", "formula": "=SUMM(G12:G15)"}]), selected="G16", cols="C:G", rows=(10, 16))
        self._handoff(window6, capsule, lambda: self._state(err=False, floor=True, floor_color=MUTED, interior=False, fit=len(self._fittings), fixchip=self.tr("fix: write SUM correctly", "Behebung: SUMME richtig schreiben")))
        error = self._tip_aside(self.xl_error(window7, "G16", "#NAME?"))
        wrong, right = (self.xl_formula_text(text, language=lang, at=(3.0, 2.1, 0.0), ink=INK) for text in ("=SUMM(G12:G15)", "=SUM(G12:G15)"))
        self._clause("name", [_engine.FadeIn(capsule), _engine.FadeIn(window7), _engine.FadeIn(error), _engine.FadeIn(wrong)], *self._ship(window7, "G16", "#NAME?"),
                     [_engine.FadeOut(error)], [_morph.retext(wrong, right, run_time=1.4)])
        self.play(_engine.FadeOut(right), run_time=0.3)

        window8 = self._window(_xl.apply(book, [{"type": "delete-row", "sheet": SHEET, "index": 6}]), selected="B8", rows=(5, 14))
        self._handoff(window7, capsule, lambda: self._state(err=True, floor=False, interior=True, fit=0, fixchip=self.tr("deleted: ", "gelöscht: ") + _xl.shown(book, values, SHEET, "A6", lang) + self.tr(" · undo", " · Rückgängig")))
        error = self._tip_aside(self.xl_error(window8, "B8", "#REF!"))
        gone = [self._tok(_xl.shown(book, values, SHEET, a1, lang), window8.cell(a1).get_center(), tone) for a1, tone in (("A6", RED), ("B6", RED))]
        self._clause("ref", [_engine.FadeIn(capsule), _engine.FadeIn(window8), _engine.FadeIn(error)], lambda: self.add(*gone),
                     [*[_morph.travel(tok, _engine.Line(tok.get_center(), tok.get_center() + np.array([-0.4 + 0.8 * i, -1.3, 0.0])), run_time=1.3) for i, tok in enumerate(gone)], _engine.FadeOut(*gone)],
                     *self._ship(window8, "B8", "#REF!"))
        self.play(_engine.FadeOut(error), run_time=0.01)

        broken = _xl.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "B6", "value": f"{self.number(self._base_il, 1)} m"}])
        window9 = self._window(broken, selected="B9", rows=(4, 13))
        self._handoff(window8, capsule, lambda: self._state(fixchip=self.tr("fix: type the number without the unit", "Behebung: Zahl ohne Einheit eintippen")))
        error = self._tip_aside(self.xl_error(window9, "B9", "#VALUE!"))
        self._clause("value", [_engine.FadeIn(capsule), _engine.FadeIn(window9), _engine.FadeIn(error)], *self._ship(window9, "B9", "#VALUE!"))
        self.play(_engine.FadeOut(error), run_time=0.01)

        fixed = _xl.display(self._base_il, '0.00 "m"', lang)
        back, line = self._tok(fixed, window9.cell("B6").get_center(), RED), self._tok(fixed, self._lines().side, RED)
        self.add(back)
        self._clause("repair", [self._fly(back, line, path_arc=0.5, run_time=1.3)], lambda: self._repair_window(window9, book, line, capsule))

        self._state(show=False)
        panel, pulse = self._legend(window9, lang)
        sky = self.tower_axon(**SIDE_FIT, lit=lit_one)
        sky_display = _snapshot.capture(sky, self.camera)
        tab = _morph.token("", kind="item", color=_draw.type_color("A")).move_to(window9.tabs[SHEET].get_center())
        target = sky.capsules[self._one].copy()
        highlighted = sky.capsules[self._one].copy()
        pairs = [(whole, part) for part, whole in _morph.capsule_pairs(target, tab)]
        self._clause("legend", [pulse], [_engine.FadeOut(panel)], lambda: self.add(tab),
                     [_engine.FadeOut(window9, scale=0.3), _engine.FadeIn(sky_display)], [_morph.morph(tab, target, pairs=pairs, path_arc=0.4, run_time=1.8)],
                     lambda: (self.remove(target), self.add(highlighted)), [_engine.Indicate(highlighted, scale_factor=1.6)], lambda: self.remove(highlighted))

    def _legend(self, window, lang):
        """📖 Eight terms in one language, each row lit together with a coloured frame around the part of the window it names."""
        bar_y, field = window.name_box.get_center()[1], window.field_left
        around = lambda mob, tone: _engine.SurroundingRectangle(mob, color=tone, buff=0.04, stroke_width=3)
        box = lambda left, right, y, tone: _draft.polyline([(left, y + 0.17), (right, y + 0.17), (right, y - 0.17), (left, y - 0.17)], tone, 3.0, closed=True)
        function = self.xl_localize("=SUM(G12:G15)", lang)
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
        rows[5].add(self.xl_formula_text("=SUM(G12:G15)", language=lang, at=(4.1, 2.1 - 0.5 * 5 - 0.08, 0.0), size=SMALL, ink=INK))
        pulse = _engine.LaggedStart(*[_engine.AnimationGroup(_engine.FadeIn(target), _engine.TransformFromCopy(target, row[0]), _engine.FadeIn(row[1:])) for row, target in zip(rows, targets)], lag_ratio=0.5, run_time=3.5)
        return _engine.VGroup(rows, *targets), pulse


def _ramp(tracker, start, stop, seconds):
    """⏩ Move a tracker between two values."""
    return _engine.UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(start + (stop - start) * alpha), rate_func=_engine.smooth, run_time=seconds)
