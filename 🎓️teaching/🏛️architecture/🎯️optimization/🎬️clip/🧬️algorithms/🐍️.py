"""🎬 Algorithms and Evolutionary Search: three solvers walk the same electricity landscape of the seminar room."""

import importlib
import math
import sys
from functools import lru_cache
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_room, _art, _model, mn = _visuals._room, _visuals._art, _visuals._model, _visuals._engine

INK, MUTED, DARK = _art.P_WHITE, _art.PASTEL_GREY, _art.P_DEEP_DARK
GRADIENT, PATTERN, GENETIC = _art.P_PRIMARY, _art.P_ORANGE, _art.PASTEL_PURPLE
PARENT_A, PARENT_B, GOOD, BAD, STAR, WARN = _art.P_CYAN, _art.P_ORANGE, _art.P_GREEN, _art.P_PRIMARY, _art.P_YELLOW, _art.P_ORANGE
WINDOWS, OVERHANG = _art.P_CYAN, _art.P_ORANGE
HEAT, COOL, LIGHT = _room.HEAT, _room.COOL, _room.LIGHT
CARD_X, CARD_PITCH = 1.9, 1.72
SLOTS = [(1.5 + 1.5 * (i % 4 - 1.5), 1.78 - 1.14 * (i // 4)) for i in range(8)]
TAPE_X0, TAPE_STEP, TAPE_Y = 1.4, 0.054, 2.55
FRACTION = 1.01
BOUNDS = {"wwr": (10.0, 80.0), "overhang": (0.0, 1.2)}
MAP_AT, MAP_W, MAP_H = (-4.25, -.1, 0), 4.4, 3.2
ROOMS_AT, ROOM_SCALE = ((.3, .85, 0), (3.85, .85, 0)), .28
BAR_AT, BAR_H = (6.2, -2.0, 0), 2.6
LEDGER_X, LEDGER_Y, ROW_Y, BAND_Y = -1.35, (-.8, -1.07, -1.34, -1.61), -1.62, -2.47
LENS_AT, LENS_R, LENS_ZOOM = (-2.75, -.2, 0), .6, 11


@lru_cache(maxsize=1)
def _data():
    """🗃️ Read every solver path once and flatten it into the ordered simulation log of each method."""
    surface, gradient, pattern = _model.surface(41), _model.gradient_path(), _model.pattern_path()
    genetic, convergence = _model.genetic(), _model.convergence()
    generations = genetic["generations"]
    logs = {"gradient": [], "pattern": [], "genetic": []}
    for item in gradient:
        logs["gradient"] += [(item["design"], item["electricity"])] + [(probe["design"], probe["electricity"]) for probe in item["probes"]]
    for index, item in enumerate(pattern):
        logs["pattern"] += [(item["design"], item["electricity"])] if index == 0 else [(probe["design"], probe["electricity"]) for probe in item["probes"]]
    logs["genetic"] = [(item["design"], item["electricity"]) for item in generations[0]["population"]]
    for generation in generations:
        logs["genetic"] += [(child["design"], child["electricity"]) for child in generation["children"]]
    best = {}
    for name, log in logs.items():
        running, designs = None, []
        for design, electricity in log:
            if running is None or electricity < running[1]:
                running = (design, electricity)
            designs.append(running[0])
        best[name] = designs
    series = {name: [value for _, value in convergence["series"][name]] for name in logs}
    limit = FRACTION * convergence["map_minimum"]
    reach = {name: next((index + 1 for index, value in enumerate(values) if value <= limit), None) for name, values in series.items()}
    budget = convergence["budget"]
    stops = {}
    for name, values in series.items():
        candidates = [("budget", budget), ("tolerance", next((n for n in range(6, budget + 1) if values[n - 6] - values[n - 1] < 1.0), None)),
                      ("stagnation", next((n for n in range(11, budget + 1) if values[n - 11] - values[n - 1] <= 0), None))]
        stops[name] = min((item for item in candidates if item[1]), key=lambda item: item[1])
    order = [item["id"] for item in generations[0]["population"]] + [child["id"] for generation in generations for child in generation["children"]]
    individuals = {}
    for generation in generations:
        for item in generation["population"]:
            individuals[item["id"]] = item
        for child in generation["children"]:
            individuals[child["id"]] = {"id": child["id"], "design": child["design"], "electricity": child["electricity"]}
    ranks = {item["id"]: rank for rank, item in enumerate(sorted(generations[0]["population"], key=lambda item: item["electricity"]), 1)}
    return {"surface": surface, "gradient": gradient, "pattern": pattern, "genetic": genetic, "convergence": convergence,
            "logs": logs, "best": best, "series": series, "reach": reach, "stops": stops, "budget": budget, "generations": generations,
            "born": {identifier: index + 1 for index, identifier in enumerate(order)}, "individuals": individuals, "ranks": ranks,
            "members": [{item["id"] for item in generation["population"]} for generation in generations]}


def _half(value, digits=1):
    """➗ Round half up, the rounding of the printed numbers."""
    return math.floor(value * 10 ** digits + .5 + 1e-9) / 10 ** digits


@lru_cache(maxsize=8192)
def _figures_of(key):
    """🧾 Every printed figure of one design, each derived from the printed figures before it, so every row on screen closes."""
    design = dict(key)
    found = _room.result(design)
    systems = _room.systems()
    kilowatt = _half(_room.constant("lighting", "power_density") * _room.constant("room", "width") * _room.constant("room", "depth") / 1000, 3)
    share = _half(100 * (1 - found["autonomy"]), 3)
    loss = _half(_loss_of(found))
    heat_demand, cold_demand = _half(found["heating_demand"]), _half(found["cooling_demand"])
    heat, cold = _half(heat_demand / systems["cop"]), _half(cold_demand / systems["eer"])
    lamps = _half(kilowatt * _room.constant("lighting", "hours") * share / 100)
    return {"loss": loss, "gains": _half(loss - heat_demand), "heat_demand": heat_demand, "cold_demand": cold_demand, "heat": heat, "cold": cold, "share": share, "lamps": lamps,
            "total": _half(heat + cold + lamps)}


def _loss_of(found):
    """🌡️ Winter transmission loss in kWh/a: (U_wall · wall + U_window · window + other) · degree hours."""
    conductance = found["u_wall"] * found["wall_area"] + _room.constant("envelope", "u_window") * found["window_area"] + _room.constant("envelope", "h_other")
    return conductance * _room.constant("climate", "heating_degree_hours") / 1000


def _figures(design):
    """🧾 Printed figures of a design (rounded cache key, so live trackers hit the cache)."""
    return _figures_of(tuple((name, round(float(design[name]), 5)) for name in ("orientation", "wwr", "overhang", "insulation")))


class Beat6_EvolutionarySearch(_visuals.ClipScene):
    """📚 Gradient descent, pattern search and a genetic algorithm walk one landscape of the seminar room."""

    TITLE = {"en": "Algorithms and Evolutionary Search", "de": "Algorithmen und Evolutionäre Suche"}
    NARRATION = [
        ["task", "Seminar room, south façade: which window share and overhang need the least electricity?", "Seminarraum, Südfassade: Welcher Fensteranteil und welches Vordach brauchen am wenigsten Strom?"],
        ["task_2", "Every design costs electricity three ways: heating, cooling and the lamps.", "Jeder Entwurf kostet auf drei Wegen Strom: Heizen, Kühlen und die Lampen."],
        ["task_3", "One simulation per design, budget 50: a solver chooses the next design.", "Eine Simulation pro Entwurf, Budget 50: Ein Solver wählt den nächsten Entwurf."],
        ["map", "The full map needs 1,681 simulations; colour shows annual electricity in kWh/a.", "Die ganze Karte braucht 1.681 Simulationen; die Farbe zeigt den Strombedarf in kWh/a."],
        ["map_2", "The valley floor is the best design: less window share, deeper overhang.", "Der Talboden ist der beste Entwurf: weniger Fensteranteil, tieferes Vordach."],
        ["map_3", "In practice we see only simulated points; the rest stays in the fog.", "In der Praxis sehen wir nur simulierte Punkte; der Rest bleibt im Nebel."],
        ["gradient", "Gradient descent starts at a 75 % window share and 0.1 m overhang.", "Der Gradientenabstieg startet bei 75 % Fensteranteil und 0,1 m Vordach."],
        ["gradient_2", "Four close neighbours reveal the slope: five simulations per step.", "Vier nahe Nachbarn verraten die Steigung: fünf Simulationen pro Schritt."],
        ["gradient_3", "More window share raises the electricity here, more overhang lowers it.", "Mehr Fensteranteil erhöht hier den Strombedarf, mehr Vordach senkt ihn."],
        ["gradient_4", "The step goes downhill: the room follows, the bar shrinks.", "Der Schritt geht bergab: Der Raum folgt, der Balken schrumpft."],
        ["overshoot", "The first step overshoots the valley floor, to too small a window share.", "Der erste Schritt schießt über den Talboden hinaus, auf einen zu kleinen Fensteranteil."],
        ["overshoot_2", "Repeating, the path zigzags; some steps even raise the electricity.", "Wiederholt zickzackt der Pfad; manche Schritte erhöhen sogar den Strombedarf."],
        ["gradient_result", "After 50 simulations, gradient descent's best design is within 1 % of the map minimum.", "Nach 50 Simulationen liegt der Gradientenabstieg innerhalb von 1 % des Kartenminimums."],
        ["gradient_result_2", "Four of every five simulations probe slopes; noise or a second valley can mislead them.", "Vier von fünf Simulationen tasten Steigungen ab; Rauschen oder ein zweites Tal können sie täuschen."],
        ["pattern", "Pattern search needs no slope: it only compares simulated designs.", "Die Mustersuche braucht keine Steigung: Sie vergleicht nur simulierte Entwürfe."],
        ["pattern_2", "From the same start it tries window share and overhang, up and down: four simulations.", "Vom selben Start probiert sie Fensteranteil und Vordach, größer und kleiner: vier Simulationen."],
        ["pattern_3", "The best neighbour wins and the room moves there.", "Der beste Nachbar gewinnt, und der Raum zieht dorthin um."],
        ["pattern_4", "If no neighbour is better, halve the step.", "Ist kein Nachbar besser, halbiere den Schritt."],
        ["pattern_5", "Round after round the steps shrink until no neighbour is better.", "Runde um Runde werden die Schritte kleiner, bis kein Nachbar besser ist."],
        ["pattern_result", "Pattern search gets within 1 % of the minimum first, after 16 simulations, with no slope.", "Die Mustersuche kommt nach 16 Simulationen als erste auf 1 % ans Minimum, ganz ohne Steigung."],
        ["population", "A genetic algorithm keeps eight designs; window share and overhang are its genes.", "Ein genetischer Algorithmus hält acht Entwürfe; Fensteranteil und Vordach sind seine Gene."],
        ["population_2", "Each card shows one design and its simulated electricity: eight simulations.", "Jede Karte zeigt einen Entwurf und seinen simulierten Strombedarf: acht Simulationen."],
        ["population_3", "Spread over the map, they explore several regions.", "Über die Karte verteilt erkunden sie mehrere Bereiche."],
        ["tournament", "Selection: draw two cards at random; the one with less electricity becomes a parent.", "Auswahl: Zwei Karten zufällig ziehen; die mit weniger Strombedarf wird ein Elternteil."],
        ["tournament_2", "Good designs win more often, yet even a mid-ranked card can win a duel.", "Gute Entwürfe gewinnen öfter, doch auch eine mittlere Karte kann ein Duell gewinnen."],
        ["crossover", "Crossover: the child takes window share from one parent, overhang from the other.", "Kreuzung: Das Kind erbt den Fensteranteil vom einen, das Vordach vom anderen Elternteil."],
        ["crossover_2", "On the map the child sits at a corner of its parents' box.", "Auf der Karte liegt das Kind an einer Ecke des Elternrechtecks."],
        ["mutation", "Mutation nudges the child a little; here the change helps.", "Die Mutation verschiebt das Kind leicht; hier hilft die Änderung."],
        ["children", "Six children, one simulation each: six simulations per generation.", "Sechs Kinder, je eine Simulation: sechs Simulationen pro Generation."],
        ["survival", "Elitism: the two best parents survive, so the best value never gets worse.", "Elitismus: Die beiden besten Eltern überleben, so verschlechtert sich der Bestwert nie."],
        ["mutation_2", "Later a mutation can also hurt: the simulated child is worse than the crossover alone.", "Später kann Mutation auch schaden: Das simulierte Kind ist schlechter als die Kreuzung."],
        ["generations", "Repeat for seven generations: the cloud contracts and the best card improves in jumps.", "Wiederhole sieben Generationen: Die Wolke schrumpft, die beste Karte verbessert sich in Sprüngen."],
        ["genetic_result", "After 50 simulations the best card is within 1 % of the map minimum.", "Nach 50 Simulationen liegt die beste Karte innerhalb von 1 % des Kartenminimums."],
        ["genetic_result_2", "Within 1 %: pattern search needs 16 simulations, the others more than twice as many.", "Bis 1 %: Die Mustersuche braucht 16 Simulationen, die anderen mehr als doppelt so viele."],
        ["compare", "Same model, same budget: the best electricity so far, plotted against simulations used.", "Gleiches Modell, gleiches Budget: der bisher beste Strombedarf, aufgetragen gegen die Simulationen."],
        ["compare_2", "Pattern search gets there first; all three end within 3.3 kWh/a of the map minimum.", "Die Mustersuche ist zuerst am Ziel; alle drei enden höchstens 3,3 kWh/a über dem Kartenminimum."],
        ["stopping", "Stop at a budget, a tolerance or stagnation, and say which.", "Stoppe bei Budget, Toleranz oder Stagnation und nenne die Regel."],
        ["stopping_2", "Tolerance fires first: the genetic run would stop after 15 simulations, still far from the best.", "Die Toleranz greift zuerst: Schon nach 15 Simulationen stoppte der genetische Lauf, weit vom Besten."],
        ["proof", "None of them proves the global optimum; for that we need the full map, 1,681 simulations.", "Keines beweist das globale Optimum; dafür brauchen wir die volle Karte, 1.681 Simulationen."],
        ["confirm", "Check the winner with a finer simulation, then judge it as architecture.", "Prüfe den Sieger mit einer feineren Simulation und beurteile ihn dann architektonisch."],
        ["tools", "Parametric tools wire this up: sliders, simulation, solver.", "Parametrische Werkzeuge verdrahten das: Regler, Simulation, Solver."],
        ["tools_2", "Choose by smoothness, noise and simulation budget; a population copes with several valleys.", "Wähle nach Glätte, Rauschen und Simulationsbudget; eine Population meistert mehrere Täler."],
        ["close", "Answer: 66 % window share, 0.99 m overhang, 494.8 kWh/a; the architect judges the rest.", "Antwort: 66 % Fensteranteil, 0,99 m Vordach, 494,8 kWh/a; den Rest beurteilt der Architekt."],
    ]

    def construct(self):
        """▶️ Present the solver comparison through the shared local rendering interface."""
        self.present("algorithms")

    def _p(self, value, unit, digits=0):
        """🔢 Live number and unit parts for a chip."""
        return _room.parts(value, unit, digits, self._german())

    def _n(self, value, digits=0):
        """🔢 One live number part."""
        return _room.Number(_room.number(value, digits, self._german()))

    def _signed(self, value, unit, digits):
        """➕ A static signed quantity such as +0.9 % or −132 kWh/a."""
        return ("+" if value > 0 else "") + self.quantity(value, unit, digits)

    def _chip(self, parts, point, *, size=14, color=INK, width=None):
        """🔖 A framed label scaled to fit ``width``."""
        return _art.window_chip(_room.text(parts, size=size, color=color, width=width), color=color).move_to(point)

    def _live_chip(self, build, *, size=14, color=INK, width=None, key=None, z=0):
        """⏱️ A chip whose parts and place follow live values."""
        def make():
            parts, point = build()
            return mn.VGroup(self._chip(parts, point, size=size, color=color, width=width)).set_z_index(z)
        return self._live(make, key)

    def _line(self, items, point, *, size=14, align="center", width=None):
        """📝 One line of multicolour words and numbers: an item is a string, a list of parts or a ``(parts, colour)`` pair."""
        pieces = []
        for item in items:
            parts, colour = item if isinstance(item, tuple) else (item, INK)
            pieces.append(_room.row(size, colour, *(parts if isinstance(parts, list) else [parts])))
        cursor = 0.0
        gap = .11 * size / 14
        group = mn.VGroup()
        for piece in pieces:
            group.add(piece.shift(np.array([cursor - piece.get_left()[0], 0.0, 0.0])))
            cursor += piece.width + gap
        if width is not None and group.width > width:
            group.scale_to_fit_width(width)
        return _room.place(group, point, align)

    def _pos(self, design):
        """📍 Map position of a design."""
        return self.ax.c2p(design["wwr"] * 100, design["overhang"])

    def _tape(self):
        """🎞️ One square per simulation up to the budget; squares take the colour of the method that spent them."""
        squares = mn.VGroup()
        for index in range(self.budget):
            square = mn.Rectangle(width=TAPE_STEP * .8, height=.14, stroke_width=.6, stroke_color=MUTED)
            square.move_to((TAPE_X0 + (index + .5) * TAPE_STEP, TAPE_Y, 0))
            square.add_updater(lambda item, index=index: item.set_fill(self.tape_colors[index], opacity=.95 if index < self.runs.get_value() - .001 else .06))
            square.update(0)
            squares.add(square)
        squares.add(mn.Line((TAPE_X0 + self.budget * TAPE_STEP, TAPE_Y - .12, 0), (TAPE_X0 + self.budget * TAPE_STEP, TAPE_Y + .12, 0), color=INK, stroke_width=1.5))
        return squares

    def _counter(self):
        """🔢 Live counter of simulations used against the current total."""
        def build():
            return ([self._n(round(self.runs.get_value())), self.tr("of", "von"), self._n(round(self.total.get_value())), self.tr("simulations", "Simulationen")], (5.45, TAPE_Y, 0))
        return self._live_chip(build, size=14, key=lambda: (round(self.runs.get_value()), round(self.total.get_value())))

    def _spend(self, start, count, colour, seconds=1.0):
        """💸 Animation that spends ``count`` simulations on the tape, beginning at simulation ``start``."""
        for index in range(count):
            self.tape_colors[start + index] = colour
        return self.runs.animate(run_time=seconds, rate_func=mn.linear).set_value(start + count)

    def _pointer(self):
        """🧭 Ring and dot on the map that follow the focus design, with the path walked so far."""
        def build():
            design = _room.values(self.focus)
            spot = self.ax.c2p(design["wwr"] * 100, design["overhang"])
            points = [self.ax.c2p(wwr * 100, overhang) for wwr, overhang in self.path[:int(self.stage.get_value() + 1e-6) + 1]]
            points = [point for index, point in enumerate(points) if index == 0 or np.linalg.norm(point - points[index - 1]) > 1e-4]
            if points and np.linalg.norm(points[-1] - spot) > 1e-4:
                points.append(spot)
            group = mn.VGroup()
            if len(points) > 1:
                group.add(mn.VMobject(color=self.colour, stroke_width=3.4, stroke_opacity=.95).set_points_as_corners(points))
            group.add(mn.Circle(radius=.15, color=INK, stroke_width=2.2).move_to(spot), mn.Dot(spot, radius=.075, color=self.colour))
            return group.set_z_index(6)
        return self._live(build, lambda: (*_room.design_key(self.focus)(), self.stage.get_value(), tuple(self.path), self.colour)).set_z_index(6)

    def _painted(self, design, electricity, radius=.07):
        """🖌️ A simulated design painted in the colour of its own electricity."""
        dot = mn.Dot(self._pos(design), radius=radius, color=self.ramp_fn(electricity)).set_stroke(INK, width=1.5, opacity=.95)
        dot.set_z_index(4)
        self.painted.append(dot)
        return dot

    def _ghost_trail(self, points, colour):
        """👻 A faint copy of a finished path."""
        return mn.VMobject(color=colour, stroke_width=2.2, stroke_opacity=.3).set_points_as_corners([self.ax.c2p(w * 100, o) for w, o in points]).set_z_index(3)

    def _method(self, en, de, colour):
        """🏷️ The method label chip; also switches the pointer colour."""
        self.colour = colour
        return self._chip([self.tr(en, de)], (0.12, TAPE_Y, 0), size=14, color=colour, width=1.95)

    def _clear(self, *, keep=()):
        """🧹 Fade out everything but the title, the caption, the follower and what is kept."""
        content = [item for item in self.mobjects if item is not self._caption and item is not self.follower and not isinstance(item, mn.ValueTracker) and item not in keep]
        if content:
            self.play(*[mn.FadeOut(item) for item in content], run_time=.5)

    def _at(self, sequence):
        """🎯 The entry of a per-simulation sequence at the current run."""
        return sequence[max(0, min(len(sequence) - 1, int(self.runs.get_value() + 1e-6) - 1))]

    def _fade(self, items):
        """🌫️ FadeOut animations for the items (or their parts) still on screen."""
        members = {id(member) for top in self.mobjects for member in top.get_family()}
        return [mn.FadeOut(item) for item in items if any(id(part) in members for part in item.get_family())]

    def _follow(self):
        """🔦 While following, the focus design is the best genetic design simulated so far."""
        if self.following and self.runs.get_value() >= 1 - 1e-6:
            for key, value in self._at(self.d["best"]["genetic"]).items():
                if key in self.focus:
                    self.focus[key].set_value(float(value))

    def _gap(self, electricity):
        """➖ Distance to the map minimum computed from the displayed one-decimal values, so the arithmetic on screen closes."""
        return round(electricity, 1) - round(self.optimum["electricity"], 1)

    def _set_focus(self, design):
        """🎚️ Jump the focus design without animation."""
        for key, value in design.items():
            if key in self.focus:
                self.focus[key].set_value(float(value))

    def _loss(self, design, found):
        """🌡️ Winter transmission loss in kWh/a."""
        return _loss_of(found)

    def sum_formula(self, point, size=14, trackers=None, width=5.6, unit="kWh/a"):
        """➕ The three electricity parts and their total at one decimal; every part is derived from its printed factors and the total is their printed sum."""
        pick = lambda key: (lambda design, found: _figures(design)[key])
        row = [self.live(pick("heat"), HEAT, 1), "+", self.live(pick("cold"), COOL, 1), "+", self.live(pick("lamps"), LIGHT, 1), "=", self.live(pick("total"), INK, 1, unit)]
        return self.formula(trackers or self.focus, [row], point, size=size, width=width)

    def _ledger(self):
        """🧾 How heating, cooling and lighting electricity of the focus design come about; factors and results are printed at one precision and every row closes."""
        tr = self.tr
        systems, hours = _room.systems(), _room.constant("lighting", "hours")
        kilowatt = _half(_room.constant("lighting", "power_density") * _room.constant("room", "width") * _room.constant("room", "depth") / 1000, 3)
        soft = _room.muted
        pick = lambda key: (lambda design, found: _figures(design)[key])
        balance = [[(tr("Loss", "Verlust"), soft(HEAT)), self.live(pick("loss"), soft(HEAT), 1), "−", (tr("useful gains", "nutzbare Gewinne"), soft(HEAT)), self.live(pick("gains"), soft(HEAT), 1), "=",
                    (tr("heat", "Wärme"), soft(HEAT)), self.live(pick("heat_demand"), HEAT, 1, "kWh/a")]]
        heating = [[(tr("Heat", "Wärme"), soft(HEAT)), self.live(pick("heat_demand"), soft(HEAT), 1, "kWh/a"), "÷", ("COP", INK), self.live(systems["cop"], INK, 1), "=", self.live(pick("heat"), HEAT, 1, "kWh/a")]]
        cooling = [[(tr("Cold", "Kälte"), soft(COOL)), self.live(pick("cold_demand"), soft(COOL), 1, "kWh/a"), "÷", ("EER", INK), self.live(systems["eer"], INK, 1), "=", self.live(pick("cold"), COOL, 1, "kWh/a")]]
        lamps = [[(tr("Lamps", "Lampen"), soft(LIGHT)), self.live(kilowatt, INK, 3, "kW"), "·", self.live(hours, INK, 0, "h/a"), "·", (tr("on", "an"), soft(LIGHT)), self.live(pick("share"), LIGHT, 3, "%"), "=",
                  self.live(pick("lamps"), LIGHT, 1, "kWh/a")]]
        return mn.VGroup(*[self.formula(self.focus, rows, (LEDGER_X, y, 0), size=16, align="left", width=6.7) for rows, y in zip((balance, heating, cooling, lamps), LEDGER_Y)])

    def _seasons(self):
        """☀️ Season tags under the two rooms with the noon sun altitude drawn in them."""
        return mn.VGroup(*[self._line([(self.tr(word, german), colour), self.tr("noon, sun altitude", "mittags, Sonnenhöhe") + " " + self.quantity(_room.sun_state(self.start["orientation"], season)["altitude"], "°", 0)],
                                      (at[0], -.33, 0), size=14, width=3.4) for at, season, word, german, colour in zip(ROOMS_AT, ("winter", "summer"), ("Winter", "Summer"), ("Winter", "Sommer"), (HEAT, COOL))])

    def _losses(self):
        """🌡️ Winter heat leaving through the glass and the wall: arrow weight and length follow the transmission losses U · A · degree hours of the live design."""
        s, origin = ROOM_SCALE, np.array(ROOMS_AT[0], dtype=float) - ROOM_SCALE * np.array([3.0, 1.5, 0.0])
        factor, scale = _room.constant("envelope", "u_window") * _room.constant("climate", "heating_degree_hours") / 1000, 1250.0

        def build():
            found = _room.result(_room.values(self.focus))
            x_o = -(_room.MASONRY + self.focus["insulation"].get_value())
            window = min(found["window_area"] / _room.WIDTH, _room.HEAD - .05)
            sill = _room.HEAD - window
            glass = factor * found["window_area"]
            wall = found["u_wall"] * found["wall_area"] * _room.constant("climate", "heating_degree_hours") / 1000
            group = mn.VGroup()
            for y, loss, count in ((sill + (k + .5) / 3 * window, glass, 3) for k in range(3)):
                start = origin + s * np.array([x_o - .1, y, 0.0])
                group.add(mn.Arrow(start, start + mn.LEFT * (.14 + .75 * loss / scale), buff=0, color=HEAT, stroke_width=2 + 6 * loss / scale, tip_length=.1 + .06 * loss / scale,
                                   max_tip_length_to_length_ratio=.5, max_stroke_width_to_length_ratio=30))
            for y in (sill / 2, (_room.HEAD + _room.HEIGHT) / 2):
                if y > .05:
                    start = origin + s * np.array([x_o - .1, y, 0.0])
                    group.add(mn.Arrow(start, start + mn.LEFT * (.14 + .75 * wall / scale), buff=0, color=HEAT, stroke_width=2 + 6 * wall / scale, tip_length=.08,
                                       max_tip_length_to_length_ratio=.5, max_stroke_width_to_length_ratio=30))
            return group
        return self._live(build, _room.design_key(self.focus))

    def _flows(self):
        """🔥 Numbers under the rooms: heat lost through the glass in winter, solar heat gained in summer."""
        loss = lambda design, found: round(_room.constant("envelope", "u_window") * found["window_area"] * _room.constant("climate", "heating_degree_hours") / 1000, 1)
        rows = ((ROOMS_AT[0][0], [(self.tr("of the loss through glass", "davon durch Glas"), _room.muted(HEAT)), self.live(loss, HEAT, 1, "kWh/a")]),
                (ROOMS_AT[1][0], [(self.tr("solar heat gain", "solare Wärmegewinne"), _room.muted(COOL)), self.live("solar_summer", COOL, 1, "kWh/a")]))
        legend = self._line([(self.tr("arrow length ∝ cooling demand", "Pfeillänge ∝ Kühlbedarf"), COOL)], (4.25, 1.45, 0), size=14, width=2.6)
        return [self.formula(self.focus, [row], (x, -.55, 0), size=14, width=3.4) for x, row in rows] + [legend]

    def _design_chip(self):
        """🪪 The design on the map as numbers: window share and overhang depth of the focus design."""
        return self._live_chip(lambda: ([self.tr("windows", "Fenster"), *self._p(self.focus["wwr"].get_value() * 100, "%", 1), "·", self.tr("overhang", "Vordach"),
                                         *self._p(self.focus["overhang"].get_value(), "m", 2)], (MAP_AT[0], 1.82, 0)), width=3.3, key=_room.design_key(self.focus), z=8)

    def _card(self, item, point, *, width=1.46, label=None, note=None, digits=(1, 2), power=1, ghost=False, ring=None, color=None, value=False, tail=(), exact=False):
        """🃏 A façade card: optional gene label and note, thumbnail in the map colour, design values and simulated electricity."""
        design, centre = item["design"], np.array([point[0], point[1], 0.0])
        height = 1.1 if not label else 1.25 if not note else 1.4
        top = height / 2
        frame = mn.Rectangle(width=width, height=height, color=ring or MUTED, stroke_width=2.8 if ring else 1.2, fill_color=DARK, fill_opacity=.92).move_to(centre)
        up = .2 if not label else top - (.52 if not note else .7)
        thumb = self.thumbnail(design, centre + mn.UP * up, .13 if not label else .12, state="ghost" if ghost else "filled", ramp=self.ramp_fn, value=None if ghost else item["electricity"])
        values_y, power_y = (-.22, -.42) if not label else (.37 - top, .15 - top) if not note else (.38 - top, .15 - top)
        group = mn.VGroup(frame, thumb, _room.text([*self._p(design["wwr"] * 100, "%", digits[0]), "·", *self._p(design["overhang"], "m", digits[1])], centre + mn.UP * values_y, size=14, width=width - .1))
        if label:
            group.add(_room.text(label, centre + mn.UP * (top - .13), size=14, color=color or INK, width=width - .1))
        if note:
            group.add(_room.text(note, centre + mn.UP * (top - .34), size=14, color=MUTED, width=width - .1))
        if value or not ghost:
            group.add(_room.text([*self._p(item["electricity"] if exact else _figures(design)["total"], "kWh/a", power), *tail], centre + mn.UP * power_y, size=14, width=width - .1))
        return group

    def _probe_row(self, rows, reader, *, search, y=ROW_Y):
        """🧪 The four neighbours of the current iterate as façade cards, each with its gene change; the winner of a compass round is ringed."""
        def build():
            index = reader()
            item, centre = rows[index], rows[max(index - (1 if search else 0), 0)]["design"]
            probes = {probe["direction"]: probe for probe in item["probes"]}
            group = mn.VGroup()
            for slot, direction in enumerate(("wwr-", "wwr+", "overhang-", "overhang+")):
                if direction not in probes:
                    continue
                probe, axis = probes[direction], direction[:-1]
                scale = 100 if axis == "wwr" else 1
                delta = (probe["design"][axis] - centre[axis]) * scale
                at = probe["design"][axis] * scale
                edge = search and (abs(at - BOUNDS[axis][0]) < 1e-6 or abs(at - BOUNDS[axis][1]) < 1e-6)
                digits = (0 if abs(delta - round(delta)) < 1e-6 else 1) if axis == "wwr" else (2 if search else 3)
                words = [self.tr("windows", "Fenster") if axis == "wwr" else self.tr("overhang", "Vordach"), self._signed(delta, "%" if axis == "wwr" else "m", digits)]
                note = None if not edge else [self.tr("limit: no move", "Rand: kein Zug")] if abs(delta) < 1e-9 else [self.tr("at limit", "am Rand"), self.quantity(at, "%" if axis == "wwr" else "m", 0 if axis == "wwr" else 2)]
                winner = search and item["moved"] and probe["design"] == item["design"]
                group.add(self._card(probe, (CARD_X + CARD_PITCH * (slot - 1.5), y, 0), width=1.68, label=words, note=note, digits=(1, 3) if not search else (1, 2),
                                     power=1 if search or axis == "wwr" else 3, exact=not search, ring=GOOD if winner else None, color=WINDOWS if axis == "wwr" else OVERHANG))
            return group
        return self._live(build, lambda: (reader(),))

    def _lens(self, item):
        """🔍 A magnifier on the map: the four probes around the start, 11 times enlarged, so their crosses can be told apart."""
        start, lens = self._pos(item["design"]), np.array(LENS_AT, dtype=float)
        group = mn.VGroup(mn.Circle(radius=LENS_R, color=INK, stroke_width=2, fill_color=DARK, fill_opacity=.92).move_to(lens))
        aim = (start - lens) / np.linalg.norm(start - lens)
        group.add(mn.Line(lens + aim * LENS_R, start, color=INK, stroke_width=1.4, stroke_opacity=.7))
        group.add(mn.Circle(radius=.05, color=INK, stroke_width=1.6).move_to(lens))
        for probe in item["probes"]:
            colour = WINDOWS if probe["direction"].startswith("wwr") else OVERHANG
            spot = lens + (self._pos(probe["design"]) - start) * LENS_ZOOM
            group.add(mn.DashedLine(lens, spot, color=colour, stroke_width=1.4, dash_length=.05, stroke_opacity=.6),
                      mn.Line(spot + mn.LEFT * .07 + mn.DOWN * .07, spot + mn.RIGHT * .07 + mn.UP * .07, color=colour, stroke_width=3),
                      mn.Line(spot + mn.LEFT * .07 + mn.UP * .07, spot + mn.RIGHT * .07 + mn.DOWN * .07, color=colour, stroke_width=3),
                      _room.text("+" if probe["direction"].endswith("+") else "−", spot + (mn.RIGHT * .17 if probe["direction"].startswith("over") else mn.UP * .17), size=14, color=colour))
        group.add(self._chip([self.tr("zoom", "Zoom"), "×", self._n(LENS_ZOOM)], lens + mn.UP * .85, size=14))
        return group.set_z_index(8)

    def _prepare(self):
        """🧱 Build the shared map, rooms, bar and counters."""
        self.d = _data()
        self.budget = self.d["budget"]
        self.start = self.d["gradient"][0]["design"]
        self.optimum = self.d["surface"]["minimum"]
        self.focus = self.study_trackers(self.start)
        self.following = False
        self.follower = mn.Mobject()
        self.follower.add_updater(lambda _: self._follow())
        self.add(self.follower)
        self.runs, self.total, self.stage, self.round = (mn.ValueTracker(value) for value in (1, self.budget, 0, 0))
        self.tape_colors = [INK] * self.budget
        self.path, self.colour, self.painted, self.ghosts, self.result_chips, self.trails = [], INK, [], [], mn.VGroup(), {}
        self.note = [self.tr("start design", "Startentwurf")]
        self.ramp_fn = self.ramp()
        self.ax, tick_labels = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label=None, y_label=None, point=MAP_AT, width=MAP_W, height=MAP_H, y_digits=1)
        titles = mn.VGroup(self.label("Window share (WWR) in %", "Fensteranteil (WWR) in %", (MAP_AT[0], -2.38, 0), size=17, width=MAP_W),
                           _room.place(_room.text(self.tr("Overhang depth in m", "Vordachtiefe in m"), size=15), (-6.45, 2.13, 0), "left"))
        self.legend_title = _room.place(_room.text(self.tr("Annual electricity", "Strombedarf"), size=15), (-6.45, 2.65, 0), "left")
        self.frame = mn.VGroup(self.ax, tick_labels, titles)
        self.heat = self.heat_map(self.ax, self.d["surface"], colour=self.ramp_fn, opacity=.9, levels=[550, 650, 750], labels=False, legend=(-3.35, 2.65, 0), legend_width=2.6,
                                  avoid=(self._pos(self.start),), ticks=(500, 600, 700, 800))
        self.contour_chips = _room.place(_room.text([self.tr("contour lines", "Höhenlinien"), *[self._n(level) for level in (550, 650, 750)], "kWh/a"], size=14, color=INK), (-4.1, 2.17, 0), "left")
        self.heat["image"].set_z_index(0)
        self.heat["contours"].set_z_index(1)
        self.contour_chips.set_z_index(1)
        self.heat["star"].set_z_index(5)
        self.star_pair = mn.VGroup(self.heat["star"], mn.VGroup(mn.Star(n=5, outer_radius=.11, inner_radius=.048, color=INK, stroke_width=1, fill_color=GOOD, fill_opacity=1).move_to((-1.75, 1.82, 0)),
                                        self._chip([self.tr("map minimum", "Minimum")], (-.8, 1.82, 0), size=14, color=GOOD, width=1.5)))
        self.fog_t = mn.ValueTracker(0)
        low_corner, high_corner = self.ax.c2p(10, 0), self.ax.c2p(80, 1.2)
        self.fog = mn.Rectangle(width=high_corner[0] - low_corner[0] + .02, height=high_corner[1] - low_corner[1] + .02, stroke_width=0, fill_color=DARK,
                                fill_opacity=0).move_to((low_corner + high_corner) / 2).set_z_index(2)
        self.fog.add_updater(lambda item: item.set_fill(DARK, opacity=self.fog_t.get_value()))
        self._mode_a()
        self.pointer = self._pointer()
        self.tape = self._tape()
        self.counter = self._counter()
        self.finals = {name: (self.d["best"][name][-1], self.d["series"][name][-1]) for name in ("gradient", "pattern", "genetic")}

    def _task(self):
        """🧑‍🏫 Pose the design question and its price."""
        self.add(self.fog)
        self.step("task", mn.FadeIn(self.frame), mn.FadeIn(self.seasons), self.reveal(*self.rooms, self.note_chip, self.design_chip), seconds=1.4)
        self.step("task_2", self.reveal(*self.ledger, self.bar), seconds=1.6)
        self.solver = self._chip([self.tr("Solver", "Solver")], (0.12, TAPE_Y, 0), size=14, width=1.95)
        self.step("task_3", mn.FadeIn(self.pointer), mn.FadeIn(self.tape), mn.FadeIn(self.counter), mn.FadeIn(self.solver))

    def _landscape(self):
        """🗺️ Reveal the full landscape, then hide it in the fog."""
        heat, runs_map = self.heat, self.d["convergence"]["runs_map"]
        spot = self._pos(self.optimum["design"])
        self.total.set_value(runs_map)
        self.step("map", mn.FadeOut(self.tape), mn.FadeOut(self.solver), mn.FadeIn(self.legend_title),
                  mn.LaggedStart(*[mn.FadeIn(column) for column in heat["image"]], lag_ratio=.05), mn.FadeIn(heat["contours"]), mn.FadeIn(self.contour_chips), mn.FadeIn(heat["legend"]),
                  self.runs.animate(rate_func=mn.smooth).set_value(runs_map), seconds=4.5)
        found = self._chip([*self._p(self.optimum["electricity"], "kWh/a", 1), "·", *self._p(self.optimum["design"]["wwr"] * 100, "%", 0), "·", *self._p(self.optimum["design"]["overhang"], "m", 2)],
                           (spot[0] - .45, spot[1] - .45, 0), color=GOOD, width=3.1).set_z_index(8)
        self.note[0] = self.tr("map minimum", "Kartenminimum")
        self.step("map_2", mn.FadeIn(self.star_pair), mn.FadeIn(found), self.set_design(self.focus, self.optimum["design"], 2.0), seconds=2.2)
        self.runs.set_value(1)
        self.total.set_value(self.budget)
        self.note[0] = self.tr("start design", "Startentwurf")
        self.step("map_3", mn.FadeOut(found), mn.FadeOut(self.star_pair), mn.FadeOut(self.contour_chips), self.fog_t.animate.set_value(.68), self.set_design(self.focus, self.start, 1.6),
                  mn.FadeIn(self.tape), mn.FadeIn(self.solver), seconds=1.8)

    def _result(self, name, design, electricity, runs, colour, strip, *, top=-1.72, extra=(), ledger=True, marks=()):
        """🏁 Replace the strip by the final design, its electricity and its gap to the map minimum."""
        minimum = self.optimum["electricity"]
        start = _figures(self.start)["total"]
        chips = mn.VGroup(
            self._chip([self.tr("best", "beste"), self.tr("windows", "Fenster"), *self._p(design["wwr"] * 100, "%", 1), "·", self.tr("overhang", "Vordach"), *self._p(design["overhang"], "m", 2), "·",
                        *self._p(electricity, "kWh/a", 1), self.tr("after", "nach"), self._n(runs), self.tr("simulations", "Simulationen")], (CARD_X, top - .26, 0), color=colour, width=6.6, size=14),
            self._chip([self.tr("start", "Start"), *self._p(start, "", 1)[:1], "→", *self._p(electricity, "kWh/a", 1), "·", self.tr("map minimum", "Kartenminimum"), *self._p(minimum, "", 1)[:1], "·",
                        self.tr("gap", "Abstand"), *self._p(self._gap(electricity), "kWh/a", 1)], (CARD_X, top - .66, 0), color=GOOD, width=6.6, size=14))
        ghost = self._ghost_trail(self.path, colour) if name != "genetic" else None
        self.note[0] = self.tr("best found", "bestes Ergebnis")
        shown = {id(member) for top in self.mobjects for member in top.get_family()}
        reveal = [] if not ledger or id(self.ledger[0]) in shown else [self.reveal(*self.ledger)]
        self.step(name + "_result", *self._fade(strip), *self._fade(extra), *reveal, mn.FadeIn(chips), self.set_design(self.focus, design, 1.0), self.fog_t.animate.set_value(.38),
                  mn.FadeIn(self.star_pair), *([mn.FadeIn(ghost)] if ghost is not None else []), *[mn.FadeIn(mark) for mark in marks])
        self.ghosts.extend(marks)
        if ghost is not None:
            self.ghosts.append(ghost)
            self.trails[name] = ghost
        self.result_chips = chips

    def _gradient(self):
        """📉 Walk downhill with measured slopes."""
        rows = self.d["gradient"]
        self.path = [(item["design"]["wwr"], item["design"]["overhang"]) for item in rows]
        self.tape_colors[0] = GRADIENT
        self.note[0] = self.tr("current design", "aktueller Entwurf")
        label = self._method("Gradient descent", "Gradientenabstieg", GRADIENT)
        self.step("gradient", mn.Transform(self.solver, label), mn.FadeIn(self._painted(self.start, rows[0]["electricity"])))
        reader = lambda: min(len(rows) - 1, int(self.stage.get_value() + 1e-6))
        cards, lens = self._probe_row(rows, reader, search=False), self._lens(rows[0])
        self.step("gradient_2", *self._fade(self.ledger), mn.FadeIn(lens), mn.FadeIn(cards, lag_ratio=.2), self._spend(1, 4, GRADIENT), seconds=1.8)
        slopes = self._slope_rows(rows[0])
        rule = self._chip([self.tr("step = −η · slope, η: one fixed step factor", "Schritt = −η · Steigung, η: ein fester Schrittfaktor")], (CARD_X, -.98, 0), size=18, width=6.7)
        self.step("gradient_3", *self._fade([cards]), mn.FadeIn(slopes), mn.FadeIn(rule))
        arrow = mn.Arrow(self._pos(rows[0]["design"]), self._pos(rows[1]["design"]), buff=.1, color=GRADIENT, stroke_width=4, max_tip_length_to_length_ratio=.12).set_z_index(5)
        steps = self._step_rows(rows)
        self.step("gradient_4", *self._fade([slopes, lens, rule]), mn.GrowArrow(arrow), mn.FadeIn(steps), self.reveal(*self.ledger), self.set_design(self.focus, rows[1]["design"], 1.8),
                  self.stage.animate(rate_func=mn.linear).set_value(1), mn.FadeIn(self._painted(rows[1]["design"], rows[1]["electricity"])), self._spend(5, 5, GRADIENT), seconds=2.0)
        target = self.optimum["design"]["wwr"] * 100
        line = mn.DashedLine(self.ax.c2p(target, 0), self.ax.c2p(target, 1.2), color=GOOD, stroke_width=2.4).set_z_index(3)
        note = self._chip([self.tr("valley floor", "Talboden"), *self._p(target, "%", 0), "·", *self._p(self.optimum["electricity"], "kWh/a", 0)], self.ax.c2p(target, .95) + mn.LEFT * 1.6,
                          color=GOOD, width=2.4).set_z_index(9)
        over = self._chip([self.tr("overshoot", "Überschuss"), *self._p(target - rows[1]["design"]["wwr"] * 100, "%", 1)],
                          self._pos(rows[1]["design"]) + mn.DOWN * .4 + mn.LEFT * .6, color=BAD, width=1.9).set_z_index(9)
        self.step("overshoot", mn.Create(line), mn.FadeIn(note), mn.FadeIn(over), mn.Flash(self._pos(rows[1]["design"]), color=GRADIENT, flash_radius=.35))
        moves = [mn.AnimationGroup(self.set_design(self.focus, rows[index]["design"], .6), self.stage.animate(rate_func=mn.linear, run_time=.6).set_value(index),
                                   mn.FadeIn(self._painted(rows[index]["design"], rows[index]["electricity"]), run_time=.6), self._spend(5 * index, 5, GRADIENT, .6))
                 for index in range(2, len(rows))]
        change = self._change_row(rows, reader)
        self.step("overshoot_2", *self._fade([arrow, line, note, over, steps]), mn.FadeIn(change), mn.Succession(*moves), seconds=5.0)
        self._result("gradient", *self.finals["gradient"], rows[-1]["evaluations"], GRADIENT, [change])
        self.tape_colors[:] = [GRADIENT if index % 5 == 0 else MUTED for index in range(self.budget)]
        legend = self._tape_legend()
        self.step("gradient_result_2", mn.FadeIn(legend), mn.Circumscribe(self.tape, color=INK, buff=.1))
        self.tape_legend = legend

    def _slope_rows(self, item):
        """📐 The two difference quotients of the first step, written with the probe values shown on the cards; the printed quotient is computed from the printed numbers."""
        out = mn.VGroup()
        for axis, colour, word, unit, scale, y in (("wwr", WINDOWS, self.tr("windows", "Fenster"), "%", 100, -1.43), ("overhang", OVERHANG, self.tr("overhang", "Vordach"), "m", 1, -2.03)):
            low, high = (next(probe for probe in item["probes"] if probe["direction"] == axis + sign) for sign in "-+")
            gap_digits, slope_digits = (1, 1) if axis == "wwr" else (3, 0)
            span = round((high["design"][axis] - low["design"][axis]) * scale, gap_digits)
            slope = item["slope"][axis]
            places = next((digits for digits in (1, 2, 3, 4) if round((round(high["electricity"], digits) - round(low["electricity"], digits)) / span, slope_digits) == round(slope, slope_digits)), 4)
            printed = (round(high["electricity"], places) - round(low["electricity"], places)) / span
            upper = self._line([(word, colour), "=", (f"({self.number(high['electricity'], places)}", colour), "−", (f"{self.number(low['electricity'], places)})", colour), "kWh/a", "÷",
                                (self.quantity(span, unit, gap_digits), colour)], (LEDGER_X, y, 0), size=18, align="left", width=6.7)
            lower = self._line([("=", INK), (self._p(printed, "kWh/a", slope_digits) + [self.tr("per " + unit, "je " + unit)], INK)], (LEDGER_X + .4, y - .3, 0), size=18, align="left", width=6.0)
            out.add(upper, lower)
        return out

    def _step_rows(self, rows):
        """👣 The first step as arithmetic on both genes and on the electricity, each row closing with the printed numbers."""
        before, after = rows[0], rows[1]

        def gene(axis, scale, digits, unit, colour, word):
            old, new = round(before["design"][axis] * scale, digits), round(after["design"][axis] * scale, digits)
            sign = "+" if new > old else "−"
            return self._line([(word, colour), (self._p(old, unit, digits), INK), (sign, INK), (self._p(abs(new - old), unit, digits), colour), "=", (self._p(new, unit, digits), INK)],
                              (LEDGER_X, 0, 0), size=18, align="left", width=6.0)
        old, new = _figures(before["design"])["total"], _figures(after["design"])["total"]
        out = mn.VGroup(gene("wwr", 100, 1, "%", WINDOWS, self.tr("windows", "Fenster")), gene("overhang", 1, 2, "m", OVERHANG, self.tr("overhang", "Vordach")),
                        self._line([(self.tr("electricity", "Strom"), INK), (self._p(old, "", 1)[:1], INK), "−", (self._p(old - new, "", 1)[:1], GOOD), "=", (self._p(new, "kWh/a", 1), INK)],
                                   (LEDGER_X, 0, 0), size=18, align="left", width=6.0))
        for slot, line in enumerate(out):
            _room.place(line, (LEDGER_X, -1.95 - .27 * slot, 0), "left")
        return out

    def _change_row(self, rows, reader):
        """📈 Live change of electricity against the previous iterate, signed."""
        def build():
            index = reader()
            if index == 0:
                return mn.VGroup()
            delta = _figures(rows[index]["design"])["total"] - _figures(rows[index - 1]["design"])["total"]
            colour = GOOD if delta < 0 else BAD
            return mn.VGroup(self._chip([self.tr("change to the previous iterate", "Änderung zum vorigen Iterat"), self._signed(_half(delta), "kWh/a", 1)], (CARD_X, ROW_Y - .36, 0), color=colour, width=4.2, size=15))
        return self._live(build, lambda: (reader(),))

    def _tape_legend(self):
        """🧾 Step and probe colours of the gradient tape."""
        items = mn.VGroup()
        for word, colour, x in ((self.tr("step", "Schritt"), GRADIENT, 1.5), (self.tr("probe", "Probe"), MUTED, 2.5)):
            label = _room.place(_room.text([word], size=14, color=INK), (x, 2.2, 0), "left")
            items.add(mn.Square(side_length=.11, stroke_width=0, fill_color=colour, fill_opacity=.95).move_to(label.get_left() + mn.LEFT * .1), label)
        return items

    def _compass(self, rows, index):
        """🎯 Dashed arms and ringed winner of one compass round; the probes are returned as painted dots."""
        centre, item = rows[index - 1]["design"], rows[index]
        arms = mn.VGroup(*[mn.DashedLine(self._pos(centre), self._pos(probe["design"]), color=PATTERN, stroke_width=2, dash_length=.07) for probe in item["probes"]])
        for probe in item["probes"]:
            if item["moved"] and probe["design"] == item["design"]:
                arms.add(mn.Circle(radius=.13, color=GOOD, stroke_width=2.6).move_to(self._pos(probe["design"])))
        arms.set_z_index(4)
        return arms, [self._painted(probe["design"], probe["electricity"], .06) for probe in item["probes"]]

    def _pattern(self):
        """🧭 Compare four neighbours and halve the step when none wins."""
        rows = self.d["pattern"]
        self.path = [(item["design"]["wwr"], item["design"]["overhang"]) for item in rows]
        old_dots, self.painted = self.painted, []
        label = self._method("Pattern search", "Mustersuche", PATTERN)
        self.stage.set_value(0)
        self.round.set_value(1)
        self.runs.set_value(1)
        self.tape_colors[:] = [PATTERN] * self.budget
        self.note[0] = self.tr("current design", "aktueller Entwurf")
        self.step("pattern", *self._fade([self.result_chips, self.tape_legend, *old_dots, self.star_pair, *self.ghosts]), mn.Transform(self.solver, label), self.set_design(self.focus, self.start, 1.4),
                  self.fog_t.animate.set_value(.68), mn.FadeIn(self._painted(self.start, rows[0]["electricity"])), seconds=1.6)
        reader = lambda: min(len(rows) - 1, int(self.round.get_value() + 1e-6))
        cards, band = self._probe_row(rows, reader, search=True, y=-1.93), self._pattern_band(rows, reader)
        rule = self._chip([self.tr("try the four neighbours: ± step in windows and in overhang", "vier Nachbarn probieren: ± Schritt bei Fenster und Vordach")], (CARD_X, -.88, 0), size=18, width=6.7)
        arms, dots = self._compass(rows, 1)
        self.step("pattern_2", *self._fade(self.ledger), mn.FadeIn(cards, lag_ratio=.2), mn.FadeIn(rule), mn.Create(arms), *[mn.FadeIn(dot) for dot in dots], self._spend(1, 4, PATTERN), seconds=1.8)
        self.step("pattern_3", mn.FadeOut(rule), mn.FadeIn(band), self.set_design(self.focus, rows[1]["design"], 1.6), self.stage.animate(rate_func=mn.linear).set_value(1), seconds=1.8)
        stall = next((index for index in range(1, len(rows)) if not rows[index]["moved"]), len(rows) - 2)
        rounds, current = [], arms
        for index in range(2, stall + 1):
            fresh, fresh_dots = self._compass(rows, index)
            rounds.append(mn.AnimationGroup(self.round.animate(run_time=.01).set_value(index), mn.FadeOut(current), mn.FadeIn(fresh), *[mn.FadeIn(dot, run_time=.5) for dot in fresh_dots],
                                            self.set_design(self.focus, rows[index]["design"], .5), self.stage.animate(rate_func=mn.linear, run_time=.5).set_value(index),
                                            self._spend(1 + 4 * (index - 1), 4, PATTERN, .5)))
            current = fresh
        self.step("pattern_4", mn.Succession(*rounds), seconds=4.5)
        montage = []
        for index in range(stall + 1, len(rows)):
            fresh, fresh_dots = self._compass(rows, index)
            montage.append(mn.AnimationGroup(self.round.animate(run_time=.01).set_value(index), mn.FadeOut(current, run_time=.35), mn.FadeIn(fresh, run_time=.35),
                                             *[mn.FadeIn(dot, run_time=.35) for dot in fresh_dots], self.set_design(self.focus, rows[index]["design"], .35),
                                             self.stage.animate(rate_func=mn.linear, run_time=.35).set_value(index), self._spend(1 + 4 * (index - 1), 4, PATTERN, .35)))
            current = fresh
        minimum = self.optimum["electricity"]
        remaining = self._live(lambda: mn.VGroup(self._line([(self.tr("after: still", "nachher: noch"), INK), (self._p(_figures(rows[reader()]["design"])["total"] - round(minimum, 1), "kWh/a", 1), BAD),
                                                             (self.tr("above the map minimum", "über dem Kartenminimum"), INK), (self._p(minimum, "kWh/a", 1), GOOD)], (CARD_X, -1.1, 0), size=18, width=6.7)),
                               lambda: (reader(),))
        self.step("pattern_5", mn.FadeIn(remaining), mn.Succession(*montage), seconds=4.0)
        reached = self.d["reach"]["pattern"]
        spot = self._pos(self.d["best"]["pattern"][reached - 1])
        marks = [mn.Circle(radius=.2, color=PATTERN, stroke_width=3).move_to(spot).set_z_index(6),
                 self._chip([self.tr("ring: within 1 % at", "Ring: innerhalb 1 % bei"), self._n(reached)], (5.85, 2.15, 0), color=PATTERN, size=14, width=2.2)]
        self._result("pattern", *self.finals["pattern"], rows[-1]["evaluations"], PATTERN, [cards, band, remaining], extra=[current], marks=marks)

    def _pattern_band(self, rows, reader):
        """⚖️ The decision of the current compass round: the design before the move and the design after it, both with their values, or the halved step."""
        def build():
            index = reader()
            item, previous = rows[index], rows[max(index - 1, 0)]
            if item["moved"]:
                return mn.VGroup(self._line([(self.tr("before", "vorher"), MUTED), (self._p(_figures(previous["design"])["total"], "kWh/a", 1), INK), "→", (self.tr("after", "nachher"), GOOD),
                                             (self._p(_figures(item["design"])["total"], "kWh/a", 1), GOOD), (self.tr("(moved to the best neighbour)", "(zum besten Nachbarn gezogen)"), MUTED)],
                                            (CARD_X, -.8, 0), size=18, width=6.7))
            step = item["step"]["wwr"] * 100
            digits = lambda value: 0 if abs(value - round(value)) < 1e-6 else 1
            return mn.VGroup(self._line([self.tr("no neighbour below", "kein Nachbar unter"), (self._p(_figures(previous["design"])["total"], "kWh/a", 1), INK), "→", (self.tr("no improvement", "keine Besserung"), BAD)],
                                        (CARD_X, -.8, 0), size=18, width=6.7),
                             self._line([(self.tr("step ÷ 2:", "Schritt ÷ 2:"), BAD), (self._p(step, "", digits(step)), INK), "→", (self._p(step / 2, "%", digits(step / 2)), BAD), "·",
                                         (self._p(item["step"]["overhang"], "", 2), INK), "→", (self._p(item["step"]["overhang"] / 2, "m", 2), BAD)], (CARD_X, -1.1, 0), size=18, width=6.7))
        return self._live(build, lambda: (reader(),))

    def _card_live(self, item, point):
        """🃏 A card that stays a dashed ghost until its simulation has been counted on the tape."""
        born = self.d["born"][item["id"]]
        return self._live(lambda: mn.VGroup(self._card(item, point, ghost=self.runs.get_value() < born - .001)), lambda: self.runs.get_value() >= born - .001)

    def _dot_live(self, item):
        """⭕ A ring on the map for one individual: hollow until simulated, tiny and faint once it left the population."""
        born, identifier = self.d["born"][item["id"]], item["id"]
        member_of = lambda: identifier in self.d["members"][min(round(self.gen.get_value()), len(self.d["members"]) - 1)]

        def build():
            evaluated, member = self.runs.get_value() >= born - .001, member_of()
            if not evaluated and not member:
                return mn.VGroup()
            circle = mn.Circle(radius=.1 if member else .045, color=GENETIC, stroke_width=2.4 if member else 1.0, stroke_opacity=1 if member else .4).move_to(self._pos(item["design"])).set_z_index(5)
            if not evaluated:
                return mn.VGroup(mn.DashedVMobject(circle, num_dashes=14).set_z_index(5))
            circle.set_fill(self.ramp_fn(item["electricity"]), opacity=.95 if member else .3)
            return mn.VGroup(circle)
        return self._live(build, lambda: (self.runs.get_value() >= born - .001, member_of()))

    def _advance(self, generation):
        """🔁 The cards of ``generation``'s population, sorted by electricity from the top left: every card is replaced, so the order always shows the ranking."""
        leaving = list(self.cards.values())
        self.cards.clear()
        self.slot_of.clear()
        entering = []
        for slot, item in enumerate(sorted(generation["population"], key=lambda member: member["electricity"])):
            self.slot_of[item["id"]] = slot
            self.cards[item["id"]] = self._card_live(item, SLOTS[slot])
            entering.append(self.cards[item["id"]])
        return leaving, entering

    def _duel(self, generation, duel, colour, y):
        """⚔️ Two cards face each other; the one with less electricity wins."""
        lookup = {item["id"]: item for item in generation["population"]}
        cards = [self._card(lookup[identifier], (x, y, 0), ring=colour if identifier == duel[2] else None) for identifier, x in zip(duel[:2], (CARD_X - 1.35, CARD_X + 1.35))]
        return mn.VGroup(*cards, _room.text("vs", (CARD_X, y, 0), size=17))

    def _hull(self, points):
        """🔷 A padded convex outline around map points."""
        pts = sorted({(round(float(p[0]), 4), round(float(p[1]), 4)) for p in points})

        def cross(o, a, b):
            return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
        lower, upper = [], []
        for p in pts:
            while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
                lower.pop()
            lower.append(p)
        for p in reversed(pts):
            while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
                upper.pop()
            upper.append(p)
        hull = lower[:-1] + upper[:-1]
        centre = np.mean(hull, axis=0)
        left, right, bottom, top = self.ax.get_left()[0], self.ax.get_right()[0], self.ax.get_bottom()[1], self.ax.get_top()[1]
        padded = [centre + (np.array(p) - centre) * 1.18 for p in hull]
        padded = [(min(max(p[0], left), right), min(max(p[1], bottom), top), 0) for p in padded]
        return mn.DashedVMobject(mn.Polygon(*padded, color=GENETIC, stroke_width=2.4), num_dashes=70).set_z_index(5)

    def _chips_live(self):
        """⏱️ Generation counter and best electricity simulated so far (from the run counter), right of the cards."""
        series = self.d["series"]["genetic"]

        def best():
            run = int(self.runs.get_value() + 1e-6)
            if run < 1:
                return ([self.tr("best: none yet", "noch kein Bestwert")], (5.75, 1.6, 0))
            return ([self.tr("best", "Bestwert"), *self._p(series[min(run, len(series)) - 1], "kWh/a", 1)], (5.75, 1.6, 0))
        generation = self._live_chip(lambda: ([self.tr("Generation", "Generation"), self._n(round(self.gen.get_value())), self.tr("of", "von"),
                                               self._n(len(self.d["generations"]) - 1)], (5.75, 2.0, 0)), size=14, width=2.2)
        return self._live_chip(best, color=GENETIC, width=1.8), generation

    def _genetic(self):
        """🧬 Population of façade cards: selection, crossover, mutation, survival and generations."""
        generations, first = self.d["generations"], self.d["generations"][0]
        self.gen = mn.ValueTracker(0)
        label = self._method("Genetic algorithm", "Genetischer Algorithmus", GENETIC)
        old_dots, self.painted = self.painted, []
        self.small = [self.room_section(self.focus, at, .19, season, labels=False) for at, season in zip(((.1, -1.62, 0), (3.55, -1.62, 0)), ("winter", "summer"))]
        self.sum_row = self.sum_formula((CARD_X, -2.5, 0))
        self.cards, self.slot_of = {}, {}
        _, entering = self._advance(first)
        self.cloud = {identifier: self._dot_live(item) for identifier, item in self.d["individuals"].items()}
        self.stage.set_value(0)
        self.path = [(first["population"][0]["design"]["wwr"], first["population"][0]["design"]["overhang"])]
        self.tape_colors[:] = [GENETIC] * self.budget
        self.runs.set_value(0)
        self.best_chip, self.gen_chip = self._chips_live()
        genes = self._chip([self.tr("genes", "Gene") + ":", self.tr("window share", "Fensteranteil"), "·", self.tr("overhang", "Vordach")], (CARD_X, -.2, 0), color=GENETIC, width=4.6)
        self.genes = genes
        first_dots = [self.cloud[item["id"]] for item in first["population"]]
        self.add(*[dot for dot in self.cloud.values() if dot not in first_dots])
        self.step("population", *self._fade([self.pointer, self.result_chips, *self.ghosts, *old_dots, self.star_pair, *self.rooms, self.seasons, *self.ledger, self.bar, self.note_chip, self.design_chip]),
                  mn.Transform(self.solver, label), mn.FadeIn(self.best_chip), self.fog_t.animate.set_value(.68), mn.FadeIn(genes),
                  mn.LaggedStart(*[mn.FadeIn(card) for card in entering], lag_ratio=.1), mn.LaggedStart(*[mn.FadeIn(dot) for dot in first_dots], lag_ratio=.1), seconds=2.4)
        self.ghosts.clear()
        self._set_focus(first["population"][0]["design"])
        self.following = True
        late = mn.squish_rate_func(mn.smooth, .15, .45)
        widgets = [*self.small, self.bar, self.sum_row, self.gen_chip]
        for widget in widgets:
            widget.update(0)
        self.step("population_2", self._spend(0, 8, GENETIC, 2.4), *[mn.FadeIn(widget, rate_func=late) for widget in widgets], seconds=2.4)
        hull = self._hull([self._pos(item["design"]) for item in first["population"]])
        self.step("population_3", mn.Create(hull, run_time=1.2))
        self.hull = hull
        self._selection(generations)
        self._breeding(generations)

    def _selection(self, generations):
        """🥊 Recorded duels pick the parents of the featured child; one more duel lets a mid-ranked card win."""
        first, ranks = generations[0], self.d["ranks"]
        self.feature = first["children"][0]
        duels = [duel for duel in first["tournaments"] if duel[0] != duel[1]]
        picks = [next(duel for duel in duels if duel[2] == parent) for parent in self.feature["from"]]
        weak = max(duels, key=lambda duel: (ranks[duel[2]], ranks[next(i for i in duel[:2] if i != duel[2])]))
        loser = next(identifier for identifier in weak[:2] if identifier != weak[2])
        rank_chip = self._chip([self.tr("rank", "Rang"), self._n(ranks[weak[2]]), self.tr("of", "von"), self._n(8), self.tr("beats rank", "schlägt Rang"), self._n(ranks[loser])], (CARD_X, .2, 0), color=GENETIC, width=4.2)
        pair = mn.VGroup(self._duel(first, picks[0], PARENT_A, 1.72), self._duel(first, picks[1], PARENT_B, .5))
        self.duels = mn.VGroup(self._duel(first, weak, STAR, 1.15), rank_chip)
        self.rule_sel = self._chip([self.tr("parent = lower electricity of two random cards", "Elternteil = weniger Strom von zwei zufälligen Karten")], (CARD_X, -.4, 0), size=18, width=6.7)
        self.step("tournament", *self._fade([self.hull, self.genes]), *[mn.FadeOut(card) for card in self.cards.values()], mn.FadeIn(pair), mn.FadeIn(self.rule_sel), seconds=1.6)
        self.step("tournament_2", mn.FadeOut(pair), mn.FadeIn(self.duels), seconds=1.6)

    def _mutation_cards(self, crossed, crossed_e, item, tone, run):
        """🃏 The what-if child of the crossover next to the simulated child after its mutation."""
        before = self._card({"design": crossed, "electricity": crossed_e}, (CARD_X - 1.25, .4, 0), width=2.2, label=[self.tr("crossover only", "nur Kreuzung")], digits=(2, 2), ghost=True, value=True,
                            note=[self.tr("what-if, not simulated", "nicht simuliert")])
        shift = item["mutation"]
        moved = [(self._signed(shift["wwr"] * 100, "%", 1), self.tr("windows", "Fenster"))] * (abs(shift["wwr"]) > 1e-9) + [(self._signed(shift["overhang"], "m", 2), self.tr("overhang", "Vordach"))] * (abs(shift["overhang"]) > 1e-9)
        words = [self.tr("nudge", "Mutation"), moved[0][0], "(" + moved[0][1] + ")"] if len(moved) == 1 else [self.tr("nudge", "Mutation"), moved[0][0], "·", moved[1][0]]
        after = self._card(item, (CARD_X + 1.25, .4, 0), width=2.2, label=words, digits=(2, 2), ring=tone, color=tone, tail=[self.tr("run", "Lauf"), self._n(run)])
        return mn.VGroup(before, after)


    def _rule_mutation(self):
        """📜 The mutation rule with the study's amplitudes and chance."""
        settings = _model.load_study()["algorithms"]["genetic"]["mutation"]
        return self._chip([self.tr("mutate:", "Mutation:"), self.tr("windows", "Fenster") + " ±" + self.number(settings["wwr"] * 100, 0) + " %,", self.tr("overhang", "Vordach") + " ±" + self.quantity(settings["overhang"], "m", 1) + ",",
                           "p = " + self.number(settings["probability"], 1)], (CARD_X, -.58, 0), size=18, width=6.7)

    def _breeding(self, generations):
        """🧬 Crossover, mutation, six children, survival and the generations."""
        first = generations[0]
        lookup = {item["id"]: item for item in first["population"]}
        child = self.feature
        id_a, id_b = child["from"]
        parent_a, parent_b = lookup[id_a], lookup[id_b]
        colour_of = lambda identifier: PARENT_A if identifier == id_a else PARENT_B
        letter_of = lambda identifier: "A" if identifier == id_a else "B"
        donors = {key: lookup[child["genes"][key]]["design"][key] for key in ("wwr", "overhang")}
        crossed = {**parent_a["design"], **donors}
        crossed_e = _room.result(crossed)["electricity"]
        xs = (CARD_X - 2.0, CARD_X + 2.0)
        faces = [self.facade(self.study_trackers(parent["design"]), (x, 1.5, 0), .2, "summer", labels=False) for parent, x in zip((parent_a, parent_b), xs)]
        names = mn.VGroup(*[self._chip([self.tr("parent", "Elternteil"), letter, *self._p(_room.result(parent["design"])["electricity"], "kWh/a", 1)], (x, 2.12, 0), color=colour, width=2.3)
                            for parent, x, colour, letter in zip((parent_a, parent_b), xs, (PARENT_A, PARENT_B), "AB")])
        genes, gene_at = mn.VGroup(), {}
        for parent, x in zip((parent_a, parent_b), xs):
            for key, shift, word, unit, digits, scale in (("wwr", -.85, self.tr("windows", "Fenster"), "%", 1, 100), ("overhang", .85, self.tr("overhang", "Vordach"), "m", 2, 1)):
                chip = self._chip([word, *self._p(parent["design"][key] * scale, unit, digits)], (x + shift, .85, 0), color=colour_of(parent["id"]), size=14, width=1.55)
                if child["genes"][key] != parent["id"]:
                    chip.set_opacity(.3)
                else:
                    gene_at[key] = chip.get_bottom()
                genes.add(chip)
        child_chips = mn.VGroup(
            self._chip([self.tr("child windows", "Kind Fenster"), *self._p(crossed["wwr"] * 100, "%", 1)], (CARD_X - 1.1, .3, 0), size=14, color=colour_of(child["genes"]["wwr"]), width=1.95),
            self._chip([self.tr("child overhang", "Kind Vordach"), *self._p(crossed["overhang"], "m", 2)], (CARD_X + 1.1, .3, 0), size=14, color=colour_of(child["genes"]["overhang"]), width=1.95))
        arrows = mn.VGroup(*[mn.Arrow(gene_at[key], target + mn.UP * .02, buff=.03, color=colour_of(child["genes"][key]), stroke_width=3.2, tip_length=.13, max_tip_length_to_length_ratio=.4)
                             for key, target in (("wwr", np.array([CARD_X - 1.1, .47, 0.0])), ("overhang", np.array([CARD_X + 1.1, .47, 0.0])))])
        rule = self._line([(self.tr("child = (windows of", "Kind = (Fenster von"), INK), (letter_of(child["genes"]["wwr"]) + ",", colour_of(child["genes"]["wwr"])),
                           (self.tr("overhang of", "Vordach von"), INK), (letter_of(child["genes"]["overhang"]) + ")", colour_of(child["genes"]["overhang"]))], (CARD_X, -.1, 0), size=18, width=6.7)
        unseen = self._line([(self.tr("not simulated yet: the counter still reads", "noch nicht simuliert: der Zähler steht auf"), MUTED), (self._n(8), MUTED)], (CARD_X, -.36, 0), size=14, width=6.0)
        parts = mn.VGroup(*faces, names)
        self.path = [(parent_a["design"]["wwr"], parent_a["design"]["overhang"]), (crossed["wwr"], crossed["overhang"])]
        self.stage.set_value(0)
        pointer = self._pointer()
        parent_rings = mn.VGroup(*[mn.Circle(radius=.19, color=colour, stroke_width=3).move_to(self._pos(parent["design"])).set_z_index(5) for parent, colour in ((parent_a, PARENT_A), (parent_b, PARENT_B))])
        what_if = mn.DashedVMobject(mn.Circle(radius=.13, color=INK, stroke_width=3).move_to(self._pos(crossed)), num_dashes=12).set_z_index(7)
        self.following = False
        self._set_focus(parent_a["design"])
        self.add(pointer)
        bar_note = self._chip([self.tr("bar: parent A", "Balken: Elternteil A")], (BAR_AT[0] - .15, -2.33, 0), size=14, width=1.8)
        self.step("crossover", *self._fade([self.duels, self.rule_sel]), mn.FadeOut(self.gen_chip), mn.FadeIn(bar_note),
                  mn.Succession(mn.AnimationGroup(mn.FadeIn(parts, run_time=.8), mn.FadeIn(genes, run_time=.8), mn.FadeIn(parent_rings, run_time=.8), mn.FadeIn(child_chips, run_time=.8)),
                                mn.AnimationGroup(*[mn.Create(arrow, run_time=1.0) for arrow in arrows], mn.FadeIn(rule, run_time=1.0), mn.FadeIn(unseen, run_time=1.0), mn.FadeIn(what_if, run_time=1.0))), seconds=5.0)
        wa, wb = parent_a["design"], parent_b["design"]
        corners = [(wa["wwr"], wa["overhang"]), (wb["wwr"], wa["overhang"]), (wb["wwr"], wb["overhang"]), (wa["wwr"], wb["overhang"])]
        box = mn.DashedVMobject(mn.Polygon(*[self.ax.c2p(c[0] * 100, c[1]) for c in corners], color=INK, stroke_width=3), num_dashes=40).set_z_index(7)
        self.step("crossover_2", mn.FadeIn(box), mn.Flash(self._pos(crossed), color=GENETIC, flash_radius=.35))
        better = child["electricity"] < crossed_e
        tone = GOOD if better else BAD
        cards = self._mutation_cards(crossed, crossed_e, child, tone, self.d["born"][child["id"]])
        nudge = mn.Arrow(self._pos(crossed), self._pos(child["design"]), buff=.04, color=tone, stroke_width=4, max_tip_length_to_length_ratio=.4).set_z_index(9)
        rule_mutation = self._rule_mutation()
        self.step("mutation", mn.FadeOut(bar_note), mn.FadeOut(genes), mn.FadeOut(arrows), mn.FadeOut(child_chips), mn.FadeOut(rule), mn.FadeOut(unseen), mn.FadeIn(cards[0]), mn.FadeIn(rule_mutation),
                  mn.Succession(mn.AnimationGroup(mn.GrowArrow(nudge), self.stage.animate(rate_func=mn.linear).set_value(1), self.set_design(self.focus, child["design"], 1.2),
                                                  self._spend(8, 1, GENETIC, 1.2)), mn.FadeIn(cards[1], run_time=.5)), seconds=2.4)
        self.following = True
        self._children(generations, [cards, nudge, box, *parts, parent_rings, pointer, what_if, rule_mutation])

    def _children(self, generations, leftovers):
        """👶 Six children replace the non-elite cards, then the generations run on."""
        first, second = generations[0], generations[1]
        old = dict(self.cards)
        leaving, entering = self._advance(second)
        for card in (*old.values(), self.gen_chip):
            card.update(0)
        born = self.runs.get_value()
        steps = mn.Succession(
            mn.AnimationGroup(*[mn.FadeIn(card, run_time=.7) for card in old.values()], mn.FadeIn(self.gen_chip, run_time=.7)),
            mn.AnimationGroup(*[mn.FadeOut(card, run_time=.6) for card in leaving], *[mn.FadeIn(card, run_time=.6) for card in entering]),
            mn.AnimationGroup(self._spend(int(born), 14 - int(born), GENETIC, 1.8), self.gen.animate(run_time=1.8).set_value(1)))
        self.step("children", *self._fade(leftovers), *self._fade([self.duels]), steps, seconds=5.0)
        self.gen.set_value(1)
        elites = first["elites"]
        corner = lambda identifier: np.array([*SLOTS[self.slot_of[identifier]], 0.0]) + np.array([-.58, .48, 0.0])
        best_id = min(second["population"], key=lambda item: item["electricity"])["id"]
        markers = mn.VGroup(*[mn.Rectangle(width=1.4, height=1.1, color=STAR, stroke_width=3.4).move_to(np.array([*SLOTS[self.slot_of[key]], 0.0])) for key in elites],
                            mn.Rectangle(width=1.4, height=1.1, color=GOOD, stroke_width=3.4).move_to(np.array([*SLOTS[self.slot_of[best_id]], 0.0])),
                            *[mn.Star(n=5, outer_radius=.12, inner_radius=.05, color=INK, stroke_width=1, fill_color=STAR, fill_opacity=1).move_to(corner(key)) for key in elites],
                            self._chip([self.tr("yellow: kept, the best two parents of generation 0", "gelb: behalten, die zwei besten Eltern der Generation 0")], (CARD_X, -.2, 0), color=STAR, width=6.4),
                            self._chip([self.tr("green: best card of generation 1", "grün: beste Karte der Generation 1")], (CARD_X, -.55, 0), color=GOOD, width=6.4)).set_z_index(9)
        self.step("survival", mn.FadeIn(markers))
        self._later(generations, markers)

    def _later(self, generations, markers):
        """🎲 A child of generation 2 whose mutation makes it worse: simulated at its own run, with its parents and the what-if next to it."""
        second_generation = generations[1]
        lookup = {item["id"]: item for item in second_generation["population"]}
        options = []
        for item in second_generation["children"]:
            if any(item["mutation"].values()):
                cross = {**lookup[item["from"][0]]["design"], **{key: lookup[item["genes"][key]]["design"][key] for key in ("wwr", "overhang")}}
                options.append((item, cross, item["electricity"] - _room.result(cross)["electricity"]))
        child, crossed, _ = max(options, key=lambda option: option[2])
        crossed_e = _room.result(crossed)["electricity"]
        tone = BAD if child["electricity"] > crossed_e else GOOD
        parents = [lookup[identifier] for identifier in child["from"]]
        xs = (CARD_X - 2.0, CARD_X + 2.0)
        faces = [self.facade(self.study_trackers(parent["design"]), (x, 1.5, 0), .2, "summer", labels=False) for parent, x in zip(parents, xs)]
        names = [self._chip([self.tr("parent", "Elternteil"), letter, *self._p(_room.result(parent["design"])["electricity"], "kWh/a", 1)], (x, 2.12, 0), color=colour, width=2.3)
                 for parent, x, colour, letter in zip(parents, xs, (PARENT_A, PARENT_B), "AB")]
        cards = self._mutation_cards(crossed, crossed_e, child, tone, self.d["born"][child["id"]])
        arrow = mn.Arrow(self._pos(crossed), self._pos(child["design"]), buff=.04, color=tone, stroke_width=4, max_tip_length_to_length_ratio=.4).set_z_index(9)
        born = self.d["born"][child["id"]]
        rule = self._rule_mutation()
        self.following = False
        self.later = mn.VGroup(*faces, *names, cards, arrow, rule)
        self.step("mutation_2", *[mn.FadeOut(card) for card in self.cards.values()], mn.FadeOut(markers), mn.FadeOut(self.gen_chip), mn.FadeIn(mn.VGroup(*faces, *names)), mn.FadeIn(cards[0]), mn.FadeIn(rule),
                  mn.Succession(mn.AnimationGroup(mn.GrowArrow(arrow), self._spend(14, born - 14, GENETIC, 1.2), self.set_design(self.focus, child["design"], 1.2)), mn.FadeIn(cards[1], run_time=.5),
                                mn.Flash(self._pos(child["design"]), color=GENETIC, flash_radius=.35)), seconds=2.6)
        self._generations(generations)

    def _generations(self, generations):
        """🔁 Run the remaining generations as a montage."""
        montage = []
        for index in range(2, len(generations)):
            leaving, entering = self._advance(generations[index])
            parts = [*[mn.FadeOut(card, run_time=.5) for card in leaving], *[mn.FadeIn(card, run_time=.5) for card in entering], self.gen.animate(run_time=.5).set_value(index),
                     self._spend(8 + 6 * (index - 1), 6, GENETIC, .6)]
            if index == 2:
                parts += [mn.FadeOut(self.later, run_time=.5), *[mn.FadeIn(card, run_time=.5) for card in self.cards.values() if card not in entering], mn.FadeIn(self.gen_chip, run_time=.5)]
            montage.append(mn.AnimationGroup(*parts))
        for card in (*self.cards.values(), self.gen_chip):
            card.update(0)
        self.following = True
        self.history = self.formula(self.focus, [[(self.tr("Generation", "Generation"), MUTED), *[(str(index), INK) for index in range(len(generations))]],
                                                 [(self.tr("best kWh/a", "beste kWh/a"), MUTED), *[self.live(item["best"]["electricity"], GENETIC, 1) for item in generations]]],
                                    (CARD_X, -.2, 0), size=14, width=5.6)
        self.step("generations", mn.Succession(*montage, mn.FadeIn(self.history, run_time=.8)), seconds=6.5)
        self.runs.set_value(generations[-1]["evaluations"])
        self._genetic_result(generations)

    def _genetic_result(self, generations):
        """🏁 Show the genetic result and how many runs each method needed."""
        best = generations[-1]["best"]
        self.following = False
        self._result("genetic", best["design"], best["electricity"], generations[-1]["evaluations"], GENETIC, [*self.cards.values(), self.history], top=1.5, ledger=False)
        reach = self.d["reach"]
        header = self._chip([self.tr("Simulations until within 1 % of the map minimum", "Simulationen bis 1 % über dem Kartenminimum")], (CARD_X + .55, 2.0, 0), width=4.1)
        rows = mn.VGroup()
        for index, (key, en, de, colour) in enumerate((("gradient", "Gradient descent", "Gradientenabstieg", GRADIENT), ("pattern", "Pattern search", "Mustersuche", PATTERN),
                                                       ("genetic", "Genetic algorithm", "Genetischer Algorithmus", GENETIC))):
            y = 1.5 - .62 * index
            width = 3.9 * (reach[key] or self.budget) / self.budget
            rows.add(self._chip([self.tr(en, de), *([self._n(reach[key]), self.tr("simulations", "Simulationen")] if reach[key] else [self.tr("not reached", "nicht erreicht")])], (CARD_X, y, 0), color=colour, width=3.9),
                     mn.Rectangle(width=3.9, height=.1, stroke_width=1, color=MUTED).move_to((CARD_X, y - .27, 0)),
                     mn.Rectangle(width=width, height=.1, stroke_width=0, fill_color=colour, fill_opacity=.95).move_to((CARD_X - 1.95 + width / 2, y - .27, 0)))
        self.step("genetic_result_2", *self._fade([self.result_chips]), mn.FadeIn(rows), mn.FadeIn(header))
        self.reach_rows = mn.VGroup(rows, header)

    def _compare(self):
        """📈 Best electricity found so far, against simulations, for all three methods."""
        self._clear()
        series, budget = self.d["series"], self.budget
        low = math.floor((self.optimum["electricity"] - 20) / 100) * 100
        high = math.ceil(max(max(values) for values in series.values()) / 100) * 100
        ax, ticks = self.plot_axes(x_range=(0, budget, 10), y_range=(low, high, 100), x_label=None, y_label=None, point=(-3.3, -.1, 0), width=5.8, height=3.4,
                                   x_ticks=list(range(0, budget + 1, 10)), y_ticks=list(range(low, high + 1, 100)))
        titles = mn.VGroup(self.label("Simulations used", "Verbrauchte Simulationen", (-3.3, -2.42, 0), size=17, width=5),
                           _room.place(_room.text(self.tr("Best electricity found so far in kWh/a", "Bester bisher gefundener Strombedarf in kWh/a"), size=15), (-6.45, 2.02, 0), "left"))
        minimum = self.optimum["electricity"]
        floor = mn.DashedLine(ax.c2p(0, minimum), ax.c2p(budget, minimum), color=GOOD, stroke_width=2.4)
        limit = FRACTION * minimum
        band = mn.Rectangle(width=ax.c2p(budget, 0)[0] - ax.c2p(0, 0)[0], height=ax.c2p(0, limit)[1] - ax.c2p(0, minimum)[1], stroke_width=0, fill_color=GOOD,
                            fill_opacity=.35).move_to((ax.c2p(budget / 2, (limit + minimum) / 2)))
        edge = mn.DashedLine(ax.c2p(0, limit), ax.c2p(budget, limit), color=GOOD, stroke_width=1.4)
        floor_label = self._chip([self.tr("map minimum", "Kartenminimum"), *self._p(minimum, "kWh/a", 1)], ax.c2p(budget * .8, low + (minimum - low) * .33), color=GOOD, width=2.3)
        band_label = self._chip([self.tr("1 % above:", "1 % darüber:"), *self._p(minimum, "", 2)[:1], "× 1.01 =" if not self._german() else "× 1,01 =", *self._p(limit, "", 1)[:1]], ax.c2p(0, low + (minimum - low) * .33) + mn.RIGHT * 1.75, color=GOOD, size=14, width=3.2)
        names = (("gradient", "Gradient descent", "Gradientenabstieg", GRADIENT), ("pattern", "Pattern search", "Mustersuche", PATTERN), ("genetic", "Genetic algorithm", "Genetischer Algorithmus", GENETIC))
        curves, rows, links = mn.VGroup(), mn.VGroup(), mn.VGroup()
        for slot, (key, en, de, colour) in enumerate(names):
            values, designs = series[key], self.d["best"][key]
            y = 1.55 - 1.6 * slot

            def build(values=values, colour=colour):
                run = max(1, min(len(values), int(self.runs.get_value() + 1e-6)))
                points = []
                for k in range(1, run + 1):
                    if k > 1:
                        points.append(ax.c2p(k, values[k - 2]))
                    points.append(ax.c2p(k, values[k - 1]))
                points.append(ax.c2p(max(1, min(len(values), self.runs.get_value())), values[run - 1]))
                return mn.VGroup(mn.VMobject(color=colour, stroke_width=4).set_points_as_corners(points), mn.Dot(points[-1], radius=.07, color=colour))
            curves.add(self._live(build).set_z_index(5))

            def link(values=values, colour=colour, y=y):
                run = max(1, min(len(values), int(self.runs.get_value() + 1e-6)))
                return mn.VGroup(mn.DashedLine(ax.c2p(run, values[run - 1]), (1.0, y, 0), color=colour, stroke_width=1.4, dash_length=.07, stroke_opacity=.6))
            links.add(self._live(link))
            rows.add(self._chip([self.tr(en, de)], (3.8, y + .55, 0), color=colour, size=14, width=3.4))
            rows.add(self._live(lambda designs=designs, y=y, colour=colour: mn.VGroup(self.thumbnail(self._at(designs), (1.5, y, 0), .24, ramp=self.ramp_fn, ring=colour))))
            rows.add(self._live_chip(lambda values=values, y=y: ([*self._p(self._at(values), "kWh/a", 1)], (3.8, y + .15, 0)), size=16, color=colour, width=2.6))
            rows.add(self._live_chip(lambda designs=designs, y=y: ([*self._p(self._at(designs)["wwr"] * 100, "%", 0), "·", *self._p(self._at(designs)["overhang"], "m", 2)], (3.8, y - .25, 0)),
                                     size=14, width=2.6))
        self.runs.set_value(1)
        self.total.set_value(budget)
        self.compare_parts = (ax, ticks, titles, floor, band, edge, floor_label, band_label, curves, links, rows)
        self.step("compare", mn.Succession(mn.AnimationGroup(mn.FadeIn(mn.VGroup(ax, ticks, titles, floor, band, edge, floor_label, band_label, curves, links, rows), run_time=.8),
                                                             mn.FadeIn(self.counter, run_time=.8)),
                                            self.runs.animate(run_time=3.5, rate_func=mn.linear).set_value(24)), seconds=4.5)
        reach = self.d["reach"]
        marks = mn.VGroup()
        for slot, (key, _, _, colour) in enumerate(names):
            if reach[key]:
                spot = ax.c2p(reach[key], series[key][reach[key] - 1])
                marks.add(mn.Circle(radius=.13 + .05 * slot, color=colour, stroke_width=3).move_to(spot).set_z_index(6),
                          self._chip([self.tr("within 1 % after", "innerhalb 1 % nach"), self._n(reach[key]), self.tr("simulations", "Simulationen")], (3.8, 1.55 - 1.6 * slot - .62, 0), color=colour, size=14, width=3.4))
        gaps = mn.VGroup(*[self._chip([self.tr("gap", "Abstand"), *self._p(self._gap(series[key][-1]), "kWh/a", 1)], (6.05, 1.55 - 1.6 * slot + .15, 0), color=colour, size=14, width=1.4)
                           for slot, (key, _, _, colour) in enumerate(names)])
        self.step("compare_2", self.runs.animate(run_time=3.0, rate_func=mn.linear).set_value(budget), mn.Succession(mn.Wait(3.0), mn.AnimationGroup(mn.FadeIn(marks), mn.FadeIn(gaps))), seconds=4.0)
        self._stopping(ax, series, names, marks, gaps, rows, links)

    def _stopping(self, ax, series, names, marks, gaps, rows, links):
        """🛑 Budget, tolerance and stagnation as stop rules; every method is ringed where its first rule fired, with the design it would deliver there."""
        budget, stops = self.budget, self.d["stops"]
        line = mn.DashedLine(ax.c2p(budget, ax.y_range[0]), ax.c2p(budget, ax.y_range[1]), color=INK, stroke_width=2.4)
        rules = (("budget", self.tr("Budget", "Budget"), [self.tr("Budget", "Budget"), self._n(budget), self.tr("simulations", "Simulationen")]),
                 ("tolerance", self.tr("tolerance", "Toleranz"), [self.tr("Tolerance: gain below", "Toleranz: Gewinn unter"), *self._p(1, "kWh/a", 0), self.tr("in", "in"), self._n(5), self.tr("simulations", "Simulationen")]),
                 ("stagnation", self.tr("stagnation", "Stagnation"), [self.tr("Stagnation: no gain in", "Stagnation: kein Gewinn in"), self._n(10), self.tr("simulations", "Simulationen")]))
        word = {rule: short for rule, short, _ in rules}
        panel = mn.VGroup(*[self._chip(parts, (2.95, 1.45 - .55 * slot, 0), width=5.9, size=18) for slot, (_, _, parts) in enumerate(rules)])
        flagged = mn.VGroup()
        for slot, (key, _, _, colour) in enumerate((names[2], names[1], names[0])):
            rule, run = stops[key]
            spot = ax.c2p(run, series[key][run - 1])
            chip = self._chip([word[rule], "·", self._n(run), self.tr("simulations", "Simulationen")], (-2.2, .85 - .4 * slot, 0), color=colour, size=14, width=2.2)
            flagged.add(mn.Circle(radius=.15, color=colour, stroke_width=3).move_to(spot).set_z_index(6),
                        mn.Line(spot, chip.get_left(), color=colour, stroke_width=1.6, stroke_opacity=.8), chip)
            if rule == "tolerance":
                left, level = ax.c2p(run - 5, 0)[0], spot[1] - .27
                flagged.add(mn.Line((left, level, 0), (spot[0], level, 0), color=colour, stroke_width=2.4), mn.Line((left, level - .07, 0), (left, level + .07, 0), color=colour, stroke_width=2.4),
                            mn.Line((spot[0], level - .07, 0), (spot[0], level + .07, 0), color=colour, stroke_width=2.4))
        brackets = self._chip([self.tr("bracket: the 5 simulations the tolerance rule compares", "Klammer: die 5 Simulationen, die die Toleranz vergleicht")], (2.95, -.25, 0), size=14, width=5.9)
        self.step("stopping", mn.Create(line), mn.FadeIn(flagged), *self._fade([rows, links, gaps, marks, self.counter]), mn.FadeIn(panel), mn.FadeIn(brackets))
        results = mn.VGroup()
        for slot, (key, en, de, colour) in enumerate(names):
            rule, run = stops[key]
            y = 1.55 - 1.6 * slot
            design, electricity = self.d["best"][key][run - 1], series[key][run - 1]
            results.add(self.thumbnail(design, (1.5, y, 0), .24, ramp=self.ramp_fn, ring=colour),
                        self._chip([self.tr(en, de)], (3.9, y + .55, 0), color=colour, size=14, width=3.4),
                        self._chip([*self._p(electricity, "kWh/a", 1), self.tr("if stopped after", "bei Stopp nach"), self._n(run), self.tr("simulations", "Simulationen")], (4.05, y + .15, 0), color=colour, size=14, width=3.6),
                        self._chip([self.tr("above the 50-run result", "über dem 50-Lauf-Ergebnis"), *self._p(electricity - series[key][-1], "kWh/a", 1)], (3.9, y - .25, 0),
                                   color=BAD if electricity - series[key][-1] > 5 else GOOD, size=14, width=3.4))
        self.step("stopping_2", mn.FadeOut(panel), mn.FadeOut(brackets), mn.FadeIn(results))
        self._proof(series)

    def _mode_a(self):
        """🏠 The shared design panel again: both rooms, season tags, the three derivations, the bar and the design chips."""
        self.rooms = [self.room_section(self.focus, at, ROOM_SCALE, season, labels=False) for at, season in zip(ROOMS_AT, ("winter", "summer"))]
        self.rooms += [self._losses(), *self._flows()]
        self.bar = self.energy_bars(self.focus, BAR_AT, BAR_H, 900, compact=True, intensity=False, digits=1, size=13, rule=False, total=lambda design, found: _figures(design)["total"])
        self.ledger = self._ledger()
        self.seasons = self._seasons()
        self.design_chip = self._design_chip()
        self.note_chip = self._live_chip(lambda: ([self.note[0]], (5.95, 1.75, 0)), size=14, width=1.6, key=lambda: (self.note[0],))

    def _proof(self, series):
        """🔬 Fifty simulations are a sliver of the map: no proof of a global optimum."""
        self._clear()
        runs_map = self.d["convergence"]["runs_map"]
        self.fog_t.set_value(.55)
        finals = [(name, *self.finals[name]) for name in ("gradient", "pattern", "genetic")]
        colours = {"gradient": GRADIENT, "pattern": PATTERN, "genetic": GENETIC}
        rings = mn.VGroup(*[mn.Circle(radius=.13, color=colours[name], stroke_width=3).move_to(self._pos(design)).set_z_index(6) for name, design, _ in finals])
        track = mn.Rectangle(width=5.4, height=.3, color=INK, stroke_width=2).move_to((3.4, 1.65, 0))
        sliver = mn.Rectangle(width=max(.05, 5.4 * self.budget / runs_map), height=.3, color=GENETIC, fill_color=GENETIC, fill_opacity=1, stroke_width=0).align_to(track, mn.LEFT).set_y(1.65)
        labels = mn.VGroup(self._chip([self._n(runs_map), self.tr("simulations: the full map", "Simulationen: die ganze Karte")], (3.4, 2.15, 0), width=5.2),
                           self._chip([self._n(self.budget), self.tr("simulations", "Simulationen"), "·", self._n(self.budget / runs_map * 100, 0), "%"], (3.4, 1.15, 0), color=GENETIC, width=5.2))
        summary = mn.VGroup()
        for index, (key, en, de) in enumerate((("gradient", "Gradient descent", "Gradientenabstieg"), ("pattern", "Pattern search", "Mustersuche"), ("genetic", "Genetic algorithm", "Genetischer Algorithmus"))):
            x = .65 + 2.5 * index
            design = self.finals[key][0]
            room = self.room_section(self.study_trackers(design), (x, .05, 0), .2, "summer", labels=False, demand=False)
            summary.add(room, self._chip([self.tr(en, de)], (x, -1.08, 0), color=colours[key], size=14, width=2.4),
                        self._chip([*self._p(self.finals[key][1], "kWh/a", 1)], (x, -1.46, 0), color=colours[key], size=14, width=2.4),
                        self._chip([*self._p(design["wwr"] * 100, "%", 0), "·", *self._p(design["overhang"], "m", 2)], (x, -1.84, 0), color=colours[key], size=14, width=2.4))
        self.step("proof", mn.FadeIn(self.frame), mn.FadeIn(self.legend_title), mn.FadeIn(self.heat["image"]), mn.FadeIn(self.heat["contours"]), mn.FadeIn(self.heat["legend"]), mn.FadeIn(self.fog),
                  mn.FadeIn(self.star_pair), mn.FadeIn(rings), mn.FadeIn(track), mn.FadeIn(sliver), mn.FadeIn(labels), mn.FadeIn(summary), seconds=2.5)
        winner = min(finals, key=lambda item: item[2])
        stamp = mn.VGroup(self._chip([self.tr("model result: best of three solvers", "Modellergebnis: bester der drei Solver")], (ROOMS_AT[0][0] + 1.75, -2.0, 0), color=colours[winner[0]], width=4.3),
                          self._chip([self.tr("check with a finer simulation", "mit feinerer Simulation prüfen")], (ROOMS_AT[0][0] + 1.75, -2.4, 0), color=colours[winner[0]], width=4.3))
        self._set_focus(winner[1])
        self.winner = winner[1]
        self.note[0] = self.tr("winner", "Sieger")
        self._mode_a()
        self.step("confirm", *self._fade([track, sliver, labels, summary]), mn.FadeIn(self.seasons), mn.FadeIn(self.design_chip), self.reveal(*self.rooms, self.bar, *self.ledger, self.note_chip), mn.FadeIn(stamp), seconds=1.8)
        self.stamp = stamp
        self._tools()

    def _slider(self, name, key, low, high, unit, digits, scale, point, tracker):
        """🎚️ A slider whose knob and value follow a tracker."""
        x0, y = point
        track = mn.Line((x0, y, 0), (x0 + 3.0, y, 0), color=MUTED, stroke_width=3)
        knob = self._live(lambda: mn.VGroup(mn.Circle(radius=.12, color=INK, stroke_width=2.4, fill_color=DARK, fill_opacity=1).move_to(
            (x0 + 3.0 * (tracker[key].get_value() - low) / (high - low), y, 0))))
        value = self._live_chip(lambda: ([*self._p(tracker[key].get_value() * scale, unit, digits)], (x0 + 3.0 + .75, y, 0)), width=1.3)
        title = _room.place(_room.text(name, size=14), (x0, y + .32, 0), "left")
        return mn.VGroup(track, knob, value, title)

    def _tools(self):
        """🛠️ The same loop inside a parametric design tool."""
        self._clear()
        generations = self.d["generations"]
        tracker = self.study_trackers(self.start)
        windows = self._slider(self.tr("Window share", "Fensteranteil"), "wwr", .1, .8, "%", 0, 100, (-6.5, 1.2), tracker)
        overhang = self._slider(self.tr("Overhang depth", "Vordachtiefe"), "overhang", 0, 1.2, "m", 2, 1, (-6.5, -.1), tracker)
        box = mn.Rectangle(width=3.6, height=2.6, color=INK, stroke_width=2).move_to((.3, .6, 0))
        sims = [self.room_section(tracker, (x, .5, 0), .15, season, labels=False, person=False) for x, season in ((-.4, "winter"), (1.0, "summer"))]
        sim_title = self._chip([self.tr("Annual simulation", "Jahressimulation")], (.3, 2.1, 0), width=3.4)
        readout = mn.Rectangle(width=3.2, height=1.2, color=INK, stroke_width=2).move_to((5.2, .6, 0))
        number = self.sum_formula((5.2, .6, 0), size=15, trackers=tracker, width=3.0, unit="")
        out_title = self._chip([self.tr("Annual electricity in kWh/a", "Strombedarf in kWh/a")], (5.2, 1.5, 0), width=3.1)
        solver = mn.Rectangle(width=4.6, height=.7, color=GENETIC, stroke_width=2.6).move_to((.3, -1.45, 0))
        solver_text = _room.text(self.tr("Solver: next design", "Solver: nächster Entwurf"), (.3, -1.45, 0), size=16, color=GENETIC, width=4.2)
        wires = mn.VGroup(mn.Arrow((-3.2, .55, 0), (-1.55, .55, 0), buff=0, color=MUTED), mn.Arrow((2.15, .6, 0), (3.45, .6, 0), buff=0, color=MUTED),
                          mn.Line((5.2, .0, 0), (5.2, -1.45, 0), color=GENETIC), mn.Arrow((5.2, -1.45, 0), (2.6, -1.45, 0), buff=0, color=GENETIC),
                          mn.Line((-2.0, -1.45, 0), (-4.9, -1.45, 0), color=GENETIC), mn.Arrow((-4.9, -1.45, 0), (-4.9, -.45, 0), buff=0, color=GENETIC))
        diagram = mn.VGroup(windows, overhang, box, *sims, sim_title, readout, number, out_title, solver, solver_text, wires)
        hops = [self.set_design(tracker, generations[index]["best"]["design"], 1.0) for index in (2, 4, 7)]
        self.step("tools", mn.FadeIn(diagram), seconds=1.6)
        chips = mn.VGroup(*[self._chip([self.tr(a, b) + ":", self.tr(c, d)], (x, -2.3, 0), color=colour, width=3.9)
                            for (a, b, c, d, colour), x in zip((("smooth model", "glattes Modell", "gradient", "Gradient", GRADIENT), ("noisy model", "rauschendes Modell", "pattern search", "Mustersuche", PATTERN),
                                                                 ("jagged, many valleys", "zackig, viele Täler", "genetic", "genetisch", GENETIC)), (-4.4, 0, 4.4))])
        self.step("tools_2", mn.FadeIn(chips), mn.Succession(*hops), seconds=3.0)
        self._close(diagram, chips)

    def _close(self, diagram, chips):
        """🏁 The answer: the map with the three trails, the optimum design in both seasons and what each solver found."""
        self._clear()
        self.winner = self.optimum["design"]
        self._set_focus(self.winner)
        self.note[0] = self.tr("map minimum", "Kartenminimum")
        self.fog_t.set_value(.45)
        self._mode_a()
        found = _room.result(self.winner)
        colours = {"gradient": GRADIENT, "pattern": PATTERN, "genetic": GENETIC}
        rings = mn.VGroup(*[mn.Circle(radius=.13, color=colours[name], stroke_width=3).move_to(self._pos(self.finals[name][0])).set_z_index(6) for name in colours])
        answer = mn.VGroup(
            self._chip([self.tr("full map minimum", "ganze Karte"), *self._p(self.winner["wwr"] * 100, "%", 0), "·", *self._p(self.winner["overhang"], "m", 2), "·", *self._p(found["electricity"], "kWh/a", 1), "·",
                        self.tr("one-off façade cost", "Fassade einmalig"), *self._p(found["cost"], "CHF", 0)], (CARD_X, -1.05, 0), color=GOOD, width=6.8, size=14),
            *[self._chip([self.tr(en, de), *self._p(self.finals[key][0]["wwr"] * 100, "%", 0), "·", *self._p(self.finals[key][0]["overhang"], "m", 2), "·", *self._p(self.finals[key][1], "kWh/a", 1),
                          "·", self.tr("gap", "Abstand"), *self._p(self._gap(self.finals[key][1]), "kWh/a", 1)], (CARD_X, -1.5 - .4 * slot, 0), color=colour, size=14, width=6.2)
              for slot, (key, en, de, colour) in enumerate((("gradient", "Gradient descent", "Gradientenabstieg", GRADIENT), ("pattern", "Pattern search", "Mustersuche", PATTERN),
                                                              ("genetic", "Genetic algorithm", "Genetischer Algorithmus", GENETIC)))])
        trails = [mn.FadeIn(trail) for trail in self.trails.values()]
        self.step("close", mn.FadeIn(self.frame), mn.FadeIn(self.legend_title), mn.FadeIn(self.heat["image"]), mn.FadeIn(self.heat["contours"]), mn.FadeIn(self.heat["legend"]), mn.FadeIn(self.fog),
                  mn.FadeIn(self.star_pair), mn.FadeIn(rings), mn.FadeIn(self.seasons), mn.FadeIn(self.design_chip), self.reveal(*self.rooms, self.bar, self.note_chip), mn.FadeIn(answer), *trails, seconds=1.6)

    def present_algorithms(self):
        """🎬 Run the whole clip: task, landscape, three solvers, comparison and tools."""
        self._prepare()
        self._task()
        self._landscape()
        self._gradient()
        self._pattern()
        self._genetic()
        self._compare()
