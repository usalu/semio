"""🌳 Data trees on the aggregated Nakagin tower: paths, branches, items, Flatten, Graft, Simplify, Path Mapper, Flip Matrix and branch matching."""

import importlib
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
_canvas = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🦗️canvas.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_engine = importlib.import_module("manim")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
_sprites = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")

INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN, SERV = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "DATA", "GOOD", "WARN", "SERV"))
TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
PINK, ITEM_C, TOWER_GREY = "#ff9db0", "#9fb4ff", "#d8d8d8"
STOREY_LIGHT, PEACH = _draft.mix(GOOD, INK, 0.55), "#ffcf9f"
TOWER_COLORS = (ARCH, WARN)
ROLE_COLORS = (INK, TOWER_GREY, GOOD, PINK)
LEVELS = ((INK, ("Building", "Gebäude")), (TOWER_GREY, ("Tower", "Turm")), (GOOD, ("Storey", "Geschoss")), (PINK, ("Facade", "Fassade")), (ITEM_C, ("Index", "Index")), (INK, ("Branch", "Ast")))
FULL = (0, 1, 2, 3)
ROLES = (("Building", "Gebäude"), ("Tower", "Turm"), ("Storey", "Geschoss"), ("Facade", "Fassade"))
ITEM_WORD = ("Capsules", "Kapseln")
FACADES = (("north", "Norden"), ("east", "Osten"), ("south", "Süden"), ("west", "Westen"))
MATERIALS = {"teal": ARCH, "orange": WARN}
LABEL, SMALL = 19.2, 16.2
TOWER_FIT = _canvas.building_fit(5.4, 5.2, centre=(4.3, 0.62, 0.0))
SHEET_AT, SHEET_W, SHEET_H = (-3.55, 2.4, 0), 6.5, 2.9
SRC_AT, COMP_AT, PANEL_AT = (-5.95, 2.4, 0), (-3.9, 2.4, 0), (-1.45, 2.4, 0)
HEAD_X = -6.8
FAN_LEFT, FAN_RIGHT, GROUND, CROWN = -5.35, -0.5, -2.3, 0.3
HEAD_Y = 0.66
SOURCE, FLAT_LINES, PROBE = "src", 7, 21
SHEET_INK, SHEET_GOOD = "#222222", "#1E6B2E"


def _fade(group, alpha):
    """🌫️ Scale every stroke and fill opacity of a freshly built group by an alpha."""
    for mob in group.get_family():
        mob.set_fill(opacity=float(mob.get_fill_opacity()) * alpha)
        mob.set_stroke(opacity=float(mob.get_stroke_opacity()) * alpha)
    return group


def _release_tower(display):
    """🧹 Release semantic vectors after their last authored use while retaining display pixels."""
    if isinstance(display, _snapshot.Snapshot) and hasattr(display, "_drawing_ref"):
        display.capsules.clear()
        display._drawing_ref.value = None


class _DrawingRef:
    """🗼 Share a semantic vector drawing when presentation frames are copied."""
    def __init__(self, value):
        self.value = value

    def __deepcopy__(self, memo):
        return self


class _Capsules(dict):
    """🧸 Keep capsule references outside snapshot animation copies."""
    def __deepcopy__(self, memo):
        return self


def _sequence(scene, builders, seconds, first=(), weights=None):
    """⏱️ Build each animation only when its stage begins."""
    weights = weights or [1 / len(builders)] * len(builders)
    if len(weights) != len(builders) or not np.isclose(sum(weights), 1):
        raise ValueError("Stage weights must sum to one")
    for index, (build, weight) in enumerate(zip(builders, weights)):
        scene.play(*(tuple(first) if index == 0 else ()), build(), run_time=seconds * weight)
        scene._settle()


class Beat5_DataTrees(_visuals.ClipScene):
    """🌳 Animate Grasshopper data trees on the aggregated Nakagin tower with explicit English and German narration."""

    TITLE = {"en": "Data Trees on the Tower", "de": "Datenbäume am Turm"}
    NARRATION = [
        ["hook", "In this teaching model, the Nakagin Capsule Tower has two towers and 140 capsules. One list holds them all, but which one sits where?",
         "In diesem Lehrmodell hat der Nakagin Capsule Tower zwei Türme und 140 Kapseln. Eine Liste enthält alle, aber welche Kapsel sitzt wo?"],
        ["flat", "Item 21 is one capsule of 140. The list does not say its tower, its storey or its facade.",
         "Element 21 ist eine von 140 Kapseln. Die Liste verrät weder Turm noch Geschoss noch Fassade."],
        ["tree", "A data tree keeps the address: building, tower, storey, facade. Each level splits into more branches.",
         "Ein Datenbaum behält die Adresse: Gebäude, Turm, Geschoss, Fassade. Jede Ebene verzweigt sich weiter."],
        ["branch", "One occupied facade group is one branch. Its length follows the reconstructed floor layout.",
         "Eine belegte Fassadengruppe bildet einen Ast. Ihre Länge folgt dem rekonstruierten Geschossgrundriss."],
        ["path", "The selected capsule keeps its path: building, tower, capsule level and facade. Read its actual address on screen.",
         "Die gewählte Kapsel behält ihren Pfad: Gebäude, Turm, Kapselebene und Fassade. Ihre tatsächliche Adresse steht im Bild."],
        ["item", "An index selects one capsule inside its branch. The path and index together identify the selected capsule.",
         "Ein Index wählt eine Kapsel innerhalb ihres Astes. Pfad und Index identifizieren gemeinsam die gewählte Kapsel."],
        ["panel", "A Panel lists the occupied facade branches. Tree Statistics reveals the different branch lengths.",
         "Ein Panel listet die belegten Fassadenäste. Tree Statistics zeigt die unterschiedlichen Astlängen."],
        ["flatten_one", "Flatten on one occupied storey joins its facade branches into one list, preserving every capsule.",
         "Flatten an einem belegten Geschoss verbindet seine Fassadenäste zu einer Liste und erhält jede Kapsel."],
        ["flatten", "Flatten puts all 140 capsules into one list. Nothing is lost, except the structure.",
         "Flatten legt alle 140 Kapseln in eine Liste. Es geht nichts verloren, außer der Struktur."],
        ["graft_one", "Graft gives every capsule of that storey its own branch, with one item per branch.",
         "Graft gibt jeder Kapsel dieses Geschosses einen eigenen Ast mit genau einem Element."],
        ["graft", "Graft does the opposite: every capsule gets its own branch, and the path grows by one index.",
         "Graft macht das Gegenteil: Jede Kapsel bekommt einen eigenen Ast, und der Pfad wird um einen Index länger."],
        ["simplify_one", "Simplify removes the common building, tower and storey prefix. The occupied facade branches remain.",
         "Simplify entfernt das gemeinsame Präfix aus Gebäude, Turm und Geschoss. Die belegten Fassadenäste bleiben."],
        ["simplify", "Simplify removes the leading 0 that every path shares, because there is only one building. The branches stay.",
         "Simplify entfernt die führende 0, die alle Pfade teilen, weil es nur ein Gebäude gibt. Die Äste bleiben."],
        ["mapper_example", "Path Mapper binds letters to the displayed address. The facade letter is dropped; the remaining address selects a storey.",
         "Path Mapper bindet Buchstaben an die angezeigte Adresse. Der Fassadenbuchstabe entfällt; die übrige Adresse wählt ein Geschoss."],
        ["mapper", "Applied to all branches, the rule {A;B;C;D}(i) to {A;B;C} merges the facades: each branch is a storey, 20 branches.",
         "Auf alle Äste angewendet, fasst die Regel {A;B;C;D}(i) nach {A;B;C} die Fassaden zusammen: Jeder Ast ist ein Geschoss, 20 Äste."],
        ["perstorey", "Each branch now contains one occupied storey. The documented counts vary: two, seven or eight capsules.",
         "Jeder Ast enthält nun ein belegtes Geschoss. Die belegten Anzahlen variieren: zwei, sieben oder acht Kapseln."],
        ["pertower", "Mapping to {A;B} gives one branch per tower: 76 capsules in A and 64 in B.",
         "Das Abbilden auf {A;B} ergibt einen Ast je Turm: 76 Kapseln in A und 64 in B."],
        ["flip", "Flip Matrix swaps branches and item positions. Short branches receive null placeholders: empty addresses, not capsules.",
         "Flip Matrix tauscht Äste und Positionen. Kurze Äste erhalten Null-Platzhalter: leere Adressen, keine Kapseln."],
        ["match", "A component pairs data branch by branch. Custom Preview takes capsules and a colour, and the paths decide who pairs with whom.",
         "Eine Komponente verknüpft Daten Ast für Ast. Custom Preview nimmt Kapseln und eine Farbe, und die Pfade entscheiden, wer zu wem gehört."],
        ["mistake", "The classic mistake: the short colour list repeats in every branch. Alternating colours follow item indices instead of tower addresses.",
         "Der klassische Fehler: Die kurze Farbliste wiederholt sich in jedem Ast. Abwechselnde Farben folgen Elementindizes statt Turmadressen."],
        ["why", "The Param Viewer shows why: no path matches, so the short colour list repeats in every branch. Read both paths first.",
         "Der Param Viewer zeigt, warum: Kein Pfad passt, also wiederholt sich die kurze Farbliste in jedem Ast. Erst beide Pfade lesen."],
        ["fix", "The fix: map the capsules to {A;B} and graft the colour list. Now both trees have the paths {0;0} and {0;1}.",
         "Die Lösung: Kapseln auf {A;B} abbilden und die Farbliste mit Graft Tree verteilen. Nun tragen beide Bäume die Pfade {0;0} und {0;1}."],
        ["result", "Matching paths send the tower colours to the documented totals: 76 teal capsules in A and 64 orange in B.",
         "Passende Pfade verteilen die Turmfarben auf die belegten Summen: 76 türkise Kapseln in A und 64 orange in B."],
        ["recap", "Recap: tree, branch, path, item, index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Ten terms, one address.",
         "Zusammenfassung: Datenbaum, Ast, Pfad, Element, Index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Zehn Begriffe, eine Adresse."]
    ]

    def construct(self):
        """▶️ Present this chapter through the native Grasshopper visual interface."""
        self.present("trees")

    def _put(self, shape, point, edge="c"):
        """📌 Move a shape so its left edge, right edge or centre sits on ``point``."""
        dx = {"l": point[0] - shape.get_left()[0], "r": point[0] - shape.get_right()[0], "c": point[0] - shape.get_center()[0]}[edge]
        return shape.shift(np.array([dx, point[1] - shape.get_center()[1], 0.0]))

    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """🔤 One label aligned on its left edge, right edge or centre."""
        return self._put(self.text(string, (0, 0, 0), size=size, color=color, width=width), point, edge)

    def _mono(self, string, point, *, size=LABEL, color=INK, edge="c"):
        """⌨️ One monospace label aligned on its left edge, right edge or centre."""
        return self._put(self.mono(string, (0, 0, 0), size=size, color=color), point, edge)

    def _word(self, pair):
        """🈯 Localized word of an English and German pair."""
        return self.tr(*pair)

    @staticmethod
    def _name(key):
        """🔑 Part name of a capsule path in a bound entity."""
        return "-".join(str(part) for part in key)

    def step(self, key, *animations, seconds=1.2):
        """⏱️ One narrated clause; flight targets that landed on identical drawn objects leave afterwards."""
        super().step(key, *animations, seconds=seconds)
        self._settle()

    def _fly(self, name, sources, targets):
        """✈️ Play the declared transition ``trees.<name>``; its target stays in ``self._born`` until ``_settle`` drops it from above the identical drawing."""
        if isinstance(sources, (dict, list)) and not sources:
            return _engine.Wait(0.01)
        animation = self.stage.transition(f"trees.{name}", source=sources, target=targets)
        self._born.append(self.stage.mobject(f"trees.{name}.target"))
        return animation

    def _copies(self, tower, keys):
        """🧸 Capsule copies of a tower drawing by part name; projected sprites when there are many."""
        if len(keys) <= 10:
            return {self._name(key): tower.capsules[key].copy() for key in keys}
        return {self._name(key): _sprites.capture(tower.capsules[key], self.camera) for key in keys}

    def _centre(self, drawing, prefix):
        """🎯 Centre of the capsules of a tower drawing whose path starts with ``prefix``."""
        return _engine.VGroup(*[mob for key, mob in drawing.capsules.items() if key[:len(prefix)] == tuple(prefix)]).get_center()

    def _hatch(self, name, tower, keys, targets):
        """🌱 Capsule copies of a tower drawing fly into their ``{key: mobject}`` targets (one declared transition)."""
        return self._fly(name, self._copies(tower, keys), {self._name(key): targets[key] for key in keys})

    def _settle(self):
        """🧹 Drop the item tokens of a finished birth; the diagram leaves lie underneath."""
        self.remove(*self._born)
        self._born = []

    def _birth(self, name, tower, diagram):
        """🌱 Capsules of a tower drawing turn into the leaves of a tree diagram, in list order, keeping their tower colour."""
        keys = sorted((k for k in diagram.leaves if k in tower.capsules), key=self._order.get)
        many = len(keys) > 10
        leaf = lambda key: _morph.token("", kind="item", color=diagram.leaves[key][1], size=0.35).move_to(diagram.leaves[key][0]) if many else diagram.leaves[key][2].copy()
        return self._hatch(name, tower, keys, {key: leaf(key) for key in keys})

    def _regroup(self, name, old, new):
        """🔀 Leaves of an old tree diagram fly to the same capsules in a new diagram (Flatten, Graft, Simplify, Path Mapper, Flip Matrix)."""
        before, after = getattr(old, "leaves", {}), getattr(new, "leaves", {})
        keys = sorted((k for k in after if k in before), key=self._order.get)
        return self._fly(name, {self._name(k): before[k][2].copy() for k in keys}, {self._name(k): after[k][2].copy() for k in keys})

    def _gather(self, name, tower, keys, chip):
        """🧲 Capsules of a tower drawing collapse into one number or chip."""
        keys = sorted(keys, key=self._order.get)
        return self._fly(name, self._copies(tower, keys), chip.copy())

    def _paint(self, name, tower, swatches, colour_of):
        """🎨 Colour swatch copies leave their swatches and land on the capsules they paint."""
        keys = sorted((k for k in tower.capsules if k in self._order), key=self._order.get)
        return self._fly(name, {self._name(k): swatches[colour_of[k]].copy() for k in keys}, {self._name(k): tower.capsules[k].copy() for k in keys})

    def _fan(self, name, swatches, diagram, colour_of):
        """🎆 The grafted colour swatches fan out: a swatch copy flies to every leaf of the diagram and paints it in its item's colour."""
        keys = sorted((k for k in diagram.leaves if k in colour_of), key=self._order.get)
        return self._fly(name, {self._name(k): swatches[colour_of[k]].copy() for k in keys}, {self._name(k): diagram.leaves[k][2].copy() for k in keys})

    def _tokens(self, parts, point, *, size=LABEL, edge="c"):
        """🎨 A monospace row of coloured ``(text, colour)`` tokens that touch each other."""
        row = _engine.VGroup(*[self.mono(text, (0, 0, 0), size=size, color=color) for text, color in parts]).arrange(_engine.RIGHT, buff=0.045)
        return self._put(row, point, edge)

    def _count_text(self, tree):
        """🔢 Branch and item count of a tree as one localized line."""
        stats = tree.stats()
        branches = stats["path_count"]
        nulls = sum(item is None for path in tree.paths() for item in tree.branch(path))
        items = stats["item_count"] - nulls
        label = self.tr(f"{branches} {'branch' if branches == 1 else 'branches'} · {items} items", f"{branches} {'Ast' if branches == 1 else 'Äste'} · {items} Elemente")
        return label + (self.tr(f" · {nulls} nulls", f" · {nulls} Nullwerte") if nulls else "")

    def _rows(self, tree, count=3, only=None):
        """📝 Panel rows (path header, then indexed items) of the first ``count`` branches or of ``only``."""
        rows, last, seen = [], None, 0
        for path, index, text in _model.panel_rows(tree, paths=True):
            key = path if path is not None else "{0}"
            if only is not None and key not in only:
                continue
            if key != last:
                seen += 1
                if only is None and seen > count:
                    break
                rows.append(("path", "", key))
                last = key
            rows.append(("item", str(index), text))
        return rows

    def _staged_step(self, key, builders, *, seconds, first=(), weights=None):
        """🎞️ Keep caption timing while constructing each sequential stage on demand."""
        row = next(row for row in self.NARRATION if row[0] == key)
        caption = row[1] if self.language() == "en" else row[2]
        if self._caption is None:
            self._caption = _visuals._art.caption_bar(caption)
            self.play(_engine.FadeIn(self._caption), run_time=0.3)
            self._vo_caption_t0 = self.renderer.time
        else:
            self._caption = _visuals._art.swap_caption(self, self._caption, caption)
        _sequence(self, builders, seconds, first, weights)
        localized = [(code, en, en if self.language() == "en" else de) for code, en, de in self.NARRATION]
        _visuals._art.hold_for(self, localized, key)

    def _swap(self, slot, new):
        """🔁 Cross-fade the mobject held in a named slot with a new one; head and thread texts retext in place so old and new strings never overlap."""
        old = self._slots.get(slot)
        if slot in ("head", "thread") and old is not None and new is not None:
            self._slots[slot] = new
            return self.stage.transition(f"trees.{slot}-retext", source=old, target=new)
        self._slots[slot] = new
        moves = [*([_engine.FadeOut(old)] if old is not None else []), *([_engine.FadeIn(new)] if new is not None else [])]
        if slot == "tower" and old is not new:
            _release_tower(old)
        return _engine.AnimationGroup(*moves) if moves else _engine.Wait(0.01)

    def _tower(self, lit=None, tint=None, types=False, ring=True):
        """🗼 The full Nakagin tower drawing at the stage anchor, capsule 21 ringed."""
        drawing = self.tower_axon(**TOWER_FIT, view="axon", lit=lit, tint=tint, types=types)
        if ring:
            self._ring(drawing)
        display = _snapshot.capture(drawing, self.camera)
        display.capsules, display._drawing_ref = _Capsules(drawing.capsules), _DrawingRef(drawing)
        return display

    def _ring(self, drawing):
        """⭕ A yellow ring around capsule 21, the object that runs through the whole video."""
        mob = drawing.capsules.get(self._probe)
        if mob is not None:
            drawing.add(_engine.Circle(radius=max(mob.width, mob.height) * 0.8, stroke_color=DATA, stroke_width=3.2, fill_opacity=0).move_to(mob.get_center()))
        return drawing

    def _locate(self, tree):
        """🔎 Path and index of capsule 21 in an engine tree."""
        for path in tree.paths():
            for index, item in enumerate(tree.branch(path)):
                if item is not None and tuple(item.tag.get("slot", ())) == self._probe:
                    return path, index
        raise KeyError("capsule 21")

    def _thread(self, tree=None):
        """🧵 The running chip: where capsule 21 sits in the tree on screen."""
        if tree is None:
            text = self.tr("capsule 21 = ?", "Kapsel 21 = ?")
        else:
            path, index = self._locate(tree)
            text = f"#21 = {_model.format_path(path)}[{index}]"
        return self._tag(text, (-0.35, HEAD_Y), DATA, "r")

    def _tag(self, string, point, color=INK, edge="c", size=SMALL):
        """🏷️ A coloured outlined badge."""
        return self.badge(string, point, color=color, size=size, filled=False, anchor={"l": "l", "r": "r", "c": "c"}[edge])

    def _chips(self, *specs):
        """🏷️ Coloured badges in one row along the top right edge, the last spec at the edge."""
        row, edge = _engine.VGroup(), 6.8
        for text, color in reversed(specs):
            chip = self._tag(text, (edge, 3.65), color, "r")
            row.add(chip)
            edge = chip.get_left()[0] - 0.15
        return row

    def _plate(self, point, width, height):
        """⬛ A dark plate that keeps light text readable on the grey canvas."""
        return _engine.RoundedRectangle(corner_radius=0.08, width=width, height=height, fill_color=_draft.BACKGROUND, fill_opacity=0.96, stroke_width=0).move_to((*point[:2], 0))

    def _diagram(self, tree, levels, *, lit=None, labels=True, leaf_color=None, big=False):
        """🌳 Draw a data tree as a real tree: trunk at the bottom (or several roots when the first level is not shared), a level per path segment, one leaf per item, all read from the engine tree.
        ``levels`` names the meaning of each path segment (indices into ``LEVELS``) for colour and label."""
        entries = [(path, index) for path in tree.paths() for index in range(len(tree.branch(path)))]
        depth = len(tree.paths()[0])
        count = len(entries)
        step = (FAN_RIGHT - FAN_LEFT) / count
        ys = [GROUND + (CROWN - GROUND) * level / (depth + 1) for level in range(depth + 2)]
        xs = {}
        for slot, (path, index) in enumerate(entries):
            x = FAN_LEFT + (slot + 0.5) * step
            for level in range(1, depth + 1):
                xs.setdefault(path[:level], []).append(x)
        centre = {prefix: sum(values) / len(values) for prefix, values in xs.items()}
        roots = [prefix for prefix in centre if len(prefix) == 1]
        base = np.array([centre[roots[0]] if len(roots) == 1 else 0.0, ys[0], 0.0])
        bright = lambda prefix: lit is None or any(prefix[:len(item)] == item[:len(prefix)] for item in lit)
        colors = [*[LEVELS[i][0] for i in levels], ITEM_C]
        tone_of = lambda prefix: TOWER_COLORS[prefix[len(prefix) - 1] % 2] if levels[len(prefix) - 1] == 1 else colors[len(prefix) - 1]
        buckets, dots, leaves = {}, _engine.VGroup(), {}
        position = lambda prefix: base if not prefix else np.array([centre[prefix], ys[len(prefix)], 0.0])
        for prefix in sorted(centre, key=lambda p: (len(p), p)):
            level = len(prefix)
            if level > 1 or len(roots) == 1:
                key = (tone_of(prefix), 1.0 if bright(prefix) else 0.16, 2.6 if level == 1 else 1.6 if level < 3 else 1.1)
                buckets.setdefault(key, []).append((position(prefix[:-1]), position(prefix)))
            if level <= 2 or count <= 40:
                dots.add(_engine.Dot(position(prefix), radius=0.075 if level == 1 else 0.05 if level == 2 else 0.035, color=tone_of(prefix),
                                     fill_opacity=1.0 if bright(prefix) else 0.16))
        radius = min(0.07 if big else 0.045, step * 0.4)
        for slot, (path, index) in enumerate(entries):
            x = FAN_LEFT + (slot + 0.5) * step
            full = (*path, index)
            is_null = tree.branch(path)[index] is None
            tone = MUTED if is_null else (leaf_color(path, index) if leaf_color else ITEM_C)
            on = lit is None or any(full[:len(item)] == item[:len(full)] for item in lit)
            buckets.setdefault((tone, 1.0 if on else 0.16, 0.9), []).append((position(path), np.array([x, ys[depth + 1], 0.0])))
            leaf = _engine.Dot(np.array([x, ys[depth + 1], 0.0]), radius=radius, color=tone, fill_opacity=0 if is_null else (1.0 if on else 0.16), stroke_width=1.2 if is_null else 0)
            dots.add(leaf)
            tag = getattr(tree.branch(path)[index], "tag", None)
            if tag and "slot" in tag:
                leaves[tuple(tag["slot"])] = (np.array([x, ys[depth + 1], 0.0]), tone, leaf)
        group = _engine.VGroup(*[_draft.segments(pairs, color, width, opacity) for (color, opacity, width), pairs in buckets.items()], dots)
        if labels:
            names = [*[self._word(LEVELS[i][1]) for i in levels], self._word(ITEM_WORD)]
            for level in range(1, depth + 2):
                group.add(self._say(names[level - 1], (FAN_LEFT - 0.18, ys[level]), size=SMALL, color=colors[level - 1], edge="r"))
        group.ys, group.leaves = ys, leaves
        return group

    def _head(self, text):
        """📏 The count line above the tree diagram."""
        return self._say(text, (HEAD_X, HEAD_Y), edge="l")

    def _small(self, tree, prefix):
        """🔬 The branches of ``tree`` under a path prefix as a tree of their own, the smallest example of an operation."""
        return _model.Tree({path: tree.branch(path) for path in tree.paths() if path[:len(prefix)] == prefix})

    def _flat_rows(self, start, lit=None):
        """📝 Rows of the flat Panel list from item ``start``."""
        return [("item", str(index), "Brep") for index in range(start, start + FLAT_LINES)] + [("line", "", "...")]

    def _pipe(self, op, tree, *, rows=None, count=3, lines=7, mapping=None):
        """🧵 Source, one component and a Panel, wired as trees."""
        panel = self.gh_panel(None, rows=rows if rows is not None else self._rows(tree, count), at=PANEL_AT, max_lines=lines, max_width=2.0)
        if op is None:
            link = self.gh_wire(self._src.grip_out(), panel.grip_in(), kind="tree")
            return _engine.VGroup(panel, link)
        comp = self.gh_component(op, at=COMP_AT)
        group = _engine.VGroup(comp, panel, self.gh_wire(self._src.grip_out(), comp.grip_in(0), kind="tree"), self.gh_wire(comp.grip_out(0), panel.grip_in(), kind="tree"))
        if mapping:
            text = self.gh_panel(None, rows=[("line", "", line) for line in mapping.split("->")][:1] + [("line", "", "->" + mapping.split("->")[1])], at=(-5.7, 1.38), max_lines=2, max_width=1.9)
            group.add(text, self.gh_wire(text.grip_out(), comp.grip_in(1), kind="item"))
        group.comp, group.panel = comp, panel
        return group

    def _mapping(self, source, target, point):
        """🗺️ A Path Mapper rule with segment letters in the colours of their levels."""
        letters = {"A": INK, "B": TOWER_GREY, "C": GOOD, "D": PINK, "i": ITEM_C}
        parts = lambda text: [(c, letters.get(c, MUTED)) for c in text]
        row = _engine.VGroup(self._tokens(parts(source), (0, 0, 0), size=SMALL), self.mono("→", (0, 0, 0), size=SMALL, color=MUTED),
                             self._tokens(parts(target), (0, 0, 0), size=SMALL)).arrange(_engine.RIGHT, buff=0.12)
        return _engine.VGroup(self._plate(point, row.width + 0.3, 0.42), self._put(row, point))

    def _detail(self, lit, tint, scale, span):
        """🔍 A magnified slice of the tower and storey of capsule 21: only the storeys within ``span`` of it are drawn."""
        drawing = self.tower_axon(at=(4.3, 0.6, 0.0), scale=scale, view="axon", lit=lit, tint=tint, towers=(self._probe[1],), core=False, capital=False, podium=False, types=False,
                                  where=lambda slot: abs(slot.storey - self._probe[2]) <= span)
        return drawing

    def _lit_all(self, count):
        """🔦 Predicate lighting the first ``count`` capsules in flat list order."""
        order = self._order
        return lambda key: order[key] < count

    def _rule_tokens(self, text, point, size=SMALL):
        """🎨 A Path Mapper expression with the letters in the colours of their levels."""
        letters = {"A": INK, "B": TOWER_GREY, "C": GOOD, "D": PINK, "i": ITEM_C}
        return self._tokens([(c, letters.get(c, MUTED)) for c in text], point, size=size)

    def _path_parts(self, path, index, colors):
        """🎨 Coloured tokens of ``{p;q;…}[i]``."""
        parts = [("{", MUTED)]
        for number, value in enumerate(path):
            parts += [(str(value), colors[number]), (";" if number < len(path) - 1 else "}", MUTED)]
        return [*parts, (f"[{index}]", ITEM_C)]

    def _has(self, tree):
        """❓ Whether capsule 21 is part of an engine tree."""
        try:
            self._locate(tree)
        except KeyError:
            return False
        return True

    def _swatches(self, panel, colors):
        """🟦 Real colour swatches at the right edge of the rows of a colour Panel."""
        squares = _engine.VGroup()
        for row, color in zip(panel.parts["rows"], colors):
            squares.add(_engine.Square(side_length=0.17, fill_color=color, fill_opacity=1, stroke_color=SHEET_INK, stroke_width=1.2).move_to((panel.body.get_right()[0] - 0.22, row.get_center()[1], 0)))
        return squares

    def present_trees(self):
        """🎞️ The whole data-tree animation on the aggregated tower, following capsule 21."""
        self.stage = _morph.stage(self, TRANSITIONS)
        self._def = _model.canvas("trees")
        self._sol = _model.evaluate(self._def)
        self._tree = self._sol.tree(SOURCE)
        self._slots, self._born = {}, []
        slots = _nakagin.slots()
        self._order = {(*slot.path, slot.item): number for number, slot in enumerate(slots)}
        branch_positions = {tuple(item.tag["slot"]): index for path in self._tree.paths() for index, item in enumerate(self._tree.branch(path))}
        probe = slots[PROBE]
        self._probe = (*probe.path, probe.item)
        towers = {tower.index: tower for tower in _nakagin.towers()}
        per_storey = [len(self._sol.tree("pm", "Tree").branch(path)) for path in self._sol.tree("pm", "Tree").paths()]
        per_tower = [len(self._sol.tree("pt", "Tree").branch(path)) for path in self._sol.tree("pt", "Tree").paths()]
        kind = lambda tree: {(*b.tag["slot"],): b.tag["material"] for path in tree.paths() for b in tree.branch(path)}
        paint = {name: kind(self._sol.tree(name)) for name in ("cp1", "cp2")}
        total = self._tree.stats()["item_count"]
        width = lambda value: self.number(value, 0)
        sheet = self.gh_background(at=SHEET_AT, width=SHEET_W, height=SHEET_H)
        self._src = self.gh_param("Brep", at=SRC_AT, label=self.tr("Capsules", "Kapseln"))
        self._dock = _engine.VGroup(sheet, self._src)
        flat_tree = self._sol.tree("fl", "Tree")
        flip_tree = self._sol.tree("fp", "Data")
        graft_tree = self._sol.tree("gr", "Tree")
        simple_tree = self._sol.tree("sm", "Tree")
        storey_tree = self._sol.tree("pm", "Tree")
        tower_tree = self._sol.tree("pt", "Tree")
        branch_path = self._tree.paths()[0]
        branch_tree = _model.Tree({branch_path: self._tree.branch(branch_path)})
        self._results = {"src": self._tree, "fl": flat_tree, "gr": graft_tree, "sm": simple_tree, "pm": storey_tree, "pt": tower_tree, "fp": flip_tree}
        dim = lambda color: _draft.mix(color, _draft.BACKGROUND, 0.5)

        self._slots["tower"] = tower = self._tower(ring=False)
        self._slots["pipe"] = pipe = self._pipe(None, None, rows=self._flat_rows(0))
        count_chip = self._say(self.tr(f"{width(total)} capsules", f"{width(total)} Kapseln"), (HEAD_X, HEAD_Y), color=INK, edge="l")
        self._slots["head"] = count_chip
        self.step("hook", _engine.FadeIn(self._dock), _engine.FadeIn(tower), _engine.FadeIn(pipe), _engine.FadeIn(count_chip), seconds=1.4)

        frame = _engine.Rectangle(width=2.0, height=0.33, stroke_color=DATA, stroke_width=3, fill_opacity=0)
        rows21 = [("item", str(index), "Brep") for index in range(PROBE - 3, PROBE + 4)] + [("line", "", "...")]
        panel21 = self._pipe(None, None, rows=rows21)
        frame.move_to(panel21[0].parts["rows"][3].get_center()).match_x(panel21[0].body)
        question = self._say(self.tr("tower? storey? facade?", "Turm? Geschoss? Fassade?"), (2.2, 3.5), color=DATA)
        self.step("flat", self._hatch("flat-probe", self._slots["tower"], [self._probe], {self._probe: frame.copy()}), self._swap("pipe", panel21), self._swap("tower", self._tower(tint=lambda key: dim(TOWER_COLORS[0]) if key == self._probe else None)), self._swap("thread", self._thread()), _engine.FadeIn(frame), _engine.FadeIn(question), seconds=1.4)
        self._settle()

        full_diag = self._diagram(self._tree, FULL, leaf_color=lambda path, index: TOWER_COLORS[path[1] % 2])
        coloured = self._tower(tint=lambda key: TOWER_COLORS[key[1]], ring=False)
        birth = self._birth("birth-tree", coloured, full_diag)
        _release_tower(coloured)
        self.step("tree", birth, self._swap("tower", self._tower(tint=lambda key: dim(TOWER_COLORS[key[1]]))), _engine.FadeOut(frame), _engine.FadeOut(question), self._swap("pipe", self._pipe(None, self._tree, count=2)), self._swap("head", self._head(self._count_text(self._tree))),
                  _engine.FadeIn(full_diag, lag_ratio=0.01), seconds=2.4)
        self._settle()
        self._slots["diag"] = full_diag

        branch_pipe = self._pipe(None, branch_tree, rows=self._rows(branch_tree, 1))
        branch_diag = self._diagram(branch_tree, FULL, big=True)
        self.step("branch", self._birth("birth-branch", self._slots["tower"], branch_diag), self._swap("diag", branch_diag), self._swap("pipe", branch_pipe), self._swap("tower", self._tower(lit=[(0, 0, 0, 0)])),
                  self._swap("head", self._head(self.tr("1 branch · 2 items", "1 Ast · 2 Elemente"))), seconds=1.4)
        self._settle()

        segments, index21 = self._locate(self._tree)
        roles_c = (INK, TOWER_COLORS[segments[1] % 2], GOOD, PINK)
        tower_key = towers[segments[1]].key
        names = [self._word(LEVELS[0][1]), self.tr(f"Tower {tower_key}", f"Turm {tower_key}"), self.tr(f"Storey {segments[2]}", f"Geschoss {segments[2]}"),
                 self.tr(f"Facade {segments[3]} ({FACADES[segments[3]][0]})", f"Fassade {segments[3]} ({FACADES[segments[3]][1]})")]

        def shown(stage):
            """🎯 Path text and the words of its first ``stage`` segments on a dark plate."""
            group = _engine.VGroup(self._plate((-2.75, 2.25), 4.6, 2.4))
            parts = [("{", MUTED)]
            for number, value in enumerate(segments):
                parts += [(str(value), roles_c[number] if number < stage else MUTED), (";" if number < 3 else "}", MUTED)]
            group.add(self._tokens(parts, (-2.75, 3.05), size=40))
            for number in range(stage):
                y = 2.52 - 0.4 * number
                group.add(self._mono(str(segments[number]), (-4.7, y), size=LABEL, color=roles_c[number], edge="l"), self._say(names[number], (-4.25, y), size=LABEL, color=roles_c[number], edge="l"))
            return group

        def scope(stage):
            """🔭 Capsules of the first ``stage`` path segments in their segment colours; the last two stages magnify a slice of tower B."""
            prefix = segments[:stage]
            colour = lambda key: next((roles_c[n] for n in range(stage - 1, -1, -1) if key[:n + 1] == segments[:n + 1]), None)
            if stage < 3:
                return self._tower(tint=colour)
            return self._ring(self._detail([prefix], colour, *{3: (0.2, 2), 4: (0.3, 1)}[stage]))

        route = lambda stage: self._diagram(self._tree, FULL, lit=[segments[:stage]])

        def born(stage):
            """🌱 One path segment is born from its tower object and lands as the digit of the path text."""
            view, plate = scope(stage), shown(stage)
            digit = plate[1][2 * stage - 1]
            return _engine.AnimationGroup(self._swap("diag", route(stage)), self._swap("tower", view), self._swap("pipe", plate),
                                          *([self._swap("thread", self._thread(self._tree))] if stage == 4 else []),
                                          self._fly("path-segment", digit.copy().scale(4).move_to(self._centre(view, segments[:stage])), digit.copy()))

        self._staged_step("path", [lambda stage=stage: born(stage) for stage in (1, 2, 3, 4)], seconds=7.0,
                          first=[self._swap("head", self._head(self.tr("path = address of a branch", "Pfad = Adresse eines Asts")))])

        item_rows = [("path", "", _model.format_path(segments)), ("item", "0", "Brep"), ("item", "1", "Brep")]
        item_pipe = _engine.VGroup(self.gh_panel(None, rows=item_rows, at=(-3.9, 2.4), max_lines=4, max_width=2.0), self._plate((-1.55, 2.4), 2.2, 0.8),
                                   self._tokens([("{" + ";".join(map(str, segments)) + "}", MUTED), (f"[{index21}]", ITEM_C)], (-1.55, 2.4), size=20))
        right_capsule = self._ring(self._detail(lambda key: key == self._probe, lambda key: ITEM_C if key == self._probe else None, 0.3, 1))
        seat = right_capsule.capsules[self._probe].get_center()
        index_chip, whole_chip = item_pipe[2][1], item_pipe[2]
        index_out = self._fly("index-out", index_chip.copy().scale(3).move_to(seat), index_chip.copy())
        chip_back = self._fly("chip-back", whole_chip.copy(), whole_chip.copy().scale(0.35 / whole_chip.width).move_to(seat))
        self.step("item", _engine.AnimationGroup(index_out, chip_back, lag_ratio=0.3), self._swap("diag", self._diagram(self._tree, FULL, lit=[(*segments, index21)], big=True)), self._swap("pipe", item_pipe), self._swap("tower", right_capsule),
                  self._swap("head", self._head(self.tr("index = position in the branch", "Index = Position im Ast"))), seconds=2.4)

        viewer = self.gh_param_viewer(_model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:4]}), at=(-3.7, 2.45), max_items=2)
        panel_tree = self.gh_panel(None, rows=self._rows(self._tree, 3), at=(-1.3, 2.4), max_lines=8, max_width=1.7)
        viewer_pipe = _engine.VGroup(viewer, panel_tree, self.gh_wire(self._src.grip_out(), viewer.grip_in(), kind="tree"), self.gh_wire(self._src.grip_out(), panel_tree.grip_in(), kind="tree"),
                                     self._say(self.tr(f"Param Viewer: {self._tree.path_count} branches", f"Param Viewer: {self._tree.path_count} Äste"), (-3.7, 1.15), size=SMALL, color=SHEET_INK))
        stats = self.gh_component("Tree Statistics", at=COMP_AT)
        p_panel = self.gh_panel(None, rows=[("line", "", _model.format_path(path)) for path in self._tree.paths()[:2]], at=(-1.5, 3.3), max_lines=2, max_width=1.6)
        l_panel = self.gh_panel(None, rows=self._rows(self._sol.tree("stl"), 1)[1:4] + [("line", "", "...")], at=(-1.5, 2.2), max_lines=4, max_width=1.6)
        c_panel = self.gh_panel(None, rows=[("line", "", str(self._sol.tree("stc").branch((0,))[0]))], at=(-1.5, 1.2), max_lines=1, max_width=1.6)
        stats_pipe = _engine.VGroup(stats, p_panel, l_panel, c_panel, self.gh_wire(self._src.grip_out(), stats.grip_in(0), kind="tree"),
                                    self.gh_wire(stats.grip_out("Paths"), p_panel.grip_in(), kind="tree"), self.gh_wire(stats.grip_out("Length"), l_panel.grip_in(), kind="tree"),
                                    self.gh_wire(stats.grip_out("Count"), c_panel.grip_in(), kind="tree"))
        top_level = towers[1].capsule_storeys - 1
        top = lambda key: key[1] == 1 and key[2] == top_level
        below = lambda key: key[1] == 1 and key[2] == top_level - 1
        uneven = self._tower(tint=lambda key: GOOD if top(key) else STOREY_LIGHT if below(key) else TOWER_COLORS[key[1]])
        tags = self._chips((self.tr(f"B, level {top_level - 1}: {len(storey_tree.branch((0, 1, top_level - 1)))} capsules", f"B, Ebene {top_level - 1}: {len(storey_tree.branch((0, 1, top_level - 1)))} Kapseln"), STOREY_LIGHT), (self.tr(f"B, level {top_level}: {len(storey_tree.branch((0, 1, top_level)))} capsules", f"B, Ebene {top_level}: {len(storey_tree.branch((0, 1, top_level)))} Kapseln"), GOOD))
        def panel_first():
            return _engine.AnimationGroup(self._swap("pipe", viewer_pipe), self._swap("diag", self._diagram(self._tree, FULL)), self._swap("tower", self._tower(tint=lambda key: TOWER_COLORS[key[1]])),
                                          self._swap("head", self._head(self._count_text(self._tree))))
        def panel_second():
            pulls = [self._gather(f"gather-storey-{index}", uneven, [k for k in uneven.capsules if k[:3] == (0, 1, level)], tags[index])
                     for level, index in ((top_level, 0), (top_level - 1, 1))]
            return _engine.AnimationGroup(self._swap("pipe", stats_pipe), self._swap("tower", uneven), self._swap("tags", tags), *pulls)
        self._staged_step("panel", [panel_first, panel_second], seconds=3.6)

        def derived(op, tree, levels):
            """🧬 One restructuring result: component, Panel, diagram, count line and the address of capsule 21 all read from the evaluated tree."""
            return self._pipe(op, tree), self._diagram(tree, levels, big=len(tree.paths()) < 5), self._head(self._count_text(tree)), (self._thread(tree) if self._has(tree) else None)

        def operate(key, op, small, big, levels, small_lit, seconds, tower_big):
            """🔬 Introduce an operation on the smallest example in its own clause, then run it on the whole tower in the next."""
            first, second = derived(op, small, levels[0]), derived(op, big, levels[1])
            stage = lambda parts, tower, name: [self._regroup(f"regroup-{name}", self._slots.get("diag"), parts[1]), self._swap("pipe", parts[0]), self._swap("diag", parts[1]), self._swap("head", parts[2]), self._swap("thread", parts[3]), self._swap("tower", tower)]
            self.step(key + "_one", self._swap("tags", None), *stage(first, self._tower(lit=[small_lit]), key + "-one"), seconds=seconds[0])
            self.step(key, *stage(second, tower_big, key), seconds=seconds[1])

        sample_storey = next(path for path in storey_tree.paths() if len(storey_tree.branch(path)) > 2)
        one_storey = self._small(self._tree, sample_storey)
        sweep = self._tower(tint=lambda key: INK)
        plain = lambda: self._tower(tint=lambda key: TOWER_COLORS[key[1]])
        operate("flatten", "Flatten Tree", one_storey.flatten(), flat_tree, ((5,), (5,)), sample_storey, (1.6, 2.6), sweep)
        operate("graft", "Graft Tree", one_storey.graft(), graft_tree, ((0, 1, 2, 3, 4), (0, 1, 2, 3, 4)), sample_storey, (1.6, 1.6), plain())
        operate("simplify", "Simplify Tree", one_storey.simplify(), simple_tree, ((3,), (1, 2, 3)), sample_storey, (1.6, 1.6), plain())

        j21 = self._locate(storey_tree)
        group21 = (segments[0], segments[1], segments[2])
        binding = _engine.VGroup(self._plate((-2.7, 2.4), 4.7, 2.55), self._rule_tokens("{A;B;C;D}(i)", (-3.9, 3.35), SMALL))
        arrow = self.mono("→", (0, 0, 0), size=SMALL, color=MUTED)
        binding.add(self._put(arrow, (-2.85, 3.35)), self._rule_tokens("{A;B;C}(i)", (-1.7, 3.35), SMALL))
        letters_row = self._put(_engine.VGroup(*[self.mono(f"{n}={v}", (0, 0, 0), size=22, color=c) for n, v, c in zip("ABCD", segments, roles_c)]).arrange(_engine.RIGHT, buff=0.4), (-2.7, 2.85))
        binding.add(letters_row)
        letters_row[-1].set_opacity(0.6)
        d_mob = _engine.VGroup(*letters_row[-1:])
        binding.add(_engine.Line(d_mob.get_left() + np.array([0, 0.02, 0]), d_mob.get_right() + np.array([0, 0.02, 0]), color=PINK, stroke_width=1.6))
        before = self._path_parts(segments, index21, roles_c)
        after = self._path_parts(j21[0], j21[1], roles_c)
        row = self._tokens([*before, ("→", MUTED), *after], (-2.7, 2.3), size=20)
        binding.add(row, self._say(self.tr("Facade items join in order:", "Fassadenelemente werden der Reihe nach"), (-2.7, 1.8), size=SMALL, color=INK),
                    self._say(self.tr(f"index {index21} becomes {j21[1]}.", f"verbunden: Index {index21} wird {j21[1]}."), (-2.7, 1.5), size=SMALL, color=INK))
        eight = len(storey_tree.branch(j21[0]))
        example_tower = self._tower(tint=lambda key: GOOD if key[:3] == group21 else None)
        letter_flight = self._fly("letters", {str(n): letters_row[n].copy().scale(3).move_to(self._centre(example_tower, segments[:n + 1])) for n in range(4)}, {str(n): letters_row[n].copy() for n in range(4)})
        self.step("mapper_example", letter_flight, self._swap("pipe", binding), self._swap("diag", self._diagram(self._tree, FULL, lit=[group21])), self._swap("tower", example_tower), self._swap("thread", self._thread(self._tree)),
                  self._swap("tags", self._chips((self.tr(f"tower {tower_key}, storey {segments[2]}: {width(eight)} capsules", f"Turm {tower_key}, Geschoss {segments[2]}: {width(eight)} Kapseln"), GOOD))), seconds=2.6)

        map_pipe = self._pipe("Path Mapper", storey_tree, mapping="{A;B;C;D}(i)->{A;B;C}(i)")
        map_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B;C}(i)", (-3.1, 1.17)))
        storey_diag = self._diagram(storey_tree, (0, 1, 2), big=True)
        self.step("mapper", self._regroup("regroup-mapper", self._slots.get("diag"), storey_diag), self._swap("pipe", map_pipe), self._swap("diag", storey_diag), self._swap("head", self._head(self._count_text(storey_tree))),
                  self._swap("thread", self._thread(storey_tree)), self._swap("tower", plain()), self._swap("tags", None), seconds=1.6)

        examples = (storey_tree.paths()[0], sample_storey, storey_tree.paths()[-1])
        sizes = {path: len(storey_tree.branch(path)) for path in examples}
        names_s = {path: (f"{towers[path[1]].key}, level {path[2]}", f"{towers[path[1]].key}, Ebene {path[2]}") for path in examples}
        def storey_stage(prefixes):
            chips = self._chips(*[(self.tr(f"{names_s[p][0]}: {width(sizes[p])} capsules", f"{names_s[p][1]}: {width(sizes[p])} Kapseln"), GOOD) for p in prefixes])
            drawing = self._tower(tint=lambda key: GOOD if any(key[:3] == p for p in prefixes) else None)
            pulls = [self._gather(f"gather-storey-{j}", drawing, [k for k in drawing.capsules if k[:3] == p], chips[len(prefixes) - 1 - j]) for j, p in enumerate(prefixes)]
            return _engine.AnimationGroup(self._swap("tower", drawing), self._swap("tags", chips), *pulls)
        self._staged_step("perstorey", [lambda prefixes=prefixes: storey_stage(prefixes) for prefixes in ((examples[0],), (examples[-1],), (examples[1], examples[-1]))], seconds=4.0)

        tower_pipe = self._pipe("Path Mapper", tower_tree, count=2, mapping="{A;B;C;D}(i)->{A;B}(i)")
        tower_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B}(i)", (-3.1, 1.17)))
        def tower_stage(group):
            chips = self._chips(*[(self.tr(f"tower {towers[i].key}: {width(per_tower[i])} capsules", f"Turm {towers[i].key}: {width(per_tower[i])} Kapseln"), TOWER_COLORS[i]) for i in group])
            drawing = self._tower(tint=lambda key: TOWER_COLORS[key[1]] if key[1] in group else None)
            pulls = [self._gather(f"gather-tower-{j}", drawing, [k for k in drawing.capsules if k[1] == i], chips[len(group) - 1 - j]) for j, i in enumerate(group)]
            return _engine.AnimationGroup(self._swap("tower", drawing), self._swap("tags", chips), *pulls)
        tower_diag = self._diagram(tower_tree, (0, 1), big=True)
        self._staged_step("pertower", [lambda group=group: tower_stage(group) for group in ((0,), (1,), (0, 1))], seconds=3.2,
                          first=[self._regroup("regroup-pertower", self._slots.get("diag"), tower_diag), self._swap("pipe", tower_pipe), self._swap("diag", tower_diag),
                                 self._swap("head", self._head(self._count_text(tower_tree))), self._swap("thread", self._thread(tower_tree))])

        item_tone = (ITEM_C, PEACH)
        side_tower = self._tower(tint=lambda key: item_tone[branch_positions[key] % len(item_tone)])
        before_diag = self._diagram(self._tree, FULL, leaf_color=lambda path, index: item_tone[index % len(item_tone)])
        after_diag = self._diagram(flip_tree, (5,), leaf_color=lambda path, index: item_tone[path[0] % len(item_tone)], big=True)
        flip_pipe = self._pipe("Flip Matrix", flip_tree)
        positions = [sum(item is not None for item in flip_tree.branch(path)) for path in flip_tree.paths()]
        chips = self._chips(*[(self.tr(f"index {i}: {count} capsules", f"Index {i}: {count} Kapseln"), item_tone[i % len(item_tone)]) for i, count in enumerate(positions)])
        def flip_first():
            return _engine.AnimationGroup(self._swap("tower", side_tower), self._swap("diag", before_diag), self._swap("head", self._head(self._count_text(self._tree))),
                                          self._swap("tags", chips), self._swap("pipe", None))
        def flip_second():
            return _engine.AnimationGroup(self._regroup("regroup-flip", before_diag, after_diag), self._swap("pipe", flip_pipe), self._swap("diag", after_diag),
                                          self._swap("head", self._head(self.tr(f"ragged → {flip_tree.path_count} × {self._tree.path_count}, nulls", f"unregelmäßig → {flip_tree.path_count} × {self._tree.path_count}, Nullwerte"))),
                                          self._swap("thread", self._thread(flip_tree)))
        self._staged_step("flip", [flip_first, flip_second], seconds=3.6)

        words = (self.tr("teal", "türkis"), self.tr("orange", "orange"))
        preview = self.gh_component("Custom Preview", at=(-2.9, 2.4))
        colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-5.25, 1.42), max_lines=2, width=1.6)
        swatches = self._swatches(colors, (ARCH, WARN))
        match_pipe = _engine.VGroup(preview, colors, swatches, self.gh_wire(self._src.grip_out(), preview.grip_in(0), kind="tree"), self.gh_wire(colors.grip_out(), preview.grip_in(1), kind="list"))
        intent = self._chips((self.tr("tower A: teal", "Turm A: türkis"), ARCH), (self.tr("tower B: orange", "Turm B: orange"), WARN))
        self.step("match", self._swap("pipe", match_pipe), self._swap("diag", self._diagram(self._tree, FULL)), self._swap("tower", plain()),
                  self._swap("head", self._head(self._count_text(self._tree))), self._swap("thread", self._thread(self._tree)), self._swap("tags", intent), seconds=1.6)

        stripes = self._tower(tint=lambda key: MATERIALS[paint["cp1"][key]])
        wrong_counts = {colour: sum(value == colour for value in paint["cp1"].values()) for colour in ("teal", "orange")}
        warn_note = self._chips((self.tr(f"no error: {wrong_counts['teal']} teal, {wrong_counts['orange']} orange", f"kein Fehler: {wrong_counts['teal']} türkis, {wrong_counts['orange']} orange"), INK))
        mistake_diag = self._diagram(self._tree, FULL, leaf_color=lambda path, index: MATERIALS[paint["cp1"][tuple(self._tree.branch(path)[index].tag["slot"])]])
        self._staged_step("mistake", [lambda: self._paint("paint-mistake", stripes, {"teal": swatches[0], "orange": swatches[1]}, paint["cp1"]),
                                       lambda: _engine.AnimationGroup(self._swap("tower", stripes), self._swap("tags", warn_note), self._swap("diag", mistake_diag))], seconds=2.4)

        viewer_a = self.gh_param_viewer(_model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:3]}), at=(-3.95, 2.4), max_items=2)
        viewer_b = self.gh_param_viewer(_model.Tree({(0,): [None, None]}), at=(-1.6, 2.4), max_items=2)
        count_a = self._tree.stats()["path_count"]
        paths_plate = _engine.VGroup(self._plate((-2.85, 1.2), 4.8, 0.44),
                                     self._tokens([*self._path_parts(self._tree.paths()[0], 0, ROLE_COLORS)[:-1], (f" ×{count_a}", MUTED)], (-4.2, 1.2), size=SMALL),
                                     self._tokens([("{", MUTED), ("0", INK), ("}", MUTED), (" ×1", MUTED)], (-1.5, 1.2), size=SMALL))
        mini = lambda tint, x: _snapshot.capture(self.tower_axon(**_canvas.building_fit(1.8, 2.5, centre=(x, -1.15, 0.0)), view="axon", tint=tint, types=False), self.camera)
        compare = _engine.Group(mini(lambda key: TOWER_COLORS[key[1]], -5.2), mini(lambda key: MATERIALS[paint["cp1"][key]], -3.2),
                                 self._say(self.tr("expected", "erwartet"), (-5.2, 0.42), size=SMALL, color=INK), self._say(self.tr("actual", "tatsächlich"), (-3.2, 0.42), size=SMALL, color=INK))
        short_list = self._fly("short-list", [paths_plate[2].copy(), paths_plate[2].copy()], [paths_plate[2].copy().move_to(paths_plate[1].get_center()) for _ in range(2)])
        self.step("why", short_list, self._swap("pipe", _engine.VGroup(viewer_a, viewer_b, paths_plate)), self._swap("diag", compare), self._swap("head", None),
                  self._swap("tags", self._chips((self.tr("different paths", "verschiedene Pfade"), SERV))), seconds=2.4)

        fix_map = self.gh_component("Path Mapper", at=(-2.9, 2.85), scale=0.8)
        fix_text = self.gh_panel(None, rows=[("line", "", "{A;B;C;D}(i)"), ("line", "", "->{A;B}(i)")], at=(-5.7, 1.38), max_lines=2, max_width=1.9)
        fix_colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-3.8, 1.4), max_lines=2, width=1.6)
        fix_swatches = self._swatches(fix_colors, (ARCH, WARN))
        fix_graft = self.gh_component("Graft Tree", at=(-2.25, 1.4), scale=0.8)
        fix_preview = self.gh_component("Custom Preview", at=(-0.95, 2.5), scale=0.8)
        first_part = _engine.VGroup(fix_map, fix_text, self.gh_wire(self._src.grip_out(), fix_map.grip_in(0), kind="tree"), self.gh_wire(fix_text.grip_out(), fix_map.grip_in(1), kind="item"))
        second_part = _engine.VGroup(fix_colors, fix_swatches, fix_graft, fix_preview, self.gh_wire(fix_colors.grip_out(), fix_graft.grip_in(0), kind="list"),
                                     self.gh_wire(fix_map.grip_out(0), fix_preview.grip_in(0), kind="tree"), self.gh_wire(fix_graft.grip_out(0), fix_preview.grip_in(1), kind="tree"))
        fix_diag = self._diagram(tower_tree, (0, 1), big=True, leaf_color=lambda path, index: MATERIALS[paint["cp2"][tuple(tower_tree.branch(path)[index].tag["slot"])]])
        mapped_diag = self._diagram(tower_tree, (0, 1), big=True)
        dim_towers = self._tower(tint=lambda key: dim(TOWER_COLORS[key[1]]))
        def fix_first():
            animation = _engine.AnimationGroup(self._swap("pipe", first_part), self._swap("diag", mapped_diag), self._swap("tower", dim_towers), self._swap("tags", None),
                                               self._swap("head", self._head(self.tr("{0;0} and {0;1}", "{0;0} und {0;1}"))), self._swap("thread", self._thread(tower_tree)))
            self._slots["pipe"] = _engine.VGroup(first_part, second_part)
            return animation
        def fix_second():
            return _engine.AnimationGroup(_engine.FadeIn(second_part), self._swap("tags", self._chips((self.tr("equal paths", "gleiche Pfade"), GOOD))),
                                           self._swap("head", self._head(self.tr("{0;0} meets {0;0} · {0;1} meets {0;1}", "{0;0} trifft {0;0} · {0;1} trifft {0;1}"))))
        self._anchors = {"diag": fix_diag, "Branch": fix_colors, "Index": fix_colors.parts["rows"][0], "Graft": fix_graft, "Path Mapper": fix_map}
        self._staged_step("fix", [fix_first, fix_second, lambda: self._fan("fan-fix", {"teal": fix_swatches[0], "orange": fix_swatches[1]}, fix_diag, paint["cp2"]),
                                  lambda: self._swap("diag", fix_diag)], seconds=3.2, weights=(.5, 1/6, 1/6, 1/6))

        counts = {name: sum(1 for value in paint["cp2"].values() if value == name) for name in ("teal", "orange")}
        done = self._tower(tint=lambda key: MATERIALS[paint["cp2"][key]])
        result_tags = self._chips((self.tr(f"tower A: {width(counts['teal'])} teal", f"Turm A: {width(counts['teal'])} türkis"), ARCH),
                                  (self.tr(f"tower B: {width(counts['orange'])} orange", f"Turm B: {width(counts['orange'])} orange"), WARN))
        self._staged_step("result", [lambda: self._paint("paint-result", done, {"teal": fix_swatches[0], "orange": fix_swatches[1]}, paint["cp2"]),
                                      lambda: _engine.AnimationGroup(self._swap("tower", done), self._swap("tags", result_tags))], seconds=2.4)

        chip = self._slots["thread"]
        home = self._fly("home-probe", chip.copy(), self._slots["tower"].capsules[self._probe].copy())
        gone, legend = self._recap()
        self.step("recap", home, _engine.AnimationGroup(*gone, legend), seconds=3.6)

    def _recap_operation_text(self, source, result, address):
        """🔢 Actual branch change and invariant item count for a recap operation."""
        return self.tr(f"{source.path_count}→{result.path_count} branches · {result.count} items: {address}",
                       f"{source.path_count}→{result.path_count} Äste · {result.count} Elemente: {address}")

    def _recap_result_label(self, text, y):
        """🏷️ Fit a recap result within its column without moving the left edge."""
        label = self._mono(text, (-2.15, y), size=SMALL, color=DATA, edge="l")
        if label.width > 4.8:
            label.scale_to_fit_width(4.8, about_edge=_engine.LEFT)
        return label

    def _recap(self):
        """📚 The ten terms as a legend beside the tower, each with a mini tree computed by the engine; capsule 21 and its address stay on screen."""
        keep = {self._slots["tower"], self._caption}
        gone = [_engine.FadeOut(mob) for mob in self.mobjects if mob not in keep]
        drawing, chip = self._slots["tower"], self._slots["thread"]
        origins = {"Tree": self._anchors["diag"], "Flatten": self._anchors["diag"], "Simplify": self._anchors["diag"][0], "Path": chip, "Item": drawing.capsules[self._probe], "Flip Matrix": drawing._drawing_ref.value,
                   **{name: self._anchors[name] for name in ("Branch", "Index", "Graft", "Path Mapper")}}
        base = _model.Tree.from_dict({"{0;0}": ["a", "b"], "{0;1}": ["c", "d"]})
        tower = self._tree
        found = {name: self._locate(tree) for name, tree in self._results.items()}
        text = lambda name: f"{_model.format_path(found[name][0])}[{found[name][1]}]"
        rows = [
            (("Tree", "Datenbaum"), base, self.tr(f"{tower.path_count} branches · {tower.count} items", f"{tower.path_count} Äste · {tower.count} Elemente")),
            (("Branch", "Ast"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), self.tr("variable items per facade group", "variable Elemente je Fassadengruppe")),
            (("Path", "Pfad"), _model.Tree.from_dict({"{0;1}": ["a"]}), _model.format_path(found["src"][0])),
            (("Item", "Element"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), self.tr("capsule 21", "Kapsel 21")),
            (("Index", "Index"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), f"[{found['src'][1]}]"),
            (("Flatten", "Flatten"), base.flatten(), self._recap_operation_text(tower, self._results["fl"], text("fl"))),
            (("Graft", "Graft"), base.graft(), self._recap_operation_text(tower, self._results["gr"], text("gr"))),
            (("Simplify", "Simplify"), _model.Tree.from_dict({"{0;0;0}": ["a", "b"], "{0;0;1}": ["c", "d"]}).simplify(), text("sm")),
            (("Path Mapper", "Path Mapper"), base.map("{A;B}(i) -> {A}"), f"{text('pm')} · {text('pt')}"),
            (("Flip Matrix", "Flip Matrix"), base.flip(), f"null: {text('fp')}"),
        ]
        flights = []
        for number, (term, tree, result) in enumerate(rows):
            y = 3.45 - 0.62 * number
            label = self._word(term) + ("" if term[0] == term[1] else " (" + self._word(term[::-1]) + ")")
            result_label = self._recap_result_label(result, y) if term[0] in ("Flatten", "Graft") else self._mono(result, (-2.15, y), size=SMALL, color=DATA, edge="l")
            row = _engine.VGroup(self._say(label, (HEAD_X, y), size=SMALL, color=INK, edge="l"), self._mini(tree, (-2.75, y), 0.62), result_label)
            source = origins[term[0]]
            flier = _sprites.capture(source, self.camera) if term[0] == "Flip Matrix" else source.copy()
            flight = self._fly("recap-" + term[0].lower().replace(" ", "-"), flier, row[0].copy())
            flights.append(_engine.Succession(flight, _engine.FadeIn(row)))
        path21, index21 = found["src"]
        tag = self._tag(self.tr(f"capsule {PROBE} = ", f"Kapsel {PROBE} = ") + f"{_model.format_path(path21)}[{index21}]", (6.8, 3.65), DATA, "r")
        return gone, _engine.AnimationGroup(_engine.LaggedStart(*flights, lag_ratio=0.18), _engine.FadeIn(tag))

    def _mini(self, tree, point, scale=1.0):
        """🌱 A small tree sketch of an engine tree: trunk, branches and leaves."""
        paths = tree.paths()
        depth = len(paths[0])
        leaves = [(p, i) for p in paths for i in range(len(tree.branch(p)))]
        count = len(leaves)
        xs = np.linspace(-0.55, 0.55, count) * scale if count > 1 else np.array([0.0])
        ys = np.linspace(-0.42, 0.42, depth + 2) * scale
        prefix_x = {}
        for x, (path, _) in zip(xs, leaves):
            for level in range(1, depth + 1):
                prefix_x.setdefault(path[:level], []).append(x)
        node = lambda prefix: np.array([point[0] + (0.0 if not prefix else sum(prefix_x[prefix]) / len(prefix_x[prefix])), point[1] + ys[len(prefix)], 0.0])
        pairs, group = [], _engine.VGroup()
        for prefix in prefix_x:
            pairs.append((node(prefix[:-1]), node(prefix)))
        for x, (path, _) in zip(xs, leaves):
            top = np.array([point[0] + x, point[1] + ys[depth + 1], 0.0])
            pairs.append((node(path), top))
            group.add(_engine.Dot(top, radius=0.045, color=ITEM_C))
        return _engine.VGroup(_draft.segments(pairs, INK, 1.6), group)
