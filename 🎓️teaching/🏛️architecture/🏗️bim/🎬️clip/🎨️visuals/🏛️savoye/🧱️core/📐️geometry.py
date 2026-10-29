"""📐 Physical geometry readers shared by the Villa Savoye drawing providers."""

from __future__ import annotations

import importlib

import numpy as np

_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
DISCIPLINES = ("architecture", "structure", "services")
_cache: dict = {}


def part_faces(part):
    """🔺 Actual side planes and upper cap of corresponding lower and upper solid rings."""
    if "section" in part or "mesh" in part or part.get("holes"):
        vertices = np.asarray(_model.part_vertices(part),float)
        result = []
        for indices in _model.part_faces(part):
            face = vertices[indices]
            normal = sum((np.cross(face[i]-face[0],face[i+1]-face[0]) for i in range(1,len(face)-1)),np.zeros(3))
            length = np.linalg.norm(normal)
            if length > 1e-12:
                result.append((face.tolist(),(normal/length).tolist()))
        return result
    bottom = np.array([[*p,part["z0"]] for p in part["outline"]],float)
    top = np.array([[*p,part["z1"]] for p in part.get("top_outline",part["outline"])],float)
    signed = np.sum(bottom[:,0]*np.roll(bottom[:,1],-1)-bottom[:,1]*np.roll(bottom[:,0],-1))
    if signed < 0:
        bottom,top = bottom[::-1],top[::-1]
    faces = []
    for i in range(len(bottom)):
        j = (i+1)%len(bottom)
        points = np.array([bottom[i],bottom[j],top[j],top[i]])
        normal = np.cross(points[1]-points[0],points[3]-points[0])
        length = np.linalg.norm(normal)
        if length > 1e-12:
            faces.append((points.tolist(),(normal/length).tolist()))
    return faces+[(top.tolist(),[0.,0.,1.])]


def surface_role(data: dict, part: dict | None = None) -> str:
    """🎨 Resolve authored material semantics consistently across drawing providers."""
    if part and part.get("material"):
        return {"MAT-GLASS":"glass","MAT-METAL":"metal","MAT-WOOD":"wood","MAT-TILE":"tile"}.get(part["material"],"concrete")
    return {"rooflight": "glass", "parquet": "wood", "tile": "tile", "bath": "tile", "chaise": "tile", "counter": "tile",
            "paving": "paving", "grass_joint": "garden", "tap": "metal", "helical_handrail": "metal", "pivoting-hoist-jib": "metal"}.get(data["kind"], "concrete")


def geometry(model=None) -> dict:
    """📐 ``building_geometry`` memoized per model object."""
    key = id(model)
    hit = _cache.get(key)
    if hit is None or hit[0] is not model:
        hit = _cache[key] = (model, _model.building_geometry(model) if model is not None else _model.building_geometry())
        if len(_cache) > 16:
            _cache.pop(next(iter(_cache)))
    return hit[1]


def role(name: str, model=None) -> str:
    """🏷️ Resolve an authored semantic role without depending on an incidental object identifier."""
    return _model.role(name, model)


def membership(model=None) -> dict:
    """🧩 GlobalId → set of partial models that contain it; empty when the model does not list element GlobalIds."""
    key = ("members", id(model))
    hit = _cache.get(key)
    if hit is None or hit[0] is not model:
        table: dict = {}
        for name in DISCIPLINES:
            try:
                info = _model.partial_model(name, model) if model is not None else _model.partial_model(name)
            except Exception:
                continue
            for guid in info.get("element_guids", ()):
                table.setdefault(guid, set()).add(name)
        hit = _cache[key] = (model, table)
    return hit[1]


def wall_frame(wall: dict, g: dict):
    """🧭 Outer edge, direction, left normal and length of an authored wall centre-line."""
    a, b = np.array(wall["start"], float), np.array(wall["end"], float)
    length = float(np.linalg.norm(b - a))
    along = (b - a) / length
    normal = np.array([-along[1], along[0]])
    return tuple(map(float, a - normal * wall["thickness"] / 2)), tuple(map(float, along)), tuple(map(float, normal)), length
