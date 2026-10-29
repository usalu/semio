"""🎬 The Nakagin room program in Excel: fittings per capsule type, the list of all 140 capsules as a table, COUNTIF, COUNTIFS, XLOOKUP, SUMPRODUCT over the free floor, totals, a chart, a swap and a sanity check."""

import importlib
import re
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
_book = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.📗️workbook.🐍️")
_x = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_art = _visuals._art
_engine = importlib.import_module("manim")
_d = _visuals._drafting

INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN, GLASS = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "DATA", "GOOD", "WARN", "GLASS"))
S_SMALL, S_LABEL, S_BODY, W_THIN, W_EDGE, W_CUT, mix = _d.S_SMALL, _d.S_LABEL, _d.S_BODY, _d.W_THIN, _d.W_EDGE, _d.W_CUT, _d.mix
BACKGROUND = _d.BACKGROUND
LEFT = np.array([-2.65, 0.0, 0.0])
WIN_WIDTH = 8.3
FULL_HEIGHT = 5.1
PLAN_AT = np.array([2.95, 0.15, 0.0])
PLAN_SCALE = 0.82
LEGEND_AT = np.array([4.55, 1.6, 0.0])
CARD_AT = (4.55, -0.25, 0)
TOWER_AT = np.array([4.3, 0.12, 0.0])
TOWER_SCALE = 0.09
FITTING_COLORS = {"bed": DATA, "bath": STRUCT, "storage": INK, "desk": mix(INK, DATA, 0.5), "sofa": mix(STRUCT, WARN, 0.5)}
PROGRAM_CELLS = ("B4:F8", "B9:B9", "B11:F13", "B15:C15", "B16:B16", "B18:B18", "B21:D25", "B26:B26", "B27:C27")
TYPE_CELLS = ("G12:G16", "B9:B10", "B16:B16")
FALSE_RED = _art.P_RED
TYPE_COLORS = {"A": _draw.type_color("A"), "B": _draw.type_color("B"), "C": _draw.type_color("C"), "D": "#9a8cff"}
TOWER_COLORS = ("#e9dfc4", "#5f7a8a")
TRACE_BLUE = "#2F6FDE"
KEEP_ALL = ("B4:F8", "B9:B9", "B11:E13", "B15:C15", "B16:B16", "B21:D24", "B25:B25", "B26:B26", "B27:C27")
VIEW_WIDTHS = {"A:E": {"A": 11, "B": 12, "C": 12, "D": 12, "E": 12}, "A:D": {"A": 11, "B": 14, "C": 14, "D": 14}, "A:F": {"A": 11, "B": 10, "C": 10, "D": 12, "E": 12, "F": 22}, "": {"A": 11, "B": 10, "C": 24, "D": 14, "E": 14, "F": 22}}
SWAP_FROM, SWAP_TO, TYPO_TYPE = "C", "A", "B"


class _SceneView(_book.EngineView):
    """📐 Engine view with scene-local column widths for the Program sheet (one set for the A:F matrix, one for the summary windows), so the shared widths of the workbook stay untouched."""

    def __init__(self, workbook, language: str, cols: str = ""):
        super().__init__(workbook, language)
        self.widths = VIEW_WIDTHS.get(cols, VIEW_WIDTHS[""])

    def column_width(self, sheet: str, col: str) -> float:
        return float(self.widths[col]) if sheet == "Program" and col in self.widths else super().column_width(sheet, col)


def _span(a1: str) -> list:
    """🔲 Every cell address of a range, row by row."""
    c1, r1, c2, r2 = _book.parse_range(a1)
    return [_book.cell_address(c, r) for r in range(r1, r2 + 1) for c in range(c1, c2 + 1)]


class Beat3_Program(_visuals.ClipScene):
    """📋 Animate the Nakagin room program in Excel: from fittings to a verified building total, with explicit English and German narration."""

    TITLE = {"en": "The Room Program in Excel", "de": "Das Raumprogramm in Excel"}
    NARRATION = [
        ["hook", "In our teaching model the Nakagin Capsule Tower carries 140 capsules of four types. How many of each, and how much floor do they offer?",
         "Im Lehrmodell trägt der Nakagin Capsule Tower 140 Kapseln in vier Typen. Wie viele von jedem Typ, und wie viel Fläche bieten sie?"],
        ["program", "A room program lists, per room, the fittings, the areas and the users. Here it is reduced to one capsule.",
         "Ein Raumprogramm listet je Raum die Einbauten, die Flächen und die Nutzer. Hier ist es auf eine Kapsel reduziert."],
        ["sheet", "One worksheet per capsule type. The sheet of type A lists this capsule's fittings with width and length in metres.",
         "Pro Kapseltyp ein Arbeitsblatt. Das Blatt von Typ A listet die Einbauten dieser Kapsel mit Breite und Länge in Metern."],
        ["area", "A formula multiplies width, length and count. Filled down, every fitting gets its floor area and its own hatch.",
         "Eine Formel multipliziert Breite, Länge und Anzahl. Nach unten ausgefüllt erhält jeder Einbau seine Grundfläche und seine eigene Schraffur."],
        ["net", "The net floor area is the interior width times the interior length.",
         "Die Nettogrundfläche ist die Innenbreite mal die Innenlänge."],
        ["free", "Net floor area minus the fittings leaves the free floor. The occupants are a typed-in constant: one person.",
         "Nettogrundfläche minus Einbauten ergibt die freie Fläche. Die Nutzerzahl ist eine eingetippte Konstante: eine Person."],
        ["volume", "Net floor area times the interior height gives the volume in m³. The cell shows a unit but stores only the number.",
         "Nettogrundfläche mal Innenhöhe ergibt das Volumen in m³. Die Zelle zeigt eine Einheit, gespeichert ist nur die Zahl."],
        ["types", "Types B, C and D have their own sheets: the same shell, other fittings, so other free floor.",
         "Typ B, C und D haben eigene Blätter: gleiche Hülle, andere Einbauten, also andere freie Fläche."],
        ["table", "How many of each type? Select the 140 capsule rows and choose Format as Table: the list becomes the table Capsules.",
         "Wie viele von jedem Typ? Die 140 Kapselzeilen markieren und Als Tabelle formatieren wählen: Die Liste wird zur Tabelle Kapseln."],
        ["row", "Each table row is one capsule. Select a row and its capsule lights up on the tower.",
         "Jede Tabellenzeile ist eine Kapsel. Wählt man eine Zeile, leuchtet ihre Kapsel am Turm."],
        ["countif", "COUNTIF counts one type in the table column. The counted capsules light up in the colour of type A.",
         "ZÄHLENWENN zählt einen Typ in der Tabellenspalte. Die gezählten Kapseln leuchten in der Farbe von Typ A."],
        ["rest", "Filled down, the criterion moves from A4 to A7 while the table column stays whole. Four colours cover the tower.",
         "Nach unten ausgefüllt wandert das Kriterium von A4 bis A7, die Tabellenspalte bleibt ganz. Vier Farben bedecken den Turm."],
        ["countifs", "COUNTIFS adds a second condition: type and tower. The matrix shows how many of each type stand in each tower.",
         "ZÄHLENWENNS fügt eine zweite Bedingung hinzu: Typ und Turm. Die Matrix zeigt, wie viele Kapseln jedes Typs in welchem Turm stehen."],
        ["pull", "Each type's free floor comes from its own sheet. The net area is the same shell for all types; the free floor tells them apart.",
         "Die freie Fläche kommt aus dem Blatt jedes Typs. Die Nettogrundfläche ist gleich; die freie Fläche unterscheidet die Typen."],
        ["xlookup", "XLOOKUP finds the type named in the cell and returns its count and its free floor: by key, not by position.",
         "XVERWEIS sucht den genannten Typ und liefert Anzahl und freie Fläche: über den Schlüssel, nicht über die Position."],
        ["sumproduct", "SUMPRODUCT multiplies each count by its free floor and adds the products: the free floor of all capsules in m².",
         "SUMMENPRODUKT multipliziert jede Anzahl mit ihrer freien Fläche und addiert die Produkte: die freie Fläche aller Kapseln in m²."],
        ["chart", "Column D multiplies count by free floor. The bar chart shows the four totals; each bar grows as its cell computes.",
         "Spalte D multipliziert Anzahl mit freier Fläche. Das Balkendiagramm zeigt die vier Summen; jeder Balken wächst, sobald seine Zelle rechnet."],
        ["towers", "Totals per tower: SUMIF adds the net floor areas of one tower from the table. Tower A and Tower B add up to the building.",
         "Summen je Turm: SUMMEWENN addiert die Nettogrundflächen eines Turms aus der Tabelle. Turm A und Turm B ergeben das Gebäude."],
        ["swap", "Metabolism idea: capsules can be exchanged. Change one type on one storey in the list, and that capsule changes colour.",
         "Metabolismus-Idee: Kapseln sind austauschbar. Ändert man in der Liste einen Typ auf einem Geschoss, wechselt diese Kapsel die Farbe."],
        ["update", "Counts and bars update at once; each capsule keeps its free floor. The total grows, because type A has more free floor than type C.",
         "Anzahlen und Balken aktualisieren sich sofort; die freie Fläche je Kapsel bleibt. Die Summe wächst, weil Typ A mehr freie Fläche hat als Typ C."],
        ["check", "Sanity check: the counts add up to the rows of the list and to the building's 140 capsules.",
         "Plausibilitätsprüfung: Die Anzahlen ergeben die Zeilen der Liste und die 140 Kapseln des Gebäudes."],
        ["typo", "A typo, a space behind the type, is not counted: that capsule turns grey and the check shows 139 and FALSE.",
         "Ein Tippfehler, ein Leerzeichen hinter dem Typ, wird nicht gezählt: Die Kapsel wird grau, die Prüfung zeigt 139 und FALSCH."],
        ["fix", "Delete the space: the capsule is counted again and the check is TRUE. A drop-down list prevents the typo.",
         "Das Leerzeichen löschen: Die Kapsel wird wieder gezählt, die Prüfung ist WAHR. Eine Auswahlliste verhindert den Tippfehler."],
        ["recap", "Recap: from 140 capsules to floor area: table, COUNTIF(S), XLOOKUP, SUMPRODUCT, SUMIF.",
         "Rückblick: Aus 140 Kapseln wird Fläche: Tabelle, ZÄHLENWENN(S), XVERWEIS, SUMMENPRODUKT, SUMMEWENN."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native Excel visual interface."""
        self.present("program")

    def present_program(self):
        """🎞️ The whole animation: fittings and areas on the type sheets, the capsule table, counting, looking up, multiplying, summing, swapping and checking."""
        self.lang = self.language()
        self.wb = _x.workbook(language=self.lang)
        self.vals = _x.evaluate(self.wb)
        self.types = {t.key: t for t in _nk.capsule_types()}
        self.slots = _nk.slots()
        self.keys = [(s.building, s.tower, s.storey, s.facade, s.item) for s in self.slots]
        self.paint, self.litset, self.stamp = {}, None, _engine.ValueTracker(0)
        self.L, self.R = None, []
        self.swap_row = next(i + 2 for i, s in enumerate(self.slots) if s.type == SWAP_FROM)
        self.typo_row = next(i + 2 for i, s in enumerate(self.slots) if s.type == TYPO_TYPE)
        self.swapped = _x.apply(self.wb, [{"type": "set-cell", "sheet": "Program", "cell": f"M{self.swap_row}", "value": self.sheet_title(f"Type {SWAP_TO}")}])
        self.swapped_vals = _x.evaluate(self.swapped)
        self.typo = _x.apply(self.swapped, [{"type": "set-cell", "sheet": "Program", "cell": f"M{self.typo_row}", "value": self.sheet_title(f"Type {TYPO_TYPE}") + " "}])
        self.typo_vals = _x.evaluate(self.typo)
        self.counts = _engine.ValueTracker(0)
        self.area_tr = {k: _engine.ValueTracker(0.0) for k in self.types}
        self.table_name = re.search(r"(\w+)\[", _x.formula_text(self.wb, "Program", "B4", self.lang)).group(1)
        self.act_hook()
        self.act_type_sheet()
        self.act_counts()
        self.act_areas()
        self.act_edit()
        self.act_recap()

    def say(self, string, point, *, size=S_SMALL, color=INK, anchor="c", width=None):
        """🔤 One label in the clip's body face."""
        return self.text(string, point, size=size, color=color, anchor=anchor, width=width)

    def chip(self, string, point, color, anchor="c"):
        """🔖 Outlined badge."""
        return self.badge(string, point, color=color, size=S_SMALL, filled=False, anchor=anchor)

    def tcol(self, key):
        """🎨 The fixed colour of a capsule type."""
        return TYPE_COLORS[key]

    def sheet_title(self, name):
        """🏷️ A sheet's tab name in the UI language, taken from the engine."""
        return _x.tab_name(self.wb, name, self.lang)

    def cell_text(self, book, values, sheet, a1):
        """🔢 A cell exactly as the window shows it."""
        return _x.shown(book, values, sheet, a1, self.lang)

    def program_book(self, base, keep=()):
        """🧮 The workbook with every computed cell of the Program sheet cleared except the kept ones, so cells can appear one at a time."""
        kept = {cell for a1 in keep for cell in _span(a1)}
        drop = [cell for a1 in PROGRAM_CELLS for cell in _span(a1) if cell not in kept]
        return _x.apply(base, [{"type": "clear-cell", "sheet": "Program", "cell": cell} for cell in drop])

    def type_book(self, key, keep=()):
        """🧮 The workbook with the computed cells of one type sheet cleared except the kept ones."""
        kept = {cell for a1 in keep for cell in _span(a1)}
        drop = [cell for a1 in TYPE_CELLS for cell in _span(a1) if cell not in kept]
        return _x.apply(self.wb, [{"type": "clear-cell", "sheet": f"Type {key}", "cell": cell} for cell in drop])

    def window(self, book, sheet, cols, rows, selected="A1", height=FULL_HEIGHT, wide=False):
        """🪟 The Excel window over a part of one sheet at the left of the stage; shorter windows keep their top edge."""
        at = LEFT + np.array([0.35 if wide else 0.0, (FULL_HEIGHT - height) / 2, 0.0])
        view = _SceneView(book, self.lang, cols) if sheet == "Program" and cols[0] in "ABCDEF" else book
        return self.xl_window(view, sheet, at=at, width=WIN_WIDTH + (0.7 if wide else 0.0), height=height, cols=cols, rows=rows, selected=selected)

    def frames(self, window, formula):
        """🟦 One colour frame per reference of a formula that is visible in the window."""
        colors = _book.reference_colors(formula)
        return [self.xl_ref_frame(window, ref.replace("$", ""), color=color) for ref, color in colors.items()
                if "!" not in ref and window.visible(ref.replace("$", "").split(":")[0])]

    def formula_card(self, formula, at):
        """🧾 The formula in the clip language with reference tokens in the colours of their frames."""
        return self.xl_formula_text(formula, language=self.lang, colors_by_ref=_book.reference_colors(formula), at=at, size=S_LABEL, ink=INK)

    def purge(self):
        """🧹 Remove every scene object that is neither a live right-panel object nor the caption; animations split windows into loose parts that a plain remove misses."""
        keep = {id(part) for mob in self.R for part in mob.get_family()}
        if self._caption is not None:
            keep |= {id(part) for part in self._caption.get_family()}
        stale = [m for m in self.mobjects if id(m) not in keep]
        if stale:
            self.remove(*stale)

    def stage(self, *animations, left=None, right=None, add=(), drop=()):
        """⏱️ The animations of one narration clause: swap the left window or the right panel, add or drop right-panel objects."""
        plays = list(animations)
        if left is not None:
            self.purge()
            self.add(left)
            self.L = left
        if right is not None:
            plays += [_engine.FadeOut(m) for m in self.R if m not in drop]
            plays += [_engine.FadeIn(m) for m in right]
            self.R = list(right)
        for m in drop:
            if m in self.R:
                plays.append(_engine.FadeOut(m))
                self.R.remove(m)
        plays += [_engine.FadeIn(m) for m in add]
        self.R += list(add)
        return plays

    def tower(self, **kw):
        """🗼 The tower drawing: capsules neutral, painted ones in their colour, optionally only a lit set; rebuilt when the stamp changes."""
        def build():
            lit = None if self.litset is None else (lambda k, s=self.litset: k in s)
            return self.tower_axon(at=TOWER_AT, scale=TOWER_SCALE, types=False, tint=lambda k: self.paint.get(k), lit=lit, **kw)
        return self.live(build, lambda: self.stamp.get_value())

    def capsule_center(self, key):
        """🎯 Screen centre of one capsule of the tower currently drawn."""
        return self.tower_axon(at=TOWER_AT, scale=TOWER_SCALE, types=False).capsules[key].get_center()

    def bump(self):
        """🔔 Make the tower rebuild with the current paint and lit set."""
        self.stamp.set_value(self.stamp.get_value() + 1000)

    def repaint(self, assign, seconds=1.6):
        """🖌️ Animation that paints the capsules of ``assign`` (key to colour) one after the other."""
        before, order = dict(self.paint), [k for k in self.keys if k in assign]
        base = self.stamp.get_value()

        def update(mob, alpha):
            n = int(round(alpha * len(order)))
            for index, k in enumerate(order):
                self.paint[k] = assign[k] if index < n else before.get(k)
                if self.paint[k] is None:
                    self.paint.pop(k)
            mob.set_value(base + n)
        return _engine.UpdateFromAlphaFunc(self.stamp, update, run_time=seconds, rate_func=_engine.linear)

    def layout(self, t):
        """📐 Teaching layout of the fittings on the net floor: rows from the core wall, wrapping at the interior width; metres (fitting index, key, x, y, width, length)."""
        x = y = row = 0.0
        out = []
        for index, f in enumerate(t.fittings):
            for _ in range(int(f["count"])):
                if x + f["width"] > t.interior_width + 1e-9:
                    x, y, row = 0.0, y + row, 0.0
                out.append((index, f["key"], x, y, f["width"], f["length"]))
                x += f["width"]
                row = max(row, f["length"])
        return out

    def plan(self, t, at, scale, *, label=True, people=False):
        """🗺️ Capsule plan: outer shell, net floor, window on the front wall, fitting rectangles in ``fits[index]`` as (rect, ring, colour) and optionally one person glyph per occupant; ``ring`` is the net floor, ``pt(x, y)`` maps interior metres to the screen."""
        iw, il = t.interior_width, t.interior_length
        pt = lambda x, y: np.array([at[0] + (x - iw / 2) * scale, at[1] + (y - il / 2) * scale, 0.0])
        shell = _engine.Rectangle(width=t.width * scale, height=t.length * scale, stroke_color=INK, stroke_width=W_CUT, fill_opacity=0).move_to(at)
        floor = _engine.Rectangle(width=iw * scale, height=il * scale, stroke_color=MUTED, stroke_width=W_THIN, fill_color=GOOD, fill_opacity=0).move_to(at)
        d = t.window_diameter
        glass = _engine.Line(pt(iw / 2 + t.window_u - d / 2, il + 0.05), pt(iw / 2 + t.window_u + d / 2, il + 0.05), color=GLASS, stroke_width=W_CUT + 2)
        group = _engine.VGroup(shell, floor, glass)
        if label:
            group.add(self.say(self.tr("core side", "Kernseite"), (at[0] + t.width * scale / 2 + 0.5, at[1] - t.length * scale / 2 + 0.1), anchor="l", color=MUTED))
        group.floor, group.fits, group.pt = floor, {}, pt
        group.ring = [pt(0, 0), pt(iw, 0), pt(iw, il), pt(0, il)]
        group.people = _engine.VGroup()
        if people:
            for k in range(int(t.occupants)):
                group.people.add(_art.person_glyph(pt(iw - 0.3 - 0.4 * k, il - 0.27), color=INK, scale=0.62 * scale))
        for index, key, x, y, w, l in self.layout(t):
            colour = FITTING_COLORS[key]
            rect = _engine.Rectangle(width=w * scale, height=l * scale, stroke_color=colour, stroke_width=W_EDGE, fill_color=colour, fill_opacity=0).move_to((pt(x, y) + pt(x + w, y + l)) / 2)
            group.fits.setdefault(index, []).append((rect, [pt(x, y), pt(x + w, y), pt(x + w, y + l), pt(x, y + l)], colour))
        return group

    def outlines(self, plan):
        """▭ The fitting rectangles of a plan."""
        return _engine.VGroup(*[rect.copy() for fits in plan.fits.values() for rect, _, _ in fits])

    def fit_hatch(self, plan, index):
        """▨ Hatch and tint of every unit of one fitting row in its colour."""
        parts = []
        for rect, ring, colour in plan.fits[index]:
            parts.append(rect.copy().set_fill(colour, opacity=0.22))
            parts.append(self.hatch([ring], color=colour, spacing=0.07, angle=45.0, width=W_THIN, opacity=0.95))
        return _engine.VGroup(*parts)

    def legend(self, t, key, areas):
        """🏷️ Fitting legend: colour chip, localized name and, for the first ``areas`` rows, the area cell as the sheet shows it."""
        group = _engine.VGroup()
        for index, f in enumerate(t.fittings):
            y = LEGEND_AT[1] - index * 0.42
            chip = _engine.Square(0.2, stroke_color=FITTING_COLORS[f["key"]], fill_color=FITTING_COLORS[f["key"]], fill_opacity=0.9, stroke_width=W_THIN).move_to((LEGEND_AT[0] + 0.1, y, 0))
            area = f"  {self.cell_text(self.wb, self.vals, f'Type {key}', f'G{12 + index}')}" if index < areas else ""
            group.add(chip, self.say(f["name"][self.lang] + area, (LEGEND_AT[0] + 0.32, y), anchor="l", width=1.95))
        return group

    def small_plans(self, hatched=True, free=True, people=True, text_cell=None):
        """🧩 The four capsule plans side by side, each with its type tag, optional fittings hatch and a value text below."""
        parts = []
        for index, key in enumerate(self.types):
            at = np.array([2.5 + index * 1.27, 0.5, 0.0])
            small = self.plan(self.types[key], at, 0.42, label=False, people=people)
            fills = _engine.VGroup(*[self.fit_hatch(small, i) for i in small.fits]) if hatched else _engine.VGroup()
            tag = self.chip(self.sheet_title(f"Type {key}"), (at[0], 2.15, 0), self.tcol(key))
            cell = text_cell(index, key) if text_cell else self.cell_text(self.wb, self.vals, f"Type {key}", "B16")
            value = self.say((self.tr("free ", "frei ") if free else "") + cell, (at[0], -1.3, 0), color=self.tcol(key), size=S_LABEL, width=1.2)
            parts += [small, fills, small.people, tag, value]
        return parts

    def act_hook(self):
        """❓ The question on the whole tower, already painted in its four types as a teaser of the answer."""
        win = self.window(self.program_book(self.wb), "Program", "A:E", (3, 8), selected="A3")
        self.paint = {k: self.tcol(s.type) for k, s in zip(self.keys, self.slots)}
        self.bump()
        tw = self.tower()
        big = self.say(self.number(len(self.slots), 0), (3.2, 2.4), size=S_BODY, color=DATA)
        q2 = self.chip("? m²", (5.3, 2.4, 0), DATA)
        counts = [self.chip(f"{self.sheet_title(f'Type {k}')[-1]}  {self.number(self.vals[('Program', f'B{4 + i}')], 0)}", (2.55 + i * 1.07, -2.42, 0), self.tcol(k)) for i, k in enumerate(self.types)]
        self.step("hook", *self.stage(left=win, right=[tw, big, q2, *counts]), seconds=2.2)
        self.paint = {}
        self.bump()

    def act_type_sheet(self):
        """🛏️ Fittings, areas, free floor, volume and the four types."""
        t = self.types["A"]
        plan = self.plan(t, PLAN_AT, PLAN_SCALE, people=True)
        outline = self.outlines(plan)
        bed = plan.fits[0][0][1]
        bed_dim = self.dimension(bed[0], bed[1], self.number(t.fittings[0]["width"], 2) + " m", offset=-0.3, color=DATA)
        heads = [self.chip(self.tr(*w), (PLAN_AT[0] + dx, 2.35, 0), INK) for w, dx in ((("fittings", "Einbauten"), -0.65), (("areas", "Flächen"), 0.7), (("users", "Nutzer"), 1.75))]
        self.step("program", *self.stage(right=[plan, outline, plan.people, bed_dim, *heads]), seconds=2.0)
        win = self.window(self.type_book("A"), "Type A", "C:G", (11, 16), selected="C12")
        legend = self.legend(t, "A", 0)
        self.step("sheet", *self.stage(left=win, drop=[*heads, bed_dim], add=[legend]), seconds=2.0)
        formula = self.wb.cell("Type A", "G12").formula
        frames, card = self.frames(win, formula), self.formula_card(formula, CARD_AT)
        dims = [self.dimension(bed[0], bed[1], self.number(t.fittings[0]["width"], 2) + " m", offset=-0.3, color=DATA),
                self.dimension(bed[1], bed[2], self.number(t.fittings[0]["length"], 2) + " m", offset=bed[1][0] - plan.pt(t.interior_width, 0)[0] - 0.3, color=DATA)]
        last = 11 + len(t.fittings)
        hatches = [self.fit_hatch(plan, i) for i in plan.fits]
        handle = self.xl_fill_handle(win, "G12", to=f"G{last}")
        legend2 = self.legend(t, "A", len(t.fittings))
        self.step("area", *self.stage(win.select("G12"), win.recalculate(self.type_book("A", keep=(f"G12:G{last}", "G16:G16"))),
                  drop=[legend], add=[*hatches, *dims, card, *frames, handle, legend2]), seconds=2.8)
        iw, il = t.interior_width, t.interior_length
        floor_hatch = self.hatch([plan.ring], color=GOOD, spacing=0.12, angle=-45.0, width=W_THIN, opacity=0.8)
        d1 = self.dimension(plan.pt(0, 0), plan.pt(iw, 0), self.number(iw, 2) + " m", offset=-0.3, color=DATA)
        d2 = self.dimension(plan.pt(iw, 0), plan.pt(iw, il), self.number(il, 2) + " m", offset=-0.3, color=DATA)
        net_win = self.window(self.type_book("A", keep=("G12:G16",)), "Type A", "A:B", (5, 10), selected="B9")
        formula = self.wb.cell("Type A", "B9").formula
        frames2, card2 = self.frames(net_win, formula), self.formula_card(formula, CARD_AT)
        net_label = self.say(self.cell_text(self.wb, self.vals, "Type A", "B9"), (LEGEND_AT[0], 1.2), size=S_BODY, color=GOOD, anchor="l")
        self.step("net", *self.stage(net_win.select("B9"), net_win.recalculate(self.type_book("A", keep=("G12:G16", "B9:B9"))), left=net_win,
                  drop=[*hatches, *dims, card, *frames, handle, legend2], add=[floor_hatch, d1, d2, net_label, card2, *frames2]), seconds=2.2)
        free_win = self.window(self.type_book("A", keep=("G12:G16", "B9:B10")), "Type A", "A:B", (16, 17), selected="B16", height=3.0)
        formula = self.wb.cell("Type A", "B16").formula
        card3 = self.formula_card(formula, CARD_AT)
        colors = _book.reference_colors(formula)
        refs = [self.say(f"{ref.replace('$', '')}  {self.cell_text(self.wb, self.vals, 'Type A', ref.replace('$', ''))}", (CARD_AT[0], CARD_AT[1] - 0.45 - 0.35 * i), size=S_LABEL, color=color, anchor="l")
                for i, (ref, color) in enumerate(colors.items())]
        occupants = self.xl_ref_frame(free_win, "B17", color=GOOD)
        floor_fill = plan.floor.copy().set_fill(GOOD, opacity=0.25)
        fit_all = _engine.VGroup(*[self.fit_hatch(plan, i) for i in plan.fits])
        free_label = self.say(self.tr("free ", "frei ") + self.cell_text(self.wb, self.vals, "Type A", "B16"), (PLAN_AT[0] - 0.3, plan.pt(0, il - 0.3)[1]), size=S_LABEL, color=GOOD)
        legend3 = self.legend(t, "A", len(t.fittings))
        self.step("free", *self.stage(free_win.recalculate(self.type_book("A", keep=("G12:G16", "B9:B10", "B16:B16"))), left=free_win,
                  drop=[floor_hatch, d1, d2, net_label, card2, *frames2], add=[floor_fill, fit_all, free_label, card3, legend3, occupants, *refs]), seconds=2.2)
        vol_win = self.window(self.type_book("A", keep=("G12:G16", "B9:B10", "B16:B16")), "Type A", "A:B", (5, 10), selected="B10")
        box = {"key": "A", "width": t.interior_width, "length": t.interior_length, "height": t.interior_height, "corner_radius": 0.0}
        axon = self.capsule_axon(box, at=np.array([3.3, 1.75, 0.0]), scale=0.36, window=False, dims=("width", "length", "height"), base=mix(BACKGROUND, DATA, 0.55))
        vol_frames = self.frames(vol_win, self.wb.cell("Type A", "B10").formula)
        pop = self.xl_cell_pop(vol_win, "B10", at=np.array([4.2, -1.15, 0.0]), width=4.6)
        self.step("volume", *self.stage(vol_win.select("B10"), left=vol_win, right=[axon, pop, *vol_frames]), seconds=2.4)
        free_note = self.chip(self.tr("free floor per capsule", "freie Fläche je Kapsel"), (4.3, -1.8, 0), GOOD)
        self.step("types", *self.stage(left=self.window(self.wb, "Type C", "C:G", (11, 16), selected="C12"), right=[*self.small_plans(free=False), free_note]), seconds=2.4)

    def table_dressing(self, lst):
        """📑 What Format as Table adds to the list: a header tint, banded rows and a marquee around the data, taken from the window's cell rectangles."""
        left, top, _, _ = lst.rect("H1")
        _, _, right, _ = lst.rect("M1")
        parts = [self.xl_ref_frame(lst, "H1:M11", color=GOOD, dashed=True)]
        for row in range(1, 12):
            _, t0, _, b0 = lst.rect(f"H{row}")
            shade = 0.45 if row == 1 else (0.14 if row % 2 == 1 else 0.0)
            if shade:
                parts.append(_engine.Rectangle(width=right - left, height=t0 - b0, stroke_width=0, fill_color=GOOD, fill_opacity=shade).move_to(((left + right) / 2, (t0 + b0) / 2, 0)))
        return parts

    def act_counts(self):
        """📋 The capsule table, COUNTIF, COUNTIFS."""
        lst = self.window(self.wb, "Program", "H:M", (1, 11), selected="H1")
        self.paint = {}
        self.bump()
        tw = self.tower()
        ribbon = self.chip(self.tr("Home > Format as Table", "Start > Als Tabelle formatieren"), (TOWER_AT[0], 2.4, 0), GOOD)
        name_chip = self.chip(f"{self.tr('Table name', 'Tabellenname')}: {self.table_name}", (TOWER_AT[0], -2.42, 0), GOOD)
        self.step("table", *self.stage(lst.select("H1"), left=lst, right=[tw, ribbon, name_chip, *self.table_dressing(lst)]), seconds=2.6)
        key = self.keys[self.swap_row - 2]
        self.litset, self.paint = {key}, {key: self.tcol(SWAP_FROM)}
        self.bump()
        tag = self.say(self.vals[("Program", f"H{self.swap_row}")], (TOWER_AT[0], -2.42), size=S_LABEL)
        row_frame = self.xl_ref_frame(lst, f"H{self.swap_row}:M{self.swap_row}", color=DATA, dashed=True)
        self.step("row", *self.stage(lst.select(f"M{self.swap_row}"), drop=[ribbon, name_chip], add=[tag, row_frame]), seconds=2.2)
        self.litset, self.paint = None, {}
        self.bump()
        summary = self.window(self.program_book(self.wb), "Program", "A:E", (3, 8), selected="B4")
        formula = self.wb.cell("Program", "B4").formula
        frames = self.frames(summary, formula)
        counter = self.live_label(lambda: self.number(round(self.counts.get_value()), 0), (TOWER_AT[0], 2.4, 0), size=S_BODY, color=self.tcol("A"))
        total_counter = self.live_label(lambda: self.number(round(self.counts.get_value()), 0), (TOWER_AT[0], 2.4, 0), size=S_BODY, color=INK)
        token = re.search(r"\w+\[[^\]]+\]", _x.formula_text(self.wb, "Program", "B4", self.lang)).group(0)
        callout = self.chip(f"{token} = M2:M141", (TOWER_AT[0], -2.42, 0), INK)
        self.counts.set_value(0)
        a_keys = {k: self.tcol("A") for k, s in zip(self.keys, self.slots) if s.type == "A"}
        self.step("countif", *self.stage(self.repaint(a_keys), _engine.UpdateFromAlphaFunc(self.counts, lambda m, a: m.set_value(self.vals[("Program", "B4")] * a), run_time=1.6),
                  summary.recalculate(self.program_book(self.wb, keep=("B4:B4",))), left=summary, drop=[tag, row_frame], add=[counter, callout, *frames]), seconds=2.4)
        handle = self.xl_fill_handle(summary, "B4", to="B7")
        everything = {k: self.tcol(s.type) for k, s in zip(self.keys, self.slots)}
        total, first = len(self.slots), self.vals[("Program", "B4")]
        self.step("rest", *self.stage(summary.select("B4"), summary.recalculate(self.program_book(self.wb, keep=("B4:B8",))), self.repaint(everything),
                  _engine.UpdateFromAlphaFunc(self.counts, lambda m, a: m.set_value(first + (total - first) * a), run_time=1.6), drop=[*frames, counter], add=[handle, total_counter]), seconds=2.6)
        mat = self.window(self.program_book(self.wb, keep=("B4:B8",)), "Program", "A:E", (10, 13), selected="B11", wide=True)
        formula = self.wb.cell("Program", "B11").formula
        frames = self.frames(mat, formula)
        only = {k: self.tcol("A") for k, s in zip(self.keys, self.slots) if s.type == "A" and s.tower == 0}
        self.litset, self.paint = set(only), only
        self.bump()
        self.step("countifs", *self.stage(mat.select("B11"), mat.recalculate(self.program_book(self.wb, keep=("B4:B8", "B11:E13"))), left=mat, drop=[handle, total_counter, callout], add=frames), seconds=2.6)

    def act_areas(self):
        """📐 Free floor per type, XLOOKUP, SUMPRODUCT, the chart and the tower totals."""
        kept = ("B4:B8", "B11:E13")
        self.litset, self.paint = None, {}
        self.bump()
        pull = self.window(self.program_book(self.wb, keep=kept), "Program", "A:D", (20, 27), selected="C21")
        arrows = [self.xl_trace(pull, f"'Type {k}'!B16", f"C{21 + i}") for i, k in enumerate(self.types)]
        net_note = self.chip(f"{self.tr('net floor area, all types', 'Nettogrundfläche, alle Typen')}: {self.cell_text(self.wb, self.vals, 'Program', 'C4')}", (4.3, -2.1, 0), GOOD)
        minis = self.small_plans(free=False, text_cell=lambda i, k: self.cell_text(self.wb, self.vals, "Program", f"C{21 + i}"))
        self.step("pull", *self.stage(pull.select("C21"), pull.recalculate(self.program_book(self.wb, keep=(*kept, "C21:C24"))), left=pull, right=[*minis, net_note], add=arrows), seconds=2.8)
        look = self.window(self.program_book(self.wb, keep=(*kept, "B21:C24")), "Program", "A:D", (19, 27), selected="C27")
        formula = self.wb.cell("Program", "C27").formula
        frames = self.frames(look, formula)
        t = self.types["C"]
        plan = self.plan(t, PLAN_AT, PLAN_SCALE, label=False, people=True)
        fills = _engine.VGroup(*[self.fit_hatch(plan, i) for i in plan.fits])
        legend = self.legend(t, "C", len(t.fittings))
        floor = plan.floor.copy().set_fill(GOOD, opacity=0.25)
        tag = self.chip(self.sheet_title("Type C"), (PLAN_AT[0], 2.35, 0), self.tcol("C"))
        val = self.say(self.tr("free ", "frei ") + self.cell_text(self.wb, self.vals, "Program", "C27"), (PLAN_AT[0], -1.95), size=S_BODY, color=GOOD)
        count = self.say(self.tr("count ", "Anzahl ") + self.cell_text(self.wb, self.vals, "Program", "B27"), (LEGEND_AT[0], -0.5), size=S_BODY, color=self.tcol("C"), anchor="l")
        self.step("xlookup", *self.stage(look.select("C27"), look.recalculate(self.program_book(self.wb, keep=(*kept, "B21:C24", "B27:C27"))), left=look,
                  right=[plan, floor, fills, plan.people, legend, tag, val, count, *frames]), seconds=2.8)
        sp = self.window(self.program_book(self.wb, keep=(*kept, "B21:C24", "B27:C27")), "Program", "A:D", (20, 27), selected="B26")
        sum_frames = self.frames(sp, self.wb.cell("Program", "B26").formula)
        tiles = self.product_tiles()
        card = self.formula_card(self.wb.cell("Program", "B26").formula, (1.9, -2.3, 0))
        total_label = self.say("= " + self.cell_text(self.wb, self.vals, "Program", "B26"), (2.0, -1.5), size=S_BODY, color=GOOD, anchor="l")
        self.step("sumproduct", *self.stage(sp.select("B26"), sp.recalculate(self.program_book(self.wb, keep=(*kept, "B21:C24", "B26:B26", "B27:C27"))), left=sp,
                  right=[*tiles, card, total_label, *sum_frames]), seconds=3.0)
        win = self.window(self.program_book(self.wb, keep=(*kept, "B21:C24", "B26:B26", "B27:C27")), "Program", "A:D", (20, 27), selected="D21")
        rows = [(self.sheet_title(f"Type {k}"), self.area_tr[k], self.tcol(k)) for k in self.types]
        chart = self.xl_bar_chart(rows, at=np.array([4.3, 0.1, 0.0]), width=4.5, height=3.7, unit="m²", title=self.tr("Free floor by type", "Freie Fläche je Typ"), decimals=1)
        grow = [_engine.UpdateFromAlphaFunc(self.area_tr[k], (lambda m, a, i=i: m.set_value(self.vals[("Program", f"D{21 + i}")] * a)), run_time=2.0) for i, k in enumerate(self.types)]
        self.step("chart", *self.stage(win.select("D21"), win.recalculate(self.program_book(self.wb, keep=KEEP_ALL)), *grow, left=win, right=[chart]), seconds=3.0)
        tw_win = self.window(self.program_book(self.wb, keep=("B4:F8", "B11:E13")), "Program", "A:F", (10, 13), selected="F11", wide=True)
        formula = self.wb.cell("Program", "F11").formula
        frames = self.frames(tw_win, formula)
        self.paint = {k: TOWER_COLORS[s.tower] for k, s in zip(self.keys, self.slots)}
        self.bump()
        tower = self.tower()
        parts = [self.cell_text(self.wb, self.vals, "Program", f"F{r}") for r in (11, 12, 13)]
        equal = self.chip(f"{parts[0]} + {parts[1]} = {parts[2]}", (TOWER_AT[0], -2.42, 0), GOOD)
        names = [self.tr("Tower A", "Turm A"), self.tr("Tower B", "Turm B")]
        n_towers = [sum(self.vals[("Program", f"{c}{11 + i}")] for c in "BCDE") for i in range(2)]
        ta = self.chip(f"{names[0]} · {self.number(n_towers[0], 0)}", (3.25, 2.4, 0), TOWER_COLORS[0])
        tb = self.chip(f"{names[1]} · {self.number(n_towers[1], 0)}", (5.75, 2.4, 0), "#9db5c4")
        self.step("towers", *self.stage(tw_win.select("F11"), tw_win.recalculate(self.program_book(self.wb, keep=(*KEEP_ALL, "F11:F13"))), left=tw_win,
                  right=[tower, equal, ta, tb, *frames]), seconds=2.8)

    def product_tiles(self):
        """🟦 One row of tiles per type: as many tiles as capsules, with the product count times free floor written below."""
        out = []
        for index, key in enumerate(self.types):
            n = int(self.vals[("Program", f"B{21 + index}")])
            y = 1.9 - index * 0.85
            row = _engine.VGroup(*[_engine.Square(0.1, stroke_width=0, fill_color=self.tcol(key), fill_opacity=0.9).move_to((2.0 + 0.115 * i, y, 0)) for i in range(n)])
            text = f"{n} × {self.number(self.vals[('Program', f'C{21 + index}')], 2)} m² = {self.cell_text(self.wb, self.vals, 'Program', f'D{21 + index}')}"
            out += [row, self.say(text, (2.0, y - 0.3), anchor="l", color=self.tcol(key), width=4.6)]
        return out

    def below_arrow(self, window, src, dst):
        """➡️ A dependency arrow along the lower edge of a row, from one cell to another, clear of the cell values."""
        a, b = window.point(src, "d"), window.point(dst, "d")
        b = np.array([window.rect(dst)[0] + 0.12, b[1], 0.0])
        y = a[1] + 0.045
        arrow = _engine.Arrow(np.array([a[0], y, 0.0]), np.array([b[0], y, 0.0]), buff=0.0, color=TRACE_BLUE, stroke_width=3, tip_length=0.14, max_tip_length_to_length_ratio=0.5, max_stroke_width_to_length_ratio=8)
        return _engine.VGroup(arrow, _engine.Dot(np.array([a[0], y, 0.0]), radius=0.045, color=TRACE_BLUE))

    def act_edit(self):
        """🔁 Swap, update, check, typo and fix."""
        old, new, key = SWAP_FROM, SWAP_TO, self.keys[self.swap_row - 2]
        lst = self.window(self.wb, "Program", "H:M", (1, 11), selected=f"M{self.swap_row}")
        edited = self.xl_ref_frame(lst, f"M{self.swap_row}", color=self.tcol(new))
        self.paint = {k: self.tcol(s.type) for k, s in zip(self.keys, self.slots)}
        self.litset = {key}
        self.bump()
        tower = self.tower()
        chips = [self.chip(self.sheet_title(f"Type {old}"), (3.2, -2.42, 0), self.tcol(old)), self.chip("→", (4.3, -2.42, 0), INK), self.chip(self.sheet_title(f"Type {new}"), (5.4, -2.42, 0), self.tcol(new))]
        self.step("swap", *self.stage(lst.select(f"M{self.swap_row}"), lst.recalculate(self.swapped), self.repaint({key: self.tcol(new)}, seconds=1.0), left=lst, right=[tower, *chips], add=[edited]), seconds=2.8)
        self.litset = None
        self.bump()
        before = self.program_book(self.wb, keep=KEEP_ALL)
        win = self.window(before, "Program", "A:D", (20, 27), selected="B21")
        rows = [(self.sheet_title(f"Type {k}"), self.area_tr[k], self.tcol(k)) for k in self.types]
        for i, k in enumerate(self.types):
            self.area_tr[k].set_value(self.vals[("Program", f"D{21 + i}")])
        chart = self.xl_bar_chart(rows, at=np.array([4.3, 0.1, 0.0]), width=4.5, height=3.7, unit="m²", title=self.tr("Free floor by type", "Freie Fläche je Typ"), decimals=1)
        va = self.swapped_vals
        trace = [self.below_arrow(win, "B21", "D21"), self.below_arrow(win, "B23", "D23")]
        delta = [self.chip("+" + self.number(va[("Program", "B21")] - self.vals[("Program", "B21")], 0), (3.4, 2.45, 0), self.tcol(new)),
                 self.chip(self.number(va[("Program", "B23")] - self.vals[("Program", "B23")], 0), (5.05, 2.45, 0), self.tcol(old))]
        gain = va[("Program", "B26")] - self.vals[("Program", "B26")]
        total_chip = self.chip(f"{self.tr('total', 'Summe')} {'+' if gain >= 0 else ''}{self.number(gain, 1)} m²", (4.3, -2.42, 0), GOOD)
        move = [_engine.UpdateFromAlphaFunc(self.area_tr[k], (lambda m, a, s=self.vals[("Program", f"D{21 + i}")], e=va[("Program", f"D{21 + i}")]: m.set_value(s + (e - s) * a)), run_time=1.8)
                for i, k in enumerate(self.types)]
        self.step("update", *self.stage(win.recalculate(self.program_book(self.swapped, keep=KEEP_ALL)), *move, left=win, right=[chart, *delta, total_chip], add=trace), seconds=3.0)
        check = self.window(self.program_book(self.swapped, keep=KEEP_ALL), "Program", "A:C", (17, 18), selected="B18", height=3.0)
        self.paint = {k: self.tcol(s.type) for k, s in zip(self.keys, self.slots)}
        self.paint[key] = self.tcol(new)
        self.bump()
        tower2 = self.tower_keep = self.tower()
        ok = self.badge(self.cell_text(self.swapped, self.swapped_vals, "Program", "B18"), (TOWER_AT[0], 2.4, 0), color=GOOD)
        eq = self.chip(f"{self.number(self.swapped_vals[('Program', 'B8')], 0)} = {self.number(self.swapped_vals[('Program', 'B17')], 0)}", (TOWER_AT[0], -2.42, 0), GOOD)
        self.step("check", *self.stage(check.select("B18"), check.recalculate(self.swapped), left=check, right=[tower2, ok, eq]), seconds=2.6)
        tkey = self.keys[self.typo_row - 2]
        lst2 = self.window(self.swapped, "Program", "H:M", (1, 11), selected=f"M{self.typo_row}")
        name = self.sheet_title(f"Type {TYPO_TYPE}")
        self.paint.pop(tkey, None)
        self.litset = {tkey}
        self.bump()
        bad = self.badge(self.cell_text(self.typo, self.typo_vals, "Program", "B18"), (TOWER_AT[0], 2.4, 0), color=FALSE_RED)
        eq2 = self.chip(f"{self.number(self.typo_vals[('Program', 'B8')], 0)} ≠ {self.number(self.typo_vals[('Program', 'B17')], 0)}", (TOWER_AT[0], -2.42, 0), FALSE_RED)
        quoted = self.chip(f'"{name} "', (2.7, 2.4, 0), FALSE_RED)
        mark = self.say(self.tr("mark = space", "Marke = Leerzeichen"), (1.55, -2.42), size=S_LABEL, color=FALSE_RED, anchor="l", width=1.5)
        cell_frame = self.xl_ref_frame(lst2, f"M{self.typo_row}", color=FALSE_RED)
        edge = lst2.cell(f"M{self.typo_row}").text.get_right()
        dot = _engine.Rectangle(width=0.1, height=0.2, stroke_width=0, fill_color=FALSE_RED, fill_opacity=0.95).move_to((edge[0] + 0.09, lst2.point(f"M{self.typo_row}")[1], 0))
        ring = _engine.Circle(radius=0.3, stroke_color=FALSE_RED, stroke_width=4).move_to(self.capsule_center(tkey))
        self.step("typo", *self.stage(lst2.recalculate(self.typo), left=lst2, drop=[ok, eq], add=[bad, eq2, quoted, mark, cell_frame, dot, ring]), seconds=2.4)
        good = self.badge(self.cell_text(self.swapped, self.swapped_vals, "Program", "B18"), (TOWER_AT[0], 2.4, 0), color=GOOD)
        eq3 = self.chip(f"{self.number(self.swapped_vals[('Program', 'B8')], 0)} = {self.number(self.swapped_vals[('Program', 'B17')], 0)}", (TOWER_AT[0], -2.42, 0), GOOD)
        fixed = self.chip(f'"{name}"', (2.7, 2.4, 0), GOOD)
        self.paint[tkey] = self.tcol(TYPO_TYPE)
        self.bump()
        self.step("fix", *self.stage(lst2.recalculate(self.swapped), drop=[bad, eq2, quoted, mark, cell_frame, dot, ring], add=[good, eq3, fixed]), seconds=2.6)
        self.litset = None
        self.bump()

    def act_recap(self):
        """🧾 Legend of terms in both languages beside the lit tower."""
        terms = [(self.tr("table", "Tabelle"), self.tr("one row per capsule, columns by name", "eine Zeile je Kapsel, Spalten über ihren Namen")),
                 ("COUNTIF", self.tr("counts the rows of one type", "zählt die Zeilen eines Typs")),
                 ("COUNTIFS", self.tr("counts rows meeting two conditions", "zählt Zeilen mit zwei Bedingungen")),
                 ("XLOOKUP", self.tr("finds a value by its key", "findet einen Wert über seinen Schlüssel")),
                 ("SUMPRODUCT", self.tr("sums counts times free floor", "summiert Anzahl mal freie Fläche")),
                 ("SUMIF", self.tr("sums the areas of one tower", "summiert die Flächen eines Turms"))]
        rows = []
        for index, (name, meaning) in enumerate(terms):
            y = 2.1 - index * 0.68
            shown = name if self.lang == "en" or index == 0 else _x.DE_FUNCTIONS.get(name, name)
            rows += [self.chip(shown, (-6.6, y, 0), INK, anchor="l"), self.say(meaning, (-3.85, y), anchor="l", width=4.2)]
        total = self.say(self.number(len(self.slots), 0) + " " + self.tr("capsules", "Kapseln"), (-6.6, -2.1), size=S_BODY, color=DATA, anchor="l")
        area = self.say(self.cell_text(self.swapped, self.swapped_vals, "Program", "B26") + " " + self.tr("free", "frei"), (-3.4, -2.1), size=S_BODY, color=GOOD, anchor="l")
        self.purge()
        self.L = None
        self.step("recap", *self.stage(drop=[m for m in self.R if m is not self.tower_keep], add=[*rows, total, area]), seconds=3.0)
