"""🦗 Grasshopper 1 canvas in Rhino 8 as clean vector widgets: components, params, sliders, panels, wires, bubbles, ribbon, groups and the Rhino viewport. See https://developer.rhino3d.com/guides/grasshopper/ and https://rhino.github.io/."""

from __future__ import annotations

import importlib
import math
import re

import numpy as np

_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_engine = importlib.import_module("manim")

mix, place, segments, polygon, polyline = _drafting.mix, _drafting.place, _drafting.segments, _drafting.polygon, _drafting.polyline


def P(point) -> np.ndarray:
    """📍 Canvas point as a float 3-vector, padded with zeros, so offsets and scaling work on it directly."""
    return np.array([*(float(value) for value in tuple(point)[:3]), 0.0, 0.0, 0.0][:3])

S_SMALL, W_EDGE, W_THIN, W_HAIR = _drafting.S_SMALL, _drafting.W_EDGE, _drafting.W_THIN, _drafting.W_HAIR

CANVAS, GRID_MINOR, GRID_MAJOR = "#EEEEEE", ("#000000", 0.07), ("#000000", 0.13)
EDGE, TEXT, SUBTLE, WHITE = "#3A3A3A", "#1B1B1B", "#5E5E5E", "#FFFFFF"
FILL = {"ok": "#E4E4E4", "warning": "#F4A640", "error": "#E4584B", "disabled": "#EFEFEF", "selected": "#97D697"}
OUTLINE = {"ok": EDGE, "warning": "#8A4A06", "error": "#7A1B12", "disabled": "#A9A9A9", "selected": "#2E8B2E"}
WIRE, WIRE_SELECTED = "#4A4A4A", "#2E9E2E"
PANEL_FILL, PANEL_EDGE, PANEL_PATH, PANEL_INDEX = "#FFF4B8", "#8C8460", "#7A5200", "#8A8A8A"
BUBBLE = {"warning": ("#F6B24C", "#8A4A06", TEXT), "error": ("#D8362B", "#6E140D", WHITE), "remark": ("#CBD6E2", "#4C6078", TEXT)}
GROUPS = ("#9FD0F2", "#F2C98B", "#B9E0A5", "#E8A8C9", "#D2C2F0")
PREVIEW_FILL, PREVIEW_EDGE, VIEWPORT_TOP, VIEWPORT_BOTTOM = "#7FD08A", "#1E7A3A", "#FAFAFA", "#CFCFCF"
RIBBON_TABS = ("Params", "Maths", "Sets", "Vector", "Curve", "Surface", "Mesh", "Intersect", "Transform", "Display")
TAB_TINTS = {"Params": "#E8B4A0", "Maths": "#A9C7E8", "Sets": "#C3D9A3", "Vector": "#E8D28A", "Curve": "#B7A7E0", "Surface": "#E0A7BD",
             "Mesh": "#9ED6CF", "Intersect": "#D8B98F", "Transform": "#A7B8C9", "Display": "#E3A5A5"}
KINDS = ("item", "list", "tree")
ROW, GRIP, PAD, SLIDER_H = 0.33, 0.085, 0.09, 0.42


def _get(obj, *names, default=None):
    """🔎 Read the first present key or attribute from a dict or object."""
    for name in names:
        if isinstance(obj, dict) and name in obj:
            return obj[name]
        if not isinstance(obj, dict) and hasattr(obj, name):
            return getattr(obj, name)
    return default


def _plain(value, language: str = "en") -> str:
    """🔤 Collapse a bilingual mapping to the Grasshopper UI string; the Grasshopper UI keeps English names in both languages."""
    return str(value.get("en", "")) if isinstance(value, dict) else str(value)


def path_text(path) -> str:
    """🧭 Grasshopper path notation, ``(0, 1)`` becomes ``{0;1}``."""
    return "{" + ";".join(str(int(i)) for i in path) + "}"


def branches(data) -> list:
    """🌳 Normalise a tree (engine ``Tree``, mapping or list of lists) to ``[(path, items)]``."""
    found = _get(data, "branches", default=data)
    found = found() if callable(found) else found
    if isinstance(found, dict):
        return [(tuple(path), list(items)) for path, items in found.items()]
    if found and isinstance(found[0], (list, tuple)) and len(found[0]) == 2 and isinstance(found[0][0], (tuple, list)):
        return [(tuple(path), list(items)) for path, items in found]
    return [((index,), list(items)) for index, items in enumerate(found)]


def semicircle(centre, facing: int = 1, radius: float = GRIP, fill=WHITE, stroke=EDGE):
    """🌓 Grip: a small half disc bulging to the left (-1) or right (+1) of an edge point."""
    cx, cy = P(centre)[:2]
    angles = np.linspace(-math.pi / 2, math.pi / 2, 14)
    return polygon([(cx + facing * radius * math.cos(a), cy + radius * math.sin(a)) for a in angles], fill, 1.0, stroke, W_THIN)


def bezier(start, end, count: int = 56) -> np.ndarray:
    """〰️ Grasshopper wire: cubic S-curve with horizontal tangents from an output grip to an input grip."""
    a, b = P(start), P(end)
    reach = max(abs(b[0] - a[0]) * 0.5, 0.45)
    c1, c2 = a + np.array([reach, 0, 0]), b - np.array([reach, 0, 0])
    t = np.linspace(0, 1, count)[:, None]
    return (1 - t) ** 3 * a + 3 * (1 - t) ** 2 * t * c1 + 3 * (1 - t) * t ** 2 * c2 + t ** 3 * b


def offset_curve(points: np.ndarray, distance: float) -> np.ndarray:
    """↔️ Parallel copy of a sampled curve at a signed distance along its left normal."""
    tangent = np.gradient(points[:, :2], axis=0)
    tangent /= np.maximum(np.linalg.norm(tangent, axis=1, keepdims=True), 1e-9)
    normal = np.stack([-tangent[:, 1], tangent[:, 0], np.zeros(len(points))], axis=1)
    return points + normal * distance


def clip_segment(a, b, box):
    """✂️ Liang-Barsky clip of a segment to ``(x0, y0, x1, y1)``; ``None`` when it lies outside."""
    (ax, ay), (bx, by) = P(a)[:2], P(b)[:2]
    t0, t1 = 0.0, 1.0
    for p, q in ((ax - bx, ax - box[0]), (bx - ax, box[2] - ax), (ay - by, ay - box[1]), (by - ay, box[3] - ay)):
        if p == 0:
            if q < 0:
                return None
            continue
        r = q / p
        if p < 0:
            t0 = max(t0, r)
        else:
            t1 = min(t1, r)
    if t0 > t1:
        return None
    return P((ax + t0 * (bx - ax), ay + t0 * (by - ay))), P((ax + t1 * (bx - ax), ay + t1 * (by - ay)))


def capsule_points(centre, width: float, height: float, chamfer: float) -> list:
    """⬡ Hexagon-ish capsule outline used by floating parameters."""
    x, y = P(centre)[:2]
    w, h = width / 2, height / 2
    return [(x - w + chamfer, y + h), (x + w - chamfer, y + h), (x + w, y), (x + w - chamfer, y - h), (x - w + chamfer, y - h), (x - w, y)]


class Widget(_engine.VGroup):
    """🧩 A canvas widget: ``parts`` by name and grip anchors ``grip_in(name)`` / ``grip_out(name)`` that move with the group."""

    def __init__(self, *members):
        super().__init__(*members)
        self.parts, self.ins, self.outs, self.refs, self.body = {}, {}, {}, {}, None

    def grip_in(self, name=None):
        """⬅️ The input grip anchor by nickname, name or index; the first one by default."""
        return self._grip(self.ins, name)

    def grip_out(self, name=None):
        """➡️ The output grip anchor by nickname, name or index; the first one by default."""
        return self._grip(self.outs, name)

    @staticmethod
    def _grip(table: dict, name):
        unique = list(dict.fromkeys(id(dot) for dot in table.values()))
        if name in table:
            return table[name]
        if name is None or len(unique) == 1:
            return next(iter(table.values()))
        raise KeyError(f"widget has no grip {name!r}")

    def anchor(self, kind: str, key, point, *aliases):
        """⚓ Register an invisible anchor under every alias."""
        dot = _engine.Dot(P(point), radius=0.004, fill_opacity=0, stroke_width=0)
        self.add(dot)
        table = {"in": self.ins, "out": self.outs}.get(kind, self.refs)
        for alias in (key, *aliases):
            table.setdefault(alias, dot)
        return dot

    def fit(self, content, margin: float = 0.2):
        """🧲 Scale and centre ``content`` inside the widget body."""
        target = self.body
        factor = min((target.width - 2 * margin) / max(content.width, 1e-6), (target.height - 2 * margin) / max(content.height, 1e-6))
        content.scale(factor).move_to(target.get_center())
        return content


class Canvas:
    """🦗 Scene mixin: Grasshopper 1 canvas widgets in the Rhino 8 look. Needs ``text``, ``mono``, ``number`` and ``tr`` from the clip scene."""

    def _gh_cap(self, size: float, mono: bool = False) -> float:
        key = ("cap", size, mono)
        if key not in _CAPS:
            _CAPS[key] = (self.mono if mono else self.text)("H", (0, 0, 0), size=size).height
        return _CAPS[key]

    def _gh_put(self, string: str, x: float, y: float, *, size=S_SMALL, color=TEXT, anchor="c", mono=False):
        shape = (self.mono if mono else self.text)(string, (0, 0, 0), size=size, color=color)
        return place(shape, (x, y - self._gh_cap(size, mono) / 2), "".join(c for c in anchor if c in "lr") + "b")

    def _gh_width(self, string: str, size=S_SMALL, mono=False) -> float:
        return (self.mono if mono else self.text)(string, (0, 0, 0), size=size).width

    def _gh_finish(self, widget, at, scale, anchor):
        if scale != 1:
            widget.scale(scale, about_point=np.zeros(3))
        if anchor == "c" and widget.body is not None:
            return widget.shift(P(at) - widget.body.get_center())
        return place(widget, at, anchor)

    def _gh_spec(self, spec) -> dict:
        if isinstance(spec, str):
            catalogue = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️").catalogue()
            spec = catalogue[spec]
        params = lambda key: [{"name": _plain(_get(p, "name", default="")), "nick": _plain(_get(p, "nickname", "nick", "name", default="")),
                               "type": _plain(_get(p, "type", default="")), "access": _get(p, "access", default="item")} for p in (_get(spec, key, default=()) or ())]
        name = _plain(_get(spec, "name", default=""))
        return {"name": name, "nick": _plain(_get(spec, "nickname", "nick", default=name)), "category": _plain(_get(spec, "category", default="")),
                "inputs": params("inputs"), "outputs": params("outputs")}

    def gh_component(self, spec, *, at=(0, 0, 0), scale: float = 1.0, state: str = "ok", nickname: bool = False, selected: bool = False, anchor: str = "c"):
        """⚙️ A component: rounded box, input nicknames left, name (or nickname) in the middle band, outputs right, grips on both edges.
        ``spec`` is a catalogue key or a spec with ``name``, ``nickname``, ``inputs``, ``outputs``. Parts: ``body``, ``centre``, ``inputs[nick]``, ``outputs[nick]``."""
        spec = self._gh_spec(spec)
        look = "selected" if selected else state
        ins, outs = spec["inputs"], spec["outputs"]
        words = spec["name"].split(" ")
        half = max(1, len(words) // 2)
        lines = [spec["nick"]] if nickname else [spec["name"]] if len(words) == 1 else [" ".join(words[:half]), " ".join(words[half:])]
        left = max([self._gh_width(p["nick"]) for p in ins], default=0.0) + 2 * PAD if ins else 0.18
        right = max([self._gh_width(p["nick"]) for p in outs], default=0.0) + 2 * PAD if outs else 0.18
        middle = max(self._gh_width(line) for line in lines) + 2 * PAD
        rows = max(len(ins), len(outs), 1)
        height = max(rows * ROW, len(lines) * 0.26) + 0.14
        width = left + middle + right
        widget = Widget()
        faded = 0.5 if state == "disabled" else 1.0
        body = _engine.RoundedRectangle(corner_radius=0.06, width=width, height=height, fill_color=FILL[look], fill_opacity=faded, stroke_color=OUTLINE[look], stroke_width=W_EDGE, stroke_opacity=faded)
        centre = _engine.RoundedRectangle(corner_radius=0.05, width=middle, height=height - 0.1, fill_color=mix(FILL[look], WHITE, 0.55), fill_opacity=faded,
                                          stroke_color=OUTLINE[look], stroke_width=W_HAIR, stroke_opacity=0.6 * faded)
        centre.shift(np.array([-width / 2 + left + middle / 2, 0, 0]))
        widget.add(body, centre)
        widget.body, widget.parts = body, {"body": body, "centre": centre, "inputs": {}, "outputs": {}}
        ink = SUBTLE if state == "disabled" else TEXT
        cx = -width / 2 + left + middle / 2
        for index, line in enumerate(lines):
            widget.add(self._gh_put(line, cx, (len(lines) - 1) / 2 * 0.26 - index * 0.26, color=ink))
        for column, params, x0, facing, kind in ((ins, ins, -width / 2, -1, "in"), (outs, outs, width / 2, 1, "out")):
            for index, param in enumerate(column):
                y = (len(params) - 1) / 2 * ROW - index * ROW
                nick = self._gh_put(param["nick"], x0 + (PAD if kind == "in" else -PAD), y, color=ink, anchor="l" if kind == "in" else "r")
                grip = semicircle((x0, y), facing, fill=mix(FILL[look], WHITE, 0.6), stroke=OUTLINE[look])
                widget.add(grip, nick)
                widget.anchor(kind, param["name"], (x0 + facing * GRIP, y), param["nick"], index)
                widget.parts["inputs" if kind == "in" else "outputs"][param["nick"]] = nick
        widget.state, widget.spec = look, spec
        return self._gh_finish(widget, at, scale, anchor)

    def gh_param(self, type: str, *, at=(0, 0, 0), label: str | None = None, state: str = "ok", selected: bool = False, scale: float = 1.0, caption: bool = True, anchor: str = "c"):
        """💊 Floating parameter: a hexagon-ish capsule with the nickname inside, the data type under it, and a grip on each tip."""
        look = "selected" if selected else state
        text = label or type
        width, height = self._gh_width(text) + 0.7, 0.42
        widget = Widget()
        capsule = polygon(capsule_points((0, 0), width, height, 0.14), FILL[look], 1.0, OUTLINE[look], W_EDGE)
        widget.add(capsule)
        widget.body, widget.parts = capsule, {"body": capsule}
        label_mob = self._gh_put(text, 0, 0, color=TEXT)
        widget.add(label_mob)
        widget.parts["label"] = label_mob
        for facing, kind in ((-1, "in"), (1, "out")):
            widget.add(semicircle((facing * width / 2, 0), facing, fill=mix(FILL[look], WHITE, 0.6), stroke=OUTLINE[look]))
            widget.anchor(kind, text, (facing * (width / 2 + GRIP), 0), type, 0)
        if caption and label and label != type:
            tag = self._gh_put(type, 0, -height / 2 - 0.2, color=SUBTLE)
            widget.add(tag)
            widget.parts["type"] = tag
        return self._gh_finish(widget, at, scale, anchor)

    def gh_slider(self, name: str, value: float, lo: float, hi: float, *, at=(0, 0, 0), decimals: int = 2, tracker=None, width: float | None = None,
                  scale: float = 1.0, selected: bool = False, unit: str = "", anchor: str = "c"):
        """🎚️ Number Slider: name cell, value cell, track with a knob, the domain under the track and an output grip. A ``ValueTracker`` moves knob and value live."""
        read = (lambda: float(tracker.get_value())) if tracker is not None else (lambda: float(value))
        look = "selected" if selected else "ok"
        sample = max((self.number(v, decimals) + (" " + unit if unit else "") for v in (lo, hi, value)), key=len)
        nw, vw = max(self._gh_width(name) + 0.2, 0.6), self._gh_width(sample) + 0.2
        span = self._gh_width(self.number(lo, decimals)) + self._gh_width(self.number(hi, decimals)) + 0.3
        total = max(width or 0.0, nw + vw + 0.32 + span) if width else nw + vw + 0.32 + max(1.2, span)
        tw = total - nw - vw
        x0, h = -total / 2, SLIDER_H
        t0, t1 = x0 + nw + vw + 0.16, x0 + total - 0.16
        widget = Widget()
        frame = _engine.RoundedRectangle(corner_radius=0.07, width=total, height=h, fill_color="#F7F7F7", fill_opacity=1, stroke_color=OUTLINE[look], stroke_width=W_EDGE)
        frame.shift(np.array([0, 0, 0]))
        rules = segments([(P((x0 + nw, h / 2)), P((x0 + nw, -h / 2))), (P((x0 + nw + vw, h / 2)), P((x0 + nw + vw, -h / 2)))], OUTLINE[look], W_HAIR, 0.7)
        title = self._gh_put(name, x0 + nw / 2, 0, color=TEXT)
        track = segments([(P((t0, 0)), P((t1, 0)))], "#B4B4B4", 5.0)
        low = self._gh_put(self.number(lo, decimals), t0, -h / 2 - 0.2, color=SUBTLE, anchor="l")
        high = self._gh_put(self.number(hi, decimals), t1, -h / 2 - 0.2, color=SUBTLE, anchor="r")
        widget.add(frame, rules, title, track, low, high)
        widget.body, widget.parts = frame, {"body": frame, "name": title, "track": track, "domain": _engine.VGroup(low, high)}

        first, last = widget.anchor("ref", "track0", (t0, 0)), widget.anchor("ref", "track1", (t1, 0))

        def moving():
            current = read()
            fraction = 0.0 if hi == lo else min(max((current - lo) / (hi - lo), 0.0), 1.0)
            kx = t0 + fraction * (t1 - t0)
            fill = segments([(P((t0, 0)), P((kx, 0)))], "#5E5E5E", 5.0)
            knob = _engine.RoundedRectangle(corner_radius=0.03, width=0.13, height=h - 0.1, fill_color="#2B2B2B", fill_opacity=1, stroke_color=WHITE, stroke_width=W_HAIR).move_to(P((kx, 0)))
            text = self._gh_put(self.number(current, decimals) + (" " + unit if unit else ""), x0 + nw + vw / 2, 0, color=TEXT)
            group = _engine.VGroup(fill, knob, text)
            factor = (last.get_center()[0] - first.get_center()[0]) / (t1 - t0)
            group.scale(factor, about_point=np.zeros(3)).shift(first.get_center() - factor * P((t0, 0)))
            group.fill, group.knob, group.value = fill, knob, text
            return group

        live = self.live(moving, key=lambda: (round(read(), 6), tuple(np.round(first.get_center(), 4)), tuple(np.round(last.get_center(), 4))))
        widget.add(live)
        widget.parts["live"] = live
        widget.add(semicircle((x0 + total, 0), 1, fill="#DADADA", stroke=OUTLINE[look]))
        widget.anchor("out", name, (x0 + total + GRIP, 0), 0)
        widget.range = (t0, t1)
        return self._gh_finish(widget, at, scale, anchor)

    def _gh_panel_rows(self, data, paths: bool, fmt) -> list:
        show = fmt or (lambda item: self.number(item, 3) if isinstance(item, float) else str(item))
        if isinstance(data, str):
            data = data.split("\n")
        tree = hasattr(data, "branches") or isinstance(data, dict)
        if not tree:
            return [("line", "", show(item)) for item in data]
        rows = []
        for path, items in branches(data):
            rows.append(("path", "", path_text(path)))
            rows.extend(("item", str(index), show(item)) for index, item in enumerate(items))
        return rows if paths else [row for row in rows if row[0] != "path"]

    def gh_panel(self, lines, *, at=(0, 0, 0), width: float | None = None, paths: bool = True, scale: float = 1.0, max_lines: int = 14, selected: bool = False,
                 fmt=None, anchor: str = "c", rows=None, max_width: float = 3.6):
        """📝 Yellow Panel. ``lines`` is text, a list of items or a ``Tree``: a tree shows ``{0;1}`` path headers and ``0 …`` item indices like Grasshopper.
        ``rows`` is a ready list of ``(kind, index text, text)`` with kind ``line|path|item``; wider text than ``max_width`` is cut with dots."""
        rows = list(rows) if rows is not None else self._gh_panel_rows(lines, paths, fmt)
        if len(rows) > max_lines:
            rows = rows[:max_lines - 1] + [("line", "", "...")]
        char = self._gh_width("0000000000", mono=True) / 10
        indent = (max((len(r[1]) for r in rows), default=0) + 1) * char + 0.12 if any(r[1] for r in rows) else 0.0
        need = max((indent + len(r[2]) * char for r in rows), default=char) + 0.34
        total = width or min(max(need, 1.0), max_width)
        fit = max(int((total - 0.34 - indent) / char), 3) if need > total + 1e-6 else 10 ** 6
        pitch = 0.27
        height = len(rows) * pitch + 0.2
        look = "selected" if selected else "ok"
        widget = Widget()
        box = _engine.Rectangle(width=total, height=height, fill_color=PANEL_FILL, fill_opacity=1, stroke_color=OUTLINE["selected"] if selected else PANEL_EDGE, stroke_width=W_EDGE)
        widget.add(box)
        widget.body, widget.parts = box, {"body": box, "rows": []}
        for index, (kind, number, text) in enumerate(rows):
            y = height / 2 - 0.1 - pitch * (index + 0.5)
            shown = text if len(text) <= fit else text[:max(fit - 3, 1)] + "..."
            if number:
                widget.add(self._gh_put(number, -total / 2 + 0.17, y, size=S_SMALL, color=PANEL_INDEX, anchor="l", mono=True))
            line = self._gh_put(shown, -total / 2 + 0.17 + (0.0 if kind == "path" else indent), y, color=PANEL_PATH if kind == "path" else TEXT, anchor="l", mono=True)
            widget.add(line)
            widget.parts["rows"].append(line)
        for facing, kind in ((-1, "in"), (1, "out")):
            widget.add(semicircle((facing * total / 2, 0), facing, fill="#F4EBB0", stroke=PANEL_EDGE))
            widget.anchor(kind, "Panel", (facing * (total / 2 + GRIP), 0), 0)
        return self._gh_finish(widget, at, scale, anchor)

    def gh_toggle(self, value: bool, *, at=(0, 0, 0), name: str = "Toggle", scale: float = 1.0, anchor: str = "c"):
        """🔘 Boolean Toggle: a name cell, a check box and the word True or False, with an output grip."""
        text = "True" if value else "False"
        nw, vw = self._gh_width(name) + 2 * PAD, self._gh_width("False") + 0.7
        total, h = nw + vw, SLIDER_H
        widget = Widget()
        frame = _engine.RoundedRectangle(corner_radius=0.07, width=total, height=h, fill_color="#F7F7F7", fill_opacity=1, stroke_color=EDGE, stroke_width=W_EDGE)
        widget.add(frame, segments([(P((-total / 2 + nw, h / 2)), P((-total / 2 + nw, -h / 2)))], EDGE, W_HAIR, 0.7))
        widget.add(self._gh_put(name, -total / 2 + nw / 2, 0))
        box = _engine.Square(0.2, fill_color="#3C9F4A" if value else WHITE, fill_opacity=1, stroke_color=EDGE, stroke_width=W_THIN).move_to(P((-total / 2 + nw + 0.2, 0)))
        widget.add(box, self._gh_put(text, -total / 2 + nw + 0.38, 0, anchor="l"))
        widget.add(semicircle((total / 2, 0), 1, fill="#DADADA"))
        widget.anchor("out", name, (total / 2 + GRIP, 0), 0)
        widget.body, widget.parts = frame, {"body": frame, "box": box}
        return self._gh_finish(widget, at, scale, anchor)

    def gh_value_list(self, items, selected: int = 0, *, at=(0, 0, 0), name: str = "Value List", is_open: bool = False, scale: float = 1.0, anchor: str = "c"):
        """📜 Value List: the chosen item with a drop arrow, or the opened list with the chosen row highlighted."""
        items = [_plain(item) for item in items]
        total = max(self._gh_width(text) for text in (*items, name)) + 0.9
        h = SLIDER_H
        widget = Widget()
        shown = items if is_open else [items[selected]]
        top = (len(shown) * h) / 2 if is_open else h / 2
        frame = _engine.RoundedRectangle(corner_radius=0.07, width=total, height=h * len(shown), fill_color="#F7F7F7", fill_opacity=1, stroke_color=EDGE, stroke_width=W_EDGE)
        widget.add(frame)
        rows = []
        for index, text in enumerate(shown):
            y = top - h * (index + 0.5)
            chosen = (index == selected) if is_open else True
            if is_open and chosen:
                widget.add(_engine.Rectangle(width=total - 0.08, height=h - 0.06, fill_color="#97D697", fill_opacity=1, stroke_width=0).move_to(P((0, y))))
            rows.append(self._gh_put(text, -total / 2 + PAD, y, anchor="l"))
        widget.add(*rows)
        if not is_open:
            arrow = polygon([(total / 2 - 0.32, 0.05), (total / 2 - 0.14, 0.05), (total / 2 - 0.23, -0.07)], EDGE, 1.0, EDGE, W_HAIR)
            widget.add(arrow)
        widget.add(semicircle((total / 2, top - h / 2), 1, fill="#DADADA"))
        widget.anchor("out", name, (total / 2 + GRIP, top - h / 2), 0)
        widget.body, widget.parts = frame, {"body": frame, "rows": rows}
        return self._gh_finish(widget, at, scale, anchor)

    def gh_wire(self, out_grip, in_grip, *, kind: str = "item", color=None, selected: bool = False):
        """🧵 Wire from an output grip to an input grip: item thin solid, list double line, tree dashed double line, selected green. Grip mobjects keep it attached while they move."""
        if kind not in KINDS:
            raise ValueError(f"wire kind must be one of {KINDS}")
        tone = color or (WIRE_SELECTED if selected else WIRE)
        point = lambda grip: np.array(grip.get_center()) if hasattr(grip, "get_center") else P(grip)

        def build():
            curve = bezier(point(out_grip), point(in_grip))
            if kind == "item":
                lines = [polyline(curve, tone, W_EDGE)]
            else:
                lines = [polyline(offset_curve(curve, d), tone, 1.5, dash=0.13 if kind == "tree" else None) for d in (0.024, -0.024)]
            group = _engine.VGroup(*lines)
            group.set_z_index(-1)
            group.kind, group.curve = kind, curve
            return group

        movable = hasattr(out_grip, "get_center") or hasattr(in_grip, "get_center")
        return self.live(build, key=lambda: (tuple(np.round(point(out_grip), 3)), tuple(np.round(point(in_grip), 3)))) if movable else build()

    def gh_bubble(self, text: str, *, level: str = "warning", target=None, at=None, width: float = 3.6, side: str = "above", scale: float = 1.0):
        """💬 Balloon with the warning, error or remark message, tail pointing at ``target`` (a widget, mobject or point)."""
        fill, edge, ink = BUBBLE[level]
        lines = _fonts.wrap_body_lines(text, font_size=S_SMALL, max_width=width - 0.8)
        pitch = 0.27
        height, total = len(lines) * pitch + 0.26, max(self._gh_width(line) for line in lines) + 0.85
        widget = Widget()
        box = _engine.RoundedRectangle(corner_radius=0.08, width=total, height=height, fill_color=fill, fill_opacity=1, stroke_color=edge, stroke_width=W_EDGE)
        icon = _engine.Circle(radius=0.13, fill_color=WHITE if level != "remark" else edge, fill_opacity=1, stroke_color=edge, stroke_width=W_THIN).move_to(P((-total / 2 + 0.28, 0)))
        mark = self._gh_put("i" if level == "remark" else "!", -total / 2 + 0.28, 0, color=WHITE if level == "remark" else edge)
        body = [self._gh_put(line, -total / 2 + 0.52, (len(lines) - 1) / 2 * pitch - index * pitch, color=ink, anchor="l") for index, line in enumerate(lines)]
        tail_dir = {"above": -1, "below": 1}.get(side, -1)
        tail = polygon([(-0.14, tail_dir * height / 2), (0.14, tail_dir * height / 2), (0, tail_dir * (height / 2 + 0.2))], fill, 1.0, edge, W_EDGE)
        cover = segments([(P((-0.12, tail_dir * height / 2)), P((0.12, tail_dir * height / 2)))], fill, W_EDGE + 1)
        widget.add(tail, box, cover, icon, mark, *body)
        tip = widget.anchor("ref", "tip", (0, tail_dir * (height / 2 + 0.2)))
        widget.body, widget.parts, widget.level = box, {"body": box, "text": _engine.VGroup(*body)}, level
        if scale != 1:
            widget.scale(scale, about_point=np.zeros(3))
        if at is not None:
            return place(widget, at, "c")
        mob = target if hasattr(target, "get_center") else None
        goal = P(target) if mob is None else np.array(mob.get_top() if side == "above" else mob.get_bottom())
        return widget.shift(goal + np.array([0, 0.04 if side == "above" else -0.04, 0]) - np.array(tip.get_center()))

    def gh_ribbon(self, tabs=RIBBON_TABS, active: str = "Params", *, at=(0, 0, 0), width: float = 12.6, groups=(), highlight=None, anchor: str = "c"):
        """🗂️ Ribbon: the tab row (with plugin tabs) and a strip of component icons for the active tab. ``groups`` is ``[(title, [names])]``. Parts: ``tabs[name]``, ``tiles[name]``."""
        sizes = [self._gh_width(tab) + 0.34 for tab in tabs]
        h_tab, h_strip = 0.4, 1.05
        widget = Widget()
        strip = _engine.Rectangle(width=max(sum(sizes), 0.1), height=h_strip, fill_color="#F4F4F4", fill_opacity=1, stroke_color=EDGE, stroke_width=W_THIN)
        strip.move_to(P((0, -h_tab / 2 - h_strip / 2 + 0.0)))
        widget.add(strip)
        widget.parts = {"body": strip, "tabs": {}, "tiles": {}}
        widget.body = strip
        x = -sum(sizes) / 2
        for tab, size in zip(tabs, sizes):
            on = tab == active
            cell = _engine.Rectangle(width=size, height=h_tab, fill_color=WHITE if on else "#D9D9D9", fill_opacity=1, stroke_color=EDGE, stroke_width=W_THIN).move_to(P((x + size / 2, 0)))
            widget.add(cell, self._gh_put(tab, x + size / 2, 0, color=TEXT if on else SUBTLE))
            widget.parts["tabs"][tab] = cell
            x += size
        gx = -sum(sizes) / 2 + 0.2
        for title, names in groups:
            start = gx
            for name in names:
                tile = _engine.RoundedRectangle(corner_radius=0.05, width=0.46, height=0.46, fill_color=TAB_TINTS.get(active, "#CCCCCC"), fill_opacity=1, stroke_color=EDGE, stroke_width=W_THIN)
                tile.move_to(P((gx + 0.23, -h_tab / 2 - 0.34)))
                initials = "".join(word[0] for word in name.split())[:2] or name[:2]
                widget.add(tile, self._gh_put(initials, gx + 0.23, -h_tab / 2 - 0.34, color=TEXT))
                widget.parts["tiles"][name] = tile
                if name == highlight:
                    widget.add(_engine.RoundedRectangle(corner_radius=0.07, width=0.56, height=0.56, stroke_color=PREVIEW_EDGE, stroke_width=W_EDGE * 1.6, fill_opacity=0).move_to(tile))
                gx += 0.54
            widget.add(self._gh_put(title, (start + gx - 0.08) / 2, -h_tab / 2 - h_strip + 0.19, color=SUBTLE))
            widget.add(segments([(P((gx + 0.04, -h_tab / 2 - 0.08)), P((gx + 0.04, -h_tab / 2 - h_strip + 0.1)))], "#B0B0B0", W_HAIR))
            gx += 0.2
        if widget.width > width:
            widget.scale_to_fit_width(width)
        return place(widget, at, anchor)

    def gh_param_viewer(self, tree, *, at=(0, 0, 0), scale: float = 1.0, max_items: int = 8, max_rows: int = 6, anchor: str = "c"):
        """🔬 Param Viewer: the tree drawn as a trunk with one row per branch (path, one dot per item, item count)."""
        rows = branches(tree)
        more = len(rows) - max_rows + 1 if len(rows) > max_rows else 0
        rows = rows[:max_rows - 1] + [((), [])] if more else rows
        paths = [path_text(path) if path else "..." for path, _ in rows]
        path_w = max(self._gh_width(p, mono=True) for p in paths) if paths else 0.5
        dot_w = 0.2 * max(min(len(items), max_items) for _, items in rows) if rows else 0.2
        pitch = 0.34
        total = 0.5 + path_w + dot_w + 1.0
        height = len(rows) * pitch + 0.26
        widget = Widget()
        box = _engine.RoundedRectangle(corner_radius=0.06, width=total, height=height, fill_color="#F7F7F7", fill_opacity=1, stroke_color=EDGE, stroke_width=W_EDGE)
        widget.add(box)
        widget.body, widget.parts = box, {"body": box, "rows": {}}
        trunk_x = -total / 2 + 0.22
        top, bottom = height / 2 - 0.13 - pitch / 2, -height / 2 + 0.13 + pitch / 2
        if len(rows) > 1:
            widget.add(segments([(P((trunk_x, top)), P((trunk_x, bottom)))], EDGE, W_THIN))
        for index, ((path, items), text) in enumerate(zip(rows, paths)):
            y = height / 2 - 0.13 - pitch * (index + 0.5)
            widget.add(segments([(P((trunk_x, y)), P((trunk_x + 0.14, y)))], EDGE, W_THIN))
            label = self._gh_put(text, trunk_x + 0.2, y, anchor="l", mono=True)
            x = trunk_x + 0.3 + path_w
            dots = _engine.VGroup(*[_engine.Dot(P((x + 0.1 + 0.2 * k, y)), radius=0.06, color="#2B2B2B") for k in range(min(len(items), max_items))])
            count = self._gh_put(f"+{more}" if not path else str(len(items)), total / 2 - 0.16, y, color=SUBTLE, anchor="r")
            widget.add(label, dots, count)
            widget.parts["rows"][path] = _engine.VGroup(label, dots, count)
        widget.add(semicircle((-total / 2, 0), -1, fill="#DADADA"))
        widget.anchor("in", "Param Viewer", (-total / 2 - GRIP, 0), 0)
        return self._gh_finish(widget, at, scale, anchor)

    def gh_group(self, members, title: str, *, color: str | None = None, pad: float = 0.22, index: int = 0):
        """🟦 Coloured group box with its title around the ``members``; it draws behind everything."""
        tone = color or GROUPS[index % len(GROUPS)]
        box = _engine.VGroup(*members) if not isinstance(members, _engine.Mobject) else members
        head = 0.36
        frame = _engine.RoundedRectangle(corner_radius=0.1, width=box.width + 2 * pad, height=box.height + 2 * pad + head, fill_color=tone, fill_opacity=0.5,
                                         stroke_color=mix(tone, "#000000", 0.45), stroke_width=W_EDGE)
        frame.move_to(box.get_center() - np.array([0, head / 2, 0]))
        name = self._gh_put(title, frame.get_left()[0] + 0.16, frame.get_bottom()[1] + 0.2, color=TEXT, anchor="l")
        widget = Widget(frame, name)
        widget.body, widget.parts = frame, {"body": frame, "title": name}
        widget.set_z_index(-2)
        return widget

    def gh_background(self, *, at=(0, 0, 0), width: float = 13.0, height: float = 4.6, grid: float = 0.5):
        """🧱 The light grey canvas with its subtle grid (every fifth line stronger)."""
        widget = Widget()
        sheet = _engine.Rectangle(width=width, height=height, fill_color=CANVAS, fill_opacity=1, stroke_color=EDGE, stroke_width=W_THIN)
        widget.add(sheet)
        columns, lines = int(width / grid / 2), int(height / grid / 2)
        for kind, (color, opacity), w in ((False, GRID_MINOR, W_HAIR), (True, GRID_MAJOR, W_THIN)):
            pairs = []
            for i in range(-columns, columns + 1):
                if (i % 5 == 0) == kind and abs(i * grid) < width / 2:
                    pairs.append((P((i * grid, -height / 2)), P((i * grid, height / 2))))
            for j in range(-lines, lines + 1):
                if (j % 5 == 0) == kind and abs(j * grid) < height / 2:
                    pairs.append((P((-width / 2, j * grid)), P((width / 2, j * grid))))
            widget.add(segments(pairs, color, w, opacity))
        widget.set_z_index(-3)
        widget.body, widget.parts = sheet, {"body": sheet}
        return place(widget, at, "c")

    def gh_viewport(self, *, at=(0, 0, 0), width: float = 6.0, height: float = 4.0, label: str = "Perspective", active: bool = True):
        """🪟 Rhino 8 viewport: light gradient, ground grid with red X and green Y axes, title tab, and world axes bottom left. ``fit(content)`` scales a drawing inside; ``origin`` is the world origin."""
        widget = Widget()
        sky = _engine.Rectangle(width=width, height=height, stroke_width=0).set_fill([VIEWPORT_TOP, VIEWPORT_BOTTOM], opacity=1).set_sheen_direction(_engine.DOWN)
        widget.add(sky)
        frame = _engine.Rectangle(width=width, height=height, stroke_color="#3F6FB5" if active else "#8A8A8A", stroke_width=W_EDGE * 1.4, fill_opacity=0)
        view = _drafting.View("axon", (0, -height * 0.12, 0), height / 16)
        reach = 8
        inside = (-width / 2, -height / 2, width / 2, height / 2 - 0.05)
        clipped = lambda pairs: [c for c in (clip_segment(a, b, inside) for a, b in pairs) if c]
        grid = [(view(i, -reach, 0), view(i, reach, 0)) for i in range(-reach, reach + 1) if i]
        grid += [(view(-reach, i, 0), view(reach, i, 0)) for i in range(-reach, reach + 1) if i]
        widget.add(segments(clipped(grid), "#BDBDBD", W_HAIR, 0.85))
        widget.add(segments(clipped([(view(-reach, 0, 0), view(reach, 0, 0))]), "#D9534F", W_THIN, 0.9), segments(clipped([(view(0, -reach, 0), view(0, reach, 0))]), "#4FB06A", W_THIN, 0.9))
        tab = self._gh_put(label, -width / 2 + 0.2, height / 2 - 0.22, color=TEXT, anchor="l")
        widget.add(tab)
        corner = P((-width / 2 + 0.42, -height / 2 + 0.38))
        triad = _drafting.View("axon", corner, 0.2)
        for vector, tone, name in (((1, 0, 0), "#D9534F", "x"), ((0, 1, 0), "#4FB06A", "y"), ((0, 0, 1), "#3F6FB5", "z")):
            tip = triad(*vector)
            widget.add(segments([(triad(0, 0, 0), tip)], tone, W_EDGE), _engine.Dot(tip, radius=0.03, color=tone))
        widget.add(frame)
        widget.body, widget.parts, widget.origin = frame, {"body": frame, "title": tab}, np.array(view(0, 0, 0))
        widget.view = view
        return place(widget, at, "c")

    def gh_canvas(self, definition, solution, *, view: str | None = None, at=(0, 0, 0), frame=None, scale: float = 1.0, only=None, gap=(0.45, 0.3), max_lines: int = 10, pixels: float = 135.0):
        """🕸️ A whole canvas (or one named ``view``) from the engine: every object as its widget, wires from the data (item thin, list double, tree dashed double),
        component states and message bubbles from the ``solution``, panels showing the real tree. Objects keep the engine columns (x) and order (y)
        with widths and heights from the actual widgets (a column is stacked downwards where objects would touch). With ``frame=(width, height)`` a grey canvas sheet is drawn and the content is scaled (never enlarged) to fit inside.
        ``pixels`` converts engine canvas units (y down) to scene units before overlaps are pushed apart. ``canvas.obj(id)`` finds a widget; ``canvas.sheet``, ``canvas.factor``, ``canvas.wires`` and ``canvas.bubbles[id]`` are kept."""
        objects = definition.view(view) if view else tuple(definition.objects.values())
        wires = definition.view_wires(view) if view else tuple(definition.wires)
        wanted = None if only is None else set(only)
        objects = tuple(o for o in objects if wanted is None or o.id in wanted)
        made = {o.id: self._gh_object(definition, solution, o, max_lines) for o in objects}
        bubbles = {o.id: self._gh_message(solution, o.id, made[o.id]) for o in objects}
        xs = sorted({o.at[0] for o in objects})
        widths = [max(made[o.id].width for o in objects if o.at[0] == x) for x in xs]
        left = np.concatenate([[0.0], np.cumsum([w + gap[0] for w in widths])])
        content = Widget()
        for index, x in enumerate(xs):
            floor = 1e9
            for o in sorted((o for o in objects if o.at[0] == x), key=lambda o: o.at[1]):
                lift = bubbles[o.id].height + 0.3 if bubbles[o.id] is not None else 0.0
                height = made[o.id].height
                centre = min(-o.at[1] / pixels, floor - lift - height / 2)
                floor = centre - height / 2 - gap[1]
                made[o.id].shift(P((left[index] + widths[index] / 2, centre)) - made[o.id].get_center())
                content.add(made[o.id])
        for o in objects:
            if bubbles[o.id] is not None:
                self._gh_tail(bubbles[o.id], made[o.id], [m for k, m in made.items() if k != o.id])
                content.add(bubbles[o.id])
        drawn = []
        for wire in wires:
            if wire.src in made and wire.dst in made:
                tree = solution.tree(wire.src, wire.out)
                kind = "tree" if tree.path_count > 1 else "list" if tree.count > 1 else "item"
                drawn.append(self.gh_wire(made[wire.src].grip_out(wire.out), made[wire.dst].grip_in(wire.inp), kind=kind))
                content.add(drawn[-1])
        content.objects, content.bubbles, content.obj, content.factor, content.sheet, content.wires = made, bubbles, made.__getitem__, 1.0, None, drawn
        content.shift(-content.get_center())
        if frame:
            room = (frame[0] - 0.5, frame[1] - 0.5)
            content.factor = min(1.0, room[0] / content.width, room[1] / content.height)
            if content.factor < 0.8 and max_lines > 4:
                return self.gh_canvas(definition, solution, view=view, at=at, frame=frame, scale=scale, only=only, gap=gap, max_lines=max_lines - 2, pixels=pixels)
            content.scale(content.factor, about_point=np.zeros(3))
            content.sheet = self.gh_background(width=frame[0], height=frame[1])
            content.add(content.sheet)
        elif scale != 1:
            content.scale(scale, about_point=np.zeros(3))
        return content.shift(P(at))

    def _gh_first(self, solution, oid, output=None):
        tree = solution.tree(oid, output)
        return tree.items()[0] if tree.count else None

    def _gh_incoming(self, definition, solution, oid):
        for param in definition.spec(oid).inputs:
            for wire in definition.wires_into(oid, param.name):
                return solution.tree(wire.src, wire.out)
        return None

    def _gh_rows(self, tree, german: bool) -> list:
        number = (lambda text: re.sub(r"(?<=\d)\.(?=\d)", ",", text)) if german else (lambda text: text)
        rows = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️").panel_rows(tree, tree.path_count > 1)
        out, last = [], None
        for path, index, text in rows:
            if path is not None and path != last:
                out.append(("path", "", path))
                last = path
            out.append(("item" if path is not None else "line", str(index) if path is not None else "", number(text)))
        return out

    def _gh_object(self, definition, solution, obj, max_lines: int):
        spec, cfg = definition.spec(obj.id), obj.settings
        kind, state = spec.kind, solution.state(obj.id)
        if kind == "slider":
            integer = cfg.get("rounding") == "integer"
            return self.gh_slider(cfg.get("label", spec.name), self._gh_first(solution, obj.id), cfg["min"], cfg["max"], decimals=0 if integer else int(cfg.get("decimals", 2)))
        if kind == "toggle":
            return self.gh_toggle(bool(self._gh_first(solution, obj.id)), name=cfg.get("label", "Toggle"))
        if kind == "valuelist":
            return self.gh_value_list([item["name"] for item in cfg["items"]], int(cfg.get("selected", 0)), name=spec.name)
        if kind == "param":
            return self.gh_param(spec.name.removesuffix(" Param"), label=cfg.get("label"), state=state)
        if kind == "viewer":
            tree = self._gh_incoming(definition, solution, obj.id)
            return self.gh_param_viewer(tree if tree is not None else {})
        if kind == "panel":
            tree = self._gh_incoming(definition, solution, obj.id)
            tree = tree if tree is not None else solution.tree(obj.id)
            rows = self._gh_rows(tree, self.language() == "de") if tree.count else [("line", "", "")]
            return self.gh_panel(None, rows=rows, max_lines=max_lines)
        return self.gh_component(spec, state=state)

    def _gh_message(self, solution, oid: str, target):
        found = solution.msgs(oid)
        for level in ("error", "warning", "remark"):
            for message in found:
                if message["level"] == level:
                    return self.gh_bubble(self.tr(message["en"], message["de"]), level=level, at=(0, 0, 0), width=4.2)
        return None

    def _gh_tail(self, bubble, target, others=()):
        bubble.shift(np.array(target.get_top()) + np.array([0, 0.04, 0]) - np.array(bubble.refs["tip"].get_center()))
        reach = max(target.width / 2 - 0.12, 0.0)
        for shift in sorted(np.linspace(-reach, reach, 9), key=abs):
            bubble.shift(np.array([shift, 0, 0]))
            box = (bubble.get_left()[0], bubble.get_bottom()[1] + 0.2, bubble.get_right()[0], bubble.get_top()[1])
            tip = bubble.refs["tip"].get_center()[0]
            free = all(m.get_right()[0] < box[0] or m.get_left()[0] > box[2] or m.get_top()[1] < box[1] or m.get_bottom()[1] > box[3] for m in others)
            if free and target.get_left()[0] + 0.1 <= tip <= target.get_right()[0] - 0.1:
                return
            bubble.shift(np.array([-shift, 0, 0]))


_CAPS: dict = {}
