"""🫗 Casting on the Nakagin Capsule Tower: every Grasshopper parameter has a type, and a wire between different types casts the data, silently, or fails."""

import importlib
import math
from dataclasses import replace
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
_live_scene = importlib.import_module("🎓️teaching.🧰️interop.🎬️scene.🐍️")
_keyed = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️keyed")
_draft = _visuals._drafting
_canvas = _visuals._canvas
_art = _visuals._art
_mn = _visuals._engine

INK, MUTED, ARCH, DATA, GOOD, WARN, SERV = _visuals.INK, _visuals.MUTED, _visuals.ARCH, _visuals.DATA, _visuals.GOOD, _visuals.WARN, _visuals.SERV
BACKGROUND = _draft.BACKGROUND
SMALL = 16.2
SHEET_AT, SHEET_SIZE = np.array([-3.4, 0.05, 0.0]), (6.8, 4.9)
PORT_AT, PORT_SIZE = np.array([3.5, 0.05, 0.0]), (6.6, 4.9)
SLOT = (0, 0, 0, 3, 0)
DARK = "#1B2A30"
INK_GOLD, INK_ORANGE, INK_TEAL, INK_GREEN, INK_RED, INK_GREY = "#6B4F00", "#7A4300", "#0B5A51", "#1E5A24", "#A11F18", "#3C403E"
CAP_GOOD, CAP_BAD = "#2E8B57", "#C23B3B"
LABEL_INK = {"X": "#A5122C", "Y": "#05563B", "Z": "#173F9A"}
SOURCE = "D. Rutten, McNeel forum 2010"
STOREYS = _nakagin.towers()[1].capsule_storeys
SLIDER = STOREYS - 0.4
LABELS_DE = {"Slot origin": "Slot-Ursprung", "Core to slot": "Kern zu Slot", "Capsule body": "Kapselkörper", "Capsule": "Kapsel", "Window": "Fenster", "Length": "Länge", "Height": "Höhe",
             "Width": "Breite", "Storeys": "Geschosse", "Type A": "Typ A", "Type B": "Typ B", "Type C": "Typ C", "Type D": "Typ D", "X-Axis": "X-Achse", "Y-Axis": "Y-Achse", "Plane at": "Ebene bei"}
RAW_PANELS = ("t25", "typa", "tde")
FAMILIES = (("numbers", "Zahlen", ("Number", "Integer")), ("truth", "Wahrheit", ("Boolean",)), ("text", "Text", ("Text",)), ("geometry", "Geometrie", ("Point", "Line", "Circle", "Box")))
COLUMN_FAMILIES = (("numbers", "Zahlen", ("Number", "Integer")), ("truth", "Wahrheit", ("Boolean",)), ("text", "Text", ("Text",)), ("geometry", "Geometrie", ("Domain", "Plane", "Curve", "Brep")))
CELLS = (("Text", "Number", ("Text>Number", "Text>Number!"), (("2.5", "2.5"), ("Type A", "Typ A"))), ("Number", "Integer", ("Number>Integer",), (f"{SLIDER:g} → {STOREYS}", f"{SLIDER:g}".replace(".", ",") + f" → {STOREYS}")),
         ("Point", "Plane", ("Point>Plane",), ("world XY", "Welt-XY")), ("Line", "Plane", ("Line>Plane",), ("Z = line", "Z = Linie")),
         ("Number", "Domain", ("Number>Domain",), ("0 To 4", "0 To 4")), ("Circle", "Curve", ("Circle>Curve",), ("outline", "Rand")),
         ("Box", "Brep", ("Box>Brep",), ("6 faces", "6 Flächen")), ("Boolean", "Integer", ("Boolean>Integer",), ("True → 1", "True → 1")),
         ("Number", "Boolean", ("Number>Boolean",), ("0 → False", "0 → False")))


def _num(value: float) -> str:
    """🔡 A Grasshopper Panel number: shortest decimal, dot separator."""
    return f"{value:.2f}".rstrip("0").rstrip(".") if abs(value) >= 0.005 or value == 0 else f"{value:g}"


def _lines(tree) -> list:
    """📝 Panel rows for every item of a tree; planes take four short rows, an empty tree reads null."""
    out = []
    for item in tree.items():
        if isinstance(item, _model.Plane):
            out += ["Plane at", "x " + _num(item.origin.x), "y " + _num(item.origin.y), "z " + _num(item.origin.z)]
        elif isinstance(item, _model.Box):
            out += ["Box", " × ".join(_num(d.hi - d.lo) for d in (item.x, item.y, item.z))]
        elif isinstance(item, float):
            out.append(_num(item))
        else:
            out.append(_model.describe(item))
    return out or ["null"]


def _plane(engine_plane):
    """🧭 A Nakagin plane from an engine plane."""
    return _nakagin.Plane(origin=engine_plane.origin.t, x=engine_plane.x.t, y=engine_plane.y.t, z=engine_plane.z.t)


class _dark_dimensions:
    """🌑 Darken the Nakagin drawing's dimension and highlight ink while a scene draws on the light Rhino viewport."""

    def __enter__(self):
        """🌘 Swap the gold ink for a dark gold in module constants and default arguments."""
        draw = _draw.NakaginDrawing
        self.saved = (_draw.DATA, draw._text.__kwdefaults__["color"], draw._away.__defaults__)
        _draw.DATA = INK_GOLD
        draw._text.__kwdefaults__["color"] = INK_GOLD
        draw._away.__defaults__ = (INK_GOLD,)

    def __exit__(self, *error):
        """🌕 Restore the original ink."""
        draw = _draw.NakaginDrawing
        _draw.DATA, draw._text.__kwdefaults__["color"], draw._away.__defaults__ = self.saved


def _gone(*mobs):
    """🫥 Stop live redraws and fade mobjects out."""
    for mob in mobs:
        for part in mob.get_family():
            part.clear_updaters()
    fades = [_mn.FadeOut(mob) for mob in mobs]
    for fade in fades:
        fade.rate_func = lambda t: _mn.smooth(min(t / 0.4, 1.0))
    return fades


class Beat4_Casting(_live_scene.RefreshMoving, _visuals.ClipScene):
    """🎓 Teach implicit type casting on the Nakagin Capsule Tower with explicit English and German narration."""

    TITLE = {"en": "Casting: When Types Meet", "de": "Casting: Wenn Typen sich treffen"}
    NARRATION = [
        ["teaser", "This capsule should face west, but its window looks north. One silent cast put it there. Which one?",
         "Diese Kapsel soll nach Westen schauen, doch ihr Fenster zeigt nach Norden. Eine stille Umwandlung hat das bewirkt. Welche?"],
        ["types", "In this teaching model of the Nakagin Capsule Tower, every parameter has a type: width is a Number, storeys an Integer, placement a Plane.",
         "In diesem Lehrmodell des Nakagin Capsule Tower hat jeder Parameter einen Typ: Breite Zahl, Geschosse Ganzzahl, Platzierung Ebene."],
        ["mismatch", "A wire connects whatever you drag. When the types differ, Grasshopper tries to cast the data.",
         "Eine Verbindung nimmt alles an, was man zieht. Passen die Typen nicht, versucht Grasshopper, die Daten umzuwandeln."],
        ["text_number", "A Panel holds the text 2.5. Number reads it as 2.5: the nominal width is 2.5 metres, the physical casing 2.6 metres.",
         "Ein Panel enthält den Text 2.5. Number liest ihn als 2,5: Die nominale Breite ist 2,5 Meter, das physische Gehäuse 2,6 Meter."],
        ["storeys", f"A slider says {SLIDER:g}, but Storeys wants an Integer. Grasshopper rounds, here to {STOREYS}; check this in Rhino 8.",
         f"Der Regler zeigt {SLIDER:g}".replace(".", ",") + f", doch Geschosse verlangt eine Ganzzahl. Grasshopper rundet, hier auf {STOREYS}; in Rhino 8 prüfen."],
        ["ties", "Grasshopper rounds; at exactly .5 it rounds to the even number: 2.5 gives 2, 3.5 gives 4. Check this in Rhino 8.",
         "Grasshopper rundet; genau bei .5 geht es zur geraden Zahl: 2,5 gibt 2, 3,5 gibt 4. In Rhino 8 prüfen."],
        ["deliberate", "To decide the rounding yourself, put a component such as Floor before the Integer input: the top storey disappears.",
         "Wer die Rundung selbst bestimmen will, setzt eine Komponente wie Floor vor den Ganzzahl-Eingang: Das oberste Geschoss verschwindet."],
        ["point_plane", "A Point at a Plane input becomes a plane at that point with the world axes. Here the point is the origin of a facade slot.",
         "Ein Punkt an einem Ebenen-Eingang wird zur Ebene an diesem Punkt mit den Weltachsen. Hier ist er der Ursprung eines Fassadenplatzes."],
        ["wrong_way", "On the west facade the capsule must face west. The world plane points it north: the window turns by 90°.",
         "An der Westfassade muss die Kapsel nach Westen schauen. Die Weltebene richtet sie nach Norden aus: Das Fenster dreht um 90°."],
        ["line_plane", "A Line becomes a plane whose Z axis follows the line. From the core to the slot Z points outwards, but X and Y are still guessed.",
         "Eine Linie wird zur Ebene, deren Z-Achse der Linie folgt. Vom Kern zum Platz zeigt Z nach außen, doch X und Y sind nur geraten."],
        ["explicit", "Build the plane explicitly from origin and axes. The capsule turns into place and faces outwards.",
         "Die Ebene wird ausdrücklich aus Ursprung und Achsen gebaut: Die Kapsel dreht sich an ihren Platz und schaut nach außen."],
        ["domain", "A Number can stand in for a Domain: the nominal study length 4.0 becomes the interval 0 To 4, and Divide Domain cuts it.",
         "Eine Zahl kann eine Domain vertreten: Die Kapsellänge 4,0 wird zum Intervall 0 To 4, und Divide Domain teilt es."],
        ["geometry", "A Circle is a Curve and a Box is a Brep: window outline and capsule body pass as the more general type.",
         "Ein Kreis ist eine Kurve, eine Box ein Brep: Fensterrand und Kapselkörper gehen als der allgemeinere Typ durch."],
        ["bool_int", "A Boolean Toggle becomes an Integer: False is 0, True is 1, and the window follows.",
         "Ein Boolean Toggle wird zur Ganzzahl: False ist 0, True ist 1, und das Fenster folgt."],
        ["int_bool", "The other way round, a number can switch the window: 0 hides it, any other value shows it.",
         "Umgekehrt kann eine Zahl das Fenster schalten: 0 blendet es aus, jeder andere Wert blendet es ein."],
        ["locale", "Mind the decimal comma: the text 2,5 may fail as a Number in a German setup. Write 2.5 or use a slider.",
         "Achtung Dezimalkomma: Der Text 2,5 kann als Zahl scheitern. Besser 2.5 schreiben oder einen Regler nehmen."],
        ["fail", "Now a Panel holds the text Type A. The Number input cannot read it: the component turns red, Data conversion failed from Text to Number.",
         "Ein Panel mit Typ A: Der Zahleneingang scheitert, die Komponente wird rot und meldet „Data conversion failed from Text to Number“."],
        ["order", "Why? Grasshopper tries in a fixed order: same type, then the source, then the target. Type A fails every step, so the slot stays blank.",
         "Warum? Grasshopper probiert in fester Reihenfolge: gleicher Typ, dann die Quelle, dann das Ziel. Typ A scheitert überall, der Platz bleibt leer."],
        ["fix", "A label is no number. A Value List delivers the real width of Type A, and the component turns grey again.",
         "Eine Bezeichnung ist keine Zahl. Eine Werteliste liefert die echte Breite von Typ A, und die Komponente wird wieder grau."],
        ["matrix", "Casts follow families: along a family they work, across families you need a component.",
         "Umwandlungen folgen Familien: Innerhalb einer Familie klappen sie, über Familien hinweg braucht es eine Komponente."],
        ["matrix_fill", "Each marked cell is a cast you have seen: green worked, red failed, dashed outlines still need a check in Rhino 8.",
         "Jede markierte Zelle ist eine gesehene Umwandlung: Grün klappte, Rot scheiterte, gestrichelte Umrisse sind in Rhino 8 noch zu prüfen."],
        ["recap", "Three habits: check with a Panel, build planes explicitly, and never feed labels into a Number input.",
         "Drei Gewohnheiten: mit einem Panel prüfen, Ebenen ausdrücklich bauen und Bezeichnungen nie in einen Zahleneingang geben."],
    ]

    def construct(self):
        """▶️ Present this chapter through the native Grasshopper visual interface."""
        self.present("casting")

    def then(self, after, after_seconds, seconds, *more):
        """⏭️ Queue the morphs that follow the next clause's first animations (``more``: further (animations, seconds) phases); returns their duration."""
        self._queued = [(after, after_seconds), *more]
        return seconds

    def step(self, key, *animations, seconds=1.2):
        """⏱️ A clause whose morph follows the first animations: the queued ``then`` plays before the narration hold."""
        phases = self.__dict__.pop("_queued", [])
        if not phases:
            return super().step(key, *animations, seconds=seconds)
        row = next(row for row in self.NARRATION if row[0] == key)
        caption = row[1] if self.language() == "en" else row[2]
        self._caption = _visuals._art.swap_caption(self, self._caption, caption)
        self.play(*animations, run_time=seconds)
        for after, after_seconds in phases:
            self.play(*after, run_time=after_seconds)
        _visuals._art.hold_for(self, [(code, en, en if self.language() == "en" else de) for code, en, de in self.NARRATION], key)

    def _route(self, chip, target):
        """🛤️ A curved flight path from a chip to the centre of a target shape."""
        start, end = np.array(chip.get_center()), np.array(target.get_center())
        return [start, (start + end) / 2 + np.array([0.0, 0.9, 0.0]), end]

    def _early(self, animation, end):
        """⏩ Finish an animation early inside its run: it completes at ``end`` (0 to 1) of the run time."""
        animation.rate_func = lambda t: _mn.smooth(min(t / end, 1.0))
        return animation

    def _later(self, animation, start):
        """⏳ Delay an animation inside its run: it idles until ``start`` (0 to 1) of the run time."""
        animation.rate_func = lambda t: _mn.smooth(min(max((t - start) / (1 - start), 0.0), 1.0))
        return animation

    def _tick_at(self, storey):
        """📍 Screen point of the numbered tick of a Tower B storey, left of its band."""
        self._tower(None)
        tower = _nakagin.towers()[1]
        z = tower.base_elevation + (storey - 0.5) * tower.storey_height
        return self._towers[None].at(tower.center[0] - tower.core_side / 2 - 4.6, 0.0, z) + np.array([0.12, 0.0, 0.0])

    def _band(self, first, last, color, opacity=0.34):
        """▮ A translucent hull around Tower B that covers storeys ``first`` to ``last`` (the whole tower body, not a side strip)."""
        self._tower(None)
        tower = _nakagin.towers()[1]
        half = tower.core_side / 2 + 2.9
        lo, hi = tower.base_elevation + (first - 1) * tower.storey_height, tower.base_elevation + last * tower.storey_height
        points = [self._towers[None].at(tower.center[0] + sx * half, sy * half, z)[:2] for z in (lo, hi) for sx in (-1, 1) for sy in (-1, 1)]
        hull = _mn.ConvexHull(*[np.array([x, y, 0.0]) for x, y in points], stroke_color=color, stroke_width=2.4, fill_color=color, fill_opacity=opacity)
        return hull

    def _lit(self, n):
        """💡 Tower B lit up to ``n`` storeys as a band over the static tower, with count tag and numbered ticks."""
        return _mn.VGroup(self._band(1, n, INK_GOLD), self._count_tag(n), self._ticks(n))

    def _changing_display(self, build, key):
        """🎞️ Stable display for a continuously changing authored view."""
        return _keyed.create(build, key, self.camera)

    def _lit_display(self, count):
        """💡 Stable changing display of the current storey band and label."""
        return _keyed.create(lambda: self._lit(count()), count, self.camera)

    def _sol(self, **overrides):
        """🧪 Evaluate the casting canvas with overrides, cached per override set."""
        key = tuple(sorted((k, repr(v)) for k, v in overrides.items()))
        if key not in self._cache:
            self._cache[key] = _model.evaluate(self._d, overrides)
        return self._cache[key]

    def _pn(self, rows, raw=False):
        """🔤 Panel rows with the decimal separator of the narration language; raw text stays untouched."""
        return list(rows) if raw or self.language() == "en" else [LABELS_DE.get(row, row.replace(".", ",")) for row in rows]

    def _lb(self, text):
        """🏷️ A user-chosen label in the narration language; built-in Grasshopper names stay English."""
        return text if text is None or self.language() == "en" else LABELS_DE.get(text, text)

    def _tag(self, string, point, color):
        """🔖 A label on a light backing plate so it stays readable over drawings."""
        label = self.text(string, point, size=SMALL, color=color)
        plate = _mn.RoundedRectangle(corner_radius=0.05, width=label.width + 0.14, height=label.height + 0.1, stroke_width=0, fill_color="#F2F2F2", fill_opacity=0.9).move_to(label)
        return _mn.VGroup(plate, label)

    def _dashed(self, widget, pad=0.1):
        """▭ A dashed outline around a widget: the one small marker for a rule that still needs a Rhino 8 check."""
        box = _mn.Rectangle(width=widget.width + 2 * pad, height=widget.height + 2 * pad, stroke_color=INK_ORANGE, stroke_width=2.6)
        return _mn.DashedVMobject(box, num_dashes=36).move_to(widget)

    def _flags(self, canvas, ids, rule, source=False):
        """🚩 Dashed outlines around the named widgets and one footnote, only when the engine rule is not verified."""
        group = _mn.VGroup()
        if _model.rules()[rule]["basis"] == "verified":
            return group
        for oid in ids:
            group.add(self._dashed(canvas.w[oid]))
        sample = _mn.DashedVMobject(_mn.Rectangle(width=0.42, height=0.22, stroke_color=INK_ORANGE, stroke_width=2.4), num_dashes=14)
        text = self.text(self.tr("check in Rhino 8", "in Rhino 8 prüfen") + ("  ·  " + SOURCE if source else ""), (0, 0, 0), size=SMALL, color=INK_ORANGE)
        foot = _mn.VGroup(sample, text.next_to(sample, _mn.RIGHT, buff=0.12))
        foot.move_to(SHEET_AT + np.array([-SHEET_SIZE[0] / 2 + 0.2 + foot.width / 2, -2.24, 0]))
        group.add(foot)
        return group

    def _view_title(self, view):
        """🪧 Small view title in the top left corner of the canvas sheet."""
        title = self._d.views[view]["title"]
        return self._title_text(title["en"], title["de"])

    def _title_text(self, en, de):
        """📰 A free title in the top left corner of the canvas sheet."""
        return self.text(self.tr(en, de), SHEET_AT + np.array([-SHEET_SIZE[0] / 2 + 0.2, SHEET_SIZE[1] / 2 - 0.25, 0]), size=SMALL, color=_canvas.TEXT, anchor="l")

    def _widget(self, obj, sol, tracker=None):
        """🧩 One canvas widget from an engine object and its solution."""
        spec, cfg = self._d.spec(obj.id), obj.settings
        state = sol.state(obj.id)
        kind = spec.kind
        if kind == "param":
            return self.gh_param(spec.outputs[0].type, label=self._lb(cfg.get("label")), state=state)
        if kind == "slider":
            return self.gh_slider(self._lb(cfg["label"]), cfg["value"], cfg["min"], cfg["max"], decimals=0 if cfg.get("rounding") == "integer" else cfg.get("decimals", 2), tracker=tracker, width=2.6)
        if kind == "toggle":
            return self.gh_toggle(bool(cfg["value"]), name=self._lb(cfg["label"]))
        if kind == "valuelist":
            return self.gh_value_list([self._lb(i["name"]) for i in cfg["items"]], int(cfg.get("selected", 0)))
        if kind == "panel":
            return self.gh_panel([self._lb(r) for r in _lines(sol.tree(obj.id))] if obj.id in RAW_PANELS and obj.id != "t25" and obj.id != "tde" else self._pn(_lines(sol.tree(obj.id)), raw=obj.id in RAW_PANELS))
        return self.gh_component({"name": spec.name, "nickname": spec.nickname, "inputs": [{"name": p.name, "nickname": p.nickname, "type": p.type} for p in spec.inputs],
                                  "outputs": [{"name": p.name, "nickname": p.nickname, "type": p.type} for p in spec.outputs]}, state=state)

    def _grip(self, widget, side, name):
        """⚓ A grip by name, falling back to the first one."""
        try:
            return widget.grip_in(name) if side == "in" else widget.grip_out(name)
        except (KeyError, StopIteration):
            return widget.grip_in(0) if side == "in" else widget.grip_out(0)

    def _view(self, view, sol, *, trackers=None, live=None, red=(), skip=(), zoom=1.0, layout=None, dy=0.0, gap=0.32):
        """🕸️ A canvas view of the casting definition: widgets in columns, wires from the engine, states from the solution."""
        layout = layout or {}
        objs, live, trackers = tuple(replace(o, at=layout.get(o.id, o.at)) for o in self._d.view(view) if o.id not in skip), live or {}, trackers or {}
        widgets = {o.id: self._widget(o, sol, trackers.get(o.id)) for o in objs}
        for widget in widgets.values():
            widget.scale(zoom)
        xs = sorted({o.at[0] for o in objs})
        colw = {x: max(widgets[o.id].width for o in objs if o.at[0] == x) for x in xs}
        cursor, colx = 0.0, {}
        for x in xs:
            colx[x] = cursor + colw[x] / 2
            cursor += colw[x] + gap
        total = cursor - gap
        group = _mn.VGroup()
        for o in objs:
            widgets[o.id].move_to(np.array([colx[o.at[0]] - total / 2, -o.at[1] * 0.0095 * zoom, 0.0]))
        bottom = min(widgets[o.id].get_bottom()[1] for o in objs)
        upper = max(widgets[o.id].get_top()[1] for o in objs)
        shift = np.array([0.0, -(upper + bottom) / 2 + dy, 0.0])
        for o in objs:
            widgets[o.id].shift(shift + SHEET_AT)
        for o in objs:
            group.add(live[o.id](widgets[o.id].get_center()) if o.id in live else widgets[o.id])
        for w in (w for w in self._d.view_wires(view) if w.src in widgets and w.dst in widgets):
            tone = _mn.ManimColor("#D8362B") if w.dst in red else None
            group.add(self.gh_wire(self._grip(widgets[w.src], "out", w.out), self._grip(widgets[w.dst], "in", w.inp), color=tone))
        group.w = widgets
        group.total = total
        return group

    def _token(self, canvas, source, sink, u, steps):
        """🪙 A data token riding the wire from ``source`` to ``sink``: ``steps`` is a list of (until, text, type, colour) forms."""
        a = np.array(self._grip(canvas.w[source[0]], "out", source[1]).get_center())
        b = np.array(self._grip(canvas.w[sink[0]], "in", sink[1]).get_center())
        curve = _canvas.bezier(a, b)
        point = curve[min(int(u * (len(curve) - 1)), len(curve) - 1)]
        form = next(step for step in steps if u <= step[0])
        chip = self.badge(form[1], (0, 0, 0), color=form[3], size=SMALL, filled=True)
        label = self.text(form[2], (0, 0, 0), size=SMALL, color=form[3])
        group = _mn.VGroup(chip, label.next_to(chip, _mn.UP, buff=0.05)).move_to(point + np.array([0.0, 0.62, 0.0]))
        return group

    def _tower(self, n=None):
        """🗼 The whole tower drawing, Tower B lit up to ``n`` capsule storeys; cached per count."""
        if n not in self._towers:
            lit = None if n is None else (lambda key: key[1] == 1 and key[2] < n)
            self._towers[n] = self.tower_axon(**self._tower_fit, view="axon", lit=lit, types=True)
        images = self.__dict__.setdefault("_tower_images", {})
        if n not in images:
            images[n] = _snapshot.capture(self._towers[n], self.camera)
        return images[n].copy()

    def _ticks(self, n, ghost=None):
        """🔢 Numbered storey ticks beside Tower B: gold up to ``n``, a red ghost number for a storey that dropped out."""
        self._tower(None)
        drawing = self._towers[None]
        tower = _nakagin.towers()[1]
        group = _mn.VGroup()
        for storey in range(1, max(n, ghost or 0) + 1):
            z = tower.base_elevation + (storey - 0.5) * tower.storey_height
            at = drawing.at(tower.center[0] - tower.core_side / 2 - 4.6, 0.0, z) + np.array([0.12, 0.0, 0.0])
            lit = storey <= n
            group.add(self.text(str(storey), at, size=SMALL, color=INK_GOLD if lit else INK_RED, anchor="l"))
            if not lit:
                group.add(_draft.segments([(at + np.array([-0.05, -0.16, 0]), at + np.array([0.3, 0.16, 0]))], INK_RED, 2.4))
        return group

    def _count_tag(self, n):
        """🔟 Count tag above the tower."""
        return self.text(self.tr(f"Tower B: {n} capsule storeys", f"Turm B: {n} Kapselgeschosse"), PORT_AT + np.array([0, 2.2, 0]), size=SMALL, color=_canvas.TEXT)

    def _capsule(self, *, ghost=False, dims=("width",), window=True, highlight=(), hatch=(), up=0.0):
        """💊 Capsule type A in the viewport with the width taken from the nakagin model."""
        cap = self.capsule_axon(_nakagin.capsule_type("A"), at=PORT_AT + np.array([0, up + 0.15, 0]), scale=0.8, dims=dims, window=window, highlight=highlight, hatch=hatch)
        if ghost:
            cap.set_opacity(0.25)
        return cap

    def _stage(self, *, correct=True, spin=None, axes=(), line=None, arrows=False, arc=False, legend=False, ghost=False, late=False):
        """🏗️ The actual west-facing Slash capsule of Tower A in 3D: wall, the slot capsule on its slot plane, an optional capsule on the world plane (``spin`` 0 world … 1 slot) with window arrows and a 90° arc."""
        slot = next(s for s in _nakagin.slots() if (s.building, s.tower, s.storey, s.facade, s.item) == SLOT)
        origin = np.array(slot.plane.origin)
        view = _draw.make_view("axon_left", PORT_AT + np.array([0.1, -0.1, 0]), 0.5, origin + np.array([-1.0, 0.4, 1.4]))
        cap = _draw._cap(_nakagin.capsule_type(slot.type))
        group = _mn.VGroup()
        wall = [(origin[0] + 0.0, origin[1] + dy, origin[2] + dz) for dy, dz in ((-2.75, -1.4), (2.75, -1.4), (2.75, 3.4), (-2.75, 3.4))]
        group.add(_draft.polygon([view(*p) for p in wall], _draw.CAPITAL, 0.95, DARK, 1.2))

        def place(plane, colour, opacity):
            made = _draw._capsule_lite(cap, plane, view, colour, window=True, layout=slot.layout)
            made.set_opacity(opacity)
            return made

        def window_arrow(plane, colour):
            R = np.array([plane.x, plane.y, plane.z], float)
            aperture = _nakagin.capsule_window(_nakagin.capsule_type(slot.type), slot.layout)
            w = np.array(plane.origin) + np.asarray(aperture.center) @ R
            tip = w + np.asarray(aperture.normal) @ R * 1.5
            a, b = np.array(view(*w)), np.array(view(*tip))
            d = (b - a) / max(np.linalg.norm(b - a), 1e-9)
            n = np.array([-d[1], d[0], 0.0])
            return _mn.VGroup(_draft.segments([(a, b - d * 0.14)], colour, 4.0), _draft.polygon([b, b - d * 0.24 + n * 0.1, b - d * 0.24 - n * 0.1], colour, 1.0, colour, 0.5))

        if ghost:
            group.add(place(slot.plane, _draft.mix(CAP_GOOD, "#FFFFFF", 0.5), 0.35))
        if correct:
            group.add(place(slot.plane, CAP_GOOD, 0.95))
        split = len(group.submobjects)
        if spin is not None and spin >= 0.99:
            spin = None
            arc = False
        if spin is not None:
            theta = math.radians(90.0 * spin)
            moving = _nakagin.Plane(origin=slot.plane.origin, x=(math.cos(theta), math.sin(theta), 0.0), y=(-math.sin(theta), math.cos(theta), 0.0), z=(0.0, 0.0, 1.0))
            group.red, group.red_arrow = place(moving, CAP_BAD, 0.78 if late else 0.5), window_arrow(moving, CAP_BAD)
            if not late:
                group.add(group.red)
        if arrows:
            group.add(window_arrow(slot.plane, CAP_GOOD))
            if spin is not None and not late:
                group.add(group.red_arrow)
        tags = _mn.VGroup()
        if arc:
            centre = origin + np.array([0, 0, 1.25])
            points = [view(*(centre + 2.1 * (math.cos(p) * np.array([-1.0, 0, 0]) + math.sin(p) * np.array([0, 1.0, 0])))) for p in np.linspace(0, math.pi / 2 * (1 - (spin or 0)), 24)]
            if len(points) > 1:
                tags.add(_draft.polyline(points, INK_ORANGE, 3.0), self._tag(f"{self.number(90 * (1 - (spin or 0)), 0)}°", points[len(points) // 2] + np.array([0.1, 0.3, 0]), INK_ORANGE))
            tags.add(self._tag("W", view(*(centre + np.array([-2.5, 0, 0]))), INK_GREY), self._tag("N", view(*(centre + np.array([0, 2.6, 0]))), INK_GREY))
        group.tags = tags
        if not late:
            group.add(tags)
        for engine_plane in axes:
            plane = _plane(engine_plane)
            group.add(self.plane_axes(plane, at=view(*plane.origin), scale=0.5, view=view, length=1.8))
        if line is not None:
            a, b = np.array(line.a.t), np.array(line.b.t)
            group.add(_draft.segments([(view(*a), view(*b))], _draw.AXIS_Z, 3.2), _mn.Dot(view(*b), radius=0.09, color=_draw.AXIS_Z))
        if legend:
            group.add(_mn.VGroup(self.badge(self.tr("slot plane", "Platzebene"), (0, 0, 0), color=CAP_GOOD, size=SMALL, filled=True),
                                 self.badge(self.tr("world plane", "Weltebene"), (0, 0, 0), color=CAP_BAD, size=SMALL, filled=True)).arrange(_mn.RIGHT, buff=0.2).move_to(PORT_AT + np.array([0, -2.2, 0])))
        group.view = view
        group.base, group.over = _mn.VGroup(*group.submobjects[:split]), _mn.VGroup(*group.submobjects[split:])
        return group

    def _numberline(self, tracker):
        """📏 Number line 1.5 to 4.5 with both ties as arrows, the slider value and its cast result."""
        y = SHEET_AT[1] - 1.45
        lo, hi, x0, x1 = 1.5, 4.5, SHEET_AT[0] - 2.3, SHEET_AT[0] + 2.3
        at = lambda v: x0 + (v - lo) / (hi - lo) * (x1 - x0)
        base = _mn.VGroup(_draft.segments([((x0, y, 0), (x1, y, 0))] + [((at(v), y - 0.07, 0), (at(v), y + 0.07, 0)) for v in (2, 2.5, 3, 3.5, 4)], MUTED, 1.6))
        base.add(*[self.text(self._pn([_num(v) if v % 1 else str(int(v))])[0], (at(v), y - 0.28, 0), size=SMALL, color=INK_GREY) for v in (2, 2.5, 3, 3.5, 4)])
        for tie in (2.5, 3.5):
            target = _model.cast(tie, "Integer").value
            base.add(_mn.Arrow((at(tie), y + 0.22, 0), (at(target), y + 0.22, 0), buff=0, stroke_width=3, color=INK_GREEN, max_tip_length_to_length_ratio=0.35))

        def build():
            value = tracker.get_value()
            out = _mn.VGroup()
            if lo <= value <= hi:
                cast = _model.cast(value, "Integer").value
                out.add(_mn.Circle(radius=0.15, color=INK_GREEN, stroke_width=2.5).move_to((at(cast), y, 0)))
            return out

        dot = _mn.Dot((at(min(max(tracker.get_value(), lo), hi)), y, 0), radius=0.11, color=INK_GOLD)
        dot.add_updater(lambda mob: mob.move_to((at(min(max(tracker.get_value(), lo), hi)), y, 0)))
        base.dot = dot
        return base, self.live(build, key=lambda: round(tracker.get_value(), 3))

    def _order(self, u, walker=True):
        """🪜 The fixed cast order as four steps with the token ``Type A`` falling through; ``u`` runs 0 to 1."""
        steps = ((self.tr("1  Same type: no match", "1  Gleicher Typ: passt nicht"), SERV), (self.tr("2  The source converts itself: no match", "2  Die Quelle wandelt um: passt nicht"), SERV),
                 (self.tr("3  The target reads it: no match", "3  Das Ziel liest es: passt nicht"), SERV), (self.tr("4  Both fail: error, slot stays blank", "4  Beides scheitert: Fehler, Platz bleibt leer"), SERV))
        group = _mn.VGroup()
        boxes = []
        for index, (text, tone) in enumerate(steps):
            box = _mn.RoundedRectangle(corner_radius=0.08, width=5.2, height=0.56, stroke_color=tone, stroke_width=2.0, fill_color=tone, fill_opacity=0.16)
            box.move_to(SHEET_AT + np.array([0.3, 1.55 - index * 0.95, 0]))
            label = self.text(text, box.get_center(), size=SMALL, color=_canvas.TEXT, width=4.9)
            boxes.append(box)
            group.add(box, label)
            if index < 3:
                group.add(_mn.Arrow(box.get_bottom(), box.get_bottom() + np.array([0, -0.39, 0]), buff=0.0, stroke_width=3, color=MUTED, max_tip_length_to_length_ratio=0.5))
                group.add(self.text("×", box.get_right() + np.array([0.22, 0, 0]), size=24, color=INK_RED) if u > (index + 0.5) / 4 else _mn.VGroup())
        position = min(u * 3, 3.0)
        index = min(int(position), 2)
        start, end = boxes[index].get_left() + np.array([-0.45, 0, 0]), boxes[index + 1].get_left() + np.array([-0.45, 0, 0])
        chip = self.badge(self.tr("Type A", "Typ A"), start + (end - start) * (position - index), color=INK_RED, size=SMALL, filled=True)
        group.add(*([chip] if walker else []), self.text(self.tr("developer.rhino3d.com · Grasshopper Data Types", "developer.rhino3d.com · Grasshopper Data Types"), SHEET_AT + np.array([0.3, -2.2, 0]), size=SMALL, color=INK_GREY))
        return group

    def _chips(self, sol):
        """🎫 Three value chips under the tower: width, storeys, plane."""
        width = self.quantity(sol.value("wid", "Number"), "m")
        slot = next(s for s in _nakagin.slots() if (s.building, s.tower, s.storey, s.facade, s.item) == SLOT)
        key = f"{{{slot.building};{slot.tower};{slot.storey};{slot.facade}}}"
        chips = [self.tr(f"Width {width}", f"Breite {width}"), self.tr(f"Storeys {sol.value('sto', 'Integer')}", f"Geschosse {sol.value('sto', 'Integer')}"), self.tr(f"Plane at slot {key}", f"Ebene am Platz {key}")]
        return _mn.VGroup(*[self.badge(text, (0, 0, 0), color=color, size=SMALL, filled=False) for text, color in zip(chips, (INK_TEAL, INK_GOLD, INK_GREEN))]).arrange(_mn.RIGHT, buff=0.18).move_to(PORT_AT + np.array([0, -2.2, 0]))

    def _domain_world(self, sol):
        """↔️ Capsule A with the length interval and the Divide Domain cuts beside it."""
        cap = self._capsule(dims=(), up=0.3)
        domain = sol.value("dom", "Domain")
        offset = 1.4
        a, b = cap.at(offset, domain.lo, 0), cap.at(offset, domain.hi, 0)
        bar = _mn.VGroup(_draft.segments([(a, b)], INK_GOLD, 3.4))
        cuts = [seg.lo for seg in sol.items("dd", "Segments")] + [sol.items("dd", "Segments")[-1].hi]
        for c in cuts:
            p = cap.at(offset, c, 0)
            bar.add(_draft.segments([(p + np.array([-0.07, 0.07, 0]), p + np.array([0.07, -0.07, 0]))], INK_GOLD, 2.4))
        bar.add(self._tag("0", a + np.array([0.25, -0.2, 0]), INK_GOLD), self._tag(self.quantity(domain.hi, "m", 1), b + np.array([0.3, 0.25, 0]), INK_GOLD))
        return _mn.VGroup(cap, bar)

    def _geometry_world(self, sol):
        """🧬 Capsule A with the window circle as its outline curve and the body tinted as one Brep."""
        cap = self._capsule(dims=(), highlight={"front": INK_GOLD, "top": INK_GOLD, "right": INK_GOLD, "left": INK_GOLD})
        circle = sol.value("crv", "Curve").shape
        centre, radius = np.array(circle.plane.origin.t), circle.radius
        ex, ey = np.array(circle.plane.x.t), np.array(circle.plane.y.t)
        points = [cap.at(*(centre + radius * (math.cos(a) * ex + math.sin(a) * ey))) for a in np.linspace(0, 2 * math.pi, 60)]
        chips = _mn.VGroup(_morph.token(self.tr("Circle → Curve", "Kreis → Kurve"), kind="number", color="#E0B84A", size=1.1), _morph.token(self.tr("Box → Brep", "Box → Brep"), kind="number", color="#E0B84A", size=1.1)).arrange(_mn.RIGHT, buff=0.25).move_to(PORT_AT + np.array([0, -2.15, 0]))
        world = _mn.VGroup(cap)
        world.rings = _mn.VGroup(_draft.polyline(points, "#FFFFFF", 5.0, closed=True), _draft.polyline(points, INK_RED, 2.4, closed=True))
        world.chips = chips
        return world

    def _ring(self, sol, cap, color, width=3.0):
        """⭕ The window circle of the capsule as one closed outline."""
        circle = sol.value("crv", "Curve").shape
        centre, radius = np.array(circle.plane.origin.t), circle.radius
        ex, ey = np.array(circle.plane.x.t), np.array(circle.plane.y.t)
        return _draft.polyline([cap.at(*(centre + radius * (math.cos(a) * ex + math.sin(a) * ey))) for a in np.linspace(0, 2 * math.pi, 60)], color, width, closed=True)

    def _standin(self, cap, sol, color):
        """🫧 Body outline and window ring of a capsule as two parts, so a token can morph into them part-wise."""
        body = _morph.silhouette(cap, color=color)
        body.set_fill(color, opacity=0.18).set_stroke(color, width=2.4)
        return _mn.VGroup(body, self._ring(sol, cap, color, 2.6))

    def _matrix(self):
        """🧮 Casting matrix grid by families: source rows, target columns, family labels."""
        cw, ch, left, gap = 1.2, 0.36, 1.35, 0.1
        n_cols = sum(len(c[2]) for c in COLUMN_FAMILIES)
        total = left + cw * n_cols + gap * (len(COLUMN_FAMILIES) - 1)
        x0, y0 = -total / 2, 2.2
        group = _mn.VGroup()
        self._cell, self._cw, self._ch = {}, cw, ch
        group.add(self.text(self.tr("From ↓  To →", "Von ↓  Nach →"), (x0 + left / 2, y0 - 0.2, 0), size=SMALL, color=INK_GREY, width=left - 0.1))
        x = x0 + left
        columns = {}
        for en, de, names in COLUMN_FAMILIES:
            group.add(self.text(self.tr(en, de), (x + cw * len(names) / 2, y0 + 0.2, 0), size=SMALL, color=GOOD))
            for name in names:
                columns[name] = x + cw / 2
                group.add(self.text(name, (x + cw / 2, y0 - 0.2, 0), size=SMALL, color=INK, width=cw - 0.1))
                x += cw
            x += gap
        y = y0 - ch
        rows = {}
        for en, de, names in FAMILIES:
            for name in names:
                y -= ch
                rows[name] = y + ch / 2
                group.add(self.text(name, (x0 + left - 0.1, y + ch / 2, 0), size=SMALL, color=INK, anchor="r", width=left - 0.15))
            y -= gap
        for r, ry in rows.items():
            for c, cx in columns.items():
                centre = np.array([cx, ry, 0.0])
                self._cell[(r, c)] = centre
                group.add(_mn.Rectangle(width=cw, height=ch, stroke_color=MUTED, stroke_width=0.8, stroke_opacity=0.45, fill_opacity=0).move_to(centre))
        self._grid = group
        return group

    def _grid_cells(self):
        """🧱 Background cells: faint green where the engine knows a cast, grey hatching where none exists."""
        cw, ch = self._cw, self._ch
        group = _mn.VGroup()
        shown = {(s, t) for s, t, _, _ in CELLS}
        rules = _model.rules()
        for (r, c), centre in self._cell.items():
            if (r, c) in shown:
                continue
            ring = [centre + np.array([sx * cw / 2, sy * ch / 2, 0]) for sx, sy in ((-1, -1), (1, -1), (1, 1), (-1, 1))]
            if r == c:
                group.add(self.text("=", centre, size=SMALL, color=INK_GREY))
            elif f"{r}>{c}" in rules:
                group.add(_mn.Rectangle(width=cw - 0.04, height=ch - 0.04, stroke_width=0, fill_color=GOOD, fill_opacity=0.14).move_to(centre))
            else:
                group.add(_draft.hatch(ring, MUTED, 0.1, 45, 1.0, 0.55))
        return group

    def _filled(self, source, target, rules, words):
        """🟩 One filled matrix cell: green when the engine says the cast works, red when it fails, dashed when not verified."""
        centre, cw, ch = self._cell[(source, target)], self._cw, self._ch
        group = _mn.VGroup()
        shares = (1.0,) if len(rules) == 1 else (0.36, 0.64)
        left = -cw / 2
        pairs = (words,) if len(rules) == 1 else words
        for rule, share, pair in zip(rules, shares, pairs):
            info = _model.rules()[rule]
            tone = GOOD if info["ok"] else SERV
            width = cw * share
            where = centre + np.array([left + width / 2, 0, 0])
            box = _mn.Rectangle(width=width - 0.04, height=ch - 0.04, stroke_color=tone, stroke_width=2.0, fill_color=tone, fill_opacity=0.32).move_to(where)
            if info["basis"] != "verified":
                box = _mn.VGroup(_mn.Rectangle(width=width - 0.04, height=ch - 0.04, stroke_width=0, fill_color=tone, fill_opacity=0.32).move_to(where),
                                 _mn.DashedVMobject(_mn.Rectangle(width=width - 0.04, height=ch - 0.04, stroke_color=tone, stroke_width=2.4), num_dashes=22).move_to(where))
            group.add(box, self.text(self.tr(*pair), where, size=SMALL, color=INK, width=width - 0.1))
            left += width
        return group

    def _legend(self):
        """🎨 Swatches: works, fails, to check, not castable; plus the family rule."""
        items = ((GOOD, False, self.tr("works (verified)", "klappt (belegt)")), (SERV, False, self.tr("fails", "scheitert")), (GOOD, True, self.tr("works, check in Rhino 8", "klappt, in Rhino 8 prüfen")))
        group = _mn.VGroup()
        for tone, dashed, label in items:
            box = _mn.Rectangle(width=0.5, height=0.26, stroke_color=tone, stroke_width=2.0, fill_color=tone, fill_opacity=0.3)
            if dashed:
                box = _mn.VGroup(_mn.Rectangle(width=0.5, height=0.26, stroke_width=0, fill_color=tone, fill_opacity=0.3), _mn.DashedVMobject(_mn.Rectangle(width=0.5, height=0.26, stroke_color=tone, stroke_width=2.0), num_dashes=14))
            text = self.text(label, (0, 0, 0), size=SMALL, color=INK)
            text.next_to(box, _mn.RIGHT, buff=0.12)
            group.add(_mn.VGroup(box, text))
        hatch = _draft.hatch([(-0.25, -0.13, 0), (0.25, -0.13, 0), (0.25, 0.13, 0), (-0.25, 0.13, 0)], MUTED, 0.1, 45, 1.0, 0.8)
        text = self.text(self.tr("no cast: needs a component", "keine Umwandlung: Komponente nötig"), (0, 0, 0), size=SMALL, color=INK)
        group.add(_mn.VGroup(hatch, text.next_to(hatch, _mn.RIGHT, buff=0.12)))
        group.arrange(_mn.RIGHT, buff=0.4).move_to(np.array([0, -2.3, 0]))
        rule = self.text(self.tr("Along a family a cast works; across families you need a component.", "Innerhalb einer Familie klappt die Umwandlung; über Familien hinweg braucht es eine Komponente."),
                         (0, -1.88, 0), size=SMALL, color=GOOD, width=11.5)
        return _mn.VGroup(group, rule)

    def _habits(self):
        """🌱 Three habit cards, each tied to a scene of the video; UI names stay English."""
        cards = ((self.tr("Check with a Panel", "Mit einem Panel prüfen"), self.tr(f"{SLIDER:g} shows {STOREYS} (check in Rhino 8)", f"{SLIDER:g}".replace(".", ",") + f" zeigt {STOREYS} (in Rhino 8 prüfen)")),
                 (self.tr("Build planes explicitly", "Ebenen ausdrücklich bauen"), self.tr("the capsule faces outwards", "die Kapsel schaut nach außen")),
                 (self.tr("Labels are no numbers", "Bezeichnungen sind keine Zahlen"), self.tr("Type A needs a Value List", "Typ A braucht eine Value List")))
        group = _mn.VGroup()
        for index, (head, body) in enumerate(cards):
            centre = SHEET_AT + np.array([0, 1.5 - index * 1.45, 0])
            box = _mn.RoundedRectangle(corner_radius=0.1, width=6.2, height=1.15, stroke_color=GOOD, stroke_width=2.0, fill_color="#FFFFFF", fill_opacity=0.9).move_to(centre)
            badge = self.badge(str(index + 1), centre + np.array([-2.75, 0, 0]), color=INK_GREEN, size=SMALL, filled=True)
            group.add(box, badge, self.text(head, centre + np.array([-2.3, 0.2, 0]), size=19.2, color=_canvas.TEXT, anchor="l", width=5.2),
                      self.text(body, centre + np.array([-2.3, -0.25, 0]), size=SMALL, color=INK_GREY, anchor="l", width=5.2))
        return group

    def present_casting(self):
        """🎬 Run the chapter with dark dimension ink."""
        with _dark_dimensions():
            self._present_casting()

    def _present_casting(self):
        """🎞️ A teaser, types and the first casts on the Nakagin capsule, rounding, planes in 3D, more casts, failure and its order, the casting matrix and three habits."""
        self._d, self._cache, self._towers = _model.canvas("casting"), {}, {}
        base = self._sol()
        self._tower_fit = _canvas.building_fit(5.4, 3.4, centre=PORT_AT + np.array([-0.3, 0.1, 0]), margin=0.0)
        sheet = self.gh_background(at=SHEET_AT, width=SHEET_SIZE[0], height=SHEET_SIZE[1])
        port = self.gh_viewport(at=PORT_AT, width=PORT_SIZE[0], height=PORT_SIZE[1], label=self.tr("Perspective", "Perspektive"))

        pt = self._sol()
        plane = pt.value("plc", "Plane")
        canvas, title = self._view("point-plane", pt), self._view_title("point-plane")
        stage = self._stage(spin=0.0, arrows=True, arc=True, legend=True)
        self.step("teaser", _mn.FadeIn(sheet), _mn.FadeIn(port), _mn.FadeIn(canvas), _mn.FadeIn(stage), _mn.FadeIn(title), seconds=1.6)

        old = (canvas, stage, title)
        canvas, world, title = self._view("types", base, zoom=1.2), self._tower(), self._view_title("types")
        chips = list(self._chips(base))
        slot = next(s for s in _nakagin.slots() if (s.building, s.tower, s.storey, s.facade, s.item) == SLOT)
        spot = self._towers[None].at(*slot.plane.origin)
        sources = [_mn.Rectangle(width=0.4, height=0.07, stroke_width=0, fill_color=INK_TEAL, fill_opacity=1.0).move_to(spot + np.array([0.0, -0.2, 0.0])),
                   self._band(1, int(base.value("sto", "Integer")), INK_GOLD),
                   _mn.Square(side_length=0.28, stroke_color=CAP_GOOD, stroke_width=2.4, fill_color=CAP_GOOD, fill_opacity=0.35).move_to(spot + np.array([0.0, 0.25, 0.0]))]
        self.add(*sources)
        self.step("types", *_gone(*old), _mn.FadeIn(canvas), _mn.FadeIn(world), _mn.FadeIn(title), _morph.scatter(sources, chips, lag=0.3, path_arc=0.5, run_time=1.4), seconds=1.8)

        old = (canvas, world, title, *chips)
        canvas, title = self._view("text-number", base, zoom=1.25), self._view_title("text-number")
        hint = self.gh_bubble(self.tr("Text → Number", "Text → Zahl"), level="remark", target=canvas.w["tn"], width=2.4)
        ghost = self._capsule(ghost=True, dims=())
        leash = _morph.tether(canvas.w["t25"], ghost.get_center(), color=INK_GOLD, width=2.8)
        ghost_display = _snapshot.capture(ghost, self.camera)
        del ghost
        self.add(leash)
        self.step("mismatch", *_gone(*old), _mn.FadeIn(canvas), _mn.FadeIn(hint), _mn.FadeIn(ghost_display), _mn.FadeIn(title), seconds=1.2)

        flags = self._flags(canvas, ("tn",), "Text>Number")
        solid = self._capsule()
        built = self._standin(solid, base, INK_TEAL)
        solid_display = _snapshot.capture(solid, self.camera)
        del solid
        arrive = self._grip(canvas.w["tn"], "in", "Number").get_center() + np.array([-0.5, 0.45, 0.0])
        quoted = _morph.token('"2.5"', kind="number", color=INK_GOLD, size=1.2).move_to(canvas.w["t25"].get_top() + np.array([0.0, 0.45, 0.0]))
        chip = _morph.token(self._pn(["2.5"])[0], kind="number", color=INK_TEAL, size=1.2).move_to(arrive)
        self.step("text_number", *_gone(leash), _mn.FadeOut(hint), _mn.FadeIn(flags), _mn.FadeIn(quoted), _morph.travel(quoted, self._route(quoted, chip), run_time=1.4),
                  seconds=self.then((_morph.retext(quoted, chip, run_time=0.7),), 0.7, 1.4, ((_mn.FadeOut(ghost_display), _morph.morph(chip, built, pairs=[(chip[0], built[0]), (chip[1], built[1])], path_arc=0.4, dip=0.15, run_time=1.5), self._later(_mn.FadeIn(solid_display), 0.55)), 1.5)))

        value = _mn.ValueTracker(SLIDER)
        count = lambda: _model.cast(value.get_value(), "Integer").value
        old = (canvas, title, flags, solid_display, built)
        canvas = self._view("storeys", self._sol(), trackers={"sl": value}, layout={"slp": (0, 130), "sto2p": (260, 130)}, live={
            "slp": lambda c: self.live(lambda: self.gh_panel(self._pn([_num(round(value.get_value(), 1))]), at=c, width=0.9), key=lambda: round(value.get_value(), 1)),
            "sto2p": lambda c: self.live(lambda: self.gh_panel([str(count())], at=c, width=0.9), key=count)})
        canvas_display = _keyed.create(lambda: (canvas.update(0), canvas)[1], lambda: round(value.get_value(), 3), self.camera)
        shell = self._tower()
        tower = self._lit_display(count)
        title, flags = self._view_title("storeys"), self._flags(canvas, ("sto2",), "Number>Integer", source=True)
        sl_chip = _morph.token(self._pn([f"{SLIDER:g}"])[0], kind="number", color=INK_GOLD, size=1.2).move_to(canvas.w["sl"].get_top() + np.array([0.0, 0.45, 0.0]))
        landing = self._grip(canvas.w["sto2"], "in", "Integer").get_center() + np.array([-0.5, 0.45, 0.0])
        int_chip = _morph.token(str(count()), kind="number", color=INK_GREEN, size=1.2).move_to(landing)
        dropped = _morph.token(self.tr(".6 dropped", ".6 entfällt"), kind="number", color="#E0807A", size=1.0).move_to(landing + np.array([1.2, 0.0, 0.0]))
        bands = [self._band(i, i, INK_GOLD) for i in range(1, count() + 1)]
        self.step("storeys", *_gone(*old), _mn.FadeIn(canvas_display), _mn.FadeIn(shell), _mn.FadeIn(tower), _mn.FadeIn(title), _mn.FadeIn(flags), _mn.FadeIn(sl_chip), _morph.travel(sl_chip, self._route(sl_chip, int_chip), run_time=1.4),
                  seconds=self.then((_morph.retext(sl_chip, int_chip, run_time=0.7), self._later(_mn.FadeIn(dropped, shift=_mn.DOWN * 0.1), 0.2)), 0.7, 1.4, ((_mn.FadeOut(dropped, shift=_mn.DOWN * 0.8), _morph.split(int_chip, bands, lag=0.15, path_arc=-0.3, run_time=1.2)), 1.6)))

        line_base, line_live = self._numberline(value)
        line_display = _keyed.create(lambda: (line_live.update(0), line_live)[1], count, self.camera)
        ramp = lambda a, b: _mn.UpdateFromAlphaFunc(value, lambda mob, alpha: mob.set_value(a + (b - a) * alpha), rate_func=_mn.smooth, run_time=1.0)
        self.step("ties", _mn.FadeIn(line_base), _mn.FadeIn(line_display), seconds=self.then((self._early(_morph.gather(bands, line_base.dot, lag=0.05, run_time=1.2), 0.4), _mn.Succession(_mn.Wait(1.2), ramp(SLIDER, 2.5), _mn.Wait(0.8), ramp(2.5, 3.5))), 4.0, 0.8))

        self.play(value.animate.set_value(SLIDER), run_time=0.8)
        floor = _model.evaluate(self._d, {"sl": SLIDER}).items("sto3", "Integer")[0]
        canvas2 = self._view("deliberate", self._sol(), trackers={"sl": value}, layout={"sto3": (260, 130), "sto3p": (260, 240)}, live={
            "sto3p": lambda c: self.live(lambda: self.gh_panel([str(floor)], at=c, width=0.9), key=lambda: floor)})
        tower2, title2 = self._lit(floor), self._view_title("deliberate")
        lost = _mn.VGroup(self.text(str(count()), self._tick_at(count()), size=26, color=INK_RED, anchor="l"),
                          _draft.segments([(self._tick_at(count()) + np.array([-0.05, -0.16, 0]), self._tick_at(count()) + np.array([0.3, 0.16, 0]))], INK_RED, 2.4))
        eleventh = self._band(count(), count(), INK_GOLD)
        self.add(eleventh)
        self.step("deliberate", *_gone(canvas_display, tower, title, flags, line_base, line_display, line_base.dot), _mn.FadeIn(canvas2), _mn.FadeIn(tower2), _mn.FadeIn(title2), seconds=self.then((_morph.morph(eleventh, lost, path_arc=0.5, run_time=1.2),), 1.2, 1.2))

        canvas3, title3 = self._view("point-plane", pt), self._view_title("point-plane")
        base_stage = self._stage()
        anchor, view0 = base_stage.base, base_stage.view
        stage1 = self._stage(axes=[plane]).over
        flags = self._flags(canvas3, ("plc",), "Point>Plane")
        origin_dot = _mn.Dot(view0(*plane.origin.t), radius=0.1, color=INK_GOLD)
        origin_chip = _morph.token(self._lb("Slot origin"), kind="number", color="#E0B84A").move_to(canvas3.w["plc"].get_top() + np.array([0.0, 0.4, 0.0]))
        origin_move = _morph.morph(origin_dot, origin_chip, ghost=0.3, path_arc=-0.5, run_time=1.3)
        self.step("point_plane", *_gone(canvas2, tower2, title2, lost, shell), _mn.FadeIn(canvas3), _mn.FadeIn(anchor), _mn.FadeIn(stage1), _mn.FadeIn(title3), _mn.FadeIn(flags), _mn.FadeIn(origin_dot), seconds=self.then((origin_move,), 1.3, 1.2))

        full2 = self._stage(spin=0.0, arrows=True, arc=True, legend=True, late=True)
        stage2, red, red_arrow, tags = full2.over, full2.red, full2.red_arrow, full2.tags
        world_plane = _morph.token("", kind="plane", color=CAP_BAD, size=1.5).move_to(view0(*plane.origin.t))
        self.step("wrong_way", _mn.FadeOut(stage1), _mn.FadeIn(stage2), _mn.FadeIn(world_plane), seconds=self.then((_morph.morph(world_plane, red, path_arc=0.3, run_time=1.4), self._later(_mn.FadeIn(red_arrow), 0.6), self._later(_mn.FadeIn(tags), 0.75)), 1.4, 1.2))

        lp = self._sol()
        canvas4, title4 = self._view("line-plane", lp, zoom=1.25), self._view_title("line-plane")
        line = lp.value("ln", "Line")
        line_mob = _mn.Line(view0(*line.a.t), view0(*line.b.t), color=_draw.AXIS_Z, stroke_width=5)
        z_axes = self.plane_axes(_plane(lp.value("lpl", "Plane")), at=view0(*plane.origin.t), scale=0.5, view=view0, length=1.8)
        flags4 = self._flags(canvas4, ("lpl",), "Line>Plane")
        self.step("line_plane", *_gone(canvas3, stage2, title3, flags, origin_chip, red, red_arrow, tags, origin_move.ghost_mobject), _mn.FadeIn(canvas4), _mn.FadeIn(line_mob), _mn.FadeIn(title4), _mn.FadeIn(flags4),
                  seconds=self.then((_morph.morph(line_mob, z_axes, path_arc=0.2, run_time=1.4),), 1.4, 1.3))

        spin = _mn.ValueTracker(0.0)
        canvas5, title5 = self._view("explicit", self._sol(), layout={"vx": (0, 100), "vy": (0, 200), "cpl": (260, 100), "capc": (260, 215), "pln": (260, 300), "ore": (520, 150)}), self._view_title("explicit")
        stage4 = self._changing_display(lambda: self._stage(spin=spin.get_value(), arrows=True, arc=True, legend=True, axes=[self._sol().value("cpl", "Plane")]).over, lambda: round(spin.get_value(), 2))
        inputs = [_morph.token(label, kind="number", color=color).move_to(canvas5.w[oid].get_top() + np.array([0.0, 0.4, 0.0]))
                  for oid, label, color in (("ore", self.tr("Origin", "Ursprung"), "#E0B84A"), ("vx", self._lb("X-Axis"), "#E08080"), ("vy", self._lb("Y-Axis"), "#80C890"))]
        built_plane = _morph.token("", kind="plane", color=CAP_GOOD, size=1.5).move_to(view0(*plane.origin.t))
        self.step("explicit", *_gone(canvas4, title4, flags4, z_axes), _mn.FadeIn(canvas5), _mn.FadeIn(stage4), _mn.FadeIn(title5), *[self._later(_mn.FadeIn(c), 0.2) for c in inputs],
                  seconds=self.then((_mn.UpdateFromAlphaFunc(spin, lambda mob, alpha: mob.set_value(alpha), rate_func=_mn.smooth, run_time=2.4), _morph.gather(inputs, built_plane, run_time=1.2)), 2.4, 1.0, ((_mn.Wait(0.8),), 0.8)))

        sd = self._sol()
        canvas6, world, title6 = self._view("domain", sd), self._domain_world(sd), self._view_title("domain")
        flags = self._flags(canvas6, ("dom",), "Number>Domain")
        length = self.quantity(sd.value("dom", "Domain").hi, "m", 1)
        length_chip = _morph.token(length, kind="number", color="#E0B84A").move_to(canvas6.w["dom"].get_top() + np.array([0.0, 0.4, 0.0]))
        self.step("domain", *_gone(canvas5, stage4, title5, built_plane), _mn.FadeIn(canvas6), _morph.morph(anchor, world[0], path_arc=0.0, run_time=1.4), _mn.FadeIn(length_chip), _mn.FadeIn(title6), _mn.FadeIn(flags),
                  seconds=self.then((_morph.morph(length_chip, world[1], path_arc=0.4, run_time=1.4),), 1.4, 1.5))
        world_display = _snapshot.capture(world, self.camera)
        self.remove(world, *world.get_family())
        self.add(world_display)
        del world

        canvas7a, canvas7b = self._view("circle-curve", sd, dy=1.0), self._view("box-brep", sd, skip=("bxv", "bxvp"), dy=-1.05)
        world7, title7 = self._geometry_world(sd), self._title_text("Circle → Curve · Box → Brep", "Kreis → Kurve · Box → Brep")
        flags7 = _mn.VGroup(self._flags(canvas7a, ("crv",), "Circle>Curve"), *[self._dashed(canvas7b.w["brp"])])
        box_src = _morph.silhouette(world7[0], color=INK_GOLD)
        box_src.set_fill(INK_GOLD, opacity=0.25)
        world7_display = _snapshot.capture(world7, self.camera)
        world7_rings, world7_chips = world7.rings, world7.chips
        del world7
        self.step("geometry", *_gone(canvas6, world_display, title6, flags), _mn.FadeIn(canvas7a), _mn.FadeIn(canvas7b), _mn.FadeIn(world7_display), _mn.FadeIn(world7_rings), _mn.FadeIn(box_src), _mn.FadeIn(title7), _mn.FadeIn(flags7),
                  seconds=self.then((_morph.morph(world7_rings, world7_chips[0], path_arc=0.4, run_time=1.4), _morph.morph(box_src, world7_chips[1], path_arc=-0.4, run_time=1.4)), 1.4, 1.6))

        flag = _mn.ValueTracker(1)
        on = lambda: flag.get_value() > 0.5
        solved = lambda: self._sol(tg=on())
        canvas9 = self._view("bool-int", self._sol(), zoom=1.15, live={
            "tg": lambda c: self.live(lambda: self.gh_toggle(on(), at=c, name=self._lb(self._d.obj("tg").settings["label"]), scale=1.15), key=on),
            "bip": lambda c: self.live(lambda: self.gh_panel([str(solved().items("bi", "Integer")[0])], at=c, width=0.9), key=on)})
        canvas9_display = _keyed.create(lambda: (canvas9.update(0), canvas9)[1], on, self.camera, cache=True)
        world9 = _keyed.create(lambda: self._capsule(window=bool(solved().items("bi", "Integer")[0]), dims=()), on, self.camera, cache=True)
        title9 = self._view_title("bool-int")
        flip = _mn.UpdateFromAlphaFunc(flag, lambda mob, alpha: mob.set_value(0 if 0.35 < alpha < 0.7 else 1), run_time=2.0)
        one = _morph.token("1", kind="number", color="#7FC48A").move_to(canvas9.w["bip"].get_top() + np.array([0.0, 0.4, 0.0]))
        ring_on = self._ring(sd, self._capsule(dims=()), INK_GREEN, 4.0)
        self.step("bool_int", *_gone(canvas7a, canvas7b, world7_display, world7_chips, box_src, world7_rings, title7, flags7), _mn.FadeIn(canvas9_display), _mn.FadeIn(world9), _mn.FadeIn(title9), self._later(_mn.FadeIn(one), 0.75),
                  seconds=self.then((_morph.travel(one, self._route(one, ring_on), run_time=1.3), flip), 2.0, 2.4, ((_morph.morph(one, ring_on, run_time=0.9),), 0.9)))

        level = _mn.ValueTracker(1)
        shown = lambda: int(round(level.get_value()))
        visible = lambda: self._sol(isl=shown()).items("win", "Boolean")[0]
        canvas10 = self._view("int-bool", self._sol(), trackers={"isl": level}, layout={"win": (260, 0), "winp": (260, 150)}, live={
            "winp": lambda c: self.live(lambda: self.gh_panel([_model.describe(visible())], at=c, width=1.2), key=visible)})
        canvas10_display = _keyed.create(lambda: (canvas10.update(0), canvas10)[1], shown, self.camera, cache=True)
        world10 = _keyed.create(lambda: self._capsule(window=visible(), dims=()), visible, self.camera, cache=True)
        title10, flags10 = self._view_title("int-bool"), self._flags(canvas10, ("win",), _model.cast(1.0, "Boolean").rule)
        path = lambda mob, alpha: mob.set_value(1 if alpha < 0.3 else 0 if alpha < 0.65 else 2)
        true_chip = _morph.token("True", kind="number", color="#E0B070").move_to(canvas10.w["winp"].get_top() + np.array([0.0, 0.4, 0.0]))
        ring_two = self._ring(sd, self._capsule(dims=()), INK_ORANGE, 4.0)
        self.step("int_bool", *_gone(canvas9_display, world9, title9, ring_on), _mn.FadeIn(canvas10_display), _mn.FadeIn(world10), _mn.FadeIn(title10), _mn.FadeIn(flags10), self._later(_mn.FadeIn(true_chip), 0.8),
                  seconds=self.then((_morph.travel(true_chip, self._route(true_chip, ring_two), run_time=1.3), _mn.UpdateFromAlphaFunc(level, path, run_time=2.6)), 2.6, 2.8, ((_morph.morph(true_chip, ring_two, run_time=0.9),), 0.9)))

        loc = self._sol()
        canvas11, title11 = self._view("locale", loc, zoom=1.2), self._view_title("locale")
        warn = self.gh_bubble(loc.msgs("tdn")[0]["en"], level="warning", target=canvas11.w["tdn"], side="below", width=3.6)
        rule = "Text>Number!comma" if "Text>Number!comma" in _model.rules() else "Text>Number!"
        flags11 = self._flags(canvas11, ("tdn",), rule)
        note = self.text(self.tr("a comma is not a decimal point here", "ein Komma ist hier kein Dezimalpunkt"), PORT_AT + np.array([0, -1.75, 0]), size=SMALL, color=INK_RED)
        ghost11 = self._capsule(ghost=True, dims=())
        ghost11_display = _snapshot.capture(ghost11, self.camera)
        german = self._tag("Grasshopper-Meldung (englisch)", warn.get_bottom() + np.array([0.0, -0.2, 0]), INK_ORANGE) if self.language() == "de" else _mn.VGroup()
        comma = _morph.token(self._pn(["2.5"])[0].replace(".", ","), kind="number", color="#E0B070", size=1.2).move_to(canvas11.w["tde"].get_top() + np.array([0.0, 0.45, 0.0]))
        arrive = canvas11.w["tdn"].get_top() + np.array([-0.6, 0.55, 0.0])
        stop = _morph.token("!", kind="number", color="#E0807A", size=1.4).move_to(arrive + np.array([0.95, 0.0, 0.0]))
        rest = _morph.token("2 5", kind="number", color="#E0B070", size=1.2).move_to(arrive)
        red_comma = _morph.token(",", kind="text", color="#D8362B", size=2.4).move_to(arrive + np.array([0.15, -0.6, 0.0]))
        self.step("locale", *_gone(canvas10_display, world10, title10, flags10, ring_two), _mn.FadeIn(canvas11), _mn.FadeIn(warn), _mn.FadeIn(german), _mn.FadeIn(ghost11_display), _mn.FadeIn(note), _mn.FadeIn(title11), _mn.FadeIn(flags11), self._later(_mn.FadeIn(comma), 0.5), seconds=self.then((_morph.travel(comma, [comma.get_center(), (comma.get_center() + arrive) / 2 + np.array([0.0, 0.5, 0.0]), arrive], run_time=1.0),), 1.0, 1.8, ((_morph.split(comma, [rest, red_comma], lag=0.1, path_arc=0.2, run_time=0.8), _mn.FadeIn(stop, scale=0.6)), 0.9)))

        fail = self._sol()
        canvas12 = self._view("fail", fail, red=("cbx",))
        message = fail.msgs("cbx")[0]
        bubble = self.gh_bubble(message["en"], level="error", at=canvas12.w["cbx"].get_bottom() + np.array([1.0, -0.7, 0]), width=4.0)
        explain = _mn.VGroup(self.gh_bubble("Der Text lässt sich nicht als Zahl lesen; der Platz bleibt leer.", level="remark", at=SHEET_AT + np.array([1.85, 1.65, 0]), width=2.9),
                             self._tag("Grasshopper-Meldung (englisch)", bubble.get_bottom() + np.array([0.0, -0.2, 0]), INK_ORANGE)) if self.language() == "de" else _mn.VGroup()
        title12 = self._view_title("fail")
        nothing = self.text(self.tr("no width, no capsule", "keine Breite, keine Kapsel"), PORT_AT + np.array([0, -1.75, 0]), size=SMALL, color=INK_RED)
        blank = _morph.token("null", kind="number", color="#D8362B", size=1.2).move_to(canvas12.w["cbxp"].get_top() + np.array([0.0, 0.4, 0.0]))
        typea = _morph.token(self.tr("Type A", "Typ A"), kind="number", color="#E0B070", size=1.2).move_to(canvas12.w["typa"].get_top() + np.array([0.0, 0.45, 0.0]))
        at_box = canvas12.w["cbx"].get_left() + np.array([-0.45, 0.6, 0.0])
        self.remove(ghost11_display)
        self.add(ghost11)
        self.step("fail", *_gone(canvas11, warn, german, note, title11, flags11, stop, rest, red_comma), _mn.FadeIn(canvas12), _mn.FadeIn(bubble), _mn.FadeIn(explain), _mn.FadeIn(nothing), _mn.FadeIn(title12), _mn.FadeIn(typea),
                  _morph.travel(typea, [typea.get_center(), typea.get_center() + np.array([0.9, 0.4, 0.0]), at_box], run_time=1.2),
                  seconds=self.then((_morph.morph(ghost11, blank, pairs=_morph.capsule_pairs(ghost11, blank), path_arc=0.4, dip=0.15, run_time=1.4),), 1.4, 1.2))

        ladder_u = _mn.ValueTracker(0.0)
        ladder = self._changing_display(lambda: self._order(ladder_u.get_value(), walker=False), lambda: round(ladder_u.get_value(), 2))
        entry = _morph.token(self.tr("Type A", "Typ A"), kind="number", color="#E0807A", size=1.2).move_to(np.array([SHEET_AT[0] + 0.3 - 2.6 - 0.45, SHEET_AT[1] + 1.55, 0.0]))
        walk = lambda mob, alpha: mob.move_to(np.array([entry.get_center()[0], entry.get_center()[1] - 0.95 * min(alpha * 3.0, 3.0), 0.0]))
        self.step("order", *_gone(canvas12, bubble, explain, title12, blank), self._later(_mn.FadeIn(ladder), 0.55), _morph.travel(typea, self._route(typea, entry), run_time=1.2),
                  seconds=self.then((_mn.UpdateFromAlphaFunc(ladder_u, lambda mob, alpha: mob.set_value(alpha), run_time=2.4), _mn.UpdateFromAlphaFunc(typea, walk, run_time=2.4)), 2.4, 1.4))

        canvas13, fixed, title13 = self._view("fix", fail), self._capsule(), self._view_title("fix")
        item = self._d.obj("vl").settings["items"][0]
        expression = self.text(f"{self._lb(item['name'])} = {self._pn([_num(item['value'])])[0]}", canvas13.w["vl"].get_bottom() + np.array([0.0, -0.22, 0]), size=SMALL, color=_canvas.TEXT)
        expression.shift(np.array([max(0.0, canvas13.w["vl"].get_left()[0] + expression.width / 2 - expression.get_center()[0]), 0, 0]))
        flags13 = self._flags(canvas13, ("cbx2",), "Text>Number")
        chip3 = _morph.token(self._pn([_num(item["value"])])[0], kind="number", color=INK_TEAL).move_to(expression.get_right() + np.array([0.55, 0.0, 0.0]))
        built3 = self._standin(fixed, sd, INK_TEAL)
        fixed_display = _snapshot.capture(fixed, self.camera)
        del fixed
        self.step("fix", *_gone(ladder, nothing), _mn.FadeIn(canvas13), _mn.FadeIn(title13), _mn.FadeIn(expression), _mn.FadeIn(flags13), _morph.morph(typea, chip3, pairs=[(typea[0], chip3[0]), (typea[1], chip3[1])], path_arc=0.4, run_time=1.2), seconds=self.then((_morph.morph(chip3, built3, pairs=[(chip3[0], built3[0]), (chip3[1], built3[1])], path_arc=0.4, dip=0.15, run_time=1.5), self._later(_mn.FadeIn(fixed_display), 0.55)), 1.5, 1.2))

        matrix, legend = self._matrix(), self._legend()
        self.step("matrix", *_gone(canvas13, fixed_display, built3, title13, sheet, port, expression, flags13), _mn.FadeIn(matrix), seconds=1.4)

        cells = _mn.VGroup(*[self._filled(source, target, rules, words) for source, target, rules, words in CELLS])
        background = self._grid_cells()
        seen = [_morph.token(self.tr(*words[0]) if len(rules) > 1 else self.tr(*words), kind="number", color=GOOD if _model.rules()[rules[0]]["ok"] else "#E0807A", size=1.0) for _, _, rules, words in CELLS]
        _mn.VGroup(*seen).arrange(_mn.RIGHT, buff=0.12).move_to(np.array([0.0, 2.95, 0.0]))
        self.add(*seen)
        self.step("matrix_fill", _mn.LaggedStart(*[_morph.travel(s, self._route(s, d), run_time=1.4) for s, d in zip(seen, cells)], lag_ratio=0.25, run_time=2.6), _mn.FadeIn(background), _mn.FadeIn(legend), seconds=self.then((_mn.LaggedStart(*[_mn.FadeTransform(s, d) for s, d in zip(seen, cells)], lag_ratio=0.1, run_time=1.0),), 1.0, 3.0))

        sheet2 = self.gh_background(at=SHEET_AT, width=SHEET_SIZE[0], height=SHEET_SIZE[1])
        port2 = self.gh_viewport(at=PORT_AT, width=PORT_SIZE[0], height=PORT_SIZE[1], label=self.tr("Perspective", "Perspektive"))
        habits = self._habits()
        done = self._stage(axes=[self._sol().value("cpl", "Plane")])
        self.step("recap", *_gone(matrix, background, legend), _mn.FadeIn(sheet2), _mn.FadeIn(port2), _mn.FadeIn(done), *[_morph.gather([cells[i] for i in picks], _mn.VGroup(*habits[4 * k:4 * k + 4]), lag=0.1, run_time=1.4) for k, picks in enumerate(((1, 7, 8), (2, 3, 4, 5, 6), (0,)))], seconds=2.4)
