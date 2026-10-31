"""🗼 From one capsule to the tower: core, planes, capsules, capital and podium as five coloured Grasshopper groups beside the Nakagin axonometric."""

import dataclasses
import importlib
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
_canvas = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🦗️canvas.🐍️")
_gh = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
_nak = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_nakd = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, DATA, GOOD, WARN, ARCH = _visuals.INK, _visuals.MUTED, _visuals.DATA, _visuals.GOOD, _visuals.WARN, _visuals.ARCH
GROUP_KEYS = ("core", "planes", "capsules", "capital", "podium")
COLORS = dict(zip(GROUP_KEYS, _canvas.GROUPS))
NAMES = {"core": ("Core", "Kern"), "planes": ("Planes", "Ebenen"), "capsules": ("Capsules", "Kapseln"), "capital": ("Capital", "Kopf"), "podium": ("Podium", "Sockel")}
LC, LW, LH = (-2.35, -0.03), 9.3, 5.05
VX, VY, VW, VH = 4.7, -0.03, 4.4, 5.05
TXT = "#1B1B1B"
S_T = 0.115
KBASE = np.array([VX, VY - 1.9, 0.0])
_A, _B = _nak.towers()
_PER = _nak.totals()["per_tower"]
_TOTAL = _nak.totals()["count"]


def _m(value, language):
    """📏 Metres with two decimals and the decimal sign of the language."""
    rendered = f"{value:.2f}"
    return rendered.replace(".", ",") if language == "de" else rendered



GERMAN_LABELS = {
    "Core side": "Kernseite", "Storey height": "Geschosshöhe", "Capsule storeys": "Kapselebenen",
    "Capsule datum": "Kapsel-Bezugshöhe", "Podium roof datum": "Bezugshöhe Sockeldach",
    "Core centre X": "Kernzentrum X", "Core centre Y": "Kernzentrum Y",
    "Absent authored attachment slots": "Ausgelassene Befestigungsplätze", "Capsule per slot": "Kapsel je Platz",
    "Capital overhang": "Überstand Kopf", "Capital height": "Höhe Kopf", "Podium length": "Länge Sockel", "Podium width": "Breite Sockel",
    "Door-port offsets": "Tür-Port-Versätze", "Door-port X axes": "Tür-Port-X-Achsen", "Door-port Y axes": "Tür-Port-Y-Achsen",
    "Local handed door ports": "Lokale gerichtete Tür-Ports", "Crown offset": "Kronenversatz",
    "Highest capsule crown offset": "Versatz der höchsten Kapselkrone", "Roof clearance": "Dachfreiraum",
}


def _display_definition(definition, language):
    """🈯 Localize authored captions while preserving the native Grasshopper component names and engine inputs."""
    lengths = {"side", "sh", "z0", "cx", "cy", "ow", "ch", "pw", "pd", "over", "tailp", "portp"}
    objects = {}
    for key, obj in definition.objects.items():
        settings = dict(obj.settings)
        if "label" in settings:
            label = settings["label"]
            settings["label"] = (GERMAN_LABELS[label] if language == "de" else label) + (" [m]" if key in lengths else "")
        objects[key] = dataclasses.replace(obj, settings=settings)
    return dataclasses.replace(definition, objects=objects)


def _rejected_capsules(solution, masks):
    """🧾 Pair rejected candidate planes with their actual unmasked evaluated Breps."""
    planes=solution.tree("slots","Geometry")
    geometry=solution.tree("ori","Geometry")
    return tuple((path,index,plane,geometry.branch(path)[index]) for path in planes.paths() for index,plane in enumerate(planes.branch(path)) if index in masks.branch(path))


def _rejected_ghosts(rejected, types, reference, to_plane):
    """👻 Draw the evaluated type and physical family at each rejected body plane."""
    return _engine.VGroup(*[_nakd._capsule_lite(_nakd._cap(types[geometry.tag["type"]]),to_plane(plane),reference.view,WARN,layout=geometry.tag["family"]).shift(reference.delta).set_opacity(.35) for _,_,plane,geometry in rejected])


class Beat3_Tower(_visuals.ClipScene):
    """🗼 Animate the Nakagin tower as one definition: core, planes, capsules, capital and podium with explicit English and German narration."""

    TITLE = {"en": "From One Capsule to the Tower", "de": "Von der Kapsel zum Turm"}
    NARRATION = [
        ["hook", f"The Nakagin Capsule Tower has {_TOTAL} capsules. How do they become two towers?",
         f"Der Nakagin Capsule Tower hat {_TOTAL} Kapseln. Wie werden sie zu zwei Türmen?"],
        ["parts", "The tower splits into five parts: core, planes, capsules, capital and podium. Each part gets its own coloured group on the canvas.",
         "Der Turm hat fünf Teile: Kern, Ebenen, Kapseln, Kopf und Sockel. Jeder bekommt eine eigene farbige Gruppe."],
        ["core", "Start with the square core envelope. Its height combines selected capsule crowns with the roof clearance of this tower.",
         "Am Anfang steht die quadratische Kernhülle. Ihre Höhe verbindet gewählte Kapselkronen mit dem Dachfreiraum dieses Turms."],
        ["shaft", "The envelope sets out the shaft. Its detailed assembly contains slabs, stairs, landings and lift doors on every storey.",
         "Die Hülle legt den Schacht fest. Seine detaillierte Baugruppe enthält Decken, Treppen, Podeste und Aufzugtüren auf jedem Geschoss."],
        ["series", f"A Series starts at the provisional {_m(_nak.podium_height(), 'en')} metre podium roof, with the sourced {_m(_A.storey_height, 'en')} metre pitch. Authored phase offsets place the capsules.",
         f"Eine Series beginnt am vorläufigen Sockeldach bei {_m(_nak.podium_height(), 'de')} m mit belegten {_m(_A.storey_height, 'de')} m Schritt. Verfasste Phasenversätze platzieren die Kapseln."],
        ["facades", "Authored origins and handed axes define the door-port frames. Construct Plane assembles each one without an inferred rotation.",
         "Verfasste Ursprünge und Achsen bestimmen die Tür-Port-Ebenen. Construct Plane baut jede ohne angenommene Drehung."],
        ["axes", "Every plane carries its axes: X runs along the facade, Y points outward, Z points up.",
         "Jede Ebene trägt ihre Achsen: X läuft entlang der Fassade, Y zeigt nach außen, Z zeigt nach oben."],
        ["items", "Each authored facade group has indexed candidate ports. The authored floor masks select the occupied positions.",
         "Jede verfasste Fassadengruppe hat indizierte Kandidaten-Ports. Die verfassten Geschossmasken wählen die belegten Positionen."],
        ["orient", "Orient maps the local door port to the reconstructed world door port. Its result is the capsule body plane, including the handed layout.",
         "Orient bildet den lokalen Tür-Port auf den rekonstruierten Welt-Port ab. Das Ergebnis ist die Körper-Ebene der Kapsel, gerichtet angeordnet."],
        ["types", "A type pattern picks the capsule type for every plane. List Item reads the pattern by index; only the list differs.",
         "Ein Typmuster wählt für jede Ebene den Kapseltyp. List Item liest das Muster per Index; nur die Liste ändert sich."],
        ["count", "The slider reveals one reconstructed capsule level after another. The total is the sum of the occupied positions on the selected levels.",
         "Der Slider zeigt eine rekonstruierte Kapselebene nach der anderen. Die Summe zählt die belegten Positionen auf den gewählten Ebenen."],
        ["nowire", "The wires stay unchanged. Floor masks, capsule crowns and the tower-specific roof clearance determine the resulting height.",
         "Die Verbindungen bleiben gleich. Geschossmasken, Kapselkronen und der Dachfreiraum jedes Turms bestimmen die resultierende Höhe."],
        ["mistake", "Without the floor masks, the regular candidate grid includes unoccupied positions. Its repeated pattern is not the documented building.",
         "Ohne die Geschossmasken enthält das regelmäßige Raster unbelegte Positionen. Das Muster entspricht nicht dem dokumentierten Gebäude."],
        ["fix", f"Cull Index removes the unoccupied positions on every level. The documented result is {_PER['A']['count']} capsules in A and {_PER['B']['count']} in B.",
         f"Cull Index entfernt die unbelegten Positionen auf jeder Ebene. Das belegte Ergebnis sind {_PER['A']['count']} Kapseln in A und {_PER['B']['count']} in B."],
        ["capital", f"The machinery head is a separate part. A has a sourced height of {_m(_A.capital_height, 'en')} metres; B uses a provisional derived {_m(_B.capital_height, 'en')} metres.",
         f"Der Maschinenkopf ist ein eigenes Bauteil. Für A sind {_m(_A.capital_height, 'de')} Meter belegt; B nutzt vorläufig abgeleitete {_m(_B.capital_height, 'de')} Meter."],
        ["ontop", f"The head starts at the core roof: {_m(_A.height, 'en')} m in A, a provisional {_m(_B.height, 'en')} m in B; each tower keeps its own clearance.",
         f"Der Kopf beginnt am Kerndach: {_m(_A.height, 'de')} m in A, vorläufig {_m(_B.height, 'de')} m in B; jeder Turm behält seinen Dachfreiraum."],
        ["podium", "The footprint and height define the podium envelope. The detailed model adds its slabs, columns, glazing, entrance and parapets.",
         "Grundriss und Höhe bestimmen die Sockelhülle. Das detaillierte Modell ergänzt Decken, Stützen, Verglasung, Eingang und Brüstungen."],
        ["why", "The envelope is the first construction step. Named assemblies carry the circulation, services and façade details throughout the building.",
         "Die Hülle ist der erste Konstruktionsschritt. Benannte Baugruppen tragen Erschließung, Versorgung und Fassadendetails durchs Gebäude."],
        ["towers", f"The same definition reads the authored floor masks for both towers: {_PER['A']['count']} capsules in A, {_PER['B']['count']} in B.",
         f"Dieselbe Definition liest die verfassten Geschossmasken beider Türme: {_PER['A']['count']} Kapseln in A, {_PER['B']['count']} in B."],
        ["total", f"Together they give the historical count, {_TOTAL} capsules, read from the definition and not typed in.",
         f"Zusammen ergeben sie die historische Zahl von {_TOTAL} Kapseln, aus der Definition gelesen und nicht eingetippt."],
        ["swap", "Metabolism proposes replaceable capsules. Here the switch changes the room program from type B to type C; the porthole stays.",
         "Der Metabolismus schlägt austauschbare Kapseln vor. Hier wechselt der Schalter das Raumprogramm von Typ B zu Typ C; das Bullauge bleibt."],
        ["recap", "Recap: core, planes, capsules, capital, podium. This is the teaching model of the Nakagin Capsule Tower.",
         "Zusammenfassung: Kern, Ebenen, Kapseln, Kopf, Sockel. So sieht das Lehrmodell des Nakagin Capsule Tower aus."],
    ]

    def construct(self):
        self.present("tower")

    def _solve(self, over=None):
        """⚙️ Solve the tower canvas once per override set."""
        key = repr(sorted((over or {}).items()))
        if key not in self._cache:
            self._cache[key] = _gh.evaluate(self._defn, dict(over or {}))
        return self._cache[key]

    def _pt(self, drawing, *xyz):
        """📍 Screen position of a model point in a pinned drawing."""
        return drawing.at(*xyz) + drawing.delta

    def _pin(self, drawing, world, screen):
        """📌 Shift a drawing so a world point lands on a fixed screen point."""
        drawing.delta = np.asarray(screen) - drawing.at(*world)
        drawing.shift(drawing.delta)
        return drawing

    def _tower(self, k, *, types=False, capital=True, podium=False, scale=S_T, base=KBASE, tint=None, lit=None, storeys=None):
        """🗼 Tower B up to k capsule storeys, pinned at its foot."""
        k = int(k)
        td = self.tower_axon(at=(VX, VY), scale=scale, towers=(1,), storeys=storeys or k + self._pods, where=lambda s, k=k: s.storey < k,
                             podium=podium, capital=capital, types=types, tint=tint, lit=lit)
        return self._pin(td, (self.B.center[0], 0.0, 0.0), base)

    def _plane(self, plane, shift=(0.0, 0.0, 0.0)):
        """🧭 An engine plane as drawable axes."""
        shift = np.asarray(shift)
        return SimpleNamespace(origin=np.array(plane.origin.t) + shift, x=np.array(plane.x.t), y=np.array(plane.y.t), z=np.array(plane.z.t))

    def _axes(self, td, plane, *, length=2.0, labels=False, scale=S_T):
        """➡️ Plane axes aligned with a pinned tower drawing."""
        return self.plane_axes(plane, at=self._pt(td, *plane.origin), scale=scale, length=length, labels=labels)

    def _say(self, string, at, size=22, color=TXT, anchor="c"):
        """🏷️ Dark label on the light canvas."""
        return self.text(string, at, size=size, color=color, anchor=anchor)

    def _fit(self, drawing, viewport, margin=0.3):
        """🧲 Scale a static drawing into the viewport."""
        return viewport.fit(drawing, margin)

    def _type_legend(self, at):
        """🎨 Colour chips of the four capsule types."""
        chips = [_engine.VGroup(_engine.Square(0.26, fill_color=_nakd.type_color(t.key), fill_opacity=1, stroke_color=TXT, stroke_width=1.5),
                                self._say(t.key, (0, 0), 20, anchor="l")).arrange(_engine.RIGHT, buff=0.12) for t in _nak.capsule_types()]
        return _engine.VGroup(*chips).arrange(_engine.RIGHT, buff=0.45).move_to(np.array([at[0], at[1], 0.0]))

    def present_tower(self):
        """🎞️ Pair every authored floor-mask computation with the architecture it places."""
        e, g = _engine, self.tr
        self._defn, self._cat, self._cache = _gh.canvas("tower"), _gh.catalogue(), {}
        self._view_defn = _display_definition(self._defn, self.language())
        self.A, self.B = _nak.towers()
        self._slots = _nak.slots()
        self._pods = self.B.first_capsule_storey
        types = {t.key: t for t in _nak.capsule_types()}
        solution = self._solve()
        lengths = solution.items("st", "Length")
        high = len(lengths)
        total = len(self._slots)
        viewport = self.gh_viewport(at=(VX, VY), width=VW, height=VH, label="Perspective")
        self.add(viewport, _fonts.section_heading(self.TITLE[self.language()]))
        current = {}

        def canvas(view, overrides=None):
            """🖇 Show the actual definition and solution for the current narrated operation."""
            return self.gh_canvas(self._view_defn, self._solve(overrides), view=view, at=(*LC, 0), frame=(LW, LH), max_lines=7)

        def fit(drawing):
            """🖼 Keep the active architectural object inside its viewport."""
            return self._fit(drawing, viewport, .45)

        def stage(key, network=None, drawing=None, note=None, seconds=2, via=None, done=None):
            """🎯 Replace the preceding computation and its architectural result together; `via` swaps a slot by morph animations instead of a fade."""
            moves, via = [], via or {}
            for name, value in (("network", network), ("drawing", drawing), ("note", note)):
                previous = current.get(name)
                current[name] = value
                if name in via:
                    moves += via[name]
                    continue
                if value is previous:
                    continue
                if name == "note" and getattr(previous, "plain", False) and getattr(value, "plain", False):
                    moves.append(_morph.retext(previous, value, run_time=1.2))
                    continue
                if previous is not None:
                    moves.append(e.FadeOut(previous))
                if value is not None and value not in self.mobjects:
                    moves.append(e.FadeIn(value))
            self.step(key, *moves, seconds=seconds)
            if done:
                done()

        def shapes(drawing, pick=None):
            """🫥 Outline copies of the capsules of a drawing, not yet in the scene."""
            return [_morph.silhouette(group) for key, group in sorted(drawing.capsules.items()) if pick is None or pick(key)]

        def sils(drawing, pick=None):
            """🫥 Outline copies of the capsules of a drawing, laid over it as morph sources."""
            found = shapes(drawing, pick)
            self.add(*found)
            return found

        def adopt(*groups):
            """🧺 Make a group of individually placed mobjects one scene object again."""
            for group in groups:
                self.remove(*group.get_family()[1:])
                self.add(group)

        def settle(*loose):
            """🧹 Fade the flat outlines of a finished morph away, so the detailed drawing below shows."""
            self.play(e.FadeOut(e.VGroup(*loose)), run_time=.4)
            self.remove(*loose)

        def label(text):
            """🪧 Place a concise readout below the active graph."""
            readout = self._say(text, (-2.35, -2.3), 22, TXT)
            readout.plain = True
            return readout

        full = fit(self.tower_axon())
        self.add(full)
        current["drawing"] = full
        count_note = label(g(f"{total} documented capsules", f"{total} belegte Kapseln"))
        count_token = _morph.token(str(total), kind="number", color=DATA, size=1.5).move_to(np.array([.9, -2.3, 0.0]))
        count_group = e.VGroup(count_note, count_token)
        stage("hook", canvas("orient"), full, count_group, via={"note": [e.FadeIn(count_note), _morph.gather(sils(full), count_token, run_time=1.6)]}, done=lambda: adopt(count_group))
        badges = {key: self.badge(g(*NAMES[key]), (-5.9 + i * 1.7, -2.3), color=COLORS[key], size=18) for i, key in enumerate(GROUP_KEYS)}
        cards = e.VGroup(*badges.values())
        part_sources = {"core": [_morph.silhouette(part) for part in full.cores.values()], "capsules": sils(full),
                        "capital": [_morph.silhouette(part) for part in full.capitals.values()], "podium": [_morph.silhouette(full.podium)]}
        self.add(*[shape for shapes in part_sources.values() for shape in shapes if shape not in self.mobjects])
        gathers = [_morph.gather(shapes, badges[key], run_time=1.4) for key, shapes in part_sources.items()]
        stage("parts", canvas("core"), full, cards, via={"note": [e.FadeOut(current["note"]), *gathers, _morph.morph(_morph.token("", kind="plane", color=COLORS["planes"], size=1.4).move_to(full.get_center()), badges["planes"], run_time=1.4)]}, done=lambda: adopt(cards))

        side = e.ValueTracker(self.B.core_side)
        pitch = e.ValueTracker(self.B.storey_height)
        core_key = lambda: (round(side.get_value(), 2), round(pitch.get_value(), 2))
        def core_study():
            """🧱 Demonstrate the live dimensions on a short, legible core using the engine height result."""
            side_value, pitch_value = core_key()
            height = self._solve({"side": side_value, "sh": pitch_value, "n": 1}).value("tot")
            shaped = dataclasses.replace(self.B, core_side=side_value, storey_height=pitch_value, storeys=self._pods + 1, height=height)
            return fit(self.core_axon(shaped))

        core_live = self.live(core_study, key=core_key)
        core_net = self.live(lambda: canvas("core", {"side": core_key()[0], "sh": core_key()[1], "n": 1}), key=core_key)
        core_source = _morph.silhouette(list(full.cores.values())[-1])
        self.add(core_source)
        stage("core", core_net, core_live, label(g("side × side × storeys × height", "Seite × Seite × Geschosse × Höhe")),
              via={"drawing": [e.FadeOut(full), _morph.morph(core_source, core_live, run_time=1.8)]})
        self.play(side.animate.set_value(self.B.core_side * 1.12), pitch.animate.set_value(self.B.storey_height * 1.08), run_time=1.5)
        self.play(side.animate.set_value(self.B.core_side), pitch.animate.set_value(self.B.storey_height), run_time=1.5)
        shaft = fit(self.core_axon(self.B))
        stage("shaft", canvas("core"), shaft, label(g("Walls · landings · lift · services", "Wände · Podeste · Aufzug · Versorgung")),
              via={"drawing": [_morph.morph(core_live, shaft, run_time=1.8)]})

        planes = solution.tree("mv", "Geometry")
        target_planes = solution.tree("slots", "Geometry")
        targets = [plane for path in target_planes.paths() for plane in target_planes.branch(path)]
        reference = self._tower(high, capital=False)
        axis_rows = e.VGroup(*[self._axes(reference, self._plane(planes.branch(path)[0]), length=1.1, labels=False, scale=S_T) for path in planes.paths()])
        levels = fit(e.VGroup(reference, axis_rows))
        levels_core = list(levels[0].cores.values())[-1].copy()
        stage("series", canvas("levels"), levels, label(g("One Series elevation per occupied level", "Eine Series-Höhe je belegter Ebene")),
              via={"drawing": [_morph.morph(shaft, levels_core, run_time=1.8), e.FadeIn(levels)]}, done=lambda: settle(levels_core))
        reference = self._tower(high, capital=False)
        first = planes.branch(planes.paths()[0])
        frame_axes = e.VGroup(*[self._axes(reference, self._plane(plane), length=1.0, labels=False, scale=S_T) for plane in first])
        frames = fit(e.VGroup(reference, frame_axes))
        level_source = levels[1][0].copy()
        self.add(level_source)
        stage("facades", canvas("facades"), frames, label(g("Authored origin and handed port-frame axes", "Verfasster Ursprung und gerichtete Port-Achsen")),
              via={"drawing": [e.FadeOut(levels), e.FadeIn(frames[0]), _morph.split(level_source, list(frames[1]), run_time=1.4)]}, done=lambda: adopt(frames))
        one_plane = self._plane(targets[0])
        axes = self.plane_axes(one_plane, at=(VX, VY, 0), scale=.6, length=2)
        frame_source = frames[1][0].copy()
        self.add(frame_source)
        stage("axes", canvas("centre"), axes, label("X · Y · Z"),
              via={"drawing": [e.FadeOut(frames), _morph.morph(frame_source, axes, run_time=1.4)]})
        candidate_count = max(len(planes.branch(path)) for path in planes.paths())
        one_storey = fit(self._tower(1, capital=False))
        port_tokens = [_morph.token("", kind="plane", color=COLORS["planes"], size=.7).move_to(group.get_center()) for _, group in sorted(one_storey.capsules.items())]
        port_source = axes.copy()
        self.add(port_source)
        stage("items", canvas("stagger"), e.VGroup(one_storey, *port_tokens), label(g(f"Up to {candidate_count} candidate ports per level; authored masks select occupancy", f"Bis zu {candidate_count} Ports je Ebene; verfasste Masken wählen die Belegung")),
              via={"drawing": [e.FadeOut(axes), e.FadeIn(one_storey), _morph.split(port_source, port_tokens, run_time=1.4)]}, done=lambda: adopt(current["drawing"]))
        ordered = fit(self._tower(high, capital=False))
        first_storey = shapes(ordered, lambda key: key[2] == 0)
        stage("orient", canvas("orient"), ordered, label(g("Local door port → reconstructed world door port → body plane", "Lokaler Tür-Port → rekonstruierter Welt-Port → Körper-Ebene")),
              via={"drawing": [e.FadeOut(current["drawing"]), e.FadeIn(ordered), _morph.scatter(port_tokens, first_storey, run_time=1.4)]}, done=lambda: settle(*first_storey))
        typed = fit(self._tower(high, types=True, capital=False))
        neutral, coloured = sils(ordered), shapes(typed)
        stage("types", canvas("orient"), typed, self._type_legend((-2.35, -2.3)),
              via={"drawing": [e.FadeOut(ordered), e.FadeIn(typed), _morph.scatter(neutral, coloured, lag=.01, path_arc=0, run_time=1.4)]}, done=lambda: settle(*coloured))

        count = e.ValueTracker(1)
        shown = lambda: max(1, min(high, int(round(count.get_value()))))
        growing = self.live(lambda: fit(self._tower(shown(), types=True)), key=shown)
        growing_net = self.live(lambda: canvas("storeys", {"n": shown()}), key=shown)
        growing_sum = self.live_label(lambda: f"Σ ({', '.join(map(str, lengths[:shown()]))}) = {sum(lengths[:shown()])}", (-2.35, -2.3), size=22, color=TXT)
        growing_token = self.live_label(lambda: str(sum(lengths[:shown()])), (1.8, -2.3), size=30, color=TXT)
        growing_group = e.VGroup(growing_sum, growing_token)
        stage("count", growing_net, growing, growing_group,
              via={"note": [e.FadeOut(current["note"]), e.FadeIn(growing_sum), _morph.gather(sils(typed), growing_token, run_time=1.6)]}, done=lambda: adopt(growing_group))
        self.play(count.animate.set_value(high), run_time=4)
        half = max(1, high // 2)
        half_tag = _morph.token(str(half), kind="number", color=COLORS["capsules"], size=1.1).move_to(growing.get_top() + np.array([.9, .1, 0.0]))
        half_end = _morph.token(str(half), kind="number", color=COLORS["capsules"], size=1.1).move_to(np.array([.9, -1.85, 0.0]))
        self.step("nowire", count.animate.set_value(half), _morph.morph(half_tag, half_end, path_arc=.5, run_time=1.6), seconds=3)
        self.play(e.FadeOut(half_end), count.animate.set_value(high), run_time=2)
        self.remove(half_end)

        masks = solution.tree("cullp")
        unmasked = {"cullp": {"tree": {_gh.format_path(path): [] for path in masks.paths()}}}
        wrong = self._solve(unmasked)
        wrong_count = sum(wrong.items("st", "Length"))
        rejected = _rejected_capsules(wrong, masks)
        rejected_index = [index for _, index, _, _ in rejected]
        ghost_reference = self._tower(high, capital=False)
        ghosts = _rejected_ghosts(rejected, types, ghost_reference, self._plane)
        flawed = fit(e.VGroup(ghost_reference, ghosts))
        phantoms = [ghost.copy() for ghost in flawed[1]]
        flawed_all = e.VGroup(flawed[0], *phantoms)
        candidate_tokens = [_morph.token(str(index), kind="item", color=WARN).move_to(np.array([.6 + .6 * i, -2.3, 0.0])) for i, index in enumerate(rejected_index)]
        stage("mistake", canvas("cull", unmasked), flawed_all, label(g(f"Unmasked candidates: {wrong_count}", f"Unmaskierte Kandidaten: {wrong_count}")),
              via={"drawing": [e.FadeOut(growing), e.FadeIn(flawed[0]), _morph.scatter(candidate_tokens, phantoms, lag=.2, path_arc=.4, run_time=1.4)]}, done=lambda: adopt(flawed_all))
        fixed = fit(self._tower(high))
        fix_note = label(g(f"Σ documented B levels = {sum(lengths)}", f"Σ belegte B-Ebenen = {sum(lengths)}"))
        cull_tokens = [_morph.token(str(index), kind="item", color=WARN).move_to(np.array([.25 + .6 * i, -2.3, 0.0])) for i, index in enumerate(rejected_index)]
        fix_group = e.VGroup(fix_note, *cull_tokens)
        stage("fix", canvas("cull"), fixed, fix_group,
              via={"drawing": [e.FadeOut(flawed_all), e.FadeIn(fixed)],
                   "note": [e.FadeOut(current["note"]), e.FadeIn(fix_note), _morph.scatter([_morph.silhouette(phantom) for phantom in phantoms], cull_tokens, lag=.2, path_arc=.4, run_time=1.4)]},
              done=lambda: adopt(fix_group))

        overhang = e.ValueTracker(self.B.capital_overhang)
        cap_height = e.ValueTracker(self.B.capital_height)
        cap_key = lambda: (round(overhang.get_value(), 2), round(cap_height.get_value(), 2))
        cap_live = self.live(lambda: fit(self.capital_axon(dataclasses.replace(self.B, capital_overhang=cap_key()[0], capital_height=cap_key()[1]))), key=cap_key)
        cap_net = self.live(lambda: canvas("capital", {"ow": cap_key()[0], "ch": cap_key()[1]}), key=cap_key)
        cap_source = _morph.silhouette(list(fixed.capitals.values())[-1])
        cap_tags = []
        for slider, value, spot in (("ch", self.B.capital_height, VY + 2.0), ("ow", self.B.capital_overhang, VY + 1.5)):
            text = f"{self.number(value)} m"
            cap_tags.append((_morph.token(text, kind="number", color=COLORS["capital"], size=1.1).move_to(cap_net.obj(slider).get_center()),
                             _morph.token(text, kind="number", color=COLORS["capital"], size=1.1).move_to(np.array([VX + 1.1, spot, 0.0]))))
        stage("capital", cap_net, cap_live, label(g("Capital: independent height and overhang", "Kopf: unabhängige Höhe und Überstand")),
              via={"drawing": [e.FadeOut(fixed), _morph.morph(cap_source, cap_live, run_time=1.6), *[_morph.morph(start, end, path_arc=.5, run_time=1.6) for start, end in cap_tags]]},
              done=lambda: (self.play(e.FadeOut(e.VGroup(*[end for _, end in cap_tags])), run_time=.4), self.remove(*[end for _, end in cap_tags])))
        self.play(overhang.animate.set_value(self.B.capital_overhang + .25), cap_height.animate.set_value(self.B.capital_height * 1.15), run_time=1.5)
        count.set_value(1)
        rising = self.live(lambda: fit(self._tower(shown())), key=shown)
        rising_net = self.live(lambda: canvas("height", {"n": shown()}), key=shown)
        storey_tag = _morph.token(str(shown()), kind="number", color=COLORS["capsules"], size=1.1).move_to(rising_net.obj("n").get_center())
        storey_end = _morph.token(str(shown()), kind="number", color=COLORS["capsules"], size=1.1).move_to(rising.get_top() + np.array([.9, .1, 0.0]))
        rising_head = list(rising.capitals.values())[-1].copy()
        stage("ontop", rising_net, rising, label(g("The head follows the selected tower height", "Der Kopf folgt der gewählten Turmhöhe")),
              via={"drawing": [_morph.morph(cap_live, rising_head, run_time=1.6), e.FadeIn(rising), _morph.morph(storey_tag, storey_end, path_arc=.5, run_time=1.6)]},
              done=lambda: (settle(rising_head), self.play(e.FadeOut(storey_end), run_time=.4), self.remove(storey_end)))
        self.play(count.animate.set_value(high), run_time=3)
        podium = fit(self.podium_axon())
        podium_net = canvas("podium")
        footprint = podium_net.obj("rect").copy()
        stage("podium", podium_net, podium, label(g("Slabs · columns · glazing · entrance", "Decken · Stützen · Verglasung · Eingang")),
              via={"drawing": [e.FadeOut(rising), _morph.morph(footprint, podium, run_time=1.6)]})
        whole = fit(self.tower_axon(types=False))
        whole_base = whole.podium.copy()
        stage("why", canvas("orient"), whole, label(g("Shared named assemblies carry detail through every view", "Gemeinsame benannte Baugruppen tragen Details in jede Ansicht")),
              via={"drawing": [_morph.morph(podium, whole_base, run_time=1.6), e.FadeIn(whole)]}, done=lambda: settle(whole_base))

        per_tower = _nak.totals()["per_tower"]
        tinted = fit(self.tower_axon(types=False, tint={(0, 0): ARCH, (0, 1): WARN}))
        number_a = _morph.token(str(per_tower["A"]["count"]), kind="number", color=ARCH, size=1.3).move_to(np.array([.7, -2.3, 0.0]))
        number_b = _morph.token(str(per_tower["B"]["count"]), kind="number", color=WARN, size=1.3).move_to(np.array([1.7, -2.3, 0.0]))
        towers_note = label(g("A: 76 capsules · B: 64 capsules", "A: 76 Kapseln · B: 64 Kapseln"))
        towers_group = e.VGroup(towers_note, number_a, number_b)
        stage("towers", canvas("storeys"), tinted, towers_group,
              via={"drawing": [e.FadeOut(whole), e.FadeIn(tinted)],
                   "note": [e.FadeOut(current["note"]), e.FadeIn(towers_note), _morph.gather(sils(tinted, lambda k: k[1] == 0), number_a, run_time=1.6), _morph.gather(sils(tinted, lambda k: k[1] == 1), number_b, run_time=1.6)]},
              done=lambda: adopt(towers_group))
        plain = fit(self.tower_axon(types=False))
        lit_a, lit_b = shapes(plain, lambda k: k[1] == 0), shapes(plain, lambda k: k[1] == 1)
        total_note = label(f"{per_tower['A']['count']} + {per_tower['B']['count']} = {total}")
        stage("total", canvas("cull"), plain, total_note,
              via={"drawing": [e.FadeOut(tinted), e.FadeIn(plain)],
                   "note": [e.FadeOut(towers_group), e.FadeIn(total_note), _morph.split(number_a, lit_a, run_time=1.6), _morph.split(number_b, lit_b, run_time=1.6)]},
              done=lambda: settle(*lit_a, *lit_b))
        old_key, new_key = "B", "C"
        programs = self.capsule_axon(types[new_key], at=(VX, VY, 0), scale=.6, cutaway=True, interior="study")
        kind_of = {(s.building, s.tower, s.storey, s.facade, s.item): s.type for s in self._slots}
        pick = lambda letter: next(key for key in sorted(plain.capsules) if kind_of[key] == letter)
        old_source = _morph.silhouette(plain.capsules[pick(old_key)])
        stage("swap", canvas("orient"), programs, label(g("Program B → C; the historical 1.30 m porthole stays", "Programm B → C; das historische Bullauge mit 1,30 m bleibt")),
              via={"drawing": [e.FadeOut(plain), _morph.morph(old_source, programs, run_time=1.8)]})
        final = fit(self.tower_axon())
        cards_end = e.VGroup(*[self.badge(g(*NAMES[key]), (-5.9 + i * 1.7, -2.3), color=COLORS[key], size=18) for i, key in enumerate(GROUP_KEYS)])
        recap_parts = {"planes": [_morph.token("", kind="plane", color=COLORS["planes"], size=1.4).move_to(final.get_center())], "core": [_morph.silhouette(part) for part in final.cores.values()], "capsules": shapes(final),
                       "capital": [_morph.silhouette(part) for part in final.capitals.values()], "podium": [_morph.silhouette(final.podium)]}
        splits = [_morph.split(_morph.silhouette(badge), recap_parts[key], run_time=1.6) for key, badge in zip(GROUP_KEYS, cards_end) if key in recap_parts]
        stage("recap", canvas("orient"), final, cards_end,
              via={"drawing": [e.FadeOut(programs), e.FadeIn(final), *splits]}, done=lambda: settle(*[shape for shapes_ in recap_parts.values() for shape in shapes_]))
