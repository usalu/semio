"""🎬 Heating demand of the Nakagin capsules in Excel: surfaces, U-values, H_T and H_V, load, annual demand, building total and a live what-if."""

import importlib
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_xl = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_drawing = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, STRUCT, SERV, DATA, GOOD, WARN, GLASS, CONCRETE, ARCH = (
    getattr(_visuals, name) for name in ("INK", "MUTED", "STRUCT", "SERV", "DATA", "GOOD", "WARN", "GLASS", "CONCRETE", "ARCH"))
BACKGROUND = _draft.BACKGROUND
LABEL, SMALL = 19.2, 17.0
LEFT, WIN_H = -7.05, 5.0
COLS = {"A:B": 6.2, "C:F": 7.6, "A:D": 10.0, "A:B+": 8.0, "D:F": 6.4}
SCALE = 0.46
BLUE = "#6aa8ff"
ARROW_PER_UA, ARROW_WIDTH = 0.14, 1.3
ROWS = {3: "top", 4: "bottom", 5: "left", 6: "right", 7: "front", 8: "window", 9: "back"}
ROW_COLOR = {3: STRUCT, 4: WARN, 5: ARCH, 6: GOOD, 7: DATA, 8: BLUE, 9: MUTED}
POINT = {"top": (0.0, 2.0, 2.5), "bottom": (0.0, 2.0, 0.0), "left": (-1.25, 2.0, 1.25), "right": (1.25, 2.0, 1.25), "front": (0.9, 4.0, 0.3), "air": (0.75, 4.0, 2.1),
         "window": (0.0, 4.0, 1.25), "back": (0.0, 0.0, 1.25)}
NORMAL = {"top": (0, 0, 1), "bottom": (0, 0, -1), "left": (-1, 0, 0), "right": (1, 0, 0), "front": (0, 1, 0), "air": (-0.3, 0.7, 0.6), "window": (0, 1, 0), "back": (0, -1, 0)}
HIDDEN = {"bottom": 0.55, "left": 0.7, "back": 0.7}
BASIS = {"sourced": (GOOD, "sourced", "belegt"), "assumed": (WARN, "assumed", "angenommen"), "derived": (MUTED, "derived", "abgeleitet")}


class Beat4_Energy(_visuals.ClipScene):
    """⚡ Present the energy calculation of the Nakagin capsule types in the Excel workbook with explicit English and German narration."""

    TITLE = {"en": "Heating Demand in the Nakagin Workbook", "de": "Heizwärmebedarf in der Nakagin-Arbeitsmappe"}
    NARRATION = [
        ["hook", "How much heat does one Nakagin capsule need in a Tokyo winter? We calculate it in Excel, sheet by sheet.",
         "Wie viel Wärme braucht eine Nakagin-Kapsel im Tokioter Winter? Wir rechnen es in Excel, Blatt für Blatt."],
        ["sheet", "Every capsule type has its own worksheet. Type A holds the dimensions, surfaces and results of this simplified teaching model.",
         "Jeder Kapseltyp hat ein eigenes Arbeitsblatt. Typ A enthält Maße, Flächen und Ergebnisse dieses vereinfachten Lehrmodells."],
        ["dims", "Three constants describe the shell: width, length and height. On the capsule they are the three dimension lines.",
         "Drei Konstanten beschreiben die Hülle: Breite, Länge und Höhe. An der Kapsel sind es die drei Maßlinien."],
        ["areas", "Each surface area is a formula from these dimensions. The front wall is width times height minus the round window.",
         "Jede Fläche ist eine Formel aus diesen Maßen. Die Frontwand ist Breite mal Höhe abzüglich des runden Fensters."],
        ["exposed", "The back wall is bolted to the core, so only part of its heat escapes: its U·A is multiplied by the attached factor b.",
         "Die Rückwand ist am Kern verschraubt, deshalb entweicht nur ein Teil ihrer Wärme: Ihr U·A wird mit dem Anbindungsfaktor b multipliziert."],
        ["uvalues", "The U-value tells how many watts pass one square metre for each kelvin of temperature difference. Glass leaks most.",
         "Der U-Wert sagt, wie viele Watt je Quadratmeter und Kelvin Temperaturunterschied hindurchgehen. Glas verliert am meisten."],
        ["ua", "Area times U-value is U·A in watts per kelvin. The larger the U·A of a surface, the longer and thicker its arrow.",
         "Fläche mal U-Wert ergibt U·A in Watt je Kelvin. Je größer das U·A einer Fläche, desto länger und dicker ihr Pfeil."],
        ["ht", "H_T sums all U·A. SUMPRODUCT does it in one formula: area times U-value times temperature factor, back wall included.",
         "H_T ist die Summe aller U·A. SUMMENPRODUKT: Fläche mal U-Wert mal Temperaturfaktor, die Rückwand eingeschlossen."],
        ["climate", "The Climate sheet stores Tokyo under defined names: temperatures, degree days, b, air change and air heat capacity.",
         "Das Blatt Klima speichert Tokio unter definierten Namen: Temperaturen, Gradtage, b, Luftwechsel und Luftwärmekapazität."],
        ["degreedays", "Heating degree days are an area: each month's shortfall below the 18 degree line, times its days, adds up over the year.",
         "Heizgradtage sind eine Fläche: Der Fehlbetrag jedes Monats unter der 18-Grad-Linie mal seine Tage, summiert über das Jahr."],
        ["mistake", "A typical mistake: treating the attached back wall like an exposed one. With b at one, H_T grows and the loss is overestimated.",
         "Ein typischer Fehler: Die angebundene Rückwand zählt wie eine freie Wand. Mit b gleich eins wächst H_T, der Verlust wird überschätzt."],
        ["fix", "The factor is one cell on the Climate sheet. Set it back, and the back-wall arrow and H_T return to their values.",
         "Der Faktor steht in einer Zelle auf dem Blatt Klima. Setzt man ihn zurück, kehren Rückwand-Pfeil und H_T zu ihren Werten zurück."],
        ["volume", "Air leaks too: air heat capacity times air change rate times volume gives the ventilation loss H_V.",
         "Auch Luft entweicht: Luftwärmekapazität mal Luftwechselrate mal Volumen ergibt den Lüftungswärmeverlust H_V."],
        ["total", "H_T plus H_V is the heat loss of the whole capsule per kelvin. Every later result is built on these two cells.",
         "H_T und H_V zusammen sind der Wärmeverlust der ganzen Kapsel je Kelvin. Alle weiteren Ergebnisse bauen auf diesen beiden Zellen auf."],
        ["load", "The heating load Φ is the loss per kelvin times the temperature difference: the watts the heater must deliver on the design day.",
         "Die Heizlast Φ ist der Verlust je Kelvin mal Temperaturdifferenz: die Watt, die die Heizung am Auslegungstag liefern muss."],
        ["annual", "Yearly demand: loss per kelvin times degree days times 24 hours, divided by 1000, in kWh. Divided by floor area: kWh per m².",
         "Jahresbedarf: Verlust je Kelvin mal Gradtage mal 24 Stunden, geteilt durch 1000, in kWh. Geteilt durch die Fläche: kWh je m²."],
        ["energy", "The Energy sheet multiplies each type's demand by its capsule count from the Program sheet and adds up the whole building.",
         "Das Blatt Energie multipliziert den Bedarf jedes Typs mit seiner Kapselzahl aus dem Blatt Programm und summiert das ganze Gebäude."],
        ["compare", "A, B and D share one shell, and fittings do not change the heat loss. Only C, with its larger window, needs more.",
         "A, B und D teilen eine Hülle, Einbauten ändern den Verlust nicht. Nur C mit dem größeren Fenster braucht mehr."],
        ["shape", "Why so much for so little floor? Five faces and a window wrap a small room. Heated neighbours above and below would cut the demand.",
         "Warum so viel für so wenig Boden? Fünf Außenflächen umhüllen einen kleinen Raum. Beheizte Nachbarn würden den Bedarf deutlich senken."],
        ["trace", "Which cells depend on the window U-value? Trace the dependents: they run through the type sheets to the building total.",
         "Welche Zellen hängen vom U-Wert des Fensters ab? Die Nachfolger laufen durch die Typ-Blätter bis zur Gebäudesumme."],
        ["whatif", "Enter a better window in this one input cell, and every dependent cell, arrow and total follows.",
         "Ein besseres Fenster in dieser einen Eingabezelle genügt, und jede abhängige Zelle, jeder Pfeil und jede Summe folgt."],
        ["frozen", "A typical mistake: someone types the building total as a constant. It no longer follows the change.",
         "Ein typischer Fehler: Jemand tippt die Gebäudesumme als Konstante ein. Sie folgt der Änderung nicht mehr."],
        ["repaired", "Put the formula back, and the total matches its rows again.",
         "Setzt man die Formel wieder ein, stimmt die Summe wieder mit ihren Zeilen überein."],
        ["recap", "Recap: U·A per surface, H_T and H_V, load from the temperature difference, demand from degree days, times the capsule count.",
         "Zusammenfassung: U·A je Fläche, H_T und H_V, Last aus der Temperaturdifferenz, Bedarf aus Gradtagen, mal Kapselzahl."]
    ]

    def construct(self):
        """▶️ Present this chapter through the native Excel visual interface."""
        self.present("energy")

    def _say(self, string, point, *, size=LABEL, color=INK, anchor="c", width=None):
        """🔤 One label anchored at a point; parentheses are drawn in the theme serif face because the body face draws brackets."""
        if "(" not in string and ")" not in string:
            return self.text(string, tuple(point), size=size, color=color, anchor=anchor, width=width)
        _fonts.register_fonts(_theme.FONT_SERIF)
        shape = _fonts.body_text(string, font_size=size, color=color, t2f={"(": _theme.FONT_SERIF, ")": _theme.FONT_SERIF})
        shape.descent = _draft.descent(size) if _draft.DESCENDERS & set(string) else 0.0
        if width and shape.width > width:
            shape.descent *= width / shape.width
            shape.scale_to_fit_width(width)
        return _draft.place(shape, tuple(point), anchor)

    def _num(self, sheet, a1):
        """🔢 Value of a cell, blended between the previous and the next workbook state while a change is animated."""
        before, after = self.vs.a.get((sheet, a1)), self.vs.b.get((sheet, a1), self.vs.a.get((sheet, a1)))
        if isinstance(before, (int, float)) and isinstance(after, (int, float)):
            return before + (after - before) * self.vs.t.get_value()
        return before

    def _show(self, sheet, a1):
        """🧾 The cell text as shown in the window: value with number format and language."""
        item = self.book.cell(sheet, a1)
        return self.xl_display(self._num(sheet, a1), item.format if item is not None and item.format else "General")

    def _label(self, sheet, a1):
        """🏷️ The bilingual label of a text cell."""
        return self.book.cell(sheet, a1).labels[self.language()]

    def _live(self, window, sheet):
        """🔗 Bind every numeric cell of a window to the blended workbook state."""
        for a1 in list(window.cells):
            item = self.book.cell(sheet, a1)
            value = self.vs.a.get((sheet, a1))
            if item is not None and not item.labels and isinstance(value, (int, float)) and not isinstance(value, bool):
                window.bind(a1, lambda s=sheet, a=a1: self._show(s, a))
        return window

    def _window(self, sheet, rows, selected, cols, left=LEFT):
        """🪟 The workbook window over one sheet with live numeric cells; the drawing column to its right follows its width."""
        width = COLS[cols]
        self.dx = (left + width + 0.3 + 7.0) / 2
        window = self.xl_window(self.book, sheet, at=(left + width / 2, -0.05, 0.0), width=width, height=WIN_H, cols=cols.rstrip("+"), rows=rows, selected=selected, values=self.vs.a)
        return self._live(window, sheet)

    def _swap(self, new):
        """🔁 Replace the current window and drop its frames."""
        old, self.win = self.win, new
        gone = [old, *self.frames]
        self.frames = []
        return [_engine.FadeOut(m) for m in gone] + [_engine.FadeIn(new)]

    def _begin(self, vals):
        """🎚️ Aim the live values at a new workbook state and return the animation that moves them there."""
        self.vs.b = vals
        return _engine.UpdateFromAlphaFunc(self.vs.t, lambda mob, alpha: mob.set_value(alpha), rate_func=_engine.smooth, run_time=3.0)

    def _end(self, vals):
        """✅ Make the aimed state the current one after its clause."""
        self.vs.a = vals
        self.vs.t.set_value(0.0)

    def _frame(self, a1, color, dashed=False):
        """🔳 A reference frame on the current window that is dropped with it."""
        frame = self.xl_ref_frame(self.win, a1, color=color, dashed=dashed)
        self.frames.append(frame)
        return frame

    def _recalc(self, book, vals):
        """♻️ Point the window at a new workbook state so the formula bar follows; the animations of unbound cells, if any."""
        return [self.win.recalculate(book, vals)]

    def _nudge(self):
        """⏱️ A short wait so live drawings refresh after a state change."""
        return _engine.Wait(0.05)

    def _heat(self, value):
        """🌡️ Surface colour of a U-value from green (tight) to red (leaky)."""
        peak = max(self.vs.a[(self.T, f"E{row}")] for row in ROWS)
        return _draft.mix(GOOD, SERV, min(value / peak, 1.0) ** 0.7)

    def _cap_state(self, **kwargs):
        """💊 The state of one drawn capsule: view, tint, hatch, dimension lines and heat-flow arrows."""
        state = SimpleNamespace(view="axon", tint="none", hatch=(), dims=(), bolts=False, rows=tuple(ROWS), mode="ua", faint=False, vent=_engine.ValueTracker(0.0), vol=_engine.ValueTracker(0.0),
                                tint_a=_engine.ValueTracker(0.0), grow=_engine.ValueTracker(0.0), kind="A")
        state.__dict__.update(kwargs)
        return state

    def _capsule(self, at, scale, state):
        state.at, state.scale = at, scale
        """💊 A live capsule drawing of type A with tint, hatch, dimensions and heat-flow arrows read from the workbook state."""
        key = lambda: (round(state.tint_a.get_value(), 3), round(state.grow.get_value(), 3), round(self.vs.t.get_value(), 4), id(self.vs.a), id(self.vs.b), state.view, state.tint,
                       state.hatch, state.dims, state.bolts, state.rows, state.mode, round(state.vent.get_value(), 3), round(state.vol.get_value(), 3), tuple(state.at), state.scale)
        return self.live(lambda: self._build_capsule(state.at, state.scale, state), key)

    def _build_capsule(self, at, scale, state):
        """🛠️ Build one frame of a live capsule."""
        alpha = state.tint_a.get_value()
        colors = {}
        if state.tint == "face" and alpha > 0.02:
            colors = {ROWS[r]: ROW_COLOR[r] for r in ROWS if ROWS[r] != "window"}
        elif state.tint == "heat" and alpha > 0.02:
            colors = {ROWS[r]: self._heat(self._num(self.T, f"E{r}")) for r in ROWS if ROWS[r] != "window"}
        elif state.tint == "exposed" and alpha > 0.02:
            colors = {ROWS[r]: ROW_COLOR[r] for r in ROWS if ROWS[r] not in ("window", "back")}
        colors = {face: _draft.mix(_drawing.CAPSULE, color, alpha) for face, color in colors.items()}
        drawing = self.capsule_axon(self.cap, at=tuple(at), scale=scale, view=state.view, bolts=state.bolts, dims=state.dims, highlight=colors, hatch=state.hatch)
        behind, front = _engine.VGroup(), _engine.VGroup()
        ring = None
        if alpha > 0.02 and state.tint in ("face", "heat") and state.view == "axon":
            color = ROW_COLOR[8] if state.tint == "face" else self._heat(self._num(self.T, "E8"))
            ring = _engine.Ellipse(width=drawing.window.width + 0.05, height=drawing.window.height + 0.05, stroke_color=color, stroke_width=3.5,
                                   fill_opacity=0).move_to(drawing.window.get_center())
            ring.set_stroke(opacity=alpha)
        grow = state.grow.get_value()
        if grow > 0.01:
            for row in state.rows:
                face = ROWS[row]
                (behind if face in HIDDEN else front).add(self._arrow(drawing, row, grow, state))
        group = _engine.VGroup(behind, drawing)
        if ring is not None:
            group.add(ring)
        if state.vol.get_value() > 0.01:
            group.add(self._volume_box(drawing, state.vol.get_value()))
        group.add(front)
        if grow > 0.6 and state.mode == "ua" and state.scale > 0.4:
            group.add(self._scale_key(at, scale))
        if state.vent.get_value() > 0.01:
            group.add(self._arrow(drawing, 99, state.vent.get_value(), state))
        return group

    def _arrow(self, drawing, row, grow, state):
        """➡️ One heat-flow arrow out of a surface (row 99: the air): length and width are proportional to its U·A (H_V for the air); hidden faces start outside the body on a dashed lead."""
        air = row == 99
        face = "air" if air else ROWS[row]
        ua = self._num(self.T, "B12" if air else f"F{row}")
        value = ua if state.mode == "ua" else ua * (self._num(self.CL, "B2") - self._num(self.CL, "B3"))
        unit = "W/K" if state.mode == "ua" else "W"
        color = BLUE if air else SERV
        point, normal = np.array(POINT[face], dtype=float), np.array(NORMAL[face], dtype=float)
        start0 = drawing.at(*point)
        step = drawing.at(*(point + normal)) - start0
        direction = step / max(np.linalg.norm(step), 1e-6)
        start = start0 + direction * HIDDEN.get(face, 0.0)
        end = start + direction * ARROW_PER_UA * ua * grow
        group = _engine.VGroup()
        if face in HIDDEN:
            group.add(_draft.polyline([start0, start], color, _draft.W_THIN, 0.7, dash=0.06))
        group.add(_engine.Arrow(start, end, buff=0, color=color, stroke_width=ARROW_WIDTH * ua, max_tip_length_to_length_ratio=0.5, max_stroke_width_to_length_ratio=40))
        if grow > 0.6:
            group.add(self._say((self.tr("air ", "Luft ") if air else "") + self.quantity(value, unit, 2 if unit == "W/K" else 0), end + direction * 0.14, size=SMALL, color=INK, anchor=self._side(direction)))
        group.add(_engine.Dot(start0, radius=0.05, color=color))
        return group

    def _scale_key(self, at, scale):
        """🔑 The arrow scale: one arrow of 1 W/K."""
        base = np.array([at[0] + 1.5, at[1] - 2.0, 0.0])
        return _engine.VGroup(_engine.Arrow(base, base + np.array([ARROW_PER_UA, 0.0, 0.0]), buff=0, color=SERV, stroke_width=ARROW_WIDTH, max_tip_length_to_length_ratio=0.5),
                              self._say("= 1 W/K", base + np.array([ARROW_PER_UA + 0.15, 0.0, 0.0]), size=SMALL, color=MUTED, anchor="l"))

    def _volume_box(self, drawing, amount):
        """🧊 The interior air volume as a translucent box inside the capsule: right wall, front wall and ceiling."""
        t = self.cap
        x, y0, y1, z0, z1 = t.interior_width / 2, (t.length - t.interior_length) / 2, (t.length + t.interior_length) / 2, (t.height - t.interior_height) / 2, (t.height + t.interior_height) / 2
        faces = [[(x, y0, z0), (x, y1, z0), (x, y1, z1), (x, y0, z1)], [(-x, y1, z0), (x, y1, z0), (x, y1, z1), (-x, y1, z1)], [(-x, y0, z1), (x, y0, z1), (x, y1, z1), (-x, y1, z1)]]
        return _engine.VGroup(*[_draft.polygon([drawing.at(*p) for p in face], BLUE, 0.22 * amount, BLUE, _draft.W_THIN, 0.9 * amount) for face in faces])

    def _side(self, direction):
        """🧭 Label anchor that keeps a label clear of the arrow tip."""
        horizontal = "r" if direction[0] < -0.3 else "l" if direction[0] > 0.3 else ""
        vertical = "d" if direction[1] < -0.3 else "u" if direction[1] > 0.3 else ""
        return (vertical + horizontal) or "c"


    def _chip(self, kind, point, anchor="l", text=None):
        """🔖 Outlined basis chip in the colour of its basis."""
        color, en, de = BASIS[kind]
        return self.badge(text or self.tr(en, de), point, color=color, size=SMALL, filled=False, anchor=anchor)

    def _readout(self, fn, point, *, size=LABEL, color=DATA, anchor="c"):
        """📟 A live label, re-typeset only when its string changes."""
        return self.live(lambda: _engine.VGroup(self._say(fn(), point, size=size, color=color, anchor=anchor)), fn)

    def _closure(self, start):
        """🕸️ Every cell that depends on a cell, followed through all sheets."""
        seen, queue = [], [start]
        while queue:
            for cell in _xl.dependents(self.book, queue.pop(0)):
                if cell not in seen:
                    seen.append(cell)
                    queue.append(cell)
        return seen

    def _chain(self, at, rows):
        """⛓️ A framed list of dependent cells: the label on one line and the live value under it."""
        pitch, width = 0.56, 3.7
        height = pitch * len(rows) + 0.2
        frame = _engine.RoundedRectangle(corner_radius=0.08, width=width, height=height, stroke_color=MUTED, stroke_width=_draft.W_THIN, fill_color=BACKGROUND, fill_opacity=0.9)
        frame.move_to(np.array([at[0], at[1], 0.0]))
        panel = _engine.VGroup(frame)
        top = at[1] + height / 2 - 0.1 - pitch / 2
        for i, (sheet, a1, label) in enumerate(rows):
            y = top - i * pitch
            panel.add(self._say(label, (at[0] - width / 2 + 0.15, y + 0.12), size=SMALL, color=INK, anchor="l", width=3.4),
                      self._readout(lambda s=sheet, a=a1: self._show(s, a), (at[0] + width / 2 - 0.15, y - 0.13), size=SMALL, color=DATA, anchor="r"))
        return panel

    def _prepare(self):
        """🧮 Load the workbook, the model and every alternative workbook state of this chapter from the engine; what-if values are the model's named variants."""
        self.book = _xl.workbook(language=self.language())
        order = self.book.order()
        self.PR, self.T, self.CL, self.EN = order[0], order[1], order[-2], order[-1]
        self.cap = _nakagin.capsule_type("A")
        base = _xl.evaluate(self.book)
        self.vs = SimpleNamespace(a=base, b=base, t=_engine.ValueTracker(0.0))
        self.vals_base = base
        self.books = {}
        for name in ("window_upgrade", "back_wall_exposed", "shared_roof_floor"):
            events = []
            for key, override in _nakagin.variant(name)["overrides"].items():
                sheet, cell = self.book.names[key].replace("$", "").split("!")
                events.append({"type": "set-cell", "sheet": sheet, "cell": cell, "value": override.value})
            book = _xl.apply(self.book, events)
            self.books[name] = (book, _xl.evaluate(book))
        better_book, better_vals = self.books["window_upgrade"]
        frozen = _xl.apply(better_book, [{"type": "set-cell", "sheet": self.EN, "cell": "D8", "value": base[(self.EN, "D8")]}])
        self.books["frozen"] = (frozen, _xl.evaluate(frozen))
        self.vals_better = self.books["window_upgrade"][1]
        self.vals_stacked = self.books["shared_roof_floor"][1]
        self.win, self.frames = None, []

    def _title(self):
        """📑 Subtitle chip: the example building and the sheet in the title."""
        return _fonts.section_heading(self.tr("Nakagin Capsule Tower · Energy", "Nakagin Capsule Tower · Energie")).to_edge(_engine.UP, buff=0.3).to_edge(_engine.LEFT, buff=0.4)

    def _note(self):
        """📖 The one-time teaching-model note at the top right."""
        note = self._say(self.tr("Teaching model · simplified, rounded", "Lehrmodell · vereinfacht, gerundet"), (0, 0), size=SMALL, color=MUTED)
        return note.move_to(np.array([6.85 - note.width / 2, 3.62 - note.height / 2 + 0.06, 0.0]))

    def _slot_types(self):
        """🧱 The type of every capsule slot of the tower, keyed by drawing key."""
        return {slot.path + (slot.item,): slot.type for slot in _nakagin.slots()}

    def present_energy(self):
        """🎞️ The whole animation: from the capsule dimensions to the building total and a live what-if."""
        self._prepare()
        T, CL, EN = self.T, self.CL, self.EN
        kinds = self._slot_types()
        ramp = lambda tracker, start, stop, seconds=2.5: _engine.UpdateFromAlphaFunc(tracker, lambda mob, a: mob.set_value(start + (stop - start) * a), run_time=seconds)

        tower = self.tower_axon(at=(0.0, 0.1, 0.0), scale=0.1, lit=lambda key: kinds.get(key) == "A")
        mark = self.badge("?", (1.7, 1.4), color=DATA, size=26)
        outside = _engine.VGroup(self._say("θe = " + self.xl_display(self.vals_base[(CL, "B3")], '0 "°C"'), (3.4, 0.6), size=LABEL, color=BLUE),
                                 self._say("θi = " + self.xl_display(self.vals_base[(CL, "B2")], '0 "°C"'), (3.4, 0.15), size=LABEL, color=SERV))
        note = self._note()
        self.step("hook", _engine.FadeIn(tower), _engine.FadeIn(mark), _engine.FadeIn(outside), _engine.FadeIn(note), seconds=2.5)

        self.win = self._window(T, (1, 10), "B1", "A:B")
        cs = self.cs = self._cap_state()
        capsule = self._capsule((self.dx, 0.1, 0.0), 0.6, cs)
        title = self._title()
        self.step("sheet", _engine.FadeOut(tower), _engine.FadeOut(mark), _engine.FadeOut(outside), _engine.FadeIn(self.win), _engine.FadeIn(capsule), _engine.FadeIn(title), seconds=2.5)

        cs.dims = ("width", "length", "height")
        self.step("dims", self.win.select("B2:B4"), _engine.FadeIn(self._frame("B2:B4", DATA)), self._nudge(), seconds=2.5)

        cs.dims, cs.tint = (), "face"
        new = self._window(T, (2, 11), "D3:D9", "C:F")
        cs.at, cs.scale = (self.dx - 0.5, 0.1, 0.0), 0.46
        swap = self._swap(new)
        frames = [self._frame(f"D{r}", ROW_COLOR[r]) for r in ROWS]
        self.step("areas", *swap, _engine.LaggedStart(*[_engine.FadeIn(f) for f in frames], lag_ratio=0.2), ramp(cs.tint_a, 0, 1), seconds=3.0)

        cs.view, cs.bolts, cs.hatch, cs.tint = "axon_back", True, ("attached",), "exposed"
        b_readout = self._readout(lambda: "b = " + self.xl_display(self._num(CL, "B6"), "0.00"), (self.dx - 0.6, 2.2), color=MUTED)
        b_chip = self._chip("assumed", (self.dx + 0.2, 2.2))
        self.step("exposed", self.win.select("D9:F9"), _engine.FadeIn(self._frame("F9", MUTED)), _engine.FadeIn(b_readout), _engine.FadeIn(b_chip), self._nudge(), seconds=2.5)

        cs.view, cs.bolts, cs.hatch, cs.tint = "axon", False, (), "heat"
        cs.tint_a.set_value(0.0)
        basis = {"window": _nakagin.value("energy.u_window").basis, "wall": _nakagin.value("energy.u_wall").basis}
        legend = self._say(self.tr("tint: low U green, high U red", "Tönung: kleines U grün, großes U rot"), (0, 0), size=SMALL, color=MUTED)
        chips = _engine.VGroup(self._chip(basis["window"], (0, 0), text=self.tr("window: benchmark", "Fenster: Richtwert")),
                               self._chip(basis["wall"], (0, 0), text=self.tr("walls, roof, floor ", "Wände, Dach, Boden ") + self.tr(*BASIS[basis["wall"]][1:])), legend)
        chips.arrange(_engine.DOWN, buff=0.15, aligned_edge=_engine.LEFT).move_to(np.array([self.dx, -2.0, 0.0]))
        self.step("uvalues", self.win.select("E3:E9"), _engine.FadeIn(self._frame("E3:E9", DATA)), ramp(cs.tint_a, 0, 1), _engine.FadeOut(b_readout), _engine.FadeOut(b_chip), _engine.FadeIn(chips),
                  seconds=2.5)

        self.step("ua", self.win.select("F3:F9"), _engine.FadeIn(self._frame("F3:F9", SERV)), ramp(cs.tint_a, 1, 0, 2.0), ramp(cs.grow, 0, 1, 3.0), _engine.FadeOut(chips), seconds=3.0)

        cs.tint = "none"
        card = self._sumproduct()
        self.step("ht", self.win.select("F3:F9"), _engine.FadeIn(card), self._nudge(), seconds=3.0)

        new = self._window(CL, (1, 9), "B6", "A:B+")
        names = self._names((self.dx, 0.2))
        self.step("climate", *self._swap(new), _engine.FadeOut(card), _engine.FadeOut(capsule), _engine.FadeIn(names), seconds=3.0)

        hdd = self._hdd_chart((self.dx, 0.2))
        self.step("degreedays", self.win.select("B5"), _engine.FadeOut(names), _engine.FadeIn(hdd), seconds=3.0)

        cs.at, cs.scale = (self.dx - 0.3, 0.1, 0.0), 0.46
        delta = self._readout(lambda: "ΔH_T = " + self.xl_display(self._num(T, "B11") - self.vals_base[(T, "B11")], '+0.00 "W/K";-0.00 "W/K"'), (self.dx + 1.0, 2.2), color=SERV)
        formula_card = _engine.VGroup(self._say(self.tr("Type A · F9", "Typ A · F9"), (self.dx - 2.3, 3.15), size=SMALL, color=MUTED, anchor="l"),
                                      self.xl_formula_text("=D9*E9*attached_factor", at=(self.dx - 2.3, 2.82, 0), size=SMALL, ink=INK),
                                      self._readout(lambda: "= " + self._show(T, "F9"), (self.dx + 0.8, 2.82), size=SMALL, color=SERV, anchor="l"))
        moving = self._begin(self.books["back_wall_exposed"][1])
        self.step("mistake", moving, self.win.select("B6"), *self._recalc(*self.books["back_wall_exposed"]), _engine.FadeIn(self._frame("B6", SERV)), _engine.FadeOut(hdd), _engine.FadeIn(capsule), _engine.FadeIn(delta),
                  _engine.FadeIn(formula_card), seconds=3.0)
        self._end(self.books["back_wall_exposed"][1])

        moving = self._begin(self.vals_base)
        self.step("fix", moving, self.win.select("B6"), *self._recalc(self.book, self.vals_base), _engine.FadeOut(formula_card), seconds=3.0)
        self._end(self.vals_base)

        new = self._window(T, (8, 16), "B12", "A:B")
        cs.at, cs.scale = (self.dx, 0.1, 0.0), 0.6
        new.edit("=air_heat_capacity*air_change*B10")
        vol_label = self._readout(lambda: "V = " + self._show(T, "B10") + " · n = " + self._show(CL, "B7"), (self.dx, 2.5), size=LABEL, color=BLUE)
        self.step("volume", *self._swap(new), _engine.FadeOut(delta), ramp(cs.vent, 0, 1), ramp(cs.vol, 0, 1), _engine.FadeIn(vol_label), seconds=3.0)

        self.win.edit(None)
        bar = self._stack((self.dx, -1.6))
        self.step("total", self.win.select("B11:B12"), _engine.FadeIn(self._frame("B11", SERV)), _engine.FadeIn(self._frame("B12", BLUE)), _engine.FadeIn(bar), ramp(cs.grow, 1, 0, 1.5),
                  ramp(cs.vent, 1, 0, 1.5), ramp(cs.vol, 1, 0, 1.5), _engine.FadeOut(vol_label), seconds=2.5)

        new = self._window(T, (8, 16), "B13", "A:B")
        cs.at, cs.scale, cs.mode, cs.rows = (self.dx, 0.1, 0.0), 0.6, "phi", tuple(ROWS)
        new.edit("=(B11+B12)*(theta_i-theta_e)")
        temps = _engine.VGroup(self._readout(lambda: "θi = " + self._show(CL, "B2"), (self.dx - 1.7, 2.2), color=SERV), self._readout(lambda: "θe = " + self._show(CL, "B3"), (self.dx + 1.7, 2.2), color=BLUE))
        self.step("load", *self._swap(new), _engine.FadeOut(bar), _engine.FadeIn(temps), ramp(cs.grow, 0, 1, 2.0), ramp(cs.vent, 0, 1, 2.0), seconds=3.0)

        new = self._window(T, (8, 16), "B14:B15", "A:B")
        new.edit("=(B11+B12)*heating_degree_days*24/1000")
        units = self._annual_tiles((self.dx, 0.5))
        plan = self._plan((self.dx + 2.2, 2.0))
        self.step("annual", *self._swap(new), ramp(cs.grow, 1, 0, 2.0), ramp(cs.vent, 1, 0, 2.0), _engine.FadeOut(temps), _engine.FadeOut(capsule), _engine.FadeIn(units), _engine.FadeIn(plan), seconds=3.0)

        new = self._window(EN, (3, 12), "B4:D7", "A:D")
        tower = self.tower_axon(at=(self.dx, -0.35, 0.0), scale=0.08, tint=lambda key: _drawing.type_color(kinds[key]), lit=None)
        counts = self._counts((self.dx, 2.6))
        self.step("energy", *self._swap(new), _engine.FadeOut(units), _engine.FadeOut(plan), _engine.FadeIn(tower), _engine.FadeIn(counts), seconds=3.0)

        new = self._window(EN, (3, 12), "F4:F7", "D:F", left=-5.6)
        badges = _engine.VGroup(*[self.badge(key, (-6.35, (new.rect(f"F{4 + i}")[1] + new.rect(f"F{4 + i}")[3]) / 2), color=_drawing.type_color(key), size=SMALL) for i, key in enumerate("ABCD")])
        self.frames.append(badges)
        rows = [(key, (lambda i=i: self._num(EN, f"F{4 + i}")), _drawing.type_color(key)) for i, key in enumerate("ABCD")]
        types_row = self._types_row((self.dx, 2.1))
        chart = self.xl_bar_chart(rows, at=(self.dx, -0.6, 0.0), width=3.7, height=2.9, unit="", title=self.tr("Demand, kWh/(m²·a)", "Bedarf, kWh/(m²·a)"), decimals=0)
        self.step("compare", *self._swap(new), _engine.FadeIn(badges), _engine.FadeOut(tower), _engine.FadeOut(counts), _engine.FadeIn(chart), _engine.FadeIn(types_row), seconds=3.0)

        shape = self._shape((self.dx, 0.0))
        self.step("shape", _engine.FadeOut(chart), _engine.FadeOut(types_row), _engine.FadeIn(shape), self._nudge(), seconds=3.5)

        new = self._window(EN, (3, 14), "B14", "A:D")
        small = self._tower_counter((self.dx, 1.8), kinds)
        chain = self._chain((self.dx, -1.15), [(EN, "B14", self.tr("Window U-value", "Fenster U-Wert")), (T, "B11", self.tr("Type A · H_T", "Typ A · H_T")),
                                              (T, "B14", self.tr("Type A · demand", "Typ A · Bedarf")), (EN, "D8", self.tr("Building total", "Gebäudesumme"))])
        count = len(self._closure((EN, "B14")))
        counted = self._say(self.tr(f"{count} dependent cells", f"{count} abhängige Zellen"), (self.dx, 0.72), size=SMALL, color=MUTED, width=3.4)
        left, top, right, bottom = new.rect("D4")
        low = new.rect("D8")
        link = _engine.CurvedArrow(np.array([right + 0.02, (top + bottom) / 2, 0.0]), np.array([right + 0.02, (low[1] + low[3]) / 2, 0.0]), angle=-1.2, color="#2F6FDE", stroke_width=3.0)
        arrows = _engine.VGroup(*[self.xl_ref_frame(new, a1, color="#2F6FDE") for a1 in ("C4", "D4", "D8")], link)
        self.step("trace", *self._swap(new), _engine.FadeOut(shape), _engine.FadeIn(small), _engine.FadeIn(chain), _engine.FadeIn(counted), _engine.FadeIn(arrows), seconds=3.5)
        self.frames.append(arrows)

        better_book, better_vals = self.books["window_upgrade"]
        moving = self._begin(better_vals)
        self.step("whatif", moving, self.win.select("B14"), *self._recalc(better_book, better_vals), seconds=4.0)
        self._end(better_vals)

        frozen_book, frozen_vals = self.books["frozen"]
        rows_sum = self._readout(lambda: self.tr("rows add up to ", "Zeilen ergeben ") + self.xl_display(sum(better_vals[(EN, f"D{r}")] for r in range(4, 8)), '#,##0 "kWh/a"'),
                                 (self.dx, 0.38), size=SMALL, color=SERV)
        stale = self._frame("D8", SERV, dashed=True)
        moving = self._begin(frozen_vals)
        self.step("frozen", moving, self.win.select("D8"), *self._recalc(frozen_book, frozen_vals), _engine.FadeIn(stale), _engine.FadeIn(rows_sum), seconds=3.0)
        self._end(frozen_vals)

        moving = self._begin(better_vals)
        self.step("repaired", moving, self.win.select("D8"), *self._recalc(better_book, better_vals), _engine.FadeOut(stale), _engine.FadeOut(rows_sum), seconds=3.0)
        self._end(better_vals)

        self._recap(self.win, small, chain, counted)

    def _hdd_chart(self, at):
        """📈 Heating degree days as an area: twelve monthly Tokyo means against the heating limit, the shortfall hatched, summing to the engine value."""
        lang = self.language()
        months = _nakagin.climate()["months"]
        limit = self.vals_base[(self.CL, "B4")]
        w, h, ymax = 4.9, 2.0, 30.0
        x0, y0 = at[0] - w / 2, at[1] - h / 2
        px = lambda i: x0 + (i + 0.5) * w / len(months)
        py = lambda t: y0 + h * t / ymax
        bar = w / len(months) * 0.62
        group = _engine.VGroup(_draft.segments([((x0, y0, 0), (x0 + w, y0, 0))], MUTED, _draft.W_EDGE))
        for i, month in enumerate(months):
            mean = float(month["mean"])
            name = month["name"][lang]
            group.add(_draft.polygon([(px(i) - bar / 2, y0), (px(i) + bar / 2, y0), (px(i) + bar / 2, py(mean)), (px(i) - bar / 2, py(mean))], BLUE, 0.75))
            if mean < limit:
                ring = [(px(i) - bar / 2, py(mean)), (px(i) + bar / 2, py(mean)), (px(i) + bar / 2, py(limit)), (px(i) - bar / 2, py(limit))]
                group.add(_draft.polygon(ring, SERV, 0.18), _draft.hatch([ring], SERV, 0.07, 45, _draft.W_HAIR, 0.95))
            group.add(self._say(name[0], (px(i), y0 - 0.2), size=SMALL, color=MUTED))
        group.add(_draft.polyline([(x0 - 0.1, py(limit)), (x0 + w + 0.1, py(limit))], DATA, _draft.W_EDGE, 1.0, dash=0.1),
                  self._say(self.quantity(limit, "°C", 0), (x0 - 0.15, py(limit)), size=SMALL, color=DATA, anchor="r"),
                  self._readout(lambda: "Σ = " + self._show(self.CL, "B5"), (at[0], y0 + h + 0.4), color=SERV),
                  self._say(self.tr("monthly mean, Tokyo", "Monatsmittel, Tokio"), (at[0], y0 - 0.6), size=SMALL, color=MUTED))
        return group

    def _annual_tiles(self, at):
        """🧱 Annual demand as a product of tiles: loss per kelvin times degree days times 24 h per day, divided by 1000; then the demand divided by the floor area."""
        T, CL = self.T, self.CL

        def tile(x, y, label, fn, color):
            box = _engine.RoundedRectangle(corner_radius=0.08, width=1.7, height=0.8, stroke_color=color, stroke_width=_draft.W_THIN, fill_color=color, fill_opacity=0.14).move_to((x, y, 0))
            return _engine.VGroup(box, self._say(label, (x, y + 0.2), size=SMALL, color=MUTED, width=1.6), self._readout(fn, (x, y - 0.13), size=SMALL, color=INK))

        xs = [at[0] - 2.8, at[0] - 0.93, at[0] + 0.93, at[0] + 2.8]
        y = at[1]
        loss = lambda: self.xl_display(self._num(T, "B11") + self._num(T, "B12"), '0.0 "W/K"')
        group = _engine.VGroup(tile(xs[0], y, "H", loss, SERV), tile(xs[1], y, self.tr("degree days", "Gradtage"), lambda: self._show(CL, "B5"), BLUE),
                               tile(xs[2], y, self.tr("hours per day", "Stunden je Tag"), lambda: "24 h/d", MUTED), tile(xs[3], y, "Q = … ÷ 1000", lambda: self._show(T, "B14"), DATA))
        for x, sign in ((at[0] - 1.865, "×"), (at[0], "×"), (at[0] + 1.865, "=")):
            group.add(self._say(sign, (x, y), size=LABEL, color=INK))
        group.add(tile(xs[0], y - 1.2, "Q", lambda: self._show(T, "B14"), DATA), tile(xs[1], y - 1.2, self.tr("floor area", "Grundfläche"), lambda: self._show(T, "B9"), GOOD),
                  tile(xs[2], y - 1.2, "Q/A", lambda: self._show(T, "B15"), DATA), self._say("÷", (at[0] - 1.865, y - 1.2), size=LABEL, color=INK), self._say("=", (at[0], y - 1.2), size=LABEL, color=INK))
        return group

    def _types_row(self, at):
        """🧩 The four capsule types side by side: A, B and D share one shell and window, C has the larger window."""
        group = _engine.VGroup()
        for i, t in enumerate(_nakagin.capsule_types()):
            x = at[0] - 1.9 + i * 1.25
            group.add(self.capsule_axon(t, at=(x, at[1], 0.0), scale=0.14, window=True),
                      self.badge(t.key, (x - 0.3, at[1] - 0.52), color=_drawing.type_color(t.key), size=SMALL),
                      self._say("Ø " + self.quantity(t.window_diameter, "m", 1), (x + 0.3, at[1] - 0.52), size=SMALL, color=WARN if t.window_diameter > 1.4 else INK, width=0.75))
        group.add(self._say(self.tr("window Ø", "Fenster Ø"), (at[0], at[1] + 0.55), size=SMALL, color=MUTED),
                  self._say(self.tr("same shell, same demand · only C has the larger window", "gleiche Hülle, gleicher Bedarf · nur C hat das größere Fenster"), (at[0], at[1] - 0.95), size=SMALL, color=MUTED, width=3.7))
        return group

    def _tower_counter(self, at, kinds):
        """🏙️ The building as a small tinted tower with the live building total and its change against the start."""
        EN = self.EN
        base = self.vals_base[(EN, "D8")]
        group = _engine.VGroup(self.tower_axon(at=(at[0] - 1.1, at[1], 0.0), scale=0.045, tint=lambda key: _drawing.type_color(kinds[key]), lit=None))
        group.add(self._readout(lambda: self._show(EN, "D8"), (at[0] + 0.9, at[1] + 0.3), color=DATA),
                  self._readout(lambda: self.xl_display(self._num(EN, "D8") / base - 1.0, '+0.0%;-0.0%;0.0%'), (at[0] + 0.9, at[1] - 0.1), size=SMALL, color=GOOD))
        return group

    def _sumproduct(self):
        """∑ H_T as the SUM of the U·A column and as one SUMPRODUCT of areas, U-values and temperature factors over all surfaces, both evaluated by the engine."""
        T = self.T
        cells = {f"{c}{r}": self.vals_base[(T, f"{c}{r}")] for r in range(3, 10) for c in "DE"}
        factors = {f"G{r}": (self.vals_base[(self.CL, "B6")] if r == 9 else 1.0) for r in range(3, 10)}
        total = _xl.calculate("=SUMPRODUCT(D3:D9,E3:E9,G3:G9)", {**cells, **factors})
        x = self.dx - 3.2
        group = _engine.VGroup()
        for y, formula, value in ((3.05, "=SUM(F3:F9)", self.vals_base[(T, "B11")]), (2.7, "=SUMPRODUCT(D3:D9,E3:E9,G3:G9)", total)):
            text = self.xl_formula_text(formula, at=(x, y, 0), size=SMALL, ink=INK)
            group.add(text, self._say(" = " + self.xl_display(value, '0.00 "W/K"'), (text.get_right()[0] + 0.05, y), size=SMALL, color=DATA, anchor="l"))
        b = self.xl_display(self.vals_base[(self.CL, "B6")], "0.00")
        note = self.tr(f"G: temperature factor, 1 for exposed surfaces; back wall with b = {b}: ", f"G: Temperaturfaktor, 1 für freie Flächen; Rückwand mit b = {b}: ")
        group.add(self._say(note + self.xl_display(self.vals_base[(T, "F9")], '0.00 "W/K"'), (x, 2.35), size=SMALL, color=MUTED, anchor="l", width=6.0))
        return group

    def _stack(self, at):
        """📊 H as a stacked bar of H_T and H_V, widths from the live cells."""
        T = self.T

        def build():
            a, b = self._num(T, "B11"), self._num(T, "B12")
            total = a + b
            width = 4.2
            x0 = at[0] - width / 2
            wa = width * a / total
            bar = _engine.VGroup(_draft.polygon([(x0, at[1] - 0.18), (x0 + wa, at[1] - 0.18), (x0 + wa, at[1] + 0.18), (x0, at[1] + 0.18)], SERV, 0.9),
                                 _draft.polygon([(x0 + wa, at[1] - 0.18), (x0 + width, at[1] - 0.18), (x0 + width, at[1] + 0.18), (x0 + wa, at[1] + 0.18)], BLUE, 0.9))
            bar.add(self._say("H_T " + self.xl_display(a, '0.00 "W/K"'), (x0, at[1] + 0.45), size=SMALL, color=SERV, anchor="r" if False else "l"),
                    self._say("H_V " + self.xl_display(b, '0.00 "W/K"'), (x0 + width, at[1] - 0.5), size=SMALL, color=BLUE, anchor="r"))
            return bar
        return self.live(build, lambda: round(self.vs.t.get_value(), 3))

    def _names(self, at):
        """🏷️ The defined names of the Climate sheet with their live values, and the climate source and period."""
        CL = self.CL
        rows = [("theta_i", "B2"), ("theta_e", "B3"), ("heating_degree_days", "B5"), ("attached_factor", "B6"), ("air_change", "B7"), ("air_heat_capacity", "B8")]
        group = _engine.VGroup()
        for i, (name, a1) in enumerate(rows):
            y = at[1] + 1.75 - i * 0.5
            group.add(self.mono(name, (at[0] - 2.6, y), size=SMALL, color=DATA, anchor="l"), self._readout(lambda a=a1: self._show(CL, a), (at[0] + 2.6, y), size=SMALL, color=INK, anchor="r"))
        climate = _nakagin.climate()
        lang = self.language()
        group.add(self._say(climate["period"][lang], (at[0], at[1] - 1.45), size=SMALL, color=MUTED, width=5.4), self._say(climate["method"][lang], (at[0], at[1] - 1.8), size=SMALL, color=MUTED, width=5.4),
                  self._chip("assumed", (at[0] - 2.6, at[1] - 2.25)))
        return group

    def _plan(self, at):
        """📐 A small plan of the capsule with its floor hatched."""
        group = _engine.VGroup(self.capsule_axon(self.cap, at=tuple(at), scale=0.28, view="plan", hatch=("top",), window=False))
        group.add(self._readout(lambda: self._show(self.T, "B9"), (at[0], at[1] - 0.8), size=SMALL, color=GOOD))
        return group

    def _counts(self, at):
        """🔢 Capsule count per type from the Program sheet, as live chips in the type colours, two by two."""
        group = _engine.VGroup()
        for i, key in enumerate("ABCD"):
            x, y = at[0] - 1.0 + (i % 2) * 2.0, at[1] - (i // 2) * 0.5
            group.add(self.badge(key, (x - 0.35, y), color=_drawing.type_color(key), size=SMALL), self._readout(lambda i=i: self._show(self.EN, f"B{4 + i}"), (x + 0.1, y), size=SMALL, color=INK, anchor="l"))
        return group

    def _shape(self, at):
        """🧊 Two capsules: five exposed faces, and heated neighbours above and below; shell per floor area and demand from the engine."""
        T, EN = self.T, self.EN
        base, alt = self.vals_base, self.vals_stacked
        shell = lambda vals, rows: sum(vals[(T, f"D{r}")] for r in rows)
        floor = base[(T, "B9")]
        group = _engine.VGroup()
        for x, rows, hatch, vals in ((at[0] - 0.95, (3, 4, 5, 6, 7, 8), (), base), (at[0] + 0.95, (5, 6, 7, 8), ("top", "bottom"), alt)):
            state = self._cap_state(tint="exposed" if not hatch else "face", tint_a=_engine.ValueTracker(1.0), hatch=hatch)
            state.tint_a.set_value(1.0)
            group.add(self._capsule((x, at[1] + 0.9, 0.0), 0.32, state))
            ratio = shell(base, rows) / floor
            group.add(self._say(self.quantity(ratio, "m²/m²", 1), (x, at[1] - 0.25), size=SMALL, color=SERV, width=1.8))
            group.add(self._say(self.quantity(vals[(EN, "F4")], "kWh/(m²·a)", 0), (x, at[1] - 0.75), size=SMALL, color=DATA, width=1.8))
        group.add(self._say(self.tr("exposed shell per floor area", "Außenhülle je Grundfläche"), (at[0], at[1] - 1.25), size=SMALL, color=MUTED, width=4.4),
                  self._say(self.tr("5 faces + window", "5 Flächen + Fenster"), (at[0] - 0.95, at[1] - 1.7), size=SMALL, color=INK, width=1.65),
                  self._say(self.tr("roof, floor shared", "Dach, Boden geteilt"), (at[0] + 0.95, at[1] - 1.7), size=SMALL, color=INK, width=1.65))
        return group

    def _recap(self, window, small, chain, counted):
        """🔁 The closing legend of terms in the language of the clip."""
        terms = [("U", self.tr("U-value", "U-Wert"), "W/(m²·K)"), ("U·A", self.tr("heat loss of one surface", "Wärmeverlust einer Fläche"), "W/K"),
                 ("b", self.tr("attached factor of the back wall", "Anbindungsfaktor der Rückwand"), "1"), ("H_T", self.tr("transmission loss", "Transmissionswärmeverlust"), "W/K"),
                 ("n", self.tr("air change rate", "Luftwechselrate"), "1/h"), ("H_V", self.tr("ventilation loss", "Lüftungswärmeverlust"), "W/K"),
                 ("Φ", self.tr("heating load", "Heizlast"), "W"), ("HDD", self.tr("heating degree days", "Heizgradtage"), "K·d"),
                 ("Q", self.tr("annual heating demand", "Jahresheizwärmebedarf"), "kWh/a"), ("Q/A", self.tr("demand per floor area", "Bedarf je Grundfläche"), "kWh/(m²·a)"),
                 ("×", self.tr("capsule count per type", "Kapselzahl je Typ"), "1"), (self.xl_localize("=SUMPRODUCT()").lstrip("=").rstrip("()"), self.tr("sum of products", "Summe der Produkte"), "")]
        legend = _engine.VGroup()
        for i, (symbol, name, unit) in enumerate(terms):
            y = 2.4 - i * 0.42
            legend.add(self._say(symbol, (-6.6, y), size=LABEL, color=DATA, anchor="l"), self._say(name, (-3.9, y), size=LABEL, color=INK, anchor="l", width=3.5),
                       self._say(unit, (-0.2, y), size=LABEL, color=MUTED, anchor="l"))
        kinds = self._slot_types()
        legend.add(self.tower_axon(at=(4.9, -0.45, 0.0), scale=0.085, tint=lambda key: _drawing.type_color(kinds[key]), lit=None))
        EN, T = self.EN, self.T
        a, b = self.vals_base, self.vals_better
        lines = [(self.tr("Type A: ", "Typ A: ") + self.xl_display(a[(T, "B14")], '#,##0') + " → " + self.xl_display(b[(T, "B14")], '#,##0 "kWh/a"')),
                 (self.xl_display(a[(T, "B15")], '0') + " → " + self.xl_display(b[(T, "B15")], '0 "kWh/(m²·a)"')),
                 (self.tr("Building: ", "Gebäude: ") + self.xl_display(a[(EN, "D8")], '#,##0') + " → " + self.xl_display(b[(EN, "D8")], '#,##0 "kWh/a"'))]
        for i, line in enumerate(lines):
            legend.add(self._say(line, (4.9, 2.75 - i * 0.36), size=SMALL, color=DATA, width=4.4))
        self.step("recap", _engine.FadeOut(window), *[_engine.FadeOut(m) for m in self.frames], _engine.FadeOut(chain), _engine.FadeOut(counted), _engine.FadeOut(small), _engine.FadeIn(legend), seconds=3.0)

