"""🏛️ Schema-first Villa Savoye teaching model: one IFC 4.3 JSON source, stdlib-only derivations."""

from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
from itertools import count as _RevisionCounter
from threading import RLock as _GeometryLock
import importlib
import hashlib
import json
import math
from pathlib import Path
from types import SimpleNamespace
import uuid

ALPHABET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$"
SEED = "semio:villa-savoye:"
LANGUAGES = ("en", "de")
ARC_STEP = 7.5


def compress_guid(value: int) -> str:
    """🔐 Encode a 128-bit integer as a 22-character IFC GlobalId."""
    if isinstance(value, bool) or not isinstance(value, int) or not 0 <= value < 1 << 128:
        raise ValueError("A GlobalId encodes an integer of 128 bits")
    digits = []
    for _ in range(22):
        value, digit = divmod(value, 64)
        digits.append(ALPHABET[digit])
    return "".join(reversed(digits))


def decompress_guid(text: str) -> int:
    """🔓 Decode a 22-character IFC GlobalId into its 128-bit integer."""
    if not isinstance(text, str) or len(text) != 22 or any(char not in ALPHABET for char in text):
        raise ValueError("A GlobalId has 22 characters of 0-9A-Za-z_$")
    value = 0
    for char in text:
        value = value * 64 + ALPHABET.index(char)
    if value >= 1 << 128:
        raise ValueError("The first GlobalId character is 0 to 3")
    return value


def global_id(tag: str) -> str:
    """🪪 Derive the stable GlobalId of a teaching object from its tag."""
    return compress_guid(uuid.uuid5(uuid.NAMESPACE_URL, SEED + tag).int)


def source_path() -> Path:
    """📍 Locate the language-independent Villa Savoye model source file."""
    return Path(__file__).with_name("🔣️.json")


@lru_cache(maxsize=1)
def source_text() -> str:
    """📄 Read the model source text once per process."""
    return source_path().read_text(encoding="utf-8")


def load_model() -> dict:
    """📦 Load a fresh, language-independent Villa Savoye teaching model."""
    return json.loads(source_text())


def _m(model):
    return load_model() if model is None else model


def role(key: str, model: dict | None = None) -> str:
    """🎭 Resolve a semantic teaching role to its actual source object tag."""
    m = _m(model)
    try:
        tag = m["roles"][key]
    except KeyError:
        raise ValueError(f"Unknown model role: {key}") from None
    return _find(m, tag)["tag"]


def _registry(model: dict) -> dict:
    items = {}
    for group in ("elements", "types", "spaces", "storeys", "materials"):
        for obj in model[group]:
            items[obj["tag"]] = obj
            items[obj["guid"]] = obj
    for name in ("project", "site", "building"):
        items[model[name]["tag"]] = items[model[name]["guid"]] = model[name]
    return items


def _find(model: dict, key: str) -> dict:
    try:
        return _registry(model)[key]
    except KeyError:
        raise ValueError(f"Unknown object: {key}") from None


def _elevations(model: dict) -> dict:
    return {s["id"]: s["elevation"]["value"] for s in model["storeys"]}


def _z(model: dict, value):
    return _elevations(model)[value] if isinstance(value, str) else value


def _box(model: dict) -> tuple:
    box = model["building"]["box"]
    return box["length"]["value"], box["width"]["value"], box["height"]["value"]


def _material(model: dict, obj: dict):
    tag = obj.get("material")
    if tag is None and obj.get("type"):
        tag = _find(model, obj["type"]).get("material")
    return tag if isinstance(tag,dict) else None if tag is None else _find(model,tag)


def _layers(model: dict, layerset: dict) -> list:
    out = []
    for layer in layerset["layers"]:
        density = _find(model, layer["material"])["density"]
        out.append({**deepcopy(layer), "density": deepcopy(density)})
    return out


def _slab_thickness(model: dict, slab: dict) -> float:
    return max(region["thickness"]["value"] for region in slab["geometry"]["regions"])

def _wall_thickness(model: dict, wall: dict) -> float:
    if "thickness" in wall["attributes"]:
        return wall["attributes"]["thickness"]["value"]
    return sum(l["thickness"]["value"] for l in _material(model, wall)["layers"])


def _top(model: dict, spec) -> float:
    """📌 Resolve unstationed slab soffits to the lowest occupied regional underside."""
    if isinstance(spec, (int, float)):
        return spec
    if spec == "overall":
        return _box(model)[2]
    if spec.startswith("soffit:"):
        slab = _find(model, spec[7:])
        return min(part["z0"] for part in _slab_geometry_entry(model, slab)["parts"])
    return _elevations(model)[spec]


def _round(value: float, digits: int = 9) -> float:
    return round(value, digits)


def _area(polygon: list) -> float:
    return abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(polygon, polygon[1:] + polygon[:1]))) / 2


def _merged_intervals(intervals: list) -> list:
    out = []
    for lo, hi in sorted(intervals):
        if hi - lo <= 1e-10:
            continue
        if out and lo <= out[-1][1] + 1e-10:
            out[-1][1] = max(out[-1][1], hi)
        else:
            out.append([lo, hi])
    return out


def _polygon_intervals(polygon: list, ordinate: float, axis: int = 1) -> list:
    crossings, boundary = [], []
    for a, b in zip(polygon, polygon[1:] + polygon[:1]):
        if abs(a[axis] - ordinate) < 1e-10 and abs(b[axis] - ordinate) < 1e-10:
            boundary.append(sorted([a[1 - axis], b[1 - axis]]))
        elif (a[axis] <= ordinate < b[axis]) or (b[axis] <= ordinate < a[axis]):
            crossings.append(a[1 - axis] + (ordinate - a[axis]) * (b[1 - axis] - a[1 - axis]) / (b[axis] - a[axis]))
    crossings.sort()
    return _merged_intervals(boundary + [[lo, hi] for lo, hi in zip(crossings[::2], crossings[1::2])])


def _union_area(polygons: list) -> float:
    edges = [(a, b) for p in polygons for a, b in zip(p, p[1:] + p[:1])]
    breaks = {p[0] for polygon in polygons for p in polygon}
    for i, (a, b) in enumerate(edges):
        dx, dy = b[0] - a[0], b[1] - a[1]
        for c, d in edges[i + 1:]:
            ex, ey = d[0] - c[0], d[1] - c[1]
            determinant = dx * ey - dy * ex
            if abs(determinant) < 1e-12:
                continue
            t = ((c[0] - a[0]) * ey - (c[1] - a[1]) * ex) / determinant
            u = ((c[0] - a[0]) * dy - (c[1] - a[1]) * dx) / determinant
            if 0 < t < 1 and 0 < u < 1:
                breaks.add(a[0] + t * dx)
    xs = sorted(breaks)
    return sum((hi - lo) * sum(b - a for a, b in _merged_intervals([interval for p in polygons for interval in _polygon_intervals(p, (lo + hi) / 2, 0)]))
               for lo, hi in zip(xs, xs[1:]) if hi - lo > 1e-10)


def _convex_hull(points: list) -> list:
    """🗻 Preserve every positive convex turn with adaptive exact binary64 orientation."""
    from fractions import Fraction

    points = sorted(set(tuple(map(float, p)) for p in points))
    if len(points) < 3:
        return [list(p) for p in points]
    def orientation(a,b,c):
        """🧭 Replay ambiguous determinant signs from original coordinate values."""
        if a[0]==b[0]==c[0] or a[1]==b[1]==c[1]:return 0
        left=(a[0]-c[0])*(b[1]-c[1]);right=(a[1]-c[1])*(b[0]-c[0]);det=left-right
        if math.isfinite(det) and max(abs(left),abs(right))>=2.2250738585072014e-308 and abs(det)>3.3306690738754716e-16*(abs(left)+abs(right)):return det
        return (Fraction(a[0])-Fraction(c[0]))*(Fraction(b[1])-Fraction(c[1]))-(Fraction(a[1])-Fraction(c[1]))*(Fraction(b[0])-Fraction(c[0]))
    def half(items):
        """🌓 Keep the minimal convex boundary while retaining arbitrarily small positive turns."""
        result = []
        for p in items:
            while len(result) > 1 and orientation(result[-2],result[-1],p)<=0:
                result.pop()
            result.append(p)
        return result
    return [list(p) for p in half(points)[:-1] + half(points[::-1])[:-1]]


def _polyhedron_cut(part: dict, value: float, axis: int) -> list:
    """💠 Intersect a convex closed polyhedron with a coordinate plane."""
    vertices = part_vertices(part)
    edges = {tuple(sorted((a,b))) for face in part_faces(part) for a,b in zip(face,face[1:]+face[:1])}
    coordinates = [i for i in range(3) if i!=axis]
    points = []
    for i,j in edges:
        a,b = vertices[i],vertices[j]
        if abs(a[axis]-value)<1e-10:
            points.append([a[k] for k in coordinates])
        if abs(b[axis]-value)<1e-10:
            points.append([b[k] for k in coordinates])
        if (a[axis]-value)*(b[axis]-value)<0:
            t = (value-a[axis])/(b[axis]-a[axis])
            points.append([a[k]+t*(b[k]-a[k]) for k in coordinates])
    ring = _convex_hull(points)
    return ring if len(ring)>=3 and _area(ring)>1e-14 else []


def _polyhedron_part(vertices: list, faces: list | None = None) -> dict:
    """🔷 Normalize a closed convex solid with planar nonhorizontal end caps."""
    n = len(vertices)//2
    faces = faces or [list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]
    if len(vertices)<4 or any(len(face)<3 or any(not 0<=i<len(vertices) for i in face) for face in faces):
        raise ValueError("A native polyhedron needs valid closed face indices")
    edges = {}
    for face in faces:
        for a,b in zip(face,face[1:]+face[:1]):
            edge = tuple(sorted((a,b)))
            edges[edge] = edges.get(edge,0)+1
    if any(count!=2 for count in edges.values()):
        raise ValueError("A native polyhedron must be closed")
    part = {"mesh":{"vertices":deepcopy(vertices),"faces":deepcopy(faces)},"outline":_convex_hull([[v[0],v[1]] for v in vertices]),"z0":min(v[2] for v in vertices),"z1":max(v[2] for v in vertices)}
    part["mesh"]["faces"] = part_faces(part)
    for face in part["mesh"]["faces"]:
        a,b,c = [vertices[i] for i in face[:3]]
        u,v = [b[k]-a[k] for k in range(3)],[c[k]-a[k] for k in range(3)]
        normal = [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
        scale = math.hypot(*normal)
        if not math.isfinite(scale) or scale==0 or any(abs(sum(normal[k]*(vertices[i][k]-a[k]) for k in range(3)))>1e-9*scale for i in face) or any(sum(normal[k]*(vertices[i][k]-a[k]) for k in range(3))>1e-9*scale for i in range(len(vertices))):
            raise ValueError("Native polyhedron faces must bound a convex solid")
    return part


def _polyhedron_volume(part: dict) -> float:
    """🧮 Integrate native faces locally, retaining exact determinants under cancellation."""
    vertices,faces = part_vertices(part),part_faces(part)
    origin,terms,magnitude = vertices[0],[],0.
    for face in faces:
        for i in range(1,len(face)-1):
            a,b,c = [[vertices[j][k]-origin[k] for k in range(3)] for j in (face[0],face[i],face[i+1])]
            products = [a[0]*b[1]*c[2],-a[0]*b[2]*c[1],a[1]*b[2]*c[0],-a[1]*b[0]*c[2],a[2]*b[0]*c[1],-a[2]*b[1]*c[0]]
            terms.extend(products)
            magnitude += sum(abs(v) for v in products)
    total = math.fsum(terms)
    if magnitude*128*math.ulp(1.) > abs(total)*1e-12:
        from fractions import Fraction
        points = [[Fraction(c) for c in p] for p in vertices]
        origin,total = points[0],Fraction(0)
        for face in faces:
            for i in range(1,len(face)-1):
                a,b,c = [[points[j][k]-origin[k] for k in range(3)] for j in (face[0],face[i],face[i+1])]
                total += a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0])
        return float(abs(total)/6)
    return abs(total)/6


def part_outline_at(part: dict, z: float) -> list:
    """📐 Interpolate compatible convex section rings at a clamped elevation."""
    if "mesh" in part:
        return _polyhedron_cut(part,z,2)
    if "section" in part:
        section = part["section"]
        intervals = _polygon_intervals(section["profile"],z)
        if not intervals:
            return []
        lo,hi = intervals[0]
        a,b = section["sweep"]
        return [[lo,a],[hi,a],[hi,b],[lo,b]] if section["plane"] == "XZ" else [[a,lo],[b,lo],[b,hi],[a,hi]]
    lower = part["outline"]
    upper = part.get("top_outline", lower)
    if len(lower) != len(upper) or part["z1"] <= part["z0"]:
        raise ValueError("A solid needs compatible rings and a positive height")
    if "top_outline" in part:
        if any(not math.isclose(_area(ring), _area(_convex_hull(ring)), rel_tol=1e-9, abs_tol=1e-10) or _area(ring) <= 1e-12 for ring in (lower,upper)):
            raise ValueError("Tapered section rings must be convex")
        for i in range(len(lower)):
            a,b = lower[i],lower[(i+1)%len(lower)]
            c,d = upper[i],upper[(i+1)%len(upper)]
            x,y,u,v = b[0]-a[0],b[1]-a[1],d[0]-c[0],d[1]-c[1]
            if abs(x*v-y*u) > 1e-10 or x*u+y*v <= 0:
                raise ValueError("Tapered section edges must have compatible parallel correspondence")
    if z<=part["z0"]:return deepcopy(lower)
    if z>=part["z1"]:return deepcopy(upper)
    t = min(1, max(0, (z-part["z0"])/(part["z1"]-part["z0"])))
    return [[a[0]+t*(b[0]-a[0]), a[1]+t*(b[1]-a[1])] for a,b in zip(lower,upper)]


def profile_mesh(rings: list) -> dict:
    """🔺 Triangulate a native occupied cap with conforming shared physical boundaries."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️").triangulate(rings)


def part_intervals(part: dict, value: float, axis: int = 1) -> list:
    """✂️ Subtract explicit inner contour intervals from a vertical profile cut."""
    spans = _polygon_intervals(part["outline"],value,axis)
    for hole in part.get("holes",()):
        for lo,hi in _polygon_intervals(hole,value,axis):
            spans = [(a,min(b,lo)) for a,b in spans if a < lo and min(b,lo)-a > 1e-12] + [(max(a,hi),b) for a,b in spans if b > hi and b-max(a,hi) > 1e-12]
    return sorted(spans)


def part_sections(part: dict, value: float, axis: int = 1) -> list:
    """🧶 Return all connected occupied polygons in a physical vertical cut."""
    if "section" in part or "top_outline" in part or "mesh" in part:
        ring = part_section(part,value,axis)
        return [ring] if ring else []
    return [[[a,part["z0"]],[b,part["z0"]],[b,part["z1"]],[a,part["z1"]]] for a,b in part_intervals(part,value,axis)]


def part_regions(part: dict, z: float) -> list:
    """🧩 Partition the true occupied horizontal slice, excluding explicit inner contours."""
    ring = part_outline_at(part,z)
    if not ring:
        return []
    holes = part.get("holes",())
    if not holes:
        return [ring]
    pieces = _analysis._convex_parts(ring)
    for hole in holes:
        pieces = [piece for current in pieces for piece in _analysis._subtract(current,hole,digits=12,tolerance=1e-12)]
    return pieces


def part_vertices(part: dict) -> list:
    """🧊 Exact world vertices of a prism, convex loft or planar section sweep."""
    if "mesh" in part:
        return deepcopy(part["mesh"]["vertices"])
    if part.get("holes"):
        mesh = profile_mesh([part["outline"],*part["holes"]])
        return [[*p,part[k]] for k in ("z0","z1") for p in mesh["vertices"]]
    if "section" in part:
        section = part["section"]
        return [[h,s,z] if section["plane"] == "XZ" else [s,h,z] for s in section["sweep"] for h,z in section["profile"]]
    return [[*p, part[key]] for key in ("z0", "z1") for p in part_outline_at(part,part[key])]


def part_footprint(part: dict) -> list:
    """🗺️ Plan silhouette, retaining concavity for ordinary prisms."""
    if "section" in part or "mesh" in part:
        return _convex_hull([[v[0],v[1]] for v in part_vertices(part)])
    return _convex_hull(part["outline"]+part["top_outline"]) if "top_outline" in part else deepcopy(part["outline"])


def part_elevation(part: dict, axis: int = 1) -> list:
    """🪞 Convex loft silhouette with horizontal coordinate orthogonal to axis."""
    return _convex_hull([[p[1-axis],p[2]] for p in part_vertices(part)])


def part_section(part: dict, value: float, axis: int = 1) -> list:
    """🩻 True convex loft intersection in horizontal-height coordinates."""
    if "mesh" in part:
        return _polyhedron_cut(part,value,axis)
    if part.get("holes"):
        raise ValueError("A profile with voids requires part_sections")
    vertices = part_vertices(part)
    n, points = len(vertices)//2, []
    edges = [(i,(i+1)%n) for i in range(n)] + [(n+i,n+(i+1)%n) for i in range(n)] + [(i,n+i) for i in range(n)]
    for i,j in edges:
        a,b = vertices[i],vertices[j]
        if abs(a[axis]-value) < 1e-10:
            points.append([a[1-axis],a[2]])
        if (a[axis]-value)*(b[axis]-value) < 0:
            t = (value-a[axis])/(b[axis]-a[axis])
            points.append([a[1-axis]+t*(b[1-axis]-a[1-axis]),a[2]+t*(b[2]-a[2])])
    result = _convex_hull(points)
    return result if len(result) >= 3 and _area(result) > 1e-12 else []


def _occupied_volume(parts: list) -> float:
    """🫙 Integrate occupied slice unions, including linearly tapered convex solids."""
    if any("csg"in p for p in parts):return _analytic_solids_backend().measure_union([_analytic_input(p)for p in parts])["volume"]
    levels = sorted({p[k] for p in parts for k in ("z0", "z1")} | {v[1] for p in parts if "section" in p for v in p["section"]["profile"]})
    total = 0.
    for lo,hi in zip(levels,levels[1:]):
        active = [p for p in parts if p["z0"] < (lo+hi)/2 < p["z1"]]
        if not any("top_outline" in p or "section" in p or "mesh" in p for p in active):
            total += (hi-lo)*_union_area([ring for p in active for ring in part_regions(p,(lo+hi)/2)])
            continue
        def area(z):
            return _union_area([ring for p in active for ring in part_regions(p,z)])
        def integrate(a,b,fa,fm,fb,estimate,tolerance,depth):
            mid = (a+b)/2
            left,right = area((a+mid)/2),area((mid+b)/2)
            lhs,rhs = (mid-a)*(fa+4*left+fm)/6,(b-mid)*(fm+4*right+fb)/6
            delta = lhs+rhs-estimate
            if depth <= 0 or abs(delta) <= 15*tolerance:
                return lhs+rhs+delta/15
            return integrate(a,mid,fa,left,fm,lhs,tolerance/2,depth-1)+integrate(mid,b,fm,right,fb,rhs,tolerance/2,depth-1)
        fa,fm,fb = area(lo),area((lo+hi)/2),area(hi)
        estimate = (hi-lo)*(fa+4*fm+fb)/6
        total += integrate(lo,hi,fa,fm,fb,estimate,1e-10*max(1.,abs(estimate)),15)
    return total


def _partition_volume(parts: list) -> float:
    """🪣 Integrate true native cells of an explicitly disjoint material partition."""
    volumes = []
    for part in parts:
        if "csg"in part:volumes.append(part_volume(part))
        elif any(key in part for key in ("mesh","section","top_outline")):
            volumes.append(_polyhedron_volume(part))
        elif part.get("holes"):
            stock = {"material":part.get("material",""),"role":part.get("role","quantity_cell"),**{key:deepcopy(value) for key,value in part.items() if key!="holes"}}
            holes = [{"outline":ring,"z0":part["z0"],"z1":part["z1"]} for ring in part["holes"]]
            volumes.extend(_polyhedron_volume(cell) for cell in _reference_material_parts(stock,holes))
        else:volumes.append(_area(part["outline"])*(part["z1"]-part["z0"]))
    return math.fsum(volumes)


def _length(path: list) -> float:
    return sum(math.dist(a, b) for a, b in zip(path, path[1:]))


def _prism_residuals(outline: list, cutters: list, exact: bool = False) -> list:
    """🔪 Replay ordered material cuts from source operands when computed ring topology is invalid."""
    from fractions import Fraction
    analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
    def coordinates(ring):
        return [[Fraction(x),Fraction(y)] for x,y in ring] if exact else ring
    pieces = analysis._convex_parts(coordinates(outline))
    for cutter in cutters:
        updated = []
        for current in pieces:
            updated += [piece for piece in analysis._subtract(current,coordinates(cutter["outline"]),digits=None if exact else 12,tolerance=1e-12) if _area(piece)>1e-12]
            for hole in cutter.get("holes",()):
                ring = coordinates(hole)
                clip = analysis._clip(current,ring if analysis._signed(ring)>0 else list(reversed(ring)),digits=None if exact else 9)
                if len(clip)>=3 and _area(clip)>1e-12:
                    updated.append(clip)
        pieces = updated
        if not pieces:
            break
    if not exact and 1<len(pieces)<=64:
        rings = analysis._loops(pieces,tolerance=1e-12)
        if rings and all(analysis._signed(ring)>1e-12 for ring in rings):
            if math.isclose(sum(_area(ring) for ring in rings),sum(_area(piece) for piece in pieces),rel_tol=1e-11,abs_tol=1e-12):
                pieces = rings
    if exact:
        pieces = [[[float(x),float(y)] for x,y in ring] for ring in pieces]
    if not all(analysis._simple(ring) for ring in pieces):
        if exact:
            raise ValueError("Exact material residuals cannot be published as simple coordinate rings")
        return _prism_residuals(outline,cutters,exact=True)
    return pieces


def _subtract_prisms(parts: list, cutters: list) -> list:
    """🪠 Partition material prisms around other explicitly owned solids."""
    def bounds(ring):
        return min(p[0] for p in ring),min(p[1] for p in ring),max(p[0] for p in ring),max(p[1] for p in ring)
    if any("section" in p for p in [*parts,*cutters]):
        raise ValueError("Material prism subtraction requires vertical profile prisms")
    parts = [{**p,"outline":ring,"holes":[]} for p in parts for ring in part_regions(p,(p["z0"]+p["z1"])/2)] if any(p.get("holes") for p in parts) else parts
    indexed = [(bounds(c["outline"]),c) for c in cutters if c["z1"]>c["z0"] and len(c["outline"])>=3]
    result = []
    for part in parts:
        xmin,ymin,xmax,ymax = bounds(part["outline"])
        active = [c for (a,b,d,e),c in indexed if c["z0"]<part["z1"]-1e-10 and c["z1"]>part["z0"]+1e-10 and a<xmax-1e-10 and d>xmin+1e-10 and b<ymax-1e-10 and e>ymin+1e-10]
        if not active:
            result.append(deepcopy(part))
            continue
        levels = sorted({part["z0"],part["z1"],*(max(part["z0"],min(part["z1"],c[key])) for c in active for key in ("z0","z1"))})
        for lo,hi in zip(levels,levels[1:]):
            cutters_at_z = [c for c in active if c["z0"]<(lo+hi)/2<c["z1"]]
            pieces = _prism_residuals(part["outline"],cutters_at_z)
            result += [{**deepcopy(part),"outline":ring,"z0":lo,"z1":hi} for ring in pieces]
    return result


def _arc(centre, radius, start, end, step=ARC_STEP) -> list:
    count = max(1, round(abs(end - start) / step))
    return [[_round(centre[0] + radius * math.cos(math.radians(start + (end - start) * i / count)), 6),
             _round(centre[1] + radius * math.sin(math.radians(start + (end - start) * i / count)), 6)] for i in range(count + 1)]


def _car(model: dict) -> dict:
    car = model["car"]
    return {**{key: car[key]["value"] for key in ("length", "width", "height", "turning_radius", "wall_clearance", "min_side_margin")},
            "ground": _elevations(model)["ground"]}


def _clean(points: list) -> list:
    out = []
    for point in points:
        if not out or math.dist(point, out[-1]) > 1e-9:
            out.append(point)
    if len(out) > 1 and math.dist(out[0], out[-1]) < 1e-9:
        out.pop()
    return out


def _expand(model: dict, items: list) -> list:
    out = []
    for item in items:
        if isinstance(item, dict) and "bezier" in item:
            control = item["bezier"]
            out += [[sum(math.comb(3,j)*(1-t)**(3-j)*t**j*control[j][i] for j in range(4)) for i in (0,1)] for t in [k/32 for k in range(33)]]
        elif isinstance(item, dict) and "arc" in item:
            arc = item["arc"]
            out += _arc(arc["centre"], arc["radius"], arc["start"], arc["end"])
        elif isinstance(item, dict):
            raise ValueError("Unknown source outline primitive")
        else:
            out.append(list(item))
    return _clean(out)


def _storey_of(model: dict, obj: dict) -> str:
    return obj["storey"]


def _by_class(model: dict, cls: str, predefined: str | None = None) -> list:
    return [e for e in model["elements"] if e["class"] == cls and (predefined is None or e.get("predefined_type") == predefined)]


def _revision(model: dict, obj: dict) -> int:
    return obj["metadata"]["revision"]


def element(key: str, model: dict | None = None) -> dict:
    """🔎 Return one object by GlobalId or tag with merged type psets and a resolved material."""
    m = _m(model)
    obj = deepcopy(_find(m, key))
    kind = _find(m, obj["type"]) if obj.get("type") else None
    if kind is not None or "psets" in obj:
        psets = deepcopy(kind["psets"]) if kind is not None else {}
        for name, properties in obj.get("psets", {}).items():
            psets.setdefault(name, {}).update(deepcopy(properties))
        obj["psets"] = psets
    material = _material(m, obj)
    if material is not None:
        obj["material_tag"] = material["tag"]
        obj["material"] = deepcopy(material)
        if "layers" in material:
            obj["material"]["layers"] = _layers(m, material)
            obj["material"]["total_thickness"] = _round(sum(l["thickness"]["value"] for l in material["layers"]))
    return obj


def _curtain_paths(model: dict, curtain: dict) -> list:
    """🛣️ Resolve source reference paths and independently authored physical member stations."""
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    return provider.resolve_paths(SimpleNamespace(**globals()),model,curtain)


def _opening_domain(model: dict, opening: dict) -> dict:
    """🚩 Resolve a native linear host without fabricating a wall for curtain apertures."""
    geometry=opening["geometry"]
    if sum(k in geometry for k in ("wall","slab","curtain_wall"))!=1:
        raise ValueError("An aperture has exactly one native host domain")
    if "wall" in geometry:
        host=_find(model,geometry["wall"])
        if host["class"]!="IfcWall":
            raise ValueError("A wall aperture requires its native wall host")
        path=_wall_path(model,host)
        thickness=_wall_thickness(model,host)
        side=host["geometry"].get("exterior_side",1)
    elif "curtain_wall" in geometry:
        host=_find(model,geometry["curtain_wall"])
        if host["class"]!="IfcCurtainWall":
            raise ValueError("A curtain aperture requires its native curtain host")
        paths=_curtain_paths(model,host)
        record=next((p for p in paths if p["tag"]==geometry["curtain_path"]),None)
        if record is None:
            raise ValueError("Unknown native curtain aperture path")
        path=record["reference_path"]
        thickness=_find(model,host["type"])["filling"]["frame_depth"]["value"]
        side=0
    else:
        raise ValueError("A linear aperture requires one wall or curtain host")
    return {"host":host["tag"],"host_class":host["class"],"path":path,"length":_length(path),"thickness":thickness,"base":_z(model,host["geometry"]["base_level"]),"top":_top(model,host["geometry"]["top_level"]),"side":side,"offset":host["geometry"].get("path_offset",{}).get("value",0.)}


def opening_geometry(key: str, model: dict | None = None) -> dict:
    """🚪 Resolve one finite native wall, slab or curtain aperture into the shared physical domain."""
    m=_m(model)
    opening=_find(m,key)
    if opening["class"]!="IfcOpeningElement":
        raise ValueError("Opening geometry requires an IfcOpeningElement")
    if sum(k in opening["geometry"] for k in ("wall","slab","curtain_wall"))!=1:
        raise ValueError("An aperture has exactly one native host domain")
    if "slab" in opening["geometry"]:
        return {**_slab_opening_geometry(m,opening),"host_class":"IfcSlab"}
    domain=_opening_domain(m,opening)
    values=_opening_values(m,opening)
    start,end,sill,head=[values[k] for k in ("start","end","sill","head")]
    if not -1e-9<=start<end<=domain["length"]+1e-9 or not -1e-9<=sill<head<=domain["top"]-domain["base"]+1e-9:
        raise ValueError("A native aperture must lie within its finite host path and height")
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    axis=provider.path_slice(domain['path'],start,end)
    raw=_path_bands(axis,domain['thickness'])
    host=_path_bands(domain['path'],domain['thickness'])
    shift=domain.get('offset',0.)/domain['thickness']
    for part in raw+host:
        a,b,c,d=part['outline']
        part['outline']=[[p[i]+shift*(q[i]-r[i]) for i in (0,1)] for p,q,r in ((a,d,a),(b,c,b),(c,c,b),(d,d,a))]
    parts=[]
    for part in raw:
        for band in host:
            if not _surface_bounds_overlap(_surface_bounds(part['outline']),_surface_bounds(band['outline'])):
                continue
            outline=_analysis._clip(part['outline'],band['outline'],digits=12)
            if _area(outline)>0:
                parts.append({**part,'outline':outline,'start':part['start']+start,'end':part['end']+start,'z0':domain['base']+sill,'z1':domain['base']+head})
    return {"tag":opening["tag"],"guid":opening["guid"],"class":opening["class"],"storey":opening["storey"],**domain,**values,"sill_z":domain["base"]+sill,"head_z":domain["base"]+head,"axis":axis,"parts":parts}


def _opening_values(model: dict, opening: dict) -> dict:
    a = opening["attributes"]
    if "head_level" in opening["geometry"]:
        domain = _opening_domain(model,opening)
        head = _top(model,opening["geometry"]["head_level"])-domain["base"]
    else:
        head = a["head"]["value"]
    return {"start":a["start"]["value"],"end":a["end"]["value"],"sill":a["sill"]["value"],"head":head}


def _path_bands(path: list, thickness: float, stations: list = ()) -> list:
    """🌀 Subdivide original mitered strips without folding short corner-adjacent stations."""
    lengths = [math.dist(a,b) for a,b in zip(path,path[1:])]
    if not lengths or min(lengths) <= 1e-10 or thickness <= 0:
        raise ValueError("A path wall requires a positive thickness and distinct vertices")
    normals = [[-(b[1]-a[1])/length,(b[0]-a[0])/length] for a,b,length in zip(path,path[1:],lengths)]
    miters = [normals[0]]
    for a,b in zip(normals,normals[1:]):
        denominator = 1+a[0]*b[0]+a[1]*b[1]
        if denominator < 1e-8:
            raise ValueError("A path wall cannot reverse through a cusp")
        miters.append([(a[i]+b[i])/denominator for i in (0,1)])
    miters.append(normals[-1])
    if math.dist(path[0],path[-1])<1e-9:
        a,b = normals[-1],normals[0]
        denominator = 1+a[0]*b[0]+a[1]*b[1]
        if denominator<1e-8:
            raise ValueError("A closed path wall cannot reverse through a cusp")
        miters[0] = [(a[i]+b[i])/denominator for i in (0,1)]
        miters[-1] = list(miters[0])
    left = [[p[i]+thickness/2*n[i] for i in (0,1)] for p,n in zip(path,miters)]
    right = [[p[i]-thickness/2*n[i] for i in (0,1)] for p,n in zip(path,miters)]
    result,offset = [],0.
    for index,length in enumerate(lengths):
        end = offset+length
        cuts = [offset,*sorted(set(s for s in stations if offset+1e-9<s<end-1e-9)),end]
        def section(side,station):
            """🪡 Interpolate an existing material edge while preserving original endpoint precision."""
            if station==offset:
                return list(side[index])
            if station==end:
                return list(side[index+1])
            t = (station-offset)/length
            return [side[index][i]+t*(side[index+1][i]-side[index][i]) for i in (0,1)]
        for a,b in zip(cuts,cuts[1:]):
            result.append({"outline":[section(right,a),section(right,b),section(left,b),section(left,a)],"start":a,"end":b})
        offset = end
    return result



def _wall_path(model: dict, wall: dict) -> list:
    """🔁 Preserve explicit native wall closure without changing polygon outline normalization."""
    raw = wall["geometry"]["path"]
    path = _expand(model,raw)
    endpoints = []
    for item,last in ((raw[0],False),(raw[-1],True)):
        if isinstance(item,list):
            point = item
        elif "bezier" in item:
            point = item["bezier"][-1 if last else 0]
        else:
            arc = item["arc"]
            angle = math.radians(arc["end" if last else "start"])
            point = [arc["centre"][0]+arc["radius"]*math.cos(angle),arc["centre"][1]+arc["radius"]*math.sin(angle)]
        endpoints.append(point)
    if len(path)>2 and math.dist(*endpoints)<1e-10 and math.dist(path[0],path[-1])>1e-10:
        path.append(list(path[0]))
    return path


def path_wall_geometry(model: dict, wall: dict) -> dict:
    """🌉 Resolve a continuous curved wall, hosted station apertures and a sloping crest into occupied solids."""
    path = _wall_path(model,wall)
    length,base,body_top = _length(path),_z(model,wall["geometry"]["base_level"]),_top(model,wall["geometry"]["top_level"])
    thickness = _wall_thickness(model,wall)
    openings = []
    for opening in _by_class(model,"IfcOpeningElement"):
        if opening["geometry"].get("wall") != wall["tag"]:
            continue
        values = _opening_values(model,opening)
        if not 0 <= values["start"] < values["end"] <= length or not (0 <= values["sill"] < values["head"] and base+values["head"] <= body_top):
            raise ValueError("Opening station and height intervals must lie inside their path wall")
        openings.append({**values,"tag":opening["tag"],"guid":opening["guid"],"sill_z":base+values["sill"],"head_z":base+values["head"]})
    if any(a["start"] < b["end"] and b["start"] < a["end"] and a["sill_z"] < b["head_z"] and b["sill_z"] < a["head_z"] for i,a in enumerate(openings) for b in openings[i+1:]):
        raise ValueError("Path wall opening station intervals must not overlap")
    bands = _path_bands(path,thickness,[o[k] for o in openings for k in ("start","end")])
    shift = wall["geometry"].get("path_offset",{}).get("value",0.)/thickness
    for band in bands:
        a,b,c,d = band["outline"]
        band["outline"] = [[p[i]+shift*(q[i]-r[i]) for i in (0,1)] for p,q,r in ((a,d,a),(b,c,b),(c,c,b),(d,d,a))]
    rings = [b["outline"] for b in bands]
    if not math.isclose(sum(_area(r) for r in rings),_union_area(rings),rel_tol=1e-8,abs_tol=1e-9):
        raise ValueError("A path wall band must not overlap itself")
    parts,voids,gross,cavities,gross_cavities,opening_material = [],[],[],[],[],[]
    zs = sorted({base,body_top,*(o[k] for o in openings for k in ("sill_z","head_z"))})
    material = _material(model,wall)
    layers = material.get("layers") if material else None
    layers = layers or [{"id":"body","material":material["tag"] if material else wall["tag"],"thickness":{"value":thickness}}]
    if not math.isclose(sum(layer["thickness"]["value"] for layer in layers),thickness,abs_tol=1e-9):
        raise ValueError("A curved wall layer set must span its envelope")
    side = wall["geometry"].get("exterior_side",1)
    apertures={o['tag']:opening_geometry(o['tag'],model) for o in openings}
    for band in bands:
        for z0,z1 in zip(zs,zs[1:]):
            part={**band,'z0':z0,'z1':z1,'role':'body'}
            cuts=[]
            for opening in openings:
                if not opening['sill_z']<(z0+z1)/2<opening['head_z']:
                    continue
                for aperture in apertures[opening['tag']]['parts']:
                    if not _surface_bounds_overlap(_surface_bounds(band['outline']),_surface_bounds(aperture['outline'])):
                        continue
                    ring=_analysis._clip(band['outline'],aperture['outline'],digits=12)
                    if _area(ring)>0:
                        cuts.append((opening['tag'],ring))
                        voids.append({**part,'outline':ring,'opening':opening['tag']})
            offset=0.
            a,b,c,d=band['outline']
            for layer in layers:
                lo,hi=offset/thickness,(offset+layer['thickness']['value'])/thickness
                offset+=layer['thickness']['value']
                if side==1:
                    lo,hi=1-hi,1-lo
                ring=[[p[i]+fraction*(q[i]-p[i]) for i in (0,1)] for p,q,fraction in ((a,d,lo),(b,c,lo),(b,c,hi),(a,d,hi))]
                resolved={**part,'outline':ring,'material':layer['material'],'layer':layer['id']}
                (gross_cavities if layer.get('is_void') else gross).append(resolved)
                remaining=[ring]
                for tag,cutter in cuts:
                    if not layer.get('is_void'):
                        removed=_analysis._clip(ring,cutter,digits=12)
                        if _area(removed)>0:
                            opening_material.append({**resolved,'outline':removed,'opening':tag})
                    remaining=[piece for subject in remaining for piece in _analysis._subtract(subject,cutter,digits=12,tolerance=1e-12)]
                target=cavities if layer.get('is_void') else parts
                target.extend({**resolved,'outline':piece} for piece in remaining)
    cap = wall["geometry"].get("cap")
    top = body_top
    if cap:
        rise,crest = cap["rise"]["value"],cap["crest_width"]["value"]
        if not 0 < crest < thickness:
            raise ValueError("A wall crest width must lie inside its band")
        top += rise
        for band in bands:
            a,b,c,d = band["outline"]
            ratio = 1-crest/thickness
            upper = [[p[i]+ratio*(q[i]-p[i]) for i in (0,1)] for p,q in ((a,d),(b,c))]+[c,d] if cap["side"]==1 else [a,b]+[[p[i]+ratio*(q[i]-p[i]) for i in (0,1)] for p,q in ((c,b),(d,a))]
            parts.append({**band,"top_outline":upper,"z0":body_top,"z1":top,"material":material["tag"] if material else wall["tag"],"role":"cap"})
    caps = [p for p in parts if p["role"]=="cap"]
    gross += caps
    interface_reference = deepcopy(parts) if wall["geometry"].get("interface_owners") or _wall_has_junction(model,wall) else None
    interfaces = _wall_interface_cutters(model,wall)
    parts,displaced,_ = _wall_owned_parts(model,parts,wall["geometry"].get("interface_owners",()),interfaces)
    cavities,_,_ = _wall_owned_parts(model,cavities,wall["geometry"].get("interface_owners",()),interfaces)
    interface_displaced = displaced if interface_reference is not None else 0.
    def volume(collection):
        return sum(_polyhedron_volume(p) if "mesh" in p else (p["z1"]-p["z0"])*(_area(p["outline"])+4*_area(part_outline_at(p,(p["z0"]+p["z1"])/2))+_area(part_outline_at(p,p["z1"])))/6 for p in collection)
    for opening in openings:
        opening["parts"] = [p for p in voids if p["opening"]==opening["tag"]]
        opening["material_parts"] = [p for p in opening_material if p["opening"]==opening["tag"]]
        opening["material_volume"] = volume(opening["material_parts"])
        opening['p0']=apertures[opening['tag']]['axis'][0]
        opening['p1']=apertures[opening['tag']]['axis'][-1]
    return {"tag":wall["tag"],"guid":wall["guid"],"storey":wall["storey"],"path":path,"base":base,"body_top":body_top,"top":top,"thickness":thickness,"length":length,"openings":openings,"parts":parts,"gross_parts":gross,"cavity_parts":cavities,"gross_material_volume":volume(gross),"gross_envelope_volume":volume([{**b,"z0":base,"z1":body_top} for b in bands]+caps),"gross_cavity_volume":volume(gross_cavities),"cavity_volume":volume(cavities),"opening_material_volume":volume(opening_material),"joint_displacement_volume":displaced,"solid_volume":volume(parts),**({"interface_reference_parts":interface_reference,"interface_displacement_volume":interface_displaced} if interface_reference is not None else {})}


def _wall_uses_path(wall: dict) -> bool:
    """🛰️ Distinguish native path geometry independently of an IFC wall classification."""
    path = wall["geometry"]["path"]
    return len(path)!=2 or any(not isinstance(point,list) for point in path)


def _wall_has_junction(model: dict, wall: dict) -> bool:
    """🪢 Distinguish declared material ownership from unrelated geometric proximity."""
    return any((relation['class']=='IfcRelConnectsPathElements' or relation['class']=='IfcRelConnectsElements' and relation.get('connection',{}).get('kind')=='material_junction') and wall['tag'] in [relation['relating'],*relation['related']] for relation in model.get('relations',()))


def _wall_junction_backend():
    """🔌 Resolve explicit wall layer continuity behind the first-party geometry interface."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️facade.🔗️junctions")


def _wall_geometry(model: dict, wall: dict, resolve_junctions=True, junctions=None, progress=None, cancel=None) -> dict:
    """🧂 Preserve full wall reference stock while assigning declared common material once."""
    if cancel and cancel():raise InterruptedError("Wall construction cancelled")
    backend = _wall_junction_backend()
    api = SimpleNamespace(**globals())
    state = junctions if junctions is not None else backend.context(api,model) if resolve_junctions else None
    result = deepcopy(state['raw'][wall['tag']]) if state is not None and wall['tag'] in state['raw'] else _wall_geometry_unjoined(model,wall)
    if not resolve_junctions or wall['tag'] not in state['index']:return result
    state['raw'][wall['tag']] = deepcopy(result)
    result['junction_reference_parts'] = deepcopy(result['parts'])
    result.setdefault('interface_reference_parts',deepcopy(result['parts']))
    result['parts'],removed = backend.partition(api,model,wall,result['parts'],state,progress=progress,cancel=cancel)
    if 'cavity_parts' in result:result['cavity_parts'],_ = backend.partition(api,model,wall,result['cavity_parts'],state,air=True,progress=progress,cancel=cancel)
    result['junction_displacement_volume'] = removed
    result['joint_displacement_volume'] += removed
    result['solid_volume'] = sum(part_volume(part) for part in result['parts'])
    if 'cavity_parts' in result:result['cavity_volume'] = sum(part_volume(part) for part in result['cavity_parts'])
    for layer in result.get('layers',()):
        layer['volume'] = sum(part_volume(part) for part in result['parts']+result.get('cavity_parts',[]) if part.get('layer')==layer['id'] and part.get('region')==layer.get('region'))
    if 'surface_finishes' in result:
        materials = [_find(model,region['material']) for region in wall['geometry'].get('layer_regions',())] or [_material(model,wall)]
        surface = surface_finishes().wall_regions(wall,result,[{'layers':material.get('layers',[])} for material in materials],lambda tag:_find(model,tag),_wall_station)
        result['surface_finishes'] = surface_finishes().summaries(surface,result['parts'],part_vertices,part_faces)
    return result


def _wall_geometry_unjoined(model: dict, wall: dict) -> dict:
    path = wall["geometry"]["path"]
    if _wall_uses_path(wall):
        return path_wall_geometry(model,wall)
    base = _z(model, wall["geometry"]["base_level"])
    top = _top(model, wall["geometry"]["top_level"])
    length = math.dist(path[0], path[1])
    direction = [(path[1][0] - path[0][0]) / length, (path[1][1] - path[0][1]) / length]
    openings = []
    for opening in [e for e in _by_class(model, "IfcOpeningElement") if e["geometry"].get("wall") == wall["tag"]]:
        v = _opening_values(model, opening)
        window = next((e for e in model["elements"] if e["class"] in ("IfcWindow", "IfcDoor") and e["geometry"].get("opening") == opening["tag"]), None)
        p0 = [path[0][0] + direction[0] * v["start"], path[0][1] + direction[1] * v["start"]]
        p1 = [path[0][0] + direction[0] * v["end"], path[0][1] + direction[1] * v["end"]]
        openings.append({"tag": opening["tag"], "guid": opening["guid"], "window_tag": window["tag"] if window else None, "window_guid": window["guid"] if window else None,
                         "fill_tag": window["tag"] if window else None, "fill_guid": window["guid"] if window else None, "fill_class": window["class"] if window else None, "hinge": "start", "swing": 1,
                         "start": v["start"], "end": v["end"], "sill": v["sill"], "head": v["head"],
                         "sill_z": _round(base + v["sill"]), "head_z": _round(base + v["head"]), "p0": [_round(c, 6) for c in p0], "p1": [_round(c, 6) for c in p1]})
    out = {"tag": wall["tag"], "guid": wall["guid"], "start": list(path[0]), "end": list(path[1]), "length": _round(length),
            "thickness": _round(_wall_thickness(model, wall)), "base": base, "top": _round(top), "openings": openings, "storey": wall["storey"],
            "is_external": element(wall["tag"], model).get("psets", {}).get("Pset_WallCommon", {}).get("IsExternal", {}).get("value", False)}

    out["layers"], out["parts"], out["joint_displacement_volume"], out["solid_volume"] = _wall_parts(model, wall, out)
    return out


_WALL_CUTTERS = {}
_WALL_OWNERSHIP = {}
_WALL_INTERFACES = {}


def _geometry_source_key(model: dict) -> bytes:
    """🗝️ Bind private geometry caches to complete source content without retaining duplicated source strings."""
    return hashlib.sha256(json.dumps(model,sort_keys=True,separators=(",",":"),ensure_ascii=False,allow_nan=False,default=_native_cache_scalar).encode()).digest()


def _wall_cutters(model: dict) -> list:
    """🔨 Resolve explicitly owned concrete and filling solids for material interfaces."""
    key = (_geometry_source_key(model),geometry_producers())
    if key not in _WALL_CUTTERS:
        parts = []
        for obj in model["elements"]:
            reserved = []
            if obj["class"] == "IfcColumn":
                column = column_geometry(model,obj)
                owned = column["parts"]
                metrics = column.get("section_metrics",owned)
                for index,part in enumerate(metrics):
                    for hole in part.get("holes",()):
                        air = {"outline":_analysis._ccw(hole),"z0":part["z0"],"z1":part["z1"],"role":"column_bore_air_reservation"}
                        reserved += _column_end_parts(air,obj["geometry"]["end_plane"],obj["geometry"]["axis"]) if "end_plane" in obj["geometry"] and index==len(metrics)-1 else [air]
            elif obj["class"] == "IfcBeam":
                solid = _solid_geometry(model,obj)
                owned = solid["parts"]+solid["shared_parts"]
            elif obj["class"] == "IfcSlab":
                solid = _slab_geometry_entry(model,obj)
                owned = solid["parts"]+solid["shared_parts"]
            elif obj["class"] in ("IfcWindow","IfcDoor"):
                owned = _filling_geometry(model,obj)["parts"]
            elif obj["class"] == "IfcCurtainWall":
                owned = _curtain_filling_geometry(model,obj)["parts"]
            else:
                continue
            parts += [{**{key:deepcopy(part[key]) for key in ("outline","z0","z1","mesh","section","top_outline","holes","material","role") if key in part},"owner":obj["tag"],"_bounds":_material_bounds(part)} for part in owned+reserved]
        _WALL_CUTTERS[key] = parts
        if len(_WALL_CUTTERS) > 8:
            _WALL_CUTTERS.pop(next(iter(_WALL_CUTTERS)))
    return _WALL_CUTTERS[key]


def _wall_owned_parts(model: dict, parts: list, excluded_owners: list | tuple = (), native_interfaces: list | tuple = ()) -> tuple:
    """🏠 Keep brick and plaster outside other members' occupied material."""
    if not parts:
        return [],0.,0.
    boxes = [_material_bounds(part) for part in parts]
    box = tuple((min(p[k][0] for p in boxes),max(p[k][1] for p in boxes)) for k in range(3))
    cutters = []
    for cutter in _wall_cutters(model):
        if cutter.get("owner") in excluded_owners and cutter.get("role")!="column_bore_air_reservation":continue
        other = cutter.get("_bounds") or _material_bounds(cutter)
        if all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,other)):cutters.append(cutter)
    extra = [c for c in native_interfaces if all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,_material_bounds(c)))]
    backend = surface_finishes()
    key = (hashlib.sha256(json.dumps([parts,cutters,extra],sort_keys=True,separators=(",",":"),default=_native_cache_scalar).encode()).hexdigest(),_subtract_prisms,_subtract_native_material,_subtract_convex_material,_native_material_cells,_material_bounds,_polyhedron_part,part_vertices,part_faces,part_outline_at,_polyhedron_volume,backend.clip_stock,backend.publish_stock,backend.stock_separated,_native_public_value)
    if key not in _WALL_OWNERSHIP:
        native = [c for c in cutters if any(key in c for key in ("mesh","section","top_outline"))]
        vertical = [c for c in cutters if not any(key in c for key in ("mesh","section","top_outline"))]
        prisms = [p for p in parts if not any(key in p for key in ("mesh","section","top_outline"))]
        shaped = [p for p in parts if any(key in p for key in ("mesh","section","top_outline"))]
        owned = []
        if prisms:
            if native or extra:
                owned,_ = _subtract_native_material(prisms,cutters+extra)
            else:
                owned = _subtract_prisms(prisms,vertical)
        if shaped:
            resolved,_ = _subtract_native_material(shaped,cutters+extra)
            owned += resolved
        volume = sum(_polyhedron_volume(p) for p in owned)
        before = sum(_polyhedron_volume(p) for p in parts)
        _WALL_OWNERSHIP[key] = owned,max(0.,before-volume),volume
        if len(_WALL_OWNERSHIP) > 256:
            _WALL_OWNERSHIP.pop(next(iter(_WALL_OWNERSHIP)))
    owned,displaced,volume = _WALL_OWNERSHIP[key]
    return deepcopy(owned),displaced,volume


def _ramp_stringer_reference_stock(model: dict, obj: dict) -> dict:
    """🛠️ Preserve the attested stringer depth below the actual source-affine structural upper datum."""
    g = obj['geometry']['interface_construction']
    if g['kind']!='ramp_stringer':raise ValueError('A stringer interface requires its native ramp construction')
    q = _source_fraction
    flight = _find(model,g['flight'])
    if flight['class']!='IfcRampFlight':raise ValueError('A stringer requires a native flight')
    stock = _ramp_reference_stock(model,flight);vertices = stock['mesh']['vertices']
    ends = sorted({p[1]:p[2] for p in vertices[4:]}.items());(y0,r0),(y1,r1) = ends
    width,depth = q(g['width']),q(g['depth'])
    if g['side']=='right':x0 = max(p[0] for p in vertices);x1 = x0+width
    else:x1 = min(p[0] for p in vertices);x0 = x1-width
    return {'mesh':{'vertices':[[x0,y0,r0-depth],[x1,y0,r0-depth],[x1,y1,r1-depth],[x0,y1,r1-depth],[x0,y0,r0],[x1,y0,r0],[x1,y1,r1],[x0,y1,r1]],'faces':[[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,4+(i+1)%4,4+i]for i in range(4)]},'material':_material(model,obj)['tag'],'role':'original_ramp_stringer'}


def _guard_reference_stocks(model: dict, obj: dict) -> list:
    """🥅 Construct original guard bands from source-affine flight controls before material ownership cuts."""
    g = obj['geometry']['interface_construction']
    if g['kind']!='ramp_guard_bands':raise ValueError('A guard interface requires native ramp band construction')
    q = _source_fraction
    ramp = _find(model,g['ramp'])
    if ramp['class']!='IfcRamp':raise ValueError('A guard construction requires a native ramp')
    faces = [[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,4+(i+1)%4,4+i] for i in range(4)]
    result = []
    def make(x0,x1,y0,y1,b0,b1,t0,t1,role):
        result.append({'mesh':{'vertices':[[x0,y0,b0],[x1,y0,b0],[x1,y1,b1],[x0,y1,b1],[x0,y0,t0],[x1,y0,t0],[x1,y1,t1],[x0,y1,t1]],'faces':deepcopy(faces)},'material':'MAT-RC','role':role})
    flights = {}
    for tag in {band['flight'] for band in g['bands']}|({g['arrival_upstand']['flight']} if 'arrival_upstand' in g else set()):
        flight = _find(model,tag)
        if flight['class']!='IfcRampFlight' or flight['geometry']['ramp']!=ramp['tag']:raise ValueError('A guard band requires a flight of its declared ramp')
        source = flight['geometry'];top = _ramp_reference_stock(model,flight)['mesh']['vertices'][4:]
        lo,hi = q(_z(model,source['from_level'])),q(_z(model,source['to_level']))
        walking = [lo+(hi-lo)*q(v) for v in source['portion']]
        by_y = {p[1]:p[2] for p in top}
        ends = sorted((q(p[1]),z,by_y[q(p[1])]) for p,z in zip(source['path'],walking))
        flights[tag] = ends
    width,height,cap,over = [q(g[key]) for key in ('core_width','core_height','cap_height','cap_overhang')]
    if 'arrival_upstand' in g:
        up = g['arrival_upstand'];(y0,_,r0),(y1,_,r1) = flights[up['flight']];x,w,h = q(up['axis_x']),q(up['width']),q(up['height'])
        make(x-w/2,x+w/2,y0,y1,r0,r1,r0+h,r1+h,'original_arrival_upstand')
    for band in g['bands']:
        (y0,w0,r0),(y1,w1,r1) = flights[band['flight']];x = q(band['axis_x'])
        make(x-width/2,x+width/2,y0,y1,r0,r1,w0+height,w1+height,'original_guard_core')
        make(x-width/2-over,x+width/2+over,y0,y1,w0+height,w1+height,w0+height+cap,w1+height+cap,'original_guard_cap')
    if g['landing_returns']:
        rg = ramp['geometry'];mid = next(p for p in rg['landings'] if p['ownership']=='ramp_component')
        walk = [[q(v) for v in p] for p in _expand(model,mid['walking_outline'])];stock = [[q(v) for v in p] for p in _expand(model,mid['outline'])]
        sx0,sx1 = min(p[0] for p in stock),max(p[0] for p in stock);wx0,wx1 = min(p[0] for p in walk),max(p[0] for p in walk)
        y0,wy1,sy1 = min(p[1] for p in walk),max(p[1] for p in walk),max(p[1] for p in stock)
        lo,hi = q(_z(model,rg['from_level'])),q(_z(model,rg['to_level']));floor = lo+(hi-lo)*q(mid['portion']);rc = floor-q(rg['finish_thickness']['value'])
        for a,b in ((sx0,wx0),(wx1,sx1)):
            make(a,b,y0,wy1,rc,rc,floor+height,floor+height,'original_landing_side_core')
            make(a-over,b+over,y0,wy1,floor+height,floor+height,floor+height+cap,floor+height+cap,'original_landing_side_cap')
        make(sx0,sx1,wy1,sy1,rc,rc,floor+height,floor+height,'original_landing_end_core')
        make(sx0-over,sx1+over,wy1-over,sy1+over,floor+height,floor+height,floor+height+cap,floor+height+cap,'original_landing_end_cap')
    return result


def _interface_owner_parts(model: dict, key: str, reference: bool = False) -> list:
    """🪅 Resolve exclusive native interface material without aggregating aliases or querying the whole building."""
    owner = _find(model,key)
    if reference:
        if owner['geometry'].get('kind')=='ramp_finish':return _ramp_finish_reference_parts(model,owner)
        if 'interface_construction' in owner['geometry']:
            if owner['geometry']['interface_construction']['kind']=='ramp_stringer':return [_ramp_stringer_reference_stock(model,owner)]
            return _guard_reference_stocks(model,owner)
        if owner['class']=='IfcRampFlight':return [_ramp_reference_stock(model,owner)]
        if owner['class'] in ('IfcColumn','IfcBeam','IfcFooting'):return _interface_reference_parts(model,key)
    if owner["class"]=="IfcRamp":
        parts = _native_public_value(_ramp_owned_reference_parts(model,owner))
    elif owner["class"]=="IfcRampFlight":
        parts = _ramp_geometry(model,owner)["parts"]
    elif owner["class"]=="IfcColumn":
        parts = column_geometry(model,owner)["parts"]
    elif owner["class"]=="IfcSlab":
        parts = _slab_geometry_entry(model,owner)["parts"]
    elif owner["class"]=="IfcWall":
        parts = _wall_geometry(model,owner)["parts"]
    elif owner["class"]in ("IfcWindow","IfcDoor"):
        parts = _filling_geometry(model,owner)["parts"]
    elif owner["class"]=="IfcCurtainWall":
        parts = _curtain_filling_geometry(model,owner)["parts"]
    elif owner["class"]!="IfcWall" and "parts" in owner["geometry"]:
        parts = _solid_geometry(model,owner,resolve_embedded=False)["parts"]
    else:
        raise ValueError("An interface owner requires independently owned native material")
    return deepcopy(parts)


def _material_bounds(part: dict) -> tuple:
    """🔲 Bound actual native vertices without replacing occupied material by its envelope."""
    if "csg"in part or(part.get("profile")and"mesh"not in part):return _analytic_bounds(part)
    vertices = part_vertices(part) if any(key in part for key in ("mesh","section","top_outline")) else [[*p,z] for z in (part["z0"],part["z1"]) for p in part["outline"]]
    return tuple((min(p[k] for p in vertices),max(p[k] for p in vertices)) for k in range(3))


@lru_cache(maxsize=256)
def _native_profile_regions(outline: tuple, holes: tuple = ()) -> tuple:
    """🥞 Cache immutable exact planar material regions without building any three-dimensional render or model cells."""
    from fractions import Fraction
    def side(a,b,c):
        return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
    def clean(points):
        ring = []
        for point in points:
            if not ring or point!=ring[-1]:ring.append(point)
        if len(ring)>1 and ring[0]==ring[-1]:ring.pop()
        while len(ring)>2:
            index = next((i for i,p in enumerate(ring) if side(ring[i-1],p,ring[(i+1)%len(ring)])==0),None)
            if index is None:break
            ring.pop(index)
        return ring
    rings = [clean([tuple(Fraction(v) for v in p) for p in ring]) for ring in [outline,*holes]]
    def area(ring):
        return sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
    if any(len(ring)<3 or area(ring)==0 for ring in rings):
        raise ValueError("Native material profiles require finite occupied rings")
    outer = rings[0] if area(rings[0])>0 else list(reversed(rings[0]))
    if len(rings)==1 and all(side(outer[i-1],p,outer[(i+1)%len(outer)])>0 for i,p in enumerate(outer)):
        profiles = [outer]
    else:
        stations = sorted({p[0] for ring in rings for p in ring})
        edges = [(a,b) for ring in rings for a,b in zip(ring,ring[1:]+ring[:1])]
        def at(edge,x):
            a,b = edge
            return a[1]+(b[1]-a[1])*(x-a[0])/(b[0]-a[0])
        profiles = []
        for lo,hi in zip(stations,stations[1:]):
            middle = (lo+hi)/2
            active = sorted((edge for edge in edges if min(edge[0][0],edge[1][0])<middle<max(edge[0][0],edge[1][0])),key=lambda edge:at(edge,middle))
            if len(active)%2:raise ValueError("Native material profile boundaries must form closed occupied intervals")
            for lower,upper in zip(active[::2],active[1::2]):
                ring = clean([(lo,at(lower,lo)),(hi,at(lower,hi)),(hi,at(upper,hi)),(lo,at(upper,lo))])
                if len(ring)>=3 and area(ring)!=0:profiles.append(ring if area(ring)>0 else list(reversed(ring)))
    return tuple(tuple(tuple(p)for p in ring)for ring in profiles)


def _native_material_cells(part: dict) -> list:
    """🧈 Resolve real convex material cells with exact profile predicates and no coordinate snapping."""
    if "csg"in part:raise ValueError("Analytic material retains semantic authority; display cells are not physical source geometry")
    metadata = {k:deepcopy(v) for k,v in part.items() if k not in ("outline","top_outline","holes","z0","z1","mesh","section")}
    if any(key in part for key in ("mesh","section","top_outline")):
        return [{**metadata,**_polyhedron_part(part_vertices(part),part_faces(part))}]
    profiles = _native_profile_regions(tuple(tuple(p)for p in part["outline"]),tuple(tuple(tuple(p)for p in ring)for ring in part.get("holes",())))
    result = []
    for ring in profiles:
        cell = {"outline":[list(p) for p in ring],"z0":part["z0"],"z1":part["z1"]}
        result.append({**metadata,**_polyhedron_part(part_vertices(cell),part_faces(cell))})
    return result


def _native_cache_scalar(value):
    """🔩 Encode exact private coordinate scalars losslessly without casting before geometry operations."""
    from fractions import Fraction
    if isinstance(value,Fraction):return {"$rational":[str(value.numerator),str(value.denominator)]}
    raise TypeError(f"Unsupported native cache scalar: {type(value).__name__}")


def _native_public_value(value):
    """📨 Publish isolated Float64 JSON values only after every operation on the private coordinates."""
    from fractions import Fraction
    if isinstance(value,Fraction):return float(value)
    if isinstance(value,dict):return {key:_native_public_value(item) for key,item in value.items()}
    if isinstance(value,list):return [_native_public_value(item) for item in value]
    if isinstance(value,tuple):return tuple(_native_public_value(item) for item in value)
    return deepcopy(value)


def _subtract_native_material(parts: list, cutters: list) -> tuple:
    """🧹 Keep exclusive material outside exact native stock planes, preserving layer and face identity."""
    if any("csg"in p for p in [*parts,*cutters]):
        result=[cell for part in parts for cell in _reference_material_parts(part,cutters)]
        removed=max(0.,_occupied_volume(parts)-_occupied_volume(result))
        return _native_public_value(result),removed
    if not parts or not cutters:
        return _native_public_value(parts),0.
    backend = surface_finishes()
    key = (hashlib.sha256(json.dumps([parts,cutters],sort_keys=True,separators=(",",":"),default=_native_cache_scalar).encode()).hexdigest(),_subtract_convex_material,_native_material_cells,_material_bounds,_polyhedron_part,part_vertices,part_faces,part_outline_at,_polyhedron_volume,backend.clip_stock,backend.publish_stock,backend.stock_separated,_native_public_value)
    if key not in _WALL_INTERFACES:
        indexed = [(_material_bounds(c),c) for c in cutters]
        normalized = {}
        result = []
        for part in parts:
            box = _material_bounds(part)
            active = [i for i,(other,c) in enumerate(indexed) if all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,other))]
            if not active:
                result.append(deepcopy(part))
                continue
            for i in active:
                if i not in normalized:normalized[i] = _native_material_cells(indexed[i][1])
            for cell in _native_material_cells(part):
                stock = {**cell,"material":part["material"],"role":part.get("role","owned_material")}
                result += _subtract_convex_material(stock,[c for i in active for c in normalized[i]])
        def volume(items):
            return sum(_polyhedron_volume(p) for p in items)
        _WALL_INTERFACES[key] = _native_public_value(result),max(0.,volume(parts)-volume(result))
        if len(_WALL_INTERFACES)>128:
            _WALL_INTERFACES.pop(next(iter(_WALL_INTERFACES)))
    return deepcopy(_WALL_INTERFACES[key])


def _declared_interfaces(model: dict, obj: dict, reference: bool = False, structural: bool = True) -> list:
    """🤝 Resolve explicit independent physical owners without aggregating child or borrowed aliases."""
    direct=obj["geometry"].get("interface_owners",());nested=obj["geometry"].get("ramp_finish",{}).get("interface_owners",())
    if any(len(values)!=len(set(values))for values in (direct,nested)):
        raise ValueError("Structural interfaces require distinct independent owners")
    refs=list(dict.fromkeys([*direct,*nested]))
    classes={"IfcColumn","IfcBeam","IfcSlab","IfcFooting","IfcRamp","IfcRampFlight","IfcRailing","IfcBuildingElementProxy"}
    if not structural:classes.update(("IfcWall","IfcWindow","IfcDoor","IfcCurtainWall","IfcCovering","IfcChimney","IfcFurnishingElement"))
    if len(refs)!=len(set(refs)) or obj["tag"]in refs:
        raise ValueError("Structural interfaces require distinct independent owners")
    result=[]
    for tag in refs:
        owner=_find(model,tag)
        if owner["class"]not in classes:
            raise ValueError("A declared material interface requires an independent structural product")
        result.extend({**part,"owner":owner["tag"],"owner_guid":owner["guid"],"owner_class":owner["class"]}for part in _interface_owner_parts(model,tag,reference=reference))
    return result


def _declared_structural_interfaces(model: dict, obj: dict, reference: bool = False) -> list:
    """🗼 Restrict structural receiving interfaces to declared independent structural products."""
    return _declared_interfaces(model,obj,reference,True)


def _declared_material_interfaces(model: dict, obj: dict, reference: bool = False) -> list:
    """🪝 Clip receiving finish material against explicitly declared independent physical material."""
    return _declared_interfaces(model,obj,reference,False)


def _wall_interface_cutters(model: dict, wall: dict) -> list:
    """🛡️ Resolve independently declared source-authoritative material before constructing a wall stock."""
    return _declared_structural_interfaces(model,wall,reference=True)


def _wall_interface_parts(model: dict, wall: dict, parts: list) -> tuple:
    """📎 Reserve additional source-authoritative native material in an inferred wall."""
    return _subtract_native_material(parts,_wall_interface_cutters(model,wall))


def _wall_station(spec: dict, geometry: dict) -> float:
    """🚏 Resolve material-band stations from the wall envelope or an actual host aperture."""
    datum = spec["datum"]
    if datum in ("base","top"):
        value = geometry[datum]
    else:
        opening = next((o for o in geometry["openings"] if o["tag"]==spec["opening"]),None)
        if opening is None:
            raise ValueError("A wall-band station must reference an opening in that wall")
        value = opening[datum+"_z"]
    return value+spec["offset"]["value"]


def surface_finishes():
    """🎨 Resolve first-party material-face identities behind the native model interface."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🐍️")


def _wall_parts(model: dict, wall: dict, geometry: dict) -> tuple:
    """🎀 Partition opening-linked material bands against apertures and physical member ownership."""
    g = geometry
    number = _source_fraction if wall["geometry"].get("interface_owners") else lambda value:value
    dx,dy = [(number(g["end"][i])-number(g["start"][i]))/number(g["length"]) for i in (0,1)]
    bands = []
    if "layer_regions" in wall["geometry"]:
        for region in wall["geometry"]["layer_regions"]:
            material = _find(model,region["material"])
            if "layers" not in material:
                raise ValueError("A regional wall material must be a layer set")
            bands.append({"tag":region["tag"],"z0":_wall_station(region["from"],g),"z1":_wall_station(region["to"],g),"layers":material["layers"]})
        bands.sort(key=lambda band:band["z0"])
        if len({b["tag"] for b in bands})!=len(bands) or not math.isclose(bands[0]["z0"],g["base"],abs_tol=1e-9) or not math.isclose(bands[-1]["z1"],g["top"],abs_tol=1e-9):
            raise ValueError("Material regions must uniquely span the full wall height")
        if any(b["z1"]<=b["z0"] for b in bands) or any(not math.isclose(a["z1"],b["z0"],abs_tol=1e-9) for a,b in zip(bands,bands[1:])):
            raise ValueError("Material regions must partition the wall without gaps or overlaps")
        side = wall["geometry"]["exterior_side"]
    else:
        material = _material(model,wall)
        if material is None or "layers" not in material:
            layers = [{"id":"body","material":material["tag"] if material else wall["tag"],"thickness":{"value":g["thickness"]}}]
            side = 1
        else:
            layers,side = material["layers"],wall["geometry"]["exterior_side"]
        bands = [{"tag":"body","z0":g["base"],"z1":g["top"],"layers":layers}]
    if not math.isclose(max(sum(l["thickness"]["value"] for l in b["layers"]) for b in bands),g["thickness"],abs_tol=1e-9):
        raise ValueError("Wall envelope thickness must equal its largest material region")
    openings = g["openings"]
    surface = surface_finishes().wall_regions(wall,g,bands,lambda tag:_find(model,tag),_wall_station)
    xs = sorted({0.,g["length"],*(max(0.,min(g["length"],o[k])) for o in openings for k in ("start","end")),*(region[k] for region in surface for k in ("start","end"))})
    resolved,parts,voids = [],[],[]
    for band in bands:
        zs = sorted({band["z0"],band["z1"],*(max(band["z0"],min(band["z1"],o[k])) for o in openings for k in ("sill_z","head_z")),*(max(band["z0"],min(band["z1"],region[k])) for region in surface for k in ("z0","z1"))})
        cells = [(a,b,c,d) for a,b in zip(xs,xs[1:]) for c,d in zip(zs,zs[1:]) if b>a and d>c and not any(o["start"]<(a+b)/2<o["end"] and o["sill_z"]<(c+d)/2<o["head_z"] for o in openings)]
        side_area = sum((b-a)*(d-c) for a,b,c,d in cells)
        offset = number(g["thickness"])/2
        for layer in band["layers"]:
            thickness = number(layer["thickness"]["value"])
            inner = offset-thickness
            lo,hi = sorted([side*inner,side*offset])
            resolved.append({"id":layer["id"],"region":band["tag"],"material":layer["material"],"offsets":[float(lo),float(hi)],"is_void":layer.get("is_void",False),"z0":band["z0"],"z1":band["z1"],"gross_side_area":g["length"]*(band["z1"]-band["z0"]),"net_side_area":side_area})
            for a,b,z0,z1 in cells:
                ring = [[number(g["start"][0])+dx*number(s)-dy*t,number(g["start"][1])+dy*number(s)+dx*t] for s,t in ((a,lo),(b,lo),(b,hi),(a,hi))]
                part = {"outline":ring,"z0":number(z0),"z1":number(z1),"layer":layer["id"],"region":band["tag"],"material":layer["material"],"start":a,"end":b,"offsets":[float(lo),float(hi)]}
                finish = surface_finishes().cell_region(surface,layer,a,b,z0,z1) if surface else None
                if finish:
                    part["material"] = finish["material"]
                    part["surface_finish"] = surface_finishes().face_identity(finish,g,[lo,hi])
                (voids if layer.get("is_void") else parts).append(part)
            offset = inner
    interfaces = _wall_interface_cutters(model,wall)
    if wall["geometry"].get("interface_owners") or _wall_has_junction(model,wall):
        g["interface_reference_parts"] = _native_public_value(parts)
    owned,displaced,volume = _wall_owned_parts(model,parts,wall["geometry"].get("interface_owners",()),interfaces)
    air,_,_ = _wall_owned_parts(model,voids,wall["geometry"].get("interface_owners",()),interfaces)
    if wall["geometry"].get("interface_owners"):
        g["interface_displacement_volume"] = displaced
    volumes = {}
    for part in owned+air:
        key = part["region"],part["layer"]
        volumes[key] = volumes.get(key,0.)+(_polyhedron_volume(part) if "mesh" in part else _area(part["outline"])*(part["z1"]-part["z0"]))
    for layer in resolved:
        layer["volume"] = volumes.get((layer["region"],layer["id"]),0.)
    if surface:
        g["surface_finishes"] = surface_finishes().summaries(surface,owned,part_vertices,part_faces)
    return resolved,owned,displaced,volume


def _panel_layout_parts(model: dict, spec: dict, origin: list, direction: list, bounds: tuple, offset: float) -> tuple:
    """🧇 Partition mixed infills, shared metal members and layered hollow leaves without duplicate material."""
    x0,x1,z0,z1=bounds
    width,depth=spec['panel_layout']['member_width']['value'],spec['frame_depth']['value']
    cells,bands=[],[]
    if width<=0 or width>=min(x1-x0,z1-z0):
        raise ValueError('Mixed filling members require a positive contained width')
    for index,cell in enumerate(spec['panel_layout']['cells']):
        u0,u1=[q['value'] for q in cell['u']]
        v0,v1=[q['value'] for q in cell['v']]
        thickness=cell['thickness']['value']
        if not (0<=u0<u1<=1 and 0<=v0<v1<=1 and 0<thickness<=depth):
            raise ValueError('Mixed filling cells require ordered unit bounds and contained material depth')
        material=_find(model,cell['material'])
        if material.get('category')=='air':
            raise ValueError('An infill cell cannot be unbounded air')
        if 'layers' in material and not math.isclose(sum(l['thickness']['value'] for l in material['layers']),thickness,abs_tol=1e-9):
            raise ValueError('Infill layers differ from the cell thickness')
        lo,hi,bottom,top=x0+(x1-x0)*u0,x0+(x1-x0)*u1,z0+(z1-z0)*v0,z0+(z1-z0)*v1
        cells.append((lo,hi,bottom,top,cell,index,material))
        for u in (lo,hi):
            if x0<u<x1: bands.append((max(x0,u-width/2),min(x1,u+width/2),bottom,top,'mullion'))
        for z in (bottom,top):
            if z0<z<z1: bands.append((lo,hi,max(z0,z-width/2),min(z1,z+width/2),'transom'))
    xs=sorted({x0,x1,*[x for c in cells for x in c[:2]],*[x for b in bands for x in b[:2]]})
    zs=sorted({z0,z1,*[z for c in cells for z in c[2:4]],*[z for b in bands for z in b[2:4]]})
    tiles=[]
    for left,right in zip(xs,xs[1:]):
        for bottom,top in zip(zs,zs[1:]):
            x,z=(left+right)/2,(bottom+top)/2
            owners=[c for c in cells if c[0]<x<c[1] and c[2]<z<c[3]]
            if len(owners)!=1:
                raise ValueError('Mixed filling cells must partition the entire clear aperture exactly once')
            members=[b for b in bands if b[0]<x<b[1] and b[2]<z<b[3]]
            tiles.append((left,right,bottom,top,owners[0],members))
    free={cell[5]:[t for t in tiles if t[4][5]==cell[5] and not t[5]] for cell in cells}
    if any(not value for value in free.values()):
        raise ValueError('Mixed filling members consume a complete infill cell')
    parts,moving=[],[]
    dx,dy=direction
    def emit(left,right,bottom,top,n0,n1,material,role,operation,cell):
        """🪄 Place one disjoint rectangular occupied material part in the native host plane."""
        outline=[[origin[0]+dx*u-dy*n,origin[1]+dy*u+dx*n] for u,n in ((left,n0),(right,n0),(right,n1),(left,n1))]
        parts.append({'outline':outline,'z0':bottom,'z1':top,'material':material,'role':role,'operation':operation,'panel_tag':cell[4]['tag'],'panel_index':cell[5]})
    for left,right,bottom,top,cell,members in tiles:
        if members:
            emit(left,right,bottom,top,offset-depth/2,offset+depth/2,spec['frame_material'],members[0][4],'fixed',cell)
            continue
        operation=cell[4]['operation']
        if operation!='fixed': moving.append((left,right,bottom,top))
        thickness=cell[4]['thickness']['value']
        material=cell[6]
        layers=material.get('layers') or [{'material':material['tag'],'thickness':{'value':thickness}}]
        n0=offset-thickness/2
        for layer in layers:
            n1=n0+layer['thickness']['value']
            if layer.get('is_void'):
                edge=spec['core_edge_width']['value']
                fragments=free[cell[5]]
                lo,hi,base,head=min(t[0] for t in fragments),max(t[1] for t in fragments),min(t[2] for t in fragments),max(t[3] for t in fragments)
                if edge<=0 or 2*edge>=min(hi-lo,head-base):
                    raise ValueError('Core battens consume the whole infill leaf')
                for a,b,c,d in ((lo,lo+edge,base,head),(hi-edge,hi,base,head),(lo+edge,hi-edge,base,base+edge),(lo+edge,hi-edge,head-edge,head)):
                    u0,u1,v0,v1=max(left,a),min(right,b),max(bottom,c),min(top,d)
                    if u0<u1 and v0<v1:
                        emit(u0,u1,v0,v1,n0,n1,spec['core_edge_material'],'batten',operation,cell)
            else:
                owned=_find(model,layer['material'])
                emit(left,right,bottom,top,n0,n1,owned['tag'],'glass' if owned.get('category')=='glass' else 'panel',operation,cell)
            n0=n1
    clear=(min(c[0] for c in moving),max(c[1] for c in moving),min(c[2] for c in moving),max(c[3] for c in moving)) if moving else bounds
    return parts,clear


@lru_cache(maxsize=2)
def _filling_structure_snapshot(source: str, producers: tuple) -> tuple:
    """🪒 Own complete-source structural cutters without recursing through wall or filling geometry."""
    model=json.loads(source);parts=[]
    for obj in model['elements']:
        if obj['class']=='IfcColumn':resolved=column_geometry(model,obj)['parts']
        elif obj['class']=='IfcBeam':resolved=_solid_geometry(model,obj,resolve_embedded=False)['parts']
        else:continue
        parts.extend({**deepcopy(part),'owner':obj['tag'],'owner_class':obj['class']} for part in resolved)
    return tuple(parts)

def _filling_structures(model: dict, filling: dict | None = None) -> tuple:
    """🏚️ Resolve complete-source structure and explicit local authority inherited by hosted filling stock."""
    source=json.dumps(model,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False)
    base=_filling_structure_snapshot(source,geometry_producers())
    if filling is None:return base
    contexts=[filling]
    if filling["geometry"].get("opening"):
        opening=_find(model,filling["geometry"]["opening"])
        contexts.extend(_find(model,opening["geometry"][field])for field in ("wall","curtain_wall","slab")if field in opening["geometry"])
    seen={part["owner"]for part in base};extra=[]
    for context in contexts:
        parts=[part for part in _declared_structural_interfaces(model,context)if part["owner"]not in seen]
        seen.update(part["owner"]for part in parts);extra.extend(parts)
    return base+tuple(extra)

def _filling_receiving_interval(structures: tuple, origin: list, direction: tuple, offset: float, depth: float, length: float, width: float, z0: float, z1: float) -> tuple:
    """📶 Keep complete perimeter jamb stock inside the actual receiving end-post faces of its gross aperture."""
    lower,upper,receivers=0.,length,[]
    if width<=0:return lower,upper,receivers
    dx,dy=direction;band=[[0.,offset-depth/2],[length,offset-depth/2],[length,offset+depth/2],[0.,offset+depth/2]]
    for part in structures:
        if part['owner_class']!='IfcColumn' or part['z0']>z0 or part['z1']<z1:continue
        ring=[[(p[0]-origin[0])*dx+(p[1]-origin[1])*dy,-(p[0]-origin[0])*dy+(p[1]-origin[1])*dx] for p in part_outline_at(part,(z0+z1)/2)]
        for convex in _analysis._convex_parts(ring):
            clipped=_analysis._clip(convex,band,digits=None)
            if not clipped or _area(clipped)==0:continue
            start,end=min(p[0] for p in clipped),max(p[0] for p in clipped)
            if start<width and end>lower:lower=end;receivers.append({'owner':part['owner'],'side':'start','station':end})
            if end>length-width and start<upper:upper=start;receivers.append({'owner':part['owner'],'side':'end','station':start})
    if upper-lower<=2*width:raise ValueError('Structural receivers consume the complete filling aperture')
    return lower,upper,receivers


def _filling_owned_parts(model: dict, parts: list, structures: tuple | None = None) -> list:
    """🪤 Retain filling stock outside actual independently owned and explicitly declared structural material."""
    if not parts:return []
    def bounds(part):
        """🚫 Use bounds only for exact rejection; clipping always uses original occupied material."""
        vertices=part_vertices(part)
        return [(min(p[k] for p in vertices),max(p[k] for p in vertices)) for k in range(3)]
    def intersects(a,b):
        """🚥 Keep every strictly positive bounding interval without an invented interface epsilon."""
        return all(a[k][0]<b[k][1] and b[k][0]<a[k][1] for k in range(3))
    structures=_filling_structures(model) if structures is None else structures
    stock_bounds=[bounds(part) for part in parts]
    cutters=[(part,bounds(part)) for part in structures]
    cutters=[(part,extent) for part,extent in cutters if any(intersects(extent,stock) for stock in stock_bounds)]
    result=[]
    for part,extent in zip(parts,stock_bounds):
        active=[c for c,area in cutters if intersects(extent,area)]
        if not active:result.append(deepcopy(part));continue
        spatial=any(any(key in p for key in ('mesh','section','top_outline')) for p in [part,*active])
        if not spatial:result.extend(_subtract_prisms([part],active));continue
        cells=[part] if any(key in part for key in ('mesh','section','top_outline')) else [{**part,'outline':ring,'holes':[]} for region in part_regions(part,(part['z0']+part['z1'])/2) for ring in _analysis._convex_parts(region)]
        for cell in cells:
            mesh={'vertices':part_vertices(cell),'faces':part_faces(cell)}
            stock={k:deepcopy(v) for k,v in cell.items() if k not in ('section','top_outline','holes','profile')}
            stock['mesh']=mesh;result.extend(_subtract_convex_material(stock,active))
    return result


def _filling_geometry(model: dict, filling: dict) -> dict:
    """🎞️ Resolve perimeter members and panel solids from one typed rough aperture."""
    opening = _find(model,filling["geometry"]["opening"])
    domain = _opening_domain(model,opening)
    spec = _find(model,filling["type"])["filling"]
    v = _opening_values(model,opening)
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    axis=provider.path_slice(domain["path"],v["start"],v["end"])
    a,b=axis[0],axis[-1]
    length=math.dist(a,b)
    if abs(_length(axis)-length)>1e-8:
        raise ValueError("A native window or door requires a planar host interval")
    dx,dy=(b[0]-a[0])/length,(b[1]-a[1])/length
    base=domain["base"]
    rough_height = v["head"]-v["sill"]
    extension = filling["geometry"].get("frame_head_extension",{}).get("value",0.)
    if extension<0:
        raise ValueError("A frame head extension cannot be negative")
    s0,s1,z0,z1=0.,length,base+v["sill"],base+v["head"]+extension
    w,d,t = [spec[k]["value"] for k in ("frame_width","frame_depth","panel_thickness")]
    offset = domain.get("offset",0.)+domain["side"]*(domain["thickness"]/2-spec["inset"]["value"]-d/2)
    offset += domain["side"]*filling["geometry"].get("normal_offset",{}).get("value",0.)
    structures=_filling_structures(model,filling)
    source_length=length;receiving_start,receiving_end,receivers=(0.,length,[]) if spec.get("side_wings") else _filling_receiving_interval(structures,a,(dx,dy),offset,d,length,w,z0,z1)
    a=[a[0]+dx*receiving_start,a[1]+dy*receiving_start];length=receiving_end-receiving_start;s1=length
    low = z0+w if spec["threshold"] else z0
    if s1-s0 <= 2*w or z1-low <= w:
        raise ValueError("A filling frame consumes its complete aperture")
    parts = []
    def part(u0,u1,h0,h1,t0,t1,material,role):
        section=spec.get("member_sections",{}).get("interior" if role=="mullion" else "perimeter") if role in ("frame","mullion") else None
        vertical=role=="mullion" or h1-h0>u1-u0
        cells=provider.member_section(u1-u0 if vertical else h1-h0,t1-t0,section,reverse=u0>length/2 if vertical else h0>(z0+z1)/2) if section else None
        bounds=[(u0+v0,u0+v1,h0,h1,(t0+t1)/2+n0,(t0+t1)/2+n1) if vertical else (u0,u1,h0+v0,h0+v1,(t0+t1)/2+n0,(t0+t1)/2+n1) for v0,v1,n0,n1 in cells] if cells else [(u0,u1,h0,h1,t0,t1)]
        for index,(s0,s1,z0p,z1p,n0,n1) in enumerate(bounds):
            ring = [[a[0]+dx*s-dy*n,a[1]+dy*s+dx*n] for s,n in ((s0,n0),(s1,n0),(s1,n1),(s0,n1))]
            parts.append({"outline":ring,"z0":z0p,"z1":z1p,"material":material,"role":role,**({"section_shape":section["shape"],"section_component":index} if section else {})})
    wing=spec.get("side_wings")
    if wing:
        wing_width,wing_depth,skin,recess=[wing[k]["value"] for k in ("width","depth","skin","recess")]
        if 2*wing_width+2*w>=length or 2*skin>=min(wing_width,wing_depth,z1-z0) or recess+d>wing_depth+1e-9:
            raise ValueError("Insulated entrance wings require a positive disjoint core and a contained recessed frame")
        offset=-wing_depth/2+recess+d/2
        for u0,u1 in ((0.,wing_width),(length-wing_width,length)):
            for x0,x1,n0,n1 in ((u0,u0+skin,-wing_depth/2,wing_depth/2),(u1-skin,u1,-wing_depth/2,wing_depth/2),(u0+skin,u1-skin,-wing_depth/2,-wing_depth/2+skin),(u0+skin,u1-skin,wing_depth/2-skin,wing_depth/2)):
                part(x0,x1,z0,z1,n0,n1,wing["skin_material"],"wing_skin")
            for lo,hi in ((z0,z0+skin),(z1-skin,z1)):
                part(u0+skin,u1-skin,lo,hi,-wing_depth/2+skin,wing_depth/2-skin,wing["skin_material"],"wing_skin")
            part(u0+skin,u1-skin,z0+skin,z1-skin,-wing_depth/2+skin,wing_depth/2-skin,wing["core_material"],"wing_insulation")
        s0,s1=wing_width,length-wing_width
    track = spec.get("track_web",{}).get("value",0.)
    if track and (spec["operation"]!="sliding" or not spec["threshold"] or 3*track>=d or track>=w):
        raise ValueError("Sliding tracks require a threshold and two clear lanes")
    if w > 0:
        members = [(s0,s0+w,z0,z1),(s1-w,s1,z0,z1)]
        if not track:
            members += [(s0+w,s1-w,z1-w,z1)]+([(s0+w,s1-w,z0,z0+w)] if spec["threshold"] else [])
        for u0,u1,h0,h1 in members:
            part(u0,u1,h0,h1,offset-d/2,offset+d/2,spec["frame_material"],"frame")
    if track:
        for bottom in (True,False):
            h0,h1 = (z0,z0+track) if bottom else (z1-track,z1)
            part(s0+w,s1-w,h0,h1,offset-d/2,offset+d/2,spec["frame_material"],"track_web")
            h0,h1 = (z0+track,z0+w) if bottom else (z1-w,z1-track)
            for n in (offset-d/2,offset-track/2,offset+d/2-track):
                part(s0+w,s1-w,h0,h1,n,n+track,spec["frame_material"],"track_guide")
    clear_start,clear_end,clear_sill_z,clear_head_z=s0+w,s1-w,low,z1-w
    if 'panel_layout' in spec:
        layout,clear=_panel_layout_parts(model,spec,a,(dx,dy),(clear_start,clear_end,clear_sill_z,clear_head_z),offset)
        parts.extend(layout)
        clear_start,clear_end,clear_sill_z,clear_head_z=clear
    else:
        panel = _find(model,spec["panel_material"])
        stations = [v["value"] for v in filling["geometry"].get("panel_stations",spec["panel_stations"])]
        operations = filling["geometry"].get("panel_operations",[spec["operation"]]*(len(stations)-1))
        glazing = [[v["value"] for v in row] for row in filling["geometry"].get("panel_glazing_stations",[[{"value":0},{"value":1}]]*(len(stations)-1))]
        if any(not row or row[0]!=0 or row[-1]!=1 or any(b<=a for a,b in zip(row,row[1:])) for row in [stations,*glazing]):
            raise ValueError("Panel and glazing stations must strictly partition the unit interval")
        if len(operations)!=len(stations)-1 or len(glazing)!=len(stations)-1 or any(value not in ("fixed","swing","sliding","mixed","unknown") for value in operations):
            raise ValueError("Each primary assembly requires one operation and glazing partition")
        clear_start,clear_end = s0+w,s1-w
        mullion = spec.get("mullion_width",{}).get("value",0.)
        sash = spec.get("sash_width",{}).get("value",0.)
        sash_depth = spec.get("sash_depth",{}).get("value",d)
        if sash_depth>d or sash_depth<t:
            raise ValueError("Sash depth must contain glass within the outer frame")
        for station in (stations[1:-1] if mullion else []):
            u = clear_start+(clear_end-clear_start)*station
            part(u-mullion/2,u+mullion/2,low,z1-w,offset-d/2,offset+d/2,spec["frame_material"],"mullion")
        for panel_index,(a0,a1) in enumerate(zip(stations,stations[1:])):
            u0,u1,h0,h1 = clear_start+(clear_end-clear_start)*a0,clear_start+(clear_end-clear_start)*a1,low,z1-w
            u0 += mullion/2 if panel_index else 0.
            u1 -= mullion/2 if panel_index<len(stations)-2 else 0.
            panel_offset = offset
            if track:
                lane_width = (d-3*track)/2
                if sash_depth>lane_width:
                    raise ValueError("Sliding sash exceeds its clear track lane")
                panel_offset = offset-d/2+track+lane_width/2+(1 if operations[panel_index]=="sliding" and "panel_operations" in filling["geometry"] else panel_index%2)*(lane_width+track)
            first_part = len(parts)
            if sash:
                if u1-u0<=2*sash or h1-h0<=2*sash:
                    raise ValueError("Sash members consume their panel")
                for x0,x1,y0,y1 in [(u0,u0+sash,h0,h1),(u1-sash,u1,h0,h1),(u0+sash,u1-sash,h0,h0+sash),(u0+sash,u1-sash,h1-sash,h1)]:
                    part(x0,x1,y0,y1,panel_offset-sash_depth/2,panel_offset+sash_depth/2,spec["frame_material"],"sash")
                u0,u1,h0,h1 = u0+sash,u1-sash,h0+sash,h1-sash
            if "layers" not in panel:
                divisions = glazing[panel_index]
                bar = spec.get("glazing_bar_width",{}).get("value",0.)
                if len(divisions)>2 and bar<=0:
                    raise ValueError("Secondary glazing divisions require a physical bar width")
                for station in divisions[1:-1]:
                    u = u0+(u1-u0)*station
                    part(u-bar/2,u+bar/2,h0,h1,panel_offset-sash_depth/2,panel_offset+sash_depth/2,spec["frame_material"],"glazing_bar")
                for glass_index,(left,right) in enumerate(zip(divisions,divisions[1:])):
                    lo,hi = u0+(u1-u0)*left+(bar/2 if glass_index else 0.),u0+(u1-u0)*right-(bar/2 if glass_index<len(divisions)-2 else 0.)
                    if hi<=lo:
                        raise ValueError("Glazing bars consume a complete pane")
                    part(lo,hi,h0,h1,panel_offset-t/2,panel_offset+t/2,panel["tag"],"glass" if panel.get("category")=="glass" else "panel")
                    parts[-1]["glazing_index"] = glass_index
            else:
                if not math.isclose(sum(l["thickness"]["value"] for l in panel["layers"]),t,abs_tol=1e-9):
                    raise ValueError("Panel layers differ from its declared thickness")
                edge = spec["core_edge_width"]["value"]
                if u1-u0 <= 2*edge or h1-h0 <= 2*edge:
                    raise ValueError("Panel core edging consumes the panel")
                n0 = offset-t/2
                for layer in panel["layers"]:
                    n1 = n0+layer["thickness"]["value"]
                    if layer.get("is_void"):
                        for x0,x1,y0,y1 in [(u0,u0+edge,h0,h1),(u1-edge,u1,h0,h1),(u0+edge,u1-edge,h0,h0+edge),(u0+edge,u1-edge,h1-edge,h1)]:
                            part(x0,x1,y0,y1,n0,n1,spec["core_edge_material"],"batten")
                    else:
                        part(u0,u1,h0,h1,n0,n1,layer["material"],"panel")
                    n0 = n1
            for member in parts[first_part:]:
                member["panel_index"] = panel_index
                member["panel_operation"] = operations[panel_index]
    parts=_filling_owned_parts(model,parts,structures)
    surface_finishes().apply_material_appearance(parts,lambda tag:_find(model,tag))
    return {"tag":filling["tag"],"guid":filling["guid"],"class":filling["class"],"opening":opening["tag"],"host":domain["host"],"host_class":domain["host_class"],"parts":parts,
            "operation":spec["operation"],"frame_width":w,"frame_depth":d,"panel_thickness":t,"rough_width":length,"source_rough_width":source_length,"receiving_stations":{"start":receiving_start,"end":receiving_end,"receivers":receivers},"rough_height":rough_height,"overall_height":z1-z0,"clear_width":clear_end-clear_start,"clear_height":clear_head_z-clear_sill_z,"clear_start":v["start"]+receiving_start+clear_start,"clear_end":v["start"]+receiving_start+clear_end,"clear_sill_z":clear_sill_z,"clear_head_z":clear_head_z,"storey":filling["storey"],"axis":[[a[0]+dx*s-dy*offset,a[1]+dy*s+dx*offset] for s in (0.,length)]}


def _path_band(path: list, thickness: float) -> list:
    """🧵 Offset a simple open polyline with continuous mitered material edges."""
    normals = []
    for a,b in zip(path,path[1:]):
        length = math.dist(a,b)
        if length <= 1e-10:
            raise ValueError("A material path cannot contain repeated vertices")
        normals.append([-(b[1]-a[1])/length,(b[0]-a[0])/length])
    sides = []
    for sign in (-1,1):
        edge = []
        for i,p in enumerate(path):
            if i in (0,len(path)-1):
                n = normals[0 if i==0 else -1]
                delta = [v*thickness*sign/2 for v in n]
            else:
                a,b = normals[i-1],normals[i]
                denominator = 1+a[0]*b[0]+a[1]*b[1]
                if denominator <= 1e-8:
                    raise ValueError("A material path cannot reverse at a miter")
                delta = [(a[k]+b[k])*thickness*sign/(2*denominator) for k in (0,1)]
            edge.append([p[k]+delta[k] for k in (0,1)])
        sides.append(edge)
    return sides[0]+list(reversed(sides[1]))


def _curtain_filling_geometry(model: dict, curtain: dict) -> dict:
    """🪩 Resolve source-authored planar panes and native metal members through one physical provider."""
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    physical=provider.physical_geometry(SimpleNamespace(**globals()),model,curtain)
    physical["parts"]=_filling_owned_parts(model,physical["parts"],_filling_structures(model,curtain))
    return physical

def _subtract_convex_material(part: dict, cutters: list) -> list:
    """🪬 Partition native convex material at physical joints with original exact plane operands."""
    backend = surface_finishes()
    states = [backend.clip_stock(part,[],exact=True)]
    for cutter in cutters:
        vertices = part_vertices(cutter)
        planes = []
        for face in part_faces(cutter):
            anchor = vertices[face[0]]
            for i in range(1,len(face)-1):
                points = [anchor,vertices[face[i]],vertices[face[i+1]]]
                u,v = [[p[k]-anchor[k] for k in range(3)] for p in points[1:]]
                normal = [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
                if any(normal):break
            else:raise ValueError("A physical joint face requires original noncollinear points")
            planes.append({"normal":normal,"offset":sum(n*x for n,x in zip(normal,anchor)),"points":points})
        retained = []
        for stock in states:
            bounds = [[min(p[k] for p in part_vertices(obj)),max(p[k] for p in part_vertices(obj))] for obj in (stock,cutter) for k in range(3)]
            if any(bounds[k][1]<=bounds[k+3][0] or bounds[k+3][1]<=bounds[k][0] for k in range(3)):
                retained.append(stock)
                continue
            if backend.stock_separated(stock,planes):
                retained.append(stock)
                continue
            inside = stock
            for plane in planes:
                reverse = {**plane,"normal":[-n for n in plane["normal"]],"offset":-plane["offset"]}
                outside = backend.clip_stock(inside,[reverse])
                if outside is not None:retained.append(outside)
                inside = backend.clip_stock(inside,[plane])
                if inside is None:break
        states = retained
    return [backend.publish_stock(stock) for stock in states]


def _source_fraction(value):
    """🔢 Preserve an authored decimal scalar before affine construction."""
    from fractions import Fraction
    return value if isinstance(value,Fraction) else Fraction(str(value))


def _ramp_reference_stock(model: dict, obj: dict) -> dict:
    """🫧 Retain original affine ramp operands privately until one native publication."""
    q = _source_fraction
    g = obj["geometry"]
    a,b = [[q(v) for v in p] for p in g["path"]]
    dx,dy = b[0]-a[0],b[1]-a[1]
    square = dx*dx+dy*dy
    if square<=0:raise ValueError("A ramp reference needs a positive source run")
    numerator,denominator = math.isqrt(square.numerator),math.isqrt(square.denominator)
    run = q(numerator)/denominator if numerator*numerator==square.numerator and denominator*denominator==square.denominator else q(math.sqrt(float(square)))
    normal = [-dy/run,dx/run]
    lo,hi = q(_z(model,g["from_level"])),q(_z(model,g["to_level"]))
    z0,z1 = [lo+(hi-lo)*q(portion) for portion in g["portion"]]
    finish = q(g["finish_thickness"]["value"])
    if g["finish_direction"]=="normal":finish *= q(math.hypot(1.,_round(float((z1-z0)/run))))
    thickness = q(g["thickness"]["value"])
    lower,upper = [q(v["value"]) for v in g["structural_offsets"]]
    top = [[p[0]+offset*normal[0],p[1]+offset*normal[1],z-finish] for p,z,offset in ((a,z0,upper),(a,z0,lower),(b,z1,lower),(b,z1,upper))]
    vertices = [[x,y,z-thickness] for x,y,z in top]+top
    return {"mesh":{"vertices":vertices,"faces":[[3,2,1,0],[4,5,6,7],[0,1,5,4],[1,2,6,5],[2,3,7,6],[3,0,4,7]]},"outline":[p[:2] for p in top],"z0":min(v[2] for v in vertices),"z1":max(v[2] for v in vertices),"owner":obj["tag"],"owner_guid":obj["guid"],"tag":obj["tag"],"role":"flight","material":_material(model,obj)["tag"]}



def _ramp_nominal_parts(model: dict, obj: dict) -> list:
    """🧗 Retain only original uncut ramp product stock, leaving children with their own source references."""
    if obj["class"]=="IfcRampFlight":return [_ramp_reference_stock(model,obj)]
    q=_source_fraction;g=obj["geometry"];low,high=q(_z(model,g["from_level"])),q(_z(model,g["to_level"]));result=[]
    for landing in g["landings"]:
        if landing["ownership"]!="ramp_component" or landing["owner"]!=obj["tag"]:continue
        top=low+(high-low)*q(landing["portion"])-q(g["finish_thickness"]["value"])
        result.append({"outline":[[q(v)for v in p]for p in _expand(model,landing["outline"])],"z0":top-q(landing["thickness"]["value"]),"z1":top,"owner":obj["tag"],"owner_guid":obj["guid"],"tag":landing["tag"],"role":"landing","material":_material(model,obj)["tag"]})
    return result

def _reference_material_parts(part: dict, cutters: list, bounds: list = (), progress=None, cancel=None) -> list:
    """🪨 Retain exact original structural stock and physical aperture operands before publication."""
    if any("csg"in p for p in [part,*cutters,*bounds]):
        return _analytic_solids_backend().reference_material_parts(_analytic_input(part),[_analytic_input(p)for p in cutters],[_analytic_input(p)for p in bounds],progress=progress,cancel=cancel)
    backend = surface_finishes()
    def planes(cell):
        vertices = part_vertices(cell)
        for face in part_faces(cell):
            anchor = vertices[face[0]]
            for i in range(1,len(face)-1):
                u,v = [[vertices[index][k]-anchor[k] for k in range(3)] for index in (face[i],face[i+1])]
                normal = [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
                if any(normal):break
            else:raise ValueError("Original structural stock requires nondegenerate faces")
            offset = sum(n*x for n,x in zip(normal,anchor))
            sides = [sum(n*x for n,x in zip(normal,p))-offset for p in vertices]
            if max(sides)>-min(sides):normal,offset = [-v for v in normal],-offset
            yield {"normal":normal,"offset":offset,"points":[anchor,vertices[face[i]],vertices[face[i+1]]]}
    stocks = []
    for cell in _native_material_cells(part):
        for boundary in bounds or (None,):
            stock = backend.clip_stock(cell,[],exact=True)
            for plane in planes(boundary) if boundary else ():
                stock = backend.clip_stock(stock,[plane])
                if stock is None:break
            if stock is not None:stocks.append(stock)
    reference_bounds = _material_bounds(part)
    for cutter_index,cutter in enumerate(cutters):
        if cancel and cancel():raise InterruptedError('Native reference construction cancelled')
        if progress:progress({'phase':'reference-material','completed':cutter_index,'total':len(cutters)})
        cutter_bounds = _material_bounds(cutter)
        if not all(a[0]<b[1] and b[0]<a[1] for a,b in zip(reference_bounds,cutter_bounds)):continue
        for cell in _native_material_cells(cutter):
            ordered = list(planes(cell))
            result = []
            for stock in stocks:
                if cancel and cancel():raise InterruptedError('Native reference construction cancelled')
                left,right = _material_bounds({"mesh":stock._exact_mesh}),_material_bounds(cell)
                if not all(a[0]<b[1] and b[0]<a[1] for a,b in zip(left,right)):
                    result.append(stock)
                    continue
                if backend.stock_separated(stock,ordered):
                    result.append(stock)
                    continue
                current = stock
                for plane in ordered:
                    outside = backend.clip_stock(current,[{**plane,"normal":[-v for v in plane["normal"]],"offset":-plane["offset"]}])
                    if outside is not None:result.append(outside)
                    current = backend.clip_stock(current,[plane])
                    if current is None:break
            stocks = result
    if cancel and cancel():raise InterruptedError('Native reference construction cancelled')
    if progress:progress({'phase':'reference-material','completed':len(cutters),'total':len(cutters)})
    return [{**{key:deepcopy(value) for key,value in part.items() if key not in ("outline","top_outline","holes","mesh","section","z0","z1")},"mesh":deepcopy(stock._exact_mesh)} for stock in stocks]


def _interface_reference_parts(model: dict, key: str) -> list:
    """🗳️ Resolve unpartitioned source-affine structural stock without inherited publication seams."""
    q = _source_fraction
    obj = _find(model,key)
    g = obj["geometry"]
    material = _material(model,obj)["tag"]
    if obj["class"]=="IfcSlab":return deepcopy(_slab_geometry_entry(model,obj,exact=True)["parts"])
    if obj["class"]=="IfcColumn":
        if "end_plane" in g:raise ValueError("A capped column interface requires its original cap construction")
        profiles = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️")
        source = g.get("sections",[{"profile":_find(model,obj["type"])["profile"],"base_level":g["base_level"],"top_level":g["top_level"]}])
        result = []
        for section in source:
            profile = section["profile"]
            if profile.get("voids"):raise ValueError("Hollow interface stock requires explicit material and bore-air reservations")
            shape = profile["shape"]
            approximation = None
            if shape=="rectangle":
                w,d = q(profile["width"]["value"])/2,q(profile["depth"]["value"])/2
                points = [[-w,-d],[w,-d],[w,d],[-w,d]]
            elif shape=="polygon":points = [[q(v) for v in p] for p in profile["outline"]]
            elif shape=="circle":
                approximation = profiles.circle_envelope(float(q(profile["diameter"]["value"])/2),1e-5,2e-9,256)
                points = [[q(v) for v in p] for p in approximation.pop("outline")]
            else:raise ValueError("The structural interface profile needs a declared conservative construction")
            angle = profile.get("rotation",0)
            if angle%90==0:c,s = ((1,0),(0,1),(-1,0),(0,-1))[int(angle//90)%4]
            else:c,s = q(math.cos(math.radians(angle))),q(math.sin(math.radians(angle)))
            x,y = [q(v) for v in section.get("axis",g["axis"])]
            part = {"outline":[[x+c*u-s*v,y+s*u+c*v] for u,v in points],"z0":q(_z(model,section["base_level"])),"z1":q(_top(model,section["top_level"])),"material":material,"role":"structural_interface"}
            if approximation:part["interface_approximation"] = approximation
            result.append(part)
        return result
    if obj["class"] not in ("IfcBeam","IfcFooting","IfcBuildingElementProxy") or "parts" not in g or g.get("voids"):
        raise ValueError("An exact structural interface needs its original independent stock and physical void construction")
    base = q(_z(model,g["base_level"]))
    result = []
    for raw in g["parts"]:
        part = {"material":raw.get("material",material),"role":raw.get("role","structural_interface")}
        if "mesh" in raw:part["mesh"] = {"vertices":[[q(x),q(y),base+q(z)] for x,y,z in raw["mesh"]["vertices"]],"faces":deepcopy(raw["mesh"]["faces"])}
        elif "section" in raw:part["section"] = {**deepcopy(raw["section"]),"profile":[[q(x),base+q(z)] for x,z in raw["section"]["profile"]],"sweep":[q(v) for v in raw["section"]["sweep"]]}
        else:
            part.update(outline=[[q(v) for v in p] for p in raw["outline"]],z0=base+q(raw["z0"]),z1=base+q(raw["z1"]))
            if "top_outline" in raw:part["top_outline"] = [[q(v) for v in p] for p in raw["top_outline"]]
            if "holes" in raw:part["holes"] = [[[q(v) for v in p] for p in ring] for ring in raw["holes"]]
        result.append(part)
    for key in g.get("embedded_slabs",()):
        slab = _find(model,key)
        spec = slab["geometry"]
        level = q(_z(model,spec["level"]))
        for region in spec["regions"]:
            if region.get("member")!=obj["tag"]:continue
            top = level+q(region["top_offset"]["value"])
            stock = {"outline":[[q(v) for v in p] for p in _expand(model,region["outline"])],"z0":top-q(region["thickness"]["value"]),"z1":top,"material":region.get("material",material),"role":"structural_interface"}
            boundary = {"outline":[[q(v) for v in p] for p in _expand(model,spec["outline"])],"z0":stock["z0"],"z1":top}
            voids = [{"outline":[[q(v) for v in p] for p in _expand(model,void["outline"])],"z0":stock["z0"],"z1":top} for void in _slab_voids(model,slab)]
            result.extend(_reference_material_parts(stock,voids,_native_material_cells(boundary)))
    return result



def _ramp_owned_reference_parts_uncached(model: dict, obj: dict, progress=None, cancel=None) -> list:
    """🥾 Retain only original source-affine owned flight and middle-landing material before publication."""
    g = obj['geometry']
    if obj['class']=='IfcRampFlight':
        joints = g.get('flight_joint_owners',())
        if len(set(joints))!=len(joints) or any(key==obj['tag'] or _find(model,key)['class']!='IfcRampFlight' or _find(model,key)['geometry'].get('flight_joint_owners') for key in joints):
            raise ValueError('Exact ramp joints require distinct acyclic primary flight owners')
        cutters = [_ramp_reference_stock(model,_find(model,key)) for key in joints]+[part for key in g.get('interface_owners',()) for part in _interface_reference_parts(model,key)]
        return _reference_material_parts(_ramp_reference_stock(model,obj),cutters,progress=progress,cancel=cancel)
    if obj['class']!='IfcRamp':raise ValueError('Exact owned ramp material requires a ramp or flight')
    q = _source_fraction
    low,high = q(_z(model,g['from_level'])),q(_z(model,g['to_level']))
    cutters = [part for key in g.get('interface_owners',()) for part in _interface_reference_parts(model,key)]
    result = []
    for landing in g['landings']:
        if landing['ownership']!='ramp_component' or landing['owner']!=obj['tag']:continue
        datum = low+(high-low)*q(landing['portion'])-q(g['finish_thickness']['value'])
        stock = {'outline':[[q(v) for v in p] for p in _expand(model,landing['outline'])],'z0':datum-q(landing['thickness']['value']),'z1':datum,'owner':obj['tag'],'owner_guid':obj['guid'],'tag':landing['tag'],'role':'landing','material':_material(model,obj)['tag']}
        result.extend(_reference_material_parts(stock,cutters,progress=progress,cancel=cancel))
    return result


def _ramp_owned_reference_parts(model: dict, obj: dict, progress=None, cancel=None) -> list:
    """🛤️ Isolate exact owned ramp cells inside the complete bounded source/producer geometry memo."""
    return _geometry_query('ramp_owned_reference',obj['tag'],model,progress=progress,cancel=cancel)


def _ramp_geometry(model: dict, obj: dict) -> dict:
    """🎢 Resolve real flight slabs and landing ownership without changing source stations."""
    if obj["class"] not in ("IfcRamp","IfcRampFlight"):
        raise ValueError("Ramp geometry requires a native ramp or ramp flight")
    ramp = obj if obj["class"]=="IfcRamp" else _find(model,obj["geometry"]["ramp"])
    if ramp["class"]!="IfcRamp":
        raise ValueError("A native flight requires an authored ramp parent")
    flights,children,parts,shared = [],[],[],[]
    references = {}
    for flight in [flight for parent in _by_class(model,"IfcRamp") for flight in _flights(model,parent)]:
        source = _find(model,flight["tag"])
        if source["geometry"]["thickness_direction"]!="vertical":
            raise ValueError("A ramp slab requires its explicit vertical thickness convention")
        a,b = flight["start"],flight["end"]
        run = math.dist(a,b)
        if run<=1e-12 or flight["width"]<=0 or flight["thickness"]<=0:
            raise ValueError("A ramp flight requires positive run, width and thickness")
        offsets,walking = flight["structural_offsets"],flight["walking_offsets"]
        if offsets[1]<=offsets[0] or walking[1]<=walking[0] or not math.isclose(walking[1]-walking[0],flight["width"],abs_tol=1e-10):
            raise ValueError("Ramp offsets must be ordered and walking span must equal clear width")
        stock = _ramp_reference_stock(model,source)
        references[flight["tag"]] = stock
        slab = surface_finishes().publish_stock(surface_finishes().clip_stock(stock,[],exact=True))
        children.append(slab)
        flights.append({**flight,"parts":[deepcopy(slab)]})
    raw = {flight["tag"]:flight["parts"][0] for flight in flights}
    descriptors = {flight["tag"]:flight for flight in flights}
    def arrivals(flight):
        source = _find(model,flight["ramp"])["geometry"]
        lo,hi = _z(model,source["from_level"]),_z(model,source["to_level"])
        return {(landing["owner"],lo+(hi-lo)*landing["portion"]) for landing in source["landings"] if landing["ownership"]=="floor_slab" and lo+(hi-lo)*landing["portion"] in (flight["z0"],flight["z1"])}
    selected = [flight for flight in flights if flight["ramp"]==ramp["tag"] and (obj["class"]=="IfcRamp" or flight["tag"]==obj["tag"])]
    for flight in selected:
        joint = _find(model,flight["tag"])["geometry"].get("flight_joint_owners",[])
        if any(tag==flight["tag"] or tag not in raw or _find(model,tag)["geometry"].get("flight_joint_owners") for tag in joint) or len(set(joint))!=len(joint):
            raise ValueError("Ramp flight joints require distinct acyclic primary flight owners")
        if any(descriptors[tag]["ramp"]!=flight["ramp"] and not arrivals(flight).intersection(arrivals(descriptors[tag])) for tag in joint):
            raise ValueError("Cross-assembly ramp joints require the same actual floor arrival")
        refs = _find(model,flight["tag"])["geometry"].get("interface_owners",[])
        if len(set(refs))!=len(refs) or any(_find(model,tag)["class"] not in ("IfcColumn","IfcBeam","IfcSlab","IfcFooting") for tag in refs):
            raise ValueError("Ramp structural interfaces require distinct independent structural owners")
        cutters = [references[tag] for tag in joint]+[part for tag in refs for part in _interface_reference_parts(model,tag)]
        flight["reference_parts"] = [deepcopy(raw[flight["tag"]])]
        if cutters:flight["parts"],flight["interface_removed_volume"] = _subtract_native_material([references[flight["tag"]]],cutters)
    flights = selected
    children = [part for flight in flights for part in flight["parts"]]
    landings = []
    if obj["class"]=="IfcRamp":
        g = obj["geometry"]
        if g["thickness_direction"]!="vertical":
            raise ValueError("A ramp assembly requires its explicit vertical thickness convention")
        lo,hi = _z(model,g["from_level"]),_z(model,g["to_level"])
        for landing in g["landings"]:
            top = _round(lo+(hi-lo)*landing["portion"])
            owner = _find(model,landing["owner"])
            if landing["ownership"]=="floor_slab":
                if owner["class"]!="IfcSlab":
                    raise ValueError("A floor arrival must reference its physical supporting slab")
                ring = _expand(model,landing["outline"])
                solid = _slab_geometry_entry(model,owner)
                resolved = []
                for part in solid["parts"]+solid["shared_parts"]:
                    for profile in _slab_surface(ring,part["outline"],[]):
                        resolved.append({**deepcopy(part),"outline":profile,"owner":owner["tag"],"owner_guid":owner["guid"],"tag":landing["tag"],"role":"floor_arrival_reference"})
                area = sum(_area(part["outline"]) for part in resolved)
                if not math.isclose(area,_area(ring),abs_tol=1e-7) or any(part["z1"]>top+1e-9 for part in resolved):
                    raise ValueError("A floor arrival requires full actual slab support below its finished datum")
                shared.extend(resolved)
                landings.append({"tag":landing["tag"],"owner":owner["tag"],"owner_guid":owner["guid"],"ownership":"floor_slab","outline":ring,"z":top,"parts":deepcopy(resolved)})
                continue
            if owner["class"]!="IfcRamp" or not any(p["tag"]==landing["tag"] and p["owner"]==owner["tag"] for p in owner["geometry"]["landings"]):
                raise ValueError("A landing must reference its native owning ramp component")
            registered = next(p for p in owner["geometry"]["landings"] if p["tag"]==landing["tag"] and p["owner"]==owner["tag"])
            origin = _z(model,owner["geometry"]["from_level"])
            owner_top = origin+(_z(model,owner["geometry"]["to_level"])-origin)*registered["portion"]
            if registered["outline"]!=landing["outline"] or registered["walking_outline"]!=landing["walking_outline"] or registered["thickness"]!=landing["thickness"] or abs(owner_top-top)>1e-10:
                raise ValueError("A borrowed landing must retain its registered owner geometry")
            structure_top = top-owner["geometry"]["finish_thickness"]["value"]
            part = {"outline":_expand(model,landing["outline"]),"z0":_round(structure_top-landing["thickness"]["value"]),"z1":structure_top,"owner":owner["tag"],"owner_guid":owner["guid"],"tag":landing["tag"],"role":"landing","material":_material(model,owner)["tag"]}
            owned = [part]
            refs = owner["geometry"].get("interface_owners",())
            if refs:
                if len(set(refs))!=len(refs) or any(_find(model,key)["class"] not in ("IfcColumn","IfcBeam","IfcFooting") for key in refs):
                    raise ValueError("Ramp landing interfaces require distinct independent structural owners")
                q = _source_fraction
                datum = q(_z(model,owner["geometry"]["from_level"]))+(q(_z(model,owner["geometry"]["to_level"]))-q(_z(model,owner["geometry"]["from_level"])))*q(landing["portion"])-q(owner["geometry"]["finish_thickness"]["value"])
                stock = {**part,"outline":[[q(v) for v in p] for p in _expand(model,landing["outline"])],"z0":datum-q(landing["thickness"]["value"]),"z1":datum}
                owned,_ = _subtract_native_material([stock],[p for key in refs for p in _interface_reference_parts(model,key)])
            (parts if owner["tag"]==obj["tag"] else shared).extend(owned)
            landings.append({"tag":landing["tag"],"owner":owner["tag"],"owner_guid":owner["guid"],"ownership":"ramp_component","outline":deepcopy(part["outline"]),"walking_outline":deepcopy(landing["walking_outline"]),"z":top,"thickness":landing["thickness"]["value"],"parts":deepcopy(owned)})
    else:
        parts,children = children,[]
    return {"tag":obj["tag"],"guid":obj["guid"],"class":obj["class"],"storey":obj["storey"],"name":deepcopy(obj["name"]),"domains":deepcopy(obj["metadata"]["domains"]),"thickness_direction":"vertical","parts":parts,"shared_parts":shared,"child_parts":children,"full_parts":deepcopy(parts+shared+children),"reference_parts":_native_public_value(_ramp_nominal_parts(model,obj)),"flights":flights,"landings":landings,"width":obj["attributes"]["width"]["value"],"children":[{"tag":f["tag"],"guid":f["guid"],"class":"IfcRampFlight"} for f in flights] if obj["class"]=="IfcRamp" else []}


def ramp_geometry(key: str, model: dict | None = None) -> dict:
    """🛝️ Expose native ramp assembly and product material through one physical interface."""
    m = _m(model)
    return _ramp_geometry(m,_find(m,key))


def _flights(model: dict, ramp: dict) -> list:
    out = []
    for flight in [e for e in _by_class(model, "IfcRampFlight") if e["geometry"]["ramp"] == ramp["tag"]]:
        g = flight["geometry"]
        lo, hi = _z(model, g["from_level"]), _z(model, g["to_level"])
        z0, z1 = _round(lo + (hi - lo) * g["portion"][0]), _round(lo + (hi - lo) * g["portion"][1])
        run = math.dist(g["path"][0], g["path"][1])
        out.append({"tag": flight["tag"], "guid": flight["guid"], "ramp": ramp["tag"], "start": list(g["path"][0]), "end": list(g["path"][1]),
                    "structural_offsets": [v["value"] for v in g["structural_offsets"]], "walking_offsets": [v["value"] for v in g["walking_offsets"]], "structural_width": g["structural_offsets"][1]["value"]-g["structural_offsets"][0]["value"],
                    "thickness": g["thickness"]["value"], "width": flight["attributes"]["width"]["value"], "z0": z0, "z1": z1, "run": _round(run), "slope": _round((z1 - z0) / run)})
    return out


def _surface_bounds(polygon: list) -> tuple:
    """🗜️ Bound an XY polygon for exact disjointness rejection."""
    return (min(p[0] for p in polygon),min(p[1] for p in polygon),max(p[0] for p in polygon),max(p[1] for p in polygon))


def _surface_bounds_overlap(a: tuple, b: tuple) -> bool:
    return a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]


@lru_cache(maxsize=256)
def _surface_support_edges(ring: tuple, orientation) -> tuple:
    """🪶 Cache exact supporting half-planes without assuming a source ring is convex."""
    result=[]
    for a,b in zip(ring,ring[1:]+ring[:1]):
        if a==b:
            continue
        signs=[orientation(a,b,p) for p in ring]
        if min(signs)>=0 and max(signs)>0:
            result.append((a,b,1))
        elif max(signs)<=0 and min(signs)<0:
            result.append((a,b,-1))
    return tuple(result)


def _surface_separated(left: list, right: list) -> bool:
    """🛑 Preserve original material when an exact support plane proves no positive planar intersection."""
    orientation=_analysis._orientation
    return any(all(side*orientation(a,b,p)<=0 for p in other)
               for ring,other in ((left,right),(right,left))
               for a,b,side in _surface_support_edges(tuple(tuple(p) for p in ring),orientation))


def _surface_difference(pieces: list, voids: list) -> list:
    """🪓 Subtract only holes whose bounding boxes can remove occupied area."""
    for void in voids:
        bounds = _surface_bounds(void)
        pieces = [region for piece in pieces for region in
                  (_analysis._subtract(piece,void,digits=12,tolerance=1e-12) if _surface_bounds_overlap(_surface_bounds(piece),bounds) and not _surface_separated(piece,void) else [piece])]
    return [p for p in pieces if len(p) >= 3 and _area(p) > 1e-12]


def _slab_surface(outline: list, bounds: list, voids: list, exact: bool = False) -> list:
    """🪧 Preserve original binary64 slab boundaries through exact rational profile operations."""
    def profiles(ring):
        """🪆 Reuse exact domain-neutral material decomposition rather than rounded polygon ears."""
        return _native_profile_regions(tuple(tuple(p)for p in ring))
    pieces=[_analysis._clip(left,right,digits=None)for left in profiles(outline)for right in profiles(bounds)]
    for void in voids:
        for cutter in profiles(void):
            pieces=[region for piece in pieces for region in _analysis._subtract(piece,cutter,digits=None,tolerance=0)]
    retained=[piece for piece in pieces if len(piece)>=3 and _area(piece)>0]
    return retained if exact else _native_public_value(retained)


def _slab_voids(model: dict, slab: dict) -> list:
    """🕳️ Resolve geometric holes and native slab-host opening occurrences."""
    voids = deepcopy(slab["geometry"]["voids"])
    bounds = _expand(model, slab["geometry"]["outline"])
    for opening in _by_class(model, "IfcOpeningElement"):
        if opening["geometry"].get("slab") != slab["tag"]:
            continue
        outline = _expand(model, opening["geometry"]["outline"])
        if _area(outline) <= 1e-9 or not math.isclose(sum(_area(p) for p in _slab_surface(outline, bounds, [])), _area(outline), abs_tol=1e-8):
            raise ValueError("Slab opening must lie inside its host outline")
        shared = set(opening["geometry"].get("shared_voids",[]))
        if not shared<={v["tag"] for v in slab["geometry"]["voids"]}:
            raise ValueError("Shared opening voids must name geometric holes in the same slab")
        overlaps = set()
        for void in voids:
            ring = _expand(model,void["outline"])
            if _surface_bounds_overlap(_surface_bounds(outline),_surface_bounds(ring)) and sum(_area(p) for p in _slab_surface(outline,ring,[]))>1e-9:
                overlaps.add(void["tag"])
        if overlaps!=shared:
            raise ValueError("Slab openings must declare every shared geometric hole and no absent intersections")
        voids.append({"tag":opening["tag"], "outline":outline})
    return voids


def _slab_opening_geometry(model: dict, opening: dict) -> dict:
    """🔳 Derive the actual removed regional material of a horizontal opening."""
    slab = _find(model, opening["geometry"]["slab"])
    if slab["class"] != "IfcSlab":
        raise ValueError("A slab opening requires an IfcSlab host")
    _slab_voids(model, slab)
    g, parts = slab["geometry"], []
    outline = _expand(model, opening["geometry"]["outline"])
    for region in g["regions"]:
        top = _z(model,g["level"]) + region["top_offset"]["value"]
        occupied = _slab_surface(_expand(model,region["outline"]),_expand(model,g["outline"]),[])
        for ring in occupied:
            for polygon in _slab_surface(outline,ring,[]):
                parts.append({"outline":polygon,"z0":top-region["thickness"]["value"],"z1":top,"region":region["tag"]})
    return {"tag":opening["tag"],"guid":opening["guid"],"host":slab["tag"],"storey":opening["storey"],"outline":outline,"parts":parts}


def _slab_finish_parts(slab, exact: bool = False):
    """🛖 Resolve a level roof finish over different structural slab depths."""
    paving = next((layer for layer in slab.get("layers", ()) if layer["id"] == "paving"), None)
    if paving is None:
        return []
    from fractions import Fraction
    top = _round(min(part["z1"] for part in slab["parts"]) + paving["thickness"])
    if exact:top=Fraction(top)
    if any(top < part["z1"] - 1e-9 for part in slab["parts"]):
        raise ValueError("Roof finish cannot be lower than the structural surface")
    return [{"outline": deepcopy(part["outline"]), "z0": part["z1"], "z1": top if exact else _round(top), "region": part["region"], "layer": "paving"}
            for part in slab["parts"] if top > part["z1"] + 1e-9]


_SLAB_GEOMETRIES = {}


def _slab_geometry_entry(model: dict, slab: dict, exact: bool = False, progress=None, cancel=None) -> dict:
    """♻️ Reuse a private occupied slab entry without exposing mutable memo data."""
    if cancel and cancel():raise InterruptedError("Native slab construction cancelled")
    producers = geometry_producers()
    cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
    context = cache.canonical_context(producers)
    identity = cache.digest(cache.encode(context[2])) if context else None
    key = (slab["tag"],exact,producers,_geometry_source_key(model),identity)
    if key not in _SLAB_GEOMETRIES:
        if exact and context:
            root, sources, slots, eligible = context
            model_snapshot, slab_snapshot = deepcopy(model), deepcopy(slab)
            folder = Path(__import__("os").environ.get("SEMIO_BIM_SLAB_CACHE", str(root / ".🧬semio" / "🎓️teaching" / "🏗️bim" / "slab")))
            report = (lambda phase: progress({"phase":"slab-cache","status":phase})) if progress else (lambda phase: None)
            build = lambda cancelled, _: _slab_geometry_uncached(model_snapshot,slab_snapshot,exact=True,progress=progress,cancel=cancelled)
            value = cache.get_or_compute(model_snapshot,slab_snapshot,root,sources,folder,build,eligibility=lambda: eligible(geometry_producers()),producer_slots=slots,cancelled=cancel or (lambda: False),progress=report)
        else:
            value = _slab_geometry_uncached(model,slab,exact=True,progress=progress,cancel=cancel) if exact else _native_public_value(_slab_geometry_entry(model,slab,exact=True,progress=progress,cancel=cancel))
        _SLAB_GEOMETRIES[key] = value
        if len(_SLAB_GEOMETRIES) > 24:
            _SLAB_GEOMETRIES.pop(next(iter(_SLAB_GEOMETRIES)))
    return _SLAB_GEOMETRIES[key]


def _slab_geometry(model: dict, slab: dict) -> dict:
    """🧺 Return an isolated complete slab geometry to each caller."""
    return deepcopy(_slab_geometry_entry(model,slab))


def _slab_geometry_uncached(model: dict, slab: dict, exact: bool = False, progress=None, cancel=None) -> dict:
    g = slab["geometry"]
    level = _z(model, g["level"])
    outline = _expand(model, g["outline"])
    resolved_voids = _slab_voids(model, slab)
    voids = [_expand(model, v["outline"]) for v in resolved_voids]
    parts, regions, reference_parts = [], [], []
    tags = set()
    for region_index,region in enumerate(g["regions"]):
        if cancel and cancel():raise InterruptedError("Native slab construction cancelled")
        if progress:progress({"phase":"slab-regions","completed":region_index,"total":len(g["regions"]),"tag":slab["tag"]})
        tag = region["tag"]
        if tag in tags:
            raise ValueError("Slab region tags must be unique")
        tags.add(tag)
        thickness = _number(region["thickness"]["value"], "Region thickness")
        top = level + _number(region["top_offset"]["value"], "Region top offset")
        if thickness <= 0:
            raise ValueError("Slab region thickness must be positive")
        ring = _expand(model, region["outline"])
        gross = _slab_surface(ring, outline, [],exact=True)
        candidates = [(polygon,_surface_bounds(polygon)) for polygon in gross]
        for polygon,bounds in candidates:
            for other,other_bounds in reference_parts:
                if _surface_bounds_overlap(bounds,other_bounds) and _area(_analysis._clip(polygon,other)) > 1e-7:
                    raise ValueError("Slab regions overlap in reference surface")
        reference_parts.extend(candidates)
        occupied = _slab_surface(ring,outline,voids,exact=True)
        for polygon in occupied:
            part = {"outline": polygon, "voids": [], "top": _round(top), "thickness": thickness,
                    "z0": _round(top - thickness), "z1": _round(top), "region": tag}
            for key in ("material", "member"):
                if key in region:
                    part[key] = region[key]
            parts.append(part)
        regions.append({"tag": tag, "outline": ring, "top": _round(top), "thickness": thickness,
                        "gross_area": sum(_area(p) for p in gross), "area": sum(_area(p) for p in occupied),
                        **{key: region[key] for key in ("material", "member") if key in region}})
    reference_area = sum(_area(p) for p in _slab_surface(outline,outline,[]))
    if not math.isclose(sum(r["gross_area"] for r in regions),reference_area,rel_tol=1e-8,abs_tol=1e-7):
        raise ValueError("Slab regions must partition the reference outline")
    area = sum(r["area"] for r in regions)
    expected = sum(_area(p) for p in _slab_surface(outline, outline, voids))
    if not parts or not math.isclose(area, expected, rel_tol=1e-8, abs_tol=1e-7):
        raise ValueError("Slab regions must partition the occupied outline")
    top = level + g["top_offset"]["value"]
    if not math.isclose(top, max(p["top"] for p in parts), abs_tol=1e-8):
        raise ValueError("Slab top offset must identify its highest structural surface")
    out = {"tag": slab["tag"], "guid": slab["guid"], "kind": slab["predefined_type"], "storey": slab["storey"], "level": g["level"], "outline": outline,
           "voids": voids, "void_tags": [v["tag"] for v in resolved_voids], "top": _round(top),
           "thickness": _round(_slab_thickness(model, slab)), "parts": parts, "regions": regions}
    if "composite" in g:
        out["composite"] = deepcopy(g["composite"])
    layerset = _material(model, slab)
    if layerset is not None and "layers" in layerset:
        out["layers"] = [{"id": l["id"], "name": l["name"], "thickness": l["thickness"]["value"], "zones": l.get("zones", []),
                          "density": _find(model, l["material"])["density"]["value"]} for l in layerset["layers"]]
        out["total_thickness"] = _round(sum(l["thickness"]["value"] for l in layerset["layers"]))
    if "zones" in g:
        out["zones"] = {name: _expand(model, _find(model, tag)["outline"]) for name, tag in g["zones"].items()}
    out["finish_parts"] = _slab_finish_parts(out,exact=True)
    out.update({"full_parts":deepcopy(out["parts"]),"full_finish_parts":deepcopy(out["finish_parts"]),"shared_parts":[],"shared_finish_parts":[]})
    if g.get("interface_owners"):
        out["parts"],out["shared_parts"] = _structural_joint_backend().reserve_interfaces(SimpleNamespace(**globals()),model,slab,out["parts"])
    if g.get("column_reserves"):
        out["parts"],column_shared,voids = _column_partition(model,slab,out["parts"],g["column_reserves"],exact=True)
        out["shared_parts"] += column_shared
        if out["finish_parts"]:
            out["finish_parts"],out["shared_finish_parts"],_ = _column_partition(model,slab,out["finish_parts"],g["column_reserves"],require_contact=False,exact=True)
        out["parts"] = [{**p,"top":p["z1"],"thickness":p["z1"]-p["z0"]} for p in out["parts"]]
        out["full_parts"] = _subtract_prisms(out["full_parts"],voids) if voids else out["full_parts"]
    if g.get("finish_voids"):
        out["finish_parts"],shared = _finish_reservations(model,slab,out["finish_parts"],exact=True,progress=progress,cancel=cancel)
        out["shared_finish_parts"] += shared
        out["finish_void_tags"] = [v["tag"] for v in g["finish_voids"]]
    return out if exact else _native_public_value(out)

def _native_parts_exact_volume(parts: list):
    """🎲 Integrate original closed native cells with exact local tetrahedron determinants."""
    from fractions import Fraction
    result=Fraction(0)
    for part in parts:
        vertices=[tuple(Fraction(v)for v in p)for p in part_vertices(part)]
        origin=vertices[0];total=Fraction(0)
        for face in part_faces(part):
            for i in range(1,len(face)-1):
                a,b,c=[tuple(vertices[j][k]-origin[k]for k in range(3))for j in (face[0],face[i],face[i+1])]
                total+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0])
        result+=abs(total)/6
    return result


def _native_parts_have_volume(parts: list) -> bool:
    """🔬 Detect any strictly positive native cell without a geometric tolerance."""
    return any(_native_parts_exact_volume([part])for part in parts)


def _finish_reservations(model: dict, slab: dict, parts: list, progress=None, cancel=None, exact: bool = False) -> tuple:
    """🎟️ Assign actual three-dimensional finish reserves to disjoint explicitly declared physical owners."""
    if not parts:raise ValueError("A finish reserve requires occupied finish material")
    backend=surface_finishes();host=[]
    for part in parts:
        material=part.get("material")
        if material is None:
            layers=_material(model,slab)
            material=next((layer["material"]for layer in layers.get("layers",())if layer["id"]==part.get("layer")),None)if layers else None
        if material is None:raise ValueError("A finish reserve requires explicit host layer material")
        host.extend(_native_material_cells({**part,"material":material,"role":part.get("role","finish")}))
    low,high=min(p["z0"]for p in parts),max(p["z1"]for p in parts)
    owners={};reserves=[];shared=[];shared_bounds=[];tags=set();records={};boxes={};indices={}
    def record(part):
        """🗃️ Index immutable operation-local geometry without retaining another model or render object."""
        key=id(part)
        if key not in records:
            from fractions import Fraction
            vertices=[tuple(Fraction(v)for v in p)for p in part_vertices(part)];planes=[]
            for face in part_faces(part):
                a=vertices[face[0]]
                for i in range(1,len(face)-1):
                    u,v=[tuple(vertices[j][k]-a[k]for k in range(3))for j in (face[i],face[i+1])]
                    n=(u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0])
                    if any(n):break
                else:raise ValueError("Native finish cells require nondegenerate faces")
                offset=sum(n[k]*a[k]for k in range(3));values=[sum(n[k]*p[k]for k in range(3))-offset for p in vertices]
                if max(values)>0:
                    if min(values)<0:raise ValueError("Native finish cells require convex material")
                    n=tuple(-v for v in n);offset=-offset
                planes.append((n,offset))
            if len(records)>=4096:records.pop(next(iter(records)))
            records[key]=(part,tuple((min(p[k]for p in vertices),max(p[k]for p in vertices))for k in range(3)),vertices,planes)
        return records[key]
    def box(part):
        """🧰 Retain original numeric bounds as a conservative operation-local broad phase."""
        key=id(part)
        if key not in boxes:
            if len(boxes)>=16384:boxes.pop(next(iter(boxes)))
            boxes[key]=(part,_material_bounds(part))
        return boxes[key][1]
    def candidates(tag,target):
        """📑 Query exact native intervals without replacing any occupied geometry."""
        from bisect import bisect_left
        if tag not in indices:
            rows=sorted(((box(part)[0][0],index,part)for index,part in enumerate(owners[tag][1])),key=lambda row:(row[0],row[1]))
            indices[tag]=([row[0]for row in rows],rows)
        lows,rows=indices[tag];target_box=box(target)
        return (part for _,_,part in rows[:bisect_left(lows,target_box[0][1])]if all(x[0]<y[1]and y[0]<x[1]for x,y in zip(box(part),target_box)))
    def overlap(a,b):
        """🔀 Reject only exact disjoint native bounding intervals."""
        return all(x[0]<y[1]and y[0]<x[1]for x,y in zip(box(a),box(b)))
    def contains(outer,inner):
        """🥡 Prove complete native-cell containment without any geometric tolerance."""
        return all(sum(n[k]*p[k]for k in range(3))<=offset for n,offset in record(outer)[3]for p in record(inner)[2])
    def intersect(left,right):
        """🎯 Intersect original convex native cells before publishing any clipped coordinate."""
        if not overlap(left,right):return []
        if contains(right,left):return [deepcopy(left)]
        if contains(left,right):return [{**{k:deepcopy(v)for k,v in left.items()if k not in ("outline","holes","top_outline","section","mesh","z0","z1")},"mesh":deepcopy(right["mesh"])}]
        return _reference_material_parts(left,[],[right],progress=progress,cancel=cancel)
    def resolve(tag):
        """🎚️ Expand only authored covering displacement owners without aggregating borrowed references."""
        if tag==slab["tag"]:raise ValueError("A finish reserve requires independent physical owners")
        if tag in owners:return
        if cancel and cancel():raise InterruptedError("Native finish reservations cancelled")
        if progress:progress({"phase":"finish-owner","tag":tag,"status":"started"})
        obj=_find(model,tag);g=obj["geometry"]
        if obj["class"]in ("IfcRamp","IfcRampFlight","IfcColumn","IfcSlab"):
            actual=_interface_owner_parts(model,tag)
        elif "parts"in g or "layer_binding"in g or g.get("kind")=="ramp_finish":
            actual=_solid_geometry(model,obj,resolve_embedded=False,progress=progress,cancel=cancel,exact=exact)["parts"]
        else:raise ValueError("A finish reserve needs independent native material owners")
        fallback=_material(model,obj);fallback=None if fallback is None else fallback["tag"]
        if cancel and cancel():raise InterruptedError("Native finish reservations cancelled")
        normalized=[]
        for part_index,part in enumerate(actual):
            if cancel and cancel():raise InterruptedError("Native finish reservations cancelled")
            try:cells=_native_material_cells(part)
            except ValueError as error:raise ValueError(f"Finish owner {tag} native part {part_index}: {error}")from error
            normalized.extend({**cell,"material":cell.get("material",fallback),"role":cell.get("role","finish_owner")}for cell in cells)
        owners[tag]=(obj,normalized)
        if progress:progress({"phase":"finish-owner","tag":tag,"status":"completed","cells":len(normalized)})
    for index,reserve in enumerate(slab["geometry"]["finish_voids"]):
        if cancel and cancel():raise InterruptedError("Native finish reservations cancelled")
        if progress:progress({"phase":"finish-reservations","completed":index,"total":len(slab["geometry"]["finish_voids"]),"tag":reserve["tag"]})
        if reserve["tag"]in tags:raise ValueError("Finish reserve tags must be unique")
        tags.add(reserve["tag"]);bounds=_native_material_cells({"outline":_expand(model,reserve["outline"]),"z0":low,"z1":high,"material":host[0]["material"],"role":"finish_reserve_extent"})
        targets=[cell for part in host for boundary in bounds for cell in intersect(part,boundary)]
        if not _native_parts_have_volume(targets):raise ValueError("A finish reserve requires occupied target material")
        earlier={id(target):[part for part,other in shared_bounds if all(a[0]<b[1]and b[0]<a[1]for a,b in zip(box(target),other))]for target in targets}
        expected=[cell for target in targets for cell in _reference_material_parts(target,earlier[id(target)],progress=progress,cancel=cancel)]
        owner_tags=[reserve["owner"],*reserve.get("joint_owners",())]
        if len(owner_tags)!=len(set(owner_tags)):raise ValueError("Finish joint owners must be distinct")
        cursor=0
        while cursor<len(owner_tags):
            tag=owner_tags[cursor];resolve(tag);obj=owners[tag][0];cursor+=1
            if obj["class"]=="IfcCovering":
                refs=[*obj["geometry"].get("interface_owners",()),*obj["geometry"].get("ramp_finish",{}).get("interface_owners",())]
                owner_tags.extend(key for key in refs if key not in owner_tags)
        common=[]
        for tag in owner_tags:
            obj,native=owners[tag]
            for target in targets:
                for part in candidates(tag,target):
                    for cell in intersect(part,target):
                        if cell.get("material")is None:raise ValueError("Finish joint material must be explicit")
                        for other in [*earlier[id(target)],*common]:
                            if other["owner"]!=tag and _native_parts_have_volume(intersect(cell,other)):
                                raise ValueError("Finish joint owners share occupied material")
                        prior=[other for other in [*earlier[id(target)],*common]if other["owner"]==tag and overlap(cell,other)]
                        for retained in _reference_material_parts(cell,prior,progress=progress,cancel=cancel)if prior else [cell]:
                            common.append({**retained,"owner":tag,"owner_guid":obj["guid"],"role":"finish_joint","joint_host":slab["tag"]})
        missing=_native_parts_exact_volume(expected)-_native_parts_exact_volume(common)
        if missing:raise ValueError(f"Finish reserve {reserve['tag']} is not fully occupied by its declared physical owners: exact missing volume {missing!r} m3")
        shared.extend(common);shared_bounds.extend((part,box(part))for part in common);reserves.extend(bounds)
    remaining=[cell for part in host for cell in _reference_material_parts(part,[boundary for boundary in reserves if overlap(part,boundary)],progress=progress,cancel=cancel)]
    publish=lambda cells:[backend.publish_stock(backend.clip_stock(cell,[],exact=True))for cell in cells]
    return (remaining,shared) if exact else (publish(remaining),publish(shared))


def _service(model: dict, e: dict) -> dict:
    if "parts" in e["geometry"]:
        if any(key in e["geometry"] for key in ("points","tube","size")):
            raise ValueError("A native composite service cannot also be a tube or equipment envelope")
        out = _solid_geometry(model,e)
        out["parts"] = [{**p,"material":p.get("material",e.get("material"))} for p in out["parts"]]
        out.update({"system":e.get("system"),"volume":_occupied_volume(out["parts"])})
        if "historical_evidence" in e:
            out["historical_evidence"] = deepcopy(e["historical_evidence"])
        return out
    kinds = {"IfcBoiler": "boiler", "IfcSpaceHeater": "radiator", "IfcWasteTerminal": "drain"}
    kind = e["geometry"].get("kind") or kinds.get(e["class"]) or ("downpipe" if e.get("system") == "rainwater" else "pipe")
    points = [[p[0], p[1], _z(model, p[2])] for p in e["geometry"]["points"]]
    out = {"tag": e["tag"], "guid": e["guid"], "class": e["class"], "storey": e["storey"], "kind": kind, "system": e.get("system"), "points": points, "parts": []}
    if "size" in e["geometry"]:
        sx, sy, sz = e["geometry"]["size"]
        out["size"] = [sx, sy, sz]
        x, y = [sum(p[i] for p in points) / len(points) for i in (0, 1)]
        z0 = min(p[2] for p in points)
        out["parts"].append({"outline": [[x - sx / 2, y - sy / 2], [x + sx / 2, y - sy / 2], [x + sx / 2, y + sy / 2], [x - sx / 2, y + sy / 2]],
                             "z0": z0, "z1": max(max(p[2] for p in points), z0 + sz)})
    if "tube" in e["geometry"]:
        diameter = e["attributes"]["diameter"]["value"]
        tube = e["geometry"]["tube"]
        thickness,segments = tube["wall_thickness"]["value"],tube["segments"]
        if len(points)!=2 or not 0 < thickness < diameter/2 or segments < 24 or segments%4:
            raise ValueError("A native hollow service needs two endpoints and a valid tube wall")
        a,b = points
        axes = [i for i in range(3) if abs(a[i]-b[i])>1e-10]
        if len(axes)!=1:
            raise ValueError("A native service segment needs one orthogonal straight axis")
        axis = axes[0]
        low,high = sorted([a[axis],b[axis]])
        transverse = [i for i in range(3) if i!=axis]
        centre = [a[i] for i in transverse]
        outer,inner = diameter/2,diameter/2-thickness
        for i in range(segments):
            angles = [2*math.pi*i/segments,2*math.pi*(i+1)/segments]
            profile = [[centre[0]+r*math.cos(t),centre[1]+r*math.sin(t)] for r,t in ((outer,angles[0]),(outer,angles[1]),(inner,angles[1]),(inner,angles[0]))]
            if "clip_start" in tube or "clip_end" in tube:
                vertices = []
                for endpoint,point in enumerate((a,b)):
                    clip = tube.get("clip_start" if endpoint==0 else "clip_end")
                    for u,v in profile:
                        vertex = list(point)
                        vertex[transverse[0]],vertex[transverse[1]] = u,v
                        if clip:
                            normal,origin = clip["normal"],clip["point"]
                            if abs(normal[axis])<1e-12:
                                raise ValueError("A tube end plane must intersect its axis")
                            vertex[axis] = origin[axis]-sum(normal[k]*(vertex[k]-origin[k]) for k in transverse)/normal[axis]
                        vertices.append(vertex)
                if any((b[axis]-a[axis])*(vertices[4+j][axis]-vertices[j][axis])<=1e-12 for j in range(4)):
                    raise ValueError("Tube end planes must retain a positive occupied segment")
                part = _polyhedron_part(vertices)
            elif axis==2:
                part = {"outline":profile,"z0":low,"z1":high}
            else:
                part = {"section":{"plane":"YZ" if axis==0 else "XZ","profile":profile,"sweep":[low,high]},"z0":min(p[1] for p in profile),"z1":max(p[1] for p in profile)}
                part["outline"] = part_footprint(part)
            out["parts"].append({**part,"material":e.get("material_tag",e.get("material")),"role":"tube_wall"})
        out.update({"diameter":diameter,"inner_diameter":diameter-2*thickness,"wall_thickness":thickness,"length":high-low,"volume":sum(_polyhedron_volume(p) for p in out["parts"]) if any("mesh" in p for p in out["parts"]) else segments*math.sin(2*math.pi/segments)/2*(outer**2-inner**2)*(high-low),"axis":axis})
    if not out["parts"]:
        raise ValueError("A native service requires occupied service geometry")
    if "historical_evidence" in e:
        out["historical_evidence"] = deepcopy(e["historical_evidence"])
    return out


def service_geometry(key: str, model: dict | None = None) -> dict:
    """🚰 Resolve equipment or a native hollow service segment without leaking a tool-specific type."""
    return _geometry_query("service", key, model)


def _column_partition(model: dict, obj: dict, parts: list, owners: list, require_contact: bool = True, exact: bool = False) -> tuple:
    """🏯 Assign exact native occupied common material and retain actual column bore air."""
    if len(set(owners))!=len(owners) or obj["tag"] in owners:raise ValueError("Column reservations require distinct independent owners")
    cutters,shared,voids = [],[],[]
    material = _material(model,obj)["tag"]
    stocks=[{**deepcopy(p),"material":p.get("material",material),"role":p.get("role","structural_member")} for p in parts]
    for tag in owners:
        owner = _find(model,tag)
        if owner["class"]!="IfcColumn":raise ValueError("A material reserve owner must be a column")
        columns=column_geometry(model,owner)["parts"];common=[]
        owner_material=_material(model,owner)["tag"]
        for part in stocks:
            box=_material_bounds(part)
            for column in columns:
                other=_material_bounds(column)
                if not all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,other)):continue
                bounds=_native_material_cells(column)
                for cell in bounds:
                    common.extend(_reference_material_parts(part,[],bounds=[cell]))
        if not common:
            if require_contact:raise ValueError(f"Material reserve owner {tag} has no occupied intersection with {obj['tag']}")
            continue
        shared.extend({**p,"owner":tag,"owner_guid":owner["guid"],"material":owner_material,"role":"column_joint","joint_host":obj["tag"]} for p in common)
        cutters.extend(columns)
        voids.extend({"outline":deepcopy(hole),"z0":p["z0"],"z1":p["z1"]} for p in columns for hole in p.get("holes",()))
    owned=[cell for part in stocks for cell in _reference_material_parts(part,[*cutters,*voids])]
    def publish(items):
        """📤 Publish the exact native construction once, retaining closed positive physical cells."""
        return [{**p,**_polyhedron_part(part_vertices(p),part_faces(p))} for p in (deepcopy(items) if exact else _native_public_value(items))]
    return publish(owned),publish(shared),deepcopy(voids) if exact else _native_public_value(voids)


def _analytic_solids_backend():
    """➖️ Resolve semantic stock difference geometry behind the first-party solid interface."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️")


def _analytic_operand(part: dict) -> dict:
    """📈 Retain original analytic profiles and native topology without display conversion."""
    affine="csg"not in part and any(k in part for k in ("section","top_outline"))
    return _analytic_solids_backend().operand(part,part_vertices(part)if affine else None,part_faces(part)if affine else None)


def _analytic_input(part: dict) -> dict:
    """🧪 Normalize an affine physical operand without publishing any generated coordinate."""
    if "csg"not in part and any(k in part for k in ("section","top_outline")):
        return {**_analytic_solids_backend()._metadata(part),"csg":_analytic_operand(part)}
    return part


def _analytic_bounds(part: dict) -> tuple:
    """🔭 Convert exact analytic extrema outward only when native Float64 cannot represent them."""
    from fractions import Fraction
    result=[]
    for low,high in _analytic_solids_backend().bounds(_analytic_input(part)):
        lo,hi=float(low),float(high)
        if Fraction(lo)>low:lo=math.nextafter(lo,-math.inf)
        if Fraction(hi)<high:hi=math.nextafter(hi,math.inf)
        result.append((lo,hi))
    return tuple(result)


def part_volume(part: dict) -> float:
    """🥤 Measure actual semantic or native material from its declared geometry authority."""
    if "csg"in part:return _analytic_solids_backend().measure(part)["volume"]
    if part.get("profile")and"mesh"not in part:return _profile_moments(part["profile"])["area"]*(part["z1"]-part["z0"])
    return _polyhedron_volume(part)


def _structural_joint_backend():
    """🦴 Resolve declared same-material structural ownership behind the native geometry interface."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️structure.🔗️joints.🐍️")


def _flooring_layer_backend():
    """🥪 Resolve editable exclusive flooring layers behind the native geometry interface."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧱️layers.🐍️")


def _ramp_finish_backend():
    """🛹 Resolve original ramp surface construction behind the native model interface."""
    return importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🛝️ramps")


def _ramp_finish_reference_parts(model: dict, obj: dict) -> list:
    """🫂 Share original exact physical floor operands with receiving material construction."""
    return _geometry_query("ramp_finish_reference",obj["tag"],model)


def _solid_stock_parts(model: dict, obj: dict, raw_parts: list, exact: bool = False) -> list:
    """🪵 Normalize source stocks while retaining original native datums and exact bound-layer coordinates."""
    from fractions import Fraction
    base=_z(model,obj["geometry"]["base_level"]);parts=[]
    def height(value,rounded=False):
        """🔖 Retain private local coordinates while preserving resolved source Float64 datums."""
        if exact and isinstance(value,Fraction):return Fraction(base)+value
        return _round(base+value)if rounded else base+value
    for raw in raw_parts:
        part=deepcopy(raw)
        if "csg"in part:
            parts.append(_analytic_solids_backend().translate(part,[0,0,base]));continue
        if "mesh" in part:
            part.update(_polyhedron_part([[v[0],v[1],height(v[2])]for v in part["mesh"]["vertices"]],part["mesh"]["faces"]))
        elif "section" in part:
            section=part["section"];profile=section["profile"]
            if section["sweep"][1]<=section["sweep"][0]or _area(profile)<=1e-12 or not math.isclose(_area(profile),_area(_convex_hull(profile)),rel_tol=1e-10,abs_tol=1e-12):
                raise ValueError("A section sweep needs a convex positive profile and increasing sweep interval")
            section["profile"]=[[h,height(z)]for h,z in profile]
            part["z0"],part["z1"]=min(p[1]for p in section["profile"]),max(p[1]for p in section["profile"])
            part["outline"]=part_footprint(part)
        else:part["z0"],part["z1"]=height(part["z0"],True),height(part["z1"],True)
        parts.append(part)
    return parts


def _solid_geometry(model: dict, obj: dict, resolve_embedded: bool = True, progress=None, cancel=None, exact: bool = False) -> dict:
    """🥣 Resolve exclusive material and optionally its native embedded reference parts."""
    g = obj["geometry"]
    if g.get("kind")=="ramp_finish":return _ramp_finish_backend().geometry(SimpleNamespace(**globals()),model,obj)
    if "section_sweep" in g:
        facade = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️facade.🐍️")
        return {"tag":obj["tag"],"guid":obj["guid"],"class":obj["class"],"storey":obj["storey"],"kind":g["kind"],"name":deepcopy(obj["name"]),"domains":deepcopy(obj["metadata"]["domains"]),"parts":facade.owned_parts(SimpleNamespace(**globals()),model,obj),"shared_parts":[]}
    raw_parts=_flooring_layer_backend().parts(SimpleNamespace(**globals()),model,obj,progress=progress,cancel=cancel,exact=True)if "layer_binding"in g else g["parts"]
    parts=_solid_stock_parts(model,obj,raw_parts,exact="layer_binding"in g)
    surface_finishes().apply_material_appearance(parts,lambda tag:_find(model,tag),obj.get("material"))
    for part in parts:
        if "top_outline" in part:
            part_vertices(part)
    if obj["class"]=="IfcFooting" and g.get("interface_owners"):
        refs = g["interface_owners"]
        if len(set(refs))!=len(refs) or obj["tag"] in refs or any(_find(model,key)["class"] not in ("IfcColumn","IfcBeam","IfcFooting","IfcBuildingElementProxy") for key in refs):
            raise ValueError("A footing joint requires distinct independent native structural owners")
        parts,_ = _subtract_native_material(_interface_reference_parts(model,obj["tag"]),[part for key in refs for part in _interface_reference_parts(model,key)])
        surface_finishes().apply_material_appearance(parts,lambda tag:_find(model,tag),obj.get("material"))
    slabs = [_slab_geometry_entry(model,_find(model,tag)) for tag in (g.get("embedded_slabs",()) if resolve_embedded else ())]
    shared = []
    for slab in slabs:
        for part in slab["parts"]+slab.get("shared_parts",[]):
            if part.get("member")==obj["tag"]:
                owner=_find(model,part.get("owner",slab["tag"]))
                shared.append({**deepcopy(part),"owner":owner["tag"],"owner_guid":owner["guid"],"role":part.get("role","embedded_slab_reference")})
    slab_full = [deepcopy(p) for slab in slabs for p in slab.get("full_parts",slab["parts"]) if p.get("member")==obj["tag"]]
    full = deepcopy(parts)
    if "layer_binding"in g:
        full=_solid_stock_parts(model,obj,g["parts"])
        surface_finishes().apply_material_appearance(full,lambda tag:_find(model,tag),obj.get("material"))
    if obj["class"]=="IfcCovering" and g.get("interface_owners"):
        blockers=_declared_material_interfaces(model,obj)
        if exact:parts=[cell for part in parts for cell in _reference_material_parts(part,blockers)]
        else:parts,_ = _subtract_native_material(parts,blockers)
    voids = []
    if g.get("joint_owners"):
        parts,joints,voids = _column_partition(model,obj,parts,g["joint_owners"])
    else:
        joints = []
    if g.get("voids"):
        lo,hi = min(p["z0"] for p in full),max(p["z1"] for p in full)
        reserved = [{"outline":_expand(model,v["outline"]),"z0":lo,"z1":hi} for v in g["voids"]]
        def ring_key(p):
            return sorted((round(x,9),round(y,9)) for x,y in p["outline"])
        voids = [p for p in voids if not any(ring_key(p)==ring_key(v) and v["z0"]<=p["z0"] and v["z1"]>=p["z1"] for v in reserved)]
        parts = _subtract_native_material([{**p,"material":p.get("material",_material(model,obj)["tag"])} for p in parts],reserved)[0]
        voids += reserved
    full = (_subtract_native_material([{**p,"material":p.get("material",_material(model,obj)["tag"])} for p in full],voids)[0] if voids else full)+slab_full
    shared += joints
    issues = []
    if g.get("structural_joints"):
        parts,shared,issues = _structural_joint_backend().partition(SimpleNamespace(**globals()),model,obj,parts,shared,progress=progress,cancel=cancel,exact=exact)
    result={"tag": obj["tag"], "guid": obj["guid"], "class": obj["class"], "storey": obj["storey"],
            "kind": g["kind"], "predefined_type":obj.get("predefined_type"), "name": deepcopy(obj["name"]), "domains":deepcopy(obj["metadata"]["domains"]), "parts": parts, "shared_parts": shared, "full_parts": full, "construction_issues":issues}
    return result if exact else _native_public_value(result)


def reference_parts(key: str, model: dict | None = None) -> list:
    """🧾 Expose isolated original full structural product stock independently of owned and assembly material."""
    source=_m(model);obj=_find(source,key);cls=obj["class"]
    if "interface_construction"in obj["geometry"]:
        parts=[_ramp_stringer_reference_stock(source,obj)]if obj["geometry"]["interface_construction"]["kind"]=="ramp_stringer"else _guard_reference_stocks(source,obj)
    elif cls in ("IfcRamp","IfcRampFlight"):parts=_ramp_nominal_parts(source,obj)
    elif cls=="IfcColumn":parts=column_geometry(source,obj)["parts"]
    elif cls=="IfcSlab":parts=_slab_geometry_entry(source,obj)["full_parts"]
    elif cls=="IfcStair":parts=_stair_geometry(source,obj)["full_parts"]
    elif cls=="IfcFooting"and obj["geometry"].get("interface_owners")and not obj["geometry"].get("voids"):parts=_interface_reference_parts(source,key)
    elif "parts"in obj.get("geometry",{}):
        geometry=solid_geometry(key,source);parts=geometry.get("full_parts",geometry["parts"])
    else:raise ValueError("Full structural reference requires an independently authored native product")
    return deepcopy(_native_public_value(parts))


def stair_geometry(key: str, model: dict | None = None) -> dict:
    """🪜 Resolve every native stair material part without substituting a tread envelope."""
    source = _m(model)
    obj = _find(source,key)
    if obj["class"]!="IfcStair":
        raise ValueError("Stair geometry requires a native stair product")
    return _stair_geometry(source,obj)


def _stair_geometry(model: dict, obj: dict, progress=None, cancel=None, exact: bool = False) -> dict:
    g = obj["geometry"]
    z0, z1 = _z(model, g["from_level"]), _z(model, g["to_level"])
    def placed(raw, base):
        part = deepcopy(raw)
        if exact or g.get("structural_joints"):
            from fractions import Fraction
            base=Fraction(base)
            if "mesh" in raw:part["mesh"]["vertices"]=[[Fraction(v)for v in point]for point in raw["mesh"]["vertices"]]
        if "mesh" in raw:
            part.update(_polyhedron_part([[v[0],v[1],base+v[2]] for v in part["mesh"]["vertices"]],part["mesh"]["faces"]))
        else:
            part["outline"] = _expand(model,raw["outline"])
            part["z0"],part["z1"] = _round(base+raw["z0"]),_round(base+raw["z1"])
        return part
    steps = [placed(part,z0) for part in g["step_parts"]]
    landings = [placed(part,z1) for part in g["landing_parts"]]
    structural = [placed(part,z0) for part in g["structural_parts"]]
    nosings = [placed(part,z0) for part in g.get("nosing_parts",())]
    finishes = _subtract_prisms(steps,nosings) if nosings else deepcopy(steps)
    full=deepcopy(finishes+nosings+landings+structural);shared=[];issues=[]
    if g.get("structural_joints"):
        structural,shared,issues=_structural_joint_backend().partition(SimpleNamespace(**globals()),model,obj,structural,[],progress=progress,cancel=cancel,exact=exact)
    result = {"tag":obj["tag"],"guid":obj["guid"],"class":obj["class"],"storey":obj["storey"],"kind":g["kind"],
            "centre":deepcopy(g["centre"]),"radius":g["radius"],"inner_radius":g["inner_radius"],"domains":deepcopy(obj["metadata"]["domains"]),
            "base":z0,"top":z1,"name":deepcopy(obj["name"]),"steps":steps,"landings":landings,"structural_parts":structural,"finish_parts":finishes,"nosings":nosings,"parts":finishes+nosings+landings+structural,"full_parts":full,"shared_parts":shared,"construction_issues":issues}
    return result if exact else _native_public_value(result)


def _profile_moments(profile: dict) -> dict:
    """🎡 Subtract native void area and parallel-axis moments before rotating the section."""
    shape = profile["shape"]
    if shape == "circle":
        d = profile["diameter"]["value"]
        area,x,y,xx,yy,xy = math.pi*d*d/4,0.,0.,math.pi*d**4/64,math.pi*d**4/64,0.
    elif shape in ("rectangle","ellipse"):
        w,d = profile["width"]["value"],profile["depth"]["value"]
        area = w*d if shape == "rectangle" else math.pi*w*d/4
        x,y,xy = 0.,0.,0.
        xx,yy = (area*w*w/12,area*d*d/12) if shape == "rectangle" else (area*w*w/16,area*d*d/16)
    else:
        ring = profile["outline"]
        if sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))<0:
            ring = list(reversed(ring))
        terms = [(a,b,a[0]*b[1]-b[0]*a[1]) for a,b in zip(ring,ring[1:]+ring[:1])]
        area = sum(c for a,b,c in terms)/2
        x = sum((a[0]+b[0])*c for a,b,c in terms)/6
        y = sum((a[1]+b[1])*c for a,b,c in terms)/6
        xx = sum((a[0]**2+a[0]*b[0]+b[0]**2)*c for a,b,c in terms)/12
        yy = sum((a[1]**2+a[1]*b[1]+b[1]**2)*c for a,b,c in terms)/12
        xy = sum((2*a[0]*a[1]+a[0]*b[1]+b[0]*a[1]+2*b[0]*b[1])*c for a,b,c in terms)/24
    for hole in profile.get("voids",()):
        hx,hy = hole["offset"]
        r = hole["diameter"]["value"]/2
        ha,hi = math.pi*r*r,math.pi*r**4/4
        area,x,y,xx,yy,xy = area-ha,x-ha*hx,y-ha*hy,xx-hi-ha*hx*hx,yy-hi-ha*hy*hy,xy-ha*hx*hy
    if area <= 0:
        raise ValueError("Profile voids must leave positive occupied area")
    cx,cy = x/area,y/area
    ix,iy,ixy = yy-area*cy*cy,xx-area*cx*cx,xy-area*cx*cy
    angle = math.radians(profile.get("rotation",0))
    c,s = math.cos(angle),math.sin(angle)
    return {"area":area,"centroid":[c*cx-s*cy,s*cx+c*cy],"second_moments":{"x":s*s*iy+c*c*ix+2*s*c*ixy,"y":c*c*iy+s*s*ix-2*s*c*ixy,"xy":s*c*(iy-ix)+(c*c-s*s)*ixy}}


def profile_geometry(profile: dict, axis: list, segments: int = 96) -> dict:
    """🧿 Resolve a native section profile into its world outline and analytic measures."""
    shape = profile["shape"]
    rotation = math.radians(profile.get("rotation", 0))
    if shape in ("circle", "ellipse"):
        width = profile["diameter"]["value"] if shape == "circle" else profile["width"]["value"]
        depth = width if shape == "circle" else profile["depth"]["value"]
        points = [[width / 2 * math.cos(2 * math.pi * i / segments), depth / 2 * math.sin(2 * math.pi * i / segments)] for i in range(segments)]
        area = math.pi * width * depth / 4
        a, b = width / 2, depth / 2
        h = ((a - b) / (a + b)) ** 2
        perimeter = math.pi * (a + b) * (1 + 3 * h / (10 + math.sqrt(4 - 3 * h)))
    elif shape == "rectangle":
        width, depth = profile["width"]["value"], profile["depth"]["value"]
        points = [[-width / 2, -depth / 2], [width / 2, -depth / 2], [width / 2, depth / 2], [-width / 2, depth / 2]]
        area, perimeter = width * depth, 2 * (width + depth)
    elif shape == "polygon":
        points = deepcopy(profile["outline"])
        area, perimeter = _area(points), _length(points + points[:1])
        if sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(points, points[1:] + points[:1])) < 0:
            points.reverse()
    else:
        raise ValueError(f"Unknown column profile {shape}")
    c, s = math.cos(rotation), math.sin(rotation)
    outline = [[axis[0] + c * x - s * y, axis[1] + s * x + c * y] for x, y in points]
    out = {"axis":deepcopy(axis),"outline": outline, "area": area, "perimeter": perimeter, "profile": deepcopy(profile),
           "curve_error": max(width,depth)/2*(1-math.cos(math.pi/segments)) if shape == "ellipse" else 0.}
    if shape == "circle":
        out["radius"] = profile["diameter"]["value"] / 2
    holes = []
    centres = []
    for hole in profile.get("voids",()):
        hx,hy = hole["offset"]
        radius = hole["diameter"]["value"]/2
        centre = [axis[0]+c*hx-s*hy,axis[1]+s*hx+c*hy]
        if not _contains_point(outline,centre) or min(_segment_distance(centre,centre,a,b) for a,b in zip(outline,outline[1:]+outline[:1])) <= radius:
            raise ValueError("Profile void must be strictly inside its outer contour")
        if any(math.dist(centre,p) <= radius+r for p,r in centres):
            raise ValueError("Profile voids must be disjoint")
        centres.append((centre,radius))
        holes.append([[centre[0]+radius*math.cos(-2*math.pi*i/segments),centre[1]+radius*math.sin(-2*math.pi*i/segments)] for i in range(segments)])
    moments = _profile_moments(profile)
    out.update({"section_scope":"authored_geometry_net_of_voids","outer_area":area,"area":moments["area"],"void_area":area-moments["area"],"outer_perimeter":perimeter,"perimeter":perimeter+sum(math.pi*h["diameter"]["value"] for h in profile.get("voids",())),"centroid":[axis[i]+moments["centroid"][i] for i in (0,1)],"second_moments":moments["second_moments"],"holes":holes})
    return out


def _column_end_parts(part: dict, plane: dict, axis: list) -> list:
    """🪚 Clip the final uniform column section to a finite positive upper plane."""
    normal,origin=plane['normal'],plane['point']
    if len(normal)!=3 or len(origin)!=3 or any(isinstance(v,bool) or not isinstance(v,(int,float)) or not math.isfinite(v) for v in normal+origin) or abs(normal[2])<1e-12:
        raise ValueError('A column end plane must intersect its vertical extrusion')
    def height(point):
        """📉 Resolve the world elevation of one exact upper profile vertex."""
        return origin[2]-sum(normal[k]*(point[k]-origin[k]) for k in (0,1))/normal[2]
    boundaries=[part['outline'],*part.get('holes',())]
    heights=[height(p) for ring in boundaries for p in ring]
    if min(heights)<=part['z0'] or max(heights)>part['z1']+1e-9:
        raise ValueError('A column end plane must preserve a positive final section within its upper bound')
    rings=part_regions(part,(part['z0']+part['z1'])/2) if part.get('holes') else _analysis._convex_parts(part['outline'])
    result=[]
    for ring in rings:
        n=len(ring)
        vertices=[[*p,part['z0']] for p in ring]+[[*p,height(p)] for p in ring]
        faces=[list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]
        profile={'shape':'polygon','outline':[[p[0]-axis[0],p[1]-axis[1]] for p in ring]}
        result.append({**profile_geometry(profile,axis),**_polyhedron_part(vertices,faces),**{key:part[key] for key in ('role','material') if key in part}})
    return result


def column_geometry(model: dict, obj: dict, segments: int = 96) -> dict:
    """🏺 Resolve native column sections at their actual placements and elevations."""
    g = obj["geometry"]
    profile = _find(model, obj["type"])["profile"]
    sections = g.get("sections", [{"profile": profile, "base_level": g["base_level"], "top_level": g["top_level"]}])
    parts = [{**profile_geometry(section["profile"], section.get("axis", g["axis"]), segments),
              "z0": _z(model, section["base_level"]), "z1": _top(model, section["top_level"]),
              **({"role":section["role"],"material":(obj["material"] if isinstance(obj.get("material"),dict) else _material(model,obj))["tag"]} if "role" in section else {})} for section in sections]
    metrics = deepcopy(parts) if "end_plane" in g else None
    if metrics is not None:
        parts = parts[:-1]+_column_end_parts(parts[-1],g["end_plane"],g["axis"])
    x, y = g["axis"]
    out = {"tag": obj["tag"], "guid": obj["guid"], "storey": obj["storey"], "x": x, "y": y,
           "profile": deepcopy(profile), "parts": parts, "outline": deepcopy(parts[0]["outline"]),
           "base": min(p["z0"] for p in parts), "top": max(p["z1"] for p in parts),
           "bounding_radius": max(math.dist([x, y], p) for part in parts for p in part["outline"])}
    if metrics is not None:
        out.update(end_plane=deepcopy(g["end_plane"]),section_metrics=metrics,outline=deepcopy(metrics[0]["outline"]))
    if all(p["profile"]["shape"] == "circle" for p in (metrics or parts)):
        out["radius"] = max(p["profile"]["diameter"]["value"] / 2 for p in (metrics or parts))
    return out


def filling_geometry(key: str, model: dict | None = None) -> dict:
    """🪟 Resolve one native window, door or curtain through the shared physical filling interface."""
    m = _m(model)
    obj = _find(m,key)
    if obj["class"] in ("IfcWindow","IfcDoor"):
        return _filling_geometry(m,obj)
    if obj["class"]=="IfcCurtainWall":
        return _curtain_filling_geometry(m,obj)
    raise ValueError(f"No physical filling geometry for {key}")


def is_architectural_covering(obj: dict) -> bool:
    """🖌️ Classify owned architectural finishes without reclassifying shared-domain MEP casings."""
    domains=obj.get("metadata",{}).get("domains",[])
    return obj.get("class")=="IfcCovering" and "architecture" in domains and "services" not in domains and "system" not in obj


def _building_geometry_uncached(m: dict) -> dict:
    """🏗️ Build complete shared metric geometry without publishing a memo value."""
    length, width, height = _box(m)
    grid = m["grid"]
    pilotis = [column_geometry(m, e) for e in _by_class(m, "IfcColumn")]
    wall_junctions = _wall_junction_backend().context(SimpleNamespace(**globals()),m)
    wall_geometries = [_wall_geometry(m,w,junctions=wall_junctions) for w in _by_class(m,"IfcWall")]
    walls = [w for w in wall_geometries if "path" not in w]
    cw = _find(m, "CW-1")
    ramps = [_ramp_geometry(m,obj) for obj in _by_class(m,"IfcRamp")]
    curtain_filling = _curtain_filling_geometry(m,cw)
    first_ramp = ramps[0]
    spaces = [{"id": s["tag"], "tag": s["tag"], "guid": s["guid"], "name": s["name"], "outline": _expand(m, s["outline"]), "storey": s["storey"],
               "area": _round(_area(_expand(m, s["outline"]))), "predefined_type": s["predefined_type"]} for s in m["spaces"]]
    return {
        "roles": deepcopy(m["roles"]),
        "box": {"length": length, "width": width, "height": height},
        "north": [-2 ** -.5, -2 ** -.5],
        "storeys": _elevations(m),
        "grid": {"x": grid["x"], "y": grid["y"], "labels_x": grid["labels_x"], "labels_y": grid["labels_y"], "spacing": grid["spacing"]["value"]},
        "pilotis": pilotis,
        "slabs": [_slab_geometry(m, s) for s in _by_class(m, "IfcSlab")],
        "walls": walls,
        "fillings": [_filling_geometry(m,e) for e in m["elements"] if e["class"] in ("IfcWindow","IfcDoor")],
        "curtain_wall": curtain_filling,
        "ramp": {"tag": first_ramp["tag"], "guid": first_ramp["guid"], "ramps": ramps, "flights": [f for ramp in ramps for f in ramp["flights"]],
                 "landings": [l for ramp in ramps for l in ramp["landings"]], "width": first_ramp["width"],
                 "voids": {v["tag"]: _expand(m, v["outline"]) for s in _by_class(m, "IfcSlab") for v in s["geometry"]["voids"] if v["tag"].startswith("V-RAMP")}},
        "screens": [w for w in wall_geometries if "path" in w],
        "spaces": spaces,
        "stairs": [_stair_geometry(m, e) for e in _by_class(m, "IfcStair")],
        "details": [_solid_geometry(m, e) for e in m["elements"] if e["class"]=="IfcBuildingElementProxy" or is_architectural_covering(e)],
        "furnishings": [_solid_geometry(m, e) for e in _by_class(m, "IfcFurnishingElement")],
        "beams": [_solid_geometry(m, e) for e in _by_class(m, "IfcBeam")],
        "foundations": [_solid_geometry(m, e) for e in _by_class(m, "IfcFooting")],
        "railings": [_solid_geometry(m, e) for e in _by_class(m, "IfcRailing")],
        "services": [_service(m,e) for e in m["elements"] if e["class"] in ("IfcBoiler","IfcSpaceHeater","IfcSanitaryTerminal","IfcLightFixture","IfcWasteTerminal","IfcPipeSegment","IfcCovering","IfcChimney") and not is_architectural_covering(e)],
    }

_GEOMETRY_REVISIONS = _RevisionCounter()


@lru_cache(maxsize=2)
def _geometry_models_cached(source: str, producers: tuple) -> dict:
    """📸 Own two canonical model snapshots and their complete producer revision."""
    return {"model": json.loads(source), "values": {}, "lock": _GeometryLock(), "revision": next(_GEOMETRY_REVISIONS)}


def geometry_producers() -> tuple:
    """🧫 Bind native geometry memoization to every construction and numeric producer."""
    return (
        _polyhedron_volume,
        _analytic_solids_backend,
        *_analytic_solids_backend().producers(),
        _analytic_operand,
        _analytic_input,
        _analytic_bounds,
        part_volume,
        part_outline_at,
        part_faces,
        part_vertices,
        column_geometry,
        profile_geometry,
        _profile_moments,
        _column_end_parts,
        _column_partition,
        _part_pose_distance,
        _column_pose_distance,
        _building_geometry_uncached,
        _ramp_finish_reference_parts,
        _ramp_finish_backend().reference_stocks,
        _ramp_finish_backend().reference_parts,
        _ramp_finish_backend().geometry,
        _ramp_finish_backend().partitioned,
        _convex_hull,
        _filling_geometry,
        _curtain_filling_geometry,
        _filling_owned_parts,
        _filling_receiving_interval,
        _filling_structures,
        _filling_structure_snapshot,
        _service,
        _solid_geometry,
        _solid_stock_parts,
        _stair_geometry,
        _ramp_geometry,
        _ramp_reference_stock,
        _ramp_nominal_parts,
        reference_parts,
        _ramp_owned_reference_parts,
        _ramp_owned_reference_parts_uncached,
        _interface_reference_parts,
        _reference_material_parts,
        _source_fraction,
        importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️").circle_envelope,
        _subtract_convex_material,
        surface_finishes().clip_stock,
        *_flooring_layer_backend().producers(),
        _flooring_layer_backend,
        *_structural_joint_backend().producers(),
        _structural_joint_backend,
        _declared_structural_interfaces,
        _declared_material_interfaces,
        _declared_interfaces,
        _finish_reservations,
        _native_parts_have_volume,
        _native_parts_exact_volume,
        _native_profile_regions,
        _geometry_source_key,
        surface_finishes().publish_stock,
        surface_finishes().stock_separated,
        _slab_geometry_entry,
        _slab_geometry_uncached,
        _slab_surface,
        _slab_voids,
        _slab_finish_parts,
        _quantities_uncached,
        _ramp_quantity_partitions,
        _occupied_volume,
        _material_quantities,
        _partition_volume,
        _parts_disjoint,
        part_footprint,
        _material_bounds,
        is_architectural_covering,
        _wall_parts,
        _wall_geometry,
        _wall_geometry_unjoined,
        path_wall_geometry,
        _wall_junction_backend,
        _wall_has_junction,
        *_wall_junction_backend().producers(),
        _wall_interface_parts,
        _wall_interface_cutters,
        _guard_reference_stocks,
        _ramp_stringer_reference_stock,
        _interface_owner_parts,
        _subtract_native_material,
        _native_material_cells,
        _wall_owned_parts,
        _wall_cutters,
        _native_cache_scalar,
        _native_public_value,
        surface_finishes().wall_regions,
        surface_finishes().cell_region,
        surface_finishes().face_identity,
        surface_finishes().summaries,
        surface_finishes().face_matches,
        surface_finishes().apply_material_appearance,
        _surface_support_edges,
        _surface_separated,
        _surface_difference,
        _analysis._orientation,
        geometry_producers,
        _geometry_memo,
        _geometry_query,
        geometry_revision,
    )


def _geometry_memo(model: dict | None) -> dict:
    """🔒 Resolve a private immutable-input snapshot for one producer revision."""
    source = json.dumps(_m(model), sort_keys=True, separators=(",", ":"), ensure_ascii=False, allow_nan=False)
    return _geometry_models_cached(source, geometry_producers())


def geometry_revision(model: dict | None = None) -> int:
    """🗂️ Identify the complete source and producer snapshot within this process."""
    return _geometry_memo(model)["revision"]


def _geometry_query(kind: str, key: str | None, model: dict | None, progress=None, cancel=None) -> dict:
    """🔍 Publish only completed native values and return isolated copies to each caller."""
    if cancel and cancel():raise InterruptedError('Native geometry query cancelled')
    m = _m(model)
    key = None if kind == "building" else _find(m, key)["tag"]
    memo = _geometry_memo(m)
    slot = (kind, key)
    with memo["lock"]:
        value = memo["values"].get(slot)
    if value is not None:
        return deepcopy(value)
    owned = memo["model"]
    if kind == "building":
        value = _building_geometry_uncached(owned)
    elif kind == "wall":
        value = _native_public_value(_wall_geometry(owned,_find(owned,key),progress=progress,cancel=cancel))
    elif kind == "service":
        value = _service(owned, _find(owned, key))
    elif kind == "solid":
        value = _solid_geometry(owned, _find(owned, key),progress=progress,cancel=cancel)
    elif kind == "quantity":
        value = _quantities_uncached(owned, _find(owned, key))
    elif kind == "ramp_owned_reference":
        value = _ramp_owned_reference_parts_uncached(owned,_find(owned,key),progress=progress,cancel=cancel)
    elif kind == "ramp_finish_reference":
        value = _ramp_finish_backend().reference_parts(SimpleNamespace(**globals()),owned,_find(owned,key))
    else:
        raise ValueError(f"Unknown geometry query: {kind}")
    if cancel and cancel():raise InterruptedError('Native geometry query cancelled')
    returned = deepcopy(value)
    with memo["lock"]:
        if kind == "building":
            for obj in (*value.get("walls", ()),*value.get("screens", ())):
                memo["values"][("wall", obj["tag"])] = obj
            for obj in value.get("services", ()):
                memo["values"][("service", obj["tag"])] = obj
            for group in ("details", "furnishings", "beams", "foundations", "railings"):
                for obj in value.get(group, ()):
                    memo["values"][("solid", obj["tag"])] = obj
        memo["values"][slot] = value
    return returned


def wall_geometry(key: str, model: dict | None = None, progress=None, cancel=None) -> dict:
    """🧱 Query one revision-bound wall Body and its declared full material Reference in isolated form."""
    m = _m(model)
    if _find(m,key)['class']!='IfcWall':raise ValueError('A wall geometry query requires an actual native wall')
    return _geometry_query('wall',key,m,progress=progress,cancel=cancel)


def building_geometry(model: dict | None = None) -> dict:
    """🌐 Give drawings isolated shared metric geometry in CMN plan axes and an explicit north vector."""
    return _geometry_query("building", None, model)



def _pose_distance(pose: list, car: dict, centre: tuple, radius: float) -> float:
    heading = math.radians(pose[2])
    dx, dy = centre[0] - pose[0], centre[1] - pose[1]
    u = dx * math.cos(heading) + dy * math.sin(heading)
    v = -dx * math.sin(heading) + dy * math.cos(heading)
    ex, ey = max(abs(u) - car["length"] / 2, 0.0), max(abs(v) - car["width"] / 2, 0.0)
    return math.hypot(ex, ey) - radius


def _point_segment(point: list, a: list, b: list) -> float:
    dx, dy = b[0] - a[0], b[1] - a[1]
    length = dx * dx + dy * dy
    t = 0 if length == 0 else max(0, min(1, ((point[0] - a[0]) * dx + (point[1] - a[1]) * dy) / length))
    return math.hypot(point[0] - a[0] - t * dx, point[1] - a[1] - t * dy)


def _segment_distance(a: list, b: list, c: list, d: list) -> float:
    def cross(p, q, r):
        return (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])

    if cross(a, b, c) * cross(a, b, d) < 0 and cross(c, d, a) * cross(c, d, b) < 0:
        return 0.0
    return min(_point_segment(p, u, v) for p, u, v in ((a, c, d), (b, c, d), (c, a, b), (d, a, b)))


def _car_outline(pose: list, car: dict) -> list:
    angle = math.radians(pose[2])
    ux, uy = math.cos(angle), math.sin(angle)
    return [[pose[0] + ux * x - uy * y, pose[1] + uy * x + ux * y] for x, y in
            ((-car["length"] / 2, -car["width"] / 2), (car["length"] / 2, -car["width"] / 2),
             (car["length"] / 2, car["width"] / 2), (-car["length"] / 2, car["width"] / 2))]


def _contains_point(ring: list, point: list) -> bool:
    inside = False
    for a, b in zip(ring, ring[1:] + ring[:1]):
        if _point_segment(point, a, b) < 1e-12:
            return True
        if (a[1] > point[1]) != (b[1] > point[1]) and point[0] < a[0] + (point[1] - a[1]) * (b[0] - a[0]) / (b[1] - a[1]):
            inside = not inside
    return inside


def _vehicle_part(part: dict, car: dict) -> bool:
    return part["z1"] > car["ground"] and part["z0"] < car["ground"] + car["height"]


def _part_pose_distance(pose: list, car: dict, part: dict) -> float:
    outline = part["outline"]
    if "radius" in part:
        if "axis" not in part:raise ValueError("An analytic circular collision part requires its explicit profile axis")
        return _pose_distance(pose, car, part["axis"], part["radius"])
    ring = _car_outline(pose, car)
    if _contains_point(ring, outline[0]) or _contains_point(outline, ring[0]):
        return 0.
    return min(_segment_distance(a,b,c,d) for a,b in zip(ring,ring[1:]+ring[:1]) for c,d in zip(outline,outline[1:]+outline[:1])) - part.get("curve_error",0.)


def _column_pose_distance(pose: list, car: dict, column: dict) -> float:
    return min((_part_pose_distance(pose,car,p) for p in column["parts"] if _vehicle_part(p,car)), default=float("inf"))


def part_band_footprints(part: dict, z0: float, z1: float) -> list:
    """🚙 Project only the occupied portion inside an open vertical collision band."""
    if 'z0' in part and 'z1' in part:
        lower,upper = part['z0'],part['z1']
    else:
        heights = [vertex[2] for vertex in part_vertices(part)]
        lower,upper = min(heights),max(heights)
    lo,hi = max(z0,lower),min(z1,upper)
    if hi-lo <= 1e-10:
        return []
    if 'mesh' in part:
        points = [[v[0],v[1]] for v in part_vertices(part) if lo-1e-10 <= v[2] <= hi+1e-10]
        return [_convex_hull(points+_polyhedron_cut(part,lo,2)+_polyhedron_cut(part,hi,2))]
    if part.get('holes'):
        return part_regions(part,(lo+hi)/2)
    if 'section' in part:
        section = part['section']
        profile = section['profile']
        spans = _polygon_intervals(profile,lo)+_polygon_intervals(profile,hi)
        for a,b in zip(profile,profile[1:]+profile[:1]):
            lower,upper = max(lo,min(a[1],b[1])),min(hi,max(a[1],b[1]))
            if upper < lower:
                continue
            if abs(b[1]-a[1]) < 1e-12:
                spans.append(sorted([a[0],b[0]]))
            else:
                spans.append(sorted(a[0]+(b[0]-a[0])*(z-a[1])/(b[1]-a[1]) for z in (lower,upper)))
        intervals = _merged_intervals(spans)
        a,b = section['sweep']
        return [[[u,a],[v,a],[v,b],[u,b]] if section['plane']=='XZ' else [[a,u],[b,u],[b,v],[a,v]] for u,v in intervals if v-u>1e-10]
    if 'top_outline' not in part:
        return [deepcopy(part['outline'])]
    return [_convex_hull(part_outline_at(part,lo)+part_outline_at(part,hi))]


def _vehicle_parts(model: dict) -> list:
    geometry = building_geometry(model)
    registry = {e['tag']:e for e in model['elements']}
    records = []
    for group in ('pilotis','walls','fillings','slabs','screens','stairs','details','furnishings','beams','foundations','railings','services'):
        for obj in geometry.get(group,[]):
            parts = obj.get('parts',obj.get('steps',[]))+obj.get('finish_parts',[])
            for index,part in enumerate(parts):
                records.append({'tag':obj['tag'],'class':registry[obj['tag']]['class'],'kind':'column' if group=='pilotis' else group,'part':index,'role':part.get('role','solid'),'curve_error':part.get('curve_error',0.),'geometry':part})
    curtain=geometry.get('curtain_wall')
    if curtain:
        for index,part in enumerate(curtain['parts']):
            records.append({'tag':curtain['tag'],'class':registry[curtain['tag']]['class'],'kind':'curtain','part':index,'role':part.get('role','solid'),'curve_error':part.get('curve_error',0.),'geometry':part})
    for ramp in geometry['ramp']['ramps']:
        for index,part in enumerate(ramp['parts']+ramp['child_parts']):
            records.append({'tag':part['owner'],'class':registry[part['owner']]['class'],'kind':'ramps','part':index,'role':part['role'],'geometry':deepcopy(part)})
    return records


@lru_cache(maxsize=12)
def _vehicle_geometry_cached(source: str, z0: float, z1: float) -> tuple:
    model = json.loads(source)
    return tuple({**{k:v for k,v in record.items() if k!='geometry'},'outline':outline,'z0':max(z0,min(v[2] for v in part_vertices(record['geometry']))),'z1':min(z1,max(v[2] for v in part_vertices(record['geometry'])))} for record in _vehicle_parts(model) for outline in part_band_footprints(record['geometry'],z0,z1))


def vehicle_obstacles(model: dict | None = None, z0: float = 0., z1: float = 1.65) -> list:
    """🚧 Return isolated physical occupied projections in an explicit vehicle height band."""
    if z1 <= z0:
        raise ValueError('Vehicle height band must be positive')
    return deepcopy(list(_vehicle_geometry_cached(json.dumps(_m(model),sort_keys=True,separators=(',',':')),z0,z1)))


def _vehicle_obstacles(model: dict, poses: list, car: dict) -> list:
    def bounds(ring):
        return min(p[0] for p in ring),min(p[1] for p in ring),max(p[0] for p in ring),max(p[1] for p in ring)

    rings = [_car_outline(pose,car) for pose in poses]
    candidates = [(ring,bounds(ring),list(zip(ring,ring[1:]+ring[:1]))) for ring in rings]
    minima = {}
    for obstacle in vehicle_obstacles(model,car['ground'],car['ground']+car['height']):
        tag,outline = obstacle['tag'],obstacle['outline']
        box = bounds(outline)
        edges = list(zip(outline,outline[1:]+outline[:1]))
        def lower_bound(candidate):
            b = candidate[1]
            return max(0.,math.hypot(max(0,box[0]-b[2],b[0]-box[2]),max(0,box[1]-b[3],b[1]-box[3]))-obstacle.get('curve_error',0.))

        value = minima.get(tag,{}).get('clearance',float('inf'))
        for ring,box_car,car_edges in sorted(candidates,key=lower_bound):
            if lower_bound((ring,box_car,car_edges)) >= value:
                break
            if _contains_point(ring,outline[0]) or _contains_point(outline,ring[0]):
                value = 0.
                break
            value = min(value,max(0.,min(_segment_distance(a,b,c,d) for a,b in car_edges for c,d in edges)-obstacle.get('curve_error',0.)))
        if tag not in minima or value < minima[tag]['clearance']:
            minima[tag] = {'tag':tag,'class':obstacle['class'],'clearance':value}
    return sorted(minima.values(),key=lambda item:item['clearance'])

def _car_path_compute(model: dict, samples: int) -> dict:
    """🛬 Check an assumed arrival from the site side to a stop before the curved vestibule."""
    m = model
    car = _car(m)
    curtain = _find(m,"CW-1")
    curtain_front = min(p[1] for path in _curtain_paths(m,curtain) for p in path["path"])
    rc = car["turning_radius"] - car["width"] / 2
    length, width, _ = _box(m)
    xe, y_start = length - 1.6, width + 4.0
    cx, cy = xe - rc, -car["length"] / 2 - .5
    y_end = curtain_front - car["length"] / 2 - car["wall_clearance"] - .075

    def straight(a, b, heading):
        return (math.dist(a, b), lambda s, a=a, b=b, h=heading: (a[0] + (b[0] - a[0]) * s, a[1] + (b[1] - a[1]) * s, h))

    def arc(centre, a0, a1, turn):
        return (math.radians(abs(a1 - a0)) * rc,
                lambda s, c=centre, a0=a0, a1=a1, t=turn: (c[0] + rc * math.cos(math.radians(a0 + (a1 - a0) * s)),
                                                           c[1] + rc * math.sin(math.radians(a0 + (a1 - a0) * s)),
                                                           (a0 + (a1 - a0) * s + t) % 360))

    parts = [straight((xe, y_start), (xe, cy), 270.0), arc((cx, cy), 0, -180, -90), straight((cx - rc, cy), (cx - rc, y_end), 90.0)]
    total = sum(p[0] for p in parts)

    def pose(distance):
        for span, fn in parts:
            if distance <= span + 1e-12:
                x, y, h = fn(min(1.0, distance / span))
                return [_round(x, 6), _round(y, 6), _round(h, 4)]
            distance -= span
        x, y, h = parts[-1][1](1.0)
        return [_round(x, 6), _round(y, 6), _round(h, 4)]

    poses = [pose(total * i / (samples - 1)) for i in range(samples)]
    dense = [pose(total * i / 1200) for i in range(1201)]
    grid = m["grid"]
    gy = grid["y"]
    crossings = []
    for j, row in enumerate(grid["labels_y"]):
        for index, (a, b) in enumerate(zip(dense, dense[1:])):
            if (a[1] - gy[j]) * (b[1] - gy[j]) < 0:
                t = (gy[j] - a[1]) / (b[1] - a[1])
                x = a[0] + (b[0] - a[0]) * t
                heading = math.radians(a[2])
                half = car["length"] / 2 * abs(math.cos(heading)) + car["width"] / 2 * abs(math.sin(heading))
                crossings.append({"row": row, "x": _round(x, 4), "y": gy[j], "half_extent": _round(half, 4), "progress": _round((index + t) / 1200, 6)})
    ground = [p for p in building_geometry(m)["pilotis"] if any(_vehicle_part(part,car) for part in p["parts"])]
    for c in crossings:
        row = [p for p in ground if abs(p["y"] - c["y"]) < 1e-6]
        left = max((p for p in row if p["x"] <= c["x"]), key=lambda p: p["x"], default=None)
        right = min((p for p in row if p["x"] > c["x"]), key=lambda p: p["x"], default=None)
        c["between"] = [left["tag"] if left else None, right["tag"] if right else None]
        near = [p for p in dense if abs(p[1] - c["y"]) < 3.0]
        for name, column in (("clear_left", left), ("clear_right", right)):
            if column:
                c[name] = _round(min(_column_pose_distance(p,car,column) for p in near), 4)
    entry = next(c for c in crossings if all(c["between"]))
    pair = entry["between"]
    supports = {p["tag"]:p for p in ground}
    edges = [[interval for part in supports[tag]["parts"] if _vehicle_part(part,car)
              for interval in _polygon_intervals(part["outline"],entry["y"])] for tag in pair]
    gap = {"start": [max(hi for lo,hi in edges[0]),entry["y"]], "end": [min(lo for lo,hi in edges[1]),entry["y"]]}
    obstacles = _vehicle_obstacles(m, dense[::4], car)
    clearance_min = min(item["clearance"] for item in obstacles)
    sample_distance = total / 300
    sample_bound = sample_distance / 2 * (1 + math.hypot(car["length"],car["width"]) / (2 * rc))
    return {"poses": poses, "length": _round(total), "entry_pair": pair, "pairs": [c["between"] for c in crossings], "crossings": crossings,
            "gap_progress": entry["progress"], "gap": gap, "centre_radius": _round(rc), "curve_centre": [cx, cy],
            "obstacles": obstacles, "sample_bound": _round(sample_bound), "certified_min_clearance": _round(clearance_min - sample_bound),
            "collision_free": clearance_min > sample_bound, "basis": "assumed",
            "note": {"en": "Assumed teaching arrival on the entry apron; not a surveyed historic traffic route. Rectangle sweep checks all height-intersecting physical building parts, fillings, equipment and services.",
                     "de": "Angenommene Lehr-Anfahrt über den Eingangsvorplatz; keine vermessene historische Fahrspur. Rechteckhüllen prüfen alle höhenüberschneidenden Bauteile, Ausfachungen, Ausstattungen und Installationen."},
            "min_clearance": _round(clearance_min, 4), "car": {"length": car["length"], "width": car["width"], "height": car["height"]}}

@lru_cache(maxsize=12)
def _car_path_cached(source: str, samples: int) -> dict:
    """🗄️ Retain a bounded route result for one exact JSON model state and sampling request."""
    return _car_path_compute(json.loads(source), samples)


def car_path(model: dict | None = None, samples: int = 120) -> dict:
    """🚘 Derive an isolated route result, reusing work only while all source content matches."""
    if isinstance(samples, bool) or not isinstance(samples, int) or samples < 2:
        raise ValueError("At least two path samples are required")
    source = source_text() if model is None else json.dumps(model, sort_keys=True, separators=(",", ":"), allow_nan=False)
    return deepcopy(_car_path_cached(source, samples))



def _entry_dimensions(model: dict, path: dict) -> dict:
    supports = {p["tag"]:p for p in building_geometry(model)["pilotis"]}
    left,right = [supports[tag] for tag in path["entry_pair"]]
    spacing = right["x"] - left["x"]
    clear = path["gap"]["end"][0] - path["gap"]["start"][0]
    return {"spacing":spacing,"clear_width":clear,"obstruction_width":spacing-clear}


def clearance(model: dict | None = None, spacing: float | None = None) -> dict:
    """🚗 Check the physical entry gap and the vehicle path against occupied source geometry."""
    m = _m(model)
    car, path = _car(m), car_path(m)
    gap = _entry_dimensions(m,path)
    actual, initial = gap["spacing"],m["grid"]["initial_spacing"]["value"]
    used = actual if spacing is None else spacing
    clear = used - gap["obstruction_width"]
    margin = (clear-car["width"])/2
    return {"spacing":used,"grid_spacing":m["grid"]["spacing"]["value"],"initial_spacing":initial,
            "obstruction_width":gap["obstruction_width"],"clear_width":clear,"clear_width_initial":initial-gap["obstruction_width"],
            "car_width":car["width"],"car_length":car["length"],"margin":margin,
            "margin_initial":(initial-gap["obstruction_width"]-car["width"])/2,"min_margin":car["min_side_margin"],
            "turning_radius":car["turning_radius"],"wall_clearance":car["wall_clearance"],
            "centre_radius":path["centre_radius"],"fits":margin>=car["min_side_margin"] and path["collision_free"],
            "fits_initial":(initial-gap["obstruction_width"]-car["width"])/2>=car["min_side_margin"],
            "gaps":path["crossings"],"entry_pair":path["entry_pair"],"min_path_clearance":path["min_clearance"]}


def clearance_limits(model: dict | None = None, *, spacing: float | None = None, diameter: float | None = None, car_width: float | None = None) -> dict:
    """🛞 Compare the physical gap, or an explicit circular-profile scenario, with the required car margin."""
    m = _m(model)
    car = _car(m)
    gap = _entry_dimensions(m,car_path(m))
    obstruction = gap["obstruction_width"] if diameter is None else diameter
    width = car["width"] if car_width is None else car_width
    used = gap["spacing"] if spacing is None else spacing
    need = width + 2*car["min_side_margin"]
    margin = (used-obstruction-width)/2
    return {"spacing":used,"obstruction_width":obstruction,"car_width":width,"min_margin":car["min_side_margin"],
            "clear_width":used-obstruction,"margin":margin,"required_clear_width":need,"min_spacing":need+obstruction,
            "slack":used-obstruction-need,"fits":margin>=car["min_side_margin"]-1e-9}


def path_pinches(model: dict | None = None, count: int = 2) -> list:
    """🧲 The closest approaches of the assumed car to a piloti along its path, tightest first: tag, clearance, progress and pose."""
    m = _m(model)
    car = _car(m)
    poses = car_path(m, samples=1201)["poses"]
    found = []
    for piloti in building_geometry(m)["pilotis"]:
        if not any(_vehicle_part(part,car) for part in piloti["parts"]):
            continue
        distance, index = min((_column_pose_distance(pose,car,piloti),i) for i,pose in enumerate(poses))
        found.append({"tag": piloti["tag"], "clearance": _round(distance, 4), "progress": _round(index / 1200, 6), "pose": list(poses[index]), "centre": [piloti["x"], piloti["y"]],
                      "parts": deepcopy(piloti["parts"]), "bounding_radius": piloti["bounding_radius"]})
    return sorted(found, key=lambda item: item["clearance"])[:count]


def class_chain(ifc_class: str, model: dict | None = None) -> list:
    """🧬 List the IFC supertype chain from IfcRoot down to a class with the attributes each level adds."""
    classes = _m(model)["classes"]
    if ifc_class not in classes:
        raise ValueError(f"Unknown IFC class: {ifc_class}")
    chain, name = [], ifc_class
    while name is not None:
        chain.append({"class": name, "adds": list(classes[name]["adds"])})
        name = classes[name]["supertype"]
    return chain[::-1]


def type_occurrences(type_id: str, model: dict | None = None) -> list:
    """🏷️ List the tags of every occurrence defined by one type."""
    m = _m(model)
    kind = _find(m, type_id)
    if kind["tag"] not in {t["tag"] for t in m["types"]}:
        raise ValueError(f"Not a type: {type_id}")
    return [e["tag"] for e in m["elements"] if e.get("type") == kind["tag"]]


def _opening_area(model: dict, wall: dict) -> float:
    return sum((v["end"] - v["start"]) * (v["head"] - v["sill"]) for v in (_opening_values(model, o) for o in _by_class(model, "IfcOpeningElement") if o["geometry"].get("wall") == wall["tag"]))


def _slab_areas(model: dict, slab: dict) -> dict:
    g = _slab_geometry_entry(model, slab)
    gross = _area(g["outline"])
    full = g["full_parts"]
    area = sum(_area(p["outline"]) for p in full)
    return {"gross_area": _round(gross), "void_area": _round(gross - area), "area": _round(area),
            "perimeter": _round(_length(g["outline"] + [g["outline"][0]])), "thickness": g["thickness"],
            "volume": _round(sum(_area(p["outline"]) * (p["z1"]-p["z0"]) for p in full)),
            "exclusive_volume":_round(_partition_volume(g["parts"])),"shared_volume":_round(_partition_volume(g["shared_parts"])),"scope":"full_member",
            "finish_volume":_round(_partition_volume(g["full_finish_parts"])),"exclusive_finish_volume":_round(_partition_volume(g["finish_parts"])),"shared_finish_volume":_round(_partition_volume(g["shared_finish_parts"])),
            "gross_volume": _round(sum(r["gross_area"] * r["thickness"] for r in g["regions"])),
            "regions": [{"tag": r["tag"], "area": _round(r["area"]), "thickness": r["thickness"],
                         "top": r["top"], "volume": _round(r["area"] * r["thickness"])} for r in g["regions"]]}

def _parts_disjoint(parts: list, progress=None, cancelled=None) -> bool:
    """🧷 Prove native polyhedral component separation by actual world bounds and exact projected support planes."""
    if cancelled and cancelled():
        raise InterruptedError("Material partition proof cancelled")
    if any(part.get("profile") or "radius" in part for part in parts):
        return False
    records=[]
    for part in parts:
        if cancelled and cancelled():
            raise InterruptedError("Material partition proof cancelled")
        box=_material_bounds(part)
        outline=[[box[0][0],box[1][0]],[box[0][1],box[1][0]],[box[0][1],box[1][1]],[box[0][0],box[1][1]]]if "csg"in part else part_footprint(part)
        records.append((box,outline))
    active=[]
    for index,(bounds,outline) in enumerate(sorted(records,key=lambda row:row[0][0][0]),1):
        if cancelled and cancelled():
            raise InterruptedError("Material partition proof cancelled")
        active=[row for row in active if row[0][0][1]>bounds[0][0]]
        for other,ring in active:
            if cancelled and cancelled():
                raise InterruptedError("Material partition proof cancelled")
            if any(bounds[k][1]<=other[k][0] or other[k][1]<=bounds[k][0] for k in (1,2)):
                continue
            if not _surface_separated(outline,ring):
                return False
        active.append((bounds,outline))
        if progress:
            progress({"completed":index,"total":len(records)})
    return True


def _material_quantities(model: dict, obj: dict, parts: list, partition: bool = False) -> dict:
    """⚖️ Integrate owned material volumes; unit weights are kN/m³, weight kN and mass kg."""
    groups = {}
    fallback = _material(model,obj)
    for part in parts:
        material = part.get("material",fallback["tag"] if fallback else None)
        if material == "MAT-AIR":
            continue
        if material is None:
            raise ValueError("An occupied assembly part requires an explicit or product material")
        groups.setdefault(material,[]).append(part)
    entries = []
    for tag,owned in groups.items():
        unit_weight = _find(model,tag)["density"]
        if unit_weight["unit"] != "kN/m³":
            raise ValueError("An assembly material requires unit weight in kN/m³")
        volume = _partition_volume(owned) if partition else _occupied_volume(owned)
        weight = volume*unit_weight["value"]
        entries.append({"material":tag,"volume":_round(volume),"unit_weight":deepcopy(unit_weight),"weight":_round(weight),"mass":_round(weight*1000/9.80665)})
    weight = sum(entry["weight"] for entry in entries)
    return {"material_quantities":entries,"weight":_round(weight),"mass":_round(weight*1000/9.80665)}


def _ramp_quantity_partitions(model: dict, obj: dict, geometry: dict) -> dict:
    """➕ Prove original component separation or complete directed stock subtraction before summing cells."""
    if obj["class"]=="IfcRampFlight":return {"owned":True,"children":True,"full":True}
    from fractions import Fraction
    q = _source_fraction
    components = [("child",f["tag"],_ramp_reference_stock(model,_find(model,f["tag"])),set(_find(model,f["tag"])["geometry"].get("flight_joint_owners",()))) for f in geometry["flights"]]
    for landing in geometry["landings"]:
        if not landing["parts"]:continue
        owner = _find(model,landing["owner"])
        original = next(p for p in obj["geometry"]["landings"] if p["tag"]==landing["tag"] and p["owner"]==landing["owner"])
        if landing["ownership"]=="floor_slab":
            low,high = min(q(p["z0"]) for p in landing["parts"]),max(q(p["z1"]) for p in landing["parts"])
        else:
            spec = owner["geometry"]
            low = high = q(_z(model,spec["from_level"]))+(q(_z(model,spec["to_level"]))-q(_z(model,spec["from_level"])))*q(original["portion"])-q(spec["finish_thickness"]["value"])
            low -= q(original["thickness"]["value"])
        stock = {"outline":[[q(v) for v in p] for p in _expand(model,original["outline"])],"z0":low,"z1":high}
        components.append(("owned" if landing["owner"]==obj["tag"] else "shared",landing["tag"],stock,set()))
    def separate(left,right):
        if left[0]==right[0]=="child" and (left[1] in right[3] or right[1] in left[3]):return True
        a,b = [[[Fraction(v) for v in p] for p in part_vertices(c[2])] for c in (left,right)]
        for item,vertices in ((left,a),(right,b)):
            for face in part_faces(item[2]):
                origin = vertices[face[0]]
                for i in range(1,len(face)-1):
                    u,v = [[vertices[j][k]-origin[k] for k in range(3)] for j in (face[i],face[i+1])]
                    axis = [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
                    if any(axis):break
                else:continue
                x,y = [[sum(n*z for n,z in zip(axis,p)) for p in points] for points in (a,b)]
                if max(x)<=min(y) or max(y)<=min(x):return True
        return False
    pairs = [(left,right,separate(left,right)) for i,left in enumerate(components) for right in components[:i]]
    return {"owned":all(valid for left,right,valid in pairs if left[0]==right[0]=="owned"),"children":all(valid for left,right,valid in pairs if left[0]==right[0]=="child"),"full":all(valid for _,_,valid in pairs)}


def _quantities_uncached(m: dict, obj: dict) -> dict:
    """📋 Derive exact physical quantities once for one complete canonical source snapshot."""
    key = obj["tag"]
    cls = obj["class"]
    if cls == "IfcColumn":
        g = column_geometry(m, obj)
        native_parts = g["parts"]
        parts = g.get("section_metrics",native_parts)
        out = {"height": _round(g["top"] - g["base"]),
               "volume": sum(_polyhedron_volume(p) if "mesh" in p else p["area"]*(p["z1"]-p["z0"]) for p in native_parts),
               "outer_surface_area": sum(p["outer_perimeter"]*(p["z1"]-p["z0"]) for p in parts),
               "bore_surface_area":sum((p["perimeter"]-p["outer_perimeter"])*(p["z1"]-p["z0"]) for p in parts),
               "lateral_surface_area":sum(p["perimeter"]*(p["z1"]-p["z0"]) for p in parts)}
        if 'end_plane' in g:
            normal,origin=g['end_plane']['normal'],g['end_plane']['point']
            last=parts[-1]
            def lateral(rings):
                """🪁 Integrate vertical boundary trapezoids beneath the actual upper plane."""
                total=0.
                for ring in rings:
                    for a,b in zip(ring,ring[1:]+ring[:1]):
                        hs=[origin[2]-sum(normal[k]*(p[k]-origin[k]) for k in (0,1))/normal[2]-last['z0'] for p in (a,b)]
                        total+=math.dist(a,b)*sum(hs)/2
                return total
            outer=sum(p['outer_perimeter']*(p['z1']-p['z0']) for p in parts[:-1])+lateral([last['outline']])
            bore=sum((p['perimeter']-p['outer_perimeter'])*(p['z1']-p['z0']) for p in parts[:-1])+lateral(last.get('holes',[]))
            out.update(outer_surface_area=outer,bore_surface_area=bore,lateral_surface_area=outer+bore)
        if all(math.isclose(p["area"],parts[0]["area"],rel_tol=1e-12) for p in parts):
            out["cross_section_area"] = parts[0]["area"]
            out["second_moments"] = deepcopy(parts[0]["second_moments"])
        if len(parts) == 1:
            out.update({k: v["value"] for k, v in parts[0]["profile"].items() if k in ("diameter", "width", "depth")})
            if "diameter" in out:
                out["radius"] = out["diameter"] / 2
        else:
            out["sections"] = [{"area": p["area"], "height": p["z1"] - p["z0"], "profile": p["profile"],"second_moments":deepcopy(p["second_moments"]),"void_area":p["void_area"]} for p in parts]
        return out
    if cls in ("IfcColumnType","IfcWindowType","IfcDoorType","IfcCurtainWallType","IfcSlabType","IfcBuildingElementProxyType"):
        occurrences = type_occurrences(obj["tag"], m)
        values = [quantities(t, m) for t in occurrences]
        out = {"count": len(occurrences), "volume": sum(q["volume"] for q in values), "occurrences": occurrences}
        if values and all("glazing_area" in q for q in values):
            out["glazing_area"] = sum(q["glazing_area"] for q in values)
        for name in ("diameter", "width", "depth", "height", "cross_section_area"):
            if values and name in values[0] and all(q.get(name) == values[0][name] for q in values):
                out[name] = values[0][name]
        return out
    if cls == "IfcWall" and not _wall_uses_path(obj):
        g = _wall_geometry(m, obj)
        gross = g["length"] * (g["top"] - g["base"])
        opening = _opening_area(m, obj)
        glazed = sum(quantities(o["fill_tag"],m)["glazing_area"] for o in g["openings"] if o["fill_tag"])
        return {"length": g["length"], "height": _round(g["top"] - g["base"]), "thickness": g["thickness"], "gross_side_area": _round(gross), "opening_area": _round(opening),
                "net_side_area": _round(gross - opening), "glazing_area":_round(glazed), "glazing_share": glazed / gross, "volume": _round(g["solid_volume"]), "joint_displacement_volume": _round(g["joint_displacement_volume"]),
                "cavity_volume": _round(sum(l["volume"] for l in g["layers"] if l["is_void"])),
                "gross_volume": _round(sum(l["gross_side_area"]*(l["offsets"][1]-l["offsets"][0]) for l in g["layers"] if not l["is_void"])),
                "envelope_volume": _round(sum(l["gross_side_area"]*(l["offsets"][1]-l["offsets"][0]) for l in g["layers"])),
                "bounding_volume": _round(gross*g["thickness"]), "layers":deepcopy(g["layers"]), **({"surface_finishes":deepcopy(g["surface_finishes"])} if "surface_finishes" in g else {})}

    if cls == "IfcWall":
        g = path_wall_geometry(m,obj)
        openings = {o["tag"] for o in g["openings"]}
        glazed = sum(quantities(e["tag"],m)["glazing_area"] for e in m["elements"] if e["class"] in ("IfcWindow","IfcDoor") and e["geometry"]["opening"] in openings)
        side = g["length"]*(g["body_top"]-g["base"])
        aperture = sum((o["end"]-o["start"])*(o["head"]-o["sill"]) for o in g["openings"])
        return {"length":g["length"],"height":g["top"]-g["base"],"thickness":g["thickness"],"gross_side_area":side,"opening_area":aperture,"net_side_area":side-aperture,"glazing_area":glazed,"glazing_share":glazed/side,"volume":g["solid_volume"],"gross_volume":g["gross_material_volume"],"envelope_volume":g["gross_envelope_volume"],"gross_cavity_volume":g["gross_cavity_volume"],"cavity_volume":g["cavity_volume"],"joint_displacement_volume":g["joint_displacement_volume"],"opening_volume":g["opening_material_volume"]}

    if cls == "IfcWallType":
        material=_find(m,obj["material"])
        if "layers" in material:
            return {"thickness":_round(sum(l["thickness"]["value"] for l in material["layers"])),"layers":[{"id":l["id"],"name":l["name"],"thickness":l["thickness"]["value"]} for l in material["layers"]]}
        return {"thickness":obj["attributes"]["thickness"]["value"],"material":material["tag"]}
    if cls == "IfcSlab":
        out = _slab_areas(m, obj)
        geometry = _slab_geometry_entry(m, obj)
        if "layers" in geometry:
            out["total_thickness"] = geometry["total_thickness"]
        if obj["geometry"].get("column_reserves") or obj["geometry"].get("interface_owners"):
            out.update(_material_quantities(m,obj,geometry["parts"],partition=True))
        return out
    if cls == "IfcOpeningElement" and "slab" in obj["geometry"]:
        g = _slab_opening_geometry(m,obj)
        return {"host":g["host"],"area":_area(g["outline"]),"volume":sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in g["parts"]),
                "depths":sorted(set(p["z1"]-p["z0"] for p in g["parts"]))}
    if cls == "IfcOpeningElement":
        g=opening_geometry(obj["tag"],m)
        return {"width":g["end"]-g["start"],"height":g["head"]-g["sill"],"area":(g["end"]-g["start"])*(g["head"]-g["sill"]),"sill":g["sill"],"head":g["head"],"host":g["host"],"host_class":g["host_class"],"volume":sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in g["parts"])}
    if cls in ("IfcOpeningElement", "IfcWindow", "IfcDoor"):
        opening = obj if cls == "IfcOpeningElement" else _find(m, obj["geometry"]["opening"])
        v = _opening_values(m, opening)
        physical = _filling_geometry(m,obj) if cls != "IfcOpeningElement" else None
        extra = {name:physical[name] for name in ("rough_width","rough_height","clear_width","clear_height","frame_width","frame_depth","panel_thickness")} if physical else {}
        if physical:
            extra["volume"] = _round(_occupied_volume(physical["parts"]))
            extra["glazing_area"] = _round(sum(_area(p["outline"])*(p["z1"]-p["z0"])/physical["panel_thickness"] for p in physical["parts"] if p["role"]=="glass"))
            extra["frame_volume"] = _round(_occupied_volume([p for p in physical["parts"] if p["role"]=="frame"]))
            if any(p["role"]=="glass" for p in physical["parts"]):
                extra["glass_thickness"] = physical["panel_thickness"]

        return {"width": _round(v["end"] - v["start"]), "height": _round(v["head"] - v["sill"]), "area": _round((v["end"] - v["start"]) * (v["head"] - v["sill"])),
                "sill": v["sill"], "head": v["head"], **extra}
    if cls == "IfcCurtainWall":
        geometry=_curtain_filling_geometry(m,obj)
        parts=geometry["parts"]
        total=sum(p["length"] for p in geometry["paths"])
        height=geometry["top"]-geometry["base"]
        spec=_find(m,obj["type"])["filling"]
        return {"length":_round(total),"height":_round(height),"area":_round(total*height),"volume":_round(sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in parts)),"part_count":len(parts),
                **{k:spec[k]["value"] for k in ("frame_width","frame_depth","panel_thickness")},"glass_thickness":spec["panel_thickness"]["value"],
                "frame_volume":_round(sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in parts if p["role"]=="frame")),
                "glazing_area":_round(sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in parts if p["role"]=="glass")/spec["panel_thickness"]["value"])}
    if cls in ("IfcRamp","IfcRampFlight"):
        g = _ramp_geometry(m,obj)
        flights = g["flights"]
        proof = _ramp_quantity_partitions(m,obj,g)
        exclusive = (_partition_volume if proof["owned"] else _occupied_volume)(g["parts"])
        shared = _occupied_volume(g["shared_parts"]) if g["shared_parts"] else 0.
        children = (_partition_volume if proof["children"] else _occupied_volume)(g["child_parts"]) if g["child_parts"] else 0.
        volume = exclusive+shared+children if proof["full"] else _occupied_volume(g["full_parts"])
        out = {"volume":_round(volume),"exclusive_volume":_round(exclusive),"shared_volume":_round(shared),"child_volume":_round(children),"scope":"assembly" if cls=="IfcRamp" else "product","thickness_direction":"vertical",**_material_quantities(m,obj,g["parts"],partition=proof["owned"])}
        if cls=="IfcRamp":
            return {**out,"width":g["width"],"flight_count":len(flights),"length":_round(sum(f["run"] for f in flights)),"rise":_round(max(f["z1"] for f in flights)-min(f["z0"] for f in flights)),"slope":_round(sum(f["slope"] for f in flights)/len(flights))}
        f = flights[0]
        return {**out,"length":f["run"],"run":f["run"],"rise":_round(f["z1"]-f["z0"]),"slope":f["slope"],"slope_percent":_round(f["slope"]*100,6),"width":f["width"],"thickness":f["thickness"],"area":_round(f["run"]*f["width"])}
    if cls == "IfcSpace":
        outline = _expand(m, obj["outline"])
        out = {"area": _round(_area(outline)), "perimeter": _round(_length(outline + [outline[0]]))}
        if "reference_area" in obj:
            out["reference_area"] = obj["reference_area"]["value"]
            out["fit"] = _round(out["area"] / out["reference_area"] - 1, 6)
        return out
    if cls in ("IfcPipeSegment","IfcCovering","IfcChimney","IfcWasteTerminal","IfcBoiler","IfcSpaceHeater","IfcSanitaryTerminal","IfcLightFixture") and not is_architectural_covering(obj):
        g = service_geometry(obj["tag"], m)
        if "parts" in obj["geometry"]:
            return {"part_count":len(g["parts"]),"volume":_round(g["volume"]),**_material_quantities(m,obj,g["parts"])}
        return {k:g[k] for k in ("length","diameter","inner_diameter","wall_thickness","volume")} if "tube" in obj["geometry"] else {"volume":sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in g["parts"])}
    if cls == "IfcStair":
        g = _stair_geometry(m, obj)
        return {"step_count": len(g["steps"]), "rise": g["top"] - g["base"], "volume": _round(_occupied_volume(g["parts"]))}
    if "parts" in obj.get("geometry", {}) or obj.get("geometry",{}).get("kind")=="ramp_finish":
        solid = solid_geometry(obj["tag"], m)
        reference = solid.get("full_parts",solid["parts"] + solid["shared_parts"])
        structural = obj["class"] in ("IfcBeam","IfcFooting") or bool(obj["geometry"].get("structural_joints"))
        parts = reference if structural else solid["parts"]
        partition = _parts_disjoint(parts)
        owned_partition = partition if parts==solid["parts"] else _parts_disjoint(solid["parts"])
        volume = _partition_volume if partition else _occupied_volume
        owned = _partition_volume if owned_partition else _occupied_volume
        measured = volume(parts)
        reference_volume = measured if reference==parts else (_partition_volume if _parts_disjoint(reference) else _occupied_volume)(reference)
        out = {"part_count":len(parts),"volume":_round(measured),"reference_volume":_round(reference_volume),**_material_quantities(m,obj,solid["parts"],partition=owned_partition)}
        if structural:
            shared = (_partition_volume if _parts_disjoint(solid["shared_parts"]) else _occupied_volume)(solid["shared_parts"]) if solid["shared_parts"] else 0.
            out.update({"exclusive_volume":_round(owned(solid["parts"])),"shared_volume":_round(shared),"scope":"full_member"})
        return out
    raise ValueError(f"No derived quantities for {key}")

def quantities(key: str, model: dict | None = None) -> dict:
    """🪙 Give each caller isolated exact quantities of an element, type total or space."""
    return _geometry_query("quantity",key,model)



def quantity(element_: dict | str, name: str, model: dict | None = None) -> float:
    """📏 Read one authored or derived quantity (a plain number) by name, layer id included."""
    m = _m(model)
    obj = _find(m, element_ if isinstance(element_, str) else element_["tag"])
    if name in obj.get("attributes", {}):
        return obj["attributes"][name]["value"]
    layerset = _material(m, obj)
    if layerset is not None and "layers" in layerset:
        for layer in layerset["layers"]:
            if layer["id"] == name:
                return layer["thickness"]["value"]
    try:
        derived = quantities(obj["tag"], m)
        if name in derived and isinstance(derived[name], (int, float)):
            return derived[name]
    except (ValueError, KeyError, StopIteration):
        pass
    if obj.get("type"):
        kind = _find(m, obj["type"])
        if name == "diameter" and "diameter" in kind.get("profile", {}):
            return kind["profile"]["diameter"]["value"]
    raise ValueError(f"Unknown quantity {name} for {obj['tag']}")


def _special(model: dict, pid: str):
    if pid in ("grid.spacing","grid.initial_spacing"):
        name = pid.split(".")[1]
        return ("spacing" if name == "spacing" else "initial_spacing",model["grid"][name])
    if pid in ("car.width","car.length","car.height","car.turning_radius"):
        name = pid.split(".")[1]
        return ("turning_radius" if name == "turning_radius" else "car_"+name,model["car"][name])
    if pid == "piloti.diameter":
        obj = _find(model,role("column_example",model))
        profile = _find(model,obj["type"])["profile"]
        if "diameter" not in profile:
            raise ValueError("The column example is not circular")
        return ("diameter",profile["diameter"])
    return None


def information(tag: str, property_id: str, language: str, model: dict | None = None) -> dict:
    """📊 Give a raw value its unit, basis, object identity, localized label and meaning."""
    m = _m(model)
    if language not in LANGUAGES:
        raise ValueError("Unknown language")
    obj = element(tag, m)
    names = m["property_names"]
    special = _special(m, property_id)
    if obj["class"] in ("IfcColumn", "IfcColumnType") and property_id in ("diameter", "width", "depth"):
        profile = (_find(m, obj["type"]) if obj.get("type") else obj)["profile"]
        if property_id not in profile or obj.get("geometry", {}).get("sections"):
            raise ValueError(f"No uniform {property_id} for {tag}")
        special = (property_id, profile[property_id])
    if special is not None:
        key, q = special
        value, unit, basis = q["value"], q["unit"], q["basis"]
    elif property_id == "material":
        material = obj.get("material")
        if material is None:
            raise ValueError(f"No material for {tag}")
        key, value, unit, basis = "material", material["name"][language], "", "assumed"
    elif property_id in ("GlobalId", "PredefinedType"):
        if property_id == "PredefinedType" and "predefined_type" not in obj:
            raise ValueError(f"PredefinedType is not an attribute of {obj['class']}")
        key, value, unit, basis = property_id, obj["guid"] if property_id == "GlobalId" else obj["predefined_type"], "", "derived"
    else:
        pset, _, prop = property_id.rpartition(".")
        found = None
        for name, properties in obj.get("psets", {}).items():
            if (not pset or name == pset) and prop in properties:
                found = properties[prop]
        if found is not None:
            key, value, unit, basis = prop, found["value"], "", found["basis"]
        else:
            key = property_id
            value = quantity(obj["tag"], property_id, m)
            unit = names.get(property_id, {}).get("unit", "m")
            attributes = obj.get("attributes", {})
            basis = attributes[property_id]["basis"] if property_id in attributes else "derived"
            unit = attributes[property_id]["unit"] if property_id in attributes else unit
    entry = names.get(key)
    label = entry["name"][language] if entry else key
    if entry and "meaning" in entry:
        meaning = entry["meaning"][language]
    else:
        meaning = f"{label} of {obj['name'][language]}" if language == "en" else f"{label} von {obj['name'][language]}"
    return {"value": value, "unit": unit, "basis": basis, "tag": obj["tag"], "guid": obj["guid"], "class": obj["class"], "storey": obj.get("storey"),
            "property_id": property_id, "label": label, "meaning": meaning}


def relations_of(key: str, model: dict | None = None) -> list:
    """🔗 List every IFC relationship touching an object together with the other side."""
    m = _m(model)
    obj = _find(m, key)
    registry = _registry(m)

    def side(tag):
        item = registry[tag]
        return {"tag": item["tag"], "guid": item["guid"], "class": item["class"]}

    out = []
    for rel in m["relations"]:
        if obj["tag"] == rel["relating"] or obj["tag"] in rel["related"]:
            role = "relating" if obj["tag"] == rel["relating"] else "related"
            others = rel["related"] if role == "relating" else [rel["relating"]]
            out.append({"guid": rel["guid"], "tag": rel["tag"], "class": rel["class"], "role": role, "name": rel["name"],
                        "relating": side(rel["relating"]), "related": [side(t) for t in rel["related"]], "others": [side(t) for t in others]})
    return out


def partial_model(model_id: str, model: dict | None = None) -> dict:
    """👥 Select one discipline model: same schema, different authors, shared GlobalIds."""
    m = _m(model)
    spec = next((x for x in m["models"] if x["id"] == model_id), None)
    if spec is None:
        raise ValueError(f"Unknown model: {model_id}")
    registry = _registry(m)
    counts = {}
    for tag in spec["element_tags"]:
        counts[registry[tag]["class"]] = counts.get(registry[tag]["class"], 0) + 1
    return {"id": spec["id"], "schema_id": spec["schema_id"], "author": spec["author"], "author_label": deepcopy(spec.get("author_label") or {"en": spec["author"], "de": spec["author"]}),
            "name": deepcopy(spec["name"]), "revision": spec["revision"],
            "element_tags": list(spec["element_tags"]), "element_guids": [registry[t]["guid"] for t in spec["element_tags"]], "counts_by_class": counts}


def _footprint(model: dict, wall: dict) -> list:
    g = _wall_geometry(model, wall)
    (x0, y0), (x1, y1) = g["start"], g["end"]
    ux, uy = (x1 - x0) / g["length"], (y1 - y0) / g["length"]
    nx, ny = -uy * g["thickness"] / 2, ux * g["thickness"] / 2
    return [[_round(x0 - nx, 6), _round(y0 - ny, 6)], [_round(x1 - nx, 6), _round(y1 - ny, 6)], [_round(x1 + nx, 6), _round(y1 + ny, 6)], [_round(x0 + nx, 6), _round(y0 + ny, 6)]]


def solid_geometry(key: str, model: dict | None = None, progress=None, cancel=None) -> dict:
    """🧁 Resolve one authored multipart element through the shared physical part interface."""
    return _geometry_query("solid", key, model,progress=progress,cancel=cancel)


def part_faces(part: dict) -> list:
    """🔻 Orient the closed polygon faces of a native extrusion or compatible convex loft."""
    if part.get("holes"):
        mesh = profile_mesh([part["outline"],*part["holes"]])
        n = len(mesh["vertices"])
        return [[i,j,n+j,n+i] for ring in mesh["boundaries"] for i,j in zip(ring,ring[1:]+ring[:1])] + [[n+i for i in t] for t in mesh["triangles"]] + [list(reversed(t)) for t in mesh["triangles"]]
    vertices = part_vertices(part)
    if "mesh" in part:
        faces = deepcopy(part["mesh"]["faces"])
    else:
        n = len(part["section"]["profile"]) if "section" in part else len(part["outline"])
        faces = [list(reversed(range(n))),list(range(n,2*n))] + [[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]
    if "mesh" not in part:
        origin = vertices[0]
        signed = []
        for face in faces:
            for i in range(1,len(face)-1):
                a,b,c = [[vertices[j][k]-origin[k] for k in range(3)] for j in (face[0],face[i],face[i+1])]
                signed.append(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))
        return faces if math.fsum(signed)>=0 else [list(reversed(face)) for face in faces]
    centre = [sum(p[k] for p in vertices)/len(vertices) for k in range(3)]
    for face in faces:
        a = vertices[face[0]]
        normal = [0.,0.,0.]
        for i in range(1,len(face)-1):
            u,v = [vertices[face[i]][k]-a[k] for k in range(3)],[vertices[face[i+1]][k]-a[k] for k in range(3)]
            cross = [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
            normal = [normal[k]+cross[k] for k in range(3)]
        if sum(normal[k]*(a[k]-centre[k]) for k in range(3)) < 0:
            face.reverse()
    return faces


def part_solid(part: dict) -> dict:
    """🗿 Preserve exact occupied solid geometry for every native physical domain."""
    if "section" in part or "mesh" in part:
        return {"kind":"polyhedron","vertices":part_vertices(part),"faces":part_faces(part)}
    if "top_outline" in part:
        return {"kind":"loft","bottom":part["outline"],"top":part["top_outline"],"z0":part["z0"],"z1":part["z1"]}
    return {"kind":"prism","polygon":part["outline"],"z0":part["z0"],"z1":part["z1"],"holes":deepcopy(part.get("holes",[]))}


def part_body(part: dict) -> dict:
    """🪃 Produce a native swept body with an explicit profile placement and world sweep direction."""
    if "mesh" in part:
        return {"kind":"faceted_brep","vertices":part_vertices(part),"faces":part_faces(part)}
    if "section" in part:
        section = part["section"]
        xz = section["plane"] == "XZ"
        a,b = section["sweep"]
        return {"kind":"extrusion","profile":{"kind":"polygon","points":deepcopy(section["profile"])},"profile_plane":section["plane"],"position":[0,a,0] if xz else [a,0,0],"axis":[0,-1,0] if xz else [1,0,0],"ref_direction":[1,0,0] if xz else [0,1,0],"direction":[0,1,0] if xz else [1,0,0],"depth":b-a}
    if "top_outline" in part:
        return {"kind":"extruded_area_tapered","profile":part["outline"],"end_profile":part["top_outline"],"position":[0,0,part["z0"]],"direction":[0,0,1],"depth":part["z1"]-part["z0"]}
    return {"kind":"extrusion","profile":{"kind":"polygon","points":part["outline"],"inner":deepcopy(part.get("holes",[]))},"holes":deepcopy(part.get("holes",[])),"position":[0,0,part["z0"]],"direction":[0,0,1],"depth":part["z1"]-part["z0"]}


def _physical_representations(parts: list, axis_points: list, cut_z: float, cut_value: float, cut_axis: int = 1) -> dict:
    """🖼️ Project the same occupied prism or loft parts into all physical representations."""
    plans = [{"kind":"polygon","role":p.get("role","outline"),"points":part_footprint(p),"holes":deepcopy(p.get("holes",[])),**{k:p[k] for k in ("material","owner","tag") if k in p}} for p in parts]
    sections = [{"kind":"polygon","role":p.get("role","outline"),"points":ring,**{k:p[k] for k in ("material","owner","tag") if k in p}} for p in parts for ring in part_sections(p,cut_value,cut_axis)]
    elevations = [{"kind":"polygon","role":p.get("role","outline"),"points":part_elevation(p),**{k:p[k] for k in ("material","owner","tag") if k in p}} for p in parts]
    solids = [{**part_solid(p),**{k:p[k] for k in ("role","material","owner","tag") if k in p}} for p in parts]
    bodies = [{**part_body(p),**{k:p[k] for k in ("role","material","owner","tag") if k in p}} for p in parts]
    solid_type = "SolidModel" if any("mesh" in p for p in parts) else "SweptSolid"
    return {"axis":("Axis","Curve2D",[{"kind":"polyline","points":axis_points}]),"footprint":("FootPrint","Curve2D",plans),"plan":("Body","SweptSolid",[{**r,"points":part_outline_at(p,cut_z)} for r,p in zip(plans,parts) if p["z0"] <= cut_z < p["z1"]]),"section":("Body","SweptSolid",sections),"elevation":("Body","SweptSolid",elevations),"3d":("Body",solid_type,solids),"body":("Body",solid_type,bodies)}


def represent(key: str, kind: str, model: dict | None = None) -> dict:
    """👁️ Derive geometry; part elevations face south in XZ, sections cut at the part extent midpoint Y."""
    m = _m(model)
    obj = _find(m, key)
    cls = obj["class"]
    if kind not in ("plan", "section", "elevation", "3d", "axis", "footprint", "body"):
        raise ValueError(f"Unknown representation: {kind}")
    if cls in ("IfcRamp","IfcRampFlight"):
        g = _ramp_geometry(m,obj)
        parts = g["parts"]+(g.get("child_parts",[]) if cls=="IfcRamp" else [])
        vertices = [v for p in parts for v in part_vertices(p)]
        cut_y = (min(v[1] for v in vertices)+max(v[1] for v in vertices))/2
        cut_z = (min(v[2] for v in vertices)+max(v[2] for v in vertices))/2
        table = _physical_representations(parts,[],cut_z,cut_y)
        table["axis"] = ("Axis","Curve3D",[{"kind":"polyline","points":[[*f["start"],f["z0"]],[*f["end"],f["z1"]]],"owner":f["tag"],"tag":f["tag"]} for f in g["flights"]])
        for name in ("3d","body"):
            table[name] = (table[name][0],"SolidModel",table[name][2])
        derived = {**quantities(key,m),"cut_z":cut_z,"cut_y":cut_y,"solid":"IfcFacetedBrep","scope":"assembly" if cls=="IfcRamp" else "product","representation_scope":"owned_assembly_members" if cls=="IfcRamp" else "exclusive_owned_material","representation_volume":_occupied_volume(parts)}
    elif cls == "IfcColumn":
        g = column_geometry(m, obj)
        x, y, z0, z1 = g["x"], g["y"], g["base"], g["top"]
        derived = {**quantities(key, m), "profile": deepcopy(g["profile"]), "solid": "IfcExtrudedAreaSolid", "cut_z": (z0 + z1) / 2}
        plans = [{"kind": "polygon", "role": "outline", "points": p["outline"],"holes":deepcopy(p["holes"])} for p in g["parts"]]
        elevations = [{"kind": "polygon", "role": "outline", "points": [[min(v[0] for v in p["outline"]), p["z0"]],
                       [max(v[0] for v in p["outline"]), p["z0"]], [max(v[0] for v in p["outline"]), p["z1"]],
                       [min(v[0] for v in p["outline"]), p["z1"]]]} for p in g["parts"]]
        cut_y = (min(v[1] for p in g["parts"] for v in p["outline"]) + max(v[1] for p in g["parts"] for v in p["outline"])) / 2
        sections = [{"kind": "polygon", "role": "outline", "points": [[lo,p["z0"]],[hi,p["z0"]],[hi,p["z1"]],[lo,p["z1"]]]}
                    for p in g["parts"] for lo, hi in part_intervals(p,cut_y)]
        prisms = [part_solid(p) for p in g["parts"]]
        bodies = [part_body(p) for p in g["parts"]]
        table = {"axis": ("Axis", "Curve3D", [{"kind": "polyline", "points": [[x,y,z0],[x,y,z1]]}]),
                 "footprint": ("FootPrint", "Curve2D", plans), "plan": ("Body", "SweptSolid", plans),
                 "section": ("Body", "SweptSolid", sections), "elevation": ("Body", "SweptSolid", elevations),
                 "3d": ("Body", "SweptSolid", prisms), "body": ("Body", "SweptSolid", bodies)}
    elif cls == "IfcWall" and not _wall_uses_path(obj):
        g = _wall_geometry(m,obj)
        parts,openings = g["parts"],g["openings"]
        dx,dy = [(g["end"][i]-g["start"][i])/g["length"] for i in (0,1)]
        def local(p):
            x,y = p[0]-g["start"][0],p[1]-g["start"][1]
            return [x*dx+y*dy,-x*dy+y*dx]
        cut = (openings[0]["sill_z"]+openings[0]["head_z"])/2 if openings else (g["base"]+g["top"])/2
        station = (openings[0]["start"]+openings[0]["end"])/2 if openings else g["length"]/2
        plans = [{"kind":"polygon","role":"outline","layer":p["layer"],"material":p["material"],"points":p["outline"]} for p in parts]
        elevations,sections = [],[]
        for part in parts:
            ring = [local(p) for p in part["outline"]]
            lo,hi = min(p[0] for p in ring),max(p[0] for p in ring)
            elevations.append({"kind":"polygon","role":"outline","layer":part["layer"],"material":part["material"],"points":[[lo,part["z0"]],[hi,part["z0"]],[hi,part["z1"]],[lo,part["z1"]]]})
            for lo,hi in _polygon_intervals(ring,station,0):
                sections.append({"kind":"polygon","role":"outline","layer":part["layer"],"material":part["material"],"points":[[lo,part["z0"]],[hi,part["z0"]],[hi,part["z1"]],[lo,part["z1"]]]})
        table = {"axis":("Axis","Curve2D",[{"kind":"polyline","points":[g["start"],g["end"]]}]),
                 "footprint":("FootPrint","Curve2D",plans),
                 "plan":("Body","SweptSolid",[r for r,p in zip(plans,parts) if p["z0"]<=cut<p["z1"]]),
                 "elevation":("Body","SweptSolid",elevations),"section":("Body","SweptSolid",sections),
                 "3d":("Body","SweptSolid",[{"kind":"prism","polygon":p["outline"],"z0":p["z0"],"z1":p["z1"],"layer":p["layer"],"material":p["material"],"holes":[]} for p in parts]),
                 "body":("Body","SweptSolid",[{"kind":"extrusion","profile":{"kind":"polygon","points":p["outline"]},"position":[0,0,p["z0"]],"direction":[0,0,1],"depth":p["z1"]-p["z0"],"layer":p["layer"],"material":p["material"]} for p in parts])}
        derived = {**quantities(key,m),"cut_z":cut,"cut_station":station,"profile":"IfcArbitraryClosedProfileDef","solid":"IfcExtrudedAreaSolid"}

    elif cls == "IfcWall":
        g = path_wall_geometry(m,obj)
        cut = (g["openings"][0]["sill_z"]+g["openings"][0]["head_z"])/2 if g["openings"] else (g["base"]+g["top"])/2
        cut_x = sum(o[0] for o in (g["openings"][0]["p0"],g["openings"][0]["p1"]))/2 if g["openings"] else sum(p[0] for p in g["path"])/len(g["path"])
        table = _physical_representations(g["parts"],g["path"],cut,cut_x,0)
        derived = {**quantities(key,m),"cut_z":cut,"cut_x":cut_x,"profile":"IfcArbitraryClosedProfileDef","solid":"IfcExtrudedAreaSolid"}

    elif cls in ("IfcPipeSegment","IfcCovering","IfcChimney","IfcWasteTerminal","IfcBoiler","IfcSpaceHeater","IfcSanitaryTerminal","IfcLightFixture") and not is_architectural_covering(obj):
        g = service_geometry(obj["tag"], m)
        axis = g.get("points")
        if not axis:
            vertices = [v for p in g["parts"] for v in part_vertices(p)]
            lower = [min(v[i] for v in vertices) for i in range(3)]
            upper = [max(v[i] for v in vertices) for i in range(3)]
            center = obj["geometry"].get("axis",[(lower[i]+upper[i])/2 for i in (0,1)])
            axis = [[*center,lower[2]],[*center,upper[2]]]
        cut_z = sum(p[2] for p in axis)/len(axis)
        cut_value = sum(p[1] for p in axis)/len(axis)
        table = _physical_representations(g["parts"],axis,cut_z,cut_value)
        table["axis"] = ("Axis","Curve3D",[{"kind":"polyline","points":axis}])
        derived = {**quantities(key,m),"cut_z":cut_z,"cut_y":cut_value,"projection":{"horizontal":"x","vertical":"z","view":"south"},"historical_state":m.get("historical_state",{}).get("id")}

    elif cls == "IfcSlab":
        g = _slab_geometry(m, obj)
        parts = g["full_parts"]
        rings = [{"kind": "polygon", "role": "outline", "region": p["region"], "points": p["outline"]} for p in parts]
        xmin, xmax = min(v[0] for p in parts for v in p["outline"]), max(v[0] for p in parts for v in p["outline"])
        cut_y = (min(v[1] for p in parts for v in p["outline"]) + max(v[1] for p in parts for v in p["outline"])) / 2
        sections = [{"kind": "polygon", "role": "outline", "region": p["region"],
                     "points": [[lo,p["z0"]],[hi,p["z0"]],[hi,p["z1"]],[lo,p["z1"]]]}
                    for p in parts for lo, hi in _polygon_intervals(p["outline"], cut_y)]
        elevations = [{"kind": "polygon", "role": "outline", "region": p["region"],
                       "points": [[min(v[0] for v in p["outline"]),p["z0"]],[max(v[0] for v in p["outline"]),p["z0"]],
                                  [max(v[0] for v in p["outline"]),p["z1"]],[min(v[0] for v in p["outline"]),p["z1"]]]} for p in parts]
        prisms = [{"kind": "prism", "polygon": p["outline"], "z0": p["z0"], "z1": p["z1"], "holes": [], "region": p["region"]} for p in parts]
        bodies = [{"kind": "extrusion", "profile": {"kind": "polygon", "outer": p["outline"], "inner": []},
                   "position": [0,0,p["z0"]], "direction": [0,0,1], "depth": p["thickness"], "region": p["region"]} for p in parts]
        derived = {**quantities(obj["tag"], m), "cut_y": _round(cut_y), "projection": {"horizontal": "x", "vertical": "z", "view": "south"},
                   "profile": "IfcArbitraryClosedProfileDef", "solid": "IfcExtrudedAreaSolid","scope":"full_member"}
        table = {"axis": ("Axis", "Curve2D", [{"kind": "polyline", "points": [[xmin,cut_y],[xmax,cut_y]]}]),
                 "footprint": ("FootPrint", "Curve2D", rings), "plan": ("Body", "SweptSolid", rings),
                 "section": ("Body", "SweptSolid", sections), "elevation": ("Body", "SweptSolid", elevations),
                 "3d": ("Body", "SweptSolid", prisms), "body": ("Body", "SweptSolid", bodies)}
    elif cls == "IfcOpeningElement" and "slab" in obj["geometry"]:
        g = _slab_opening_geometry(m,obj)
        parts = g["parts"]
        cut_y = (min(v[1] for v in g["outline"])+max(v[1] for v in g["outline"]))/2
        rings = [{"kind":"polygon","role":"hole","points":g["outline"]}]
        section = [{"kind":"polygon","role":"hole","points":[[lo,p["z0"]],[hi,p["z0"]],[hi,p["z1"]],[lo,p["z1"]]]} for p in parts for lo,hi in _polygon_intervals(p["outline"],cut_y)]
        elevation = [{"kind":"polygon","role":"hole","points":part_elevation(p)} for p in parts]
        solids = [{"kind":"prism","polygon":p["outline"],"z0":p["z0"],"z1":p["z1"],"holes":[]} for p in parts]
        bodies = [{"kind":"extrusion","profile":{"kind":"polygon","points":p["outline"]},"position":[0,0,p["z0"]],"direction":[0,0,1],"depth":p["z1"]-p["z0"]} for p in parts]
        derived = {**quantities(key,m),"cut_y":cut_y,"projection":{"horizontal":"x","vertical":"z","view":"south"}}
        table = {"axis":("Axis","Curve2D",[{"kind":"polyline","points":[[min(v[0] for v in g["outline"]),cut_y],[max(v[0] for v in g["outline"]),cut_y]]}]),
                 "footprint":("FootPrint","Curve2D",rings),"plan":("Body","SweptSolid",rings),"section":("Body","SweptSolid",section),
                 "elevation":("Body","SweptSolid",elevation),"3d":("Body","SweptSolid",solids),"body":("Body","SweptSolid",bodies)}
    elif cls == "IfcOpeningElement":
        geometry=opening_geometry(key,m)
        parts=geometry["parts"]
        cut_z=(geometry["sill_z"]+geometry["head_z"])/2
        cut_x=(min(v[0] for p in parts for v in p["outline"])+max(v[0] for p in parts for v in p["outline"]))/2
        table=_physical_representations(parts,geometry["axis"],cut_z,cut_x,0)
        derived={**quantities(key,m),"cut_z":cut_z,"cut_x":cut_x,"host":geometry["host"]}
    elif cls in ("IfcWindow","IfcDoor"):
        physical=_filling_geometry(m,obj)
        parts=physical["parts"]
        cut_z=(physical["clear_sill_z"]+physical["clear_head_z"])/2
        cut_y=(min(v[1] for p in parts for v in p["outline"])+max(v[1] for p in parts for v in p["outline"]))/2
        table=_physical_representations(parts,physical["axis"],cut_z,cut_y)
        derived={**quantities(key,m),"cut_z":cut_z,"cut_y":cut_y,"host":physical["host"],"host_class":physical["host_class"],"projection":{"horizontal":"x","vertical":"z","view":"south"}}

    elif cls == "IfcCurtainWall":
        physical = _curtain_filling_geometry(m,obj)
        paths = [p["path"] for p in physical["paths"]]
        parts = physical["parts"]
        cut_z = (min(p["z0"] for p in parts)+max(p["z1"] for p in parts))/2
        cut_y = (min(v[1] for p in parts for v in p["outline"])+max(v[1] for p in parts for v in p["outline"]))/2
        outlines = [{"kind":"polygon","role":p["role"],"material":p["material"],"points":p["outline"]} for p in parts]
        table = {"axis":("Axis","Curve2D",[{"kind":"polyline","points":path} for path in paths]),
                 "footprint":("FootPrint","Curve2D",outlines),
                 "plan":("Body","SweptSolid",[r for r,p in zip(outlines,parts) if p["z0"]<=cut_z<p["z1"]]),
                 "elevation":("Body","SweptSolid",[{"kind":"polygon","role":p["role"],"points":[[min(v[0] for v in p["outline"]),p["z0"]],[max(v[0] for v in p["outline"]),p["z0"]],[max(v[0] for v in p["outline"]),p["z1"]],[min(v[0] for v in p["outline"]),p["z1"]]]} for p in parts]),
                 "section":("Body","SweptSolid",[{"kind":"polygon","role":p["role"],"points":[[lo,p["z0"]],[hi,p["z0"]],[hi,p["z1"]],[lo,p["z1"]]]} for p in parts for lo,hi in _polygon_intervals(p["outline"],cut_y)]),
                 "3d":("Body","SweptSolid",[{"kind":"prism","role":p["role"],"material":p["material"],"polygon":p["outline"],"z0":p["z0"],"z1":p["z1"],"holes":[]} for p in parts]),
                 "body":("Body","SweptSolid",[{"kind":"extrusion","profile":{"kind":"polygon","points":p["outline"]},"position":[0,0,p["z0"]],"direction":[0,0,1],"depth":p["z1"]-p["z0"],"role":p["role"],"material":p["material"]} for p in parts])}
        derived = {**quantities(key,m),"cut_z":cut_z,"cut_y":cut_y,"projection":{"horizontal":"x","vertical":"z","view":"south"}}
    elif cls == "IfcStair" or "parts" in obj.get("geometry", {}) or obj.get("geometry",{}).get("kind")=="ramp_finish":
        stair = _stair_geometry(m, obj) if cls == "IfcStair" else None
        solid = None if stair else solid_geometry(obj["tag"], m)
        parts = stair["parts"] if stair else solid["parts"]
        derived = quantities(obj["tag"], m)
        derived = {**derived,"representation_scope":"exclusive_owned_material","representation_volume":derived.get("exclusive_volume",derived["volume"])}
        footprints = [point for p in parts for point in part_footprint(p)]
        xmin,xmax = min(v[0] for v in footprints),max(v[0] for v in footprints)
        ymin,ymax = min(v[1] for v in footprints),max(v[1] for v in footprints)
        cut_y = (ymin+ymax)/2
        cut_z = (min(p["z0"] for p in parts)+max(p["z1"] for p in parts))/2
        derived = {**derived,"cut_y":_round(cut_y),"cut_z":cut_z,"projection":{"horizontal":"x","vertical":"z","view":"south"}}
        table = _physical_representations(parts,[[xmin,cut_y],[xmax,cut_y]],cut_z,cut_y)

    else:
        raise ValueError(f"No representation for {cls}")
    identifier, kind_type, primitives = table[kind]
    return {"key": obj["tag"], "guid": obj["guid"], "kind": kind, "identifier": identifier, "type": kind_type, "primitives": primitives, "derived": derived}


def _wall_geometry_list(model: dict) -> list:
    junctions = _wall_junction_backend().context(SimpleNamespace(**globals()),model)
    return [_wall_geometry(model,w,junctions=junctions) for w in _by_class(model,"IfcWall") if not _wall_uses_path(w)]


def _bump(model: dict, tags: list) -> None:
    model["revision"] += 1
    registry = _registry(model)
    for tag in tags:
        registry[tag]["metadata"]["revision"] = model["revision"]
    for spec in model["models"]:
        if any(tag in spec["element_tags"] for tag in tags):
            spec["revision"] = model["revision"]


def demo_event(name: str, model: dict | None = None) -> dict:
    """🎬 Return an authored scene event payload by name (sill, roof_soil, roof_soil_alt)."""
    events = _m(model)["demo_events"]
    if name not in events:
        raise ValueError(f"Unknown demo event: {name}")
    return deepcopy(events[name])


def _number(value, name: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise ValueError(f"{name} must be a finite number")
    return value


def apply_event(event: dict, model: dict | None = None) -> dict:
    """⚡ Apply one design event to a copy of the model, raise revisions and keep every GlobalId."""
    source = deepcopy(_m(model))
    kind = event.get("type")
    allowed = {"OpeningResized": {"type", "object_guid", "sill", "head"}, "LayerThicknessChanged": {"type", "object_guid", "layer", "value", "unit"}}
    if kind not in allowed or not set(event) - {"basis", "note"} == allowed[kind]:
        raise ValueError("Invalid event")
    registry = _registry(source)
    target = registry.get(event["object_guid"])
    if target is None:
        raise ValueError("Unknown object")
    if kind == "OpeningResized":
        if target["class"] != "IfcWindow":
            raise ValueError("OpeningResized needs a window")
        sill, head = _number(event["sill"], "sill"), _number(event["head"], "head")
        opening = registry[target["geometry"]["opening"]]
        domain = _opening_domain(source,opening)
        wall_height = domain["top"]-domain["base"]
        if not 0 <= sill < head <= wall_height:
            raise ValueError("Opening must lie between floor and wall top")
        opening["attributes"]["sill"]["value"], opening["attributes"]["head"]["value"] = sill, head
        _bump(source, [target["tag"], opening["tag"], domain["host"]])
    else:
        if target["class"] != "IfcSlab" or event["unit"] != "m":
            raise ValueError("LayerThicknessChanged needs a layered slab and metres")
        value = _number(event["value"], "value")
        layerset = _material(source, target)
        if value <= 0 or layerset is None or "layers" not in layerset:
            raise ValueError("Layer thickness must be positive on a layered slab")
        layer = next((l for l in layerset["layers"] if l["id"] == event["layer"]), None)
        if layer is None:
            raise ValueError("Unknown layer")
        layer["thickness"]["value"] = value
        _bump(source, _flooring_layer_backend().dependencies(source,layerset["tag"],layer["id"]))
    return source


def _link_values(model: dict, kind: str) -> dict:
    function = globals().get("link_values")
    if function is None:
        raise RuntimeError("The analysis submodule is missing")
    return function(model, kind)


def consistency_issues(model: dict | None = None) -> list:
    """🚦 Compare analysis links with the current model: revision lag and differing used values."""
    m = _m(model)
    registry = _registry(m)
    issues = []
    for link in m["links"]:
        target = registry.get(link["object_guid"])
        if target is None:
            issues.append({"code": "unknown-object", "id": link["id"], "object_guid": link["object_guid"], "expected": None, "actual": link["object_guid"]})
            continue
        if _revision(m, target) > link["source_revision"]:
            issues.append({"code": "stale-link", "id": link["id"], "object_guid": link["object_guid"], "expected": _revision(m, target), "actual": link["source_revision"]})
        current = _link_values(m, link["kind"])
        for name, used in link["used"].items():
            if not math.isclose(current[name], used, rel_tol=1e-9, abs_tol=1e-9):
                issues.append({"code": "stale-value", "id": link["id"], "object_guid": link["object_guid"], "key": name, "expected": current[name], "actual": used})
    for spec in m["models"]:
        for tag in spec["element_tags"]:
            if tag not in registry:
                issues.append({"code": "unknown-object", "id": spec["id"], "object_guid": tag, "expected": None, "actual": tag})
    return issues


def resolve_link(model: dict | None, link_id: str) -> dict:
    """🔧 Regenerate one analysis link from the current model on a copy and keep the revision."""
    source = deepcopy(_m(model))
    link = next((x for x in source["links"] if x["id"] == link_id), None)
    if link is None:
        raise ValueError(f"Unknown link: {link_id}")
    link["used"] = _link_values(source, link["kind"])
    link["source_revision"] = source["revision"]
    return source



def service_topology(model: dict | None = None) -> dict:
    """🕸️ Return validated source-evidence connectivity, separately from unknown metric pipe routes."""
    source=_m(model)
    domain=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔧️services.🐍️")
    return domain.validated(source["service_topology"],{e["tag"] for e in source["elements"]})

def service_reachable(start: str, model: dict | None = None) -> list:
    """📡 Query attested logical service reachability without implying an authored concealed pipe run."""
    domain=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔧️services.🐍️")
    return domain.reachable(service_topology(model),start)

_exchange = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
_analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
step_excerpt, wall_junction_excerpt, mesh, ids_check = _exchange.step_excerpt, _exchange.wall_junction_excerpt, _exchange.mesh, _exchange.ids_check
translate_structure, tributary = _analysis.translate_structure, _analysis.tributary
clash_check, link_values = _analysis.clash_check, _analysis.link_values
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
