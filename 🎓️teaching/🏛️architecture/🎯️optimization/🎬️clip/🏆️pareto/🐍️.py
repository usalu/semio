"""🎬 Pareto Dominance and Trade-offs on the seminar-room façade: cost versus annual electricity, one live cursor, one room."""

import functools
import importlib
import math
import sys
from pathlib import Path
from types import SimpleNamespace

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_art, mn = _visuals._art, _visuals._engine

INK, MUTED, GOOD, COST, BAD, GLASS = _visuals.INK, _visuals.MUTED, _visuals.GOOD, _art.P_ORANGE, _art.P_RED, _art.P_CYAN
AXES_AT, AXES_WIDTH, AXES_HEIGHT = (-4.0, .5, 0), 4.5, 4.0
ROOM_AT, ROOM_SCALE, BAR_AT, BAR_HEIGHT = (2.9, .98, 0), .5, (6.0, .75, 0), 2.35
BAND_LEFT, SIDE_LEFT, SIDE_RIGHT = -.85, 2.8, 6.8
LEGEND_COLUMNS = (4.47, 5.19, 6.84)
LEGEND_TOP = BAR_AT[1] - .85
ROWS = (-1.1, -1.48, -1.86, -2.24, -2.55)
PAIR_X, PAIR_Y, PAIR_SCALE = (-.1, 1.65), -1.2, .1
LEGEND_AT, LEGEND_WIDTH = (-4.65, 1.45, 0), 2.4
LEFT_STRIP, STRIP_WIDTH = -6.8, 2.15
THUMB = .046
SOFT = mn.interpolate_color(mn.ManimColor(MUTED), mn.ManimColor(INK), .45).to_hex()
VARIABLES = {"wwr": ("WWR", 100, "%", 0), "overhang": ("d", 1, "m", 1), "insulation": ("t", 100, "cm", 0)}
VARIABLE_COLOR = {"wwr": GLASS, "overhang": COST, "insulation": _room.INSULATION}
CHANGE_ANCHOR = {"overhang": (.45, _room.HEIGHT + _room.SLAB + .62), "wwr": (1.6, 2.02), "insulation": (.3, -.78)}
CHART_ORIGIN, CHART_WIDTH, CHART_HEIGHT, CHART_YEARS, CHART_CHF = (-.45, -2.3, 0), 3.0, 1.0, 36.0, 700.0
PAYBACK_ROWS = (-1.08, -1.42, -1.76)
TABLE_ROWS = (-1.97, -2.17, -2.39)
PATH_ORDER = ("overhang", "wwr", "insulation")


def _spot(x, y):
    """📍 Screen point of the section's model coordinates ``(x, y)`` in metres."""
    return (ROOM_AT[0] + ROOM_SCALE * (x - 3.0), ROOM_AT[1] + ROOM_SCALE * (y - 1.5), 0)


def _clip(point, direction, box):
    """✂️ Part of the line through ``point`` inside the box ``(x0, y0, x1, y1)``."""
    low, high = -1e9, 1e9
    for origin, step, near, far in ((point[0], direction[0], box[0], box[2]), (point[1], direction[1], box[1], box[3])):
        if abs(step) < 1e-12:
            if origin < near or origin > far:
                return None
            continue
        first, second = sorted(((near - origin) / step, (far - origin) / step))
        low, high = max(low, first), min(high, second)
    return ((point[0] + low * direction[0], point[1] + low * direction[1]), (point[0] + high * direction[0], point[1] + high * direction[1])) if low < high else None


def _at(label, x, y, edge="left"):
    """📌 Put a fresh label's baseline on ``y`` and its left, right or centre edge on ``x``."""
    anchor = {"left": label.get_left()[0], "right": label.get_right()[0], "center": label.get_center()[0]}[edge]
    return label.shift(mn.RIGHT * (x - anchor) + mn.UP * y)


def _places(value, steps=(10, 1)):
    """🔢 Decimals that keep a quantity readable: none from ``steps[0]`` up, one from ``steps[1]`` up, else two."""
    value = abs(value)
    return 0 if value >= steps[0] else 1 if value >= steps[1] else 2


@functools.lru_cache(maxsize=1)
def _prices():
    """🏷️ Unit prices of the façade parts from the study."""
    return {key: float(item["value"]) for key, item in _model.load_study()["parameters"]["prices"].items()}


def _cost_parts(design):
    """🧾 Fixed, wall, insulation, glass and overhang cost in CHF of a design, adding up to its model cost."""
    found, prices = _room.result(design), _prices()
    parts = {"fixed": prices["fixed"], "wall": found["wall_area"] * prices["wall"],
             "insulation": found["wall_area"] * prices["insulation_per_cm"] * 100 * design["insulation"],
             "glass": found["window_area"] * prices["window"], "overhang": prices["overhang"] * _room.WIDTH * design["overhang"]}
    assert abs(sum(parts.values()) - found["cost"]) < 1e-6
    return parts


class Beat5_ParetoFront(_visuals.ClipScene):
    """📚 Walk from two goals to dominance, the front, the price of every step and the choice, on one seminar room."""

    TITLE = {"en": "Pareto Optimality", "de": "Pareto-Optimalität"}
    NARRATION = [
        [
            "room",
            "Which window share, insulation and overhang suit this seminar room's glazed façade?",
            "Welcher Fensteranteil, welche Dämmung und welches Vordach passen zur verglasten Fassade dieses Seminarraums?"
        ],
        [
            "room_2",
            "A simplified annual teaching model gives every façade its cost and its electricity.",
            "Ein vereinfachtes Jahresmodell liefert für jede Fassade Kosten und Strombedarf."
        ],
        [
            "objectives",
            "We want both small: cost to the left, annual electricity at the bottom.",
            "Beides soll klein sein: Kosten links, Strombedarf unten."
        ],
        [
            "objectives_2",
            "Every façade becomes one point: cost along the horizontal, electricity up.",
            "Jede Fassade wird ein Punkt: Kosten waagerecht, Strombedarf senkrecht."
        ],
        [
            "grid",
            "We simulate every combination of six window shares, three insulation thicknesses and two overhang depths.",
            "Wir simulieren jede Kombination aus sechs Fensteranteilen, drei Dämmstärken und zwei Vordachtiefen."
        ],
        [
            "grid_2",
            "Point at any tile: room, cost and electricity follow.",
            "Zeige auf eine Kachel: Raum, Kosten und Strombedarf folgen."
        ],
        [
            "extremes",
            "The cheapest and the most frugal façade lie far apart: none is both.",
            "Die günstigste und sparsamste Fassade liegen weit auseinander: keine ist beides."
        ],
        [
            "dominated",
            "Compare two façades: this one has a larger window share and more insulation.",
            "Vergleiche zwei Fassaden: Diese hat einen größeren Fensteranteil und mehr Dämmung."
        ],
        [
            "dominated_2",
            "That one is cheaper and uses less electricity.",
            "Jene ist günstiger und braucht weniger Strom."
        ],
        [
            "dominated_3",
            "So the first is dominated: nobody would pick it.",
            "Die erste ist dominiert: Niemand würde sie wählen."
        ],
        [
            "rule",
            "Dominance: no worse in cost and no worse in electricity.",
            "Dominanz: bei Kosten und Strombedarf nicht schlechter."
        ],
        [
            "rule_2",
            "And strictly better in at least one of them; here, in both.",
            "Und bei mindestens einer Zielgröße echt besser; hier bei beiden."
        ],
        [
            "front",
            "Sweep from cheap to expensive: keep only façades whose electricity beats all cheaper ones.",
            "Von günstig zu teuer: Es bleiben nur Fassaden, deren Strombedarf alle günstigeren unterbietet."
        ],
        [
            "front_2",
            "What remains is the Pareto front of the simulated designs: no gain without extra cost.",
            "Übrig bleibt die Pareto-Front der simulierten Entwürfe: kein Gewinn ohne Mehrkosten."
        ],
        [
            "approximate",
            "Off the grid, a bigger window shaded by a deeper overhang, paid for by thinner insulation, dominates a front member.",
            "Abseits des Rasters dominiert ein größeres Fenster mit tieferem Vordach und dünnerer Dämmung ein Frontmitglied."
        ],
        [
            "sets",
            "The room is design space, the plot is objective space; simulation maps one onto the other.",
            "Der Raum ist der Entwurfsraum, das Diagramm der Zielraum; die Simulation bildet das eine auf das andere ab."
        ],
        [
            "walk",
            "Walk along the front, starting at the cheapest façade.",
            "Wir gehen die Front entlang, beginnend bei der günstigsten Fassade."
        ],
        [
            "walk_2",
            "Step one: a 0.6 m overhang costs 192 CHF more and saves 65 kWh/a.",
            "Schritt eins: Ein Vordach von 0,6 m kostet 192 CHF mehr, die Ersparnis beträgt 65 kWh/a."
        ],
        [
            "walk_3",
            "Divide the two: 3.0 CHF for every kWh/a saved.",
            "Teilt man beides: 3,0 CHF für jede eingesparte kWh/a."
        ],
        [
            "payback",
            "At 0.25 CHF per kWh the savings repay this step in about 12 years.",
            "Bei 0,25 CHF/kWh amortisiert sich der Schritt in etwa 12 Jahren."
        ],
        [
            "payback_2",
            "A façade lasts about 30 years: only steps that pay back within that lifetime are worth it.",
            "Eine Fassade hält etwa 30 Jahre: Nur Schritte, die sich in dieser Zeit amortisieren, lohnen sich."
        ],
        [
            "knee",
            "Walk on: each step costs more per kWh/a saved, until the payback exceeds the lifetime.",
            "Weiter: Jeder Schritt kostet mehr je eingesparter kWh/a, bis die Amortisation die Lebensdauer überschreitet."
        ],
        [
            "knee_2",
            "Here is the knee: the next step would take about 34 years to pay back, longer than the façade lasts.",
            "Hier ist das Knie: Der nächste Schritt braucht etwa 34 Jahre bis zur Amortisation, länger als die Fassade hält."
        ],
        [
            "knee_3",
            "At 0.40 CHF per kWh that step repays in 21 years, so the knee moves: same front, different best choice.",
            "Bei 0,40 CHF/kWh amortisiert sich dieser Schritt in 21 Jahren: Das Knie wandert, gleiche Front, andere beste Wahl."
        ],
        [
            "third",
            "Add daylight as a third objective: tile colour shows the daylight factor in percent.",
            "Als dritte Zielgröße kommt das Tageslicht hinzu: Die Kachelfarbe zeigt den Tageslichtquotienten in Prozent."
        ],
        [
            "third_2",
            "In 2D it loses: a dimmer room is cheaper and needs less electricity.",
            "In 2D verliert er: Ein dunklerer Raum ist billiger und sparsamer."
        ],
        [
            "third_3",
            "With daylight counted, nobody beats it on all three objectives.",
            "Mit Tageslicht schlägt ihn keiner in allen drei Zielgrößen."
        ],
        [
            "third_4",
            "A two-dimensional picture can hide a trade-off: check every objective.",
            "Zwei Dimensionen können Zielkonflikte verbergen: Prüfe jede Zielgröße."
        ],
        [
            "weights",
            "To pick one façade, weigh the objectives as in the index video: the weights fix what one kWh/a is worth.",
            "Um eine Fassade zu wählen, gewichten wir die Zielgrößen wie im Index-Video: Die Gewichte legen fest, was eine kWh/a wert ist."
        ],
        [
            "weights_2",
            "The winner always sits on the front and slides along it as the weights change.",
            "Der Sieger liegt stets auf der Front und gleitet entlang, wenn sich die Gewichte ändern."
        ],
        [
            "decision",
            "The front removes dominated designs; it does not choose: tariff, lifetime and weights do.",
            "Die Front entfernt dominierte Entwürfe; sie wählt nicht: Tarif, Lebensdauer und Gewichte tun es."
        ],
        [
            "decision_2",
            "For this room, the knee: 30 % window share, 0.6 m overhang, 12 cm insulation at 0.25 CHF/kWh.",
            "Für diesen Raum das Knie: 30 % Fensteranteil, 0,6 m Vordach, 12 cm Dämmung bei 0,25 CHF/kWh."
        ]
    ]

    def construct(self):
        """▶️ Present this independent architecture optimization chapter."""
        self.present("pareto")

    def present_pareto(self):
        """🏆 Tell the whole chapter on one stage: plot left, room and readings right."""
        s = self._pareto_stage()
        self._pareto_intro(s)
        self._pareto_dominance(s)
        self._pareto_front(s)
        self._pareto_walk(s)
        self._pareto_daylight(s)
        self._pareto_choice(s)

    def _pareto_stage(self):
        """🎛️ Gather model numbers, axes, façade tiles and the shared trackers."""
        data = _model.pareto()
        designs = data["designs"]
        s = SimpleNamespace(data=data, designs=designs, by_id={d["id"]: d for d in designs}, front=data["front"], steps=data["steps"],
                            tariff=data["tariff"], tariff_hi=data["tariff_hi"], life=data["service_life"], german=self.language() == "de")
        costs, powers, lights = [d["cost"] for d in designs], [d["electricity"] for d in designs], [d["daylight"] for d in designs]
        s.ramp = self.ramp(min(powers), max(powers))
        s.x_range = (math.floor((min(costs) - 100) / 250) * 250, math.ceil((max(costs) + 100) / 250) * 250)
        s.y_range = (math.floor((min(powers) - 50) / 50) * 50, math.ceil((max(powers) + 110) / 50) * 50)
        s.axes, labels = self.plot_axes(x_range=(*s.x_range, 250), y_range=(*s.y_range, 50), point=AXES_AT, width=AXES_WIDTH, height=AXES_HEIGHT,
                                        x_ticks=[value for value in range(s.x_range[0], s.x_range[1] + 1, 250) if value % 1000 == 0],
                                        y_ticks=[value for value in range(s.y_range[0], s.y_range[1] + 1, 50) if value % 100 == 0],
                                        x_label=self.tr("Façade cost [CHF]", "Fassadenkosten [CHF]"),
                                        y_label=self.tr("Electricity [kWh/a]", "Strombedarf [kWh/a]"))
        labels[1].move_to([s.axes.get_left()[0] - .45 + labels[1].width / 2, s.axes.get_top()[1] + .32, 0])
        labels[0].move_to([s.axes.get_center()[0], s.axes.get_bottom()[1] - .52, 0])
        for tick in labels[2:]:
            if tick.get_center()[0] < s.axes.get_left()[0] - .05:
                tick.shift(mn.LEFT * .08)
        s.labels = labels
        s.spot = lambda d: s.axes.c2p(d["cost"], d["electricity"])
        low, high = min(lights), max(lights)
        start, end = mn.ManimColor("#2a7f8a"), mn.ManimColor(_art.P_YELLOW)

        def light(value):
            return mn.interpolate_color(start, end, min(1.0, max(0.0, (value - low) / (high - low)))).to_hex()

        light.low, light.high = low, high
        s.light = light
        s.tiles = {d["id"]: self._tile(s, d) for d in designs}
        s.tile_group = mn.VGroup(*s.tiles.values())
        s.intro_id = min(designs, key=lambda d: abs(d["design"]["wwr"] - .45) + abs(d["design"]["overhang"] - .6) + abs(d["design"]["insulation"] - .16))["id"]
        s.focus = self.study_trackers(s.by_id[s.intro_id]["design"])
        s.driver = mn.Mobject()
        self.add(s.driver)
        s.knee = lambda tariff: ([step["to"] for step in s.steps if step["ratio"] / tariff <= s.life] or [s.front[0]])[-1]
        s.walk, s.delta, s.peek, s.stage = mn.ValueTracker(0), mn.ValueTracker(0), mn.ValueTracker(0), mn.ValueTracker(0)
        s.flags = {key: mn.ValueTracker(0) for key in ("pair", "cards", "price", "life", "chart", "table")}
        s.index = lambda: max(0, min(math.ceil(s.walk.get_value() + s.peek.get_value() - 1e-9) - 1, len(s.steps) - 1))
        return s

    def _legend(self, colour, title, unit, digits=0, point=LEGEND_AT, width=LEGEND_WIDTH):
        """🌈 Colour strip of a tile ramp with the unit on both end values and a title."""
        cells, left = 30, point[0] - width / 2
        strip = mn.VGroup(*[mn.Rectangle(width=width / cells + .004, height=.16, stroke_width=0, fill_opacity=1,
                                         fill_color=colour(colour.low + (colour.high - colour.low) * (index + .5) / cells)).move_to([left + width * (index + .5) / cells, point[1], 0])
                            for index in range(cells)])
        ends = mn.VGroup(_room.place(_visuals._text(self.quantity(colour.low, unit, digits), size=15, width=1.5), (left, point[1] - .27, 0), "left"),
                         _room.place(_visuals._text(self.quantity(colour.high, unit, digits), size=15, width=1.5), (left + width, point[1] - .27, 0), "right"))
        return mn.VGroup(strip, ends, _visuals._text(title, (point[0] + .05, point[1] + .3, 0), size=15, width=2.7))

    def _eq(self, items, x, y, *, size=15, edge="left", width=None, gap=.11, z=0):
        """🧮 One baseline-aligned line of coloured tokens ``(text, colour)``, left, right or centre aligned at ``x``."""
        labels, cursor = [], 0.0
        for token, color in items:
            label = _room.row(size, color, token)
            label.shift(mn.RIGHT * (cursor - label.get_left()[0]))
            labels.append(label)
            cursor = label.get_right()[0] + gap
        line = mn.VGroup(*labels)
        if width is not None and line.width > width:
            line.scale(width / line.width, about_point=mn.ORIGIN)
        return _at(line, x, y, edge).set_z_index(z)

    def _segments(self, design):
        """🎨 Lighting, cooling and heating electricity with one decimal, adding up exactly to the shown total."""
        found = _room.result(design)
        values = [found[key] for key in ("lighting", "cooling", "heating")]
        shown = _room.apportion(values, 1, found["electricity"])
        return dict(zip(("lighting", "cooling", "heating"), shown)) | {"electricity": round(found["electricity"], 1)}

    def _bar_legend(self, s):
        """🎨 Live segment names with their annual kWh/a (parts add up to the total) and, while a step is walked, the signed change against the previous façade (changes add up to the total change)."""
        german = s.german
        keys = ("lighting", "cooling", "heating")
        names = ((self.tr("Light", "Licht"), "lighting", _room.LIGHT), (self.tr("Cool", "Kühlung"), "cooling", _room.COOL),
                 (self.tr("Heat", "Heizung"), "heating", _room.HEAT), (self.tr("Total", "Summe"), "electricity", INK))

        def build():
            design = _room.values(s.focus)
            now = self._segments(design)
            group, shown = mn.VGroup(), s.delta.get_value() > .5
            if shown:
                before = self._segments(s.by_id[s.steps[s.index()]["from"]]["design"])
                changes = {key: round(now[key] - before[key], 1) + 0.0 for key in (*keys, "electricity")}
                group.add(_at(_room.row(13, MUTED, "Δ kWh/a"), LEGEND_COLUMNS[2], LEGEND_TOP + .24, "right"))
            for row, (name, key, color) in enumerate(names):
                y = LEGEND_TOP - .24 * row - (.05 if key == "electricity" else 0)
                group.add(_at(_room.row(13, color, name), LEGEND_COLUMNS[0], y),
                          _at(_room.row(13, color, *_room.parts(now[key], "kWh/a", 1, german)), LEGEND_COLUMNS[1], y))
                if shown:
                    change = changes[key]
                    group.add(_at(_room.row(13, color, ("+" if change > 0 else "−" if change < 0 else "±") + _room.number(abs(change), 1, german)), LEGEND_COLUMNS[2], y, "right"))
                if key == "heating":
                    group.add(mn.Line([LEGEND_COLUMNS[0], y - .15, 0], [LEGEND_COLUMNS[2], y - .15, 0], color=MUTED, stroke_width=1.2))
            return group

        return self._live(build, lambda: (_room.design_key(s.focus)(), s.index(), s.delta.get_value() > .5))

    def _element(self, key, design):
        """📐 Box ``(x0, y0, x1, y1)`` in section metres of the overhang slab, the window opening or the insulation layer."""
        found, x_o = _room.result(design), -(_room.MASONRY + design["insulation"])
        if key == "overhang":
            return x_o - design["overhang"], _room.HEIGHT, x_o, _room.HEIGHT + _room.SLAB
        if key == "wwr":
            return x_o - .06, _room.HEAD - min(found["window_area"] / _room.WIDTH, _room.HEAD - .05), x_o + .26, _room.HEAD
        return x_o, 0.0, -_room.MASONRY, _room.HEIGHT

    def _outline(self, box, color, opacity=1.0):
        """⬚ Dashed outline of a section box, slightly padded."""
        x0, y0, x1, y1 = box[0] - .08, box[1] - .08, box[2] + .08, box[3] + .08
        corners = [_spot(x0, y0), _spot(x1, y0), _spot(x1, y1), _spot(x0, y1)]
        return mn.VGroup(*[_room.dashed(corners[i], corners[(i + 1) % 4], color=color, stroke_width=2.4, dash=.07, gap=.05, opacity=opacity) for i in range(4)])

    def _change(self, key, before, after):
        """🔁 Label parts ``d 0.6 → 0.99 m`` of one design variable that changes; ``d`` shows two decimals when needed."""
        label, factor, unit, digits = VARIABLES[key]
        fine = lambda item: 2 if key == "overhang" and abs(round(item[key], 1) - item[key]) > 1e-6 else digits
        return [label, _room.parts(before[key] * factor, "", fine(before), self._german())[0], "→", *_room.parts(after[key] * factor, unit, fine(after), self._german())]

    def _changes(self, s, pair, key):
        """🔦 In the room: dashed target and previous outline plus a delta chip for every variable that changes between ``pair() = (before, after)``."""
        def build():
            a, b = pair()
            now = _room.values(s.focus)
            group = mn.VGroup()
            for name in VARIABLES:
                if abs(a[name] - b[name]) < 1e-9:
                    continue
                colour = VARIABLE_COLOR[name]
                before, after = self._element(name, {**now, name: a[name]}), self._element(name, {**now, name: b[name]})
                if name != "overhang" or a[name] > .01:
                    group.add(self._outline(before, MUTED, .8))
                group.add(self._outline(after, colour))
                chip = _room.place(_room.chip(self._change(name, a, b), size=14, color=colour), _spot(*CHANGE_ANCHOR[name]), "left").set_z_index(12)
                target = _spot(after[2] + .08, after[3] + .08 if name == "overhang" else (after[1] + after[3]) / 2) if name != "insulation" else None
                if target:
                    group.add(_room.dashed(chip.get_left(), target, color=colour, stroke_width=1.6, opacity=.8))
                group.add(chip)
            return group

        return self._live(build, key)

    def _daylight_chips(self, s):
        """💡 In the room: the daylight factor and the share of time the lamps are on, both from the model."""
        def build():
            found = _room.result(_room.values(s.focus))
            return mn.VGroup(_room.place(_room.chip([self.tr("Daylight factor", "Tageslichtquotient"), *_room.parts(found["daylight"], "%", 1, s.german)], size=14, color=_art.P_YELLOW),
                                         _spot(.45, 3.78), "left"),
                             _room.place(_room.chip([self.tr("Lamps on", "Lampen an"), *_room.parts((1 - found["autonomy"]) * 100, "%", 0, s.german), self.tr("of the time", "der Zeit")],
                                                    size=14, color=INK), _spot(.45, 4.5), "left"))

        return self._live(build, _room.design_key(s.focus))

    def _tile(self, s, item):
        """🏘️ One design on the plot as a façade tile with plain, grey, daylight and front-ring layers."""
        spot, design = s.spot(item), item["design"]
        layers = {"plain": self.thumbnail(design, spot, THUMB, state="filled", ramp=s.ramp),
                  "grey": self.thumbnail(design, spot, THUMB, state="ghost"),
                  "light": self.thumbnail(design, spot, THUMB, state="filled", ramp=s.light, value=item["daylight"]),
                  "ring": mn.Rectangle(width=THUMB * 4.8 + .04, height=THUMB * 4.0 + .04, color=GOOD, stroke_width=3.5, fill_opacity=0).move_to(spot)}
        tile = mn.VGroup(layers["light"], layers["grey"], layers["plain"], layers["ring"])
        tile.layers, tile.weights = layers, {"plain": 1.0, "grey": 0.0, "light": 0.0, "ring": 0.0}
        self._apply(tile)
        return tile

    def _apply(self, tile):
        """🎚️ Show each tile layer with its current weight."""
        for name, layer in tile.layers.items():
            if name == "ring":
                layer.set_stroke(opacity=tile.weights[name])
            else:
                layer.set_opacity(tile.weights[name])

    def _morph(self, s, goals):
        """🔀 Animation that blends tile layers towards ``{design id: {layer: weight}}``."""
        started = {}

        def update(mob, alpha):
            for key, goal in goals.items():
                tile = s.tiles[key]
                started.setdefault(key, dict(tile.weights))
                for name, value in goal.items():
                    tile.weights[name] = started[key][name] + (value - started[key][name]) * alpha
                self._apply(tile)

        return mn.UpdateFromAlphaFunc(s.tile_group, update, rate_func=mn.smooth)

    def _drive(self, s, function):
        """🔗 Let one function set the shared design trackers every frame, or release them."""
        s.driver.clear_updaters()
        if function is not None:
            s.driver.add_updater(lambda mob: function())

    def _put(self, s, design):
        """📌 Jump the shared trackers to a design."""
        for key in _room.KEYS:
            s.focus[key].set_value(float(design[key]))

    def _cost_chip(self, trackers, point):
        """🪙 Live price chip of the design in the trackers."""
        word, german = self.tr("Façade cost", "Fassadenkosten"), self._german()
        return self._live(lambda: _room.place(_room.chip([word, *_room.parts(_room.result(_room.values(trackers))["cost"], "CHF", 0, german)], size=17, color=COST), point, "left"),
                          _room.design_key(trackers))

    def _readout(self, s):
        """🧷 Live chip ``cost · electricity`` right of the cursor, clear of both axes."""
        german = s.german

        def build():
            item = _room.result(_room.values(s.focus))
            spot = s.spot(item)
            box = _room.chip([*_room.parts(item["cost"], "CHF", 0, german), "·", *_room.parts(item["electricity"], "kWh/a", 0, german)], size=15, color=COST)
            right = s.axes.c2p(s.x_range[1], 0)[0] - .1
            return box.move_to([min(spot[0] + .3 + box.width / 2, right - box.width / 2), spot[1] + .5, 0])

        return self._live(build, _room.design_key(s.focus))

    def _tick(self, point, ok=True):
        """✔️ A drawn check or cross mark."""
        x, y = float(point[0]), float(point[1])
        if ok:
            return mn.VMobject(color=GOOD, stroke_width=4).set_points_as_corners([(x - .09, y, 0), (x - .03, y - .08, 0), (x + .1, y + .1, 0)])
        return _art.cross_mark(BAD, .09).move_to([x, y, 0])

    def _row(self, items, ok, y, *, x=SIDE_LEFT, size=16, width=3.7):
        """📋 One reading row: a drawn mark, then a coloured equation line of tokens."""
        return mn.VGroup(self._tick((x, y + .06), ok), self._eq(items, x + .24, y, size=size, width=width))

    def _signed(self, value, digits):
        """➕ A number with its explicit sign."""
        return ("+" if value > 0 else "−" if value < 0 else "") + self.number(abs(value), digits)

    def _pareto_intro(self, s):
        """🏠 Room, two goals, the grid of designs and the cursor that links tile and room."""
        intro = self.study_trackers(s.by_id[s.intro_id]["design"])
        room0 = self.room_section(intro, (0.0, -.15, 0), .56, "summer", demand=False)
        s.choices = self._choices(s)
        lines = (self.tr("summer noon sun,", "Sommer-Mittagssonne,"), self.tr("66° above the horizon", "66° über dem Horizont"))
        sun0 = mn.VGroup(*[self._eq([(line, MUTED)], -1.85, 2.42 - .24 * k, size=13) for k, line in enumerate(lines)])
        s.sun = mn.VGroup(*[self._eq([(line, MUTED)], .85, 3.2 - .24 * k, size=13) for k, line in enumerate(lines)])
        self.step("room", mn.FadeIn(room0), mn.FadeIn(s.choices), mn.FadeIn(sun0), seconds=1.5)
        bars0 = self.energy_bars(intro, (3.6, -1.9, 0), 3.2, s.y_range[1], width=.8, size=14, digits=1)
        cost0 = self._cost_chip(intro, (2.4, 2.4, 0))
        s.anatomy = self._anatomy(s, intro)
        floor = _room.WIDTH * _room.DEPTH
        found = _room.result(_room.values(intro))
        per_floor = self._eq([(self.quantity(found["electricity"], "kWh/a", 1), INK), ("÷", MUTED), (self.quantity(floor, "m²") + self.tr(" floor", " Boden"), MUTED), ("≈", MUTED),
                              (self.number(found["intensity"], 1) + " kWh/(m²·a)", INK)], 4.6, -2.5, size=14, edge="center", width=4.6)
        self.step("room_2", mn.FadeIn(bars0), mn.FadeIn(cost0), mn.FadeOut(s.choices), mn.FadeIn(s.anatomy), mn.FadeIn(per_floor), seconds=1.5)
        s.room = self.room_section(s.focus, ROOM_AT, ROOM_SCALE, "summer", demand=False)
        s.bars = self.energy_bars(s.focus, BAR_AT, BAR_HEIGHT, s.y_range[1], width=1.1, compact=True, size=13, digits=1)
        s.legend = self._bar_legend(s)
        s.cost = self._cost_chip(s.focus, (3.3, 3.45, 0))
        corner = lambda fx, fy: s.axes.c2p(s.x_range[0] + fx * (s.x_range[1] - s.x_range[0]), s.y_range[0] + fy * (s.y_range[1] - s.y_range[0]))
        s.better = mn.VGroup(mn.Arrow(corner(.34, .11), corner(.1, .03), buff=0, color=GOOD, stroke_width=4, max_tip_length_to_length_ratio=.3),
                             self.label("better", "besser", corner(.5, .1), size=18, color=GOOD, width=1.2))
        self.step("objectives", mn.FadeOut(room0), mn.FadeOut(sun0), mn.FadeOut(bars0), mn.FadeOut(cost0), mn.FadeOut(s.anatomy), mn.FadeOut(per_floor), mn.FadeIn(s.sun), mn.FadeIn(s.axes), mn.FadeIn(s.labels), mn.FadeIn(s.room),
                  mn.FadeIn(s.bars), mn.FadeIn(s.legend), mn.FadeIn(s.cost), mn.FadeIn(s.better), seconds=1.8)
        s.cursor = self.live_marker(s.axes, s.focus, lambda d: _room.result(d)["cost"], lambda d: _room.result(d)["electricity"],
                                    x_unit="CHF", y_unit="kWh/a", color=COST, readout=False)
        s.readout = self._readout(s)
        self.step("objectives_2", mn.FadeIn(s.cursor), mn.FadeIn(s.readout), mn.Indicate(s.cost, color=COST, scale_factor=1.1), seconds=1.5)
        hops = s.designs[::max(1, len(s.designs) // 8)]
        levels = [_room.Number(str(len({d["design"][key] for d in s.designs}))) for key in ("wwr", "insulation", "overhang")]
        corner = s.axes.c2p(s.x_range[1], s.y_range[1]) + mn.LEFT * .15 + mn.DOWN * .08
        s.total = total = mn.VGroup(_room.place(_room.chip([levels[0], self.tr("window shares", "Fensteranteile"), "×", levels[1], self.tr("insulations", "Dämmstärken"), "×", levels[2], self.tr("overhangs", "Vordächer")], size=13, color=INK), corner, "right"),
                                    _room.place(_room.chip(["=", _room.Number(str(len(s.designs))), self.tr("designs", "Entwürfe")], size=14, color=INK), corner + mn.DOWN * .42, "right"))
        s.plain = self.live_marker(s.axes, s.focus, lambda d: _room.result(d)["cost"], lambda d: _room.result(d)["electricity"], x_unit="CHF", y_unit="kWh/a", color=COST, readout=False, drops=False)

        def drop_lines():
            spot = s.spot(_room.result(_room.values(s.focus)))
            return mn.VGroup(_room.dashed(spot, [spot[0], s.axes.c2p(0, s.y_range[0])[1], 0], color=MUTED, stroke_width=1.6), _room.dashed(spot, [s.axes.c2p(s.x_range[0], 0)[0], spot[1], 0], color=MUTED, stroke_width=1.6))

        s.drops = self._live(drop_lines, _room.design_key(s.focus))
        s.halo = self._live(lambda: mn.VGroup(mn.Circle(radius=.3, color=COST, stroke_width=4).move_to(s.spot(_room.result(_room.values(s.focus)))).set_z_index(10)), _room.design_key(s.focus))
        self.remove(s.cursor)
        self.add(s.plain, s.halo, s.drops)
        self.step("grid", mn.FadeOut(s.readout), mn.FadeIn(total), mn.FadeOut(s.better), mn.LaggedStart(*[mn.FadeIn(tile) for tile in s.tiles.values()], lag_ratio=.07),
                  mn.Succession(*[self.set_design(s.focus, d["design"], .5) for d in hops]), seconds=4.5)
        by_cooling = max(s.designs, key=lambda d: d["cooling"])
        by_light = max((d for d in s.designs if d["design"]["overhang"] > 0), key=lambda d: d["daylight"])
        by_lamps = max(s.designs, key=lambda d: d["lighting"])
        self.step("grid_2", mn.Succession(*[self.set_design(s.focus, d["design"], 1.3) for d in (by_cooling, by_light, by_lamps)]), seconds=4.2)
        cheap = min(s.designs, key=lambda d: (d["cost"], d["electricity"]))
        frugal = min(s.designs, key=lambda d: (d["electricity"], d["cost"]))
        ideal = s.axes.c2p(cheap["cost"], frugal["electricity"])
        s.ideal = mn.VGroup(mn.DashedLine(s.spot(cheap), ideal, color=MUTED, stroke_width=2), mn.DashedLine(s.spot(frugal), ideal, color=MUTED, stroke_width=2),
                            _art.cross_mark(GOOD, .1).move_to(ideal),
                            self.label("best of both (not a façade)", "beides optimal (keine Fassade)", ideal + mn.RIGHT * .1 + mn.DOWN * .3, size=15, color=GOOD, width=2.9).shift(mn.RIGHT * 1.4))
        names = (self.tr("Cheapest", "Günstigste"), self.tr("Most frugal", "Sparsamste"))
        dc, de = round(frugal["cost"]) - round(cheap["cost"]), round(cheap["electricity"]) - round(frugal["electricity"])
        s.extremes = mn.VGroup(self._pair_card(s, cheap, frugal, names=names, tones=(COST, INK), rings=(COST, GOOD)),
                               _at(_room.row(15, MUTED, self.tr("most frugal minus cheapest", "Sparsamste minus Günstigste")), SIDE_LEFT, ROWS[0]),
                               self._eq([(self.number(frugal["cost"]) + " − " + self.number(cheap["cost"]) + " = ", MUTED), (self._signed(dc, 0) + " CHF", COST)], SIDE_LEFT, ROWS[1], size=17, width=4.2),
                               self._eq([(self.number(frugal["electricity"]) + " − " + self.number(cheap["electricity"]) + " = ", MUTED), (self._signed(-de, 0) + " kWh/a", INK)], SIDE_LEFT, ROWS[2], size=17, width=4.2))
        self.step("extremes", mn.Succession(self.set_design(s.focus, cheap["design"], 1.4), self.set_design(s.focus, frugal["design"], 1.6)),
                  mn.FadeIn(s.ideal, run_time=.6), mn.FadeIn(s.extremes, run_time=1.2), seconds=3.2)

    def _choices(self, s):
        """🧭 The three design choices with their levels, in the colours of the room's chips."""
        rows = ((self.tr("Window share (WWR)", "Fensteranteil (WWR)"), "wwr", 100, "%", 0, GLASS), (self.tr("Insulation thickness (t)", "Dämmstärke (t)"), "insulation", 100, "cm", 0, _room.INSULATION),
                (self.tr("Overhang depth (d)", "Vordachtiefe (d)"), "overhang", 1, "m", 1, COST))
        group = mn.VGroup(_at(_room.row(17, MUTED, self.tr("What we can choose", "Was wir wählen können")), LEFT_STRIP, 2.2))
        for index, (name, key, factor, unit, digits, color) in enumerate(rows):
            levels = sorted({d["design"][key] for d in s.designs})
            y = 1.55 - index * .85
            group.add(self._eq([(name, color)], LEFT_STRIP, y, size=16, width=STRIP_WIDTH + .15),
                      self._eq([(" ".join(self.number(level * factor, digits) for level in levels) + " " + unit, INK)], LEFT_STRIP, y - .32, size=14, width=STRIP_WIDTH))
        return group

    def _anatomy(self, s, trackers):
        """🧾 Where the façade cost comes from: fixed part, wall, insulation, glass and overhang of the design in the trackers, adding up to the price."""
        design = _room.values(trackers)
        found = _room.result(design)
        parts = _cost_parts(design)
        shown = dict(zip(parts, _room.apportion(list(parts.values()), 0, found["cost"])))
        wall = self.quantity(found["wall_area"], "m²", 1)
        names = {"fixed": (self.tr("Fixed items", "Feste Posten"), MUTED), "wall": (self.tr("Wall ", "Wand ") + wall, INK),
                 "insulation": (self.tr("Insulation ", "Dämmung ") + self.quantity(design["insulation"] * 100, "cm"), _room.INSULATION),
                 "glass": (self.tr("Glass ", "Glas ") + self.quantity(found["window_area"], "m²", 1), GLASS), "overhang": (self.tr("Overhang ", "Vordach ") + self.quantity(design["overhang"], "m", 1), COST)}
        group = mn.VGroup(_at(_room.row(17, COST, self.tr("Façade cost [CHF]", "Fassadenkosten [CHF]")), LEFT_STRIP, 2.2))
        for index, key in enumerate(parts):
            y = 1.72 - index * .4
            name, color = names[key]
            group.add(_at(_room.row(14, color, name), LEFT_STRIP, y), _at(_room.row(14, color, self.number(shown[key])), LEFT_STRIP + STRIP_WIDTH, y, "right"))
        y = 1.72 - len(parts) * .4 - .06
        group.add(mn.Line([LEFT_STRIP, y + .28, 0], [LEFT_STRIP + STRIP_WIDTH, y + .28, 0], color=MUTED, stroke_width=1.4),
                  _at(_room.row(16, COST, self.tr("Total", "Summe")), LEFT_STRIP, y), _at(_room.row(16, COST, self.number(found["cost"])), LEFT_STRIP + STRIP_WIDTH, y, "right"))
        return group

    def _pair_card(self, s, first, second, *, names=None, tones=(MUTED, INK), rings=(MUTED, GOOD), tag=None, only_differing=False, daylight=False):
        """🆚 Two façade thumbnails with their variables (the ones that differ in colour), cost and electricity; ``.numbers`` holds cost and electricity separately."""
        german, base, numbers = s.german, mn.VGroup(), mn.VGroup()
        differs = {key for key in VARIABLES if abs(first["design"][key] - second["design"][key]) > 1e-9}
        keys = [key for key in VARIABLES if key in differs] if only_differing else list(VARIABLES)
        top = (-1.95 if names else -1.6 - .2 * len(keys) - .04)
        for slot, (item, tone, ring) in enumerate(zip((first, second), tones, rings)):
            x = PAIR_X[slot]
            base.add(self.thumbnail(item["design"], (x, PAIR_Y, 0), PAIR_SCALE, state="filled", ramp=s.ramp, ring=ring))
            if names:
                base.add(_visuals._text(names[slot], (x, -1.62, 0), size=15, color=tone, width=1.7))
            else:
                for row, key in enumerate(keys):
                    label, factor, unit, digits = VARIABLES[key]
                    digits = 2 if key == "overhang" and abs(round(item["design"][key], 1) - item["design"][key]) > 1e-6 else digits
                    base.add(_room.text([label, *_room.parts(item["design"][key] * factor, unit, digits, german)], (x, -1.6 - .2 * row, 0), size=14, color=VARIABLE_COLOR[key] if key in differs else SOFT))
            numbers.add(_room.text(_room.parts(item["cost"], "CHF", 0, german), (x, top, 0), size=14, color=COST if tone is not MUTED else SOFT),
                        _room.text(_room.parts(item["electricity"], "kWh/a", 0, german), (x, top - .22, 0), size=14, color=SOFT if tone is MUTED else INK))
            if daylight:
                numbers.add(_room.text(_room.parts(item["daylight"], "%", 1, german), (x, top - .44, 0), size=14, color=GLASS))
        base.add(mn.Arrow((PAIR_X[0] + .4, PAIR_Y, 0), (PAIR_X[1] - .4, PAIR_Y, 0), buff=0, color=INK, stroke_width=3, tip_length=.14, max_tip_length_to_length_ratio=.4))
        if tag:
            base.add(_room.text(tag, ((PAIR_X[0] + PAIR_X[1]) / 2, PAIR_Y + .26, 0), size=13, color=MUTED))
        card = mn.VGroup(base, numbers)
        card.base, card.numbers = base, numbers
        return card

    def _pareto_dominance(self, s):
        """🥊 Quadrant, check rows and the greyed loser."""
        pair = s.data["dominated_example"]
        winner, loser = s.by_id[pair["winner"]], s.by_id[pair["loser"]]
        card = self._pair_card(s, loser, winner, tag=[self.tr("compare", "Vergleich")])
        self.step("dominated", mn.FadeOut(s.ideal), mn.FadeOut(s.extremes), self.set_design(s.focus, loser["design"], 1.5), mn.FadeIn(card.base), seconds=1.5)
        ring = mn.Circle(radius=.26, color=COST, stroke_width=3).move_to(s.spot(loser))
        low, high = s.axes.c2p(s.x_range[0], s.y_range[0]), s.spot(loser)
        quadrant = mn.Rectangle(width=high[0] - low[0], height=high[1] - low[1], stroke_width=2, color=GOOD, fill_color=GOOD, fill_opacity=.13).move_to((low + high) / 2)
        mid = [s.spot(winner)[0], high[1], 0]
        dc, de = round(winner["cost"]) - round(loser["cost"]), round(winner["electricity"]) - round(loser["electricity"])
        cost_leg = _room.place(_room.chip([self._signed(dc, 0), "CHF"], size=14, color=COST), (mid[0] + .5 * (high[0] - mid[0]), high[1] + .3, 0), "center")
        power_leg = _room.place(_room.chip([self._signed(de, 0), "kWh/a"], size=14, color=INK), (mid[0] - .15, high[1] + .3, 0), "left")
        equations = mn.VGroup(self._eq([(self.tr("cost:", "Kosten:"), MUTED), (self.number(winner["cost"]) + " − " + self.number(loser["cost"]) + " =", INK), (self._signed(dc, 0) + " CHF", COST)], SIDE_LEFT, ROWS[0], size=17, width=4.0),
                              self._eq([(self.tr("electricity:", "Strombedarf:"), MUTED), (self.number(winner["electricity"]) + " − " + self.number(loser["electricity"]) + " =", INK), (self._signed(de, 0) + " kWh/a", INK)],
                                       SIDE_LEFT, ROWS[1], size=17, width=4.0))
        legs = mn.VGroup(mn.DashedLine(high, mid, color=COST, stroke_width=3), mn.DashedLine(mid, s.spot(winner), color=INK, stroke_width=3), cost_leg.set_z_index(12), power_leg.set_z_index(12))
        note = self._eq([(self.tr("designs that beat it", "bessere Entwürfe"), GOOD)], low[0] + .55, low[1] + .2, size=14, width=1.9, z=12)
        self.add(ring)
        self.step("dominated_2", mn.FadeIn(quadrant), mn.FadeIn(note), self.set_design(s.focus, winner["design"], 2.0), mn.FadeIn(legs, run_time=1), mn.FadeIn(card.numbers), mn.FadeIn(equations), seconds=2.4)
        cross = _art.cross_mark(BAD, .1).move_to(s.spot(loser)).set_z_index(12)
        stamp = _room.place(_room.chip([self.tr("dominated", "dominiert")], size=17, color=BAD), (4.4, 3.0, 0), "center")
        self.step("dominated_3", self._morph(s, {loser["id"]: {"plain": 0, "grey": 1}}), self.set_design(s.focus, loser["design"], 1.4), mn.FadeIn(cross), mn.FadeIn(stamp), mn.FadeOut(ring), mn.FadeOut(quadrant),
                  mn.FadeOut(legs), mn.FadeOut(note), mn.FadeOut(equations), seconds=1.4)
        rows = mn.VGroup(self._row([(self.tr("cost ", "Kosten ") + self.number(winner["cost"]) + " ≤ " + self.quantity(loser["cost"], "CHF"), COST)], True, ROWS[0], x=2.55, size=17),
                         self._row([(self.tr("electricity ", "Strombedarf ") + self.number(winner["electricity"]) + " ≤ " + self.quantity(loser["electricity"], "kWh/a"), INK)], True, ROWS[1], x=2.55, size=17))
        strict = mn.VGroup(self._row([(self.tr("cost: strictly less, ", "Kosten: echt weniger, ") + self._signed(dc, 0) + " CHF", COST)], True, ROWS[2], x=2.55, size=17),
                           self._row([(self.tr("electricity: strictly less, ", "Strombedarf: echt weniger, ") + self._signed(de, 0) + " kWh/a", INK)], True, ROWS[3], x=2.55, size=17))
        self.step("rule", mn.FadeIn(rows), seconds=1.5)
        self.step("rule_2", mn.FadeIn(strict), seconds=1.0)
        s.pair_group = mn.VGroup(card, rows, strict, cross, stamp)

    def _sweep(self, s, order, kept):
        """🧹 Sweep line with a running-minimum staircase that greys or rings every tile it passes."""
        lo, hi = s.x_range
        line = mn.Line(s.axes.c2p(lo, s.y_range[0]), s.axes.c2p(lo, s.y_range[1]), color=COST, stroke_width=3)
        stair = mn.VMobject(color=GOOD, stroke_width=4)
        group, done = mn.VGroup(line, stair), set()

        def update(mob, alpha):
            x = lo + (hi - lo) * alpha
            line.put_start_and_end_on(s.axes.c2p(x, s.y_range[0]), s.axes.c2p(x, s.y_range[1]))
            points = []
            for item in order:
                if item["cost"] <= x and item["id"] not in done:
                    done.add(item["id"])
                    tile = s.tiles[item["id"]]
                    if item["id"] in kept:
                        tile.weights.update(ring=1.0)
                        tile.set_z_index(3)
                        self._put(s, item["design"])
                    else:
                        tile.weights.update(plain=0.0, grey=1.0, light=0.0)
                    self._apply(tile)
                if item["cost"] <= x and item["id"] in kept:
                    points.append(s.spot(item))
            if points:
                corners = [points[0]]
                for a, b in zip(points, points[1:]):
                    corners += [[b[0], a[1], 0], b]
                corners.append([s.axes.c2p(x, 0)[0], points[-1][1], 0])
                stair.set_points_as_corners(corners)

        return group, mn.UpdateFromAlphaFunc(group, update, rate_func=mn.linear)

    def _path(self, s):
        """🛤️ The way from the beaten front member to the off-grid design: one variable at a time, with the model's cost and electricity change of each stage."""
        refined = s.data["refinement"]
        member = s.by_id[refined["member"]]
        designs = [member["design"]]
        for key in PATH_ORDER:
            designs.append({**designs[-1], key: refined["design"][key]})
        found = [_room.result(design) for design in designs]
        assert abs(found[-1]["cost"] - refined["cost"]) < 1e-6
        stages = [{"key": key, "before": designs[k], "after": designs[k + 1], "cost": found[k + 1]["cost"] - found[k]["cost"],
                   "electricity": found[k + 1]["electricity"] - found[k]["electricity"]} for k, key in enumerate(PATH_ORDER)]
        return SimpleNamespace(designs=designs, stages=stages, cost=found[-1]["cost"] - found[0]["cost"], electricity=found[-1]["electricity"] - found[0]["electricity"])

    def _path_rows(self, s, path):
        """🧾 Live rows of the path: variable change, cost change and electricity change per stage, then the net; they add up exactly."""
        german = s.german
        costs = _room.apportion([stage["cost"] for stage in path.stages], 1, path.cost)
        powers = _room.apportion([stage["electricity"] for stage in path.stages], 1, path.electricity)

        def build():
            group, stage = mn.VGroup(), s.stage.get_value()
            for k, item in enumerate(path.stages):
                if stage > k + .5:
                    y = ROWS[k]
                    group.add(_at(_room.row(13, VARIABLE_COLOR[item["key"]], *self._change(item["key"], item["before"], item["after"])), SIDE_LEFT, y),
                              _at(_room.row(13, COST, self._signed(costs[k], 1) + " CHF"), 5.5, y, "right"),
                              _at(_room.row(13, INK, self._signed(powers[k], 1) + " kWh/a"), SIDE_RIGHT, y, "right"))
            if stage > 2.5:
                y = ROWS[3]
                group.add(mn.Line([SIDE_LEFT, y + .2, 0], [SIDE_RIGHT, y + .2, 0], color=MUTED, stroke_width=1.4),
                          _at(_room.row(15, GOOD, self.tr("net", "gesamt")), SIDE_LEFT, y),
                          _at(_room.row(13, GOOD, self._signed(sum(costs), 1) + " CHF"), 5.5, y, "right"),
                          _at(_room.row(13, GOOD, self._signed(sum(powers), 1) + " kWh/a"), SIDE_RIGHT, y, "right"))
            return group

        return self._live(build, lambda: tuple(s.stage.get_value() > k + .5 for k in range(4)))

    def _pareto_front(self, s):
        """🏅 Sweep, front, the unevaluated design and the two spaces."""
        order = sorted(s.designs, key=lambda d: (d["cost"], d["electricity"]))
        kept, best = [], math.inf
        for item in order:
            if item["electricity"] < best:
                kept.append(item["id"])
                best = item["electricity"]
        assert kept == s.front
        group, sweep = self._sweep(s, order, set(kept))
        self.add(group)
        notes = mn.VGroup(*[self._eq([(text, color)], BAND_LEFT, y, size=16, width=4.5) for text, color, y in
                            ((self.tr("orange line: cost swept so far", "orange Linie: bisher geprüfte Kosten"), COST, ROWS[0]),
                             (self.tr("green steps: lowest electricity so far", "grüne Stufen: bisher geringster Strombedarf"), GOOD, ROWS[1]),
                             (self.tr("grey: beaten by a cheaper façade", "grau: von einer günstigeren Fassade geschlagen"), MUTED, ROWS[2]))])
        self.step("front", sweep, mn.FadeOut(s.pair_group), mn.FadeIn(notes), seconds=6)
        points = [s.spot(s.by_id[i]) for i in s.front]
        s.guide = mn.DashedVMobject(mn.VMobject(color=GOOD, stroke_width=3).set_points_as_corners(points), num_dashes=40)
        s.thumbs = self._legend(s.ramp, self.tr("Tile colour: electricity", "Kachelfarbe: Strombedarf"), "kWh/a")
        front_note = self._eq([(self.tr("Pareto front of the ", "Pareto-Front der ") + self.number(len(s.designs)) + self.tr(" simulated designs", " simulierten Entwürfe"), GOOD)], BAND_LEFT, ROWS[0], size=17, width=4.6)
        self.step("front_2", mn.FadeOut(group), mn.FadeOut(notes), mn.FadeIn(s.guide), mn.FadeIn(s.thumbs), mn.FadeIn(front_note),
                  mn.LaggedStart(*[mn.Flash(point, color=GOOD, flash_radius=.35, line_length=.12) for point in points], lag_ratio=.15), seconds=2.2)
        refined, path = s.data["refinement"], self._path(s)
        member = s.by_id[s.front[-1]]
        gone = [i for i in refined["dominates"] if i in s.front]
        spot = s.axes.c2p(refined["cost"], refined["electricity"])
        ghost = self.thumbnail(refined["design"], spot, THUMB * 1.25, state="ghost")
        ghost.set_z_index(4)
        swapped = sorted([s.spot(s.by_id[i]) for i in s.front if i not in gone] + [spot], key=lambda point: point[0])
        guide2 = mn.DashedVMobject(mn.VMobject(color=GOOD, stroke_width=3).set_points_as_corners(swapped), num_dashes=40)
        tag_at = s.axes.c2p(refined["cost"], s.y_range[1] - .13 * (s.y_range[1] - s.y_range[0])) + mn.LEFT * 1.35
        tag = mn.VGroup(self.label("not in the grid", "nicht im Raster", tag_at, size=16, color=INK, width=1.8),
                        _room.text([*_room.parts(refined["cost"], "CHF", 0, s.german), "·", *_room.parts(refined["electricity"], "kWh/a", 0, s.german)], tag_at + mn.DOWN * .26, size=14, color=COST),
                        _room.text([self.tr("dominates", "dominiert"), _room.Number(str(len(gone))), self.tr("front façades", "Frontfassaden")], tag_at + mn.DOWN * .5, size=14, color=GOOD),
                        mn.Line(tag_at + mn.DOWN * .66, spot + mn.UP * .2, color=INK, stroke_width=1.4, stroke_opacity=.6))
        home = s.guide.copy()
        beaten = s.by_id[refined["member"]]
        card = self._pair_card(s, beaten, refined, tones=(MUTED, INK), rings=(MUTED, COST), tag=[self.tr("better", "besser")])
        rows = self._path_rows(s, path)
        changes = self._changes(s, lambda: (path.designs[min(2, max(0, math.ceil(s.stage.get_value() - 1e-9) - 1))], path.designs[min(3, max(1, math.ceil(s.stage.get_value() - 1e-9)))]),
                                lambda: (round(s.stage.get_value(), 3), _room.design_key(s.focus)()))
        walk = mn.Succession(*[mn.AnimationGroup(self.set_design(s.focus, {item["key"]: item["after"][item["key"]]}, 1.1), s.stage.animate(run_time=1.1).set_value(k + 1)) for k, item in enumerate(path.stages)])
        self.add(rows)
        self.step("approximate", mn.FadeOut(s.thumbs), mn.FadeOut(front_note), mn.FadeIn(ghost), mn.FadeIn(tag), mn.FadeIn(card), mn.FadeIn(changes),
                  self._morph(s, {i: {"plain": 0, "grey": 1, "ring": 0} for i in gone}), mn.ReplacementTransform(s.guide, guide2), walk, seconds=4.2)
        s.guide = home
        heads = mn.VGroup(self.label("OBJECTIVE SPACE: cost · electricity", "ZIELRAUM: Kosten · Strombedarf", s.axes.c2p(s.x_range[0], s.y_range[1]) + mn.RIGHT * 1.6 + mn.DOWN * .7, size=16, color=GOOD, width=3.0),
                          self.label("DESIGN SPACE", "ENTWURFSRAUM", (3.4, 2.66, 0), size=17, color=GLASS, width=2.4),
                          self.label("window share · overhang · insulation", "Fensteranteil · Vordach · Dämmung", (3.4, 2.4, 0), size=14, color=GLASS, width=3.0),
                          mn.Arrow((2.2, -1.3, 0), (-1.15, -1.3, 0), buff=0, color=INK, stroke_width=3, tip_length=.16),
                          self.label("simulation", "Simulation", (.5, -1.05, 0), size=16, color=INK, width=1.6))
        s.heads = heads
        self.step("sets", mn.FadeOut(s.total), mn.FadeOut(ghost), mn.FadeOut(tag), mn.FadeOut(card), mn.FadeOut(rows), mn.FadeOut(changes), self._morph(s, {i: {"plain": 1, "grey": 0, "ring": 1} for i in gone}),
                  mn.ReplacementTransform(guide2, home), self.set_design(s.focus, member["design"], 1.2), mn.FadeIn(heads), seconds=1.4)

    def _step_card(self, s):
        """🧾 Live card of the front step being walked: both façades with the changed variable in colour, and the model numbers of the destination."""
        count = len(s.steps)

        def build():
            if s.flags["pair"].get_value() < .5:
                return mn.VGroup()
            index = s.index()
            step = s.steps[index]
            card = self._pair_card(s, s.by_id[step["from"]], s.by_id[step["to"]], tones=(INK, INK), rings=(MUTED, GOOD),
                                   tag=[self.tr("Step", "Schritt"), _room.Number(str(index + 1)), self.tr("of", "von"), _room.Number(str(count))])
            return mn.VGroup(card.base, card.numbers)

        return self._live(build, lambda: (s.index(), s.flags["pair"].get_value() > .5))

    def _pay(self, s, step, rate):
        """💶 Printed numbers of a step at a tariff: saved kWh/a from the printed one-decimal totals (two decimals for small steps) and the yearly saving rounded half up from them."""
        a, b = s.by_id[step["from"]], s.by_id[step["to"]]
        saved = round(round(a["electricity"], 1) - round(b["electricity"], 1), 1) if step["saved"] >= 10 else round(step["saved"], 2)
        yearly = saved * rate
        digits = 1 if yearly >= 10 else 2 if yearly >= .1 else 3
        return saved, 1 if step["saved"] >= 10 else 2, math.floor(yearly * 10 ** digits + .5) / 10 ** digits, digits

    def _step_rows(self, s):
        """💰 Live equations of the step just walked: extra cost, saved electricity and price per saved kWh/a."""
        flags, german = s.flags, s.german

        def build():
            if flags["cards"].get_value() < .5:
                return mn.VGroup()
            step = s.steps[s.index()]
            a, b = s.by_id[step["from"]], s.by_id[step["to"]]
            places = 1
            cost = round(b["cost"]) - round(a["cost"])
            saved = round(round(a["electricity"], 1) - round(b["electricity"], 1), 1)
            group = mn.VGroup(self._eq([(self.tr("extra cost:", "Mehrkosten:"), MUTED), (self.number(b["cost"]) + " − " + self.number(a["cost"]) + " =", INK), (self._signed(cost, 0) + " CHF", COST)],
                                       SIDE_LEFT, ROWS[0], size=16, width=4.0),
                              self._eq([(self.tr("saved:", "Ersparnis:"), MUTED), (self.number(a["electricity"], places) + " − " + self.number(b["electricity"], places) + " =", INK),
                                        (self.number(saved, places) + " kWh/a", INK)], SIDE_LEFT, ROWS[1], size=16, width=4.0))
            if flags["price"].get_value() > .5:
                group.add(self._eq([(self.number(cost) + " CHF ÷ " + self.number(saved, places) + " kWh/a ≈", INK),
                                    (self.number(step["ratio"], 1) + " " + self.tr("CHF per kWh/a", "CHF je kWh/a"), GOOD)], SIDE_LEFT, ROWS[2], size=16, width=4.0))
            return group

        return self._live(build, lambda: (s.index(), flags["cards"].get_value() > .5, flags["price"].get_value() > .5))

    def _chart_y(self, chf):
        return CHART_ORIGIN[1] + CHART_HEIGHT * min(chf, CHART_CHF) / CHART_CHF

    def _chart_x(self, years):
        return CHART_ORIGIN[0] + CHART_WIDTH * min(years, CHART_YEARS) / CHART_YEARS

    def _payback_chart(self, s, tariff):
        """📈 Live chart: the yearly saving at the tariff adds up over the years against the step's extra cost; where the lines cross is the payback."""
        german, flags = s.german, s.flags

        def build():
            if flags["chart"].get_value() < .5:
                return mn.VGroup()
            rate, step = tariff.get_value(), s.steps[s.index()]
            per_year, years = step["saved"] * rate, step["ratio"] / rate
            pay = self._pay(s, step, rate)
            ok, life = years <= s.life, flags["life"].get_value() > .5
            origin, top = CHART_ORIGIN, self._chart_y(CHART_CHF)
            group = mn.VGroup(mn.Line(origin, [origin[0] + CHART_WIDTH, origin[1], 0], color=MUTED, stroke_width=2), mn.Line(origin, [origin[0], top, 0], color=MUTED, stroke_width=2))
            for tick in (0, 10, 20, 30):
                group.add(mn.Line([self._chart_x(tick), origin[1], 0], [self._chart_x(tick), origin[1] - .05, 0], color=MUTED, stroke_width=1.5),
                          _at(_room.row(13, GOOD if tick == 30 and life else MUTED, str(tick), *(["a"] if tick == 30 else [])), self._chart_x(tick), origin[1] - .22, "center"))
            for tick in (0, 200, 400, 600):
                group.add(_at(_room.row(13, MUTED, str(tick)), origin[0] - .08, self._chart_y(tick) - .045, "right"))
            group.add(_at(_room.row(13, MUTED, "CHF"), 0, 0, "center").rotate(mn.PI / 2).move_to([origin[0] - .53, (origin[1] + top) / 2, 0]))
            invest = self._chart_y(step["cost"])
            group.add(mn.DashedLine([origin[0], invest, 0], [origin[0] + CHART_WIDTH, invest, 0], color=COST, stroke_width=2.4))
            end = min(CHART_YEARS, CHART_CHF / per_year)
            group.add(mn.Line([origin[0], origin[1], 0], [self._chart_x(end), self._chart_y(per_year * end), 0], color=GOOD, stroke_width=4))
            group.add(self._eq([(self.tr("cost", "Kosten") + " " + self.quantity(step["cost"], "CHF"), COST), ("·", MUTED),
                                (self.tr("saves", "spart") + " " + self.quantity(pay[2], "CHF/a", pay[3]), GOOD)], origin[0], -1.18, size=13, width=3.25))
            if life:
                group.add(mn.DashedLine([self._chart_x(s.life), origin[1], 0], [self._chart_x(s.life), top + .05, 0], color=GOOD, stroke_width=2))
            if years <= CHART_YEARS:
                group.add(mn.Dot([self._chart_x(years), invest, 0], radius=.07, color=GOOD if ok else BAD),
                          mn.DashedLine([self._chart_x(years), invest, 0], [self._chart_x(years), origin[1], 0], color=GOOD if ok else BAD, stroke_width=1.8))
                if years < 26:
                    group.add(_at(_room.row(13, GOOD if ok else BAD, *_room.parts(years, "a", 0, german)), self._chart_x(years) + .09, origin[1] + .12))
            else:
                group.add(_at(_room.row(13, BAD, self.tr("payback", "Amortisation"), *_room.parts(years, "a", 0, german), "→"), self._chart_x(29.5) - .05, invest + .12, "right"))
            return group

        return self._live(build, lambda: (round(tariff.get_value(), 3), s.index(), *[flags[key].get_value() > .5 for key in ("chart", "life")]))

    def _payback_title(self, s):
        """🏷️ Live chart title: which front step and which variable it changes."""
        count = len(s.steps)

        def build():
            if s.flags["chart"].get_value() < .5:
                return mn.VGroup()
            index = s.index()
            step = s.steps[index]
            a, b = s.by_id[step["from"]]["design"], s.by_id[step["to"]]["design"]
            changed = [key for key in VARIABLES if abs(a[key] - b[key]) > 1e-9]
            parts = [self.tr("Step", "Schritt"), _room.Number(str(index + 1)), self.tr("of", "von"), _room.Number(str(count))]
            return _at(_room.row(14, INK, *parts), CHART_ORIGIN[0] - .35, -.97)

        return self._live(build, lambda: (s.index(), s.flags["chart"].get_value() > .5))

    def _payback_rows(self, s, tariff):
        """🧮 Live arithmetic of the payback: yearly saving at the tariff, then extra cost divided by it, then the lifetime check."""
        flags, german = s.flags, s.german

        def build():
            if flags["chart"].get_value() < .5:
                return mn.VGroup()
            rate, step = tariff.get_value(), s.steps[s.index()]
            years = step["ratio"] / rate
            ok = years <= s.life
            saved, places, per_year, digits = self._pay(s, step, rate)
            group = mn.VGroup(
                self._eq([(self.number(saved, places) + " kWh/a", INK), ("×", MUTED), (self.number(rate, 2) + " CHF/kWh", COST), ("≈", MUTED), (self.number(per_year, digits) + " CHF/a", GOOD)],
                         SIDE_LEFT, PAYBACK_ROWS[0], size=15, width=4.0),
                self._eq([(self.number(step["cost"]) + " CHF", COST), ("÷", MUTED), (self.number(per_year, digits) + " CHF/a", GOOD), ("≈", MUTED), (self.number(years) + " a", GOOD if ok else BAD)],
                         SIDE_LEFT, PAYBACK_ROWS[1], size=15, width=4.0))
            if flags["life"].get_value() > .5:
                group.add(self._row([(self.number(years) + " a " + ("≤" if ok else ">") + " " + self.quantity(s.life, "a") + self.tr(" lifetime", " Lebensdauer"), GOOD if ok else BAD)], ok, PAYBACK_ROWS[2], size=15))
            return group

        return self._live(build, lambda: (round(tariff.get_value(), 3), s.index(), *[flags[key].get_value() > .5 for key in ("chart", "life")]))

    def _payback_table(self, s, shown, tariff):
        """📋 Payback in years of every front step at the tariff; grown cells only, green within the lifetime, red beyond."""
        german, flags, count = s.german, s.flags, len(s.steps)
        cell = .56

        def build():
            if flags["chart"].get_value() < .5:
                return mn.VGroup()
            rate, current = tariff.get_value(), s.index()
            group = mn.VGroup(_at(_room.row(13, MUTED, self.tr("Payback [a] per front step #", "Amortisation [a] je Frontschritt #")), SIDE_LEFT, TABLE_ROWS[0]))
            for index, step in enumerate(s.steps):
                if shown.get_value() - index <= .5:
                    continue
                years = step["ratio"] / rate
                x = SIDE_LEFT + cell * (index + .5)
                color = GOOD if years <= s.life else BAD
                group.add(_at(_room.row(13, INK if index == current else MUTED, "#" + str(index + 1)), x, TABLE_ROWS[1], "center"), _at(_room.row(13, color, self.number(years)), x, TABLE_ROWS[2], "center"))
                if index == current:
                    group.add(mn.Line([x - cell / 2 + .06, TABLE_ROWS[2] - .1, 0], [x + cell / 2 - .06, TABLE_ROWS[2] - .1, 0], color=INK, stroke_width=3))
            return group

        return self._live(build, lambda: (round(tariff.get_value(), 3), round(shown.get_value(), 2) > 0, tuple(shown.get_value() - i > .5 for i in range(count)), s.index(), flags["chart"].get_value() > .5))

    def _pareto_walk(self, s):
        """🚶 Walk the front: the room morphs, the steps are priced and the knee appears."""
        count = len(s.steps)
        walk, flags = s.walk, s.flags
        shown, tariff = mn.ValueTracker(0), mn.ValueTracker(s.tariff)

        def drive():
            position = walk.get_value()
            index = max(0, min(int(math.floor(position)), count - 1))
            a, b = s.by_id[s.front[index]]["design"], s.by_id[s.front[index + 1]]["design"]
            fraction = position - index
            for key in _room.KEYS:
                s.focus[key].set_value(a[key] + (b[key] - a[key]) * fraction)

        def trail():
            position = walk.get_value()
            points = [s.spot(s.by_id[i]) for i in s.front[:int(math.floor(position)) + 1]]
            index = int(math.floor(position))
            if index < count and position > index:
                a, b = s.spot(s.by_id[s.front[index]]), s.spot(s.by_id[s.front[index + 1]])
                points.append(a + (b - a) * (position - index))
            line = mn.VGroup()
            if len(points) > 1:
                line.add(mn.VMobject(color=GOOD, stroke_width=6).set_points_as_corners(points))
            return line

        first = s.by_id[s.front[0]]
        s.trail = self._live(trail)
        step_pair = lambda: (s.by_id[s.steps[s.index()]["from"]]["design"], s.by_id[s.steps[s.index()]["to"]]["design"])
        card, changes, rows = self._step_card(s), self._changes(s, step_pair, lambda: (s.index(), _room.design_key(s.focus)())), self._step_rows(s)
        chart, title, formulas, table = self._payback_chart(s, tariff), self._payback_title(s), self._payback_rows(s, tariff), self._payback_table(s, shown, tariff)
        self.step("walk", mn.FadeOut(s.heads), self.set_design(s.focus, first["design"], 1.2), mn.FadeIn(card), mn.FadeIn(changes), s.delta.animate(run_time=.1).set_value(1),
                  flags["pair"].animate(run_time=.1).set_value(1), seconds=1.4)
        self._drive(s, drive)
        self.add(s.trail, rows, chart, title, formulas, table)
        a, b = s.by_id[s.front[0]], s.by_id[s.front[1]]
        corner = [s.spot(b)[0], s.spot(a)[1], 0]
        cost_chip = _room.place(_room.chip(["+" + self.number(b["cost"] - a["cost"]), "CHF"], size=14, color=COST), ((s.spot(a)[0] + corner[0]) / 2 + .2, corner[1] + .34, 0), "center").set_z_index(12)
        power_chip = _room.place(_room.chip(["−" + self.number(a["electricity"] - b["electricity"]), "kWh/a"], size=14, color=INK), (corner[0] + .4, (corner[1] + s.spot(b)[1]) / 2, 0), "left").set_z_index(12)
        brackets = mn.VGroup(mn.DashedLine(s.spot(a), corner, color=COST, stroke_width=3), mn.DashedLine(corner, s.spot(b), color=INK, stroke_width=3), cost_chip, power_chip)
        self.step("walk_2", walk.animate(run_time=2.2).set_value(1), mn.FadeIn(brackets), flags["cards"].animate(run_time=.3).set_value(1), seconds=2.4)
        self.step("walk_3", flags["price"].animate(run_time=.3).set_value(1), mn.Indicate(s.trail, color=GOOD), seconds=1.2)
        self.step("payback", mn.FadeOut(brackets), flags["pair"].animate(run_time=.1).set_value(0), flags["cards"].animate(run_time=.1).set_value(0), flags["price"].animate(run_time=.1).set_value(0),
                  flags["chart"].animate(run_time=.1).set_value(1), shown.animate(run_time=1).set_value(1), seconds=1.4)
        self.step("payback_2", flags["life"].animate(run_time=.5).set_value(1), seconds=.8)
        self.step("knee", mn.Succession(*[mn.AnimationGroup(walk.animate(run_time=1.1).set_value(k), shown.animate(run_time=1.1).set_value(k)) for k in range(2, count)], shown.animate(run_time=.6).set_value(count)), seconds=4.5)
        knee_at = lambda: s.spot(s.by_id[s.knee(tariff.get_value())])
        corner = s.axes.get_corner(mn.DL) + mn.RIGHT * 2.05 + mn.UP * .42
        knee_ring = self._live(lambda: mn.VGroup(mn.Circle(radius=.27, color=GOOD, stroke_width=4).move_to(knee_at()),
                                                mn.Line(knee_at() + mn.DOWN * .27, corner + mn.UP * .15 + mn.LEFT * .3, color=GOOD, stroke_width=2),
                                                _room.place(_room.chip([self.tr("knee", "Knie")], size=16, color=GOOD), corner, "center").set_z_index(12)),
                               lambda: round(tariff.get_value(), 3))
        k0 = s.front.index(s.knee(s.tariff))
        self.step("knee_2", walk.animate(run_time=1.6).set_value(k0), s.peek.animate(run_time=1.6).set_value(1), s.delta.animate(run_time=.1).set_value(0), mn.FadeIn(knee_ring), seconds=1.8)
        k1 = s.front.index(s.knee(s.tariff_hi))
        self.step("knee_3", mn.Succession(tariff.animate(run_time=1.6).set_value(s.tariff_hi), mn.AnimationGroup(walk.animate(run_time=1.0).set_value(k1), s.peek.animate(run_time=1.0).set_value(0), s.delta.animate(run_time=1.0).set_value(1))), seconds=2.8)
        s.walk_objects = mn.VGroup(s.trail, rows, chart, title, formulas, table, knee_ring, card, changes)
        self._drive(s, None)

    def _daylight_formula(self, s):
        """☀️ Live substituted daylight factor of the design in the trackers: DF = τ · A_w · θ / (A_s · (1 − ρ²)), with the visible sky angle θ shrinking under a deep overhang.

        See https://en.wikipedia.org/wiki/Daylight_factor
        """
        study = _model.load_study()["parameters"]["daylight"]
        tau, surface, rho, sky, loss, half = (float(study[key]["value"]) for key in ("transmittance", "surface_area", "reflectance", "sky_angle", "sky_angle_loss", "overhang_half_depth"))
        angle = lambda design, found: sky * (1 - loss * design["overhang"] ** 2 / (design["overhang"] ** 2 + half ** 2))
        numbers = [(self.tr("DF =", "TQ ="), MUTED), self.live(tau, INK, 1), "·", self.live("window_area", GLASS, 1, "m²"), "·", self.live(angle, COST, 0, "°"), "/",
                   ("(" + self.quantity(surface, "m²") + " · " + self.number(1 - rho ** 2, 2) + ")", INK), "=", self.live("daylight", _art.P_YELLOW, 1, "%")]
        legend = self._eq([(self.tr("glass · window · sky angle / room surfaces", "Glas · Fenster · Himmelswinkel / Raumflächen"), MUTED)], SIDE_LEFT, ROWS[0] - .34, size=13, width=4.0)
        return mn.VGroup(self.formula(s.focus, [numbers], (SIDE_LEFT - .25, ROWS[0] + .05, 0), size=14, align="left", width=4.25), legend)

    def _pareto_daylight(self, s):
        """☀️ Daylight as third criterion: tile colour, one revived design and the grown front."""
        s.states = {key: dict(tile.weights) for key, tile in s.tiles.items()}
        legend = self._legend(s.light, self.tr("Tile colour: daylight factor", "Kachelfarbe: Tageslichtquotient"), "%", 1, point=(-4.8, 1.15, 0), width=2.2)
        third = s.data["third_example"]
        low_item, dominator = s.by_id[third["dominated_2d"]], s.by_id[third["dominator"]]
        chips, formula = self._daylight_chips(s), self._daylight_formula(s)
        self.step("third", mn.FadeOut(s.walk_objects), mn.FadeOut(s.guide), self._morph(s, {key: {"plain": 0, "grey": 0, "light": 1} for key in s.tiles}), s.delta.animate(run_time=.1).set_value(0),
                  s.peek.animate(run_time=.1).set_value(0), mn.FadeIn(legend), mn.FadeIn(chips), mn.FadeIn(formula), mn.FadeOut(s.drops), self.set_design(s.focus, dominator["design"], 1.4), seconds=2.2)
        card = self._pair_card(s, low_item, dominator, tones=(MUTED, INK), rings=(INK, GOOD), tag=[self.tr("compare", "Vergleich")], only_differing=True, daylight=True)
        dc, de, dd = round(low_item["cost"]) - round(dominator["cost"]), round(low_item["electricity"]) - round(dominator["electricity"]), round(low_item["daylight"], 1) - round(dominator["daylight"], 1)
        rows = mn.VGroup(self._row([(self.tr("Cost: ", "Kosten: ") + self._signed(dc, 0) + " CHF " + self.tr("more", "mehr"), COST)], False, ROWS[2], size=16),
                         self._row([(self.tr("Electricity: ", "Strombedarf: ") + self._signed(de, 0) + " kWh/a " + self.tr("more", "mehr"), INK)], False, ROWS[3], size=16),
                         self._row([(self.tr("Daylight: ", "Tageslicht: ") + self._signed(dd, 1) + " % " + self.tr("more", "mehr"), GLASS)], True, ROWS[4], size=16))
        mark = mn.VGroup(mn.Circle(radius=.27, color=INK, stroke_width=3).move_to(s.spot(low_item)), mn.Circle(radius=.27, color=GOOD, stroke_width=3).move_to(s.spot(dominator)))
        self.step("third_2", self.set_design(s.focus, low_item["design"], 1.8), mn.FadeIn(mark), mn.FadeIn(card), mn.FadeIn(rows), seconds=2.0)
        fresh = [key for key in s.data["front3"] if key not in s.front]
        counter = mn.VGroup(_room.place(_room.chip([self.tr("Front", "Front"), _room.Number(str(len(s.front))), self.tr("of", "von"), _room.Number(str(len(s.designs))), self.tr("with 2 objectives", "mit 2 Zielgrößen")],
                                                   size=13, color=INK), s.axes.c2p(s.x_range[0], s.y_range[1]) + mn.RIGHT * .2 + mn.DOWN * .3, "left"),
                            _room.place(_room.chip([self.tr("Front", "Front"), _room.Number(str(len(s.data["front3"]))), self.tr("of", "von"), _room.Number(str(len(s.designs))), self.tr("with 3 (green)", "mit 3 (grün)")],
                                                   size=13, color=GOOD), s.axes.c2p(s.x_range[0], s.y_range[1]) + mn.RIGHT * .2 + mn.DOWN * .65, "left"))
        self.step("third_3", self._morph(s, {key: {"ring": 1} for key in fresh}), self.set_design(s.focus, dominator["design"], 1.4), mn.FadeIn(counter), seconds=1.8)
        verdict = self._row([(self.tr("better in daylight, so not dominated", "beim Tageslicht besser, also nicht dominiert"), GOOD)], True, ROWS[4] - .0, size=16)
        verdict.shift(mn.DOWN * .0)
        self.step("third_4", mn.Succession(*[mn.Indicate(row, color=color, scale_factor=1.06) for row, color in zip(rows, (COST, INK, GLASS))]), mn.FadeOut(rows[2], run_time=.2), mn.FadeIn(verdict, run_time=.4), seconds=2.2)
        s.third_objects = mn.VGroup(mark, counter, legend, rows, card, chips, formula, verdict)

    def _worth(self, s, weight):
        """⚖️ CHF that one kWh/a is worth at the electricity weight: ratio of the front's cost range to its electricity range, scaled by the weights."""
        front = [s.by_id[i] for i in s.front]
        span_c = max(d["cost"] for d in front) - min(d["cost"] for d in front)
        span_e = max(d["electricity"] for d in front) - min(d["electricity"] for d in front)
        return weight * span_c / ((1 - weight) * span_e)

    def _worth_rows(self, s, weight, picked):
        """🧮 Live rows: what one kWh/a is worth, the price of the step into the pick and of the next step."""
        shown = lambda value: self.number(value, 0 if value >= 100 else 1)

        def build():
            worth, index = self._worth(s, weight.get_value()), s.front.index(picked()["id"])
            group = mn.VGroup(self._eq([(self.tr("1 kWh/a is worth", "1 kWh/a ist wert"), MUTED), (shown(worth) + " CHF", COST)], SIDE_LEFT, ROWS[0], size=16, width=4.0))
            if index > 0:
                group.add(self._row([(self.tr("step into the pick: ", "Schritt zur Wahl: ") + shown(s.steps[index - 1]["ratio"]) + " CHF " + self.tr("per kWh/a ≤ ", "je kWh/a ≤ ") + shown(worth), GOOD)], True, ROWS[1], size=15))
            if index < len(s.steps):
                group.add(self._row([(self.tr("next step: ", "nächster Schritt: ") + shown(s.steps[index]["ratio"]) + " CHF " + self.tr("per kWh/a > ", "je kWh/a > ") + shown(worth), BAD)], False, ROWS[2], size=15))
            return group

        return self._live(build, lambda: (picked()["id"], round(weight.get_value(), 3)))

    def _badge(self, s, item, label, color):
        """🔢 Numbered chip beside a tile at the first of eight directions where it covers no other tile, no front segment and stays inside the axes."""
        spot, box = s.spot(item), _room.chip([label], size=14, color=color)
        half = (box.width / 2, box.height / 2)
        left, bottom, right, top = *s.axes.c2p(s.x_range[0], s.y_range[0])[:2], *s.axes.c2p(s.x_range[1], s.y_range[1])[:2]
        others = [s.spot(d) for d in s.designs if d["id"] != item["id"]]
        guide = [s.spot(s.by_id[i]) for i in s.front]

        def distance(point, a, b):
            ab, ap = b[:2] - a[:2], point[:2] - a[:2]
            t = max(0.0, min(1.0, float(ab @ ap) / float(ab @ ab)))
            return float(math.hypot(*(ap - t * ab)))

        for angle in (135, 90, 180, 45, 225, 270, 0, 315):
            centre = spot + .5 * mn.np.array([math.cos(math.radians(angle)), math.sin(math.radians(angle)), 0])
            inside = left + half[0] < centre[0] < right - half[0] and bottom + half[1] < centre[1] < top - half[1]
            free = all(abs(centre[0] - o[0]) > half[0] + THUMB * 2.4 + .02 or abs(centre[1] - o[1]) > half[1] + THUMB * 2 + .02 for o in others)
            clear = all(distance(centre, a, b) > half[1] + .06 for a, b in zip(guide, guide[1:]))
            if inside and free and clear:
                break
        return box.move_to(centre).set_z_index(12)

    def _pareto_choice(self, s):
        """🧲 Weighted sum slides along the front; the closing cards name the choices."""
        german, weight = s.german, mn.ValueTracker(.5)
        picked = lambda: s.by_id[_model.weighted_pick(weight.get_value())]
        box = (s.x_range[0], s.y_range[0], s.x_range[1], s.y_range[1])
        ranges = {key: (min(s.by_id[i][key] for i in s.front), max(s.by_id[i][key] for i in s.front)) for key in ("cost", "electricity")}

        def iso():
            w, item = weight.get_value(), picked()
            span_c, span_e = ranges["cost"][1] - ranges["cost"][0], ranges["electricity"][1] - ranges["electricity"][0]
            found = _clip((item["cost"], item["electricity"]), (w * span_c, -(1 - w) * span_e), box)
            group = mn.VGroup(mn.Circle(radius=.27, color=COST, stroke_width=4).move_to(s.spot(item)))
            index = s.front.index(item["id"])
            if index > 0:
                group.add(mn.Circle(radius=.2, color=GOOD, stroke_width=3).move_to(s.spot(s.by_id[s.front[index - 1]])))
            if index < len(s.steps):
                group.add(mn.Circle(radius=.2, color=BAD, stroke_width=3).move_to(s.spot(s.by_id[s.front[index + 1]])))
            if found:
                group.add(mn.Line(s.axes.c2p(*found[0]), s.axes.c2p(*found[1]), color=COST, stroke_width=2.5))
            return group

        def slider():
            w, item = weight.get_value(), picked()
            left, right, y = BAND_LEFT + .3, 1.95, -1.75
            group = mn.VGroup(mn.Line([left, y, 0], [right, y, 0], color=MUTED, stroke_width=4), mn.Dot([left + (right - left) * w, y, 0], radius=.12, color=COST))
            group.add(_at(_room.row(15, COST, self.tr("Cost", "Kosten")), left, y + .28), _at(_room.row(15, INK, self.tr("Electricity", "Strombedarf")), right, y + .28, "right"),
                      _at(_room.row(15, COST, *_room.parts(100 * (1 - w), "%", 0, german)), left, y - .38), _at(_room.row(15, INK, *_room.parts(100 * w, "%", 0, german)), right, y - .38, "right"),
                      _room.place(_room.chip([self.tr("Pick", "Wahl"), *_room.parts(item["cost"], "CHF", 0, german), "·", *_room.parts(item["electricity"], "kWh/a", 0, german)], size=14, color=GOOD), (left, -2.42, 0), "left"))
            return group

        def pick():
            self._put(s, picked()["design"])

        restore = self._morph(s, {key: state for key, state in s.states.items()})
        isoline, bar, worth = self._live(iso), self._live(slider), self._worth_rows(s, weight, picked)
        note = self._eq([(self.tr("orange line: equal weighted score", "orange Linie: gleiche Bewertung"), COST)], SIDE_LEFT, ROWS[3], size=15, width=4.3)
        self._drive(s, pick)
        self.step("weights", mn.FadeOut(s.third_objects), mn.FadeIn(s.guide), restore, mn.FadeIn(isoline), mn.FadeIn(bar), mn.FadeIn(worth), mn.FadeIn(note), mn.FadeIn(s.thumbs), mn.FadeIn(s.drops), seconds=1.6)
        self.step("weights_2", mn.Succession(weight.animate(run_time=2, rate_func=mn.linear).set_value(.05), weight.animate(run_time=4, rate_func=mn.linear).set_value(.98)), seconds=6)
        self._drive(s, None)
        cheap = s.by_id[s.front[0]]
        knee = s.by_id[s.knee(s.tariff)]
        middle = s.by_id[_model.weighted_pick(.5)]
        worth_mid = self._worth(s, .5)
        knee_step = s.steps[s.front.index(knee["id"]) - 1]
        cards = []
        knee_name = self.tr("Knee at ", "Knie bei ") + self.quantity(s.tariff, "CHF/kWh", 2)
        options = ((self.tr("Cheapest", "Günstigste"), cheap, COST, self.tr("no extra spending", "keine Mehrausgabe")),
                   (knee_name, knee, GOOD, self.tr("payback ", "Amortisation ") + self.quantity(knee_step["payback"], "a") + " ≤ " + self.quantity(s.life, "a")),
                   (self.tr("Balanced weights", "Ausgewogene Gewichte"), middle, INK, self.tr("1 kWh/a is worth ", "1 kWh/a ist wert ") + self.quantity(worth_mid, "CHF", 1)))
        for index, (name, item, color, extra) in enumerate(options):
            y = 1.9 - index * 1.5
            tile = self.thumbnail(item["design"], (-.1, y, 0), .28, state="filled", ramp=s.ramp, ring=color)
            design = item["design"]
            line = "WWR " + self.quantity(design["wwr"] * 100, "%") + " · d " + self.quantity(design["overhang"], "m", 1) + " · t " + self.quantity(design["insulation"] * 100, "cm")
            text = mn.VGroup(_visuals._text(str(index + 1) + "  " + name, (0, 0, 0), size=17, color=color, width=3.8), _visuals._text(line, (0, 0, 0), size=14, color=MUTED, width=3.8),
                             _visuals._text(self.quantity(item["cost"], "CHF") + " · " + self.quantity(item["electricity"], "kWh/a"), (0, 0, 0), size=15, color=INK, width=3.8),
                             _visuals._text(extra, (0, 0, 0), size=14, color=color, width=3.8)).arrange(mn.DOWN, aligned_edge=mn.LEFT, buff=.1)
            text.move_to([tile.get_right()[0] + .25 + text.width / 2, y, 0])
            ring = mn.Circle(radius=.27, color=color, stroke_width=3).move_to(s.spot(item))
            badge = self._badge(s, item, str(index + 1), color)
            cards.append(mn.VGroup(tile, text, ring, badge))
        self.step("decision", mn.FadeOut(s.room), mn.FadeOut(s.sun), mn.FadeOut(s.bars), mn.FadeOut(s.legend), mn.FadeOut(s.cost), mn.FadeOut(isoline), mn.FadeOut(bar), mn.FadeOut(worth), mn.FadeOut(note), mn.FadeOut(s.plain), mn.FadeOut(s.drops),
                  mn.FadeOut(s.halo), mn.FadeOut(s.thumbs), mn.LaggedStart(*[mn.FadeIn(card) for card in cards], lag_ratio=.25), seconds=2.4)
        verdict = self._eq([(self.number(knee_step["payback"]) + " a ≤ " + self.quantity(s.life, "a") + self.tr(" lifetime at ", " Lebensdauer bei ") + self.quantity(s.tariff, "CHF/kWh", 2), GOOD)], SIDE_LEFT, ROWS[1], size=16, width=4.0)
        self.step("decision_2", *[mn.FadeOut(mn.VGroup(*card[:2])) for card in cards], mn.FadeOut(cards[0][2:]), mn.FadeOut(cards[2][2:]), mn.FadeOut(cards[1][3]),
                  self.reveal(s.room, s.bars, s.legend, s.cost), mn.FadeIn(s.sun), self.set_design(s.focus, knee["design"], 1.4), mn.FadeIn(verdict), seconds=2.5)
