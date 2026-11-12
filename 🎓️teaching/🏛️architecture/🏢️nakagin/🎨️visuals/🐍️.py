"""🏢 Manim drawings of the Nakagin teaching model: capsule, orthographic views, plane axes, core, capital, podium, tower axonometric and elevation.

Every widget is a pure function of its parameters and returns a drawing with named parts, so ``always_redraw`` and ``Drafting.live`` can rebuild it every frame.
Related: [Kisho Kurokawa, Nakagin Capsule Tower](https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower)."""

from __future__ import annotations

import copy
import importlib
import io
import hashlib
import heapq
import json
import os
import pickle
import tempfile
from collections import OrderedDict
from collections.abc import Sequence
import math
import sys
from dataclasses import replace
from functools import lru_cache, wraps
from pathlib import Path
from typing import NamedTuple

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_fragments = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.✂️fragments.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_cache = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🗃️cache.🐍️")
_disk = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🗃️cache.💾️disk.🐍️")
_engine = importlib.import_module("manim")
_lod = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🎚️detail")

INK, MUTED, DATA, GOOD, WARN, ARCH, BACKGROUND, CONCRETE = _d.INK, _d.MUTED, _d.DATA, _d.GOOD, _d.WARN, _d.ARCH, _d.BACKGROUND, _d.CONCRETE
W_CUT, W_EDGE, W_THIN, W_HAIR, S_SMALL = _d.W_CUT, _d.W_EDGE, _d.W_THIN, _d.W_HAIR, _d.S_SMALL
CAPSULE = _theme.color("light-8-9")
CAPITAL = _theme.color("light-gray")
GLASS_DARK = _d.mix(BACKGROUND, ARCH, 0.14)
AXIS_X, AXIS_Y, AXIS_Z = _art.P_RED, _theme.color("diff-added"), _d.mix(_art.P_CYAN, "#2f55ff", 0.62)
TYPE_COLORS = (_art.P_CYAN, _art.P_ORANGE, _art.P_GREEN, _art.P_RED)
VIEWS = {"axon": (55.0, 24.0), "axon_back": (-55.0, 24.0), "axon_left": (125.0, 24.0), "axon_high": (55.0, 42.0)}
ORTHO = {"plan": "plan", "north": "north", "south": "south", "east": "east", "west": "west"}
LIGHT = np.array([-0.3, 0.45, 0.85]) / math.sqrt(0.09 + 0.2025 + 0.7225)

_TEXTS = _cache.BoundedCache("nakagin.text", max_entries=512, max_bytes=32 << 20, sizeof=lambda drawing: sum(member.points.nbytes + 512 for member in drawing.get_family()))
_ORDER_DISK_BYTES = 4 << 30
_ORDER_DISK_AGE_S = 30 * 24 * 3600
_ORDER_SUFFIX = ".jsonl"
_ORDER_PRUNE_EVERY = 16
_ORDER_STORES = [0]
_TOWERS: dict = {}
_TOWER_FACETS: dict = {}
_OCCLUSION_ORDERS: OrderedDict = OrderedDict()
_ORDER_MEMORY_ROWS = 50_000
_FROZEN_SHAPES = 200_000


def P(point) -> np.ndarray:
    """📏 Drawing point as a float 3-vector, padded with zeros, so projections and offsets work on it directly."""
    return np.array([*(float(value) for value in tuple(point)[:3]), 0.0, 0.0, 0.0][:3])


def type_color(key: str) -> str:
    """🎨 The fixed colour of a capsule type: A teal, B orange, C green, D red."""
    keys = [t.key for t in _model.capsule_types()]
    return TYPE_COLORS[keys.index(key) % len(TYPE_COLORS)]


@lru_cache(maxsize=8192)
def _blend(first: str, second: str, amount: float) -> str:
    return _d.mix(first, second, amount)


def shade_of(base: str, tone: float) -> str:
    """🌗 A face colour: the base lit by a tone in 0…1 against the background."""
    return _blend(BACKGROUND, base, round(float(tone), 3))


def tone_of(normal) -> float:
    """☀️ Face tone from a fixed light at the upper left front: roof 0.95, front 0.76, left 0.7, right 0.58."""
    return 0.58 + 0.4 * float(min(1.0, max(0.0, np.asarray(normal, float) @ LIGHT)))


class Mesh(NamedTuple):
    """🧊 Polyhedral mesh: vertices ``V``, outward unit normals ``N``, faces as index arrays and a name per face."""

    V: np.ndarray
    N: np.ndarray
    faces: tuple
    names: tuple


class Item(NamedTuple):
    """🧩 One depth-sorted unit; surface draw callbacks accept an actual polygons override."""

    tier: int
    centre: np.ndarray
    verts: np.ndarray
    draw: object
    normal: object = None
    polygons: tuple = ()
    opacity: float | None = None
    material: str | None = None
    geometry_key: bytes | None = None
    paint_owner: object = None
    paint_color: object = None
    part_key: str | None = None
    origin: object = None
    polygon: int | None = None


def source_files():
    """🗂️ Sources owned by the shared renderer and its fragment geometry contract."""
    return (Path(__file__),*_fragments.source_files())


class _FragmentRecipe(NamedTuple):
    """🧾 Immutable geometry-only source references, grouped only across consecutive paint fragments."""
    source: int
    polygon: int
    polygons: tuple
    normal: tuple


def _coalesce_fragments(rows):
    """🪢 Keep one compound fill for adjacent fragments without crossing intervening geometry."""
    result=[]
    for row in rows:
        if isinstance(row,int):
            result.append(row)
            continue
        if result and isinstance(result[-1],_FragmentRecipe) and (result[-1].source,result[-1].polygon,result[-1].normal)==(row.source,row.polygon,row.normal):
            previous=result[-1]
            result[-1]=previous._replace(polygons=(*previous.polygons,row.vertices))
        else:
            result.append(_FragmentRecipe(row.source,row.polygon,(row.vertices,),row.normal))
    return tuple(result)


def _source_item(item):
    """🔗 Resolve a current render fragment to its original current input surface."""
    return item if item.origin is None else item.origin


def _polygon_normal(vertices, orientation):
    """📐 Derive a stable actual facet plane from the largest local anchor triangle."""
    points=np.asarray(vertices,float)
    vectors=points[1:]-points[0]
    crosses=np.cross(vectors[:,None,:],vectors[None,:,:]).reshape(-1,3)
    normal=crosses[int(np.argmax(np.einsum('ij,ij->i',crosses,crosses)))]
    length=float(np.linalg.norm(normal))
    if not length:
        raise ValueError('A render boundary must have nonzero material area')
    normal=normal/length
    return normal if normal@orientation>=0 else -normal


def _polygon_facets(vertices):
    """🔺 Keep planar boundaries whole and facet warped quads along their shorter diagonal."""
    points=np.asarray(vertices,float)
    normal=_polygon_normal(points,np.cross(points[1]-points[0],points[-1]-points[0]))
    epsilon=64*math.ulp(float(np.max(np.abs(points))) or 1.)
    if np.max(np.abs((points-points[0])@normal))<=epsilon:
        return (points,)
    if len(points)!=4:
        raise ValueError('A nonplanar source boundary requires explicit triangular facets')
    diagonal02=float(np.sum((points[2]-points[0])**2))
    diagonal13=float(np.sum((points[3]-points[1])**2))
    indices=((0,1,2),(0,2,3)) if diagonal02<=diagonal13 else ((0,1,3),(1,2,3))
    return tuple(points[list(row)] for row in indices)


def _material_facing(item,towards):
    """🔦 Keep uncertain source material when any canonical facet winding faces the camera."""
    for polygon in item.polygons or (item.verts,):
        for facet in _polygon_facets(polygon):
            local=facet-facet[0]
            orientation=np.cross(local,np.roll(local,-1,axis=0)).sum(axis=0)
            if _polygon_normal(facet,orientation)@towards>1e-6:
                return True
    return False


def _original_boundary_segments(original, fragment, normal):
    """✏️ Clip only genuine source edges; partition and triangulation edges never become strokes."""
    axes=[i for i in range(3) if i!=int(np.argmax(np.abs(normal)))]
    boundary=np.asarray(fragment)[:,axes]
    sign=1 if _cross2(boundary,np.roll(boundary,-1,axis=0)).sum()>=0 else -1
    result=[]
    for start,end in zip(original,np.roll(original,-1,axis=0)):
        first,last=0.,1.
        for a,b in zip(boundary,np.roll(boundary,-1,axis=0)):
            low=float(sign*_cross2(b-a,start[axes]-a))+1e-10
            high=float(sign*_cross2(b-a,end[axes]-a))+1e-10
            slope=high-low
            if abs(slope)<1e-20:
                if low<0:
                    last=-1
                    break
            elif slope>0:
                first=max(first,-low/slope)
            else:
                last=min(last,-low/slope)
        if last>first+1e-12:
            result.append((start+(end-start)*first,start+(end-start)*last))
    return tuple(result)


def _paint_normal(item,polygon):
    """🪞 Preserve validated fragment planes; derive only unsplit source winding geometrically."""
    if item.origin is not None:
        return tuple(map(float,item.normal))
    points=np.asarray(polygon,float)
    local=points-points[0]
    winding=np.cross(local,np.roll(local,-1,axis=0)).sum(axis=0)
    return tuple(map(float,_polygon_normal(points,winding)))


def _fragment_item(items, face):
    """🌈 Paint only actual fragment polygons with current source styling and semantic owners."""
    current=items[face.source]
    origin=_source_item(current)
    polygons=tuple(np.asarray(polygon,float) for polygon in face.polygons)
    vertices=np.vstack(polygons)
    normal=np.array(face.normal,float)
    normal.flags.writeable=False
    boundary=(origin.polygons or (origin.verts,))[face.polygon]
    @wraps(current.draw)
    def draw(view,polygons=polygons):
        vertices=np.vstack(polygons)
        shape=current.draw(view,polygons=polygons)
        width=float(shape.get_stroke_width())
        color=shape.get_stroke_color()
        opacity=float(shape.get_stroke_opacity())
        shape.remove(*shape.submobjects)
        shape.set_stroke(width=0)
        shape.world_vertices=_frozen(vertices)
        shape.world_normal=normal
        shape.world_depth=_order(view,vertices.mean(axis=0))
        shape.source_polygon=face.polygon
        segments=tuple(segment for polygon in polygons for segment in _original_boundary_segments(np.asarray(boundary),polygon,normal)) if width else ()
        if segments:
            lines=_lean(_d.segments([tuple(_screen(view,np.asarray(segment))) for segment in segments],color,width,opacity))
            lines.world_vertices=_frozen(np.vstack(segments))
            lines.world_normal=normal
            lines.world_depth=_order(view,lines.world_vertices.mean(axis=0))
            shape.add(lines)
        return shape
    return current._replace(centre=vertices.mean(axis=0),verts=vertices,draw=draw,normal=normal,polygons=polygons,geometry_key=None,origin=origin,polygon=face.polygon)


def _order_directory():
    """📂 The occlusion-order cache beside the other teaching caches."""
    base = os.environ.get("SEMIO_TEACHING_CACHE_ROOT")
    return (_WORKSPACE_ROOT / base if base else _WORKSPACE_ROOT / ".🧬semio/🎓️teaching/⚡️cache") / "nakagin-orders"


_DIGEST: list = [None, None]


def _source_digest():
    """🧬 Digest of every source file that decides an ordering, so a code change invalidates all stored orders; revalidated against the file stamps on every call so a long-lived editor process never serves a stale digest."""
    paths = [Path(path) for path in source_files()]
    stamp = tuple((str(path), path.stat().st_mtime_ns, path.stat().st_size) for path in paths)
    if _DIGEST[0] != stamp:
        digest = hashlib.blake2b(digest_size=20)
        for path in paths:
            digest.update(path.read_bytes())
        _DIGEST[:] = [stamp, digest.hexdigest()]
    return _DIGEST[1]


def _order_path(key):
    """🔑 The file of one ordering, named by the digest of the sources (its orphan marker) and a digest of the geometry key."""
    sources = _source_digest()
    return _order_directory() / f"{sources[:12]}-{hashlib.blake2b(key + sources.encode(), digest_size=20).hexdigest()}{_ORDER_SUFFIX}"


def _order_rows(order):
    """📤 JSON rows of a recipe tuple, produced lazily one row at a time: source indices stay integers, fragments become objects."""
    for row in order:
        yield row if isinstance(row, int) else {"source": row.source, "polygon": row.polygon, "polygons": [[[float(c) for c in p] for p in polygon] for polygon in row.polygons], "normal": [float(c) for c in row.normal]}


def _frozen_array(values):
    array = np.array(values, float)
    array.flags.writeable = False
    return array


def _order_recipes(rows):
    """📥 Recipe tuple from JSON rows, consumed as a stream; the arrays are read-only so a cached order can never be mutated through a restored item."""
    return tuple(row if isinstance(row, int) else _FragmentRecipe(row["source"], row["polygon"], tuple(_frozen_array(polygon) for polygon in row["polygons"]), _frozen_array(row["normal"])) for row in rows)


def _orphan_order(path):
    return not path.name.startswith(_source_digest()[:12])


def _prune_orders(keep=frozenset()):
    """🧹 Bound the order folder by bytes and age: orders of other source digests go first, then the least recently used."""
    return _disk.prune(_order_directory(), max_bytes=_ORDER_DISK_BYTES, max_age_s=_ORDER_DISK_AGE_S, suffixes=(_ORDER_SUFFIX,), orphan=_orphan_order, keep=keep)


def _store_order(key, order):
    """💾 Persist one ordering atomically, streamed row by row into a unique temporary file; any failure leaves the cache untouched."""
    path = _order_path(key)
    try:
        _disk.publish_lines(path, {"key": path.stem, "sources": _source_digest()}, _order_rows(order))
    except OSError:
        return
    _ORDER_STORES[0] += 1
    if _ORDER_STORES[0] % _ORDER_PRUNE_EVERY == 1:
        _prune_orders(frozenset((path.name,)))


def _load_order(key):
    """📖 Load a stored ordering only when its recorded key and sources match; anything else rebuilds. Reading is line by line and marks the entry recently used."""
    path = _order_path(key)
    try:
        records = _disk.read_lines(path)
        header = next(records)
        if header["key"] != path.stem or header["sources"] != _source_digest():
            return None
        order = _order_recipes(records)
    except (OSError, ValueError, KeyError, TypeError, StopIteration):
        return None
    _disk.touch(path)
    return order


def _remember_order(key, order):
    """🧠 Keep small orders in memory; an order of a whole building stays on disk only, because its recipes cost about a gigabyte."""
    if len(order) > _ORDER_MEMORY_ROWS:
        return
    _OCCLUSION_ORDERS[key] = order
    while len(_OCCLUSION_ORDERS) > 24:
        _OCCLUSION_ORDERS.popitem(last=False)


class _Restored(Sequence):
    """🔂 Painter-ordered items bound afresh from geometry recipes, built on demand so a whole building never holds all its fragment items at once."""

    def __init__(self, items, recipes):
        self.items, self.recipes = items, recipes

    def __len__(self):
        return len(self.recipes)

    def __getitem__(self, index):
        if isinstance(index, slice):
            return [self[position] for position in range(*index.indices(len(self)))]
        row = self.recipes[index]
        return self.items[row] if isinstance(row, int) else _fragment_item(self.items, row)

    def __iter__(self):
        for row in self.recipes:
            yield self.items[row] if isinstance(row, int) else _fragment_item(self.items, row)


def _restore_order(items, recipes):
    """🔁 Reuse only geometry recipes, binding every current style and owner afresh."""
    return _Restored(items, recipes)


class NakaginDrawing3D(_engine.VGroup):
    """🏗️ A Nakagin drawing: ``view``, named ``parts``, ``anchors`` and ``at(x, y, z)`` back to screen space."""

    def __init__(self, view=None):
        super().__init__()
        self.view, self.parts, self.anchors = view, {}, {}

    def at(self, *args) -> np.ndarray:
        """📍 Screen position of a model coordinate (or of a part tag)."""
        if len(args) == 1 and isinstance(args[0], str):
            return self.anchors[args[0]] if args[0] in self.anchors else np.array(self.parts[args[0]].get_center())
        return self.view(*args)


def make_view(view, at, scale: float, centre) -> _d.View:
    """🔭 The projection of a drawing: an axon name, ``(azimuth, elevation)``, ``plan``, ``north|south|east|west`` or a ready view; ``centre`` is the model point that sits at ``at``."""
    if isinstance(view, _d.View):
        return _d.View(view.kind, at, scale, centre, azimuth=view.azimuth, elevation=view.elevation, face=view.face)
    if isinstance(view, str) and view in ORTHO:
        return _d.View("plan" if view == "plan" else "elevation", at, scale, centre, face="south" if view == "plan" else view)
    azimuth, elevation = VIEWS[view] if isinstance(view, str) else view
    return _d.View("axon", at, scale, centre, azimuth=azimuth, elevation=elevation)


def project(point, view: _d.View) -> np.ndarray:
    """🌐 Screen position of a model point in a view."""
    return view(*P(point))


def _basis(view: _d.View):
    if view.kind == "axon":
        return np.array([view.right, view.up]), view.towards
    if view.kind == "plan":
        return np.array([[1.0, 0, 0], [0, 1.0, 0]]), np.array([0.0, 0, 1.0])
    table = {"south": ((1, 0, 0), (0, 0, 1), (0, -1, 0)), "north": ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
             "east": ((0, 1, 0), (0, 0, 1), (1, 0, 0)), "west": ((0, -1, 0), (0, 0, 1), (-1, 0, 0))}
    a, b, t = table[view.face]
    return np.array([a, b], float), np.array(t, float)


def _screen(view: _d.View, points) -> np.ndarray:
    B, _ = _basis(view)
    local = (np.asarray(points, float).reshape(-1, 3) - view.centre) @ B.T
    out = np.zeros((len(local), 3))
    out[:, :2] = view.point[:2] + view.scale * local
    return out


def _order(view: _d.View, centre) -> float:
    _, T = _basis(view)
    offset = np.asarray(centre, float) - view.centre
    return float(offset @ T)


def _poly_points(polys) -> np.ndarray:
    chunks = []
    for poly in polys:
        ring = np.vstack([poly, poly[:1]])
        a, b = ring[:-1], ring[1:]
        chunks.append(np.stack([a, a + (b - a) / 3, a + 2 * (b - a) / 3, b], 1).reshape(-1, 3))
    return np.vstack(chunks)


class _Facet(_engine.VMobject):
    """🪶 A VMobject keeping its unchanged defaults on the class, so a drawing of many faces costs about a third of the memory per face; copies share the read-only world data."""

    def __deepcopy__(self, memo):
        clone = object.__new__(type(self))
        memo[id(self)] = clone
        state = {}
        for name, value in self.__dict__.items():
            if name in _WORLD:
                state[name] = value
            elif isinstance(value, np.ndarray):
                state[name] = value.copy()
            elif name == "submobjects":
                state[name] = [copy.deepcopy(child, memo) for child in value]
            else:
                state[name] = copy.deepcopy(value, memo)
        return _adopt(clone, state)

    def __getattr__(self, name):
        if name in _LAZY:
            try:
                source = object.__getattribute__(self, "_source")
            except AttributeError:
                raise AttributeError(name) from None
            world = source[0].world(source[1])
            if name in world:
                return world[name]
        raise AttributeError(name)


_LAZY = ("world_vertices", "world_normal", "world_depth", "world_facets")
_WORLD = frozenset((*_LAZY, "_source", "paint", "semantic_role", "semantic_key", "semantic_sources", "source_polygon"))


_OWN = frozenset(("points", "submobjects", "updaters", "fill_rgbas", "stroke_rgbas", "background_stroke_rgbas"))
_DEFAULTS = {name: value for name, value in vars(_engine.VMobject()).items() if name not in _OWN}
for _name, _value in _DEFAULTS.items():
    if isinstance(_value, np.ndarray):
        _value.flags.writeable = False
    setattr(_Facet, _name, _value)


def _lean(shape):
    """🧹 Drop every instance attribute equal to the class default of a ``_Facet``, leaving the drawing unchanged."""
    return _adopt(object.__new__(_Facet), {name: value for name, value in shape.__dict__.items() if name not in _DEFAULTS or not _default(value, _DEFAULTS[name])})


def _interned(value):
    """🧵 One shared string object per distinct text."""
    return sys.intern(value) if isinstance(value, str) else value


def _adopt(shape, state):
    """🧲 Give a face its attributes one by one, so faces of the same make share one key table instead of each holding a dictionary."""
    for name, value in state.items():
        setattr(shape, name, value)
    return shape


def _default(value, default):
    """🟰 Whether an instance attribute equals the class default of the same type."""
    if type(value) is not type(default):
        return False
    return bool(np.array_equal(value, default)) if isinstance(value, np.ndarray) else bool(value == default)


def _share_style(drawing):
    """🫧 Let the faces of a cached drawing share one read-only colour array per distinct colour; clones copy them again before any change."""
    table = {}
    for shape in drawing.get_family():
        if isinstance(shape, _Facet):
            for name in ("fill_rgbas", "stroke_rgbas", "background_stroke_rgbas"):
                array = shape.__dict__.get(name)
                if array is not None and array.flags.writeable:
                    key = (name, array.tobytes())
                    if key not in table:
                        table[key] = _frozen(array.copy())
                    shape.__dict__[name] = table[key]
    return drawing


def _frozen(array):
    """🥶 Make an array read-only so drawings may share it."""
    array.flags.writeable = False
    return array


def _shape(polys, fill, opacity: float = 1.0, stroke=None, width: float = 0.0, stroke_opacity: float = 1.0):
    shape = _engine.VMobject()
    shape.set_points(_poly_points(polys))
    shape.set_fill(fill, opacity)
    shape.set_stroke(stroke or fill, width, stroke_opacity)
    return _lean(shape)


def _circle(centre, u, v, radius: float, count: int = 28, start: float = 0.0, stop: float = 360.0) -> np.ndarray:
    angles = np.radians(np.linspace(start, stop, count, endpoint=stop - start < 360))
    return np.asarray(centre, float) + radius * (np.cos(angles)[:, None] * np.asarray(u, float) + np.sin(angles)[:, None] * np.asarray(v, float))


def _profile(w: float, h: float, r: float, segments: int) -> np.ndarray:
    r = max(min(r, w / 2 - 1e-3, h / 2 - 1e-3), 1e-3)
    points = []
    for (cx, cz), start in (((w / 2 - r, r), -90), ((w / 2 - r, h - r), 0), ((-w / 2 + r, h - r), 90), ((-w / 2 + r, r), 180)):
        for index in range(segments + 1):
            angle = math.radians(start + 90 * index / segments)
            points.append((cx + r * math.cos(angle), cz + r * math.sin(angle)))
    return np.array(points)


@lru_cache(maxsize=512)
def capsule_mesh(w: float, l: float, h: float, r: float, segments: int) -> Mesh:
    """💊 The rounded-profile capsule mesh in its local frame: x centred, back at y=0, front at y=l, base at z=0; face names front, back, top, bottom, left, right."""
    profile = _profile(w, h, r, segments)
    n = len(profile)
    V = np.vstack([np.column_stack([profile[:, 0], np.zeros(n), profile[:, 1]]), np.column_stack([profile[:, 0], np.full(n, l), profile[:, 1]])])
    edge = np.roll(profile, -1, axis=0) - profile
    normal = np.column_stack([edge[:, 1], np.zeros(n), -edge[:, 0]])
    normal /= np.linalg.norm(normal, axis=1)[:, None]
    faces = [np.arange(n)[::-1], np.arange(n, 2 * n)] + [np.array([i, (i + 1) % n, n + (i + 1) % n, n + i]) for i in range(n)]
    names = ["back", "front"] + [("top" if nz > 0 else "bottom") if abs(nz) > abs(nx) else ("right" if nx > 0 else "left") for nx, _, nz in normal]
    return Mesh(V, np.vstack([[0, -1.0, 0], [0, 1.0, 0], normal]), tuple(faces), tuple(names))


def box_mesh(x0: float, x1: float, y0: float, y1: float, z0: float, z1: float) -> Mesh:
    """📦 An axis-aligned box mesh; face names bottom, top, back (−y), right (+x), front (+y), left (−x)."""
    V = np.array([(x0, y0, z0), (x1, y0, z0), (x1, y1, z0), (x0, y1, z0), (x0, y0, z1), (x1, y0, z1), (x1, y1, z1), (x0, y1, z1)], float)
    faces = ((0, 3, 2, 1), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7))
    N = np.array([(0, 0, -1), (0, 0, 1), (0, -1, 0), (1, 0, 0), (0, 1, 0), (-1, 0, 0)], float)
    return Mesh(V, N, tuple(np.array(f) for f in faces), ("bottom", "top", "back", "right", "front", "left"))


def solid_mesh(solid) -> Mesh:
    """🔷 A mesh from a model ``Solid`` (outward-wound vertex loops)."""
    V = np.array(solid.vertices, float)
    faces, normals = [], []
    for loop in solid.faces:
        ring = V[list(loop)]
        normal = np.cross(ring[1] - ring[0], ring[2] - ring[0]) if len(ring) > 2 else np.array([0, 0, 1.0])
        total = np.zeros(3)
        for a, b in zip(ring, np.roll(ring, -1, axis=0)):
            total += np.cross(a, b)
        normal = total if np.linalg.norm(total) > 1e-9 else normal
        faces.append(np.array(loop))
        normals.append(normal / max(np.linalg.norm(normal), 1e-12))
    return Mesh(V, np.array(normals), tuple(faces), tuple(f"face{i}" for i in range(len(faces))))


def _world(mesh: Mesh, plane) -> Mesh:
    R = np.array([plane.x, plane.y, plane.z], float)
    return Mesh(np.asarray(plane.origin, float) + mesh.V @ R, mesh.N @ R, mesh.faces, mesh.names)


_CAP_NAMES = ('width', 'length', 'height', 'corner_radius', 'corner_segments', 'window_diameter', 'window_u', 'window_v', 'interior_width', 'interior_length', 'interior_height')
_CAP_KEYS = ('w', 'l', 'h', 'r', 'seg', 'd', 'u', 'v', 'interior_width', 'interior_length', 'interior_height')


def _clip_mesh(mesh, planes):
    """✂️ Intersect source material with exact halfspaces without adding section caps."""
    planes=tuple((np.asarray(normal,float),float(limit)) for normal,limit in planes)
    if not planes or all((mesh.V@normal<=limit).all() for normal,limit in planes):
        return mesh
    vertices,normals,faces,names=[],[],[],[]
    for index,face in enumerate(mesh.faces):
        source=mesh.V[face]
        epsilon=_fragments._epsilon(source)
        if any((source@normal>limit).all() for normal,limit in planes):
            continue
        if all((source@normal<=limit).all() for normal,limit in planes):
            boundaries=(source,)
        else:
            boundaries=[]
            for facet in _polygon_facets(source):
                winding=np.cross(facet-facet[0],np.roll(facet-facet[0],-1,axis=0)).sum(axis=0)
                normal=_polygon_normal(facet,winding)
                prepared=_fragments.ordered_fragments((_fragments.Face(0,0,tuple(map(tuple,facet)),tuple(normal)),),tuple(normal))
                points=[]
                identifiers={}
                def point(value):
                    key=tuple(map(float,value))
                    if key not in identifiers:
                        identifiers[key]=len(points)
                        points.append(np.asarray(key,float))
                    return identifiers[key]
                pieces=[tuple(point(value) for value in part.vertices) for part in prepared]
                crossings={}
                for plane_index,(axis,limit) in enumerate(planes):
                    clipped=[]
                    for piece in pieces:
                        result=[]
                        for a,b in zip(piece,(*piece[1:],piece[0])):
                            da,db=float(points[a]@axis-limit),float(points[b]@axis-limit)
                            da=0.0 if abs(da)<=epsilon else da
                            db=0.0 if abs(db)<=epsilon else db
                            if da<=0:
                                result.append(a)
                            if (da<=0)!=(db<=0):
                                if da==0 or db==0:
                                    result.append(a if da==0 else b)
                                    continue
                                edge=(plane_index,min(a,b),max(a,b))
                                if edge not in crossings:
                                    first,last=edge[1:]
                                    start,stop=points[first],points[last]
                                    fraction=(limit-float(start@axis))/float((stop-start)@axis)
                                    crossings[edge]=point(start+(stop-start)*fraction)
                                result.append(crossings[edge])
                        result=tuple(value for i,value in enumerate(result) if value!=result[i-1])
                        if len(result)>=3:
                            clipped.append(result)
                    pieces=clipped
                edges=set()
                for piece in pieces:
                    for a,b in zip(piece,(*piece[1:],piece[0])):
                        if a==b:
                            continue
                        if (b,a) in edges:
                            edges.remove((b,a))
                        else:
                            edges.add((a,b))
                while edges:
                    chain=[min(edges)[0]]
                    positions={chain[0]:0}
                    while True:
                        adjacent=sorted(b for a,b in edges if a==chain[-1])
                        if not adjacent:
                            raise ValueError('Clipped source boundary must remain closed')
                        target=adjacent[0]
                        edges.remove((chain[-1],target))
                        if target in positions:
                            loop=chain[positions[target]:]
                            polygon=np.asarray([points[value] for value in loop],float)
                            if len(loop)>=3 and np.linalg.norm(np.cross(polygon-polygon[0],np.roll(polygon-polygon[0],-1,axis=0)).sum(axis=0))>0:
                                boundaries.append(polygon)
                            if positions[target]==0:
                                break
                            chain=chain[:positions[target]+1]
                            positions={value:i for i,value in enumerate(chain)}
                        else:
                            positions[target]=len(chain)
                            chain.append(target)
        for polygon in boundaries:
            faces.append(np.arange(len(vertices),len(vertices)+len(polygon)))
            vertices.extend(polygon)
            normals.append(mesh.N[index])
            names.append(mesh.names[index])
    return Mesh(np.asarray(vertices,float).reshape(-1,3),np.asarray(normals,float).reshape(-1,3),tuple(faces),tuple(names))


def _cap(t) -> dict:
    if isinstance(t, dict):
        if "layout" in t:
            raise ValueError("Physical family must be supplied through the explicit layout argument")
        basis = t.get("dimensional_basis")
        if basis == "physical":
            t = _model.capsule_parameters_from_casing(t)
        elif basis != "nominal":
            raise ValueError("Capsule dictionaries must declare dimensional_basis as nominal or physical")
    base = _model.capsule_type(t['key']) if isinstance(t, dict) and 'key' in t else (_model.capsule_types()[0] if isinstance(t, dict) else t)
    get = lambda name: float(t[name] if isinstance(t, dict) and name in t else getattr(base, name))
    cap = dict(zip(_CAP_KEYS[:8], (get(name) for name in _CAP_NAMES[:8])))
    cap['seg'], cap['key'] = (int(cap['seg']) if isinstance(t, dict) and 'corner_segments' in t else None), getattr(base, 'key', None)
    for name in _CAP_NAMES[8:]:
        if not isinstance(t, dict) or name in t:
            cap[name] = get(name)
    return cap


def _cap_parameters(cap):
    return {"dimensional_basis": "nominal", **{name: cap[key] for name, key in zip(_CAP_NAMES, _CAP_KEYS) if key in cap and cap[key] is not None}}


def _window_outline(cap, count=28, layout="Slash"):
    circle = _model.capsule_window(_cap_parameters(cap), layout)
    casing = _model.capsule_casing(_cap_parameters(cap))
    normal = np.asarray(circle.normal)
    centre = np.asarray(circle.center) + normal * ((casing.front - circle.center[1]) if normal[1] else (casing.right - circle.center[0]))
    tangent = np.array([1.0, 0, 0]) if normal[1] else np.array([0., 1., 0.])
    return _circle(centre, tangent, np.array([0, 0, 1.0]), circle.radius, count)


def _visible(mesh: Mesh, view: _d.View, minimum: float = 1e-6):
    _, T = _basis(view)
    return [i for i, n in enumerate(mesh.N @ T) if n > minimum]


def _detail(mesh: Mesh, view: _d.View, base: str, colors: dict):
    S = _screen(view, mesh.V)
    faces, rings = {}, {}
    for i in _visible(mesh, view):
        name = mesh.names[i]
        fill = colors.get(name, base)
        color = shade_of(fill, tone_of(mesh.N[i]))
        faces.setdefault(name, _engine.VGroup()).add(_shape([S[mesh.faces[i]]], color, 1.0, color, 0.7))
        rings.setdefault(name, []).append(S[mesh.faces[i]][:, :2])
    return faces, rings


def _material(material, base):
    return {"shell": base, "metal": _blend(base, INK, 0.26), "glass": GLASS_DARK,
            "concrete": CONCRETE, "wood": _blend(CAPITAL, WARN, 0.18), "ceramic": CAPSULE,
            "fabric": _blend(base, BACKGROUND, 0.35), "paper": "#E4D9BA", "brass": "#A8893B", "rubber": "#262A29", "diffuser": "#F4ECD6", "dark": _blend(BACKGROUND, INK, 0.22)}.get(material, base)


def _slot_look(slot, *, lit, tint, types, base):
    """🖌 Colour and lit state of one tower slot: base or type blend, the tint override, dimmed when not lit."""
    key = (slot.building, slot.tower, slot.storey, slot.facade, slot.item)
    color = _blend(base, type_color(slot.type), 0.5) if types else base
    color = _match(tint, key) or color
    on = lit is None or _match(lit, key, True)
    return (color if on else _blend(BACKGROUND, color, 0.4)), on


def _recolour(group, colour, outlined):
    """🖍️ Recolour a drawn capsule in place from the paint of its shapes, equal to drawing it afresh in ``colour``."""
    for shape in list(group.get_family()):
        paint = getattr(shape, "paint", None)
        if paint is None:
            continue
        material, tone, hairline, cover = paint
        fill = shade_of(_material(material, colour), tone)
        stroke = fill if cover else (DATA if outlined else shade_of(_material(material, colour), 0.42))
        width = 1.2 if outlined else hairline
        shape.set_fill(fill, family=False)
        if cover:
            shape.set_stroke(stroke, family=False)
        elif hasattr(shape, "source_polygon"):
            shape.set_stroke(stroke, family=False)
            for line in list(shape.submobjects):
                if width:
                    line.set_stroke(stroke, width)
                else:
                    shape.remove(line)
        else:
            shape.set_stroke(stroke, width, family=False)


@lru_cache(maxsize=256)
def _capsule_parts(parameters, detail, layout="Slash", interior="reference", program=None):
    parameters = {name: value for name, value in zip(_CAP_NAMES, parameters) if value is not None}
    if program is not None:
        parameters["key"] = program
    return tuple((part, solid_mesh(part.solid)) for part in _model.capsule_assembly(parameters, detail=detail, layout=layout, interior=interior).parts)


def _slot_parts(slot, segments=None):
    cap = _cap(_model.capsule_type(slot.type))
    cap["seg"] = segments
    return _capsule_parts(tuple(cap.get(key) for key in _CAP_KEYS), "exterior", slot.layout)


def _cap_parts(cap, detail="exterior", interior="reference", layout="Slash"):
    return _capsule_parts(tuple(cap.get(key) for key in _CAP_KEYS), detail, layout=layout, interior=interior, program=cap["key"] if detail == "full" and interior == "study" else None)


def _semantic_face(normal):
    axis = int(np.argmax(np.abs(normal)))
    return (("left", "right"), ("back", "front"), ("bottom", "top"))[axis][int(normal[axis] > 0)]


def _assembly_items(parts, *, base, groups, plane=None, highlight=None, window=True, bolts=True, face_groups=None, lod=False, outline=None, glass_opacity=.25):
    if not 0 <= glass_opacity <= 1:
        raise ValueError("Glass opacity must lie between zero and one")
    items, paint_owners = [], {}
    for part, local in parts:
        if not window and (part.solid.meta.get("window_unit") or part.role in ("window_rim", "window_frame", "glazing")):
            continue
        if not bolts and part.role in ("bolt_shaft", "bolt_head"):
            continue
        mesh = local if plane is None else _world(local, plane)
        group = groups.setdefault(part.key, _engine.VGroup())
        category = "window" if part.solid.meta.get("window_unit") else ("bolt" if part.role in ("bolt_shaft", "bolt_head") else "body")
        owner = paint_owners.get(category)
        if owner is None:
            owner = paint_owners[category] = _engine.VGroup()
        group.material_underpaint = owner
        buckets = {}
        for index, face in enumerate(mesh.faces):
            normal = mesh.N[index]
            lighting_normal = normal
            if lod and part.role == "shell":
                axis = int(np.argmax(np.abs(local.N[index])))
                local_normal = np.zeros(3)
                local_normal[axis] = np.sign(local.N[index][axis])
                lighting_normal = local_normal if plane is None else local_normal @ np.array([plane.x, plane.y, plane.z], float)
            polygon = mesh.V[face]
            local_normal=local.N[index]
            local_polygon=local.V[local.faces[index]]
            depth=round(float(local_normal@local_polygon[0]),7)
            key=(*np.round(local_normal,7),depth)
            buckets.setdefault(key, []).append((index, polygon, normal, lighting_normal))
        for polygons in buckets.values():
            index = polygons[0][0]
            vertices, normal = np.vstack([row[1] for row in polygons]), polygons[0][2]
            lighting_normal = polygons[0][3]
            name = _semantic_face(local.N[index])
            color = _material(part.material, base)
            if isinstance(highlight, dict) and part.role.startswith("shell"):
                color = highlight.get(name, color)
            def draw(v, polygons=tuple(row[1] for row in polygons), normal=normal, lighting_normal=lighting_normal, color=color, group=group, role=part.role, name=name, part_key=part.key, opacity=glass_opacity if part.material == "glass" else 1.0, material=part.material):
                vertices=np.vstack(polygons)
                tone = tone_of(lighting_normal)
                fill = shade_of(color, tone)
                hairline = 0 if role in ("shell", "window_rim", "window_frame", "glazing") else 0.45
                shape = _shape([_screen(v, polygon) for polygon in polygons], fill, opacity, outline or shade_of(color, 0.42), 1.2 if outline else hairline)
                shape.paint = (material, tone, hairline, False)
                shape.world_depth = _order(v, vertices.mean(axis=0))
                shape.world_vertices = _frozen(vertices.copy())
                shape.world_normal = _frozen(normal.copy())
                shape.semantic_role, shape.semantic_key = role, part_key
                group.add(shape)
                owners = [group]
                if face_groups is not None and role in ("shell", "shell_back"):
                    face_group = face_groups.setdefault(name, _engine.VGroup())
                    face_group.add(shape)
                    owners.append(face_group)
                shape._batch_groups = tuple(owners)
                return shape
            intrinsic = hashlib.blake2b(digest_size=20)
            intrinsic.update(np.asarray((plane.x, plane.y, plane.z) if plane is not None else np.eye(3), float).tobytes())
            for face_index, _, _, _ in polygons:
                intrinsic.update(np.asarray(local.V[local.faces[face_index]], float).tobytes())
                intrinsic.update(np.asarray(local.N[face_index], float).tobytes())
            items.append(Item(0, vertices.mean(axis=0), vertices, draw, normal, tuple(row[1] for row in polygons), glass_opacity if part.material == "glass" else 1.0, part.material, intrinsic.digest(), owner, color, part.key))
    return items


def _capsule_lite(cap, plane, view, base, *, window=True, highlight=None, layout="Slash"):
    """🧷 Project a shared exterior capsule assembly onto an existing drawing plane."""
    groups = {}
    parts = _cap_parts(cap, layout=layout)
    items = _assembly_items(parts, base=base, groups=groups, glass_opacity=1, plane=plane, window=window, lod=True, outline=highlight)
    return _engine.VGroup(*_paint_ordered(_section_order(items, view), view))


class _ProjectedPolygon(NamedTuple):
    """⬜ Cached projected boundaries, support intervals and interior probes for one physical face."""
    points: np.ndarray
    lower: np.ndarray
    upper: np.ndarray
    edges: np.ndarray
    axes: np.ndarray
    minimum: np.ndarray
    maximum: np.ndarray
    convex: bool
    samples: np.ndarray


def _cross2(a, b):
    return a[..., 0] * b[..., 1] - a[..., 1] * b[..., 0]


def _prepare_polygon(points):
    points = np.asarray(points, float)
    edges = np.roll(points, -1, axis=0) - points
    area = _cross2(points, np.roll(points, -1, axis=0)).sum()
    normals = np.column_stack((-edges[:, 1], edges[:, 0]))
    lengths = np.linalg.norm(normals, axis=1)
    axes = normals[lengths > 1e-12] / lengths[lengths > 1e-12, None]
    projection = points @ axes.T
    turns = _cross2(edges, np.roll(edges, -1, axis=0))
    convex = bool((turns >= -1e-12).all() or (turns <= 1e-12).all())
    inward = normals * (1 if area >= 0 else -1) / np.maximum(lengths[:, None], 1e-15)
    epsilon = max(float(np.ptp(points, axis=0).max()), 1) * 1e-8
    prepared = _ProjectedPolygon(points, points.min(axis=0), points.max(axis=0), edges, axes, projection.min(axis=0), projection.max(axis=0), convex, np.empty((0, 2)))
    probes = np.vstack((points.mean(axis=0), points + edges / 2 + inward * epsilon))
    samples = np.vstack((points, probes[_inside_polygon(probes, prepared)]))
    return prepared._replace(samples=samples)


def _inside_polygon(points, polygon):
    a, delta = polygon.points, polygon.edges
    b = a + delta
    offset = points[:, None, :] - a
    distance = (offset * delta).sum(axis=2)
    boundary = (np.abs(_cross2(delta, offset)) <= 1e-10) & (distance >= -1e-10) & (distance <= (delta * delta).sum(axis=1) + 1e-10)
    crossing = (a[:, 1] > points[:, None, 1]) != (b[:, 1] > points[:, None, 1])
    divisor = np.where(np.abs(delta[:, 1]) > 1e-15, delta[:, 1], 1)
    x = a[:, 0] + (points[:, None, 1] - a[:, 1]) * delta[:, 0] / divisor
    return (((crossing & (points[:, None, 0] < x)).sum(axis=1) % 2) == 1) & ~boundary.any(axis=1)


def _overlap_candidates(first, second, *, strict=False, margin=0.0):
    """🎯 Yield only overlapping polygon bounds in original order without Python pair scans."""
    if not first or not second:
        return
    if len(second)==1:
        lower,upper=second[0].lower,second[0].upper
        for i,polygon in enumerate(first):
            minimum,maximum=polygon.lower,polygon.upper
            rejected=((maximum<=lower+margin).any() or (upper<=minimum+margin).any()) if strict else ((maximum<lower).any() or (upper<minimum).any())
            if not rejected:
                yield i,0
        return
    lower=np.asarray([polygon.lower for polygon in second],float).reshape(-1,2)
    upper=np.asarray([polygon.upper for polygon in second],float).reshape(-1,2)
    for i,polygon in enumerate(first):
        minimum,maximum=polygon.lower,polygon.upper
        mask=~((maximum<=lower+margin)|(upper<=minimum+margin)).any(axis=1) if strict else ~((maximum<lower)|(upper<minimum)).any(axis=1)
        for j in np.flatnonzero(mask):
            yield i,int(j)


def _projected_overlap(first, second):
    """🔎 Require positive projected face overlap rather than bounding-box contact."""
    first = [polygon if isinstance(polygon, _ProjectedPolygon) else _prepare_polygon(polygon) for polygon in first]
    second = [polygon if isinstance(polygon, _ProjectedPolygon) else _prepare_polygon(polygon) for polygon in second]
    for i,j in _overlap_candidates(first,second,strict=True,margin=1e-10):
        a,b=first[i],second[j]
        if a.convex and b.convex:
            if not len(a.axes) or not len(b.axes):
                continue
            pb = b.points @ a.axes.T
            if ((pb.max(axis=0) <= a.minimum + 1e-10) | (pb.min(axis=0) >= a.maximum - 1e-10)).any():
                continue
            pa = a.points @ b.axes.T
            if ((pa.max(axis=0) <= b.minimum + 1e-10) | (pa.min(axis=0) >= b.maximum - 1e-10)).any():
                continue
            return True
        if _inside_polygon(a.samples, b).any() or _inside_polygon(b.samples, a).any():
            return True
        offset = b.points[None, :, :] - a.points[:, None, :]
        denominator = _cross2(a.edges[:, None, :], b.edges[None, :, :])
        safe = np.where(np.abs(denominator) > 1e-12, denominator, 1)
        ta = _cross2(offset, b.edges[None, :, :]) / safe
        tb = _cross2(offset, a.edges[:, None, :]) / safe
        if ((np.abs(denominator) > 1e-12) & (ta > 1e-9) & (ta < 1 - 1e-9) & (tb > 1e-9) & (tb < 1 - 1e-9)).any():
            return True
    return False


def _cyclic_components(edges):
    """🕸️ Isolate broad-phase cycles before exact projected polygon tests."""
    visited, order = set(), []
    for start in range(len(edges)):
        if start in visited:
            continue
        stack = [(start, False)]
        while stack:
            node, finished = stack.pop()
            if finished:
                order.append(node)
            elif node not in visited:
                visited.add(node)
                stack.append((node, True))
                stack.extend((target, False) for target in edges[node] if target not in visited)
    reverse = [[] for _ in edges]
    for source, targets in enumerate(edges):
        for target in targets:
            reverse[target].append(source)
    visited = set()
    for start in reversed(order):
        if start in visited:
            continue
        component, stack = set(), [start]
        while stack:
            node = stack.pop()
            if node in visited:
                continue
            visited.add(node)
            component.add(node)
            stack.extend(reverse[node])
        if len(component) > 1:
            yield component


_OPAQUE_VISIBILITY = OrderedDict()


def _convex_difference(subject, cover):
    """🪓 Subtract one convex opaque projection, preserving disjoint visible convex fragments."""
    if _cross2(cover, np.roll(cover, -1, axis=0)).sum() < 0:
        cover = cover[::-1]
    remaining, inside = [], subject
    for a, b in zip(cover, np.roll(cover, -1, axis=0)):
        if len(inside) < 3:
            break
        distance = _cross2(b - a, inside - a) / max(float(np.linalg.norm(b - a)), 1e-15) + 1e-10
        kept, rejected = [], []
        for i, point in enumerate(inside):
            following = (i + 1) % len(inside)
            (kept if distance[i] >= 0 else rejected).append(point)
            if (distance[i] >= 0) != (distance[following] >= 0):
                intersection = point + (inside[following] - point) * distance[i] / (distance[i] - distance[following])
                kept.append(intersection)
                rejected.append(intersection)
        if len(rejected) >= 3:
            polygon = np.asarray(rejected)
            if abs(float(_cross2(polygon, np.roll(polygon, -1, axis=0)).sum())) > 1e-14:
                remaining.append(polygon)
        inside = np.asarray(kept)
    return remaining


def _single_cover_bounds(cover):
    """🗳️ Bound the actual expanded convex half-planes, retaining uncertain acute geometry."""
    lengths=np.linalg.norm(cover.edges,axis=1)
    if not cover.convex or not np.isfinite(lengths).all() or np.any(lengths==0):
        return np.full(2,-np.inf),np.full(2,np.inf)
    unit=cover.edges/lengths[:,None]
    half=.5*np.linalg.norm(unit+np.roll(unit,1,axis=0),axis=1)
    sine=float(half.min())-64*np.finfo(float).eps
    if sine<=0:
        return np.full(2,-np.inf),np.full(2,np.inf)
    margin=1e-10*float(np.max(np.maximum(lengths,1e-15)/lengths))/sine
    margin+=64*math.ulp(float(np.max(np.abs(cover.points))) or 1.)
    return np.nextafter(cover.lower-margin,-np.inf),np.nextafter(cover.upper+margin,np.inf)


def _single_cover(footprints, cover, bounds=None):
    """🛡️ Prove every target vertex lies in one expanded convex material projection."""
    if not cover.convex or not footprints:
        return False
    points = np.asarray(footprints[0],float) if len(footprints)==1 else np.vstack(footprints)
    lower,upper=_single_cover_bounds(cover) if bounds is None else bounds
    if (points.min(axis=0)<lower).any() or (points.max(axis=0)>upper).any():
        return False
    orientation = 1 if _cross2(cover.points, np.roll(cover.points, -1, axis=0)).sum() >= 0 else -1
    distance = orientation * _cross2(cover.edges, points[:, None, :] - cover.points) / np.maximum(np.linalg.norm(cover.edges, axis=1), 1e-15) + 1e-10
    return bool((distance >= 0).all())


def _coverage_samples_possible(footprint, covers):
    """🔍 Retain exposed footprint probes; covered probes still require exact union subtraction."""
    probes = np.vstack((footprint, footprint.mean(axis=0)))
    for cover in covers:
        orientation = 1 if _cross2(cover.points, np.roll(cover.points, -1, axis=0)).sum() >= 0 else -1
        distance = orientation * _cross2(cover.edges, probes[:, None, :] - cover.points) / np.maximum(np.linalg.norm(cover.edges, axis=1), 1e-15) + 1e-10
        probes = probes[~(distance >= 0).all(axis=1)]
        if not len(probes):
            return True
    return False


def _raster_footprint(scale, stroke_width):
    """🔬 Bound Cairo half-stroke plus two antialias pixels in physical projected units."""
    pixel = max(float(_engine.config.frame_width) / _engine.config.pixel_width, float(_engine.config.frame_height) / _engine.config.pixel_height)
    return (stroke_width * .005 + 2 * pixel) / scale


def _opaque_visible(items, view, padding=0.0):
    """🧿 Retain uncertain faces; remove only projections covered by physically nearer opaque convex polygons."""
    if not items:
        return []
    basis, towards = _basis(view)
    digest = hashlib.blake2b(digest_size=20)
    digest.update(np.asarray(basis, float).tobytes())
    digest.update(np.asarray([padding], float).tobytes())
    anchor = items[0].centre
    for item in items:
        digest.update(np.asarray([item.tier, len(item.verts), *(len(p) for p in item.polygons)], np.int64).tobytes())
        if item.geometry_key is not None:
            digest.update(item.geometry_key)
        else:
            digest.update(np.asarray(item.verts - anchor, float).tobytes())
            digest.update(np.asarray(item.normal if item.normal is not None else [0, 0, 0], float).tobytes())
        digest.update(np.asarray([-1 if item.opacity is None else item.opacity], float).tobytes())
        digest.update(str(item.material).encode())
    signature = digest.digest()
    if signature in _OPAQUE_VISIBILITY:
        _OPAQUE_VISIBILITY.move_to_end(signature)
        return [items[index] for index in _OPAQUE_VISIBILITY[signature]]
    planes={i:_item_plane(item) for i,item in enumerate(items) if item.tier>=0 and item.normal is not None}
    eligible=[i for i,item in enumerate(items) if item.tier<0 or item.normal is None or (planes[i][0]@towards>1e-6 if planes[i] is not None else _material_facing(item,towards))]
    polygons = {i: [_prepare_polygon(np.asarray(p) @ basis.T) for p in (items[i].polygons or (items[i].verts,))] for i in eligible}
    bounds = {i: (np.vstack([p.points for p in polygons[i]]).min(axis=0), np.vstack([p.points for p in polygons[i]]).max(axis=0)) for i in eligible}
    cells, large = {}, set()
    def grid(lower, upper):
        lo, hi = np.floor(lower / .25).astype(int), np.floor(upper / .25).astype(int)
        return ((x, y) for x in range(lo[0], hi[0] + 1) for y in range(lo[1], hi[1] + 1))
    for i in eligible:
        item = items[i]
        if item.opacity is None or item.opacity < 1 or item.normal is None or item.tier < 0 or planes[i] is None:
            continue
        lo, hi = bounds[i]
        if np.prod(np.floor(hi / .25) - np.floor(lo / .25) + 1) > 2048:
            large.add(i)
        else:
            for cell in grid(lo, hi):
                cells.setdefault(cell, set()).add(i)
    areas={id(p):abs(float(_cross2(p.points,np.roll(p.points,-1,axis=0)).sum()))/2 for rows in polygons.values() for p in rows if p.convex}
    blockers={i:tuple(sorted((p for p in polygons[i] if p.convex),key=lambda p:-areas[id(p)])) for i in eligible}
    priorities={i:max((areas[id(p)] for p in blockers[i]),default=0.) for i in eligible}
    single_bounds={id(p):_single_cover_bounds(p) for rows in blockers.values() for p in rows}
    retained = []
    for i in eligible:
        item = items[i]
        if item.normal is None or item.tier < 0:
            retained.append(i)
            continue
        lo, hi = bounds[i]
        candidates = set(large)
        for cell in grid(lo, hi):
            candidates.update(cells.get(cell, ()))
        lower, upper = lo - padding, hi + padding
        footprint = np.array([lower, [upper[0], lower[1]], upper, [lower[0], upper[1]]])
        single = False
        covers = []
        for j in sorted(candidates-{i},key=lambda index:(-priorities[index],index)):
            other = items[j]
            lower, upper = bounds[j]
            if (upper < lo).any() or (lower > hi).any():
                continue
            axis=planes[j][0]
            if float((item.verts@axis).max())>=float((other.verts@axis).min())-1e-7:
                continue
            if any(_single_cover([footprint],p,single_bounds[id(p)]) for p in blockers[j]):
                single = True
                break
            covers.extend(blockers[j])
        if single:
            continue
        covers.sort(key=lambda p: float(np.prod(p.upper - p.lower)), reverse=True)
        hidden = bool(covers)
        for polygon in polygons[i]:
            if not hidden or (not polygon.convex and padding <= 0):
                hidden = False
                break
            lower, upper = polygon.lower - padding, polygon.upper + padding
            footprint = np.array([lower, [upper[0], lower[1]], upper, [lower[0], upper[1]]]) if padding > 0 else polygon.points
            if not _coverage_samples_possible(footprint, covers):
                hidden = False
                break
            fragments = [footprint]
            for cover in covers:
                fragments = [piece for fragment in fragments for piece in ([fragment] if (fragment.max(axis=0) < cover.lower).any() or (fragment.min(axis=0) > cover.upper).any() else _convex_difference(fragment, cover.points))]
                if not fragments:
                    break
            if fragments:
                hidden = False
                break
        if not hidden:
            retained.append(i)
    result = tuple(retained)
    _OPAQUE_VISIBILITY[signature] = result
    while len(_OPAQUE_VISIBILITY) > 64:
        _OPAQUE_VISIBILITY.popitem(last=False)
    return [items[index] for index in result]


def _batch_pair(first, second):
    """🧺 Combine consecutive opaque facets sharing their semantic owners and paint."""
    a, b = getattr(first, "_batch_groups", ()), getattr(second, "_batch_groups", ())
    if not a or len(a) != len(b) or any(x is not y for x, y in zip(a, b)):
        return False
    if first.submobjects or second.submobjects or first.get_stroke_width() or second.get_stroke_width():
        return False
    if not np.all(first.fill_rgbas[:, 3] == 1) or not np.array_equal(first.fill_rgbas, second.fill_rgbas):
        return False
    if any(getattr(first, key, None) != getattr(second, key, None) for key in ("paint", "semantic_role", "semantic_key")):
        return False
    if not np.array_equal(first.world_normal, second.world_normal):
        return False
    amin, amax = first.points[:, :2].min(axis=0), first.points[:, :2].max(axis=0)
    bmin, bmax = second.points[:, :2].min(axis=0), second.points[:, :2].max(axis=0)
    if not np.any((amax + .1 < bmin) | (bmax + .1 < amin)):
        return False
    facets = getattr(first, "world_facets", ((first.world_vertices, first.world_normal),))
    first.world_facets = (*facets, *getattr(second, "world_facets", ((second.world_vertices, second.world_normal),)))
    first.world_vertices = _frozen(np.vstack([first.world_vertices, second.world_vertices]))
    first.set_points(np.vstack([first.points, second.points]))
    for group in a:
        group.remove(second)
    return True


def _paint_ordered(items, view):
    """🖼️ Paint actual opaque material unions before their shaded facets, retaining source holes and physical ordering."""
    bundles = {}
    for item in items:
        if item.paint_owner is not None and item.paint_color is not None and item.opacity == 1 and item.normal is not None:
            polygons, facets, vertices, parts = bundles.setdefault((id(item.paint_owner), item.material, str(item.paint_color)), ([], [], [], []))
            normal = _frozen(np.array(item.normal))
            for polygon in item.polygons or (item.verts,):
                screen = _screen(view, polygon)
                polygons.append(screen if _cross2(screen[:, :2], np.roll(screen[:, :2], -1, axis=0)).sum() >= 0 else screen[::-1])
                facets.append((_frozen(np.array(polygon)), normal))
            vertices.append(item.verts)
            parts.append(item.part_key)
    painted = []
    for item in items:
        bundle = bundles.pop((id(item.paint_owner), item.material, str(item.paint_color)), None)
        if bundle is not None:
            polygons, facets, vertices, parts = bundle
            coverage = _shape(polygons, shade_of(item.paint_color, .85), 1)
            coverage.world_vertices = _frozen(np.vstack(vertices))
            coverage.world_depth = _order(view, coverage.world_vertices.mean(axis=0))
            coverage.world_facets, coverage.world_normal = tuple(facets), None
            coverage.semantic_role, coverage.semantic_key = "material_coverage", None
            coverage.paint = (item.material, .85, 0, True)
            coverage.semantic_sources = tuple(dict.fromkeys(part for part in parts if part is not None))
            item.paint_owner.add(coverage)
            if painted and hasattr(painted[-1], "_batch_groups"):
                del painted[-1]._batch_groups
            painted.append(coverage)
        shape = item.draw(view)
        if not painted or not _batch_pair(painted[-1], shape):
            if painted and hasattr(painted[-1], "_batch_groups"):
                del painted[-1]._batch_groups
            painted.append(shape)
        elif hasattr(shape, "_batch_groups"):
            del shape._batch_groups
    if painted and hasattr(painted[-1], "_batch_groups"):
        del painted[-1]._batch_groups
    return painted


def _item_plane(item):
    """🪛 Return one actual common geometric plane only when every boundary vertex shares it."""
    if item.normal is None:
        return None
    polygon=(item.polygons or (item.verts,))[0]
    normal=_polygon_normal(polygon,item.normal)
    epsilon=64*math.ulp(float(np.max(np.abs(item.verts))) or 1.)
    if np.max(np.abs((item.verts-polygon[0])@normal))>epsilon:
        return None
    return normal,float(normal@polygon[0])


def _overlap_plane_order(first, second, projected_first, projected_second, basis, towards, planes=None):
    """⛰️ Order actual overlapping convex surface regions only when their full ray-depth sign agrees."""
    planes=(_item_plane(first),_item_plane(second)) if planes is None else planes
    if any(plane is None for plane in planes):
        return 2 if _projected_overlap(projected_first,projected_second) else 0
    normals=np.asarray([plane[0] for plane in planes])
    denominators=normals@towards
    if np.any(denominators<=1e-6):
        return 0
    gradients=normals@basis.T/denominators[:,None]
    constants=np.array([plane[1] for plane in planes])/denominators
    minimum,maximum=float('inf'),float('-inf')
    for i,j in _overlap_candidates(projected_first,projected_second):
        one,two=projected_first[i],projected_second[j]
        if not one.convex or not two.convex:
            return 0
        overlap=one.points
        boundary=two.points
        if _cross2(boundary,np.roll(boundary,-1,axis=0)).sum()<0:
            boundary=boundary[::-1]
        for start,end in zip(boundary,np.roll(boundary,-1,axis=0)):
            if len(overlap)<3:
                break
            distances=_cross2(end-start,overlap-start)
            clipped=[]
            for i,point in enumerate(overlap):
                following=(i+1)%len(overlap)
                if distances[i]>=0:
                    clipped.append(point)
                if (distances[i]>=0)!=(distances[following]>=0):
                    clipped.append(point+(overlap[following]-point)*distances[i]/(distances[i]-distances[following]))
            overlap=np.asarray(clipped)
        if len(overlap)<3 or abs(float(_cross2(overlap,np.roll(overlap,-1,axis=0)).sum()))<1e-14:
            continue
        difference=constants[1]-constants[0]-overlap@(gradients[1]-gradients[0])
        minimum=min(minimum,float(difference.min()))
        maximum=max(maximum,float(difference.max()))
    if minimum<-1e-9 and maximum>1e-9:
        return 2
    if minimum>=-1e-9 and maximum>1e-9:
        return 1
    if maximum<=1e-9 and minimum<-1e-9:
        return -1
    return 0


def _candidate_support(vertices, starts, lengths, candidates, other_vertices, normals, other_normal):
    """⚡ Project actual support intervals only for overlapping candidates in bounded blocks."""
    result = np.empty((4, len(candidates)))
    for first in range(0, len(candidates), 128):
        block = candidates[first:first + 128]
        sizes = lengths[block]
        offsets = np.cumsum(np.r_[0, sizes[:-1]])
        indices = np.repeat(starts[block] - offsets, sizes) + np.arange(sizes.sum())
        projection = vertices[indices] @ other_normal
        result[0, first:first + len(block)] = np.minimum.reduceat(projection, offsets)
        result[1, first:first + len(block)] = np.maximum.reduceat(projection, offsets)
        opposite = other_vertices @ normals[block].T
        result[2, first:first + len(block)] = opposite.min(axis=0)
        result[3, first:first + len(block)] = opposite.max(axis=0)
    return tuple(result)


def _dependency_order(edges, depths):
    """🔀 Respect condensation dependencies and refine residual cycles before local depth choices."""
    ordered, pending = [], [list(range(len(edges)))] if edges else []
    while pending:
        nodes = pending.pop()
        if len(nodes) == 1:
            ordered.extend(nodes)
            continue
        local = {node: index for index, node in enumerate(nodes)}
        adjacency = [{local[target] for target in edges[node] if target in local} for node in nodes]
        groups = list(_cyclic_components(adjacency))
        grouped = set().union(*groups)
        groups.extend({index} for index in range(len(nodes)) if index not in grouped)
        if len(groups) == 1:
            pivot = min(nodes, key=lambda node: (depths[node], node))
            ordered.append(pivot)
            pending.append([node for node in nodes if node != pivot])
            continue
        component = {node: index for index, group in enumerate(groups) for node in group}
        dag = [set() for _ in groups]
        for source, targets in enumerate(adjacency):
            dag[component[source]].update(component[target] for target in targets if component[target] != component[source])
        incoming = [0] * len(groups)
        for targets in dag:
            for target in targets:
                incoming[target] += 1
        priority = [min((depths[nodes[node]], nodes[node]) for node in group) for group in groups]
        ready = [(*priority[index], index) for index in range(len(groups)) if not incoming[index]]
        heapq.heapify(ready)
        components = []
        while ready:
            _, _, index = heapq.heappop(ready)
            components.append(index)
            for target in dag[index]:
                incoming[target] -= 1
                if not incoming[target]:
                    heapq.heappush(ready, (*priority[target], target))
        if len(components) != len(groups):
            raise ValueError("Strong-component condensation must be acyclic")
        pending.extend([nodes[node] for node in sorted(groups[index])] for index in reversed(components))
    return ordered


def _order_checkpoint(progress,cancelled,phase,completed,total):
    """⏱️ Report actual bounded ordering work and honour explicit cancellation before continuing."""
    if cancelled is not None and cancelled():
        raise _fragments.FragmentCancelled()
    if progress is not None:
        progress({'phase':phase,'completed':completed,'total':total})
        if cancelled is not None and cancelled():
            raise _fragments.FragmentCancelled()


def _section_order(items, view, progress=None, cancelled=None):
    """👁️ Draw overlapping surfaces behind their physical separating planes, with a projected sweep and bounded geometry cache."""
    if cancelled is not None and cancelled():
        raise _fragments.FragmentCancelled()
    basis, towards = _basis(view)
    checking=progress is not None or cancelled is not None
    digest = hashlib.blake2b(digest_size=20)
    digest.update(np.asarray(towards, float).tobytes())
    digest.update(np.asarray(basis, float).tobytes())
    tiers, counts, polygon_lengths, normals, vertices = [], [], [], [], []
    for completed,item in enumerate(items):
        if checking and completed%128==0:
            _order_checkpoint(progress,cancelled,'occlusion-inputs',completed,len(items))
        tiers.append(item.tier)
        counts.append((len(item.verts), len(item.polygons)))
        polygon_lengths.extend(len(polygon) for polygon in item.polygons)
        normals.append((0, 0, 0) if item.normal is None else item.normal)
        vertices.append(np.asarray(item.verts, float))
    for array in (np.asarray(tiers, np.int64), np.asarray(counts, np.int64).reshape(-1, 2), np.asarray(polygon_lengths, np.int64), np.asarray(normals, float).reshape(-1, 3), np.concatenate(vertices) if vertices else np.zeros((0, 3))):
        digest.update(array.tobytes())
    if checking:
        _order_checkpoint(progress,cancelled,'occlusion-inputs',len(items),len(items))
    key = digest.digest()
    if key in _OCCLUSION_ORDERS:
        _OCCLUSION_ORDERS.move_to_end(key)
        return _restore_order(items,_OCCLUSION_ORDERS[key])
    stored=_load_order(key)
    if stored is not None:
        _remember_order(key,stored)
        return _restore_order(items,stored)
    source_planes=[]
    for completed,item in enumerate(items):
        if checking and completed%128==0:
            _order_checkpoint(progress,cancelled,'occlusion-planes',completed,len(items))
        source_planes.append(_item_plane(item) if item.tier>=0 else None)
    if checking:
        _order_checkpoint(progress,cancelled,'occlusion-planes',len(items),len(items))
    background = [index for index, item in enumerate(items) if item.tier < 0]
    indices = [index for index, item in enumerate(items) if item.tier >= 0 and (item.normal is None or source_planes[index] is None or source_planes[index][0]@towards>1e-6)]
    if not indices:
        return [items[index] for index in background]
    visible = [items[index] for index in indices]
    planes=[source_planes[index] for index in indices]
    bounds,polygons=[],[]
    for completed,item in enumerate(visible):
        if checking and completed%128==0:
            _order_checkpoint(progress,cancelled,'occlusion-projection',completed,len(visible))
        points=np.asarray(item.verts)@basis.T
        bounds.append((points.min(axis=0),points.max(axis=0)))
        polygons.append([_prepare_polygon(facet@basis.T) for polygon in (item.polygons or (item.verts,)) for facet in _polygon_facets(polygon)])
    if checking:
        _order_checkpoint(progress,cancelled,'occlusion-projection',len(visible),len(visible))
    bounds=np.asarray(bounds)
    normals = np.asarray([([0,0,0] if item.normal is None else towards) if plane is None else plane[0] for item,plane in zip(visible,planes)])
    _, representatives, axis_indices = np.unique(np.round(normals, 10), axis=0, return_index=True, return_inverse=True)
    axes = normals[representatives]
    lengths = np.asarray([len(item.verts) for item in visible], int)
    starts = np.cumsum(np.r_[0, lengths[:-1]]) if visible else np.zeros(0, int)
    vertices = np.vstack([item.verts for item in visible]) if visible else np.zeros((0, 3))
    support_normals = axes[axis_indices]
    separation = 1e-7 + 2 * float(np.abs(vertices).max(initial=0)) * float(np.abs(normals - support_normals).sum(axis=1).max(initial=0))
    projection = np.einsum('ij,ij->i', vertices, np.repeat(support_normals, lengths, axis=0))
    own_minimum = np.minimum.reduceat(projection, starts)
    own_maximum = np.maximum.reduceat(projection, starts)
    edges = [set() for _ in visible]
    count=len(visible)
    pool_index,pool_x,pool_low,pool_high=np.empty(count,int),np.empty(count),np.empty(count),np.empty(count)
    size=0
    for completed,b in enumerate(sorted(range(count),key=lambda index:bounds[index,0,0])):
        if checking and completed%128==0:
            _order_checkpoint(progress,cancelled,'occlusion-overlap',completed,count)
        if completed%256==0 and size:
            alive=pool_x[:size]>=bounds[b,0,0]
            size=int(alive.sum())
            pool_index[:size],pool_x[:size],pool_low[:size],pool_high[:size]=(pool[:len(alive)][alive] for pool in (pool_index,pool_x,pool_low,pool_high))
        second = visible[b]
        if second.normal is not None and size:
            near=(pool_x[:size]>=bounds[b,0,0])&(pool_high[:size]>=bounds[b,0,1])&(pool_low[:size]<=bounds[b,1,1])
            candidates=pool_index[:size][near]
            low, high, other_low, other_high = _candidate_support(vertices, starts, lengths, candidates, second.verts, support_normals, support_normals[b])
            forward = (other_low > own_maximum[candidates] + separation) | (high < own_minimum[b] - separation)
            backward = (other_high < own_minimum[candidates] - separation) | (low > own_maximum[b] + separation)
            for a, ahead, behind in zip(candidates, forward, backward):
                if visible[a].normal is None:
                    continue
                if ahead == behind:
                    if planes[a] is not None and planes[b] is not None and np.linalg.norm(planes[a][0]-planes[b][0])<1e-10:
                        continue
                    order=_overlap_plane_order(visible[a],second,polygons[a],polygons[b],basis,towards,(planes[a],planes[b]))
                    uncertain=not order and any(not polygon.convex for polygon in (*polygons[a],*polygons[b]))
                    if order==2 or (uncertain and _projected_overlap(polygons[a],polygons[b])):
                        edges[int(a)].add(b)
                        edges[b].add(int(a))
                        continue
                    if not order:
                        continue
                    source,target=(int(a),b) if order>0 else (b,int(a))
                else:
                    source,target=(int(a),b) if ahead else (b,int(a))
                edges[source].add(target)
        pool_index[size],pool_x[size],pool_low[size],pool_high[size]=b,bounds[b,1,0],bounds[b,0,1],bounds[b,1,1]
        size+=1
    if checking:
        _order_checkpoint(progress,cancelled,'occlusion-overlap',len(visible),len(visible))
    cyclic=list(_cyclic_components(edges))
    refinement_total=sum(len(edges[source]&component) for component in cyclic for source in component) if checking else 0
    refinement_completed=0
    for component in cyclic:
        for source in component:
            for target in edges[source] & component:
                if checking and refinement_completed%128==0:
                    _order_checkpoint(progress,cancelled,'occlusion-refine',refinement_completed,refinement_total)
                if not _projected_overlap(polygons[source], polygons[target]):
                    edges[source].remove(target)
                refinement_completed+=1
    if checking:
        _order_checkpoint(progress,cancelled,'occlusion-refine',refinement_completed,refinement_total)
    depths = [_order(view, item.centre) for item in visible]
    components=[tuple(sorted(row)) for row in _cyclic_components(edges)]
    assigned={index for row in components for index in row}
    components.extend((index,) for index in range(len(visible)) if index not in assigned)
    labels={index:group for group,row in enumerate(components) for index in row}
    dependencies=[set() for _ in components]
    for source,targets in enumerate(edges):
        dependencies[labels[source]].update(labels[target] for target in targets if labels[target]!=labels[source])
    component_depths=[min(depths[index] for index in row) for row in components]
    recipes=list(background)
    for completed,group in enumerate(_dependency_order(dependencies,component_depths)):
        if cancelled is not None and cancelled():
            raise _fragments.FragmentCancelled()
        row=components[group]
        if len(row)==1 and (visible[row[0]].normal is None or planes[row[0]] is not None):
            recipes.append(indices[row[0]])
        else:
            faces=[]
            for index in row:
                item=visible[index]
                for polygon_id,polygon in enumerate(item.polygons or (item.verts,)):
                    source_polygon=polygon_id if item.polygon is None else item.polygon
                    facets=_polygon_facets(polygon)
                    for facet in facets:
                        orientation=np.cross(facet[1]-facet[0],facet[2]-facet[0]) if len(facets)>1 else item.normal
                        normal=_polygon_normal(facet,orientation)
                        if normal@towards>1e-6:
                            faces.append(_fragments.Face(indices[index],source_polygon,tuple(tuple(map(float,p)) for p in facet),tuple(map(float,normal))))
            try:
                pieces=_fragments.ordered_fragments(tuple(faces),tuple(map(float,towards)),progress=progress,cancelled=cancelled)
            except ValueError:
                for face in faces:
                    try:
                        _fragments.ordered_fragments((face,),tuple(map(float,towards)),cancelled=cancelled)
                    except ValueError as error:
                        raise ValueError(f'{items[face.source].part_key} source {face.source} polygon {face.polygon}: {error}') from error
                raise
            recipes.extend(pieces)
        if progress is not None:
            progress({'phase':'occlusion-components','completed':completed+1,'total':len(components)})
    order=_coalesce_fragments(recipes)
    _store_order(key,order)
    _remember_order(key,order)
    return _restore_order(items,order)


def _part_group(groups):
    roots = {id(part.material_underpaint): part.material_underpaint for part in groups.values() if hasattr(part, "material_underpaint")}
    group = _engine.VGroup(*roots.values(), *groups.values())
    group.semantic_parts = groups
    return group


def _parts(assembly):
    return tuple((part, solid_mesh(part.solid)) for part in assembly.parts)


@lru_cache(maxsize=64)
def _core_parts(index, storeys, side, pitch, height, detail):
    t = replace(_model.towers()[index], storeys=storeys, core_side=side, storey_height=pitch, height=height)
    return _parts(_model.core_assembly(t, detail=detail))


@lru_cache(maxsize=32)
def _capital_parts(index, height, capital_height, overhang, side):
    return _parts(_model.capital_assembly(replace(_model.towers()[index], height=height, capital_height=capital_height, capital_overhang=overhang, core_side=side)))


@lru_cache(maxsize=1)
def _podium_parts():
    return _parts(_model.podium_assembly())


@lru_cache(maxsize=1)
def _bridge_parts():
    return tuple(item for level in _model.load()["detail"]["bridge_storeys"] for item in _parts(_model.bridge_assembly(level)))


class _Saver(pickle.Pickler):
    """🥡 Pickles a drawing with every face replaced by its number, numbering faces as they appear."""

    def __init__(self, file, ids, order):
        super().__init__(file, protocol=pickle.HIGHEST_PROTOCOL)
        self.ids, self.order = ids, order

    def persistent_id(self, value):
        if not isinstance(value, _Facet):
            return None
        index = self.ids.get(id(value))
        if index is None:
            index = self.ids[id(value)] = len(self.order)
            self.order.append(value)
        return index


class _Loader(pickle.Unpickler):
    """🍳 Unpickles a drawing, asking ``build`` for the face of every number."""

    def __init__(self, file, build):
        super().__init__(file)
        self.build = build

    def persistent_load(self, index):
        return self.build(index)


class _Frozen:
    """❄️ A cached whole-building drawing resting in a temporary file: only the group skeleton and the byte offset of every face stay in memory, so a clone is the only live copy of the faces."""

    def __init__(self, drawing):
        ids, order, skeletons = {}, [], []
        for value in (drawing, *drawing.capsules.values()):
            buffer = io.BytesIO()
            _Saver(buffer, ids, order).dump(value)
            skeletons.append(buffer.getvalue())
        self.skeleton, self.capsule_skeletons = skeletons[0], dict(zip(drawing.capsules, skeletons[1:]))
        self.coverage_padding, self.outlined, self.view = drawing.coverage_padding, drawing.outlined, drawing.view
        directory = _order_directory().with_name("nakagin-shapes")
        directory.mkdir(parents=True, exist_ok=True)
        self.file = tempfile.TemporaryFile(dir=directory)
        offsets, index = [0], 0
        while index < len(order):
            core, world = {}, {}
            for name, value in order[index].__dict__.items():
                (world if name in _LAZY else core)[name] = value
            for part in (core, world):
                buffer = io.BytesIO()
                _Saver(buffer, ids, order).dump(part)
                self.file.write(buffer.getbuffer())
                offsets.append(offsets[-1] + buffer.tell())
            index += 1
        self.file.flush()
        self.offsets, self.count = np.asarray(offsets, np.int64), len(order)

    def _record(self, record):
        self.file.seek(int(self.offsets[record]))
        return io.BytesIO(self.file.read(int(self.offsets[record + 1] - self.offsets[record])))

    def world(self, index):
        """🌍 The world-space data of one face, read only when somebody asks for it."""
        return pickle.load(self._record(2 * index + 1))

    def _build(self, made, index, scale, offset):
        shape = made.get(index)
        if shape is None:
            shape = made[index] = object.__new__(_Facet)
            state = _Loader(self._record(2 * index), lambda other: self._build(made, other, scale, offset)).load()
            state["_source"] = (self, index)
            if scale is not None:
                state["points"] = state["points"] * scale + offset
            if "paint" in state:
                state["paint"] = (_interned(state["paint"][0]), *state["paint"][1:])
            for name in ("semantic_role", "semantic_key"):
                if name in state:
                    state[name] = _interned(state[name])
            _adopt(shape, state)
        return shape

    def thaw(self, scale, offset):
        """♨️ A fresh live drawing at ``points * scale + offset``, read from the file face by face."""
        made = {}
        for index in range(self.count):
            self._build(made, index, scale, offset)
        drawing = _Loader(io.BytesIO(self.skeleton), lambda index: made[index]).load()
        for member in drawing.get_family():
            if not isinstance(member, _Facet) and len(member.points):
                member.points = member.points * scale + offset
        return drawing

    def capsule(self, key):
        """🧸 A fresh copy of one capsule group at template scale."""
        made = {}
        return _Loader(io.BytesIO(self.capsule_skeletons[key]), lambda index: self._build(made, index, None, None)).load()


class NakaginDrawing:
    """🎞️ Scene mixin: Nakagin capsule, views, axes, core, capital, podium and tower drawings. Combine with ``Drafting`` for text and dimensions."""

    def _text(self, string: str, point=(0, 0, 0), *, size: float = S_SMALL, color=DATA, anchor: str = "c"):
        key = _cache.scoped_key("nakagin.text", string, size, color)
        drawing = _TEXTS.get(key)
        if drawing is None:
            drawing = self.text(string, (0, 0, 0), size=size, color=color)
            _TEXTS.put(key, drawing)
        return _d.place(drawing.copy(), point, anchor)

    def _away(self, start, end, text, centre, offset: float, color=DATA):
        start, end = P(start), P(end)
        delta = end - start
        normal = np.array([-delta[1], delta[0], 0.0])
        normal /= max(np.linalg.norm(normal), 1e-9)
        sign = 1.0 if float(normal @ ((start + end) / 2 - P(centre))) >= 0 else -1.0
        shape = self.dimension(start, end, None, offset=sign * abs(offset), color=color)
        label = self._text(text, (0, 0, 0), color=color)
        shift = abs(normal[0]) * label.width / 2 + abs(normal[1]) * label.height / 2 + 0.07
        shape.add(label.move_to((start + end) / 2 + normal * sign * (abs(offset) + shift)))
        shape.label = label
        return shape

    def _chip(self, string: str, point, color=DATA):
        label = self._text(string, point, color=color)
        chip = _engine.RoundedRectangle(corner_radius=0.05, width=label.width + 0.14, height=label.height + 0.1, stroke_width=0, fill_color=BACKGROUND, fill_opacity=0.82).move_to(label)
        return _engine.VGroup(chip, label)

    def analytical_box_axon(self, width, length, height, *, at=(0, 0, 0), scale=.62, view="axon", dims=False, base=CAPSULE):
        """📊 Draw a requested analytical space exactly, without fabrication allowances or capsule fittings."""
        width, length, height = float(width), float(length), float(height)
        if min(width, length, height) <= 0:
            raise ValueError("Analytical box dimensions must be positive")
        part = _model.NamedPart("analytical-volume", "analytical_volume", _model._box(-width / 2, width / 2, 0, length, 0, height), "shell")
        groups = {}
        items = _assembly_items(_parts(_model.Assembly((part,))), base=base, groups=groups)
        drawing = self._compose(items, view, at, scale)
        drawing.parts, drawing.dims = groups, {}
        drawing.dimensional_basis = "analytical"
        if dims:
            v = drawing.view
            centre = v(0, length / 2, height / 2)
            spans = {"width": ((-width / 2, length, 0), (width / 2, length, 0), width), "length": ((width / 2, 0, 0), (width / 2, length, 0), length), "height": ((-width / 2, length, 0), (-width / 2, length, height), height)}
            for name, (a, b, value) in spans.items():
                dimension = self._away(v(*a), v(*b), self.quantity(value, "m", 2), centre, .42)
                dimension.measurement_value, dimension.measurement_unit = value, "m"
                dimension.world_endpoints = np.asarray([a, b])
                drawing.dims[name] = dimension
                drawing.add(dimension)
        return drawing

    def window_unit_axon(self, t, *, layout="Slash", state=None, at=(0, 0, 0), scale=1, view="axon", glass_opacity=.25, base=CAPSULE):
        """🪟 Inspect the standalone physical window at an authored state without moving or concealing its components."""
        parameters = _cap_parameters(_cap(t))
        assembly = _model.capsule_window_unit(parameters, layout, state)
        groups = {}
        drawing = self._compose(_assembly_items(_parts(assembly), base=base, groups=groups, glass_opacity=glass_opacity), view, at, scale)
        drawing.parts = drawing.semantic_parts = groups
        drawing.source_assembly, drawing.layout = assembly, layout
        drawing.unit_state = dict(assembly.parts[0].solid.meta["window_state"])
        drawing.aperture = _model.capsule_window(parameters, layout)
        return drawing

    def capsule_axon(self, t, *, at=(0, 0, 0), scale: float = 0.62, view="axon", window: bool = True, bolts: bool = True, dims=(),
                     highlight=(), hatch=(), base: str = CAPSULE, segments: int | None = None, dim_offset: float = 0.42, cutaway: bool = False, interior: str = "reference", layout: str = "Slash", glass_opacity: float = .25):
        """🧴 One shaded capsule in axonometric with its round window, physical bolts enabled by default, dimension lines and hatched faces.

        ``t`` is a ``CapsuleType`` or a dict of parameters (e.g. live width/length/height/window_diameter); ``at`` is the screen position of the capsule centre.
        ``dims`` picks from ``width length height window``; ``hatch`` picks ``exposed``, ``attached`` or face names (front back top bottom left right);
        ``highlight`` is a tuple of face names or a dict of face name to colour. ``cutaway`` removes the roof and nearest long wall to reveal the shared interior. Parts: ``body``, ``faces[name]``, ``window``, ``bolts``, ``dims[name]``, ``hatches``."""
        cap = _cap(t)
        cap["seg"] = cap["seg"] if segments is None else int(segments)
        parameters = _cap_parameters(cap)
        casing = _model.capsule_casing(parameters)
        usable = _model.capsule_interior(parameters)
        view = (55, 55) if cutaway and view == "axon" else view
        body_centre = ((casing.left + casing.right) / 2, (casing.back + casing.front) / 2, (casing.bottom + casing.top) / 2)
        v = make_view(view, at, scale, body_centre)
        colors = highlight if isinstance(highlight, dict) else {name: DATA for name in highlight}
        colors = {name: _blend(base, color, 0.55) for name, color in colors.items()}
        faces, rings = {}, {}
        drawing = NakaginDrawing3D(v)
        drawing.physical_casing, drawing.nominal_parameters = casing, parameters
        drawing.layout, drawing.aperture = layout, _model.capsule_window(parameters, layout)
        drawing.faces, drawing.dims, drawing.hatches = faces, {}, {}
        drawing.body = _engine.VGroup()
        drawing.window = _engine.VGroup()
        drawing.bolts = _engine.VGroup()
        semantic, rendered_faces = {}, {}
        exterior = {part.key for part, mesh in _cap_parts(cap, "exterior", interior, layout)}
        parts = []
        for part, mesh in _cap_parts(cap, "full", interior, layout):
            if not cutaway and glass_opacity >= 1 and part.key not in exterior:
                continue
            if cutaway and part.role in ("shell", "lining", "steel_truss"):
                sign = 1 if _basis(v)[1][0] >= 0 else -1
                thickness = float(part.solid.meta.get("wall_thickness", .1))
                cut_depth = .2 if part.role == "steel_truss" else max(.15, thickness * 1.5)
                bounds = usable if part.role == "lining" else casing
                edge = bounds.right if sign > 0 else -bounds.left
                mesh = _clip_mesh(mesh, (((0, 0, 1), bounds.top - cut_depth), ((sign, 0, 0), edge - cut_depth)))
            parts.append((part, mesh))
            if part.role in ("shell", "shell_back"):
                for face, normal in zip(mesh.faces, mesh.N):
                    polygon = mesh.V[face]
                    if normal @ _basis(v)[1] > 1e-6 and normal @ (polygon.mean(axis=0) - body_centre) > 1e-6:
                        rings.setdefault(_semantic_face(normal), []).append(_screen(v, polygon)[:, :2])
        items = _assembly_items(parts, base=base, groups=semantic, highlight=colors, window=window, bolts=bolts, face_groups=rendered_faces, glass_opacity=glass_opacity)
        if not window:
            cover = _window_outline(cap, layout=layout)
            normal = np.asarray(drawing.aperture.normal)
            closed = _engine.VGroup()
            semantic["window-cover"] = closed

            def cover_face(view,polygons=(cover,)):
                vertices=np.vstack(polygons)
                color = colors.get("front", base)
                face = _shape([_screen(view,polygon) for polygon in polygons], shade_of(color, tone_of(normal)))
                face.world_depth, face.semantic_role = _order(view,vertices.mean(axis=0)), "shell"
                face.world_vertices = vertices.copy()
                closed.add(face)
                rendered_faces.setdefault("front", _engine.VGroup()).add(face)
                return face

            items.append(Item(0, cover.mean(axis=0), cover, cover_face, normal))

        ordered = _section_order(items, v)
        drawing.add(*_paint_ordered(ordered, v))
        drawing.faces = rendered_faces
        faces = rendered_faces
        drawing.semantic_parts = semantic
        roles = {part.key: part.role for part, _ in _cap_parts(cap, "full", interior, layout)}
        unit_keys = {part.key for part, _ in _cap_parts(cap, "full", interior, layout) if part.solid.meta.get("window_unit")}
        for key, group in semantic.items():
            role = roles.get(key, "shell")
            if key in unit_keys:
                drawing.window.add(group)
            elif role in ("bolt_shaft", "bolt_head"):
                drawing.bolts.add(group)
            else:
                drawing.body.add(group)
        names = {"exposed": ("front", "top", "bottom", "left", "right"), "attached": ("back",)}
        for tag in hatch:
            group = _engine.VGroup()
            members = names.get(tag, (tag,))
            polys = [ring for name in members for ring in rings.get(name, [])]
            if polys:
                color = {"exposed": WARN, "attached": GOOD}.get(tag, DATA)
                group.add(_d.hatch(polys, color, spacing=0.1, angle=40, width=W_THIN, opacity=0.95))
            drawing.hatches[tag] = group
            drawing.add(group)
        centre2 = v(*body_centre)
        w, l, h = casing.width, casing.length, casing.height
        specs = {"width": ((casing.left, casing.front, casing.bottom), (casing.right, casing.front, casing.bottom), w), "length": ((casing.right, casing.back, casing.bottom), (casing.right, casing.front, casing.bottom), l), "height": ((casing.left, casing.front, casing.bottom), (casing.left, casing.front, casing.top), h)}
        for name in dims:
            if name in specs:
                a, b, value = specs[name]
                drawing.dims[name] = self._away(v(*a), v(*b), self.quantity(value, "m", 2), centre2, dim_offset)
                drawing.dims[name].measurement_value = value
                drawing.dims[name].world_endpoints = np.array([a, b])
            elif name == "window" and "front" in faces and window:
                circle = _model.capsule_window(parameters, layout)
                u0, v0, radius = circle.center[0], circle.center[2], circle.radius
                world_a, world_b = (u0 - radius, casing.front, v0), (u0 + radius, casing.front, v0)
                a, b = v(*world_a), v(*world_b)
                group = _engine.VGroup(_d.segments([(a, b)], DATA, W_THIN))
                direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
                normal = np.array([-direction[1], direction[0], 0.0])
                group.add(_d.segments([(p + normal * 0.07, p - normal * 0.07) for p in (a, b)], DATA, W_EDGE))
                label = self._chip("Ø " + self.quantity(cap["d"], "m", 2), v(u0, casing.front, (v0 + radius + casing.top) / 2))
                group.add(label)
                group.label = label
                group.measurement_value, group.world_endpoints = 2 * radius, np.array([world_a, world_b])
                drawing.dims[name] = group
            if name in drawing.dims:
                drawing.dims[name].measurement_unit = "m"
                drawing.add(drawing.dims[name])
        drawing.parts.update({**semantic, "body": drawing.body, "window": drawing.window, "bolts": drawing.bolts})
        for target, category in ((drawing.body, "body"), (drawing.window, "window"), (drawing.bolts, "bolt")):
            roots = {id(group.material_underpaint): group.material_underpaint for key, group in semantic.items() if hasattr(group, "material_underpaint") and ("window" if key in unit_keys else ("bolt" if roles.get(key) in ("bolt_shaft", "bolt_head") else "body")) == category}
            target.add(*roots.values())
        return drawing

    def capsule_views(self, t, *, at=(0, 0, 0), scale: float = 0.58, dims: bool = True, base: str = CAPSULE, gap: float = 1.0, interior: str = "reference", layout: str = "Slash", glass_opacity: float = .25):
        """📃 Plan and orthographic elevations measure the physical casing and finished interior; nominal program parameters remain separate."""
        cap = _cap(t)
        parameters = _cap_parameters(cap)
        casing, usable = _model.capsule_casing(parameters), _model.capsule_interior(parameters)
        circle = _model.capsule_window(parameters, layout)
        w, l, h, d, u, vv = casing.width, casing.length, casing.height, 2 * circle.radius, circle.center[0], circle.center[2]
        my, mz = (casing.back + casing.front) / 2, (casing.bottom + casing.top) / 2
        sheet_w, sheet_h = (w + l) * scale + gap, (l + h) * scale + gap
        fx, fy = at[0] - sheet_w / 2 + w * scale / 2, at[1] - sheet_h / 2
        px, py = fx, fy + h * scale + gap
        sx, sy = fx + w * scale / 2 + gap, fy
        front_pt = lambda x, z: P((fx - x * scale, fy + (z - casing.bottom) * scale))
        side_pt = lambda y, z: P((sx + (y - casing.back) * scale, sy + (z - casing.bottom) * scale))
        plan_pt = lambda x, y: P((px - x * scale, py + (casing.front - y) * scale))
        drawing = NakaginDrawing3D()
        drawing.physical_casing, drawing.nominal_parameters = casing, parameters
        drawing.layout, drawing.aperture = layout, circle
        front = self.capsule_axon(t, at=front_pt(0, mz), scale=scale, view="north", base=base, interior=interior, layout=layout, bolts=True, glass_opacity=glass_opacity)
        side = self.capsule_axon(t, at=side_pt(my, mz), scale=scale, view="east", base=base, interior=interior, layout=layout, bolts=True, glass_opacity=glass_opacity)
        plan = _engine.VGroup()
        quarters = _model._capsule_quarters(parameters, int(_model._detail_value("segments")))
        boundary = _model._profile(w, l, _model._param(parameters, "corner_radius"), quarters)
        box = np.array([plan_pt(x, casing.back + y) for x, y in boundary])
        inner = np.array([plan_pt(usable.left, usable.back), plan_pt(usable.right, usable.back), plan_pt(usable.right, usable.front), plan_pt(usable.left, usable.front)])
        plan.add(_shape([box], shade_of(base, 0.9), 1.0, INK, W_CUT), _shape([inner], BACKGROUND, 0.7, INK, W_THIN),
                 _d.hatch([box[:, :2], inner[:, :2]], MUTED, spacing=0.05, angle=45, width=W_HAIR, opacity=0.9))
        opening = ((usable.right, circle.center[1] - d / 2), (usable.right, circle.center[1] + d / 2), (casing.right, circle.center[1] + d / 2), (casing.right, circle.center[1] - d / 2)) if circle.normal[0] else ((u - d / 2, usable.front), (u + d / 2, usable.front), (u + d / 2, casing.front), (u - d / 2, casing.front))
        plan.window = _shape([np.array([plan_pt(x, y) for x, y in opening])], BACKGROUND, 1.0, DATA, W_THIN)
        plan.add(plan.window)
        plan.semantic_parts = {}
        excluded = {"shell", "shell_back", "lining", "steel_truss", "window_rim", "window_frame", "glazing", "bolt_shaft", "bolt_head", "service_rail", "service_pipe"}
        for part, mesh in sorted(_cap_parts(cap, "full", interior, layout), key=lambda pair: pair[1].V[:, 2].mean()):
            if part.role in excluded:
                continue
            group = _engine.VGroup()
            for index, normal in enumerate(mesh.N):
                if normal[2] > 0.5:
                    polygon = np.array([plan_pt(x, y) for x, y, _ in mesh.V[mesh.faces[index]]])
                    group.add(_shape([polygon], _material(part.material, base), 1.0, MUTED, W_HAIR))
            plan.semantic_parts[part.key] = group
            plan.add(group)
        for name, group in (("plan", plan), ("front", front), ("side", side)):
            group.dims = {}
            drawing.parts[name] = group
            drawing.add(group)
        if dims:
            q = lambda value: self.quantity(value, "m", 2)
            def dimension(a, b, value, centre, endpoints):
                result = self._away(a, b, q(value), centre, .3)
                result.measurement_value, result.measurement_unit = value, "m"
                result.world_endpoints = np.asarray(endpoints)
                return result
            pc, fc, sc = plan_pt(0, my), front_pt(0, mz), side_pt(my, mz)
            width_points = ((casing.right, casing.back, casing.bottom), (casing.left, casing.back, casing.bottom))
            length_points = ((casing.right, casing.back, casing.bottom), (casing.right, casing.front, casing.bottom))
            height_points = ((casing.left, casing.front, casing.bottom), (casing.left, casing.front, casing.top))
            plan.dims["width"] = dimension(plan_pt(casing.right, casing.back), plan_pt(casing.left, casing.back), w, pc, width_points)
            plan.dims["length"] = dimension(plan_pt(casing.right, casing.back), plan_pt(casing.right, casing.front), l, pc, length_points)
            front.dims["width"] = dimension(front_pt(casing.right, casing.bottom), front_pt(casing.left, casing.bottom), w, fc, width_points)
            front.dims["height"] = dimension(front_pt(casing.left, casing.bottom), front_pt(casing.left, casing.top), h, fc, height_points)
            if circle.normal[0]:
                y = circle.center[1]
                side.dims["window"] = _engine.VGroup(_d.segments([(side_pt(y + d / 2, vv), side_pt(y - d / 2, vv))], DATA, W_THIN), self._chip("Ø " + q(d), side_pt(y, (vv - d / 2) / 2)))
            else:
                front.dims["window"] = _engine.VGroup(_d.segments([(front_pt(u + d / 2, vv), front_pt(u - d / 2, vv))], DATA, W_THIN), self._chip("Ø " + q(d), front_pt(u, (vv - d / 2) / 2)))
                if abs(u) > 1e-6:
                    front.dims["offset"] = _engine.VGroup(_d.segments([(front_pt(0, vv - d / 2 - 0.15), front_pt(0, vv + d / 2 + 0.15))], MUTED, W_HAIR),
                                                          _d.segments([(front_pt(0, vv + d / 2 + 0.1), front_pt(u, vv + d / 2 + 0.1))], DATA, W_THIN),
                                                          self._text(q(abs(u)), front_pt(u / 2, vv + d / 2 + 0.1) + np.array([0, 0.16, 0])))
            side.dims["length"] = dimension(side_pt(casing.back, casing.bottom), side_pt(casing.front, casing.bottom), l, sc, length_points)
            side.dims["height"] = dimension(side_pt(casing.front, casing.bottom), side_pt(casing.front, casing.top), h, sc, height_points)
            for group in (plan, front, side):
                group.add(*group.dims.values())
            side_offset = np.array([max(0.0, front.get_right()[0] + gap - side.get_left()[0]), 0.0, 0.0])
            side.shift(side_offset)
            side.view = make_view(side.view, side.view.point + side_offset, side.view.scale, side.view.centre)
            row_top = max(front.get_top()[1], side.get_top()[1])
            plan.shift(np.array([0.0, max(0.0, row_top + gap - plan.get_bottom()[1]), 0.0]))
        return drawing

    def plane_axes(self, plane, *, at=(0, 0, 0), scale: float = 0.62, view="axon", length: float = 2.0, labels: bool = True, square: bool = True):
        """🧭 A Rhino plane: red X, green Y, blue Z arrows from the origin dot with an optional faint XY square; ``at`` is the screen position of the plane origin.

        Parts: ``axes["x"|"y"|"z"]`` (line, head, label), ``origin``, ``square``."""
        v = make_view(view, at, scale, plane.origin)
        origin = np.asarray(plane.origin, float)
        drawing = NakaginDrawing3D(v)
        drawing.axes = {}
        if square:
            corners = [origin + length * (a * np.asarray(plane.x) + b * np.asarray(plane.y)) for a, b in ((0, 0), (1, 0), (1, 1), (0, 1))]
            drawing.square = _shape([_screen(v, np.array(corners))], AXIS_Z, 0.12, AXIS_Z, W_HAIR, 0.5)
            drawing.add(drawing.square)
        for key, axis, color in (("x", plane.x, AXIS_X), ("y", plane.y, AXIS_Y), ("z", plane.z, AXIS_Z)):
            tip = v(*(origin + length * np.asarray(axis)))
            start = v(*origin)
            delta = tip - start
            norm = max(float(np.linalg.norm(delta)), 1e-9)
            direction = delta / norm
            normal = np.array([-direction[1], direction[0], 0.0])
            head = min(0.2, norm * 0.4)
            arrow = _engine.VGroup(_d.segments([(start, tip - direction * head * 0.6)], color, 2.6),
                                   _d.polygon([tip, tip - direction * head + normal * head * 0.38, tip - direction * head - normal * head * 0.38], color, 1.0, color, 0.5))
            if labels:
                arrow.add(self._text(key.upper(), tip + direction * 0.2, size=_d.S_LABEL, color=color))
            drawing.axes[key] = arrow
            drawing.add(arrow)
        drawing.origin = _engine.Dot(P(v(*origin)), radius=0.06, color=INK)
        drawing.add(drawing.origin)
        drawing.parts.update({"origin": drawing.origin, **{k: m for k, m in drawing.axes.items()}})
        return drawing

    def _compose(self, items, view, at, scale, drawing=None):
        verts = np.vstack([item.verts for item in items]) if items else np.zeros((1, 3))
        centre = (verts.min(axis=0) + verts.max(axis=0)) / 2
        v = make_view(view, at, scale, centre)
        drawing = NakaginDrawing3D() if drawing is None else drawing
        drawing.view = v
        drawing.add(*_paint_ordered(_section_order(items, v), v))
        return drawing

    def _core_items(self, tower, *, storeys, side, pitch, z0, tag, drawing=None, color=None, lod=False, roof=True, view=None, coverage_padding=0.0):
        height = tower.height * pitch / tower.storey_height if int(storeys) == tower.storeys else storeys * pitch
        t = replace(tower, storeys=int(storeys), core_side=side, storey_height=pitch, height=height)
        parts = [(part, mesh) for part, mesh in _core_parts(t.index, t.storeys, t.core_side, t.storey_height, t.height, _lod.core_assembly()) if mesh.V[:, 2].max() >= z0]
        if not roof:
            parts = [(part, mesh) for part, mesh in parts if part.role not in ("roof", "vent", "roof_vent_support")]
        groups = {}
        items = _assembly_items(parts, base=color or CONCRETE, groups=groups, lod=lod, glass_opacity=1 if lod else .25)
        if view is not None:
            items = _opaque_visible(items, view, coverage_padding)
        if drawing is not None:
            drawing(tag, _part_group(groups))
        return items

    def _capital_items(self, tower, *, drawing=None, tag=None, z0=None):
        groups = {}
        parts = _capital_parts(tower.index, tower.height, tower.capital_height, tower.capital_overhang, tower.core_side)
        if z0 is not None:
            delta = np.array([0, 0, z0 - _model.capital_base(tower)])
            parts = [(part, Mesh(mesh.V + delta, mesh.N, mesh.faces, mesh.names)) for part, mesh in parts]
        items = _assembly_items(parts, base=CAPITAL, groups=groups)
        if drawing is not None:
            drawing(tag, _part_group(groups))
        return items

    def _podium_items(self, *, drawing=None):
        groups = {}
        items = _assembly_items(_podium_parts(), base=CONCRETE, groups=groups)
        if drawing is not None:
            drawing("podium", _part_group(groups))
        return items

    def core_axon(self, tower, *, at=(0, 0, 0), scale: float = 0.2, view="axon", storeys: int | None = None, side: float | None = None, storey_height: float | None = None):
        """🧱 The concrete core with storey lines and the ground door; ``storeys``, ``side`` and ``storey_height`` override the model for live parametric cores. ``at`` is the screen position of the core centre. Parts: ``core``."""
        t = _tower(tower)
        drawing = NakaginDrawing3D()
        items = self._core_items(t, storeys=t.storeys if storeys is None else storeys, side=t.core_side if side is None else side,
                               pitch=t.storey_height if storey_height is None else storey_height, z0=0.0, tag="core", drawing=lambda tag, group: drawing.parts.update(group.semantic_parts))
        self._compose(items, view, at, scale, drawing)
        drawing.semantic_parts = dict(drawing.parts)
        drawing.parts["core"] = drawing
        return drawing

    def capital_axon(self, tower, *, at=(0, 0, 0), scale: float = 0.2, view="axon"):
        """👑 The diagonal machinery roof screen of a tower; ``at`` is the screen position of its centre. Parts: ``capital``."""
        drawing = NakaginDrawing3D()
        self._compose(self._capital_items(_tower(tower), drawing=lambda tag, group: drawing.parts.update(group.semantic_parts)), view, at, scale, drawing)
        drawing.semantic_parts = dict(drawing.parts)
        drawing.parts["capital"] = drawing
        return drawing

    def podium_axon(self, *, at=(0, 0, 0), scale: float = 0.1, view="axon"):
        """🏬 Detailed podium with opaque fascia, recessed strip glazing, pilotis and explicitly inferred interior rooms."""
        drawing = NakaginDrawing3D()
        self._compose(self._podium_items(drawing=lambda tag, group: drawing.parts.update(group.semantic_parts)), view, at, scale, drawing)
        drawing.semantic_parts = dict(drawing.parts)
        drawing.parts["podium"] = drawing
        return drawing

    def _tower_items(self, *, lit, tint, types, capital, podium, where, core, ground, segments, capsules, cores, capitals, base, towers, storeys, view=None, coverage_padding=0.0):
        items = []
        tower_list = _model.towers()
        pitch = _model.value("core.storey_height").value
        z0 = 0.0
        if podium:
            items.extend(self._podium_items(drawing=lambda tag, group: capitals.setdefault("podium", group)))
        limits, heights = {}, {}
        for t in (tower_list[i] for i in towers):
            count = t.storeys if storeys is None else max(0, min(t.storeys, storeys))
            levels = max(0, count - t.first_capsule_storey)
            limits[t.index] = levels
            partial = count != t.storeys
            crown = max((slot.plane.origin[2] + _model.capsule_type(slot.type).height for slot in _model.slots() if slot.tower == t.index and slot.storey < levels), default=t.base_elevation)
            height = (crown + t.roof_clearance if levels else _model.value(f"podium.levels.{count}").value) if partial else t.height
            heights[t.index] = height
            shaped = replace(t, storeys=count, height=height)
            if core and count:
                items.extend(self._core_items(shaped, storeys=count, side=t.core_side, pitch=t.storey_height, z0=z0, tag=t.index, lod=True,
                                              drawing=lambda tag, group: cores.setdefault(tag, group), roof=bool(levels), view=view, coverage_padding=coverage_padding))
            if capital and levels:
                items.extend(self._capital_items(shaped, tag=t.index, z0=height if partial else None, drawing=lambda tag, group: capitals.setdefault(tag, group)))
        bridge_groups = {}
        if core and len(towers) == 2 and min(heights.values()) > 0:
            bridge_parts = [(part, mesh) for part, mesh in _bridge_parts() if (storeys is None or mesh.V[:, 2].max() <= min(heights.values()))]
            items.extend(_assembly_items(bridge_parts, base=CONCRETE, groups=bridge_groups))
            capitals["bridges"] = _engine.VGroup(*bridge_groups.values())
        types_by_key = {c.key: _cap(c) for c in _model.capsule_types()}
        for slot in _model.slots():
            key = (slot.building, slot.tower, slot.storey, slot.facade, slot.item)
            if slot.tower not in towers or (where is not None and not where(slot)):
                continue
            cap = dict(types_by_key[slot.type], seg=segments)
            if storeys is not None and slot.storey >= limits[slot.tower]:
                continue
            colour, on = _slot_look(slot, lit=lit, tint=tint, types=types, base=base)
            groups = {}
            parts = _slot_parts(slot, segments)
            local_items = _assembly_items(parts, base=colour, groups=groups, plane=slot.plane, lod=True, outline=DATA if lit is not None and on else None, glass_opacity=1)
            items.extend(_opaque_visible(local_items, view, coverage_padding) if view is not None else local_items)
            capsules[key] = _part_group(groups)
            capsules[key].semantic_parts = groups
        if ground and items:
            verts = np.vstack([item.verts for item in items])
            lo, hi = verts.min(axis=0) - 3.0, verts.max(axis=0) + 3.0

            def draw_ground(v):
                S = _screen(v, np.array([(lo[0], lo[1], 0), (hi[0], lo[1], 0), (hi[0], hi[1], 0), (lo[0], hi[1], 0)]))
                return _shape([S], _d.GROUND_PLANE, 1.0, _d.GREEN_GROUND, W_HAIR, 0.6)

            items.append(Item(-1, np.array([(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, 0.0]), np.array([(lo[0], lo[1], 0), (hi[0], hi[1], 0)]), draw_ground))
        return items

    def _tower_template(self, *, scale, view, lit, capital, podium, core, ground, where, towers, storeys, segments):
        """🗼 The cached whole-building drawing for one geometry in neutral paint; colour, tint and lighting are applied to clones, so the key holds geometry only."""
        if scale <= 0:
            raise ValueError("Drawing scale must be positive")
        included = tuple((slot.building, slot.tower, slot.storey, slot.facade, slot.item) for slot in _model.slots() if where is None or where(slot))
        viewpoint = (view.kind, view.azimuth, view.elevation, view.face) if isinstance(view, _d.View) else (tuple(view) if isinstance(view, (tuple, list)) else view)
        segments = _lod.segments(segments)
        key = (viewpoint, lit is None, included, capital, podium, core, ground, tuple(towers), storeys, segments, _lod.core_assembly())
        coverage_padding = _raster_footprint(scale, 1.2 if lit is not None else .45)
        template = _TOWERS.pop(key, None)
        if template is None or template.coverage_padding < coverage_padding:
            template = self._tower_drawing(at=(0, 0, 0), scale=1, view=view, lit=None if lit is None else (lambda path: True), tint=None, types=False, capital=capital, podium=podium,
                                           core=core, ground=ground, where=where, towers=towers, storeys=storeys, segments=segments, base=CAPSULE, coverage_padding=coverage_padding)
            template.outlined = lit is not None
            facets = sum(isinstance(member, _Facet) for member in template.get_family())
            if facets >= _FROZEN_SHAPES:
                template, facets = _Frozen(template), 0
            else:
                _share_style(template)
            _TOWER_FACETS[key] = facets
        _TOWERS[key] = template
        entries, budget = _lod.template_cache()
        while len(_TOWERS) > 1 and (len(_TOWERS) > entries or sum(_TOWER_FACETS[held] for held in _TOWERS) > budget):
            _TOWER_FACETS.pop(next(iter(_TOWERS)), None)
            _TOWERS.pop(next(iter(_TOWERS)))
        return template

    def _tint_capsules(self, capsules, template, *, lit, tint, types, base):
        """🖋️ Recolour the capsule groups of a template clone in place for the requested tint, types, base and lit selection."""
        for slot in _model.slots():
            path = (slot.building, slot.tower, slot.storey, slot.facade, slot.item)
            if path not in capsules:
                continue
            colour, on = _slot_look(slot, lit=lit, tint=tint, types=types, base=base)
            outlined = lit is not None and bool(on)
            if colour != CAPSULE or outlined != template.outlined:
                _recolour(capsules[path], colour, outlined)

    def tower_axon(self, *, at=(0, 0, 0), scale: float = 0.1, view="axon", lit=None, tint=None, types: bool = False, capital: bool = True, podium: bool = True,
                   core: bool = True, ground: bool = False, where=None, towers=(0, 1), storeys: int | None = None, segments: int = 2, base: str = CAPSULE):
        """🛸 Clone the cached, globally sorted building drawing for live position and scale, recolouring its capsules in place for tint, types and lit selections."""
        template = self._tower_template(scale=scale, view=view, lit=lit, capital=capital, podium=podium, core=core, ground=ground, where=where, towers=towers, storeys=storeys, segments=segments)
        offset = P(at)
        if isinstance(template, _Frozen):
            drawing = template.thaw(scale, offset)
        else:
            drawing = template.copy()
            drawing.apply_points_function_about_point(lambda points: points * scale + offset, about_point=np.zeros(3))
        drawing.view = make_view(view, at, scale, template.view.centre)
        self._tint_capsules(drawing.capsules, template, lit=lit, tint=tint, types=types, base=base)
        return drawing

    def tower_capsules(self, keys, color, *, at=(0, 0, 0), scale: float = 0.1, view="axon", capital: bool = True, podium: bool = True, core: bool = True,
                       ground: bool = False, where=None, towers=(0, 1), storeys: int | None = None, segments: int = 2):
        """🪆 Copies of single capsules of the tower in one flat ``color`` (the neutral paint when ``None``) at their place on the screen, taken from the cached drawing without cloning the building."""
        template = self._tower_template(scale=scale, view=view, lit=None, capital=capital, podium=podium, core=core, ground=ground, where=where, towers=towers, storeys=storeys, segments=segments)
        offset = P(at)
        copies = []
        for key in keys:
            capsule = template.capsule(key) if isinstance(template, _Frozen) else template.capsules[key].copy()
            capsule.apply_points_function_about_point(lambda points: points * scale + offset, about_point=np.zeros(3))
            if color is not None:
                _recolour(capsule, color, False)
            copies.append(capsule)
        return copies

    def _tower_drawing(self, *, at=(0, 0, 0), scale: float = 0.1, view="axon", lit=None, tint=None, types: bool = False, capital: bool = True, podium: bool = True,
                   core: bool = True, ground: bool = False, where=None, towers=(0, 1), storeys: int | None = None, segments: int = 2, base: str = CAPSULE, coverage_padding: float = 0.0):
        """🌆 The whole building in axonometric: podium, both cores, 140 capsules, both capitals, depth-sorted by the painter's algorithm.

        ``lit`` (None, a collection of paths or a predicate) lights those capsules and dims the others; a path is a prefix of ``(b, t, s, f, i)`` so ``(0, 1, 4, 2)`` lights a facade branch.
        ``tint`` maps paths (or a predicate) to colours; ``types`` colours capsules by type; ``where`` filters slots, ``towers`` picks tower indices, ``storeys`` caps the core height. ``at`` is the screen position of the bounding-box centre.
        Parts: ``cores[tower]``, ``capsules[(b, t, s, f, i)]``, ``capitals[tower]`` and ``podium``."""
        drawing = NakaginDrawing3D()
        drawing.coverage_padding = coverage_padding
        drawing.capsules, drawing.cores, drawing.capitals = {}, {}, {}
        items = self._tower_items(lit=lit, tint=tint, types=types, capital=capital, podium=podium, where=where, core=core, ground=ground, segments=segments,
                                  capsules=drawing.capsules, cores=drawing.cores, capitals=drawing.capitals, base=base, towers=towers, storeys=storeys, view=make_view(view, (0, 0, 0), 1, (0, 0, 0)), coverage_padding=coverage_padding)
        self._compose(items, view, at, scale, drawing)
        drawing.podium = drawing.capitals.pop("podium", _engine.VGroup())
        drawing.bridges = drawing.capitals.pop("bridges", _engine.VGroup())
        drawing.parts.update({"cores": drawing.cores, "capsules": drawing.capsules, "capitals": drawing.capitals, "podium": drawing.podium, "bridges": drawing.bridges})
        return drawing

    def tower_elevation(self, *, at=(0, 0, 0), scale: float = 0.1, face: str = "north", lit=None, tint=None, types: bool = False, capital: bool = True,
                        podium: bool = True, where=None, towers=(0, 1), storeys: int | None = None, segments: int = 2, base: str = CAPSULE):
        """🏙 The building as an orthographic elevation seen from ``face`` (north: capsule windows face the viewer), same parts and options as ``tower_axon``."""
        return self.tower_axon(at=at, scale=scale, view=face, lit=lit, tint=tint, types=types, capital=capital, podium=podium, where=where, towers=towers, storeys=storeys, segments=segments, base=base)


def _tower(tower):
    if isinstance(tower, _model.Tower):
        return tower
    towers = _model.towers()
    return towers[tower] if isinstance(tower, int) else next(t for t in towers if t.key == tower)


def _match(spec, key, boolean: bool = False):
    if spec is None:
        return None if not boolean else True
    if callable(spec):
        return spec(key)
    if isinstance(spec, dict):
        best = max((len(p) for p in spec if tuple(key[:len(p)]) == tuple(p)), default=None)
        return None if best is None else spec[next(p for p in spec if len(p) == best and tuple(key[:len(p)]) == tuple(p))]
    return any(tuple(key[:len(p)]) == tuple(p) for p in spec)
