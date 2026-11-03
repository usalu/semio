"""📐 Physical geometry readers shared by the Villa Savoye drawing providers."""

from __future__ import annotations

import importlib
import copy
import hashlib
import json
from pathlib import Path

import numpy as np

_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
face_appearance = _model.surface_finishes().face_appearance
appearance_colour = _model.surface_finishes().appearance_colour
DISCIPLINES = ("architecture", "structure", "services")
_sources: dict = {}
_snapshots: dict = {}
_members: dict = {}
_default_source = None


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
        return {"MAT-RAMP-RESILIENT":"resilient_gray","MAT-GRAVEL":"gravel","MAT-SAND":"sand","MAT-GRES-BLACK":"gres_black","MAT-PLANTING-SUBSTRATE":"substrate","MAT-PAVING-UNRESOLVED":"paving","MAT-GRES-UNRESOLVED":"tile","MAT-GRES-WHITE":"gres_white","MAT-GRES-OCHRE":"gres_ochre","MAT-BITUMEN":"bitumen","MAT-OAK":"wood","MAT-CAST-IRON":"cast_iron","MAT-PAINTED-WOOD":"painted_wood","MAT-PARQUET":"wood","MAT-PAVING":"paving","MAT-TURF":"garden","MAT-GLASS":"glass","MAT-METAL":"metal","MAT-WOOD":"wood","MAT-TILE":"tile","MAT-MOSAIC-BLUE":"mosaic_blue","MAT-MOSAIC-BLACK":"mosaic_black"}.get(part["material"],"concrete")
    return {"rooflight": "glass", "parquet": "wood", "tile": "tile", "bath": "tile", "chaise": "tile", "counter": "tile",
            "paving": "paving", "grass_joint": "garden", "tap": "metal", "helical_handrail": "metal", "pivoting-hoist-jib": "metal"}.get(data["kind"], "concrete")


class _ReadOnlyDict(dict):
    """🧊 Shared JSON mapping whose owned copies remain ordinary mutable JSON."""
    def _deny(self,*args,**kwargs):
        raise TypeError("Visual snapshots are read-only; deepcopy for owned edits")
    __setitem__ = __delitem__ = clear = pop = popitem = setdefault = update = __ior__ = __setattr__ = _deny

    def __deepcopy__(self,memo):
        return {copy.deepcopy(k,memo):copy.deepcopy(v,memo) for k,v in self.items()}


class _ReadOnlyList(list):
    """🧊 Shared JSON sequence with explicit owned-copy semantics."""
    def _deny(self,*args,**kwargs):
        raise TypeError("Visual snapshots are read-only; deepcopy for owned edits")
    __setitem__ = __delitem__ = append = clear = extend = insert = pop = remove = reverse = sort = __iadd__ = __imul__ = _deny

    def __deepcopy__(self,memo):
        return [copy.deepcopy(v,memo) for v in self]


def _freeze(value):
    """❄️ Freeze JSON containers while sharing immutable scalar values."""
    if isinstance(value,dict):
        return _ReadOnlyDict((k,_freeze(v)) for k,v in value.items())
    if isinstance(value,list):
        return _ReadOnlyList(_freeze(v) for v in value)
    return value


def _retain(cache,key,value):
    """📦 Keep at most two source revisions per visual cache."""
    cache[key] = value
    while len(cache)>2:
        cache.pop(next(iter(cache)))
    return value


def _source_revision():
    """📄 Detect source publication without reparsing the model for each actor."""
    stat = _model.source_path().stat()
    return stat.st_mtime_ns,stat.st_size,_model.source_text


def source_model(model=None):
    """🗂️ Resolve a content-bound immutable source snapshot without constructing geometry."""
    global _default_source
    if isinstance(model,_ReadOnlyDict) and hasattr(model,"_source_digest"):
        return model
    if model is None:
        revision = _source_revision()
        if _default_source is not None and _default_source[0]==revision:
            hit = _sources.get(_default_source[1])
            if hit is not None:
                return hit
        model = json.loads(_model.source_text())
    encoded = json.dumps(model,ensure_ascii=False,sort_keys=True,separators=(",",":"))
    key = hashlib.sha256(encoded.encode("utf-8")).hexdigest()
    source = _sources.get(key)
    if source is None:
        source = _freeze(model)
        object.__setattr__(source,"_source_digest",key)
        _retain(_sources,key,source)
    if 'revision' in locals():
        _default_source = revision,key
    return source


def geometry_context(model=None):
    """🧭 Return one source snapshot and its matching immutable physical geometry."""
    source = source_model(model)
    key = source._source_digest,_model.building_geometry,tuple(_model.geometry_producers()),_freeze
    result = _snapshots.get(key)
    if result is None:
        result = _retain(_snapshots,key,_freeze(_model.building_geometry(source)))
    return source,result


def geometry(model=None) -> dict:
    """📐 Read physical geometry bound to source content and actual producer identities."""
    return geometry_context(model)[1]


def role(name: str, model=None) -> str:
    """🏷️ Resolve an authored semantic role without depending on an incidental object identifier."""
    return _model.role(name, model)


def membership(model=None) -> dict:
    """🧩 Resolve read-only discipline membership from the same source snapshot."""
    source = source_model(model)
    key = source._source_digest,_model.partial_model
    table = _members.get(key)
    if table is None:
        found = {}
        for name in DISCIPLINES:
            info = _model.partial_model(name,source)
            for guid in info.get("element_guids",()):
                found.setdefault(guid,set()).add(name)
        table = _retain(_members,key,_ReadOnlyDict((k,frozenset(v)) for k,v in found.items()))
    return table


def wall_frame(wall: dict, g: dict):
    """🧭 Outer edge, direction, left normal and length of an authored wall centre-line."""
    a, b = np.array(wall["start"], float), np.array(wall["end"], float)
    length = float(np.linalg.norm(b - a))
    along = (b - a) / length
    normal = np.array([-along[1], along[0]])
    return tuple(map(float, a - normal * wall["thickness"] / 2)), tuple(map(float, along)), tuple(map(float, normal)), length


def service_surface_role(data: dict, part: dict) -> str:
    """🛁 Keep explicitly authored decorative finishes on otherwise schematic service bodies."""
    role = surface_role(data,part)
    return role if role in ("resilient_gray","gravel","sand","mosaic_blue","mosaic_black","cast_iron","painted_wood","gres_white","gres_ochre","bitumen","gres_black","substrate") else "service"
