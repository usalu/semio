"""🧪 Learn main effects, interaction and curvature from a planned two-factor experiment on the seminar room's summer cooling."""

import importlib
import math
import numpy as np
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_mn = _visuals._engine
_art = _visuals._art
_room = _visuals._room
_model = _visuals._model
_text = _visuals._text
_fonts = _visuals._fonts
INK, MUTED, DESIGN, DATA, GOOD, WARN = _visuals.INK, _visuals.MUTED, _visuals.DESIGN, _visuals.DATA, _visuals.GOOD, _visuals.WARN
COOL, OPEN, SHADED, ALERT, MIXED = _art.P_BLUE, _art.P_ORANGE, _art.P_GREEN, _art.P_RED, _art.PASTEL_PURPLE
UP, DOWN, LEFT, RIGHT = _mn.UP, _mn.DOWN, _mn.LEFT, _mn.RIGHT

FACTORIAL, FRACTION = _model.factorial(), _model.fractional()
RUNS = FACTORIAL["runs"]
LOW_WWR, HIGH_WWR = FACTORIAL["factors"][0]["levels"]
LOW_D, HIGH_D = FACTORIAL["factors"][1]["levels"]
YMAX = 200 * math.ceil(max(run["response"] for run in RUNS) / 200)
STUDY = _model.load_study()["parameters"]
EER = STUDY["systems"]["eer"]["value"]
DIRECT = STUDY["climate"]["direct_share_summer"]["value"]
FACADE_WIDTH, STEP, CUT, REACH = STUDY["room"]["width"]["value"], .3, 3.2, 1.5
RAY_COUNT = round(RUNS[2]["window_area"] / FACADE_WIDTH / STEP)
RAY_SUN = RUNS[2]["solar_summer"] / RAY_COUNT
RAY_BEAM, RAY_SKY = RAY_SUN * DIRECT, RAY_SUN * (1 - DIRECT)
SHOWN = [round(run["response"], 1) for run in RUNS]
SPACE = " "

CARD_X, CARD_Y, CARD_S, CARD_W, CARD_H, CARD_DY = (-5.2, -1.8), (1.28, -1.27), .44, 3.3, 2.5, .1
BAR_HEIGHT = 1.6
BIG_S, BIG_ORIGIN, BIG_BAR, BIG_HEIGHT = .85, (-3.6, -1.85, 0), (.1, -1.85, 0), 3.2
MODEL_S, MODEL_ORIGIN, WF_X, WF_Y, WF_HEIGHT, WF_PITCH, MODEL_X = .6, (-4.2, -1.5, 0), -1.75, -1.5, 2.5, .9, 4.6
PANEL_X = 3.75
AXES_POINT, AXES_WIDTH, AXES_HEIGHT, X_RANGE = (4.2, -.6, 0), 4.4, 2.3, (10, 90, 10)


def _geometry(design):
    """📐 Sun profile angle, sill height, ray heights along the glass and the number of rays the overhang stops."""
    found = _room.result(design)
    sun = _room.sun_state(design["orientation"], "summer")
    sill = _room.HEAD - min(found["window_area"] / FACADE_WIDTH, _room.HEAD - .05)
    heights, index = [], 0
    while _room.HEAD - STEP * (index + .5) > sill + .05:
        heights.append(_room.HEAD - STEP * (index + .5))
        index += 1
    return math.radians(sun["profile"]), sill, heights, min(len(heights), round(len(heights) * found["shading_summer"])), sun


def _box(label, pad=.08):
    """📦 Padded screen box of a label as (left, bottom, right, top)."""
    return (label.get_left()[0] - pad, label.get_bottom()[1] - pad, label.get_right()[0] + pad, label.get_top()[1] + pad)


def _hits(box, line, step=.03):
    """🎯 Whether a polyline passes through a box, sampled every ``step`` units."""
    for start, end in zip(line, line[1:]):
        start, end = np.array(start, dtype=float)[:2], np.array(end, dtype=float)[:2]
        count = max(2, int(np.linalg.norm(end - start) / step))
        for k in range(count + 1):
            x, y = start + (end - start) * k / count
            if box[0] <= x <= box[2] and box[1] <= y <= box[3]:
                return True
    return False


class Beat3_FactorialExperiments(_visuals.ClipScene):
    """🔬 Distinguish experimental learning, fitted assumptions and confirmation."""

    TITLE = {"en":"Design of Experiments", "de":"Statistische Versuchsplanung"}
    NARRATION = [
        ["question", "Sampling asks where to simulate. Experiments start with a question about this room.", "Sampling fragt, wo wir simulieren. Versuchsplanung beginnt mit einer Frage zu diesem Raum."],
        ["question_2", "How do window share and overhang change the summer cooling electricity of our seminar room?", "Wie verändern Fensteranteil und Vordach den sommerlichen Kühlstrom unseres Seminarraums?"],
        ["question_3", "Sun, room and weather stay fixed. Only window share and overhang change.", "Sonne, Raum und Wetter bleiben fest. Nur Fensteranteil und Vordach ändern sich."],
        ["factors", "Factor A is the window-to-wall ratio (WWR): 30 % or 70 %.", "Faktor A ist der Fensterflächenanteil (WWR): 30 % oder 70 %."],
        ["factors_2", "Factor B is the overhang depth: 0 m (no overhang) or 1 m.", "Faktor B ist die Vordachtiefe: 0\u00a0m (kein Vordach) oder 1\u00a0m."],
        ["factors_3", "Code every low level as minus one and every high level as plus one.", "Kodiere jede niedrige Stufe als minus eins und jede hohe als plus eins."],
        ["plan", "A full two-by-two plan simulates all four combinations of A and B.", "Ein vollständiger Zwei-mal-zwei-Plan simuliert alle vier Kombinationen von A und B."],
        ["plan_2", "Each corner is one summer room: along A the window share grows, along B the overhang.", "Jede Ecke ist ein Sommerraum: Entlang A wächst der Fensteranteil, entlang B das Vordach."],
        ["response", "The summer sun enters all four rooms. Every ray carries the same amount of solar energy.", "Die Sommersonne scheint in alle vier Räume. Jeder Strahl trägt gleich viel Sonnenenergie."],
        ["response_2", "The model returns each room's cooling electricity in kWh/a. Lower is better.", "Das Modell liefert den Kühlstrom jedes Raums in kWh/a. Kleiner ist besser."],
        ["response_3", "A simplified annual teaching model, not a measurement.", "Ein vereinfachtes Jahresmodell, keine Messung."],
        ["plot", "We plot cooling electricity against window share, with one line per overhang depth.", "Wir tragen den Kühlstrom über dem Fensteranteil auf, eine Linie je Vordachtiefe."],
        ["plot_2", "Each point is one room: orange without overhang, green with 1 m.", "Jeder Punkt ist ein Raum: orange ohne Vordach, grün mit 1\u00a0m."],
        ["saving_30", "At 30 % the window catches few rays, so the overhang saves little cooling electricity.", "Bei 30 % fängt das Fenster wenige Strahlen, daher spart das Vordach wenig Kühlstrom."],
        ["saving_70", "At 70 % the window catches many more rays, so the same overhang saves much more.", "Bei 70 % fängt das Fenster viel mehr Strahlen, daher spart dasselbe Vordach deutlich mehr."],
        ["interaction", "The savings differ, so the lines are not parallel: that is an interaction.", "Die Einsparungen unterscheiden sich, die Linien sind nicht parallel: eine Wechselwirkung."],
        ["interaction_2", "The effect of the overhang depends on the window share.", "Die Wirkung des Vordachs hängt vom Fensteranteil ab."],
        ["main", "Main effect of A: the mean at 70 % minus the mean at 30 % window share.", "Haupteffekt A: Mittelwert bei 70 % minus Mittelwert bei 30 % Fensteranteil."],
        ["main_2", "Main effect of B: the mean with overhang minus the mean without.", "Haupteffekt B: Mittelwert mit Vordach minus Mittelwert ohne Vordach."],
        ["difference", "The interaction effect is half the difference of the two overhang effects.", "Der Wechselwirkungseffekt ist die halbe Differenz der beiden Vordach-Effekte."],
        ["difference_2", "Half that difference is the interaction: a bigger window share makes the overhang save more.", "Halbe Differenz ist die Wechselwirkung: Je größer der Fensteranteil, desto mehr spart das Vordach."],
        ["model", "Four simulations fit a bilinear model: a constant, two slopes and a product term, A times B.", "Vier Simulationen ergeben ein bilineares Modell: Konstante, zwei Steigungen und ein Produktterm A·B."],
        ["model_2", "Setting A and B to zero predicts the plan's middle: the mean of all simulations.", "A und B gleich null sagen die Planmitte voraus: den Mittelwert aller Simulationen."],
        ["model_3", "Each coefficient is half an effect, in kWh/a per coded step.", "Jeder Koeffizient ist ein halber Effekt, in kWh/a je kodiertem Schritt."],
        ["curvature", "Two levels only draw straight lines. The true cooling electricity may bend between them.", "Zwei Stufen zeichnen nur Geraden. Der wahre Kühlstrom kann dazwischen gekrümmt verlaufen."],
        ["confirm", "So we add a confirmation simulation at the middle: 50 % window share, 0.5 m overhang.", "Darum eine Bestätigungssimulation in der Mitte: Fensteranteil 50 %, Vordach 0,5 m."],
        ["confirm_2", "The model predicts more cooling electricity than the simulation returns.", "Das Modell sagt mehr Kühlstrom voraus, als die Simulation liefert."],
        ["confirm_3", "The first 0.5 m of overhang already delivers most of the saving: the curve bends.", "Schon die ersten 0,5\u00a0m Vordach bringen den Großteil der Einsparung: Die Kurve krümmt sich."],
        ["repeat", "Running the same simulation again gives exactly the same value: no estimate of noise.", "Dieselbe Simulation liefert erneut genau denselben Wert: keine Schätzung des Rauschens."],
        ["repeat_2", "Measured rooms scatter: physical experiments repeat measurements to estimate variability.", "Gemessene Räume streuen: Reale Versuche wiederholen Messungen, um die Streuung zu schätzen."],
        ["purpose", "Plan simulations to answer the question, estimate effects and confirm.", "Plane Simulationen, um die Frage zu beantworten, Effekte zu schätzen und zu bestätigen."],
        ["purpose_2", "Add insulation as factor C: a full plan needs eight simulations, a fractional plan only four.", "Dämmstärke als Faktor C: Der volle Plan braucht acht Simulationen, ein Teilplan nur vier."],
        ["purpose_3", "Here C moves exactly with the interaction A×B: the two effects are confounded.", "Hier ändert sich C genau wie die Wechselwirkung A·B: beide Effekte sind vermengt."],
        ["purpose_4", "For our room, insulation as factor C saves four simulations, but its effect hides in A×B.", "Für unseren Raum spart Dämmung als Faktor C vier Simulationen, aber ihr Effekt steckt in A·B."],
        ["answer", "In our room a 1 m overhang saves 279 kWh/a at a 70 % window share, but only 122 kWh/a at 30 %.", "In unserem Raum spart 1\u00a0m Vordach bei 70 % Fensteranteil 279\u00a0kWh/a Kühlstrom, bei 30 % nur 122\u00a0kWh/a."],
    ]

    def construct(self):
        """▶️ Animate a complete experimental question through confirmation."""
        self.present("experiments")

    def clear_stage(self):
        """🧹 Fade everything except the caption and model source tag."""
        keep = {id(self._caption), id(self.source), id(getattr(self, "heading", None))}
        content = [item for item in self.mobjects if id(item) not in keep]
        if content:
            self.play(*[_mn.FadeOut(item) for item in content], run_time=.5)

    def _head(self, en, de):
        """📑 Swap the standalone instructional heading chip of the current act."""
        new = _fonts.section_heading(self.tr(en, de))
        old, self.heading = getattr(self, "heading", None), new
        return [_mn.FadeOut(old), _mn.FadeIn(new)] if old is not None else [_mn.FadeIn(new)]

    def _t(self, value, point, size=18, color=INK, width=6.0):
        """🔤 Place one static label, shrunk to the given width."""
        return _text(value, point, size=size, color=color, width=width)

    def _at(self, value, point, size=16, color=INK, width=3.0):
        """⬅️ Place one static label with its left edge on a point."""
        return _room.place(self._t(value, (0, 0, 0), size, color, width), point, "left")

    def _chip(self, value, point, size=17, color=INK, width=6.0):
        """🔖 Frame a static label in the shared chip chrome."""
        return _art.window_chip(self._t(value, (0, 0, 0), size, color, width), color=color).move_to(point)

    def _sign(self, value, digits=0):
        """➕ Format a number with an explicit plus sign."""
        return ("+" if round(value, digits) > 0 else "") + self.number(value, digits)

    def _coded(self, value):
        """🔢 Format a coded level as −1, 0 or +1."""
        return "+1" if value > 0 else ("−1" if value < 0 else "0")

    def _cooling(self, design):
        """❄️ Model cooling electricity of one design in kWh/a."""
        return _room.result(design)["cooling"]

    def _wwr(self, value):
        """🪟 Window-to-wall ratio label in percent."""
        return self.quantity(value * 100, "%").replace(SPACE, " ")

    def _depth(self, value):
        """📏 Overhang depth label in metres."""
        return self.quantity(value, "m", 1).replace(SPACE, " ")

    def _kwh(self, value, digits=1, sign=False):
        """⚡ Energy label in kWh/a."""
        return ("+" if sign and round(value, digits) > 0 else "") + self.quantity(value, "kWh/a", digits).replace(SPACE, " ")

    def _rect(self, point, width, height, color, *, fill=0.0, stroke=2.4, dashed=False):
        """▭ Rectangle centred on a point."""
        rectangle = _mn.Rectangle(width=width, height=height, color=color, stroke_width=stroke, fill_color=color, fill_opacity=fill).move_to(point)
        return _mn.DashedVMobject(rectangle, num_dashes=28) if dashed else rectangle

    def _origin(self, card_centre):
        """📍 Screen position of the local origin of the section inside one card."""
        return np.array([card_centre[0] - .71, card_centre[1] - .8, 0.0])

    def _section(self, design, point, s, *, labels=False, ghost=None, person=True, reach=REACH, demand=False, daylight=False):
        """🏠 Cropped summer section of one design as named layers: shell, rays, light, sun, cooling and optional labels."""
        mn, german = _mn, self.language() == "de"
        design = {**_room.baseline(), **design}
        found = _room.result(design)
        alpha, sill, heights, blocked, sun = _geometry(design)
        t, d = design["insulation"], max(design["overhang"], 0.0)
        x_o = -(_room.MASONRY + t)
        x_tip = x_o - d
        ghost = ghost() if callable(ghost) else ghost
        demand = demand() if callable(demand) else demand
        origin = np.array(point, dtype=float)
        P = lambda x, y: origin + s * np.array([x, y, 0.0])
        rect = lambda x0, y0, x1, y1: mn.Rectangle(width=max(s * (x1 - x0), 1e-4), height=max(s * (y1 - y0), 1e-4)).move_to(P((x0 + x1) / 2, (y0 + y1) / 2))
        solid = lambda *box: _art.section_material(rect(*box))
        top, slab = _room.HEIGHT, _room.SLAB
        ground = mn.VGroup(_room.strokes([(P(x_o - 1.8, 0), P(x_o, 0))], _room.GRAY, 2), _room.strokes([(P(x_o - .35 * k, 0), P(x_o - .35 * k - .25, -.25)) for k in range(1, 6)], _room.GRAY, 1, .6))
        zone = mn.VGroup()
        if daylight:
            depth = min(CUT, _room.daylit_depth(found["daylight"]))
            if depth > .05:
                zone.add(*[rect(depth * k / 12, 0, depth * (k + 1) / 12, top).set_fill(_room.DAYLIGHT, opacity=.4 * (1 - (k + .5) / 12) ** 1.2).set_stroke(width=0) for k in range(12)],
                         _room.strokes([(P(0, _room.WORK), P(depth, _room.WORK)), (P(depth, _room.WORK - .12), P(depth, _room.WORK + .12))], _room.DAYLIGHT, 3, .9))
        insulation = mn.VGroup()
        for lo, hi in ((0, sill), (_room.HEAD, top)):
            insulation.add(rect(x_o, lo, -_room.MASONRY, hi).set_fill(_room.INSULATION, opacity=.55).set_stroke(_room.INSULATION, width=1.2),
                           _room._hatch((x_o, lo, -_room.MASONRY, hi), .16, P, _room.INSULATION, 1.1, .9))
        glass = rect(x_o, sill, x_o + .2, _room.HEAD).set_fill(_room.GLASS, opacity=.85).set_stroke(INK, width=1.6)
        cut = _room.dashed(P(CUT, 0), P(CUT, top), color=_room.GRAY, stroke_width=1.4)
        shell = mn.VGroup(zone, ground, insulation, solid(-_room.MASONRY, 0, 0, sill), solid(-_room.MASONRY, _room.HEAD, 0, top), solid(x_o, -slab, CUT, 0), solid(x_o, top, CUT, top + slab), glass, cut)
        overhang = mn.VGroup()
        if d > .01:
            overhang.add(_art.section_material(rect(x_tip, top, x_o, top + slab)).set_fill(OPEN, opacity=.95).set_stroke(INK, width=1.4, opacity=1))
        if ghost:
            overhang.add(mn.DashedVMobject(rect(x_o - ghost, top, x_o, top + slab).set_stroke(_room.GRAY, width=1.8).set_fill(opacity=0), num_dashes=24))
        direction = np.array([math.cos(alpha), -math.sin(alpha)])
        hits = [np.array([x_o, y]) for y in heights]
        sun_at = hits[0] - direction * reach
        start_of = lambda hit: hit - direction * float((hit - sun_at) @ direction)
        boxes = [(x_o, -slab, 0, sill), (x_o, _room.HEAD, 0, top), (x_o, top, CUT + .5, top + slab), (x_o, -slab, CUT + .5, 0)]
        strength = found["sun_summer"]
        opacity = .55 + .4 * strength
        rays, blocked_rays, marks, skies, passing = mn.VGroup(), mn.VGroup(), mn.VGroup(), mn.VGroup(), []
        for index, hit in enumerate(hits):
            if index < blocked:
                topside = x_o - (top + slab - hit[1]) / math.tan(alpha)
                end = np.array([topside, top + slab]) if topside >= x_tip else np.array([x_tip, min(top + slab, max(top, hit[1] + d * math.tan(alpha)))])
                line = mn.VMobject(stroke_color=_room.SUN, stroke_width=1.2 + 2.2 * s, stroke_opacity=.5, fill_opacity=0)
                line.set_points_as_corners([P(*start_of(hit)), P(*end)])
                cross = _art.cross_mark(ALERT, .02 + .09 * s).move_to(P(*end))
                blocked_rays.add(line)
                marks.add(cross)
                rays.add(line)
                landing = hit + direction * _room._hit(hit - direction * 1e-6, direction, boxes)[0]
                sky = mn.VMobject(stroke_color=_room.DAYLIGHT, stroke_width=.8 + 1.2 * s, stroke_opacity=.75, fill_opacity=0)
                sky.set_points_as_corners([P(*hit), P(*landing)])
                skies.add(sky)
            else:
                landing = hit + direction * _room._hit(hit - direction * 1e-6, direction, boxes)[0]
                line = mn.VMobject(stroke_color=_room.SUN, stroke_width=1.4 + 3.2 * s, stroke_opacity=opacity, fill_opacity=0)
                line.set_points_as_corners([P(*start_of(hit)), P(*hit), P(*landing)])
                rays.add(line)
                passing.append((hit, landing))
                if ghost and x_o - ghost <= x_o - (top + slab - hit[1]) / math.tan(alpha) <= x_o:
                    marks.add(_art.cross_mark(_room.GRAY, .02 + .07 * s).move_to(P(x_o - (top + slab - hit[1]) / math.tan(alpha), top + slab)))
        light = mn.VGroup(skies)
        floor_hits = [landing[0] for _, landing in passing if abs(landing[1]) < 1e-6]
        sill_hits = [landing[0] for _, landing in passing if abs(landing[1] - sill) < 1e-6 and landing[0] < 0]
        if len(passing) > 1:
            first, last = passing[0], passing[-1]
            light.add(mn.Polygon(P(*first[0]), P(*first[1]), P(*last[1]), P(*last[0]), stroke_width=0, fill_color=_room.SUN, fill_opacity=.1 + .06 * strength))
        if floor_hits:
            low, high = min(floor_hits) - STEP / math.tan(alpha) / 2, max(floor_hits) + STEP / math.tan(alpha) / 2
            light.add(rect(low, 0, high, .3).set_fill(_room.SUN, opacity=.22).set_stroke(width=0), rect(low, 0, high, .09).set_fill(_room.SUN, opacity=.95).set_stroke(width=0))
            count = max(2, min(5, round((high - low) / .16)))
            for index in range(count):
                x = low + (high - low) * (index + .5) / count
                wave = mn.VMobject(stroke_color=ALERT, stroke_width=2.2, stroke_opacity=.75, fill_opacity=0)
                wave.set_points_smoothly([P(x + .06 * math.sin(2.4 * j + index), .16 + .07 * j) for j in range(11)])
                light.add(wave)
        if sill_hits:
            light.add(rect(x_o, sill, 0, sill + .08).set_fill(_room.SUN, opacity=.9).set_stroke(width=0))
            for index in range(2):
                x = x_o + (-x_o) * (index + .5) / 2
                wave = mn.VMobject(stroke_color=ALERT, stroke_width=2.2, stroke_opacity=.75, fill_opacity=0)
                wave.set_points_smoothly([P(x + .05 * math.sin(2.4 * j + index), sill + .14 + .07 * j) for j in range(8)])
                light.add(wave)
        sun_group = _room.sun_marker(P(*sun_at), s * .7 / 1.4, 1.0)
        extras = mn.VGroup()
        share = found["cooling"] / YMAX
        length = .3 + .9 * min(1.0, share)
        extras.add(_room._cooler(P(2.7, 2.8), s, COOL))
        for x in (2.35, 2.7, 3.05):
            extras.add(mn.Arrow(P(x, 2.58), P(x, 2.58 - length), buff=0, color=COOL, stroke_width=1.5 + 2 * share, max_tip_length_to_length_ratio=.4, tip_length=.1 * s / .5, max_stroke_width_to_length_ratio=12))
        if person:
            figure = _art.person_glyph(P(1.5, .875), _art.P_ORANGE)
            figure.scale_to_fit_height(1.75 * s).move_to(P(1.5, .875))
            extras.add(figure)
        low_a, high_a = _room.autonomy_range()
        brightness = 1 - min(1.0, max(0.0, (found["autonomy"] - low_a) / max(high_a - low_a, 1e-9)))
        for x in (.55, 1.6):
            lamp = _art.lamp_glyph(P(x, top), drop=.1 * s / .5, scale=s / .6)
            halo = mn.Circle(radius=.28 * s / .6 * (.4 + .6 * brightness), stroke_width=0, fill_color=_room.LIGHT, fill_opacity=.35 * brightness)
            bulb = _room.clone(lamp["bulb"]).set_fill(opacity=.08 + .92 * brightness)
            extras.add(halo.move_to(bulb.get_center()), _room.clone(lamp["cable"]), _room.clone(lamp["shade"]), bulb)
        notes = mn.VGroup()
        if labels:
            size = 15
            column = x_o - 1.1
            level = min(_room.HEAD, (sill + _room.HEAD) / 2)
            chips = [(_room.chip(["A ·", self.tr("window", "Fenster"), *_room.parts(design["wwr"] * 100, "%", 0, german)], size=size - 1, color=_room.GLASS), level, x_o - .35),
                     (_room.chip([self.tr("insulation", "Dämmung"), *_room.parts(t * 100, "cm", 0, german)], size=size - 1, color=_room.INSULATION), -.62, x_o)]
            over = _room.chip(["B ·", self.tr("overhang", "Vordach"), *_room.parts(d, "m", 1, german)], size=size - 1, color=OPEN)
            _room.place(over, P(.15, top + slab + .3), "left")
            notes.add(over, mn.Line(over.get_left() + np.array([-.02, 0, 0]), P(x_tip, top + slab + .35), color=OPEN, stroke_width=1.4, stroke_opacity=.8))
            lowest = math.inf
            for item, y, end in chips:
                drawn = min(y, lowest - item.height / s - .1 / s) if lowest < math.inf else y
                _room.place(item, P(column, drawn), "right")
                notes.add(item, _room._leader(item, P(end, y), item[0].get_stroke_color()))
                lowest = drawn - item.height / s / 2
            notes.add(_room._dimension(P(x_o - .35, sill), P(x_o - .35, _room.HEAD), color=_room.GLASS), _room._dimension(P(x_o, -.62), P(-_room.MASONRY, -.62), color=_room.INSULATION))
            if d > .05:
                notes.add(_room._dimension(P(x_tip, top + slab + .35), P(x_o, top + slab + .35), color=OPEN))
            notes.add(_room.place(_room.text([self.tr("Summer sun", "Sommersonne"), *_room.parts(sun["profile"], "°", 0, german)], (0, 0, 0), size=size - 1, color=_room.SUN), P(sun_at[0] + .3, sun_at[1] + .62), "center"))
        if demand:
            size = _room.label_size(s) - 1
            first = _room.text([self.tr("Cooling load", "Kühllast"), *_room.parts(found["cooling_demand"], "kWh/a", 1, german)], (0, 0, 0), size=size, color=COOL)
            second = _room.text(["÷ EER", *_room.parts(EER, "", 1, german), "=", *_room.parts(found["cooling"], "kWh/a", 1, german)], (0, 0, 0), size=size, color=COOL)
            stack = mn.VGroup(first, second).arrange(DOWN, buff=.06, aligned_edge=RIGHT)
            notes.add(_room.place(stack, P(3.2, top + slab + 1.1), "right"))
        layers = {"shell": shell, "overhang": overhang, "light": light, "rays": rays, "marks": marks, "sun": mn.VGroup(sun_group), "extras": extras, "notes": notes}
        return layers, {"blocked": mn.VGroup(blocked_rays, marks), "glass": glass, "slab": overhang, "rays": rays, "sun": sun_group, "sill": sill}

    def _merge(self, layers, skip=()):
        """🧱 Stack named layers in drawing order."""
        return _mn.VGroup(*[layers[key] for key in ("shell", "overhang", "light", "rays", "marks", "sun", "extras", "notes") if key not in skip])

    def _live_section(self, trackers, point, s, **options):
        """🔴 Section that follows the shared trackers."""
        return self._live(lambda: self._merge(self._section(_room.values(trackers), point, s, **options)[0]))

    def _bar(self, base, value_fn, grow, *, height=BAR_HEIGHT, width=.3, color=COOL, size=14, digits=1, unit=True):
        """📊 Live cooling bar from a base point with its number underneath; the number stays hidden while the run is not evaluated."""
        scale, german, base = height / YMAX, self.language() == "de", np.array(base, dtype=float)

        def build():
            level = grow.get_value()
            if level <= .02:
                return _mn.VGroup(_mn.Dot(base, radius=1e-4, fill_opacity=0, stroke_width=0))
            value = value_fn() * level
            span = max(value * scale, 1e-3)
            group = _mn.VGroup(_mn.Rectangle(width=width, height=span, color=color, stroke_width=1.2, fill_color=color, fill_opacity=.78).move_to(base + UP * span / 2))
            if level > .02:
                group.add(_room.text(_room.parts(value, "kWh/a" if unit else "", digits, german), base + DOWN * .22, size=size, color=INK))
            return group

        return _mn.always_redraw(build)

    def _ghost_bar(self, base, value, *, height=BAR_HEIGHT, width=.3, color=MUTED):
        """👻 Dashed outline of a reference bar value."""
        span = value * height / YMAX
        return self._rect(np.array(base) + UP * span / 2, width, span, color, stroke=1.8, dashed=True)

    def _formula_rows(self, rows, point, *, size=17, width=6.2, align="center", trackers=None):
        """🧮 Substituted formula in the shared energy style: words above their numbers, redrawn from the design trackers (a fixed dummy when the numbers are constants)."""
        return self.formula(trackers or self.dummy, rows, point, size=size, width=width, align=align)

    def _lines(self, *lines, top, gap=.31, x=PANEL_X, width=6.6):
        """📝 Centred coloured text lines, each ``(text, colour, size)``, shrunk to the panel width."""
        group = _mn.VGroup()
        for index, (value, color, size) in enumerate(lines):
            group.add(self._t(value, (x, top - gap * index, 0), size, color, width))
        return group

    def _legend(self):
        """💡 One ray equals one fixed amount of sun heat: the legend that links every drawn ray to kWh/a."""
        ray = _mn.Arrow((.55, 2.98, 0), (1.05, 2.98 - .26, 0), buff=0, color=_room.SUN, stroke_width=3, max_tip_length_to_length_ratio=.5, tip_length=.12)
        beam, sky = round(RAY_BEAM), round(RAY_SUN) - round(RAY_BEAM)
        text = self._at(f"1 {self.tr('ray', 'Strahl')} = 0,3 m {self.tr('of glass', 'Glas')} = {self._kwh(RAY_SUN, 0)} {self.tr('sun heat', 'Sonnenwärme')} ({beam} {self.tr('direct', 'direkt')} + {sky} {self.tr('diffuse', 'diffus')})".replace("0,3", self.number(.3, 1)),
                        (1.2, 2.84, 0), 14, _room.SUN, 5.65)
        return _mn.VGroup(ray, text)

    def _slider(self, y, tracker, title, color, fmt, low, high):
        """🎚️ Level slider whose marker follows a design tracker, with ticks and coded labels at the levels."""
        centre, half, x = (low + high) / 2, (high - low) / 2, PANEL_X
        position = lambda value: x + (value - centre) / half * 1.9
        static = _mn.VGroup(_mn.Line((x - 2.45, y, 0), (x + 2.45, y, 0), color=MUTED, stroke_width=2), self._t(title, (x, y + .75, 0), 20, color, 5.8))
        coded = _mn.VGroup()
        for level in (-1, 0, 1):
            value = centre + level * half
            static.add(_mn.Line((position(value), y - .1, 0), (position(value), y + .1, 0), color=MUTED, stroke_width=2),
                       self._t(fmt(value), (position(value), y - .38, 0), 16, color if level else MUTED, 1.3))
            coded.add(self._t(self._coded(level), (position(value), y + .42, 0), 19, DATA, 1.0))
        marker = _mn.always_redraw(lambda: _mn.Triangle(fill_color=color, fill_opacity=1, stroke_width=0).scale(.13).rotate(math.pi).move_to((position(tracker.get_value()), y + .2, 0)))
        return {"static": static, "coded": coded, "marker": marker}

    def _table(self, headers, rows, left, top, widths, *, height=.46, size=17, colors=None, skip=()):
        """▦ Text table without TeX; returns the group and a ``(row, column)`` cell map (row −1 is the header); ``skip`` columns stay out of the group."""
        cells = {}
        edges = [left + sum(widths[:index]) for index in range(len(widths) + 1)]
        for column, head in enumerate(headers):
            cells[(-1, column)] = self._t(head, ((edges[column] + edges[column + 1]) / 2, top - height / 2, 0), size - 3, MUTED, widths[column] - .08)
        for row, values in enumerate(rows):
            for column, value in enumerate(values):
                cells[(row, column)] = self._t(value, ((edges[column] + edges[column + 1]) / 2, top - height * (row + 1.5), 0), size, (colors or {}).get(column, INK), widths[column] - .1)
        group = _mn.VGroup(*[cell for (row, column), cell in cells.items() if column not in skip], _mn.Line((left, top - height - .05, 0), (edges[-1], top - height - .05, 0), color=MUTED, stroke_width=1.4))
        return group, cells, edges

    def _headline(self, design, third=False):
        """🏷️ Physical name of one design: window size, overhang and optionally insulation."""
        depth = design["overhang"]
        over = self.tr("no overhang", "kein Vordach") if depth <= 0 else self._depth(depth) + " " + self.tr("overhang", "Vordach")
        if third:
            return f"{self._wwr(design['wwr'])} · {self._depth(depth)} · {self.quantity(design['insulation'] * 100, 'cm').replace(SPACE, ' ')} {self.tr('insulation', 'Dämmung')}"
        return f"WWR {self._wwr(design['wwr'])} · {over}"

    def _badge(self, point, number, *, color=INK, shape="circle", fill=False):
        """🔢 Run number in a small circle or square."""
        outline = (_mn.Square(side_length=.25) if shape == "square" else _mn.Circle(radius=.14)).set_stroke(color, width=2).set_fill(color, opacity=1 if fill else 0).move_to(point)
        return _mn.VGroup(outline, self._t(str(number), point, 14, _art.P_DEEP_DARK if fill else color, .2))

    def _card(self, run, number, *, third=False):
        """🏠 One run as a static summer room with header, run badge and a cooling bar that starts empty."""
        column, row = int(run["coded"][0] > 0), int(run["coded"][1] < 0)
        centre = np.array([CARD_X[column], CARD_Y[row], 0.0])
        design = run["design"]
        layers, refs = self._section(design, self._origin(centre), CARD_S)
        header = _mn.VGroup(self._at(self._headline(design, third), (centre[0] - 1.32, centre[1] + 1.2, 0), 15, INK, 2.7))
        badge = self._badge((centre[0] - 1.52, centre[1] + 1.2, 0), number)
        card = {"run": run, "centre": centre, "base": centre + np.array([1.0, -.8, 0]), "grow": _mn.ValueTracker(0), "layers": layers, "refs": refs, "badge": badge, "header": header}
        card["bar"] = self._bar(card["base"], lambda run=run: run["response"], card["grow"], digits=1)
        card["group"] = _mn.VGroup(self._merge(layers, skip=("rays", "marks", "sun", "light")), header, badge)
        if not third:
            card["sun_heat"] = _room.place(self._t(f"{self.tr('sun heat', 'Sonnenwärme')} {self._kwh(run['solar_summer'], 0)}", (0, 0, 0), 14, _room.SUN, 2.4), (centre[0] + 1.5, centre[1] + .9, 0), "right")
        return card

    def _shine(self, card):
        """🔦 Sun appears, rays run to the glass or the overhang, then the light patch and the heat waves."""
        layers = card["layers"]
        return _mn.Succession(_mn.FadeIn(layers["sun"], run_time=.3), _mn.LaggedStart(*[_mn.Create(ray, rate_func=_mn.linear) for ray in layers["rays"]], lag_ratio=.18, run_time=1.2),
                              _mn.AnimationGroup(_mn.FadeIn(layers["light"]), _mn.FadeIn(layers["marks"]), run_time=.5))

    def _veil(self, card):
        """🌑 Dimming veil over one card."""
        x, y = card["centre"][:2]
        return _mn.Rectangle(width=CARD_W, height=CARD_H, stroke_width=0, fill_color=_art.P_DEEP_DARK, fill_opacity=.66).move_to((x, y + CARD_DY, 0)).set_z_index(5)

    def _frame(self, card, color, stroke=3):
        """🔲 Colour frame around one card."""
        x, y = card["centre"][:2]
        return self._rect((x, y + CARD_DY, 0), CARD_W, CARD_H, color, stroke=stroke).set_z_index(6)

    def _everything(self, cards):
        """🗂️ Every drawn part of some cards, whatever state they are in now."""
        return _mn.VGroup(*[_mn.VGroup(card["group"], card["layers"]["rays"], card["layers"]["marks"], card["layers"]["sun"], card["layers"]["light"], card["badge"], card["bar"], card["sun_heat"]) for card in cards])

    def _signs(self, cards, signs, color):
        """➕ Contrast sign of each card as a round plus or minus badge."""
        group = _mn.VGroup()
        for card, sign in zip(cards, signs):
            point = np.array(card["centre"]) + np.array([1.4, .45, 0])
            badge = _mn.Circle(radius=.17, color=color, stroke_width=2.4, fill_color=_art.P_DEEP_DARK, fill_opacity=1).move_to(point)
            bar = _mn.Line(point + LEFT * .09, point + RIGHT * .09, color=color, stroke_width=3)
            group.add(badge, bar, *([_mn.Line(point + DOWN * .09, point + UP * .09, color=color, stroke_width=3)] if sign > 0 else []))
        return group.set_z_index(7)

    def _chart(self, *, x_range=X_RANGE, x_ticks=(30, 50, 70), x_label=None, y_max=YMAX, y_step=200, x_digits=0):
        """📈 Axes of cooling electricity against a design factor; the spines become label obstacles."""
        self.axes_, labels = self.plot_axes(x_range=x_range, y_range=(0, y_max, y_step), x_label=x_label or self.tr("Window-to-wall ratio WWR (%)", "Fensteranteil WWR (%)"),
                                            y_label=self.tr("Cooling electricity (kWh/a)", "Kühlstrom (kWh/a)"), point=AXES_POINT, width=AXES_WIDTH,
                                            height=AXES_HEIGHT, x_digits=x_digits, x_ticks=list(x_ticks), y_ticks=list(range(0, y_max + 1, y_step)), corner=True)
        axes = self.axes_
        self.lines_ = [[axes.c2p(x_range[0], 0), axes.c2p(x_range[0], y_max)], [axes.c2p(x_range[0], 0), axes.c2p(x_range[1], 0)]]
        return _mn.VGroup(axes, labels)

    def _at_plot(self, wwr, value):
        """📍 Plot position of a window ratio (in percent) and a cooling value."""
        return self.axes_.c2p(wwr, value)

    def _spot(self, run):
        """📍 Plot position of one run."""
        return self._at_plot(run["design"]["wwr"] * 100, run["response"])

    def _mark(self, run):
        """🔴 Plot marker of one run: circle without overhang, square with it."""
        marker = _mn.Square(side_length=.17, color=SHADED, fill_color=SHADED, fill_opacity=1) if run["design"]["overhang"] > 0 else _mn.Dot(radius=.09, color=OPEN)
        return marker.move_to(self._spot(run))

    def _near(self, label, anchor, order=("up", "down", "right", "left"), gap=.27, avoid=()):
        """🧭 Put a label beside an anchor at the first side where it touches no plotted line, spine or frame."""
        anchor = np.array(anchor, dtype=float)
        sides = {"up": (0, gap, "center"), "down": (0, -gap, "center"), "right": (gap - .07, 0, "left"), "left": (-(gap - .07), 0, "right"),
                 "upright": (.14, gap - .07, "left"), "downright": (.14, -(gap - .07), "left"), "upleft": (-.14, gap - .07, "right"), "downleft": (-.14, -(gap - .07), "right")}
        for side in order:
            dx, dy, edge = sides[side]
            _room.place(label, anchor + np.array([dx, dy, 0]), edge)
            if not any(_hits(_box(label), line) for line in (*self.lines_, *avoid)):
                return label
        _room.place(label, anchor + np.array([sides[order[0]][0], sides[order[0]][1], 0]), sides[order[0]][2])
        return label

    def _tag(self, point, value, color, size=14, digits=1):
        """🏷️ Number-only label."""
        return _room.text(_room.parts(value, "", digits, self.language() == "de"), point, size=size, color=color)

    def present_experiments(self):
        """🎞️ Present the question, plan, plot, effects, model, curvature, replication and fractional design."""
        self.dummy = self.study_trackers()
        self.source = self._t(self.tr("Simplified annual teaching model · Zurich-like climate · south façade", "Vereinfachtes Jahresmodell · Zürich-ähnliches Klima · Südfassade"), (0, 0, 0), 15, INK, 7.0)
        _room.place(self.source, (6.88, 3.62, 0), "right").set_opacity(.85)
        self.add(self.source)
        self._question()
        self._factors()
        self._plan()
        self._response()
        self._plot()
        self._savings()
        self._effects()
        self._model_fit()
        self._curvature()
        self._replicates()
        self._fraction()
        self._answer()

    def _big_room(self, trackers, grow=None, *, ghost=None, bar=True, demand=False):
        """🏠 The large live summer room with its cooling bar."""
        room = self._live_section(trackers, BIG_ORIGIN, BIG_S, labels=True, ghost=ghost, reach=2.4, demand=demand, daylight=True)
        group = _mn.VGroup(room)
        if bar:
            group.add(self._bar(BIG_BAR, lambda: self._cooling(_room.values(trackers)), _mn.ValueTracker(1) if grow is None else grow, height=BIG_HEIGHT, width=.46, size=16),
                      self._t(self.tr("Cooling electricity", "Kühlstrom"), (BIG_BAR[0], BIG_BAR[1] - .55, 0), 14, COOL, 2.1))
        return group

    def _question(self):
        """❓ One summer room, its place on the compass and the question about its two design factors."""
        self.trackers = self.study_trackers({"wwr": .5, "overhang": .5})
        self.big = self._big_room(self.trackers, bar=False)
        compass = self.plan(self.trackers, (PANEL_X, .1, 0), .3, dimensions=True)
        self.step("question", *self._head("Question: two design factors, one response", "Frage: zwei Entwurfsfaktoren, eine Zielgröße"), _mn.FadeIn(self.big), _mn.FadeIn(compass), seconds=2)
        title = self._t(self.tr("Cooling electricity = f(A, B)", "Kühlstrom = f(A, B)"), (PANEL_X, 1.8, 0), 30, DATA, 6.2)
        chips = _mn.VGroup(self._chip(self.tr("A · window share (WWR, %)", "A · Fensteranteil (WWR, %)"), (PANEL_X, .85, 0), 18, DESIGN),
                           self._chip(self.tr("B · overhang depth (m)", "B · Vordachtiefe (m)"), (PANEL_X, .25, 0), 18, OPEN),
                           self._chip(self.tr("Result · cooling electricity (kWh/a)", "Ergebnis · Kühlstrom (kWh/a)"), (PANEL_X, -.35, 0), 18, COOL))
        self.question_card = _mn.VGroup(title, chips)
        sweep = _mn.Succession(*[self.trackers[key].animate.set_value(value) for key, value in (("wwr", HIGH_WWR), ("wwr", LOW_WWR), ("wwr", .5), ("overhang", HIGH_D), ("overhang", LOW_D), ("overhang", .5))])
        self.step("question_2", _mn.FadeOut(compass), _mn.FadeIn(self.question_card), sweep, seconds=8)
        sun = _room.sun_state(180, "summer")
        row = _mn.VGroup()
        room = STUDY["room"]
        size = f"{self.number(room['width']['value'])} × {self.number(room['depth']['value'])} × {self.number(room['height']['value'])} m"
        facade = _mn.VGroup(_mn.Rectangle(width=.5, height=.375, color=INK, stroke_width=2), _mn.Line((-.25, -.1875, 0), (.25, -.1875, 0), color=_room.GLASS, stroke_width=5), _mn.Arrow((0, -.2, 0), (0, -.36, 0), buff=0, color=_room.GLASS, stroke_width=3, tip_length=.08))
        for index, (icon, text) in enumerate(((_room.sun_marker((0, 0, 0), .2, 1.0), self.tr(f"Sun: summer noon, {self.number(sun['profile'])}°", f"Sonne: Sommer-Mittag, {self.number(sun['profile'])}°")),
                                              (facade, self.tr(f"Room: {size}, south façade", f"Raum: {size}, Südfassade")),
                                              (self._cloud(), self.tr("Weather: Zurich-like year", "Wetter: Zürich-ähnliches Jahr")))):
            y = -.95 - .55 * index
            row.add(icon.move_to((1.45, y, 0)), self._at(text, (1.85, y, 0), 17, INK, 5.0))
        self.fixed = row
        self.step("question_3", _mn.FadeIn(self.fixed), seconds=1.5)

    def _cloud(self):
        """☁️ Small weather icon from three circles and a base."""
        parts = [_mn.Circle(radius=r, stroke_width=0, fill_color=MUTED, fill_opacity=1).move_to((x, y, 0)) for x, y, r in ((-.1, .02, .1), (.05, .07, .13), (.2, 0, .09))]
        return _mn.VGroup(*parts, _mn.Rectangle(width=.4, height=.09, stroke_width=0, fill_color=MUTED, fill_opacity=1).move_to((.05, -.04, 0)))

    def _factors(self):
        """🎚️ The two factors as sliders with their coded levels and their physical meaning on the room."""
        german = self.language() == "de"
        a = self._slider(1.45, self.trackers["wwr"], self.tr("A · window share (WWR)", "A · Fensteranteil (WWR)"), DESIGN, self._wwr, LOW_WWR, HIGH_WWR)
        b = self._slider(-.5, self.trackers["overhang"], self.tr("B · overhang depth", "B · Vordachtiefe"), OPEN, self._depth, LOW_D, HIGH_D)
        facade = FACADE_WIDTH * STUDY["room"]["height"]["value"]
        read_a = self._live(lambda: _room.text([self.tr("window", "Fenster"), *_room.parts(self.trackers["wwr"].get_value() * facade, "m²", 1, german), self.tr("of the façade", "der Fassade"),
                                                *_room.parts(facade, "m²", 0, german)], (PANEL_X, .72, 0), size=16, color=DESIGN))
        read_b = self._live(lambda: _room.text([self.tr("overhang shades", "Vordach verschattet"), *_room.parts(_room.result(_room.values(self.trackers))["shading_summer"] * 100, "%", 0, german), self.tr("of the glass", "des Glases")],
                                               (PANEL_X, -1.25, 0), size=16, color=OPEN))
        self.step("factors", *self._head("Factors and levels", "Faktoren und Stufen"), _mn.FadeOut(self.question_card), _mn.FadeOut(self.fixed), _mn.FadeIn(a["static"]), _mn.FadeIn(a["marker"]), _mn.FadeIn(read_a),
                  _mn.Succession(*[self.trackers["wwr"].animate.set_value(v) for v in (HIGH_WWR, LOW_WWR)]), seconds=4)
        self.step("factors_2", _mn.FadeIn(b["static"]), _mn.FadeIn(b["marker"]), _mn.FadeIn(read_b), _mn.Succession(*[self.trackers["overhang"].animate.set_value(v) for v in (HIGH_D, LOW_D)]), seconds=4)
        half_a, half_b = (HIGH_WWR - LOW_WWR) / 2, (HIGH_D - LOW_D) / 2
        centre_a, centre_b = (HIGH_WWR + LOW_WWR) / 2, (HIGH_D + LOW_D) / 2
        coding = _mn.VGroup(self._t(f"A = ({self.tr('window share', 'Fensteranteil')} − {self._wwr(centre_a)}) / {self._wwr(half_a)}", (PANEL_X, -1.85, 0), 17, DESIGN, 5.8),
                            self._t(f"B = ({self.tr('overhang', 'Vordach')} − {self._depth(centre_b)}) / {self._depth(half_b)}", (PANEL_X, -2.22, 0), 17, OPEN, 5.8))
        corners = [self.set_design(self.trackers, {"wwr": run["design"]["wwr"], "overhang": run["design"]["overhang"]}, 1.0) for run in (RUNS[2], RUNS[3], RUNS[1], RUNS[0])]
        self.step("factors_3", _mn.FadeIn(a["coded"]), _mn.FadeIn(b["coded"]), _mn.FadeIn(coding), _mn.Succession(*corners), seconds=6)
        self.sliders = _mn.VGroup(a["static"], a["marker"], b["static"], b["marker"], a["coded"], b["coded"], coding, read_a, read_b)

    def _plan(self):
        """🔲 The four runs as a square of summer rooms and the design matrix."""
        self.clear_stage()
        self.cards = [self._card(run, index + 1) for index, run in enumerate(RUNS)]
        headers = [self.tr("No.", "Nr."), self.tr("A\nwindow", "A\nFenster"), self.tr("B\noverhang", "B\nVordach"), self.tr("WWR\n(%)", "WWR\n(%)"), self.tr("Overhang\n(m)", "Vordach\n(m)"), self.tr("Cooling\n(kWh/a)", "Kühlstrom\n(kWh/a)")]
        rows = [[str(index + 1), self._coded(run["coded"][0]), self._coded(run["coded"][1]), self.number(run["design"]["wwr"] * 100), self.number(run["design"]["overhang"], 1), self.number(SHOWN[index], 1)]
                for index, run in enumerate(RUNS)]
        self.table, self.cells, _ = self._table(headers, rows, .85, 2.5, [.55, .9, 1.0, .85, 1.15, 1.45], height=.5, size=19, colors={1: MUTED, 2: MUTED, 5: COOL}, skip=(5,))
        self.step("plan", *self._head("Full plan: 2² = 4 simulations", "Vollständiger Plan: 2² = 4 Simulationen"), _mn.LaggedStart(*[_mn.FadeIn(self.cards[i]["group"]) for i in (0, 2, 1, 3)], lag_ratio=.25), _mn.FadeIn(self.table), seconds=3)
        self.runs_chip = self._chip(self.tr("2² = 4 simulations", "2² = 4 Simulationen"), (PANEL_X, -.85, 0), 20, GOOD)
        self.order_note = self._t(self.tr("cards: A grows to the right, B grows upwards", "Karten: A wächst nach rechts, B nach oben"), (PANEL_X, -.3, 0), 15, MUTED, 6.0)
        windows = _mn.LaggedStart(*[_mn.Indicate(self.cards[i]["refs"]["glass"], color=DESIGN, scale_factor=1.5) for i in (0, 1, 2, 3)], lag_ratio=.25)
        slabs = _mn.LaggedStart(*[_mn.Indicate(self.cards[i]["refs"]["slab"], color=OPEN, scale_factor=1.5) for i in (1, 3)], lag_ratio=.25)
        self.step("plan_2", _mn.Succession(windows, slabs), _mn.FadeIn(self.runs_chip), _mn.FadeIn(self.order_note), seconds=4)

    def _response(self):
        """📊 Sun heat of every run as rays, then its cooling electricity as a bar and a table entry."""
        self.add(*[card["bar"] for card in self.cards])
        self.cooling_cells = _mn.VGroup(*[self.cells[(row, 5)] for row in range(-1, 4)])
        self.legend = self._legend()
        fills = [_mn.FadeIn(cell) for cell in self.cooling_cells]
        best, worst = min(self.cards, key=lambda c: c["run"]["response"]), max(self.cards, key=lambda c: c["run"]["response"])
        self.frames = _mn.VGroup(self._frame(best, SHADED), self._frame(worst, OPEN))
        self.low_chip = self._chip(self.tr("Lower is better", "Kleiner ist besser"), (PANEL_X, -1.35, 0), 18, GOOD)
        flow = _mn.VGroup(self._chip(self.tr("WWR · overhang", "WWR · Vordach"), (0, 0, 0), 15, DESIGN, 2.2), self._t("→", (0, 0, 0), 20, INK, .4),
                          self._chip(self.tr("teaching model", "Lehrmodell"), (0, 0, 0), 15, DATA, 2.2), self._t("→", (0, 0, 0), 20, INK, .4),
                          self._chip(self.tr("kWh/a, not measured", "kWh/a, nicht gemessen"), (0, 0, 0), 15, COOL, 2.6)).arrange(RIGHT, buff=.12).scale_to_fit_width(6.2).move_to((PANEL_X, -2.05, 0))
        self.note = flow
        self.step("response", *self._head("Simulate each room: sun in, cooling electricity out", "Jeden Raum simulieren: Sonne rein, Kühlstrom raus"), _mn.LaggedStart(*[self._shine(card) for card in self.cards], lag_ratio=.3), _mn.FadeIn(self.legend), _mn.LaggedStart(*[_mn.FadeIn(card["sun_heat"]) for card in self.cards], lag_ratio=.3), seconds=4.5)
        self.step("response_2", _mn.Succession(_mn.AnimationGroup(_mn.LaggedStart(*[card["grow"].animate.set_value(1) for card in self.cards], lag_ratio=.3), _mn.LaggedStart(*fills, lag_ratio=.25), run_time=2.5),
                                               _mn.AnimationGroup(_mn.Create(self.frames), _mn.FadeIn(self.low_chip), run_time=1)), seconds=3.5)
        self.step("response_3", _mn.FadeIn(self.note), _mn.Indicate(self.source, color=INK, scale_factor=1.04), seconds=1.5)

    def _plot(self):
        """📈 The four runs as two lines of cooling against window size."""
        axes = self._chart()
        lines = _mn.VGroup(*[_mn.Line(self._spot(RUNS[i]), self._spot(RUNS[j]), color=color, stroke_width=3) for i, j, color in ((0, 2, OPEN), (1, 3, SHADED))])
        self.lines_ += [[self._spot(RUNS[0]), self._spot(RUNS[2])], [self._spot(RUNS[1]), self._spot(RUNS[3])]]
        self.marks = [self._mark(run) for run in RUNS]
        tags = _mn.VGroup(*[self._near(self._tag((0, 0, 0), SHOWN[i], OPEN if run["design"]["overhang"] == 0 else SHADED), self._spot(run), ("up", "upleft", "left") if run["design"]["overhang"] == 0 else ("down", "downright", "right"), gap=.27 if run["design"]["overhang"] == 0 else .42)
                            for i, run in enumerate(RUNS)])
        corner = self._at_plot(10, YMAX)
        legend = _mn.VGroup(self._at("● " + self.tr("no overhang", "kein Vordach"), (corner[0] + .2, corner[1] - .12, 0), 15, OPEN, 1.9),
                            self._at("■ " + self._depth(HIGH_D) + " " + self.tr("overhang", "Vordach"), (corner[0] + .2, corner[1] - .35, 0), 15, SHADED, 1.9))
        self.chart = _mn.VGroup(axes, lines, tags, legend)
        starts = [_mn.Dot(np.array(card["base"]) + UP * card["run"]["response"] * BAR_HEIGHT / YMAX, radius=.07, color=COOL) for card in self.cards]
        self.add(*starts)
        flights = _mn.LaggedStart(*[_mn.ReplacementTransform(start, mark, path_arc=-.7) for start, mark in zip(starts, self.marks)], lag_ratio=.25)
        self.step("plot", _mn.FadeOut(self.table), _mn.FadeOut(self.cooling_cells), _mn.FadeOut(self.runs_chip), _mn.FadeOut(self.order_note), _mn.FadeOut(self.low_chip), _mn.FadeOut(self.note), _mn.FadeOut(self.frames),
                  _mn.Succession(_mn.FadeIn(axes, run_time=1), flights, _mn.AnimationGroup(_mn.Create(lines), _mn.FadeIn(tags), _mn.FadeIn(legend), run_time=1.2)), seconds=6)
        colored = [self._badge(card["badge"][0].get_center(), index + 1, color=SHADED if card["run"]["design"]["overhang"] > 0 else OPEN, shape="square" if card["run"]["design"]["overhang"] > 0 else "circle", fill=True) for index, card in enumerate(self.cards)]
        pulses = [_mn.AnimationGroup(_mn.Indicate(self.marks[i], color=INK, scale_factor=1.9), _mn.ReplacementTransform(self.cards[i]["badge"], colored[i])) for i in (0, 2, 1, 3)]
        self.step("plot_2", _mn.LaggedStart(*pulses, lag_ratio=.5), seconds=4)
        for index, card in enumerate(self.cards):
            card["badge"] = colored[index]

    def _saving(self, low, high, *, side, fraction):
        """💚 Ghost bar and plot arrow for two runs that differ only in the overhang; the two-line value label stands left or right of the arrow."""
        unit, saving, german = BAR_HEIGHT / YMAX, SHOWN[low] - SHOWN[high], self.language() == "de"
        base = np.array(self.cards[high]["base"])
        segment = _mn.Rectangle(width=.3, height=saving * unit, stroke_width=0, fill_color=SHADED, fill_opacity=.6).move_to(base + UP * (SHOWN[high] + saving / 2) * unit)
        card = _mn.VGroup(self._ghost_bar(base, SHOWN[low]), segment)
        top, bottom = self._spot(RUNS[low]), self._spot(RUNS[high])
        arrow = _mn.Arrow(top + DOWN * .1, bottom + UP * .1, buff=0, color=SHADED, stroke_width=4, tip_length=.15, max_tip_length_to_length_ratio=.6)
        label = _mn.VGroup(_room.text(_room.parts(-saving, "", 1, german), (0, 0, 0), size=15, color=SHADED), _room.text("kWh/a", (0, 0, 0), size=14, color=SHADED)).arrange(DOWN, buff=.07)
        anchor = bottom + (top - bottom) * fraction
        self.lines_.append([top, bottom])
        note = self._near(label, anchor, (side, "left" if side == "right" else "right"), gap=.32)
        return card, _mn.VGroup(arrow, note)

    def _chain(self, low, high):
        """⛓️ Sun beam stopped, cooling load removed and cooling electricity saved: every printed step follows from the model."""
        shade = RUNS[high]["shading_summer"]
        stopped = RUNS[low]["solar_summer"] - RUNS[high]["solar_summer"]
        rays = len(_geometry(RUNS[high]["design"])[2])
        loads = (round(RUNS[low]["cooling_demand"]), round(RUNS[high]["cooling_demand"]))
        return self._lines((f"{self.tr('Direct sun stopped', 'Direktsonne gestoppt')}: {rays} × {self._kwh(RAY_BEAM)} × {self.number(shade, 3)} {self.tr('shaded', 'beschattet')} = {self._kwh(stopped, 0)}", _room.SUN, 16),
                           (f"{self.tr('Cooling load', 'Kühllast')}: {self.number(loads[0])} → {self._kwh(loads[1], 0)} ({self.number(loads[1] - loads[0])})", COOL, 16),
                           (f"÷ EER {self.number(EER, 1)}: {self.tr('electricity', 'Strom')} {self.number(SHOWN[low], 1)} → {self._kwh(SHOWN[high])} ({self.number(SHOWN[high] - SHOWN[low], 1)})", SHADED, 16), top=2.5, width=6.3)

    def _convention(self):
        """⚖️ One sign convention for every effect."""
        return self._chip(self.tr("Effect = high level − low level · negative = saves cooling electricity", "Effekt = hohe Stufe − niedrige Stufe · negativ = spart Kühlstrom"), (PANEL_X, 1.4, 0), 15, MUTED, 6.5)

    def _savings(self):
        """🧮 The overhang saves little at the small window and much more at the large one."""
        veil_large, veil_small = _mn.VGroup(*[self._veil(self.cards[i]) for i in (2, 3)]), _mn.VGroup(*[self._veil(self.cards[i]) for i in (0, 1)])
        card_small, arrow_small = self._saving(0, 1, side="left", fraction=.5)
        card_large, arrow_large = self._saving(2, 3, side="right", fraction=.85)
        chain_small, chain_large = self._chain(0, 1), self._chain(2, 3)
        blocked_small, blocked_large = self.cards[1]["refs"]["blocked"], self.cards[3]["refs"]["blocked"]
        self.convention = self._convention()
        self.step("saving_30", *self._head("The overhang saves more at a large window", "Das Vordach spart mehr bei großem Fenster"), _mn.FadeIn(veil_large), _mn.FadeIn(card_small), _mn.FadeIn(chain_small), _mn.FadeIn(self.convention), _mn.GrowFromEdge(arrow_small, UP), _mn.Indicate(blocked_small, color=ALERT, scale_factor=1.12), seconds=3.5)
        self.step("saving_70", _mn.FadeOut(veil_large), _mn.FadeIn(veil_small), _mn.FadeIn(card_large), _mn.ReplacementTransform(chain_small, chain_large), _mn.GrowFromEdge(arrow_large, UP),
                  _mn.Indicate(blocked_large, color=ALERT, scale_factor=1.08), seconds=3.5)
        stopped = (RUNS[2]["solar_summer"] - RUNS[3]["solar_summer"], RUNS[0]["solar_summer"] - RUNS[1]["solar_summer"])
        saved = (SHOWN[2] - SHOWN[3], SHOWN[0] - SHOWN[1])
        rays = (len(_geometry(RUNS[3]["design"])[2]), len(_geometry(RUNS[1]["design"])[2]))
        ratios = self._lines((f"{self.tr('rays', 'Strahlen')} {rays[0]} ÷ {rays[1]} = {self.number(rays[0] / rays[1], 1)} · {self.tr('direct sun stopped', 'Direktsonne gestoppt')} {self.number(stopped[0])} ÷ {self.number(stopped[1])} = {self.number(stopped[0] / stopped[1], 1)}", _room.SUN, 16),
                             (f"{self.tr('cooling electricity effect', 'Kühlstrom-Effekt')} {self.number(-saved[0], 1)} ÷ ({self.number(-saved[1], 1)}) = {self.number(saved[0] / saved[1], 1)}", SHADED, 16), top=2.5, width=6.3)
        self.savings, self.gain = _mn.VGroup(card_small, card_large, arrow_small, arrow_large), chain_large
        start = self._at_plot(RUNS[1]["design"]["wwr"] * 100, SHOWN[1])
        slope = SHOWN[2] - SHOWN[0]
        end_value = SHOWN[1] + slope
        parallel = _mn.DashedLine(start, self._at_plot(70, end_value), color=OPEN, stroke_width=2.4)
        self.lines_.append([start, self._at_plot(70, end_value)])
        gap_x = self._at_plot(70, 0)[0] + .3
        top_end, bottom_end = self._at_plot(70, end_value), self._spot(RUNS[3])
        gap_label = _mn.VGroup(self._t(self.tr("difference", "Differenz"), (0, 0, 0), 14, MIXED, 1.4), self._t(self._kwh(end_value - SHOWN[3]), (0, 0, 0), 14, MIXED, 1.4)).arrange(DOWN, buff=.08, aligned_edge=LEFT)
        _room.place(gap_label, (gap_x + .12, (top_end[1] + bottom_end[1]) / 2, 0), "left")
        gap = _mn.VGroup(_mn.Line((gap_x, top_end[1], 0), (gap_x, bottom_end[1], 0), color=MIXED, stroke_width=3), _mn.Line((gap_x - .08, top_end[1], 0), (gap_x + .08, top_end[1], 0), color=MIXED, stroke_width=3),
                         _mn.Line((gap_x - .08, bottom_end[1], 0), (gap_x + .08, bottom_end[1], 0), color=MIXED, stroke_width=3), gap_label)
        key_x, key_y = self._at_plot(10, YMAX)[0] + .2, self._at_plot(10, YMAX)[1] - .58
        key = _mn.VGroup(_room.dashed((key_x, key_y, 0), (key_x + .4, key_y, 0), OPEN, 2.4), self._at(self.tr("if parallel", "wenn parallel"), (key_x + .5, key_y, 0), 15, OPEN, 1.7))
        self.parallel = _mn.VGroup(parallel, gap, key)
        change = self._chip(f"{self.tr('Overhang effect', 'Vordach-Effekt')}: {self._kwh(SHOWN[1] - SHOWN[0], sign=True)} {self.tr('at 30 %', 'bei 30 %')} · {self._kwh(SHOWN[3] - SHOWN[2], sign=True)} {self.tr('at 70 %', 'bei 70 %')}", (PANEL_X, 2.5, 0), 16, MIXED, 6.3)
        rule = self._t(self.tr("depends on the window share = interaction", "hängt vom Fensteranteil ab = Wechselwirkung"), (PANEL_X, 2.05, 0), 16, MIXED, 6.3)
        self.step("interaction", _mn.FadeOut(veil_small), _mn.FadeOut(self.legend), _mn.ReplacementTransform(chain_large, ratios), _mn.Create(parallel), _mn.FadeIn(gap), _mn.FadeIn(key), seconds=2.5)
        self.step("interaction_2", _mn.ReplacementTransform(ratios, _mn.VGroup(change, rule)), _mn.Indicate(blocked_small, color=ALERT, scale_factor=1.12), _mn.Indicate(blocked_large, color=ALERT, scale_factor=1.08), seconds=2)
        self.rule = _mn.VGroup(change, rule)
        self.gain = _mn.VGroup()


    def _effects(self):
        """📐 Main effects and the interaction effect as column, row and diagonal contrasts."""
        effects = FACTORIAL["effects"]
        r1, r2, r3, r4 = SHOWN
        self.lines_ = self.lines_[:4]
        columns = [self._rect((CARD_X[i], (CARD_Y[0] + CARD_Y[1]) / 2 + CARD_DY, 0), CARD_W, CARD_Y[0] - CARD_Y[1] + CARD_H, color, stroke=3.4).set_z_index(6) for i, color in ((0, DESIGN), (1, DATA))]
        rows = [self._rect(((CARD_X[0] + CARD_X[1]) / 2, CARD_Y[i] + CARD_DY, 0), CARD_X[1] - CARD_X[0] + CARD_W, CARD_H, color, stroke=3.4).set_z_index(6) for i, color in ((0, SHADED), (1, OPEN))]
        diagonals = [self._frame(self.cards[i], MIXED, 3.4) for i in (0, 3)] + [self._frame(self.cards[i], DESIGN, 3.4) for i in (1, 2)]
        signs_a, signs_b, signs_ab = self._signs(self.cards, [-1, -1, 1, 1], DATA), self._signs(self.cards, [-1, 1, -1, 1], SHADED), self._signs(self.cards, [1, -1, -1, 1], MIXED)
        a_high, a_low = (r3 + r4) / 2, (r1 + r2) / 2
        b_high, b_low = (r2 + r4) / 2, (r1 + r3) / 2
        a_effect, b_effect = round(a_high - a_low, 1), round(b_high - b_low, 1)
        small, large = round(r2 - r1, 1), round(r4 - r3, 1)
        ab_effect = round((large - small) / 2, 1)
        german = self.language() == "de"
        x_70 = self._at_plot(70, 0)[0]
        diamonds = [_mn.Square(side_length=.17, color=DATA, stroke_width=3).rotate(math.pi / 4).move_to(self._at_plot(wwr, value)) for wwr, value in ((30, a_low), (70, a_high))]
        level = _mn.DashedLine(diamonds[0].get_center(), (x_70, diamonds[0].get_center()[1], 0), color=DATA, stroke_width=2)
        arrow_a = _mn.Arrow((x_70 + .3, self._at_plot(70, a_low)[1], 0), (x_70 + .3, self._at_plot(70, a_high)[1], 0), buff=0, color=DATA, stroke_width=4, tip_length=.15, max_tip_length_to_length_ratio=.6)
        note_a = _room.place(_mn.VGroup(_room.text(_room.parts(a_effect, "", 1, german), (0, 0, 0), size=15, color=DATA), _room.text("kWh/a", (0, 0, 0), size=14, color=DATA)).arrange(DOWN, buff=.07), (x_70 + .42, (diamonds[0].get_center()[1] + diamonds[1].get_center()[1]) / 2, 0), "left")
        self.lines_ += [[self._at_plot(30, a_low), (x_70, self._at_plot(30, a_low)[1], 0)]]
        means_a = _mn.VGroup(*[self._near(self._t(self.number(value, 2), (0, 0, 0), 14, DATA, 1.0), diamond.get_center(), ("left", "upleft", "downleft"), gap=.3) for value, diamond in ((a_low, diamonds[0]), (a_high, diamonds[1]))])
        overlay_a = _mn.VGroup(*diamonds, level, arrow_a, note_a, means_a, signs_a)
        circles = [_mn.Circle(radius=.1, color=SHADED, stroke_width=3).move_to(self._at_plot(50, value)) for value in (b_low, b_high)]
        means_line = _mn.VGroup(*[_mn.DashedLine(self._at_plot(10, value), self._at_plot(90, value), color=SHADED, stroke_width=1.8) for value in (b_low, b_high)])
        arrow_b = _mn.Arrow(self._at_plot(50, b_low) + DOWN * .1, self._at_plot(50, b_high) + UP * .1, buff=0, color=SHADED, stroke_width=4, tip_length=.15, max_tip_length_to_length_ratio=.6)
        self.lines_ += [[self._at_plot(10, value), self._at_plot(90, value)] for value in (b_low, b_high)]
        note_b = self._near(_room.text(_room.parts(b_effect, "kWh/a", 1, german), (0, 0, 0), size=15, color=SHADED), (self._at_plot(50, b_low) + self._at_plot(50, b_high)) / 2, ("right", "left"), gap=.3)
        means_b = _mn.VGroup(*[self._near(self._t(self.number(value, 2), (0, 0, 0), 14, SHADED, 1.0), self._at_plot(90, value) + np.array([-.35, 0, 0]), order, gap=.2) for value, order in ((b_low, ("up", "down")), (b_high, ("down", "up")))])
        overlay_b = _mn.VGroup(*circles, means_line, arrow_b, note_b, means_b, signs_b)
        kwh = self._kwh
        num = lambda value, digits=1: self.number(value, digits)
        diff = round(large - small, 1)
        form_a = self._formula_rows([[(self.tr("Effect A", "Effekt A"), DATA), "=", (self.tr("mean at 70 %", "Mittel bei 70 %"), DATA), "−", (self.tr("mean at 30 %", "Mittel bei 30 %"), DESIGN)],
                                     ["=", (f"({num(r3)} + {num(r4)}) ÷ 2", DATA), "−", (f"({num(r1)} + {num(r2)}) ÷ 2", DESIGN)],
                                     ["=", (num(a_high, 2), DATA), "−", (num(a_low, 2), DESIGN), "=", (kwh(a_effect, 1, True), DATA)]], (PANEL_X, 2.4, 0), size=20, width=6.3)
        form_b = self._formula_rows([[(self.tr("Effect B", "Effekt B"), SHADED), "=", (self.tr("mean with 1.0 m", "Mittel mit 1,0 m"), SHADED), "−", (self.tr("mean without", "Mittel ohne"), OPEN)],
                                     ["=", (f"({num(r2)} + {num(r4)}) ÷ 2", SHADED), "−", (f"({num(r1)} + {num(r3)}) ÷ 2", OPEN)],
                                     ["=", (num(b_high, 2), SHADED), "−", (num(b_low, 2), OPEN), "=", (kwh(b_effect, 1, True), SHADED)]], (PANEL_X, 2.4, 0), size=20, width=6.3)
        form_ab = self._formula_rows([[(self.tr("difference", "Differenz"), MIXED), "=", (self.tr("effect at 70 %", "Effekt bei 70 %"), MIXED), "−", (self.tr("effect at 30 %", "Effekt bei 30 %"), MIXED)],
                                      ["=", (kwh(large, 1, True), INK), "−", (f"({kwh(small, 1, True)})", INK), "=", (kwh(diff, 1, True), MIXED)],
                                      [("A·B", MIXED), "=", "½", "×", (f"({kwh(diff, 1, True)})", MIXED), "=", (kwh(ab_effect, 1, True), MIXED)]], (PANEL_X, 2.4, 0), size=20, width=6.3)
        meaning = _mn.VGroup(self._t(self.tr("Interaction A·B", "Wechselwirkung A·B") + " = " + self._kwh(ab_effect, 1, True), (PANEL_X, 2.5, 0), 22, MIXED, 6.4),
                             self._t(self.tr("negative: the bigger the window, the more the overhang saves", "negativ: je größer das Fenster, desto mehr spart das Vordach"), (PANEL_X, 2.1, 0), 16, INK, 6.4))
        top_edge = (CARD_Y[0] + CARD_Y[1]) / 2 + CARD_DY + (CARD_Y[0] - CARD_Y[1] + CARD_H) / 2
        tags_a = self._column_tags([(self.tr("mean at 30 %", "Mittel bei 30 %"), DESIGN, CARD_X[0], 2.98, top_edge, "up"), (self.tr("mean at 70 %", "Mittel bei 70 %"), DATA, CARD_X[1], 2.98, top_edge, "up")])
        tags_b = self._column_tags([(self.tr("mean with 1.0 m", "Mittel mit 1,0 m"), SHADED, CARD_X[0], 2.98, top_edge, "up"), (self.tr("mean without", "Mittel ohne"), OPEN, CARD_X[1] + CARD_W / 2 + .1, CARD_Y[1] + CARD_DY, CARD_X[1] + CARD_W / 2, "side")])
        tags_ab = self._column_tags([(self.tr("effect at 30 %", "Effekt bei 30 %"), MIXED, CARD_X[0], 2.98, top_edge, "up"), (self.tr("effect at 70 %", "Effekt bei 70 %"), MIXED, CARD_X[1], 2.98, top_edge, "up")])
        self.step("main", *self._head("Effects from the four runs", "Effekte aus den vier Läufen"), _mn.FadeIn(tags_a), _mn.FadeOut(self.savings), _mn.FadeOut(self.gain), _mn.FadeOut(self.parallel), _mn.FadeOut(self.rule), _mn.FadeIn(columns[0]), _mn.FadeIn(columns[1]), _mn.FadeIn(overlay_a), _mn.FadeIn(form_a), seconds=3.5)
        self.step("main_2", _mn.FadeOut(_mn.VGroup(*columns)), _mn.FadeOut(tags_a), _mn.FadeIn(tags_b), _mn.FadeOut(overlay_a), _mn.FadeOut(form_a), _mn.FadeIn(_mn.VGroup(*rows)), _mn.FadeIn(overlay_b), _mn.FadeIn(form_b), seconds=3.5)
        self.step("difference", _mn.FadeOut(tags_b), _mn.FadeIn(tags_ab), _mn.FadeOut(_mn.VGroup(*rows)), _mn.FadeOut(overlay_b), _mn.FadeOut(form_b), _mn.FadeIn(_mn.VGroup(*diagonals)), _mn.FadeIn(signs_ab), _mn.FadeIn(self.savings), _mn.FadeIn(form_ab), seconds=3.5)
        self.step("difference_2", _mn.ReplacementTransform(form_ab, meaning), seconds=2)
        self.cleanup = _mn.VGroup(self.savings, _mn.VGroup(*diagonals), signs_ab, meaning, self.convention, tags_ab)

    def _column_tags(self, items):
        """🏷️ Coloured tags above or beside card frames with a short connector to the frame edge: the formula term and the drawn rooms it averages share one colour."""
        group = _mn.VGroup()
        for text, color, x, y, edge_y, side in items:
            label = _room.place(self._t(text, (0, 0, 0), 15, color, 2.6), (x, y, 0), "center" if side == "up" else "left")
            if side == "up":
                group.add(label, _mn.Line((x, y - .14, 0), (x, edge_y, 0), color=color, stroke_width=2.4))
            else:
                group.add(label, _mn.Line((x - .02, y, 0), (edge_y, y, 0), color=color, stroke_width=2.4))
        return group

    def _waterfall(self, trackers):
        """🪜 Run prediction as a waterfall of the constant and the three terms, live from the coded levels of the design trackers."""
        c, german, unit = FACTORIAL["coefficients"], self.language() == "de", WF_HEIGHT / YMAX
        names = [(self.tr("mean", "Mittel"), COOL), (self.tr("window", "Fenster"), DESIGN), (self.tr("overhang", "Vordach"), OPEN), ("A·B", MIXED), (self.tr("result", "Ergebnis"), COOL)]

        def build():
            va, vb = self._coded_of(trackers)
            steps = [c[0], c[1] * va, c[2] * vb, c[3] * va * vb]
            levels = [c[0], c[0] + steps[1], c[0] + steps[1] + steps[2], c[0] + sum(steps[1:])]
            spans = [(0, levels[0]), (levels[0], levels[1]), (levels[1], levels[2]), (levels[2], levels[3]), (0, levels[3])]
            shown = _room.apportion(steps, 0, sum(steps))
            group = _mn.VGroup()
            for index, ((lo, hi), (name, color)) in enumerate(zip(spans, names)):
                x = WF_X + WF_PITCH * index
                low, high = min(lo, hi), max(lo, hi)
                span = max((high - low) * unit, 1e-3)
                group.add(_mn.Rectangle(width=.54, height=span, color=color, stroke_width=1.2, fill_color=color, fill_opacity=.78).move_to((x, WF_Y + low * unit + span / 2, 0)))
                value = shown[0] if index == 0 else (round(sum(steps)) if index == 4 else shown[index])
                if index in (0, 4):
                    group.add(_room.text(_room.parts(value, "", 0, german), (x, WF_Y + high * unit + .2, 0), size=15, color=INK))
                elif abs(value) >= .5:
                    group.add(_room.text(("+" if value > 0 else "") + _room.number(value, 0, german), (x, WF_Y + high * unit + .2, 0), size=14, color=color))
                if index < 4:
                    group.add(_room.dashed((x + .27, WF_Y + levels[index] * unit, 0), (x + WF_PITCH - .27, WF_Y + levels[index] * unit, 0), color=MUTED, stroke_width=1.2))
            return group

        static = _mn.VGroup(_mn.Line((WF_X - .4, WF_Y, 0), (WF_X + WF_PITCH * 4 + .4, WF_Y, 0), color=MUTED, stroke_width=2),
                            *[self._t(name, (WF_X + WF_PITCH * index, WF_Y - .22, 0), 14, color, .8) for index, (name, color) in enumerate(names)])
        return _mn.VGroup(static, self._live(build))

    def _coded_of(self, trackers):
        """🔢 Coded levels A and B of the live design."""
        return (trackers["wwr"].get_value() - (HIGH_WWR + LOW_WWR) / 2) / ((HIGH_WWR - LOW_WWR) / 2), (trackers["overhang"].get_value() - (HIGH_D + LOW_D) / 2) / ((HIGH_D - LOW_D) / 2)

    def _term_rows(self, trackers):
        """🧾 The bilinear prediction with the live coded levels substituted, parts printed so that they add up to the printed total."""
        c = FACTORIAL["coefficients"]
        sign = lambda value: ("+ " if round(value) >= 0 else "− ") + self.number(abs(value), 0)
        coded = lambda value: ("+" if round(value, 2) > 0 else "") + self.number(value, 1 if abs(value - round(value)) > .05 else 0)

        def rows(design, found):
            a, b = self._coded_of(trackers)
            terms = [c[0], c[1] * a, c[2] * b, c[3] * a * b]
            shown = _room.apportion(terms, 0, sum(terms))
            return [[(self.number(round(c[0])), COOL), (f"{sign(round(c[1]))} × A ({coded(a)})", DESIGN), (f"{sign(round(c[2]))} × B ({coded(b)})", OPEN), (f"{sign(round(c[3]))} × A·B ({coded(a * b)})", MIXED), ("kWh/a", MUTED)],
                    [("=", MUTED), (self.number(shown[0]), COOL), (sign(shown[1]), DESIGN), (sign(shown[2]), OPEN), (sign(shown[3]), MIXED), (f"= {self.number(sum(shown))} kWh/a", COOL)]]

        return rows

    def _model_fit(self):
        """📏 The bilinear model, one room that follows the coded levels and its prediction as a waterfall and a substituted formula."""
        coefficients, effects = FACTORIAL["coefficients"], FACTORIAL["effects"]
        self.trackers_model = self.study_trackers(RUNS[3]["design"])
        terms = [(self.tr("Cooling =", "Kühlstrom ="), MUTED), (self.number(coefficients[0]), COOL)] + [(f"{'+' if value > 0 else '−'} {self.number(abs(value))}·{name}", color) for value, name, color in zip(coefficients[1:], ("A", "B", "A·B"), (DESIGN, OPEN, MIXED))] + [("kWh/a", MUTED)]
        formula = _mn.VGroup(*[self._t(text, (0, 0, 0), 22, color, 6) for text, color in terms]).arrange(RIGHT, buff=.16).move_to((.9, 2.62, 0))
        room = self._live_section(self.trackers_model, MODEL_ORIGIN, MODEL_S, labels=True, person=False, daylight=True)
        waterfall = self._waterfall(self.trackers_model)
        level = lambda value: self._sign(value, 0 if abs(value - round(value)) < .05 else 1) if round(value, 1) else "0"
        readout = self._live(lambda: self._t(f"A {level(self._coded_of(self.trackers_model)[0])} · B {level(self._coded_of(self.trackers_model)[1])}", (WF_X + WF_PITCH * 2, -2.17, 0), 16, INK, 2.4))
        rows = self._formula_rows(self._term_rows(self.trackers_model), (2.4, 1.75, 0), size=20, width=8.4, trackers=self.trackers_model)
        run4 = self._chip(f"{self.tr('simulation 4', 'Simulation 4')}: WWR {self._wwr(.7)} · {self._depth(1.0)}\n→ {self._kwh(SHOWN[3])}", (MODEL_X, .45, 0), 15, COOL, 4.4)
        t = self.trackers_model
        tour = _mn.Succession(t["wwr"].animate.set_value(LOW_WWR), t["wwr"].animate.set_value(HIGH_WWR), t["overhang"].animate.set_value(LOW_D), t["overhang"].animate.set_value(HIGH_D))
        fades = _mn.AnimationGroup(_mn.FadeOut(self.cleanup), _mn.FadeOut(self._everything(self.cards)), _mn.FadeOut(self.chart), _mn.FadeOut(_mn.VGroup(*self.marks)), _mn.FadeIn(formula), _mn.FadeIn(room),
                                   _mn.FadeIn(waterfall), _mn.FadeIn(readout), _mn.FadeIn(rows), _mn.FadeIn(run4), *self._head("Bilinear model from four simulations", "Bilineares Modell aus vier Simulationen"), run_time=1.0)
        self.step("model", _mn.Succession(fades, tour), seconds=9)
        centre = FACTORIAL["centre"]["design"]
        mean_chip = self._chip(f"{self.tr('plan centre', 'Planmitte')}: WWR {self._wwr(centre['wwr'])} · {self._depth(centre['overhang'])}\n{self.tr('mean of 4 runs, unrounded', 'Mittel der 4 Läufe, ungerundet')} {self._kwh(coefficients[0])}", (MODEL_X, .45, 0), 15, COOL, 4.4)
        self.step("model_2", self.set_design(self.trackers_model, {"wwr": centre["wwr"], "overhang": centre["overhang"]}, 3), _mn.ReplacementTransform(run4, mean_chip), seconds=4)
        halves = self._lines((f"{self.tr('window', 'Fenster')}: {self._sign(effects['a'], 1)} ÷ 2 = {self._sign(coefficients[1], 1)}", DESIGN, 18),
                             (f"{self.tr('overhang', 'Vordach')}: {self._sign(effects['b'], 1)} ÷ 2 = {self._sign(coefficients[2], 1)}", OPEN, 18),
                             (f"A·B: {self._sign(effects['ab'], 1)} ÷ 2 = {self._sign(coefficients[3], 1)}", MIXED, 18),
                             (self.tr("kWh/a per coded step", "kWh/a je kodiertem Schritt"), MUTED, 15), top=-.45, x=MODEL_X, width=4.4)
        self.step("model_3", _mn.FadeOut(mean_chip), _mn.FadeIn(halves), seconds=3)
        self.model_view = _mn.VGroup(formula, room, waterfall, readout, rows, halves)

    def _curvature(self):
        """🌀 Two levels cannot see curvature; each plotted end has its room, the centre run tests the straight prediction along the overhang depth."""
        centre, coefficients = FACTORIAL["centre"], FACTORIAL["coefficients"]
        low, high = coefficients[0] - coefficients[2], coefficients[0] + coefficients[2]
        chart = self._chart(x_range=(-.2, 1.2, .1), x_ticks=(0, .5, 1.0), x_label=self.tr("Overhang depth (m)", "Vordachtiefe (m)"), y_max=500, y_step=100, x_digits=1)
        axes = self.axes_
        german = self.language() == "de"
        straight = _mn.DashedLine(axes.c2p(0, low), axes.c2p(1, high), color=MUTED, stroke_width=2.6)
        self.lines_.append([axes.c2p(0, low), axes.c2p(1, high)])
        ends = _mn.VGroup(*[_mn.Circle(radius=.09, color=INK, stroke_width=2.4).move_to(axes.c2p(x, y)) for x, y in ((0, low), (1, high))])
        simulated = [(depth, self._cooling({**centre["design"], "overhang": depth})) for depth in (LOW_D, HIGH_D)]
        dots = _mn.VGroup(*[_mn.Dot(axes.c2p(depth, value), radius=.06, color=DATA) for depth, value in simulated])
        key_x, key_y = axes.c2p(.2, 500)[0], axes.c2p(.2, 500)[1] + .06
        key_straight = _mn.VGroup(_room.dashed((key_x, key_y, 0), (key_x + .4, key_y, 0), MUTED, 2.6), self._at(self.tr("straight line (model), two levels", "Gerade (Modell), zwei Stufen"), (key_x + .5, key_y, 0), 14, MUTED, 2.7))
        key_dots = _mn.VGroup(_mn.Dot((key_x + .2, key_y - .22, 0), radius=.06, color=DATA), self._at(self.tr("simulated room", "simulierter Raum"), (key_x + .5, key_y - .22, 0), 14, DATA, 2.4))
        key_true = _mn.VGroup(_mn.Line((key_x, key_y - .22, 0), (key_x + .4, key_y - .22, 0), color=DATA, stroke_width=3), self._at(self.tr("simulated, 50 % window", "simuliert, 50 % Fenster"), (key_x + .5, key_y - .22, 0), 14, DATA, 2.4))
        end_tags = _mn.VGroup(self._near(self._tag((0, 0, 0), low, MUTED), axes.c2p(0, low), ("upright", "up", "right"), gap=.3), self._near(self._tag((0, 0, 0), high, MUTED), axes.c2p(1, high), ("upright", "up", "right"), gap=.3),
                              self._near(self._tag((0, 0, 0), simulated[1][1], DATA), axes.c2p(1, simulated[1][1]), ("right", "downleft", "down"), gap=.3))
        rooms = _mn.VGroup()
        for (depth, value), x_c in zip(simulated, (2.9, 5.75)):
            layers, _ = self._section({**centre["design"], "overhang": depth}, (x_c - .35, 1.3, 0), .32, person=False)
            marker = _mn.Circle(radius=.07, color=INK, stroke_width=2).set_fill(DATA, opacity=1)
            label = _mn.VGroup(marker, _room.text([*_room.parts(depth, "m", 1, german), "·", *_room.parts(value, "kWh/a", 1, german)], (0, 0, 0), size=15, color=INK)).arrange(RIGHT, buff=.1).move_to((x_c, 3.15, 0))
            rooms.add(self._merge(layers), label)
        self.centre = self.study_trackers(centre["design"])
        grow = _mn.ValueTracker(0)
        self.ghost_depth = _mn.ValueTracker(0)
        big = _mn.VGroup(self._live_section(self.centre, BIG_ORIGIN, BIG_S, labels=True, ghost=lambda: self.ghost_depth.get_value(), reach=2.4, demand=lambda: grow.get_value() > .02, daylight=True),
                         self._bar(BIG_BAR, lambda: self._cooling(_room.values(self.centre)), grow, height=BIG_HEIGHT, width=.46, size=16),
                         self._t(self.tr("Cooling electricity", "Kühlstrom"), (BIG_BAR[0], BIG_BAR[1] - .55, 0), 14, COOL, 2.1))
        ghost = _mn.VGroup(self._ghost_bar(BIG_BAR, centre["predicted"], height=BIG_HEIGHT, width=.46),
                           self._t(self.tr("model", "Modell") + " " + self.number(centre["predicted"], 1), np.array(BIG_BAR) + UP * (centre["predicted"] * BIG_HEIGHT / YMAX + .27), 15, MUTED, 1.4))
        self.model_view.clear_updaters()
        fading = _mn.VGroup(self.model_view).clear_updaters()
        self.step("curvature", _mn.FadeOut(fading), _mn.FadeIn(big), _mn.FadeIn(ghost), _mn.FadeIn(chart), *self._head("Confirm the middle: is the model straight?", "Mitte bestätigen: ist das Modell gerade?"),
                  _mn.Succession(_mn.AnimationGroup(_mn.Create(straight), _mn.FadeIn(ends), _mn.FadeIn(rooms), _mn.FadeIn(dots), _mn.FadeIn(end_tags), _mn.FadeIn(key_straight), _mn.FadeIn(key_dots), run_time=1.6)), seconds=5)
        self.big = big
        actual = _mn.Dot(axes.c2p(.5, centre["actual"]), radius=.1, color=DATA)
        hollow = _mn.Circle(radius=.11, color=MUTED, stroke_width=2.4).move_to(axes.c2p(.5, centre["predicted"]))
        confirm_tags = _mn.VGroup(self._near(self._tag((0, 0, 0), centre["predicted"], MUTED), hollow.get_center(), ("upright", "up", "right"), gap=.3), self._near(self._tag((0, 0, 0), centre["actual"], DATA), actual.get_center(), ("downleft", "down", "left"), gap=.3))
        self.step("confirm", _mn.FadeIn(hollow), grow.animate.set_value(1), _mn.FadeIn(actual), _mn.FadeIn(confirm_tags), seconds=3)
        gap = _mn.Line(hollow.get_center() + DOWN * .12, actual.get_center() + UP * .12, color=ALERT, stroke_width=3)
        unit = BIG_HEIGHT / YMAX
        segment = _mn.Rectangle(width=.46, height=(centre["predicted"] - centre["actual"]) * unit, stroke_width=0, fill_color=ALERT, fill_opacity=.6).move_to(np.array(BIG_BAR) + UP * (centre["actual"] + centre["predicted"]) / 2 * unit)
        percent = self.number(centre["curvature"] * 100, 1)
        gap_note = _mn.VGroup(self._chip(f"{self.tr('model', 'Modell')} {self.number(centre['predicted'], 1)} · {self.tr('simulation', 'Simulation')} {self.number(centre['actual'], 1)} kWh/a", (4.4, -1.05, 0), 15, ALERT, 4.5),
                              self._t(f"{self.number(centre['predicted'], 1)} + {self.number(coefficients[1], 1)}·0 − {self.number(abs(coefficients[2]), 1)}·0 − {self.number(abs(coefficients[3]), 1)}·0·0 = {self.number(centre['predicted'], 1)} kWh/a", (4.4, -1.45, 0), 14, INK, 4.5))
        gap_label = _room.place(_mn.VGroup(self._t(f"{self.number(centre['actual'] - centre['predicted'], 1)} kWh/a", (0, 0, 0), 14, ALERT, 1.3), self._t(f"({percent} %)", (0, 0, 0), 14, ALERT, 1.3)).arrange(DOWN, buff=.07, aligned_edge=LEFT),
                                np.array(BIG_BAR) + np.array([.3, (centre["actual"] + centre["predicted"]) / 2 * unit, 0]), "left")
        self.step("confirm_2", _mn.Create(gap), _mn.FadeIn(gap_note), _mn.FadeIn(segment), _mn.FadeIn(gap_label), _mn.Indicate(actual, color=INK, scale_factor=1.8), seconds=2.5)
        saving = FACTORIAL["centre_savings"]
        share_note = _mn.VGroup(self._chip(self._depth(.5) + self.tr(" delivers ", " liefert ") + self.number(saving["share"] * 100) + " % " + self.tr("of the saving of ", "der Einsparung von ") + self._depth(HIGH_D), (4.4, -1.05, 0), 15, SHADED, 4.5),
                                self._t(f"{self.number(saving['half'], 1)} {self.tr('of', 'von')} {self.number(saving['full'], 1)} kWh/a ({self.tr('50 % window', '50 % Fenster')})", (4.4, -1.45, 0), 14, SHADED, 4.5))
        true = self._curve(axes, lambda x: self._cooling({**centre["design"], "overhang": x}), 0, 1, color=DATA)
        trace = self.live_marker(axes, self.centre, lambda d: d["overhang"], lambda d: self._cooling(d), color=DATA, drops=False, readout=False)
        sweep = _mn.Succession(self.centre["overhang"].animate.set_value(LOW_D), _mn.AnimationGroup(self.centre["overhang"].animate.set_value(HIGH_D), _mn.Create(true, rate_func=_mn.linear), self.ghost_depth.animate.set_value(HIGH_D), run_time=3), self.centre["overhang"].animate.set_value(.5))
        self.step("confirm_3", _mn.FadeOut(gap_note), _mn.FadeOut(segment), _mn.FadeOut(gap_label), _mn.FadeIn(share_note), _mn.FadeIn(trace), _mn.ReplacementTransform(key_dots, key_true), sweep, seconds=6)
        self.stage_end = _mn.VGroup(share_note, true, trace, key_true)

    def _replicates(self):
        """🔁 A repeated simulation returns the same value; repeated measurements scatter."""
        centre = FACTORIAL["centre"]
        keep = {id(self.big), id(self._caption), id(self.source), id(self.heading)}
        self.play(*[_mn.FadeOut(item) for item in self.mobjects if id(item) not in keep], self.ghost_depth.animate.set_value(0), run_time=.5)
        base_y, height = -1.5, 2.4
        bar = lambda x, value, color, opacity=.78: _mn.Rectangle(width=.6, height=value * height / YMAX, color=color, stroke_width=1.4, fill_color=color, fill_opacity=opacity).move_to((x, base_y + value * height / YMAX / 2, 0))
        german = self.language() == "de"
        tag = lambda x, value: _room.text(_room.parts(value, "kWh/a", 1, german), (x, base_y - .25, 0), size=16, color=INK)
        title_one = self._t(self.tr("Simulate it again", "Noch einmal simulieren"), (3.9, 2.2, 0), 24, DATA, 4.6)
        pair = _mn.VGroup(bar(2.5, centre["actual"], COOL), bar(5.0, centre["actual"], COOL), tag(2.5, centre["actual"]), tag(5.0, centre["actual"]),
                          self._t(self.tr("1st simulation", "1. Simulation"), (2.5, base_y - .55, 0), 15, INK, 1.8), self._t(self.tr("2nd simulation", "2. Simulation"), (5.0, base_y - .55, 0), 15, INK, 1.8),
                          self._t("=", (3.75, base_y + centre["actual"] * height / YMAX / 2, 0), 40, DATA, .6),
                          self._t(self.tr("same inputs → same output:\nno scatter, no noise estimate", "gleiche Eingaben → gleiche Ausgabe:\nkeine Streuung, keine Rauschschätzung"), (3.9, 1.6, 0), 16, INK, 6.2))
        self.step("repeat", *self._head("Repeat: simulation versus measurement", "Wiederholen: Simulation gegen Messung"), _mn.FadeOut(self.big[1]), _mn.FadeOut(self.big[2]), _mn.FadeIn(title_one), _mn.FadeIn(pair), seconds=3)
        title_two = self._t(self.tr("Measure it again", "Noch einmal messen"), (3.9, 2.2, 0), 24, DATA, 4.6)
        spread = (-.9, .45, -.3, .8)
        dots = _mn.VGroup(*[_mn.Dot((2.9 + .7 * i, -.1 + w * .5, 0), radius=.11, color=MUTED) for i, w in enumerate(spread)])
        band = _mn.DashedVMobject(_mn.Rectangle(width=3.3, height=1.0, color=MIXED, stroke_width=2).move_to((3.95, -.1 + .17 * .5, 0)), num_dashes=28)
        icons = _mn.VGroup(self._cloud().scale(1.6).move_to((2.4, -1.35, 0)), _art.person_glyph((3.9, -1.35, 0), _art.P_ORANGE).scale_to_fit_height(.55).move_to((3.9, -1.35, 0)),
                           _mn.VGroup(_mn.Rectangle(width=.34, height=.46, color=INK, stroke_width=2), _mn.Polygon((.17, .23, 0), (.42, .14, 0), (.42, -.34, 0), (.17, -.23, 0), color=INK, stroke_width=2, fill_color=_room.GLASS, fill_opacity=.4)).move_to((5.4, -1.35, 0)))
        scatter = _mn.VGroup(dots, band, icons, self._t(self.tr("weather · occupants · open windows", "Wetter · Nutzer · offene Fenster"), (3.9, -1.95, 0), 16, INK, 5.0),
                             self._t(self.tr("each repeat measures a different value\n(schematic: the room on the left is the simulated one)", "jede Wiederholung misst anders\n(Schema: der Raum links ist der simulierte)"), (3.9, 1.6, 0), 16, INK, 6.0),
                             self._t(self.tr("assumed spread (schematic)", "angenommene Streuung (Schema)"), (3.95, .62, 0), 15, MIXED, 3.0))
        self.step("repeat_2", _mn.FadeIn(self.big[1]), _mn.FadeIn(self.big[2]), _mn.FadeOut(pair), _mn.FadeOut(title_one), _mn.FadeIn(title_two), _mn.FadeIn(scatter), seconds=3)

    def _cube(self, point):
        """🧊 Sketch of the 2³ plan: eight corners, the four of the half fraction filled."""
        front = [(-.45, -.45), (.25, -.45), (.25, .25), (-.45, .25)]
        shift = np.array([.32, .3])
        lines = [(np.array(a), np.array(b)) for square in (front, [tuple(np.array(p) + shift) for p in front]) for a, b in zip(square, square[1:] + square[:1])]
        lines += [(np.array(p), np.array(p) + shift) for p in front]
        group = _mn.VGroup(*[_mn.Line((point[0] + a[0], point[1] + a[1], 0), (point[0] + b[0], point[1] + b[1], 0), color=MUTED, stroke_width=1.6) for a, b in lines])
        corners = [(p, 0) for p in front] + [(tuple(np.array(p) + shift), 1) for p in front]
        for index, (p, layer) in enumerate(corners):
            on = (index % 4 in (0, 2)) == (layer == 0)
            group.add(_mn.Dot((point[0] + p[0], point[1] + p[1], 0), radius=.07, color=GOOD if on else MUTED).set_fill(GOOD if on else _art.P_DEEP_DARK, opacity=1))
        return group

    def _fraction(self):
        """🧩 Purpose of the plan and a half fraction whose insulation factor is confounded with the interaction."""
        self.clear_stage()
        effects, centre = FACTORIAL["effects"], FACTORIAL["centre"]
        lines = [(self.tr("Question: window share and overhang → summer cooling electricity", "Frage: Fensteranteil und Vordach → sommerlicher Kühlstrom")),
                 (self.tr("Effects: window share ", "Effekte: Fensteranteil ") + f"{self._kwh(effects['a'], sign=True)} · {self.tr('overhang', 'Vordach')} {self._kwh(effects['b'], sign=True)} · {self.tr('interaction', 'Wechselwirkung')} {self._kwh(effects['ab'], sign=True)}"),
                 (self.tr("Confirmation: middle simulation ", "Bestätigung: mittlere Simulation ") + f"{self.number(centre['actual'], 1)} " + self.tr("vs model", "statt Modell") + f" {self.number(centre['predicted'], 1)} kWh/a")]
        entries = _mn.VGroup(*[_mn.VGroup(self._tick((-6.4, 2.0 - i * .7, 0)), self._at(text, (-6.0, 2.0 - i * .7, 0), 20, INK, 12.4)) for i, text in enumerate(lines)])
        runs = RUNS + [{"design": centre["design"], "response": centre["actual"], "coded": [0, 0]}]
        german = self.language() == "de"
        minis = _mn.VGroup()
        for index, run in enumerate(runs):
            x = -5.4 + index * 2.7
            layers, _ = self._section(run["design"], (x - .3, -1.5, 0), .34, person=False)
            minis.add(self._merge(layers),
                      _room.text(_room.parts(run["response"], "kWh/a", 1, german), (x, -1.95, 0), size=16, color=COOL),
                      self._t(self._headline(run["design"]), (x, -2.3, 0), 14, MUTED, 2.6))
        self.step("purpose", *self._head("Purpose of the plan", "Zweck des Plans"), _mn.LaggedStart(*[_mn.FadeIn(entry) for entry in entries], lag_ratio=.4), _mn.FadeIn(minis), seconds=4)
        self.clear_stage()
        fraction = FRACTION["runs"]
        cards = [self._card(run, index + 1, third=True) for index, run in enumerate(fraction)]
        headers = [self.tr("No.", "Nr."), self.tr("A\nwindow", "A\nFenster"), self.tr("B\noverhang", "B\nVordach"), self.tr("C\ninsulation", "C\nDämmung"), "A·B", self.tr("Cooling\n(kWh/a)", "Kühlstrom\n(kWh/a)")]
        rows = [[str(i + 1), self._coded(run["coded"][0]), self._coded(run["coded"][1]), self._coded(run["coded"][2]), self._coded(run["coded"][0] * run["coded"][1]), self.number(run["response"], 1)] for i, run in enumerate(fraction)]
        table, cells, edges = self._table(headers, rows, 1.0, 2.5, [.55, .85, .95, 1.05, .75, 1.45], height=.5, colors={1: MUTED, 2: MUTED, 3: GOOD, 4: MUTED, 5: COOL}, skip=(4,))
        self.add(*[card["bar"] for card in cards])
        cube = _mn.VGroup(self._cube((6.3, -.75, 0)), self._t(self.tr("full plan: 2³ = 8 simulations · fraction: 4", "voller Plan: 2³ = 8 Simulationen · Teilplan: 4"), (3.35, -.75, 0), 17, GOOD, 4.2),
                          self._t(self.tr("C · insulation 10 / 30 cm (the 2² plan used 16 cm)", "C · Dämmung 10 / 30 cm (der 2²-Plan nutzte 16 cm)"), (PANEL_X, -1.6, 0), 16, INK, 6.4))
        self.step("purpose_2", *self._head("Fractional plan: 4 instead of 8 simulations", "Teilplan: 4 statt 8 Simulationen"), _mn.LaggedStart(*[_mn.FadeIn(card["group"]) for card in cards], lag_ratio=.2), _mn.FadeIn(table), _mn.FadeIn(cube),
                  _mn.LaggedStart(*[_mn.AnimationGroup(self._shine(card), card["grow"].animate.set_value(1)) for card in cards], lag_ratio=.2), seconds=4)
        frame_x, width = (edges[3] + edges[5]) / 2, edges[5] - edges[3]
        frame = self._rect((frame_x, 1.3, 0), width + .06, 2.7, ALERT, stroke=3)
        equals = self._chip("C = A·B", (frame_x, -.4, 0), 17, ALERT, 1.6)
        marked = _mn.VGroup(*[self._frame(cards[i], ALERT, 3.4) for i in (0, 3)])
        self.step("purpose_3", _mn.FadeIn(cells[(-1, 4)]), *[_mn.FadeIn(cells[(row, 4)]) for row in range(4)], _mn.Create(frame), _mn.FadeIn(equals), _mn.FadeIn(marked), seconds=3)
        contrast, truth = FRACTION["alias_contrast"], FRACTION["true"]
        thin, thick = (_room.result({**centre["design"], "wwr": .7, "overhang": 1.0, "insulation": level}) for level in (.1, .3))
        result = self._chip(self.tr("contrast = A·B + C = ", "Kontrast = A·B + C = ") + self._kwh(contrast, 1, True), (PANEL_X, -.4, 0), 19, ALERT, 5.7)
        reveal = self._t(self.tr("all eight simulations (mean effects): ", "alle acht Simulationen (mittlere Effekte): ") + f"A·B {self._kwh(truth['ab'], 1, True)} · C {self._kwh(truth['c'], 1, True)}", (PANEL_X, -.88, 0), 16, INK, 5.7)
        error = self._chip(f"{self.tr('error from confounding', 'Fehler durch Vermengung')}: {self._kwh(abs(contrast - truth['ab']), 1)} · {self.tr('document it', 'dokumentieren')}", (PANEL_X, -1.36, 0), 16, OPEN, 5.7)
        reason = self._lines((self.tr("at 70 %, 1.0 m a thicker wall keeps heat in:", "bei 70 %, 1,0 m hält eine dickere Wand Wärme drin:"), INK, 15),
                             (f"U-{self.tr('wall', 'Wand')} {self.number(thin['u_wall'], 2)} → {self.quantity(thick['u_wall'], 'W/(m²·K)', 2)}", COOL, 16),
                             (f"{self.tr('cooling', 'Kühlung')} {self.number(thin['cooling'], 1)} → {self._kwh(thick['cooling'])}", COOL, 16), top=-1.82, gap=.3, width=5.7)
        self.step("purpose_4", _mn.FadeOut(cube), _mn.FadeOut(equals), _mn.FadeIn(result), _mn.FadeIn(reveal), _mn.FadeIn(error), _mn.FadeIn(reason), seconds=3)

    def _answer(self):
        """🏁 The recommendation in two rooms: the same 1 m overhang saves a lot at 70 % window share and little at 30 %."""
        self.clear_stage()
        german = self.language() == "de"
        unit = 2.9 / YMAX
        static, live, gaps, sweeps = _mn.VGroup(), _mn.VGroup(), _mn.VGroup(), []
        for index, (low, high) in enumerate(((2, 3), (0, 1))):
            bare, shaded = RUNS[low], RUNS[high]
            x0, floor = -5.3 + 6.6 * index, -1.6
            trackers = self.study_trackers(bare["design"])
            base = np.array([x0 + 3.05, floor, 0.0])
            saving = SHOWN[low] - SHOWN[high]
            live.add(self._live_section(trackers, (x0, floor, 0), .7, reach=1.5),
                     self._bar(base, lambda trackers=trackers: self._cooling(_room.values(trackers)), _mn.ValueTracker(1), height=2.9, width=.4, size=15))
            ghost_top = _mn.VGroup(self._t(self._kwh(bare["response"]), (0, 0, 0), 14, MUTED, 1.3), self._t(self.tr("without overhang", "ohne Vordach"), (0, 0, 0), 14, MUTED, 1.5)).arrange(DOWN, buff=.08)
            _room.place(ghost_top, base + UP * (bare["response"] * unit + .45) + RIGHT * .3, "center")
            static.add(self._t(f"WWR {self._wwr(bare['design']['wwr'])}", (x0 + 1.0, 2.4, 0), 26, DESIGN, 3.0),
                       self._t(f"{self._depth(LOW_D)} → {self._depth(HIGH_D)} {self.tr('overhang', 'Vordach')}", (x0 + 1.0, 1.95, 0), 18, OPEN, 3.2),
                       self._ghost_bar(base, bare["response"], height=2.9, width=.4), ghost_top,
                       self._t(self.tr("Cooling electricity", "Kühlstrom"), (base[0], floor - .55, 0), 14, COOL, 2.1))
            gaps.add(_mn.Rectangle(width=.4, height=saving * unit, stroke_width=0, fill_color=SHADED, fill_opacity=.6).move_to(base + UP * (shaded["response"] + saving / 2) * unit),
                     self._at(self._kwh(-saving), base + np.array([.35, (shaded["response"] + saving / 2) * unit + .2, 0]), 22, SHADED, 2.3),
                     self._at(self.tr("by the overhang", "durch das Vordach"), base + np.array([.35, (shaded["response"] + saving / 2) * unit - .15, 0]), 14, MUTED, 1.9))
            sweeps.append(self.set_design(trackers, {"overhang": HIGH_D}, 3))
        self.step("answer", *self._head("Answer for this room", "Antwort für diesen Raum"), _mn.FadeIn(static), _mn.FadeIn(live), _mn.Succession(_mn.Wait(1.2), _mn.AnimationGroup(*sweeps), _mn.FadeIn(gaps, run_time=1)), seconds=6.5)

    def _tick(self, point):
        """✔️ Check mark."""
        mark = _mn.VMobject(color=GOOD, stroke_width=5)
        x, y = point[0], point[1]
        mark.set_points_as_corners([(x - .16, y, 0), (x - .04, y - .13, 0), (x + .18, y + .16, 0)])
        return mark
