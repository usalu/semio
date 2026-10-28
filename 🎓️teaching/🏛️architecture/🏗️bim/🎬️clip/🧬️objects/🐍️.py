"""🎬 From Schema to Object: the IFC class tree, one piloti, its type and one façade wall of the Villa Savoye teaching model."""

import copy
import importlib
import math
import sys
from collections import Counter
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_panels = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧾️panels.🐍️")
_sheet = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN, GLASS = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "SERV", "DATA", "GOOD", "WARN", "GLASS"))
CONCRETE, GREEN_GROUND = _visuals.CONCRETE, _visuals.GREEN_GROUND
S_SMALL, S_LABEL, S_BODY, W_THIN, W_EDGE, W_HAIR = _drafting.S_SMALL, _drafting.S_LABEL, _drafting.S_BODY, _drafting.W_THIN, _drafting.W_EDGE, _drafting.W_HAIR
mix, place, segments, polygon, polyline, P, BACKGROUND = _drafting.mix, _drafting.place, _drafting.segments, _drafting.polygon, _drafting.polyline, _drafting.P, _drafting.BACKGROUND
frame_box = _panels.frame_box

SELECTED = "B3"
WALL = "W-S"
LEFT = -4.7
LW = 4.1
STAGE = (2.35, -0.1, 0)
AXON_SCALE = 0.18
AXON_POINT = (2.35, 0.1, 0)
XRAY = ("walls", "windows", "screens")
SLAB = mix(SERV, INK, 0.35)
LABEL = mix(MUTED, INK, 0.5)
CLASS_COLOR = {"IfcColumn": STRUCT, "IfcSlab": SLAB, "IfcWall": ARCH, "IfcWindow": GOOD}
CLASS_NOUN = {"IfcColumn": ("pilotis", "Pilotis"), "IfcSlab": ("slabs", "Decken"), "IfcWall": ("walls", "Wände"), "IfcWindow": ("windows", "Fenster")}
LAYER_COLOR = {"render": mix(INK, BACKGROUND, 0.9), "brick": mix(WARN, BACKGROUND, 0.5), "plaster": mix(INK, BACKGROUND, 0.62)}
BIG = SimpleNamespace(scale=0.92, enlarge=5, ex=-0.3, base=-2.1, cx=2.3, cy=-0.2, elevation=True, ghost=False, labels=True, volume=True, offset=0.5)
CHANGE = SimpleNamespace(scale=0.5, enlarge=4, ex=0.0, base=0.0, cx=4.85, cy=1.7, elevation=False, ghost=True, labels=False, volume=False, offset=0.5)
MINI = SimpleNamespace(scale=0.5, enlarge=3, ex=-4.8, base=-2.25, cx=-2.9, cy=-1.4, elevation=True, ghost=False, labels=False, volume=False, offset=0.35)
TOTAL = SimpleNamespace(scale=0.17, point=(1.5, -0.2, 0))


def clamp(value: float) -> float:
    """📎 Limit a value to the unit interval."""
    return min(max(value, 0.0), 1.0)


class Beat2_ObjectOrientation(_visuals.ClipScene):
    """📚 Animate from schema to object with explicit English and German narration."""

    TITLE = {"en": "From Schema to Object", "de": "Vom Schema zum Objekt"}
    NARRATION = [
        ["schema",
         "A schema defines vocabulary and rules for objects and relationships. For buildings, that schema is IFC 4.3.",
         "Ein Schema definiert Begriffe und Regeln für Objekte und Beziehungen. Für Gebäude ist das Schema IFC 4.3."],
        ["class",
         "A class describes a kind of thing: IfcColumn, IfcSlab, IfcWall and IfcWindow are kinds of built element.",
         "Eine Klasse beschreibt eine Art von Dingen: IfcColumn, IfcSlab, IfcWall und IfcWindow sind Arten von Bauteilen."],
        ["instance",
         "An instance is one particular object: this piloti of the Villa Savoye, with its own GlobalId.",
         "Eine Instanz ist ein bestimmtes Objekt: diese Stütze der Villa Savoye mit eigener GlobalId."],
        ["occurrences",
         "A type holds what all pilotis share; every piloti placed in the building is an occurrence of it.",
         "Ein Typ hält, was alle Pilotis gemeinsam haben; jede platzierte Stütze ist ein Vorkommen davon."],
        ["pset",
         "A property set adds named values to the type: LoadBearing says that each piloti carries its share of the floor.",
         "Ein Property Set ergänzt benannte Werte am Typ: LoadBearing sagt, dass jede Stütze ihren Anteil der Decke trägt."],
        ["representations",
         "One object can have several representations: its axis, its footprint and its body.",
         "Ein Objekt kann mehrere Darstellungen haben: Achse, Grundfläche und Körper."],
        ["geometry",
         "The body is a circle extruded along its axis: the radius and the height are values, drawn as dimension lines.",
         "Der Körper ist ein entlang der Achse extrudierter Kreis: Radius und Höhe sind Werte, gezeichnet als Maßlinien."],
        ["methods",
         "A method is behaviour: the application computes the cross-section and the volume from these values.",
         "Eine Methode ist Verhalten: Die Anwendung berechnet Querschnitt und Volumen aus diesen Werten."],
        ["total",
         "The same method runs for every occurrence and adds up the concrete of all pilotis.",
         "Dieselbe Methode wird für jedes Vorkommen ausgeführt und summiert den Beton aller Pilotis."],
        ["change",
         "Change the diameter in the type and every occurrence follows; volume and total update, the GlobalId stays.",
         "Ändert man den Durchmesser im Typ, folgen alle Vorkommen: Volumen und Summe ändern sich, die GlobalId bleibt."],
        ["inheritance",
         "Inheritance: each class passes attributes down to IfcColumn, which adds only its PredefinedType.",
         "Vererbung: Jede Klasse reicht Attribute an IfcColumn weiter, die nur PredefinedType ergänzt."],
        ["values",
         "Here six inherited attributes have values, among them GlobalId, name, placement and tag; three stay empty.",
         "Hier haben sechs geerbte Attribute einen Wert, darunter GlobalId, Name, Rasterplatz und Tag; drei bleiben leer."],
        ["spatial",
         "Elements are contained in a storey; the storey belongs to the building, the building to the site.",
         "Bauteile liegen in einem Geschoss; das Geschoss gehört zum Gebäude, das Gebäude zum Grundstück."],
        ["opening",
         "Composition means parts: the wall has an opening, which IfcRelVoidsElement cuts out of it.",
         "Komposition bedeutet Teile: Die Wand hat eine Öffnung, die IfcRelVoidsElement aus ihr ausschneidet."],
        ["window",
         "A window fills that opening through IfcRelFillsElement; the glass takes the place of the opening.",
         "Ein Fenster füllt diese Öffnung über IfcRelFillsElement; das Glas nimmt den Platz der Öffnung ein."],
        ["layers",
         "The wall also has a material: a layer set of render, brick and plaster adds up to its thickness.",
         "Die Wand hat auch ein Material: Ein Schichtenaufbau aus Außenputz, Ziegel und Innenputz ergibt ihre Dicke."],
        ["distinction",
         "Keep is-a and has-a apart: a piloti is a built element; a window is not a wall, it fills an opening.",
         "Unterscheide »ist ein« und »hat ein«: Die Stütze ist ein Bauteil; das Fenster ist keine Wand, es füllt eine Öffnung."],
        ["exchange",
         "IFC exchanges declarative data and relationships: class, GlobalId, radius, height and properties travel as text.",
         "IFC tauscht deklarative Daten und Beziehungen aus: Klasse, GlobalId, Radius, Höhe und Eigenschaften reisen als Text."],
        ["recompute",
         "The volume method does not travel: the receiving application runs its own method and gets the same value.",
         "Die Volumenmethode reist nicht mit: Die empfangende Anwendung führt ihre eigene Methode aus und erhält denselben Wert."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("objects")

    def present_objects(self):
        """🧬 Follow one piloti from the schema to the exchange file, then one wall for composition."""
        s = self._facts()
        s.chrome = {id(m) for m in self.mobjects}
        tag = self.text("IFC 4.3 · ISO 16739-1:2024", (6.8, 3.7, 0), size=S_SMALL, color=MUTED, anchor="ru")
        s.chrome.add(id(tag))
        self.add(tag)
        for beat in (self._schema, self._class, self._instance, self._occurrences, self._pset, self._representations, self._geometry, self._methods, self._total,
                     self._change, self._inheritance, self._values, self._spatial, self._opening, self._window, self._layers, self._distinction, self._exchange,
                     self._recompute):
            beat(s)

    def _facts(self):
        """📦 Every model value the story reads, resolved once from the one model object."""
        model = _model.load_model()
        geo = _model.building_geometry(model)
        kind = next(t for t in model["types"] if t["class"] == "IfcColumnType")
        occ = _model.type_occurrences(kind["tag"], model)
        wall = next(w for w in geo["walls"] if w["tag"] == WALL)
        s = SimpleNamespace(
            model=model, geo=geo, kind=kind, occ=occ, count=len(occ), lang=self.language(), wall=wall, opening=wall["openings"][0], sub=None, chains={},
            sel=next(p for p in geo["pilotis"] if p["tag"] == SELECTED), element=_model.element(SELECTED, model),
            q_sel=_model.quantities(SELECTED, model), q_all=_model.quantities(kind["tag"], model), body=_model.represent(SELECTED, "body", model),
            slab=next(x for x in geo["slabs"] if x["kind"] == "FLOOR"), q_wall=_model.quantities(WALL, model),
            wall_element=_model.element(WALL, model), kind_element=_model.element(kind["tag"], model), classes=Counter(e["class"] for e in model["elements"]),
            base_diameter=kind["profile"]["diameter"]["value"])
        s.q_slab = _model.quantities(s.slab["tag"], model)
        return s

    def _chain(self, s, diameter):
        """🧮 Area, volume and total for one type diameter, each from the previous displayed value so every product on screen holds."""
        key = round(round(diameter / 0.02) * 0.02, 2)
        if key not in s.chains:
            twin = copy.deepcopy(s.model)
            next(t for t in twin["types"] if t["class"] == "IfcColumnType")["profile"]["diameter"]["value"] = key
            q = _model.quantities(SELECTED, twin)
            area = round(q["cross_section_area"], 5)
            volume = round(area * q["height"], 4)
            s.chains[key] = SimpleNamespace(diameter=key, radius=q["radius"], height=q["height"], area=area, volume=volume, count=s.count, total=round(s.count * volume, 4))
        return s.chains[key]

    def _leave(self, s, *keep):
        """🚪 Retire consumed drawing callbacks before fading, preserving chrome, caption, takeaway and kept live objects."""
        spared = {id(m) for m in keep} | s.chrome
        leaving = [m for m in list(self.mobjects) if id(m) not in spared and m is not self._caption and m is not s.sub and not isinstance(m, _engine.ValueTracker)]
        for mobject in leaving:
            mobject.clear_updaters()
        return [_engine.FadeOut(m) for m in leaving]

    def _slide(self, mob, start, seconds=0.8):
        """🛷 Slide a mobject from a start point to where it stands now while it fades in."""
        return _engine.FadeIn(mob, target_position=P(start), run_time=seconds)

    def _chip(self, text, point, color=MUTED, *, width=None, size=S_LABEL):
        """🔲 A rounded label chip around one string; the string is squeezed when it does not fit the width."""
        label = self.text(text, (0, 0, 0), size=size, color=color, width=(width - 0.2) if width else None)
        frame = _engine.RoundedRectangle(corner_radius=0.05, width=(width or label.width + 0.28), height=0.36, stroke_color=color, stroke_width=W_THIN,
                                         fill_color=_visuals._art.CHROME_FILL, fill_opacity=0.95)
        chip = _engine.VGroup(frame, label.move_to(frame))
        chip.frame, chip.label = frame, label
        return place(chip, point)

    def _ramp(self, tracker, value, seconds):
        """📈 Drive a ValueTracker linearly to a value, starting from wherever it stands when the animation begins."""
        start = []

        def update(mob, alpha):
            if not start:
                start.append(mob.get_value())
            mob.set_value(start[0] + (value - start[0]) * alpha)
        return _engine.UpdateFromAlphaFunc(tracker, update, run_time=seconds, rate_func=_engine.linear)

    def _appear(self, mobject, seconds=1.0):
        """🌅 Fade a hidden mobject in through its opacity; it joins the stage when its turn comes, above everything added before."""
        return _engine.UpdateFromAlphaFunc(mobject, lambda mob, alpha: mob.set_opacity(alpha), run_time=seconds, introducer=True)

    def _pulse(self, mobject, **options):
        """💓 Indicate one mobject; it joins the stage when its turn comes, not at the start of the beat."""
        return _engine.Indicate(mobject, introducer=True, **options)

    def _tick(self, point, color=GOOD, size=0.16):
        """✓ A check mark drawn from two strokes."""
        x, y, _ = P(point)
        return polyline([(x - size * 0.6, y), (x - size * 0.15, y - size * 0.5), (x + size * 0.7, y + size * 0.55)], color, 3.0)

    def _cross(self, point, color=WARN, size=0.16):
        """✗ A cross mark drawn from two strokes."""
        x, y, _ = P(point)
        return segments([((x - size * 0.6, y - size * 0.6), (x + size * 0.6, y + size * 0.6)), ((x - size * 0.6, y + size * 0.6), (x + size * 0.6, y - size * 0.6))], color, 3.0)

    def _pointer(self, start, end, color=MUTED, width=1.6):
        """➡️ A thin arrow with a small tip between two points."""
        return _engine.Arrow(P(start), P(end), buff=0.03, color=color, stroke_width=width, tip_length=0.1, max_tip_length_to_length_ratio=0.6)

    def _assumed(self, point, anchor="c"):
        """🏷️ The "assumed" badge that marks values derived from teaching assumptions."""
        return self.badge(self.tr("assumed", "angenommen"), point, color=WARN, filled=False, anchor=anchor)

    def _basis(self, s, info):
        """🔖 Basis of a value; a derived value inherits the basis of the first-floor level it hangs on."""
        if info["basis"] != "derived":
            return info["basis"]
        return next(t["elevation"]["basis"] for t in s.model["storeys"] if t["id"] == "first")

    def _act(self, s, en, de):
        """📑 Swap the section heading chip; returns the animations."""
        new = _fonts.section_heading(self.tr(en, de))
        old, s.sub = s.sub, new
        return [_engine.FadeIn(new)] + ([_engine.FadeOut(old)] if old is not None else [])

    def _axon(self, s, scale=AXON_SCALE, point=AXON_POINT, **options):
        """🧊 The shared axonometric of the villa."""
        options.setdefault("disciplines", ("architecture", "structure"))
        return self.villa_axon(point, scale, model=s.model, **options)

    def _xray(self, s, scale=AXON_SCALE, point=AXON_POINT):
        """🔍 Axonometric with the first-floor box ghosted so every piloti shows; the selected one is lit and ringed."""
        axon = self._axon(s, scale, point, highlight=(SELECTED,), ghost=XRAY, shell=0.5)
        axon.ring = _engine.Circle(radius=0.28 if scale > 0.15 else 0.18, stroke_color=DATA, stroke_width=2.4).move_to(axon.anchors[SELECTED])
        axon.add(axon.ring)
        return axon

    def _overlay(self, axon, tags, color, opacity=0.9):
        """🎨 A flat recoloured copy of some axon parts, drawn on top."""
        group = _engine.VGroup(*[axon.parts[tag] for tag in tags if tag in axon.parts]).copy()
        group.set_color(color)
        group.set_opacity(opacity)
        return group

    def _beam(self, tags, axon, color, seconds=0.6):
        """✨ Flash the parts of some tags in one colour and let them go."""
        overlay = self._overlay(axon, tags, color)
        return _engine.Succession(_engine.FadeIn(overlay, run_time=seconds / 2), _engine.FadeOut(overlay, run_time=seconds / 2))

    def _tags(self, s, cls):
        """🏷️ Tags of all drawn parts that belong to one IFC class."""
        geo = s.geo
        if cls == "IfcColumn":
            return [p["tag"] for p in geo["pilotis"]]
        if cls == "IfcSlab":
            return [x["tag"] for x in geo["slabs"]]
        if cls == "IfcWall":
            return list(dict.fromkeys([w["tag"] for w in geo["walls"]] + [x["tag"] for x in geo.get("screens", ())]))
        return [o["window_tag"] for w in geo["walls"] for o in w["openings"]]

    def _plan(self, s, point, scale, **options):
        """📋 Ground-floor plan from the shared widget; ``plan.centres[tag]`` are the piloti centres on the screen."""
        plan = self.villa_plan("ground", point, scale, model=s.model, names=False, layers=("structure", "architecture"), **options)
        plan.centres = {p["tag"]: plan.to_screen(p["x"], p["y"]) for p in s.geo["pilotis"]}
        return plan

    def _plan_lit(self, s, plan, count):
        """💡 Pilotis lit in occurrence order while ``count`` runs from 0 to the occurrence count."""
        def build():
            value = count.get_value()
            group = _engine.VGroup()
            for index, tag in enumerate(s.occ):
                fraction = min(max(value - index, 0.0), 1.0)
                if fraction <= 0:
                    continue
                lit = tag == SELECTED
                radius = (0.1 if lit else 0.075) + 0.07 * math.sin(math.pi * fraction) if fraction < 1 else (0.1 if lit else 0.075)
                group.add(_engine.Circle(radius=radius, stroke_width=0, fill_color=DATA if lit else STRUCT, fill_opacity=0.4 + 0.6 * fraction).move_to(plan.centres[tag]))
            return group
        return self.live(build, key=lambda: round(count.get_value() * 12))

    def _plan_follow(self, s, plan, count):
        """🔖 The tag of the piloti that is lighting right now."""
        def build():
            value = count.get_value()
            if value <= 0 or value >= s.count:
                return _engine.VGroup()
            tag = s.occ[int(value)]
            return _engine.VGroup(self.text(tag, plan.centres[tag] + np.array([0.3, 0.22, 0]), size=S_LABEL, color=INK))
        return self.live(build, key=lambda: int(count.get_value() * 4))

    def _schema(self, s):
        """1️⃣ The schema as a card beside the villa; each row flashes its examples on the building."""
        m, lang = s.model, s.lang
        rows = [(self.tr("Vocabulary", "Begriffe"), GOOD), "IfcColumn · IfcSlab · IfcWall · IfcWindow", (self.tr("Rules", "Regeln"), GOOD),
                self.tr("every IfcRoot object has a GlobalId", "jedes IfcRoot-Objekt hat eine GlobalId"),
                (self.tr("Relationships", "Beziehungen"), GOOD), "IfcRelVoidsElement · IfcRelFillsElement"]
        s.card = self.class_box(m["schema_id"], rows, (LEFT, 1.2, 0), color=GOOD, width=LW)
        years = m["building"]["years"]
        title = self.text(f"{m['building']['name'][lang]} · {m['site']['name'][lang]} · {years['design_start']}–{years['completed']}", (0, 0, 0), size=S_LABEL, color=INK)
        note = [self.label("teaching model: simplified,", "Lehrmodell: vereinfacht,", (0, 0, 0), size=S_SMALL, color=MUTED),
                self.label("rounded, partly assumed", "gerundet, teils angenommen", (0, 0, 0), size=S_SMALL, color=MUTED),
                self.label("ribbon windows: extent disputed", "Bandfenster: Ausdehnung umstritten", (0, 0, 0), size=S_SMALL, color=MUTED)]
        lines = _engine.VGroup(title, *note).arrange(_engine.DOWN, buff=0.1)
        frame = frame_box(max(LW, lines.width + 0.4), lines.height + 0.3, MUTED)
        s.badges = _engine.VGroup(frame, lines.move_to(frame))
        s.badges.move_to((LEFT, -1.7, 0))
        s.axon = self._axon(s)
        guids = {e["guid"] for e in m["elements"]}
        s.count_chip = self._chip(f"{len(m['elements'])} " + self.tr("objects = ", "Objekte = ") + f"{len(guids)} GlobalIds", (0, 0, 0), DATA)
        place(s.count_chip, (6.8, -2.3, 0), "r")
        marks = (("B3", STRUCT), (s.slab["tag"], SLAB), (WALL, ARCH))
        s.guid_lines = _engine.VGroup(*[self.mono(f"{tag:<5}{_model.global_id(tag)}", (LEFT - 2.0, -0.28 - 0.25 * i, 0), size=S_SMALL, color=color, anchor="l") for i, (tag, color) in enumerate(marks)])
        flashes = [self._beam(self._tags(s, cls), s.axon, CLASS_COLOR[cls]) for cls in CLASS_COLOR]
        sparks = [_engine.Flash(s.axon.anchors[tag], color=DATA, flash_radius=0.17, line_length=0.1, num_lines=8, run_time=0.5) for tag in s.occ]
        relations = [self._beam([WALL], s.axon, ARCH, 1.0), self._beam([s.opening["window_tag"]], s.axon, GOOD, 1.0)]
        self.step("schema", _engine.Succession(
            _engine.AnimationGroup(_engine.FadeIn(s.card), _engine.FadeIn(s.badges), _engine.FadeIn(s.axon), *self._act(s, "Schema = vocabulary + rules", "Schema = Begriffe + Regeln"), run_time=2.0),
            _engine.AnimationGroup(self._pulse(s.card.rows[1], color=DATA), _engine.LaggedStart(*flashes, lag_ratio=0.7, run_time=2.2), run_time=2.2),
            _engine.AnimationGroup(self._pulse(s.card.rows[3], color=DATA), _engine.FadeIn(s.count_chip), _engine.FadeIn(s.guid_lines), _engine.LaggedStart(*sparks, lag_ratio=0.08, run_time=1.4), run_time=1.4),
            _engine.AnimationGroup(self._pulse(s.card.rows[5], color=DATA), *relations, run_time=1.0)), seconds=7.0)

    def _tree(self, s):
        """🌳 The IFC supertype chain from IfcRoot to IfcBuiltElement; the four leaf classes hang on one bus line, each with its count."""
        chain = _model.class_chain("IfcColumn", s.model)[:-1]
        tree = _engine.VGroup()
        tree.chips, tree.leaves, tree.arrows, tree.marks, tree.counts = {}, {}, [], [], {}
        left = -6.5
        for index, node in enumerate(chain):
            chip = self._chip(node["class"], (left + 1.35, 2.3 - index * 0.44, 0), MUTED, width=2.7)
            tree.chips[node["class"]] = chip
            tree.add(chip)
            if index:
                arrow = self._pointer(tree.chips[chain[index - 1]["class"]].get_bottom(), chip.get_top())
                tree.arrows.append(arrow)
                tree.add(arrow)
        built = tree.chips["IfcBuiltElement"]
        bus_x = left + 0.22
        last = None
        for index, name in enumerate(CLASS_COLOR):
            y = built.get_center()[1] - 0.48 - index * 0.42
            leaf = self._chip(name, (left + 0.5 + 0.85, y, 0), CLASS_COLOR[name], width=1.7)
            noun = self.tr(*CLASS_NOUN[name])
            count = self.text(f"{s.classes[name]} {noun}" + ("*" if name == "IfcColumn" else ""), (leaf.get_right()[0] + 0.15, y, 0), size=S_SMALL, color=LABEL, anchor="l")
            tick = segments([((bus_x, y), (leaf.get_left()[0], y))], CLASS_COLOR[name], W_EDGE)
            tree.leaves[name], tree.counts[name] = leaf, count
            tree.marks.append(tick)
            tree.add(leaf, count, tick)
            last = y
        bus = segments([((bus_x, built.get_bottom()[1]), (bus_x, last))], MUTED, W_EDGE)
        tree.marks.append(bus)
        shown = sum(s.classes[name] for name in CLASS_COLOR)
        other = len(s.model["elements"]) - shown
        tree.note = self.label(f"+ {other} other objects: openings, ramps, services", f"+ {other} weitere Objekte: Öffnungen, Rampen, Haustechnik", (left, last - 0.36, 0), size=S_SMALL, color=MUTED, anchor="l", width=LW - 0.1)
        tree.legend = self._legend((left, last - 0.64, 0))
        tree.add(bus, tree.note, tree.legend)
        return tree

    def _class(self, s):
        """2️⃣ The class tree; every class lights all villa parts of its kind in its own colour."""
        gone = self._leave(s, s.axon)
        tree = s.tree = self._tree(s)
        trunk = [*tree.chips.values(), *tree.arrows]
        leaf_parts = [(name, tree.leaves[name], tree.counts[name], tree.marks[index]) for index, name in enumerate(CLASS_COLOR)]
        trunk.append(tree.marks[-1])
        for mob in [*trunk, tree.note, tree.legend, *tree.marks, *tree.leaves.values(), *tree.counts.values()]:
            mob.set_opacity(0)
        self.add(*tree.submobjects)
        s.overlays = {name: self._overlay(s.axon, self._tags(s, name), CLASS_COLOR[name]) for name in CLASS_COLOR}
        dim = _engine.UpdateFromAlphaFunc(s.axon, lambda mob, alpha: mob.set_opacity(1 - 0.7 * alpha), run_time=1.6)
        steps = [_engine.AnimationGroup(*gone, dim,
                                        _engine.LaggedStart(*[self._appear(m, 0.6) for m in trunk], lag_ratio=0.2), *self._act(s, "Class = a kind of thing", "Klasse = eine Art von Ding"), run_time=2.0)]
        for name, leaf, count, tick in leaf_parts:
            steps.append(_engine.AnimationGroup(self._appear(leaf), self._appear(count), self._appear(tick), _engine.FadeIn(s.overlays[name]), run_time=0.9))
        steps.append(_engine.AnimationGroup(self._appear(tree.note, 0.7), self._appear(tree.legend, 0.7), self._pulse(tree.leaves["IfcColumn"].frame, color=DATA), run_time=0.8))
        self.step("class", _engine.Succession(*steps), seconds=6.4)

    def _instance(self, s):
        """3️⃣ One piloti lit with its card and GlobalId chip beside it; the others have their own GlobalIds."""
        lang = s.lang
        gone = self._leave(s)
        axon = self._xray(s)
        s.instance = self.class_box(f"{SELECTED} : IfcColumn", [f"Name  {s.element['name'][lang]}", f"Tag  {SELECTED}"], (LEFT, 1.8, 0), color=DATA, width=LW)
        s.guid = self.guid_chip(SELECTED, (5.15, -2.0, 0), model=s.model)
        s.leader = self.leader(s.guid, axon.anchors[SELECTED], color=DATA)
        others = (s.occ[0], s.occ[-1])
        s.others = _engine.VGroup(self.text(f"1 / {s.count}*  ·  " + self.tr("the others:", "die anderen:"), (LEFT - 2.0, 0.7, 0), size=S_SMALL, color=MUTED, anchor="l"),
                                  *[self.mono(f"{tag}  {_model.global_id(tag)}", (LEFT - 2.0, 0.38 - 0.32 * i, 0), size=S_SMALL, color=LABEL, anchor="l") for i, tag in enumerate(others)],
                                  self._legend((LEFT - 2.0, -0.35, 0)))
        s.axon = axon
        self.step("instance", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(axon), _engine.FadeIn(s.instance), *self._act(s, "Instance = one particular object", "Instanz = ein bestimmtes Objekt"), run_time=1.4),
            _engine.AnimationGroup(_engine.FadeIn(s.guid), _engine.Create(s.leader), run_time=1.0),
            _engine.AnimationGroup(_engine.Flash(axon.anchors[SELECTED], color=DATA, flash_radius=0.4, run_time=1.0), _engine.FadeIn(s.others), run_time=1.0)), seconds=4.4)

    def _legend(self, point, anchor="l"):
        """⭐ The legend of the piloti-count star: 25 in this teaching model, sources differ."""
        return self.label("* in this teaching model; sources differ: 15–25", "* in diesem Lehrmodell; Quellen weichen ab: 15–25", point, size=S_SMALL, color=MUTED, anchor=anchor, width=LW - 0.1)

    def _occurrences(self, s):
        """4️⃣ One type, all occurrences light up on the ground-floor plan."""
        lang = s.lang
        kind = s.kind_element
        gone = self._leave(s)
        s.type_card = self.class_box(f"IfcColumnType · {kind['name'][lang]}", [self.tr("Profile: circle", "Profil: Kreis"), self.tr("Material: ", "Material: ") + kind["material"]["name"][lang], self.tr("profile, material:", "Profil, Material:")],
                                     (LEFT, 1.75, 0), color=STRUCT, width=LW)
        s.type_card.add(self._assumed((s.type_card.rows[2].get_right()[0] + 0.15, s.type_card.rows[2].get_center()[1], 0), "l"))
        s.chip = self._chip(f"{SELECTED} : IfcColumn", (LEFT, 0.5, 0), DATA, width=2.7)
        s.relation = _engine.VGroup(self._pointer(s.type_card.get_bottom(), s.chip.get_top()), self.text("IfcRelDefinesByType", (LEFT + 0.15, 0.85, 0), size=S_SMALL, color=MUTED, anchor="l"))
        s.plan = self._plan(s, STAGE, 0.19)
        gx, gy, box = s.geo["grid"]["x"], s.geo["grid"]["y"], s.geo["box"]
        view = s.plan.view
        unit = self.quantity(s.geo["grid"]["spacing"], "m", 2)
        s.plan.add(*[self.dimension(view(a, box["width"]), view(b, box["width"]), unit, offset=0.3, size=S_SMALL) for a, b in zip(gx[:-1], gx[1:])],
                   *[self.dimension(view(box["length"], a), view(box["length"], b), unit, offset=-0.3, size=S_SMALL) for a, b in zip(gy[:-1], gy[1:])])
        s.plan_title = self.label("Ground-floor plan", "Grundriss EG", (6.8, -2.38, 0), size=S_SMALL, color=MUTED, anchor="r")
        s.counter_value = _engine.ValueTracker(0)
        s.lit = self._plan_lit(s, s.plan, s.counter_value)
        s.follow = self._plan_follow(s, s.plan, s.counter_value)
        s.ring = _engine.Circle(radius=0.2, stroke_color=DATA, stroke_width=2.2).move_to(s.plan.centres[SELECTED])
        s.counter = self.live_label(lambda: f"{int(round(s.counter_value.get_value() - 0.4999))} / {s.count}*", (LEFT, -0.45, 0), size=44, color=STRUCT)
        s.counter_label = _engine.VGroup(self.label("occurrences in this teaching model*", "Vorkommen in diesem Lehrmodell*", (LEFT, -1.1, 0), size=S_LABEL, color=INK, width=LW - 0.1),
                                         self.label("* sources differ: 15–25", "* Quellen weichen ab: 15–25", (LEFT, -1.6, 0), size=S_SMALL, color=MUTED, width=LW - 0.1))
        self.step("occurrences", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.plan), _engine.FadeIn(s.plan_title), _engine.FadeIn(s.type_card), _engine.FadeIn(s.chip), _engine.FadeIn(s.relation),
                                   _engine.FadeIn(s.counter), _engine.FadeIn(s.counter_label), *self._act(s, "Type = what all occurrences share", "Typ = was alle Vorkommen teilen"), run_time=1.2),
            _engine.Add(s.lit, s.follow),
            self._ramp(s.counter_value, s.count, 4.0),
            _engine.AnimationGroup(_engine.FadeIn(s.ring), self._pulse(s.chip, color=DATA), run_time=0.8)), seconds=6.0)

    def _pset(self, s):
        """🧾 The property set of the type, B3's own slab share hatched on the plan and the load path of LoadBearing TRUE drawn from slab to foundation, with every printed product holding."""
        gone = self._leave(s, s.plan, s.lit, s.follow, s.ring, s.plan_title)
        lang = s.lang
        trib = _model.tributary(s.model)
        entry = next(p for p in trib["pilotis"] if p["tag"] == SELECTED)
        areas = [p["area"] for p in trib["pilotis"]]
        parts = [trib["load_breakdown"][key] for key in ("self_weight", "finishes", "imposed")]
        q = trib["load_breakdown"]["total"]["value"]
        area = round(entry["area"], 3)
        reaction = round(area * q, 2)
        rows = []
        for name, prop in s.kind_element["psets"]["Pset_ColumnCommon"].items():
            value = prop["value"]
            rows.append((name, ("TRUE" if value else "FALSE") if isinstance(value, bool) else str(value), "", prop["basis"]))
        s.pset = self.property_table(rows, (LEFT + 0.15, 1.6, 0), width=4.6, title="Pset_ColumnCommon · IfcColumnType", color=GOOD, size=S_SMALL)
        rings = [[s.plan.to_screen(x, y) for x, y in piece] for piece in entry["pieces"]]
        s.share = _engine.VGroup(*[polygon(ring, DATA, 0.2, None) for ring in rings], _drafting.hatch(rings, DATA, 0.06, 45, W_HAIR, 0.9))
        s.share_value = _engine.ValueTracker(0)
        s.share_label = self.live_label(lambda: self.quantity(area * s.share_value.get_value(), "m²", 3), (LEFT, -0.35, 0), size=40, color=DATA)
        spread = f"{self.number(min(areas), 2)}–{self.quantity(max(areas), 'm²', 2)}"
        composition = " + ".join(self.number(p["value"], 2) for p in parts)
        names = " + ".join(p[lang].lower() if lang == "en" else p[lang] for p in parts)
        small = dict(size=S_SMALL, color=MUTED, width=LW - 0.1)
        s.share_text = _engine.VGroup(self.label("B3 carries this part of the slab:", "B3 trägt diesen Teil der Decke:", (LEFT, 0.25, 0), size=S_LABEL, color=INK),
                                      self.label("tributary estimate (hatched), ramp void excluded", "Einzugsfläche geschätzt (schraffiert), ohne Rampe", (LEFT, -0.82, 0), **small),
                                      self.text(f"q = {self.quantity(q, 'kN/m²', 2)} = {composition}", (LEFT, -1.07, 0), size=S_SMALL, color=INK, width=LW - 0.1),
                                      self.text(names, (LEFT, -1.32, 0), **small),
                                      self.text(f"{self.quantity(area, 'm²', 3)} × {self.quantity(q, 'kN/m²', 2)} = {self.quantity(reaction, 'kN', 2)} *", (LEFT, -1.58, 0), size=S_SMALL, color=DATA, width=LW - 0.1),
                                      self.label("occurrence value, B3 alone;", "Vorkommenswert, nur für B3;", (LEFT, -1.84, 0), **small),
                                      self.label(f"all pilotis: {spread}", f"alle Stützen: {spread}", (LEFT, -2.08, 0), **small),
                                      self.label("* teaching-model loads", "* Lasten des Lehrmodells", (LEFT, -2.38, 0), **small))
        load = s.load = self._load_path(s, reaction)
        self.step("pset", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.pset), _engine.FadeIn(s.share_text), _engine.FadeIn(s.share_label), run_time=1.2),
            _engine.AnimationGroup(_engine.FadeIn(s.share), self._ramp(s.share_value, 1.0, 1.8), run_time=1.8),
            _engine.AnimationGroup(_engine.FadeIn(load.frame), _engine.LaggedStart(*[_engine.GrowArrow(a) for a in load.arrows], lag_ratio=0.25, run_time=1.6), run_time=1.6),
            _engine.AnimationGroup(_engine.FadeIn(load.labels), self._pulse(s.pset.rows[0], color=DATA, run_time=0.8), run_time=0.8)), seconds=6.0)

    def _load_path(self, s, reaction):
        """⬇️ The path of the load for LoadBearing TRUE: slab load on SL-1, through B3, into the foundation; the reaction is B3's own value."""
        cx, base, scale = -1.3, -1.75, 0.42
        top = base + s.q_sel["height"] * scale
        deck = top + s.slab["thickness"] * scale
        frame = _engine.VGroup(_drafting.hatch([(cx - 0.8, base), (cx + 0.8, base), (cx + 0.8, base - 0.14), (cx - 0.8, base - 0.14)], MUTED, 0.07, 45, 0.9, 0.6),
                               segments([((cx - 0.8, base), (cx + 0.8, base))], INK, 2.0),
                               polygon([(cx - 0.85, top), (cx + 0.85, top), (cx + 0.85, deck), (cx - 0.85, deck)], SLAB, 0.55, SLAB, W_EDGE),
                               polygon([(cx - 0.08, base), (cx + 0.08, base), (cx + 0.08, top), (cx - 0.08, top)], STRUCT, 0.7, STRUCT, W_EDGE))
        arrows = [self._pointer((cx + dx, deck + 0.5), (cx + dx, deck + 0.04), DATA, 2.4) for dx in (-0.65, -0.325, 0.0, 0.325, 0.65)]
        arrows.append(self._pointer((cx, base - 0.02), (cx, base - 0.5), DATA, 4.0))
        labels = _engine.VGroup(self.text("LoadBearing TRUE", (cx, deck + 0.95, 0), size=S_SMALL, color=GOOD),
                                self.label("slab load*", "Deckenlast*", (cx, deck + 0.7, 0), size=S_SMALL, color=DATA),
                                self.text(self.quantity(reaction, "kN", 2) + "*", (cx - 0.15, (base + top) / 2, 0), size=S_SMALL, color=DATA, anchor="r"),
                                self.label("foundation", "Fundament", (cx + 0.1, base - 0.38, 0), size=S_SMALL, color=MUTED, anchor="l"))
        return SimpleNamespace(frame=frame, arrows=arrows, labels=labels)

    def _pilot_static(self, s, layout):
        """🧱 Ground hatch, ground line and the first-floor slab above one piloti; the base carries the localized ±0.00 mark."""
        d = layout
        group = _engine.VGroup()
        wide = 1.0
        top = d.base + s.q_sel["height"] * d.scale
        slab_top = top + s.slab["thickness"] * d.scale
        group.add(_drafting.hatch([(d.ex - wide, d.base), (d.ex + wide, d.base), (d.ex + wide, d.base - 0.16), (d.ex - wide, d.base - 0.16)], MUTED, 0.07, 45, 0.9, 0.6))
        group.add(segments([((d.ex - wide, d.base), (d.ex + wide, d.base))], INK, 2.2))
        group.add(polygon([(d.ex - wide, top), (d.ex + wide, top), (d.ex + wide, slab_top), (d.ex - wide, slab_top)], CONCRETE, 0.45, CONCRETE, W_EDGE))
        group.add(_drafting.hatch([(d.ex - wide, top), (d.ex + wide, top), (d.ex + wide, slab_top), (d.ex - wide, slab_top)], CONCRETE, 0.06, 45, 0.9, 0.7))
        if d.labels:
            group.add(self.level_mark((d.ex + wide + 0.02, d.base), "±" + self.number(0.0, 2), size=S_SMALL, length=0.0, side=1))
            group.add(self.text(f"{s.slab['tag']} · IfcSlab", (d.ex + wide + 0.1, (top + slab_top) / 2, 0), size=S_SMALL, color=LABEL, anchor="l"))
        return group

    def _pilot(self, s, t, layout):
        """🔩 One piloti drawn as elevation and enlarged section, live in every tracker: representations, dimensions, poché, concrete fill and type diameter."""
        d, q = layout, s.q_sel

        def build():
            rep, reveal, area, fill, dia = (t.rep.get_value(), t.reveal.get_value(), t.area.get_value(), t.fill.get_value(), t.dia.get_value())
            chain = self._chain(s, dia)
            half = dia / 2 * d.scale
            top = d.base + q["height"] * d.scale
            radius = dia / 2 * d.scale * d.enlarge
            centre = np.array([d.cx, d.cy, 0.0])
            group = _engine.VGroup()
            keep = 1 - 0.7 * max(reveal, area, fill)
            axis_a, foot_a, body_a = clamp(rep) * keep, clamp(rep - 1) * keep, clamp(rep - 2)
            named = d.labels and max(reveal, area, fill) == 0
            if axis_a > 0 and d.elevation:
                group.add(polyline([(d.ex, d.base - 0.15), (d.ex, top + 0.12)], GOOD, 2.4, axis_a, dash=0.11))
                if named:
                    group.add(self.text(self.tr("Axis", "Achse"), (d.ex + half + 0.14, top - 0.25, 0), size=S_SMALL, color=GOOD, anchor="l").set_opacity(axis_a))
            if foot_a > 0:
                if d.elevation:
                    group.add(segments([((d.ex - half - 0.05, d.base), (d.ex + half + 0.05, d.base))], ARCH, 4.0, foot_a))
                group.add(_engine.Circle(radius=radius, stroke_color=ARCH, stroke_width=2.6, stroke_opacity=foot_a, fill_opacity=0).move_to(centre))
                if named:
                    group.add(self.text(self.tr("FootPrint", "Grundfläche"), centre + np.array([0, radius + 0.28, 0]), size=S_SMALL, color=ARCH).set_opacity(foot_a))
            if d.ghost and abs(dia - s.base_diameter) > 1e-6:
                ghost = s.base_diameter / 2 * d.scale * d.enlarge
                group.add(polyline(_drafting.circle_points(centre, ghost, 40), MUTED, W_EDGE, 0.8, closed=True, dash=0.08))
            if body_a > 0:
                if d.elevation:
                    group.add(polygon([(d.ex - half, d.base), (d.ex + half, d.base), (d.ex + half, top), (d.ex - half, top)], STRUCT, 0.45 * body_a, STRUCT, W_EDGE, body_a))
                group.add(_engine.Circle(radius=radius, stroke_width=0, fill_color=STRUCT, fill_opacity=0.4 * body_a).move_to(centre))
                group.add(segments([((d.cx - radius - 0.2, d.cy), (d.cx + radius + 0.2, d.cy)), ((d.cx, d.cy - radius - 0.2), (d.cx, d.cy + radius + 0.2))], MUTED, 0.7, 0.7 * body_a))
            if area > 0.001:
                ring = _drafting.circle_points(centre, radius * math.sqrt(area), 36)
                group.add(polygon(ring, CONCRETE, 0.8, CONCRETE, W_THIN), _drafting.hatch(ring, INK, 0.07, 45, W_HAIR, 0.55))
            if fill > 0.001 and d.elevation:
                level = d.base + q["height"] * d.scale * fill
                group.add(polygon([(d.ex - half, d.base), (d.ex + half, d.base), (d.ex + half, level), (d.ex - half, level)], CONCRETE, 0.9, None))
                group.add(segments([((d.ex - 0.4, level), (d.ex + 0.4, level))], DATA, 2.4))
                if d.volume:
                    group.add(self.text("V = " + self.quantity(chain.volume * fill, "m³", 4), (d.ex + 0.5, level - 0.14, 0), size=S_LABEL, color=DATA, anchor="l"))
            if reveal > 0.001:
                angle = math.radians(35)
                tip = centre + np.array([radius * math.cos(angle), radius * math.sin(angle), 0])
                group.add(segments([(centre, tip)], DATA, 2.2, reveal), _engine.Dot(tip, radius=0.04, color=DATA).set_opacity(reveal),
                          self.text("r = " + self.quantity(chain.radius, "m", 2), tip + np.array([0.26, 0.14, 0]), size=S_LABEL, color=DATA, anchor="l").set_opacity(reveal))
                group.add(_sheet.dim(self, (d.cx - radius, d.cy, 0), (d.cx + radius, d.cy, 0), None, offset=-(radius + 0.35), color=DATA, size=S_LABEL, opacity=reveal),
                          self.text("Ø " + self.quantity(chain.diameter, "m", 2), (d.cx, d.cy - radius - 0.62, 0), size=S_LABEL, color=DATA).set_opacity(reveal))
                if d.elevation:
                    group.add(_sheet.dim(self, (d.ex - half, d.base, 0), (d.ex - half, top, 0), "h = " + self.quantity(chain.height, "m", 2), offset=d.offset, color=DATA, size=S_LABEL, opacity=reveal))
            return group
        return self.live(build, key=lambda: tuple(round(v * 100) for v in (t.rep.get_value(), t.reveal.get_value(), t.area.get_value(), t.fill.get_value(), t.dia.get_value())))

    def _trackers(self, s, **start):
        """🎚️ The shared trackers of the piloti drawing."""
        values = dict(rep=0, reveal=0, area=0, fill=0, dia=s.base_diameter, done=0)
        values.update(start)
        return SimpleNamespace(**{name: _engine.ValueTracker(value) for name, value in values.items()})

    def _representations(self, s):
        """6️⃣ Axis, FootPrint and Body: three stored representations of the same piloti switch on one after the other."""
        gone = self._leave(s)
        t = s.t = self._trackers(s)
        reps = [_model.represent(SELECTED, kind, s.model) for kind in ("axis", "footprint", "body")]
        tones = (GOOD, ARCH, STRUCT)
        s.rep_box = self.class_box("IfcProductDefinitionShape", [(f"{r['identifier']} · {r['type']}", c) for r, c in zip(reps, tones)], (LEFT, 1.7, 0), color=INK, width=LW)
        s.rep_note = _engine.VGroup(self.mono(s.element["guid"], (LEFT, 0.55, 0), size=S_SMALL, color=DATA),
                                    self.label("one object, one GlobalId,", "ein Objekt, eine GlobalId,", (LEFT, 0.15, 0), size=S_SMALL, color=MUTED),
                                    self.label("three stored representations", "drei gespeicherte Darstellungen", (LEFT, -0.12, 0), size=S_SMALL, color=MUTED))
        s.static = self._pilot_static(s, BIG)
        s.pilot = self._pilot(s, t, BIG)
        s.mini = self._xray(s, 0.075, (5.45, 1.4, 0))
        self.add(s.pilot)
        self.step("representations", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.rep_box), _engine.FadeIn(s.rep_note), _engine.FadeIn(s.static), _engine.FadeIn(s.mini),
                                   *self._act(s, "Values and methods belong to the object", "Werte und Methoden gehören zum Objekt"), run_time=1.2),
            _engine.AnimationGroup(self._ramp(t.rep, 1.0, 0.9), self._pulse(s.rep_box.rows[0], color=DATA), run_time=1.0),
            _engine.AnimationGroup(self._ramp(t.rep, 2.0, 0.9), self._pulse(s.rep_box.rows[1], color=DATA), run_time=1.0),
            _engine.AnimationGroup(self._ramp(t.rep, 3.0, 0.9), self._pulse(s.rep_box.rows[2], color=DATA), run_time=1.0)), seconds=5.0)

    def _geometry(self, s):
        """7️⃣ Radius of the circle profile and extrusion height of the Body, drawn as dimension lines, with their basis."""
        gone = self._leave(s, s.pilot, s.static, s.mini)
        t, derived = s.t, s.body["derived"]
        diameter = _model.information(SELECTED, "piloti.diameter", s.lang, s.model)
        height = _model.information(SELECTED, "height", s.lang, s.model)
        rows = [(self.tr("Radius r", "Radius r"), derived["radius"], "m", diameter["basis"]), (self.tr("Height h", "Höhe h"), derived["height"], "m", self._basis(s, height)),
                ("Ø = 2 · r", derived["diameter"], "m", diameter["basis"])]
        s.table = self.property_table(rows, (LEFT + 0.15, 1.55, 0), width=4.6, title=f"{s.body['identifier']} · {s.body['type']}", color=STRUCT, size=S_LABEL)
        s.basis_chips = _engine.VGroup(*[row.basis for row in s.table.rows])
        s.basis_chips.set_opacity(0)
        s.ifc_names = _engine.VGroup(self.text(f"r → {derived['profile']}", (LEFT - 2.0, 0.3, 0), size=S_SMALL, color=LABEL, anchor="l"),
                                     self.text(f"h → {derived['solid']} · Depth", (LEFT - 2.0, -0.02, 0), size=S_SMALL, color=LABEL, anchor="l"))
        s.note = _engine.VGroup(self.label("assumed = teaching assumption:", "angenommen = Annahme des Lehrmodells:", (LEFT, -0.7, 0), size=S_SMALL, color=MUTED),
                                self.label("no source gives the real size", "keine Quelle nennt das echte Maß", (LEFT, -0.97, 0), size=S_SMALL, color=MUTED),
                                self.label("one student slide, unverified: r 0.30 m, h 2.87 m", "Studentenfolie, ungeprüft: r 0,30 m, h 2,87 m", (LEFT, -1.24, 0), size=S_SMALL, color=MUTED, width=LW - 0.1))
        self.step("geometry", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.table), _engine.FadeIn(s.ifc_names), _engine.FadeIn(s.note), run_time=1.0),
            self._ramp(t.reveal, 1.0, 1.6),
            self._appear(s.basis_chips, 0.8),
            self._pulse(s.table.rows[1].value, color=DATA, run_time=0.7)), seconds=4.4)

    def _methods(self, s):
        """8️⃣ Two methods compute area and volume with the substituted numbers while the concrete fills the piloti."""
        gone = self._leave(s, s.pilot, s.static, s.mini)
        t = s.t
        chain = self._chain(s, s.base_diameter)
        r, h = self.quantity(chain.radius, "m", 2), self.quantity(chain.height, "m", 2)
        x = LEFT - 2.0
        s.app_label = self.label("Application", "Anwendung", (x, 2.3, 0), size=S_LABEL, color=GOOD, anchor="l")
        s.method_a = self.badge("cross_section_area()", (x, 1.85, 0), color=GOOD, anchor="l")
        s.method_v = self.badge("volume()", (x, 0.3, 0), color=GOOD, anchor="l")
        s.formulas = _engine.VGroup(
            self.text("A = π · r²", (x, 1.42, 0), size=S_LABEL, color=INK, anchor="l"),
            self.text(f"= π · ({r})²", (x, 1.08, 0), size=S_LABEL, color=INK, anchor="l"),
            self.live_label(lambda: "= " + self.quantity(chain.area * t.area.get_value(), "m²", 5), (x, 0.74, 0), size=S_LABEL, color=CONCRETE, anchor="l"),
            self.text("V = A · h", (x, -0.1, 0), size=S_LABEL, color=INK, anchor="l"),
            self.text(f"= {self.quantity(chain.area, 'm²', 5)} · {h}", (x, -0.44, 0), size=S_LABEL, color=INK, anchor="l"),
            self.live_label(lambda: "= " + self.quantity(chain.volume * t.fill.get_value(), "m³", 4), (x, -0.78, 0), size=S_LABEL, color=CONCRETE, anchor="l"))
        for mob in (s.formulas[0], s.formulas[1], s.formulas[3], s.formulas[4]):
            mob.set_opacity(0)
        inputs = self._assumed((x, -1.3, 0), "l")
        s.formula_hint = _engine.VGroup(self.label("≈ rounded to the digits shown", "≈ gerundet auf die gezeigten Stellen", (x, -1.65, 0), size=S_SMALL, color=MUTED, anchor="l"), inputs,
                                        self.label("inputs r, h", "Eingaben r, h", (inputs.get_right()[0] + 0.12, -1.3, 0), size=S_SMALL, color=MUTED, anchor="l"))
        self.step("methods", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.app_label), _engine.FadeIn(s.method_a), _engine.FadeIn(s.method_v), _engine.FadeIn(s.formula_hint), _engine.FadeIn(s.formulas[2]),
                                   _engine.FadeIn(s.formulas[5]), run_time=0.9),
            _engine.AnimationGroup(self._appear(_engine.VGroup(s.formulas[0], s.formulas[1]), 0.5), self._ramp(t.area, 1.0, 1.3), run_time=1.4),
            _engine.AnimationGroup(self._appear(_engine.VGroup(s.formulas[3], s.formulas[4]), 0.5), self._ramp(t.fill, 1.0, 2.6), run_time=2.7),
            self._pulse(s.formulas[5], color=DATA, run_time=0.6)), seconds=6.0)
        s.formula_group = s.formulas

    def _bars(self, s, t):
        """📊 Two bars on one scale: concrete of all pilotis (live) against the first-floor slab."""
        slab = s.q_slab
        top = 3.2

        def build():
            chain = self._chain(s, t.dia.get_value())
            group = _engine.VGroup()
            for index, (name, value, color) in enumerate(((self.tr("all pilotis", "alle Pilotis"), chain.total, STRUCT), (self.tr("slab SL-1", "Decke SL-1"), slab["volume"], SLAB))):
                y = -1.15 - 0.45 * index
                width = max(value / slab["volume"] * top, 0.04)
                group.add(self.text(name, (LEFT - 2.0, y, 0), size=S_SMALL, color=LABEL, anchor="l"))
                group.add(polygon([(LEFT - 0.55, y - 0.1), (LEFT - 0.55 + width, y - 0.1), (LEFT - 0.55 + width, y + 0.1), (LEFT - 0.55, y + 0.1)], color, 0.9, color, W_THIN))
                group.add(self.text(self.quantity(value, "m³", 1 if value > 20 else 2), (LEFT - 0.55 + width + 0.1, y, 0), size=S_SMALL, color=color, anchor="l"))
            return group
        return self.live(build, key=lambda: round(t.dia.get_value() * 50))

    def _sum_panel(self, s, t, *, change):
        """➕ Left panel of the total beats: the substituted product, its rounded headline and the slab volume with its bars; ``change`` adds the live diameter chain."""
        x = LEFT - 2.0
        parts = _engine.VGroup()
        chain_of = lambda: self._chain(s, t.dia.get_value())
        parts.product = self.live_label(lambda: f"{int(t.done.get_value() + 0.0001)} × {self.quantity(chain_of().volume, 'm³', 4)} = {self.quantity(int(t.done.get_value() + 0.0001) * chain_of().volume, 'm³', 4)}",
                                        (x, 1.12, 0), size=S_LABEL, color=INK, anchor="l")
        parts.headline = self.live_label(lambda: "≈ " + self.quantity(int(t.done.get_value() + 0.0001) * chain_of().volume, "m³", 2), (x, 0.42, 0), size=40, color=DATA, anchor="l")
        slab = s.q_slab
        parts.legend = self._legend((x, 0.86, 0))
        parts.slab = _engine.VGroup(
            self.label("first-floor slab SL-1", "Geschossdecke SL-1", (x, -0.15, 0), size=S_SMALL, color=LABEL, anchor="l"),
            self.text(f"{self.quantity(slab['area'], 'm²', 2)} × {self.quantity(slab['thickness'], 'm', 2)} = {self.quantity(slab['volume'], 'm³', 3)}", (x, -0.5, 0), size=S_SMALL, color=INK, anchor="l"),
            self._assumed((x + 3.3, -0.15, 0), "l"))
        parts.bars = self._bars(s, t)
        if change:
            parts.diameter = self.live_label(lambda: "Ø = " + self.quantity(chain_of().diameter, "m", 2), (x, 2.3, 0), size=S_LABEL, color=DATA, anchor="l")
            parts.area = self.live_label(lambda: f"A = π · ({self.quantity(chain_of().radius, 'm', 2)})² = {self.quantity(chain_of().area, 'm²', 5)}", (x, 1.95, 0), size=S_SMALL, color=INK, anchor="l")
            parts.volume = self.live_label(lambda: f"V = {self.quantity(chain_of().area, 'm²', 5)} · {self.quantity(chain_of().height, 'm', 2)} = {self.quantity(chain_of().volume, 'm³', 4)}",
                                           (x, 1.6, 0), size=S_SMALL, color=INK, anchor="l")
        else:
            parts.diameter = self.label(f"all {s.count}* pilotis of the type", f"alle {s.count}* Pilotis des Typs", (x, 2.3, 0), size=S_LABEL, color=MUTED, anchor="l")
            parts.area = self.live_label(lambda: "V = " + self.quantity(chain_of().volume, "m³", 4), (x, 1.85, 0), size=S_LABEL, color=INK, anchor="l")
            parts.volume = self.label("per piloti, from the method", "je Stütze, aus der Methode", (x, 1.55, 0), size=S_SMALL, color=MUTED, anchor="l")
        return parts

    def _concrete_axon(self, s, axon, t):
        """🏗️ Concrete rising in every piloti of the axonometric, one after the other; the stroke width follows the type diameter."""
        view, pilotis = axon.view, s.geo["pilotis"]
        order = {tag: index for index, tag in enumerate(s.occ)}

        def build():
            value, width = t.done.get_value(), 5.0 * t.dia.get_value() / s.base_diameter
            lines = []
            for p in pilotis:
                f = clamp(value - order[p["tag"]])
                if f > 0.001:
                    lines.append(_engine.Line(view(p["x"], p["y"], p["base"]), view(p["x"], p["y"], p["base"] + f * (p["top"] - p["base"])), color=CONCRETE, stroke_width=width))
            return _engine.VGroup(*lines)
        return self.live(build, key=lambda: (round(t.done.get_value() * 12), round(t.dia.get_value() * 100)))

    def _total(self, s):
        """7️⃣ Every piloti fills one after the other; the sum counts up and is set against the slab."""
        gone = self._leave(s)
        t = s.t
        axon = s.total_axon = self._axon(s, TOTAL.scale, TOTAL.point, disciplines=("structure",), ghost=("IfcColumn", s.slab["tag"]))
        s.sum = self._sum_panel(s, t, change=False)
        s.filled = self._concrete_axon(s, axon, t)
        flash = self._overlay(axon, [s.slab["tag"]], SLAB, 0.45)
        self.step("total", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(axon), _engine.FadeIn(s.sum.diameter), _engine.FadeIn(s.sum.area), _engine.FadeIn(s.sum.volume),
                                   _engine.FadeIn(s.sum.product), _engine.FadeIn(s.sum.headline), _engine.FadeIn(s.sum.bars), _engine.FadeIn(s.sum.legend), run_time=1.2),
            _engine.Add(s.filled),
            self._ramp(t.done, s.count, 3.6),
            _engine.AnimationGroup(_engine.FadeIn(s.sum.slab), _engine.FadeIn(flash), run_time=1.0),
            _engine.FadeOut(flash, run_time=0.4)), seconds=6.4)

    def _change(self, s):
        """🔁 The type diameter moves; the section, every occurrence, A, V and the total follow while the GlobalId stays."""
        t = s.t
        keep = (s.total_axon, s.filled, s.sum.product, s.sum.headline, s.sum.slab, s.sum.bars, s.sum.legend)
        gone = self._leave(s, *keep)
        guid = self.guid_chip(SELECTED, (LEFT, -2.15, 0), model=s.model)
        same = self.label("= same GlobalId", "= dieselbe GlobalId", (-3.05, -2.15, 0), size=S_SMALL, color=GOOD, anchor="l")
        detail = self._pilot(s, t, CHANGE)
        s.sum_change = self._sum_panel(s, t, change=True)
        title = self.label("type diameter: what-if", "Typdurchmesser: Was-wäre-wenn", (CHANGE.cx, 2.5, 0), size=S_SMALL, color=MUTED)
        self.step("change", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.sum_change.diameter), _engine.FadeIn(s.sum_change.area), _engine.FadeIn(s.sum_change.volume), _engine.FadeIn(guid), _engine.FadeIn(same),
                                   _engine.FadeIn(title), _engine.FadeIn(detail), run_time=1.0),
            self._ramp(t.dia, 0.4, 3.2), self._pulse(guid, color=DATA, run_time=0.8), self._ramp(t.dia, s.base_diameter, 1.4)), seconds=6.8)

    def _inheritance(self, s):
        """8️⃣ Attribute rows are copied from each supertype into the IfcColumn box."""
        gone = self._leave(s)
        chain = _model.class_chain("IfcColumn", s.model)
        chips = [self._chip(node["class"], (-5.45, 2.25 - index * 0.62, 0), DATA if node["class"] == "IfcColumn" else MUTED, width=2.5) for index, node in enumerate(chain)]
        spine = _engine.VGroup(*chips)
        arrows = _engine.VGroup(*[self._pointer(a.get_bottom(), b.get_top()) for a, b in zip(chips, chips[1:])])
        names = [name for node in chain for name in node["adds"]]
        box = self.class_box("IfcColumn", names, (-1.9, 0.6, 0), color=DATA, width=4.5)
        rows = list(box.rows)
        for row in rows:
            row.set_color(LABEL)
        shell = _engine.VGroup(*[m for m in box.submobjects if m not in rows])
        box.remove(*rows)
        s.inh_box, s.inh_rows, s.inh_chips, s.inh_names = box, dict(zip(names, rows)), chips, names
        s.inh_spine, s.inh_arrows, s.inh_shell = spine, arrows, shell
        axon = s.inh_axon = self._xray(s, 0.14, (3.7, 0.15, 0))
        link = self.leader((box.frame.get_right()[0], 0.6, 0), axon.anchors[SELECTED], color=DATA)
        owner = self.text(f"{SELECTED} : IfcColumn", (6.8, -2.3, 0), size=S_SMALL, color=DATA, anchor="r")
        values = self._attribute_values(s)
        s.inh_values = {}
        for name, row in zip(names, rows):
            s.inh_values[name] = (self.mono if name == "GlobalId" else self.text)(values[name], (box.frame.get_right()[0] - 0.12, row.get_center()[1], 0), size=S_SMALL, color=DATA, anchor="r")
        steps = [_engine.AnimationGroup(*gone, _engine.FadeIn(spine), _engine.FadeIn(arrows), _engine.FadeIn(shell), _engine.FadeIn(axon),
                                        *self._act(s, "Inheritance: attributes come from supertypes", "Vererbung: Attribute kommen von Oberklassen"), run_time=1.2)]
        index = 0
        for node, chip in zip(chain, chips):
            count = len(node["adds"])
            animations = [self._pulse(chip, color=DATA, scale_factor=1.05)]
            for k in range(count):
                animations.append(self._slide(s.inh_rows[names[index + k]], chip.get_right() + np.array([0.1, 0, 0]), 0.8))
            steps.append(_engine.AnimationGroup(*animations, run_time=0.8 if count else 0.4))
            index += count
        steps.append(_engine.AnimationGroup(_engine.Create(link), _engine.FadeIn(owner), _engine.Flash(axon.anchors[SELECTED], color=DATA, flash_radius=0.4, run_time=1.0), run_time=1.2))
        self.step("inheritance", _engine.Succession(*steps), seconds=8.2)

    def _attribute_values(self, s):
        """🧾 The value of every IfcColumn attribute on B3, in display strings."""
        lang, sel, element, body = s.lang, s.sel, s.element, s.body
        notset, absent = self.tr("not set", "nicht gesetzt"), self.tr("not in model", "nicht im Modell")
        return {"GlobalId": element["guid"], "OwnerHistory": absent, "Name": element["name"][lang], "Description": notset, "ObjectType": notset,
                "ObjectPlacement": f"({self.number(sel['x'], 2)}{self.tr(', ', '; ')}{self.number(sel['y'], 2)}) m", "Representation": f"{body['identifier']} · {body['type']}",
                "Tag": SELECTED, "PredefinedType": f".{element['predefined_type']}."}

    def _values(self, s):
        """9️⃣ Each inherited row shows its value and is tied to the drawing: GlobalId by a leader, placement by dimensions, tag by the grid bubbles."""
        sel = s.sel
        keep = (s.inh_spine, s.inh_arrows, s.inh_shell, *s.inh_rows.values())
        gone = self._leave(s, *keep)
        plan = s.inh_plan = self._plan(s, (3.75, -0.1, 0), 0.17)
        view = plan.view
        length, width = s.geo["box"]["length"], s.geo["box"]["width"]
        dot = _engine.Circle(radius=0.1, stroke_width=0, fill_color=DATA, fill_opacity=1).move_to(plan.centres[SELECTED])
        ring = _engine.Circle(radius=0.2, stroke_color=DATA, stroke_width=2.0).move_to(plan.centres[SELECTED])
        top, right = view(sel["x"], width), view(length, sel["y"])
        guide_x = polyline([plan.centres[SELECTED], (top[0], top[1] + 0.34, 0)], DATA, W_THIN, 0.8, dash=0.09)
        guide_y = polyline([plan.centres[SELECTED], (right[0] + 0.34, right[1], 0)], DATA, W_THIN, 0.8, dash=0.09)
        dim_x = self.dimension(view(0, width), view(sel["x"], width), self.quantity(sel["x"], "m", 2), offset=0.38, size=S_SMALL)
        dim_y = self.dimension(view(length, 0), view(length, sel["y"]), self.quantity(sel["y"], "m", 2), offset=-0.38, size=S_SMALL)
        start = s.inh_values["GlobalId"].get_right() + np.array([0.08, 0, 0])
        route = view(sel["x"], (s.geo["grid"]["y"][3] + s.geo["grid"]["y"][4]) / 2)[1]
        target = plan.centres[SELECTED]
        leader = _engine.VGroup(polyline([start, start + np.array([0.3, 0, 0]), (start[0] + 0.3, route, 0), (target[0], route, 0), (target[0], target[1] + 0.1, 0)], DATA, W_THIN, 1.0),
                                _engine.Dot(target, radius=0.05, color=DATA))
        tint = _engine.AnimationGroup(*[self._pulse(s.inh_values[n], color=INK, scale_factor=1.04) for n in ("Name", "ObjectPlacement", "Tag")])
        bubbles = _engine.AnimationGroup(self._pulse(plan.parts["label_y:B"], color=DATA), self._pulse(plan.parts["label_x:3"], color=DATA))
        empty = [s.inh_rows[n] for n in ("OwnerHistory", "Description", "ObjectType")]
        s.values_group = _engine.VGroup(plan, dot, ring, guide_x, guide_y, dim_x, dim_y, leader)
        self.step("values", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(plan), _engine.FadeIn(dot), *[_engine.FadeIn(v) for v in s.inh_values.values()], run_time=1.4),
            _engine.AnimationGroup(_engine.Create(leader), _engine.FadeIn(ring), run_time=1.0),
            _engine.AnimationGroup(_engine.Create(guide_x), _engine.Create(guide_y), _engine.FadeIn(dim_x), _engine.FadeIn(dim_y), run_time=1.4),
            _engine.AnimationGroup(bubbles, tint, run_time=0.9),
            _engine.AnimationGroup(*[self._pulse(r, color=WARN, scale_factor=1.04) for r in empty], run_time=0.8)), seconds=6.4)



    def _spatial(self, s):
        """🗺️ Spatial containment: site, building and storey chips on the left, the section with the two storey bands on the right."""
        lang = s.lang
        gone = self._leave(s)
        relations = _model.relations_of(SELECTED, s.model)
        contained = next(r["class"] for r in relations if "Contained" in r["class"])
        aggregates = next(r["class"] for r in _model.relations_of(s.model["building"]["tag"], s.model) if "Aggregates" in r["class"])
        sec = s.sec = self.villa_section((3.45, -0.2, 0), 0.28, model=s.model, cut="row:3", highlight=(SELECTED, s.slab["tag"], WALL), levels=False, labels=False,
                                         highlight_colors={SELECTED: DATA, s.slab["tag"]: SLAB, WALL: ARCH})
        storeys = {t["id"]: s.geo["storeys"][t["id"]] for t in s.model["storeys"]}
        width = s.geo["box"]["width"]
        edges = {"ground": (0.0, storeys["first"]), "first": (storeys["first"], storeys["roof"])}
        tones = {"ground": STRUCT, "first": ARCH}
        name = {t["id"]: t["name"][lang] for t in s.model["storeys"]}
        bands = _engine.VGroup(*[polygon([sec.at(-1.2, z0), sec.at(width + 1.2, z0), sec.at(width + 1.2, z1), sec.at(-1.2, z1)], tones[key], 0.1, tones[key], W_THIN, 0.7) for key, (z0, z1) in edges.items()])
        storey_names = _engine.VGroup(*[self.text(name[key], sec.at(-1.2, (z0 + z1) / 2) + np.array([-0.1, 0.0, 0]), size=S_SMALL, color=tones[key], anchor="r") for key, (z0, z1) in edges.items()])
        site = self._chip(f"IfcSite · {s.model['site']['name'][lang]}", (LEFT, 2.3, 0), GOOD, width=3.6)
        building = self._chip(f"IfcBuilding · {s.model['building']['name'][lang]}", (LEFT, 1.6, 0), GOOD, width=3.6)
        ground = self._storey_chip(name["ground"], (LEFT - 1.0, 0.6, 0), STRUCT)
        first = self._storey_chip(name["first"], (LEFT + 1.0, 0.6, 0), ARCH)
        agg = _engine.VGroup(self._pointer(site.get_bottom(), building.get_top()), self.text(aggregates, (LEFT + 0.15, 1.95, 0), size=S_SMALL, color=MUTED, anchor="l"),
                             self._pointer((-5.9, building.get_bottom()[1], 0), (-5.9, ground.get_top()[1], 0)), self._pointer((-3.5, building.get_bottom()[1], 0), (-3.5, first.get_top()[1], 0)),
                             self.text(aggregates, (LEFT, 1.15, 0), size=S_SMALL, color=MUTED))
        column = self._chip(f"{SELECTED} : IfcColumn", (LEFT - 1.0, -0.4, 0), DATA, width=1.85, size=S_SMALL)
        slab = self._chip(f"{s.slab['tag']} : IfcSlab", (LEFT + 1.0, -0.4, 0), SLAB, width=1.85, size=S_SMALL)
        wall = self._chip(f"{WALL} : IfcWall", (LEFT + 1.0, -0.9, 0), ARCH, width=1.85, size=S_SMALL)
        members = _engine.VGroup(column, slab, wall, self._pointer(ground.get_bottom(), column.get_top()), self._pointer(first.get_bottom(), slab.get_top()),
                                 self.text(contained, (LEFT, -1.5, 0), size=S_SMALL, color=MUTED),
                                 self.label("also cut: ramp (zigzag), solarium screen", "auch geschnitten: Rampe (Zickzack), Solarium-Wand", (LEFT, -1.9, 0), size=S_SMALL, color=MUTED, width=LW - 0.1), self.label("solarium screen", "Solarium-Wand", (3.44, 0.74, 0), size=S_SMALL, color=MUTED))
        self.step("spatial", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(sec), _engine.FadeIn(bands), _engine.FadeIn(storey_names), _engine.FadeIn(site), _engine.FadeIn(building),
                                   *self._act(s, "Spatial structure: where it sits", "Raumstruktur: wo es liegt"), run_time=1.3),
            _engine.AnimationGroup(_engine.FadeIn(agg), _engine.FadeIn(ground), _engine.FadeIn(first), run_time=1.3),
            _engine.AnimationGroup(_engine.FadeIn(members), self._pulse(sec.parts[SELECTED], color=DATA), run_time=1.6)), seconds=5.6)

    def _storey_chip(self, name, point, color):
        """🏢 A two-line chip: the class IfcBuildingStorey above the storey name."""
        kind = self.text("IfcBuildingStorey", (0, 0, 0), size=S_SMALL, color=MUTED)
        label = self.text(name, (0, 0, 0), size=S_LABEL, color=color)
        frame = _engine.RoundedRectangle(corner_radius=0.05, width=1.85, height=0.62, stroke_color=color, stroke_width=W_THIN, fill_color=_visuals._art.CHROME_FILL, fill_opacity=0.95)
        chip = _engine.VGroup(frame, kind.move_to(frame.get_center() + np.array([0, 0.15, 0])), label.move_to(frame.get_center() + np.array([0, -0.13, 0])))
        return place(chip, point)

    def _south(self, s, point, scale):
        """🏛️ South elevation: set-back ground floor, pilotis, first-floor wall band, ribbon opening, window and roof screens, all in elevation coordinates."""
        g = s.geo
        box = g["box"]
        view = _drafting.View("elevation", point, scale, (box["length"] / 2, 0, box["height"] / 2), face="south")
        v = lambda x, z: view(x, 0, z)
        parts = {}
        out = _engine.VGroup()
        L = box["length"]
        ground = [sp for sp in g["spaces"] if sp["storey"] == "ground"]
        xs = [x for sp in ground for x, _ in sp["outline"]]
        soffit = s.slab["top"] - s.slab["thickness"]
        out.add(_drafting.hatch([v(-1.0, 0), v(L + 1.0, 0), v(L + 1.0, -0.4), v(-1.0, -0.4)], MUTED, 0.08, 45, 0.9, 0.5))
        out.add(segments([(v(-1.0, 0), v(L + 1.0, 0))], INK, 2.0))
        out.add(polygon([v(min(xs), 0), v(max(xs), 0), v(max(xs), soffit), v(min(xs), soffit)], GREEN_GROUND, 0.9, GREEN_GROUND, 1.0))
        pilotis = _engine.VGroup()
        r = g["pilotis"][0]["radius"]
        for x in sorted({p["x"] for p in g["pilotis"]}):
            pilotis.add(polygon([v(x - r, 0), v(x + r, 0), v(x + r, soffit), v(x - r, soffit)], STRUCT, 0.75, STRUCT, 1.2))
        parts["pilotis"] = pilotis
        out.add(pilotis)
        top1 = s.slab["top"]
        slab_face = polygon([v(0, soffit), v(L, soffit), v(L, top1), v(0, top1)], SLAB, 0.55, SLAB, W_EDGE)
        parts["slab"] = slab_face
        out.add(slab_face)
        wall = s.wall
        face = polygon([v(0, wall["base"]), v(L, wall["base"]), v(L, wall["top"]), v(0, wall["top"])], mix(ARCH, BACKGROUND, 0.7), 1.0, ARCH, W_EDGE)
        parts["wall"] = face
        out.add(face)
        roof = next(x for x in g["slabs"] if x["kind"] == "ROOF")
        out.add(polygon([v(0, wall["top"]), v(L, wall["top"]), v(L, roof["top"]), v(0, roof["top"])], SLAB, 0.55, SLAB, W_EDGE))
        for screen in g.get("screens", ()):
            sx = [x for x, _ in screen["path"]]
            out.add(polygon([v(min(sx), roof["top"]), v(max(sx), roof["top"]), v(max(sx), screen["top"]), v(min(sx), screen["top"])], mix(INK, BACKGROUND, 0.7), 0.9, INK, 1.2))
            if scale > 0.2:
                out.add(self.label("solarium screen", "Solarium-Wand", v((min(sx) + max(sx)) / 2, (roof["top"] + screen["top"]) / 2), size=S_SMALL, color=LABEL, width=1.6))
        op = s.opening
        corners = [v(op["start"], op["sill_z"]), v(op["end"], op["sill_z"]), v(op["end"], op["head_z"]), v(op["start"], op["head_z"])]
        parts["opening"] = polygon(corners, BACKGROUND, 1.0, DATA, W_EDGE)
        out.add(parts["opening"])
        panes = [(v(op["start"] + (op["end"] - op["start"]) * k / 12, op["sill_z"]), v(op["start"] + (op["end"] - op["start"]) * k / 12, op["head_z"])) for k in range(1, 12)]
        parts["window"] = _engine.VGroup(polygon(corners, GOOD, 0.4, GOOD, W_EDGE), segments(panes, GOOD, 1.0, 0.9))
        parts["corners"] = corners
        parts["column"] = v(s.sel["x"], soffit / 2)
        parts["band"] = v(L * 0.06, (wall["base"] + wall["top"]) / 2)
        parts["glass"] = v((op["start"] + op["end"]) / 2, (op["sill_z"] + op["head_z"]) / 2)
        out.view, out.parts, out.v = view, parts, v
        return out

    def _opening(self, s):
        """🕳️ The wall has an opening: its area is hatched on the elevation and dimensioned; IfcRelVoidsElement cuts it out."""
        lang = s.lang
        gone = self._leave(s)
        el, op, q = s.wall_element, s.opening, s.q_wall
        classes = {r["class"] for r in _model.relations_of(WALL, s.model) + _model.relations_of(op["tag"], s.model)}
        voids, fills = (next(c for c in classes if key in c) for key in ("Voids", "Fills"))
        elevation = s.south = self._south(s, (2.65, 0.3, 0), 0.3)
        v = elevation.v
        wall_box = self.class_box(f"IfcWall · {WALL}", [], (LEFT, 2.1, 0), color=ARCH, width=3.6)
        opening_chip = self._chip(f"IfcOpeningElement · {op['tag']}", (LEFT, 1.2, 0), DATA, width=3.6)
        window_chip = self._chip(f"IfcWindow · {op['window_tag']}", (LEFT, 0.45, 0), GOOD, width=3.6)
        usage = self._chip("IfcMaterialLayerSetUsage", (LEFT, -0.55, 0), MUTED, width=3.6)
        layers = el["material"]["layers"]
        layer_box = self.class_box("IfcMaterialLayerSet", [f"{_sheet.short_name(l['name'], lang)}  {self.quantity(l['thickness']['value'], 'm', 2)}" for l in layers], (LEFT, -1.65, 0), color=INK, width=3.6)
        voids_link = _engine.VGroup(self._pointer(wall_box.get_bottom(), opening_chip.get_top()), self.text(voids, (LEFT + 0.15, 1.65, 0), size=S_SMALL, color=MUTED, anchor="l"))
        fills_link = _engine.VGroup(self._pointer(opening_chip.get_bottom(), window_chip.get_top()), self.text(fills, (LEFT + 0.15, 0.83, 0), size=S_SMALL, color=MUTED, anchor="l"))
        material_link = _engine.VGroup(polyline([wall_box.get_left(), (-6.7, 2.1, 0), (-6.7, -0.55, 0), usage.get_left()], MUTED, W_THIN, 0.9, dash=0.1),
                                       self.text("IfcRelAssociatesMaterial", (LEFT + 0.15, 0.03, 0), size=S_SMALL, color=MUTED, anchor="l"),
                                       self.label("associated, not a part", "zugeordnet, kein Teil", (LEFT + 0.15, -0.2, 0), size=S_SMALL, color=MUTED, anchor="l"),
                                       self._pointer(usage.get_bottom(), layer_box.get_top()))
        reveal = _engine.ValueTracker(0)
        corners = elevation.parts["corners"]
        left, right, bottom, top = corners[0][0], corners[1][0], corners[0][1], corners[2][1]
        hatch = self.live(lambda: _engine.VGroup(_drafting.hatch([(left, bottom), (left + (right - left) * reveal.get_value(), bottom), (left + (right - left) * reveal.get_value(), top), (left, top)], DATA, 0.07, 45, 0.9, 0.9)),
                          key=lambda: round(reveal.get_value() * 30))
        width, height = op["end"] - op["start"], op["head_z"] - op["sill_z"]
        soffit = s.slab["top"] - s.slab["thickness"]
        dims = _engine.VGroup(
            self.dimension(v(op["start"], soffit - 1.2), v(op["end"], soffit - 1.2), self.quantity(width, "m", 2) + " *", offset=0, size=S_SMALL, text_at=0.373),
            self.dimension(v(op["end"], op["sill_z"]), v(op["end"], op["head_z"]), self.quantity(height, "m", 2) + " *", offset=-(v(s.geo["box"]["length"], 0)[0] - v(op["end"], 0)[0] + 0.2), size=S_SMALL))
        area = self.live(lambda: _engine.VGroup(self.text(f"{self.quantity(width, 'm', 2)} × {self.quantity(height, 'm', 2)} = " + self.quantity(width * height * reveal.get_value(), "m²", 2) + " *",
                                                          (-0.15, 2.42, 0), size=S_LABEL, color=DATA, anchor="l")), key=lambda: round(reveal.get_value() * 30))
        wall_line = self.text(self.tr("wall ", "Wand ") + f"{WALL}: {self.quantity(q['length'], 'm', 2)} × {self.quantity(q['height'], 'm', 2)} = {self.quantity(q['gross_side_area'], 'm²', 2)} *",
                              (-0.15, 2.05, 0), size=S_SMALL, color=LABEL, anchor="l")
        orientation = self.label("south façade · teaching orientation", "Südfassade · Annahme des Lehrmodells", (-0.15, -1.55, 0), size=S_SMALL, color=MUTED, anchor="l")
        footnote = self.label("* teaching-model assumption: ribbon size, wall height", "* Annahme des Lehrmodells: Bandfenstergröße, Wandhöhe", (-0.15, -1.85, 0), size=S_SMALL, color=MUTED, anchor="l")
        s.comp = SimpleNamespace(wall_box=wall_box, opening=opening_chip, window=window_chip, usage=usage, layers=layer_box, voids=voids_link, fills=fills_link,
                                 material=material_link, hatch=hatch, area=area, wall_line=wall_line, dims=dims, orientation=orientation, footnote=footnote, reveal=reveal, v=v, width=width, height=height)
        self.step("opening", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(elevation), _engine.FadeIn(wall_box), _engine.FadeIn(wall_line), _engine.FadeIn(orientation), _engine.FadeIn(footnote),
                                   *self._act(s, "Composition: has-a", "Komposition: hat-ein"), run_time=1.3),
            _engine.AnimationGroup(_engine.FadeIn(opening_chip), _engine.FadeIn(voids_link), _engine.FadeIn(hatch), _engine.FadeIn(area), self._ramp(reveal, 1.0, 1.8), run_time=1.8),
            _engine.AnimationGroup(_engine.FadeIn(dims), run_time=1.0)), seconds=5.4)

    def _window(self, s):
        """🪟 The window fills the opening: the glass takes its place and the dimensions give way to the glazing share."""
        c, q = s.comp, s.q_wall
        gone = self._leave(s, s.south, c.wall_box, c.opening, c.voids, c.wall_line, c.orientation, c.footnote)
        share = c.width * c.height / q["gross_side_area"]
        c.share = self.text(self.tr("glazing share ", "Glasanteil ") + f"{self.quantity(c.width * c.height, 'm²', 2)} / {self.quantity(q['gross_side_area'], 'm²', 2)} = {self.number(share * 100, 1)} % *",
                            (-0.15, 2.42, 0), size=S_LABEL, color=GOOD, anchor="l")
        self.step("window", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(s.south.parts["window"]), run_time=1.2),
            _engine.AnimationGroup(_engine.FadeIn(c.window), _engine.FadeIn(c.fills), _engine.FadeIn(c.share), run_time=1.2),
            self._pulse(s.south.parts["window"], color=GOOD, scale_factor=1.02, run_time=1.0)), seconds=4.4)

    def _layers(self, s):
        """🧱 The wall's material: a layer set drawn as hatched strips whose widths add up to the wall thickness."""
        lang = s.lang
        c = s.comp
        gone = self._leave(s, c.wall_box, c.opening, c.voids, c.window, c.fills)
        layers = s.wall_element["material"]["layers"]
        total = s.wall_element["material"]["total_thickness"]
        scale = 14
        origin = np.array([3.0 - total * scale / 2, -0.3, 0])
        strip, names = _engine.VGroup(), _engine.VGroup()
        x = origin[0]
        anchors = ("r", "c", "l")
        for index, layer in enumerate(layers):
            thickness = layer["thickness"]["value"]
            width = thickness * scale
            ring = [(x, origin[1]), (x + width, origin[1]), (x + width, origin[1] + 1.2), (x, origin[1] + 1.2)]
            strip.add(polygon(ring, LAYER_COLOR[layer["id"]], 0.9, INK, 1.4), _drafting.hatch(ring, INK, 0.07, (45, 135, 45)[index], W_HAIR, 0.6))
            label_x = (x + width / 2, 3.0, 4.7)[index] if index != 0 else origin[0] + 0.4
            label = self.text(f"{_sheet.short_name(layer['name'], lang)}  {self.quantity(thickness, 'm', 2)}", (label_x, origin[1] + 1.55, 0), size=S_SMALL, color=INK, anchor=anchors[index])
            lead = segments([((x + width / 2, origin[1] + 1.2), (x + width / 2, origin[1] + 1.4))], MUTED, W_HAIR, 0.8)
            names.add(label, lead)
            x += width
        parts = [self.quantity(l["thickness"]["value"], "m", 2) for l in layers]
        total_dim = self.dimension((origin[0], origin[1], 0), (origin[0] + total * scale, origin[1], 0), "  +  ".join(parts) + " = " + self.quantity(total, "m", 2), offset=-0.3, size=S_LABEL)
        sides = _engine.VGroup(self.label("outside", "außen", (origin[0] - 0.12, origin[1] + 0.6, 0), size=S_SMALL, color=MUTED, anchor="r"),
                               self.label("inside", "innen", (origin[0] + total * scale + 0.12, origin[1] + 0.6, 0), size=S_SMALL, color=MUTED, anchor="l"))
        title = self.label("section through the wall W-S", "Schnitt durch die Wand W-S", (3.0, 2.4, 0), size=S_LABEL, color=INK)
        cut = self._cut_marker(s)
        assumed = self._assumed((3.0, -1.45, 0))
        self.step("layers", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(c.usage), _engine.FadeIn(c.material), _engine.FadeIn(c.layers), _engine.FadeIn(title), run_time=1.3),
            _engine.AnimationGroup(_engine.FadeIn(strip), _engine.FadeIn(sides), _engine.FadeIn(cut), run_time=1.2),
            _engine.FadeIn(names, run_time=0.8),
            _engine.AnimationGroup(_engine.FadeIn(total_dim), _engine.FadeIn(assumed), run_time=1.0)), seconds=5.2)

    def _cut_marker(self, s):
        """📌 A small south elevation in the corner with the cut line through the wall."""
        thumb = self._south(s, (5.75, 2.1, 0), 0.08)
        x = thumb.v(s.opening["start"] + 3.0, 0)[0]
        line = polyline([(x, thumb.v(0, 3.2)[1] - 0.1, 0), (x, thumb.v(0, 6.8)[1] + 0.1, 0)], WARN, W_EDGE, 1.0, dash=0.06)
        return _engine.VGroup(thumb, line)

    def _distinction(self, s):
        """🔟 Is-a for the piloti, has-a for the window in the wall; each class name stands on its own drawn part."""
        gone = self._leave(s)
        heading = self._act(s, "Is-a and has-a", "Ist-ein und hat-ein")
        south = s.south_final = self._south(s, (3.2, 0.2, 0), 0.28)
        v = south.v
        built = self._chip("IfcBuiltElement", (LEFT, 2.15, 0), MUTED, width=2.6)
        column = self._chip("IfcColumn", (LEFT, 1.2, 0), STRUCT, width=1.9)
        is_a = _engine.VGroup(built, column, self._pointer(column.get_top(), built.get_bottom(), GOOD, 2.2),
                              self.label("is-a", "ist ein", (LEFT + 0.3, 1.68, 0), size=S_SMALL, color=GOOD, anchor="l"),
                              self.label("A piloti is a built element", "Eine Stütze ist ein Bauteil", (LEFT, 0.7, 0), size=S_LABEL, color=INK))
        is_a_tick = self._tick((LEFT + 2.15, 0.7, 0))
        wall = self._chip("IfcWall", (LEFT, -0.05, 0), ARCH, width=2.4)
        opening = self._chip("IfcOpeningElement", (LEFT, -0.75, 0), DATA, width=2.4)
        window = self._chip("IfcWindow", (LEFT, -1.45, 0), GOOD, width=2.4)
        links = _engine.VGroup(self._pointer(wall.get_bottom(), opening.get_top(), DATA, 2.0), self._pointer(opening.get_bottom(), window.get_top(), DATA, 2.0),
                               self.text("IfcRelVoidsElement", (LEFT + 0.2, -0.4, 0), size=S_SMALL, color=MUTED, anchor="l"),
                               self.text("IfcRelFillsElement", (LEFT + 0.2, -1.1, 0), size=S_SMALL, color=MUTED, anchor="l"))
        has_a = _engine.VGroup(wall, opening, window, links, self.label("has-a", "hat ein", (LEFT - 1.6, 0.3, 0), size=S_SMALL, color=DATA, anchor="l"))
        edge = -6.05
        wrong = polyline([window.get_left(), (edge, window.get_left()[1], 0), (edge, wall.get_left()[1], 0), wall.get_left()], WARN, 2.0, 1.0, dash=0.1)
        wrong_label = self.label("is-a?", "ist ein?", (edge - 0.15, -0.75, 0), size=S_SMALL, color=WARN, anchor="r")
        cross = self._cross((edge, -0.75, 0))
        sentence = _engine.VGroup(self.label("A window is not a wall:", "Ein Fenster ist keine Wand:", (LEFT, -1.95, 0), size=S_LABEL, color=INK),
                                  self.label("it fills an opening in it", "es füllt eine Öffnung darin", (LEFT, -2.27, 0), size=S_LABEL, color=INK))
        marks = _engine.VGroup(
            self.leader(v(19.6, 1.9), v(20.1, 1.9), color=STRUCT), self.text("IfcColumn", v(15.9, 1.9), size=S_SMALL, color=STRUCT, anchor="l"),
            self.text("IfcWall", v(14.0, 8.4), size=S_SMALL, color=ARCH), self.leader(v(14.0, 7.85), v(14.0, 6.0), color=ARCH),
            self.text("IfcWindow", v(18.5, 8.4), size=S_SMALL, color=GOOD), self.leader(v(18.5, 7.85), v(18.5, s.opening["head_z"] - 0.2), color=GOOD))
        self.step("distinction", _engine.Succession(
            _engine.AnimationGroup(*gone, *heading, _engine.FadeIn(south), _engine.FadeIn(is_a), run_time=1.2),
            _engine.AnimationGroup(_engine.FadeIn(is_a_tick), _engine.FadeIn(marks[:2]), self._pulse(south.parts["pilotis"], color=DATA, scale_factor=1.03), run_time=1.0),
            _engine.AnimationGroup(_engine.FadeIn(has_a), _engine.FadeIn(marks[2:]), _engine.FadeIn(south.parts["window"]), run_time=1.4),
            _engine.AnimationGroup(_engine.Create(wrong), _engine.FadeIn(wrong_label), self._pulse(south.parts["window"], color=DATA, scale_factor=1.03), run_time=0.9),
            _engine.AnimationGroup(_engine.FadeIn(cross), _engine.FadeIn(sentence), run_time=0.7)), seconds=6.0)

    def _step_excerpt(self, s):
        """⌨️ Five STEP lines of the selected piloti with their token spans, the element line last; the property set is bound to the type, so no occurrence-level binding line is shown."""
        raw = _model.step_excerpt(SELECTED, s.model)
        names = ("rel_type", "prop_LoadBearing", "solid", "profile", "element")
        indices = [raw["entities"][name] for name in names]
        remap = {old: new for new, old in enumerate(indices)}
        return {"lines": [raw["lines"][i] for i in indices], "spans": [{**sp, "line": remap[sp["line"]]} for sp in raw["spans"] if sp["line"] in remap]}

    def _mini_pilot(self, s, t, layout):
        """🔩 The shared piloti drawing at the small size used by the exchange beats."""
        return _engine.VGroup(self._pilot_static(s, layout), self._pilot(s, t, layout))

    def _exchange(self, s):
        """1️⃣1️⃣ The STEP lines carry class, GlobalId, radius, height and properties; the drawn piloti shows what each token means."""
        gone = self._leave(s)
        t = s.t_exchange = self._trackers(s, rep=3, reveal=1.0)
        excerpt = self._step_excerpt(s)
        panel = s.panel = self.step_panel(excerpt, (0, 1.3, 0), width=12.4, color=mix(INK, BACKGROUND, 0.78))
        head = self.text("IFC4X3_ADD2 · STEP (ISO 10303-21)", (-6.7, -0.12, 0), size=S_SMALL, color=MUTED, anchor="l")
        tokens = {name: panel.token(*args) for name, args in (("guid", ("IFCCOLUMN", "guid")), ("class", ("IFCCOLUMN", "class")), ("profile", ("IFCCIRCLEPROFILEDEF", "profile")),
                                                              ("depth", ("IFCEXTRUDEDAREASOLID", "depth")), ("property", ("LoadBearing", "property")))}
        tones = {"guid": DATA, "class": ARCH, "profile": DATA, "depth": DATA, "property": GOOD}
        rings = {name: segments([(tok.get_corner(_engine.DL) + np.array([0, -0.045, 0]), tok.get_corner(_engine.DR) + np.array([0, -0.045, 0]))], tones[name], 3.2) for name, tok in tokens.items()}
        pilot = s.pilot_mini = self._mini_pilot(s, t, MINI)
        guid = self.guid_chip(SELECTED, (0.0, -1.85, 0), model=s.model)
        start = tokens["guid"].get_corner(_engine.DR) + np.array([-0.3, -0.1, 0])
        link = _engine.VGroup(polyline([start, (start[0], -0.32, 0), (0.0, -0.32, 0), (0.0, guid.get_top()[1] + 0.02, 0)], DATA, W_THIN, 1.0),
                              _engine.Dot((0.0, guid.get_top()[1] + 0.02, 0), radius=0.045, color=DATA))
        loadbearing = str(s.kind_element["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"]).upper()
        relations = _engine.VGroup(
            self._chip("IfcRelDefinesByType → IfcColumnType · " + s.kind_element["name"][s.lang], (4.2, -0.75, 0), GOOD, width=5.2, size=S_SMALL),
            self._chip("IfcColumnType.HasPropertySets → Pset_ColumnCommon", (4.2, -1.3, 0), GOOD, width=5.2, size=S_SMALL),
            self._chip(f"LoadBearing → {loadbearing}", (4.2, -1.85, 0), GOOD, width=5.2, size=S_SMALL))
        self.step("exchange", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(panel), _engine.FadeIn(head), *self._act(s, "Exchange: data travels, code stays", "Austausch: Daten reisen, Code bleibt"), run_time=1.5),
            _engine.AnimationGroup(_engine.Create(rings["guid"]), _engine.Create(rings["class"]), _engine.FadeIn(guid), _engine.Create(link), run_time=1.3),
            _engine.AnimationGroup(_engine.FadeIn(pilot), _engine.Create(rings["profile"]), _engine.Create(rings["depth"]), run_time=1.4),
            _engine.AnimationGroup(_engine.Create(rings["property"]), _engine.FadeIn(relations), run_time=1.2)), seconds=7.0)

    def _recompute(self, s):
        """1️⃣2️⃣ The method does not travel: application B runs its own volume() on the received values and gets the same V."""
        gone = self._leave(s)
        chain = self._chain(s, s.base_diameter)
        fill_b = _engine.ValueTracker(0)
        excerpt = self._step_excerpt(s)
        carrier = self.step_panel(excerpt, (0, 1.95, 0), width=12.4, only=[2, 3, 4], color=mix(INK, BACKGROUND, 0.78))
        titles = (self.tr("Authoring software", "Planungssoftware"), self.tr("Quantity take-off", "Mengenermittlung"))
        boxes = []
        for title, x in zip(titles, (-4.4, 4.4)):
            frame = frame_box(4.6, 3.1, MUTED)
            frame.move_to((x, -0.95, 0))
            boxes.append(_engine.VGroup(frame, self.text(title, (x, 0.35, 0), size=S_LABEL, color=INK)).set_opacity(0))

        def mini(x, level):
            y0, height = -2.1, 1.2
            return _engine.VGroup(segments([((x - 0.4, y0), (x + 0.4, y0))], INK, 2.0),
                                  polygon([(x - 0.07, y0), (x + 0.07, y0), (x + 0.07, y0 + height * level), (x - 0.07, y0 + height * level)], CONCRETE, 0.9, STRUCT, 1.0),
                                  polygon([(x - 0.07, y0), (x + 0.07, y0), (x + 0.07, y0 + height), (x - 0.07, y0 + height)], None, stroke=STRUCT, width=1.4))
        mini_a = mini(-6.0, 1.0)
        mini_b = self.live(lambda: mini(2.8, fill_b.get_value()), key=lambda: round(fill_b.get_value() * 30))
        method_a = self.badge("volume()", (-5.3, -0.15, 0), color=GOOD, anchor="l")
        method_b = self.badge("volume()", (3.5, -0.15, 0), color=GOOD, anchor="l")
        value_a = self.text("V = " + self.quantity(chain.volume, "m³", 4), (-5.3, -0.7, 0), size=S_LABEL, color=CONCRETE, anchor="l")
        value_b = self.live_label(lambda: "V = " + self.quantity(chain.volume * fill_b.get_value(), "m³", 4), (3.5, -0.7, 0), size=S_LABEL, color=CONCRETE, anchor="l")
        guid_a = self.mono(s.element["guid"], (-5.3, -1.2, 0), size=S_SMALL, color=DATA, anchor="l")
        guid_b = self.mono(s.element["guid"], (3.5, -1.2, 0), size=S_SMALL, color=DATA, anchor="l")
        size_text = f"r = {self.quantity(chain.radius, 'm', 2)}  ·  h = {self.quantity(chain.height, 'm', 2)}"
        size_a = self.text(size_text, (-5.3, -1.65, 0), size=S_SMALL, color=INK, anchor="l")
        size_b = self.text(size_text, (3.5, -1.65, 0), size=S_SMALL, color=INK, anchor="l")
        assumed = self._assumed((0.0, -2.35, 0))
        carry = _engine.VGroup(
            self.label("IFC carries:", "IFC trägt:", (0, 0.25, 0), size=S_LABEL, color=GOOD),
            self.label("class · values · relationships", "Klasse · Werte · Beziehungen", (0, -0.1, 0), size=S_LABEL, color=GOOD),
            self.label("never the code of volume()", "nie den Code von volume()", (0, -0.75, 0), size=S_LABEL, color=WARN),
            self.label("a result may be stored as", "ein Ergebnis darf gespeichert sein als", (0, -1.25, 0), size=S_SMALL, color=MUTED, width=3.4),
            self.mono("Qto_ColumnBaseQuantities", (0, -1.55, 0), size=S_SMALL, color=MUTED))
        code = _engine.VGroup(polyline([(-4.2, -0.15, 0), (-2.3, -0.15, 0)], WARN, 2.2, 1.0, dash=0.1), self._cross((-2.2, -0.15, 0), WARN, 0.2))
        agree = _engine.VGroup(self.text(f"{self.quantity(chain.volume, 'm³', 4)} = {self.quantity(chain.volume, 'm³', 4)}", (-0.2, -2.0, 0), size=S_SMALL, color=GOOD, width=3.2), self._tick((1.75, -2.0, 0), GOOD, 0.18))
        self.step("recompute", _engine.Succession(
            _engine.AnimationGroup(*gone, _engine.FadeIn(carrier), *[self._appear(b, 1.0) for b in boxes], _engine.FadeIn(mini_a), _engine.FadeIn(method_a), _engine.FadeIn(method_b), _engine.FadeIn(value_a),
                                   _engine.FadeIn(value_b), _engine.FadeIn(mini_b), _engine.FadeIn(guid_a), _engine.FadeIn(size_a), _engine.FadeIn(carry), _engine.FadeIn(assumed), run_time=1.8),
            _engine.AnimationGroup(_engine.FadeIn(guid_b), _engine.FadeIn(size_b), _engine.FadeIn(code), run_time=1.4),
            _engine.AnimationGroup(self._pulse(method_b, color=GOOD), self._ramp(fill_b, 1.0, 2.4), run_time=2.4),
            _engine.AnimationGroup(_engine.FadeIn(agree), self._pulse(value_b, color=DATA), run_time=0.9)), seconds=7.4)
