"""📐 Portable solid representations checked against independent GEOS operations."""
import importlib
import json
import math
from pathlib import Path

model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _contract():
    return json.loads(Path(__file__).with_name("📐️representations.json").read_text(encoding="utf-8"))


def _parts(tag):
    return model.represent(tag, "3d")["primitives"]


def _occupied(primitives):
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    return unary_union([Polygon(p["points"]) for p in primitives])


def _vertices(primitive):
    """📍 Read the explicit native three-dimensional representation vertices."""
    if primitive['kind']=='polyhedron':return primitive['vertices']
    if primitive['kind']=='loft':return [[x,y,primitive['z0']] for x,y in primitive['bottom']]+[[x,y,primitive['z1']] for x,y in primitive['top']]
    assert primitive['kind']=='prism'
    return [[x,y,primitive[z]] for z in ('z0','z1') for x,y in primitive['polygon']]


def _convex(primitive):
    """💎 Certify native mesh faces against an independent SciPy convex hull."""
    import numpy as np
    from scipy.spatial import ConvexHull
    vertices=np.asarray(_vertices(primitive),dtype=float);hull=ConvexHull(vertices)
    if primitive['kind']=='polyhedron':
        edges={}
        for face in primitive['faces']:
            for a,b in zip(face,face[1:]+face[:1]):
                key=tuple(sorted((a,b)));edges[key]=edges.get(key,0)+1
        assert edges and set(edges.values())=={2}
        for face in primitive['faces']:
            points=vertices[face];normal=np.cross(points[1]-points[0],points[2]-points[0]);length=np.linalg.norm(normal)
            assert length>1e-12
            distances=(vertices-points[0])@normal/length
            assert np.max(np.abs((points-points[0])@normal/length))<1e-8
            assert distances.min()>-1e-8 or distances.max()<1e-8
    return vertices,hull


def _slice(primitive,axis,value,convex=None):
    """✂️ Intersect actual occupied material with a coordinate plane independently."""
    from shapely.geometry import LineString,MultiPoint,Polygon,box
    from shapely.ops import unary_union
    if primitive['kind']=='prism':
        shape=Polygon(primitive['polygon'],primitive.get('holes',[]))
        if axis==2:return shape if primitive['z0']<=value<=primitive['z1'] else Polygon()
        assert axis==1
        lines=shape.intersection(LineString([(-100,value),(100,value)]))
        return unary_union([box(line.bounds[0],primitive['z0'],line.bounds[2],primitive['z1']) for line in getattr(lines,'geoms',[lines]) if line.geom_type=='LineString' and line.length>1e-10])
    vertices,hull=convex if convex is not None else _convex(primitive);remaining=[i for i in range(3) if i!=axis];points=[]
    for simplex in hull.simplices:
        for a,b in zip(simplex,list(simplex[1:])+[simplex[0]]):
            p,q=vertices[a],vertices[b];d,e=p[axis]-value,q[axis]-value
            if abs(d)<1e-12:points.append(p[remaining])
            if d*e<0:points.append((p+(q-p)*(-d)/(e-d))[remaining])
    shape=MultiPoint(points).convex_hull if points else Polygon()
    return shape if shape.geom_type=='Polygon' else Polygon()


def _projection(primitive):
    """🌄 Project every occupied native part onto the south XZ plane."""
    from shapely.geometry import MultiPoint,box
    if primitive['kind']=='prism':
        return box(min(p[0] for p in primitive['polygon']),primitive['z0'],max(p[0] for p in primitive['polygon']),primitive['z1'])
    vertices,_=_convex(primitive)
    return MultiPoint(vertices[:,[0,2]]).convex_hull


def _volume(primitives):
    """⚖️ Integrate independent GEOS slice unions with SciPy quadrature."""
    from scipy.integrate import quad
    from shapely.ops import unary_union
    compiled=[(p,min(v[2] for v in _vertices(p)),max(v[2] for v in _vertices(p)),None if p['kind']=='prism' else _convex(p)) for p in primitives]
    levels=sorted({v[2] for p in primitives for v in _vertices(p)})
    def area(z):return unary_union([_slice(p,2,z,convex) for p,lo,hi,convex in compiled if lo<z<hi]).area
    estimates=[quad(area,lo,hi,epsabs=1e-10,epsrel=1e-10,limit=200) for lo,hi in zip(levels,levels[1:])]
    assert sum(error for _,error in estimates)<1e-8
    return math.fsum(value for value,_ in estimates)


def test_sections_and_elevations_use_actual_xz_solids():
    """🪚 Compare every projected area with polygon-line intersections or south projection."""
    from shapely.geometry import LineString, Polygon, box
    from shapely.ops import unary_union
    case=json.loads(Path(__file__).with_name('🪜️stair-parts.json').read_text(encoding='utf8'))
    mesh={'kind':'polyhedron',**case['structure']['mesh']}
    for axis,area in enumerate(case['boundary_areas']):
        levels=[v[axis] for v in _vertices(mesh)]
        for level in (min(levels),max(levels)):
            assert math.isclose(_slice(mesh,axis,level).area,area,abs_tol=1e-12)
    for tag in _contract()["objects"]:
        parts = _parts(tag)
        ymin = min(v[1] for p in parts for v in _vertices(p))
        ymax = max(v[1] for p in parts for v in _vertices(p))
        cut = (ymin + ymax) / 2
        expected = [_slice(p,1,cut) for p in parts]
        section = model.represent(tag, "section")
        assert math.isclose(section["derived"]["cut_y"], cut, abs_tol=1e-9)
        assert section["derived"]["projection"] == {"horizontal": "x", "vertical": "z", "view": "south"}
        assert _occupied(section["primitives"]).symmetric_difference(unary_union(expected)).area < 1e-8, tag
        elevation = model.represent(tag, "elevation")
        expected = unary_union([_projection(p) for p in parts])
        actual = _occupied(elevation["primitives"])
        assert actual.symmetric_difference(expected).area < 1e-8, tag
        assert math.isclose(actual.bounds[1], min(v[2] for p in parts for v in _vertices(p)), abs_tol=1e-9)
        assert math.isclose(actual.bounds[3], max(v[2] for p in parts for v in _vertices(p)), abs_tol=1e-9)
        for kind, identifier in (("axis", "Axis"), ("footprint", "FootPrint")):
            representation = model.represent(tag, kind)
            assert (representation["identifier"], representation["type"]) == (identifier, "Curve2D")


def test_sections_preserve_real_cavities():
    """🫙 Require empty basin and planter sections beside occupied fixture material."""
    from shapely.geometry import Point
    source = model.load_model()
    for case in _contract()["section_voids"]:
        level = model._z(source, case["base_level"])
        point = [case["point"][0], level + case["point"][1]]
        material = [case["occupied"][0], level + case["occupied"][1]]
        occupied = _occupied(model.represent(case["tag"], "section")["primitives"])
        assert not occupied.intersects(Point(point).buffer(.02)), case["tag"]
        assert occupied.covers(Point(material)), case["tag"]


def test_stair_quantities_equal_occupied_solid_union_including_landings():
    """🌀 Integrate GEOS horizontal slice unions rather than double-count overlapping prisms."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    case=json.loads(Path(__file__).with_name('🪜️stair-parts.json').read_text(encoding='utf8'))
    assert math.isclose(_volume([{'kind':'polyhedron',**case['structure']['mesh']}]),case['expected']['mesh_volume'],abs_tol=1e-12)
    for tag in _contract()["stairs"]:
        parts = _parts(tag)
        stairs = next(s for s in model.building_geometry()["stairs"] if s["tag"] == tag)
        assert len(parts) == len(stairs['parts']) == len(stairs['steps'])+len(stairs['landings'])+len(stairs['structural_parts'])
        assert stairs['structural_parts'] and any(p['kind']=='polyhedron' for p in parts)
        volume = _volume(parts)
        summed = math.fsum(_volume([p]) for p in parts)
        steps_only = math.fsum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in stairs['steps'])
        assert steps_only < volume <= summed+1e-8
        assert math.isclose(model.quantities(tag)["volume"], volume, abs_tol=1e-8), tag


def test_polygon_union_area_handles_crossings_and_concavity():
    """🔀 Check the internal sweep against portable examples and independent GEOS unions."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    for case in _contract()["union_cases"]:
        expected = unary_union([Polygon(p) for p in case["polygons"]]).area
        assert math.isclose(expected, case["area"], abs_tol=1e-9)
        assert math.isclose(model._union_area(case["polygons"]), expected, abs_tol=1e-9)


def test_polygon_sections_handle_concavity_and_boundary_cuts():
    """✂️ Preserve disconnected intervals and cuts coinciding with polygon edges."""
    from shapely.geometry import LineString, Polygon
    from shapely.ops import unary_union
    for case in _contract()["section_cases"]:
        for cut in case["cuts"]:
            expected = Polygon(case["polygon"]).intersection(LineString([(-100, cut), (100, cut)]))
            actual = unary_union([LineString([(lo, cut), (hi, cut)]) for lo, hi in model._polygon_intervals(case["polygon"], cut)])
            assert expected.symmetric_difference(actual).length < 1e-8
