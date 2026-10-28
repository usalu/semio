"""🎬 Data Become Information on the Villa Savoye ground floor: a lone number becomes a question, an answer, a model and a handoff."""

import importlib
import math
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_panels = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧾️panels.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_sheet = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_plan_widget = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN, GLASS, CONCRETE, GREEN_GROUND = (
    getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "SERV", "DATA", "GOOD", "WARN", "GLASS", "CONCRETE", "GREEN_GROUND"))
BACKGROUND = _draft.BACKGROUND
LABEL, SMALL = 19.2, 18.0
ORIGIN = np.array([-3.0, 0.0, 0.0])
SCALE = 0.22
PX, PW = 3.65, 6.3
BAY_X, BAY_Y, BAY_K = PX, 0.85, 0.5
DRIVE_FROM = 0.09
INFERRED = _draft.mix(GOOD, WARN, 0.5)
TONES = {"sourced": GOOD, "assumed": WARN, "inferred": INFERRED, "open": WARN, "derived": MUTED}
WORDS = {"sourced": ("sourced", "belegt"), "assumed": ("assumed", "angenommen"), "inferred": ("inferred", "gefolgert"), "open": ("open", "offen"),
         "derived": ("derived", "abgeleitet")}


def _fade(group, alpha):
    """🌫️ Scale every stroke and fill opacity of a freshly built group by an alpha so partial opacities survive a reveal."""
    if alpha >= 0.999:
        return group
    for mob in group.get_family():
        mob.set_fill(opacity=float(mob.get_fill_opacity()) * alpha)
        mob.set_stroke(opacity=float(mob.get_stroke_opacity()) * alpha)
    return group


class Beat1_DataToInformation(_visuals.ClipScene):
    """📚 Animate data become information on the Villa Savoye ground floor with explicit English and German narration."""

    TITLE = {"en": "Data Become Information", "de": "Aus Daten wird Information"}
    NARRATION = [
        ["raw", "This number is data. Alone, it does not say what was measured.",
         "Diese Zahl ist ein Datenwert. Allein sagt sie nicht, was gemessen wurde."],
        ["guess", "Metres, kilograms, degrees: without a unit it could be anything.",
         "Meter, Kilogramm, Grad: ohne Einheit könnte es alles sein."],
        ["context", "Context: it is the distance between two piloti axes on the ground floor of the Villa Savoye.",
         "Kontext: Es ist der Achsabstand zweier Pilotis im Erdgeschoss der Villa Savoye."],
        ["place", "The model names the place: site, building, storey and the two grid axes.",
         "Das Modell benennt den Ort: Grundstück, Gebäude, Geschoss und die beiden Rasterachsen."],
        ["chain", "Does it add up? The bays sum to the grid width; what remains to the building edge is still open.",
         "Stimmt die Summe? Die Felder ergeben die Rasterbreite; der Rest bis zur Gebäudekante bleibt offen."],
        ["meaning", "The data model adds meaning: property, unit, grid, and whether the value is sourced or assumed.",
         "Das Datenmodell ergänzt die Bedeutung: Eigenschaft, Einheit, Raster und ob der Wert belegt oder angenommen ist."],
        ["source", "A value carries its origin. The first grid was wider and was reduced to save cost.",
         "Ein Wert trägt seine Herkunft. Das erste Raster war weiter und wurde aus Kostengründen verkleinert."],
        ["question", "Information serves a question: can a car (size assumed) drive between the pilotis and turn under the house?",
         "Information dient einer Frage: Fährt ein Auto (Maße angenommen) zwischen den Pilotis durch und wendet unter dem Haus?"],
        ["diameter", "Spacing counts from axis to axis, but the car needs the free space between the piloti surfaces.",
         "Der Abstand gilt von Achse zu Achse; das Auto braucht den freien Raum zwischen den Stützen."],
        ["clear", "The clear width is the spacing minus the piloti diameter. In this model, that is the room the car has.",
         "Die lichte Weite ist der Abstand abzüglich des Pilotidurchmessers. Im Lehrmodell hat das Auto so viel Raum."],
        ["fits", "Driving straight through, the car plus a safety margin on each side must fit into it. With these inputs, it does.",
         "Bei Geradeausfahrt müssen Auto und ein Sicherheitsabstand je Seite hineinpassen. Mit diesen Annahmen tun sie das."],
        ["reduced", "Whether the grid is the first or the reduced one, the car fits.",
         "Ob erstes oder verkleinertes Raster: Das Auto passt."],
        ["gauge", "How sure is the answer? The verdict flips only below a threshold, far under this spacing.",
         "Wie sicher ist die Antwort? Das Urteil kippt erst unter einer Schwelle, weit unter diesem Abstand."],
        ["sensitive", "A thicker piloti or a wider car shifts the threshold, but not enough to change the answer.",
         "Eine dickere Stütze oder ein breiteres Auto verschiebt die Schwelle, ändert die Antwort aber nicht."],
        ["drive", "The car drives through the bay and turns under the house.",
         "Das Auto fährt durch das Feld und wendet unter dem Haus."],
        ["pinch", "On the curve, two pilotis come closer than the straight-driving margin; the car still clears them, but only just.",
         "In der Kurve unterschreiten zwei Pilotis den Abstand der Geradeausfahrt; das Auto kommt noch frei vorbei, aber knapp."],
        ["wall", "The curved glass wall follows the car's turning circle: its curve is calculated from the car.",
         "Die gebogene Glaswand folgt dem Wendekreis des Autos: Ihre Kurve ist aus dem Auto berechnet."],
        ["properties", "A piloti carries more properties: its material, whether it bears load, and whether it is external.",
         "Eine Stütze trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen liegt."],
        ["relations", "Relationships complete the meaning: storey, type, the floor slab it carries and its grid position.",
         "Beziehungen vervollständigen die Bedeutung: Geschoss, Typ, die getragene Geschossdecke und die Rasterposition."],
        ["handoff", "BIM keeps the data in one structured model, so people read the drawing and applications read the data.",
         "BIM hält die Daten in einem strukturierten Modell: Menschen lesen die Zeichnung, Anwendungen die Daten."],
        ["same", "Both describe the same piloti: the GlobalId links the drawn column to its STEP lines.",
         "Beide beschreiben dieselbe Stütze: Die GlobalId verknüpft die gezeichnete Stütze mit ihren STEP-Zeilen."]
    ]

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("information")

    def _put(self, shape, point, edge="c"):
        """📌 Move a shape so its left edge, right edge or centre sits on ``point``."""
        dx = {"l": point[0] - shape.get_left()[0], "r": point[0] - shape.get_right()[0], "c": point[0] - shape.get_center()[0]}[edge]
        return shape.shift(np.array([dx, point[1] - shape.get_center()[1], 0.0]))

    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """🔤 One label aligned on its left edge, right edge or centre."""
        return self._put(self.text(string, (0, 0, 0), size=size, color=color, width=width), point, edge)

    def _tag(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """🆔 A label that sets IFC identifiers (Ifc…, Pset_…, GlobalId) in the bundled monospace face, where capital I and lowercase l differ, and other text in the body face."""
        if not (string.startswith("Ifc") or "Pset_" in string or "GlobalId" in string):
            return self._say(string, point, size=size, color=color, edge=edge, width=width)
        shape = self.mono(string, (0, 0, 0), size=size * 0.9, color=color)
        if width and shape.width > width:
            shape.scale_to_fit_width(width)
        return self._put(shape, point, edge)

    def _at(self, x, y):
        """📍 Screen position of a plan coordinate in metres."""
        return ORIGIN + SCALE * np.array([x - self._mid[0], y - self._mid[1], 0.0])

    def _lab(self, tag):
        """🏷️ Grid name of a piloti tag, E3 becomes E/3."""
        return tag[0] + "/" + tag[1:]

    def _word(self, kind):
        """🧾 Localized basis word of a model value."""
        return self.tr(*WORDS[kind])

    def _chip(self, kind, point, anchor="l", text=None):
        """🔖 Outlined basis chip in the colour of its basis."""
        return self.badge(text or self._word(kind), point, color=TONES[kind], size=SMALL, filled=False, anchor=anchor)

    def _ramp(self, tracker, start, stop, seconds, rate=_engine.smooth):
        """⏩ Move a tracker between two values."""
        return _engine.UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(start + (stop - start) * alpha), rate_func=rate, run_time=seconds)

    def _gone(self, *mobs):
        """🫥 Stop live redraws and fade the mobjects out."""
        for mob in mobs:
            for part in mob.get_family():
                part.clear_updaters()
        return [_engine.FadeOut(mob) for mob in mobs]

    def _xy(self, tag):
        """📍 Plan coordinates of a piloti in metres."""
        item = next(p for p in self._g["pilotis"] if p["tag"] == tag)
        return item["x"], item["y"]

    def _tex(self, value, unit="", digits=2):
        """🔢 A number for typeset formulas with language punctuation and an upright unit."""
        text = f"{abs(value):.{digits}f}"
        if self.language() == "de":
            text = text.replace(".", "{,}")
        return text + (r"\;\mathrm{" + unit + "}" if unit else "")

    def _math(self, parts, size=24, buff=0.12):
        """🧮 Typeset formula row from ``(key, latex, colour)`` parts on one baseline."""
        return _art.math_row(parts, font_size=size, buff=buff)[0]

    def _plan(self, lit=()):
        """📋 Ground-floor plan from the shared villa drawing, the entry pilotis lit."""
        plan = self.villa_plan("ground", ORIGIN, SCALE, model=self._m, highlight=lit, names=False, ramp_arrow=False, label_sides=("bottom", "left"), halo=0.12)
        plan.glass, plan.outline = plan.parts["CW-1"], plan.parts["box"]
        walls = _sheet.ground_walls(self._g)
        poche = _engine.VGroup(*[_draft.polygon([self._at(*p) for p in _plan_widget.quad(p0, p1, walls["thickness"] / 2)], _draft.mix(INK, BACKGROUND, 0.42), 0.95, INK, 0.6)
                                 for p0, p1, _ in walls["outer"] + walls["inner"]])
        plan.add(poche)
        return plan

    def _section_mark(self):
        """✂️ Section cut A–A through the piloti axis: dash-dot line, bold heads, view arrows towards the east and the letter A at both ends."""
        x, _ = self._xy(self._entry[0])
        top, bottom = self._at(x, self._g["box"]["width"] + 0.8), self._at(x, -0.6)
        east = np.array([1.0, 0.0, 0.0])
        mark = _engine.VGroup(_draft.segments(_sheet.dash_dot(bottom, top), INK, _draft.W_THIN, 0.8))
        for end, out in ((top, 1.0), (bottom, -1.0)):
            inward = np.array([0.0, -out * 0.2, 0.0])
            tip = end + east * 0.3
            mark.add(_draft.segments([(end, end + inward)], INK, _draft.W_CUT), _draft.segments([(end, tip)], INK, _draft.W_EDGE),
                     _draft.polygon([tip + east * 0.1, tip - east * 0.02 + np.array([0, 0.06, 0]), tip - east * 0.02 - np.array([0, 0.06, 0])], INK),
                     self._say("A", end + east * 0.5 + np.array([0.0, out * 0.04, 0.0]), size=SMALL, color=INK))
        return mark

    def _plan_dim(self, text, a=None, b=None, color=DATA, offset=0.2, x_b=None):
        """↔️ Dimension between two piloti axes along row E, drawn outside the north façade; ``x_b`` overrides the second end in metres."""
        a, b = a or self._entry[0], b or self._entry[1]
        (xa, ya), (xb, yb) = self._xy(a), self._xy(b)
        return self.dimension(self._at(xa, ya), self._at(xb if x_b is None else x_b, yb), text, offset=offset, color=color, size=LABEL)

    def _norm(self):
        """📖 Standard reference and the one-time teaching-model note, pinned to the top-right corner like the energy clips."""
        ref = _engine.Text("IFC 4.3 · ISO 16739-1:2024", font_size=LABEL - 3, color=_art.P_PRIMARY)
        ref.set_opacity(0.72)
        note = self._say(self.tr("Teaching model · simplified, rounded · orientation schematic", "Lehrmodell · vereinfacht, gerundet · Ausrichtung schematisch"),
                         (0, 0, 0), size=SMALL, color=MUTED)
        group = _engine.VGroup(ref, note).arrange(_engine.DOWN, buff=0.08, aligned_edge=_engine.RIGHT)
        return group.move_to(np.array([6.85 - group.width / 2, 3.62 - group.height / 2 + 0.06, 0.0]))

    def _subtitle(self):
        """📑 Subtitle chip under the title: the example building."""
        return _fonts.section_heading(self.tr("Villa Savoye · Ground floor", "Villa Savoye · Erdgeschoss"))

    def _car(self, scale, heading=0.0, color=DATA):
        """🚘 Top view of the assumed period car (long bonnet, separate wings, running boards), nose along +x before rotation by the heading in degrees."""
        length, width = self._c["car_length"] * scale, self._c["car_width"] * scale
        ink = _draft.mix(color, BACKGROUND, 0.55)
        dark = _draft.mix(color, BACKGROUND, 0.82)
        body = _engine.RoundedRectangle(width=length * 0.96, height=width * 0.64, corner_radius=width * 0.2, stroke_color=color, stroke_width=1.6, fill_color=ink, fill_opacity=0.95)
        bonnet = _engine.RoundedRectangle(width=length * 0.3, height=width * 0.46, corner_radius=width * 0.1, stroke_color=color, stroke_width=1.0, fill_color=dark, fill_opacity=0.9).shift(
            np.array([length * 0.31, 0, 0]))
        cabin = _engine.RoundedRectangle(width=length * 0.26, height=width * 0.54, corner_radius=width * 0.1, stroke_width=0, fill_color=BACKGROUND, fill_opacity=0.85).shift(
            np.array([-length * 0.09, 0, 0]))
        screen = _draft.segments([((length * 0.05, -width * 0.24, 0), (length * 0.05, width * 0.24, 0))], color, 1.4)
        wings = [_engine.RoundedRectangle(width=length * (0.24 if sx > 0 else 0.2), height=width * 0.18, corner_radius=width * 0.08, stroke_color=color, stroke_width=1.2,
                                          fill_color=dark, fill_opacity=1.0).move_to(np.array([sx * length * (0.3 if sx > 0 else 0.3), sy * width * 0.41, 0]))
                 for sx in (-1, 1) for sy in (-1, 1)]
        boards = [_engine.Rectangle(width=length * 0.2, height=width * 0.07, stroke_width=0, fill_color=color, fill_opacity=0.8).move_to(np.array([0.0, sy * width * 0.36, 0]))
                  for sy in (-1, 1)]
        lights = [_engine.Circle(radius=width * 0.06, stroke_width=0, fill_color=INK, fill_opacity=1).move_to(np.array([length * 0.48, sy * width * 0.2, 0])) for sy in (-1, 1)]
        return _engine.VGroup(*boards, body, bonnet, cabin, screen, *wings, *lights).rotate(math.radians(heading), about_point=np.zeros(3))

    def _pose(self, t):
        """🧭 Position and heading of the car at fraction ``t`` of its path, interpolated between model samples."""
        poses = self._path["poses"]
        f = min(max(t, 0.0), 1.0) * (len(poses) - 1)
        i = min(int(f), len(poses) - 2)
        u = f - i
        p, q = poses[i], poses[i + 1]
        return p[0] + (q[0] - p[0]) * u, p[1] + (q[1] - p[1]) * u, p[2] + ((q[2] - p[2] + 180) % 360 - 180) * u

    def _driver(self, t):
        """🛣️ Car on the plan with the area it has swept so far."""
        group = _engine.VGroup()
        half = self._c["car_width"] / 2
        steps = max(int((t - DRIVE_FROM) * 160), 2)
        left, right = [], []
        for k in range(steps + 1):
            x, y, h = self._pose(DRIVE_FROM + (t - DRIVE_FROM) * k / steps)
            n = np.array([-math.sin(math.radians(h)), math.cos(math.radians(h))]) * half
            left.append(self._at(x + n[0], y + n[1]))
            right.append(self._at(x - n[0], y - n[1]))
        group.add(_draft.polygon(left + right[::-1], DATA, 0.14, DATA, _draft.W_HAIR, 0.55))
        centre_points = [self._at(*self._pose(DRIVE_FROM + (t - DRIVE_FROM) * k / steps)[:2]) for k in range(steps + 1)]
        group.add(_draft.polyline(centre_points, INK, _draft.W_THIN, 0.9, dash=0.06))
        for metres in np.arange(5.0, self._path["length"], 5.0):
            fraction = metres / self._path["length"]
            if DRIVE_FROM < fraction <= t:
                x, y, h = self._pose(fraction)
                n = np.array([-math.sin(math.radians(h)), math.cos(math.radians(h))]) * 0.09
                centre = self._at(x, y)
                group.add(_draft.segments([(centre + np.array([n[0], n[1], 0.0]), centre - np.array([n[0], n[1], 0.0]))], INK, _draft.W_EDGE))
        x, y, h = self._pose(t)
        group.add(self._car(SCALE, h).move_to(self._at(x, y)))
        return group

    def _rows(self, rows, point, width, title=None):
        """📋 Table of rows ``{name, value, sub, basis, dot, vtone, ntone, live}``: the frame and title as the result, ``result.rows`` as separate groups."""
        pitch = [0.66 if r.get("sub") or r.get("basis") else 0.42 for r in rows]
        head = 0.46 if title else 0.0
        height = sum(pitch) + 0.2 + head
        cx, cy = point[0], point[1]
        left, right = cx - width / 2 + 0.16, cx + width / 2 - 0.16
        frame = _panels.frame_box(width, height, MUTED).move_to((cx, cy, 0))
        table = _engine.VGroup(frame)
        top = cy + height / 2 - 0.1
        if title:
            table.add(self._say(title, (left, top - 0.2), color=GOOD, edge="l", width=width - 0.3))
            table.add(_draft.segments([((cx - width / 2, top - 0.42, 0), (cx + width / 2, top - 0.42, 0))], GOOD, _draft.W_THIN))
            top -= head - 0.02
        table.rows = []
        table.dots = []
        for spec, step in zip(rows, pitch):
            two = step > 0.5
            y1 = top - (0.18 if two else step / 2)
            y2 = top - 0.5
            row = _engine.VGroup()
            x0 = left
            if spec.get("dot"):
                dot = _engine.Dot((left + 0.08, y1, 0), radius=0.07, color=spec["dot"])
                row.add(dot)
                table.dots.append(dot)
                x0 = left + 0.24
            elif spec.get("num"):
                row.add(self.badge(spec["num"][0], (left + 0.12, y1, 0), color=spec["num"][1], size=SMALL))
                x0 = left + 0.4
            row.add(self._tag(spec["name"], (x0, y1), color=spec.get("ntone", INK), edge="l", width=width * 0.5))
            if spec.get("live"):
                row.add(self.live_label(spec["live"], (right, y1), size=LABEL, color=spec.get("vtone", DATA), anchor="r"))
            elif spec.get("value"):
                row.add(self._tag(spec["value"], (right, y1), color=spec.get("vtone", DATA), edge="r", width=width * 0.52))
            chip = self._say(spec["basis"][0], (right, y2), size=SMALL, color=spec["basis"][1], edge="r", width=width * 0.5) if spec.get("basis") else None
            if chip is not None:
                row.add(chip)
            if spec.get("sub"):
                room = (chip.get_left()[0] - 0.2 if chip is not None else right) - left
                row.add(self._tag(spec["sub"], (left, y2), size=SMALL, color=MUTED, edge="l", width=room))
            table.rows.append(row)
            top -= step
        return table

    def _bay(self):
        """🔍 Magnified top view of the entry bay, redrawn from the spacing, diameter and car-width trackers; every part is faded by its reveal tracker."""
        st = self.st
        s, d, w = st.s.get_value(), st.d.get_value(), st.w.get_value()
        lim = _model.clearance_limits(self._m, spacing=s, diameter=d, car_width=w)
        k, cx, cy = BAY_K, BAY_X, BAY_Y
        xa, xb = cx - s * k / 2, cx + s * k / 2
        r = d * k / 2
        half = self._c["car_length"] * k / 2
        reach = w * k / 2
        a, b = self._entry
        rv = {name: tracker.get_value() for name, tracker in st.rv.items()}
        bay = _engine.VGroup()
        axes = _draft.segments([pair for x in (xa, xb) for pair in _draft.dashes([(x, cy - half - 0.25, 0), (x, cy + half + 0.25, 0)], 0.1, 0.05)], MUTED, _draft.W_HAIR, 0.7)
        pil = _engine.VGroup(*[_engine.Circle(radius=max(r, 0.04), stroke_color=INK, stroke_width=1.4, fill_color=CONCRETE, fill_opacity=1.0).move_to((x, cy, 0)) for x in (xa, xb)],
                             self._say(self._lab(a), (xa - r - 0.18, cy + 0.3), color=DATA, edge="r"), self._say(self._lab(b), (xb + r + 0.18, cy + 0.3), color=DATA, edge="l"))
        span = self.dimension((xa, cy, 0), (xb, cy, 0), self.quantity(s, "m"), offset=half + 0.5, color=DATA, size=LABEL)
        diam = _engine.VGroup(self.dimension((xa - r, cy, 0), (xa + r, cy, 0), None, offset=-0.28, color=WARN),
                              self._say("Ø " + self.quantity(d, "m"), (xa - r - 0.14, cy - 0.28), color=WARN, edge="r"),
                              self._say(self._word("assumed"), (xa - r - 0.14, cy - 0.56), size=SMALL, color=WARN, edge="r"))
        rect = [(xa + r, cy - 0.16), (xb - r, cy - 0.16), (xb - r, cy + 0.16), (xa + r, cy + 0.16)]
        clear = _engine.VGroup(_draft.polygon(rect, GOOD, 0.16), _draft.hatch(rect, GOOD, 0.07, 45, _draft.W_HAIR, 0.9),
                               self.dimension((xa + r, cy, 0), (xb - r, cy, 0), self.quantity(lim["clear_width"], "m"), offset=-(half + 0.55), color=GOOD, size=LABEL))
        keep = _engine.VGroup()
        m_min = lim["min_margin"] * k
        for index, (x0, x1) in enumerate(((xa + r, xa + r + m_min), (xb - r - m_min, xb - r))):
            ring = [(x0, cy - 0.7), (x1, cy - 0.7), (x1, cy + 0.9), (x0, cy + 0.9)]
            left = index == 0
            keep.add(_draft.polygon(ring, WARN, 0.1), _draft.polyline(ring, WARN, _draft.W_HAIR, 0.8, closed=True, dash=0.08),
                     self._say(self.quantity(lim["min_margin"], "m"), (xa - 0.16 if left else xb + 0.16, cy + 0.68), size=SMALL, color=WARN, edge="r" if left else "l"),
                     _draft.segments([((xa - 0.1 if left else xb + 0.1, cy + 0.68, 0), ((x0 + x1) / 2, cy + 0.68, 0))], WARN, _draft.W_HAIR, 0.9))
        car = self._car(k, 270).move_to((cx, cy, 0))
        car.stretch(w / self._c["car_width"], 0)
        car_dim = _engine.VGroup(self.dimension((cx - reach, cy, 0), (cx + reach, cy, 0), None, offset=half + 0.1, color=DATA),
                                 self._say(self.quantity(w, "m"), (cx, cy + half + 0.3), color=DATA))
        fit = lim["fits"]
        tone = GOOD if fit else SERV
        margins = _engine.VGroup()
        if lim["margin"] > 0.3:
            for left, right in ((xa + r, cx - reach), (cx + reach, xb - r)):
                margins.add(self.dimension((left, cy, 0), (right, cy, 0), None, offset=-(half + 0.12), color=DATA),
                            self._say(self.quantity(lim["margin"], "m"), ((left + right) / 2, cy - half - 0.36), size=_draft.S_SMALL, color=DATA))
        clear_row = _engine.VGroup(
            self._math([("s", "s", DATA), (None, "−", None), ("d", "Ø", WARN), (None, "=", None), ("w", "w", GOOD)], size=24).move_to((cx - 1.7, -1.55, 0)),
            self._math([("s", self._tex(s, "m"), DATA), (None, "−", None), ("d", self._tex(d, "m"), WARN), (None, "=", None), ("w", self._tex(lim["clear_width"], "m"), GOOD)],
                       size=22).move_to((cx + 1.2, -1.55, 0)))
        need_row = self._math([("b", self._tex(w, "m"), WARN), (None, "+ 2 ×", None), ("m", self._tex(lim["min_margin"], "m"), WARN), (None, "=", None),
                               ("n", self._tex(lim["required_clear_width"], "m"), WARN), (None, "≤" if fit else ">", None), ("w", self._tex(lim["clear_width"], "m"), tone)], size=19.5, buff=0.1)
        need_row.move_to(np.array([0.62 + need_row.width / 2, -1.95, 0.0]))
        verdict = self.badge(self.tr("fits · assumed", "passt · angenommen") if fit else self.tr("does not fit", "passt nicht"), (0.68, 2.84), color=WARN if fit else SERV,
                             size=_draft.S_SMALL, filled=not fit, anchor="l")
        parts = {"axes": axes, "pil": pil, "span": span, "diam": diam, "clear": clear, "keep": keep, "car": _engine.VGroup(car, car_dim), "margins": margins,
                 "formula": clear_row, "need": need_row, "verdict": verdict}
        for name, part in parts.items():
            bay.add(_fade(part, rv.get(name, 1.0)))
        hint = self._say(self.tr("s: spacing axis to axis · Ø: piloti diameter", "s: Abstand von Achse zu Achse · Ø: Pilotidurchmesser"), (cx, -1.7), size=SMALL, color=MUTED, width=5.6)
        bay.add(_fade(hint, rv["pil"] * (1.0 - rv["clear"])))
        return bay

    def _ruler(self, t):
        """📏 Distance ruler along the model path: a bar that grows with the drive, ticks every 5 m with numerals."""
        length = self._path["length"]
        x0, y, k = 0.95, -1.35, 5.4 / length
        ruler = _engine.VGroup(_draft.polygon([(x0, y - 0.07), (x0 + length * k, y - 0.07), (x0 + length * k, y + 0.07), (x0, y + 0.07)], None, 1.0, MUTED, _draft.W_THIN),
                               _draft.polygon([(x0, y - 0.07), (x0 + t * length * k, y - 0.07), (x0 + t * length * k, y + 0.07), (x0, y + 0.07)], INK, 0.7))
        for metres in range(0, int(length) + 1, 5):
            ruler.add(_draft.segments([((x0 + metres * k, y - 0.07, 0), (x0 + metres * k, y - 0.17, 0))], MUTED, _draft.W_EDGE),
                      self._say(self.number(metres, 0) + (_draft.NARROW + "m" if metres + 5 > length else ""), (x0 + metres * k, y - 0.38), size=_draft.S_SMALL, color=MUTED))
        return ruler

    def _gauge(self):
        """📏 Spacing gauge: the fit zone, the no-fit zone growing in from the moving threshold, the current spacing and the first grid, all from the trackers."""
        st = self.st
        s, d, w = st.s.get_value(), st.d.get_value(), st.w.get_value()
        lim = _model.clearance_limits(self._m, spacing=s, diameter=d, car_width=w)
        first = self._c["initial_spacing"]
        x0, x1, y = 0.95, 6.35, -1.56
        unit = (x1 - x0) / 5.5
        px = lambda v: x0 + v * unit
        reveal, cut_alpha = st.rv["gauge"].get_value(), st.rv["cut"].get_value()
        cut = x0 + (px(lim["min_spacing"]) - x0) * cut_alpha
        bad = [(x0, y - 0.17), (cut, y - 0.17), (cut, y + 0.17), (x0, y + 0.17)]
        good = [(cut, y - 0.17), (x1, y - 0.17), (x1, y + 0.17), (cut, y + 0.17)]
        base = _engine.VGroup(_draft.polygon(good, GOOD, 0.22), _draft.polyline([(x0, y - 0.17), (x1, y - 0.17), (x1, y + 0.17), (x0, y + 0.17)], MUTED, _draft.W_THIN, 0.9, closed=True),
                              self._say(self.tr("fits", "passt"), (x1 - 0.1, y), size=SMALL, color=INK, edge="r"))
        ticks = _engine.VGroup(*[_draft.segments([((px(v), y - 0.17, 0), (px(v), y - 0.27, 0))], MUTED, _draft.W_THIN) for v in range(6)],
                               *[self._say(self.number(v, 0) + (_draft.NARROW + "m" if v == 5 else ""), (px(v), y - 0.42), size=_draft.S_SMALL, color=MUTED) for v in range(6)])
        zone = _engine.VGroup()
        if cut_alpha > 0.02:
            zone.add(_draft.polygon(bad, SERV, 0.3),
                     self._say(self.tr("does not fit", "passt nicht"), (x0 + 0.1, y), size=SMALL, color=INK, edge="l", width=max(cut - x0 - 0.15, 0.3)),
                     _draft.segments([((cut, y - 0.17, 0), (cut, y - 0.29, 0))], WARN, _draft.W_EDGE),
                     self._say(self.tr("threshold ", "Schwelle ") + self.quantity(lim["min_spacing"], "m"), (cut + 0.1, y - 0.7), size=SMALL, color=WARN, edge="r"))
        marker = _engine.VGroup(_draft.polygon([(px(s), y + 0.19), (px(s) - 0.09, y + 0.37), (px(s) + 0.09, y + 0.37)], DATA, 1.0),
                                self._say("s = " + self.quantity(s, "m"), (px(s) - 0.14, y + 0.34), color=DATA, edge="r"))
        ghost = _engine.VGroup(_draft.segments([((px(first), y - 0.17, 0), (px(first), y - 0.29, 0))], MUTED, _draft.W_EDGE),
                               self._say(self.tr("first grid ", "1. Raster ") + self.quantity(first, "m"), (x1 + 0.12, y - 0.7), size=_draft.S_SMALL, color=MUTED, edge="r"))
        gauge = _engine.VGroup(base, ticks, zone, marker, ghost)
        return _fade(gauge, reveal)

    def _para(self, string, point, width, size=SMALL, color=MUTED):
        """📄 Wrapped paragraph of centred lines no wider than ``width``."""
        lines = _fonts.wrap_body_lines(string, font_size=size, max_width=width)
        group = _engine.VGroup(*[self.text(line, (0, 0, 0), size=size, color=color) for line in lines]).arrange(_engine.DOWN, buff=0.08)
        return group.move_to(np.array([point[0], point[1], 0.0]))

    def _nearest(self, item):
        """📍 The point of the car outline closest to a piloti at the car's pose, in model metres."""
        px, py = item["centre"]
        x, y, heading = item["pose"]
        rad = math.radians(heading)
        cs, sn = math.cos(rad), math.sin(rad)
        rel = np.array([px - x, py - y])
        local = np.array([rel[0] * cs + rel[1] * sn, -rel[0] * sn + rel[1] * cs])
        half = np.array([self._c["car_length"], self._c["car_width"]]) / 2
        near = np.clip(local, -half, half)
        return np.array([x, y]) + np.array([near[0] * cs - near[1] * sn, near[0] * sn + near[1] * cs])

    def _away(self, item):
        """🧭 Unit screen direction from the closest car point to the piloti, the side where a marker stays clear of the car."""
        vec = np.array(item["centre"]) - self._nearest(item)
        vec = vec / max(np.linalg.norm(vec), 1e-9)
        return np.array([vec[0], vec[1], 0.0])

    def _vignette(self, item, centre, number, scale=0.42):
        """🔎 Magnified closest approach of the car to one piloti: car outline at its pose, the piloti, and the clearance drawn between them."""
        frame = _panels.frame_box(PW - 0.3, 2.6, MUTED).move_to((centre[0], centre[1], 0))
        px, py = item["centre"]
        x, y, heading = item["pose"]
        offset = np.array([x - px, y - py, 0.0]) * scale
        base = np.array([centre[0] + 0.3, centre[1] - 0.2, 0.0]) - offset * 0.5
        radius = max(item["radius"] * scale, 0.07)
        pil = _engine.Circle(radius=radius, stroke_color=INK, stroke_width=1.4, fill_color=CONCRETE, fill_opacity=1.0).move_to(base)
        car = self._car(scale, heading).move_to(base + offset)
        near = self._nearest(item)
        start = base + np.array([near[0] - px, near[1] - py, 0.0]) * scale
        toward = (start - base) / max(np.linalg.norm(start - base), 1e-9)
        end = base + toward * radius
        side = np.array([-toward[1], toward[0], 0.0])
        gap = _engine.VGroup(_draft.segments([(start, end), (start + side * 0.07, start - side * 0.07), (end + side * 0.07, end - side * 0.07)], WARN, _draft.W_EDGE))
        label = self._say(self.quantity(item["clearance"], "m"), base - toward * 0.6, color=WARN, size=LABEL)
        head = _engine.VGroup(self.badge(str(number), (centre[0] - PW / 2 + 0.4, centre[1] + 1.05, 0), color=WARN),
                              self._say(self._lab(item["tag"]), (centre[0] - PW / 2 + 0.68, centre[1] + 1.05), color=INK, edge="l"),
                              self._say(self.tr("below the ", "unter dem Abstand ") + self.quantity(self._c["min_margin"], "m") + self.tr(" margin", ""), (centre[0] + PW / 2 - 0.3, centre[1] + 1.05),
                                        size=SMALL, color=WARN, edge="r"))
        margin_ring = _draft.polyline(_draft.circle_points(base, (item["radius"] + self._c["min_margin"]) * scale, 48), WARN, _draft.W_THIN, 0.9, closed=True, dash=0.07)
        return _engine.VGroup(frame, margin_ring, car, pil, gap, label, head)

    def _wall_detail(self):
        """🪟 Magnified quarter of the curved glass wall: the car at its turning circle, the swept edges and the radius split into glass radius, clearance and car width."""
        g, c, path = self._g, self._c, self._path
        cx_, cy_ = path["curve_centre"]
        rc, hw = path["centre_radius"], c["car_width"] / 2
        inner, outer = rc - hw, rc + hw
        view = _draft.View("plan", (PX, 0.6, 0), 0.62, centre=(6.0, 14.3, 0))
        pt = lambda x, y: view(x, y, 0.0)
        out = _engine.VGroup(_panels.frame_box(PW, 5.5, MUTED, fill=0.5).move_to((PX, 0.3, 0)))
        points = [p for p in g["curtain_wall"]["path"] if p[0] <= cx_ + 1e-6]
        points[0] = [points[0][0], 11.7]
        out.add(_draft.polyline([pt(*p) for p in points], GLASS, _draft.W_CUT + 0.8, 1.0))
        for radius in (inner, outer):
            out.add(_draft.polyline([pt(cx_ + radius * math.cos(math.radians(an)), cy_ + radius * math.sin(math.radians(an))) for an in np.linspace(90, 180, 40)], DATA, _draft.W_THIN, 0.9, dash=0.09))
        poses = [self._pose(t) for t in np.linspace(0.4, 0.8, 200)]
        pick = min(poses, key=lambda p: abs(math.degrees(math.atan2(p[1] - cy_, p[0] - cx_)) % 360 - 135))
        out.add(self._car(0.62, pick[2]).set_opacity(0.55).move_to(pt(pick[0], pick[1])))
        out.add(_engine.Dot(pt(cx_, cy_), radius=0.06, color=DATA))
        glass_end, inner_end, outer_end = cy_ + c["curve_radius"], cy_ + inner, cy_ + outer
        out.add(_draft.segments([(pt(cx_, cy_), pt(cx_, glass_end))], GLASS, _draft.W_CUT), _draft.segments([(pt(cx_, glass_end), pt(cx_, inner_end))], WARN, _draft.W_CUT + 1.0),
                _draft.segments([(pt(cx_, inner_end), pt(cx_, outer_end))], INK, _draft.W_CUT))
        out.add(self.dimension(pt(cx_, cy_), pt(cx_, outer_end), None, offset=0.3, color=DATA))
        sym = lambda en, de, colour: self._math([("x", self.tr(en, de), colour)], size=22)
        for label, y, side in ((sym("R_{glass}", "R_{Glas}", GLASS), (cy_ + glass_end) / 2, 1), (sym("c", "c", WARN), (glass_end + inner_end) / 2 + 0.15, 1),
                               (sym("b_{car}", "b_{Auto}", INK), (inner_end + outer_end) / 2 + 0.1, 1), (sym("R_{turn}", "R_{Wende}", DATA), cy_ + 1.1, -1)):
            at_pt = pt(cx_, y) + np.array([side * 0.2 if side > 0 else -0.5, 0.0, 0.0])
            label.move_to(at_pt + np.array([label.width / 2 * side, 0.0, 0.0]))
            out.add(label)
        formula = self._math([("g", self.tr("R_{glass}", "R_{Glas}"), GLASS), (None, "=", None), ("t", self.tr("R_{turn}", "R_{Wende}"), DATA), (None, "−", None),
                              ("b", self.tr("b_{car}", "b_{Auto}"), INK), (None, "−", None), ("c", "c", WARN)], size=26).move_to((PX, -2.0, 0))
        chips = _engine.VGroup(self._chip("sourced", (0, 0), text=self.tr("principle sourced", "Prinzip belegt")), self._chip("open", (0, 0), text=self.tr("radii not sourced", "Radien nicht belegt")))
        chips.arrange(_engine.RIGHT, buff=0.3).move_to((PX, -1.35, 0))
        out.add(formula, chips)
        return out

    def _column(self, x0=1.55, base=-1.95, kx=0.7):
        """🏛️ Piloti elevation with the slab above, its circular section and the dimensions, all from the model."""
        a = self._entry[0]
        q = _model.quantities(a, self._m)
        slab = next(s for s in self._g["slabs"] if s["kind"] == "FLOOR")
        w, h, t = q["diameter"] * kx, q["height"] * kx, slab["thickness"] * kx
        shaft = [(x0 - w / 2, base), (x0 + w / 2, base), (x0 + w / 2, base + h), (x0 - w / 2, base + h)]
        part = _engine.VGroup()
        part.shaft = _draft.polygon(shaft, BACKGROUND, 0.7, INK, 1.8)
        part.slab = _draft.polygon([(x0 - 0.62, base + h), (x0 + 0.62, base + h), (x0 + 0.62, base + h + t), (x0 - 0.62, base + h + t)], CONCRETE, 0.55, INK, 1.4)
        part.ground = _draft.segments([((x0 - 0.7, base, 0), (x0 + 0.7, base, 0))], INK, 2.0)
        centre, radius = (x0, 2.35), q["radius"] * 3.4
        part.section = _engine.Circle(radius=radius, stroke_color=INK, stroke_width=1.8, fill_color=BACKGROUND, fill_opacity=0.7).move_to((*centre, 0))
        part.hatch_shaft = _draft.hatch(shaft, STRUCT, 0.05, 45, _draft.W_HAIR, 0.9)
        part.hatch_section = _draft.hatch(_draft.circle_points(centre, radius, 28), STRUCT, 0.07, 45, _draft.W_HAIR, 0.9)
        part.dims = _engine.VGroup(
            self.dimension((centre[0] - radius, centre[1], 0), (centre[0] + radius, centre[1], 0), None, offset=-0.5, color=WARN, size=LABEL),
            self._say("Ø " + self.quantity(q["diameter"], "m"), (centre[0], centre[1] - 0.82), color=WARN),
            self._say(self._word("assumed"), (centre[0], centre[1] - 1.12), size=SMALL, color=WARN),
            self.dimension((x0 - w / 2, base, 0), (x0 - w / 2, base + h, 0), self.quantity(q["height"], "m"), offset=0.32, color=WARN, size=LABEL))
        top = base + h + t
        part.arrows = _engine.VGroup(*[_engine.Arrow((x0 + dx, top + 0.5, 0), (x0 + dx, top + 0.08, 0), buff=0, color=DATA, stroke_width=3, max_tip_length_to_length_ratio=0.3)
                                       for dx in (-0.4, 0.0, 0.4)], _engine.Arrow((x0, base - 0.5, 0), (x0, base - 0.04, 0), buff=0, color=DATA, stroke_width=3, max_tip_length_to_length_ratio=0.3))
        part.exposure = _draft.polyline([(x0 + w / 2 + 0.14, base), (x0 + w / 2 + 0.14, base + h)], MUTED, _draft.W_EDGE, 0.9, dash=0.08)
        part.static = _engine.VGroup(part.shaft, part.slab, part.ground, part.section, part.dims)
        part.base, part.top, part.x0, part.w = base, base + h, x0, w
        return part

    def _pick(self, excerpt, *needles):
        """✂️ The lines of a STEP excerpt that contain one of the needles, with their token spans."""
        keep = [i for i, line in enumerate(excerpt["lines"]) if any(n in line for n in needles)]
        return {"lines": [excerpt["lines"][i] for i in keep],
                "spans": [{**span, "line": keep.index(span["line"])} for span in excerpt["spans"] if span["line"] in keep]}

    def _route(self, points, color, width=_draft.W_THIN, dot=True):
        """➰ An elbow leader through screen points ending in a dot."""
        shape = _engine.VGroup(_draft.polyline(points, color, width, 0.95))
        if dot:
            shape.add(_engine.Dot(points[-1], radius=0.045, color=color))
        return shape

    def present_information(self):
        """🎞️ The whole animation: a lone number, its context, its meaning, a question it answers, and the model around it."""
        lang = self.language()
        m = _model.load_model()
        self._m, self._g, self._c, self._path = m, _model.building_geometry(m), _model.clearance(m), _model.car_path(m)
        g, c, path = self._g, self._c, self._path
        self._entry = path["entry_pair"]
        self._mid = (g["box"]["length"] / 2, g["box"]["width"] / 2)
        a, b = self._entry
        at = self._at
        (xa, ya), (xb, yb) = self._xy(a), self._xy(b)
        gap_x = (xa + xb) / 2
        storey = next(s for s in m["storeys"] if s["id"] == "ground")["name"][lang]
        ground_mark = "±" + self.number(abs(g["storeys"]["ground"]))
        spacing = c["spacing"]
        self.st = st = SimpleNamespace(s=_engine.ValueTracker(spacing), d=_engine.ValueTracker(c["diameter"]), w=_engine.ValueTracker(c["car_width"]),
                                       rv={name: _engine.ValueTracker(0.0) for name in ("axes", "pil", "span", "diam", "clear", "keep", "car", "margins", "formula", "need", "verdict", "gauge", "cut")})
        info = _model.information(a, "grid.spacing", lang, m)

        raw = self._say(self.number(spacing), (0, 0.4), size=110, color=DATA)
        self.step("raw", _engine.FadeIn(raw, scale=0.85), seconds=3.0)

        guesses = _engine.VGroup(*[self.badge(f"{self.number(spacing)} {unit}?", (0, 0, 0), color=MUTED, size=30, filled=False) for unit in ("m", "kg", "kN", "°C")]).arrange(_engine.RIGHT, buff=0.4)
        guesses.move_to((0, -1.2, 0))
        self.step("guess", _engine.LaggedStart(*[_engine.FadeIn(chip, shift=_engine.UP * 0.25) for chip in guesses], lag_ratio=0.45), seconds=4.0)

        plan = self._plan(lit=(a, b))
        dim = self._plan_dim(self.quantity(spacing, "m"))
        dim_lines = _engine.VGroup(*dim[:3])
        axon = self.villa_axon((PX, 1.45, 0), 0.13, model=m, highlight=(a, b), disciplines=("architecture", "structure"))
        y_a, y_b = axon.parts[a].get_center()[1], axon.parts[b].get_center()[1]
        left_x, right_x = axon.get_left()[0] - 0.1, axon.get_right()[0] + 0.1
        axon_labels = _engine.VGroup(
            self.badge(self._lab(a), (left_x, y_a, 0), color=DATA, size=SMALL, anchor="r"), _draft.segments([((left_x, y_a, 0), (axon.parts[a].get_left()[0], y_a, 0))], DATA, _draft.W_THIN),
            self.badge(self._lab(b), (right_x, y_b, 0), color=DATA, size=SMALL, anchor="l"), _draft.segments([((right_x, y_b, 0), (axon.parts[b].get_right()[0], y_b, 0))], DATA, _draft.W_THIN))
        scale_bar = self.scale_bar(at(16.4, 16.4), SCALE, 2.0, color=MUTED, steps=1)
        north = self.north_arrow((-0.3, -1.95, 0), 0.34, color=MUTED)
        subtitle, norm = self._subtitle(), self._norm()
        self.step("context", _engine.AnimationGroup(
            *self._gone(guesses), _engine.FadeIn(plan), _engine.FadeIn(dim_lines), _engine.ReplacementTransform(raw, dim.label),
            _engine.FadeIn(scale_bar), _engine.FadeIn(north), _engine.FadeIn(subtitle), _engine.FadeIn(norm), _engine.FadeIn(axon), _engine.FadeIn(axon_labels)), seconds=5.0)

        places = self._rows([
            {"name": "IfcSite", "value": m["site"]["name"][lang], "ntone": MUTED},
            {"name": "IfcBuilding", "value": m["building"]["name"][lang], "ntone": MUTED},
            {"name": "IfcBuildingStorey", "value": f"{storey} {ground_mark}", "ntone": MUTED},
            {"name": self.tr("Grid axes", "Rasterachsen"), "value": f"{self._lab(a)} – {self._lab(b)}", "ntone": MUTED}], (PX, -1.45), PW - 0.3)
        self.step("place", _engine.LaggedStart(
            _engine.FadeIn(places), *[_engine.FadeIn(r) for r in places.rows],
            _engine.AnimationGroup(_engine.Indicate(plan.parts["label_x:" + a[1:]], color=DATA), _engine.Indicate(plan.parts["label_x:" + b[1:]], color=DATA),
                                   _engine.Indicate(plan.parts["label_y:" + a[0]], color=DATA)), lag_ratio=0.3), seconds=5.0)

        gx, ly = g["grid"]["x"], g["box"]["length"]
        bays = [(gx[i], gx[i + 1]) for i in range(len(gx) - 1)]
        box_basis = m["building"]["box"]["length"]["basis"]
        width_m = g["box"]["width"]
        total = gx[-1] - gx[0]
        remainder = ly - total
        chain = _engine.VGroup(*[self.dimension(at(x0, ya), at(x1, ya), self.quantity(x1 - x0, "m"), offset=0.2, color=DATA, size=LABEL) for x0, x1 in bays])
        strips = _engine.VGroup()
        for x0, x1 in ((0.0, gx[0]), (gx[-1], ly)):
            ring = [at(x0, 0), at(x1, 0), at(x1, width_m), at(x0, width_m)]
            tag = self._say(self.quantity(x1 - x0, "m"), (at((x0 + x1) / 2, 7.1)[0], at(0, 7.1)[1]), size=SMALL, color=WARN).rotate(math.pi / 2)
            strips.add(_engine.VGroup(_draft.polygon(ring, WARN, 0.2), tag))
        running = _engine.ValueTracker(0.0)
        chain_table = self._rows([
            {"name": self.tr("Grid chain", "Rasterkette"), "value": "", "live": lambda: f"{len(bays)} × {self.quantity(spacing, 'm')} = {self.quantity(running.get_value(), 'm')}",
             "basis": (self._word(info["basis"]), TONES[info["basis"]]), "sub": self.tr("row E, axes 1 to 5", "Reihe E, Achsen 1 bis 5")},
            {"name": self.tr("Building box", "Baukörper"), "value": self.quantity(ly, "m"), "basis": (self._word(box_basis), TONES[box_basis]),
             "sub": self.tr("length · medium confidence, one source", "Länge · mittlere Sicherheit, eine Quelle")},
            {"name": self.tr("Remainder", "Rest"), "value": f"2 × {self.quantity(gx[0], 'm')} = {self.quantity(remainder, 'm')}", "vtone": WARN, "basis": (self._word("open"), WARN),
             "sub": self.tr("split assumed · overhang or cantilever?", "Aufteilung angenommen · Überstand oder Kragarm?")}], (PX, 0.75), PW - 0.3,
            title=self.tr("GRID CHAIN · ROW E", "RASTERKETTE · REIHE E"))
        bar_k, bar_left, bar_y = 0.26, 0.95, -1.4
        cuts = [bar_left, bar_left + gx[0] * bar_k, bar_left + (gx[0] + total) * bar_k, bar_left + ly * bar_k]
        totals = _engine.VGroup()
        for (left, right), tone, text in ((cuts[0:2], WARN, self.quantity(gx[0], "m")), (cuts[1:3], GOOD, self.quantity(total, "m")), (cuts[2:4], WARN, self.quantity(gx[0], "m"))):
            ring = [(left, bar_y - 0.16), (right, bar_y - 0.16), (right, bar_y + 0.16), (left, bar_y + 0.16)]
            totals.add(_draft.polygon(ring, tone, 0.22, tone, _draft.W_THIN))
            if tone == WARN:
                totals.add(_draft.hatch(ring, WARN, 0.07, 45, _draft.W_HAIR, 0.8), self._say(text, ((left + right) / 2, bar_y + 0.42), size=SMALL, color=WARN))
            else:
                totals.add(self._say(text, ((left + right) / 2, bar_y), color=GOOD))
        totals.add(self.dimension((cuts[0], bar_y, 0), (cuts[3], bar_y, 0), self.quantity(ly, "m"), offset=-0.45, color=DATA, size=LABEL))
        strip_start = at((gx[-1] + ly) / 2, 12.0)
        strip_link = self._route([strip_start, (0.15, strip_start[1], 0), (0.15, chain_table.rows[2].get_center()[1], 0), chain_table.rows[2].get_left() + np.array([-0.1, 0, 0])], WARN)
        self.step("chain", _engine.AnimationGroup(
            *self._gone(axon, axon_labels, places, *places.rows), _engine.FadeOut(dim_lines), _engine.FadeOut(dim.label),
            _engine.LaggedStart(*[_engine.FadeIn(d_) for d_ in chain], lag_ratio=0.18), _engine.FadeIn(strips), self._ramp(running, 0.0, total, 3.0, _engine.linear),
            _engine.FadeIn(chain_table), *[_engine.FadeIn(r) for r in chain_table.rows], _engine.FadeIn(totals), _engine.FadeIn(strip_link)), seconds=6.0)

        unit_dim = self._plan_dim(self.quantity(spacing, info["unit"]))
        table = self._rows([
            {"name": self.tr("Property", "Eigenschaft"), "value": info["label"], "vtone": INK},
            {"name": self.tr("Value", "Wert"), "value": self.number(info["value"])},
            {"name": self.tr("Unit", "Einheit"), "value": info["unit"]},
            {"name": self.tr("Grid", "Raster"), "value": f"{self._lab(a)} – {self._lab(b)}"},
            {"name": self.tr("Basis", "Herkunft"), "value": self._word(info["basis"]), "vtone": TONES[info["basis"]],
             "sub": self.tr("origin of the value: sourced or assumed", "Herkunft des Werts: belegt oder angenommen")}],
            (PX, 0.7), PW - 0.3, title=self.tr("TEACHING DATA MODEL · QUANTITY", "LEHRDATENMODELL · GRÖSSE"))
        meaning = self._say(info["meaning"], (PX, -1.0), size=SMALL, color=MUTED, width=PW - 0.5)
        sourced = self._chip(info["basis"], (unit_dim.label.get_right()[0] + 0.3, unit_dim.label.get_center()[1]))
        self.step("meaning", _engine.LaggedStart(
            _engine.AnimationGroup(*[_engine.FadeOut(part) for part in (chain, strips, totals, chain_table, *chain_table.rows, strip_link)]),
            _engine.AnimationGroup(_engine.FadeIn(table), _engine.FadeIn(unit_dim), *[_engine.FadeIn(r) for r in table.rows[:2]]),
            _engine.FadeIn(table.rows[2]),
            _engine.AnimationGroup(_engine.FadeIn(table.rows[3]), _engine.Indicate(plan.parts["label_x:" + a[1:]], color=DATA), _engine.Indicate(plan.parts["label_x:" + b[1:]], color=DATA),
                                   _engine.Indicate(plan.parts["label_y:" + a[0]], color=DATA)),
            _engine.AnimationGroup(_engine.FadeIn(table.rows[4]), _engine.FadeIn(sourced)), _engine.FadeIn(meaning), lag_ratio=0.3), seconds=6.0)
        dim, dim_lines = unit_dim, _engine.VGroup(*unit_dim[:3])

        first_x = xa + c["initial_spacing"]
        first_s = _engine.ValueTracker(c["initial_spacing"])
        moving = _engine.always_redraw(lambda: self._plan_dim(self.quantity(first_s.get_value(), "m"), x_b=xa + first_s.get_value(), color=DATA))
        ghost_dim = self._plan_dim(self.quantity(c["initial_spacing"], "m"), x_b=first_x, color=MUTED, offset=0.8)
        provenance = self._rows([
            {"name": "EN Wikipedia", "value": self.tr("grid reduction", "Rasterverkleinerung"), "dot": GOOD},
            {"name": "Cité de l'architecture", "value": self.tr("sheet (snippet)", "Blatt (Ausschnitt)"), "dot": MUTED, "ntone": MUTED},
            {"name": "Designing Buildings", "value": self.tr("wiki (snippet)", "Wiki (Ausschnitt)"), "dot": MUTED, "ntone": MUTED}], (PX, 1.5), PW - 0.3,
            title=self.tr("PROVENANCE · GRID SPACING", "HERKUNFT · RASTERABSTAND"))
        bar_k, bar_x = 1.0, 0.9
        bars = []
        for index, (value, text, tone) in enumerate(((c["initial_spacing"], self.tr("first grid", "erstes Raster"), MUTED),
                                                     (spacing, self.tr("reduced for cost", "aus Kostengründen verkleinert"), GOOD))):
            y = -0.35 - index * 0.9
            bars.append(_engine.VGroup(
                _draft.polygon([(bar_x, y - 0.15), (bar_x + value * bar_k, y - 0.15), (bar_x + value * bar_k, y + 0.15), (bar_x, y + 0.15)], tone, 0.45, tone, _draft.W_THIN),
                self._say(self.quantity(value, "m"), (bar_x + value * bar_k - 0.12, y), size=SMALL, color=INK, edge="r"),
                self._say(text, (bar_x + 0.1, y - 0.35), size=SMALL, color=tone, edge="l", width=4.0)))
        self.step("source", _engine.AnimationGroup(
            *self._gone(table, *table.rows, meaning, sourced, *dim[:3]), _engine.FadeOut(unit_dim.label),
            _engine.FadeIn(ghost_dim), _engine.FadeIn(provenance), *[_engine.FadeIn(r) for r in provenance.rows],
            _engine.Succession(_engine.AnimationGroup(_engine.FadeIn(moving), _engine.FadeIn(bars[0])),
                               _engine.AnimationGroup(self._ramp(first_s, c["initial_spacing"], spacing, 2.0), _engine.FadeIn(bars[1])))), seconds=6.5)

        turn = _draft.polyline(_draft.circle_points(at(*g["curtain_wall"]["centre"]), SCALE * c["turning_radius"], 72), DATA, _draft.W_THIN, 0.8, closed=True, dash=0.1)
        headline = self._say(self.tr("Does it fit through?", "Passt es hindurch?"), (PX, 2.75), size=30, width=PW - 0.4)
        car_scale, car_at = 0.62, np.array([PX - 0.2, 1.5, 0.0])
        big_car = self._car(car_scale).move_to(car_at)
        car_l, car_w = c["car_length"] * car_scale, c["car_width"] * car_scale
        car_dims = _engine.VGroup(
            self.dimension((car_at[0] + car_l / 2 + 0.05, car_at[1] - car_w / 2, 0), (car_at[0] + car_l / 2 + 0.05, car_at[1] + car_w / 2, 0), self.quantity(c["car_width"], "m"), offset=-0.25, size=LABEL),
            self.dimension((car_at[0] - car_l / 2, car_at[1] - car_w / 2 - 0.05, 0), (car_at[0] + car_l / 2, car_at[1] - car_w / 2 - 0.05, 0), self.quantity(c["car_length"], "m"), offset=-0.25, size=LABEL))
        car_note = self._say(self.tr("A car · generic body, size assumed", "Ein Auto · Karosserie schematisch, Maße angenommen"), (PX, 0.12), size=SMALL, color=MUTED, width=PW - 0.4)
        width_info = _model.information(a, "car.width", lang, m)
        car_table = self._rows([
            {"name": width_info["label"], "value": self.quantity(width_info["value"], width_info["unit"]), "basis": (self._word(width_info["basis"]), WARN)},
            {"name": self.tr("Car length", "Autolänge"), "value": self.quantity(c["car_length"], "m"), "basis": (self._word("assumed"), WARN)},
            {"name": self.tr("Turning circle", "Wendekreis"), "value": self.tr("from the car", "aus dem Auto"), "vtone": INK, "dot": DATA,
             "sub": self.tr("principle sourced · radius not sourced", "Prinzip belegt · Radius nicht belegt"), "basis": (self._word("open"), WARN)}], (PX, -1.3), PW - 0.3)
        question = self._say("?", (at(gap_x, ya)[0], at(gap_x, ya)[1] - 0.32), size=40, color=DATA)
        self.step("question", _engine.LaggedStart(
            _engine.AnimationGroup(*[_engine.FadeOut(part) for part in (provenance, *provenance.rows, *bars, ghost_dim)]),
            _engine.FadeIn(headline), _engine.FadeIn(big_car), _engine.FadeIn(car_dims), _engine.FadeIn(car_note), _engine.FadeIn(car_table),
            _engine.AnimationGroup(*[_engine.FadeIn(r) for r in car_table.rows]),
            _engine.AnimationGroup(_engine.Indicate(plan.glass, color=GLASS, scale_factor=1.0), _engine.FadeIn(turn)),
            _engine.FadeIn(question), lag_ratio=0.14), seconds=8.0)

        frame = _panels.frame_box(PW, 5.5, MUTED, fill=0.5).move_to((PX, 0.3, 0))
        bracket = _draft.polyline([at(xa - 0.65, ya - 1.2), at(xb + 0.65, ya - 1.2), at(xb + 0.65, ya + 0.5), at(xa - 0.65, ya + 0.5)], DATA, _draft.W_THIN, 0.9, closed=True, dash=0.1)
        top_right = at(xb + 0.65, ya + 0.5)
        link = _draft.polyline([top_right, np.array([0.5, top_right[1], 0.0])], DATA, _draft.W_THIN, 0.7, dash=0.1)
        bay = self.live(lambda: self._bay(), key=lambda: (round(st.s.get_value(), 4), round(st.d.get_value(), 4), round(st.w.get_value(), 4),
                                                         tuple(round(v.get_value(), 3) for v in st.rv.values())))
        reveal = lambda *names: _engine.AnimationGroup(*[self._ramp(st.rv[n], 0.0, 1.0, 1.4) for n in names])
        self.step("diameter", _engine.AnimationGroup(
            *self._gone(headline, big_car, car_dims, car_note, car_table, *car_table.rows, question), _engine.FadeIn(frame), _engine.FadeIn(bracket), _engine.FadeIn(link),
            _engine.FadeIn(bay), reveal("axes", "pil", "span", "diam")), seconds=6.0)
        self.step("clear", reveal("clear", "formula"), seconds=5.5)
        self.step("fits", reveal("car", "margins", "keep", "need", "verdict"), seconds=6.0)

        first = self.badge(self.tr("first grid", "erstes Raster"), (0.68, -1.2), color=MUTED, size=SMALL, filled=False, anchor="l")
        reduced = self.badge(self.tr("reduced for cost", "aus Kostengründen"), (0.68, -1.2), color=GOOD, size=SMALL, filled=False, anchor="l")
        gauge = self.live(lambda: self._gauge(), key=lambda: (round(st.s.get_value(), 3), round(st.d.get_value(), 3), round(st.w.get_value(), 3),
                                                              round(st.rv["gauge"].get_value(), 3), round(st.rv["cut"].get_value(), 3)))
        self.add(gauge)
        self.step("reduced", _engine.Succession(
            _engine.AnimationGroup(_engine.FadeIn(first), self._ramp(st.rv["formula"], 1.0, 0.0, 0.8), self._ramp(st.rv["need"], 1.0, 0.0, 0.8),
                                   self._ramp(st.rv["verdict"], 1.0, 0.0, 0.8), self._ramp(st.rv["gauge"], 0.0, 1.0, 0.8)),
            self._ramp(st.s, spacing, c["initial_spacing"], 1.5), _engine.AnimationGroup(_engine.FadeOut(first, run_time=0.4), _engine.FadeIn(reduced, run_time=0.4)),
            self._ramp(st.s, c["initial_spacing"], spacing, 2.0)), seconds=6.0)

        limits = _model.clearance_limits(m)
        low = limits["min_spacing"] - 0.35
        self.step("gauge", _engine.Succession(
            _engine.AnimationGroup(_engine.FadeOut(reduced), self._ramp(st.rv["cut"], 0.0, 1.0, 1.2)), self._ramp(st.s, spacing, low, 3.0), self._ramp(st.s, low, spacing, 2.5)), seconds=7.0)

        weak = _model.clearance_limits(m, diameter=0.6, car_width=2.05)
        weak_chip = self.badge(self.tr("Ø: weak source", "Ø: schwache Quelle"), (0.68, -1.2), color=WARN, size=SMALL, filled=False, anchor="l")
        self.step("sensitive", _engine.AnimationGroup(_engine.FadeIn(weak_chip), self._ramp(st.d, c["diameter"], weak["diameter"], 3.0), self._ramp(st.w, c["car_width"], weak["car_width"], 3.0)),
                  seconds=6.0)

        drive_t = _engine.ValueTracker(DRIVE_FROM)
        mover = _engine.always_redraw(lambda: self._driver(drive_t.get_value()))
        legend = self._rows([
            {"name": self.tr("Swept path", "Fahrspur"), "value": "", "sub": self.tr("area the car's outline covers", "Fläche, die der Autoumriss überstreicht"), "dot": DATA},
            {"name": self.tr("Turning circle", "Wendekreis"), "value": "", "sub": self.tr("outer edge of the swept path", "Außenrand der Fahrspur"), "dot": DATA},
            {"name": self.tr("Curved glass wall", "Gebogene Glaswand"), "value": "", "sub": self.tr("derived from the turning circle", "aus dem Wendekreis abgeleitet"), "dot": GLASS},
            {"name": self.tr("Distance driven", "Gefahrene Strecke"), "value": "", "dot": INK,
             "live": lambda: self.quantity(drive_t.get_value() * path["length"], "m", 1)}], (PX, 1.2), PW - 0.3, title=self.tr("DRIVING THE CAR", "FAHRT DES AUTOS"))
        ruler = _engine.always_redraw(lambda: self._ruler(drive_t.get_value()))
        self.add(mover, ruler)
        self.step("drive", _engine.AnimationGroup(
            *self._gone(bay, gauge, frame, bracket, link, moving), _engine.FadeOut(weak_chip), _engine.FadeIn(legend), *[_engine.FadeIn(r) for r in legend.rows],
            self._ramp(drive_t, DRIVE_FROM, 1.0, 6.0, _engine.linear)), seconds=6.5)

        pinches = sorted(_model.path_pinches(m, 2), key=lambda item: item["progress"])
        rings = _engine.VGroup(*[_engine.VGroup(_draft.polyline(_draft.circle_points(at(*item["centre"]), 0.16, 30), WARN, _draft.W_EDGE, 1.0, closed=True),
                                                self.badge(str(index + 1), at(*item["centre"]) + self._away(item) * 0.36, color=WARN, size=SMALL))
                                 for index, item in enumerate(pinches)])
        ghosts = _engine.VGroup(*[self._car(SCALE, item["pose"][2], WARN).set_opacity(0.4).move_to(at(item["pose"][0], item["pose"][1])) for item in pinches])
        vignettes = [self._vignette(item, (PX, 1.55 - 2.75 * index), index + 1) for index, item in enumerate(pinches)]
        self.step("pinch", _engine.AnimationGroup(
            *self._gone(legend, *legend.rows, ruler), _engine.FadeIn(rings), _engine.FadeIn(ghosts), *[_engine.FadeIn(v) for v in vignettes]), seconds=6.0)

        detail = self._wall_detail()
        self.step("wall", _engine.AnimationGroup(
            *[_engine.FadeOut(part) for part in (*vignettes, rings, ghosts)], *self._gone(mover), _engine.FadeIn(detail)), seconds=7.0)

        column = self._column(x0=0.8)
        info_material = _model.information(a, "material", lang, m)
        info_load = _model.information(a, "Pset_ColumnCommon.LoadBearing", lang, m)
        info_external = _model.information(a, "Pset_ColumnCommon.IsExternal", lang, m)
        truth = lambda value: self.tr("TRUE", "WAHR") if value else self.tr("FALSE", "FALSCH")
        props = self._rows([
            {"name": self.tr("Class", "Klasse"), "value": "IfcColumn", "sub": "PredefinedType · " + _model.element(a, m)["predefined_type"], "num": ("1", ARCH), "vtone": ARCH},
            {"name": info_material["label"], "value": info_material["value"], "vtone": INK, "sub": "IfcMaterial", "num": ("2", STRUCT), "basis": (self._word(info_material["basis"]), TONES[info_material["basis"]])},
            {"name": info_load["label"], "value": truth(info_load["value"]), "vtone": GOOD, "sub": "Pset_ColumnCommon.LoadBearing", "num": ("3", DATA),
             "basis": (self._word(info_load["basis"]), TONES[info_load["basis"]])},
            {"name": info_external["label"], "value": truth(info_external["value"]), "vtone": GOOD, "sub": "Pset_ColumnCommon.IsExternal", "num": ("4", MUTED),
             "basis": (self._word(info_external["basis"]), TONES[info_external["basis"]])}], (4.65, 0.95), 4.6)
        slab_t = next(s_["thickness"] for s_ in g["slabs"] if s_["kind"] == "FLOOR")
        caveat = _engine.VGroup(
            self._say(self.tr("teaching model, this piloti only", "Lehrmodell, nur diese Stütze"), (4.65, -0.65), size=SMALL, color=MUTED, width=4.3),
            self._say(f"{self.quantity(_model.quantities(a, m)['height'], 'm')} = {self.quantity(g['storeys']['first'], 'm')} − {self.quantity(slab_t, 'm')} · {self._word('assumed')}", (4.65, -1.05),
                      size=SMALL, color=WARN, width=4.1))
        marks = _engine.VGroup(
            self.badge("1", (column.section.get_right()[0] + 0.3, 2.35, 0), color=ARCH),
            self.badge("2", (column.x0 + column.w / 2 + 0.3, column.base + 1.0, 0), color=STRUCT),
            self.badge("3", (column.x0 + 1.05, column.top + 0.95, 0), color=DATA),
            self.badge("4", (column.x0 + column.w / 2 + 0.42, column.base + 1.7, 0), color=MUTED))
        level_up = self._say("+" + self.quantity(self._g["storeys"]["first"], "m"), (column.x0 + 0.66, column.top + 0.28), size=_draft.S_SMALL, color=WARN, edge="l")
        level_down = self._say("±" + self.quantity(0, "m"), (column.x0 + 0.66, column.base + 0.18), size=_draft.S_SMALL, color=MUTED, edge="l")
        section_mark = self._section_mark()
        section_label = self._say("A–A", (column.x0 + 0.62, column.base - 0.3), size=SMALL, color=INK, edge="l")
        self.step("properties", _engine.LaggedStart(
            _engine.AnimationGroup(*self._gone(detail), _engine.FadeIn(column.static), _engine.FadeIn(level_up), _engine.FadeIn(level_down), _engine.FadeIn(section_mark), _engine.FadeIn(section_label)),
            _engine.AnimationGroup(_engine.FadeIn(props), _engine.FadeIn(props.rows[0]), _engine.FadeIn(marks[0])),
            _engine.AnimationGroup(_engine.FadeIn(props.rows[1]), _engine.FadeIn(column.hatch_shaft), _engine.FadeIn(column.hatch_section), _engine.FadeIn(marks[1])),
            _engine.AnimationGroup(_engine.FadeIn(props.rows[2]), _engine.LaggedStart(*[_engine.GrowArrow(arrow) for arrow in column.arrows], lag_ratio=0.2), _engine.FadeIn(marks[2])),
            _engine.AnimationGroup(_engine.FadeIn(props.rows[3]), _engine.FadeIn(marks[3]), _engine.FadeIn(column.exposure)), _engine.FadeIn(caveat), lag_ratio=0.25), seconds=7.0)

        rels = {r["class"]: r for r in _model.relations_of(a, m)}
        slab = next(s_ for s_ in g["slabs"] if s_["kind"] == "FLOOR")
        type_name = _model.element("TY-PILOTI", m)["name"]["en"]
        tone = {"storey": GOOD, "type": STRUCT, "slab": CONCRETE, "grid": DATA}
        relations = self._rows([
            {"name": self.tr("Contained in", "Enthalten in"), "value": f"{storey} {ground_mark}", "dot": tone["storey"], "sub": rels["IfcRelContainedInSpatialStructure"]["class"]},
            {"name": self.tr("Defined by type", "Typ"), "value": type_name, "dot": tone["type"], "sub": rels["IfcRelDefinesByType"]["class"],
             "basis": (f"{len(_model.type_occurrences('TY-PILOTI', m))} " + self.tr("in this teaching model*", "in diesem Lehrmodell*"), WARN)},
            {"name": self.tr("Connected to", "Verbunden mit"), "value": self.tr("First-floor slab", "Decke über dem Erdgeschoss"), "dot": tone["slab"],
             "sub": rels["IfcRelConnectsElements"]["class"], "basis": ("+" + self.quantity(slab["top"], "m") + " · " + self._word("assumed"), WARN)},
            {"name": self.tr("Grid position", "Rasterposition"), "value": self._lab(a), "dot": tone["grid"],
             "sub": self.tr("grid axes ", "Rasterachsen ") + a[0] + self.tr(" and ", " und ") + a[1:]}], (PX, 1.0), PW - 0.4)
        legend_count = self._say(self.tr("* sources differ: 15–25", "* Quellen weichen ab: 15–25"), (PX, -0.78), size=SMALL, color=MUTED)
        outline = [at(0, 0), at(ly, 0), at(ly, g["box"]["width"]), at(0, g["box"]["width"])]
        storey_ring = _draft.polyline(outline, tone["storey"], _draft.W_EDGE + 0.6, 1.0, closed=True)
        type_rings = _engine.VGroup(*[_draft.polyline(_draft.circle_points(at(*self._xy(t)), 0.1, 24), tone["type"], _draft.W_EDGE, 1.0, closed=True) for t in (a, b)])
        head_a = at(*self._xy(a))
        connector = _engine.VGroup(_draft.polygon([head_a + np.array([sx * 0.07, sy * 0.07, 0.0]) for sx, sy in ((-1, -1), (1, -1), (1, 1), (-1, 1))], tone["slab"], 1.0, INK, _draft.W_THIN))
        slab_rings = [[at(*p) for p in slab["outline"]]] + [[at(*p) for p in void] for void in slab["voids"]]
        slab_shape = _engine.VGroup(_draft.region(slab_rings, CONCRETE, 0.08), _draft.hatch(slab_rings, CONCRETE, 0.12, 45, _draft.W_HAIR, 0.4))
        axes = (plan.parts["axis_x:" + a[1:]], plan.parts["axis_y:" + a[0]])
        leaders = [
            self._route([relations.dots[0].get_center(), at(ly, 18.9)], tone["storey"]),
            self._route([relations.dots[1].get_center(), at(*self._xy(b)) + np.array([0.0, -0.1, 0.0])], tone["type"]),
            self._route([relations.dots[2].get_center(), head_a + np.array([0.07, -0.07, 0.0])], tone["slab"]),
            self._route([relations.dots[3].get_center(), plan.parts["label_x:" + a[1:]].get_center() + np.array([0.0, 0.13, 0.0])], tone["grid"])]
        self.step("relations", _engine.LaggedStart(
            _engine.AnimationGroup(*self._gone(column.static), _engine.FadeOut(scale_bar), *[_engine.FadeOut(part) for part in (column.hatch_shaft, column.hatch_section, column.arrows, column.exposure, props, caveat, marks, level_up, level_down, section_mark, section_label, *props.rows)]),
            _engine.AnimationGroup(_engine.FadeIn(relations), _engine.FadeIn(relations.rows[0]), _engine.FadeIn(storey_ring), _engine.FadeIn(leaders[0])),
            _engine.AnimationGroup(_engine.FadeIn(relations.rows[1]), _engine.FadeIn(legend_count), _engine.FadeIn(type_rings), _engine.FadeIn(leaders[1])),
            _engine.AnimationGroup(_engine.FadeIn(relations.rows[2]), _engine.FadeIn(slab_shape), _engine.FadeIn(connector), _engine.FadeIn(leaders[2])),
            _engine.AnimationGroup(_engine.FadeIn(relations.rows[3]), *[axis.animate.set_stroke(DATA, 2.4, 1.0) for axis in axes], _engine.FadeIn(leaders[3]),
                                   _engine.Indicate(plan.parts["label_x:" + a[1:]], color=DATA), _engine.Indicate(plan.parts["label_y:" + a[0]], color=DATA)), lag_ratio=0.3), seconds=8.0)

        excerpt = self._pick(_model.step_excerpt(a, m, language="en"), "=IFCCOLUMN(", "'LoadBearing'", "'Pset_ColumnCommon'", "=IFCRELDEFINESBYPROPERTIES(")
        panel = self.step_panel(excerpt, (3.75, 0.0), width=6.0)
        for run in panel.runs:
            if run.role in ("reference", "number", None):
                run.set_color(_draft.mix(MUTED, INK, 0.15))
        apps = self.badge(self.tr("Applications read the data", "Anwendungen lesen die Daten"), (3.75, panel.get_top()[1] + 0.55), color=STRUCT, size=LABEL, filled=False)
        people = self.badge(self.tr("People read the drawing", "Menschen lesen die Zeichnung"), (3.75, apps.get_center()[1] + 0.55), color=ARCH, size=LABEL, filled=False)
        ring = at(*self._xy(a))
        people_route = self._route([people.get_left() + np.array([-0.05, 0.0, 0.0]), (ring[0] - 0.085, people.get_center()[1], 0), (ring[0] - 0.085, ring[1] + 0.085, 0)], ARCH, _draft.W_EDGE)
        apps_arrow = _engine.Arrow(apps.get_bottom(), panel.get_top() + np.array([0, 0.03, 0]), buff=0.04, color=STRUCT, stroke_width=3, max_tip_length_to_length_ratio=0.5)
        self.step("handoff", _engine.LaggedStart(
            _engine.AnimationGroup(*[_engine.FadeOut(part) for part in (relations, *relations.rows, legend_count, storey_ring, type_rings, slab_shape, connector, *leaders)],
                                   *[axis.animate.set_stroke(MUTED, _draft.W_HAIR, 0.5) for axis in axes]),
            _engine.AnimationGroup(_engine.FadeIn(people), _engine.FadeIn(people_route), _engine.Indicate(plan.parts[a], color=DATA)),
            _engine.AnimationGroup(_engine.FadeIn(apps), _engine.FadeIn(apps_arrow)), _engine.FadeIn(panel), lag_ratio=0.35), seconds=7.0)

        chip = self.guid_chip(a, (2.4, panel.get_bottom()[1] - 0.6))
        head = chip.head
        chip.remove(head)
        chip.add(self.mono("GlobalId  " + a, (0, 0, 0), size=_draft.S_SMALL, color=MUTED).move_to(head.get_left(), aligned_edge=_engine.LEFT))
        same = _engine.VGroup(self._say(self.tr("same piloti", "dieselbe Stütze"), (0, 0), size=SMALL, color=MUTED, edge="l"),
                              _engine.VGroup(self._say(self.tr("same ", "dieselbe "), (0, 0), size=SMALL, color=MUTED, edge="l"),
                                             self.mono("GlobalId", (0, 0, 0), size=_draft.S_SMALL, color=MUTED)).arrange(_engine.RIGHT, buff=0.08)).arrange(_engine.DOWN, buff=0.1, aligned_edge=_engine.LEFT)
        same.move_to(np.array([chip.get_right()[0] + 0.25 + same.width / 2, chip.get_center()[1], 0.0]))
        left_edge = panel.frame.get_left()[0]
        class_token, load_token = panel.token("=IFCCOLUMN(", "class"), panel.token("'LoadBearing'", "property")
        guid_token = panel.token("=IFCCOLUMN(", "guid")
        marks_step = _engine.VGroup(*[_draft.segments([(tok.get_corner(_engine.DL) + np.array([-0.02, -0.07, 0]), tok.get_corner(_engine.DR) + np.array([0.02, -0.07, 0]))], DATA, _draft.W_EDGE + 0.6)
                                      for tok in (class_token, guid_token, load_token)])
        trunk_x = left_edge - 0.35
        trunk = _engine.VGroup(_draft.polyline([ring + np.array([0.085, 0.085, 0.0]), (ring[0] + 0.085, 2.34, 0), (trunk_x, 2.34, 0), (trunk_x, chip.get_center()[1], 0), (chip.get_left()[0] - 0.02, chip.get_center()[1], 0)],
                                               DATA, _draft.W_EDGE, 0.95),
                               _engine.Dot((chip.get_left()[0] - 0.02, chip.get_center()[1], 0), radius=0.045, color=DATA))
        taps = _engine.VGroup(*[self._route([(trunk_x, y, 0), (left_edge + 0.02, y, 0)], DATA) for y in (class_token.get_center()[1], load_token.get_center()[1])])
        self.step("same", _engine.LaggedStart(
            _engine.FadeIn(trunk), _engine.FadeIn(taps), _engine.FadeIn(marks_step), _engine.FadeIn(chip), _engine.FadeIn(same),
            _engine.Indicate(plan.parts[a], color=DATA), lag_ratio=0.3), seconds=6.0)
