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
_engine = importlib.import_module("manim")

INK, MUTED, DATA, GOOD, WARN = _visuals.INK, _visuals.MUTED, _visuals.DATA, _visuals.GOOD, _visuals.WARN
GROUP_KEYS = ("core", "planes", "capsules", "capital", "podium")
COLORS = dict(zip(GROUP_KEYS, _canvas.GROUPS))
NAMES = {"core": ("Core", "Kern"), "planes": ("Planes", "Ebenen"), "capsules": ("Capsules", "Kapseln"), "capital": ("Capital", "Kopf"), "podium": ("Podium", "Sockel")}
MEMBERS = {
    "core": ("side", "sh", "n", "ps", "two", "tn", "tot", "hc", "cp", "xy", "co", "core"),
    "capsules": ("capp", "ori", "st", "stl", "stc"),
    "capital": ("ow", "ch", "ow2", "cw", "chh", "czz", "cpt", "cpl", "capital"),
    "podium": ("pw", "pd", "rect", "uzp", "ext"),
}
LC, LW, LH = (-2.35, -0.03), 9.3, 5.05
VX, VY, VW, VH = 4.7, -0.03, 4.4, 5.05
TXT = "#1B1B1B"
MINUS = _visuals.MINUS
UNITS = {"side": "m", "sh": "m", "pitch": "m", "ow": "m", "ch": "m", "pw": "m", "pd": "m"}
GERMAN = {"Core side": "Kernseite", "Storey height": "Geschosshöhe", "Capsule storeys": "Kapselgeschosse", "Podium storeys": "Sockelgeschosse", "Slot pitch": "Raster",
          "Capital overhang": "Überstand Kopf", "Capital height": "Höhe Kopf", "Podium length": "Länge Sockel", "Podium width": "Breite Sockel",
          "Capsule per slot": "Kapsel pro Slot", "Cull indices": "Cull-Indizes"}
S_T, S_C, S_K = 0.115, 0.34, 0.22
CLOSE = np.array([VX, VY - 1.55, 0.0])
KBASE = np.array([VX, VY - 1.9, 0.0])


def _ink(color, amount=0.45):
    """🎨 Darker stroke colour of a group colour."""
    return _canvas.mix(color, "#000000", amount)


class Beat3_Tower(_visuals.ClipScene):
    """🗼 Animate the Nakagin tower as one definition: core, planes, capsules, capital and podium with explicit English and German narration."""

    TITLE = {"en": "From One Capsule to the Tower", "de": "Von der Kapsel zum Turm"}
    NARRATION = [
        ["hook", "The Nakagin Capsule Tower has 140 capsules. How do they become two towers?",
         "Der Nakagin Capsule Tower hat 140 Kapseln. Wie werden sie zu zwei Türmen?"],
        ["parts", "The tower splits into five parts: core, planes, capsules, capital and podium. Each part gets its own coloured group on the canvas.",
         "Der Turm hat fünf Teile: Kern, Ebenen, Kapseln, Kopf und Sockel. Jeder bekommt eine eigene farbige Gruppe."],
        ["core", "The core is a square extrusion. Plan side, storey height and storey count are the sliders; they decide the height of the box.",
         "Der Kern ist eine quadratische Extrusion. Seite, Geschosshöhe und Geschossanzahl sind Slider und bestimmen die Höhe."],
        ["shaft", "The result is the shaft: one box with a line on every storey, and the storey count is read straight from the definition.",
         "Das Ergebnis ist der Schacht: ein Quader mit einer Linie auf jedem Geschoss. Die Geschossanzahl wird direkt aus der Definition gelesen."],
        ["series", "A Series gives one elevation per storey. Each elevation becomes a level for the capsules on the shaft.",
         "Eine Series liefert eine Höhe pro Geschoss. Jede Höhe wird zu einer Ebene für die Kapseln am Schacht."],
        ["facades", "A second list holds the four facade angles. Rotating one plane by each angle gives one plane per facade.",
         "Eine zweite Liste enthält die vier Fassadenwinkel. Dreht man eine Ebene um jeden Winkel, entsteht eine Ebene pro Fassade."],
        ["axes", "Every plane carries its axes: X runs along the facade, Y points outward, Z points up.",
         "Jede Ebene trägt ihre Achsen: X läuft entlang der Fassade, Y zeigt nach außen, Z zeigt nach oben."],
        ["items", "Two capsules sit side by side on each facade: item 0 and item 1, one step to each side along X.",
         "Auf jeder Fassade sitzen zwei Kapseln nebeneinander: Item 0 und Item 1, je einen Schritt beidseits entlang X."],
        ["orient", "Orient moves the capsule from its base plane onto each target plane. One capsule, one component, every plane.",
         "Orient setzt die Kapsel von ihrer Basisebene auf jede Zielebene. Eine Kapsel, eine Komponente, jede Ebene."],
        ["types", "A type pattern picks the capsule type for every plane. List Item reads the pattern by index; only the list differs.",
         "Ein Typmuster wählt für jede Ebene den Kapseltyp. List Item liest das Muster per Index; nur die Liste ändert sich."],
        ["count", "Now the capsule storeys slider: the count is storeys times facades times items, and the drawing grows with it.",
         "Nun der Slider Kapselgeschosse: Die Anzahl ist Geschosse mal Fassaden mal Items, und die Zeichnung wächst mit."],
        ["nowire", "Not one wire changes. The definition does not know how tall the tower is; it only follows the number.",
         "Keine einzige Verbindung ändert sich. Die Definition weiß nicht, wie hoch der Turm ist; sie folgt nur der Zahl."],
        ["mistake", "A typical mistake: without Cull Index, tower B gets 88 capsules, two of them west on the top storey, where the building has none.",
         "Typischer Fehler: Ohne Cull Index hat Turm B 88 Kapseln, zwei davon im obersten Geschoss im Westen, wo es keine gibt."],
        ["fix", "Cull Index removes exactly those two planes, indices 6 and 7. Now the definition gives 86, the number the building really has.",
         "Cull Index entfernt genau diese zwei Ebenen, die Indizes 6 und 7. Nun liefert die Definition 86, die Zahl des Gebäudes."],
        ["capital", "The capital is its own small parametric part. Height and overhang are two sliders on top of the core.",
         "Der Kopf ist ein eigenes kleines Parametermodell. Höhe und Überstand sind zwei Slider auf dem Kern."],
        ["ontop", "It sits on top of the core at the height the capsule storeys slider gives, so it rises with the tower.",
         "Er sitzt auf dem Kern genau auf der Höhe, die der Slider Kapselgeschosse vorgibt, und steigt mit dem Turm."],
        ["podium", "The lower building is only an extruded massing: one footprint, one height.",
         "Nur ein extrudierter Baukörper: Grundriss und Höhe."],
        ["why", "That is on purpose. The podium is context, not the subject: one extrusion is enough, while one capsule parameter changes every copy.",
         "Das ist Absicht. Der Sockel ist Kontext, nicht das Thema: eine Extrusion genügt, ein Kapselparameter dagegen ändert jede Kopie."],
        ["towers", "The same definition builds tower A and tower B. Capsule storeys and facades differ; the formulas show both.",
         "Dieselbe Definition baut Turm A und Turm B. Kapselgeschosse und Fassaden unterscheiden sich; die Formeln zeigen beides."],
        ["total", "Together they give the historical count, 140 capsules, read from the definition and not typed in.",
         "Zusammen ergeben sie die historische Zahl von 140 Kapseln, aus der Definition gelesen und nicht eingetippt."],
        ["swap", "Metabolism: capsules are replaceable. Swap type B for type C, and every capsule of that type gets the larger window at once.",
         "Metabolismus: Kapseln sind austauschbar. Ersetzt man Typ B durch Typ C, bekommt jede Kapsel dieses Typs sofort das größere Fenster."],
        ["recap", "Recap: core, planes, capsules, capital, podium. This is the teaching model of the Nakagin Capsule Tower.",
         "Zusammenfassung: Kern, Ebenen, Kapseln, Kopf, Sockel. So sieht das Lehrmodell des Nakagin Capsule Tower aus."],
    ]

    def construct(self):
        self.present("tower")

    def _fmt(self, value):
        """🔢 Trimmed localized number as a Grasshopper panel shows it."""
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            return str(value)
        text = f"{value:.3f}".rstrip("0").rstrip(".")
        text = "0" if text in ("-0", "") else text
        return text.replace(".", ",") if self.language() == "de" else text

    def _solve(self, over=None):
        """⚙️ Solve the tower canvas once per override set."""
        key = repr(sorted((over or {}).items()))
        if key not in self._cache:
            self._cache[key] = _gh.evaluate(self._defn, dict(over or {}))
        return self._cache[key]

    def _lines(self, tree):
        """📝 Panel data of an engine tree: path to formatted items."""
        return {path: [self._fmt(i) if isinstance(i, (int, float)) else _gh.describe(i) for i in tree.branch(path)] for path in tree.paths()}

    def _widget(self, oid, at, sol, over, width=None, selected=False, lines=None):
        """🧩 One engine object as a canvas widget with its solved value."""
        obj, spec = self._defn.objects[oid], self._cat[self._defn.objects[oid].component]
        s = dict(obj.settings)
        if spec.kind == "slider":
            value = _gh.slider_value({**s, "value": over.get(oid, s["value"])})
            digits = 0 if s.get("rounding") == "integer" else int(s.get("decimals", 2))
            label = s["label"] if self.language() == "en" else GERMAN.get(s["label"], s["label"])
            return self.gh_slider(label, value, s["min"], s["max"], at=at, decimals=digits, width=width or 3.6, selected=selected, unit=UNITS.get(oid, ""))
        if spec.kind == "toggle":
            return self.gh_toggle(bool(over.get(oid, s["value"])), at=at, name=self._swap_label)
        if spec.kind == "panel":
            tree = sol.tree(oid)
            return self.gh_panel(lines or self._lines(tree), at=at, width=width, paths=len(tree.paths()) > 1, max_lines=14 if oid == "gp" or not width else 4, fmt=lambda item: str(item), selected=selected)
        if spec.kind == "param":
            label = s.get("label")
            label = label if self.language() == "en" else GERMAN.get(label, label)
            return self.gh_param(spec.outputs[0].type, at=at, label=label, selected=selected)
        return self.gh_component(obj.component, at=at, state=sol.state(oid), selected=selected)

    def _wire(self, made, sol, wire, selected=False):
        """🔌 One engine wire between two drawn widgets, styled by what it carries."""
        tree = sol.trees.get((wire.src, wire.out))
        kind = "tree" if tree is not None and len(tree.paths()) > 1 else "list" if tree is not None and len(tree.items()) > 1 else "item"
        plain = lambda oid: self._cat[self._defn.objects[oid].component].kind in ("slider", "panel", "param", "toggle")
        out = made[wire.src].grip_out(None if plain(wire.src) else wire.out)
        inp = made[wire.dst].grip_in(None if plain(wire.dst) else wire.inp)
        return self.gh_wire(out, inp, kind=kind, selected=selected)

    def _net(self, layout, *, over=None, widths=None, selected=(), bridges=(), group=None, panels=None, extra=()):
        """🕸️ A subnet of the tower canvas at given positions: widgets, the engine's wires between them and dashed bridges over hidden objects."""
        over, widths = over or {}, widths or {}
        sol = self._solve(over)
        made = {oid: self._widget(oid, at, sol, over, widths.get(oid), oid in selected, (panels or {}).get(oid)) for oid, at in layout.items()}
        wires = [self._wire(made, sol, w, w.src in selected and w.dst in selected) for w in self._defn.wires if w.src in made and w.dst in made]
        for src, out, dst, inp in bridges:
            wires.append(self.gh_wire(made[src].grip_out(None if src in ("n", "ps", "sh", "pitch", "ch", "ow") else out), made[dst].grip_in(inp), kind="tree"))
        net = _engine.VGroup(*made.values(), *wires)
        net.made, net.wires = made, wires
        if group:
            frame = self.gh_group(_engine.VGroup(*made.values(), *extra), NAMES[group][0 if self.language() == "en" else 1], color=COLORS[group])
            net.add(frame)
            net.frame = frame
        return net

    def _mark(self, part, key, buff=0.06):
        """🖍️ Outline a drawn part in the colour of its canvas group."""
        box = _engine.SurroundingRectangle(part, color=_ink(COLORS[key], 0.25), buff=buff, stroke_width=4)
        box.set_fill(COLORS[key], opacity=0.22)
        return box

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

    def _axes(self, td, plane, *, length=2.0, labels=False, scale=S_C):
        """➡️ Plane axes aligned with a pinned tower drawing."""
        return self.plane_axes(plane, at=self._pt(td, *plane.origin), scale=scale, length=length, labels=labels)

    def _storey_planes(self, k=None):
        """🧭 Engine planes per storey: branch to the eight planes of that storey."""
        tree = self._solve({"n": k} if k else None).tree("mv", "Geometry")
        return [tree.branch(p) for p in tree.paths()]

    def _shaft_dims(self, td, side, height):
        """📏 Plan side and storey height dimensions on the pinned core."""
        cx, cy, s = self.B.center[0], self.B.center[1], side / 2
        width = self.dimension(self._pt(td, cx - s, cy + s, 0.0), self._pt(td, cx + s, cy + s, 0.0), self.quantity(side, "m", 1), offset=-0.45, color=DATA)
        count = round(height / self.B.storey_height)
        rise = self.dimension(self._pt(td, cx + s, cy + s, 0.0), self._pt(td, cx + s, cy + s, height), self.quantity(height, "m"), offset=0.6, color=WARN)
        return _engine.VGroup(width, rise)

    def _swap(self, old, new, seconds=1.0):
        """🔁 Replace one group by another with a cross fade."""
        return _engine.AnimationGroup(*[_engine.FadeOut(o) for o in old], *[_engine.FadeIn(n) for n in new])

    def _marks(self, td):
        """🖍️ Group-coloured outlines on the cores, capitals and podium of a drawing."""
        parts = [(self._mark(core, "core"),) for core in td.cores.values()] + [(self._mark(cap, "capital"),) for cap in td.capitals.values()]
        parts.append((self._mark(td.podium, "podium", 0.02),))
        return [m for group in parts for m in group]

    def _say(self, string, at, size=22, color=TXT, anchor="c"):
        """🏷️ Dark label on the light canvas."""
        return self.text(string, at, size=size, color=color, anchor=anchor)

    def _fit(self, drawing, viewport, margin=0.3):
        """🧲 Scale a static drawing into the viewport."""
        return viewport.fit(drawing, margin)

    def _define(self, spec, key):
        """🧪 Solve a small custom definition on the engine and return it with its solution."""
        defn = _gh.define(spec, key)
        return defn, _gh.evaluate(defn, dict(spec.get("over", {})))

    def _custom(self, defn, sol, layout, group, extra=(), widths=None, panels=None):
        """🕸️ Draw a custom engine definition at given positions with its wires and one group."""
        saved = self._defn
        self._defn = defn
        try:
            made = {oid: self._widget(oid, at, sol, {}, (widths or {}).get(oid), lines=(panels or {}).get(oid)) for oid, at in layout.items()}
            wires = [self._wire(made, sol, w) for w in defn.wires if w.src in made and w.dst in made]
        finally:
            self._defn = saved
        net = _engine.VGroup(*made.values(), *wires)
        net.made, net.wires = made, wires
        frame = self.gh_group(_engine.VGroup(*made.values(), *extra), NAMES[group][0 if self.language() == "en" else 1], color=COLORS[group])
        net.add(frame)
        return net

    def _type_legend(self, at):
        """🎨 Colour chips of the four capsule types."""
        chips = [_engine.VGroup(_engine.Square(0.26, fill_color=_nakd.type_color(t.key), fill_opacity=1, stroke_color=TXT, stroke_width=1.5),
                                self._say(t.key, (0, 0), 20, anchor="l")).arrange(_engine.RIGHT, buff=0.12) for t in _nak.capsule_types()]
        return _engine.VGroup(*chips).arrange(_engine.RIGHT, buff=0.45).move_to(np.array([at[0], at[1], 0.0]))

    def present_tower(self):
        """🎞️ The whole animation: one capsule, the five parts, the count rising with the storeys, the Cull Index mistake, both towers, the swap of a type."""
        lang = self.language()
        self._defn, self._cat, self._cache = _gh.canvas("tower"), _gh.catalogue(), {}
        towers, self._slots = _nak.towers(), _nak.slots()
        self.A, self.B = towers[0], towers[1]
        self._pods = int(_nak.value("core.podium_storeys").value)
        types = {t.key: t for t in _nak.capsule_types()}
        kind = {(s.building, s.tower, s.storey, s.facade, s.item): s.type for s in self._slots}
        total = len(self._slots)
        e, g = _engine, self.tr
        name = lambda key: NAMES[key][0 if lang == "en" else 1]
        sep = "; " if lang == "de" else ", "
        full_solution = self._solve()
        lengths = full_solution.items("st", "Length")
        rotated = full_solution.tree("rot", "Geometry")
        facades, items = len(rotated.paths()), len(rotated.branch(rotated.paths()[0]))
        high = sum(1 for v in lengths if v == lengths[0])
        low = max(1, high // 3)
        settings = lambda oid: self._defn.objects[oid].settings
        self.k = k = e.ValueTracker(low)
        shown = lambda: int(round(k.get_value()))
        self.add(self.gh_background(at=LC, width=LW, height=LH), _fonts.section_heading(self.TITLE[lang]))
        viewport = self.gh_viewport(at=(VX, VY), width=VW, height=VH, label="Perspective")
        self.add(viewport)

        cap_a = self.capsule_axon(types["A"], at=(-4.6, 0.3), scale=0.72)
        one = self._say(g("1 capsule", "1 Kapsel"), (-4.6, -1.5), 26)
        full = self.tower_axon(at=(VX, VY), scale=0.1, lit={(0, 1, 4, 0, 0)}, types=False)
        self._fit(full, viewport)
        counter = e.ValueTracker(1)
        number = self.live_label(lambda: self.number(round(counter.get_value()), 0), (-1.0, 0.3), size=96, color=TXT)
        unit = self._say(g("capsules in the towers", "Kapseln in den Türmen"), (-1.0, -0.6), 26)
        self.step("hook", e.FadeIn(cap_a), e.FadeIn(one), e.FadeIn(full), e.FadeIn(number), e.FadeIn(unit), counter.animate.set_value(total), seconds=6.0)

        full_plain = self.tower_axon(at=(VX, VY), scale=0.1, types=False, tint=lambda key: COLORS["capsules"])
        self._fit(full_plain, viewport)
        marks = self._marks(full_plain)
        samples = {"core": "Center Box", "planes": "Rotate", "capsules": "Orient", "capital": "Center Box", "podium": "Extrude"}
        spots = {"core": (-5.2, 1.45), "planes": (-2.5, 1.45), "capsules": (0.3, 1.45), "capital": (-3.6, -1.1), "podium": (-0.8, -1.1)}
        boxes = []
        for key in GROUP_KEYS:
            part = self.gh_component(samples[key], at=spots[key])
            boxes.append(e.VGroup(part, self.gh_group(part, name(key), color=COLORS[key])))
        part_cards = e.VGroup(*boxes)
        self.step("parts", e.FadeOut(cap_a), e.FadeOut(one), e.FadeOut(number), e.FadeOut(unit), e.FadeOut(full), e.FadeIn(full_plain),
                  e.LaggedStart(*[e.FadeIn(b) for b in boxes], lag_ratio=0.35), e.LaggedStart(*[e.FadeIn(m) for m in marks], lag_ratio=0.3), seconds=6.0)

        n_t = e.ValueTracker(high // 2)
        step_n = lambda: int(round(n_t.get_value()))
        layout_core = {"side": (-4.7, 1.45), "sh": (-4.7, 0.5), "n": (-4.7, -0.45), "ps": (-4.7, -1.4), "tn": (-1.9, -1.0), "tot": (-0.5, -0.1), "core": (1.2, 0.8)}
        core_net = self.live(lambda: self._net(layout_core, over={"n": step_n()}, group="core"), key=step_n)

        def build_shaft():
            drawing = self.core_axon(self.B, at=(VX, VY), scale=S_T, storeys=step_n() + self._pods)
            return self._pin(drawing, (self.B.center[0], 0.0, 0.0), KBASE)

        shaft = self.live(build_shaft, key=step_n)
        dims = self.live(lambda: self._shaft_dims(shaft, self.B.core_side, (step_n() + self._pods) * self.B.storey_height), key=step_n)
        equation = self.live_label(lambda: f"{step_n() + self._pods} × {self.number(self.B.storey_height)} m = {self.number((step_n() + self._pods) * self.B.storey_height)} m", (-3.0, -2.3), size=24, color=TXT)
        self.step("core", e.FadeIn(equation), e.FadeOut(part_cards), *[e.FadeOut(m) for m in marks], e.FadeOut(full_plain), e.FadeIn(core_net), e.FadeIn(shaft), e.FadeIn(dims), n_t.animate.set_value(self.B.capsule_storeys), seconds=7.0)

        levels = e.VGroup(*[self.level_mark(self._pt(shaft, self.B.center[0] - self.B.core_side / 2, self.B.center[1] + self.B.core_side / 2, i * self.B.storey_height),
                                            self.number(i, 0), size=16, length=0.4, side=-1, color=TXT) for i in (self.B.storeys,)])
        tn_box = self._mark(core_net.made["tn"], "core", 0.04)
        self.step("shaft", e.FadeIn(tn_box), e.Indicate(shaft, color=DATA), e.FadeIn(levels), seconds=5.0)

        plane_net = self._net({"ps": (-4.7, 1.55), "sh": (-4.7, 0.5), "n": (-4.7, -0.9), "z0": (-1.8, 1.3), "ser": (-0.5, 0.15), "gp": (1.5, 0.0)}, widths={"gp": 1.4}, group="planes",
                              bridges=(("ser", "Series", "gp", "Stream"),))
        series_axes = e.VGroup(*[self._axes(shaft, self._plane(b[0]), length=2.2, scale=S_T) for b in self._storey_planes()])
        self.step("series", e.FadeOut(equation), e.FadeOut(core_net), e.FadeOut(tn_box), e.FadeOut(levels), e.FadeOut(dims), e.FadeIn(plane_net),
                  e.LaggedStart(*[e.FadeIn(a) for a in series_axes], lag_ratio=0.1), seconds=6.0)

        closeup = self.tower_axon(at=(VX, VY), scale=S_C, towers=(1,), storeys=3, where=lambda s: False, podium=False, capital=False)
        self._pin(closeup, (self.B.center[0], 0.0, 0.0), CLOSE)
        angle_net = self._net({"p0s": (-5.0, 1.2), "ang": (-5.4, -1.1), "rad": (-3.6, -1.1), "ga": (-1.9, -1.1), "rot": (0.7, 0.2)}, widths={"ang": 1.2}, group="planes")
        first = self._storey_planes()[0]
        facade_planes = [dataclasses.replace(first[items * f], origin=_gh.Point(*(sum(np.array(first[items * f + i].origin.t) for i in range(items)) / items))) for f in range(facades)]
        facade_axes = e.VGroup(*[self._axes(closeup, self._plane(p), length=2.2, labels=False) for p in facade_planes])
        self.step("facades", e.FadeOut(plane_net), e.FadeOut(series_axes), e.FadeOut(shaft), e.FadeIn(closeup), e.FadeIn(angle_net),
                  e.LaggedStart(*[e.FadeIn(a) for a in facade_axes], lag_ratio=0.4), seconds=7.0)

        triple = lambda v, d=3: sep.join(self._fmt(round(float(c), d)) for c in v.t)
        plane = first[0]
        panel = self.gh_panel([g("Plane", "Ebene"), "O  " + triple(plane.origin) + " m", "X  " + triple(plane.x), "Y  " + triple(plane.y), "Z  " + triple(plane.z)], at=(-2.4, 0.2), width=4.6)
        panel_group = self.gh_group(panel, name("planes"), color=COLORS["planes"])
        big = self._axes(closeup, self._plane(plane), length=3.4, labels=True)
        self.step("axes", e.FadeOut(angle_net), e.FadeOut(facade_axes), e.FadeIn(panel), e.FadeIn(panel_group), e.FadeIn(big), seconds=6.0)

        both = e.VGroup(*[self._axes(closeup, self._plane(first[i]), length=2.6, labels=False) for i in range(items)])
        tags = e.VGroup(*[self._say(f"[{i}]", self._pt(closeup, *self._plane(first[i]).origin) + np.array([0, -0.45, 0]), 20, "#FFFFFF") for i in range(items)])
        sx_panel = self.gh_panel(self._lines(full_solution.tree("sx")), at=(1.4, -0.8), width=1.4)
        centre = sum(np.array(first[i].origin.t) for i in range(items)) / items
        step_dims = e.VGroup(self.dimension(self._pt(closeup, *self._plane(first[0]).origin), self._pt(closeup, *self._plane(first[1]).origin),
                                            self.quantity(abs(float(first[1].origin.t[0] - first[0].origin.t[0])), "m"), offset=-0.55, color=DATA))
        items_net = self._net({"pitch": (-4.7, 1.4), "hp": (-1.9, 1.4), "nhp": (-1.9, -0.3), "sx": (0.0, 0.7)}, group="planes", extra=(sx_panel,))
        items_wire = self.gh_wire(items_net.made["sx"].grip_out(), sx_panel.grip_in(), kind="list")
        self.step("items", e.FadeOut(panel), e.FadeOut(panel_group), e.FadeOut(big), e.FadeIn(items_net), e.FadeIn(sx_panel), e.FadeIn(items_wire), e.FadeIn(both), e.FadeIn(tags), e.FadeIn(step_dims), seconds=6.0)

        orient_net = self._net({"capp": (-5.0, 1.2), "xy": (-5.4, -0.8), "slots": (-2.3, -1.0), "ori": (0.6, 0.2)}, group="capsules", bridges=(("xy", "Plane", "ori", "Source"),))
        origin = self._plane(first[0])
        source = SimpleNamespace(origin=origin.origin + np.array([6.0, 0.0, 0.0]), x=origin.x, y=origin.y, z=origin.z)
        source_axes = self._axes(closeup, source, length=2.6, labels=False)
        half = np.array([0.0, types["A"].length / 2, types["A"].height / 2])
        start = self.capsule_axon(types["A"], at=self._pt(closeup, *(source.origin + half)), scale=S_C)
        shift = self._pt(closeup, *(origin.origin + half)) - self._pt(closeup, *(source.origin + half))
        slot_cap = self._tower(1, types=False, capital=False, scale=S_C, storeys=3, base=CLOSE)
        self.play(e.FadeOut(items_net), e.FadeOut(sx_panel), e.FadeOut(items_wire), e.FadeOut(tags), e.FadeOut(step_dims), e.FadeIn(orient_net), e.FadeIn(source_axes), e.FadeIn(start), run_time=1.5)
        self.step("orient", start.animate.shift(shift), seconds=4.0)
        self.play(e.FadeIn(slot_cap), e.FadeOut(start), e.FadeOut(source_axes), e.FadeOut(both), e.FadeOut(closeup), run_time=1.5)

        types_view = self._tower(low, types=True, capital=False, scale=S_K, storeys=low + self._pods + 1, base=KBASE)
        kinds = [s.type for s in self._slots if s.tower == 1 and s.storey == 0]
        types_spec = {"objects": [{"id": "pat", "component": "Panel", "settings": {"lines": kinds}}, {"id": "cnt", "component": "Panel", "settings": {"lines": [str(len(kinds))]}},
                                  {"id": "ser", "component": "Series", "settings": {}}, {"id": "li", "component": "List Item", "settings": {}}, {"id": "out", "component": "Panel", "settings": {"lines": [""]}}],
                      "wires": ["pat.Stream>li.List", "cnt.Stream>ser.Count", "ser.Series>li.Index", "li.Item>out.Stream"]}
        types_defn, types_sol = self._define(types_spec, "types")
        legend = self._type_legend((-2.6, -2.15))
        numbers = e.VGroup(*[e.VGroup(e.Circle(radius=0.14, fill_color="#FFFFFF", fill_opacity=0.95, stroke_color=TXT, stroke_width=2), self._say(str(2 * sl.facade + sl.item), (0, 0), 16)).move_to(
            self._pt(types_view, *sl.plane.origin) + np.array([0.0, 0.0, 0.0])) for sl in self._slots if sl.tower == 1 and sl.storey == 0])
        types_net = self._custom(types_defn, types_sol, {"pat": (-5.6, 0.2), "cnt": (-5.6, -1.5), "ser": (-3.5, -1.2), "li": (-1.7, 0.3), "out": (0.7, 0.2)}, "capsules")
        self.step("types", e.FadeOut(orient_net), e.FadeOut(slot_cap), e.FadeIn(types_net), e.FadeIn(legend), e.FadeIn(types_view), e.FadeIn(numbers), seconds=6.0)

        count_layout = {"n": (-4.7, 1.4), "ser": (-1.4, 1.4), "ori": (0.7, 1.4), "st": (-2.0, -0.6), "stc": (1.2, -0.3), "stl": (1.2, -1.3)}
        net_live = self.live(lambda: self._net(count_layout, over={"n": shown()}, widths={"stl": 1.2}, group="capsules", bridges=(("ser", "Series", "ori", "Target"),),
                                                panels={"stc": {(0,): [str(shown())]}, "stl": {(0,): [str(v) for v in lengths[:shown()]]}}), key=shown)
        tower_live = self.live(lambda: self._tower(k.get_value()), key=shown)
        formula = self.live_label(lambda: f"{shown()} × {facades} × {items} = {sum(lengths[:shown()])}", (-4.5, -1.6), size=30, color=TXT)
        k.set_value(low)
        self.step("count", e.FadeOut(types_net), e.FadeOut(legend), e.FadeOut(types_view), e.FadeOut(numbers), e.FadeIn(net_live), e.FadeIn(tower_live), e.FadeIn(formula), k.animate.set_value(high), seconds=9.0)

        same = self._say(g("same wires, other value", "gleiche Verbindungen, anderer Wert"), (-3.0, -2.35), 24)
        self.step("nowire", e.FadeIn(same), k.animate.set_value(high // 2), seconds=4.0)
        self.wait(0.1)
        self.play(k.animate.set_value(high), e.FadeOut(same), run_time=2.5)

        empty = {"cullp": {"tree": {"{0;%d}" % i: [] for i in range(len(lengths))}}}
        wrong = self._solve(empty)
        wrong_lengths = wrong.items("st", "Length")
        cull_layout = {"mv": (-5.4, 0.8), "cull": (-3.1, 0.0), "st": (-1.0, 0.1), "stc": (1.1, -0.9), "stl": (1.1, 0.7)}
        cull_tree = full_solution.tree("cullp")
        cull_lines = lambda: {path: [str(v) for v in cull_tree.branch(path)] for path in cull_tree.paths() if cull_tree.branch(path)}
        panel_wrong, panel_right = self.gh_panel(cull_lines(), at=(-5.4, -1.1), width=1.5), self.gh_panel(cull_lines(), at=(-5.4, -1.1), width=1.5)
        cull_wrong = self._net(cull_layout, over=empty, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),), extra=(panel_wrong,))
        cull_wrong.add(panel_wrong)
        cull_right = self._net(cull_layout, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),), extra=(panel_right,))
        cull_right.add(panel_right, self.gh_wire(panel_right.grip_out(), cull_right.made["cull"].grip_in("Indices"), kind="tree"))
        sum_wrong = self._say(f"Σ = {sum(wrong_lengths)}", (-2.6, -1.45), 30)
        sum_right = self._say(f"Σ = {sum(lengths)}", (-2.6, -1.45), 30)
        top_planes = self._solve().tree("mv", "Geometry")
        last = top_planes.branch(top_planes.paths()[-1])
        removed = [i for i in range(len(last)) if wrong_lengths[-1] - lengths[-1] and i >= lengths[-1]]
        tower_full = self.live(lambda: self._tower(k.get_value()), key=shown)
        ghosts = e.VGroup(*[_nakd._capsule_lite(_nakd._cap(types["A"]), SimpleNamespace(origin=np.array(last[i].origin.t), x=np.array(last[i].x.t), y=np.array(last[i].y.t), z=np.array(last[i].z.t)),
                                                tower_full.view, "#D8362B").shift(tower_full.delta).set_opacity(0.55) for i in removed])
        crosses = e.VGroup(*[e.VGroup(e.Circle(radius=0.12, color="#D8362B", stroke_width=4), e.Cross(scale_factor=0.09, stroke_color="#D8362B", stroke_width=4)).move_to(
            self._pt(tower_full, *self._plane(last[i]).origin)) for i in removed])
        self.step("mistake", e.FadeOut(net_live), e.FadeOut(formula), e.FadeIn(cull_wrong), e.FadeIn(sum_wrong), e.FadeIn(ghosts), e.FadeIn(crosses), k.animate.set_value(high + 1), seconds=5.0)
        self.step("fix", e.ReplacementTransform(cull_wrong, cull_right), e.ReplacementTransform(sum_wrong, sum_right), e.FadeOut(crosses), e.FadeOut(ghosts), seconds=5.0)

        ow_t, ch_t = e.ValueTracker(self.B.capital_overhang), e.ValueTracker(self.B.capital_height)
        round2 = lambda: (round(ch_t.get_value(), 2), round(ow_t.get_value(), 2))
        capital_net = self.live(lambda: self._net({"ow": (-4.7, 1.3), "ch": (-4.7, 0.2), "cw": (-1.3, 1.0), "capital": (1.2, 0.6)}, over={"ow": round2()[1], "ch": round2()[0]},
                                                   group="capital", bridges=(("ow", "Number", "cw", "B"), ("ch", "Number", "capital", "Z"))), key=round2)

        def build_capital():
            shaped = dataclasses.replace(self.B, capital_height=round2()[0], capital_overhang=round2()[1])
            drawing = self.capital_axon(shaped, at=(VX, VY), scale=0.4)
            return self._pin(drawing, (self.B.center[0], self.B.center[1], shaped.height), (VX, VY - 0.2, 0.0))

        cap_live = self.live(build_capital, key=round2)
        stub = self.core_axon(self.B, at=(VX, VY), scale=0.4, storeys=1)
        self._pin(stub, (self.B.center[0], self.B.center[1], self.B.storey_height), (VX, VY - 0.2, 0.0))

        def build_cap_dim():
            b = self.B.core_side / 2 + round2()[1]
            x, y = self.B.center[0] + b, self.B.center[1] + b
            return self.dimension(self._pt(cap_live, x, y, self.B.height), self._pt(cap_live, x, y, self.B.height + round2()[0]), self.quantity(round2()[0], "m"), offset=-0.35, color=DATA)

        cap_dim = self.live(build_cap_dim, key=round2)
        self.step("capital", e.FadeOut(cull_right), e.FadeOut(sum_right), e.FadeOut(tower_live), e.FadeIn(capital_net), e.FadeIn(stub), e.FadeIn(cap_live), e.FadeIn(cap_dim), seconds=4.0)
        self.play(ch_t.animate.set_value(settings("ch")["max"]), ow_t.animate.set_value(settings("ow")["max"]), run_time=2.5)
        self.play(ch_t.animate.set_value(self.B.capital_height), ow_t.animate.set_value(self.B.capital_overhang), run_time=2.0)

        k.set_value(low + 1)
        top_net = self.live(lambda: self._net({"n": (-4.7, 1.5), "tot": (-1.6, 1.5), "chh": (-1.6, 0.0), "czz": (-0.6, 0.75), "capital": (1.2, 0.75)}, over={"n": shown()},
                                               group="capital", bridges=(("n", "Number", "tot", "B"), ("czz", "Result", "capital", "B"))), key=shown)
        height_label = self.live_label(lambda: g("Capital base Z", "Kopf Unterkante Z") + f" = {self._fmt(self._solve({'n': shown()}).value('tot'))} m", (-3.4, -1.5), size=26, color=TXT)
        rising = self.live(lambda: self._tower(k.get_value()), key=shown)
        self.step("ontop", e.FadeOut(capital_net), e.FadeOut(cap_live), e.FadeOut(cap_dim), e.FadeOut(stub), e.FadeIn(top_net), e.FadeIn(rising), e.FadeIn(height_label), k.animate.set_value(high), seconds=6.0)

        podium_net = self._net({"pw": (-4.7, 1.7), "pd": (-4.7, 0.8), "ps": (-4.7, -0.1), "sh": (-4.7, -1.0), "rect": (-1.7, 1.2), "z0": (-1.7, -0.5), "uzp": (0.2, -1.0), "ext": (1.0, 0.2)}, group="podium")
        podium_view = self.podium_axon(at=(VX, VY), scale=0.1)
        self._fit(podium_view, viewport, 0.5)
        length, width, height = _nak.value("podium.length").value, _nak.value("podium.width").value, _nak.podium_height()
        p_dims = self._say(f"{self.number(length, 1)} × {self.number(width, 1)} × {self.number(height, 1)} m", (VX, VY - 2.2), 22)
        self.step("podium", e.FadeOut(top_net), e.FadeOut(rising), e.FadeOut(height_label), e.FadeIn(podium_net), e.FadeIn(podium_view), e.FadeIn(p_dims), seconds=5.0)

        capsule_comps = len(_gh.canvas("capsule").objects)
        cards = []
        for key, head, body_lines in (("podium", g("Podium: context", "Sockel: Kontext"), [g(f"{len(MEMBERS['podium'])} components", f"{len(MEMBERS['podium'])} Komponenten"), g("one extrusion", "eine Extrusion")]),
                                      ("capsules", g("Capsule: the subject", "Kapsel: das Thema"), [g(f"{capsule_comps} components", f"{capsule_comps} Komponenten"), g(f"{total} copies", f"{total} Kopien")])):
            box = e.RoundedRectangle(corner_radius=0.12, width=5.4, height=1.9, fill_color=COLORS[key], fill_opacity=0.55, stroke_color=_ink(COLORS[key]), stroke_width=3)
            lines = e.VGroup(self._say(head, (0, 0), 26), *[self._say(t, (0, 0), 22) for t in body_lines]).arrange(e.DOWN, buff=0.15).move_to(box)
            cards.append(e.VGroup(box, lines))
        e.VGroup(*cards).arrange(e.DOWN, buff=0.5).move_to(np.array([-2.45, 0.2, 0.0]))
        capsule_lit = self.tower_axon(at=(VX, VY), scale=0.1, lit=lambda key: True, types=False)
        self._fit(capsule_lit, viewport)
        podium_outline = self._mark(capsule_lit.podium, "podium", 0.02)
        self.step("why", e.FadeOut(podium_net), e.FadeOut(podium_view), e.FadeOut(p_dims), e.FadeIn(capsule_lit), e.FadeIn(podium_outline), e.LaggedStart(*[e.FadeIn(c) for c in cards], lag_ratio=0.4), seconds=7.0)

        per_tower = _nak.totals()["per_tower"]
        sliders = e.VGroup(*[self.gh_slider(g("Capsule storeys ", "Kapselgeschosse ") + t.key, t.capsule_storeys, 1, 13, at=(-4.7, 1.0 - 1.2 * i), decimals=0, width=3.6) for i, t in enumerate((self.A, self.B))])
        generic = {t.key: t.capsule_storeys * len(_nak.facade_storeys(t)[0]) * items for t in (self.A, self.B)}
        formula_lines = [f"A  {self.A.capsule_storeys} × {len(_nak.facade_storeys(self.A)[0])} × {items} = {generic['A']}",
                         f"B  {self.B.capsule_storeys} × {len(_nak.facade_storeys(self.B)[0])} × {items} = {generic['B']}",
                         f"Cull Index  {MINUS}{generic['B'] - per_tower['B']['count']}", f"B  = {per_tower['B']['count']}"]
        a_panel = self.gh_panel(formula_lines, at=(-0.9, 0.3), width=3.3)
        a_group = self.gh_group(e.VGroup(sliders, a_panel), name("capsules"), color=COLORS["capsules"])
        tint_a, tint_b = "#3FA7D6", "#E59A3C"
        build_a = self.tower_axon(at=(VX, VY), scale=0.1, tint={(0, 0): tint_a}, types=False)
        self._fit(build_a, viewport, 0.7)
        build_a.shift(np.array([0.0, -0.2, 0.0]))
        build_b = self.tower_axon(at=(VX, VY), scale=0.1, tint={(0, 0): tint_a, (0, 1): tint_b}, types=False)
        self._fit(build_b, viewport, 0.7)
        build_b.shift(np.array([0.0, -0.2, 0.0]))
        facade_count = lambda t: len(_nak.facade_storeys(t)[0])
        tag_a = self._say("A  " + g(f"{self.A.capsule_storeys} storeys, {facade_count(self.A)} facades", f"{self.A.capsule_storeys} Geschosse, {facade_count(self.A)} Fassaden"), np.array([VX - 2.05, VY + 2.05, 0.0]), 18, tint_a, "l")
        tag_b = self._say("B  " + g(f"{self.B.capsule_storeys} storeys, {facade_count(self.B)} facades", f"{self.B.capsule_storeys} Geschosse, {facade_count(self.B)} Fassaden"), np.array([VX - 2.05, VY + 1.78, 0.0]), 18, tint_b, "l")
        self.play(e.FadeOut(*cards), e.FadeOut(podium_outline), e.FadeOut(capsule_lit), e.FadeIn(sliders), e.FadeIn(a_panel), e.FadeIn(a_group), e.FadeIn(build_a), run_time=1.5)
        self.step("towers", e.FadeIn(tag_a), e.Indicate(build_a, color=tint_a), seconds=3.0)
        self.play(e.FadeOut(build_a), e.FadeIn(build_b), e.FadeIn(tag_b), run_time=1.5)

        sum_panel = self.gh_panel([f"A  {per_tower['A']['count']}", f"B  {per_tower['B']['count']}", f"Σ  {total}"], at=(-1.0, -2.0), width=1.6)
        lit_all = self.tower_axon(at=(VX, VY), scale=0.1, tint={(0, 0): tint_a, (0, 1): tint_b}, lit=lambda key: True, types=False)
        self._fit(lit_all, viewport, 0.7)
        lit_all.shift(np.array([0.0, -0.2, 0.0]))
        self.step("total", e.FadeIn(sum_panel), e.FadeOut(build_b), e.FadeOut(tag_a), e.FadeOut(tag_b), e.FadeIn(lit_all), seconds=5.0)

        old_key, new_key = "B", "C"
        self._swap_label = f"{old_key} → {new_key}"
        flag = lambda key: [str(sl.type == key) for sl in self._slots]
        swap_spec = lambda on: {"objects": [{"id": "all", "component": "Panel", "settings": {"lines": [sl.type for sl in self._slots]}},
                                            {"id": "pb", "component": "Panel", "settings": {"lines": flag(old_key)}}, {"id": "pd", "component": "Panel", "settings": {"lines": flag(new_key)}},
                                            {"id": "db", "component": "Dispatch", "settings": {}}, {"id": "dd", "component": "Dispatch", "settings": {}},
                                            {"id": "cb", "component": "List Length", "settings": {}}, {"id": "cd", "component": "List Length", "settings": {}},
                                            {"id": "sw", "component": "Boolean Toggle", "settings": {"value": on}},
                                            {"id": "mul", "component": "Multiplication", "settings": {}}, {"id": "add", "component": "Addition", "settings": {}},
                                            {"id": "sub", "component": "Subtraction", "settings": {}},
                                            {"id": "rb", "component": "Panel", "settings": {"lines": [""]}}, {"id": "rd", "component": "Panel", "settings": {"lines": [""]}}],
                                "wires": ["all.Stream>db.List", "pb.Stream>db.Pattern", "all.Stream>dd.List", "pd.Stream>dd.Pattern", "db.List A>cb.List", "dd.List A>cd.List",
                                          "cb.Length>mul.A", "sw.Boolean>mul.B", "cd.Length>add.A", "mul.Result>add.B", "cb.Length>sub.A", "mul.Result>sub.B", "sub.Result>rb.Stream", "add.Result>rd.Stream"]}
        swap_layout = {"sw": (-5.3, 1.7), "all": (-5.9, -0.5), "cb": (-3.9, 0.5), "cd": (-3.9, -1.4), "mul": (-2.0, 1.3), "sub": (-0.75, 1.75), "add": (-0.75, -0.5), "rb": (1.3, 1.75), "rd": (1.3, -0.5)}
        before_defn, before_sol = self._define(swap_spec(False), "swap")
        after_defn, after_sol = self._define(swap_spec(True), "swap")

        def swap_net(defn, sol):
            labelled = {oid: {(0,): [f"{self.tr('Type', 'Typ')} {key}: {self._fmt(sol.items(oid)[0])}"]} for oid, key in (("rb", old_key), ("rd", new_key))}
            net = self._custom(defn, sol, swap_layout, "capsules", widths={"all": 0.8, "rb": 2.0, "rd": 2.0}, panels=labelled)
            net.add(*[self.gh_wire(net.made["all"].grip_out(), net.made[oid].grip_in(), kind="list") for oid in ("cb", "cd")])
            return net

        before_net, after_net = swap_net(before_defn, before_sol), swap_net(after_defn, after_sol)
        swap_legend = self._type_legend((-2.6, -2.15))
        recolor = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: _nakd.type_color(new_key) if kind[key] == old_key else None)
        recolor.scale_to_fit_width(VW - 1.7).move_to(np.array([VX, VY - 0.75, 0.0]))
        pair_old = self.capsule_axon(types[old_key], at=(VX - 1.0, VY + 1.65), scale=0.3, dims=("window",))
        pair_new = self.capsule_axon(types[new_key], at=(VX + 1.0, VY + 1.65), scale=0.3, dims=("window",), highlight={"front": _nakd.type_color(new_key)})
        pair_arrow = e.Arrow(np.array([VX - 0.25, VY + 1.65, 0]), np.array([VX + 0.25, VY + 1.65, 0]), color=TXT, buff=0)
        pair = e.VGroup(pair_old, pair_new, pair_arrow)
        self.play(e.FadeOut(sum_panel), e.FadeOut(sliders), e.FadeOut(a_panel), e.FadeOut(a_group), e.FadeIn(before_net), e.FadeIn(swap_legend), run_time=1.5)
        self.step("swap", e.ReplacementTransform(before_net, after_net), e.FadeOut(lit_all), e.FadeIn(recolor), e.FadeIn(pair), seconds=5.0)

        final = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: COLORS["capsules"])
        self._fit(final, viewport)
        outlines = self._marks(final)
        pairs = [("Plane", "Ebene"), ("Facade", "Fassade"), ("Storey", "Geschoss"), ("Group", "Gruppe"), ("Capital", "Kopf"), ("Podium", "Sockel")]
        words = [f"{a} = {b}" if lang == "en" else f"{b} = {a}" for a, b in pairs]
        terms = e.VGroup(self._say("  ·  ".join(words[:3]), (-2.35, -2.0), 19), self._say("  ·  ".join(words[3:]), (-2.35, -2.3), 19))
        recap_cards = part_cards.copy().scale(0.82).shift(np.array([0.0, 0.3, 0.0]))
        self.step("recap", e.FadeOut(after_net), e.FadeOut(swap_legend), e.FadeOut(recolor), e.FadeOut(pair), e.FadeIn(final), e.FadeIn(recap_cards), e.FadeIn(terms), e.LaggedStart(*[e.FadeIn(m) for m in outlines], lag_ratio=0.3), seconds=5.0)
