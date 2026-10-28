"""📉 Formulate a feasible optimization problem and distinguish local from global extrema."""

import importlib
import math
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_room = _visuals._room
_art = _visuals._art
_fonts = _visuals._fonts
_mn = _visuals._engine

INK, MUTED, DESIGN, DATA, GOOD, WARN = _visuals.INK, _visuals.MUTED, _visuals.DESIGN, _visuals.DATA, _visuals.GOOD, _visuals.WARN
HEAT, COOL, LIGHT = _art.P_RED, _art.P_BLUE, _art.P_YELLOW
LOCAL = _art.PASTEL_PURPLE
PARTS = (("heating", ("Heating", "Heizstrom"), HEAT), ("cooling", ("Cooling", "Kühlstrom"), COOL), ("lighting", ("Lighting", "Beleuchtung"), LIGHT))
BEARINGS = ((90, "East", "Ost"), (180, "South", "Süd"), (270, "West", "West"), (360, "North", "Nord"), (450, "East", "Ost"))
SECTION_SCALE = .38
SECTION_POINT = {"summer": np.array([-3.95, 1.12, 0.]), "winter": np.array([-3.95, -1.25, 0.])}
AXES_AT, AXES_SIZE, Y_MAX, BAR_MAX = np.array([1.0, -.7, 0.]), (3.9, 1.9), 1700, 1500.
ZONE_X, ZONE_TOP, ZONE_MID, ROW = -1.75, 2.72, 2.25, .26
PLAN_AT, PLAN_SCALE = np.array([5.95, 1.5, 0.]), .14
BAR_AT, BAR_HEIGHT, GHOST_X = np.array([5.7, -2.1, 0.]), 2.5, 4.0


BALANCE_KEYS = {"heating": ("heating_losses", "heating_gains", "heating_eta", "heating_used"), "cooling": ("cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed")}


def half_up(value, digits=0):
    """➗ Round half up, the way the printed numbers are rounded."""
    return math.floor(value * 10 ** digits + .5) / 10 ** digits


def balance_lines(found, part):
    """🎚️ The printed digits of a balance: the fewest decimals (kWh/a, then percent) at which ``share % × printed base`` rounds to the printed result and ``top − result = demand`` closes.

    See https://en.wikipedia.org/wiki/Largest_remainder_method
    """
    first, second, eta, used = BALANCE_KEYS[part]
    for digits in (0, 1, 2, 3):
        top, taken = _room.apportion([found[first], -found[used]], digits, found[part + "_demand"])
        base = half_up(found[second], digits)
        for places in (1, 2, 3, 4):
            percent = half_up(100 * found[eta], places)
            if half_up(percent / 100 * base, digits) == -taken:
                return {"digits": digits, "places": places, "top": top, "taken": -taken, "base": base, "percent": percent, "exact": abs(percent / 100 * base + taken) < 1e-9}
    raise ValueError(f"no printable balance for {part}")


class Beat1_LocalGlobalExtrema(_visuals.ClipScene):
    """🏔️ Teach direction, feasible domain and extrema by turning a glazed façade and reading its annual electricity."""

    TITLE = {"en":"Minimization and Maximization", "de":"Minimierung und Maximierung"}
    NARRATION = [
        ["formulate", "This seminar room has one glazed façade. Which compass direction should it face?", "Dieser Seminarraum hat eine verglaste Fassade. In welche Himmelsrichtung soll sie zeigen?"],
        ["formulate_2", "The façade orientation θ is our design variable: east 90°, south 180°, west 270°, north 360°.", "Die Ausrichtung der Fassade θ ist unsere Entwurfsvariable: Ost 90°, Süd 180°, West 270°, Nord 360°."],
        ["fixed", "Everything else stays fixed: window share, overhang and insulation. Only the orientation θ turns.", "Alles andere bleibt fest: Fensteranteil, Vordach und Dämmung. Nur die Ausrichtung θ dreht sich."],
        ["response", "Objective: the electricity for heating, cooling and lighting, in kWh/a.", "Die Zielgröße ist der Strombedarf aus Heizstrom, Kühlstrom und Beleuchtung, in kWh/a."],
        ["response_2", "We minimize it, so lower is better. The values come from a simplified teaching model of one year.", "Wir minimieren ihn, denn kleiner ist besser. Die Werte stammen aus einem stark vereinfachten Jahresmodell."],
        ["domain", "We turn the façade from east, 90°, through all orientations and simulate electricity in kWh/a.", "Wir drehen die Fassade von Osten, 90°, über alle Ausrichtungen und simulieren den Strombedarf in kWh/a."],
        ["sweep", "Turning south, the winter sun heats the room for free: heating falls.", "Beim Drehen nach Süden heizt die tiefe Wintersonne den Raum gratis: Der Heizstrom sinkt."],
        ["sweep_2", "The high summer sun hits south glass steeply: cooling electricity drops to the lowest so far.", "Die hohe Sommersonne trifft das Südglas steil: Der Kühlstrom sinkt auf den bisher kleinsten Wert."],
        ["sweep_3", "Facing west, the low evening sun floods the room: cooling stays high, and the total peaks near 282°.", "Nach Westen flutet die tiefe Abendsonne den Raum: Der Kühlstrom bleibt hoch, die Summe gipfelt bei 282°."],
        ["sweep_4", "Facing north, only diffuse light reaches the glass: cooling is least, heating and lighting rise.", "Nach Norden erreicht nur diffuses Licht das Glas: Kühlstrom minimal, Heiz- und Beleuchtungsstrom steigen."],
        ["sweep_5", "At 450° the façade faces east again, so the curve ends where it began.", "Bei 450° zeigt die Fassade wieder nach Ost, die Kurve schließt sich."],
        ["local", "Start facing northwest, 315°, and always turn toward less electricity.", "Wir starten mit der Fassade nach Nordwest, 315°, und drehen stets dorthin, wo der Strombedarf sinkt."],
        ["local_2", "At 360°, north, cooling rises faster than heating falls: every neighbour is worse, a local minimum.", "Bei 360°, Nord, steigt der Kühlstrom schneller als der Heizstrom sinkt: Jeder Nachbar ist schlechter, ein lokales Minimum."],
        ["global", "But south, 180°, is lower still: it is the global minimum of the whole range.", "Doch Süd, 180°, liegt noch tiefer: Es ist das globale Minimum."],
        ["global_2", "From north to south the free winter sun saves far more heating than the summer sun adds cooling.", "Von Nord nach Süd spart die gratis Wintersonne weit mehr Heizstrom, als die Sommersonne Kühlstrom addiert."],
        ["global_3", "Stopping in the north valley wastes electricity every year, and that costs CHF as well.", "Der Halt im Nordtal verschwendet jedes Jahr Strom und Geld."],
        ["argument", "The optimal design is θ* = 180°, south. Its value, E at θ*, is the annual electricity, in kWh/a.", "Der optimale Entwurf ist θ* = 180°, Süd. Sein Wert, E bei θ*, ist der Strombedarf in kWh/a."],
        ["argument_2", "Keep them apart: the design θ* is a direction; the value E at θ* is electricity in kWh/a.", "Nicht verwechseln: Der Entwurf θ* ist eine Richtung, der Wert E bei θ* ist der Strombedarf in kWh/a."],
        ["compare", "With one variable we can scan every degree and compare all valleys, not just the nearest.", "Mit einer Variablen prüfen wir jeden Grad und vergleichen alle Täler, nicht nur das nächste."],
        ["compare_2", "With many variables, a search can end in the wrong valley, just as our start at 315° did.", "Bei vielen Variablen kann eine Suche im falschen Tal enden, wie es unser Start bei 315° tat."],
        ["maximize", "Now the opposite: how much do we save against an east façade, in kWh/a and CHF/a?", "Nun umgekehrt: Wie viel sparen wir gegenüber einer Ostfassade, in kWh/a und CHF/a?"],
        ["negate", "Minimizing E equals maximizing the saving S: valleys turn into peaks and the curve flips over.", "E minimieren heißt die Einsparung S maximieren: Täler werden Gipfel, die Kurve klappt um."],
        ["negate_2", "The highest saving sits at the same orientation as the lowest electricity: θ* = 180°.", "Die größte Einsparung liegt bei derselben Ausrichtung wie der kleinste Strom: θ* = 180°."],
        ["constraint", "The street to the south is loud: the glazed façade must face the quiet courtyard, north ± 90°.", "Die Straße im Süden ist laut: Die Glasfassade muss zum ruhigen Hof zeigen, Nord ± 90°."],
        ["constraint_2", "A constraint limits which designs are allowed. It adds no objective and changes no value of E.", "Eine Nebenbedingung begrenzt die zulässigen Entwürfe. Sie fügt keine Zielgröße hinzu und ändert keinen Wert von E."],
        ["feasible", "Among the allowed orientations, north, 360°, is now the best one.", "Unter den zulässigen Ausrichtungen ist Nord, 360°, nun die beste."],
        ["feasible_2", "The quiet courtyard has a price: extra kWh/a and CHF/a compared with the free south optimum.", "Der ruhige Hof hat seinen Preis: Mehr kWh/a und CHF/a als beim freien Optimum im Süden."],
        ["scope", "Another window share moves the optimum's value; an overhang moves its direction as well.", "Ein anderer Fensteranteil verschiebt den Optimalwert; ein Vordach verschiebt auch die Richtung."],
        ["scope_2", "With the courtyard rule 270°–450°, the answer here is north, 360°.", "Mit der Hofregel 270° bis 450° lautet die Antwort hier: Nord, 360°."],
    ]

    def construct(self):
        """▶️ Animate the full formulation through the shared local rendering interface."""
        self.present("extrema")

    def present_extrema(self):
        """🎞️ Run the phases: formulate, fix, sweep, local and global minimum, maximize, constrain, scope."""
        study = self.study = _model.orientation_study()
        table = dict(study["samples"])
        self.low, self.high = study["range"]
        self.energy = lambda theta: self._interpolate(table, theta)
        self.reference_energy = study["reference_electricity"]
        self.saving = lambda theta: self.reference_energy - self.energy(theta)
        self.fixed = dict(study["design"])
        self.peak, self.peak_energy = study["local_maxima"][0], study["local_maxima_electricity"][0]
        self.dials = self.study_trackers({**self.fixed, "orientation": 90.0})
        self.upto = _mn.ValueTracker(self.low)
        self.maximum = BAR_MAX
        self.m = {"zone": None}
        self._subtitle = None
        self._phase_formulate()
        self._phase_fixed()
        self._phase_response()
        self._phase_sweep()
        self._phase_extrema()
        self._phase_maximize()
        self._phase_constraint()
        self._phase_scope()

    @staticmethod
    def _interpolate(table, theta):
        """📈 Linear interpolation of the one-degree electricity table."""
        theta = min(max(theta, min(table)), max(table))
        floor = int(np.floor(theta))
        ceil = min(floor + 1, max(table))
        return table[floor] + (table[ceil] - table[floor]) * (theta - floor)

    def _direction(self, theta):
        """🧿 The compass word of a bearing as printed on the axis ticks (East, South, ...), the plan's abbreviation in between."""
        word = {round(angle) % 360: (en, de) for angle, en, de in BEARINGS}.get(round(theta) % 360)
        return self.tr(*word) if word and abs(theta - round(theta)) < 1e-6 else _room.cardinal(theta, self._german())

    def _at(self, theta, **design):
        """⚙️ Model result of the study design at one façade bearing."""
        return _room.result({**self.fixed, **design, "orientation": float(theta)})

    def _wholes(self, found, digits=0, keys=("heating", "cooling", "lighting"), total="electricity"):
        """🧮 The parts of one result as printed numbers that add up to the printed total, and that total (toolkit largest-remainder rounding)."""
        shown = self.parts([found[key] for key in keys], digits, found[total])
        return shown, round(sum(shown), digits)

    def _t(self, value, point=(0, 0, 0), *, size=18, color=INK, width=6.5, edge="center"):
        """🔤 One label in the scene typeface, aligned by its left, right or centre edge."""
        label = _visuals._text(value, (0, 0, 0), size=size, color=color, width=width)
        return _room.place(label, point, edge)

    def _l(self, en, de, point=(0, 0, 0), **kwargs):
        """🏷️ One bilingual label."""
        return self._t(self.tr(en, de), point, **kwargs)

    def _sub(self, en, de):
        """📑 Swap the standalone instructional section heading."""
        new = _fonts.section_heading(self.tr(en, de))
        old, self._subtitle = self._subtitle, new
        return [_mn.FadeOut(old), _mn.FadeIn(new)] if old is not None else [_mn.FadeIn(new)]

    def _zone(self, new):
        """🔝 Swap the content of the formula zone above the plot."""
        old, self.m["zone"] = self.m["zone"], new
        return ([_mn.FadeOut(old)] if old is not None else []) + ([_mn.FadeIn(new)] if new is not None else [])

    def _footnote(self):
        """📖 Model disclaimer pinned to the empty top-right corner."""
        note = _fonts.body_text(self.tr("Simplified annual teaching model · Zurich-like climate", "Vereinfachtes Jahresmodell · Klima wie Zürich"),
                                font_size=_fonts.LABEL_FONT_SIZE - 3, color=_art.P_PRIMARY)
        note.set_opacity(.72)
        return note.to_corner(_mn.UR, buff=.3)

    def _design_chips(self):
        """🧱 The design values held fixed, in the colours of the section drawing."""
        d, german = self.fixed, self._german()
        chips = _mn.VGroup(_room.chip([self.tr("Window share", "Fensteranteil"), *_room.parts(d["wwr"] * 100, "%", 0, german)], size=14, color=_art.P_CYAN),
                           _room.chip([self.tr("Overhang", "Vordach"), *_room.parts(d["overhang"], "m", 1, german)], size=14, color=WARN),
                           _room.chip([self.tr("Insulation", "Dämmung"), *_room.parts(d["insulation"] * 100, "cm", 0, german)], size=14, color=GOOD)).arrange(_mn.RIGHT, buff=.14)
        return chips.to_corner(_mn.UR, buff=.3).shift(_mn.DOWN * .27)

    def _travel(self, tracker, thetas, *, follow=None, beat=1.0):
        """🧭 Move a tracker through bearings, easing every leg; ``follow`` trackers receive the same value."""
        legs = list(zip(thetas, thetas[1:]))
        weights = [1 + abs(b - a) / 30 * beat for a, b in legs]
        edges = np.cumsum([0, *weights]) / sum(weights)

        def update(mob, alpha):
            leg = min(int(np.searchsorted(edges, alpha, side="right")) - 1, len(legs) - 1)
            a, b = legs[leg]
            value = a + (b - a) * _mn.smooth((alpha - edges[leg]) / (edges[leg + 1] - edges[leg]))
            mob.set_value(value)
            for other in follow or []:
                other.set_value(value)

        return _mn.UpdateFromAlphaFunc(tracker, update, rate_func=_mn.linear)

    def _within(self, total, begin, end, *animations):
        """⏳ Run animations only between ``begin`` and ``end`` seconds of a step of ``total`` seconds, so live widgets are never faded while their tracker moves."""
        parts = ([_mn.Wait(begin)] if begin > 0 else []) + [_mn.AnimationGroup(*animations, run_time=end - begin)] + ([_mn.Wait(total - end)] if total - end > 0 else [])
        return _mn.Succession(*parts)

    def _plan(self, **extras):
        """🗺️ Compass plan in the right upper corner (the sun is drawn in the sections)."""
        return self.plan(self.dials, PLAN_AT, PLAN_SCALE, **extras)

    def _bars(self):
        """📊 Stacked electricity bar in the right column."""
        return _mn.VGroup(self.energy_bars(self.dials, BAR_AT, BAR_HEIGHT, self.maximum, size=14, labels=False, intensity=False), self._bar_values())

    def _segment_labels(self, found, x, edge):
        """🔢 The kWh/a of the three stacked segments, each beside its own segment in the colour of the segment."""
        shown, _ = self._wholes(found)
        group, base, floor = _mn.VGroup(), 0.0, BAR_AT[1] + .14
        for value, exact, (_, _, color) in zip(shown, [found[key] for key in ("heating", "cooling", "lighting")], PARTS):
            y = max(BAR_AT[1] + (base + exact / 2) * BAR_HEIGHT / self.maximum, floor)
            group.add(_room.place(_room.text(_room.parts(value, "", 0, self._german())[:1], size=14, color=color), (x, y, 0), edge))
            base, floor = base + exact, y + .24
        return group

    def _bar_values(self):
        """🔠 Live segment values right of the live bar."""
        return self._widget(lambda: _mn.VGroup(self._segment_labels(_room.result(_room.values(self.dials)), BAR_AT[0] + .5, "left")), _room.design_key(self.dials))

    def _bar_y(self, kwh):
        """📏 Screen height of an electricity value on the bar scale."""
        return BAR_AT[1] + kwh * BAR_HEIGHT / self.maximum

    def _ghost_bar(self, theta, color=MUTED):
        """👻 A second, fixed bar of another façade bearing beside the live bar, named by that bearing."""
        fixed = self.study_trackers({**self.fixed, "orientation": float(theta)})
        bars = self.energy_bars(fixed, np.array([GHOST_X, BAR_AT[1], 0.]), BAR_HEIGHT, self.maximum, size=14, compact=True, intensity=False)
        bars[0].remove(bars[0][1], bars[0][2])
        total = self._at(theta)["electricity"]
        tag = _room.text([self._direction(theta), *_room.parts(theta, "°", 0, self._german())], (GHOST_X, self._bar_y(total) + .62, 0), size=14, color=color)
        return _mn.VGroup(bars, tag, self._segment_labels(self._at(theta), GHOST_X + .5, "left"))

    def _live_name(self):
        """🔖 The live bearing above the live bar."""
        german = self._german()

        def build():
            found = _room.result(_room.values(self.dials))
            theta = self.dials["orientation"].get_value()
            return _mn.VGroup(_room.text([self._direction(theta), *_room.parts(theta, "°", 0, german)], (BAR_AT[0], self._bar_y(found["electricity"]) + .62, 0), size=14, color=INK))

        return self._widget(build, _room.design_key(self.dials))

    def _cost_tag(self):
        """💶 The electricity bill of the live design under the live bar."""
        german, tariff = self._german(), self.study["tariff"]

        def build():
            found = _room.result(_room.values(self.dials))
            return _mn.VGroup(_room.text(["≈", *_room.parts(found["electricity"] * tariff, "CHF/a", 1, german)], (BAR_AT[0], BAR_AT[1] - .3, 0), size=15, color=WARN))

        return self._widget(build, _room.design_key(self.dials))

    def _cells(self, rows, edges, *, top=ZONE_TOP, size=15):
        """🧾 Static table in the formula zone: ``edges`` are ``(offset, "left"|"right")`` columns, every row ``(text, colour[, width])`` cells."""
        group, y = _mn.VGroup(), top
        for cells in rows:
            for (text, color, *width), (offset, edge) in zip(cells, edges):
                if text:
                    group.add(self._t(text, (ZONE_X + offset, y, 0), size=size, color=color, width=width[0] if width else 2.6 if edge == "left" else 2.5, edge=edge))
            y -= ROW
        return group

    def _signed(self, value, digits=1):
        """➕ A change in kWh/a with an explicit sign."""
        value = round(value, digits)
        return ("+" if value > 0 else "−" if value < 0 else "") + self.number(abs(value), digits)

    def _sum_formula(self, point, size=18):
        """⚡ ``Heating + Cooling + Lighting = Total`` with the live kWh/a in the colours of the bar, in one German term set."""
        keys, colors = ("heating", "cooling", "lighting"), (HEAT, COOL, LIGHT)
        names = [self.tr(*words) for _, words, _ in PARTS]
        numbers = [item for key, color in zip(keys, colors) for item in (self.live(key, color, parts_of=keys), "+")][:-1]
        table = [[item for name, color in zip(names, colors) for item in ((name, color), "+")][:-1] + ["=", (self.tr("Total", "Summe"), INK)],
                 [*numbers, "=", self.live("electricity", INK, unit="kWh/a")]]
        return self.formula(self.dials, table, point, size=size, align="left", width=6.4)

    def _demand(self, part, point, size=15):
        """🧪 The balance behind a demand as two short exact lines: useful share of the gains (or removable heat) in %, then the whole-number subtraction that closes as printed."""
        winter = part == "heating"
        color = HEAT if winter else COOL
        soft = _room.muted(color)

        def table(design, found):
            line = balance_lines(found, part)
            digits = line["digits"]
            names = ((self.tr("Useful gains", "Nutzbare Gewinne"), self.tr("Losses", "Verluste"), self.tr("Heating demand", "Heizbedarf")) if winter else
                     (self.tr("Removed heat", "Abgeführt"), self.tr("Gains", "Gewinne"), self.tr("Cooling demand", "Kühlbedarf")))
            share = [(names[0], soft), "=", self.live(line["percent"], INK, line["places"], "%"), self.tr("of", "von"), self.live(line["base"], soft, digits), "=" if line["exact"] else "≈", self.live(line["taken"], soft, digits, "kWh/a")]
            balance = [(names[1], soft), self.live(line["top"], soft, digits), "−", (names[0].lower() if self.language() == "en" else names[0], soft), self.live(line["taken"], soft, digits), "=", (names[2], color), self.live(part + "_demand", color, digits, "kWh/a")]
            return [share, balance]

        return self.formula(self.dials, table, point, size=size, align="left", width=6.4)

    def _conversion(self, part, point, size=15, width=6.4, short=False):
        """🪫 The last step to electricity as one line: heating demand ÷ COP, cooling demand ÷ EER (German: annual performance factor, JAZ)."""
        winter = part == "heating"
        color, factor = (HEAT, "cop") if winter else (COOL, "eer")
        soft = _room.muted(color)
        label = self.tr("COP" if winter else "EER", "JAZ" if winter else "Kälte-JAZ")
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            digits = balance_lines(found, part)["digits"]
            sign = "=" if half_up(half_up(found[part + "_demand"], digits) / found[factor]) == half_up(found[part]) else "≈"
            name = [] if short else [(self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), soft)]
            return [[*name, self.live(part + "_demand", soft, digits, "kWh/a"), "÷", (label, INK), self.live(factor, INK, 1), sign,
                     (self.tr(*PARTS[0 if winter else 1][1]), color), self.live(part, color, 0, "kWh/a", parts_of=keys)]]

        return self.formula(self.dials, table, point, size=size, align="left", width=width)

    def _lamps(self, point, size=15, width=6.4):
        """💡 The live lighting product: installed power × hours × share of the hours the lamps burn, printed so that it closes."""
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(found["lighting_power"] * found["lighting_hours"] * half_up(100 * found["lamps_on"], 1) / 100) == half_up(found["lighting"]) else "≈"
            head = [(self.tr("Installed power", "Leistung"), INK), "×", (self.tr("Hours", "Stunden"), INK), "×", (self.tr("Lamps on", "Lampen an"), LIGHT), sign, (self.tr("Lighting", "Beleuchtung"), LIGHT)]
            return [head, [self.live("lighting_power", INK, 3, "kW"), "×", self.live("lighting_hours", INK, 0, "h/a"), "×", self.live("lamps_on", LIGHT, 1, "%", scale=100), sign, self.live("lighting", LIGHT, 0, "kWh/a", parts_of=keys)]]

        return self.formula(self.dials, table, point, size=size, align="left", width=width)

    def _daylight_chain(self, point, size=15):
        """🌤️ The live chain from the window to the lamps: daylight factor, daylight autonomy, share of the hours the lamps burn."""
        glow = _room.muted(_room.DAYLIGHT, .2)
        table = [[(self.tr("Daylight factor", "Tageslichtquotient"), glow), "→", (self.tr("Daylight autonomy", "Tageslichtautonomie"), glow), "→", (self.tr("Lamps on", "Lampen an"), LIGHT)],
                 [self.live("daylight", glow, 1, "%"), "→", self.live("autonomy", glow, 1, "%", scale=100), "→", self.live("lamps_on", LIGHT, 1, "%", scale=100)]]
        return self.formula(self.dials, table, point, size=size, align="left", width=6.4)

    def _variant(self, part=None):
        """📋 The live sum above the live derivation of one part, set off by a rule: demand balance and ÷ COP or EER, or daylight factor and lamps on."""
        group = _mn.VGroup(self._sum_formula((ZONE_X, ZONE_TOP - .2, 0)))
        if part is not None:
            group.add(_mn.Line((ZONE_X, ZONE_TOP - .62, 0), (ZONE_X + 6.3, ZONE_TOP - .62, 0), color=MUTED, stroke_width=1.2, stroke_opacity=.6))
        if part in ("heating", "cooling"):
            group.add(self._demand(part, (ZONE_X, ZONE_TOP - 1.0, 0)), self._conversion(part, (ZONE_X, ZONE_TOP - 1.6, 0)))
        elif part == "lighting":
            group.add(self._daylight_chain((ZONE_X, ZONE_TOP - 1.0, 0)), self._lamps((ZONE_X, ZONE_TOP - 1.62, 0)))
        return group

    def _season_note(self, season, point, color):
        """🗓️ Left of the section: hours of direct sun and the sun on the glass relative to the sunniest bearing; above its roof: the season and what the arrows mean."""
        german, s = self._german(), SECTION_SCALE
        at = lambda x, y: np.array(point, dtype=float) + s * np.array([x - 3.0, y - 1.5, 0.0])
        summer = season == "summer"
        name = self.tr("Summer" if summer else "Winter", "Sommer" if summer else "Winter")
        arrow, shade = (self.tr("heat removed →", "Wärme abgeführt →"), COOL) if summer else (self.tr("← heat lost", "← Wärmeverlust"), HEAT)
        header = _mn.VGroup(self._t(name, size=15, color=color, width=1.2), self._t(arrow, size=14, color=shade, width=1.7)).arrange(_mn.RIGHT, buff=.2)
        header = _room.place(header, at(7.7, 3.9), "right")

        def build():
            theta = round(float(self.dials["orientation"].get_value()), 2)
            found = _room.result(_room.values(self.dials))
            count = len(_model.sun_samples(theta, season))
            lines = (_room.text([self.tr("direct sun", "direkte Sonne"), _room.Number(str(count)), "h"], size=14, color=color, width=1.75),
                     _room.text([self.tr("sun on glass", "Glasstrahlung"), _room.Number(_room.number(found["sun_" + season] * 100, 0, german)), "%"], size=14, color=color, width=1.75),
                     _room.text([self.tr("of the best bearing", "der besten Lage")], size=14, color=color, width=1.75))
            return _mn.VGroup(*[_room.place(line, at(-.1, -.5 - .55 * index), "right") for index, line in enumerate(lines)])

        return _mn.VGroup(header, self._widget(build, _room.design_key(self.dials)))

    def _views(self):
        """🏠 Winter and summer section of the live room, one above the other, with their season notes."""
        views = {}
        for season, color in (("summer", DATA), ("winter", WARN)):
            views[season] = _mn.VGroup(self.room_section(self.dials, SECTION_POINT[season], SECTION_SCALE, season, labels=False), self._season_note(season, SECTION_POINT[season], color))
        return views

    def _focus(self, center, size, color):
        """🔲 A static frame that draws the eye to one region of the stage."""
        return _mn.Rectangle(width=size[0], height=size[1], color=color, stroke_width=3).move_to(center).set_stroke(opacity=.9)

    def _facts(self):
        """🏫 The seminar room in words and numbers, with the two solstice suns."""
        w, d, h = _room.WIDTH, _room.DEPTH, _room.HEIGHT
        sun = self.study["sun"]
        rows = [self._l("Seminar room", "Seminarraum", size=30, color=INK),
                self._t(f"{self.number(w)} m × {self.number(d)} m × {self.number(h)} m · {self.number(w * d)} m²", size=21, color=DESIGN),
                self._l(f"One glazed façade of {self.number(w * h)} m²", f"Eine verglaste Fassade von {self.number(w * h)} m²", size=21, color=DESIGN),
                self._l("Zurich-like climate", "Klima wie Zürich", size=21, color=MUTED),
                self._l(f"Summer sun, noon altitude {self.number(sun['summer_altitude'])}°", f"Sommersonne, Mittagshöhe {self.number(sun['summer_altitude'])}°", size=19, color=DATA),
                self._l(f"Winter sun, noon altitude {self.number(sun['winter_altitude'])}°", f"Wintersonne, Mittagshöhe {self.number(sun['winter_altitude'])}°", size=19, color=WARN)]
        group = _mn.VGroup(*rows).arrange(_mn.DOWN, aligned_edge=_mn.LEFT, buff=.2)
        return group.move_to((3.9, 1.35, 0))

    def _strip(self):
        """📅 The compass unrolled into the bearing strip of the design variable."""
        left, right, y = 1.75, 6.35, 1.35
        x = lambda theta: left + (theta - self.low) / (self.high - self.low) * (right - left)
        group = _mn.VGroup(self._l("Design variable θ", "Entwurfsvariable θ", (1.3, 2.35, 0), size=26, color=DESIGN, edge="left", width=5.5),
                           self._l("Orientation of the glazed façade", "Ausrichtung der Glasfassade", (1.3, 1.95, 0), size=19, color=MUTED, edge="left", width=5.5),
                           _mn.Line((left, y, 0), (right, y, 0), color=MUTED, stroke_width=2))
        for theta, en, de in BEARINGS:
            group.add(_mn.Line((x(theta), y - .08, 0), (x(theta), y + .08, 0), color=MUTED, stroke_width=2),
                      self._t(self.tr(en, de), (x(theta), y - .3, 0), size=16, color=INK, width=.9), self._t(f"{theta}°", (x(theta), y - .6, 0), size=16, color=INK, width=.7))
        pointer = _mn.always_redraw(lambda: _mn.VGroup(
            _mn.Dot((x(self.dials["orientation"].get_value()), y, 0), radius=.1, color=GOOD),
            _mn.Line((x(self.dials["orientation"].get_value()), y, 0), (x(self.dials["orientation"].get_value()), y + .22, 0), color=GOOD, stroke_width=2)))
        return group, pointer

    def _phase_formulate(self):
        """🛋️ One room, one design variable: the bearing θ of its glazed façade."""
        m, dial = self.m, self.dials["orientation"]
        m["plan_big"] = self.plan(self.dials, (-3.5, .05, 0), .27, arcs=True, dimensions=True, season_labels=False)
        m["facts"] = self._facts()
        m["elevation"] = self.facade(self.dials, (3.9, -1.3, 0), .46, "summer")
        m["footnote"] = self._footnote()
        self.step("formulate", *self._sub("Design variable and objective", "Entwurfsvariable und Zielgröße"), _mn.FadeIn(m["footnote"]),
                  _mn.AnimationGroup(_mn.FadeIn(m["plan_big"], run_time=1.2), _mn.FadeIn(m["facts"], run_time=1.2), _mn.FadeIn(m["elevation"], run_time=1.2),
                                     self._within(4.0, 1.2, 4.0, self._travel(dial, [90, 180, 270, 360, 225]))), seconds=4.0)
        m["strip"], m["pointer"] = self._strip()
        self.add(m["pointer"])
        self.step("formulate_2", _mn.FadeOut(m["facts"]), _mn.FadeIn(m["strip"]), self._travel(dial, [225, 90, 450, 150]).set_run_time(2.8), seconds=2.8)

    def _phase_fixed(self):
        """📌 Window share, overhang and insulation are fixed: what each one is in the drawing."""
        m, found = self.m, self._at(180)
        big = self.room_section(self.dials, (-2.2, -.15, 0), .5, "winter", demand=False)
        german = self._german()
        rows = ((self.tr("Window share", "Fensteranteil"), _room.parts(self.fixed["wwr"] * 100, "%", 0, german), _art.P_CYAN,
                 self.tr(f"{self.number(found['window_area'], 1)} m² of glass in the {self.number(_room.WIDTH * _room.HEIGHT)} m² façade", f"{self.number(found['window_area'], 1)} m² Glas in der {self.number(_room.WIDTH * _room.HEIGHT)} m² großen Fassade")),
                (self.tr("Overhang", "Vordach"), _room.parts(self.fixed["overhang"], "m", 1, german), WARN,
                 self.tr("none: the sun reaches the whole glass", "keines: Die Sonne trifft das ganze Glas")),
                (self.tr("Insulation", "Dämmung"), _room.parts(self.fixed["insulation"] * 100, "cm", 0, german), GOOD,
                 self.tr(f"wall U-value {self.number(found['u_wall'], 2)} W/m²K", f"Wand-U-Wert {self.number(found['u_wall'], 2)} W/m²K")))
        card = _mn.VGroup()
        for index, (name, value, color, meaning) in enumerate(rows):
            y = 1.4 - index * 1.15
            card.add(_room.place(_room.chip([name, *value], size=19, color=color), (1.1, y, 0), "left"), self._t(meaning, (1.1, y - .5, 0), size=17, color=INK, width=5.8, edge="left"))
        card.add(self._l("Only the façade orientation θ turns", "Nur die Fassadenausrichtung θ dreht sich", (1.1, -2.1, 0), size=19, color=GOOD, width=5.8, edge="left"))
        m["chips"], m["big"], m["fixed_card"] = self._design_chips(), big, card
        self.step("fixed", _mn.FadeOut(m["plan_big"]), _mn.FadeOut(m["elevation"]), _mn.FadeOut(m["strip"]), _mn.FadeOut(m["pointer"]), _mn.FadeIn(big), _mn.FadeIn(card), _mn.FadeIn(m["chips"]),
                  self._within(2.4, .7, 2.4, self._travel(self.dials["orientation"], [150, 180])), seconds=2.4)

    def _icons(self):
        """🔣 Radiator, cooling unit and lamp as the three meanings of the stack."""
        lamp = _art.lamp_glyph((0, .25, 0), drop=.12, scale=1.5)
        return [_art.radiator((0, 0, 0), HEAT, height=.6, width=.85), _room._cooler((0, 0, 0), .85, COOL), _mn.VGroup(lamp["cable"], lamp["shade"], lamp["bulb"])]

    def _causes(self):
        """🔌 Three icon rows: each part of the electricity with the live derivation that produces it."""
        group = _mn.VGroup()
        for index, (icon, part) in enumerate(zip(self._icons(), ("heating", "cooling", "lighting"))):
            y = .75 - index * .8
            line = self._lamps((-.55, y, 0), width=5.2) if part == "lighting" else self._conversion(part, (-.55, y, 0), width=5.2, short=True)
            group.add(icon.move_to((-1.2, y, 0)), line)
        return group

    def _phase_response(self):
        """🎯 The objective: annual electricity from heating, cooling and lighting."""
        m = self.m
        m["plan"], m["bars"], m["views"] = self._plan(), self._bars(), self._views()
        m["cost"], m["causes"], base = self._cost_tag(), self._causes(), self._variant()
        m["objective"] = self._l("Objective E: electricity per year", "Ziel E: Strombedarf pro Jahr", (1.0, 1.3, 0), size=21, color=INK, width=3.9)
        self.step("response", _mn.FadeOut(m["big"]), _mn.FadeOut(m["fixed_card"]),
                  *[_mn.FadeIn(item) for item in (m["plan"], m["bars"], m["views"]["summer"], m["views"]["winter"], m["objective"], m["causes"], m["cost"])], *self._zone(base), seconds=1.8)
        goal = _mn.VGroup(self._l("Goal: minimize electricity E", "Ziel: Strombedarf E minimieren", size=20, color=GOOD, width=3.2), _mn.Arrow((0, .25, 0), (0, -.25, 0), buff=0, color=GOOD, stroke_width=5, tip_length=.16),
                          self._l("lower is better", "Kleiner ist besser", size=17, color=GOOD, width=1.7)).arrange(_mn.RIGHT, buff=.2).move_to((1.0, -1.75, 0))
        tariff = self._t(f"{self.tr('Tariff', 'Tarif')} {self.quantity(self.study['tariff'], 'CHF/kWh', 2)}", (1.0, -2.3, 0), size=17, color=WARN, width=3.6)
        m["goal"] = _mn.VGroup(goal, tariff)
        self.step("response_2", _mn.FadeIn(m["goal"]), seconds=1.2)

    def _stack(self, theta):
        """📚 Cumulative heating, cooling and lighting electricity at one bearing."""
        found = self._at(theta)
        return [0.0, found["heating"], found["heating"] + found["cooling"], found["electricity"]]

    def _bands(self, axes, upto):
        """🥞 The stacked area under the curve: heating, cooling, lighting, from the low end of the range up to ``upto``."""
        stops = [self.low + step for step in range(0, int(upto - self.low) + 1, 2)]
        if upto - stops[-1] > .01:
            stops.append(upto)
        group = _mn.VGroup()
        if len(stops) < 2:
            return group
        rows = [self._stack(x) for x in stops]
        for index, color in enumerate((HEAT, COOL, LIGHT)):
            top = [axes.c2p(x, row[index + 1]) for x, row in zip(stops, rows)]
            bottom = [axes.c2p(x, row[index]) for x, row in zip(reversed(stops), reversed(rows))]
            group.add(_mn.Polygon(*top, *bottom, stroke_width=0, fill_color=color, fill_opacity=.62))
        return group

    def _build_axes(self, y_range, y_ticks, y_label):
        """📐 Plot axes in the middle column with two-line bearing ticks."""
        axes, labels = self.plot_axes(x_range=(self.low, self.high, 90), y_range=y_range, x_label=None, y_label=y_label,
                                      point=AXES_AT, width=AXES_SIZE[0], height=AXES_SIZE[1], x_ticks=[], y_ticks=y_ticks)
        ticks = _mn.VGroup()
        for theta, en, de in BEARINGS:
            at = axes.c2p(theta, y_range[0])
            ticks.add(self._t(self.tr(en, de), at + _mn.DOWN * .24, size=14, width=.75), self._t(f"{theta}°", at + _mn.DOWN * .46, size=14, width=.6))
        return axes, labels[0], _mn.VGroup(*labels[1:]), ticks

    def _marker(self, axes, y_of, value_fn, label=""):
        """📍 Live marker with drop lines and a readout chip pinned to the upper right corner of the plot, clear of every other mark."""
        german = self._german()
        corner = np.array([axes.c2p(self.high, 0)[0], axes.c2p(0, axes.y_range[1])[1] - .2, 0.])
        dot = self.live_marker(axes, self.dials, "orientation", value_fn, readout=False)

        def build():
            design = _room.values(self.dials)
            chip = _room.chip(["θ", *_room.parts(design["orientation"], "°", 0, german), "·", *([label] if label else []), *_room.parts(y_of(design), "kWh/a", 0, german)], size=15, color=INK)
            return _mn.VGroup(_room.place(chip, corner, "right"))

        return _mn.VGroup(dot, self._live(build))

    def _phase_sweep(self):
        """🔄 Turn the façade through the whole range and trace the electricity curve as three stacked parts."""
        m, dial = self.m, self.dials["orientation"]
        axes, y_title, y_labels, ticks = self._build_axes((0, Y_MAX, 500), [0, 500, 1000, 1500], self.tr("E, kWh/a", "E, kWh/a"))
        x_title = self._l("Orientation θ of the glazed façade", "Ausrichtung θ der Glasfassade", (AXES_AT[0], -2.45, 0), size=17, width=AXES_SIZE[0] + .6)
        m["axes"], m["axes_e"], m["y_labels_e"], m["y_title_e"], m["ticks"], m["x_title"] = axes, axes, y_labels, y_title, ticks, x_title
        marker = self._marker(axes, lambda d: _room.result(d)["electricity"], "electricity")
        bands = _mn.always_redraw(lambda: self._bands(axes, self.upto.get_value()))
        edge = _mn.always_redraw(lambda: self._partial(axes, self.energy, self.upto.get_value(), DESIGN))
        m["marker"], m["bands"], m["edge"] = marker, bands, edge
        self.add(bands, edge)
        self.play(self.set_design(self.dials, {"orientation": 90.0}, .8), run_time=.8)
        self.step("domain", *self._sub("Sweeping the façade orientation", "Fassadenausrichtung durchfahren"), _mn.FadeOut(m["objective"]), _mn.FadeOut(m["goal"]), _mn.FadeOut(m["causes"]),
                  _mn.Create(axes), _mn.FadeIn(x_title), _mn.FadeIn(ticks), _mn.FadeIn(y_title), _mn.FadeIn(y_labels), _mn.FadeIn(marker), seconds=1.4)
        winter, summer = self._focus((-4.45, -1.39, 0), (5.0, 2.38), WARN), self._focus((-4.45, 1.165, 0), (5.0, 2.77), DATA)
        heating, cooling, lighting = self._variant("heating"), self._variant("cooling"), self._variant("lighting")
        self.step("sweep", _mn.FadeIn(winter), self._within(3, 0, .5, *self._zone(heating)), self._within(3, .5, 3, self._travel(dial, [90, 180], follow=[self.upto])), seconds=3)
        self.step("sweep_2", *self._swap(winter, summer), *self._zone(cooling), seconds=1.4)
        self.step("sweep_3", _mn.FadeOut(summer), self._travel(dial, [180, self.peak], follow=[self.upto]), seconds=2.6)
        peak = self._peak_tag(axes)
        self.step("sweep_4", _mn.FadeIn(peak), self._within(2.4, 0, .5, *self._zone(lighting)), self._within(2.4, .5, 2.4, self._travel(dial, [self.peak, 360], follow=[self.upto])), seconds=2.4)
        self.step("sweep_5", _mn.FadeOut(peak), self._within(1.8, 0, .5, *self._zone(self._variant())), self._within(1.8, .5, 1.8, self._travel(dial, [360, 450], follow=[self.upto])), seconds=1.8)
        m["curve"] = self._curve(axes, self.energy, self.low, self.high, color=DESIGN)
        m["fills"] = self._bands(axes, self.high)
        bands.clear_updaters(), edge.clear_updaters()
        self.remove(bands, edge)
        self.add(m["fills"], m["curve"])
        self.bring_to_front(marker)

    def _peak_tag(self, axes):
        """🗻 The model's maximum, marked on the curve with a dashed drop line and its bearing."""
        at = axes.c2p(self.peak, self.peak_energy)
        guide = _room.dashed(at, axes.c2p(self.peak, 0), color=WARN, stroke_width=2)
        tag = _mn.VGroup(self._t(f"{self.tr('maximum', 'Maximum')} {self.peak}°", size=14, color=WARN, width=1.5), self._t(self._kwh(self.peak_energy), size=14, color=WARN, width=1.5)).arrange(_mn.DOWN, buff=.04)
        _room.place(tag, (at[0] - .55 - tag.width / 2, at[1] + .22, 0), "center")
        return _mn.VGroup(guide, tag)

    def _partial(self, axes, function, upto, color):
        """✏️ The curve drawn so far, from the low end of the range up to ``upto``."""
        stops = [self.low + step for step in range(0, int(upto - self.low) + 1, 2)]
        if upto - stops[-1] > .01:
            stops.append(upto)
        curve = _mn.VMobject(color=color, stroke_width=3)
        if len(stops) > 1:
            curve.set_points_smoothly([axes.c2p(x, function(x)) for x in stops])
        return curve

    def _span(self, axes, a, b, color, width=5):
        """🖍️ The curve between two bearings as a thick trail."""
        low, high = sorted((float(a), float(b)))
        stops = [low + step for step in range(int(high - low) + 1)]
        if high - stops[-1] > .01:
            stops.append(high)
        trail = _mn.VMobject(color=color, stroke_width=width)
        if len(stops) > 1:
            trail.set_points_smoothly([axes.c2p(x, self.energy(x)) for x in stops])
        return trail

    def _kwh(self, value, digits=0, signed=False):
        """🔋 Electricity text in kWh/a, optionally with an explicit plus sign."""
        return ("+" if signed and value > 0 else "") + self.quantity(value, "kWh/a", digits)

    def _price_row(self, *lines, point=(1.5, ZONE_MID, 0)):
        """🪙 One boxed block of one or two value lines above the plot."""
        block = _mn.VGroup(*[self._t(line, size=16, color=WARN, width=6.0) for line in lines]).arrange(_mn.DOWN, buff=.08)
        return _art.window_chip(block, color=WARN).move_to(point)

    def _neighbours(self, theta):
        """🔍 The two neighbours at the search step and what each part of E does there: the decomposition of the local test."""
        step, here = self.study["neighbour_step"], self._at(theta)
        rows = [[(self.tr("θ", "θ"), MUTED), (self.tr("ΔE", "ΔE"), INK), (self.tr("Heating", "Heizstrom"), HEAT), (self.tr("Cooling", "Kühlstrom"), COOL), (self.tr("Lighting", "Beleuchtung"), LIGHT)]]
        for angle in (theta - step, theta + step):
            there = self._at(angle)
            change = {key: there[key] - here[key] for key in ("heating", "cooling", "lighting", "electricity")}
            parts, total = self._wholes(change, 1)
            shade = WARN if total > .05 else GOOD
            rows.append([(f"{angle}°", INK), (self._signed(total), shade), *[(self._signed(value), color) for value, (_, _, color) in zip(parts, PARTS)]])
        head = self._t(self.tr(f"Neighbours of {theta}°: change in kWh/a", f"Nachbarn von {theta}°: Änderung in kWh/a"), (ZONE_X, ZONE_TOP + .02, 0), size=15, color=LOCAL, width=5.5, edge="left")
        table = self._cells(rows, ((0, "left"), (1.9, "right"), (3.35, "right"), (4.8, "right"), (6.25, "right")), top=ZONE_TOP - ROW)
        return _mn.VGroup(head, table)

    def _compare_table(self, before, after, title, saved=False, extra=""):
        """⚖️ Heating, cooling and lighting at two bearings side by side, with the change (or the saving) and the total."""
        a, b = self._at(before), self._at(after)
        sign = -1 if saved else 1
        (low, low_total), (high, high_total) = self._wholes(a), self._wholes(b)
        head = [[(title, MUTED, 4.4), ("", MUTED), ("kWh/a", MUTED)]]
        rows = [[(self.tr(*words), color), (f"{self.number(low[index])} → {self.number(high[index])} kWh/a", INK), (self._signed(sign * (high[index] - low[index]), 0), color)]
                for index, (_, words, color) in enumerate(PARTS)]
        total = sign * (high_total - low_total)
        sums = [[(self.tr("Electricity E", "Strombedarf E"), INK), (f"{self.number(low_total)} → {self.number(high_total)} kWh/a", INK),
                 (f"{self._signed(total, 0)} kWh/a{extra}", GOOD if b["electricity"] < a["electricity"] else WARN)]]
        return self._cells(head + rows + sums, ((0, "left"), (1.7, "left"), (6.2, "right")), top=ZONE_TOP)

    def _phase_extrema(self):
        """⛰️ The local descent ends in the north valley; the south valley is the global minimum."""
        m, dial, axes, study = self.m, self.dials["orientation"], self.m["axes"], self.study
        path = [item["theta"] for item in study["descent"]]
        start, stop = study["start"], path[-1]
        e_start, e_local, e_global = self.energy(start), study["local_minima_electricity"][1], study["local_minima_electricity"][0]
        at_start = axes.c2p(start, e_start)
        ring = _mn.Circle(radius=.2, color=INK, stroke_width=3).move_to(at_start)
        tag = self._l("start", "Start", at_start + _mn.RIGHT * .32 + _mn.UP * .12, size=15, color=INK, width=1.0, edge="left")
        m["start"] = _mn.VGroup(ring, tag)
        near = self._neighbours(start)
        self.step("local", *self._sub("Local and global minimum", "Lokales und globales Minimum"), _mn.FadeIn(m["start"]), self._within(2.4, 0, .5, *self._zone(near)),
                  self._within(2.4, .5, 2.4, self._travel(dial, [450, start])), seconds=2.4)
        trail = _mn.always_redraw(lambda: self._span(axes, start, dial.get_value(), LOCAL))
        self.add(trail)
        near_stop = self._neighbours(stop)
        local = _mn.Circle(radius=.2, color=LOCAL, stroke_width=4).move_to(axes.c2p(stop, e_local))
        self.step("local_2", _mn.FadeOut(m["start"]), self._travel(dial, [start, *path]), self._within(3.2, 2.4, 3.2, *self._zone(near_stop), _mn.FadeIn(local)), seconds=3.2)
        m["local"], m["trail"] = local, trail
        glob_at = axes.c2p(study["global_minimum"], e_global)
        square = _mn.Square(side_length=.2, color=GOOD, fill_color=GOOD, fill_opacity=1).move_to(glob_at)
        legend = self._legend([(_mn.Circle(radius=.08, color=LOCAL, stroke_width=3), self.tr("local minimum", "Lokales Minimum"), LOCAL),
                               (_mn.Square(side_length=.16, color=GOOD, fill_color=GOOD, fill_opacity=1), self.tr("global minimum", "Globales Minimum"), GOOD)])
        level = _mn.DashedLine(glob_at, axes.c2p(stop, e_global), color=GOOD, stroke_width=2)
        north = self._ghost_bar(stop, LOCAL)
        name = self._live_name()
        trail.clear_updaters()
        self.step("global", self._within(2.5, 0, .5, _mn.FadeOut(trail),*self._zone(self._variant())), self._within(2.5, .5, 2.5, self._travel(dial, [stop, study["global_minimum"]])),
                  _mn.FadeIn(north), self._within(2.5, 2.0, 2.5, _mn.FadeIn(name)), _mn.FadeIn(square), _mn.FadeIn(legend), _mn.Create(level), seconds=2.5)
        m["ghost_bar"], m["name"], m["global"], m["legend"], m["level"] = north, name, square, legend, level
        gap = e_local - e_global
        table = self._compare_table(stop, study["global_minimum"], self.tr("North 360° → South 180°", "Nord 360° → Süd 180°"))
        self.step("global_2", *self._zone(table), _mn.FadeOut(legend), seconds=2.4)
        bracket = _mn.DoubleArrow(axes.c2p(stop, e_global), axes.c2p(stop, e_local), buff=0, color=WARN, stroke_width=3, tip_length=.14)
        row = self._price_row(f"{self.tr('North 360° − South 180°', 'Nord 360° − Süd 180°')}: {self.number(e_local)} − {self.number(e_global)} kWh/a",
                              f"= +{self.quantity(gap, 'kWh/a')}  ·  +{self.number(gap / e_global * 100, 1)} % {self.tr('of South', 'vom Süden')}  ·  +{self.quantity(gap * study['tariff'], 'CHF/a')}")
        m["gap"] = bracket
        self.step("global_3", *self._zone(row), _mn.GrowFromCenter(bracket), seconds=1.4)
        self._argument(m, study, e_global)
        self._compare(m, study, axes, dial)

    def _legend(self, entries, start=0.0):
        """🗝️ Marker legend as one row in the last line of the formula zone, between the formula rows and the plot title."""
        rows = [_mn.VGroup(shape, self._t(label, size=14, color=color, width=2.1)).arrange(_mn.RIGHT, buff=.12) for shape, label, color in entries]
        return _room.place(_mn.VGroup(*rows).arrange(_mn.RIGHT, buff=.45), (ZONE_X + start, 1.62, 0), "left")
    def _argument(self, m, study, e_global):
        """🆚 Distinguish the optimal design θ* from its value E at θ*."""
        theta_g = study["global_minimum"]
        design = _art.window_chip(self._t(f"θ* = {theta_g}° · {self.tr('south', 'Süd')}", size=17, color=GOOD, width=2.8), color=GOOD)
        value = _art.window_chip(self._t(f"{self.tr('E at θ*', 'E bei θ*')} = {self._kwh(e_global)}", size=17, color=GOOD, width=2.8), color=GOOD)
        design.move_to((-.1, ZONE_MID - .1, 0))
        value.move_to((3.2, ZONE_MID - .1, 0))
        arrow = _mn.Arrow(design.get_right(), value.get_left(), buff=.08, color=MUTED, stroke_width=3, tip_length=.15)
        tags = _mn.VGroup(self._l("design", "Entwurf", design.get_left() + _mn.UP * .4 + _mn.RIGHT * .02, size=16, color=MUTED, width=1.5, edge="left"),
                          self._l("value", "Wert", value.get_left() + _mn.UP * .4 + _mn.RIGHT * .02, size=16, color=MUTED, width=1.5, edge="left"))
        m["argument"] = _mn.VGroup(design, value, arrow, tags)
        self.step("argument", *self._zone(m["argument"]), seconds=1.4)
        self.step("argument_2", _mn.Indicate(design, color=GOOD), _mn.Indicate(value, color=GOOD), seconds=1.6)

    def _compare(self, m, study, axes, dial):
        """🔎 Scan every degree and compare the valleys; a search can end in the wrong one."""
        reveal = _mn.ValueTracker(self.low)
        scan = _mn.always_redraw(lambda: _mn.Line(axes.c2p(reveal.get_value(), axes.y_range[0]), axes.c2p(reveal.get_value(), axes.y_range[1]), color=INK, stroke_width=2, stroke_opacity=.7))
        cards = _mn.VGroup(
            _art.window_chip(self._t(f"{self.tr('North', 'Nord')} {study['local_minima'][1]}° · {self._kwh(study['local_minima_electricity'][1])}", size=15, color=LOCAL, width=2.8), color=LOCAL),
            _art.window_chip(self._t(f"{self.tr('South', 'Süd')} {study['local_minima'][0]}° · {self._kwh(study['local_minima_electricity'][0])} · {self.tr('best', 'Beste')}", size=15, color=GOOD, width=3.5), color=GOOD)).arrange(_mn.RIGHT, buff=.3).move_to((1.5, ZONE_MID, 0))
        self.add(scan)
        self.step("compare", *self._zone(cards), self._travel(dial, [study["global_minimum"], self.low, self.high, study["global_minimum"]], follow=[reveal], beat=.5).set_run_time(2.6), seconds=2.6)
        scan.clear_updaters()
        self.remove(scan)
        points = [axes.c2p(item["theta"], item["electricity"]) for item in study["descent"]]
        route = _mn.VMobject(color=LOCAL, stroke_width=4).set_points_as_corners(points)
        start = _mn.VGroup(_mn.Circle(radius=.2, color=INK, stroke_width=3).move_to(points[0]), self._l("start", "Start", points[0] + _mn.RIGHT * .32 + _mn.UP * .12, size=15, color=INK, width=1.0, edge="left"))
        self.step("compare_2", _mn.Create(route), _mn.FadeIn(start), seconds=1.6)
        m["cards"], m["route"], m["route_start"] = cards, route, start

    def _phase_maximize(self):
        """🔁 Maximize the savings against an east façade: the same curve, mirrored."""
        m, study, axes, dial = self.m, self.study, self.m["axes"], self.dials["orientation"]
        east, south = float(study["reference"]), float(study["global_minimum"])
        e_east, e_south = self.reference_energy, study["local_minima_electricity"][0]
        east_bar = self._ghost_bar(east, GOOD)
        plan_ref = self._plan(ghost=study["reference"])
        ghost_tag = self._l(f"dashed: east {int(east)}°", f"gestrichelt: Ost {int(east)}°", (PLAN_AT[0], .24, 0), size=14, color=GOOD, width=1.9)
        line = _mn.DashedLine(axes.c2p(self.low, e_east), axes.c2p(self.high, e_east), color=WARN, stroke_width=2)
        line_tag = _mn.VGroup(self._l("east reference", "Ost-Referenz", size=14, color=WARN, width=1.5, edge="left"),
                              self._t(self._kwh(e_east), size=14, color=WARN, width=1.9, edge="left")).arrange(_mn.DOWN, aligned_edge=_mn.LEFT, buff=.04)
        line_tag.move_to(axes.c2p(self.low, e_east) + _mn.UP * .4 + _mn.RIGHT * (.2 + line_tag.width / 2))
        gap_mark = _mn.VGroup(_mn.DoubleArrow(axes.c2p(south, e_east), axes.c2p(south, e_south), buff=0, color=GOOD, stroke_width=3, tip_length=.14),
                              self._t("S", axes.c2p(south, (e_east + e_south) / 2) + _mn.RIGHT * .2, size=18, color=GOOD, width=.4))
        saving = e_east - e_south
        rows = self._compare_table(east, south, self.tr("East 90° → South 180°: saving S", "Ost 90° → Süd 180°: Einsparung S"), saved=True,
                                   extra=f" ≈ {self.quantity(saving * study['tariff'], 'CHF/a', 1)}")
        bar_x = BAR_AT[0] + 1.15
        bracket = _mn.VGroup(self._bracket(e_east, e_south, bar_x, GOOD),
                             _room.dashed((GHOST_X + .45, self._bar_y(e_east), 0), (bar_x, self._bar_y(e_east), 0), color=GOOD, stroke_width=1.6),
                             _room.dashed((BAR_AT[0] + .45, self._bar_y(e_south), 0), (bar_x, self._bar_y(e_south), 0), color=GOOD, stroke_width=1.6))
        clear =[m["route"], m["route_start"], m["local"], m["global"], m["level"], m["gap"], m["ghost_bar"], m["fills"]]
        self.step("maximize", *self._sub("Maximizing the saving", "Einsparung maximieren"), *[_mn.FadeOut(item) for item in clear], *self._zone(rows), _mn.FadeIn(east_bar), *self._swap(m["plan"], plan_ref), _mn.FadeIn(ghost_tag),
                  _mn.Create(line), _mn.FadeIn(line_tag), _mn.GrowFromCenter(gap_mark), _mn.GrowFromCenter(bracket), seconds=2.6)
        m.update(ghost_bar=east_bar, plan=plan_ref, line=line, line_tag=line_tag, ghost_tag=ghost_tag, gap_mark=gap_mark, saving=bracket)
        axes2, y_title2, y_labels2, _ = self._build_axes((-100, 500, 100), [0, 200, 400], self.tr("S, kWh/a", "S, kWh/a"))
        formula = _mn.VGroup(self._l("S at θ = E at 90° − E at θ", "S bei θ = E bei 90° − E bei θ", size=19, color=GOOD, width=4.2), self._t("min E  ⇔  max S", size=19, color=GOOD, width=2.0)).arrange(_mn.RIGHT, buff=.4).move_to((1.5, ZONE_MID, 0))
        dips = [float(theta) for theta in study["local_maxima"]]
        left, zero = axes2.c2p(self.low, 0)[0] + .12, axes2.c2p(0, 0)[1]
        zero_tag = _mn.VGroup(self._l("S = 0:", "S = 0:", (left, zero - .16, 0), size=14, color=WARN, width=.7, edge="left"), self._l("east 90°", "Ost 90°", (left, zero - .36, 0), size=14, color=WARN, width=.72, edge="left"))
        centre = axes2.c2p(dips[0], 0)[0] + .35
        worse = _mn.VGroup(self._l("S < 0:", "S < 0:", (centre, zero - .16, 0), size=14, color=WARN, width=1.0), self._l("worse than east", "schlechter als Ost", (centre, zero - .36, 0), size=14, color=WARN, width=1.6))
        curve2 = self._curve(axes2, self.saving, self.low, self.high, color=GOOD)
        marker2 = self._marker(axes2, lambda d: self.saving(d["orientation"]), lambda d: self.saving(d["orientation"]), "S")
        self.step("negate", _mn.ReplacementTransform(m["curve"], curve2), _mn.FadeOut(m["axes"]), _mn.FadeOut(m["y_labels_e"]), _mn.FadeOut(m["y_title_e"]), _mn.FadeIn(axes2), _mn.FadeIn(y_labels2), _mn.FadeIn(y_title2),
                  _mn.FadeOut(line), _mn.FadeOut(line_tag), _mn.FadeOut(gap_mark), _mn.FadeIn(zero_tag), _mn.FadeIn(worse), *self._swap(m["marker"], marker2), *self._zone(formula), seconds=2.6)
        m["axes"], m["y_labels_s"], m["y_title_s"], m["marker"], m["curve"] = axes2, y_labels2, y_title2, marker2, curve2
        m["zero_tag"], m["worse"], m["formula"] = zero_tag, worse, formula
        tariff = self.study["tariff"]
        pay = self._price_row(f"S = {self.quantity(saving, 'kWh/a')} × {self.quantity(tariff, 'CHF/kWh', 2)} = {self.quantity(saving * tariff, 'CHF/a', 1)}")
        self.step("negate_2", self._travel(dial, [south] * 2).set_run_time(.5), *self._zone(pay), _mn.Indicate(bracket, color=GOOD), seconds=1.4)

    def _swap(self, old, new):
        """🔀 Cross-fade one widget into another."""
        return [_mn.FadeOut(old), _mn.FadeIn(new)]

    def _bracket(self, top, bottom, x, color):
        """↕️ A vertical double arrow between two electricity levels on the bar scale."""
        return _mn.DoubleArrow((x, self._bar_y(bottom), 0), (x, self._bar_y(top), 0), buff=0, color=color, stroke_width=3, tip_length=.14)

    def _phase_constraint(self):
        """🚧 The courtyard rule: infeasible orientations, and the optimum moves."""
        m, study, dial = self.m, self.study, self.dials["orientation"]
        axes = m["axes_e"]
        lo, hi = study["constraint"]
        curve = self._curve(axes, self.energy, self.low, self.high, color=DESIGN)
        marker = self._marker(axes, lambda d: _room.result(d)["electricity"], "electricity")
        band = _mn.Polygon(axes.c2p(self.low, axes.y_range[0]), axes.c2p(lo, axes.y_range[0]), axes.c2p(lo, axes.y_range[1]), axes.c2p(self.low, axes.y_range[1]),
                           stroke_width=0, fill_color=WARN, fill_opacity=.2)
        limit = _mn.DashedLine(axes.c2p(lo, axes.y_range[0]), axes.c2p(lo, axes.y_range[1]) + _mn.DOWN * .62, color=WARN, stroke_width=3)
        band_tag = _mn.VGroup(_mn.Square(side_length=.16, color=WARN, fill_color=WARN, fill_opacity=.4), self._l("infeasible: street noise", "Unzulässig: Straßenlärm", size=14, color=WARN, width=2.2)).arrange(_mn.RIGHT, buff=.12)
        band_tag = _room.place(band_tag, (ZONE_X, 1.62, 0), "left")
        plan_c = self._plan(sector=(study["reference"], lo, WARN))
        sites = _mn.VGroup(self._l("Courtyard", "Innenhof", (PLAN_AT[0], PLAN_AT[1] + 1.32, 0), size=14, color=GOOD, width=1.3), self._l("Street", "Straße", (PLAN_AT[0] + .6, PLAN_AT[1] - 1.0, 0), size=14, color=WARN, width=.9))
        rule = _art.window_chip(self._t(f"{self.tr('Constraint', 'Nebenbedingung')}: {lo}° ≤ θ ≤ {hi}°", size=19, color=WARN, width=4.8), color=WARN).move_to((1.5, ZONE_MID, 0))
        old = [m["axes"], m["y_labels_s"], m["y_title_s"], m["zero_tag"], m["worse"], m["saving"], m["curve"], m["ghost_bar"], m["ghost_tag"]]
        self.step("constraint", *self._sub("Constraint: the quiet courtyard", "Nebenbedingung: der ruhige Hof"), *[_mn.FadeOut(item) for item in old], _mn.FadeIn(axes), _mn.FadeIn(m["y_labels_e"]), _mn.FadeIn(m["y_title_e"]), _mn.FadeIn(curve), _mn.FadeIn(m["fills"]),
                  self._within(2.8, 0, .6, *self._swap(m["marker"], marker), *self._swap(m["plan"], plan_c), *self._zone(rule)), _mn.FadeIn(band), _mn.Create(limit), _mn.FadeIn(band_tag), _mn.FadeIn(sites),
                  self._within(2.8, .6, 2.8, self._travel(dial, [study["global_minimum"], 270, 360])), seconds=2.8)
        m.update(curve=curve, marker=marker, plan=plan_c, band=band, limit=limit, band_tag=band_tag, sites=sites, rule=rule, axes=axes)
        e_global = study["local_minima_electricity"][0]
        glob_at = axes.c2p(study["global_minimum"], e_global)
        ghost = _mn.Square(side_length=.2, color=MUTED, fill_opacity=.3, fill_color=MUTED).move_to(glob_at)
        cross = _art.cross_mark(WARN).move_to(glob_at)
        legend = self._legend([(_mn.Square(side_length=.16, color=GOOD, fill_color=GOOD, fill_opacity=1), self.tr("feasible minimum", "Zulässig"), GOOD),
                               (_mn.VGroup(_mn.Square(side_length=.16, color=MUTED, fill_color=MUTED, fill_opacity=.3), _art.cross_mark(WARN).scale(.5)), self.tr("excluded", "Ausgeschlossen"), MUTED)], start=2.7)
        self.step("constraint_2", _mn.FadeIn(ghost), _mn.FadeIn(cross), self._travel(dial, [360, 300, lo + 2, 300, 360], beat=.5).set_run_time(2.6), _mn.Indicate(band, color=WARN), seconds=2.6)
        m["ghost"], m["cross"] = ghost, cross
        e_local = study["local_minima_electricity"][1]
        stop = study["feasible_minimum"]
        optimum = _mn.Square(side_length=.2, color=GOOD, fill_color=GOOD, fill_opacity=1).move_to(axes.c2p(stop, e_local))
        south = self._ghost_bar(study["global_minimum"], MUTED)
        self.step("feasible", _mn.FadeIn(optimum), _mn.FadeIn(legend), _mn.FadeIn(south), *self._zone(self._variant()), seconds=1.4)
        gap = e_local - e_global
        bracket = _mn.DoubleArrow(axes.c2p(stop, e_global), axes.c2p(stop, e_local), buff=0, color=WARN, stroke_width=3, tip_length=.14)
        level = _mn.DashedLine(glob_at, axes.c2p(stop, e_global), color=MUTED, stroke_width=2)
        price = self._price_row(f"{self.tr('Courtyard price', 'Hofpreis')}, {self.tr('North 360° − South 180°', 'Nord 360° − Süd 180°')}: {self.number(e_local)} − {self.number(e_global)} kWh/a",
                                f"= +{self.quantity(gap, 'kWh/a')}  ·  +{self.number(gap / e_global * 100, 1)} % {self.tr('of South', 'vom Süden')}  ·  +{self.quantity(gap * study['tariff'], 'CHF/a')}")
        self.step("feasible_2", _mn.Create(level), _mn.GrowFromCenter(bracket), *self._zone(price), seconds=1.6)
        m["constraint_extras"] = _mn.VGroup(ghost, cross, optimum, legend, level, bracket, south)

    def _optimum(self, design):
        """🥇 Best feasible bearing of a design on the one-degree grid, and its electricity."""
        lo, hi = self.study["constraint"]
        energy = {theta: _room.result({**self.fixed, **design, "orientation": float(theta)})["electricity"] for theta in range(lo, hi + 1)}
        best = min(energy, key=energy.get)
        return best, energy[best], energy

    def _phase_scope(self):
        """📜 State direction, domain and scope of the optimality claim."""
        m, study = self.m, self.study
        lo, hi = study["constraint"]
        keep = {id(self._caption), id(self._subtitle), id(m["footnote"])}
        content = [item for item in self.mobjects if id(item) not in keep]
        for item in content:
            item.clear_updaters()
        d, german = self.fixed, self._german()
        rows = [(self.tr("Direction", "Richtung"), self.tr("minimize E in kWh/a", "E minimieren in kWh/a"), GOOD),
                (self.tr("Domain", "Bereich"), f"{lo}° ≤ θ ≤ {hi}° · {self.tr('courtyard side', 'Hofseite')}", WARN),
                (self.tr("Scope", "Geltung"), self.tr(f"window share {self.number(d['wwr'] * 100)} %, overhang {self.number(d['overhang'], 1)} m, insulation {self.number(d['insulation'] * 100)} cm",
                                                      f"Fensteranteil {self.number(d['wwr'] * 100)} %, Vordach {self.number(d['overhang'], 1)} m, Dämmung {self.number(d['insulation'] * 100)} cm"), DESIGN)]
        card_rows = _mn.VGroup()
        for name, text, color in rows:
            card_rows.add(_mn.VGroup(self._t(name, size=21, color=color, width=2.0, edge="left"), self._t(text, size=21, color=INK, width=5.6, edge="left")).arrange(_mn.RIGHT, buff=.35, aligned_edge=_mn.UP))
        card_rows.add(self._l("Zurich-like climate · simplified annual teaching model", "Klima wie Zürich · vereinfachtes Jahresmodell", size=17, color=MUTED, width=7, edge="left"))
        card_rows.arrange(_mn.DOWN, aligned_edge=_mn.LEFT, buff=.3)
        frame = _art.window_frame(card_rows.width + .7, card_rows.height + .5, color=GOOD, fill_opacity=.88).move_to(card_rows)
        card = _mn.VGroup(frame, card_rows).move_to((.6, 1.45, 0))
        plan = self.plan(self.dials, (-5.0, .1, 0), .26, ring_radius=6.0, sector=(study["reference"], lo, WARN), site=(180, 0))
        shown = [({}, self.tr("window share 60 %", "Fensteranteil 60 %"), INK), ({"wwr": .3}, self.tr("window share 30 %", "Fensteranteil 30 %"), MUTED), ({"overhang": .6}, self.tr("overhang 0.6 m", "Vordach 0,6 m"), DESIGN)]
        found = [self._optimum(design) for design, _, _ in shown]
        floor_e = math.floor(min(min(curve.values()) for _, _, curve in found) / 100) * 100
        ceil_e = math.ceil(max(max(curve.values()) for _, _, curve in found) / 100) * 100
        axes, labels = self.plot_axes(x_range=(lo, hi, 90), y_range=(floor_e, ceil_e, 100), x_label=None, y_label=self.tr("E, kWh/a", "E, kWh/a"), point=(.0, -1.15, 0), width=4.6, height=1.6,
                                      x_ticks=[], y_ticks=[floor_e, ceil_e])
        x_names = _mn.VGroup(*[self._t(f"{self.tr(en, de)} {theta}°", axes.c2p(theta, floor_e) + _mn.DOWN * .24, size=14, width=1.1) for theta, en, de in BEARINGS[2:]])
        lines = [_mn.VMobject(color=color, stroke_width=3).set_points_smoothly([axes.c2p(t, e) for t, e in curve.items()]) for (_, _, color), (_, _, curve) in zip(shown, found)]
        dots = [_mn.Dot(axes.c2p(best, e), radius=.06, color=GOOD) for best, e, _ in found]
        values = [e for _, e, _ in found]
        colour = self.ramp(min(values), max(values))
        rows_y = (-.2, -.85, -1.5)
        tiles = [self.thumbnail({**self.fixed, **design, "orientation": float(best)}, (3.3, y, 0), .12, ramp=colour, value=e, ring=GOOD if index == 0 else None)
                 for index, ((design, _, _), (best, e, _), y) in enumerate(zip(shown, found, rows_y))]
        notes = [_mn.VGroup(self._t(name, (3.75, y + .14, 0), size=15, color=color, width=3.2, edge="left"), self._t(f"θ* = {best}°  ·  {self._kwh(e)}", (3.75, y - .14, 0), size=15, color=INK, width=3.2, edge="left"))
                 for (_, name, color), (best, e, _), y in zip(shown, found, rows_y)]
        strip = self.ramp_strip(colour, (5.3, -2.25, 0), width=2.4, size=14, title=self.tr("electricity at θ*", "Strombedarf bei θ*"))
        ripple = self._l("ripples: hourly sun samples", "Wellen: stündliche Sonnenstände", (0, -2.5, 0), size=14, color=MUTED, width=3.4)
        answer = _mn.Circle(radius=.2, color=GOOD, stroke_width=3).move_to(axes.c2p(found[0][0], found[0][1]))
        self.step("scope", *self._sub("Direction, domain and scope", "Richtung, Bereich und Geltung"), *[_mn.FadeOut(item) for item in content], _mn.FadeIn(card), _mn.FadeIn(plan), _mn.FadeIn(axes), _mn.FadeIn(labels), _mn.FadeIn(x_names),
                  *[_mn.FadeIn(item) for item in (*lines, *dots, *tiles, *notes)], _mn.FadeIn(strip), _mn.FadeIn(ripple), _mn.Succession(_mn.Wait(1.2), _mn.Indicate(card_rows[2], color=DESIGN)), seconds=2.8)
        bill = self._t(f"≈ {self.quantity(found[0][1] * study['tariff'], 'CHF/a', 1)}", (3.75, -.5, 0), size=15, color=WARN, width=2.0, edge="left")
        self.step("scope_2", _mn.FadeIn(bill), *[_mn.FadeOut(item) for item in (*lines[1:], *dots[1:], *tiles[1:], *notes[1:], strip, ripple)], _mn.FadeIn(answer), _mn.Indicate(card_rows[1], color=WARN), seconds=1.8)
