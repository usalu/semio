"""🧮 Structural analysis views of the Villa Savoye: analysis model, load arrows, tributary cells and triangle meshes."""

from __future__ import annotations

import importlib
import math
from collections import Counter

import numpy as np

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_engine = importlib.import_module("manim")

P = _d.P
EPS = 1e-6
ARROW_KN = 0.003
ARROW_Q = 0.05
KINDS = ("interior", "edge", "corner", "cantilever")
KIND_COLORS = {"interior": _d.ARCH, "edge": _d.GOOD, "corner": _d.STRUCT, "cantilever": _d.mix(_d.SERV, _d.STRUCT, 0.45)}
KIND_NAMES = {"interior": ("interior", "innen"), "edge": ("edge", "Rand"), "corner": ("corner", "Ecke"), "cantilever": ("cantilever", "Kragarm")}
ID_KINDS = ("shells", "members", "nodes")
MESH_VIEWS = {"axon": "axon", "side": "side", "elevation": "side", "top": "top"}
_meshes: dict = {}


def piloti_kind(tributary: dict, entry: dict) -> str:
    """🏷️ Tributary class of one piloti: interior, edge, corner, or cantilever (an edge piloti whose cell carries the slab overhang)."""
    kind = entry.get("kind", "interior")
    if kind != "edge" or "x" not in entry:
        return kind
    xs, ys = sorted({p["x"] for p in tributary["pilotis"]}), sorted({p["y"] for p in tributary["pilotis"]})
    reach = tributary.get("cantilever", {})
    over = ((entry["x"] == xs[0] and reach.get("west", 0) > EPS) or (entry["x"] == xs[-1] and reach.get("east", 0) > EPS)
            or (entry["y"] == ys[0] and reach.get("south", 0) > EPS) or (entry["y"] == ys[-1] and reach.get("north", 0) > EPS))
    return "cantilever" if over else "edge"


def _names(selection) -> set:
    """🎯 A tag, a name or an iterable of them as a set."""
    return {selection} if isinstance(selection, str) else set(selection)


def _centroid(ring) -> np.ndarray:
    """⊕ Area-weighted centroid of a 2D polygon; the vertex mean when the area vanishes."""
    pts = np.asarray([p[:2] for p in ring], float)
    nxt = np.roll(pts, -1, axis=0)
    cross = pts[:, 0] * nxt[:, 1] - nxt[:, 0] * pts[:, 1]
    area = cross.sum() / 2
    if abs(area) < 1e-9:
        return pts.mean(axis=0)
    return np.array([((pts[:, 0] + nxt[:, 0]) * cross).sum(), ((pts[:, 1] + nxt[:, 1]) * cross).sum()]) / (6 * area)


def _mesh_of(mesh, segments, model):
    """🔺 A mesh dict from a mesh, an element key or a segment count; memoized per key and segments."""
    if isinstance(mesh, dict) and (segments is None or mesh.get("segments") == segments or "tag" not in mesh):
        return mesh
    key = mesh if isinstance(mesh, str) else mesh["tag"]
    count = 16 if segments is None else int(segments)
    slot = (key, count, id(model))
    if slot not in _meshes:
        if len(_meshes) > 48:
            _meshes.pop(next(iter(_meshes)))
        _meshes[slot] = _model.mesh(key, segments=count) if model is None else _model.mesh(key, segments=count, model=model)
    return _meshes[slot]


class Analysis:
    """🧮 Scene mixin: ``analysis_view``, ``load_arrows``, ``tributary_view``, ``tributary_legend``, ``mesh_view``."""

    def _note(self, string: str, point, *, size=_d.S_SMALL, color=_d.INK, anchor="c", halo: bool = True):
        """🏷️ A cached label: typeset once per string, copied per frame, so live redraws stay cheap; ``halo`` adds a background-coloured outline."""
        cache = self.__dict__.setdefault("_analysis_labels", {})
        key = (string, size, color, halo)
        if key not in cache:
            shape = self.text(string, (0, 0, 0), size=size, color=color)
            cache[key] = shape.set_stroke(_d.BACKGROUND, 5.0, 0.85, background=True) if halo else shape
        return _d.place(cache[key].copy(), point, anchor)

    def _declutter(self, notes) -> list:
        """🧹 Keep a label only while it does not overlap an earlier kept label."""
        boxes, kept = [], []
        for note in notes:
            box = (note.get_left()[0], note.get_right()[0], note.get_bottom()[1], note.get_top()[1])
            if all(box[0] >= other[1] or box[1] <= other[0] or box[2] >= other[3] or box[3] <= other[2] for other in boxes):
                boxes.append(box)
                kept.append(note)
        return kept

    def _projection(self, projection: str, point, scale: float, model):
        """🔭 The projection of ``villa_axon`` (``"axon"``) or a plan with the same origin convention; ``point`` is the box centre."""
        if projection == "axon":
            return self.axon_view(point, scale, model=model)
        if projection != "plan":
            raise ValueError("projection is plan or axon")
        box = _c.geometry(model)["box"]
        return _d.View("plan", point, scale, (box["length"] / 2, box["width"] / 2, 0.0))

    def _arrow(self, tail, tip, color, width: float = 2.6):
        """➡️ One arrow: shaft and filled head; shorter than a head it collapses to a scaled head."""
        tail, tip = P(tail), P(tip)
        length = float(np.linalg.norm(tip - tail))
        if length < 1e-4:
            return _engine.VGroup()
        direction = (tip - tail) / length
        normal = np.array([-direction[1], direction[0], 0.0])
        head = min(0.14, 0.55 * length)
        half = head * 0.42
        base = tip - direction * head
        shape = _engine.VGroup(_d.polygon([tip, base + normal * half, base - normal * half], color, 1.0, color, 0.4))
        if length > head + 1e-4:
            shape.add(_d.segments([(tail, base + direction * 0.02)], color, width))
        return shape

    def load_arrows(self, values, points, scale: float = ARROW_KN, *, unit: str = "kN", per: str | None = None, direction=(0, -1), anchor: str = "tail",
                    colors=None, labels: bool = True, digits: int | None = None, tags=None, width: float = 2.6, size: float = _d.S_SMALL):
        """➡️ One arrow per value; ``points`` are screen positions (tails with ``anchor="tail"``, tips with ``anchor="tip"``), length is ``|value| × scale`` screen units.

        Pass the same ``scale`` to every call that must compare (``ARROW_KN`` for forces, ``ARROW_Q`` for kN/m²). A negative value flips the arrow.
        ``direction`` is the screen direction of positive values, ``per`` an area or length unit (``"m²"`` gives ``kN/m²``), ``colors`` one colour or a list, ``tags`` the part names (default ``"0"``, ``"1"`` …).
        The label with the value and unit sits at the free end. Returns a ``Drawing`` with ``parts[tag]``, ``.arrows`` (list), ``.tips``, ``.tails`` and ``.values``; ``at(tag)`` is the tip."""
        unit_text = unit + ("/" + per if per else "")
        digits = (0 if per is None else 2) if digits is None else digits
        way = np.array(direction[:2], float)
        way = way / max(np.linalg.norm(way), 1e-9)
        drawing = _d.Drawing(None)
        drawing.arrows, drawing.tips, drawing.tails, drawing.values = [], [], [], list(values)
        for index, (value, spot) in enumerate(zip(values, points)):
            heading = np.array([*way * (1.0 if value >= 0 else -1.0), 0.0])
            length = abs(value) * scale
            spot = P(spot)
            tail = spot if anchor == "tail" else spot - heading * length
            tip = tail + heading * length
            color = colors[index] if isinstance(colors, (list, tuple)) else (colors or _d.DATA)
            group = self._arrow(tail, tip, color, width)
            if labels:
                free, outward = (tip, heading) if anchor == "tail" else (tail, -heading)
                side = "u" if outward[1] < -0.5 else "d" if outward[1] > 0.5 else "l" if outward[0] > 0 else "r"
                shift = outward * 0.07
                group.add(self._note(self.quantity(value, unit_text, digits), free + shift, size=size, color=color, anchor=side))
            drawing.arrows.append(group)
            drawing.tips.append(tip)
            drawing.tails.append(tail)
            drawing.part(str(index) if tags is None else str(tags[index]), group, tip)
            drawing.add(group)
        return drawing

    def _support_glyph(self, view, position, fixed: bool, color: str, scale: float, strong: bool):
        """⏚ Support symbol: fixed is a hatched ground block, pinned a triangle on a ground line."""
        x, y, z = position
        half = min(max(0.4 * scale, 0.045), 0.13)
        width = _d.W_EDGE if strong else _d.W_THIN
        at = P(view(x, y, z))
        group = _engine.VGroup()
        if view.kind == "plan":
            if fixed:
                ring = [at + (-half, -half, 0), at + (half, -half, 0), at + (half, half, 0), at + (-half, half, 0)]
                group.add(_d.polygon(ring, _d.mix(color, _d.BACKGROUND, 0.7), 0.9, color, width), _d.hatch(ring, color, half * 0.55, 45, _d.W_HAIR, 0.9))
            else:
                ring = [at + (0, half * 1.15, 0), at + (-half * 1.05, -half * 0.8, 0), at + (half * 1.05, -half * 0.8, 0)]
                group.add(_d.polygon(ring, _d.mix(color, _d.BACKGROUND, 0.7), 0.9, color, width))
            return group
        if fixed:
            reach = 0.55
            ring = [view(x + sx * reach, y + sy * reach, z) for sx, sy in ((-1, -1), (1, -1), (1, 1), (-1, 1))]
            group.add(_d.polygon(ring, _d.mix(color, _d.BACKGROUND, 0.7), 0.9, color, width), _d.hatch(ring, color, max(half * 0.5, 0.03), 45, _d.W_HAIR, 0.9))
        else:
            h = half * 1.5
            ring = [at, at + (-half, -h, 0), at + (half, -h, 0)]
            group.add(_d.polygon(ring, _d.mix(color, _d.BACKGROUND, 0.7), 0.9, color, width),
                      _d.segments([(at + (-half * 1.5, -h, 0), at + (half * 1.5, -h, 0))], color, width))
        return group

    def analysis_view(self, analysis: dict, point=(0, 0, 0), scale: float = 0.26, *, projection: str = "plan", loads=None, stale: bool = False, highlight=(),
                      nodes: bool = True, supports: bool = True, ids=False, load_scale: float | None = None, revision: bool | None = None, window=None, model=None):
        """🧮 The structural analysis model of ``translate_structure()``: shell mesh on the first-floor slab (``IfcStructuralSurfaceMember``), pilotis as curve members
        (``IfcStructuralCurveMember``) down to their support nodes, support symbols (fixed: hatched ground block, pinned: triangle) and node dots.

        ``point`` is the screen position of the box centre and ``scale`` screen units per metre, exactly like ``villa_axon``; ``projection`` is ``"plan"`` or ``"axon"``.
        ``highlight`` takes piloti tags, member, shell or node ids, or ``"SL-1"`` for the whole slab. ``ids`` is ``True``, one of ``"shells" "members" "nodes"`` or a tuple of them.
        ``loads`` is ``None``, a mapping piloti tag (or support node id) → reaction in kN (axon: arrows up into the foot, plan: labels), or one number in kN/m² (axon: arrows onto every shell, plan: ⊗ marks);
        ``load_scale`` is the arrow length per unit (defaults ``ARROW_KN`` / ``ARROW_Q``). ``stale=True`` draws everything in the warning colour with a ``revision n`` badge
        (``revision=True`` shows the badge also when current). ``window=(x0, x1, y0, y1)`` crops in metres.
        Returns a ``Drawing``: ``parts[shell id | member id | piloti tag | node id | "tag.support"]``, ``.shells``, ``.members``, ``.nodes``, ``.supports``, ``.loads``, ``.labels``, ``.tag``, ``.at(tag)`` = member middle."""
        view = self._projection(projection, point, scale, model)
        plan = view.kind == "plan"
        base = _d.WARN if stale else _d.STRUCT
        chosen = _names(highlight)
        wanted = set(ID_KINDS) if ids is True else _names(ids) if ids else set()
        position = {n["id"]: n["position"] for n in analysis["nodes"]}
        drawing = _d.Drawing(view)
        dot = min(max(0.25 * scale, 0.022), 0.045)

        def screen(node_id):
            return P(view(*position[node_id]))

        def visible(node_ids):
            return _c.inside(window, [position[i] for i in node_ids])

        curves = [m for m in analysis["members"] if m["kind"] == "curve" and visible(m["nodes"])]
        lit_curves = {m["id"] for m in curves if {m["id"], m.get("source_tag"), m.get("source_guid")} & chosen}
        lit_nodes = {i for m in curves if m["id"] in lit_curves for i in m["nodes"]}
        shell_group, member_group, node_group, support_group, label_group = (_engine.VGroup() for _ in range(5))

        shells = [s for s in analysis["shells"] if visible(s["nodes"])]
        counted = Counter()
        for shell in shells:
            ring = [screen(i) for i in shell["nodes"]]
            lit = bool({shell["id"], shell.get("source_tag"), shell.get("source_guid")} & chosen)
            colour = _d.DATA if lit else base
            shape = _engine.VGroup(_d.polygon(ring, colour, 0.22 if lit else 0.14, colour, _d.W_EDGE if lit else _d.W_THIN, 1.0 if lit else 0.85))
            shell_group.add(shape)
            drawing.part(shell["id"], shape, np.mean(ring, axis=0))
            for a, b in zip(shell["nodes"], shell["nodes"][1:] + shell["nodes"][:1]):
                counted[frozenset((a, b))] += 1
        edge = [tuple(pair) for pair, times in counted.items() if times == 1]
        outline = _d.segments([(screen(a), screen(b)) for a, b in edge], base, _d.W_EDGE)
        if shells:
            drawing.parts[shells[0].get("source_tag", "SL-1")] = shell_group

        for member in curves:
            foot, top = member["nodes"]
            lit = member["id"] in lit_curves
            colour = _d.DATA if lit else base
            tag = member.get("source_tag", member["id"])
            if plan:
                radius = max(0.5 * member.get("diameter", 0.3) * scale, 0.032)
                shape = _engine.VGroup(_engine.Circle(radius=radius * (1.5 if lit else 1.0), stroke_color=colour, stroke_width=_d.W_THIN, fill_color=colour, fill_opacity=0.95).move_to(screen(top)))
                if lit:
                    shape.add(_engine.Circle(radius=0.2, stroke_color=colour, stroke_width=_d.W_THIN, stroke_opacity=0.8).move_to(screen(top)))
            else:
                shape = _engine.VGroup(_d.segments([(screen(foot), screen(top))], colour, 3.6 if lit else 2.4))
                if lit:
                    shape.add(_d.segments([(screen(foot), screen(top))], colour, 10.0, 0.22))
            member_group.add(shape)
            drawing.part(member["id"], shape, (screen(foot) + screen(top)) / 2)
            drawing.parts[tag] = shape
            drawing.anchors[tag] = (screen(foot) + screen(top)) / 2

        if supports:
            for support in analysis["supports"]:
                if not visible([support["node"]]):
                    continue
                tag = support.get("source_tag", support["node"])
                lit = tag in chosen or support["node"] in chosen or support.get("source_guid") in chosen
                colour = _d.DATA if lit else base
                fixity = str(support.get("fixity") or support.get("condition", "")).lower()
                glyph = self._support_glyph(view, position[support["node"]], "fix" in fixity or "einge" in fixity, colour, scale, lit)
                support_group.add(glyph)
                drawing.part(tag + ".support", glyph, screen(support["node"]))
                drawing.parts[support["node"] + ".support"] = glyph

        if nodes:
            for node in analysis["nodes"]:
                if not _c.inside(window, [node["position"]]):
                    continue
                lit = node["id"] in chosen or node["id"] in lit_nodes
                colour = _d.DATA if lit else _d.mix(base, _d.INK, 0.35)
                shape = _engine.Dot(screen(node["id"]), radius=dot * (1.5 if lit else 1.0), color=colour)
                node_group.add(shape)
                drawing.part(node["id"], shape, screen(node["id"]))

        if "shells" in wanted:
            for shell in shells:
                ring = [screen(i) for i in shell["nodes"]]
                span = max(p[0] for p in ring) - min(p[0] for p in ring)
                name = shell["id"]
                note = self._note(name, np.mean(ring, axis=0), color=_d.MUTED, halo=False)
                if note.width > 0.92 * span:
                    note = self._note(name.split("-", 1)[-1], np.mean(ring, axis=0), color=_d.MUTED, halo=False)
                label_group.add(note)
        if "members" in wanted:
            notes = []
            for member in curves:
                foot, top = member["nodes"]
                spot = screen(top) + np.array([0.12, 0.1, 0.0]) if plan else (screen(foot) + screen(top)) / 2 + np.array([0.07, 0.0, 0.0])
                notes.append(self._note(member.get("source_tag", member["id"]), spot, color=_d.INK, anchor="ld" if plan else "l"))
            label_group.add(*self._declutter(notes))
        if "nodes" in wanted:
            notes = [self._note(node["id"], screen(node["id"]) + np.array([0.05, 0.05, 0.0]), color=_d.INK, anchor="ld")
                     for node in analysis["nodes"] if _c.inside(window, [node["position"]])]
            label_group.add(*self._declutter(notes))

        load_group = _engine.VGroup()
        if loads is not None:
            colour = _d.WARN if stale else _d.GOOD
            if isinstance(loads, (int, float)):
                q = float(loads)
                spots = [np.mean([position[i] for i in s["nodes"]], axis=0) for s in shells]
                if plan:
                    for spot in spots:
                        centre = P(view(*spot))
                        load_group.add(_engine.Circle(radius=0.055, stroke_color=colour, stroke_width=_d.W_THIN).move_to(centre),
                                       _d.segments([(centre + (-0.039, -0.039, 0), centre + (0.039, 0.039, 0)), (centre + (-0.039, 0.039, 0), centre + (0.039, -0.039, 0))], colour, _d.W_THIN))
                else:
                    arrows = self.load_arrows([q] * len(spots), [view(*spot) for spot in spots], ARROW_Q if load_scale is None else load_scale, per="m²", anchor="tip",
                                              colors=colour, labels=False, width=2.0)
                    load_group.add(*arrows.arrows)
                    drawing.parts["q"] = load_group
            else:
                feet = {s["node"]: s for s in analysis["supports"]}
                by_tag = {s.get("source_tag"): s["node"] for s in analysis["supports"]}
                keys = [k for k in loads if (loads[k] is not None) and ((k in feet) or (k in by_tag)) and visible([by_tag.get(k, k)])]
                values = [float(loads[k]) for k in keys]
                spots = [screen(by_tag.get(k, k)) for k in keys]
                if plan:
                    for key, value, spot in zip(keys, values, spots):
                        load_group.add(self._note(self.quantity(value, "kN", 0), spot + np.array([0.09, -0.09, 0.0]), color=colour, anchor="lu"))
                else:
                    arrows = self.load_arrows(values, spots, ARROW_KN if load_scale is None else load_scale, direction=(0, 1), anchor="tip", colors=colour, tags=keys, width=2.4)
                    load_group.add(*arrows.arrows)
                    for key, tip in zip(keys, arrows.tips):
                        drawing.parts[key + ".load"] = arrows.parts[key]

        order = (shell_group, outline, support_group, member_group, node_group, load_group, label_group) if plan else (support_group, member_group, shell_group, outline, node_group, load_group, label_group)
        for layer in order:
            drawing.add(layer)
        drawing.shells, drawing.members, drawing.nodes, drawing.supports, drawing.loads, drawing.labels = shell_group, member_group, node_group, support_group, load_group, label_group
        drawing.outline = outline
        drawing.tag = _engine.VGroup()
        if (stale if revision is None else revision) and len(drawing.submobjects):
            corner = np.array([drawing.get_left()[0], drawing.get_top()[1] + 0.1, 0.0])
            text = self.tr("revision", "Revision") + " " + str(analysis.get("source_revision", ""))
            drawing.tag = self.badge(text, corner, color=_d.WARN if stale else _d.GOOD, anchor="ld")
            drawing.add(drawing.tag)
        return drawing

    def _slab_plane(self, level: str, model) -> float:
        """📐 Mid-surface elevation of the slab of a level."""
        geometry = _c.geometry(model)
        roof = level == "roof"
        slab = next(s for s in geometry["slabs"] if (s["kind"] == "ROOF") == roof)
        return slab["top"] - slab["thickness"] / 2

    def tributary_view(self, tributary: dict, point=(0, 0, 0), scale: float = 0.26, *, projection: str = "plan", highlight=(), level: str | None = None,
                       labels=None, reactions=False, arrow_scale: float = ARROW_KN, window=None, model=None):
        """🧩 The tributary cells of ``tributary()``: one hatched cell per piloti in the colour of its class (interior, edge, corner, cantilever), the piloti on its cell.

        ``point`` is the box centre, ``scale`` screen units per metre, ``projection`` ``"plan"`` or ``"axon"`` (same projection as ``villa_axon``; the cells lie in the slab mid-surface,
        pilotis drop to the ground). ``level`` is ``"first"`` or ``"roof"`` (default: ``tributary["level"]``) and picks the slab plane.
        ``highlight`` takes tags, GlobalIds or class names (``"corner"``); other cells fade. ``labels`` is a tuple of ``"tag" "area" "reaction"`` (``True`` = area, default area in plan, none in axon).
        ``reactions`` is ``True`` or a set of tags: arrows up into the piloti foot in the axon (length ``reaction × arrow_scale``), labels in the plan.
        Returns a ``Drawing``: ``parts[tag]`` (the cell), ``at(tag)`` = the piloti position (scale a cell about it), ``.cells``, ``.pilotis``, ``.labels``, ``.arrows`` and ``.kinds[tag]``."""
        view = self._projection(projection, point, scale, model)
        plan = view.kind == "plan"
        z = self._slab_plane(level or tributary.get("level", "first"), model)
        chosen = _names(highlight)
        wanted = {"area"} if labels is True else set() if labels is False else _names(labels) if labels is not None else ({"area"} if plan else set())
        everyone = reactions is True
        shown = _names(reactions) if reactions not in (True, False) else set()
        drawing = _d.Drawing(view)
        cells, feet, lines, texts, pins = (_engine.VGroup() for _ in range(5))
        drawing.kinds = {}
        keys, values, spots = [], [], []
        for entry in tributary["pilotis"]:
            ring = entry.get("cell") or (entry.get("pieces") or [[]])[0]
            if len(ring) < 3 or not _c.inside(window, ring):
                continue
            kind = piloti_kind(tributary, entry)
            drawing.kinds[entry["tag"]] = kind
            lit = bool({entry["tag"], entry.get("guid"), kind} & chosen)
            fade = bool(chosen) and not lit
            colour = _d.DATA if lit else KIND_COLORS[kind]
            screen_ring = [view(x, y, z) for x, y in ring]
            shape = _engine.VGroup(_d.polygon(screen_ring, colour, 0.36 if lit else 0.07 if fade else 0.16, colour, _d.W_EDGE if lit else _d.W_THIN, 1.0 if lit else 0.5 if fade else 0.9),
                                   _d.hatch(screen_ring, colour, 0.075, 45, _d.W_THIN if lit else _d.W_HAIR, 1.0 if lit else 0.35 if fade else 0.8))
            cells.add(shape)
            top, foot = P(view(entry["x"], entry["y"], z)), P(view(entry["x"], entry["y"], 0.0))
            drawing.part(entry["tag"], shape, top)
            if not plan:
                lines.add(_d.segments([(foot, top)], _d.mix(colour, _d.INK, 0.25) if lit else _d.mix(_d.STRUCT, _d.BACKGROUND, 0.35 if fade else 0.0), 2.2 if lit else 1.6, 0.6 if fade else 0.95))
            dot = _engine.Dot(top, radius=0.055 if lit else 0.038, color=_d.DATA if lit else _d.INK)
            feet.add(dot)
            lines_of = []
            if "tag" in wanted:
                lines_of.append((entry["tag"], _d.DATA if lit else _d.INK))
            if "area" in wanted:
                lines_of.append((self.quantity(entry["area"], "m²", 1), _d.DATA if lit else _d.INK))
            if "reaction" in wanted:
                lines_of.append((self.quantity(entry["reaction"], "kN", 0), _d.DATA if lit else _d.INK))
            if lines_of:
                middle = P(view(*_centroid(ring), z))
                block = _engine.VGroup(*(self._note(text, (0, 0, 0), color=color) for text, color in lines_of)).arrange(_engine.DOWN, buff=0.03)
                _d.place(block, middle)
                if abs(top[0] - middle[0]) < block.width / 2 + 0.05 and abs(top[1] - middle[1]) < block.height / 2 + 0.05:
                    below = top[1] > middle[1] + 0.02
                    block.shift(np.array([0.0, (top[1] - 0.07 - block.get_top()[1]) if below else (top[1] + 0.07 - block.get_bottom()[1]), 0.0]))
                texts.add(block)
            if everyone or entry["tag"] in shown:
                keys.append(entry["tag"])
                values.append(entry["reaction"])
                spots.append(foot)
        arrows = _engine.VGroup()
        if keys and not plan:
            tips = self.load_arrows(values, spots, arrow_scale, direction=(0, 1), anchor="tip", colors=[_d.DATA if k in chosen else _d.GOOD for k in keys], tags=keys, width=2.4)
            arrows.add(*tips.arrows)
            for key in keys:
                drawing.parts[key + ".load"] = tips.parts[key]
        elif keys:
            for key, value, spot in zip(keys, values, spots):
                texts.add(self._note(self.quantity(value, "kN", 0), spot + np.array([0.0, -0.12, 0.0]), color=_d.GOOD, anchor="u"))
        for layer in ((cells, lines, feet, arrows, texts) if not plan else (cells, feet, arrows, texts)):
            drawing.add(layer)
        drawing.cells, drawing.pilotis, drawing.labels, drawing.arrows = cells, feet, texts, arrows
        return drawing

    def tributary_legend(self, point=(0, 0, 0), *, kinds=KINDS, direction: str = "row", size: float = _d.S_SMALL, gap: float = 0.3):
        """🔑 Colour key of the tributary classes: a hatched swatch and the localized class name per class; ``direction`` is ``"row"`` or ``"column"``."""
        group = _engine.VGroup()
        for kind in kinds:
            colour = KIND_COLORS[kind]
            ring = [P((0, 0)), P((0.3, 0)), P((0.3, 0.2)), P((0, 0.2))]
            swatch = _engine.VGroup(_d.polygon(ring, colour, 0.2, colour, _d.W_THIN), _d.hatch(ring, colour, 0.06, 45, _d.W_HAIR, 0.9))
            name = self._note(self.tr(*KIND_NAMES[kind]), (0.38, 0.1, 0), size=size, color=_d.INK, anchor="l")
            item = _engine.VGroup(swatch, name)
            group.add(item)
        if direction == "column":
            group.arrange(_engine.DOWN, buff=0.12, aligned_edge=_engine.LEFT)
        else:
            group.arrange(_engine.RIGHT, buff=gap)
        return _d.place(group, point)

    def mesh_view(self, mesh, point=(0, 0, 0), scale: float = 0.75, *, view: str = "axon", segments: int | None = None, highlight=(), vertices: bool = True,
                  counts: bool = True, true_shape: bool = True, azimuth: float | None = None, elevation: float | None = None, model=None):
        """🔺 The triangle mesh of ``mesh(tag)`` as drawn solid: visible triangles with their edges, vertex dots, the dashed true shape and the live counts.

        ``mesh`` is a ``mesh()`` result or an element tag; ``segments`` re-tessellates (so a ``ValueTracker`` can drive ``mesh_view(m, …, segments=int(t.get_value()))``).
        ``point`` is the screen position of the bounding-box centre, ``scale`` screen units per metre. ``view`` is ``"axon"``, ``"side"`` (south elevation, also ``"elevation"``) or ``"top"``.
        ``highlight`` is a set of triangle indices. Faces are orientated outward (convex bodies such as the piloti) and sorted far to near; ``true_shape`` adds the dashed true circle (top),
        the true silhouette (side) or the true rims (axon). Returns a ``Drawing`` with ``.triangles``, ``.vertices``, ``.true``, ``.counts``, ``parts["t<i>"]`` and ``.stats = (vertices, triangles)``."""
        kind = MESH_VIEWS[view]
        data = _mesh_of(mesh, segments, model)
        points, faces = np.asarray(data["vertices"], float), np.asarray(data["faces"], int)
        low, high = points.min(axis=0), points.max(axis=0)
        centre = (low + high) / 2
        radius = float(np.hypot(points[:, 0] - centre[0], points[:, 1] - centre[1]).max())
        if kind == "axon":
            projector = _d.View("axon", point, scale, centre, azimuth=_d.DEFAULT_AZIMUTH if azimuth is None else azimuth,
                                elevation=_d.DEFAULT_ELEVATION if elevation is None else elevation)
        elif kind == "side":
            projector = _d.View("elevation", point, scale, centre, face="south")
        else:
            projector = _d.View("plan", point, scale, centre)
        a, b, c = points[faces[:, 0]], points[faces[:, 1]], points[faces[:, 2]]
        normals = np.cross(b - a, c - a)
        normals /= np.maximum(np.linalg.norm(normals, axis=1, keepdims=True), 1e-12)
        middle = (a + b + c) / 3
        normals[((middle - points.mean(axis=0)) * normals).sum(axis=1) < 0] *= -1
        order = sorted((i for i in range(len(faces)) if projector.facing(normals[i]) > 1e-6), key=lambda i: projector.depth(*middle[i]))
        lit = {int(i) for i in highlight}
        drawing = _d.Drawing(projector)
        triangles, dots = _engine.VGroup(), _engine.VGroup()
        used = set()
        for i in order:
            tone = _axon.brightness(normals[i])
            colour = _d.DATA if i in lit else _d.shade(_d.STRUCT, 0.3 + 0.55 * tone)
            shape = _d.polygon([projector(*points[j]) for j in faces[i]], colour, 1.0, _d.DATA if i in lit else _d.mix(_d.INK, _d.BACKGROUND, 0.12), 1.2 if i in lit else _d.W_THIN)
            triangles.add(shape)
            drawing.parts[f"t{i}"] = _engine.VGroup(shape)
            used.update(int(j) for j in faces[i])
        if vertices:
            for j in sorted(used):
                dots.add(_engine.Dot(projector(*points[j]), radius=0.032, color=_d.DATA))
        true = _engine.VGroup()
        if true_shape:
            if kind == "top":
                ring = [projector(centre[0] + radius * math.cos(t), centre[1] + radius * math.sin(t), centre[2]) for t in np.linspace(0, 2 * math.pi, 73)]
                true.add(_d.polyline(ring, _d.ARCH, _d.W_THIN, 0.95, closed=True, dash=0.09))
            elif kind == "side":
                for sign in (-1, 1):
                    true.add(_d.polyline([projector(centre[0] + sign * radius, centre[1], low[2]), projector(centre[0] + sign * radius, centre[1], high[2])], _d.ARCH, _d.W_THIN, 0.95, dash=0.09))
            else:
                for z in (low[2], high[2]):
                    ring = [projector(centre[0] + radius * math.cos(t), centre[1] + radius * math.sin(t), z) for t in np.linspace(0, 2 * math.pi, 73)]
                    true.add(_d.polyline(ring, _d.ARCH, _d.W_THIN, 0.95, closed=True, dash=0.09))
        drawing.add(true, triangles, dots)
        drawing.triangles, drawing.vertices, drawing.true = triangles, dots, true
        drawing.stats = (int(data["vertex_count"]), int(data["triangle_count"]))
        drawing.counts = _engine.VGroup()
        if counts:
            every = np.array([projector(*p) for p in points])
            floor = float(every[:, 1].min()) - 0.12
            n, m = drawing.stats
            text = self.tr(f"{self.number(n, 0)} vertices · {self.number(m, 0)} triangles", f"{self.number(n, 0)} Punkte · {self.number(m, 0)} Dreiecke")
            drawing.counts = _engine.VGroup(self._note(text, (float(every[:, 0].mean()), floor, 0.0), color=_d.INK, anchor="u"))
            drawing.add(drawing.counts)
        return drawing
