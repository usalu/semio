"""🦗 Schema-first Grasshopper 1 semantics: data trees, casting, components and the Nakagin canvases (stdlib only)."""

from __future__ import annotations

import ast
import importlib.util
import json
import math
import re
import sys
from dataclasses import dataclass, field, replace
from functools import lru_cache
from pathlib import Path as File

LANGUAGES = ("en", "de")
NAKAGIN = File(__file__).resolve().parents[3] / "🏢️nakagin" / "🐍️.py"
Path = tuple
IDENTITY = ((1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), (0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0))
NUMBER = r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?"


def format_path(path: Path) -> str:
    """🧭 Write a branch path the way Grasshopper shows it, e.g. {0;1;2}."""
    return "{" + ";".join(str(i) for i in path) + "}"


def parse_path(text: str) -> Path:
    """🧭 Read a path such as {0;1;2} back into a tuple of indices."""
    match = re.fullmatch(r"\s*\{\s*(\d+(?:\s*;\s*\d+)*)?\s*\}\s*", text)
    if not match:
        raise ValueError(f"Not a path: {text}")
    return tuple(int(i) for i in (match.group(1) or "").split(";")) if match.group(1) else ()


def format_number(value: float, language: str = "en") -> str:
    """🔢 Format a number like a Panel does: shortest form, locale decimal mark."""
    if isinstance(value, float) and value == int(value) and abs(value) < 1e15:
        text = str(int(value))
    else:
        text = f"{value:.10g}"
    return text.replace(".", ",") if language == "de" else text


def _sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def _dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def _cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def _len(a):
    return math.sqrt(_dot(a, a))


def _unit(a):
    n = _len(a)
    if n == 0:
        raise ValueError("Zero-length vector")
    return (a[0] / n, a[1] / n, a[2] / n)


@dataclass(frozen=True)
class Point:
    """📍 A point in world coordinates."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    @property
    def t(self):
        return (self.x, self.y, self.z)


@dataclass(frozen=True)
class Vector:
    """➡️ A direction with magnitude."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    @property
    def t(self):
        return (self.x, self.y, self.z)

    @property
    def length(self):
        return _len(self.t)


@dataclass(frozen=True)
class Plane:
    """🧭 An origin with orthonormal X, Y and Z axes."""

    origin: Point
    x: Vector
    y: Vector
    z: Vector

    @staticmethod
    def from_axes(origin: Point, x: Vector, y: Vector) -> "Plane":
        ux = _unit(x.t)
        yo = _sub(y.t, tuple(_dot(y.t, ux) * c for c in ux))
        uy = _unit(yo)
        return Plane(origin, Vector(*ux), Vector(*uy), Vector(*_cross(ux, uy)))

    @staticmethod
    def world_xy(origin: Point = Point()) -> "Plane":
        return Plane.from_axes(origin, Vector(1, 0, 0), Vector(0, 1, 0))

    @staticmethod
    def world_xz(origin: Point = Point()) -> "Plane":
        return Plane.from_axes(origin, Vector(1, 0, 0), Vector(0, 0, 1))

    @staticmethod
    def world_yz(origin: Point = Point()) -> "Plane":
        return Plane.from_axes(origin, Vector(0, 1, 0), Vector(0, 0, 1))


@dataclass(frozen=True)
class Domain:
    """↔️ A numeric interval (Grasshopper Domain)."""

    lo: float
    hi: float

    @property
    def length(self):
        return self.hi - self.lo

    @property
    def mid(self):
        return (self.lo + self.hi) / 2


@dataclass(frozen=True)
class Circle:
    """⭕ A circle on a plane."""

    plane: Plane
    radius: float


@dataclass(frozen=True)
class Line:
    """📏 A straight segment."""

    a: Point
    b: Point


@dataclass(frozen=True)
class Rectangle:
    """▭ A planar rectangle spanned by two domains."""

    plane: Plane
    x: Domain
    y: Domain


@dataclass(frozen=True)
class Curve:
    """〰️ A curve wrapping a circle, line or rectangle."""

    shape: object


@dataclass(frozen=True)
class Box:
    """📦 A box on a base plane with three domains."""

    plane: Plane
    x: Domain
    y: Domain
    z: Domain

    @property
    def volume(self):
        return self.x.length * self.y.length * self.z.length

    @property
    def center(self):
        o, p = self.plane.origin.t, self.plane
        return Point(*(o[i] + self.x.mid * p.x.t[i] + self.y.mid * p.y.t[i] + self.z.mid * p.z.t[i] for i in range(3)))


def _mul(a, b):
    return tuple(tuple(sum(a[i][k] * b[k][j] for k in range(4)) for j in range(4)) for i in range(4))


def xform_point(m, p):
    return tuple(m[i][0] * p[0] + m[i][1] * p[1] + m[i][2] * p[2] + m[i][3] for i in range(3))


def xform_vector(m, v):
    return tuple(m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2] for i in range(3))


def translation(v) -> tuple:
    """➕ Translation matrix of a vector."""
    t = v.t if hasattr(v, "t") else v
    return ((1.0, 0.0, 0.0, t[0]), (0.0, 1.0, 0.0, t[1]), (0.0, 0.0, 1.0, t[2]), (0.0, 0.0, 0.0, 1.0))


def rotation(angle: float, axis: Vector, center: Point) -> tuple:
    """🔄 Rotation matrix about an axis through a center (Rodrigues)."""
    k, c, s = _unit(axis.t), math.cos(angle), math.sin(angle)
    kx = ((0, -k[2], k[1]), (k[2], 0, -k[0]), (-k[1], k[0], 0))
    r = [[c * (i == j) + s * kx[i][j] + (1 - c) * k[i] * k[j] for j in range(3)] for i in range(3)]
    t = [center.t[i] - sum(r[i][j] * center.t[j] for j in range(3)) for i in range(3)]
    return tuple(tuple(r[i]) + (t[i],) for i in range(3)) + ((0.0, 0.0, 0.0, 1.0),)


def plane_to_plane(a: Plane, b: Plane) -> tuple:
    """🧲 Matrix that maps plane a onto plane b (the Orient component)."""
    r = [[sum(q.t[i] * p.t[j] for p, q in ((a.x, b.x), (a.y, b.y), (a.z, b.z))) for j in range(3)] for i in range(3)]
    t = [b.origin.t[i] - sum(r[i][j] * a.origin.t[j] for j in range(3)) for i in range(3)]
    return tuple(tuple(r[i]) + (t[i],) for i in range(3)) + ((0.0, 0.0, 0.0, 1.0),)


def _rot_inverse(m):
    return tuple(tuple(m[j][i] for j in range(3)) + (0.0,) for i in range(3)) + ((0.0, 0.0, 0.0, 1.0),)


def invert(m) -> tuple:
    """↩️ Inverse of a rigid transform."""
    r = _rot_inverse(m)
    t = xform_vector(r, (m[0][3], m[1][3], m[2][3]))
    return tuple(tuple(r[i][:3]) + (-t[i],) for i in range(3)) + ((0.0, 0.0, 0.0, 1.0),)


@dataclass(frozen=True)
class Brep:
    """🧱 A solid recorded as a construction tree (box, fillet, cylinder, extrusion, difference) with a placement matrix."""

    solid: dict
    xform: tuple = IDENTITY
    tag: dict = field(default_factory=dict)

    def volume(self) -> float:
        return solid_volume(self.solid)

    def root_box(self) -> Box | None:
        s = self.solid
        while s["kind"] in ("fillet", "difference"):
            s = s["base"]
        return apply(self.xform, s["box"]) if s["kind"] == "box" else None

    def cuts(self) -> tuple:
        return tuple(self.solid["cuts"]) if self.solid["kind"] == "difference" else ()

    def window(self) -> Circle | None:
        for cut in self.cuts():
            if cut["kind"] == "cylinder":
                return apply(self.xform, Circle(cut["plane"], cut["radius"]))
        return None

    def fillet_radius(self) -> float:
        s = self.solid
        while s["kind"] == "difference":
            s = s["base"]
        return s["radius"] if s["kind"] == "fillet" else 0.0


def shape_area(shape) -> float:
    if isinstance(shape, Circle):
        return math.pi * shape.radius ** 2
    if isinstance(shape, Rectangle):
        return shape.x.length * shape.y.length
    return 0.0


def _shape_normal(shape):
    return shape.plane.z.t if hasattr(shape, "plane") else None


def solid_volume(s: dict) -> float:
    k = s["kind"]
    if k == "box":
        return s["box"].volume
    if k == "cylinder":
        return math.pi * s["radius"] ** 2 * s["length"]
    if k == "extrusion":
        n = _shape_normal(s["curve"].shape)
        return shape_area(s["curve"].shape) * abs(_dot(n, s["direction"].t)) if n else 0.0
    if k == "fillet":
        b, r = s["base"], s["radius"]
        box = b["box"]
        a = [d.length - 2 * r for d in (box.x, box.y, box.z)]
        if min(a) < 0:
            raise ValueError("Fillet radius exceeds half the smallest box size")
        return a[0] * a[1] * a[2] + 2 * r * (a[0] * a[1] + a[0] * a[2] + a[1] * a[2]) + math.pi * r * r * sum(a) + 4 / 3 * math.pi * r ** 3
    if k == "difference":
        return solid_volume(s["base"]) - sum(solid_volume(c) for c in s["cuts"])
    raise ValueError(f"Unknown solid {k}")


def apply(m, g):
    """🪄 Apply a rigid transform to any Grasshopper geometry value."""
    if isinstance(g, Point):
        return Point(*xform_point(m, g.t))
    if isinstance(g, Vector):
        return Vector(*xform_vector(m, g.t))
    if isinstance(g, Plane):
        return Plane(apply(m, g.origin), apply(m, g.x), apply(m, g.y), apply(m, g.z))
    if isinstance(g, Line):
        return Line(apply(m, g.a), apply(m, g.b))
    if isinstance(g, Circle):
        return Circle(apply(m, g.plane), g.radius)
    if isinstance(g, Rectangle):
        return Rectangle(apply(m, g.plane), g.x, g.y)
    if isinstance(g, Curve):
        return Curve(apply(m, g.shape))
    if isinstance(g, Box):
        return Box(apply(m, g.plane), g.x, g.y, g.z)
    if isinstance(g, Brep):
        return replace(g, xform=_mul(m, g.xform))
    raise TypeError(f"Cannot transform {type_name(g)}")


def _bake(s: dict, m) -> dict:
    k = s["kind"]
    if k == "box":
        return {"kind": k, "box": apply(m, s["box"])}
    if k == "cylinder":
        return {**s, "plane": apply(m, s["plane"])}
    if k == "extrusion":
        return {**s, "curve": apply(m, s["curve"]), "direction": apply(m, s["direction"])}
    if k == "fillet":
        return {**s, "base": _bake(s["base"], m)}
    return {**s, "base": _bake(s["base"], m), "cuts": tuple(_bake(c, m) for c in s["cuts"])}


GEOMETRY = ("Point", "Circle", "Line", "Curve", "Box", "Brep")


def type_name(value) -> str:
    """🏷️ Grasshopper type name of a value."""
    if value is None:
        return "Null"
    if isinstance(value, bool):
        return "Boolean"
    if isinstance(value, int):
        return "Integer"
    if isinstance(value, float):
        return "Number"
    if isinstance(value, str):
        return "Text"
    if isinstance(value, tuple) and len(value) == 4 and all(isinstance(r, tuple) for r in value):
        return "Transform"
    if isinstance(value, Rectangle):
        return "Curve"
    return type(value).__name__


def describe(value) -> str:
    """🖥️ Text a Panel shows for a value."""
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "True" if value else "False"
    if isinstance(value, (int, float)):
        return format_number(value)
    if isinstance(value, str):
        return value
    if isinstance(value, (Point, Vector)):
        return "{" + ", ".join(format_number(c) for c in value.t) + "}"
    if isinstance(value, Domain):
        return f"{format_number(value.lo)} To {format_number(value.hi)}"
    if isinstance(value, Plane):
        return f"Plane (O={describe(value.origin)}, X={describe(value.x)}, Y={describe(value.y)}, Z={describe(value.z)})"
    if isinstance(value, Circle):
        return f"Circle (R:{format_number(value.radius)})"
    if isinstance(value, Line):
        return f"Line ({describe(value.a)} to {describe(value.b)})"
    return type_name(value)


class Tree:
    """🌳 A Grasshopper data tree: ordered branches addressed by paths."""

    __slots__ = ("_b",)

    def __init__(self, branches=None):
        self._b = {tuple(p): tuple(v) for p, v in (branches or {}).items()}

    @staticmethod
    def from_list(items, path: Path = (0,)) -> "Tree":
        return Tree({path: tuple(items)})

    @staticmethod
    def from_item(item, path: Path = (0,)) -> "Tree":
        return Tree({path: (item,)})

    @staticmethod
    def from_dict(data: dict) -> "Tree":
        return Tree({parse_path(k): v for k, v in data.items()})

    def to_dict(self) -> dict:
        return {format_path(p): list(v) for p, v in self._b.items()}

    def paths(self) -> tuple:
        return tuple(self._b)

    def branch(self, path: Path) -> tuple:
        return self._b[tuple(path)]

    def branches(self) -> tuple:
        return tuple(self._b.values())

    def items(self) -> tuple:
        return tuple(i for v in self._b.values() for i in v)

    def item(self, path: Path, index: int, wrap: bool = False):
        b = self._b[tuple(path)]
        if wrap:
            return b[index % len(b)]
        if not 0 <= index < len(b):
            raise IndexError(f"Index {index} outside {format_path(path)}")
        return b[index]

    @property
    def count(self) -> int:
        return sum(len(v) for v in self._b.values())

    @property
    def path_count(self) -> int:
        return len(self._b)

    def counts(self) -> dict:
        return {p: len(v) for p, v in self._b.items()}

    def stats(self) -> dict:
        depths = sorted({len(p) for p in self._b})
        return {"path_count": self.path_count, "item_count": self.count, "counts": self.counts(), "depths": depths,
                "longest": max((len(v) for v in self._b.values()), default=0)}

    def map_items(self, fn) -> "Tree":
        return Tree({p: tuple(fn(i) for i in v) for p, v in self._b.items()})

    def merge(self, other: "Tree") -> "Tree":
        out = dict(self._b)
        for p, v in other._b.items():
            out[p] = out.get(p, ()) + v
        return Tree(out)

    def flatten(self, path: Path = (0,)) -> "Tree":
        return Tree({tuple(path): self.items()})

    def graft(self) -> "Tree":
        return Tree({p + (i,): (x,) for p, v in self._b.items() for i, x in enumerate(v)})

    def simplify(self, mode: str = "all") -> "Tree":
        ps = self.paths()
        if not ps:
            return self
        if len(ps) == 1:
            return Tree({(0,): self._b[ps[0]]})
        n, lead, trail = min(len(p) for p in ps), 0, 0
        if mode in ("all", "front"):
            while lead < n - 1 and len({p[lead] for p in ps}) == 1:
                lead += 1
        if mode in ("all", "back"):
            while trail < n - 1 - lead and len({p[len(p) - 1 - trail] for p in ps}) == 1:
                trail += 1
        out = {}
        for p, v in self._b.items():
            q = p[lead:len(p) - trail]
            out[q] = out.get(q, ()) + v
        return Tree(out)

    def flip(self) -> "Tree":
        if len({len(p) for p in self._b}) > 1:
            raise ValueError("Flip Matrix needs branches of equal path depth")
        rows = list(self._b.values())
        m = max((len(r) for r in rows), default=0)
        return Tree({(j,): tuple(r[j] if j < len(r) else None for r in rows) for j in range(m)})

    def explode(self) -> tuple:
        return tuple((p, v) for p, v in self._b.items())

    @staticmethod
    def entwine(trees) -> "Tree":
        return Tree({p + (k,): v for k, t in enumerate(trees) for p, v in t._b.items()})

    def map(self, source: str, target: str | None = None) -> "Tree":
        if target is None:
            source, target = split_mapping(source)
        src_mask, src_index = parse_mask(source)
        dst_mask, dst_index = parse_mask(target)
        groups: dict = {}
        for n, (path, items) in enumerate(self._b.items()):
            if len(path) != len(src_mask):
                continue
            env = {"path_count": self.path_count, "item_count": len(items), "path_index": n}
            ok = True
            for name, i in zip(src_mask, path):
                if name.lstrip("-").isdigit():
                    ok = ok and int(name) == i
                else:
                    env[name] = i
            if not ok:
                continue
            for i, item in enumerate(items):
                local = {**env, **({src_index: i} if src_index else {})}
                target_path = tuple(_eval_index(e, local) for e in dst_mask)
                key = _eval_index(dst_index, local) if dst_index else None
                groups.setdefault(target_path, []).append((key, len(groups.get(target_path, [])), item))
        out = {}
        for p, rows in groups.items():
            if dst_index:
                rows = sorted(rows, key=lambda r: (r[0], r[1]))
            out[p] = tuple(r[2] for r in rows)
        return Tree(out)

    def __eq__(self, other):
        return isinstance(other, Tree) and self._b == other._b

    def __repr__(self):
        return "Tree(" + ", ".join(f"{format_path(p)}: {list(v)}" for p, v in self._b.items()) + ")"


_MAPPING = re.compile(r"^\s*(\{[^}]*\}\s*(?:\([^)]*\))?)\s*(?:->|→)\s*(\{[^}]*\}\s*(?:\([^)]*\))?)\s*$")
_MASK = re.compile(r"^\s*\{([^}]*)\}\s*(?:\(([^)]*)\))?\s*$")


def split_mapping(text: str):
    m = _MAPPING.match(text)
    if not m:
        raise ValueError(f"Not a Path Mapper mapping: {text}")
    return m.group(1), m.group(2)


def parse_mask(text: str):
    m = _MASK.match(text)
    if not m:
        raise ValueError(f"Not a path mask: {text}")
    parts = [p.strip() for p in m.group(1).split(";")] if m.group(1).strip() else []
    return parts, (m.group(2).strip() if m.group(2) is not None else None)


def _eval_index(expr: str, env: dict) -> int:
    node = ast.parse(expr.replace("/", "//"), mode="eval").body

    def go(n):
        if isinstance(n, ast.Constant) and isinstance(n.value, int):
            return n.value
        if isinstance(n, ast.Name):
            return env[n.id]
        if isinstance(n, ast.UnaryOp) and isinstance(n.op, ast.USub):
            return -go(n.operand)
        if isinstance(n, ast.BinOp):
            ops = {ast.Add: lambda a, b: a + b, ast.Sub: lambda a, b: a - b, ast.Mult: lambda a, b: a * b,
                   ast.FloorDiv: lambda a, b: a // b, ast.Mod: lambda a, b: a % b}
            return ops[type(n.op)](go(n.left), go(n.right))
        raise ValueError(f"Unsupported expression {expr}")

    return go(node)


@lru_cache(maxsize=1)
def _source() -> str:
    return File(__file__).with_name("🔣️.json").read_text(encoding="utf-8")


def load_model() -> dict:
    """📦 Load a fresh copy of the schema-validated Grasshopper model data."""
    return json.loads(_source())


@lru_cache(maxsize=1)
def _model() -> dict:
    return json.loads(_source())


@dataclass(frozen=True)
class Cast:
    """🔀 Result of an implicit conversion."""

    ok: bool
    value: object
    message: dict
    rule: str
    basis: str


class _Fail(ValueError):
    pass


def _parse_number(text, culture):
    t = text.strip()
    if culture == "de" and "," in t and "." not in t:
        t = t.replace(",", ".")
    if not re.fullmatch(NUMBER, t):
        raise _Fail(text)
    return float(t)


def _parse_triple(text):
    t = text.strip().strip("{}")
    parts = [p.strip() for p in t.split(",")]
    if len(parts) != 3 or not all(re.fullmatch(NUMBER, p) for p in parts):
        raise _Fail(text)
    return tuple(float(p) for p in parts)


def _parse_domain(text):
    m = re.fullmatch(rf"\s*({NUMBER})\s+to\s+({NUMBER})\s*", text, re.IGNORECASE)
    if not m:
        raise _Fail(text)
    return Domain(float(m.group(1)), float(m.group(2)))


def _plane_of_line(line):
    z = _unit(_sub(line.b.t, line.a.t))
    x = (1.0, 0.0, 0.0) if abs(z[2]) > 1 - 1e-9 else _unit(_cross((0.0, 0.0, 1.0), z))
    return Plane.from_axes(line.a, Vector(*x), Vector(*_cross(z, x)))


def _shape(value, kind):
    if isinstance(value, Curve) and isinstance(value.shape, kind):
        return value.shape
    raise _Fail(value)


def _brep_box(v):
    box = v.root_box()
    if box is None:
        raise _Fail(v)
    return box


def _text_of(v, culture):
    if isinstance(v, bool):
        return "true" if v else "false"
    return describe(v) if not isinstance(v, (int, float)) else format_number(v, "en")


def _to_integer(v, culture):
    return round(v) if isinstance(v, float) else (int(v) if isinstance(v, bool) else _int_of_text(v, culture))


def _int_of_text(v, culture):
    return round(_parse_number(v, culture))


def _to_boolean_text(v, culture):
    t = v.strip().lower()
    if t in ("true", "yes", "1"):
        return True
    if t in ("false", "no", "0"):
        return False
    raise _Fail(v)


_CONVERT = {
    ("Integer", "Number"): lambda v, c: float(v),
    ("Boolean", "Number"): lambda v, c: 1.0 if v else 0.0,
    ("Text", "Number"): lambda v, c: _parse_number(v, c),
    ("Domain", "Number"): lambda v, c: float(v.length),
    ("Point", "Number"): lambda v, c: _len(v.t),
    ("Number", "Integer"): _to_integer,
    ("Boolean", "Integer"): _to_integer,
    ("Text", "Integer"): _to_integer,
    ("Number", "Boolean"): lambda v, c: v != 0,
    ("Integer", "Boolean"): lambda v, c: v != 0,
    ("Text", "Boolean"): _to_boolean_text,
    ("Number", "Text"): _text_of,
    ("Integer", "Text"): _text_of,
    ("Boolean", "Text"): _text_of,
    ("Point", "Text"): _text_of,
    ("Vector", "Text"): _text_of,
    ("Domain", "Text"): _text_of,
    ("Text", "Point"): lambda v, c: Point(*_parse_triple(v)),
    ("Vector", "Point"): lambda v, c: Point(*v.t),
    ("Plane", "Point"): lambda v, c: v.origin,
    ("Point", "Vector"): lambda v, c: Vector(*v.t),
    ("Line", "Vector"): lambda v, c: Vector(*_sub(v.b.t, v.a.t)),
    ("Text", "Vector"): lambda v, c: Vector(*_parse_triple(v)),
    ("Point", "Plane"): lambda v, c: Plane.world_xy(v),
    ("Line", "Plane"): lambda v, c: _plane_of_line(v),
    ("Circle", "Plane"): lambda v, c: v.plane,
    ("Number", "Domain"): lambda v, c: Domain(0.0, float(v)),
    ("Integer", "Domain"): lambda v, c: Domain(0.0, float(v)),
    ("Text", "Domain"): lambda v, c: _parse_domain(v),
    ("Circle", "Curve"): lambda v, c: Curve(v),
    ("Line", "Curve"): lambda v, c: Curve(v),
    ("Curve", "Circle"): lambda v, c: _shape(v, Circle),
    ("Curve", "Line"): lambda v, c: _shape(v, Line),
    ("Curve", "Plane"): lambda v, c: _shape(v, Circle).plane,
    ("Box", "Brep"): lambda v, c: Brep({"kind": "box", "box": v}),
    ("Brep", "Box"): lambda v, c: _brep_box(v),
}


def rules() -> dict:
    """📚 The casting rules with basis and bilingual explanations."""
    return _model()["casting"]["rules"]


def cast(value, target: str, culture: str = "en") -> Cast:
    """🔀 Convert a value to a Grasshopper parameter type; a failure keeps the real Grasshopper message."""
    table = rules()
    src = type_name(value)

    def done(rule, v):
        r = table[rule]
        return Cast(True, v, {"en": r["en"], "de": r["de"]}, rule, r["basis"])

    def fail(rule, suffix="!"):
        gh = f"Data conversion failed from {src} to {target}"
        r = table.get(rule + suffix) or table["*>*!"]
        return Cast(False, None, {"en": gh, "de": r["de"].format(source=src, target=target, gh=gh), "gh": gh}, rule + suffix if rule + suffix in table else "*>*!", r["basis"])

    if value is None or target == "Generic":
        return Cast(True, value, {"en": "", "de": ""}, "same", "verified")
    if src == target:
        return done("same", value)
    if target == "Geometry":
        return done(f"{src}>Geometry", value) if src in GEOMETRY else fail(f"{src}>Geometry")
    if target == "Transform":
        return fail(f"{src}>{target}")
    fn = _CONVERT.get((src, target))
    if fn is None:
        return fail(f"{src}>{target}")
    try:
        return done(f"{src}>{target}", fn(value, culture))
    except _Fail:
        if (src, target) == ("Text", "Number") and re.fullmatch(r"\s*[-+]?\d+,\d+\s*", value):
            return fail("Text>Number", "!comma")
        return fail(f"{src}>{target}")


@dataclass(frozen=True)
class ParamSpec:
    name: str
    nickname: str
    type: str
    access: str
    optional: bool = False
    default: object = None
    description: dict = field(default_factory=dict)


def _missing(component, name):
    raise KeyError(f"{component} has no parameter {name}")


@dataclass(frozen=True)
class ComponentSpec:
    name: str
    nickname: str
    kind: str
    category: str
    subcategory: str
    description: dict
    inputs: tuple
    outputs: tuple

    def input(self, name):
        return next((p for p in self.inputs if p.name == name), None) or _missing(self.name, name)

    def output(self, name):
        return next((p for p in self.outputs if p.name == name), None) or _missing(self.name, name)


def _param(d):
    return ParamSpec(d["name"], d["nickname"], d["type"], d["access"], d.get("optional", False), d.get("default"), d.get("description", {}))


@lru_cache(maxsize=1)
def _catalogue() -> dict:
    return {n: ComponentSpec(n, c["nickname"], c["kind"], c["category"], c["subcategory"], c["description"],
                             tuple(_param(p) for p in c["inputs"]), tuple(_param(p) for p in c["outputs"]))
            for n, c in _model()["catalogue"].items()}


def catalogue() -> dict:
    """🧩 The component catalogue with real Grasshopper names."""
    return dict(_catalogue())


def _f(seq):
    return tuple(float(c) for c in seq)


def decode(x):
    """🔤 Turn a JSON literal into a Grasshopper value."""
    if isinstance(x, dict):
        (k, v), = x.items()
        if k == "point":
            return Point(*_f(v))
        if k == "vector":
            return Vector(*_f(v))
        if k == "plane":
            return Plane.from_axes(Point(*_f(v["origin"])), Vector(*_f(v["x"])), Vector(*_f(v["y"])))
        if k == "domain":
            return Domain(*_f(v))
        if k == "circle":
            return Circle(decode(v["plane"]), float(v["radius"]))
        if k == "line":
            return Line(Point(*_f(v[0])), Point(*_f(v[1])))
        if k == "box":
            return Box(decode(v["plane"]), Domain(*_f(v["x"])), Domain(*_f(v["y"])), Domain(*_f(v["z"])))
        if k == "capsule":
            return make_capsule_of(v)
        if k == "capsule_window":
            return make_capsule_of(v).window()
        if k == "capsule_box":
            return make_capsule_of(v).root_box()
        if k == "nakagin":
            return nakagin_value(v)
        if k == "slot":
            return slot_part(v["index"], v["part"])
        raise ValueError(f"Unknown literal {k}")
    return x


@lru_cache(maxsize=1)
def _nakagin():
    spec = importlib.util.spec_from_file_location("nakagin_model_for_grasshopper", NAKAGIN)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def nakagin_value(path: str):
    """🏢 A value of the shared Nakagin teaching model."""
    v = _nakagin().value(path).value
    return float(v) if isinstance(v, float) else v


@lru_cache(maxsize=1)
def capsule_types() -> tuple:
    """💊 Capsule type rows read from the Nakagin model (dimensions, window, rotation) plus the engine wall thickness."""
    wall = _model()["capsule_wall"]["value"]
    return tuple({"key": t.key, "name": t.name, "width": t.width, "length": t.length, "height": t.height, "corner_radius": t.corner_radius,
                  "window_diameter": t.window_diameter, "window_u": t.window_u, "window_v": t.window_v, "rotation": t.rotation, "wall": wall}
                 for t in _nakagin().capsule_types())


def capsule_type(key: str) -> dict:
    return next(t for t in capsule_types() if t["key"] == key)


def make_capsule(width, length, height, corner, window, wall, tag=None, window_u=0.0, window_v=None) -> Brep:
    """💊 Build a capsule exactly like the capsule canvas: box, fillet, window cut (back face at y=0, base on XY)."""
    plane = Plane.world_xy(Point(0.0, length / 2, height / 2))
    box = Box(plane, Domain(-width / 2, width / 2), Domain(-length / 2, length / 2), Domain(-height / 2, height / 2))
    fillet = {"kind": "fillet", "base": {"kind": "box", "box": box}, "radius": corner, "edges": None}
    v = height / 2 if window_v is None else window_v
    cyl = {"kind": "cylinder", "plane": Plane.world_xz(Point(window_u, length, v)), "radius": window / 2, "length": wall}
    return Brep({"kind": "difference", "base": fillet, "cuts": (cyl,)}, IDENTITY, dict(tag or {}))


def make_capsule_of(key: str) -> Brep:
    t = capsule_type(key)
    return make_capsule(t["width"], t["length"], t["height"], t["corner_radius"], t["window_diameter"], t["wall"], {"type": key}, t["window_u"], t["window_v"])


def _plane_of(p) -> Plane:
    return Plane(Point(*p.origin), Vector(*p.x), Vector(*p.y), Vector(*p.z))


def slot_part(index, part: str):
    """📍 A part of a Nakagin slot: its plane, the unrotated facade origin or axes, or the line from the core centre."""
    slot = next(s for s in _nakagin().slots() if (s.building, s.tower, s.storey, s.facade, s.item) == tuple(index))
    base = _plane_of(slot.facade_plane)
    if part == "plane":
        return _plane_of(slot.plane)
    if part == "origin":
        return base.origin
    if part == "x":
        return base.x
    if part == "y":
        return base.y
    cx, cy = _nakagin().towers()[slot.tower].center
    return Line(Point(cx, cy, base.origin.z), base.origin)


def aggregate_tree() -> Tree:
    """🏢 The aggregated {BUILDING;TOWER;STORY;FACADE} tree of the Nakagin model with its oriented capsules per branch."""
    cache = {t["key"]: make_capsule_of(t["key"]) for t in capsule_types()}
    out = {}
    for path, slots in _nakagin().tree().items():
        items = []
        for s in slots:
            b = cache[s.type]
            items.append(replace(b, xform=plane_to_plane(Plane.world_xy(), _plane_of(s.plane)), tag={**b.tag, "slot": (s.building, s.tower, s.storey, s.facade, s.item)}))
        out[tuple(path)] = tuple(items)
    return Tree(out)


def _tower(key: str):
    return next(t for t in _nakagin().towers() if t.key == key)


def slot_tree(tower: str, part: str) -> Tree:
    """🗼 Per-slot data of one Nakagin tower as a tree {0;storey}: yaw degrees, indices of absent planes to cull, or oriented-ready capsule Breps."""
    nak, t = _nakagin(), _tower(tower)
    base = nak.load()["towers"][t.index]["facades"]
    present = nak.facade_storeys(t)
    by_storey = {}
    for slot in nak.slots():
        if slot.tower == t.index:
            by_storey.setdefault(slot.storey, []).append(slot)
    if part == "cull":
        return Tree({(0, s): tuple(p for p in range(2 * len(base)) if base[p // 2] not in present[s]) for s in range(t.capsule_storeys)})
    if part == "yaw":
        return Tree({(0, s): tuple(float(x.rotation) for x in by_storey[s]) for s in range(t.capsule_storeys)})
    if part == "capsule":
        cache = {c["key"]: make_capsule_of(c["key"]) for c in capsule_types()}
        return Tree({(0, s): tuple(cache[x.type] for x in by_storey[s]) for s in range(t.capsule_storeys)})
    raise ValueError(f"Unknown slot tree part {part}")


def tower_overrides(tower: str) -> dict:
    """🗼 Overrides that turn the tower canvas into tower A or B of the Nakagin model (storeys, facades, centre, per-slot data)."""
    nak, t = _nakagin(), _tower(tower)
    base = nak.load()["towers"][t.index]["facades"]
    lit = lambda part: {"slot_tree": {"tower": tower, "part": part}}
    return {"n": t.capsule_storeys, "ang": [str(-90 * f) for f in base], "cx": t.center[0], "cy": t.center[1],
            "cullp": lit("cull"), "yawp": lit("yaw"), "capp": lit("capsule")}


def data_tree(data) -> Tree:
    """🌳 Persistent data of a parameter as a tree."""
    if isinstance(data, dict) and "aggregate" in data:
        return aggregate_tree()
    if isinstance(data, dict) and "slot_tree" in data:
        return slot_tree(data["slot_tree"]["tower"], data["slot_tree"]["part"])
    if isinstance(data, dict) and "tree" in data:
        return Tree({parse_path(k): tuple(decode(i) for i in v) for k, v in data["tree"].items()})
    return Tree.from_list(tuple(decode(i) for i in data))


@dataclass(frozen=True)
class Obj:
    id: str
    component: str
    at: tuple
    settings: dict


@dataclass(frozen=True)
class Wire:
    src: str
    out: str
    dst: str
    inp: str


@dataclass(frozen=True)
class Definition:
    """🦗 A canvas: objects with positions, wires and named views."""

    key: str
    title: dict
    objects: dict
    wires: tuple
    views: dict

    def obj(self, oid: str) -> Obj:
        return self.objects[oid]

    def spec(self, oid: str) -> ComponentSpec:
        return _catalogue()[self.objects[oid].component]

    def wires_into(self, oid: str, name: str) -> tuple:
        return tuple(w for w in self.wires if w.dst == oid and w.inp == name)

    def view(self, name: str) -> tuple:
        v = self.views[name]
        return tuple(replace(self.objects[i], at=tuple(v["layout"][i])) for i in v["objects"])

    def view_wires(self, name: str) -> tuple:
        ids = set(self.views[name]["objects"])
        return tuple(w for w in self.wires if w.src in ids and w.dst in ids)

    def bounds(self, view: str | None = None) -> tuple:
        objs = self.view(view) if view else tuple(self.objects.values())
        xs, ys = [o.at[0] for o in objs], [o.at[1] for o in objs]
        return (min(xs), min(ys), max(xs), max(ys))

    def order(self) -> tuple:
        pending, done, out = list(self.objects), set(), []
        deps = {o: {w.src for w in self.wires if w.dst == o} for o in self.objects}
        while pending:
            ready = [o for o in pending if deps[o] <= done]
            if not ready:
                raise ValueError("Cycle in definition " + self.key)
            for o in ready:
                out.append(o)
                done.add(o)
                pending.remove(o)
        return tuple(out)


def _parse_wire(text: str) -> Wire:
    left, right = text.split(">")
    s, o = left.split(".", 1)
    d, i = right.split(".", 1)
    return Wire(s, o, d, i)


def _bound(settings: dict) -> dict:
    out = dict(settings)
    if "bind" in out:
        out["value"] = nakagin_value(out["bind"])
    if "items" in out:
        out["items"] = [{**i, "value": nakagin_value(i["bind"])} if "bind" in i else i for i in out["items"]]
    return out


def _line(x) -> str:
    if isinstance(x, dict):
        (k, v), = x.items()
        return format_number(nakagin_value(v) if k == "nakagin" else _model()[v]["value"])
    return x


def validate(defn: Definition) -> None:
    """✅ Check ids, wire endpoints and view members of a definition."""
    cat = _catalogue()
    for o in defn.objects.values():
        if o.component not in cat:
            raise ValueError(f"{defn.key}: unknown component {o.component}")
    for w in defn.wires:
        if w.src not in defn.objects or w.dst not in defn.objects:
            raise ValueError(f"{defn.key}: wire to unknown object {w}")
        defn.spec(w.src).output(w.out)
        defn.spec(w.dst).input(w.inp)
    for v in defn.views.values():
        for i in v["objects"]:
            if i not in defn.objects:
                raise ValueError(f"{defn.key}: view lists unknown object {i}")


def canvas_keys() -> tuple:
    return tuple(_model()["canvases"])


def define(spec: dict, key: str = "custom") -> Definition:
    """🧪 Build a definition from canvas-shaped data (objects, wires, optional views) without binding to the Nakagin model."""
    objs = {o["id"]: Obj(o["id"], o["component"], tuple(o.get("at", (0, 0))), o.get("settings", {})) for o in spec["objects"]}
    defn = Definition(key, spec.get("title", {"en": key, "de": key}), objs, tuple(_parse_wire(w) for w in spec["wires"]), spec.get("views", {}))
    validate(defn)
    return defn


@lru_cache(maxsize=None)
def _canvas(key: str) -> Definition:
    c = _model()["canvases"][key]
    return define({**c, "objects": [{**o, "settings": _bound(o.get("settings", {}))} for o in c["objects"]]}, key)


def canvas(key: str) -> Definition:
    """🖼️ One of the five canvases: anatomy, capsule, tower, casting, trees."""
    return _canvas(key)


@dataclass(frozen=True)
class Solution:
    """📋 Result of solving a definition: outputs as trees, state and messages per object."""

    definition: Definition
    trees: dict
    states: dict
    messages: dict

    def tree(self, oid: str, name: str | None = None) -> Tree:
        name = name or self.definition.spec(oid).outputs[0].name
        return self.trees[(oid, name)]

    def items(self, oid: str, name: str | None = None) -> tuple:
        return self.tree(oid, name).items()

    def value(self, oid: str, name: str | None = None):
        return self.items(oid, name)[0]

    def state(self, oid: str) -> str:
        return self.states[oid]

    def msgs(self, oid: str) -> tuple:
        return self.messages[oid]


def _msg(level, en, de):
    return {"level": level, "en": en, "de": de}


def slider_value(s: dict):
    """🎚️ The value a Number Slider delivers after clamping and rounding."""
    lo, hi, v = float(s["min"]), float(s["max"]), float(s["value"])
    v = min(max(v, lo), hi)
    mode = s.get("rounding", "float")
    if mode == "integer":
        return int(round(v))
    if mode == "even":
        return int(2 * round(v / 2))
    if mode == "odd":
        return int(2 * math.floor(v / 2) + 1)
    return round(v, s.get("decimals", 2))


def _apply_overrides(obj: Obj, spec: ComponentSpec, overrides: dict) -> dict:
    s = dict(obj.settings)
    key_for = {"slider": "value", "toggle": "value", "valuelist": "selected", "panel": "lines", "param": "data"}
    if obj.id in overrides and spec.kind in key_for:
        s[key_for[spec.kind]] = overrides[obj.id]
    for k, v in overrides.items():
        if k.startswith(obj.id + "."):
            s[k[len(obj.id) + 1:]] = v
    return s


class _Warn(Exception):
    pass


IMPL: dict = {}


def component(name):
    def wrap(fn):
        IMPL[name] = fn
        return fn

    return wrap


def _bin(name, fn):
    IMPL[name] = lambda a, s: {"Result": fn(a["A"], a["B"])}


_bin("Addition", lambda a, b: a + b)
_bin("Subtraction", lambda a, b: a - b)
_bin("Multiplication", lambda a, b: a * b)
_bin("Division", lambda a, b: a / b)
IMPL["Negative"] = lambda a, s: {"Result": -a["Value"]}
IMPL["Radians"] = lambda a, s: {"Radians": math.radians(a["Degrees"])}
IMPL["Round"] = lambda a, s: {"Result": float(round(a["Number"])), "Integer": int(round(a["Number"]))}
IMPL["Floor"] = lambda a, s: {"Result": float(math.floor(a["Number"])), "Integer": math.floor(a["Number"])}
IMPL["Ceiling"] = lambda a, s: {"Result": float(math.ceil(a["Number"])), "Integer": math.ceil(a["Number"])}
IMPL["Series"] = lambda a, s: {"Series": [a["Start"] + i * a["Step"] for i in range(max(a["Count"], 0))]}
IMPL["Range"] = lambda a, s: {"Range": [a["Domain"].lo + i * a["Domain"].length / a["Steps"] for i in range(a["Steps"] + 1)]}
IMPL["Construct Domain"] = lambda a, s: {"Domain": Domain(a["Domain start"], a["Domain end"])}
IMPL["Divide Domain"] = lambda a, s: {"Segments": [Domain(a["Domain"].lo + i * a["Domain"].length / a["Count"], a["Domain"].lo + (i + 1) * a["Domain"].length / a["Count"]) for i in range(a["Count"])]}
IMPL["Construct Point"] = lambda a, s: {"Point": Point(a["X coordinate"], a["Y coordinate"], a["Z coordinate"])}
IMPL["XY Plane"] = lambda a, s: {"Plane": Plane.world_xy(a["Origin"])}
IMPL["XZ Plane"] = lambda a, s: {"Plane": Plane.world_xz(a["Origin"])}
IMPL["YZ Plane"] = lambda a, s: {"Plane": Plane.world_yz(a["Origin"])}
IMPL["Plane Origin"] = lambda a, s: {"Plane": replace(a["Plane"], origin=a["Origin"])}
IMPL["Construct Plane"] = lambda a, s: {"Plane": Plane.from_axes(a["Origin"], a["X-Axis"], a["Y-Axis"])}
IMPL["Unit X"] = lambda a, s: {"Unit vector": Vector(a["Factor"], 0.0, 0.0)}
IMPL["Unit Y"] = lambda a, s: {"Unit vector": Vector(0.0, a["Factor"], 0.0)}
IMPL["Unit Z"] = lambda a, s: {"Unit vector": Vector(0.0, 0.0, a["Factor"])}


@component("Rotate")
def _rotate(a, s):
    p = a["Plane"]
    m = rotation(a["Angle"], p.z, p.origin)
    return {"Geometry": apply(m, a["Geometry"]), "Transform": m}


@component("Move")
def _move(a, s):
    m = translation(a["Motion"])
    return {"Geometry": apply(m, a["Geometry"]), "Transform": m}


@component("Orient")
def _orient(a, s):
    m = plane_to_plane(a["Source"], a["Target"])
    return {"Geometry": apply(m, a["Geometry"]), "Transform": m}


@component("List Item")
def _list_item(a, s):
    lst, i = a["List"], a["Index"]
    if a["Wrap"] and lst:
        return {"Item": lst[i % len(lst)]}
    if not 0 <= i < len(lst):
        raise _Warn(f"Index {i} is outside the list of {len(lst)} items")
    return {"Item": lst[i]}


IMPL["List Length"] = lambda a, s: {"Length": len(a["List"])}
IMPL["Repeat Data"] = lambda a, s: {"Data": [a["Data"][i % len(a["Data"])] for i in range(a["Length"])]}


def _centered(n):
    return Domain(-n / 2, n / 2)


IMPL["Center Box"] = lambda a, s: {"Box": Box(a["Base"], _centered(a["X"]), _centered(a["Y"]), _centered(a["Z"]))}
IMPL["Domain Box"] = lambda a, s: {"Box": Box(a["Base"], a["X"], a["Y"], a["Z"])}
IMPL["Cylinder"] = lambda a, s: {"Cylinder": Brep({"kind": "cylinder", "plane": a["Base"], "radius": a["Radius"], "length": a["Length"]})}
IMPL["Circle"] = lambda a, s: {"Circle": Circle(a["Plane"], a["Radius"])}
IMPL["Rectangle"] = lambda a, s: {"Rectangle": Curve(Rectangle(a["Plane"], Domain(-a["X Size"] / 2, a["X Size"] / 2), Domain(-a["Y Size"] / 2, a["Y Size"] / 2))),
                                  "Length": 2 * (a["X Size"] + a["Y Size"])}
IMPL["Extrude"] = lambda a, s: {"Extrusion": Brep({"kind": "extrusion", "curve": a["Base"], "direction": a["Direction"]})}


@component("Fillet Edge")
def _fillet(a, s):
    b = a["Shape"]
    edges = tuple(a["Edges"]) if a["Edges"] else None
    return {"Result": replace(b, solid={"kind": "fillet", "base": b.solid, "radius": a["Radius"], "edges": edges})}


@component("Solid Difference")
def _difference(a, s):
    base = a["Brep A"][0]
    cuts = tuple(_bake(c.solid, _mul(invert(base.xform), c.xform)) for c in a["Brep B"])
    return {"Result": [replace(base, solid={"kind": "difference", "base": base.solid, "cuts": cuts})]}


@component("Volume")
def _volume(a, s):
    g = a["Geometry"]
    if isinstance(g, Box):
        return {"Volume": g.volume, "Centroid": g.center}
    box = g.root_box()
    return {"Volume": g.volume(), "Centroid": box.center if box else None}


@component("Cull Index")
def _cull_index(a, s):
    lst, idx = list(a["List"]), a["Indices"] or ()
    drop = {i % len(lst) for i in idx} if a["Wrap"] and lst else set(idx)
    return {"List": [x for i, x in enumerate(lst) if i not in drop]}


IMPL["Sine"] = lambda a, s: {"Result": math.sin(a["Value"])}
IMPL["Absolute"] = lambda a, s: {"Result": abs(a["Value"])}
IMPL["Vector XYZ"] = lambda a, s: {"Vector": Vector(a["X component"], a["Y component"], a["Z component"]),
                                   "Length": _len((a["X component"], a["Y component"], a["Z component"]))}
IMPL["Deconstruct Plane"] = lambda a, s: {"Origin": a["Plane"].origin, "X-Axis": a["Plane"].x, "Y-Axis": a["Plane"].y, "Z-Axis": a["Plane"].z}
IMPL["Amplitude"] = lambda a, s: {"Vector": Vector(*(c * a["Amplitude"] for c in _unit(a["Vector"].t)))}


@component("Dispatch")
def _dispatch(a, s):
    pattern = a["Pattern"]
    first = [x for i, x in enumerate(a["List"]) if pattern[i % len(pattern)]]
    second = [x for i, x in enumerate(a["List"]) if not pattern[i % len(pattern)]]
    return {"List A": first, "List B": second}


@component("Merge")
def _merge(a, s):
    out = Tree()
    for k in ("Data 1", "Data 2", "Data 3", "Data 4"):
        out = out.merge(a[k])
    return {"Result": out}


IMPL["Entwine"] = lambda a, s: {"Result": Tree.entwine([a[k] for k in ("Branch 1", "Branch 2", "Branch 3") if a[k].path_count])}
IMPL["Graft Tree"] = lambda a, s: {"Tree": a["Tree"].graft()}
IMPL["Flatten Tree"] = lambda a, s: {"Tree": a["Tree"].flatten(parse_path(a["Path"]) if a["Path"] else (0,))}
IMPL["Simplify Tree"] = lambda a, s: {"Tree": a["Tree"].simplify("front" if a["Front"] else "all")}
IMPL["Path Mapper"] = lambda a, s: {"Tree": a["Tree"].map(a["Mapping"] or s["mapping"])}
IMPL["Flip Matrix"] = lambda a, s: {"Data": a["Data"].flip()}
IMPL["Tree Statistics"] = lambda a, s: {"Paths": [format_path(p) for p in a["Tree"].paths()], "Length": list(len(v) for v in a["Tree"].branches()),
                                        "Count": a["Tree"].path_count}


@component("Explode Tree")
def _explode(a, s):
    rows = a["Data"].explode()
    return {f"Branch {i + 1}": list(rows[i][1]) if i < len(rows) else [] for i in range(4)}


@component("Tree Item")
def _tree_item(a, s):
    return {"Item": a["Tree"].item(parse_path(a["Path"]), a["Index"], a["Wrap"])}


@component("Custom Preview")
def _preview(a, s):
    g = a["Geometry"]
    return {"Preview": replace(g, tag={**g.tag, "material": a["Material"]})}


def _merged(trees) -> Tree:
    out = Tree()
    for t in trees:
        out = out.merge(t)
    return out


def _cast_tree(tree: Tree, typ: str, name: str):
    msgs, seen = [], set()

    def one(v):
        c = cast(v, typ)
        if not c.ok and c.message["en"] not in seen:
            seen.add(c.message["en"])
            msgs.append(_msg("warning", c.message["en"], c.message["de"]))
        return c.value

    return tree.map_items(one), msgs


def _call_all(spec, inputs, impl, settings):
    iters = [p for p in spec.inputs if p.access != "tree" and inputs[p.name].path_count]
    out = {o.name: {} for o in spec.outputs}
    warns = []

    def store(path, res, count_j, j):
        for o in spec.outputs:
            v = res.get(o.name)
            if o.access == "item":
                out[o.name].setdefault(path, []).append(v)
            elif o.access == "list":
                out[o.name][path if count_j == 1 else path + (j,)] = list(v or [])
            elif v is not None:
                for p, items in v._b.items():
                    out[o.name][p] = list(items)

    def call(args):
        try:
            return impl(args, settings)
        except _Warn as w:
            warns.append(_msg("warning", str(w), "Der Index liegt außerhalb der Liste." if "Index" in str(w) else str(w)))
            return {}

    base = {p.name: (inputs[p.name] if p.access == "tree" else None) for p in spec.inputs}
    if not iters:
        res = call({**base, **{p.name: None for p in spec.inputs if p.access != "tree"}})
        for o in spec.outputs:
            v = res.get(o.name)
            if o.access == "tree":
                if v is not None:
                    out[o.name] = {p: list(i) for p, i in v._b.items()}
            elif o.access == "list":
                out[o.name][(0,)] = list(v or [])
            else:
                out[o.name][(0,)] = [v]
        return out, warns
    driver = max(iters, key=lambda p: inputs[p.name].path_count)
    for k, path in enumerate(inputs[driver.name].paths()):
        branches = {}
        for p in iters:
            t = inputs[p.name]
            branches[p.name] = t.branch(path) if path in t._b else t.branches()[min(k, t.path_count - 1)]
        lens = [len(branches[p.name]) for p in iters if p.access == "item"]
        n = max(lens) if lens else 1
        if lens and min(lens) == 0:
            continue
        for j in range(n):
            args = dict(base)
            for p in iters:
                b = branches[p.name]
                args[p.name] = b[min(j, len(b) - 1)] if p.access == "item" else tuple(b)
            if any(args[p.name] is None for p in iters if p.access == "item"):
                store(path, {}, n, j)
                continue
            store(path, call(args), n, j)
    return out, warns


def _input_tree(defn, oid, p, trees):
    wires = defn.wires_into(oid, p.name)
    if wires:
        return _merged(trees.get((w.src, w.out), Tree()) for w in wires)
    if p.default is not None:
        return Tree.from_item(decode(p.default))
    return Tree()


def evaluate(defn: Definition, overrides: dict | None = None) -> Solution:
    """⚙️ Solve a canvas: wires carry trees, inputs are cast, item inputs iterate with longest-list matching and branch matching."""
    overrides = overrides or {}
    cat = _catalogue()
    trees, states, messages = {}, {}, {}
    for oid in defn.order():
        obj = defn.objects[oid]
        spec = cat[obj.component]
        s = _apply_overrides(obj, spec, overrides)
        msgs, state = [], "ok"
        first = spec.outputs[0].name if spec.outputs else None
        if spec.kind == "slider":
            trees[(oid, first)] = Tree.from_item(slider_value(s))
        elif spec.kind == "toggle":
            trees[(oid, first)] = Tree.from_item(bool(s["value"]))
        elif spec.kind == "valuelist":
            item = s["items"][int(s.get("selected", 0))]
            trees[(oid, first)] = Tree.from_item(decode(item["value"]))
        elif spec.kind == "panel":
            wires = defn.wires_into(oid, "Stream")
            if wires:
                trees[(oid, first)] = _merged(trees.get((w.src, w.out), Tree()) for w in wires)
            else:
                lines = s.get("lines", [])
                trees[(oid, first)] = Tree.from_list(tuple(_line(x) for x in ([lines] if isinstance(lines, str) else lines)))
        elif spec.kind == "param":
            typ = spec.outputs[0].type
            wires = defn.wires_into(oid, spec.inputs[0].name)
            raw = _merged(trees.get((w.src, w.out), Tree()) for w in wires) if wires else data_tree(s.get("data", []))
            tree, msgs = _cast_tree(raw, typ, first)
            trees[(oid, first)] = tree
        elif spec.kind == "viewer":
            wires = defn.wires_into(oid, spec.inputs[0].name)
            trees[(oid, "Data")] = _merged(trees.get((w.src, w.out), Tree()) for w in wires)
        else:
            inputs = {}
            for p in spec.inputs:
                raw = _input_tree(defn, oid, p, trees)
                tree, m = (raw, []) if p.type == "Generic" else _cast_tree(raw, p.type, p.name)
                inputs[p.name] = tree
                msgs += m
                if not any(i is not None for i in tree.items()) and not p.optional:
                    msgs.append(_msg("error", f"Input parameter {p.name} failed to collect data",
                                     f'Der Eingang "{p.name}" hat keine Daten erhalten ("Input parameter {p.name} failed to collect data").'))
            if any(m["level"] == "error" for m in msgs):
                out = {o.name: {} for o in spec.outputs}
            else:
                try:
                    out, warns = _call_all(spec, inputs, IMPL[obj.component], s)
                    msgs += warns
                except (ValueError, ZeroDivisionError, TypeError, IndexError, KeyError) as e:
                    msgs.append(_msg("error", f"Solution exception: {e}", f"Fehler bei der Berechnung: {e}"))
                    out = {o.name: {} for o in spec.outputs}
            for o in spec.outputs:
                trees[(oid, o.name)] = Tree({p: tuple(v) for p, v in out[o.name].items()})
        if any(m["level"] == "error" for m in msgs):
            state = "error"
        elif msgs:
            state = "warning"
        states[oid], messages[oid] = state, tuple(msgs)
        for o in spec.outputs:
            trees.setdefault((oid, o.name), Tree())
    return Solution(defn, trees, states, messages)


def panel_rows(tree: Tree, paths: bool = True) -> tuple:
    """🖥️ Rows a Panel shows: (path text or None, index, item text); one block per branch when paths are on."""
    rows = []
    for p, v in tree._b.items():
        for i, x in enumerate(v):
            rows.append((format_path(p) if paths else None, i, describe(x)))
    return tuple(rows)


def plain(value):
    """📤 JSON-friendly form of any engine value (trees, points, planes, solids)."""
    if isinstance(value, Tree):
        return {format_path(p): [plain(i) for i in v] for p, v in value._b.items()}
    if isinstance(value, (Point, Vector)):
        return list(value.t)
    if isinstance(value, Plane):
        return {"origin": plain(value.origin), "x": plain(value.x), "y": plain(value.y), "z": plain(value.z)}
    if isinstance(value, Domain):
        return [value.lo, value.hi]
    if isinstance(value, Circle):
        return {"plane": plain(value.plane), "radius": value.radius}
    if isinstance(value, Line):
        return [plain(value.a), plain(value.b)]
    if isinstance(value, Rectangle):
        return {"plane": plain(value.plane), "x": plain(value.x), "y": plain(value.y)}
    if isinstance(value, Curve):
        return {"curve": plain(value.shape)}
    if isinstance(value, Box):
        return {"plane": plain(value.plane), "x": plain(value.x), "y": plain(value.y), "z": plain(value.z)}
    if isinstance(value, Brep):
        return {"volume": value.volume(), "tag": {k: list(v) if isinstance(v, tuple) else v for k, v in value.tag.items()}}
    if isinstance(value, tuple):
        return [plain(i) for i in value]
    return value
