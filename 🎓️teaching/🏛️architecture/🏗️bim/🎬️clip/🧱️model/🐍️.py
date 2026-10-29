"""🏛️ Schema-first Villa Savoye teaching model: one IFC 4.3 JSON source, stdlib-only derivations."""

from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
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


@lru_cache(maxsize=1)
def _source() -> str:
    return Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8")


def load_model() -> dict:
    """📦 Load a fresh, language-independent Villa Savoye teaching model."""
    return json.loads(_source())


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
    """📍 Resolve unstationed slab soffits to the lowest occupied regional underside."""
    if isinstance(spec, (int, float)):
        return spec
    if spec == "overall":
        return _box(model)[2]
    if spec.startswith("soffit:"):
        slab = _find(model, spec[7:])
        return min(part["z0"] for part in _slab_geometry(model, slab)["parts"])
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
    """🔺 Counterclockwise boundary of a convex set of planar points."""
    points = sorted(set(tuple(map(float, p)) for p in points))
    if len(points) < 3:
        return [list(p) for p in points]
    def half(items):
        result = []
        for p in items:
            while len(result) > 1 and (result[-1][0]-result[-2][0])*(p[1]-result[-2][1])-(result[-1][1]-result[-2][1])*(p[0]-result[-2][0]) <= 1e-12:
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
        scale = math.sqrt(sum(x*x for x in normal))
        if scale<1e-14 or any(abs(sum(normal[k]*(vertices[i][k]-a[k]) for k in range(3)))>1e-9*scale for i in face) or any(sum(normal[k]*(vertices[i][k]-a[k]) for k in range(3))>1e-9*scale for i in range(len(vertices))):
            raise ValueError("Native polyhedron faces must bound a convex solid")
    return part


def _polyhedron_volume(part: dict) -> float:
    """🧮 Integrate outward native face triangles around a local origin."""
    vertices = part_vertices(part)
    origin,total = vertices[0],0.
    for face in part_faces(part):
        for i in range(1,len(face)-1):
            a,b,c = [[vertices[j][k]-origin[k] for k in range(3)] for j in (face[0],face[i],face[i+1])]
            total += a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0])
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
    t = min(1., max(0., (z-part["z0"])/(part["z1"]-part["z0"])))
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
    """🕳️ Return all connected occupied polygons in a physical vertical cut."""
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
    """🏛️ Convex loft silhouette with horizontal coordinate orthogonal to axis."""
    return _convex_hull([[p[1-axis],p[2]] for p in part_vertices(part)])


def part_section(part: dict, value: float, axis: int = 1) -> list:
    """✂️ True convex loft intersection in horizontal-height coordinates."""
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
    """🧮 Integrate occupied slice unions, including linearly tapered convex solids."""
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
    """📦 Integrate an explicitly disjoint vertical material partition."""
    return sum((_area(p["outline"])-sum(_area(h) for h in p.get("holes",())))*(p["z1"]-p["z0"]) for p in parts)


def _length(path: list) -> float:
    return sum(math.dist(a, b) for a, b in zip(path, path[1:]))


def _subtract_prisms(parts: list, cutters: list) -> list:
    """🪚 Partition material prisms around other explicitly owned solids."""
    analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
    def bounds(ring):
        return min(p[0] for p in ring),min(p[1] for p in ring),max(p[0] for p in ring),max(p[1] for p in ring)
    if any("section" in p for p in [*parts,*cutters]):
        raise ValueError("Material prism subtraction requires vertical profile prisms")
    parts = [{**p,"outline":ring,"holes":[]} for p in parts for ring in part_regions(p,(p["z0"]+p["z1"])/2)] if any(p.get("holes") for p in parts) else parts
    indexed = [(bounds(c["outline"]),c) for c in cutters if c["z1"] > c["z0"] and len(c["outline"]) >= 3]
    result = []
    for part in parts:
        xmin,ymin,xmax,ymax = bounds(part["outline"])
        active = [c for (a,b,d,e),c in indexed if c["z0"] < part["z1"]-1e-10 and c["z1"] > part["z0"]+1e-10 and a < xmax-1e-10 and d > xmin+1e-10 and b < ymax-1e-10 and e > ymin+1e-10]
        if not active:
            result.append(deepcopy(part))
            continue
        levels = sorted({part["z0"],part["z1"],*(max(part["z0"],min(part["z1"],c[key])) for c in active for key in ("z0","z1"))})
        for lo,hi in zip(levels,levels[1:]):
            pieces = analysis._convex_parts(part["outline"])
            for cutter in active:
                if cutter["z0"] < (lo+hi)/2 < cutter["z1"]:
                    updated = []
                    for current in pieces:
                        updated += [piece for piece in analysis._subtract(current,cutter["outline"],digits=12,tolerance=1e-12) if _area(piece)>1e-12]
                        for hole in cutter.get("holes",()):
                            clip = analysis._clip(current,hole if analysis._signed(hole)>0 else list(reversed(hole)))
                            if len(clip)>=3 and _area(clip)>1e-12:
                                updated.append(clip)
                    pieces = updated
                if not pieces:
                    break
            if 1 < len(pieces) <= 64:
                rings = analysis._loops(pieces,tolerance=1e-12)
                if rings and all(analysis._signed(ring) > 1e-12 for ring in rings):
                    if math.isclose(sum(_area(ring) for ring in rings),sum(_area(piece) for piece in pieces),rel_tol=1e-11,abs_tol=1e-12):
                        pieces = rings
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
    """🧱 Return one object by GlobalId or tag with merged type psets and a resolved material."""
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
    """🪟 Resolve source reference paths and independently authored physical member stations."""
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    return provider.resolve_paths(SimpleNamespace(**globals()),model,curtain)


def _opening_domain(model: dict, opening: dict) -> dict:
    """🧭 Resolve a native linear host without fabricating a wall for curtain apertures."""
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
    """🕳️ Resolve one finite native wall, slab or curtain aperture into the shared physical domain."""
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
    """🌀 Partition a simple mitered band at cumulative path stations without overlapping boxes."""
    lengths = [math.dist(a,b) for a,b in zip(path,path[1:])]
    if not lengths or min(lengths) <= 1e-10 or thickness <= 0:
        raise ValueError("A path wall requires a positive thickness and distinct vertices")
    nodes, offsets = [path[0]], [0.]
    station = 0.
    for a,b,length in zip(path,path[1:],lengths):
        for s in sorted(v for v in stations if station+1e-9 < v < station+length-1e-9):
            nodes.append([a[i]+(b[i]-a[i])*(s-station)/length for i in (0,1)])
            offsets.append(s)
        nodes.append(b)
        station += length
        offsets.append(station)
    normals = [[-(b[1]-a[1])/math.dist(a,b),(b[0]-a[0])/math.dist(a,b)] for a,b in zip(nodes,nodes[1:])]
    miters = [normals[0]]
    for a,b in zip(normals,normals[1:]):
        denominator = 1+a[0]*b[0]+a[1]*b[1]
        if denominator < 1e-8:
            raise ValueError("A path wall cannot reverse through a cusp")
        miters.append([(a[i]+b[i])/denominator for i in (0,1)])
    miters.append(normals[-1])
    if math.dist(nodes[0],nodes[-1])<1e-9:
        a,b = normals[-1],normals[0]
        denominator = 1+a[0]*b[0]+a[1]*b[1]
        if denominator<1e-8:
            raise ValueError("A closed path wall cannot reverse through a cusp")
        miters[0] = [(a[i]+b[i])/denominator for i in (0,1)]
        miters[-1] = list(miters[0])
    left = [[p[i]+thickness/2*n[i] for i in (0,1)] for p,n in zip(nodes,miters)]
    right = [[p[i]-thickness/2*n[i] for i in (0,1)] for p,n in zip(nodes,miters)]
    return [{"outline":[right[i],right[i+1],left[i+1],left[i]],"start":offsets[i],"end":offsets[i+1]} for i in range(len(nodes)-1)]


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
    """🧱 Resolve a continuous curved wall, hosted station apertures and a sloping crest into occupied solids."""
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
    body,displaced,body_volume = _wall_owned_parts(model,[p for p in parts if p["role"]=="body"])
    caps = [p for p in parts if p["role"]=="cap"]
    gross += caps
    parts = body+caps
    cavities,_,_ = _wall_owned_parts(model,cavities)
    def volume(collection):
        return sum((p["z1"]-p["z0"])*(_area(p["outline"])+4*_area(part_outline_at(p,(p["z0"]+p["z1"])/2))+_area(part_outline_at(p,p["z1"])))/6 for p in collection)
    for opening in openings:
        opening["parts"] = [p for p in voids if p["opening"]==opening["tag"]]
        opening["material_parts"] = [p for p in opening_material if p["opening"]==opening["tag"]]
        opening["material_volume"] = volume(opening["material_parts"])
        opening['p0']=apertures[opening['tag']]['axis'][0]
        opening['p1']=apertures[opening['tag']]['axis'][-1]
    return {"tag":wall["tag"],"guid":wall["guid"],"storey":wall["storey"],"path":path,"base":base,"body_top":body_top,"top":top,"thickness":thickness,"length":length,"openings":openings,"parts":parts,"gross_parts":gross,"cavity_parts":cavities,"gross_material_volume":volume(gross),"gross_envelope_volume":volume([{**b,"z0":base,"z1":body_top} for b in bands]+caps),"gross_cavity_volume":volume(gross_cavities),"cavity_volume":volume(cavities),"opening_material_volume":volume(opening_material),"joint_displacement_volume":displaced,"solid_volume":volume(parts)}


def _wall_uses_path(wall: dict) -> bool:
    """🧭 Distinguish native path geometry independently of an IFC wall classification."""
    path = wall["geometry"]["path"]
    return len(path)!=2 or any(not isinstance(point,list) for point in path)


def _wall_geometry(model: dict, wall: dict) -> dict:
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


def _wall_cutters(model: dict) -> list:
    """🏗️ Resolve explicitly owned concrete and filling solids for material interfaces."""
    classes = {"IfcColumn","IfcBeam","IfcSlab","IfcWall","IfcOpeningElement","IfcWindow","IfcDoor","IfcCurtainWall"}
    dependencies = [model[key] for key in ("types","materials","storeys","car","grid","building","spaces")]
    dependencies.append([obj for obj in model["elements"] if obj["class"] in classes])
    key = hashlib.sha256(json.dumps(dependencies,sort_keys=True,separators=(",",":")).encode()).hexdigest()
    if key not in _WALL_CUTTERS:
        parts = []
        for obj in model["elements"]:
            if obj["class"] == "IfcColumn":
                owned = column_geometry(model,obj)["parts"]
            elif obj["class"] == "IfcBeam":
                solid = _solid_geometry(model,obj)
                owned = solid["parts"]+solid["shared_parts"]
            elif obj["class"] in ("IfcWindow","IfcDoor"):
                owned = _filling_geometry(model,obj)["parts"]
            elif obj["class"] == "IfcCurtainWall":
                owned = _curtain_filling_geometry(model,obj)["parts"]
            else:
                continue
            parts += [{"outline":part["outline"],"z0":part["z0"],"z1":part["z1"],"owner":obj["tag"]} for part in owned]
        _WALL_CUTTERS[key] = parts
        if len(_WALL_CUTTERS) > 8:
            _WALL_CUTTERS.pop(next(iter(_WALL_CUTTERS)))
    return _WALL_CUTTERS[key]


def _wall_owned_parts(model: dict, parts: list) -> tuple:
    """🧩 Keep brick and plaster outside other members' occupied material."""
    if not parts:
        return [],0.,0.
    xmin = min(p[0] for part in parts for p in part["outline"])
    xmax = max(p[0] for part in parts for p in part["outline"])
    ymin = min(p[1] for part in parts for p in part["outline"])
    ymax = max(p[1] for part in parts for p in part["outline"])
    zmin,zmax = min(p["z0"] for p in parts),max(p["z1"] for p in parts)
    cutters = [c for c in _wall_cutters(model) if c["z0"] < zmax-1e-10 and c["z1"] > zmin+1e-10
               and min(p[0] for p in c["outline"]) < xmax-1e-10 and max(p[0] for p in c["outline"]) > xmin+1e-10
               and min(p[1] for p in c["outline"]) < ymax-1e-10 and max(p[1] for p in c["outline"]) > ymin+1e-10]
    key = hashlib.sha256(json.dumps([parts,cutters],sort_keys=True,separators=(",",":")).encode()).hexdigest()
    if key not in _WALL_OWNERSHIP:
        owned = _subtract_prisms(parts,cutters)
        volume = sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in owned)
        before = sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in parts)
        _WALL_OWNERSHIP[key] = owned,max(0.,before-volume),volume
        if len(_WALL_OWNERSHIP) > 256:
            _WALL_OWNERSHIP.pop(next(iter(_WALL_OWNERSHIP)))
    owned,displaced,volume = _WALL_OWNERSHIP[key]
    return deepcopy(owned),displaced,volume


def _wall_station(spec: dict, geometry: dict) -> float:
    """📍 Resolve material-band stations from the wall envelope or an actual host aperture."""
    datum = spec["datum"]
    if datum in ("base","top"):
        value = geometry[datum]
    else:
        opening = next((o for o in geometry["openings"] if o["tag"]==spec["opening"]),None)
        if opening is None:
            raise ValueError("A wall-band station must reference an opening in that wall")
        value = opening[datum+"_z"]
    return value+spec["offset"]["value"]


def _wall_parts(model: dict, wall: dict, geometry: dict) -> tuple:
    """🧱 Partition opening-linked material bands against apertures and physical member ownership."""
    g = geometry
    dx,dy = [(g["end"][i]-g["start"][i])/g["length"] for i in (0,1)]
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
    xs = sorted({0.,g["length"],*(max(0.,min(g["length"],o[k])) for o in openings for k in ("start","end"))})
    resolved,parts,voids = [],[],[]
    for band in bands:
        zs = sorted({band["z0"],band["z1"],*(max(band["z0"],min(band["z1"],o[k])) for o in openings for k in ("sill_z","head_z"))})
        cells = [(a,b,c,d) for a,b in zip(xs,xs[1:]) for c,d in zip(zs,zs[1:]) if b>a and d>c and not any(o["start"]<(a+b)/2<o["end"] and o["sill_z"]<(c+d)/2<o["head_z"] for o in openings)]
        side_area = sum((b-a)*(d-c) for a,b,c,d in cells)
        offset = g["thickness"]/2
        for layer in band["layers"]:
            thickness = layer["thickness"]["value"]
            inner = offset-thickness
            lo,hi = sorted([side*inner,side*offset])
            resolved.append({"id":layer["id"],"region":band["tag"],"material":layer["material"],"offsets":[lo,hi],"is_void":layer.get("is_void",False),"z0":band["z0"],"z1":band["z1"],"gross_side_area":g["length"]*(band["z1"]-band["z0"]),"net_side_area":side_area})
            for a,b,z0,z1 in cells:
                ring = [[g["start"][0]+dx*s-dy*t,g["start"][1]+dy*s+dx*t] for s,t in ((a,lo),(b,lo),(b,hi),(a,hi))]
                (voids if layer.get("is_void") else parts).append({"outline":ring,"z0":z0,"z1":z1,"layer":layer["id"],"region":band["tag"],"material":layer["material"],"start":a,"end":b,"offsets":[lo,hi]})
            offset = inner
    owned,displaced,volume = _wall_owned_parts(model,parts)
    air,_,_ = _wall_owned_parts(model,voids)
    volumes = {}
    for part in owned+air:
        key = part["region"],part["layer"]
        volumes[key] = volumes.get(key,0.)+_area(part["outline"])*(part["z1"]-part["z0"])
    for layer in resolved:
        layer["volume"] = volumes.get((layer["region"],layer["id"]),0.)
    return resolved,owned,displaced,volume


def _panel_layout_parts(model: dict, spec: dict, origin: list, direction: list, bounds: tuple, offset: float) -> tuple:
    """🪟 Partition mixed infills, shared metal members and layered hollow leaves without duplicate material."""
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
        """🧩 Place one disjoint rectangular occupied material part in the native host plane."""
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


def _filling_geometry(model: dict, filling: dict) -> dict:
    """🪟 Resolve perimeter members and panel solids from one typed rough aperture."""
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
    s0,s1,z0,z1=0.,length,base+v["sill"],base+v["head"]
    w,d,t = [spec[k]["value"] for k in ("frame_width","frame_depth","panel_thickness")]
    offset = domain.get("offset",0.)+domain["side"]*(domain["thickness"]/2-spec["inset"]["value"]-d/2)
    low = z0+w if spec["threshold"] else z0
    if s1-s0 <= 2*w or z1-low <= w:
        raise ValueError("A filling frame consumes its complete aperture")
    parts = []
    def part(u0,u1,h0,h1,t0,t1,material,role):
        ring = [[a[0]+dx*s-dy*n,a[1]+dy*s+dx*n] for s,n in ((u0,t0),(u1,t0),(u1,t1),(u0,t1))]
        parts.append({"outline":ring,"z0":h0,"z1":h1,"material":material,"role":role})
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
        stations = [v["value"] for v in spec["panel_stations"]]
        if stations[0] != 0 or stations[-1] != 1 or any(b<=a for a,b in zip(stations,stations[1:])):
            raise ValueError("Panel stations must strictly partition the unit interval")
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
                panel_offset = offset-d/2+track+lane_width/2+(panel_index%2)*(lane_width+track)
            first_part = len(parts)
            if sash:
                if u1-u0<=2*sash or h1-h0<=2*sash:
                    raise ValueError("Sash members consume their panel")
                for x0,x1,y0,y1 in [(u0,u0+sash,h0,h1),(u1-sash,u1,h0,h1),(u0+sash,u1-sash,h0,h0+sash),(u0+sash,u1-sash,h1-sash,h1)]:
                    part(x0,x1,y0,y1,panel_offset-sash_depth/2,panel_offset+sash_depth/2,spec["frame_material"],"sash")
                u0,u1,h0,h1 = u0+sash,u1-sash,h0+sash,h1-sash
            if "layers" not in panel:
                part(u0,u1,h0,h1,panel_offset-t/2,panel_offset+t/2,panel["tag"],"glass" if panel.get("category")=="glass" else "panel")
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
    return {"tag":filling["tag"],"guid":filling["guid"],"class":filling["class"],"opening":opening["tag"],"host":domain["host"],"host_class":domain["host_class"],"parts":parts,
            "operation":spec["operation"],"frame_width":w,"frame_depth":d,"panel_thickness":t,"rough_width":length,"rough_height":z1-z0,"clear_width":clear_end-clear_start,"clear_height":clear_head_z-clear_sill_z,"clear_start":v["start"]+clear_start,"clear_end":v["start"]+clear_end,"clear_sill_z":clear_sill_z,"clear_head_z":clear_head_z,"storey":filling["storey"],"axis":[[a[0]+dx*s-dy*offset,a[1]+dy*s+dx*offset] for s in (0.,length)]}


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
    """🪟 Resolve source-authored planar panes and native metal members through one physical provider."""
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    return provider.physical_geometry(SimpleNamespace(**globals()),model,curtain)

def _ramp_geometry(model: dict, obj: dict) -> dict:
    """🛝️ Resolve real flight slabs and landing ownership without changing source stations."""
    if obj["class"] not in ("IfcRamp","IfcRampFlight"):
        raise ValueError("Ramp geometry requires a native ramp or ramp flight")
    ramp = obj if obj["class"]=="IfcRamp" else _find(model,obj["geometry"]["ramp"])
    if ramp["class"]!="IfcRamp":
        raise ValueError("A native flight requires an authored ramp parent")
    flights,children,parts,shared = [],[],[],[]
    for flight in _flights(model,ramp):
        if obj["class"]=="IfcRampFlight" and flight["tag"]!=obj["tag"]:
            continue
        source = _find(model,flight["tag"])
        if source["geometry"]["thickness_direction"]!="vertical":
            raise ValueError("A ramp slab requires its explicit vertical thickness convention")
        a,b = flight["start"],flight["end"]
        run = math.dist(a,b)
        if run<=1e-12 or flight["width"]<=0 or flight["thickness"]<=0:
            raise ValueError("A ramp flight requires positive run, width and thickness")
        normal = [-(b[1]-a[1])/run,(b[0]-a[0])/run]
        top = [[p[0]+sign*normal[0]*flight["width"]/2,p[1]+sign*normal[1]*flight["width"]/2,z] for p,z,sign in ((a,flight["z0"],1),(a,flight["z0"],-1),(b,flight["z1"],-1),(b,flight["z1"],1))]
        slab = {**_polyhedron_part([[x,y,z-flight["thickness"]] for x,y,z in top]+top),"owner":flight["tag"],"owner_guid":flight["guid"],"tag":flight["tag"],"role":"flight","material":_material(model,source)["tag"]}
        children.append(slab)
        flights.append({**flight,"parts":[deepcopy(slab)]})
    landings = []
    if obj["class"]=="IfcRamp":
        g = obj["geometry"]
        if g["thickness_direction"]!="vertical":
            raise ValueError("A ramp assembly requires its explicit vertical thickness convention")
        lo,hi = _z(model,g["from_level"]),_z(model,g["to_level"])
        for landing in g["landings"]:
            top = _round(lo+(hi-lo)*landing["portion"])
            owner = _find(model,landing["owner"])
            if owner["class"]!="IfcRamp" or not any(p["tag"]==landing["tag"] and p["owner"]==owner["tag"] for p in owner["geometry"]["landings"]):
                raise ValueError("A landing must reference its native owning ramp component")
            registered = next(p for p in owner["geometry"]["landings"] if p["tag"]==landing["tag"] and p["owner"]==owner["tag"])
            origin = _z(model,owner["geometry"]["from_level"])
            owner_top = origin+(_z(model,owner["geometry"]["to_level"])-origin)*registered["portion"]
            if registered["outline"]!=landing["outline"] or registered["thickness"]!=landing["thickness"] or abs(owner_top-top)>1e-10:
                raise ValueError("A borrowed landing must retain its registered owner geometry")
            part = {"outline":_expand(model,landing["outline"]),"z0":_round(top-landing["thickness"]["value"]),"z1":top,"owner":owner["tag"],"owner_guid":owner["guid"],"tag":landing["tag"],"role":"landing","material":_material(model,owner)["tag"]}
            (parts if owner["tag"]==obj["tag"] else shared).append(part)
            landings.append({"tag":landing["tag"],"owner":owner["tag"],"owner_guid":owner["guid"],"outline":deepcopy(part["outline"]),"z":top,"thickness":landing["thickness"]["value"],"parts":[deepcopy(part)]})
    else:
        parts,children = children,[]
    return {"tag":obj["tag"],"guid":obj["guid"],"class":obj["class"],"storey":obj["storey"],"name":deepcopy(obj["name"]),"domains":deepcopy(obj["metadata"]["domains"]),"thickness_direction":"vertical","parts":parts,"shared_parts":shared,"child_parts":children,"full_parts":deepcopy(parts+shared+children),"flights":flights,"landings":landings,"width":obj["attributes"]["width"]["value"],"children":[{"tag":f["tag"],"guid":f["guid"],"class":"IfcRampFlight"} for f in flights] if obj["class"]=="IfcRamp" else []}


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
                    "thickness": g["thickness"]["value"], "width": flight["attributes"]["width"]["value"], "z0": z0, "z1": z1, "run": _round(run), "slope": _round((z1 - z0) / run)})
    return out


def _surface_bounds(polygon: list) -> tuple:
    """🗜️ Bound an XY polygon for exact disjointness rejection."""
    return (min(p[0] for p in polygon),min(p[1] for p in polygon),max(p[0] for p in polygon),max(p[1] for p in polygon))


def _surface_bounds_overlap(a: tuple, b: tuple) -> bool:
    return a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]


def _surface_difference(pieces: list, voids: list) -> list:
    """🪓 Subtract only holes whose bounding boxes can remove occupied area."""
    for void in voids:
        bounds = _surface_bounds(void)
        pieces = [region for piece in pieces for region in
                  (_analysis._subtract(piece,void,digits=12,tolerance=1e-12) if _surface_bounds_overlap(_surface_bounds(piece),bounds) else [piece])]
    return [p for p in pieces if len(p) >= 3 and _area(p) > 1e-12]


def _slab_surface(outline: list, bounds: list, voids: list) -> list:
    pieces = [_analysis._clip(a,b,digits=12) for a in _analysis._convex_parts(outline) for b in _analysis._convex_parts(bounds)]
    return _surface_difference([p for p in pieces if len(p) >= 3 and _area(p) > 1e-12],voids)


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


def _slab_finish_parts(slab):
    """🧱 Resolve a level roof finish over different structural slab depths."""
    paving = next((layer for layer in slab.get("layers", ()) if layer["id"] == "paving"), None)
    if paving is None:
        return []
    top = min(part["z1"] for part in slab["parts"]) + paving["thickness"]
    if any(top < part["z1"] - 1e-9 for part in slab["parts"]):
        raise ValueError("Roof finish cannot be lower than the structural surface")
    return [{"outline": deepcopy(part["outline"]), "z0": part["z1"], "z1": _round(top), "region": part["region"], "layer": "paving"}
            for part in slab["parts"] if top > part["z1"] + 1e-9]


_SLAB_GEOMETRIES = {}


def _slab_geometry(model: dict, slab: dict) -> dict:
    """⚡ Cache occupied slab partitions by their actual geometry dependencies."""
    dependency = [slab, model["storeys"], model["materials"], model["types"], model["spaces"], model["car"],
                  [obj for obj in model["elements"] if obj["class"] == "IfcCurtainWall" or (obj["class"] == "IfcOpeningElement" and obj["geometry"].get("slab") == slab["tag"])],
                  [obj for obj in model["elements"] if obj["tag"] in set(slab["geometry"].get("column_reserves",[])) | {v["owner"] for v in slab["geometry"].get("finish_voids",[])}]]
    key = json.dumps(dependency, sort_keys=True, separators=(",", ":"))
    if key not in _SLAB_GEOMETRIES:
        _SLAB_GEOMETRIES[key] = _slab_geometry_uncached(model, slab)
        if len(_SLAB_GEOMETRIES) > 24:
            _SLAB_GEOMETRIES.pop(next(iter(_SLAB_GEOMETRIES)))
    return deepcopy(_SLAB_GEOMETRIES[key])


def _slab_geometry_uncached(model: dict, slab: dict) -> dict:
    g = slab["geometry"]
    level = _z(model, g["level"])
    outline = _expand(model, g["outline"])
    resolved_voids = _slab_voids(model, slab)
    voids = [_expand(model, v["outline"]) for v in resolved_voids]
    parts, regions, reference_parts = [], [], []
    tags = set()
    for region in g["regions"]:
        tag = region["tag"]
        if tag in tags:
            raise ValueError("Slab region tags must be unique")
        tags.add(tag)
        thickness = _number(region["thickness"]["value"], "Region thickness")
        top = level + _number(region["top_offset"]["value"], "Region top offset")
        if thickness <= 0:
            raise ValueError("Slab region thickness must be positive")
        ring = _expand(model, region["outline"])
        gross = _slab_surface(ring, outline, [])
        candidates = [(polygon,_surface_bounds(polygon)) for polygon in gross]
        for polygon,bounds in candidates:
            for other,other_bounds in reference_parts:
                if _surface_bounds_overlap(bounds,other_bounds) and _area(_analysis._clip(polygon,other)) > 1e-7:
                    raise ValueError("Slab regions overlap in reference surface")
        reference_parts.extend(candidates)
        occupied = _surface_difference(gross,voids)
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
    out["finish_parts"] = _slab_finish_parts(out)
    out.update({"full_parts":deepcopy(out["parts"]),"full_finish_parts":deepcopy(out["finish_parts"]),"shared_parts":[],"shared_finish_parts":[]})
    if g.get("column_reserves"):
        out["parts"],out["shared_parts"],voids = _column_partition(model,slab,out["parts"],g["column_reserves"])
        if out["finish_parts"]:
            out["finish_parts"],out["shared_finish_parts"],_ = _column_partition(model,slab,out["finish_parts"],g["column_reserves"],require_contact=False)
        out["parts"] = [{**p,"top":p["z1"],"thickness":p["z1"]-p["z0"]} for p in out["parts"]]
        out["full_parts"] = _subtract_prisms(out["full_parts"],voids) if voids else out["full_parts"]
    if g.get("finish_voids"):
        out["finish_parts"],shared = _finish_reservations(model,slab,out["finish_parts"])
        out["shared_finish_parts"] += shared
        out["finish_void_tags"] = [v["tag"] for v in g["finish_voids"]]
    return out

def _finish_reservations(model: dict, slab: dict, parts: list) -> tuple:
    """🧱 Assign displaced finish to an explicit native prismatic material owner."""
    if not parts:
        raise ValueError("A finish reserve requires occupied finish material")
    cutters,shared,tags = [],[],set()
    lo,hi = min(p["z0"] for p in parts),max(p["z1"] for p in parts)
    for reserve in slab["geometry"]["finish_voids"]:
        if reserve["tag"] in tags:
            raise ValueError("Finish reserve tags must be unique")
        tags.add(reserve["tag"])
        owner = _find(model,reserve["owner"])
        if owner["tag"]==slab["tag"] or "parts" not in owner["geometry"] or slab["tag"] in owner["geometry"].get("embedded_slabs",[]):
            raise ValueError("A finish reserve needs an independent native part owner")
        native = _solid_geometry(model,owner)["parts"]
        if any(any(key in p for key in ("mesh","section","top_outline")) or p.get("holes") for p in native):
            raise ValueError("Finish ownership requires native vertical prismatic material")
        polygons = _analysis._convex_parts(_expand(model,reserve["outline"]))
        if any(_area(_analysis._clip(a,b["outline"]))>1e-10 for a in polygons for b in cutters):
            raise ValueError("Finish reserves must not overlap")
        expected,common = [],[]
        for part in parts:
            for a in _analysis._convex_parts(part["outline"]):
                for b in polygons:
                    if not _surface_bounds_overlap(_surface_bounds(a),_surface_bounds(b)):
                        continue
                    ring = _analysis._clip(a,b)
                    if len(ring)>=3 and _area(ring)>1e-12:
                        expected.append({**deepcopy(part),"outline":ring})
        for part in expected:
            for owned in native:
                z0,z1 = max(part["z0"],owned["z0"]),min(part["z1"],owned["z1"])
                if z1<=z0+1e-12 or not _surface_bounds_overlap(_surface_bounds(part["outline"]),_surface_bounds(owned["outline"])):
                    continue
                for polygon in _analysis._convex_parts(owned["outline"]):
                    ring = _analysis._clip(part["outline"],polygon)
                    if len(ring)>=3 and _area(ring)>1e-12:
                        common.append({**deepcopy(part),"outline":ring,"z0":z0,"z1":z1,"owner":owner["tag"],"material":owned.get("material",owner["material"]),"role":"finish_joint","joint_host":slab["tag"]})
        target = sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in expected)
        occupied = sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in common)
        if target<=1e-12 or not math.isclose(target,occupied,rel_tol=1e-9,abs_tol=1e-10):
            raise ValueError("Finish reserve must be exactly occupied by its owner")
        cutters += [{"outline":ring,"z0":lo,"z1":hi} for ring in polygons]
        shared += common
    return _subtract_prisms(parts,cutters),shared


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
    m = _m(model)
    return _service(m,_find(m,key))


def _column_partition(model: dict, obj: dict, parts: list, owners: list, require_contact: bool = True) -> tuple:
    """🏗️ Keep full member geometry while assigning occupied common material to columns."""
    analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
    cutters,shared,voids = [],[],[]
    for tag in owners:
        owner = _find(model,tag)
        if owner["class"] != "IfcColumn":
            raise ValueError("A material reserve owner must be a column")
        columns = column_geometry(model,owner)["parts"]
        common = []
        for part in parts:
            for column in columns:
                if not _surface_bounds_overlap(_surface_bounds(part["outline"]),_surface_bounds(column["outline"])):
                    continue
                lo,hi = max(part["z0"],column["z0"]),min(part["z1"],column["z1"])
                if hi <= lo+1e-10:
                    continue
                for a in analysis._convex_parts(part["outline"]):
                    for b in analysis._convex_parts(column["outline"]):
                        ring = analysis._clip(a,b)
                        if len(ring)>=3 and _area(ring)>1e-12:
                            pieces = [{**deepcopy(part),"outline":ring,"z0":lo,"z1":hi,"owner":tag,"material":_material(model,owner)["tag"],"role":"column_joint","joint_host":obj["tag"]}]
                            holes = [{"outline":hole,"z0":lo,"z1":hi} for hole in column.get("holes",())]
                            common += _subtract_prisms(pieces,holes) if holes else pieces
        if not common:
            if require_contact:
                raise ValueError(f"Material reserve owner {tag} has no occupied intersection with {obj['tag']}")
            continue
        shared += common
        cutters += [{**p,"holes":[]} for p in columns]
        voids += [{"outline":hole,"z0":p["z0"],"z1":p["z1"]} for p in columns for hole in p.get("holes",())]
    return _subtract_prisms(parts,cutters),shared,voids


def _solid_geometry(model: dict, obj: dict) -> dict:
    g = obj["geometry"]
    if "section_sweep" in g:
        facade = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️facade.🐍️")
        return {"tag":obj["tag"],"guid":obj["guid"],"class":obj["class"],"storey":obj["storey"],"kind":g["kind"],"name":deepcopy(obj["name"]),"domains":deepcopy(obj["metadata"]["domains"]),"parts":facade.owned_parts(SimpleNamespace(**globals()),model,obj),"shared_parts":[]}
    base = _z(model, g["base_level"])
    parts = []
    for raw in g["parts"]:
        part = deepcopy(raw)
        if "mesh" in part:
            part.update(_polyhedron_part([[v[0],v[1],base+v[2]] for v in part["mesh"]["vertices"]],part["mesh"]["faces"]))
        elif "section" in part:
            section = part["section"]
            profile = section["profile"]
            if section["sweep"][1] <= section["sweep"][0] or _area(profile) <= 1e-12 or not math.isclose(_area(profile),_area(_convex_hull(profile)),rel_tol=1e-10,abs_tol=1e-12):
                raise ValueError("A section sweep needs a convex positive profile and increasing sweep interval")
            section["profile"] = [[h,base+z] for h,z in profile]
            part["z0"],part["z1"] = min(p[1] for p in section["profile"]),max(p[1] for p in section["profile"])
            part["outline"] = part_footprint(part)
        else:
            part["z0"],part["z1"] = _round(base+part["z0"]),_round(base+part["z1"])
        parts.append(part)
    for part in parts:
        if "top_outline" in part:
            part_vertices(part)
    slabs = [_slab_geometry(model,_find(model,tag)) for tag in g.get("embedded_slabs",())]
    shared = [deepcopy(p) for slab in slabs for p in slab["parts"]+slab.get("shared_parts",[]) if p.get("member")==obj["tag"]]
    slab_full = [deepcopy(p) for slab in slabs for p in slab.get("full_parts",slab["parts"]) if p.get("member")==obj["tag"]]
    full = deepcopy(parts)
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
        parts = _subtract_prisms(parts,reserved)
        voids += reserved
    full = (_subtract_prisms(full,voids) if voids else full)+slab_full
    shared += joints
    return {"tag": obj["tag"], "guid": obj["guid"], "class": obj["class"], "storey": obj["storey"],
            "kind": g["kind"], "name": deepcopy(obj["name"]), "domains":deepcopy(obj["metadata"]["domains"]), "parts": parts, "shared_parts": shared, "full_parts": full}


def stair_geometry(key: str, model: dict | None = None) -> dict:
    """🪜 Resolve every native stair material part without substituting a tread envelope."""
    source = _m(model)
    obj = _find(source,key)
    if obj["class"]!="IfcStair":
        raise ValueError("Stair geometry requires a native stair product")
    return _stair_geometry(source,obj)


def _stair_geometry(model: dict, obj: dict) -> dict:
    g = obj["geometry"]
    z0, z1 = _z(model, g["from_level"]), _z(model, g["to_level"])
    def placed(raw, base):
        part = deepcopy(raw)
        if "mesh" in raw:
            part.update(_polyhedron_part([[v[0],v[1],base+v[2]] for v in raw["mesh"]["vertices"]],raw["mesh"]["faces"]))
        else:
            part["outline"] = _expand(model,raw["outline"])
            part["z0"],part["z1"] = _round(base+raw["z0"]),_round(base+raw["z1"])
        return part
    steps = [placed(part,z0) for part in g["step_parts"]]
    landings = [placed(part,z1) for part in g["landing_parts"]]
    structural = [placed(part,z0) for part in g["structural_parts"]]
    return {"tag":obj["tag"],"guid":obj["guid"],"class":obj["class"],"storey":obj["storey"],"kind":g["kind"],
            "centre":deepcopy(g["centre"]),"radius":g["radius"],"inner_radius":g["inner_radius"],"domains":deepcopy(obj["metadata"]["domains"]),
            "base":z0,"top":z1,"name":deepcopy(obj["name"]),"steps":steps,"landings":landings,"structural_parts":structural,"parts":steps+landings+structural}


def _profile_moments(profile: dict) -> dict:
    """📐 Subtract native void area and parallel-axis moments before rotating the section."""
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
    out = {"outline": outline, "area": area, "perimeter": perimeter, "profile": deepcopy(profile),
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


def column_geometry(model: dict, obj: dict, segments: int = 96) -> dict:
    """🏺 Resolve native column sections at their actual placements and elevations."""
    g = obj["geometry"]
    profile = _find(model, obj["type"])["profile"]
    sections = g.get("sections", [{"profile": profile, "base_level": g["base_level"], "top_level": g["top_level"]}])
    parts = [{**profile_geometry(section["profile"], section.get("axis", g["axis"]), segments),
              "z0": _z(model, section["base_level"]), "z1": _top(model, section["top_level"]),
              **({"role":section["role"],"material":(obj["material"] if isinstance(obj.get("material"),dict) else _material(model,obj))["tag"]} if "role" in section else {})} for section in sections]
    x, y = g["axis"]
    out = {"tag": obj["tag"], "guid": obj["guid"], "storey": obj["storey"], "x": x, "y": y,
           "profile": deepcopy(profile), "parts": parts, "outline": deepcopy(parts[0]["outline"]),
           "base": min(p["z0"] for p in parts), "top": max(p["z1"] for p in parts),
           "bounding_radius": max(math.dist([x, y], p) for part in parts for p in part["outline"])}
    if all(p["profile"]["shape"] == "circle" for p in parts):
        out["radius"] = max(p["profile"]["diameter"]["value"] / 2 for p in parts)
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


def building_geometry(model: dict | None = None) -> dict:
    """🗺️ Give drawings shared metric geometry in CMN plan axes, with an explicit north vector."""
    m = _m(model)
    length, width, height = _box(m)
    grid = m["grid"]
    pilotis = [column_geometry(m, e) for e in _by_class(m, "IfcColumn")]
    wall_geometries = [_wall_geometry(m,w) for w in _by_class(m,"IfcWall")]
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
        "details": [_solid_geometry(m, e) for e in _by_class(m, "IfcBuildingElementProxy")],
        "furnishings": [_solid_geometry(m, e) for e in _by_class(m, "IfcFurnishingElement")],
        "beams": [_solid_geometry(m, e) for e in _by_class(m, "IfcBeam")],
        "foundations": [_solid_geometry(m, e) for e in _by_class(m, "IfcFooting")],
        "railings": [_solid_geometry(m, e) for e in _by_class(m, "IfcRailing")],
        "services": [_service(m,e) for e in m["elements"] if e["class"] in ("IfcBoiler","IfcSpaceHeater","IfcWasteTerminal","IfcPipeSegment","IfcCovering","IfcChimney")],
    }


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
        centre = [sum(p[i] for p in outline) / len(outline) for i in (0,1)]
        return _pose_distance(pose, car, centre, part["radius"])
    ring = _car_outline(pose, car)
    if _contains_point(ring, outline[0]) or _contains_point(outline, ring[0]):
        return 0.
    return min(_segment_distance(a,b,c,d) for a,b in zip(ring,ring[1:]+ring[:1]) for c,d in zip(outline,outline[1:]+outline[:1])) - part.get("curve_error",0.)


def _column_pose_distance(pose: list, car: dict, column: dict) -> float:
    return min((_part_pose_distance(pose,car,p) for p in column["parts"] if _vehicle_part(p,car)), default=float("inf"))


def part_band_footprints(part: dict, z0: float, z1: float) -> list:
    """🚙 Project only the occupied portion inside an open vertical collision band."""
    lo,hi = max(z0,part['z0']),min(z1,part['z1'])
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
    return tuple({**{k:v for k,v in record.items() if k!='geometry'},'outline':outline,'z0':max(z0,record['geometry']['z0']),'z1':min(z1,record['geometry']['z1'])} for record in _vehicle_parts(model) for outline in part_band_footprints(record['geometry'],z0,z1))


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
    """🚘 Check an assumed arrival from the site side to a stop before the curved vestibule."""
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
    gx, gy = grid["x"], grid["y"]
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
    """🚘 Retain a bounded route result for one exact JSON model state and sampling request."""
    return _car_path_compute(json.loads(source), samples)


def car_path(model: dict | None = None, samples: int = 120) -> dict:
    """🚘 Derive an isolated route result, reusing work only while all source content matches."""
    if isinstance(samples, bool) or not isinstance(samples, int) or samples < 2:
        raise ValueError("At least two path samples are required")
    source = _source() if model is None else json.dumps(model, sort_keys=True, separators=(",", ":"), allow_nan=False)
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
    """🚧 Compare the physical gap, or an explicit circular-profile scenario, with the required car margin."""
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
    """📍 The closest approaches of the assumed car to a piloti along its path, tightest first: tag, clearance, progress and pose."""
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
    """🔢 List the tags of every occurrence defined by one type."""
    m = _m(model)
    kind = _find(m, type_id)
    if kind["tag"] not in {t["tag"] for t in m["types"]}:
        raise ValueError(f"Not a type: {type_id}")
    return [e["tag"] for e in m["elements"] if e.get("type") == kind["tag"]]


def _opening_area(model: dict, wall: dict) -> float:
    return sum((v["end"] - v["start"]) * (v["head"] - v["sill"]) for v in (_opening_values(model, o) for o in _by_class(model, "IfcOpeningElement") if o["geometry"].get("wall") == wall["tag"]))


def _slab_areas(model: dict, slab: dict) -> dict:
    g = _slab_geometry(model, slab)
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


def quantities(key: str, model: dict | None = None) -> dict:
    """🧮 Derive the quantities of an element, a type total or a space from the authored values."""
    m = _m(model)
    obj = _find(m, key)
    cls = obj["class"]
    if cls == "IfcColumn":
        g = column_geometry(m, obj)
        parts = g["parts"]
        out = {"height": _round(g["top"] - g["base"]),
               "volume": sum(p["area"] * (p["z1"] - p["z0"]) for p in parts),
               "outer_surface_area": sum(p["outer_perimeter"]*(p["z1"]-p["z0"]) for p in parts),
               "bore_surface_area":sum((p["perimeter"]-p["outer_perimeter"])*(p["z1"]-p["z0"]) for p in parts),
               "lateral_surface_area":sum(p["perimeter"]*(p["z1"]-p["z0"]) for p in parts)}
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
                "bounding_volume": _round(gross*g["thickness"]), "layers":deepcopy(g["layers"])}

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
        geometry = _slab_geometry(m, obj)
        if "layers" in geometry:
            out["total_thickness"] = geometry["total_thickness"]
        if obj["geometry"].get("column_reserves"):
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
        out = {"volume":_round(_occupied_volume(g["full_parts"])),"exclusive_volume":_round(_occupied_volume(g["parts"])),"shared_volume":_round(_occupied_volume(g["shared_parts"])),"child_volume":_round(_occupied_volume(g["child_parts"])),"scope":"assembly" if cls=="IfcRamp" else "product","thickness_direction":"vertical",**_material_quantities(m,obj,g["parts"])}
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
    if cls in ("IfcPipeSegment","IfcCovering","IfcChimney","IfcWasteTerminal","IfcBoiler","IfcSpaceHeater"):
        g = _service(m,obj)
        if "parts" in obj["geometry"]:
            return {"part_count":len(g["parts"]),"volume":_round(_occupied_volume(g["parts"])),**_material_quantities(m,obj,g["parts"])}
        return {k:g[k] for k in ("length","diameter","inner_diameter","wall_thickness","volume")} if "tube" in obj["geometry"] else {"volume":sum(_area(p["outline"])*(p["z1"]-p["z0"]) for p in g["parts"])}
    if cls == "IfcStair":
        g = _stair_geometry(m, obj)
        return {"step_count": len(g["steps"]), "rise": g["top"] - g["base"], "volume": _round(_occupied_volume(g["parts"]))}
    if "parts" in obj.get("geometry", {}):
        solid = _solid_geometry(m, obj)
        parts = solid.get("full_parts",solid["parts"] + solid["shared_parts"])
        volume = _partition_volume if obj["class"]=="IfcBeam" else _occupied_volume
        out = {"part_count": len(parts), "volume": _round(volume(parts)), **_material_quantities(m,obj,solid["parts"],partition=obj["class"]=="IfcBeam")}
        if obj["class"] == "IfcBeam":
            out.update({"exclusive_volume": _round(volume(solid["parts"])), "shared_volume": _round(volume(solid["shared_parts"]))})
        return out
    raise ValueError(f"No derived quantities for {key}")


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
    """🧩 Select one discipline model: same schema, different authors, shared GlobalIds."""
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


def solid_geometry(key: str, model: dict | None = None) -> dict:
    """🧊 Resolve one authored multipart element through the shared physical part interface."""
    m = _m(model)
    return _solid_geometry(m,_find(m,key))


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
    """🧱 Preserve exact occupied solid geometry for every native physical domain."""
    if "section" in part or "mesh" in part:
        return {"kind":"polyhedron","vertices":part_vertices(part),"faces":part_faces(part)}
    if "top_outline" in part:
        return {"kind":"loft","bottom":part["outline"],"top":part["top_outline"],"z0":part["z0"],"z1":part["z1"]}
    return {"kind":"prism","polygon":part["outline"],"z0":part["z0"],"z1":part["z1"],"holes":deepcopy(part.get("holes",[]))}


def part_body(part: dict) -> dict:
    """🏗️ Produce a native swept body with an explicit profile placement and world sweep direction."""
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
    """👁️ Project the same occupied prism or loft parts into all physical representations."""
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
        parts = g["full_parts"]
        vertices = [v for p in parts for v in part_vertices(p)]
        cut_y = (min(v[1] for v in vertices)+max(v[1] for v in vertices))/2
        cut_z = (min(v[2] for v in vertices)+max(v[2] for v in vertices))/2
        table = _physical_representations(parts,[],cut_z,cut_y)
        table["axis"] = ("Axis","Curve3D",[{"kind":"polyline","points":[[*f["start"],f["z0"]],[*f["end"],f["z1"]]],"owner":f["tag"],"tag":f["tag"]} for f in g["flights"]])
        for name in ("3d","body"):
            table[name] = (table[name][0],"SolidModel",table[name][2])
        derived = {**quantities(key,m),"cut_z":cut_z,"cut_y":cut_y,"solid":"IfcFacetedBrep","scope":"assembly" if cls=="IfcRamp" else "product"}
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

    elif cls in ("IfcPipeSegment","IfcCovering","IfcChimney","IfcWasteTerminal","IfcBoiler","IfcSpaceHeater"):
        g = _service(m,obj)
        axis = g.get("points") or [[*obj["geometry"]["axis"],min(p["z0"] for p in g["parts"])],[*obj["geometry"]["axis"],max(p["z1"] for p in g["parts"])]]
        cut_z = sum(p[2] for p in axis)/2
        cut_value = sum(p[1] for p in axis)/2
        table = _physical_representations(g["parts"],axis,cut_z,cut_value)
        table["axis"] = ("Axis","Curve3D",[{"kind":"polyline","points":axis}])
        derived = {**quantities(key,m),"cut_z":cut_z,"cut_y":cut_value,"historical_state":m.get("historical_state",{}).get("id")}

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
    elif cls == "IfcStair" or "parts" in obj.get("geometry", {}):
        stair = _stair_geometry(m, obj) if cls == "IfcStair" else None
        solid = None if stair else _solid_geometry(m, obj)
        parts = stair["parts"] if stair else solid["parts"] + solid["shared_parts"]
        derived = quantities(obj["tag"], m)
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
    return [_wall_geometry(model,w) for w in _by_class(model,"IfcWall") if not _wall_uses_path(w)]


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
        _bump(source, [target["tag"]])
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


_exchange = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
_analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
step_excerpt, mesh, ids_check = _exchange.step_excerpt, _exchange.mesh, _exchange.ids_check
translate_structure, tributary = _analysis.translate_structure, _analysis.tributary
clash_check, link_values = _analysis.clash_check, _analysis.link_values
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               