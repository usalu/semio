"""🧱 Shared geometry readers, element catalogue, selection styling and polygon helpers of the Villa Savoye drawings."""

from __future__ import annotations

import importlib
import math

import numpy as np

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, DATA, BACKGROUND = _d.INK, _d.MUTED, _d.DATA, _d.BACKGROUND
KIND_CLASSES = {"pilotis": "IfcColumn", "slabs": "IfcSlab", "walls": "IfcWall", "windows": "IfcWindow", "curtain": "IfcCurtainWall",
                "ramp": "IfcRamp", "screens": "IfcWall"}
DEFAULT_DISCIPLINES = {"pilotis": ("structure",), "slabs": ("architecture", "structure"), "walls": ("architecture",), "windows": ("architecture",),
                       "curtain": ("architecture",), "ramp": ("architecture",), "screens": ("architecture",), "services": ("services",)}
_cache: dict = {}


def geometry(model=None) -> dict:
    """📐 ``building_geometry`` memoized per model object."""
    key = id(model)
    hit = _cache.get(key)
    if hit is None or hit[0] is not model:
        hit = _cache[key] = (model, _model.building_geometry(model) if model is not None else _model.building_geometry())
        if len(_cache) > 16:
            _cache.pop(next(iter(_cache)))
    return hit[1]


def membership(model=None) -> dict:
    """🧩 GlobalId → set of partial models that contain it; empty when the model does not list element GlobalIds."""
    key = ("members", id(model))
    hit = _cache.get(key)
    if hit is None or hit[0] is not model:
        table: dict = {}
        for name in _d.DISCIPLINES:
            try:
                info = _model.partial_model(name, model) if model is not None else _model.partial_model(name)
            except Exception:
                continue
            for guid in info.get("element_guids", ()):
                table.setdefault(guid, set()).add(name)
        hit = _cache[key] = (model, table)
    return hit[1]


def disciplines_of(guid: str, kind: str, model=None) -> tuple:
    """🏷️ Disciplines whose model holds the element; falls back to class defaults."""
    found = membership(model).get(guid)
    return tuple(name for name in _d.DISCIPLINES if name in found) if found else DEFAULT_DISCIPLINES[kind]


def catalogue(model=None) -> list:
    """📚 Every drawable element: ``{tag, guid, kind, cls, data, disciplines}`` in a stable order."""
    g = geometry(model)
    items = []

    def add(kind, data, cls=None):
        items.append({"tag": data["tag"], "guid": data["guid"], "kind": kind, "cls": cls or KIND_CLASSES[kind], "data": data,
                      "disciplines": disciplines_of(data["guid"], kind, model)})

    for item in g["pilotis"]:
        add("pilotis", item)
    for item in g["slabs"]:
        add("slabs", item)
    for item in g["walls"]:
        add("walls", item)
        for opening in item.get("openings", ()):
            add("windows", {**opening, "tag": opening["window_tag"], "guid": opening["window_guid"], "wall": item["tag"]})
    if g.get("curtain_wall"):
        add("curtain", g["curtain_wall"])
    if g.get("ramp"):
        add("ramp", g["ramp"])
    for item in g.get("screens", ()):
        add("screens", item)
    for item in g.get("services", ()):
        add("services", item, item["class"])
    return items


class Selection:
    """🎯 Highlight / ghost state of every tag: tags, IFC classes and kind names are all accepted."""

    def __init__(self, highlight=(), ghost=(), dim: float = 0.62):
        self.highlight = {highlight} if isinstance(highlight, str) else set(highlight)
        self.ghost = {ghost} if isinstance(ghost, str) else set(ghost)
        self.dim = dim

    def state(self, item) -> str:
        names = {item["tag"], item["cls"], item["kind"], item["guid"]}
        if names & self.ghost:
            return "ghost"
        if self.highlight:
            return "highlight" if names & self.highlight else "dim"
        return "normal"

    def colors(self, item, base: str, brightness: float = 1.0):
        """🖌️ ``(fill, fill_opacity, stroke, stroke_opacity, stroke_width)`` of a face with the given base colour and brightness."""
        state = self.state(item)
        if state == "highlight":
            return _d.shade(DATA, 0.4 + 0.6 * brightness), 1.0, DATA, 1.0, 2.2
        if state == "ghost":
            return base, 0.12, base, 0.5, 1.0
        face = _d.shade(base, 0.32 + 0.68 * brightness)
        if state == "dim":
            return _d.mix(face, BACKGROUND, self.dim), 1.0, _d.mix(face, BACKGROUND, self.dim + 0.1), 1.0, 0.8
        return face, 1.0, _d.shade(base, 0.2 + 0.4 * brightness), 1.0, 0.8


def box_outline(g: dict) -> list:
    """⬜ The box rectangle in plan."""
    box = g["box"]
    return [[0.0, 0.0], [box["length"], 0.0], [box["length"], box["width"]], [0.0, box["width"]]]


def wall_frame(wall: dict, g: dict):
    """🧭 Origin, unit direction, inward unit normal and length of a wall; ``start → end`` is the outer face, thickness goes inward."""
    a, b = np.array(wall["start"], float), np.array(wall["end"], float)
    length = float(np.linalg.norm(b - a))
    along = (b - a) / length
    normal = np.array([-along[1], along[0]])
    centre = np.array([g["box"]["length"], g["box"]["width"]]) / 2
    if (centre - a) @ normal < 0:
        normal = -normal
    return a, along, normal, length


def scan(rings, t: float, axis: int = 1) -> list:
    """📏 Intervals of a polygon with holes along the line ``coordinate[axis] == t`` (even-odd); returns sorted ``(start, end)``."""
    other = 1 - axis
    hits = []
    for ring in rings:
        pts = np.asarray(ring, float)
        for index in range(len(pts)):
            a, b = pts[index], pts[(index + 1) % len(pts)]
            if (a[axis] < t) != (b[axis] < t):
                hits.append(float(a[other] + (b[other] - a[other]) * (t - a[axis]) / (b[axis] - a[axis])))
    hits.sort()
    return list(zip(hits[0::2], hits[1::2]))


def bounds(points) -> tuple:
    """🔲 ``(xmin, ymin, xmax, ymax)`` of 2D points."""
    pts = np.asarray([p[:2] for p in points], float)
    return (*pts.min(axis=0), *pts.max(axis=0))


def inside(window, points, margin: float = 0.0) -> bool:
    """🪟 Whether the bounds of points touch the crop window ``(x0, x1, y0, y1)``; ``None`` keeps everything."""
    if window is None:
        return True
    x0, y0, x1, y1 = bounds(points)
    return x1 >= window[0] - margin and x0 <= window[1] + margin and y1 >= window[2] - margin and y0 <= window[3] + margin


def roof_layers(model=None) -> list:
    """🥞 Build-up of the roof slab from bottom to top: ``[{key, en, de, thickness, color_role}]``; thickness in metres, read from the model."""
    g = geometry(model)
    roof = next(s for s in g["slabs"] if s["kind"] == "ROOF")
    try:
        material = _model.element(roof["tag"], model)["material"]
        raw = material["layers"] if isinstance(material, dict) else material
    except Exception:
        raw = None
    if not raw:
        raw = [{"key": "concrete", "name": {"en": "Reinforced concrete", "de": "Stahlbeton"}, "thickness": roof["thickness"]}]
    layers = []
    for layer in raw:
        name = layer.get("name") or {"en": layer.get("key", ""), "de": layer.get("key", "")}
        name = name if isinstance(name, dict) else {"en": name, "de": name}
        thickness = layer["thickness"]
        thickness = thickness["value"] if isinstance(thickness, dict) else thickness
        key = layer.get("key") or layer.get("id") or str(name["en"]).lower().split()[0]
        layers.append({"key": key, "en": name["en"], "de": name["de"], "thickness": float(thickness)})
    return layers


LAYER_COLORS = {"concrete": _d.CONCRETE, "waterproofing": _d.mix(INK, BACKGROUND, 0.82), "screed": _d.mix(_d.STRUCT, BACKGROUND, 0.25),
                "soil": _d.mix(_d.GOOD, BACKGROUND, 0.58), "gravel": _d.mix(INK, BACKGROUND, 0.4), "insulation": _d.mix(_d.DATA, BACKGROUND, 0.45)}


def layer_color(key: str) -> str:
    """🎨 Colour of a build-up layer by its key (first word match)."""
    key = str(key).lower()
    return next((color for name, color in LAYER_COLORS.items() if name in key), _d.CONCRETE)


def flight_direction(flight: dict):
    """➡️ Unit plan direction of a ramp flight."""
    d = np.array(flight["end"], float) - np.array(flight["start"], float)
    return d / np.linalg.norm(d)
