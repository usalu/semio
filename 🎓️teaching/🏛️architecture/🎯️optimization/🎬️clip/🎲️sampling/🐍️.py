"""🎲 Compare equal-budget grid, random and Latin hypercube evaluation designs."""

import importlib
import math
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import numpy as np

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
mn = _visuals._engine

INK, GRAY, GOOD, WARN, SUN = _room.INK, _room.GRAY, _room.GOOD, _room.WARN, _room.SUN
HEAT, COOL, LIGHT = _room.HEAT, _room.COOL, _room.LIGHT
PANEL = "#0e2a30"
SETS = {
    "grid": {"en": "Grid", "de": "Raster", "color": _visuals.DESIGN, "shape": "square", "scale": 0.19},
    "random": {"en": "Random", "de": "Zufall", "color": _visuals.WARN, "shape": "diamond", "scale": 0.11},
    "lhs": {"en": "Latin hypercube", "de": "Latin-Hypercube", "color": _visuals.GOOD, "shape": "triangle", "scale": 0.11},
}
FACTORS = {"wwr": _visuals.DESIGN, "overhang": _visuals.WARN, "orientation": _visuals.DATA, "insulation": _visuals.GOOD}
SECTION_SCALE = 0.32
SECTIONS = {"winter": (2.5, 0.85), "summer": (5.8, 0.85)}
BAR_BASE, BAR_HEIGHT = (1.0, -1.8), 1.35
BOARD_X = 4.25
CHIP_TOP = (4.15, 3.3, 0)
SLOT = 1.93
PANEL_X, PANEL_ROWS = 1.95, (-1.3, -1.57, -1.84, -2.11, -2.38)
ALONE_BASE = (0.8, -2.2)
PLANE_X, FLOOR = (0, 88), -0.22


def _clip(value):
    return min(1.0, max(0.0, value))


def _half_up(value, digits=0):
    return math.floor(value * 10 ** digits + 0.5) / 10 ** digits


def _smooth(value):
    value = _clip(value)
    return value * value * (3 - 2 * value)


def _fader(group):
    """🌫️ Make a static group fade by scaling the opacities it had when it was registered."""
    bases = [(member, member.pixel_array[:, :, 3].copy(), None) if isinstance(member, mn.ImageMobject)
             else (member, member.get_fill_opacity(), member.get_stroke_opacity()) for member in group.family_members_with_points()]

    def fade(amount):
        for member, fill, stroke in bases:
            if stroke is None:
                member.pixel_array[:, :, 3] = (fill * amount).astype("uint8")
                continue
            member.set_fill(opacity=fill * amount, family=False)
            member.set_stroke(opacity=stroke * amount, family=False)

    fade(0.0)
    return fade


class SnapshotSafeGroup(_room.LiveGroup):
    """📸 A live group whose static snapshots can be copied again, as ``FadeIn`` and ``Transform`` do."""

    def refresh(self):
        if self._builder is not None:
            super().refresh()
        return self


class Drive(mn.Animation):
    """⏱️ Run a pure function of the animation progress; live drawings follow because it only moves trackers and opacities."""

    def __init__(self, update, **kwargs):
        super().__init__(mn.Mobject(), **kwargs)
        self.update_state = update

    def interpolate_mobject(self, alpha):
        self.update_state(alpha)


class Beat2_GridRandomSampling(_visuals.ClipScene):
    """🗾 Explain coverage and dimensional cost without promising an optimum."""

    TITLE = {"en": "Sampling the Design Space", "de": "Den Entwurfsraum abtasten"}
    NARRATION = [
        ["parameters", "Which window share and overhang depth suit this south façade?", "Welcher Fensteranteil, welche Vordachtiefe passen zur Südfassade?"],
        ["parameters_2", "Each pair is one design: point and room move together.", "Jedes Paar ist ein Entwurf: Punkt und Raum wandern gemeinsam."],
        ["run", "One simulation sums a year of heat, sun and daylight; two solstice days are shown.", "Eine Simulation summiert ein Jahr aus Wärme, Sonne, Tageslicht; gezeigt: zwei Sonnwendtage."],
        ["run_2", "Winter sun cuts heating electricity, summer sun adds cooling, daylight cuts lighting.", "Wintersonne senkt Heizstrom, Sommersonne erhöht Kühlstrom, Tageslicht senkt Beleuchtung."],
        ["run_3", "One number per design: electricity in kWh/a, carried by each point.", "Eine Zahl je Entwurf: Strombedarf in kWh/a, getragen von jedem Punkt."],
        ["budget", "But only nine simulations are affordable. Which nine designs?", "Doch nur neun Simulationen sind bezahlbar. Welche neun Entwürfe?"],
        ["grid", "A full grid uses three levels per axis and combines them all.", "Ein volles Raster nutzt drei Stufen je Achse und kombiniert alle."],
        ["grid_2", "Simulate them one by one: room, sun rays and electricity bar for each.", "Wir simulieren sie nacheinander: Raum, Sonnenstrahlen und Strombalken für jeden Entwurf."],
        ["grid_3", "The ninth simulation empties the budget.", "Die neunte Simulation leert das Budget."],
        ["grid_4", "The best of nine is ringed; its room shows why it beats the worst.", "Der beste der neun ist umrandet; sein Raum zeigt, warum er den schlechtesten schlägt."],
        ["growth", "More design variables: orientation with four levels, insulation with three.", "Mehr Entwurfsvariablen: Ausrichtung mit vier, Dämmung mit drei Stufen."],
        ["growth_2", "Levels multiply: 3 · 3 · 4 · 3 makes 108 simulations.", "Stufen multiplizieren sich: 3 · 3 · 4 · 3 ergibt 108 Simulationen."],
        ["growth_3", "Every extra level multiplies the simulations: here twelve times our budget.", "Jede weitere Stufe vervielfacht die Simulationen: hier das Zwölffache unseres Budgets."],
        ["random", "Random sampling draws nine independent designs, recorded so everyone sees the same points.", "Zufälliges Sampling zieht neun unabhängige Entwürfe, gespeichert: alle sehen dieselben Punkte."],
        ["random_3", "Same budget, same model: nine more simulations.", "Gleiches Budget, gleiches Modell: neun weitere Simulationen."],
        ["gaps", "No draw lies inside this box (edges excluded): WWR 35 to 70 %, overhang 0.15 to 1.20 m.", "Kein Entwurf liegt in dieser Box (Rand ausgenommen): Fenster 35–70 %, Vordach 0,15–1,20 m."],
        ["gaps_2", "Elsewhere draws crowd: two nearly identical rooms, two simulations.", "Anderswo drängen sich Ziehungen: zwei fast gleiche Räume."],
        ["gaps_3", "The best draw sits in the crowded pair: no even coverage.", "Die beste Ziehung liegt im dichten Paar: keine gleichmäßige Abdeckung."],
        ["lhs", "Latin hypercube sampling: nine strata per axis, exactly one sample in each stratum.", "Latin-Hypercube-Sampling teilt jede Achse in neun Schichten, je Schicht genau eine Stichprobe."],
        ["marginals", "This vertical strip is one window-share band with exactly one design.", "Dieser senkrechte Streifen ist ein Band des Fensteranteils und enthält genau einen Entwurf."],
        ["marginals_2", "The horizontal strip also holds exactly one: each axis is covered evenly.", "Auch der waagerechte Streifen enthält genau einen: Jede Achse ist gleichmäßig abgedeckt."],
        ["cells", "But the plane has 81 cells and only nine points: empty regions remain.", "Doch die Ebene hat 81 Zellen, nur neun Punkte: leere Bereiche bleiben."],
        ["evaluate", "Until simulated, a point is only a position, with no result.", "Ohne Simulation ist ein Punkt nur eine Position, ohne Ergebnis."],
        ["evaluate_2", "Each point becomes a full building and runs through the same model.", "Jeder Punkt wird ein volles Gebäude und durchläuft dasselbe Modell."],
        ["reveal", "To judge the point sets, reveal the surface with a dense reference grid.", "Zur Beurteilung der Punktmengen legen wir die Fläche mit einem dichten Referenzraster frei."],
        ["reveal_2", "Darker means less electricity. The lowest point lies inside the range.", "Dunkler heißt weniger Strombedarf. Der tiefste Punkt liegt innen."],
        ["compare", "Compare each point set's best design with the optimum of the dense reference grid.", "Vergleiche den besten Entwurf jeder Punktmenge mit dem Optimum des dichten Referenzrasters."],
        ["compare_2", "Each best design is the best of nine, yet none reaches the optimum.", "Jeder beste Entwurf ist der beste von neun, keiner erreicht das Optimum."],
        ["compare_3", "The remaining gap is the price of having only nine simulations.", "Die verbleibende Lücke ist der Preis für nur neun Simulationen."],
        ["limits", "Sampling shows coverage and promising regions to search next.", "Sampling zeigt Abdeckung und aussichtsreiche Bereiche für die Suche."],
        ["limits_2", "A finite sample cannot prove that it found the global optimum.", "Eine endliche Stichprobe kann nicht beweisen, dass sie das globale Optimum gefunden hat."],
        ["limits_3", "The model itself still needs validation.", "Und das Modell selbst braucht noch eine Prüfung."],
        ["limits_4", "Nine simulations found a good south façade design, no proven optimum.", "Neun Simulationen fanden für diese Südfassade einen guten Entwurf, kein bewiesenes Optimum."],
    ]

    def construct(self):
        """▶️ Show equal-budget point sets through the shared local rendering interface."""
        self.present("sampling")

    def _live(self, builder, key=None):
        """♻️ Build every live widget of this scene as a snapshot-safe group."""
        return SnapshotSafeGroup(builder, key)

    def _q(self, value, unit, digits=0):
        """🔢 Live label parts for a number and its unit."""
        return _room.parts(value, unit, digits, self._german())

    def _n(self, value, digits=0):
        """🔟 One live number part in the scene language."""
        return _room.Number(self.number(value, digits))

    def _txt(self, *parts, at=(0, 0, 0), size=15, color=INK, edge="center"):
        """🔤 Place one label made of words and numbers."""
        return _room.place(_room.text(list(parts), size=size, color=color), at, edge)

    def _chip(self, *parts, at=(0, 0, 0), size=15, color=INK):
        """🔖 Place one label in a chip."""
        return _room.chip(list(parts), at, size=size, color=color)

    def _at(self, sample):
        """📍 Screen point of a design on the plane."""
        return self.plane.c2p(sample["wwr"] * 100, sample["overhang"])

    def _slot(self, x=45):
        """🎫 Screen point of the chip slot above the plane, centred on a window share."""
        return (self.plane.c2p(x, 0)[0], SLOT, 0)

    def _shape(self, kind, point, size, fill=None):
        """🔷 Set marker: square, diamond or triangle in the colour of its set."""
        spec = SETS[kind]
        style = {"color": spec["color"], "stroke_width": 3, "fill_color": fill or spec["color"], "fill_opacity": 1}
        shape = {"square": lambda: mn.Square(side_length=size, **style),
                 "diamond": lambda: mn.Square(side_length=size * 0.85, **style).rotate(mn.PI / 4),
                 "triangle": lambda: mn.Triangle(**style).scale(size * 0.62).rotate(0)}[spec["shape"]]()
        return shape.move_to(point)

    def _section(self, trackers, point, scale, season, *, person=False, dims=False, labels=False):
        """🏠 Room section of the shared design, with or without chip labels and dimension lines."""
        options = {"labels": labels, "demand": False, "person": person}
        widget = self.room_section(trackers, point, scale, season, **options)
        build = getattr(widget, "_builder", None)
        if dims or build is None:
            return widget

        def trimmed():
            group = build()
            kids = group.submobjects
            if not labels and isinstance(kids[-1], _room.LiveText):
                group.submobjects = kids[:-2]
            return group

        return self._live(trimmed, lambda: tuple(round(value, 4) for value in _room.values(trackers).values()))

    def _prepare(self):
        """🧰 Build the shared trackers, ramp, plane and the thumbnails of the three sets."""
        self.T = self.study_trackers()
        self.P = mn.ValueTracker(0.0)
        self.colour = self.ramp()
        self.full = self.colour.high
        self.sun = _model.load_study()["sun"]
        self.surface = _model.surface(41)
        self.levels = _room.nice_levels(min(min(column) for column in self.surface["electricity"]), max(max(column) for column in self.surface["electricity"]), 3)
        self.samples = {kind: _model.samples(kind) for kind in SETS}
        self.strata = _model.lhs_strata()
        self.best = {kind: None for kind in SETS}
        self.lit, self.kind = 0, None
        self.plane = mn.Axes(x_range=[*PLANE_X, 5], y_range=[FLOOR, 1.35, 0.15], x_length=PLANE_X[1] * 0.0675, y_length=(1.35 - FLOOR) * 2.2).move_to((-3.02, 1.75 - (1.35 - FLOOR) * 1.1, 0))
        self.tariff = _room.constant("economics", "tariff")
        self.marks = {kind: self._marks(kind) for kind in SETS}

    def _marks(self, kind):
        """🏘️ Ghost and evaluated thumbnail of every sample of a set, with their faders."""
        scale, marks = SETS[kind]["scale"], []
        for sample in self.samples[kind]:
            design = {"wwr": sample["wwr"], "overhang": sample["overhang"]}
            at = self._at(sample)
            ghost = self.thumbnail(design, at, scale, state="ghost")
            full = self.thumbnail(design, at, scale, ramp=self.colour, value=sample["electricity"], ring=SETS[kind]["color"])
            tag = mn.VGroup(self._chip(self._n(sample["electricity"], 1), at=at + np.array([0.02, -0.2, 0.0]), size=14)).set_z_index(15)
            marks.append({"sample": sample, "design": design, "at": at, "ghost": ghost, "full": full, "tag": tag,
                          "fade_ghost": _fader(ghost), "fade_full": _fader(full), "fade_tag": _fader(tag)})
        return marks

    def _plane_group(self):
        """📐 Axis lines, ticks, numbers, titles and the dashed design-space box."""
        ax = self.plane
        low_left, low_right, top_left = ax.c2p(PLANE_X[0], FLOOR), ax.c2p(PLANE_X[1], FLOOR), ax.c2p(PLANE_X[0], 1.35)
        group = mn.VGroup(mn.Line(low_left, low_right, color=GRAY, stroke_width=2), mn.Line(low_left, top_left, color=GRAY, stroke_width=2))
        corner_a, corner_b = ax.c2p(10, 0), ax.c2p(80, 1.2)
        box = mn.Rectangle(width=corner_b[0] - corner_a[0], height=corner_b[1] - corner_a[1]).move_to((corner_a + corner_b) / 2)
        group.add(mn.DashedVMobject(box.set_stroke(GRAY, width=1.2, opacity=0.6), num_dashes=70))
        for wwr in range(10, 90, 10):
            spot = ax.c2p(wwr, FLOOR)
            group.add(mn.Line(spot, spot + mn.UP * 0.07, color=GRAY, stroke_width=1.6), self._txt(self._n(wwr), at=spot + mn.DOWN * 0.26, size=17))
        for depth in (0, 0.3, 0.6, 0.9, 1.2):
            spot = ax.c2p(PLANE_X[0], depth)
            group.add(mn.Line(spot, spot + mn.RIGHT * 0.07, color=GRAY, stroke_width=1.6), self._txt(self._n(depth, 1), at=spot + mn.LEFT * 0.12, size=17, edge="right"))
        group.add(self._txt(self.tr("Window-to-wall ratio WWR in %", "Fensterflächenanteil WWR in %"), at=(ax.c2p(45, FLOOR)[0], low_left[1] - 0.66, 0), size=17),
                  self._txt(self.tr("Overhang depth d in m", "Vordachtiefe d in m"), at=(top_left[0] - 0.15, top_left[1] + 0.78, 0), size=17, edge="left"))
        return group

    def _bars(self, trackers, base, height, progress=None, *, labels=True, width=0.6, frozen=False, total_label=True, footer=True):
        """📊 Stacked heating, cooling and lighting electricity that fills with ``progress`` at the clip's fixed scale; empty progress reads not simulated.

        Printed parts are apportioned so they add up to the printed total; the footer names the daylight autonomy behind the lighting part and the electricity bill.
        """
        unit = height / self.full
        names = (("heating", self.tr("Heating", "Heizstrom"), HEAT), ("cooling", self.tr("Cooling", "Kühlstrom"), COOL), ("lighting", self.tr("Lighting", "Beleuchtung"), LIGHT))
        base = np.array([base[0], base[1], 0.0])
        rule = mn.VGroup(mn.Line(base + mn.LEFT * (width * 0.75), base + mn.RIGHT * (width * 0.75), color=GRAY, stroke_width=2),
                         mn.Line(base + mn.LEFT * (width / 2 + 0.25), base + mn.LEFT * (width / 2 + 0.25) + mn.UP * height, color=GRAY, stroke_width=1.2))
        corners = [base + np.array([sx * width / 2, sy * height * 0.45, 0.0]) for sx, sy in ((-1, 0), (1, 0), (1, 1), (-1, 1))]

        def build():
            found = _room.result(_room.values(trackers))
            share = 1.0 if progress is None else progress.get_value()
            group, cursor, anchors, total = mn.VGroup(), 0.0, [], 0.0
            if share <= 1e-3:
                group.add(*[_room.dashed(a, b, color=WARN, stroke_width=1.8) for a, b in zip(corners, corners[1:] + corners[:1])],
                          self._txt("?", at=base + mn.UP * height * 0.22, size=26, color=WARN),
                          self._txt(self.tr("not simulated", "Nicht simuliert"), at=base + np.array([-width / 2, height * 0.45 + 0.3, 0.0]), size=15, color=WARN, edge="left"))
                return group
            partial = [found[key] * _clip(share * 3 - index) for index, (key, _, _) in enumerate(names)]
            printed = _room.apportion(partial, 1)
            for index, (key, name, colour) in enumerate(names):
                span = partial[index] * unit
                if span > 1e-3:
                    group.add(mn.Rectangle(width=width, height=span, color=colour, stroke_width=1.2, fill_color=colour, fill_opacity=0.78).move_to(base + mn.UP * (cursor + span / 2)))
                    anchors.append((base + np.array([width / 2, cursor + span / 2, 0.0]), [name, *self._q(printed[index], "kWh/a", 1)], colour))
                cursor += span
            total = sum(printed)
            if total_label:
                group.add(self._txt(*self._q(total, "kWh/a", 1), at=base + np.array([-width / 2 - 0.1, cursor + 0.28, 0.0]), size=16, edge="left"))
            if labels and anchors:
                group.add(_room.column(anchors, x=base[0] + width / 2 + 0.6, top=base[1] + height - 0.05, bottom=base[1] + 0.1, size=13))
            if footer and share >= 0.999:
                left = base + np.array([-width / 2 - 0.4, 0.0, 0.0])
                group.add(self._txt(self.tr("Daylight autonomy", "Tageslichtautonomie"), *self._q(found["autonomy"] * 100, "%", 2), at=left + mn.DOWN * 0.3, size=14, color=LIGHT, edge="left"),
                          self._txt(self.tr("Electricity bill", "Stromkosten"), *self._q(_half_up(total * self.tariff), "CHF/a"), at=left + mn.DOWN * 0.58, size=14, color=INK, edge="left"))
            return group

        live = self._live(build)
        if frozen:
            live.clear_updaters()
        return mn.VGroup(rule, live)

    def _card(self, sample, centre, title, colour):
        """🪪 One simulated design as a card: title, design, summer room, the three electricity parts with total, daylight autonomy, window and shade, and its bar on the fixed scale."""
        cx, cy = centre
        design = {"wwr": sample["wwr"], "overhang": sample["overhang"]}
        trackers = self.study_trackers(design)
        room = self._section(trackers, (cx - 0.3, cy + 0.35, 0), 0.3, "summer")
        room.clear_updaters()
        found = _room.result(design)
        names = ((self.tr("Heating", "Heizstrom"), HEAT, "heating"), (self.tr("Cooling", "Kühlstrom"), COOL, "cooling"), (self.tr("Lighting", "Beleuchtung"), LIGHT, "lighting"))
        printed = _room.apportion([found[key] for _, _, key in names], 1)
        bars = self._bars(trackers, (cx - 1.35, cy - 2.3), 1.3, None, labels=False, width=0.4, frozen=True, total_label=False, footer=False)
        bars[0].remove(bars[0][1])
        left, group = cx - 1.05, mn.VGroup(room, bars)
        group.add(self._txt(title, at=(cx - 1.45, cy + 2.55, 0), size=17, color=colour, edge="left"),
                  self._txt("WWR", *self._q(design["wwr"] * 100, "%"), "·", "d", *self._q(design["overhang"], "m", 2), at=(cx - 1.45, cy + 2.25, 0), size=14, edge="left"))
        for index, ((name, tint, _), value) in enumerate(zip(names, printed)):
            group.add(self._txt(name, *self._q(value, "kWh/a", 1), at=(left, cy - 0.38 - 0.29 * index, 0), size=15, color=tint, edge="left"))
        group.add(mn.Line((left, cy - 1.14, 0), (left + 2.0, cy - 1.14, 0), color=GRAY, stroke_width=1.2),
                  self._txt(self.tr("Total", "Summe"), *self._q(sum(printed), "kWh/a", 1), at=(left, cy - 1.32, 0), size=15, edge="left"),
                  self._txt(self.tr("Daylight autonomy", "Tageslichtautonomie"), *self._q(found["autonomy"] * 100, "%", 2), at=(left, cy - 1.65, 0), size=13, color=LIGHT, edge="left"),
                  self._txt(self.tr("Window", "Fenster"), *self._q(found["window_area"], "m²", 1), at=(left, cy - 1.93, 0), size=14, color=GRAY, edge="left"),
                  self._txt(self.tr("Glass in shade", "Glas im Schatten"), *self._q(found["shading_summer"] * 100, "%"), at=(left, cy - 2.21, 0), size=14, color=WARN, edge="left"))
        return group

    def _meters(self):
        """🌗 Live gauges with physical ends: share of the glass in the overhang's shade per season, the daylight autonomy that decides how long the lamps burn, and the daylit depth named under each room."""
        def gauge(parts, share, colour, x, y, width, ends=("0 %", "100 %")):
            full = max(width * share, 1e-3)
            return mn.VGroup(self._txt(*parts, at=(x - 1.0, y, 0), size=17, color=colour, edge="left"),
                             mn.Rectangle(width=width, height=0.1, color=GRAY, stroke_width=1.4).move_to((x + width / 2, y - 0.21, 0)),
                             mn.Rectangle(width=full, height=0.1, stroke_width=0, fill_color=colour, fill_opacity=0.95).move_to((x + full / 2, y - 0.21, 0)),
                             self._txt(ends[0], at=(x - 0.08, y - 0.21, 0), size=17, color=INK, edge="right"),
                             self._txt(ends[1], at=(x + width + 0.08, y - 0.21, 0), size=17, color=INK, edge="left"))

        def shade(season):
            def build():
                share = _room.result(_room.values(self.T))["shading_" + season]
                return gauge([self.tr("Shaded", "Schatten"), *self._q(share * 100, "%")], share, WARN, SECTIONS[season][0] - 0.9, -0.26, 1.2)

            return self._live(build)

        def daylit(season):
            def build():
                found = _room.result(_room.values(self.T))
                return mn.VGroup(self._txt(self.tr("Daylit to", "Hell bis"), *self._q(_room.daylit_depth(found["daylight"]), "m", 1), at=(SECTIONS[season][0] - 1.2, 0.06, 0), size=17, color=_room.DAYLIGHT, edge="left"))

            return self._live(build)

        def daylight():
            def build():
                share = _room.result(_room.values(self.T))["autonomy"]
                return gauge([self.tr("Daylight", "Tageslicht"), *self._q(share * 100, "%", 2), "→", self.tr("lamps on", "Lampen an"), *self._q((1 - share) * 100, "%", 2)],
                             share, LIGHT, PANEL_X + 0.75, -0.76, 3.0)

            return self._live(build)

        return {"winter": shade("winter"), "summer": shade("summer"), "daylit_winter": daylit("winter"), "daylit_summer": daylit("summer"), "daylight": daylight()}

    def _panel(self, trackers, x=PANEL_X, size=15):
        """🧮 The substituted equations behind the bar at one decimal: demand ÷ efficiency, lamp power · hours · share on, the sum and the bill; every number is live from the model and the printed arithmetic closes."""
        names = ("heating", "cooling", "lighting")
        constants = {"heating": (HEAT, _room.systems()["cop"], "COP", self.tr("heat", "Wärme")), "cooling": (COOL, _room.systems()["eer"], "EER", self.tr("cold", "Kälte"))}

        def printed(found, part):
            return dict(zip(names, _room.apportion([found[key] for key in names], 1)))[part]

        def conversion(part):
            colour, factor, name, word = constants[part]
            soft = _room.muted(colour)

            def rows(design, found):
                sign = "=" if _half_up(_half_up(found[part + "_demand"], 1) / factor, 1) == printed(found, part) else "≈"
                return [[self.live(part + "_demand", soft, 1, "kWh/a"), (word, soft), "÷", (name, INK), self.live(factor, INK, 1), sign, self.live(part, colour, 1, "kWh/a", parts_of=names)]]

            return rows

        def lighting(design, found):
            kilowatt = _room.constant("lighting", "power_density") * _room.constant("room", "width") * _room.constant("room", "depth") / 1000
            hours, on = _room.constant("lighting", "hours"), 100 * (1 - found["autonomy"])
            sign = "=" if _half_up(_half_up(kilowatt, 3) * hours * _half_up(on, 2) / 100, 1) == printed(found, "lighting") else "≈"
            return [[(self.tr("Lamps", "Lampen"), LIGHT), self.live(kilowatt, INK, 3, "kW"), "·", self.live(hours, INK, 0, "h/a"), "·", self.live(on, LIGHT, 2, "%"), sign,
                     self.live("lighting", LIGHT, 1, "kWh/a", parts_of=names)]]

        def total(design, found):
            return sum(_room.apportion([found[key] for key in names], 1))

        def summed(design, found):
            return [[self.live("heating", HEAT, 1, parts_of=names), "+", self.live("cooling", COOL, 1, parts_of=names), "+", self.live("lighting", LIGHT, 1, parts_of=names), "=", self.live(total, INK, 1, "kWh/a")]]

        def bill(design, found):
            return [[(self.tr("Bill", "Kosten"), INK), self.live(total, INK, 1, "kWh/a"), "×", self.live(self.tariff, INK, 2, "CHF/kWh"), "=",
                     self.live(lambda d, f: _half_up(total(d, f) * self.tariff), INK, 0, "CHF/a")]]

        return [self.formula(trackers, row, (x, y, 0), size=size, align="left", columns=False, width=4.95)
                for row, y in zip([conversion("heating"), conversion("cooling"), lighting, summed, bill], PANEL_ROWS)]

    def _pips(self):
        """🟩 Nine budget pips of the running set that light with every finished simulation."""
        def build():
            colour = SETS[self.kind]["color"] if self.kind else GRAY
            group = mn.VGroup(self._txt(self.tr("Run", "Lauf"), self._n(self.lit), "/ 9", at=(BOARD_X + 0.3, -0.68, 0), size=15, edge="left"))
            for index in range(9):
                group.add(mn.Square(side_length=0.15, color=colour, stroke_width=1.6, fill_color=colour, fill_opacity=1 if index < self.lit else 0).move_to((BOARD_X + 0.08 + 0.24 * index, -0.98, 0)))
            return group

        return self._live(build)

    def _row(self, kind, y):
        """🏁 Scoreboard row of one set: marker, name and best electricity so far."""
        def build():
            spec = SETS[kind]
            group = mn.VGroup(self._shape(kind, (BOARD_X + 0.1, y, 0), 0.2), self._txt(self.tr(spec["en"], spec["de"]), at=(BOARD_X + 0.32, y, 0), size=15, edge="left"))
            if self.best[kind] is not None:
                group.add(self._txt(self._n(self.best[kind], 1), at=(6.85, y, 0), size=16, edge="right"))
            return group

        return self._live(build)

    def _board(self):
        """🏆 Pips, column header and one row per set."""
        header = self._txt(self.tr("Best so far in kWh/a", "Bester bisher in kWh/a"), at=(6.85, -1.3, 0), size=14, color=INK, edge="right")
        rows = {kind: self._row(kind, y) for kind, y in zip(SETS, (-1.65, -2.05, -2.45))}
        return {"pips": self._pips(), "header": header, "rows": rows}

    def _sections(self):
        """🏡 Winter and summer room sections with their sun captions."""
        group = mn.VGroup()
        captions = {}
        for season, (x, y) in SECTIONS.items():
            group.add(self._section(self.T, (x, y, 0), SECTION_SCALE, season))
            altitude = self.sun[season + "_altitude"]
            captions[season] = self._txt(self.tr("Winter sun", "Wintersonne") if season == "winter" else self.tr("Summer sun", "Sommersonne"),
                                         *self._q(altitude, "°"), at=(x - 0.35, 2.55, 0), size=17, color=SUN)
            group.add(captions[season])
        return group, captions

    def _design_chip(self, at=CHIP_TOP):
        """🎚️ Live chip with the window share and overhang depth of the room on screen."""
        return self._live(lambda: mn.VGroup(self._chip("WWR", *self._q(self.T["wwr"].get_value() * 100, "%"), "·", "d", *self._q(self.T["overhang"].get_value(), "m", 2), at=at, size=16)))

    def _hold(self, design, seconds=1.0):
        """⏳ Move the room to a design that is not simulated and let the bar empty."""
        start = self.P.get_value()
        return mn.AnimationGroup(self.set_design(self.T, design, seconds),
                                 Drive(lambda alpha: self.P.set_value(start * (1 - _smooth(alpha * 2))), run_time=seconds, rate_func=mn.linear))

    def _probe(self, design, at):
        """❓ Ghost section with a question mark for a design nobody has simulated."""
        return mn.VGroup(self.thumbnail(design, at, 0.15, state="ghost"), self._txt("?", at=np.array(at) + mn.RIGHT * 0.12, size=24, color=WARN))

    def _sweep(self, kind, order, *, tags=False):
        """⏲️ Evaluate the designs ``order`` of a set one after another: room, rays, bar fill, colour, pip and best so far."""
        marks, trackers, share = self.marks[kind], self.T, self.P
        start, lit0, best0 = _room.values(trackers), self.lit, self.best[kind]
        designs = [marks[index]["design"] for index in order]
        spec = SETS[kind]
        cursor = mn.Rectangle(width=4.8 * spec["scale"] + 0.1, height=4.0 * spec["scale"] + 0.1, color=INK, stroke_width=2.6).set_z_index(20)
        count = len(order)
        self.add(cursor)

        def update(alpha):
            clock = alpha * count
            step = min(int(clock), count - 1)
            local = clock - step
            before = designs[step - 1] if step else start
            move = _smooth(local / 0.3)
            for key in ("wwr", "overhang"):
                trackers[key].set_value(before.get(key, start[key]) + (designs[step][key] - before.get(key, start[key])) * move)
            share.set_value(_clip((local - 0.3) / 0.5))
            done, best = 0, best0
            for position, index in enumerate(order):
                shown = _clip(_clip(clock - position - 0.8) / 0.15)
                marks[index]["fade_ghost"](1 - shown)
                marks[index]["fade_full"](shown)
                if tags:
                    marks[index]["fade_tag"](shown)
                if shown >= 1:
                    done += 1
                    value = marks[index]["sample"]["electricity"]
                    best = value if best is None else min(best, value)
            self.lit, self.best[kind] = lit0 + done, best
            cursor.move_to(marks[order[step]]["at"])
            cursor.set_stroke(opacity=0 if alpha >= 1 else 1)

        return Drive(update, rate_func=mn.linear), cursor

    def _visit(self, designs, on_step=None, *, empty=True, lead=0.08):
        """🛰️ Move the shared room through ``designs`` one after another; the bar empties (not simulated) while it travels and ``on_step(index, local)`` follows along."""
        start, first = _room.values(self.T), self.P.get_value()

        def update(alpha):
            clock = alpha * len(designs)
            step = min(int(clock), len(designs) - 1)
            before = designs[step - 1] if step else start
            move = _smooth((clock - step) / 0.55)
            for key in ("wwr", "overhang"):
                self.T[key].set_value(before[key] + (designs[step][key] - before[key]) * move)
            if empty:
                self.P.set_value(first * (1 - _smooth(alpha / lead)))
            if on_step is not None:
                on_step(step, clock - step, alpha)

        return Drive(update, rate_func=mn.linear)

    def _walk(self, path):
        """🚶 Move the shared design through waypoints of window share and overhang depth."""
        def update(alpha):
            position = alpha * (len(path) - 1)
            index = min(int(position), len(path) - 2)
            ease = _smooth(position - index)
            for slot, key in enumerate(("wwr", "overhang")):
                self.T[key].set_value(path[index][slot] + (path[index + 1][slot] - path[index][slot]) * ease)

        return Drive(update, rate_func=mn.linear)

    def _slider(self, key, name, low, high, unit, digits, scale, y, colour):
        """🎛️ Parameter slider whose thumb follows a shared tracker."""
        x0, x1 = -5.5, -1.3
        static = mn.VGroup(mn.Line((x0, y, 0), (x1, y, 0), color=GRAY, stroke_width=3), self._txt(name, at=(x0, y + 0.42, 0), size=17, color=colour, edge="left"),
                           self._txt(*self._q(low, unit, digits), at=(x0, y - 0.3, 0), size=14, color=GRAY),
                           self._txt(*self._q(high, unit, digits), at=(x1, y - 0.3, 0), size=14, color=GRAY))

        def build():
            value = self.T[key].get_value() * scale
            x = x0 + (x1 - x0) * (value - low) / (high - low)
            return mn.VGroup(mn.Dot((x, y, 0), radius=0.11, color=colour), self._chip(*self._q(value, unit, digits), at=(x, y + 0.32, 0), size=14, color=colour))

        return mn.VGroup(static, self._live(build))

    def _key(self, at):
        """🗝️ Colour key of the stacked electricity bars."""
        group, x = mn.VGroup(), 0.0
        for word, colour in ((self.tr("Heating", "Heizstrom"), HEAT), (self.tr("Cooling", "Kühlstrom"), COOL), (self.tr("Lighting", "Beleuchtung"), LIGHT)):
            label = self._txt(word, at=(x + 0.22, 0, 0), size=14, color=colour, edge="left")
            group.add(mn.Square(side_length=0.16, stroke_width=0, fill_color=colour, fill_opacity=0.85).move_to((x + 0.08, 0, 0)), label)
            x += 0.22 + label.width + 0.35
        return group.move_to(at)

    def _tile(self, kind, sample, centre, title, *, shape=True, delta=None, unit=0.02):
        """🧩 Compare tile: set title, the best room of the set, its bar, electricity and the gap to the optimum on a shared scale."""
        cx, cy = centre
        design = {"wwr": sample["wwr"], "overhang": sample["overhang"]}
        trackers = self.study_trackers(design)
        room = self._section(trackers, (cx - 0.35, cy - 0.2, 0), 0.19, "summer", dims=False)
        room.clear_updaters()
        bars = self._bars(trackers, (cx + 1.0, cy - 0.78), 1.1, None, labels=False, width=0.4, frozen=True, total_label=False, footer=False)
        head = mn.VGroup()
        if shape:
            head.add(self._shape(kind, (cx - 1.38, cy + 1.1, 0), 0.2))
        head.add(self._txt(title, at=(cx - 1.16, cy + 1.1, 0), size=16, edge="left"),
                 self._txt("WWR", *self._q(design["wwr"] * 100, "%"), "·", "d", *self._q(design["overhang"], "m", 2), at=(cx - 1.45, cy + 0.85, 0), size=14, edge="left"))
        facts = mn.VGroup(self._txt(*self._q(sample["electricity"], "kWh/a", 1), at=(cx - 1.45, cy - 0.85, 0), size=17, edge="left"))
        extra = mn.VGroup()
        if delta is not None:
            gap, share = delta
            extra.add(self._txt("Δ", *self._q(gap, "kWh/a", 1), "·", *self._q(share, "%", 1), at=(cx - 1.45, cy - 1.1, 0), size=14, color=WARN, edge="left"),
                      mn.Rectangle(width=1.4, height=0.1, color=GRAY, stroke_width=1.2).move_to((cx - 1.45 + 0.7, cy - 1.3, 0)),
                      mn.Rectangle(width=max(gap * unit, 0.03), height=0.1, stroke_width=0, fill_color=WARN, fill_opacity=0.95).move_to((cx - 1.45 + max(gap * unit, 0.03) / 2, cy - 1.3, 0)))
        return {"base": mn.VGroup(head, room, bars, facts), "extra": extra}

    def present_sampling(self):
        """🎞️ Tell the sampling story: example, grid, growth, random, Latin hypercube, true surface, comparison, limits."""
        self._prepare()
        self._intro()
        self._grid()
        self._growth()
        self._random()
        self._lhs()
        self._reveal()
        self._compare()
        self._limits()

    def _intro(self):
        """🏛️ Introduce the two design choices, the map, one simulation and the budget."""
        T = self.T
        elevation = self.facade(T, (-3.4, 1.0, 0), 0.52, "summer", labels=True)
        single = self._section(T, (4.95, 0.45, 0), 0.48, "summer", person=True, dims=True, labels=True)
        picture_caption = self._txt(self.tr("Section through the façade, summer sun", "Schnitt durch die Fassade, Sommersonne"), at=(4.4, 3.45, 0), size=16, color=INK)
        sliders = mn.VGroup(self._slider("wwr", "WWR", 10, 80, "%", 0, 100, -1.15, _visuals.DESIGN),
                            self._slider("overhang", self.tr("Overhang d", "Vordach d"), 0, 1.2, "m", 1, 1, -2.2, _visuals.WARN))
        self.play(mn.FadeIn(elevation), mn.FadeIn(single), mn.FadeIn(picture_caption), mn.FadeIn(sliders), run_time=0.5)
        self.step("parameters", self._walk([(0.45, 0.6), (0.8, 0.6), (0.1, 0.6), (0.45, 0.6), (0.45, 1.2), (0.45, 0.0), (0.45, 0.6)]), seconds=5)
        self.plane_group = self._plane_group()
        marker = self.live_marker(self.plane, T, "wwr", lambda design: design["overhang"], x_label="WWR", x_unit="%", y_label="d", y_unit="m", y_digits=1, x_scale=100)
        precise = self.live_marker(self.plane, T, "wwr", lambda design: design["overhang"], x_label="WWR", x_unit="%", y_label="d", y_unit="m", y_digits=2, x_scale=100)
        self.play(mn.FadeOut(elevation), mn.FadeOut(sliders), mn.FadeOut(picture_caption), mn.FadeIn(self.plane_group), mn.FadeIn(marker), run_time=0.5)
        self.step("parameters_2", self._walk([(0.45, 0.6), (0.2, 0.15), (0.72, 1.0), (0.45, 0.6)]), seconds=4)
        self.sections, captions = self._sections()
        self.bars = self._bars(T, BAR_BASE, BAR_HEIGHT, self.P)
        self.bars_alone = self._bars(T, ALONE_BASE, BAR_HEIGHT, self.P, labels=False, footer=False)
        self.design_chip = self._design_chip()
        meters = self.meters = self._meters()
        note = self._txt(self.tr("Simplified teaching model · Zurich-like climate", "Vereinfachtes Jahresmodell · Klima wie Zürich"), "·", *self._q(self.tariff, "CHF/kWh", 2), at=(6.85, 2.95, 0), size=17, color=INK, edge="right")
        self.play(mn.FadeOut(single), mn.FadeOut(marker), mn.FadeIn(precise), mn.FadeIn(self.sections), mn.FadeIn(note), mn.FadeIn(self.design_chip), run_time=0.5)
        self.step("run", mn.AnimationGroup(mn.Indicate(captions["winter"], color=SUN), mn.Indicate(captions["summer"], color=SUN), lag_ratio=1),
                  *[mn.FadeIn(meters[key]) for key in meters], seconds=4)
        self.play(mn.FadeIn(self.bars_alone), run_time=0.5)
        s = SECTION_SCALE
        lamps = mn.Rectangle(width=s * 3.4, height=s * 1.1, stroke_width=2.6, color=LIGHT).move_to((SECTIONS["winter"][0] + s * (2.6 - 3), SECTIONS["winter"][1] + s * 1.0, 0))
        cues = [mn.SurroundingRectangle(captions["winter"], color=HEAT, buff=0.1, stroke_width=2.6), mn.SurroundingRectangle(captions["summer"], color=COOL, buff=0.1, stroke_width=2.6), lamps]
        for cue in cues:
            cue.set_stroke(opacity=0)
        self.add(*cues)

        def fill(alpha):
            self.P.set_value(alpha)
            for index, cue in enumerate(cues):
                cue.set_stroke(opacity=_clip(1 - abs(alpha * 3 - index - 0.5) * 2.2))

        panel = self.panel = self._panel(T)
        for widget in panel:
            widget.update(0)
        self.step("run_2", Drive(fill, rate_func=mn.linear), mn.LaggedStart(*[mn.FadeIn(widget) for widget in panel[:4]], lag_ratio=0.9), seconds=6)
        self.remove(*cues)
        centre = self.marks["grid"][4]
        centre["fade_full"](1)
        centre["fade_tag"](1)
        self.add(centre["full"], centre["tag"])
        self.strip = self.ramp_strip(self.colour, (-1.55, 2.7, 0), 2.6, size=15, ticks=self.levels)
        self.best["grid"], self.lit, self.kind = centre["sample"]["electricity"], 1, "grid"
        self.step("run_3", mn.FadeIn(self.strip), mn.FadeIn(panel[4]), mn.FadeOut(note), mn.FadeOut(precise), mn.Indicate(centre["full"], color=INK), seconds=2.5)
        self.board = self._board()
        self.step("budget", mn.FadeIn(self.board["pips"]), mn.FadeIn(self.bars), mn.FadeOut(self.bars_alone), *[mn.FadeOut(widget) for widget in [*panel, *meters.values()]], seconds=1.5)

    def _grid(self):
        """🔲 Evaluate the full 3 × 3 grid one design at a time and ring the best one."""
        marks, board = self.marks["grid"], self.board
        spec = SETS["grid"]
        levels = _model.levels()
        guides = mn.VGroup()
        for wwr in levels["wwr"]:
            guides.add(mn.DashedLine(self.plane.c2p(wwr * 100, 0), self.plane.c2p(wwr * 100, 1.2), color=GRAY, stroke_width=1.2, stroke_opacity=0.5),
                       self._chip(*self._q(wwr * 100, "%"), at=(self.plane.c2p(wwr * 100, 1.2)[0], 2.1, 0), size=14, color=spec["color"]))
        for depth in levels["overhang"]:
            guides.add(mn.DashedLine(self.plane.c2p(10, depth), self.plane.c2p(80, depth), color=GRAY, stroke_width=1.2, stroke_opacity=0.5))
        for index, item in enumerate(marks):
            item["fade_ghost"](0 if index == 4 else 1)
        ghosts = [item["ghost"] for index, item in enumerate(marks) if index != 4]
        row = board["rows"]["grid"]
        order = [0, 1, 2, 3, 5, 6, 7, 8]
        visit = mn.Rectangle(width=4.8 * spec["scale"] + 0.1, height=4.0 * spec["scale"] + 0.1, color=INK, stroke_width=2.6).set_z_index(20)
        self.add(visit)

        def follow(step, local, alpha):
            visit.move_to(marks[order[step]]["at"])
            visit.set_stroke(opacity=0 if alpha >= 1 else 1)

        self.step("grid", mn.FadeIn(guides), mn.LaggedStart(*[mn.FadeIn(ghost) for ghost in ghosts], lag_ratio=0.15), mn.FadeIn(board["header"]), mn.FadeIn(row),
                  self._visit([marks[index]["design"] for index in order], follow), seconds=6)
        self.remove(visit)
        first, cursor = self._sweep("grid", [0, 1, 2, 3], tags=True)
        self.add(*[item["full"] for item in marks], *[item["tag"] for item in marks])
        self.step("grid_2", first, seconds=6)
        second, cursor_b = self._sweep("grid", [5, 6, 7, 8], tags=True)
        self.step("grid_3", second, seconds=6)
        self.remove(cursor, cursor_b)
        best = min(marks, key=lambda item: item["sample"]["electricity"])
        worst = max(marks, key=lambda item: item["sample"]["electricity"])
        cards = mn.VGroup(self._card(worst["sample"], (2.2, 0.45), self.tr("Worst of nine", "Schlechtester der neun"), WARN),
                          self._card(best["sample"], (5.4, 0.45), self.tr("Best of nine", "Bester der neun"), GOOD))
        self.rings = {"grid": mn.Rectangle(width=4.8 * spec["scale"] + 0.12, height=4.0 * spec["scale"] + 0.12, color=GOOD, stroke_width=4).move_to(best["at"]).set_z_index(21)}
        away = [self.sections, self.bars, self.design_chip, board["pips"], board["header"], row]
        self.step("grid_4", mn.FadeIn(self.rings["grid"]), mn.FadeIn(cards), *[mn.FadeOut(item) for item in away], self.set_design(self.T, best["design"], 1.2), seconds=2.5)
        self.guides, self.cards = guides, cards

    def _growth(self):
        """📈 Add orientation and insulation: levels multiply from 9 to 108 simulations."""
        levels = _model.levels()
        german = self._german()
        counts = [len(levels[key]) for key in ("wwr", "overhang", "orientation", "insulation")]
        total = int(np.prod(counts))
        self.clear_stage()
        titles = {"wwr": self.tr("Window share", "Fensteranteil"), "overhang": self.tr("Overhang", "Vordach"),
                  "orientation": self.tr("Orientation", "Ausrichtung"), "insulation": self.tr("Insulation", "Dämmung")}
        units = {"wwr": ("%", 100, 0), "overhang": ("m", 1, 1), "orientation": ("°", 1, 0), "insulation": ("cm", 100, 0)}
        columns = []
        for slot, key in enumerate(("wwr", "overhang", "orientation", "insulation")):
            x0 = -5.1 + slot * 3.4
            group = mn.VGroup(self._txt(titles[key], at=(x0, 2.4, 0), size=17, color=FACTORS[key]))
            unit, factor, digits = units[key]
            for position, value in enumerate(levels[key]):
                x = x0 + (position - (len(levels[key]) - 1) / 2) * 0.82
                if key == "orientation":
                    glyph = self._orientation_glyph(value, (x, 1.5, 0))
                    name = [_room.cardinal(value, german)]
                else:
                    design = {"wwr": value} if key == "wwr" else {"overhang": value} if key == "overhang" else {"insulation": value}
                    glyph = self.thumbnail(design, (x, 1.5, 0), 0.15, ramp=lambda _: PANEL)
                    name = []
                group.add(glyph, self._txt(*self._q(value * factor, unit, digits), *name, at=(x, 0.82, 0), size=14, color=INK))
            group.add(self._txt(self._n(counts[slot]), self.tr("levels", "Stufen"), at=(x0, 0.4, 0), size=15, color=FACTORS[key]))
            columns.append(group)
        blocks, pitch = [], 0.72
        origin = np.array([-4.75, -0.6, 0.0])
        names = mn.VGroup()
        for orient in range(counts[2]):
            names.add(self._txt(_room.cardinal(levels["orientation"][orient], german), at=origin + np.array([orient * pitch + 0.17, 0.32, 0.0]), size=14, color=FACTORS["orientation"]))
            for insul in range(counts[3]):
                blocks.append(mn.VGroup(*[mn.Square(side_length=0.13, stroke_width=0, fill_color=GRAY, fill_opacity=0.85)
                                          .move_to(origin + np.array([orient * pitch + (i % 3) * 0.17, -insul * pitch - (i // 3) * 0.17, 0])) for i in range(9)]))
        for insul in range(counts[3]):
            names.add(self._txt(*self._q(levels["insulation"][insul] * 100, "cm"), at=origin + np.array([-0.3, -insul * pitch - 0.17, 0.0]), size=14, color=FACTORS["insulation"], edge="right"))
        self._count = 9
        legend = mn.VGroup(*[mn.Square(side_length=0.13, stroke_width=0, fill_color=GOOD, fill_opacity=0.95).move_to((-0.35 + (i % 3) * 0.17, -0.12 - (i // 3) * 0.17, 0)) for i in range(9)])
        factors_label = self._live(lambda: mn.VGroup(self._txt(self._n(self._count), self.tr("simulations", "Simulationen"), at=(0.3, -0.3, 0), size=28, color=INK, edge="left")))
        tokens = mn.VGroup()
        for slot, key in enumerate(("wwr", "overhang", "orientation", "insulation")):
            tokens.add(self._txt(self._n(counts[slot]), size=30, color=FACTORS[key]), self._txt("·" if slot < 3 else "=", size=30))
        tokens.add(self._txt(self._n(total), size=30, color=INK))
        tokens.arrange(mn.RIGHT, buff=0.22).move_to((2.9, -1.2, 0))
        others = list(blocks)
        fades = [_fader(block) for block in others]
        for fade in fades:
            fade(0.3)
        token_fade = [_fader(token) for token in tokens]
        self.add(*blocks, tokens)
        self.step("growth", mn.LaggedStart(*[mn.FadeIn(column) for column in columns], lag_ratio=0.3), mn.FadeIn(names), mn.FadeIn(legend), mn.FadeIn(factors_label), seconds=4)

        def grow(alpha):
            for position, fade in enumerate(fades):
                fade(0.3 + 0.7 * _clip(alpha * (len(fades) + 4) - position - 2))
            for position, fade in enumerate(token_fade):
                fade(_clip(alpha * 6 - position * 0.6))
            self._count = 9 + round((total - 9) * _clip(alpha * 1.1))

        self.step("growth_2", Drive(grow, rate_func=mn.linear), seconds=4)
        over = total - 9
        notes = mn.VGroup(self._chip(self._n(over), self.tr("over budget", "über dem Budget"), at=(-0.6, -0.85, 0), size=17, color=WARN),
                          self._chip(self._n(total / 9), "×", self.tr("budget", "Budget"), at=(-0.6, -1.55, 0), size=17, color=WARN))
        self.step("growth_3", mn.LaggedStart(*[block.animate.set_fill(WARN, opacity=0.9) for block in others], lag_ratio=0.05), mn.FadeIn(notes), seconds=3)
        self.growth_items = (columns, blocks, legend, factors_label, tokens, notes)

    def _orientation_glyph(self, bearing, point):
        """🧭 Mini plan of the room turned to a bearing, glazed façade highlighted and a north tick on the ring."""
        ring = mn.Circle(radius=0.34, color=GRAY, stroke_width=1.4)
        north = mn.Line((0, 0.34, 0), (0, 0.44, 0), color=INK, stroke_width=2)
        room = mn.Rectangle(width=0.4, height=0.3, color=INK, stroke_width=1.8, fill_color=PANEL, fill_opacity=1)
        facade = mn.Line((-0.2, 0.15, 0), (0.2, 0.15, 0), color=_visuals.DATA, stroke_width=5)
        group = mn.VGroup(room, facade).rotate(-np.radians(bearing), about_point=np.zeros(3))
        return mn.VGroup(ring, north, group).move_to(point)

    def _random(self):
        """🎰 Draw nine recorded random pairs while the room visits each draw, expose the gap and the cluster, then evaluate them."""
        spec = SETS["random"]
        self.clear_stage()
        self.lit, self.kind = 0, "random"
        marks = self.marks["random"]
        left, pitch = 2.55, 0.5
        table = mn.VGroup()
        for position, mark in enumerate(marks):
            x = left + pitch * position
            table.add(self._txt(self._n(position + 1), at=(x, -0.7, 0), size=14, color=spec["color"]),
                      self._txt(self._n(mark["sample"]["wwr"] * 100), at=(x, -1.1, 0), size=14),
                      self._txt(self._n(mark["sample"]["overhang"], 2), at=(x, -1.5, 0), size=14))
        heads = mn.VGroup(self._txt("#", at=(1.0, -0.7, 0), size=15, color=spec["color"], edge="left"),
                          self._txt(self.tr("WWR in %", "Fenster in %"), at=(1.0, -1.1, 0), size=14, edge="left"),
                          self._txt(self.tr("d in m", "Vordach in m"), at=(1.0, -1.5, 0), size=14, edge="left"))
        recorded = self._chip(self.tr("Recorded with the study", "Mit der Studie gespeichert"), at=(4.5, -2.1, 0), size=15, color=spec["color"])
        column = mn.Rectangle(width=pitch, height=1.25, color=spec["color"], stroke_width=2.4).set_z_index(20)
        visit = mn.Rectangle(width=4.8 * spec["scale"] + 0.1, height=4.0 * spec["scale"] + 0.1, color=INK, stroke_width=2.6).set_z_index(20)
        boxes = [(mark["at"], np.array([0.26, 0.22, 1.0])) for mark in marks]
        spots = []
        for position, mark in enumerate(marks):
            options = [mark["at"] + np.array([dx, dy, 0.0]) for dx, dy in ((0.42, 0.34), (-0.42, 0.34), (0.42, -0.34), (-0.42, -0.34), (0.0, 0.5), (0.0, -0.5))]
            free = [spot for spot in options if not any(np.all(np.abs(centre - spot) < half + np.array([0.15, 0.15, 0.0])) for centre, half in boxes)
                    and not any(np.all(np.abs(other - spot) < np.array([0.4, 0.3, 1.0])) for other in spots)]
            spots.append(free[0] if free else options[0])
        badges = [self._chip(self._n(position + 1), at=spots[position], size=13, color=spec["color"]).set_z_index(16) for position in range(len(marks))]
        self.P.set_value(0.0)
        self.play(mn.FadeIn(self.plane_group), mn.FadeIn(self.strip), mn.FadeIn(self.sections), mn.FadeIn(self.design_chip), run_time=0.5)
        self.add(*[mark["ghost"] for mark in marks], table, heads, recorded, column, visit, *badges)
        fade_recorded, fade_heads = _fader(recorded), _fader(heads)
        faders = [_fader(part) for part in table]
        fade_badges = [_fader(badge) for badge in badges]
        for position, mark in enumerate(marks):
            mark["fade_ghost"](0.0)

        def follow(step, local, alpha):
            for position, mark in enumerate(marks):
                shown = 1.0 if position < step else _clip(local / 0.45) if position == step else 0.0
                mark["fade_ghost"](shown)
                mark["ghost"].move_to(mark["at"] + mn.UP * 0.8 * (1 - _smooth(shown)))
                fade_badges[position](shown)
                for part in range(position * 3, position * 3 + 3):
                    faders[part](shown)
            fade_heads(_clip(alpha * 12))
            fade_recorded(_clip((alpha - 0.9) / 0.1))
            column.move_to((left + pitch * step, -1.1, 0))
            visit.move_to(marks[step]["at"])
            column.set_stroke(opacity=0 if alpha >= 1 else 1)
            visit.set_stroke(opacity=0 if alpha >= 1 else 1)

        self.step("random", self._visit([mark["design"] for mark in marks], follow), seconds=9)
        self.remove(column, visit)
        board = self.board
        self.play(mn.FadeOut(table), mn.FadeOut(recorded), mn.FadeOut(heads), mn.FadeIn(self.bars), mn.FadeIn(board["pips"]), mn.FadeIn(board["header"]),
                  mn.FadeIn(board["rows"]["grid"]), mn.FadeIn(board["rows"]["random"]), run_time=0.5)
        sweep, cursor = self._sweep("random", list(range(9)), tags=False)
        self.add(*[mark["full"] for mark in marks])
        self.step("random_3", sweep, seconds=6)
        self.remove(cursor)
        diag = _model.sampling_diagnostics("random")
        box = diag["largest_empty_box"]
        low, high = self.plane.c2p(box["wwr"][0] * 100, box["overhang"][0]), self.plane.c2p(box["wwr"][1] * 100, box["overhang"][1])
        area = mn.DashedVMobject(mn.Rectangle(width=high[0] - low[0], height=high[1] - low[1], color=WARN, stroke_width=3).move_to((low + high) / 2), num_dashes=60)
        area.set_z_index(-1)
        label = self._chip("WWR", *self._q(box["wwr"][0] * 100, "%"), "–", *self._q(box["wwr"][1] * 100, "%"), "·", "d", *self._q(box["overhang"][0], "m", 2), "–", *self._q(box["overhang"][1], "m", 2),
                           at=self._slot(), size=14, color=WARN)
        centre = {"wwr": _half_up((box["wwr"][0] + box["wwr"][1]) / 2, 2), "overhang": _half_up((box["overhang"][0] + box["overhang"][1]) / 2, 2)}
        probe = self._probe(centre, self._at(centre))
        self.step("gaps", mn.Create(area), mn.FadeIn(label), mn.FadeIn(probe), *[mn.FadeOut(badge) for badge in badges], self._hold(centre, 1.5), seconds=2.5)
        first, second, gap = diag["closest_pair"][0], diag["closest_pair"][1], diag["closest_delta"]
        pair = [marks[first], marks[second]]
        middle = (pair[0]["at"] + pair[1]["at"]) / 2
        ring = mn.Circle(radius=float(np.linalg.norm(pair[0]["at"] - pair[1]["at"])) / 2 + 0.3, color=WARN, stroke_width=3).move_to(middle).set_z_index(21)
        difference = round(abs(_half_up(pair[0]["sample"]["electricity"], 1) - _half_up(pair[1]["sample"]["electricity"], 1)), 1)
        callout = self._chip("Δ WWR", *self._q(gap["wwr"] * 100, self.tr("pp", "%-Pkt.")), "·", "Δ d", *self._q(gap["overhang"], "m", 2), "·", "Δ", *self._q(difference, "kWh/a", 1),
                             at=self._slot(), size=14, color=WARN)
        cards = mn.VGroup(*[self._card(item["sample"], (centre_x, 0.45), f"{self.tr('Draw', 'Ziehung')} {index + 1}", WARN)
                            for item, index, centre_x in zip(pair, (first, second), (2.2, 5.4))])
        away = [self.sections, self.bars, self.design_chip, board["pips"], board["header"], board["rows"]["grid"], board["rows"]["random"]]
        self.step("gaps_2", mn.FadeOut(probe), mn.FadeOut(area), mn.FadeOut(label), mn.FadeIn(ring), mn.FadeIn(callout), mn.FadeIn(cards), *[mn.FadeOut(item) for item in away], seconds=3)
        best = min(marks, key=lambda item: item["sample"]["electricity"])
        self.rings["random"] = mn.Rectangle(width=4.8 * spec["scale"] + 0.12, height=4.0 * spec["scale"] + 0.12, color=GOOD, stroke_width=4).move_to(best["at"]).set_z_index(21)
        winner = self._chip(self.tr("Best of nine draws:", "Bester von neun Ziehungen:"), *self._q(best["sample"]["electricity"], "kWh/a", 1), at=self._slot(), size=14, color=GOOD)
        self.P.set_value(1.0)
        back = [self.sections, self.bars, self.design_chip, board["pips"], board["header"], board["rows"]["grid"], board["rows"]["random"]]
        self.step("gaps_3", mn.FadeIn(self.rings["random"]), mn.FadeOut(ring), mn.FadeOut(callout), mn.FadeOut(cards), mn.FadeIn(winner), *[mn.FadeIn(item) for item in back], self.set_design(self.T, best["design"], 1.2), seconds=2)
        self.random_items = (area, label, ring, callout, winner, cards)

    def _lhs(self):
        """🧊 Latin hypercube strata, strips and cells, then the evaluation of the nine samples."""
        spec = SETS["lhs"]
        marks, strata = self.marks["lhs"], self.strata
        self.lit, self.kind = 0, "lhs"
        edges_x, edges_y = [value * 100 for value in strata["wwr"]], strata["overhang"]
        lines = mn.VGroup().set_z_index(-2)
        for value in edges_x:
            lines.add(mn.Line(self.plane.c2p(value, 0), self.plane.c2p(value, 1.2), color=GRAY, stroke_width=1.1, stroke_opacity=0.55))
        for value in edges_y:
            lines.add(mn.Line(self.plane.c2p(10, value), self.plane.c2p(80, value), color=GRAY, stroke_width=1.1, stroke_opacity=0.55))
        step_x, step_y = edges_x[1] - edges_x[0], edges_y[1] - edges_y[0]
        width_chip = self._chip(self._n(strata["count"]), self.tr("strata per axis:", "Schichten je Achse:"), *self._q(step_x, self.tr("pp", "%-Pkt."), 1), "·", *self._q(step_y, "m", 2), "·", self.tr("one tick per sample", "ein Strich je Stichprobe"),
                                at=self._slot(), size=14, color=spec["color"])
        old = [self.guides, *self.random_items, self.rings["grid"], self.rings["random"]]
        gone = [mark[part] for mark in self.marks["grid"] + self.marks["random"] for part in ("full", "tag")]
        ticks = mn.VGroup()
        for index, mark in enumerate(marks):
            i, j = mark["sample"]["strata_index"]
            ticks.add(mn.Square(side_length=0.07, stroke_width=0, fill_color=spec["color"], fill_opacity=1).move_to((self.plane.c2p(edges_x[i] + step_x / 2, FLOOR)[0], self.plane.c2p(0, FLOOR)[1], 0)),
                      mn.Square(side_length=0.07, stroke_width=0, fill_color=spec["color"], fill_opacity=1).move_to((self.plane.c2p(0, 0)[0], self.plane.c2p(0, edges_y[j] + step_y / 2)[1], 0)))
        tick_fades = [_fader(tick) for tick in ticks]
        self.add(*[mark["ghost"] for mark in marks], ticks)

        def arrive(alpha):
            for index, mark in enumerate(marks):
                shown = _clip(_clip((alpha - 0.3) / 0.7) * (len(marks) + 1.5) - index)
                mark["fade_ghost"](shown)
                mark["ghost"].move_to(mark["at"] + mn.UP * 0.9 * (1 - _smooth(shown)))
                tick_fades[2 * index](shown)
                tick_fades[2 * index + 1](shown)

        self.step("lhs", *[mn.FadeOut(item) for item in old + gone], mn.Create(lines), mn.FadeIn(width_chip), mn.FadeIn(self.board["rows"]["lhs"]), Drive(arrive, rate_func=mn.linear),
                  self._visit([mark["design"] for mark in marks]), seconds=8)
        indices = [mark["sample"]["strata_index"] for mark in marks]
        pick = min(range(len(marks)), key=lambda k: (abs(indices[k][0] - 4) + abs(indices[k][1] - 4), k))
        i, j = indices[pick]
        top_a, top_b = self.plane.c2p(edges_x[i], 1.2), self.plane.c2p(edges_x[i + 1], 0)
        strip_v = mn.Rectangle(width=top_b[0] - top_a[0], height=top_a[1] - top_b[1], stroke_width=1.4, color=spec["color"], fill_color=spec["color"], fill_opacity=0.18).move_to((top_a + top_b) / 2).set_z_index(-2)
        left_a, left_b = self.plane.c2p(10, edges_y[j + 1]), self.plane.c2p(80, edges_y[j])
        strip_h = mn.Rectangle(width=left_b[0] - left_a[0], height=left_a[1] - left_b[1], stroke_width=1.4, color=_visuals.WARN, fill_color=_visuals.WARN, fill_opacity=0.1).move_to((left_a + left_b) / 2).set_z_index(-3)
        in_column, in_row = sum(1 for entry in indices if entry[0] == i), sum(1 for entry in indices if entry[1] == j)
        chip_v = self._chip("WWR", *self._q(edges_x[i], "%", 1), "–", *self._q(edges_x[i + 1], "%", 1), "·", self._n(in_column), self.tr("point", "Punkt"),
                            at=self._slot(25), size=14, color=spec["color"])
        chip_h = self._chip("d", *self._q(edges_y[j], "m", 2), "–", *self._q(edges_y[j + 1], "m", 2), "·", self._n(in_row), self.tr("point", "Punkt"),
                            at=self._slot(72), size=14, color=_visuals.WARN)
        chosen = marks[pick]
        self.step("marginals", mn.FadeOut(width_chip), mn.FadeIn(strip_v), mn.FadeIn(chip_v), mn.Indicate(chosen["ghost"], color=INK), self._hold(chosen["design"], 1.5), seconds=2)
        self.step("marginals_2", mn.FadeIn(strip_h), mn.FadeIn(chip_h), seconds=1.5)
        cells = mn.VGroup().set_z_index(-2)
        for mark_indices in indices:
            a, b = self.plane.c2p(edges_x[mark_indices[0]], edges_y[mark_indices[1] + 1]), self.plane.c2p(edges_x[mark_indices[0] + 1], edges_y[mark_indices[1]])
            cells.add(mn.Rectangle(width=b[0] - a[0], height=a[1] - b[1], stroke_width=0, fill_color=spec["color"], fill_opacity=0.22).move_to((a + b) / 2))
        diag = _model.sampling_diagnostics("lhs")
        box = diag["largest_empty_box"]
        low, high = self.plane.c2p(box["wwr"][0] * 100, box["overhang"][0]), self.plane.c2p(box["wwr"][1] * 100, box["overhang"][1])
        empty = mn.DashedVMobject(mn.Rectangle(width=high[0] - low[0], height=high[1] - low[1], color=WARN, stroke_width=3).move_to((low + high) / 2), num_dashes=60)
        centre = {"wwr": (box["wwr"][0] + box["wwr"][1]) / 2, "overhang": (box["overhang"][0] + box["overhang"][1]) / 2}
        middle = self._at(centre)
        count_chip = self._chip(self._n(len(indices)), self.tr("of", "von"), self._n(strata["count"] ** 2), self.tr("cells occupied · empty box:", "Zellen belegt · leere Box:"), "WWR", *self._q(box["wwr"][0] * 100, "%"), "–",
                                *self._q(box["wwr"][1] * 100, "%"), "· d", *self._q(box["overhang"][0], "m", 2), "–", *self._q(box["overhang"][1], "m", 2), at=self._slot(), size=13, color=spec["color"])
        caption = mn.VGroup(
                            mn.Line(middle + mn.LEFT * 0.1, middle + mn.RIGHT * 0.1, color=WARN, stroke_width=2.4), mn.Line(middle + mn.UP * 0.1, middle + mn.DOWN * 0.1, color=WARN, stroke_width=2.4))
        self.step("cells", mn.FadeOut(strip_v), mn.FadeOut(strip_h), mn.FadeOut(chip_v), mn.FadeOut(chip_h), mn.LaggedStart(*[mn.FadeIn(cell) for cell in cells], lag_ratio=0.1), mn.FadeIn(count_chip),
                  mn.Create(empty), mn.FadeIn(caption), self._hold(centre, 1.5), seconds=4)
        self.play(mn.FadeOut(empty), mn.FadeOut(count_chip), mn.FadeOut(cells), mn.FadeOut(caption), run_time=0.5)
        spot = mn.Rectangle(width=4.8 * spec["scale"] + 0.1, height=4.0 * spec["scale"] + 0.1, color=INK, stroke_width=2.6).set_z_index(20)
        self.add(spot)

        def follow(step, local, alpha):
            spot.move_to(marks[step]["at"])
            spot.set_stroke(opacity=0 if alpha >= 1 else 1)

        self.step("evaluate", mn.LaggedStart(*[mn.Indicate(mark["ghost"], color=INK) for mark in marks], lag_ratio=0.1), self._visit([mark["design"] for mark in marks], follow), seconds=7)
        self.remove(spot)
        sweep, cursor = self._sweep("lhs", list(range(9)), tags=False)
        self.add(*[mark["full"] for mark in marks])
        best = min(marks, key=lambda item: item["sample"]["electricity"])
        ring = mn.Rectangle(width=4.8 * spec["scale"] + 0.12, height=4.0 * spec["scale"] + 0.12, color=GOOD, stroke_width=4).move_to(best["at"]).set_z_index(21)
        self.rings["lhs"] = ring
        self.step("evaluate_2", sweep, seconds=6)
        self.remove(cursor)
        self.add(ring)
        self.lhs_items = (lines, ticks)

    def _level_segments(self, level):
        """〰️ Screen segments of the contour of the dense surface at ``level`` kWh/a."""
        xs, ys, grid = [value * 100 for value in self.surface["wwr"]], self.surface["overhang"], self.surface["electricity"]
        return [(self.plane.c2p(xa, ya), self.plane.c2p(xb, yb)) for (xa, ya), (xb, yb) in _room._marching(xs, ys, grid, level)]

    def _free_spot(self, target, half, lines, pins, taken):
        """📌 The free spot of the plane nearest to ``target`` where a label of half-size ``half`` touches no contour point, no marker and no earlier label."""
        left, right = self.plane.c2p(10, 0), self.plane.c2p(80, 1.2)
        margin = np.array([0.1, 0.1, 0.0])
        best, best_gap = None, 1e9
        for x in np.arange(left[0] + half[0] + 0.1, right[0] - half[0] - 0.05, 0.1):
            for y in np.arange(left[1] + half[1] + 0.05, right[1] - half[1] - 0.05, 0.08):
                spot = np.array([x, y, 0.0])
                gap = float(np.linalg.norm(spot - target))
                if gap >= best_gap or np.any(np.all(np.abs(lines - spot) < half, axis=1)) or np.any(np.all(np.abs(pins - spot) < half + margin, axis=1)):
                    continue
                if any(np.all(np.abs(other - spot) < 2 * half) for other in taken):
                    continue
                best, best_gap = spot, gap
        return best

    def _reveal(self):
        """🗺️ Uncover the dense-grid surface under all three sets while the room sweeps across it, then show the optimum."""
        levels = self.levels
        heat = self.heat_map(self.plane, self.surface, colour=self.colour, opacity=0.9, levels=levels, labels=False, star=True)
        heat["image"].set_z_index(-3)
        heat["contours"].set_z_index(2)
        heat["star"].set_z_index(30)
        runs = len(self.surface["wwr"]) * len(self.surface["overhang"])
        self._runs = 0
        counter = self._live(lambda: mn.VGroup(self._chip(self.tr("Dense grid:", "Dichtes Raster:"), self._n(len(self.surface["wwr"])), "×", self._n(len(self.surface["overhang"])), "=", self._n(self._runs), self.tr("simulations", "Simulationen"),
                                                         at=self._slot(), size=14, color=INK)))
        self.markers = {}
        dots = mn.VGroup()
        for kind in SETS:
            self.markers[kind] = [self._shape(kind, mark["at"], 0.2, fill=self.colour(mark["sample"]["electricity"])).set_z_index(10) for mark in self.marks[kind]]
            dots.add(*self.markers[kind])
        optimum = self.surface["minimum"]
        design = optimum["design"]
        probe = self._live(lambda: mn.VGroup(mn.Circle(radius=0.17, color=INK, stroke_width=2.6), mn.Dot(radius=0.05, color=INK)).move_to(self._at(_room.values(self.T))).set_z_index(40))
        contours = heat["contours"]
        old = [mark[part] for kind in SETS for mark in self.marks[kind] for part in ("ghost", "full", "tag")]
        old += [self.lhs_items[0], self.lhs_items[1], self.rings["grid"], self.rings["random"], self.rings["lhs"]]
        columns = list(heat["image"].submobjects) if len(heat["image"].submobjects) == len(self.surface["wwr"]) else [heat["image"]]
        column_fades, contour_fade = [_fader(column) for column in columns], _fader(contours)
        self.add(counter, heat["image"], contours)
        start = _room.values(self.T)
        low_wwr, high_wwr = self.surface["wwr"][0], self.surface["wwr"][-1]

        def sweep(alpha):
            self._runs = round(runs * _clip(alpha * 1.15))
            front = _clip(alpha * 1.1) * (len(columns) - 1)
            for position, fade in enumerate(column_fades):
                fade(_clip(front - position + 1))
            contour_fade(_clip((alpha - 0.8) / 0.2))
            phase = (alpha * 12) % 2
            blend = _smooth(alpha / 0.08)
            wwr = low_wwr + (high_wwr - low_wwr) * _clip(alpha * 1.1)
            depth = 1.2 * (phase if phase <= 1 else 2 - phase)
            self.T["wwr"].set_value(start["wwr"] + (wwr - start["wwr"]) * blend)
            self.T["overhang"].set_value(start["overhang"] + (depth - start["overhang"]) * blend)
            self.P.set_value(1.0)

        self.step("reveal", *[mn.FadeOut(item) for item in old], mn.FadeIn(dots), mn.FadeIn(probe), Drive(sweep, rate_func=mn.linear), seconds=6)
        note = self._chip("WWR", *self._q(design["wwr"] * 100, "%"), "·", "d", *self._q(design["overhang"], "m", 2), "·", *self._q(optimum["electricity"], "kWh/a", 1),
                          at=self._slot(), size=14, color=GOOD)
        self.play(mn.FadeOut(counter), run_time=0.3)
        self.remove(counter)
        board = self.board
        shown = [self.bars_alone, *self.panel[:4], *self.meters.values()]
        for widget in shown:
            widget.update(0)
        lhs_best = min(self.marks["lhs"], key=lambda item: item["sample"]["electricity"])
        crossings = np.array([point for level in self.levels for a, b in self._level_segments(level) for point in (a, (a + b) / 2, b)])
        pins = np.array([mark["at"] for kind in SETS for mark in self.marks[kind]] + [heat["star"].get_center()])
        tags, taken = mn.VGroup(), []
        for text, value, target, away in ((self.tr("Optimum", "Optimum"), optimum["electricity"], self.plane.c2p(design["wwr"] * 100, design["overhang"]), np.array([-1.1, 0.3, 0.0])),
                                    (self.tr("Best Latin hypercube", "Bester Latin-Hypercube"), lhs_best["sample"]["electricity"], lhs_best["at"], np.array([0.8, -0.35, 0.0]))):
            chip = self._chip(text, size=13, color=GOOD)
            half = np.array([chip.width / 2 + 0.03, chip.height / 2 + 0.03, 1.0])
            spot = self._free_spot(target + away, half, crossings, pins, taken)
            if spot is None:
                continue
            taken.append(spot)
            tags.add(mn.Line(spot, target, color=INK, stroke_width=1.2, stroke_opacity=0.8).set_z_index(14), chip.move_to(spot).set_z_index(15))
        shown.append(tags)
        self.step("reveal_2", mn.FadeIn(heat["star"]), mn.FadeIn(note), mn.FadeOut(probe), self.set_design(self.T, design, 1.5),
                  *[mn.FadeOut(item) for item in (self.bars, board["pips"], board["header"], *board["rows"].values())], *[mn.FadeIn(widget) for widget in shown], seconds=4)
        self.heat, self.note, self.dots, self.panel_items = heat, note, dots, shown

    def _compare(self):
        """⚖️ Show the best room of each set and of the dense grid with consistent bars and gaps."""
        optimum = {"wwr": self.surface["minimum"]["design"]["wwr"], "overhang": self.surface["minimum"]["design"]["overhang"], "electricity": self.surface["minimum"]["electricity"]}
        bests = {kind: min(self.samples[kind], key=lambda item: item["electricity"]) for kind in SETS}
        centres = [(2.5, 1.4), (5.4, 1.4), (2.5, -1.2), (5.4, -1.2)]
        shown_optimum = round(optimum["electricity"], 1)
        gaps = {kind: round(round(bests[kind]["electricity"], 1) - shown_optimum, 1) for kind in SETS}
        unit = 1.4 / max(gaps.values())
        tiles = {}
        for kind, centre in zip(SETS, centres):
            tiles[kind] = self._tile(kind, bests[kind], centre, self.tr(SETS[kind]["en"], SETS[kind]["de"]), delta=(gaps[kind], gaps[kind] / shown_optimum * 100), unit=unit)
        tiles["optimum"] = self._tile("grid", optimum, centres[3], self.tr("Dense-grid optimum", "Optimum dichtes Raster"), shape=False)
        star = mn.Star(n=5, outer_radius=0.12, inner_radius=0.05, color=INK, stroke_width=1.2, fill_color=GOOD, fill_opacity=1).move_to((centres[3][0] - 1.38, centres[3][1] + 1.1, 0))
        tiles["optimum"]["base"].add(star)
        rings = mn.VGroup(*[mn.Circle(radius=0.2, color=SETS[kind]["color"], stroke_width=3.5).move_to(self._at(bests[kind])).set_z_index(31) for kind in SETS])
        key = self._key((4.1, 3.22, 0))
        right = [self.sections, self.design_chip, *self.panel_items]
        bar_note = self._chip(self.tr("Bars: full height =", "Balken: volle Höhe ="), *self._q(self.full, "kWh/a", 1), at=(4.1, 2.78, 0), size=15, color=INK)
        self.step("compare", *[mn.FadeOut(item) for item in right], mn.LaggedStart(*[mn.FadeIn(tiles[kind]["base"]) for kind in SETS], lag_ratio=0.35), mn.FadeOut(self.note), mn.FadeIn(rings), mn.FadeIn(key), mn.FadeIn(bar_note), seconds=4)
        scale_note = self._chip(self.tr("Δ bar: full length =", "Δ-Balken: volle Länge ="), *self._q(max(gaps.values()), "kWh/a", 1), at=(4.1, 2.78, 0), size=15, color=WARN)
        self.step("compare_2", mn.FadeIn(tiles["optimum"]["base"]), mn.FadeOut(bar_note), mn.FadeIn(scale_note), mn.LaggedStart(*[mn.FadeIn(tiles[kind]["extra"]) for kind in SETS], lag_ratio=0.3), seconds=3)
        price = self._chip(self._n(9), self.tr("vs.", "gegen"), self._n(len(self.surface["wwr"]) * len(self.surface["overhang"])), self.tr("simulations", "Simulationen"),
                           at=self._slot(), size=14, color=WARN)
        self.step("compare_3", mn.FadeIn(price), *[mn.Indicate(tiles[kind]["extra"], color=INK) for kind in SETS], seconds=2)
        self.compare_items = (tiles, rings, price, optimum, bests, key, scale_note)

    def _region(self, level):
        """🌿 Filled cells of the dense surface whose mean electricity stays at or below ``level``."""
        xs, ys, grid = [value * 100 for value in self.surface["wwr"]], self.surface["overhang"], self.surface["electricity"]
        group = mn.VGroup().set_z_index(1)
        for i in range(len(xs) - 1):
            start = None
            for j in range(len(ys)):
                inside = j < len(ys) - 1 and (grid[i][j] + grid[i + 1][j] + grid[i][j + 1] + grid[i + 1][j + 1]) / 4 <= level
                if inside and start is None:
                    start = j
                if not inside and start is not None:
                    low, high = self.plane.c2p(xs[i], ys[start]), self.plane.c2p(xs[i + 1], ys[j])
                    group.add(mn.Rectangle(width=high[0] - low[0] + 0.004, height=high[1] - low[1], stroke_width=0, fill_color=GOOD, fill_opacity=0.22).move_to((low + high) / 2))
                    start = None
        return group

    def _limits(self):
        """🚧 Close on what coverage reveals and what it cannot prove."""
        tiles, rings, price, optimum, bests, key, scale_note = self.compare_items
        within = _half_up(optimum["electricity"], 1) * 1.05
        outline = mn.VMobject(color=GOOD, stroke_width=3.5).set_z_index(4)
        for a, b in self._level_segments(within):
            outline.start_new_path(a)
            outline.add_line_to(b)
        fill = self._region(within)
        region = self._chip("≤ +5 %", self.tr("of the optimum", "des Optimums"), "·", "≤", *self._q(within, "kWh/a", 1), at=self._slot(), size=14, color=GOOD)
        pick = min(self.marks["lhs"], key=lambda item: item["sample"]["electricity"])
        cards = mn.VGroup(self._chip(self.tr("Dense grid", "Dichtes Raster"), "·", self._n(len(self.surface["wwr"]) * len(self.surface["overhang"])), self.tr("simulations", "Simulationen"), "→", *self._q(optimum["electricity"], "kWh/a", 1),
                                     at=(4.0, -0.95, 0), size=15, color=INK),
                          self._chip(self.tr(SETS["lhs"]["en"], SETS["lhs"]["de"]), "·", self._n(9), self.tr("simulations", "Simulationen"), "→", *self._q(pick["sample"]["electricity"], "kWh/a", 1),
                                     at=(4.0, -1.6, 0), size=15, color=SETS["lhs"]["color"]))
        self.step("limits", *[mn.FadeOut(tile["base"]) for tile in tiles.values()], *[mn.FadeOut(tile["extra"]) for tile in tiles.values() if len(tile["extra"].submobjects)],
                  mn.FadeOut(price), mn.FadeOut(key), mn.FadeOut(scale_note), mn.Create(outline), mn.FadeIn(fill), mn.FadeIn(region), mn.FadeIn(self.sections), mn.FadeIn(self.design_chip), seconds=2.5)
        self.step("limits_2", mn.FadeIn(cards), seconds=1.5)
        closing = self._chip(self.tr("Coverage informs the search; it cannot prove an optimum.", "Abdeckung leitet die Suche, ersetzt keinen Beweis."), at=(4.0, -2.25, 0), size=15, color=INK)
        note = self._txt(self.tr("Simplified annual teaching model · Zurich-like climate", "Vereinfachtes Jahresmodell · Klima wie Zürich"), at=(-5.8, 3.15, 0), size=17, color=INK, edge="left")
        self.step("limits_3", mn.FadeIn(closing), mn.FadeIn(note), seconds=1.5)
        answer = self._chip(self.tr(SETS["lhs"]["en"], SETS["lhs"]["de"]), "·", self.tr("best of nine", "Bester von neun"), *self._q(pick["sample"]["electricity"], "kWh/a", 1), "·", "Δ",
                            *self._q(_half_up(pick["sample"]["electricity"], 1) - _half_up(optimum["electricity"], 1), "kWh/a", 1), at=self._slot(), size=14, color=SETS["lhs"]["color"])
        self.P.set_value(1.0)
        shown = [self.bars_alone, *self.panel[:4], *self.meters.values()]
        for widget in shown:
            widget.update(0)
        self.step("limits_4", mn.FadeOut(cards), mn.FadeOut(closing), mn.FadeOut(region), mn.FadeIn(answer), *[mn.FadeIn(widget) for widget in shown], self.set_design(self.T, pick["design"], 1.5), seconds=3)
