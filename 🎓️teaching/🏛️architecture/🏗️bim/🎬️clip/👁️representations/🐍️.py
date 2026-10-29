"""🎬 One Object, Many Views: the northwest wall of the Villa Savoye in plan, section, elevation and 3D, its stored IFC representations and one design change."""

import importlib
import math
import re
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_engine = importlib.import_module("manim")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")

INK, MUTED, ARCH, GLASS, DATA, GOOD, WARN, BACKGROUND = (getattr(_drafting, name) for name in ("INK", "MUTED", "ARCH", "GLASS", "DATA", "GOOD", "WARN", "BACKGROUND"))
S_SMALL, S_LABEL, S_BODY = _drafting.S_SMALL, _drafting.S_LABEL, _drafting.S_BODY
W_CUT, W_EDGE, W_THIN, W_HAIR = _drafting.W_CUT, _drafting.W_EDGE, _drafting.W_THIN, _drafting.W_HAIR
P, mix = _drafting.P, _drafting.mix
NARROW, CONCRETE = _drafting.NARROW, _drafting.CONCRETE

TAG = "W-S"
RIGHT = 4.45
SLOTS = {"plan": (3.2, 1.62, 0.0), "section": (5.7, 1.62, 0.0), "elevation": (3.2, 0.4, 0.0), "3d": (5.7, 0.4, 0.0)}
SLOT_W, SLOT_H = 2.3, 0.95
STEPS = 18
PAPER = 0.055
SLOT_NAMES = {"plan": ("Plan", "Grundriss"), "section": ("Section", "Schnitt"), "elevation": ("Elevation", "Ansicht"), "3d": ("3D", "3D")}
LAYER_TONES = {"render": INK, "brick": WARN, "outer-leaf": WARN, "inner-leaf": WARN, "cavity": MUTED, "plaster": MUTED}
HEADINGS = {
    "source": ("Villa Savoye · northwest wall", "Villa Savoye · Nordwestwand"), "plan": ("Plan · cut at window height", "Grundriss · Schnitt auf Fensterhöhe"),
    "section": ("Section · sill and head", "Schnitt · Brüstung und Sturz"), "elevation": ("Elevation · northwest façade", "Ansicht · Nordwestfassade"),
    "three": ("3D · body, void and fill", "3D · Körper, Aussparung, Füllung"), "stored": ("Stored · Axis, FootPrint, Body", "Gespeichert · Axis, FootPrint, Body"),
    "lod": ("Level of detail", "Detailstufe"), "scale200": ("Scale 1:200", "Maßstab 1:200"), "scale20": ("Scale 1:20", "Maßstab 1:20"),
    "sweep": ("Cut plane sweep", "Schnittebene wandert"), "derived": ("Take-off · areas", "Mengen · Flächen"),
    "update": ("Design change · sill lowered", "Planänderung · Brüstung gesenkt"), "takeoff": ("Take-off · live", "Mengenermittlung · live"),
    "volume": ("Take-off · volume", "Mengenermittlung · Volumen"), "copies": ("Copies · stale", "Kopien · veraltet"),
    "drafted": ("Drafted line", "Gezeichnete Linie"), "regenerate": ("Regeneration", "Neuerzeugung"), "identity": ("One object · one GlobalId", "Ein Objekt · eine GlobalId")}
CHIP_ROWS = ((("IfcWall", "W-S"), DATA), (("IfcWindow", "RW-S"), GLASS))


def seq(*animations, lag: float = 0.3):
    """🎞️ Run animations one after another with overlap so a whole beat is one timed step."""
    return _engine.AnimationGroup(*animations, lag_ratio=lag)


class Beat3_SameObject(_visuals.ClipScene):
    """📚 Animate one wall of the Villa Savoye through four drawings, three stored representations and one design change."""

    TITLE = {"en": "One Object, Many Views", "de": "Ein Objekt, viele Darstellungen"}
    NARRATION = [
        ["source", "Teaching model of the Villa Savoye: one object, the northwest wall of the first floor with its ribbon window.",
         "Lehrmodell der Villa Savoye: ein Objekt, die Nordwestwand des Obergeschosses mit ihrem Bandfenster."],
        ["plan", "The plan cuts the wall at window height: the wall in poché, the ribbon window as thin glass lines.",
         "Der Grundriss schneidet die Wand auf Fensterhöhe: Wand als Poché, Bandfenster als dünne Glaslinien."],
        ["section", "The section cuts through slab, pilotis and window: sill and head heights become dimension lines.",
         "Der Schnitt geht durch Geschossdecke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien."],
        ["elevation", "The elevation shows the northwest façade and the asymmetric solarium screen with its landscape aperture and sloping cap.",
         "Die Ansicht zeigt die Nordwestfassade und die asymmetrische Solariumswand mit Landschaftsöffnung und geneigter Abdeckung."],
        ["three", "The 3D view shows occupied wall leaves, separate structural members, and an opening filled by frame and glass.",
         "Die 3D-Ansicht zeigt feste Wandschalen, getrennte Tragglieder und eine mit Rahmen und Glas gefüllte Öffnung."],
        ["stored", "In this model the wall is stored as Axis, FootPrint and Body; plan, section and elevation are derived from it.",
         "Im Modell ist die Wand als Axis, FootPrint und Body gespeichert; Grundriss, Schnitt und Ansicht sind abgeleitet."],
        ["lod", "Three levels of detail for one wall: Axis as a line, FootPrint as an outline, Body as a solid with layers.",
         "Drei Detailstufen einer Wand: Axis als Linie, FootPrint als Umriss, Body als Volumen mit Schichten."],
        ["scale200", "At 1:200 the outline is enough: on paper the wall is a thin line and its layers vanish.",
         "Im Maßstab 1:200 genügt der Umriss: Auf dem Papier ist die Wand eine dünne Linie, ihre Schichten verschwinden."],
        ["scale20", "At 1:20 the plan needs the body: the layers of the wall become visible, millimetre by millimetre.",
         "Im Maßstab 1:20 braucht der Plan den Body: Die Schichten der Wand werden Millimeter für Millimeter sichtbar."],
        ["sweep", "Move the cut plane: the plan changes from solid wall to window band and back, because it is derived.",
         "Wandert die Schnittebene, wechselt der Grundriss von Vollwand zu Fensterband und zurück: Er ist abgeleitet."],
        ["derived", "The same geometry gives the areas: gross wall, opening, net wall and the glazing share.",
         "Dieselbe Geometrie liefert die Flächen: Bruttowandfläche, Öffnung, Nettowandfläche und Glasanteil."],
        ["update", "The sill drops from 0.89 to 0.70 m; the head stays at 2.04 m. The opening grows from 1.15 to 1.34 m.",
         "Die Brüstung sinkt von 0,89 auf 0,70 m; der Sturz bleibt bei 2,04 m. Die Öffnung wächst von 1,15 auf 1,34 m."],
        ["takeoff", "The take-off distinguishes rough openings from actual glass; net wall area and glazing share follow the sill.",
         "Die Mengenermittlung unterscheidet Rohbauöffnungen und tatsächliches Glas; Nettowandfläche und Glasanteil folgen der Brüstung."],
        ["volume", "The wall volume counts solid leaves and excludes the air cavity; the lowered sill removes more solid material.",
         "Das Wandvolumen zählt feste Schalen ohne Lufthohlraum; die tiefere Brüstung entfernt zusätzliches festes Material."],
        ["copies", "Copied drawings would stay behind: each copy needs manual coordination, view by view.",
         "Kopien würden zurückbleiben: Jede Kopie müsste von Hand abgeglichen werden, Ansicht für Ansicht."],
        ["drafted", "A line drafted only in the plan is not in the model: the other views do not know it.",
         "Eine nur im Grundriss gezeichnete Linie steht nicht im Modell: Die anderen Ansichten kennen sie nicht."],
        ["regenerate", "On regeneration the drafted line is lost: a view shows the model and keeps nothing of its own.",
         "Bei der Neuerzeugung geht die gezeichnete Linie verloren: Eine Ansicht zeigt das Modell und speichert nichts."],
        ["identity", "A representation shows the object; a design change updates it, and its GlobalId stays the same.",
         "Eine Darstellung zeigt das Objekt; eine Planänderung aktualisiert es, und seine GlobalId bleibt dieselbe."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("representations")

    def present_representations(self):
        """🧭 Tell the whole chapter: source, four drawings, stored representations, derived areas, design change, copies, identity."""
        self._setup()
        self._beat_source()
        self._beat_plan()
        self._beat_section()
        self._beat_elevation()
        self._beat_three()
        self._beat_stored()
        self._beat_lod()
        self._beat_scale200()
        self._beat_scale20()
        self._beat_sweep()
        self._beat_derived()
        self._beat_update()
        self._beat_takeoff()
        self._beat_volume()
        self._beat_copies()
        self._beat_drafted()
        self._beat_regenerate()
        self._beat_identity()

    def _setup(self):
        """🧱 Read every value of the chapter from the model once: base revision, demo event, changed revision."""
        base = _model.load_model()
        event = _model.demo_event("sill", base)
        new = _model.apply_event(event, base)
        geo = _model.building_geometry(base)
        wall = next(w for w in geo["walls"] if w["tag"] == TAG)
        op = wall["openings"][0]
        element = _model.element(TAG, base)
        opening = _model.element(op["tag"], base)
        layers = element["material"]["layers"]
        self.m = SimpleNamespace(
            base=base, new=new, event=event, geo=geo, wall=wall, op=op, op1=next(w for w in _model.building_geometry(new)["walls"] if w["tag"] == TAG)["openings"][0],
            window=op["fill_tag"], opening_tag=op["tag"], q0=_model.quantities(TAG, base), q1=_model.quantities(TAG, new), element=element, layers=layers,
            window_element=_model.element(op["fill_tag"], base), cache={},
            basis={"length": base["building"]["box"]["length"]["basis"], "sill": opening["attributes"]["sill"]["basis"], "head": opening["attributes"]["head"]["basis"],
                   "start": opening["attributes"]["start"]["basis"], "end": opening["attributes"]["end"]["basis"],
                   "thickness": "assumed" if any(layer["thickness"]["basis"] == "assumed" for layer in layers) else ("inferred" if any(layer["thickness"]["basis"] == "inferred" for layer in layers) else "sourced"),
                   "levels": "assumed" if any(s["elevation"]["basis"] == "assumed" for s in base["storeys"]) else "sourced"},
            cut=_model.represent(TAG, "plan", base)["derived"]["cut_z"], states={})
        self.slots, self.thumbs, self.legend, self.hero, self.lives = {}, {}, None, _engine.VGroup(), []
        self.trail = self.chip = self.card = self.sub = None

    def _geo(self, model):
        """🗺️ Geometry of one model state, memoized per state."""
        cache = self.m.cache
        if id(model) not in cache:
            cache[id(model)] = (model, _model.building_geometry(model))
        return cache[id(model)][1]

    def _wall_of(self, model):
        """🪟 The northwest wall and its ribbon opening in one model state."""
        wall = next(w for w in self._geo(model)["walls"] if w["tag"] == TAG)
        return wall, wall["openings"][0]

    def _state(self, k: int):
        """🧮 The model after the design change, with the sill moved ``k / STEPS`` of the way."""
        states = self.m.states
        if k not in states:
            sill = self.m.op["sill"] + (self.m.event["sill"] - self.m.op["sill"]) * k / STEPS
            states[k] = _model.apply_event({**self.m.event, "sill": sill}, self.m.base) if k else self.m.base
        return states[k]

    def _basis(self, name: str) -> str:
        """⭐ Mark a value that is assumed in the teaching model."""
        return "*" if self.m.basis.get(name) in ("assumed", "inferred") else ""

    def _star(self, text: str, basis: str) -> str:
        """⭐ Append the assumption mark to a value text."""
        return text + ("*" if basis in ("assumed", "inferred") else "")

    def _sign(self, value: float, digits: int = 2) -> str:
        """🔻 Elevation text with sign: ±0.00, +3.50."""
        digits=max(digits,3 if abs(value-round(value,2))>1e-9 else 2)
        return ("±" if abs(value)<1e-9 else "+" if value>0 else "−")+self.number(abs(value),digits)

    def _percent(self, value: float, digits: int = 0) -> str:
        """％ Language-aware percentage."""
        return self.number(value * 100, digits) + (NARROW + "%" if self.language() == "de" else "%")

    def _name(self, item: dict) -> str:
        """🏷️ Localized name of a model object."""
        return item["name"][self.language()]

    def _draw(self, kind: str, point, scale: float, model=None, thumb: bool = False, **extra):
        """🖼️ One toolkit drawing of the villa with the wall in yellow and the ribbon window in cyan."""
        lit = (TAG, self.m.window)
        colors = {TAG: DATA, self.m.window: GLASS}
        if kind == "plan":
            return self.villa_plan("first", point, scale, model=model, highlight=lit, highlight_colors=colors, thumbnail=thumb, **({"label_sides": ("top", "right")} | extra))
        if kind == "section":
            return self.villa_section(point, scale, model=model, cut="salon", highlight=lit, highlight_colors=colors, thumbnail=thumb, **extra)
        if kind == "elevation":
            return self.villa_elevation("south", point, scale, model=model, highlight=lit, highlight_colors=colors, thumbnail=thumb, **({"levels": False} | extra))
        return self.villa_axon(point, scale, model=model, highlight=lit, **extra)

    def _place(self, kind: str, centre, width: float, height: float, model=None, thumb: bool = False, **extra):
        """📐 Screen point and scale at which a drawing fills a box centred on ``centre``."""
        centre = np.asarray(P(centre), float)
        first = self._draw(kind, centre, 0.2, model, thumb, **extra)
        k = min(width / first.width, height / first.height)
        offset = (np.array(first.get_center()) - centre) * k
        return centre - offset, 0.2 * k

    def _fit(self, kind: str, centre, width: float, height: float, model=None, thumb: bool = False, **extra):
        """📐 Build a drawing at the largest scale that fits a box and centre it there."""
        point, scale = self._place(kind, centre, width, height, model, thumb, **extra)
        return self._draw(kind, point, scale, model, thumb, **extra)

    def _break(self, a, b, teeth: int = 5, amp: float = 0.06, color=INK):
        """〰️ A break line between two screen points."""
        a, b = np.asarray(P(a), float), np.asarray(P(b), float)
        d = b - a
        n = np.array([-d[1], d[0], 0.0]) / max(np.linalg.norm(d), 1e-9)
        points = [a] + [a + d * (i / (2 * teeth)) + n * amp * (1 if i % 2 else -1) for i in range(1, 2 * teeth)] + [b]
        return _drafting.polyline(points, color, W_THIN)

    def _prune(self, drawing, x_edge: float):
        """✂️ Remove every part of a drawing that lies entirely right of a crop edge."""
        for member in list(drawing.get_family()):
            for child in list(member.submobjects):
                if (child.has_points() or child.submobjects) and child.get_left()[0] > x_edge + 0.02:
                    member.remove(child)

    def _loupe(self, centre, radius: float, target, color=MUTED):
        """🔍 A ring on a drawing with a thin line to the detail it magnifies."""
        c, t = np.asarray(P(centre), float), np.asarray(P(target), float)
        start = c + (t - c) / max(np.linalg.norm(t - c), 1e-9) * radius
        return _engine.VGroup(_engine.Circle(radius=radius, stroke_color=color, stroke_width=W_EDGE).move_to(c), _drafting.segments([(start, t)], color, W_THIN))

    def _head(self, key: str):
        """📑 Swap the section heading chip of the beat; returns the animations."""
        new = _fonts.section_heading(self.tr(*HEADINGS[key]))
        old, self.sub = self.sub, new
        return [_engine.FadeIn(new)] + ([_engine.FadeOut(old)] if old is not None else [])

    def _dimension(self, *args, **kwargs):
        """↔️ A toolkit dimension whose label is pushed a little further from the end ticks."""
        shape = self.dimension(*args, **kwargs)
        label = getattr(shape, "label", None)
        if label is not None:
            v = np.array(label.get_center()) - np.array(shape.line.get_center())
            v[2] = 0.0
            shape.label.shift(v / max(np.linalg.norm(v), 1e-9) * 0.06)
        return shape

    def _layer_name(self, layer: dict) -> str:
        """🏷️ Localized layer name with a bullet instead of brackets."""
        return re.sub(r"\s*\((.*?)\)", r" · \1", self._name(layer))

    def _grow_dimension(self, start, end, tracker, value: float, unit: str, *, offset: float, digits: int = 2, basis: str = "", color=DATA, size=S_SMALL, flip: bool = False, text_at: float = 0.5):
        """↔️ A dimension line that grows from its start while its number counts up from zero."""
        a, b = np.asarray(P(start), float), np.asarray(P(end), float)

        def build():
            f = tracker.get_value()
            if f < 0.02:
                return _engine.VGroup()
            return self._dimension(a, a + (b - a) * f, self.quantity(value * f, unit, digits) + (basis if f > 0.98 else ""), offset=offset, color=color, size=size, flip=flip, text_at=text_at)

        return self.live(build, key=lambda: round(tracker.get_value(), 3))

    def _legend(self, right: bool = False):
        """⭐ Footnote of the assumption mark."""
        point, anchor = ((6.85, -2.47, 0), "r") if right else ((-6.85, -2.47, 0), "l")
        return self.label("* assumed or inferred in the teaching model", "* im Lehrmodell angenommen oder abgeleitet", point, size=S_SMALL, color=MUTED, anchor=anchor)

    def _chip(self, point=(RIGHT, 2.72, 0), model=None):
        """🆔 GlobalId chip of the wall."""
        return self.guid_chip(TAG, point, model=model)

    def _card(self):
        """🧾 Object card: the IFC properties of the wall with their localized meaning, the data that never change from view to view."""
        m = self.m
        language = self.language()
        words = {"sourced": (GOOD, "sourced", "Quelle"), "assumed": (WARN, "assumed", "angenommen"), "inferred": (ARCH, "inferred", "gefolgert")}
        short = {"Pset_WallCommon.IsExternal": "grenzt an die Außenluft", "Pset_WallCommon.LoadBearing": "trägt Lasten", "Pset_WallCommon.FireRating": "Feuerwiderstandsklasse"}
        rows = [("TotalThickness", self.tr("layer set · total", "Schichtaufbau · Summe"), self.quantity(m.q0["thickness"], "m"), m.basis["thickness"])]
        for pid in ("Pset_WallCommon.IsExternal", "Pset_WallCommon.LoadBearing", "Pset_WallCommon.FireRating"):
            item = _model.information(TAG, pid, language, m.base)
            value = item["value"]
            rows.append((pid, short[pid] if language == "de" else item["meaning"], ("TRUE" if value else "FALSE") if isinstance(value, bool) else str(value), item["basis"]))
        frame = _engine.RoundedRectangle(corner_radius=0.06, width=4.9, height=2.3, stroke_color=GOOD, stroke_width=W_THIN, fill_color=BACKGROUND, fill_opacity=0.9).move_to((RIGHT, -1.35, 0))
        group = _engine.VGroup(frame, self.text(f"IfcWall · {self._name(_model.element(m.element['type'], m.base))}", (2.1, -0.42, 0), size=S_LABEL, color=GOOD, anchor="l", width=3.1),
                               self.label("stays the same", "bleibt gleich", (6.8, -0.42, 0), size=S_SMALL, color=MUTED, anchor="r"),
                               _drafting.segments([(P((2.0, -0.62)), P((6.9, -0.62)))], GOOD, W_THIN))
        for index, (name, meaning, value, basis) in enumerate(rows):
            y = -0.86 - 0.44 * index
            tone, en, de = words[basis]
            group.add(self.text(name, (2.1, y, 0), size=S_SMALL, color=INK, anchor="l", width=3.3), self.text(value, (6.8, y, 0), size=S_SMALL, color=DATA, anchor="r"),
                      self.text(meaning, (2.1, y - 0.19, 0), size=S_SMALL, color=MUTED, anchor="l", width=3.25), self.label(en, de, (6.8, y - 0.19, 0), size=S_SMALL, color=tone, anchor="r"))
        group.table = frame
        return group

    def _trail_group(self):
        """🧭 Where the wall lives: project, site, building, storey and the wall itself, with the names of the model."""
        m = self.m
        base = m.base
        storey = next(s for s in base["storeys"] if s["id"] == m.element["storey"])
        rows = [("IfcProject", self._name(base["project"])), ("IfcSite", self._name(base["site"])), ("IfcBuilding", self._name(base["building"])),
                ("IfcBuildingStorey", self._name(storey)), ("IfcWall", f"{self._name(m.element)} · {TAG}")]
        group = _engine.VGroup()
        ys = [2.03 - 0.43 * i for i in range(len(rows))]
        group.add(_drafting.segments([(P((2.25, ys[0])), P((2.25, ys[-1])))], MUTED, W_THIN))
        for index, ((cls, name), y) in enumerate(zip(rows, ys)):
            tone = DATA if index == len(rows) - 1 else ARCH
            group.add(_engine.Dot(P((2.25, y)), radius=0.06, color=tone), self.text(cls, (2.45, y, 0), size=S_SMALL, color=tone, anchor="l"),
                      self.text(name, (4.3, y, 0), size=S_SMALL, color=INK, anchor="l", width=2.45))
        return group

    def _key_chips(self, point):
        """🔑 Colour key: yellow is the wall, cyan is the window, with their class, tag and name from the model."""
        m = self.m
        group = _engine.VGroup()
        x, y = point[0], point[1]
        for ((cls, tag), tone), item in zip(CHIP_ROWS, (m.element, m.window_element)):
            group.add(_engine.Rectangle(width=0.22, height=0.16, stroke_color=tone, stroke_width=W_THIN, fill_color=tone, fill_opacity=0.8).move_to((x + 0.11, y, 0)))
            text = self.text(f"{cls} · {tag}  {self._name(item)}", (x + 0.3, y, 0), size=S_SMALL, color=tone, anchor="l")
            group.add(text)
            x += 0.3 + text.width + 0.35
        return group

    def _slot(self, kind: str):
        """🗂️ Dashed slot frame with its label above."""
        x, y, _ = np.asarray(P(SLOTS[kind]), float)
        frame = _engine.Rectangle(width=SLOT_W, height=SLOT_H, stroke_color=MUTED, stroke_width=W_THIN, fill_opacity=0).move_to((x, y, 0))
        frame.set_stroke(opacity=0.8)
        en, de = SLOT_NAMES[kind]
        label = self.label(en, de, (x - SLOT_W / 2, y + SLOT_H / 2 + 0.13, 0), size=S_SMALL, color=MUTED, anchor="l")
        group = _engine.VGroup(frame, label)
        group.frame, group.label = frame, label
        return group

    def _thumb(self, kind: str, model=None):
        """🖼️ Thumbnail of a view inside its slot."""
        return self._fit(kind, SLOTS[kind], SLOT_W - 0.2, SLOT_H - 0.14, model, True)

    def _fill(self, kind: str, model=None):
        """🖼️ Animations that put the thumbnail of the beat's own view into its slot; the first call also shows all four slot frames and clears the spatial trail."""
        thumb = self._thumb(kind, model)
        fresh = []
        if not self.slots:
            self.slots = {name: self._slot(name) for name in SLOTS}
            fresh = list(self.slots.values())
        gone = []
        if self.trail is not None:
            gone, self.trail = [_engine.FadeOut(self.trail)], None
        self.thumbs[kind] = thumb
        return [*[_engine.FadeIn(s) for s in fresh], _engine.FadeIn(thumb), *gone]

    def _leave(self):
        """🚢 Take the current hero off the stage; live dimensions fade with it."""
        hero, lives = self.hero, self.lives
        self.hero, self.lives = _engine.VGroup(), []
        return _engine.AnimationGroup(_engine.FadeOut(hero), *[_engine.FadeOut(live) for live in lives])

    def _stage(self, hero, lives=()):
        """🎭 Register the new hero drawing and its live parts; the returned animation puts the live parts on stage above the hero, where they draw themselves."""
        self.hero, self.lives = hero, list(lives)
        return _engine.Add(*self.lives)

    def _column(self):
        """📌 Everything that stays in the right column."""
        parts = [self.chip, self.card, *self.slots.values(), *self.thumbs.values()]
        return _engine.VGroup(*[p for p in parts if p is not None])

    def _beat_source(self):
        """1️⃣ The whole villa with the northwest wall and its ribbon window lit, the spatial path, the GlobalId chip and the object card."""
        m = self.m
        axon = self._fit("3d", (-3.95, 0.15, 0), 5.9, 4.6, m.base)
        op, wall = m.op, m.wall
        wall_point = axon.at(op["end"] + 0.7, 0, wall["base"] + 0.5)
        window_point = axon.at(op["end"] - 2.4, 0, (op["sill_z"] + op["head_z"]) / 2)
        wall_at, window_at = np.asarray(P((-1.05, -1.1)), float), np.asarray(P((-1.05, 1.4)), float)
        callouts = _engine.VGroup(
            self.leader(window_at, window_point, None, color=GLASS), self.leader(wall_at, wall_point, None, color=DATA),
            self.text(f"IfcWindow · {m.window}", window_at + (0.1, 0.17, 0), size=S_LABEL, color=GLASS, anchor="l"),
            self.text(self._name(m.window_element), window_at + (0.1, -0.12, 0), size=S_SMALL, color=MUTED, anchor="l"),
            self.text(f"{self.quantity(op['end'] - op['start'], 'm')} × {self.quantity(op['head'] - op['sill'], 'm')}{self._basis('sill')}", window_at + (0.1, -0.38, 0), size=S_SMALL, color=MUTED, anchor="l"),
            self.text(f"IfcWall · {TAG}", wall_at + (0.1, 0.17, 0), size=S_LABEL, color=DATA, anchor="l"),
            self.text(self._name(m.element), wall_at + (0.1, -0.12, 0), size=S_SMALL, color=MUTED, anchor="l"),
            self.text(f"{self.quantity(m.q0['length'], 'm')} × {self.quantity(m.q0['height'], 'm')}{self._basis('levels')}", wall_at + (0.1, -0.38, 0), size=S_SMALL, color=MUTED, anchor="l"))
        north = axon.at(-1, -1, 0) - axon.at(0, 0, 0)
        north = north / np.linalg.norm(north)
        origin = np.asarray(P((-6.0, -1.9)), float)
        compass = _engine.VGroup(_drafting.segments([(origin, origin + north * 0.5)], INK, W_EDGE), self.text("N", origin + north * 0.75, size=S_SMALL, color=INK),
                                 self.badge(self.tr("teaching orientation", "Lehrorientierung"), origin + np.array([0.55, -0.02, 0]), color=MUTED, filled=False, anchor="l"))
        self.chip, self.card, self.trail = self._chip(), self._card(), self._trail_group()
        self.legend = self._legend()
        simplification = self.label("other façades: ribbon windows are a teaching simplification", "andere Fassaden: Bandfenster sind eine Lehrvereinfachung", (-6.8, -2.2, 0), size=S_SMALL, color=MUTED, anchor="l", width=6.0)
        self._stage(_engine.VGroup(axon, callouts, compass, simplification))
        self.step("source", *self._head("source"), _engine.Succession(seq(_engine.FadeIn(axon), _engine.FadeIn(callouts), _engine.FadeIn(compass), _engine.FadeIn(simplification), _engine.FadeIn(self.legend), _engine.FadeIn(self.chip), _engine.FadeIn(self.trail), _engine.FadeIn(self.card), lag=0.4),
                                               _engine.Indicate(axon.parts[TAG], color=DATA)), seconds=4.2)

    def _plan_detail(self):
        """🔩 Plan cut through the wall at the east end of the ribbon window: poché in the build-up layers, glass lines, thickness and pier length."""
        m = self.m
        wall, op = m.wall, m.op
        k, x0, y0 = 1.0, -1.32, 1.95
        x_from = op["end"] - 0.4
        sx = lambda x: x0 + k * (x - x_from)
        thick = m.q0["thickness"]
        frame = _engine.Rectangle(width=3.3, height=2.6, stroke_color=MUTED, stroke_width=W_THIN, fill_opacity=0).move_to((0.08, 1.77, 0))
        frame.set_stroke(opacity=0.7)
        group = _engine.VGroup(frame, self.label("detail · wall cut at the window", "Detail · Wand im Fensterschnitt", (0.08, 2.88, 0), size=S_SMALL, color=MUTED, width=3.1))
        cursor = 0.0
        for layer in m.layers:
            t = layer["thickness"]["value"]
            tone = LAYER_TONES.get(layer["id"], MUTED)
            ring = [P((sx(op["end"]), y0 + cursor * k)), P((sx(wall["length"]), y0 + cursor * k)), P((sx(wall["length"]), y0 + (cursor + t) * k)), P((sx(op["end"]), y0 + (cursor + t) * k))]
            group.add(_drafting.polygon(ring, tone, 0.0 if layer.get("is_void", False) else 0.8, tone, W_THIN))
            if layer["id"] in ("brick", "outer-leaf", "inner-leaf") and not layer.get("is_void", False):
                group.add(self.hatch([ring], INK, 0.07, 45))
            cursor += t
        glass_y = y0 + thick * k / 2
        for sign in (1, -1):
            group.add(_drafting.polyline([P((sx(x_from), glass_y + sign * thick * k * 0.28)), P((sx(op["end"]), glass_y + sign * thick * k * 0.28))], GLASS, W_EDGE))
        group.add(_drafting.segments([(P((sx(op["end"]), y0)), P((sx(op["end"]), y0 + thick * k)))], GLASS, W_EDGE),
                  _drafting.polygon([P((sx(x_from), y0)), P((sx(op["end"]), y0)), P((sx(op["end"]), y0 + thick * k)), P((sx(x_from), y0 + thick * k))], GLASS, 0.12, GLASS, W_HAIR),
                  self._break((sx(x_from), y0 - 0.1), (sx(x_from), y0 + thick * k + 0.1), 3, 0.05))
        group.add(self._dimension(P((sx(wall["length"]), y0)), P((sx(wall["length"]), y0 + thick * k)), self.quantity(thick, "m", 2) + self._basis("thickness"), offset=-0.14, color=DATA),
                  self._dimension(P((sx(op["end"]), y0)), P((sx(wall["length"]), y0)), self.quantity(wall["length"] - op["end"], "m", 2) + self._basis("end"), offset=-0.22, color=DATA))
        group.add(self.leader(P((sx(x_from) - 0.05, y0 + thick * k + 0.3)), P((sx(x_from) + 0.2, glass_y)), None, color=GLASS),
                  self.text(self.tr("glass line", "Glaslinie"), (sx(x_from) - 0.05, y0 + thick * k + 0.42, 0), size=S_SMALL, color=GLASS, anchor="l"),
                  self.text(self.tr("wall in poché", "Wand als Poché"), (sx(wall["length"]) - 0.05, y0 + thick * k + 0.18, 0), size=S_SMALL, color=INK, anchor="r"))
        for index, layer in enumerate(m.layers):
            y = y0 - 0.78 - 0.26 * index
            tone = LAYER_TONES.get(layer["id"], MUTED)
            group.add(_engine.Rectangle(width=0.2, height=0.14, stroke_color=tone, stroke_width=W_THIN, fill_color=tone, fill_opacity=0.0 if layer.get("is_void", False) else 0.8).move_to((-1.3, y, 0)),
                      self.text(f"{self._layer_name(layer)}  {self.quantity(layer['thickness']['value'], 'm', 3)}{self._star('', layer['thickness']['basis'])}", (-1.15, y, 0), size=S_SMALL, color=tone, anchor="l", width=2.6))
        return group

    def _cut_key(self, model, origin=(-1.35, -2.25), k: float = 0.55, length: float = 2.6, cut: float | None = None):
        """📏 Elevation key of the wall between slab and roof: where the plan cut meets the window band, with sill, head and cut level."""
        wall, op = self._wall_of(model)
        geo = self._geo(model)
        slab = next(s for s in geo["slabs"] if s["tag"] == "SL-1")
        roof = next(s for s in geo["slabs"] if s["tag"] == "SL-R")
        z_lo, z_hi = slab["top"] - slab["thickness"], roof["top"]
        moving = cut is not None
        cut = _model.represent(TAG, "plan", model)["derived"]["cut_z"] if cut is None else cut
        sx, sy = (lambda x: origin[0] + k * x), (lambda z: origin[1] + k * (z - z_lo))
        rect = lambda x0, x1, z0, z1: [P((sx(x0), sy(z0))), P((sx(x1), sy(z0))), P((sx(x1), sy(z1))), P((sx(x0), sy(z1)))]
        group = _engine.VGroup()
        for ring in (rect(0, length, z_lo, slab["top"]), rect(0, length, wall["top"], z_hi)):
            group.add(_drafting.polygon(ring, CONCRETE, 0.35, CONCRETE, W_THIN), self.hatch([ring], CONCRETE, 0.07, 45))
        group.add(_drafting.polygon(rect(0, op["start"], wall["base"], wall["top"]), DATA, 0.75, DATA, W_THIN),
                  _drafting.polygon(rect(op["start"], length, wall["base"], op["sill_z"]), DATA, 0.75, DATA, W_THIN),
                  _drafting.polygon(rect(op["start"], length, op["head_z"], wall["top"]), DATA, 0.75, DATA, W_THIN),
                  _drafting.polygon(rect(op["start"], length, op["sill_z"], op["head_z"]), GLASS, 0.3, GLASS, W_EDGE))
        group.add(_drafting.polyline([P((sx(-0.15), sy(cut))), P((sx(length + 0.15), sy(cut)))], INK, W_EDGE, dash=0.1))
        x = sx(length) + 0.16
        for name_en, name_de, z, tone in (("head", "Sturz", op["head_z"], MUTED), ("cut", "Schnitt", cut, INK), ("sill", "Brüstung", op["sill_z"], MUTED))[::2 if moving else 1]:
            group.add(self.text(f"{self.tr(name_en, name_de)} {self._sign(z)}{self._basis('levels')}", (x, sy(z), 0), size=S_SMALL, color=tone, anchor="l"))
        if moving:
            group.add(self._dimension(P((sx(0), sy(wall["base"]))), P((sx(0), sy(cut))), self.quantity(cut - wall["base"], "m", 2), offset=0.3, color=DATA))
        group.add(self.label("where the plan cuts", "Wo der Grundriss schneidet", (origin[0] + 1.35, sy(z_hi) + 0.22, 0), size=S_SMALL, color=MUTED, width=3.0))
        return group

    def _beat_plan(self):
        """2️⃣ First-floor plan: the northwest wall lit, window and wall length dimensions, a magnified poché detail and the cut level."""
        m = self.m
        leave = self._leave()
        plan = self._fit("plan", (-4.3, 1.0, 0), 4.9, 3.9, m.base, grid=True, labels=True)
        wall, op = m.wall, m.op
        f1, f2 = _engine.ValueTracker(0), _engine.ValueTracker(0)
        window_dim = self._grow_dimension(plan.at(op["start"], 0, 0), plan.at(op["end"], 0, 0), f1, op["end"] - op["start"], "m", offset=-0.42, basis=self._basis("start"))
        wall_dim = self._grow_dimension(plan.at(0, 0, 0), plan.at(wall["length"], 0, 0), f2, wall["length"], "m", offset=-0.88, basis=self._basis("length"))
        loupe = self._loupe(plan.at(op["end"], 0.15, 0), 0.22, (-1.55, 1.9))
        detail, key = self._plan_detail(), self._cut_key(m.base)
        add = self._stage(_engine.VGroup(plan, loupe, detail, key), (window_dim, wall_dim))
        self.step("plan", *self._head("plan"), *self._fill("plan"), seq(leave, _engine.FadeIn(plan), add, f1.animate.set_value(1), f2.animate.set_value(1), _engine.FadeIn(loupe), _engine.FadeIn(detail), _engine.FadeIn(key), lag=0.35), seconds=5.0)

    def _wall_cut(self, trackers):
        """🔩 Section cut through the wall at the window at detail scale: slab, piloti stub, build-up layers, glass and the sill, window and head dimension chain."""
        m = self.m
        wall, op, geo = m.wall, m.op, m.geo
        slab = next(s for s in geo["slabs"] if s["tag"] == "SL-1")
        roof = next(s for s in geo["slabs"] if s["tag"] == "SL-R")
        z_lo = slab["top"] - slab["thickness"]
        k, xw, y0 = 1.2, -5.55, -1.62
        sy, sx = (lambda z: y0 + k * (z - z_lo)), (lambda depth: xw + k * depth)
        thick = m.q0["thickness"]
        rect = lambda d0, d1, z0, z1: [P((sx(d0), sy(z0))), P((sx(d1), sy(z0))), P((sx(d1), sy(z1))), P((sx(d0), sy(z1)))]
        frame = _engine.Rectangle(width=3.05, height=5.2, stroke_color=MUTED, stroke_width=W_THIN, fill_opacity=0).move_to((-5.35, 0.3, 0))
        frame.set_stroke(opacity=0.7)
        group = _engine.VGroup(frame, self.label("detail · wall cut at the window", "Detail · Wand im Fensterschnitt", (-5.35, 2.7, 0), size=S_SMALL, color=MUTED, width=2.85))
        station=(op["start"]+op["end"])/2
        origin=np.asarray(wall["start"],dtype=float)
        direction=(np.asarray(wall["end"])-origin)/wall["length"]
        axis=0 if abs(direction[0])>abs(direction[1]) else 1
        at=origin+station*direction
        filling=next(item for item in geo["fillings"] if item["tag"]==m.window)
        auxiliary=[item for item in geo["beams"] if _model.element(item["tag"],m.base).get("geometry",{}).get("section_sweep",{}).get("wall")==TAG]
        for item in [wall,filling,slab,roof,*auxiliary]:
            for part in item["parts"]:
                for section in _model.part_sections(part,float(at[axis]),axis):
                    local=[]
                    for horizontal,z in section:
                        xy=[float(at[0]),horizontal] if axis==0 else [horizontal,float(at[1])]
                        local.append([self._wall_local_ring([xy])[0][1],z])
                    local=self._clip_station(local,0)
                    local=[[-d,z] for d,z in self._clip_station([[-d,z] for d,z in local],-1.45)]
                    if not local:
                        continue
                    ring=[P((sx(d),sy(z))) for d,z in local]
                    tone=GLASS if part.get("role")=="glass" else (LAYER_TONES.get(part.get("layer"),CONCRETE) if item is wall else ARCH if item is filling else CONCRETE)
                    group.add(_drafting.polygon(ring,tone,.55 if tone==GLASS else .8,tone,W_THIN))
                    if tone!=GLASS:
                        group.add(self.hatch([ring],INK,.07,45))
        marks = [(wall["base"], op["sill_z"], op["sill"], "sill"), (op["sill_z"], op["head_z"], op["head"] - op["sill"], None), (op["head_z"], wall["top"], wall["top"] - op["head_z"], None)]
        dims = [self._grow_dimension(P((xw, sy(z0))), P((xw, sy(z1))), t, value, "m", offset=0.3, basis=self._basis("sill") if index == 0 else (self._basis("head") if index == 1 else "*")) for index, (t, (z0, z1, value, _)) in enumerate(zip(trackers[:3], marks))]
        dims.append(self._grow_dimension(P((sx(thick), sy(wall["base"]))), P((sx(thick), sy(wall["top"]))), trackers[3], wall["top"] - wall["base"], "m", offset=-0.3, basis="*"))
        return group, dims

    def _beat_section(self):
        """3️⃣ Section through the salon: the villa's south part with slab, pilotis and window, a magnified wall cut with sill, window and head dimensions, level marks."""
        m = self.m
        leave = self._leave()
        scale, hc, zc, crop = 0.46, 4.0, 4.7, 8.5
        point = np.asarray(P((-3.1 + scale * hc, 0.55)), float)
        section = self._draw("section", point, scale, m.base, labels=False, levels=False, centre=(hc, zc))
        section.set_z_index(-2)
        x_c = section.at(crop, 0)[0]
        self._prune(section, x_c)
        mask = _engine.Rectangle(width=7.5 - x_c, height=5.8, stroke_width=0, fill_color=BACKGROUND, fill_opacity=1).move_to(((x_c + 7.5) / 2, 0.3, 0)).set_z_index(-1)
        edge = self._break((x_c, section.at(0, -0.5)[1]), (x_c, section.at(0, 9.6)[1]), 14, 0.05)
        geo = m.geo
        marks = _engine.VGroup()
        for tag, value, basis in (("ground", geo["storeys"]["ground"], m.basis["levels"]), ("first", geo["storeys"]["first"], m.basis["levels"]), ("roof", geo["storeys"]["roof"], m.basis["levels"]), ("top", geo["box"]["height"], m.basis["levels"])):
            marks.add(self.level_mark(P((x_c + 0.05, section.at(0, value)[1])), self._star(self._sign(value), basis), size=S_SMALL, length=0.1))
        cut = m.cut
        cut_line = _drafting.polyline([section.at(0, cut), section.at(crop, cut)], DATA, W_THIN, dash=0.12)
        cut_label = self.text(self.tr("plan cut ", "Schnittebene ") + self._sign(cut) + self._basis("levels"), section.at(4.6, cut + 1.05), size=S_SMALL, color=DATA)
        loupe = self._loupe(section.at(0.15, cut), 0.2, (-3.85, section.at(0.15, cut)[1]))
        trackers = [_engine.ValueTracker(0) for _ in range(4)]
        detail, dims = self._wall_cut(trackers)
        add = self._stage(_engine.VGroup(section, mask, edge, marks, cut_line, cut_label, loupe, detail), dims)
        self.step("section", *self._head("section"), *self._fill("section"), _engine.Succession(seq(leave, _engine.FadeIn(section), _engine.FadeIn(mask), _engine.FadeIn(edge), _engine.FadeIn(marks), lag=0.2),
                                                seq(_engine.FadeIn(cut_line), _engine.FadeIn(cut_label), _engine.FadeIn(loupe), _engine.FadeIn(detail), add, *[t.animate.set_value(1) for t in trackers], lag=0.3)), seconds=5.4)

    def _beat_elevation(self):
        """4️⃣ South elevation of the whole façade with the sill, window and height chain, the window and wall length dimensions and the colour key."""
        m = self.m
        leave = self._leave()
        elev = self._fit("elevation", (-2.25, 1.2, 0), 6.9, 3.5, m.base)
        wall, op = m.wall, m.op
        ground = elev.at(0, -0.5)[1]
        bottom = elev.at(0, wall["base"])[1]
        trackers = [_engine.ValueTracker(0) for _ in range(6)]
        rows = ((wall["base"], op["sill_z"], op["sill"], "sill"), (op["sill_z"], op["head_z"], op["head"] - op["sill"], "head"), (op["head_z"], wall["top"], wall["top"] - op["head_z"], ""))
        dims = [self._grow_dimension(elev.at(0, z0), elev.at(0, z1), trackers[i], value, "m", offset=0.28, basis=self._basis(name) if name else "*") for i, (z0, z1, value, name) in enumerate(rows)]
        dims.append(self._grow_dimension(elev.at(wall["length"], wall["base"]), elev.at(wall["length"], wall["top"]), trackers[3], wall["top"] - wall["base"], "m", offset=-0.25, basis="*"))
        drop = bottom - ground
        dims.append(self._grow_dimension(elev.at(op["start"], wall["base"]), elev.at(op["end"], wall["base"]), trackers[4], op["end"] - op["start"], "m", offset=-(drop + 0.3), basis=self._basis("start")))
        dims.append(self._grow_dimension(elev.at(0, wall["base"]), elev.at(wall["length"], wall["base"]), trackers[5], wall["length"], "m", offset=-(drop + 0.75), basis=self._basis("length")))
        chips = self._key_chips((-6.8, -2.1))
        self.legend = self._legend() if self.legend is None else self.legend
        add = self._stage(_engine.VGroup(elev, chips), dims)
        self.step("elevation", *self._head("elevation"), *self._fill("elevation"), seq(leave, _engine.FadeIn(elev), _engine.FadeIn(chips), add, *[t.animate.set_value(1) for t in trackers], lag=0.25), seconds=5.2)

    def _wall_overlay(self, axon, e: float, o: float, w: float):
        """🧱 The wall body from the toolkit, aligned on the villa axonometric: extrusion ``e``, opening lifted out ``o``, window slid in ``w`` (all 0..1)."""
        if e < 0.01:
            return _engine.VGroup()
        wall = self.m.wall
        drawing = self.element_axon(TAG, (0, 0, 0), axon.view.scale, model=self.m.base, extrude=e, opening=o, window=w, highlight=(TAG,), colors="material")
        drawing.shift(axon.at(0, 0, wall["base"]) - drawing.at(0, 0, wall["base"]))
        return drawing

    def _chain(self, y: float = -1.68):
        """🔗 Wall, opening and window with the two IFC relations; class names and tags come from the relations of the wall and the window."""
        m = self.m
        voids = next(r for r in _model.relations_of(TAG, m.base) if r["class"] == "IfcRelVoidsElement")
        fills = next(r for r in _model.relations_of(m.window, m.base) if r["class"] == "IfcRelFillsElement")
        side = voids["others"][0]
        spec = ((-5.95, 1.9, "IfcWall", TAG, DATA, ("solid body", "massiver Körper")), (-2.55, 2.6, side["class"], side["tag"], MUTED, ("void in the wall", "Aussparung in der Wand")),
                (0.6, 2.3, "IfcWindow", m.window, GLASS, ("fills the void", "füllt die Aussparung")))
        boxes = [self.class_box(name, [tag, (self.tr(*note), MUTED)], (x, y, 0), color=tone, width=width) for x, width, name, tag, tone, note in spec]
        group = _engine.VGroup(*boxes)
        for a, b, name in ((boxes[0], boxes[1], voids["class"]), (boxes[1], boxes[2], fills["class"])):
            start, end = np.array(a.get_right()) + (0.05, 0.1, 0), np.array(b.get_left()) - (0.05, -0.1, 0)
            group.add(_engine.Arrow(start, end, buff=0, color=DATA, stroke_width=2.4, max_tip_length_to_length_ratio=0.25), self.text(name, (start + end) / 2 + np.array([0, 0.62, 0]), size=S_SMALL, color=DATA, width=(end - start)[0] + 0.9))
        return _engine.VGroup(group[0], group[3], group[4], group[1], group[5], group[6], group[2])

    def _beat_three(self):
        """5️⃣ The wall as a body inside the villa: extrusion, opening lifted out, window filled in, with the IFC relations."""
        m = self.m
        leave = self._leave()
        axon = self._fit("3d", (-2.65, 1.0, 0), 7.8, 3.7, m.base, ghost=(TAG, m.window))
        e, o, w = (_engine.ValueTracker(0) for _ in range(3))
        solid = self.live(lambda: self._wall_overlay(axon, e.get_value(), o.get_value(), w.get_value()), key=lambda: (round(e.get_value(), 2), round(o.get_value(), 2), round(w.get_value(), 2)))
        f = [_engine.ValueTracker(0) for _ in range(2)]
        wall = m.wall
        dims = (self._grow_dimension(axon.at(0, 0, wall["base"]), axon.at(wall["length"], 0, wall["base"]), f[0], wall["length"], "m", offset=-0.95, basis=self._basis("length")),
                self._grow_dimension(axon.at(wall["length"], 0, wall["base"]), axon.at(wall["length"], 0, wall["top"]), f[1], wall["top"] - wall["base"], "m", offset=-0.05, basis="*"))
        chain = self._chain()
        add = self._stage(_engine.VGroup(axon, chain), (solid, *dims))
        self.step("three", *self._head("three"), *self._fill("3d"), seq(leave, _engine.FadeIn(axon), add, e.animate.set_value(1), f[0].animate.set_value(1), f[1].animate.set_value(1),
                               seq(o.animate.set_value(1), _engine.FadeIn(chain[:3]), lag=0.2), seq(w.animate.set_value(1), _engine.FadeIn(chain[3:]), lag=0.2), lag=0.5), seconds=5.6)

    def _extrusion(self, axon, e: float):
        """⬆️ Actual material-owned wall extrusions, preserving openings and cavity."""
        return self._wall_overlay(axon, e, 1.0, 0.0)

    def _row_axis(self, rep, y: float):
        """➖ Axis row: the actual source wall axis with endpoint coordinates."""
        m = self.m
        scale = 0.2
        view = _drafting.View("plan", (-1.7, y, 0), scale, centre=(m.geo["box"]["length"] / 2, 0.15, 0))
        group = _engine.VGroup()
        points = rep["primitives"][0]["points"]
        group.add(_drafting.polyline([view(*p[:2]) for p in points], ARCH, W_CUT), *[_engine.Dot(view(*p[:2]), radius=0.05, color=ARCH) for p in points])
        for p, anchor in zip((points[0], points[-1]), ("l", "r")):
            group.add(self.text(f"({self.number(p[0], 2)}; {self.number(p[1], 2)})", view(*p[:2]) + np.array([0.0, 0.27, 0.0]), size=S_SMALL, color=ARCH, anchor=anchor))
        return group

    def _row_footprint(self, rep, y: float):
        """▭ Actual occupied leaf polygons in the first three metres of wall-local coordinates."""
        m=self.m
        scale,crop=1.3,3.0
        view=_drafting.View("plan",(-3.95,y,0),scale,centre=(0,0,0))
        origin=np.asarray(m.wall["start"],dtype=float)
        axis=(np.asarray(m.wall["end"])-origin)/m.wall["length"]
        normal=np.array([-axis[1],axis[0]])
        rings=[[[float(np.dot(np.asarray(p)-origin,axis)),float(np.dot(np.asarray(p)-origin,normal))] for p in primitive["points"]]
               for primitive in rep["primitives"] if primitive["kind"]=="polygon"]
        low=min(p[1] for ring in rings for p in ring)
        high=max(p[1] for ring in rings for p in ring)
        shapes=_engine.VGroup()
        for ring in rings:
            if max(p[0] for p in ring)<=0 or min(p[0] for p in ring)>=crop:
                continue
            clipped=self._clip_station(ring,0)
            clipped=[[-x,y] for x,y in self._clip_station([[-x,y] for x,y in clipped],-crop)]
            if not clipped:
                continue
            shapes.add(_drafting.polyline([view(*p) for p in clipped],ARCH,W_CUT,closed=True))
        start=m.op["start"]
        tick=_drafting.polyline([view(start,low-.1),view(start,high+.1)],GLASS,W_EDGE,dash=.1)
        label=self.label("window starts", "Fenster beginnt", view(start,high+.1)+np.array([0,.17,0]),size=S_SMALL,color=GLASS)
        end_break=self._break(view(crop,low-.1),view(crop,high+.1),3,.05)
        dim=self._dimension(view(crop,low),view(crop,high),self.quantity(high-low,"m",3)+self._basis("thickness"),offset=-.3,color=DATA)
        along=self._dimension(view(0,low),view(start,low),self.quantity(start,"m",2)+self._basis("start"),offset=-.22,color=GLASS)
        local=self.label("wall-local station / offset", "Wandlokale Station / Versatz",view(0,low)+np.array([0,-.72,0]),size=S_SMALL,color=MUTED,anchor="l")
        return _engine.VGroup(shapes,tick,label,end_break,dim,along,local)

    def _beat_stored(self):
        """6️⃣ The three stored shape representations of one product, with their real coordinates, and the derived drawings."""
        m = self.m
        leave = self._leave()
        rep = {kind: _model.represent(TAG, kind, m.base) for kind in ("axis", "footprint", "body")}
        title = self.label("one GlobalId · three representations", "eine GlobalId · drei Formdarstellungen", (-6.55, 3.0, 0), size=S_LABEL, color=MUTED, anchor="l")
        ys = {"axis": 2.1, "footprint": 0.6, "body": -1.2}
        sublabels = {"axis": ("centre line", "Mittellinie"), "footprint": ("outline in plan", "Umriss im Grundriss"), "body": ("solid", "Volumenkörper")}
        chips = {kind: self.class_box(rep[kind]["identifier"], [(rep[kind]["type"], DATA), (self.tr(*sublabels[kind]), MUTED)], (-5.5, y, 0), color=ARCH, width=2.2) for kind, y in ys.items()}
        bracket = _drafting.segments([(P((-6.8, ys["axis"] + 0.55)), P((-6.8, ys["body"] - 0.6)))], ARCH, W_EDGE)
        axis, foot = self._row_axis(rep["axis"], ys["axis"] + 0.05), self._row_footprint(rep["footprint"], ys["footprint"] + 0.5)
        body_reps = rep["body"]["primitives"]
        overall_height = m.wall["top"] - m.wall["base"]
        point, scale = self._place("3d", (-3.0, ys["body"], 0), 3.2, 2.3, m.base, ghost=(TAG, m.window))
        axon = self._draw("3d", point, scale, m.base, ghost=(TAG, m.window))
        e = _engine.ValueTracker(0)
        solid = self.live(lambda: self._extrusion(axon, e.get_value()), key=lambda: round(e.get_value(), 2))
        depth = self._grow_dimension(axon.at(m.wall["length"], 0, m.wall["base"]), axon.at(m.wall["length"], 0, m.wall["base"] + overall_height), e, overall_height, "m", offset=-0.05, basis="*")
        nx = -0.5
        note = _engine.VGroup(
            self.label("modelled as", "modelliert als", (nx, -0.3, 0), size=S_SMALL, color=MUTED, anchor="l"),
            self.text(rep["body"]["derived"]["solid"], (nx, -0.57, 0), size=S_SMALL, color=DATA, anchor="l"),
            self.text(rep["body"]["derived"]["profile"], (nx, -0.84, 0), size=S_SMALL, color=DATA, anchor="l"),
            self.text(self.tr("{} physical layer parts", "{} physische Schichtteile").format(len(body_reps)), (nx, -1.15, 0), size=S_SMALL, color=INK, anchor="l"),
            self.label("the window opening is carried by", "die Fensteröffnung trägt", (nx, -1.6, 0), size=S_SMALL, color=MUTED, anchor="l"),
            self.text("IfcOpeningElement", (nx, -1.87, 0), size=S_SMALL, color=MUTED, anchor="l"))
        derived = self._derived_marks()
        hero = _engine.VGroup(title, bracket, *chips.values(), axis, foot, axon, note, derived)
        add = self._stage(hero, (solid, depth))
        self.step("stored", *self._head("stored"), seq(leave, _engine.FadeIn(title), _engine.FadeIn(bracket), _engine.FadeIn(chips["axis"]), _engine.FadeIn(axis), _engine.FadeIn(chips["footprint"]), _engine.FadeIn(foot), _engine.FadeIn(chips["body"]),
                                _engine.FadeIn(axon), add, e.animate.set_value(1), _engine.FadeIn(note), _engine.FadeIn(derived), lag=0.35), seconds=6.0)

    def _derived_marks(self):
        """⤴️ Dashed bracket along the plan, section and elevation slots marking them as derived from the Body."""
        top, bottom = SLOTS["plan"][1] + SLOT_H / 2 + 0.12, SLOTS["elevation"][1] - SLOT_H / 2 - 0.05
        x = 1.9
        bracket = _drafting.segments(_drafting.dashes([P((x, top)), P((x, bottom))], 0.12, 0.08), GOOD, W_EDGE)
        text = self.label("derived", "abgeleitet", (x - 0.1, top - 0.1, 0), size=S_SMALL, color=GOOD, anchor="r")
        return _engine.VGroup(bracket, text)

    def _area_hatch(self, elev, tracker, mode: str, tone, net=None):
        """▨ A hatch over the wall band (gross), the opening or the net wall that wipes in from the west with the tracker."""
        wall, op = self.m.wall, self.m.op
        length = wall["length"]
        rect = lambda x0, x1, z0, z1: [elev.at(x0, z0), elev.at(x1, z0), elev.at(x1, z1), elev.at(x0, z1)]

        def build():
            f = tracker.get_value()
            if f < 0.01:
                return _engine.VGroup()
            xe = length * f
            if mode == "opening":
                if xe <= op["start"]:
                    return _engine.VGroup()
                half, mid = 0.6 / elev.view.scale, length / 2
                spans = [(op["start"], min(xe, mid - half)), (mid + half, min(xe, op["end"]))]
                return _engine.VGroup(*[self.hatch([rect(a, b, op["sill_z"], op["head_z"])], tone, 0.07, angle) for a, b in spans if b > a for angle in (45, -45)])
            if mode == "gross":
                return _engine.VGroup(self.hatch([rect(0, xe, wall["base"], wall["top"])], tone, 0.07, 45)) if net.get_value() < 0.01 else _engine.VGroup()
            rings = [rect(0, xe, wall["base"], wall["top"])]
            if xe > op["start"]:
                rings.append(rect(op["start"], min(xe, op["end"]), op["sill_z"], op["head_z"]))
            return _engine.VGroup(self.hatch(rings, tone, 0.07, -45))

        return self.live(build, key=lambda: (round(tracker.get_value(), 3), round(net.get_value(), 3) if net is not None else 0))

    def _beat_derived(self):
        """7️⃣ Areas from the same geometry, hatched on the elevation while their numbers count up, with their formulas."""
        m = self.m
        leave = self._leave()
        elev = self._fit("elevation", (-2.55, 1.55, 0), 7.4, 3.1, m.base)
        wall, op, q = m.wall, m.op, m.q0
        tg, to, tn = (_engine.ValueTracker(0) for _ in range(3))
        outer = [elev.at(0, wall["base"]), elev.at(wall["length"], wall["base"]), elev.at(wall["length"], wall["top"]), elev.at(0, wall["top"])]
        hole = [elev.at(op["start"], op["sill_z"]), elev.at(op["end"], op["sill_z"]), elev.at(op["end"], op["head_z"]), elev.at(op["start"], op["head_z"])]
        cover = _drafting.region([outer, hole], BACKGROUND, 0.72)
        outline = _drafting.polyline(outer, INK, W_EDGE, closed=True, dash=0.1)
        hatches = (self._area_hatch(elev, tg, "gross", INK, tn), self._area_hatch(elev, to, "opening", DATA), self._area_hatch(elev, tn, "net", GOOD))
        centre = elev.at(wall["length"] / 2, (op["sill_z"] + op["head_z"]) / 2)
        pill = self.live(lambda: _engine.VGroup(_drafting.polygon([centre + (-0.5, -0.12, 0), centre + (0.5, -0.12, 0), centre + (0.5, 0.12, 0), centre + (-0.5, 0.12, 0)], BACKGROUND, 1.0, DATA, W_THIN),
                                                self.text(self._percent(q["glazing_share"], 1) + "*", centre, size=S_LABEL, color=DATA)) if to.get_value() > 0.99 else _engine.VGroup(), key=lambda: to.get_value() > 0.99)
        width, height = op["end"] - op["start"], op["head"] - op["sill"]
        n = self.number
        specs = [("gross wall area", "Bruttowandfläche", tg, q["gross_side_area"], INK, f"{n(wall['length'])} m × {n(wall['top'] - wall['base'])} m{self._basis('levels')}", "m²"),
                 ("opening area", "Öffnungsfläche", to, q["opening_area"], DATA, f"{n(width)} m × {n(height)} m{self._basis('sill')}", "m²"),
                 ("net wall area", "Nettowandfläche", tn, q["net_side_area"], GOOD, f"{n(q['gross_side_area'])} − {n(q['opening_area'])}", "m²")]
        rows, lives = _engine.VGroup(), []
        for index, (en, de, tracker, value, tone, formula, unit) in enumerate(specs):
            y = -0.45 - 0.5 * index
            ring = [P((-6.8, y - 0.12)), P((-6.4, y - 0.12)), P((-6.4, y + 0.12)), P((-6.8, y + 0.12))]
            rows.add(_drafting.polyline(ring, tone, W_EDGE, closed=True, dash=0.06) if tone == INK else self.hatch([ring], tone, 0.06, -45 if tone == GOOD else 45), _engine.Rectangle(width=0.4, height=0.24, stroke_color=tone, stroke_width=W_THIN, fill_opacity=0).move_to((-6.6, y, 0)),
                     self.label(en, de, (-6.25, y, 0), size=S_LABEL, color=INK, anchor="l", width=2.5), self.mono(formula, (-3.55, y, 0), size=S_SMALL, color=MUTED, anchor="l"))
            lives.append(self.live_label(lambda t=tracker, v=value, u=unit: self.quantity(v * t.get_value(), u, 2) + "*", (1.5, y, 0), size=S_BODY, color=DATA, anchor="r"))
        y = -0.45 - 0.5 * 3
        rows.add(self.label("glazing share", "Glasanteil", (-6.25, y, 0), size=S_LABEL, color=INK, anchor="l", width=2.5),
                 self.mono(f"{n(q['glazing_area'])} ÷ {n(q['gross_side_area'])}", (-3.55, y, 0), size=S_SMALL, color=MUTED, anchor="l"))
        lives.append(self.live_label(lambda: self._percent(q["glazing_share"] * to.get_value(), 1) + "*", (1.5, y, 0), size=S_BODY, color=DATA, anchor="r"))
        self.legend = self._legend() if self.legend is None else self.legend
        add = self._stage(_engine.VGroup(elev, cover, outline, rows), (*hatches, pill, *lives))
        self.step("derived", *self._head("derived"), seq(leave, _engine.FadeIn(elev), _engine.FadeIn(cover), _engine.FadeIn(outline), _engine.FadeIn(rows), add, tg.animate.set_value(1), to.animate.set_value(1), tn.animate.set_value(1), lag=0.5), seconds=6.0)

    def _board_elevation(self, k: int, placement):
        """🏛️ Elevation of the sheet in one model state: old sill dashed, opening hatched, sill and window height dimensions."""
        model = self._state(k)
        point, scale = placement
        drawing = self._draw("elevation", point, scale, model)
        wall, op = self._wall_of(model)
        old = self.m.op
        group = _engine.VGroup(drawing)
        ring = [drawing.at(op["start"], op["sill_z"]), drawing.at(op["end"], op["sill_z"]), drawing.at(op["end"], op["head_z"]), drawing.at(op["start"], op["head_z"])]
        group.add(self.hatch([ring], DATA, 0.07, 45), self.hatch([ring], DATA, 0.07, -45),
                  _drafting.polyline([drawing.at(old["start"], old["sill_z"]), drawing.at(old["end"], old["sill_z"])], INK, W_EDGE, dash=0.1))
        group.add(self._dimension(drawing.at(0, wall["base"]), drawing.at(0, op["sill_z"]), self.quantity(op["sill"], "m", 2) + self._basis("sill"), offset=0.28),
                  self._dimension(drawing.at(0, op["sill_z"]), drawing.at(0, op["head_z"]), self.quantity(op["head"] - op["sill"], "m", 2) + self._basis("head"), offset=0.28))
        return group

    def _board_section(self, k: int):
        """✂️ Section of the sheet in one model state, cropped to the south part like the section beat."""
        model = self._state(k)
        scale, hc, zc, crop = 0.25, 4.0, 4.7, 8.5
        section = self._draw("section", P((-1.45 + scale * hc, -1.2)), scale, model, labels=False, levels=False, centre=(hc, zc))
        section.set_z_index(-2)
        x_c = section.at(crop, 0)[0]
        self._prune(section, x_c)
        mask = _engine.Rectangle(width=7.5 - x_c, height=2.9, stroke_width=0, fill_color=BACKGROUND, fill_opacity=1).move_to(((x_c + 7.5) / 2, -1.2, 0)).set_z_index(-1)
        geo = self._geo(model)
        marks = _engine.VGroup(*[self.level_mark(P((x_c + 0.05, section.at(0, value)[1])), self._star(self._sign(value), basis), size=S_SMALL, length=0.1) for value, basis in ((geo["storeys"]["ground"], self.m.basis["levels"]), (geo["storeys"]["first"], self.m.basis["levels"]), (geo["storeys"]["roof"], self.m.basis["levels"]), (geo["box"]["height"], self.m.basis["levels"]))])
        return _engine.VGroup(section, mask, self._break((x_c, section.at(0, -0.5)[1]), (x_c, section.at(0, 9.6)[1]), 10, 0.04), marks)

    def _panel(self, t, key):
        """📟 Readout panel of the sheet: GlobalId, event, revision and the live values."""
        m = self.m
        event = m.event
        mono = lambda text, y, color=INK: self.mono(text, (2.1, y, 0), size=S_SMALL, color=color, anchor="l")
        card = _engine.VGroup(mono(self.tr("OpeningResized", "Öffnung geändert"), 2.1, DATA), mono(f"{self.tr('object', 'Objekt')}  {m.window}", 1.82), mono(f"{self.tr('sill', 'Brüstung')}  {self.number(m.op['sill'], 2)} → {self.number(event['sill'], 2)} m{self._basis('sill')}", 1.54), mono(f"{self.tr('head', 'Sturz')}  {self.number(event['head'], 2)} m{self._basis('head')}", 1.26))
        chip = self.guid_chip(TAG, (RIGHT, 2.72, 0))
        revision = self.live_label(lambda: self.tr("revision ", "Revision ") + str(m.base["revision"] + (1 if t.get_value() > 0.99 else 0)), (6.8, 1.55, 0), size=S_LABEL, color=GOOD, anchor="r")
        quantity = lambda name: (lambda: _model.quantities(TAG, self._state(key()))[name])
        rows = [("opening area", "Öffnungsfläche", quantity("opening_area"), "m²"), ("net wall area", "Nettowandfläche", quantity("net_side_area"), "m²"), ("glazing share", "Glasanteil", quantity("glazing_share"), "%")]
        static, lives = _engine.VGroup(card, chip), [revision]
        for index, (en, de, fn, unit) in enumerate(rows):
            y = 0.8 - 0.37 * index
            static.add(self.label(en, de, (2.1, y, 0), size=S_SMALL, color=MUTED, anchor="l", width=2.6))
            lives.append(self.live_label((lambda f=fn: self._percent(f(), 1) + "*") if unit == "%" else (lambda f=fn, u=unit: self.quantity(f(), u, 2) + "*"), (6.8, y, 0), size=S_BODY, color=DATA, anchor="r"))
        return static, lives

    def _beat_update(self):
        """8️⃣ The sheet: elevation, readouts, plan, section and 3D regenerate from one tracker."""
        m = self.m
        t = _engine.ValueTracker(0)
        key = lambda: round(t.get_value() * STEPS)
        placement = self._place("elevation", (-2.45, 1.55, 0), 6.7, 3.0, m.base)
        elev = self.live(lambda: self._board_elevation(key(), placement), key=key)
        section = self.live(lambda: self._board_section(key()), key=key)
        a_point, a_scale = self._place("3d", (4.45, -1.2, 0), 4.7, 2.05, m.base)
        axon = self.live(lambda: self._draw("3d", a_point, a_scale, self._state(key())), key=key)
        plan = self._fit("plan", (-5.75, -1.15, 0), 2.2, 2.0, m.base, thumb=True)
        plan_note = self.label("regenerated · same drawing", "neu erzeugt · identisch", (-5.6, -2.38, 0), size=S_SMALL, color=GOOD, width=2.6)
        cut_key = self.live(lambda: self._cut_key(self._state(key()), origin=(-4.5, -2.2), k=0.45, length=2.0), key=key)
        panel, panel_lives = self._panel(t, key)
        sweep = _engine.Rectangle(width=0.06, height=5.6, stroke_width=0, fill_color=DATA, fill_opacity=0.35).move_to((-6.9, 0.275, 0))
        before = _engine.VGroup(self.hero, self._column(), *self.lives, *([self.legend] if self.legend is not None else []))
        leaving = _engine.AnimationGroup(_engine.FadeOut(before), run_time=1.0)
        legend = self._legend(right=True)
        board = _engine.VGroup(elev, section, axon, plan, plan_note, cut_key, legend, panel, *panel_lives)
        arriving = seq(*[_engine.FadeIn(part) for part in (elev, section, axon, plan, plan_note, cut_key, legend, panel, *panel_lives)], lag=0.12)
        change = _engine.Succession(_engine.Add(sweep), _engine.AnimationGroup(t.animate(run_time=3.2, rate_func=_engine.smooth).set_value(1), sweep.animate(run_time=3.2, rate_func=_engine.linear).move_to((6.9, 0.275, 0))), _engine.FadeOut(sweep))
        self.hero, self.lives, self.legend, self.slots, self.thumbs, self.chip, self.card = board, [], None, {}, {}, None, None
        self.step("update", *self._head("update"), _engine.Succession(leaving, arriving, change), seconds=7.4)

    def _per_metre(self, denominator: int) -> float:
        """📏 Screen units per metre of a drawing at paper scale 1:denominator; one millimetre of paper is ``PAPER`` units."""
        return PAPER * 1000 / denominator

    def _lod_badges(self, point, used: str | None = None, vertical: bool = False):
        """🏷️ Axis, FootPrint and Body as badges; the representation the drawing needs is filled."""
        group, x = _engine.VGroup(), point[0]
        for kind in ("axis", "footprint", "body"):
            name = _model.represent(TAG, kind, self.m.base)["identifier"]
            badge = self.badge(name, (x, point[1], 0), color=GOOD if name == used else MUTED, filled=name == used, anchor="l")
            group.add(badge)
            x += 0 if vertical else badge.width + 0.2
            point = (point[0], point[1] - (0.4 if vertical else 0))
        return group

    def _jamb(self, kind: str, ox: float, oy: float, k: float, x_from: float):
        """🔩 Compare native axis, occupied footprint and physical window-height material in one crop."""
        m=self.m
        wall,op=m.wall,m.op
        t,x_to=m.q0["thickness"],wall["length"]
        sx,sy=(lambda x:ox+k*(x-x_from)),(lambda y:oy+k*y)
        group=_engine.VGroup()
        if kind=="axis":
            group.add(_drafting.polyline([P((sx(x_from),sy(t/2))),P((sx(x_to),sy(t/2)))],ARCH,W_CUT))
        elif kind=="footprint":
            for primitive in _model.represent(TAG,"footprint",m.base)["primitives"]:
                if primitive["kind"]=="polygon":
                    local=self._clip_station(self._wall_local_ring(primitive["points"]),x_from)
                    if local:
                        group.add(_drafting.polyline([P((sx(x),sy(y))) for x,y in local],ARCH,W_CUT,closed=True))
        else:
            group.add(self._cut_plan((op["sill"]+op["head"])/2,ox,oy,k,x_from))
        group.add(self._break((sx(x_from),sy(-.1)),(sx(x_from),sy(t+.1)),3,.05))
        return group

    def _clip_station(self, ring, minimum):
        """✂️ Clip a local occupied polygon to the displayed station without collapsed edge polygons."""
        result=[]
        for a,b in zip(ring,ring[1:]+ring[:1]):
            inside_a,inside_b=a[0]>=minimum,b[0]>=minimum
            if inside_a:
                result.append(list(a))
            if inside_a!=inside_b:
                f=(minimum-a[0])/(b[0]-a[0])
                result.append([minimum,a[1]+f*(b[1]-a[1])])
        area=abs(sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(result,result[1:]+result[:1])))/2
        return result if len(result)>=3 and area>1e-12 else []

    def _layer_rows(self, x: float, y: float, den: int, unit_mm: bool = True):
        """🎨 One legend row per build-up layer with its thickness and the millimetres it takes on paper at 1:``den``."""
        group = _engine.VGroup()
        for index, layer in enumerate(self.m.layers):
            tone = LAYER_TONES.get(layer["id"], MUTED)
            t = layer["thickness"]["value"]
            text = f"{self._layer_name(layer)}  {self.quantity(t, 'm', 3)}{self._star('', layer['thickness']['basis'])} → {self.number(t * 1000 / den, 2 if den > 100 else 1)} mm"
            group.add(_engine.Rectangle(width=0.2, height=0.14, stroke_color=tone, stroke_width=W_THIN, fill_color=tone, fill_opacity=0.0 if layer.get("is_void", False) else 0.8).move_to((x + 0.1, y - 0.3 * index, 0)),
                      self.text(text, (x + 0.3, y - 0.3 * index, 0), size=S_SMALL, color=tone, anchor="l", width=4.4))
        return group

    def _beat_lod(self):
        """🔬 One wall at three levels of detail in the same plan window: the Axis line, the FootPrint outline and the Body layers."""
        m = self.m
        leave = self._leave()
        rep = {kind: _model.represent(TAG, kind, m.base) for kind in ("axis", "footprint", "body")}
        k, x_from, ox = 1.8, m.wall["length"] - 2.1, -4.1
        t = m.q0["thickness"]
        ys = {"axis": 2.15, "footprint": 0.85, "body": -0.45}
        sub = {"axis": ("line", "Linie"), "footprint": ("outline", "Umriss"), "body": ("solid with layers", "Volumen mit Schichten")}
        chips = {kind: self.class_box(rep[kind]["identifier"], [(rep[kind]["type"], DATA), (self.tr(*sub[kind]), MUTED)], (-5.6, y, 0), color=ARCH, width=2.3) for kind, y in ys.items()}
        strips = {kind: self._jamb(kind, ox, y - (t * k / 2), k, x_from) for kind, y in ys.items()}
        notes = {"axis": (("length", "Länge"), self.quantity(m.q0["length"], "m")), "footprint": (("maximum envelope", "maximale Hülle"), self.quantity(t, "m") + self._basis("thickness")),
                 "body": (("build-up", "Aufbau"), f"{len(m.layers)} " + self.tr("layers", "Schichten"))}
        texts = _engine.VGroup(*[_engine.VGroup(self.label(*notes[kind][0], (-0.1, y + 0.17, 0), size=S_SMALL, color=MUTED, anchor="l"), self.text(notes[kind][1], (-0.1, y - 0.15, 0), size=S_LABEL, color=DATA, anchor="l", width=1.7)) for kind, y in ys.items()])
        title = self.label("same wall · same plan window · three levels of detail", "dieselbe Wand · derselbe Planausschnitt · drei Detailstufen", (-6.8, 3.0, 0), size=S_LABEL, color=MUTED, anchor="l", width=8.4)
        legend = _engine.VGroup(self.label("upper screen layers on paper at 1:20", "obere Schirmschichten auf Papier im Maßstab 1:20", (-4.1, -1.1, 0), size=S_SMALL, color=MUTED, anchor="l"), self._layer_rows(-4.1, -1.45, 20))
        pieces = (title, *[x for kind in ys for x in (chips[kind], strips[kind])], texts, legend)
        self._stage(_engine.VGroup(*pieces))
        self.step("lod", *self._head("lod"), seq(leave, *[_engine.FadeIn(x) for x in pieces], lag=0.3), seconds=5.4)

    def _beat_scale200(self):
        """📐 The villa plan at 1:200 with the wall as footprint outline only: thickness, length and layers converted to millimetres of paper, with a drawn scale bar."""
        m = self.m
        leave = self._leave()
        k = self._per_metre(200)
        wall, t = m.wall, m.q0["thickness"]
        plan = self._draw("plan", P((-3.9, 0.4)), k, m.base, grid=False, labels=False)
        x = -0.8
        mm = lambda value, digits=1: self.number(value * 1000 / 200, digits)
        layer = m.layers[0]["thickness"]["value"]
        side = _engine.VGroup(
            self._lod_badges((x, 2.85), "FootPrint", vertical=True),
            self.text("1:200", (x, 1.45, 0), size=S_BODY, color=DATA, anchor="l"), self.scale_bar(P((x, 0.95)), k, 5, steps=1),
            self.label("wall thickness", "Wandstärke", (x, 0.3, 0), size=S_SMALL, color=MUTED, anchor="l"), self.text(f"{self.quantity(t, 'm')}{self._basis('thickness')} → {mm(t)} mm", (x, 0.02, 0), size=S_LABEL, color=DATA, anchor="l", width=2.6),
            self.label("wall length", "Wandlänge", (x, -0.55, 0), size=S_SMALL, color=MUTED, anchor="l"), self.text(f"{self.quantity(wall['length'], 'm')} → {mm(wall['length'])} mm", (x, -0.83, 0), size=S_LABEL, color=INK, anchor="l", width=2.5),
            self.label("one layer", "eine Schicht", (x, -1.4, 0), size=S_SMALL, color=MUTED, anchor="l"), self.text(f"{self.quantity(layer, 'm')}{self._star('', m.layers[0]['thickness']['basis'])} → {mm(layer, 2)} mm", (x, -1.68, 0), size=S_LABEL, color=WARN, anchor="l", width=2.5),
            self.label("too thin to draw", "zu dünn zum Zeichnen", (x, -2.0, 0), size=S_SMALL, color=WARN, anchor="l", width=2.5))
        self._stage(_engine.VGroup(plan, side))
        self.step("scale200", *self._head("scale200"), seq(leave, _engine.FadeIn(plan), _engine.FadeIn(side), lag=0.4), seconds=5.4)

    def _beat_scale20(self):
        """📐 The wall at 1:20: the Body with its layers in millimetres of paper, with a drawn scale bar."""
        m = self.m
        leave = self._leave()
        k = self._per_metre(20)
        wall, op, t = m.wall, m.op, m.q0["thickness"]
        x_from, ox, oy = wall["length"] - 2.0, -6.3, 1.0
        strip = self._jamb("body", ox, oy, k, x_from)
        end = ox + k * (wall["length"] - x_from)
        dims = _engine.VGroup(self._dimension(P((end, oy)), P((end, oy + k * t)), f"{self.quantity(t, 'm')} = {self.number(t * 1000 / 20, 0)} mm" + self._basis("thickness"), offset=-0.16, color=DATA),
                              self._dimension(P((ox + k * (op["end"] - x_from), oy)), P((end, oy)), self.quantity(wall["length"] - op["end"], "m") + self._basis("end"), offset=-0.2, color=DATA))
        glass = self.text(self.tr("glass line", "Glaslinie"), (ox + 0.1, oy + k * t + 0.25, 0), size=S_SMALL, color=GLASS, anchor="l")
        bar = _engine.VGroup(self.text("1:20", (1.4, 2.85, 0), size=S_BODY, color=DATA, anchor="r"), self.scale_bar(P((-6.3, 0.1)), k, 2, steps=2))
        badges = self._lod_badges((-6.8, 2.8), "Body")
        rows = self._layer_rows(-6.8, -0.8, 20)
        note = self.label("the layers only exist in the body", "die Schichten gibt es nur im Body", (-6.8, -1.95, 0), size=S_SMALL, color=MUTED, anchor="l")
        self._stage(_engine.VGroup(strip, dims, glass, bar, badges, rows, note))
        self.step("scale20", *self._head("scale20"), seq(leave, _engine.FadeIn(strip), _engine.FadeIn(dims), _engine.FadeIn(glass), _engine.FadeIn(bar), _engine.FadeIn(badges), _engine.FadeIn(rows), _engine.FadeIn(note), lag=0.35), seconds=5.4)

    def _wall_local_ring(self, points):
        """📐 Resolve physical world outlines into wall station and envelope depth coordinates."""
        wall=self.m.wall
        origin=np.asarray(wall["start"],dtype=float)
        axis=(np.asarray(wall["end"])-origin)/wall["length"]
        normal=np.array([-axis[1],axis[0]])
        return [[float(np.dot(np.asarray(p)-origin,axis)),float(np.dot(np.asarray(p)-origin,normal))+wall["thickness"]/2] for p in points]

    def _cut_plan(self, h: float, ox: float, oy: float, k: float, x_from: float, spacing: float = 0.07):
        """✂️ Slice occupied wall, native filling and auxiliary members at the physical cut level."""
        m=self.m
        wall=m.wall
        z=wall["base"]+h
        sx,sy=(lambda x:ox+k*(x-x_from)),(lambda y:oy+k*y)
        group=_engine.VGroup()
        filling=next(item for item in m.geo["fillings"] if item["tag"]==m.window)
        auxiliary=[item for item in m.geo["beams"] if _model.element(item["tag"],m.base).get("geometry",{}).get("section_sweep",{}).get("wall")==TAG]
        for item in [wall,filling,*auxiliary]:
            for part in item["parts"]:
                if not part["z0"]<=z<=part["z1"]:
                    continue
                for region in _model.part_regions(part,z):
                    local=self._clip_station(self._wall_local_ring(region),x_from)
                    if not local:
                        continue
                    ring=[P((sx(x),sy(y))) for x,y in local]
                    tone=GLASS if part.get("role")=="glass" else (ARCH if item is filling else DATA)
                    group.add(_drafting.polygon(ring,tone,.45 if tone==GLASS else .8,tone,W_THIN))
                    if tone!=GLASS:
                        group.add(self.hatch([ring],INK,spacing,45))
        return group

    def _beat_sweep(self):
        """🧹 The plan cut height sweeps from +0.50 m to +1.50 m above the first floor and back: solid wall, window band, solid wall; the cut is dimensioned in the section key."""
        m = self.m
        leave = self._leave()
        wall, op = m.wall, m.op
        h = _engine.ValueTracker(0.5)
        key = lambda: round(h.get_value() * 40)
        strip = self.live(lambda: self._cut_plan(h.get_value(), -6.6, 2.1, 0.3, 0.0, 0.04), key=key)
        detail = self.live(lambda: self._cut_plan(h.get_value(), -6.5, -0.6, 1.45, wall["length"] - 3.0), key=key)
        title = self.live_label(lambda: self.tr("plan cut ", "Schnittebene ") + self.quantity(h.get_value(), "m", 2) + self.tr(" above the first floor", " über dem Obergeschoss"), (-6.8, 2.95, 0), size=S_LABEL, color=DATA, anchor="l")
        status = self.live(lambda: self.label("window band · glass lines", "Fensterband · Glaslinien", (-6.8, 1.45, 0), size=S_LABEL, color=GLASS, anchor="l") if op["sill"] <= h.get_value() <= op["head"]
                           else self.label("solid wall · poché", "Vollwand · Poché", (-6.8, 1.45, 0), size=S_LABEL, color=DATA, anchor="l"), key=lambda: op["sill"] <= h.get_value() <= op["head"])
        frame = _engine.Rectangle(width=5.0, height=1.5, stroke_color=MUTED, stroke_width=W_THIN, fill_opacity=0).move_to((-4.35, -0.5, 0))
        frame.set_stroke(opacity=0.6)
        loupe = self._loupe(P((-6.6 + 0.3 * (wall["length"] - 1.5), 2.1 + 0.3 * m.q0["thickness"] / 2)), 0.2, (-3.9, 0.25))
        window = self.text(f"{self.tr('window', 'Fenster')}  {self.tr('sill', 'Brüstung')} {self.quantity(op['sill'], 'm')}{self._basis('sill')} · {self.tr('head', 'Sturz')} {self.quantity(op['head'], 'm')}{self._basis('head')}", (-6.8, -1.55, 0), size=S_SMALL, color=MUTED, anchor="l", width=5.0)
        cut_key = self.live(lambda: self._cut_key(m.base, origin=(-1.35, -2.25), k=0.55, length=2.4, cut=wall["base"] + h.get_value()), key=key)
        add = self._stage(_engine.VGroup(frame, loupe, window), (strip, detail, title, status, cut_key))
        target = m.cut - wall["base"]
        sweep = lambda value, run: h.animate(run_time=run, rate_func=_engine.smooth).set_value(value)
        self.step("sweep", *self._head("sweep"), _engine.Succession(seq(leave, _engine.FadeIn(frame), _engine.FadeIn(loupe), _engine.FadeIn(window), add, lag=0.3), sweep(1.5, 2.2), sweep(0.5, 2.2), sweep(target, 1.2)), seconds=8.0)

    def _q(self, k: int):
        """🧮 Quantities of the wall in one model state, memoized per state."""
        cache = self.m.cache
        if ("q", k) not in cache:
            cache[("q", k)] = _model.quantities(TAG, self._state(k))
        return cache[("q", k)]

    def _takeoff_elevation(self, k: int, placement):
        """🏛️ Elevation of one state with the net wall hatched green and the actual glass hatched yellow, plus the live sill and window dimensions."""
        group = self._board_elevation(k, placement)
        drawing = group[0]
        wall, op = self._wall_of(self._state(k))
        outer = [drawing.at(0, wall["base"]), drawing.at(wall["length"], wall["base"]), drawing.at(wall["length"], wall["top"]), drawing.at(0, wall["top"])]
        hole = [drawing.at(op["start"], op["sill_z"]), drawing.at(op["end"], op["sill_z"]), drawing.at(op["end"], op["head_z"]), drawing.at(op["start"], op["head_z"])]
        group.add(_drafting.region([outer, hole], BACKGROUND, 0.72), self.hatch([outer, hole], GOOD, 0.07, 45))
        state = self._state(k)
        direction = (np.asarray(wall["end"])-np.asarray(wall["start"]))/wall["length"]
        for filling in self._geo(state)["fillings"]:
            if filling["host"] != wall["tag"]:
                continue
            for part in filling["parts"]:
                if part["role"] != "glass":
                    continue
                stations = [float(np.dot(np.asarray(p)-wall["start"],direction)) for p in part["outline"]]
                lo,hi = min(stations),max(stations)
                ring = [drawing.at(s,z) for s,z in ((lo,part["z0"]),(hi,part["z0"]),(hi,part["z1"]),(lo,part["z1"]))]
                group.add(self.hatch([ring],DATA,0.07,-45))
        return group

    def _glass_formula(self, k: int) -> str:
        """💎 Express the actual clear glass dimensions from the selected shared filling."""
        state = self._state(k)
        wall,opening = self._wall_of(state)
        q = _model.quantities(opening["fill_tag"],state)
        return f"{self.number(q['clear_width'])} × {self.number(q['clear_height'])} = {self.number(self._q(k)['glazing_area'])} m²*"

    def _beat_takeoff(self):
        """🧾 The same sill event as live quantity take-off: net wall area, actual glass area and glazing share from the Body, hatched on the elevation."""
        m = self.m
        leave = self._leave()
        t = _engine.ValueTracker(0)
        key = lambda: round(t.get_value() * STEPS)
        placement = self._place("elevation", (-2.6, 1.5, 0), 6.4, 3.0, m.base)
        elev = self.live(lambda: self._takeoff_elevation(key(), placement), key=key)
        q0, n = m.q0, self.number
        specs = [("net wall area", "Nettowandfläche", "net_side_area", "m²", GOOD), ("glass area", "Glasfläche", "glazing_area", "m²", DATA), ("glazing share", "Glasanteil", "glazing_share", "%", GLASS)]
        static, lives = _engine.VGroup(), []
        for index, (en, de, name, unit, tone) in enumerate(specs):
            y = 2.45 - 1.2 * index
            before = (self._percent(q0[name], 1) if unit == "%" else self.quantity(q0[name], unit)) + "*"
            static.add(_engine.Rectangle(width=0.14, height=0.5, stroke_width=0, fill_color=tone, fill_opacity=0.9).move_to((1.6, y - 0.05, 0)),
                       self.label(en, de, (1.85, y + 0.17, 0), size=S_SMALL, color=MUTED, anchor="l"), self.text(f"{self.tr('rev ', 'Rev. ')}{m.base['revision']}  {before}", (1.85, y - 0.2, 0), size=S_SMALL, color=MUTED, anchor="l"))
            lives.append(self.live_label((lambda nm=name, u=unit: (self._percent(self._q(key())[nm], 1) if u == "%" else self.quantity(self._q(key())[nm], u)) + "*"), (6.8, y - 0.05, 0), size=S_BODY, color=DATA, anchor="r"))
        formulas = [lambda: f"{n(self._q(key())['gross_side_area'])} − {n(self._q(key())['opening_area'])} = {n(self._q(key())['net_side_area'])} m²*",
                    lambda: self._glass_formula(key()),
                    lambda: f"{n(self._q(key())['glazing_area'])} ÷ {n(self._q(key())['gross_side_area'])} = {self._percent(self._q(key())['glazing_share'], 1)}*"]
        for index, fn in enumerate(formulas):
            lives.append(self.live_label(fn, (-6.8, -0.75 - 0.45 * index, 0), size=S_LABEL, color=INK, anchor="l"))
        event = _engine.VGroup(self.mono(f"{self.tr('OpeningResized', 'Öffnung geändert')}  {self.tr('sill', 'Brüstung')} {n(m.op['sill'])} → {n(m.event['sill'])} m{self._basis('sill')}", (1.6, -0.75, 0), size=S_SMALL, color=DATA, anchor="l"),
                               self.badge(self.tr("one model · one Body", "ein Modell · ein Body"), (1.6, -1.25, 0), color=GOOD, anchor="l"))
        legend = self._legend()
        self.legend = legend
        add = self._stage(_engine.VGroup(static, event, legend), (elev, *lives))
        self.step("takeoff", *self._head("takeoff"), _engine.Succession(seq(leave, _engine.FadeIn(static), _engine.FadeIn(event), _engine.FadeIn(legend), add, lag=0.3), t.animate(run_time=3.6, rate_func=_engine.smooth).set_value(1)), seconds=7.0)

    def _volume_scene(self, k: int, placement):
        """🧊 The wall solid of one state with its void lifted out, dimensioned by length and height, with leaders to the void and the wall."""
        point, scale = placement
        model = self._state(k)
        wall, _ = self._wall_of(model)
        drawing = self.element_axon(TAG, point, scale, model=model, extrude=1.0, opening=0.35, window=1.0, highlight=(TAG,), colors="material")
        q = self._q(k)
        n = self.number
        void = q["gross_volume"] - q["volume"]
        group = _engine.VGroup(drawing)
        group.add(self._dimension(drawing.at(0, 0, wall["base"]), drawing.at(wall["length"], 0, wall["base"]), self.quantity(wall["length"], "m"), offset=-0.3, color=DATA),
                  self.leader(P((-0.1, 2.2)), drawing.parts["void"].get_center(), None, color=WARN),
                  self.text(f"{self.tr('void', 'Aussparung')}  {self.quantity(void, 'm³', 3)}*", (0.0, 2.35, 0), size=S_LABEL, color=WARN, anchor="l"),
                  self.leader(P((-6.3, 2.2)), drawing.at(2.0, 0.0, wall["base"] + 0.4), None, color=GOOD),
                  self.text(f"{self.tr('wall', 'Wand')}  {self.quantity(q['volume'], 'm³', 3)}*", (-6.2, 2.35, 0), size=S_LABEL, color=GOOD, anchor="l"))
        return group

    def _beat_volume(self):
        """🧊 The wall volume as the Body minus its void, live with the sill event."""
        m = self.m
        leave = self._leave()
        t = _engine.ValueTracker(0)
        key = lambda: round(t.get_value() * STEPS)
        first = self.element_axon(TAG, (0, 0, 0), 0.2, model=m.base, extrude=1.0, opening=0.35, window=1.0, highlight=(TAG,), colors="material")
        c = np.asarray(P((-2.7, 0.0)), float)
        scale = 0.2 * min(6.8 / first.width, 3.7 / first.height)
        placement = (c - (np.array(first.get_center()) * scale / 0.2), scale)
        solid = self.live(lambda: self._volume_scene(key(), placement), key=key)
        n, q0 = self.number, m.q0
        void = lambda k: self._q(k)["gross_volume"] - self._q(k)["volume"]
        specs = [("gross solid material", "Brutto-Feststoff", lambda: self.quantity(self._q(0)["gross_volume"], "m³", 3) + "*", INK, None), ("openings / embedded joints", "Öffnungen / eingebettete Anschlüsse", lambda: self.quantity(void(key()), "m³", 3) + "*", WARN, void),
                 ("owned wall material", "Wandeigenes Material", lambda: self.quantity(self._q(key())["volume"], "m³", 3) + "*", GOOD, lambda k: self._q(k)["volume"])]
        static, lives = _engine.VGroup(), []
        for index, (en, de, fn, tone, before) in enumerate(specs):
            y = 1.9 - 0.8 * index
            static.add(_engine.Rectangle(width=0.14, height=0.45, stroke_width=0, fill_color=tone, fill_opacity=0.9).move_to((2.15, y - 0.05, 0)), self.label(en, de, (2.4, y + 0.1, 0), size=S_SMALL, color=MUTED, anchor="l"))
            if before is not None:
                static.add(self.text(f"{self.tr('rev ', 'Rev. ')}{m.base['revision']}  {self.quantity(before(0), 'm³', 3)}*", (2.4, y - 0.17, 0), size=S_SMALL, color=MUTED, anchor="l"))
            lives.append(self.live_label(fn, (6.8, y - 0.05, 0), size=S_BODY, color=tone, anchor="r"))
        lives.append(self.live_label(lambda: f"Σ Vᵣ = {n(self._q(key())['volume'], 3)} m³*", (2.15, -0.7, 0), size=S_SMALL, color=INK, anchor="l"))
        lives.append(self.live_label(lambda: f"{n(self._q(0)['gross_volume'], 3)} − {n(void(key()), 3)} = {n(self._q(key())['volume'], 3)} m³*", (2.15, -1.1, 0), size=S_SMALL, color=INK, anchor="l"))
        note = self.label("sum actual solid parts · cavity and embedded joints excluded", "Summe fester Teile · Hohlraum und eingebettete Anschlüsse abgezogen", (2.15, -1.6, 0), size=S_SMALL, color=MUTED, anchor="l", width=4.6)
        legend = self._legend()
        self.legend = legend
        add = self._stage(_engine.VGroup(static, note, legend), (solid, *lives))
        self.step("volume", *self._head("volume"), _engine.Succession(seq(leave, _engine.FadeIn(static), _engine.FadeIn(note), _engine.FadeIn(legend), add, lag=0.3), t.animate(run_time=3.6, rate_func=_engine.smooth).set_value(1)), seconds=7.0)

    def _end_table(self, plan_value: float, tone):
        """📋 Where the window ends in the plan drawing, in the model and in the elevation."""
        m = self.m
        rows = [(self.tr("plan drawing", "Grundrisszeichnung"), plan_value, "m"), (self.tr("model", "Modell"), m.op["end"], "m"), (self.tr("elevation", "Ansicht"), m.op["end"], "m")]
        return self.property_table(rows, (-3.6, -1.1, 0), width=6.4, title=self.tr("where the window ends", "wo das Fenster endet"), basis=False, color=tone)

    def _corner_context(self, ox: float, oy: float, k: float, x_from: float, y_to: float = 1.0):
        """🧱 The south-east corner of the first-floor plan around the wall end: the east wall outline, the pilotis below, a break line."""
        m = self.m
        sx, sy = (lambda x: ox + k * (x - x_from)), (lambda y: oy + k * y)
        group = _engine.VGroup()
        east = _model.represent("W-E", "footprint", m.base)["primitives"]
        for primitive in east:
            clipped = [(max(x, x_from), min(y, y_to)) for x, y in primitive["points"]]
            if max(p[0] for p in clipped) - min(p[0] for p in clipped) > 1e-9 and max(p[1] for p in clipped) - min(p[1] for p in clipped) > 1e-9:
                group.add(_drafting.polygon([P((sx(x), sy(y))) for x, y in clipped], INK, 0.25, INK, W_THIN))
        group.add(self._break((sx(m.wall["length"]) - .2, sy(y_to)), (sx(m.wall["length"]) + .05, sy(y_to)), 4, .04, MUTED))
        for pil in m.geo["pilotis"]:
            if x_from <= pil["x"] <= m.wall["length"] and pil["y"] <= y_to:
                for part in pil["parts"]:
                    if part["z0"] < m.geo["storeys"]["first"] and part["z1"] > m.geo["storeys"]["ground"]:
                        group.add(_drafting.polyline([P((sx(x), sy(y))) for x, y in part["outline"]], MUTED, W_THIN, closed=True, dash=.06))
        return group

    def _beat_drafted(self):
        """✏️ A jamb line drafted only in the plan at 19.20 m turns orange: the model, the elevation and the other views still end the window at 20.00 m."""
        m = self.m
        leave = self._leave()
        wall, op, t = m.wall, m.op, m.q0["thickness"]
        k, x_from, ox, oy = 1.6, wall["length"] - 4.0, -6.5, 0.9
        sx = lambda x: ox + k * (x - x_from)
        x_d = op["end"] - 0.8
        plan = _engine.VGroup(self._corner_context(ox, oy, k, x_from), self._cut_plan(m.cut - wall["base"], ox, oy, k, x_from))
        top = oy + k * t
        pencil = _drafting.polyline([P((sx(x_d), oy - 0.45)), P((sx(x_d), top + 0.45))], INK, W_CUT)
        label = lambda tone: self.text(f"{self.tr('drafted in the plan', 'im Grundriss gezeichnet')} · {self.quantity(x_d, 'm')}", (sx(x_d), oy - 0.7, 0), size=S_SMALL, color=tone)
        model_mark = _engine.VGroup(_drafting.polyline([P((sx(op["end"]), oy - 0.45)), P((sx(op["end"]), top + 0.45))], GLASS, W_EDGE, dash=0.1),
                                    self.text(f"{self.tr('model', 'Modell')} · {self.quantity(op['end'], 'm')}", (sx(op["end"]) - 0.55, top + 0.7, 0), size=S_SMALL, color=GLASS))
        title = self.label("plan drawing · south-east corner", "Grundrisszeichnung · Südostecke", (-6.8, 3.0, 0), size=S_LABEL, color=MUTED, anchor="l")
        elev = self._fit("elevation", (3.7, 1.7, 0), 6.0, 2.6, m.new, True)
        wall1, op1 = self._wall_of(m.new)
        a, b = elev.at(op1["end"], wall1["base"] - 0.4), elev.at(op1["end"], wall1["top"] + 0.4)
        elev_mark = _engine.VGroup(_drafting.polyline([a, b], GLASS, W_EDGE, dash=0.1), self.text(f"{self.tr('elevation', 'Ansicht')} · {self.quantity(op1['end'], 'm')}", b + np.array([-0.7, 0.25, 0.0]), size=S_SMALL, color=GLASS))
        others = _engine.VGroup(*[x for kind, x0 in (("section", 2.5), ("3d", 5.3)) for x in (self._fit(kind, (x0, -1.2, 0), 2.6, 1.5, m.new, True), self.label(*SLOT_NAMES[kind], (x0, -0.28, 0), size=S_SMALL, color=MUTED))])
        table = self._end_table(x_d, WARN)
        badge = self.badge(self.tr("only in this drawing · not in the model", "nur in dieser Zeichnung · nicht im Modell"), (-6.8, -2.2, 0), color=WARN, anchor="l")
        ink, orange = label(INK), label(WARN)
        self.draft = SimpleNamespace(static=_engine.VGroup(title, plan, model_mark, elev, elev_mark, others), edits=_engine.VGroup(pencil, orange, badge, table), plan=plan)
        self._stage(_engine.VGroup(self.draft.static, self.draft.edits))
        self.step("drafted", *self._head("drafted"), _engine.Succession(seq(leave, *[_engine.FadeIn(x) for x in (title, plan, model_mark, elev, elev_mark, others)], _engine.FadeIn(pencil), _engine.FadeIn(ink), lag=0.3),
                                                _engine.AnimationGroup(pencil.animate.set_color(WARN), _engine.FadeOut(ink), _engine.FadeIn(orange), _engine.FadeIn(badge), _engine.FadeIn(table), lag_ratio=0.3)), seconds=6.0)

    def _beat_regenerate(self):
        """♻️ Regeneration: a sweep redraws the plan from the model, the drafted line disappears and the three window ends agree again."""
        m = self.m
        draft = self.draft
        table = self._end_table(m.op["end"], GOOD)
        badge = self.badge(self.tr("regenerated from the model · drafted line lost", "aus dem Modell neu erzeugt · gezeichnete Linie verloren"), (-6.8, -2.2, 0), color=GOOD, anchor="l")
        sweep = _engine.Rectangle(width=0.06, height=3.2, stroke_width=0, fill_color=DATA, fill_opacity=0.35).move_to((-6.9, 1.5, 0))
        self.hero = _engine.VGroup(draft.static, table, badge)
        move = _engine.Succession(_engine.Add(sweep), sweep.animate(run_time=2.0, rate_func=_engine.linear).move_to((0.1, 1.5, 0)), _engine.FadeOut(sweep))
        self.step("regenerate", *self._head("regenerate"), _engine.Succession(move, _engine.AnimationGroup(_engine.FadeOut(draft.edits), _engine.FadeIn(table), _engine.FadeIn(badge), _engine.Indicate(draft.plan, color=GOOD, scale_factor=1.02), lag_ratio=0.2)), seconds=5.4)

    def _beat_copies(self):
        """9️⃣ A static copy of the old elevation next to the regenerated one, with stale and regenerated mini views below."""
        m = self.m
        op, op1 = m.op, m.op1
        live = self._fit("elevation", (-4.0, 1.25, 0), 5.5, 2.45, m.new)
        copy = self._fit("elevation", (3.2, 1.25, 0), 5.5, 2.45, m.base)
        lack = [copy.at(op1["start"], op1["sill_z"]), copy.at(op1["end"], op1["sill_z"]), copy.at(op1["end"], op["sill_z"]), copy.at(op1["start"], op["sill_z"])]
        should = _engine.VGroup(_drafting.polygon(lack, WARN, 0.95, WARN, W_EDGE), self.hatch([lack], INK, 0.05, 45),
                                self.leader(P((4.9, 2.05)), copy.at(18.5, op1["sill_z"]), None, color=WARN),
                                self.text(self.tr("missing: ", "fehlt: ") + self.quantity(op["sill"] - op1["sill"], "m", 2), (4.95, 2.05, 0), size=S_SMALL, color=WARN, anchor="l"))
        east = m.wall["length"]
        dim_live = self._dimension(live.at(east, m.wall["base"]), live.at(east, op1["sill_z"]), self.quantity(op1["sill"], "m", 2) + self._basis("sill"), offset=-0.12, color=GOOD)
        dim_copy = self._dimension(copy.at(east, m.wall["base"]), copy.at(east, op["sill_z"]), self.quantity(op["sill"], "m", 2) + self._basis("sill"), offset=-0.12, color=WARN)
        frame_live = _engine.SurroundingRectangle(_engine.VGroup(live, dim_live), color=GOOD, buff=0.1, stroke_width=W_THIN)
        frame_copy = _engine.SurroundingRectangle(_engine.VGroup(copy, dim_copy), color=WARN, buff=0.1, stroke_width=W_THIN)
        titles = _engine.VGroup(self.label("regenerated from the model", "aus dem Modell neu erzeugt", (-3.3, 2.85, 0), size=S_LABEL, color=GOOD, anchor="c"),
                                self.label("copy of the drawing", "Kopie der Zeichnung", (3.5, 2.85, 0), size=S_LABEL, color=WARN))
        revs = _engine.VGroup(self.badge(self.tr("rev ", "Rev. ") + str(m.new["revision"]), (-6.8, 2.85, 0), color=GOOD, anchor="l"), self.badge(self.tr("rev ", "Rev. ") + str(m.base["revision"]), (0.4, 2.85, 0), color=WARN, anchor="l"))
        unequal = self.text("≠", (-0.25, 1.25, 0), size=S_BODY * 1.6, color=WARN)
        minis = _engine.VGroup()
        for model, tone, x0 in ((m.new, GOOD, -6.0), (m.base, WARN, 0.9)):
            for index, kind in enumerate(("plan", "section", "3d")):
                centre = (x0 + 2.05 * index, -1.2, 0)
                thumb = self._fit(kind, centre, 1.75, 1.05, model, True)
                frame = _engine.Rectangle(width=1.9, height=1.2, stroke_color=tone, stroke_width=W_THIN, fill_opacity=0).move_to(centre)
                minis.add(thumb, frame, self.label(*SLOT_NAMES[kind], (centre[0], centre[1] + 0.77, 0), size=S_SMALL, color=tone))
        notes = _engine.VGroup(self.badge(self.tr("regenerated · no action", "neu erzeugt · ohne Eingriff"), (-3.9, -2.12, 0), color=GOOD),
                               self.label("each copy by hand", "jede Kopie von Hand", (3.9, -2.12, 0), size=S_SMALL, color=MUTED))
        self.legend = self._legend()
        leave = self._leave()
        self.pair = _engine.VGroup(live, copy, frame_live, frame_copy, should, dim_live, dim_copy, titles, revs, unequal, minis, notes, self.legend)
        self._stage(self.pair)
        self.step("copies", *self._head("copies"), seq(leave, _engine.FadeIn(live), _engine.FadeIn(frame_live), _engine.FadeIn(titles), _engine.FadeIn(revs), _engine.FadeIn(dim_live), _engine.FadeIn(copy), _engine.FadeIn(frame_copy), _engine.FadeIn(dim_copy),
                                _engine.FadeIn(should), _engine.FadeIn(unequal), _engine.FadeIn(minis), _engine.FadeIn(notes), _engine.FadeIn(self.legend), lag=0.25), seconds=5.6)

    def _beat_identity(self):
        """🔟 Back to the object in its villa: new revision, same GlobalId, every view from one model."""
        m = self.m
        axon = self._fit("3d", (-2.9, 1.15, 0), 7.4, 3.3, m.new)
        slots = [self._slot(kind) for kind in SLOTS]
        thumbs = {kind: self._thumb(kind, m.new) for kind in SLOTS}
        chip = self._chip(model=m.new)
        card = self._card()
        old_id, new_id = _model.element(TAG, m.base)["guid"], _model.element(TAG, m.new)["guid"]
        rows = _engine.VGroup(self.mono(f"{self.tr('rev ', 'Rev. ')}{m.base['revision']}  {old_id}", (-6.4, -1.0, 0), size=S_LABEL, color=MUTED, anchor="l"),
                              self.mono(f"{self.tr('rev ', 'Rev. ')}{m.new['revision']}  {new_id}", (-6.4, -1.5, 0), size=S_LABEL, color=GOOD, anchor="l"))
        tick = self.text("=" if old_id == new_id else "≠", (-1.6, -1.25, 0), size=S_BODY * 1.5, color=GOOD if old_id == new_id else WARN)
        caption = self.label("same GlobalId · every view from one model", "dieselbe GlobalId · jede Ansicht aus einem Modell", (-6.4, -2.0, 0), size=S_LABEL, color=INK, anchor="l", width=7.0)
        pulses = _engine.AnimationGroup(*[_engine.Indicate(thumbs[k], color=GOOD, scale_factor=1.06) for k in SLOTS], _engine.Indicate(chip, color=GOOD), lag_ratio=0.25)
        self.step("identity", *self._head("identity"), _engine.Succession(seq(self._leave(), _engine.FadeIn(axon), *[_engine.FadeIn(g) for g in slots], *[_engine.FadeIn(x) for x in thumbs.values()], _engine.FadeIn(chip), _engine.FadeIn(card),
                                                       _engine.FadeIn(rows), _engine.FadeIn(tick), _engine.FadeIn(caption), lag=0.2), pulses), seconds=5.6)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       