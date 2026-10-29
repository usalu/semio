"""🎬 Models Share a Schema: three discipline models of the Villa Savoye on one IFC schema, federated, checked and translated into a structural analysis model."""

import copy
import importlib
import re
import sys
from collections import Counter
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_panels = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧾️panels.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_sheet = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV = _visuals.INK, _visuals.MUTED, _visuals.ARCH, _visuals.STRUCT, _visuals.SERV
DATA, GOOD, WARN, CONCRETE = _visuals.DATA, _visuals.GOOD, _visuals.WARN, _visuals.CONCRETE
P, polygon, region, segments, polyline, circle_points, mix, hatch = (_drafting.P, _drafting.polygon, _drafting.region, _drafting.segments, _drafting.polyline,
                                                                     _drafting.circle_points, _drafting.mix, _drafting.hatch)
BACKGROUND = _drafting.BACKGROUND
frame_box = _panels.frame_box
(VGroup, FadeIn, FadeOut, Indicate, LaggedStart, AnimationGroup, Succession, ValueTracker, Dot, Circle, Rectangle, Flash, Line, Arrow, ReplacementTransform) = (
    getattr(_engine, name) for name in ("VGroup", "FadeIn", "FadeOut", "Indicate", "LaggedStart", "AnimationGroup", "Succession", "ValueTracker",
                                         "Dot", "Circle", "Rectangle", "Flash", "Line", "Arrow", "ReplacementTransform"))
LINEAR, SMOOTH = _engine.linear, _engine.smooth
DISCIPLINES = ("architecture", "structure", "services")
COLORS = dict(zip(DISCIPLINES, (ARCH, STRUCT, SERV)))
AT, SCALE = np.array([-3.45, 0.3, 0.0]), 0.165
RIGHT = 3.55
LEFT_PLAN = np.array([-3.3, 0.0, 0.0])
SPACE_ORDER = ("ground", "first", "roof")
HEADINGS = {
    "whole": ("Villa Savoye · one building", "Villa Savoye · ein Gebäude"), "architecture": ("Architecture model · {architecture} objects", "Architekturmodell · {architecture} Objekte"),
    "rooms": ("Rooms · IfcSpace per storey", "Räume · IfcSpace je Geschoss"), "buildup": ("Wall build-up · plan cut", "Wandaufbau · Grundrissschnitt"),
    "structure": ("Structure model · {structure} objects", "Tragwerksmodell · {structure} Objekte"), "services": ("Services model · {services} objects", "Haustechnikmodell · {services} Objekte"),
    "schema": ("One schema · {classes} classes", "Ein Schema · {classes} Klassen"), "federation": ("Federation · one origin, one GlobalId", "Koordination · ein Ursprung, eine GlobalId"),
    "slab": ("One slab · one GlobalId", "Eine Decke · eine GlobalId"), "owners": ("Ground floor · who draws what", "Erdgeschoss · wer zeichnet was"),
    "check": ("Clash check · clearance", "Kollisionsprüfung · Abstand"), "translation": ("Translation · structure to analysis", "Übersetzung · Tragwerk zur Analyse"),
    "members": ("Analysis mesh on the slab", "Analysenetz auf der Decke"), "midsurface": ("Mid-surface · section at 19", "Mittelfläche · Schnitt bei 19"),
    "supports": ("Supports · selected births", "Lagerung · gewählte Stützenanfänge"), "loads": ("Loads · q builds up", "Lasten · q baut sich auf"),
    "loss": ("Losses · what is not kept", "Verluste · was nicht erhalten bleibt"), "properties": ("Piloti 19 · kept and dropped", "Piloti 19 · erhalten und verworfen"),
    "trace": ("Trace · source GlobalId", "Rückverfolgung · Quell-GlobalId")}
KIND_NAMES = {
    "IfcWindow:element": ("Window", "Fenster"), "IfcOpeningElement:element": ("Opening", "Öffnung"), "IfcWall.STANDARD:element": ("Wall", "Wand"),
    "IfcWall.STANDARD:material.layer.render": ("Render", "Putz"), "IfcCurtainWall:element": ("Glass", "Glas"), "IfcWall.SOLIDWALL:element": ("Screen", "Schirm"),
    "IfcDoor:element": ("Door", "Tür"), "IfcStair:element": ("Stair", "Treppe"), "IfcRailing:element": ("Guardrail", "Geländer"),
    "IfcFurnishingElement:element": ("Furniture", "Möbel"), "IfcBuildingElementProxy:element": ("Shelter", "Überdachung"),
    "IfcRamp:element": ("Ramp", "Rampe"), "IfcRampFlight:element": ("Flight", "Lauf"), "IfcSpace:element": ("Space", "Raum"), "IfcSlab.ROOF:element": ("Roof", "Dach"),
    "IfcSlab.FLOOR:voids": ("Void", "Loch"), "IfcColumn:Pset_ColumnCommon.FireRating": ("Fire", "Feuer"), "IfcColumn:Pset_ColumnCommon.IsExternal": ("External", "Außen")}
STAR = "*"
KIND_LABELS = {"SOLIDWALL": ("screen", "Schirm"), "ROOF": ("roof", "Dach"), "voids": ("void", "Öffnung"), "render": ("render", "Putz")}


class Beat4_PartialModels(_visuals.ClipScene):
    """📚 Animate three discipline models, their federation, a clash check and the translation to a structural analysis model with English and German narration."""

    TITLE = {"en": "Models Share a Schema", "de": "Modelle teilen ein Schema"}
    NARRATION = [
        ["whole",
         "Different disciplines organize the information they need about the reconstructed original house of 1930–1931.",
         "Verschiedene Fachrichtungen ordnen die benötigte Information zum rekonstruierten ursprünglichen Haus von 1930–1931."],
        ["architecture",
         "The architecture model retains rooms, entrance construction, the roof stair enclosure, its canopy and the open metal ramp guard.",
         "Das Architekturmodell enthält Räume, Eingangskonstruktion, Dachtreppenhaus, dessen Vordach und das offene Metallgeländer an der Rampe."],
        ["rooms",
         "Rooms are objects too: the modelled spaces, each with its area, belong to their storey.",
         "Auch Räume sind Objekte: die modellierten Räume mit ihrer Fläche gehören zu ihrem Geschoss."],
        ["buildup",
         "The facade build-up distinguishes render, two brick leaves, an air cavity and plaster.",
         "Der Fassadenaufbau unterscheidet Außenputz, zwei Ziegelschalen, einen Lufthohlraum und Innenputz."],
        ["structure",
         "The structure model holds pilotis, slabs, ramp, stairs and auxiliary lintels, hollow U supports and shelves; several members are shared with architecture.",
         "Das Tragwerksmodell enthält Pilotis, Decken, Rampe, Treppen sowie Zusatzstürze, hohle U-Träger und Konsolen; mehrere Glieder teilt es mit der Architektur."],
        ["services",
         "Original services include drains inside supports 2 and 4, heating and water risers, soil stack and flue. Detailed envelopes and incomplete routes retain their uncertainties.",
         "Ursprüngliche Haustechnik umfasst Fallrohre in Stützen 2 und 4, Heizungs- und Wasserstränge, Abwasserstrang und Rauchzug. Detailhüllen und unvollständige Leitungswege behalten ihre Unsicherheiten."],
        ["schema",
         "Three models share one schema. Counts refer to IFC products; a stair assembly can contain many physical parts.",
         "Drei Modelle teilen ein Schema. Die Zahlen zählen IFC-Objekte; eine Treppenbaugruppe kann viele physische Teile enthalten."],
        ["federation",
         "Coordination puts the models on one origin. In this teaching model one GlobalId marks the shared slab.",
         "Koordination legt Modelle auf einen Ursprung. Im Lehrmodell markiert eine GlobalId die gemeinsame Decke."],
        ["slab",
         "The shared slab keeps one GlobalId: each model reads from it the properties it needs.",
         "Die gemeinsame Decke behält eine GlobalId: Jedes Modell liest daraus, was es braucht."],
        ["owners",
         "On the ground-floor plan discipline memberships may overlap: the mechanical hoist jib belongs to architecture, structure and services.",
         "Im Grundriss können Fachzuordnungen überlappen: Der Hebeausleger gehört zu Architektur, Tragwerk und Haustechnik."],
        ["check",
         "In this teaching scenario, a clash check finds what no single model shows: a downpipe too close to a piloti.",
         "Kollisionsprüfung im Lehrbeispiel: ein Fallrohr liegt zu nah an einer Pilotistütze, was kein Einzelmodell zeigt."],
        ["translation",
         "For one use case, the structure model is translated into a structural analysis model: IFC's analysis domain.",
         "Für einen Anwendungsfall wird das Tragwerksmodell in ein Tragwerksanalysemodell übersetzt: die IFC-Analysedomäne."],
        ["members",
         "The first-floor slab becomes clipped surface members; selected slab supports become line members.",
         "Die Obergeschossdecke wird zu zugeschnittenen Flächenelementen; gewählte Deckenstützen werden zu Stabelementen."],
        ["midsurface",
         "The slab is cut at its mid-surface: the piloti line reaches it, the thickness becomes an attribute.",
         "Die Decke wird an der Mittelfläche geschnitten: Der Stab reicht dorthin, die Dicke wird zum Attribut."],
        ["supports",
         "Each selected analytical birth receives an assumed fixed restraint, including transfer-born members.",
         "Jeder gewählte analytische Stützenanfang erhält eine angenommene Einspannung, einschließlich abgefangener Stützen."],
        ["loads",
         "The load is assumed too: self-weight, finishes and imposed load add up to the area load.",
         "Auch die Last ist angenommen: Eigengewicht, Ausbau und Nutzlast ergeben die Flächenlast."],
        ["loss",
         "Ribbon windows, infill walls, render, the curved glass wall and more are not in the analysis.",
         "Fensterbänder, Ausfachungen, Putz, die gebogene Glaswand und mehr fehlen in der Analyse."],
        ["properties",
         "Even kept pilotis lose properties: fire rating and external flag have no place in the analysis.",
         "Selbst erhaltene Pilotis verlieren Eigenschaften: Feuerwiderstand und Außenbauteil-Kennzeichen entfallen."],
        ["trace",
         "Each member keeps its source GlobalId, so results trace back; the translation is not reversible.",
         "Jedes Element behält seine Quell-GlobalId, so finden Ergebnisse zurück; umkehrbar ist die Übersetzung nicht."]]
    BEATS = tuple(row[0] for row in NARRATION)

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("models")

    def present_models(self):
        """🎞️ Run the nineteen beats: three models, schema, federation, one shared slab, plan, check, translation, mesh, assumptions, losses, trace."""
        self._prepare()
        for key in self.BEATS:
            getattr(self, "_" + key)()

    def _prepare(self):
        """🧰 Read every number the beats show from the model once."""
        self.stage = VGroup()
        self.sub = None
        self.ghost = None
        self.info = {name: _model.partial_model(name) for name in DISCIPLINES}
        self.model = _model.load_model()
        self.geometry = _model.building_geometry()
        self.analysis = _model.translate_structure()
        self.scenario = copy.deepcopy(self.model)
        target = next(p for p in self.geometry["pilotis"] if p["tag"] == _model.role("column_example"))
        pipe = next(e for e in self.scenario["elements"] if e["tag"] == "DP-1")
        required = self.scenario["rules"]["clearance"][0]["required"]["value"]
        offset = target["radius"] + pipe["attributes"]["diameter"]["value"] / 2 + required / 2
        delta=np.array([target["x"]+offset,target["y"]])-np.array(pipe["geometry"]["points"][0][:2])
        affected={pipe["tag"],pipe["tag"]+"-OUTLET"}
        for service in self.scenario["elements"]:
            if service["tag"] not in affected:
                continue
            geometry=service["geometry"]
            geometry["points"]=[[p[0]+delta[0],p[1]+delta[1],p[2]] for p in geometry["points"]]
            for name,plane in geometry.get("tube",{}).items():
                if name.startswith("clip_"):
                    p=plane["point"]
                    plane["point"]=[p[0]+delta[0],p[1]+delta[1],p[2]]
            geometry.pop("penetrations",None)
        self.scenario["relations"]=[relation for relation in self.scenario["relations"] if not (relation.get("connection",{}).get("kind")=="hosted_conduit" and (relation["relating"] in affected or set(relation["related"])&affected))]
        self.scenario_geometry = _model.building_geometry(self.scenario)
        self.clash = next(clash for clash in _model.clash_check(self.scenario) if clash["a_tag"]==pipe["tag"] and clash["b_tag"]==target["tag"])
        self.trib = _model.tributary()

    def text(self, string, point=(0, 0, 0), **kwargs):
        """🔤 Text without bracket glyphs: a bracketed remark becomes a comma clause."""
        return super().text(re.sub(r"\s*\(([^)]*)\)", r", \1", string), point, **kwargs)

    def _villa_analysis(self, point, scale, *, tags=(), nodes=False, highlight=()):
        """🏙️ The analysis model drawn on the villa: structure axon with grid bubbles and slab outline underneath, members and shells on top."""
        base = self.villa_axon(point, scale, disciplines=("structure",), grid=True, shell=0.25, ground=False)
        view = self.analysis_view(self.analysis, point, scale, projection="axon", highlight=highlight)
        if not nodes:
            view.remove(view.nodes, view.supports)
        base.grid.remove(base.grid.submobjects[-1])
        view.submobjects.insert(0, base)
        view.add(*[self.leader((view.anchors[tag][0], view.get_top()[1] + 0.3, 0), view.anchors[tag], tag, color=DATA, size=16.2, anchor="c") for tag in tags])
        view.base = base
        return view

    def _roof_note(self, point):
        """⬚ Legend for the dashed outline on first-floor plans: the opening of the roof slab above the hanging garden."""
        x, y, _ = np.asarray(P(point), float)
        return VGroup(polyline([(x, y - 0.07), (x + 0.3, y - 0.07), (x + 0.3, y + 0.07), (x, y + 0.07)], MUTED, 1.2, closed=True, dash=0.06),
                      self.label("dashed: roof slab opening above the garden", "gestrichelt: Öffnung der Dachdecke über dem Garten", (x + 0.42, y, 0), size=15.5, color=MUTED, anchor="l", width=5.0))

    def _legend(self, point, *, assumed=False, piloti=False, width=5.8):
        """⭐ Star legend lines: assumed teaching-model values and the disputed piloti count."""
        x, y, _ = np.asarray(P(point), float)
        lines = ([self.tr("* assumed in the teaching model", "* im Lehrmodell angenommen")] if assumed else []) + ([self.tr("* source-informed teaching reconstruction", "* quellenbezogene Lehrrekonstruktion")] if piloti else [])
        return self._lines(lines, (x, y, 0), size=15.5, color=MUTED, pitch=0.24, width=width)

    def _piloti_label(self):
        """🏛️ The piloti count wording: the teaching-model number with its star."""
        return (f"registered column objects{STAR}", f"registrierte Stützenobjekte{STAR}")

    def _chains(self, plan):
        """📏 Grid dimension chains on top and right of a plan; the short façade bays carry their number above the extension lines."""
        box, grid = self.geometry["box"], self.geometry["grid"]
        xs, ys, group = [0.0, *grid["x"], box["length"]], list(grid["y"]), VGroup()
        for first, second in zip(xs, xs[1:]):
            a, b = plan.to_screen(first, box["width"]), plan.to_screen(second, box["width"])
            group.add(self.dimension(a, b, None, offset=0.45, color=MUTED), self.text(self.number(second - first, 2), (a + b) / 2 + np.array([0.0, 0.67, 0.0]), size=15.5, color=MUTED))
        for first, second in zip(ys, ys[1:]):
            a, b = plan.to_screen(box["length"], first), plan.to_screen(box["length"], second)
            group.add(self.dimension(a, b, self.number(second - first, 2), offset=-0.45, color=MUTED, size=15.5))
        return group

    def _ground_plan(self, scale=0.22, model=None):
        """🗺️ The toolkit ground-floor plan in model colours with heavy poché on the cut walls, the double-line glass wall and filled pilotis."""
        owner = {tag: COLORS[name] for name in DISCIPLINES for tag in self.info[name]["element_tags"]}
        plan = self.villa_plan("ground", (-3.3, 0.1, 0), scale, layers=DISCIPLINES, dims=(), north=True, halo=0.0, highlight=list(owner), highlight_colors=owner, model=model)
        plan.add(self._chains(plan), self.scale_bar((-0.95, -2.3, 0), scale, 5.0, steps=2))
        return plan

    def _timeline(self, *stages):
        """⏳ Stages of ``(seconds, [animations])`` as one animation and its total length."""
        return Succession(*[AnimationGroup(*animations, run_time=seconds) for seconds, animations in stages]), sum(seconds for seconds, _ in stages)

    def _show(self, key, *stages):
        """🎬 Play the stages with the clause ``key``."""
        counts = {name: self.number(len(self.info[name]["element_guids"]), 0) for name in DISCIPLINES}
        counts["classes"] = self.number(len(self.model["classes"]), 0)
        heading = _fonts.section_heading(self.tr(*HEADINGS[key]).format(**counts))
        old, self.sub = self.sub, heading
        first = stages[0]
        stages = ((first[0], [*first[1], FadeIn(heading), *([FadeOut(old)] if old is not None else [])]), *stages[1:])
        animation, total = self._timeline(*stages)
        self.step(key, animation, seconds=total)

    def _leave(self, *keep):
        """🚪 Fade out everything of the previous beat except ``keep``."""
        return FadeOut(VGroup(*[m for m in self.stage.submobjects if m not in keep]))

    def _revision(self, number) -> str:
        """🔖 Revision label in the clip language."""
        return self.tr(f"rev {number}", f"Rev. {number}")

    def _level_text(self, value: float) -> str:
        """🔻 Elevation text in language punctuation: ±0.00, +3.50."""
        return _sheet.level_text(self, value)

    def _chip(self, string, point, color=INK, size=17.0, anchor="c"):
        """🔲 A short framed text."""
        label = self.text(string, (0, 0, 0), size=size, color=color)
        frame = frame_box(label.width + 0.3, 0.38, color)
        return _drafting.place(VGroup(frame, label.move_to(frame)), point, anchor)

    def _lines(self, strings, point, *, size=16.2, color=MUTED, pitch=0.25, width=None, anchor="l"):
        """📝 Several short lines below each other starting at ``point``."""
        x, y, _ = np.asarray(P(point), float)
        return VGroup(*[self.text(string, (x, y - index * pitch, 0), size=size, color=color, width=width, anchor=anchor) for index, string in enumerate(strings)])

    def _wrap(self, text, limit=58) -> list:
        """↩️ Break a long sentence into two balanced lines at a space."""
        if len(text) <= limit:
            return [text]
        words = text.split(" ")
        best = min(range(1, len(words)), key=lambda index: abs(len(" ".join(words[:index])) - len(" ".join(words[index:]))))
        return [" ".join(words[:best]), " ".join(words[best:])]

    def _counter(self, tracker, point, *, size=21.0, color=DATA, anchor="r", digits=0, unit=""):
        """🔢 A number that follows a tracker."""
        return self.live_label(lambda: self.number(tracker.get_value(), digits) + unit, point, size=size, color=color, anchor=anchor)

    def _card(self, name, y):
        """🗂️ Model card: coloured name, revision, teaching-role author and schema chip of one discipline model."""
        info, tone, language = self.info[name], COLORS[name], self.language()
        left, right = RIGHT - 2.95, RIGHT + 2.95
        return VGroup(frame_box(5.9, 0.95, tone).move_to((RIGHT, y, 0)),
                      self.text(info["name"][language], (left + 0.15, y + 0.2, 0), size=21.0, color=tone, anchor="l", width=3.6),
                      self.badge(self._revision(info["revision"]), (right - 0.14, y + 0.2, 0), color=tone, anchor="r"),
                      self.text(info["author_label"][language], (left + 0.15, y - 0.18, 0), size=16.2, color=INK, anchor="l", width=3.7),
                      self.badge(info["schema_id"], (right - 0.14, y - 0.18, 0), color=MUTED, filled=False, anchor="r"))

    def _arch(self, point, scale, **kwargs):
        """🏢 The architecture layer in material colours with a thin discipline outline."""
        drawing = self.villa_axon(point, scale, disciplines=("architecture",), colors="material", ground=False, **kwargs)
        drawing.set_stroke(color=ARCH, width=1.0)
        return drawing

    def _tags(self, name, ifc_class):
        """🏷️ Tags of one class inside a discipline model."""
        return [tag for tag in self.info[name]["element_tags"] if _model.element(tag)["class"] == ifc_class]

    def _keys(self, drawing, tag):
        """🔑 Drawn parts that stand for a model tag; a window is drawn through its opening."""
        if tag in drawing.parts:
            return [tag]
        return [other["tag"] for relation in _model.relations_of(tag) for other in relation["others"]
                if other["class"] == "IfcOpeningElement" and other["tag"] in drawing.parts]

    def _lit(self, drawing, tags, color=DATA):
        """💡 One light-up per drawn part of the tags, one after another."""
        keys = [key for tag in tags for key in self._keys(drawing, tag)]
        return LaggedStart(*[Indicate(drawing.parts[key], color=color, scale_factor=1.0) for key in keys], lag_ratio=min(0.6, 3.0 / max(len(keys), 1)))

    def _flash(self, group, color):
        """🔦 Show a ghosted group briefly in a bright colour."""
        copy = group.copy().set_color(color).set_stroke(opacity=1.0).set_fill(opacity=0.5)
        return Succession(FadeIn(copy, run_time=0.5), FadeOut(copy, run_time=0.5))

    def _class_row(self, tracker, ifc_class, text, y, color, x=0.95):
        """📊 Count, IFC class and meaning of one kind of object."""
        return VGroup(self._counter(tracker, (x, y, 0), size=19.0),
                      self.text(ifc_class, (x + 0.15, y, 0), size=17.0, color=color, anchor="l"),
                      self.label(text[0], text[1], (x + 2.32, y, 0), size=16.2, color=INK, width=3.4, anchor="l"))

    def _discipline_rows(self, name):
        """🗂️ List every current native discipline class with explicit bilingual teaching meaning."""
        labels={
            "IfcColumn":self._piloti_label(),"IfcBeam":("primary and auxiliary members","Haupt- und Zusatzträger"),
            "IfcFooting":("pads and cellar foundations","Fundamente und Kellergründung"),
            "IfcWall":("occupied walls and solarium screen","feste Wände und Solariumswand"),
            "IfcSlab":("slabs and hatch covers","Platten und Lukendeckel"),
            "IfcRamp":("ramps between storeys","Rampen zwischen Geschossen"),"IfcRampFlight":("their straight flights","ihre geraden Läufe"),
            "IfcOpeningElement":("native host openings","native Wirtsöffnungen"),
            "IfcWindow":("frames and actual glass","Rahmen und tatsächliches Glas"),"IfcDoor":("door assemblies","Türbaugruppen"),
            "IfcCurtainWall":("entrance surround","Eingangsumfassung"),"IfcStair":("stair assemblies","Treppenbaugruppen"),
            "IfcBuildingElementProxy":("native assemblies and roof details","Baugruppen und Dachdetails"),
            "IfcFurnishingElement":("fitted/movable furnishings","feste/bewegliche Ausstattung"),"IfcRailing":("guardrail assemblies","Geländerbaugruppen"),
            "IfcSpace":("rooms and outdoor regions","Räume und Außenbereiche"),
            "IfcBoiler":("inferred cellar boiler envelope","abgeleitete Kellerkesselhülle"),"IfcSpaceHeater":("radiator under the salon window","Heizkörper unter dem Salonfenster"),
            "IfcWasteTerminal":("roof rainwater receivers","Regenwasserabläufe am Dach"),"IfcPipeSegment":("rainwater/heating/water/soil","Regen-, Heiz-, Wasser-/Abwasser"),
            "IfcCovering":("separate casing and insulation","getrennte Ummantelung und Dämmung"),"IfcChimney":("chimney assemblies","Schornsteinbaugruppen")}
        counts=self.info[name]["counts_by_class"]
        unknown=set(counts)-set(labels)
        if unknown:
            raise ValueError(f"Missing bilingual class meanings: {sorted(unknown)}")
        return [(kind,text) for kind,text in labels.items() if kind in counts]

    def _rows(self, name, rows, y0, drawing, pitch=0.31, seconds=0.55):
        """🧮 Class rows of one model with their counting animation; every class of the model is listed so the rows add up to its total."""
        counts = self.info[name]["counts_by_class"]
        if set(counts) != {ifc_class for ifc_class, _ in rows}:
            raise ValueError(f"Rows must list every class of the {name} model")
        group, steps = VGroup(), []
        for index, (ifc_class, text) in enumerate(rows):
            tracker = ValueTracker(0)
            row = self._class_row(tracker, ifc_class, text, y0 - index * pitch, COLORS[name])
            group.add(row)
            steps.append((seconds, [FadeIn(row, run_time=0.15), tracker.animate(rate_func=LINEAR).set_value(counts[ifc_class]), self._lit(drawing, self._tags(name, ifc_class))]))
        return group, steps

    def _sum_line(self, name, classes, y, x=0.95):
        """➕ The class counts added up to the object total of the model."""
        counts = self.info[name]["counts_by_class"]
        total = len(self.info[name]["element_guids"])
        if sum(counts[c] for c in classes) != total:
            raise ValueError(f"Class counts of the {name} model do not add up")
        terms = self.tr("class counts", "Klassenzahlen")
        return self.text(f"Σ {self.number(len(classes),0)} {terms} = {self.number(total,0)} " + self.tr("objects", "Objekte"), (x,y,0), size=17.5,color=DATA,anchor="l",width=5.7)

    def _equation(self, items, point, *, size=19.0, gap=0.13):
        """🧮 A formula line centred on ``point``; each item is ``(text or function, colour, widest text)`` and functions stay live."""
        widths = [self.text(final, (0, 0, 0), size=size).width for _, _, final in items]
        x, group = point[0] - (sum(widths) + gap * (len(items) - 1)) / 2, VGroup()
        for (content, color, _), width in zip(items, widths):
            if callable(content):
                group.add(self.live_label(content, (x + width, point[1], 0), size=size, color=color, anchor="r"))
            else:
                group.add(self.text(content, (x + width / 2, point[1], 0), size=size, color=color))
            x += width + gap
        return group

    def _whole(self):
        """🏛️ Beat 1: the villa as one building; three disciplines ask three questions of it and the entries add up to the unique objects."""
        whole = self.villa_axon(AT, SCALE, colors="material", ground=False)
        questions = {"architecture": ("Where are rooms, walls and windows?", "Wo sind Räume, Wände und Fenster?"),
                     "structure": ("What carries the loads to the ground?", "Was trägt die Lasten in den Boden?"),
                     "services": ("Where do heat and rain water go?", "Wohin gehen Wärme und Regenwasser?")}
        names = {"architecture": ("Architecture", "Architektur"), "structure": ("Structure", "Tragwerk"), "services": ("Services", "Haustechnik")}
        years, building = self.model["building"]["years"], self.model["building"]["name"][self.language()]
        heading = VGroup(self.text(f"{building} · Poissy · {years['design_start']}–{years['completed']}", (RIGHT, 2.72, 0), size=18.0, color=INK),
                         self.label("Teaching model: simplified, rounded values", "Lehrmodell: vereinfachte, gerundete Werte", (RIGHT, 2.45, 0), size=16.2, color=MUTED))
        rows, trackers, steps = VGroup(), {}, []
        for index, name in enumerate(DISCIPLINES):
            y = 1.55 - index * 1.1
            trackers[name] = ValueTracker(0)
            row = VGroup(frame_box(5.8, 0.95, COLORS[name]).move_to((RIGHT, y, 0)),
                         self.label(*names[name], (RIGHT - 2.75, y + 0.2, 0), size=22.0, color=COLORS[name], anchor="l"),
                         self.label(*questions[name], (RIGHT - 2.75, y - 0.18, 0), size=16.2, color=INK, width=3.6, anchor="l"),
                         self._counter(trackers[name], (RIGHT + 2.75, y + 0.17, 0), size=24.0),
                         self.label("objects", "Objekte", (RIGHT + 2.75, y - 0.2, 0), size=16.2, color=MUTED, anchor="r"))
            rows.add(row)
            steps.append(AnimationGroup(FadeIn(row, shift=_engine.LEFT * 0.2, run_time=0.4), trackers[name].animate(rate_func=LINEAR).set_value(len(self.info[name]["element_guids"])),
                                        Indicate(whole.layers[name], color=DATA, scale_factor=1.0), lag_ratio=0.0))
        sizes = [len(self.info[name]["element_guids"]) for name in DISCIPLINES]
        unique = len(set().union(*[set(info["element_guids"]) for info in self.info.values()]))
        entries, total = self.number(sum(sizes), 0), ValueTracker(0)
        summary = VGroup(self.text(" + ".join(self.number(size, 0) for size in sizes) + f" = {entries} " + self.tr("entries", "Einträge"), (RIGHT - 2.9, -1.55, 0), size=18.0, color=INK, anchor="l"),
                         self._counter(total, (RIGHT - 2.9 + 0.5, -1.97, 0), size=24.0),
                         self.label(f"unique objects · {self.number(sum(sizes) - unique, 0)} are in two models", f"verschiedene Objekte · {self.number(sum(sizes) - unique, 0)} stehen in zwei Modellen",
                                    (RIGHT - 2.9 + 0.65, -1.97, 0), size=17.0, color=INK, anchor="l", width=5.0))
        self._show("whole",
                   (1.0, [LaggedStart(*[FadeIn(whole.layers[name], shift=_engine.UP * 0.1) for name in ("structure", "architecture", "services")], lag_ratio=0.3), FadeIn(heading)]),
                   (2.2, [LaggedStart(*steps, lag_ratio=0.7)]),
                   (0.9, [FadeIn(summary, run_time=0.2), total.animate(rate_func=LINEAR).set_value(unique), Indicate(whole, color=DATA, scale_factor=1.0)]))
        self.stage = VGroup(whole, heading, rows, summary)

    def _architecture(self):
        """🏢 Beat 2: the architecture model with every class and count, each class lit on the villa."""
        arch = self._arch(AT, SCALE)
        card = self._card("architecture", 2.1)
        spec = self._discipline_rows("architecture")
        rows, steps = self._rows("architecture", spec, 1.55, arch, pitch=0.235, seconds=0.4)
        total = self._sum_line("architecture", [name for name, _ in spec], -2.25)
        choice = self.label("Open hanging garden; salon and kiosk glazing remain distinct", "Offener hängender Garten; Salon- und Kioskverglasung bleiben getrennt", (0.95, -2.65, 0), size=16.2, color=MUTED, anchor="l", width=5.7)
        flights = [flight["tag"] for flight in self.geometry["ramp"]["flights"]]
        ramp = VGroup(self.villa_section((-5.75, 2.1, 0), 0.085, cut="ramp", layers=("structure", "architecture"), highlight=flights, thumbnail=True),
                      self.label("ramp section", "Rampenschnitt", (-5.75, 2.78, 0), size=15.5, color=MUTED, width=1.6))
        self._show("architecture", (0.8, [self._leave(), FadeIn(arch), FadeIn(card)]), *steps, (0.7, [FadeIn(total), FadeIn(choice), FadeIn(ramp)]))
        self.stage = VGroup(arch, card, rows, total, choice, ramp)

    def _rooms(self):
        """🚪 Draw all authored spaces on their three storeys with derived counts and areas."""
        plans, summaries = VGroup(), VGroup()
        for index, level in enumerate(SPACE_ORDER):
            x = (-4.4, 0.0, 4.4)[index]
            spaces = [space for space in self.geometry["spaces"] if space["storey"] == level]
            tags = [space["tag"] for space in spaces]
            plan = self.villa_plan(level, (x, 0.4, 0), 0.155, layers=("architecture",), grid=False, highlight=tags, highlight_colors={tag: ARCH for tag in tags})
            storey = next(item for item in self.model["storeys"] if item["id"] == level)
            title = self.text(storey["name"][self.language()], (x, 2.65, 0), size=18.0, color=INK)
            count = self.text(f"{len(spaces)} IfcSpace", (x, -1.85, 0), size=18.0, color=ARCH)
            area = self.text(self.quantity(sum(space["area"] for space in spaces), "m²", 1) + STAR, (x, -2.25, 0), size=17.0, color=MUTED)
            plans.add(plan, title)
            summaries.add(count, area)
        note = self.label("* Areas derived from the teaching reconstruction", "* Flächen aus der Lehrrekonstruktion abgeleitet", (0, -2.8, 0), size=15.5, color=MUTED)
        self._show("rooms", (0.8, [self._leave(), FadeIn(plans)]), (0.9, [FadeIn(summaries), FadeIn(note)]))
        self.stage = VGroup(plans, summaries, note)

    def _buildup(self):
        """🧱 Beat 4: a facade cut with solid leaves and an empty cavity growing to the envelope thickness."""
        wall = _model.element("W-S")["material"]
        layers, total = wall["layers"], wall["total_thickness"]
        plan = self.villa_plan("first", (-3.4, 0.2, 0), 0.2, layers=("architecture",), grid=False, highlight=("W-S",), highlight_colors={"W-S": DATA})
        length = self.geometry["walls"][0]["length"]
        cut = plan.to_screen(length * 0.62, 0.15)
        mark = VGroup(segments([(cut + (0, -0.32, 0), cut + (0, 0.42, 0))], DATA, 2.6), self.text("A", cut + (0, 0.62, 0), size=17.0, color=DATA))
        zoom, x0, y0, height, grow = 8.0, 1.2, 1.25, 1.5, ValueTracker(0)
        tones = [(mix(BACKGROUND, INK, 0.6), INK, 45, 0.07), (mix(BACKGROUND, WARN, 0.5), WARN, 45, 0.12), (mix(BACKGROUND, INK, 0.3), INK, 135, 0.07)]

        def strip():
            group, left = VGroup(), x0
            for index, layer in enumerate(layers):
                fill, line, angle, spacing = tones[index % len(tones)]
                span = max(layer["thickness"]["value"] * zoom * grow.get_value(), 1e-3)
                ring = [(left, y0 - height / 2), (left + span, y0 - height / 2), (left + span, y0 + height / 2), (left, y0 + height / 2)]
                empty = layer.get("is_void", False)
                group.add(polygon(ring, BACKGROUND if empty else fill, 0.0 if empty else 1.0, MUTED if empty else INK, 1.0))
                if not empty:
                    group.add(hatch(ring, line, spacing, angle, 0.8, 0.8))
                left += span
            group.add(self.dimension((x0, y0 - height / 2, 0), (left, y0 - height / 2, 0), self.quantity(total * grow.get_value(), "m", 3), offset=-0.22, color=DATA, size=17.0))
            return group

        live = self.live(strip, key=lambda: round(grow.get_value(), 3))
        names, language = VGroup(), self.language()
        for index, layer in enumerate(layers):
            fill, line, _, _ = tones[index % len(tones)]
            y = 2.0 - index * 0.3
            names.add(Rectangle(width=0.2, height=0.2, stroke_color=line, stroke_width=1.2, fill_color=fill, fill_opacity=0.0 if layer.get("is_void", False) else 1.0).move_to((x0 + total * zoom + 0.35, y, 0)),
                      self.text(f"{layer['name'][language]} · {self.quantity(layer['thickness']['value'], 'm', 3)}{STAR if layer['thickness']['basis'] != 'sourced' else ''}", (x0 + total * zoom + 0.55, y, 0), size=16.2, color=INK, anchor="l", width=2.55))
        names.add(self.label("* cavity inferred; envelope measured", "* Hohlraum abgeleitet; Hülle belegt", (x0 + total * zoom + 0.35, .3, 0), size=15.0, color=MUTED, anchor="l", width=2.9))
        sides = VGroup(self.label("outside", "außen", (x0, y0 + height / 2 + 0.2, 0), size=16.2, color=MUTED, anchor="l"),
                       self.label("inside", "innen", (x0 + total * zoom, y0 + height / 2 + 0.2, 0), size=16.2, color=MUTED, anchor="r"))
        pset = next(name for name, props in _model.element("W-S")["psets"].items() if "LoadBearing" in props)
        flag = _model.information("W-S", "LoadBearing", language)
        roof = self._roof_note((1.2, -1.75, 0))
        chip = VGroup(frame_box(5.9, 0.95, GOOD).move_to((RIGHT, -0.72, 0)),
                      self.text(f"{pset}.LoadBearing = {'TRUE' if flag['value'] else 'FALSE'}", (RIGHT, -0.55, 0), size=17.5, color=GOOD, width=5.5),
                      self.label("infill on the concrete frame: it carries no load", "Ausfachung am Betonrahmen: sie trägt keine Last", (RIGHT, -0.9, 0), size=16.2, color=MUTED, width=5.5))
        link = segments([(cut + (0.05, 0.0, 0), (x0 - 0.2, y0 - height / 2, 0)), (cut + (0.05, 0.0, 0), (x0 - 0.2, y0 + height / 2, 0))], DATA, 1.0, 0.6)
        self._show("buildup", (0.8, [self._leave(), FadeIn(plan)]), (0.6, [FadeIn(mark), FadeIn(link), FadeIn(sides)]),
                   (1.8, [FadeIn(live, run_time=0.1), grow.animate(rate_func=SMOOTH).set_value(1.0), FadeIn(names, run_time=1.0)]), (0.6, [FadeIn(chip), FadeIn(roof)]))
        self.stage = VGroup(plan, mark, link, sides, live, names, chip, roof)

    def _structure(self):
        """🏗️ Beat 4: the structure model; what the architecture model has and this one leaves out stays as a ghost."""
        struct = self.villa_axon(AT, SCALE, disciplines=("structure",), ground=False)
        self.ghost = self.villa_axon(AT, SCALE, disciplines=("architecture",), ground=False, ghost=("walls", "windows", "curtain", "ramp", "screens", "slabs", "spaces"))
        card = self._card("structure", 2.1)
        spec = self._discipline_rows("structure")
        rows, steps = self._rows("structure", spec, 1.3, struct, pitch=0.28)
        steps[0] = (1.5, steps[0][1])
        total = self._sum_line("structure", [name for name, _ in spec], -1.8)
        flag = _model.information(_model.role("column_example"), "LoadBearing", self.language())
        pset = next(name for name, props in _model.element(_model.role("column_example"))["psets"].items() if "LoadBearing" in props)
        piloti_note = self._legend((0.95, -0.38, 0), piloti=True)
        chip = VGroup(frame_box(5.9, 0.46, GOOD).move_to((RIGHT, -0.85, 0)),
                      self.text(f"{pset}.LoadBearing = {'TRUE' if flag['value'] else 'FALSE'}", (RIGHT, -0.85, 0), size=16.2, color=GOOD, width=5.0))
        c2 = next(item for item in self.geometry["pilotis"] if item["tag"] == _model.role("column_example"))
        top = struct.at(c2["x"], c2["y"], c2["top"])
        ring = segments([(struct.at(c2["x"], c2["y"], c2["base"]), top)], GOOD, 4.5)
        leader = self.leader(chip[0], top, color=GOOD, dot=True)
        arch_counts, kept = self.info["architecture"]["counts_by_class"], self.info["structure"]["counts_by_class"]
        missing = {name: count for name, count in arch_counts.items() if name not in kept}
        layer_count = len(_model.element("W-S")["material"]["layers"])
        walls = len(self._tags("architecture", "IfcWall"))
        omitted = VGroup(self.label("Not in this model", "Nicht in diesem Modell", (0.95, -1.4, 0), size=16.2, color=MUTED, anchor="l"),
                         *self._lines([" · ".join(f"{name} ×{self.number(count, 0)}" for name, count in list(missing.items())[:3]),
                                       " · ".join(f"{name} ×{self.number(count, 0)}" for name, count in list(missing.items())[3:]),
                                       self.tr(f"render: layer 1 of {self.number(layer_count, 0)} in {self.number(walls - 1, 0)} walls", f"Putz: Schicht 1 von {self.number(layer_count, 0)} in {self.number(walls - 1, 0)} Wänden")],
                                      (0.95, -1.72, 0), size=17.0, color=WARN, pitch=0.3, width=5.7))
        ghost_parts = VGroup(*[self.ghost.parts[key] for tag in self.info["architecture"]["element_tags"] if tag not in self.info["structure"]["element_tags"] for key in self._keys(self.ghost, tag)])
        self.ghost.set_stroke(opacity=0.3, width=0.8).set_fill(opacity=0.04)
        self._show("structure", (0.8, [self._leave(), FadeIn(struct), FadeIn(self.ghost), FadeIn(card)]), *steps,
                   (0.7, [FadeIn(total)]),
                   (0.9, [FadeOut(rows), FadeOut(total), FadeIn(piloti_note), FadeIn(chip), FadeIn(ring), FadeIn(leader), Indicate(struct.parts[_model.role("column_example")], color=DATA, scale_factor=1.6)]),
                   (1.0, [FadeIn(omitted), self._flash(ghost_parts, WARN)]))
        self.stage = VGroup(struct, self.ghost, card, rows, total, piloti_note, chip, ring, leader, omitted)

    def _services(self):
        """🔧 Beat 5: the services model over the architecture ghost: boiler, radiator, drains, downpipes and the hanging garden they drain."""
        services = self.villa_axon(AT, SCALE, disciplines=("services",), ground=False).set_stroke(width=3.2)
        card = self._card("services", 2.1)
        spec = self._discipline_rows("services")
        rows, steps = self._rows("services", spec, 1.3, services, pitch=0.28)
        self.ghost.set_stroke(opacity=0.16, width=0.7).set_fill(opacity=0.0)
        steps[next(i for i,(kind,_) in enumerate(spec) if kind=="IfcSpaceHeater")][1].append(self._flash(self.ghost.parts["OP-S"], ARCH))
        names = VGroup(self.leader((-6.55, -1.35, 0), services.at("RAD-1"), self.tr("radiator RAD-1", "Heizkörper RAD-1"), color=SERV, size=16.2, anchor="l"),
                       self.leader((-6.55, -1.9, 0), services.at("BL-1"), self.tr("boiler BL-1", "Kessel BL-1"), color=SERV, size=16.2, anchor="l"))
        total = self._sum_line("services", [name for name, _ in spec], -1.1)
        garden = next(space for space in self.geometry["spaces"] if space["id"] == "SP-TERRACE")
        level = self.geometry["storeys"][garden["storey"]]
        hatched = self.hatch([services.at(x, y, level)[:2] for x, y in garden["outline"]], GOOD, 0.07)
        area, flow = ValueTracker(0), ValueTracker(0)
        drains = self.number(len(self._tags("services", "IfcWasteTerminal")), 0)
        row = VGroup(self._counter(area, (0.95, -0.5, 0), size=19.0, color=GOOD),
                     self.label("m² hanging garden, first floor, open air", "m² hängender Garten, 1. OG, offen", (1.1, -0.5, 0), size=16.2, color=GOOD, anchor="l", width=5.4),
                     *self._lines([self.tr("its floor drains are not modelled;", "seine Bodenabläufe sind nicht modelliert;"),
                                   self.tr(f"the {drains} roof drains belong to the roof level", f"die {drains} Dachabläufe gehören zur Dachebene")], (1.1, -0.76, 0), size=15.5, color=MUTED, pitch=0.22, width=5.4))
        row.shift((0,-1.0,0))
        scenario = self._lines([self.tr("Original service identities; detailed routes and", "Ursprüngliche Haustechnik; genaue Leitungswege und"),
                                self.tr("equipment envelopes retain explicit uncertainties.", "Gerätehüllen behalten ausdrückliche Unsicherheiten.")], (0.95, -2.25, 0), width=5.6)
        pipes = [item["points"] for item in self.geometry["services"] if item["kind"] == "downpipe"]

        def drops():
            t = flow.get_value()
            return VGroup(*[Dot(services.at(*[a + (b - a) * t for a, b in zip(top, foot)]), radius=0.05, color=_drafting.GLASS) for top, foot in pipes]) if 0 < t < 0.97 else VGroup()

        water = self.live(drops, key=lambda: round(flow.get_value(), 2))
        self.add(water)
        self._show("services", (0.8, [self._leave(self.ghost), FadeIn(services), FadeIn(card)]), *steps, (0.5, [FadeIn(total)]),
                   (1.1, [FadeIn(hatched), FadeIn(row), FadeIn(names), area.animate(rate_func=LINEAR).set_value(garden["area"]), flow.animate(rate_func=LINEAR).set_value(1.0)]),
                   (0.5, [FadeIn(scenario)]))
        self.remove(water)
        self.stage = VGroup(services, self.ghost, card, rows, total, hatched, row, names, scenario)

    def _schema(self):
        """🧬 Beat 6: one matrix of IFC classes against the three models, every class in every column, zero where a model has none."""
        language = self.language()
        counts = {name: self.info[name]["counts_by_class"] for name in DISCIPLINES}
        classes = sorted(set().union(*counts.values()))
        schema = self.info["architecture"]["schema_id"]
        bar = VGroup(frame_box(13.0, 0.4, GOOD).move_to((0, 2.82, 0)),
                     self.text(f"{schema} · " + self.tr("one schema: every class exists for every model, every object has a GlobalId (IfcRoot)",
                                                       "ein Schema: jede Klasse gibt es für jedes Modell, jedes Objekt hat eine GlobalId (IfcRoot)"), (0, 2.82, 0), size=17.0, color=GOOD, width=12.6))
        top, pitch, xs = 0.6, min(0.232,3.2/max(1,len(classes)-1)), (-2.7, 1.1, 4.8)
        labels = VGroup(*[self.text(name, (-6.6, top - index * pitch, 0), size=15.5, color=GOOD if sum(name in counts[m] for m in DISCIPLINES) > 1 else INK, anchor="l")
                          for index, name in enumerate(classes)])
        parts, columns = VGroup(), []
        for x, name in zip(xs, DISCIPLINES):
            tone, info, drive = COLORS[name], self.info[name], ValueTracker(0)
            drawing = self._arch((x - 0.9, 1.7, 0), 0.055) if name == "architecture" else VGroup(self._arch((x - 0.9, 1.7, 0), 0.055).set_opacity(0.14),
                                                                                                  self.villa_axon((x - 0.9, 1.7, 0), 0.055, disciplines=(name,), ground=False))
            head = VGroup(self.text(info["name"][language], (x + 0.2, 1.95, 0), size=17.0, color=tone, anchor="l", width=(1.65, 1.55, 1.7)[DISCIPLINES.index(name)]),
                          self.text(self._revision(info["revision"]), (x + 0.2, 1.62, 0), size=16.2, color=INK, anchor="l"))
            cells = VGroup()
            for index, ifc_class in enumerate(classes):
                value, peak = counts[name].get(ifc_class, 0), max(counts[m].get(ifc_class, 0) for m in DISCIPLINES)
                y = top - index * pitch
                if value:
                    cells.add(Rectangle(width=1.5, height=0.2, stroke_width=0, fill_color=tone, fill_opacity=0.14 + 0.5 * value / peak).move_to((x, y, 0)),
                              self.live_label(lambda drive=drive, value=value, star=(ifc_class == "IfcColumn"): self.number(round(drive.get_value() * value), 0) + (STAR if star else ""), (x, y, 0), size=15.5, color=INK))
                else:
                    cells.add(self.text("·", (x, y, 0), size=15.5, color=MUTED))
            total = len(info["element_guids"])
            foot = VGroup(segments([((x - 0.75, top - len(classes) * pitch + 0.1, 0), (x + 0.75, top - len(classes) * pitch + 0.1, 0))], tone, 1.2),
                          self.live_label(lambda drive=drive, total=total: "= " + self.number(round(drive.get_value() * total), 0) + " " + self.tr("objects", "Objekte"),
                                          (x, top - len(classes) * pitch - 0.05, 0), size=16.2, color=tone))
            parts.add(drawing, head, cells, foot)
            columns.append((0.4, [FadeIn(VGroup(drawing, head, cells, foot))]))
            columns.append((0.9, [drive.animate(rate_func=LINEAR).set_value(1.0), Indicate(drawing, color=DATA, scale_factor=1.0)]))
        key_note = self._legend((-6.6, 2.15, 0), piloti=True, width=2.0)
        self._show("schema", (0.7, [self._leave(), FadeIn(bar), FadeIn(labels), FadeIn(key_note)]), *columns)
        self.stage = VGroup(bar, labels, parts, key_note)

    def _federation(self):
        """🧱 Beat 7: explode 1 → 0 lowers the three layers onto one origin and grid; the leaders to the shared slab GlobalId meet in one point."""
        at, scale = np.array([-3.8, 0.5, 0.0]), 0.15
        explode = ValueTracker(1.0)
        layers = {"services": self.villa_axon(at, scale, disciplines=("services",), ground=False),
                  "structure": self.villa_axon(at, scale, disciplines=("structure",), grid=True, shell=0.3, ground=False),
                  "architecture": self._arch(at, scale)}
        base = {name: np.array(layer.get_center()) for name, layer in layers.items()}

        def offset(name):
            return np.asarray(P(self.axon_point(0, 0, 0, at, scale, discipline=name, explode=explode.get_value())), float) - np.asarray(P(self.axon_point(0, 0, 0, at, scale)), float)

        for name, layer in layers.items():
            layer.add_updater(lambda mob, name=name: mob.move_to(base[name] + offset(name)))
            layer.update()
        layers["structure"].grid.submobjects[-1].shift(_engine.RIGHT * 0.35 + _engine.DOWN * 0.62)
        slab = next(item for item in self.geometry["slabs"] if item["kind"] == "FLOOR")
        spot = (0.75 * self.geometry["box"]["length"], 0.5 * self.geometry["box"]["width"], slab["top"])
        chip = self.guid_chip("SL-1", (RIGHT, 0.35, 0), color=DATA)
        marks = VGroup()
        for name in ("architecture", "structure"):
            dot = Dot(radius=0.06, color=COLORS[name])
            dot.add_updater(lambda mob, name=name: mob.move_to(self.axon_point(*spot, at, scale, discipline=name, explode=explode.get_value())))
            dot.update()
            corner = chip.frame.get_corner(_engine.UL if name == "architecture" else _engine.DL)
            line = Line(dot.get_center(), corner, stroke_width=1.3, color=COLORS[name])
            line.add_updater(lambda mob, name=name, corner=corner: mob.put_start_and_end_on(self.axon_point(*spot, at, scale, discipline=name, explode=explode.get_value()), corner))
            marks.add(dot, line)
        panel = VGroup()
        for index, name in enumerate(DISCIPLINES):
            info, y = self.info[name], 2.4 - index * 0.62
            panel.add(VGroup(frame_box(5.0, 0.54, COLORS[name]).move_to((RIGHT + 0.3, y, 0)),
                             self.text(info["name"][self.language()], (RIGHT - 2.1, y + 0.12, 0), size=17.0, color=COLORS[name], anchor="l", width=2.8),
                             self.text(info["author_label"][self.language()], (RIGHT - 2.1, y - 0.14, 0), size=15.0, color=INK, anchor="l", width=3.1),
                             self.text(self._revision(info["revision"]), (RIGHT + 2.7, y, 0), size=17.0, color=COLORS[name], anchor="r")))
        membership = Counter(guid for info in self.info.values() for guid in info["element_guids"])
        shared = sum(1 for count in membership.values() if count > 1)
        models = membership[chip.guid] if chip.guid in membership else sum(chip.guid in info["element_guids"] for info in self.info.values())
        same = self.label(f"same GlobalId in {self.number(models, 0)} models · {self.number(shared, 0)} shared objects in all",
                          f"gleiche GlobalId in {self.number(models, 0)} Modellen · {self.number(shared, 0)} gemeinsame Objekte insgesamt", (RIGHT, -0.18, 0), size=16.2, color=DATA, width=5.8)
        note = self._lines([self.tr("Teaching model: both reference one slab.", "Lehrmodell: beide verweisen auf eine Decke.")], (RIGHT - 2.7, -0.7, 0), width=5.4)
        revisions = " · ".join(f"{self.tr('rev', 'Rev.')} {self.info[name]['revision']}" for name in DISCIPLINES)
        federated = self.text(self.tr("Federated: ", "Zusammengeführt: ") + revisions, (RIGHT - 2.7, -1.5, 0), size=17.0, color=INK, anchor="l", width=5.4)
        self._show("federation",
                   (0.9, [self._leave(), *[FadeIn(layers[name]) for name in layers], FadeIn(panel), FadeIn(marks), FadeIn(chip), FadeIn(same)]),
                   (3.0, [explode.animate(rate_func=SMOOTH).set_value(0.0), FadeIn(note, run_time=2.0)]),
                   (0.8, [Flash(self.axon_point(*spot, at, scale, discipline="structure"), color=DATA, flash_radius=0.4), FadeIn(federated)]))
        self.stage = VGroup(*layers.values(), marks, panel, chip, same, note, federated)

    def _slab(self):
        """🟫 Beat 8: the first-floor slab as one object: gross area minus all source openings in plan, thickness in section, read by two models."""
        slab = self.analysis["slab"]
        data = next(item for item in self.geometry["slabs"] if item["kind"] == "FLOOR")
        plan = self.villa_plan("first", (-3.3, 1.0, 0), 0.17, layers=("structure",), grid=False, ramp_arrow=False)
        outline = [plan.to_screen(x, y)[:2] for x, y in data["outline"]]
        openings = [(tag, ring, abs(_drafting.signed_area(ring))) for tag, ring in zip(data["void_tags"], data["voids"])]
        voids = [[plan.to_screen(x, y)[:2] for x, y in ring] for _, ring, _ in openings]
        hatched = hatch([outline, *voids], ARCH, 0.08, 45, 0.8, 0.8)
        hole = VGroup(*[polygon(void, WARN, 0.35, WARN, 1.8) for void in voids])
        opening_labels = VGroup(*[self.text(f"{tag} · {self.quantity(area, 'm²', 2)}{STAR}", (-5.4, -0.6 - index * 0.27, 0), size=14.0, color=WARN, anchor="l", width=4.2)
                                  for index, (tag, _, area) in enumerate(openings)])
        length, width = self.geometry["box"]["length"], self.geometry["box"]["width"]
        edge = polyline(outline, INK, 2.0, closed=True)
        dims = VGroup(self.dimension(outline[0], outline[1], self.quantity(length, "m"), offset=-0.2, color=INK, size=16.2),
                      self.dimension(outline[1], outline[2], self.quantity(width, "m"), offset=-0.2, color=INK, size=16.2))
        gross, cut, net = ValueTracker(0), ValueTracker(0), ValueTracker(0)
        formula = self._equation([(lambda: self.quantity(gross.get_value(), "m²"), INK, self.quantity(slab["gross_area"], "m²")), ("−", MUTED, "−"),
                                  (lambda: self.quantity(cut.get_value(), "m²"), WARN, self.quantity(slab["void_area"], "m²")), ("=", MUTED, "="),
                                  (lambda: self.quantity(net.get_value(), "m²"), ARCH, self.quantity(slab["net_area"], "m²"))], (-3.3, -1.2, 0), size=18.0)
        legend = VGroup(*[self.text(text, (x, -1.5, 0), size=15.0, color=color, anchor="c") for text, x, color in
                          ((self.tr("gross outline", "Bruttofläche"), -5.35, INK), (self.tr("openings", "Öffnungen"), -3.3, WARN), (self.tr("net, hatched", "netto, schraffiert"), -1.3, ARCH))])
        zoom, x0, y0 = 2.4, -5.0, -2.1
        thickness = slab["thickness"] * zoom
        box = [(x0, y0 - thickness / 2, 0), (x0 + 3.4, y0 - thickness / 2, 0), (x0 + 3.4, y0 + thickness / 2, 0), (x0, y0 + thickness / 2, 0)]
        section = VGroup(polygon(box, STRUCT, 0.3, STRUCT, 1.8), self.hatch(box, STRUCT, 0.07, 45, 0.8, 0.8),
                         self.dimension((x0 - 0.05, y0 - thickness / 2, 0), (x0 - 0.05, y0 + thickness / 2, 0), self.quantity(slab["thickness"], "m") + STAR, offset=0.15, color=DATA, size=16.2),
                         self.label("slab section", "Deckenschnitt", (x0 + 3.55, y0, 0), size=16.2, color=MUTED, anchor="l"))
        left = self._property_card("architecture", 2.05, [("Qto_SlabBaseQuantities.NetArea", self.quantity(slab["net_area"], "m²") + STAR),
                                                         ("Pset_SlabCommon.IsExternal", self._flag("SL-1", "IsExternal") + STAR)])
        right = self._property_card("structure", -0.35, [(self.tr("Thickness", "Dicke"), self.quantity(slab["thickness"], "m") + STAR),
                                                        ("Pset_SlabCommon.LoadBearing", self._flag("SL-1", "LoadBearing") + STAR)])
        chip = self.guid_chip("SL-1", (RIGHT, 0.85, 0), color=DATA)
        ties = VGroup(segments([((RIGHT, 0.85 + 0.4, 0), (RIGHT, 1.4, 0))], DATA, 1.4), segments([((RIGHT, 0.85 - 0.4, 0), (RIGHT, 0.3, 0))], DATA, 1.4))
        both = VGroup(self.label("one object, one GlobalId", "ein Objekt, eine GlobalId", (RIGHT, -1.3, 0), size=16.2, color=DATA),
                      self.label(f"teaching box {self.quantity(length, 'm', 1)} × {self.quantity(width, 'm', 1)}; sources report other footprints", f"Lehrbox {self.quantity(length, 'm', 1)} × {self.quantity(width, 'm', 1)}; Quellen nennen andere Grundflächen", (RIGHT, -1.75, 0), size=15.5, color=MUTED, width=5.8), self._roof_note((0.95, -2.2, 0)), self._legend((0.95, -2.45, 0), assumed=True))
        grid_x = self.geometry["grid"]["x"]
        spacing = grid_x[1] - grid_x[0]
        first_bay = self.dimension(plan.to_screen(grid_x[0], width), plan.to_screen(grid_x[1], width), self.quantity(spacing, "m"), offset=0.2, color=STRUCT, size=15.5)
        dims.add(first_bay, self.scale_bar((-1.0, 2.5, 0), 0.17, 5.0, steps=2))
        self._show("slab", (0.8, [self._leave(), FadeIn(plan), FadeIn(dims), FadeIn(legend)]),
                   (0.9, [FadeIn(edge), gross.animate(rate_func=LINEAR).set_value(slab["gross_area"]), FadeIn(formula, run_time=0.2)]),
                   (0.9, [FadeIn(hole), FadeIn(opening_labels), cut.animate(rate_func=LINEAR).set_value(slab["void_area"])]),
                   (1.0, [FadeIn(hatched), net.animate(rate_func=LINEAR).set_value(slab["net_area"])]),
                   (0.8, [FadeIn(section), FadeIn(chip), FadeIn(ties), FadeIn(left), FadeIn(right), FadeIn(both)]))
        self.stage = VGroup(plan, edge, dims, legend, hatched, hole, opening_labels, formula, section, chip, ties, left, right, both)

    def _flag(self, tag, property_id) -> str:
        """🚩 A boolean or label property value as text."""
        value = _model.information(tag, property_id, self.language())["value"]
        return ("TRUE" if value else "FALSE") if isinstance(value, bool) else str(value)

    def _property_card(self, name, y, rows):
        """📋 A small property table of one discipline model: ``(name, value, basis)`` rows."""
        card = self.property_table([(label, value, "", "assumed") for label, value in rows], (RIGHT, y, 0), width=5.9, basis=False,
                                   title=self.info[name]["name"][self.language()] + " · " + self._revision(self.info[name]["revision"]), color=COLORS[name])
        return card

    def _owners(self):
        """🗺️ Beat 9: the ground-floor plan with every line in the colour of the model it comes from."""
        plan = self._ground_plan()
        self.plan = plan
        spacing = _model.clearance()["grid_spacing"]
        rows, steps = VGroup(), []
        for index, name in enumerate(DISCIPLINES):
            tags = [tag for tag in self.info[name]["element_tags"] if tag in plan.parts and not tag.startswith("SL-")]
            classes = Counter(_model.element(tag)["class"] for tag in tags)
            y, tracker = 1.6 - index * 1.15, ValueTracker(0)
            row = VGroup(frame_box(5.9, 1.0, COLORS[name]).move_to((RIGHT, y, 0)),
                         self.text(self.info[name]["name"][self.language()], (RIGHT - 2.8, y + 0.26, 0), size=19.0, color=COLORS[name], anchor="l", width=3.0),
                         self._counter(tracker, (RIGHT + 2.0, y + 0.26, 0), size=21.0),
                         self.label("objects", "Objekte", (RIGHT + 2.1, y + 0.26, 0), size=16.2, color=MUTED, anchor="l", width=0.7),
                         self.text(" · ".join(f"{ifc_class} ×{self.number(count, 0)}" + (STAR if ifc_class == "IfcColumn" else "") for ifc_class, count in classes.items()), (RIGHT - 2.8, y - 0.14, 0), size=15.0, color=INK, anchor="l", width=5.6))
            rows.add(row)
            lights = LaggedStart(*[Indicate(plan.parts[tag], color=DATA, scale_factor=1.0) for tag in tags], lag_ratio=min(0.5, 2.0 / max(len(tags), 1)))
            steps.append((0.9, [FadeIn(row, run_time=0.2), tracker.animate(rate_func=LINEAR).set_value(len(tags)), lights]))
        grid = VGroup(self.label(f"Structural grid {self.quantity(spacing, 'm')}, sourced", f"Tragraster {self.quantity(spacing, 'm')}, Quelle", (RIGHT, -1.95, 0), size=17.0, color=STRUCT, width=5.6),
                     self._legend((RIGHT - 2.9, -2.3, 0), piloti=True))
        head = self.label("Objects drawn on the ground-floor plan", "Im Grundriss gezeichnete Objekte", (RIGHT, 2.7, 0), size=17.0, color=MUTED, width=5.6)
        self._show("owners", (0.9, [self._leave(), FadeIn(plan), FadeIn(head)]), *steps, (0.5, [FadeIn(grid)]))
        self.stage = VGroup(plan, head, rows, grid)

    def _check(self):
        """⚠️ Draw the hypothetical routing check from native occupied pipe and column sections."""
        clash,plan=self.clash,self._ground_plan(model=self.scenario)
        self.remove(self.plan)
        self.add(plan)
        pipe=next(item for item in self.scenario_geometry["services"] if item["tag"]==clash["a_tag"])
        piloti=next(item for item in self.scenario_geometry["pilotis"] if item["tag"]==clash["b_tag"])
        z=(clash["a_point"][2]+clash["b_point"][2])/2
        profiles={item["tag"]:[ring for part in item["parts"] if part["z0"]<=z<=part["z1"] for ring in _model.part_regions(part,z)] for item in (pipe,piloti)}
        all_points=np.asarray([p for rings in profiles.values() for ring in rings for p in ring])
        low,high=np.min(all_points,axis=0),np.max(all_points,axis=0)
        zoom=2.5/max(high-low)
        centre=np.array([3.6,1.05,0.0])
        midpoint=(low+high)/2
        at=lambda p:centre+zoom*np.array([p[0]-midpoint[0],p[1]-midpoint[1],0.0])
        spot=np.asarray(P(plan.to_screen(*clash["point"][:2])), float)
        marker=Circle(radius=.26,stroke_color=WARN,stroke_width=2.4).move_to(spot)
        glass=Circle(radius=1.6,stroke_color=WARN,stroke_width=2,fill_color=BACKGROUND,fill_opacity=1).move_to(centre)
        link=segments([(spot,centre+(spot-centre)/max(np.linalg.norm(spot-centre),1e-9)*1.6)],WARN,1.1,.55)
        solid=VGroup(*[polygon([at(p) for p in ring],color,.7,color,1.2) for item,color in ((piloti,STRUCT),(pipe,SERV)) for ring in profiles[item["tag"]]])
        for item,color in ((piloti,STRUCT),(pipe,SERV)):
            points=np.asarray([p for ring in profiles[item["tag"]] for p in ring])
            solid.add(self.text(item["tag"],at(np.mean(points,axis=0))+(0,.6 if item is pipe else -.6,0),size=16.2,color=color))
        first,second=np.asarray(clash["b_point"][:2]),np.asarray(clash["a_point"][:2])
        start,end=at(first),at(second)
        middle=(start+end)/2
        dimension=VGroup(self.dimension(start,end,None,offset=.12,color=WARN,tick=.06),self.text(self.quantity(clash["distance"],"m"),middle+(0,.4,0),size=16.2,color=WARN))
        pipe_radius=pipe["diameter"]/2
        ring_radius=pipe_radius+clash["required"]
        pipe_at=at(pipe["points"][0][:2])
        ring_value=ValueTracker(0)
        ring=self.live(lambda:VGroup(polyline(circle_points(pipe_at,max(ring_value.get_value()*zoom,1e-3),48),WARN,1.8,closed=True,dash=.1)),key=lambda:round(ring_value.get_value(),3))
        clearance,required,short=(self.number(clash[key]) for key in ("distance","required","deficit"))
        card=VGroup(self.text(self.tr(f"{clash['id']} · pipe {clash['a_tag']} ↔ support {clash['b_tag']}",f"{clash['id']} · Rohr {clash['a_tag']} ↔ Stütze {clash['b_tag']}"),(RIGHT,-.92,0),size=18,color=INK,width=5.9),
                    self.text(self.tr(f"clearance {clearance} m < required {required} m{STAR} · short by {short} m",f"Abstand {clearance} m < gefordert {required} m{STAR} · es fehlen {short} m"),(RIGHT,-1.2,0),size=17.5,color=WARN,width=5.9),
                    self.label("Hypothetical routing · native hollow pipe section","Hypothetische Führung · nativer hohler Rohrquerschnitt",(RIGHT,-1.45,0),size=16.2,color=MUTED,width=5.9),self._legend((RIGHT-2.95,-1.69,0),assumed=True))
        chips=VGroup(self.guid_chip(clash["a_tag"],(1.95,-2.17,0),color=SERV),self.guid_chip(clash["b_tag"],(5.15,-2.17,0),color=STRUCT))
        self._show("check",(.8,[self._leave(plan),Indicate(plan.parts[clash["a_tag"]],color=WARN,scale_factor=2),Indicate(plan.parts[clash["b_tag"]],color=WARN,scale_factor=2)]),
                   (1.2,[FadeIn(marker),FadeIn(link),FadeIn(glass),FadeIn(solid)]),(1.2,[FadeIn(ring,run_time=.1),ring_value.animate(rate_func=SMOOTH).set_value(ring_radius),FadeIn(dimension)]),(.8,[FadeIn(card),FadeIn(chips)]))
        self.stage=VGroup(plan,marker,link,glass,solid,ring,dimension,card,chips)

    def _analysis(self, point, scale, projection="axon", **kwargs):
        """🧮 The analysis model without its node dots and supports; those come in later beats."""
        view = self.analysis_view(self.analysis, point, scale, projection=projection, **kwargs)
        view.remove(view.nodes, view.supports)
        return view

    def _translation(self):
        """🔁 Beat 11: the structure model on the left, the analysis model on the villa on the right, the three mapping rules in between."""
        analysis, language = self.analysis, self.language()
        surfaces, shells = len({item["source_guid"] for item in analysis["members"] if item["kind"] == "surface"}), analysis["counts"]["shells"]
        struct = self.villa_axon((-4.85, 0.1, 0), 0.1, disciplines=("structure",), grid=True, shell=0.4, ground=False)
        struct.grid.remove(struct.grid.submobjects[-1])
        result = self._villa_analysis((4.85, 0.0, 0), 0.1, tags=(_model.role("column_example"),))
        source = self.text(f"{self.info['structure']['name'][language]} · {self._revision(analysis['source_revision'])}", (-4.85, 2.45, 0), size=17.5, color=STRUCT, width=3.4)
        target = self.label("Structural analysis model", "Tragwerksanalysemodell", (4.85, 2.45, 0), size=17.5, color=GOOD, width=3.4)
        schema = VGroup(self._chip(self.tr(analysis["target_schema"], f"{analysis['target_schema'].split(' ')[0]} · Strukturanalyse-Domäne"), (0, 2.65, 0), GOOD, 18.0),
                        self.label("the analysis domain of the same schema", "die Analysedomäne desselben Schemas", (0, 2.27, 0), size=16.2, color=MUTED, width=5.4))
        columns, slab_tag = self._tags("structure", "IfcColumn"), analysis["slab"]["tag"]
        one = self.number(surfaces, 0)
        slab_note = self.tr(f"{one} IfcSlab · {slab_tag} → {one} IfcStructuralSurfaceMember, meshed into {self.number(shells, 0)} shell elements",
                            f"{one} IfcSlab · {slab_tag} → {one} IfcStructuralSurfaceMember, vernetzt in {self.number(shells, 0)} Schalenelemente")
        mapping = analysis["mapping"]
        spec = [(mapping[0]["count"], mapping[0]["source_class"], mapping[0]["target_class"], mapping[0][language], lambda: self._lit(struct, columns), [result.parts[tag] for tag in columns]),
                (surfaces, f"IfcSlab · {slab_tag}", mapping[1]["target_class"], slab_note, lambda: self._lit(struct, [slab_tag]), [result.shells]),
                (mapping[2]["count"], mapping[2]["source_class"], mapping[2]["target_class"], mapping[2][language], lambda: self._lit(struct, columns, GOOD), [result.parts[tag] for tag in columns])]
        rules, steps = VGroup(), []
        for index, (count, origin_name, goal_name, text, light, lit_result) in enumerate(spec):
            y, tracker = 1.4 - index * 1.0, ValueTracker(0)
            origin = self._chip(origin_name, (-2.2, y, 0), STRUCT, 15.5, "l")
            start = origin.get_right()[0] + 0.05
            arrow = Arrow((start, y, 0), (start + 0.28, y, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.5)
            goal = self._chip(goal_name, (start + 0.36, y, 0), GOOD, 15.5, "l")
            note = VGroup(*[self.text(line, (-2.2, y - 0.36 - number * 0.27, 0), size=15.5, color=MUTED, anchor="l", width=5.0) for number, line in enumerate(self._wrap(text, 52))])
            rule = VGroup(self._counter(tracker, (-2.3, y, 0), size=20.0, unit=STAR if "IfcColumn" in origin_name else ""), origin, arrow, goal, note)
            rules.add(rule)
            steps.append((0.9, [FadeIn(rule), tracker.animate(rate_func=LINEAR).set_value(count), light(),
                                LaggedStart(*[Indicate(part, color=DATA, scale_factor=1.0) for part in lit_result], lag_ratio=min(0.5, 2.0 / len(lit_result)))]))
        carry = VGroup(Arrow((-2.8, -1.95, 0), (2.8, -1.95, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.1),
                       self.text(self.tr("translation of ", "Übersetzung von ") + f"{self.info['structure']['name'][language]} {self._revision(analysis['source_revision'])}", (0, -1.68, 0), size=16.2, color=MUTED, width=5.5))
        key_note = self._legend((-2.2, -2.3, 0), piloti=True)
        self._show("translation", (0.8, [self._leave(), FadeIn(struct), FadeIn(source)]), *steps,
                   (1.1, [FadeIn(schema), FadeIn(carry), FadeIn(result, shift=_engine.RIGHT * 0.3), FadeIn(target), FadeIn(key_note)]))
        self.stage = VGroup(struct, source, rules, schema, carry, result, target, key_note)

    def _members(self):
        """🧮 Show selected supports, the source slab, clipped surface members and actual slab nodes."""
        analysis, counts = self.analysis, self.analysis["counts"]
        actual={"curve_members":sum(m["kind"]=="curve" for m in analysis["members"]), "surface_members":sum(m["kind"]=="surface" for m in analysis["members"]),
                "shells":len(analysis["shells"]),"slab_nodes":sum(n["role"]=="slab" for n in analysis["nodes"])}
        if any(counts[key]!=value for key,value in actual.items()):
            raise ValueError("Analysis counts must match the actual clipped members and nodes")
        surfaces = len({item["source_guid"] for item in analysis["members"] if item["kind"] == "surface"})
        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=(), ramp_arrow=False)
        base.add(self._chains(base), self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan", supports=False)
        groups = {"members": list(mesh.members.submobjects), "shells": list(mesh.shells.submobjects), "nodes": list(mesh.nodes.submobjects)}
        groups["members"] = [VGroup(member, Circle(radius=0.11, stroke_color=STRUCT, stroke_width=1.6).move_to(member.get_center())) for member in groups["members"]]
        mesh.remove(mesh.members, mesh.shells, mesh.nodes, mesh.outline)
        slab_tag = analysis["slab"]["tag"]
        spec = [("IfcStructuralCurveMember", counts["curve_members"], ("line members: selected slab supports", "Stabelemente: gewählte Deckenstützen"), groups["members"], STRUCT),
                ("IfcSlab", surfaces, (f"source slab: {slab_tag}", f"Quelldecke: {slab_tag}"), [mesh.outline], STRUCT),
                ("IfcStructuralSurfaceMember", counts["surface_members"], ("regional patches clipped at native voids", "regionale Flächen mit nativen Aussparungen"), groups["shells"], GOOD),
                ("IfcStructuralPointConnection", counts["slab_nodes"], ("actual nodes at patch and axis intersections", "tatsächliche Knoten an Flächen- und Achsenschnitten"), groups["nodes"], DATA)]
        rows, steps = VGroup(), []
        for index, (name, value, text, parts, color) in enumerate(spec):
            y, tracker = 2.45 - index * 0.68, ValueTracker(0)
            rows.add(VGroup(self._counter(tracker, (0.95, y, 0), size=21.0, unit=STAR if value == counts["curve_members"] else ""), self.text(name, (1.1, y, 0), size=17.5, color=color, anchor="l", width=5.5),
                            self.label(*text, (1.1, y - 0.28, 0), size=15.5, color=MUTED, anchor="l", width=5.5)))
            reveal = LaggedStart(*[FadeIn(part) for part in parts], lag_ratio=min(0.6, 1.0 / max(len(parts), 1)))
            steps.append((1.0, [FadeIn(rows[-1], run_time=0.2), tracker.animate(rate_func=LINEAR).set_value(value), reveal]))
        void, gross, net = analysis["slab"]["void_area"], analysis["slab"]["gross_area"], analysis["slab"]["net_area"]
        notes = VGroup(*self._lines([self.tr("line member ≠ column solid", "Stabelement ≠ Stützenkörper"), self.tr("surface member ≠ slab solid", "Flächenelement ≠ Deckenkörper")],
                                    (0.95, -0.35, 0), size=17.0, color=INK, pitch=0.32, width=5.6),
                       *self._lines([self.tr(f"The mesh retains native ramp, stair and service voids of {self.quantity(void, 'm²')}:", f"Das Netz berücksichtigt native Rampen-, Treppen- und Haustechniköffnungen von {self.quantity(void, 'm²')}:"),
                                     self.tr(f"analysis slab {self.quantity(net, 'm²')}, model slab {self.quantity(net, 'm²')}.", f"Analysedecke {self.quantity(net, 'm²')}, Modelldecke {self.quantity(net, 'm²')}.")],
                                    (0.95, -1.25, 0), size=16.2, color=WARN, pitch=0.28, width=5.7))
        roof = VGroup(self._roof_note((0.95, -2.05, 0)), self._legend((0.95, -2.35, 0), piloti=True))
        self._show("members", (0.9, [self._leave(), FadeIn(base)]), *steps, (0.8, [FadeIn(notes), FadeIn(roof)]))
        self.stage = VGroup(base, rows, notes, roof, mesh, *[VGroup(*items) for items in groups.values()], mesh.outline)

    def _midsurface(self):
        """✂️ Beat 13: a section through piloti 19 and the slab; the solids idealise into the mid-surface and a line member that ends on it."""
        analysis, language = self.analysis, self.language()
        piloti = next(item for item in self.geometry["pilotis"] if item["tag"] == _model.role("column_example"))
        member = next(item for item in analysis["members"] if item["kind"] == "curve" and item["source_tag"] == _model.role("column_example"))
        slab = dict(analysis["slab"])
        mid_z = next(node["position"][2] for node in analysis["nodes"] if node["id"] == member["nodes"][1])
        soffit = piloti["top"]
        slab["thickness"] = 2 * (mid_z - soffit)
        half = mid_z - soffit
        if abs(member["physical_height"] + half - member["length"]) > 1e-6:
            raise ValueError("Member length must be the piloti height plus half the slab thickness")
        section = self.villa_section((-4.45, 0.6, 0), 0.24, cut=("x", piloti["x"]), layers=("structure", "architecture"), highlight=(_model.role("column_example"),), level_lines=False, labels=False)
        section.add(self.scale_bar((-6.5, -1.55, 0), 0.24, 5.0, steps=2),
                    self.leader((-5.0, -1.1, 0), section.at(3.0, 3.5), analysis["slab"]["tag"] + self.tr(" first-floor slab", " Geschossdecke"), color=INK, size=16.2, anchor="l"),
                    self.text(_model.role("column_example"), section.at(piloti["y"], 0.0) + np.array([0.0, -0.25, 0.0]), size=16.2, color=DATA),
                    self.leader((-3.2, -0.8, 0), section.parts["RF-1A"].get_center(), self.tr("ramp flights", "Rampenläufe"), color=MUTED, size=16.2, anchor="l"))
        joint = section.at(piloti["y"], mid_z)
        zoom, cx, cy, reach, bottom = 1.6, 3.9, 0.45, 0.85, soffit - 0.8

        def at(h, z):
            return np.array([cx + h * zoom, cy + (z - mid_z) * zoom, 0.0])

        ideal = ValueTracker(0)

        def detail():
            t = ideal.get_value()
            slab_ring = [at(-reach, soffit), at(reach, soffit), at(reach, soffit + slab["thickness"]), at(-reach, soffit + slab["thickness"])]
            leg_ring = [at(-piloti["radius"], bottom), at(piloti["radius"], bottom), at(piloti["radius"], soffit), at(-piloti["radius"], soffit)]
            group = VGroup()
            for ring in (slab_ring, leg_ring):
                group.add(polygon(ring, STRUCT, 0.4 * (1 - 0.75 * t), STRUCT, 1.8, 1 - 0.6 * t), hatch(ring, STRUCT, 0.07, 45, 0.7, 0.8 * (1 - 0.7 * t)))
            if t > 0:
                group.add(polyline([at(-reach, mid_z), at(-reach + 2 * reach * t, mid_z)], GOOD, 2.2, dash=0.12),
                          polyline([at(0, bottom), at(0, bottom + (mid_z - bottom) * t)], STRUCT, 5.0))
            if t > 0.95:
                group.add(Dot(at(0, mid_z), radius=0.075, color=DATA))
            return group

        frame = frame_box(5.75, 3.3, MUTED, fill=0.0).move_to((3.875, cy - 0.15, 0))
        ring = Circle(radius=0.3, stroke_color=DATA, stroke_width=2.0).move_to(joint)
        links = segments([(joint + (0.2, 0.2, 0), (1.0, cy + 1.5, 0)), (joint + (0.2, -0.2, 0), (1.0, cy - 1.8, 0))], DATA, 1.0, 0.6)
        live = self.live(detail, key=lambda: round(ideal.get_value(), 2))
        lift = self.dimension(at(reach + 0.1, soffit), at(reach + 0.1, mid_z), self.quantity(half, "m", 3) + STAR, offset=-0.12, color=DATA, size=16.2)
        thick = self.dimension(at(-reach - 0.1, soffit), at(-reach - 0.1, soffit + slab["thickness"]), self.quantity(slab["thickness"], "m") + STAR, offset=0.15, color=DATA, size=16.2)
        names = VGroup(self.leader(at(-reach + 0.9, soffit + slab["thickness"]) + np.array([0.0, 0.35, 0.0]), at(-reach + 0.9, mid_z), self.tr("mid-surface", "Mittelfläche"), color=GOOD, size=15.5, anchor="r"),
                       self.label("line member", "Stabelement", at(0.1, bottom + 0.25) + (0.2, 0, 0), size=15.5, color=STRUCT, anchor="l"),
                       self.label("thickness: an attribute of the surface member", "Dicke: Attribut des Flächenelements", (cx, cy + 1.28, 0), size=16.2, color=DATA, width=4.2))
        terms = [self.quantity(value, "m", 3) + STAR for value in (member["physical_height"], half, member["length"])]
        formula = self._equation([(terms[0], STRUCT, terms[0]), ("+", MUTED, "+"), (terms[1], DATA, terms[1]), ("=", MUTED, "="), (terms[2], GOOD, terms[2])], (cx, -2.1, 0), size=19.0)
        title = VGroup(self.label("line member length", "Länge des Stabelements", (cx, -1.78, 0), size=16.2, color=MUTED), self._legend((cx - 1.6, -2.45, 0), assumed=True))
        self._show("midsurface", (0.9, [self._leave(), FadeIn(section), FadeIn(ring), FadeIn(links), FadeIn(frame), FadeIn(live)]),
                   (1.8, [ideal.animate(rate_func=SMOOTH).set_value(1.0)]), (0.9, [FadeIn(lift), FadeIn(thick), FadeIn(names)]), (0.6, [FadeIn(formula), FadeIn(title)]))
        self.stage = VGroup(section, ring, links, frame, live, lift, thick, names, formula, title)

    def _supports(self):
        """⏚ Show assumed fixed restraints at selected analytical births against the pinned alternative."""
        analysis, language = self.analysis, self.language()
        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=(), ramp_arrow=False)
        base.add(self._chains(base), self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan")
        supports = list(mesh.supports.submobjects)
        mesh.remove(mesh.supports)
        count, first = ValueTracker(0), analysis["supports"][0]
        row = VGroup(self._counter(count, (0.95, 2.4, 0), size=21.0, unit=STAR), self.text("IfcStructuralPointConnection", (1.1, 2.4, 0), size=17.5, color=DATA, anchor="l", width=5.5),
                     self.label("assumed restraints at selected analytical births", "angenommene Lager an gewählten analytischen Stützenanfängen", (1.1, 2.11, 0), size=15.5, color=MUTED, anchor="l", width=5.5))
        condition = VGroup(frame_box(5.9, 0.9, WARN).move_to((RIGHT, 1.35, 0)),
                           self.text(first["condition"] + STAR, (RIGHT - 2.8, 1.55, 0), size=17.5, color=INK, anchor="l", width=5.4),
                           self.text(self.tr("translations and rotations restrained", "Verschiebungen und Verdrehungen gebunden"), (RIGHT - 2.8, 1.17, 0), size=16.2, color=MUTED, anchor="l", width=5.6))
        ground, height = -1.0, 1.1
        sketches = VGroup()
        for x, fixed, name in ((2.2, False, ("pinned: rotation free", "gelenkig: Drehung frei")), (4.9, True, ("fixed: nothing moves", "eingespannt: starr"))):
            color = DATA if fixed else MUTED
            sketches.add(polygon([(x - 0.15, ground, 0), (x + 0.15, ground, 0), (x + 0.15, ground + height, 0), (x - 0.15, ground + height, 0)], STRUCT, 0.4, STRUCT, 1.6))
            if fixed:
                block = [(x - 0.55, ground, 0), (x + 0.55, ground, 0), (x + 0.55, ground - 0.35, 0), (x - 0.55, ground - 0.35, 0)]
                sketches.add(polygon(block, color, 0.18, color, 2.0), hatch(block, color, 0.07, 45, 0.9, 0.9))
            else:
                sketches.add(polygon([(x, ground, 0), (x - 0.3, ground - 0.4, 0), (x + 0.3, ground - 0.4, 0)], None, stroke=color, width=2.0), segments([((x - 0.5, ground - 0.4, 0), (x + 0.5, ground - 0.4, 0))], color, 2.0))
            sketches.add(self.label(*name, (x, ground - 0.75, 0), size=15.5, color=color, width=2.5))
        roof = VGroup(self._roof_note((0.95, -2.42, 0)), self._legend((0.95, -2.0, 0), assumed=True, piloti=True))
        chosen = VGroup(roof, self.label("chosen assumption", "gewählte Annahme", (4.9, ground + height + 0.3, 0), size=15.5, color=DATA), self.label("alternative", "Alternative", (2.2, ground + height + 0.3, 0), size=15.5, color=MUTED))
        pop = LaggedStart(*[FadeIn(glyph) for glyph in supports], lag_ratio=1.0 / len(supports))
        self._show("supports", (0.9, [self._leave(), FadeIn(base), FadeIn(mesh)]),
                   (1.4, [FadeIn(row, run_time=0.2), count.animate(rate_func=LINEAR).set_value(len(analysis["supports"])), pop]),
                   (1.0, [FadeIn(condition), FadeIn(sketches), FadeIn(chosen)]))
        self.stage = VGroup(base, mesh, VGroup(*supports), row, condition, sketches, chosen)

    def _loads(self):
        """⬇️ Beat 15: self-weight, finishes and imposed load build up q: arrows over the slab, a stacked bar and the formula follow one tracker."""
        analysis, language = self.analysis, self.language()
        loads, slab = analysis["loads"], analysis["slab"]
        parts = [("self_weight", STRUCT), ("finishes", GOOD), ("imposed", DATA)]
        values = [loads[key]["value"] for key, _ in parts]
        step = ValueTracker(0)

        def share(index):
            return values[index] * min(max(step.get_value() - index, 0.0), 1.0)

        def q():
            return sum(share(index) for index in range(3))

        axon = self._villa_analysis((-3.4, 0.2, 0), 0.17, nodes=True, tags=(_model.role("column_example"),))
        shells = [shell["id"] for shell in analysis["shells"]]
        arrows = self.live(lambda: self.load_arrows([q()] * len(shells), [axon.at(shell) for shell in shells], 0.05, per="m²", anchor="tip", colors=GOOD, labels=False, width=2.0),
                           key=lambda: round(q(), 2))
        readout = self.live_label(lambda: "q = " + self.quantity(q(), "kN/m²") + STAR, (-6.3, 2.35, 0), size=19.0, color=GOOD, anchor="l")
        x0, y0, zoom = 0.95, 1.65, 3.0
        strip = [(x0, y0 - slab["thickness"] * zoom / 2, 0), (x0 + 2.8, y0 - slab["thickness"] * zoom / 2, 0), (x0 + 2.8, y0 + slab["thickness"] * zoom / 2, 0), (x0, y0 + slab["thickness"] * zoom / 2, 0)]
        finish = [(x0, y0 + slab["thickness"] * zoom / 2, 0), (x0 + 2.8, y0 + slab["thickness"] * zoom / 2, 0), (x0 + 2.8, y0 + slab["thickness"] * zoom / 2 + 0.09, 0), (x0, y0 + slab["thickness"] * zoom / 2 + 0.09, 0)]
        section = VGroup(polygon(strip, STRUCT, 0.3, STRUCT, 1.8), self.hatch(strip, STRUCT, 0.07, 45, 0.8, 0.8), polygon(finish, GOOD, 0.8, GOOD, 1.2),
                         self.dimension(strip[0], strip[3], self.quantity(slab["thickness"], "m") + STAR, offset=0.15, color=STRUCT, size=16.2),
                         self.label("first-floor slab, section", "Geschossdecke, Schnitt", (x0 + 1.4, y0 + 0.62, 0), size=16.2, color=MUTED),
                         *self._lines([self.tr("RC ribs + hollow units + beam strips", "Stahlbetonrippen + Hohlkörper + Balkenstreifen"), self.quantity(loads["self_weight"]["density"], "kN/m³", 0) + STAR], (x0 + 3.0, y0 + 0.15, 0), size=16.2, color=STRUCT, pitch=0.3, width=2.6))
        equation = self._equation([("q =", INK, "q ="), (lambda: self.number(share(0)) + STAR, STRUCT, self.number(values[0]) + STAR), ("+", MUTED, "+"),
                                   (lambda: self.number(share(1)) + STAR, GOOD, self.number(values[1]) + STAR), ("+", MUTED, "+"), (lambda: self.number(share(2)) + STAR, DATA, self.number(values[2]) + STAR),
                                   ("=", MUTED, "="), (lambda: self.quantity(q(), "kN/m²") + STAR, INK, self.quantity(sum(values), "kN/m²") + STAR)], (RIGHT, 0.7, 0), size=19.0)
        sub = self.text(f"{self.number(values[0])}{STAR} = {self.quantity(loads['self_weight']['thickness'], 'm')}{STAR} × {self.quantity(loads['self_weight']['density'], 'kN/m³', 0)}{STAR}", (RIGHT, 0.28, 0, ), size=16.2, color=STRUCT)
        bar_x, bar_y, unit = 1.0, -0.4, 0.5

        def stack():
            group, left = VGroup(), bar_x
            for index, (_, color) in enumerate(parts):
                span = share(index) * unit
                if span > 1e-3:
                    group.add(polygon([(left, bar_y - 0.18, 0), (left + span, bar_y - 0.18, 0), (left + span, bar_y + 0.18, 0), (left, bar_y + 0.18, 0)], color, 0.85, INK, 1.2))
                left += span
            return group

        bar = self.live(stack, key=lambda: round(q(), 2))
        edges = [bar_x + sum(values[:i]) * unit for i in range(4)] + [bar_x + sum(values) * unit]
        legend = VGroup(self.text(loads["self_weight"][language], ((edges[0] + edges[1]) / 2, bar_y - 0.5, 0), size=15.5, color=STRUCT),
                        self.text(loads["finishes"][language], (edges[2] - 0.1, bar_y - 0.5, 0), size=15.5, color=GOOD, anchor="r"),
                        self.text(loads["imposed"][language], (edges[2] + 0.12, bar_y - 0.5, 0), size=15.5, color=DATA, anchor="l"))
        area = self.trib["slab_area"]
        reaction = self._equation([(self.quantity(area, "m²"), ARCH, self.quantity(area, "m²")), ("×", MUTED, "×"), (self.quantity(sum(values), "kN/m²") + STAR, GOOD, self.quantity(sum(values), "kN/m²") + STAR), ("=", MUTED, "="),
                                   (lambda: self.quantity(q() * area, "kN", 0), DATA, self.quantity(self.trib["total_reaction"], "kN", 0))], (RIGHT, -1.5, 0), size=19.0)
        note = self._legend((RIGHT - 2.9, -2.1, 0), assumed=True)
        self.add(arrows)
        self._show("loads", (0.9, [self._leave(), FadeIn(axon), FadeIn(section), FadeIn(equation), FadeIn(sub), FadeIn(readout), FadeIn(bar, run_time=0.1), FadeIn(legend), FadeIn(reaction)]),
                   (3.0, [step.animate(rate_func=LINEAR).set_value(3.0)]), (0.6, [FadeIn(note)]))
        self.remove(arrows)
        self.stage = VGroup(axon, arrows, readout, section, equation, sub, bar, legend, reaction, note)

    def _loss_groups(self):
        """🗃️ Partition source objects omitted as analysis members without duplicating shared discipline identities."""
        registry={item["tag"]:item for group in ("elements","spaces") for item in self.model[group]}
        source=set().union(*(set(info["element_tags"]) for info in self.info.values()))
        kept={member["source_tag"] for member in self.analysis["members"]}
        omitted=source-kept
        categories=[
            (("other column bodies","weitere Stützenkörper"),{"IfcColumn"},("outside this first-floor member selection","außerhalb dieser Geschossauswahl")),
            (("other slabs and covers","weitere Platten und Deckel"),{"IfcSlab"},("not this analysis surface","nicht diese Analysefläche")),
            (("primary and auxiliary beams","Haupt- und Zusatzträger"),{"IfcBeam"},("physical beam objects are not transferred as line members","physische Trägerobjekte werden nicht als Stäbe übertragen")),
            (("walls and screen","Wände und Schirm"),{"IfcWall"},("occupied bodies omitted","feste Körper entfallen")),
            (("opening records","Öffnungsdatensätze"),{"IfcOpeningElement"},("void geometry remains; separate products do not","Aussparungsgeometrie bleibt; eigene Produkte entfallen")),
            (("windows, doors and glazing","Fenster, Türen und Verglasung"),{"IfcWindow","IfcDoor","IfcCurtainWall"},("frames and glass are omitted","Rahmen und Glas entfallen")),
            (("circulation and guards","Erschließung und Geländer"),{"IfcRamp","IfcRampFlight","IfcStair","IfcRailing"},("physical circulation bodies omitted","physische Erschließungskörper entfallen")),
            (("space objects","Raumobjekte"),{"IfcSpace"},("room semantics omitted","Raumsemantik entfällt")),
            (("furnishings","Ausstattung"),{"IfcFurnishingElement"},("fitted and movable objects omitted","feste und bewegliche Objekte entfallen")),
            (("foundations","Gründungen"),{"IfcFooting"},("replaced by declared support assumptions","durch erklärte Lagerannahmen ersetzt")),
            (("cellar and other assemblies","Keller- und weitere Baugruppen"),{"IfcBuildingElementProxy"},("includes the shared mechanical jib once","gemeinsamer Hebeausleger genau einmal")),
            (("service products","Haustechnikprodukte"),{"IfcBoiler","IfcSpaceHeater","IfcWasteTerminal","IfcPipeSegment","IfcCovering","IfcChimney"},("equipment, hollow pipes, casing and flue omitted","Geräte, hohle Rohre, Hüllen und Rauchzug entfallen"))]
        rows=[]
        for title,classes,note in categories:
            tags=sorted(tag for tag in omitted if registry[tag]["class"] in classes)
            if tags:
                rows.append((self.tr(*title)+f" ×{self.number(len(tags),0)}",tags,self.tr(*note),True,False))
        actual=[tag for _,tags,_,_,_ in rows for tag in tags]
        if len(actual)!=len(set(actual)) or set(actual)!=omitted:
            raise ValueError("Loss categories must partition every omitted source identity")
        return rows

    def _loss(self):
        """🗑️ Beat 16: what the analysis does not keep, one row per kind, each tied to the elements it came from; rows with an object count add up to the headline."""
        analysis, language = self.analysis, self.language()
        villa = self.villa_axon((-4.55, 0.3, 0), 0.12, colors="discipline", ground=False)
        groups=self._loss_groups()
        table, steps, links = VGroup(), [], VGroup()
        for index, (head, tags, sentence, counted, void) in enumerate(groups):
            y = 2.62 - index * 0.28
            row = VGroup(self.text(head, (-2.0, y, 0), size=16.2, color=WARN, anchor="l", width=2.45), self.text(sentence, (0.6, y, 0), size=16.2, color=INK, anchor="l", width=6.0))
            table.add(row)
            parts = [villa.parts[key] for tag in tags for key in self._keys(villa, tag)] if counted else []
            keys = [key for tag in tags for key in self._keys(villa, tag)]
            guides=[]
            if keys:
                target=np.mean([villa.parts[key].get_center() for key in keys],axis=0)
                line=Line((-2.08,y,0),target,stroke_width=.8,color=WARN,stroke_opacity=.5)
                dot=Dot(target,radius=.04,color=WARN)
                links.add(line,dot)
                guides.append(FadeIn(VGroup(line,dot),run_time=.2))
            steps.append(AnimationGroup(FadeIn(row,run_time=.2),*guides,*[part.animate.set_stroke(WARN,2,1).set_fill(WARN,.2) for part in parts]))
        dropped = len({tag for _, tags, _, counted, _ in groups if counted for tag in tags})
        kept = len({item["source_tag"] for item in analysis["members"]})
        if dropped + kept != len(set().union(*[set(info["element_guids"]) for info in self.info.values()])):
            raise ValueError("Dropped and kept objects must add up to the unique objects of the three models")
        if sum(len(set(tags)) for _, tags, _, counted, _ in groups if counted) != dropped:
            raise ValueError("Counted loss rows must not share objects")
        curve, surfaces = analysis["counts"]["curve_members"], len({item["source_guid"] for item in analysis["members"] if item["kind"] == "surface"})
        gone, stays = ValueTracker(0), ValueTracker(0)
        footer = VGroup(self._counter(gone, (-1.5, -0.95, 0), size=22.0, color=WARN, anchor="r"),
                        self.label("objects are not in the analysis: the counted rows above", "Objekte fehlen in der Analyse: die gezählten Zeilen oben", (-1.35, -0.95, 0), size=17.0, color=INK, anchor="l", width=6.0),
                        self._counter(stays, (-1.5, -1.4, 0), size=22.0, color=GOOD, anchor="r"),
                        self.label(f"objects are kept: {self.number(curve, 0)}{STAR} line members, {self.number(surfaces, 0)} surface member", f"Objekte bleiben: {self.number(curve, 0)}{STAR} Stabelemente, {self.number(surfaces, 0)} Flächenelement", (-1.35, -1.4, 0), size=17.0, color=INK, anchor="l", width=6.0),
                        self.label("together: the unique objects of the three models", "zusammen: die eindeutigen Objekte der drei Modelle", (-1.35, -1.85, 0), size=16.2, color=MUTED, anchor="l", width=6.0))
        legend = VGroup(self.label("Orange: not in the analysis", "Orange: nicht in der Analyse", (-4.55, -2.0, 0), size=16.2, color=WARN), self._legend((-1.35, -2.25, 0), piloti=True))
        self._show("loss", (0.8, [self._leave(), FadeIn(villa)]), (2.8, [LaggedStart(*steps, lag_ratio=0.35), FadeIn(legend)]),
                   (0.7, [FadeIn(footer), gone.animate(rate_func=LINEAR).set_value(dropped), stays.animate(rate_func=LINEAR).set_value(kept)]))
        self.stage = VGroup(villa, table, links, footer, legend)

    def _properties(self):
        """🏷️ Beat 17: piloti 19 as the analysis keeps it: what stays on the line member and what drops off."""
        analysis, language = self.analysis, self.language()
        piloti = next(item for item in self.geometry["pilotis"] if item["tag"] == _model.role("column_example"))
        member = next(item for item in analysis["members"] if item["kind"] == "curve" and item["source_tag"] == _model.role("column_example"))
        lost = {item["property"]: item for item in analysis["loss_groups"] if item["class"] == "IfcColumn"}
        scale, x, ground = 0.72, -5.2, -1.5

        def z(value):
            return ground + value * scale

        leg = [(x - 0.16, z(0), 0), (x + 0.16, z(0), 0), (x + 0.16, z(piloti["top"]), 0), (x - 0.16, z(piloti["top"]), 0)]
        slab = [(x - 0.8, z(piloti["top"]), 0), (x + 0.8, z(piloti["top"]), 0), (x + 0.8, z(piloti["top"] + analysis["slab"]["thickness"]), 0), (x - 0.8, z(piloti["top"] + analysis["slab"]["thickness"]), 0)]
        elevation = VGroup(polygon(leg, STRUCT, 0.35, STRUCT, 1.6), polygon(slab, STRUCT, 0.35, STRUCT, 1.6), segments([((x - 0.9, ground, 0), (x + 0.9, ground, 0))], INK, 2.0),
                           self.text("19 · IfcColumn", (x, z(piloti["top"]) + 0.65, 0), size=17.0, color=DATA),
                           self.dimension((x - 0.5, z(0), 0), (x - 0.5, z(piloti["top"]), 0), self.quantity(piloti["top"], "m") + STAR, offset=0.2, color=MUTED, size=15.5))
        line = VGroup(segments([((x, z(0), 0), (x, z(next(node["position"][2] for node in analysis["nodes"] if node["id"] == member["nodes"][1])), 0))], STRUCT, 5.0), Dot((x, z(next(node["position"][2] for node in analysis["nodes"] if node["id"] == member["nodes"][1])), 0), radius=0.07, color=DATA),
                      self.dimension((x + 0.16, z(0), 0), (x + 0.16, z(next(node["position"][2] for node in analysis["nodes"] if node["id"] == member["nodes"][1])), 0), self.tr("line ", "Stab ") + self.quantity(member["length"], "m", 3) + STAR, offset=-0.2, color=STRUCT, size=15.5))
        centre, radius = np.array([-2.5, 0.2, 0.0]), piloti["radius"] * 5.0
        ring = circle_points(centre, radius, 32)
        section = VGroup(polygon(ring, STRUCT, 0.4, STRUCT, 1.8), hatch(ring, STRUCT, 0.07, 45, 0.8, 0.85),
                         self.dimension(centre + (-radius, 0, 0), centre + (radius, 0, 0), "Ø " + self.quantity(piloti["radius"] * 2, "m") + STAR, offset=-(radius + 0.2), color=GOOD, size=16.2),
                         self.label("section", "Querschnitt", centre + (0, radius + 0.25, 0), size=15.5, color=MUTED))
        keep = [(self.tr("line member ", "Stabelement ") + self.quantity(member["length"], "m", 3) + STAR, GOOD), (self.tr("section Ø ", "Querschnitt Ø ") + self.quantity(member["diameter"], "m") + STAR, GOOD),
                (self.tr("reinforced concrete ", "Stahlbeton ") + self.quantity(analysis["loads"]["self_weight"]["density"], "kN/m³", 0) + STAR, GOOD),
                (self.tr("support: fixed", "Lagerung: eingespannt"), GOOD), ("GlobalId " + member["source_guid"], GOOD)]
        pset = next(name for name, props in _model.element(_model.role("column_example"))["psets"].items() if "FireRating" in props)
        drop = [(f"{pset}.FireRating = {self._flag(_model.role("column_example"), 'FireRating')}", WARN), (f"{pset}.IsExternal = {self._flag(_model.role("column_example"), 'IsExternal')}", WARN)]
        tags = lost["Pset_ColumnCommon.FireRating"]["source_tags"]
        rows, steps, strikes = VGroup(), [], VGroup()
        for index, (text, color) in enumerate(keep + drop):
            y = 2.35 - index * 0.4 - (0.3 if index >= len(keep) else 0)
            mono = text.startswith("GlobalId")
            shape = self.mono(text, (1.35, y, 0), size=16.2, color=color, anchor="l") if mono else self.text(text, (1.35, y, 0), size=17.0, color=color, anchor="l", width=5.0)
            if color == GOOD:
                dot = Circle(radius=0.07, stroke_color=color, stroke_width=1.6, fill_color=color, fill_opacity=0.9).move_to((1.1, y, 0))
            else:
                dot = segments([((1.03, y - 0.07, 0), (1.17, y + 0.07, 0)), ((1.03, y + 0.07, 0), (1.17, y - 0.07, 0))], WARN, 2.6)
            rows.add(VGroup(dot, shape))
            if color == WARN:
                strikes.add(shape)
        heads = VGroup(self.label("kept in the analysis", "bleibt in der Analyse", (1.1, 2.7, 0), size=16.2, color=GOOD, anchor="l"),
                       self.label("dropped", "entfällt", (1.1, 0.4, 0), size=16.2, color=WARN, anchor="l"))
        span = f"{tags[0]}–{tags[-1]} ×{self.number(len(tags), 0)}{STAR}"
        same = VGroup(self.label(f"the same on all pilotis: {span}", f"gleich bei allen Pilotis: {span}", (1.1, -0.85, 0), size=16.2, color=MUTED, anchor="l", width=5.6),
                      self._legend((1.1, -1.4, 0), assumed=True, piloti=True))
        self._show("properties", (0.9, [self._leave(), FadeIn(elevation), FadeIn(section)]),
                   (1.5, [LaggedStart(*[FadeIn(row) for row in rows[:len(keep)]], lag_ratio=0.3), FadeIn(line), FadeIn(heads[0])]),
                   (1.2, [LaggedStart(*[FadeIn(row) for row in rows[len(keep):]], lag_ratio=0.5), FadeIn(heads[1]), FadeIn(same)]))
        self.stage = VGroup(elevation, section, line, rows, heads, strikes, same)

    def _trace_footer(self, records, kinds):
        """📏 Reserve measured live-counter widths and center the localized loss-record row."""
        labels = [self.label("loss records in", "Verlustvermerke in", (0, 0, 0), size=17.0, color=INK, anchor="l"),
                  self.label("kinds: the only trace of what was dropped", "Arten: die einzige Spur des Weggelassenen", (0, 0, 0), size=17.0, color=INK, anchor="l", width=5.5)]
        totals = [len(self.analysis["losses"]), len(self.analysis["loss_groups"])]
        widths = [max(self.text(self.number(value, 0), (0, 0, 0), size=21.0, color=WARN).width for value in range(total + 1)) for total in totals]
        gap, footer = 0.15, VGroup()
        x = -(sum(widths) + sum(label.width for label in labels) + 3 * gap) / 2
        for tracker, label, width in zip((records, kinds), labels, widths):
            x += width
            counter = self._counter(tracker, (x, -2.1, 0), size=21.0, color=WARN)
            label.shift((x + gap - label.get_left()[0], -2.1 - label.get_center()[1], 0))
            footer.add(counter, label)
            x += 2 * gap + label.width
        return footer

    def _kind_chart(self, fill):
        """📊 One bar per kind of loss: its height is the number of loss records, its name stands under the bar; the 13 bars sum to the 82 records."""
        groups = self.analysis["loss_groups"]
        if {group["id"] for group in groups} != set(KIND_NAMES):
            raise ValueError("Every loss kind needs a name")
        base, pitch, unit = -0.5, 3.8 / len(groups), 0.014
        names, counts, centres = VGroup(), VGroup(), []
        for index, group in enumerate(groups):
            x = -2.05 + pitch * (index + 0.5)
            centres.append(x)
            name = self.text(self.tr(*KIND_NAMES.get(group["id"], (group["id"].split(":")[0], group["id"].split(":")[0]))), (0, 0, 0), size=15.0, color=INK)
            name.rotate(np.pi / 2)
            name.scale_to_fit_height(min(name.height, 0.78))
            names.add(name.move_to((x, base - 0.07 - name.height / 2, 0)))
            counts.add(self.live_label(lambda fill=fill, value=group["count"]: self.number(round(fill.get_value() * value), 0), (x, base + group["count"] * unit + 0.15, 0), size=15.0, color=WARN))

        def bars():
            return VGroup(*[Rectangle(width=0.17, height=max(group["count"] * unit * fill.get_value(), 1e-3), stroke_width=0, fill_color=WARN, fill_opacity=0.9).move_to(
                (x, base + group["count"] * unit * fill.get_value() / 2, 0)) for x, group in zip(centres, groups)])

        return VGroup(self.live(bars, key=lambda: round(fill.get_value(), 3)), names, counts)

    def _trace(self):
        """🔗 Beat 18: the analysis member carries the GlobalId of its source object; the way back is not automatic."""
        analysis, language = self.analysis, self.language()
        member = next(item for item in analysis["members"] if item["kind"] == "curve" and item["source_tag"] == _model.role("column_example"))
        struct = self.villa_axon((-3.9, 0.8, 0), 0.098, disciplines=("structure",), grid=True, highlight=(_model.role("column_example"),), ground=False)
        struct.grid.remove(struct.grid.submobjects[-1])
        result = self._villa_analysis((3.9, 0.8, 0), 0.098, highlight=(_model.role("column_example"),), tags=(_model.role("column_example"),))
        heads = VGroup(self.text(f"{self.info['structure']['name'][language]} · {self._revision(analysis['source_revision'])}", (-3.9, 2.75, 0), size=17.5, color=STRUCT, width=3.8),
                       self.label("Structural analysis model", "Tragwerksanalysemodell", (3.9, 2.75, 0), size=17.5, color=GOOD, width=3.8))
        c2 = Line(struct.anchors[_model.role("column_example")], result.anchors[_model.role("column_example")], stroke_width=1.6, color=DATA, stroke_opacity=0.9)
        one_way = VGroup(Arrow((-0.9, 1.9, 0), (0.9, 1.9, 0), buff=0.0, color=MUTED, stroke_width=3.0, max_tip_length_to_length_ratio=0.2),
                         Arrow((0.9, 1.35, 0), (-0.9, 1.35, 0), buff=0.0, color=WARN, stroke_width=2.0, max_tip_length_to_length_ratio=0.2, stroke_opacity=0.5),
                         segments([((-0.2, 1.2, 0), (0.2, 1.5, 0)), ((-0.2, 1.5, 0), (0.2, 1.2, 0))], WARN, 3.0),
                         self.label("translation", "Übersetzung", (0, 2.2, 0), size=16.2, color=MUTED),
                         self.label("no automatic way back", "kein automatischer Rückweg", (0, 0.95, 0), size=15.5, color=WARN, width=2.4))
        chips = VGroup(self.guid_chip(_model.role("column_example"), (-3.9, -1.5, 0), color=DATA),
                       self.label("source GlobalId of member " + member["id"], "Quell-GlobalId des Elements " + member["id"], (3.9, -0.98, 0), size=15.5, color=MUTED, width=3.9),
                       self.guid_chip(_model.role("column_example"), (3.9, -1.5, 0), color=DATA, label=False),
                       self.text("=", (0, -1.5, 0), size=30.0, color=GOOD))
        if member["source_guid"] != _model.global_id(_model.role("column_example")):
            raise ValueError("The member keeps the GlobalId of its source piloti")
        records, kinds, fill = ValueTracker(0), ValueTracker(0), ValueTracker(0)
        footer, squares = self._trace_footer(records, kinds), self._kind_chart(fill)
        self._show("trace", (0.9, [self._leave(), FadeIn(struct), FadeIn(result), FadeIn(heads)]),
                   (1.2, [FadeIn(c2), FadeIn(chips)]), (1.0, [FadeIn(one_way)]),
                   (1.2, [FadeIn(footer), records.animate(rate_func=LINEAR).set_value(len(analysis["losses"])), kinds.animate(rate_func=LINEAR).set_value(len(analysis["loss_groups"])),
                          FadeIn(squares), fill.animate(rate_func=LINEAR).set_value(1.0)]))
        self.stage = VGroup(struct, result, heads, c2, one_way, chips, footer, squares)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               