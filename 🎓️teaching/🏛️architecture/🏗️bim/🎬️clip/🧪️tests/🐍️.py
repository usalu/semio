"""🧪 Portable BIM teaching contracts with independent oracles (uuid, NumPy, SciPy, AJV)."""

import ast
import copy
import importlib.util
import json
import math
import random
import re
import sys
import unittest
import uuid
from collections import Counter
from pathlib import Path

import numpy as np

sys.dont_write_bytecode = True
CLIP = Path(__file__).resolve().parent.parent
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
ROOT = next(parent for parent in CLIP.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
ALPHABET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$"
GLOBAL_ID = re.compile(r"^[0-3][0-9A-Za-z_$]{21}$")
SOURCE_TOKEN = r"(?:VS-\d+(?:\.\d+)*|(?:CSIC2022|SAB2024|Savoye-as-[Bb]uilt|SavoyeAsBuilt2024|FLC)[-A-Za-z0-9.]+|teaching-services-original)"
SOURCE_ID = re.compile(r"^(?:" + SOURCE_TOKEN + r"(?:;\s*" + SOURCE_TOKEN + r")*|https://[^\s]+)$")


def model_api():
    """🧱 Load the semantic model without importing drawing or speech engines."""
    path = CLIP / "🧱️model/🐍️.py"
    spec = importlib.util.spec_from_file_location("architecture_bim_model", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def oracle_encode(number):
    """🔐 Encode 128 bits as two leading bits followed by 21 groups of six bits (shifts, no division)."""
    if not 0 <= number < 1 << 128:
        raise ValueError("A GlobalId encodes 128 bits")
    groups = [(number >> 126) & 3] + [(number >> shift) & 63 for shift in range(120, -1, -6)]
    return "".join(ALPHABET[group] for group in groups)


def oracle_decode(text):
    """🔓 Decode a 22-character IFC GlobalId by folding six-bit groups into one big integer."""
    if not GLOBAL_ID.match(text):
        raise ValueError("Not a GlobalId")
    value = 0
    for char in text:
        value = (value << 6) | ALPHABET.index(char)
    return value


def oracle_global_id(tag):
    """🪪 Derive the GlobalId of a tag with the uuid library and the independent encoder."""
    return oracle_encode(uuid.uuid5(uuid.NAMESPACE_URL, CASES["guid"]["seed"] + tag).int)


def shoelace(points):
    """📐 Measure a simple polygon with the shoelace formula in NumPy."""
    p = np.asarray(points, dtype=float)
    return abs(float(np.dot(p[:, 0], np.roll(p[:, 1], -1)) - np.dot(p[:, 1], np.roll(p[:, 0], -1)))) / 2


def profile_area(profile, outline, holes=()):
    """🕳️ Integrate native analytic profile dimensions and independent GEOS hole polygons."""
    from shapely.geometry import Polygon
    def area(item,ring):
        if item["shape"]=="circle":
            return float(np.pi)*(item["diameter"]["value"]/2)**2
        if item["shape"]=="ellipse":
            return float(np.pi)*item["width"]["value"]*item["depth"]["value"]/4
        if item["shape"]=="rectangle":
            return item["width"]["value"]*item["depth"]["value"]
        return Polygon(ring).area
    gross=area(profile,outline)
    voids=profile.get("voids",[])
    return gross-sum(area(hole,hole.get("outline",[])) for hole in voids) if voids else gross-sum(Polygon(hole).area for hole in holes)


def sampling_candidates(polygon, points):
    """📦 Keep every point inside the inclusive polygon bounds before exact ray casting."""
    p,q=np.asarray(polygon,dtype=float),np.asarray(points,dtype=float).reshape(-1,2)
    low,high=p.min(axis=0),p.max(axis=0)
    return np.flatnonzero((q[:,0]>=low[0])&(q[:,0]<=high[0])&(q[:,1]>=low[1])&(q[:,1]<=high[1]))


def inside(polygon, points):
    """📍 Test points against a polygon by vectorised ray casting."""
    p, q = np.asarray(polygon, dtype=float), np.asarray(points, dtype=float).reshape(-1, 2)
    indices=sampling_candidates(p,q)
    output=np.zeros(len(q),dtype=bool)
    q=q[indices]
    result = np.zeros(len(q), dtype=bool)
    for (x0, y0), (x1, y1) in zip(p, np.roll(p, -1, axis=0)):
        crosses = (y0 > q[:, 1]) != (y1 > q[:, 1])
        with np.errstate(divide="ignore", invalid="ignore"):
            cut = x0 + (q[:, 1] - y0) * (x1 - x0) / (y1 - y0)
        result ^= crosses & (q[:, 0] < cut)
    output[indices]=result
    return output


def sample_surface(outer, holes, step):
    """🧮 Sample the cell centres of a polygon with holes on a regular NumPy grid."""
    p = np.asarray(outer, dtype=float)
    xs = np.arange(p[:, 0].min() + step / 2, p[:, 0].max(), step)
    ys = np.arange(p[:, 1].min() + step / 2, p[:, 1].max(), step)
    gx, gy = np.meshgrid(xs, ys)
    points = np.column_stack([gx.ravel(), gy.ravel()])
    mask = inside(outer, points)
    for hole in holes:
        mask &= ~inside(hole, points)
    return points[mask]


def rectangle_circle_gap(pose, length, width, centres, radius):
    """🚘 Measure the free distance between an oriented car rectangle and circular pilotis."""
    heading = math.radians(pose[2])
    c, s = math.cos(heading), math.sin(heading)
    offset = np.asarray(centres, dtype=float) - np.asarray(pose[:2], dtype=float)
    along, across = offset[:, 0] * c + offset[:, 1] * s, -offset[:, 0] * s + offset[:, 1] * c
    return np.hypot(np.maximum(np.abs(along) - length / 2, 0), np.maximum(np.abs(across) - width / 2, 0)) - radius


def registry(model):
    """🗂️ Index every tagged object of the model by tag."""
    items = {}
    for group in ("elements", "types", "spaces", "storeys", "materials", "relations"):
        for item in model[group]:
            items[item["tag"]] = item
    for name in ("project", "site", "building"):
        items[model[name]["tag"]] = model[name]
    return items


def walk(value, path=()):
    """🚶 Visit every node of a JSON document with its path."""
    yield path, value
    if isinstance(value, dict):
        for key, item in value.items():
            yield from walk(item, path + (key,))
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from walk(item, path + (index,))


class StepError(ValueError):
    """🧾 Report a violation of the ISO 10303-21 syntax subset."""


STEP_VALUE = re.compile(r"\s*(?:#(?P<ref>\d+)|'(?P<str>(?:[^']|'')*)'|\.(?P<enum>[A-Z0-9_]+)\.|(?P<typed>[A-Z][A-Z0-9_]*)\(|(?P<null>\$)|(?P<star>\*)|(?P<num>[-+]?\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)|(?P<open>\())")
STEP_ENTITY = re.compile(r"#(\d+)\s*=\s*([A-Z][A-Z0-9_]*)\s*\(", re.S)


def step_records(lines):
    """✂️ Split a STEP text into records at semicolons outside strings."""
    text = "\n".join(lines)
    records, buffer, quoted, index = [], [], False, 0
    while index < len(text):
        char = text[index]
        if char == "'":
            if quoted and text[index + 1:index + 2] == "'":
                buffer.append("''")
                index += 2
                continue
            quoted = not quoted
        if char == ";" and not quoted:
            records.append("".join(buffer).strip())
            buffer = []
        else:
            buffer.append(char)
        index += 1
    if quoted or "".join(buffer).strip():
        raise StepError("unterminated string or record")
    return records


def step_arguments(text, position):
    """🔣 Parse the argument list of one STEP entity into nested Python values."""
    state = {"position": position}

    def sequence():
        items = []
        while True:
            at = state["position"]
            while at < len(text) and text[at].isspace():
                at += 1
            if text[at:at + 1] == ")":
                state["position"] = at + 1
                return items
            items.append(value())
            at = state["position"]
            while at < len(text) and text[at].isspace():
                at += 1
            if text[at:at + 1] == ",":
                state["position"] = at + 1
            elif text[at:at + 1] != ")":
                raise StepError(f"expected comma or closing parenthesis at {at}")

    def value():
        match = STEP_VALUE.match(text, state["position"])
        if not match:
            raise StepError(f"bad value at {state['position']}: {text[state['position']:state['position'] + 20]!r}")
        state["position"] = match.end()
        kind = match.lastgroup
        if kind == "ref":
            return ("ref", int(match.group("ref")))
        if kind == "str":
            return match.group("str")
        if kind == "enum":
            return ("enum", match.group("enum"))
        if kind == "typed":
            return ("typed", match.group("typed"), sequence())
        if kind == "null":
            return None
        if kind == "star":
            return "*"
        if kind == "num":
            number = match.group("num")
            return float(number) if any(char in number for char in ".eE") else int(number)
        return sequence()

    items = sequence()
    if text[state["position"]:].strip():
        raise StepError("trailing characters after the entity")
    return items


def step_decode(text):
    """🔡 Decode the STEP string escapes for apostrophes and extended characters."""
    text = re.sub(r"\\X2\\([0-9A-F]+)\\X0\\", lambda match: bytes.fromhex(match.group(1)).decode("utf-16-be"), text)
    return text.replace("''", "'").replace("\\\\", "\\")


def step_parse(lines):
    """📄 Parse and validate a STEP fragment: header, DATA section, unique and defined entity numbers."""
    records = step_records(lines)
    if records[:2] != ["ISO-10303-21", "HEADER"] or records[-1] != "END-ISO-10303-21" or records[-2] != "ENDSEC":
        raise StepError("missing ISO-10303-21 envelope")
    header_end = records.index("ENDSEC")
    header = records[2:header_end]
    if records[header_end + 1] != "DATA":
        raise StepError("missing DATA section")
    entities = {}
    for record in records[header_end + 2:-2]:
        match = STEP_ENTITY.match(record)
        if not match:
            raise StepError(f"not an entity: {record[:40]}")
        number = int(match.group(1))
        if number in entities:
            raise StepError(f"entity #{number} defined twice")
        arguments = step_arguments(record, match.end())
        entities[number] = (match.group(2), arguments)
    for number, (name, arguments) in entities.items():
        for _, item in walk(arguments):
            if isinstance(item, tuple) and item[0] == "ref" and item[1] not in entities:
                raise StepError(f"#{number} {name} references undefined #{item[1]}")
    return {"header": header, "entities": entities}


def step_refs(value):
    """🔗 Collect the entity numbers referenced anywhere inside a parsed value."""
    return [item[1] for _, item in walk(value) if isinstance(item, tuple) and item[0] == "ref"]


def mesh_report(vertices, faces):
    """🔺 Measure a triangle mesh with NumPy: edges, orientation, Euler characteristic, volume and area."""
    v, f = np.asarray(vertices, dtype=float), np.asarray(faces, dtype=int)
    directed = np.concatenate([f[:, [0, 1]], f[:, [1, 2]], f[:, [2, 0]]])
    undirected, counts = np.unique(np.sort(directed, axis=1), axis=0, return_counts=True)
    unique_directed = np.unique(directed, axis=0)
    a, b, c = v[f[:, 0]], v[f[:, 1]], v[f[:, 2]]
    return {"edges": len(undirected), "edge_use": set(counts.tolist()), "oriented": len(unique_directed) == len(directed),
            "euler": len(v) - len(undirected) + len(f), "volume": float(np.einsum("ij,ij->i", a, np.cross(b, c)).sum() / 6),
            "area": float(np.linalg.norm(np.cross(b - a, c - a), axis=1).sum() / 2), "used": len(np.unique(f))}


class ModelCase(unittest.TestCase):
    """🏛️ Share one loaded model and prove that no derivation mutates it."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.model = cls.api.load_model()
        cls.snapshot = copy.deepcopy(cls.model)
        cls.items = registry(cls.model)
        cls.geometry = cls.api.building_geometry(cls.model)

    @classmethod
    def tearDownClass(cls):
        if cls.model != cls.snapshot:
            raise AssertionError("a model function mutated its input")

    def event(self, name, **overrides):
        spec = {**CASES["events"][name], **overrides}
        payload = {key: value for key, value in spec.items() if key != "object_tag"}
        payload.setdefault("object_guid", self.api.global_id(spec["object_tag"]))
        return payload

    def piloti_radius(self):
        return self.items[CASES["tags"]["piloti_type"]]["profile"]["diameter"]["value"] / 2

    def wall(self, tag):
        return next(wall for wall in self.geometry["walls"] if wall["tag"] == tag)

    def slab(self, tag):
        return next(slab for slab in self.geometry["slabs"] if slab["tag"] == tag)

    def entry_obstruction(self, model):
        """🚘 Measure the facing aboveground outlines, excluding square stems below vehicle occupancy."""
        from shapely.geometry import Polygon
        geometry=self.api.building_geometry(model)
        columns={p["tag"]:p for p in geometry["pilotis"]}
        ground=geometry["storeys"]["ground"]
        top=ground+model["car"]["height"]["value"]
        widths=[]
        for index,tag in enumerate(self.api.car_path(model)["entry_pair"]):
            column=columns[tag]
            outlines=[Polygon(part["outline"]) for part in column["parts"] if part["z0"]<top and part["z1"]>ground]
            widths.append(max(shape.bounds[2]-column["x"] if index==0 else column["x"]-shape.bounds[0] for shape in outlines))
        return sum(widths)

    def assert_partition(self, polygons, expected, tolerance=1e-6):
        """🧩 Prove coverage using summed area, containment and indexed pair intersections without a fragment union."""
        from shapely.strtree import STRtree
        self.assertTrue(expected.is_valid)
        self.assertTrue(all(p.is_valid and p.area>1e-12 for p in polygons))
        total=sum(p.area for p in polygons)
        outside=sum(p.difference(expected).area for p in polygons if not expected.covers(p))
        overlap=0.0
        tree=STRtree(polygons)
        for index,polygon in enumerate(polygons):
            for other in tree.query(polygon):
                if int(other)>index:
                    overlap+=polygon.intersection(polygons[int(other)]).area
        self.assertAlmostEqual(total,expected.area,places=6)
        self.assertLess(abs(total-expected.area)+overlap+2*outside,tolerance)

    def slab_domain(self, slab):
        """📐 Reconstruct primary slab occupancy from its source boundary and native void rings."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        return Polygon(slab["outline"]).difference(unary_union([Polygon(ring) for ring in slab["voids"]]))

    def physical_supports(self, model, slab):
        """🏛️ Independently intersect actual profile prisms with occupied regional soffits."""
        from shapely.geometry import Polygon
        geometry=self.api.building_geometry(model)
        supports=[]
        for pil in geometry["pilotis"]:
            if any(part["z0"] < region["z0"]+1e-8 and part["z1"] >= region["z0"]-1e-8 and Polygon(part["outline"]).intersection(Polygon(region["outline"])).area > 1e-9
                   for part in pil["parts"] for region in slab["full_parts"]):
                supports.append(pil)
        return supports

    def load_patches(self, model, level):
        """⚖️ Independently authored local load patches use GEOS geometry and explicit material densities."""
        from shapely.geometry import Polygon
        items=registry(model)
        slab=next(s for s in self.api.building_geometry(model)["slabs"] if s["tag"]==self.api.role({"first":"floor","roof":"roof"}[level],model))
        comp=items[slab["tag"]]["geometry"]["composite"]
        t,b,p=(comp[k]["value"] for k in ("compression_layer","rib_width","rib_pitch"))
        rc,clay=(items[comp[k]]["density"]["value"] for k in ("concrete","ceramic"))
        if level=="roof":
            layers=items["LS-ROOF"]["layers"]
            nominal=next(l["thickness"]["value"] for l in layers if l["id"]=="structural")
            finish=next(l for l in layers if l["id"]=="paving")
            density=items[finish["material"]]["density"]["value"]
            solarium=Polygon(slab["zones"]["solarium"])
        patches=[]
        for part in slab["full_parts"]:
            shape=Polygon(part["outline"])
            h=part["thickness"]
            c=t+b/p*(h-t)
            own=rc*c+clay*(h-c) if part["material"]=="MAT-HOLLOW-SLAB" else items[part["material"]]["density"]["value"]*h
            zones=[("slab",shape)] if level=="first" else [("solarium",shape.intersection(solarium)),("roof",shape.difference(solarium))]
            for zone,polygon in zones:
                if polygon.area<1e-12:
                    continue
                finish_weight=model["loads"]["finishes"]["first"]["value"] if level=="first" else density*(finish["thickness"]["value"]-(h-nominal))
                imposed=model["loads"]["imposed"]["first" if level=="first" else zone]["value"]
                patches.append({"shape":polygon,"area":polygon.area,"q":own+finish_weight+imposed,"zone":zone,"depth":h,"self_weight":own,"finish":finish_weight,"imposed":imposed,"region":part["region"]})
        return patches

    def roof_zones(self, model):
        """⚖️ Independently average source material patches within each actual roof zone."""
        roof=next(s for s in self.api.building_geometry(model)["slabs"] if s["tag"]==self.api.role("roof",model))
        patches=self.load_patches(model,"roof")
        return roof,{zone:sum(p["area"]*p["q"] for p in patches if p["zone"]==zone)/sum(p["area"] for p in patches if p["zone"]==zone) for zone in ("roof","solarium")}


    def roof_load(self, model):
        """⚖️ Weight the independently integrated zones by actual occupied area."""
        from shapely.geometry import Polygon
        roof,loads=self.roof_zones(model)
        occupied=self.slab_domain(roof)
        solarium=occupied.intersection(Polygon(roof["zones"]["solarium"])).area
        return (loads["solarium"]*solarium+loads["roof"]*(occupied.area-solarium))/occupied.area


    @staticmethod
    def points_in_patch(shape,points):
        """🔬 Exact GEOS interior membership after a conservative NumPy bounds filter."""
        from shapely import contains_xy
        xmin,ymin,xmax,ymax=shape.bounds
        candidates=np.flatnonzero((points[:,0]>=xmin)&(points[:,0]<=xmax)&(points[:,1]>=ymin)&(points[:,1]<=ymax))
        inside=np.zeros(len(points),dtype=bool)
        inside[candidates]=contains_xy(shape,points[candidates,0],points[candidates,1])
        return inside

    def roof_reactions(self, model, step=0.025):
        """📐 SciPy nearest-support sampling integrates independent material/zone patch weights."""
        from scipy.spatial import cKDTree
        roof,_=self.roof_zones(model)
        points=sample_surface(roof["outline"],roof["voids"],step)
        supports=self.physical_supports(model,roof)
        nearest=cKDTree(np.array([[p["x"],p["y"]] for p in supports])).query(points)[1]
        q=np.zeros(len(points))
        for patch in self.load_patches(model,"roof"):
            q[self.points_in_patch(patch["shape"],points)]=patch["q"]
        return {pil["tag"]:float(q[nearest==i].sum()*step*step) for i,pil in enumerate(supports)}


    def floor_load(self, model):
        """🧱 Composite fields and solid RC beam strips use their own physical local weights."""
        patches=self.load_patches(model,"first")
        return sum(p["area"]*p["q"] for p in patches)/sum(p["area"] for p in patches)


class BimGlobalIdContract(ModelCase):
    """🪪 GlobalIds are the 22-character IFC encoding of 128-bit UUIDs, derived from tags."""

    def test_portable_vectors_match_independent_encoder_and_uuid_library(self):
        self.assertEqual(CASES["guid"]["namespace"], str(uuid.NAMESPACE_URL))
        self.assertEqual(CASES["guid"]["alphabet"], ALPHABET)
        for vector in CASES["guid"]["vectors"]:
            with self.subTest(vector=vector["uuid"]):
                number = uuid.UUID(vector["uuid"]).int
                self.assertEqual(oracle_encode(number), vector["globalId"])
                self.assertEqual(self.api.compress_guid(number), vector["globalId"])
                self.assertEqual(self.api.decompress_guid(vector["globalId"]), number)
                self.assertEqual(str(uuid.UUID(int=self.api.decompress_guid(vector["globalId"]))), vector["uuid"])

    def test_random_uuids_roundtrip_against_the_oracle(self):
        generator = random.Random(CASES["guid"]["random_seed"])
        for _ in range(CASES["guid"]["random_count"]):
            number = generator.getrandbits(128)
            text = self.api.compress_guid(number)
            self.assertEqual(text, oracle_encode(number))
            self.assertRegex(text, GLOBAL_ID)
            self.assertEqual(self.api.decompress_guid(text), number)
            self.assertEqual(oracle_decode(text), number)
            self.assertEqual(uuid.UUID(int=number), uuid.UUID(int=oracle_decode(text)))

    def test_invalid_inputs_are_rejected(self):
        for text in CASES["guid"]["invalid_text"]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                self.api.decompress_guid(text)
        for text in CASES["guid"]["invalid_int"]:
            with self.subTest(number=text), self.assertRaises(ValueError):
                self.api.compress_guid(int(text))
        for value in (True, 1.5, "1", None):
            with self.subTest(value=value), self.assertRaises(ValueError):
                self.api.compress_guid(value)

    def test_every_stored_guid_is_derived_from_its_tag_and_unique(self):
        seen = {}
        for tag, item in self.items.items():
            with self.subTest(tag=tag):
                self.assertEqual(item["guid"], oracle_global_id(tag))
                self.assertEqual(item["guid"], self.api.global_id(tag))
                self.assertRegex(item["guid"], GLOBAL_ID)
                self.assertNotIn(item["guid"], seen)
                seen[item["guid"]] = tag
        self.assertEqual(len(seen), len(self.items))

    def test_lookup_accepts_tag_and_guid_and_rejects_unknown_keys(self):
        for tag in CASES["tags"]["lookup"]:
            by_tag, by_guid = self.api.element(tag, self.model), self.api.element(self.items[tag]["guid"], self.model)
            self.assertEqual(by_tag, by_guid)
        for key in ("MISSING", "", "0" * 22):
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.api.element(key, self.model)


class BimPartitionOracleContract(unittest.TestCase):
    """🧮 Bounded independent partition proofs retain coverage, disjointness and containment."""

    def test_partition_proof_matches_geos_for_coverage_overlap_and_leaks(self):
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        fixture=CASES["partition_oracle"]
        expected=Polygon(fixture["domain"])
        for case in fixture["cases"]:
            with self.subTest(case=case["id"]):
                polygons=[Polygon(ring) for ring in case["pieces"]]
                union=unary_union(polygons)
                independent=union.symmetric_difference(expected).area<1e-9 and abs(sum(p.area for p in polygons)-union.area)<1e-9
                self.assertEqual(independent,case["valid"])
                if case["valid"]:
                    ModelCase.assert_partition(self,polygons,expected)
                else:
                    with self.assertRaises(AssertionError):
                        ModelCase.assert_partition(self,polygons,expected)


    def test_patch_sampling_preserves_holes_and_boundaries(self):
        from shapely.geometry import shape
        from shapely import contains_xy
        fixture=CASES["patch_sampling"]
        points=np.asarray(fixture["points"],dtype=float)
        for patch in fixture["patches"]:
            polygon=shape(patch["shape"])
            expected=np.zeros(len(points),dtype=bool)
            expected[patch["inside"]]=True
            np.testing.assert_array_equal(contains_xy(polygon,points[:,0],points[:,1]),expected)
            np.testing.assert_array_equal(ModelCase.points_in_patch(polygon,points),expected)


    def test_slab_load_and_support_oracles_use_full_semantic_member_scope(self):
        from types import SimpleNamespace
        from shapely.geometry import Polygon
        import numpy as np
        case=CASES['slab_scope']
        def part(outline):
            return {'outline':outline,'z0':6.34,'z1':6.51,'thickness':.17,'material':'MAT-HOLLOW-SLAB','region':'R'}
        slab={'tag':'SL-R','outline':case['outline'],'voids':[],'zones':{'solarium':case['outline']},'parts':[part(ring) for ring in case['owned']],'full_parts':[part(case['outline'])]}
        column={'tag':'C','x':.5,'y':.5,'parts':[{'outline':case['column'],'z0':3.14,'z1':9.03}]}
        geometry={'slabs':[slab],'pilotis':[column]}
        model={key:[] for key in ('elements','types','spaces','storeys','materials','relations')}
        model.update({key:{'tag':key} for key in ('project','site','building')})
        quantity=lambda value:{'value':value}
        model['elements']=[{'tag':'SL-R','geometry':{'composite':{'compression_layer':quantity(.055),'rib_width':quantity(.08),'rib_pitch':quantity(.5),'concrete':'RC','ceramic':'CLAY'}}}]
        model['materials']=[{'tag':'RC','density':quantity(25)},{'tag':'CLAY','density':quantity(14)},{'tag':'PAVING','density':quantity(22)},{'tag':'LS-ROOF','layers':[{'id':'structural','thickness':quantity(.17)},{'id':'paving','thickness':quantity(.09),'material':'PAVING'}]}]
        model['loads']={'finishes':{'first':quantity(1)},'imposed':{'first':quantity(2),'roof':quantity(1),'solarium':quantity(1.5)}}
        helper=ModelCase()
        helper.api=SimpleNamespace(building_geometry=lambda model:geometry,role=lambda key,model:'SL-R')
        self.assertAlmostEqual(sum(Polygon(p['outline']).area for p in slab['parts']),case['owned_area'])
        self.assertEqual([p['tag'] for p in helper.physical_supports(model,slab)],['C'])
        patches=helper.load_patches(model,'roof')
        self.assertAlmostEqual(sum(p['area'] for p in patches),case['area'])
        concrete=.055+.08/.5*(.17-.055)
        independent=float(np.dot([concrete,.17-concrete,.09,1],[25,14,22,1.5]))
        self.assertAlmostEqual(independent,case['load'])
        self.assertAlmostEqual(sum(p['area']*p['q'] for p in patches),case['load'])
        slab['zones']['solarium']=[[0,0],[.7,0],[.7,1],[0,1]]
        self.assertAlmostEqual(helper.roof_load(model),independent-.15)

    def test_dense_sampling_candidates_preserve_ray_boundary_semantics(self):
        from shapely.geometry import Polygon,Point
        case=CASES['sampling_candidates']
        points=np.asarray(case['points'],dtype=float)
        np.testing.assert_array_equal(sampling_candidates(case['polygon'],points),case['indices'])
        np.testing.assert_array_equal(inside(case['polygon'],points),case['inside'])
        polygon=Polygon(case['polygon'])
        for point,expected in zip(points,case['inside']):
            if polygon.boundary.distance(Point(point))>1e-12:
                self.assertEqual(polygon.contains(Point(point)),expected)

class BimModelStructureContract(ModelCase):
    """🧬 The authored JSON is bilingual, sourced or assumed, and referentially complete."""

    def test_top_level_identity_and_languages(self):
        self.assertEqual(self.model["languages"], CASES["languages"])
        self.assertEqual(self.model["schema_id"], CASES["model"]["schema_id"])
        self.assertGreaterEqual(self.model["revision"], 1)
        self.assertEqual({key: self.model[key]["tag"] for key in ("project", "site", "building")}, CASES["model"]["spatial"])
        self.assertEqual(self.model["building"]["class"], "IfcBuilding")
        self.assertEqual(self.model["site"]["class"], "IfcSite")
        self.assertEqual(self.model["project"]["class"], "IfcProject")

    def test_every_localized_text_has_both_languages(self):
        found = 0
        for path, value in walk(self.model):
            if isinstance(value, dict) and ({"en", "de"} & set(value)):
                found += 1
                with self.subTest(path=path):
                    self.assertTrue({"en", "de"} <= set(value))
                    for language in CASES["languages"]:
                        self.assertIsInstance(value[language], str)
                        self.assertTrue(value[language].strip())
        self.assertGreater(found, 100)

    def test_every_quantity_is_sourced_or_assumed_with_a_unit(self):
        units, found = set(CASES["model"]["units"]), 0
        for path, value in walk(self.model):
            if path[:1] != ("demo_events",) and isinstance(value, dict) and "value" in value and ("unit" in value or "ifc_type" in value):
                found += 1
                with self.subTest(path=path):
                    self.assertIn(value["basis"], ("sourced", "assumed", "inferred"))
                    if "unit" in value:
                        self.assertIn(value["unit"], units)
                    if value["basis"] == "sourced":
                        self.assertRegex(value["source"], SOURCE_ID)
                    if isinstance(value["value"], (int, float)) and not isinstance(value["value"], bool):
                        self.assertTrue(math.isfinite(value["value"]))
        self.assertGreater(found, 80)

    def test_sourced_facts_match_the_research(self):
        sourced = CASES["sourced"]
        box = self.model["building"]["box"]
        for name in ("length", "width", "height"):
            self.assertEqual(box[name]["value"], sourced["box"][name])
            self.assertEqual(box[name]["basis"], sourced["box_basis"][name])
        grid = self.model["grid"]
        self.assertEqual((grid["spacing"]["value"], grid["spacing"]["basis"]), (sourced["spacing"], "sourced"))
        self.assertEqual((grid["initial_spacing"]["value"], grid["initial_spacing"]["basis"]), (sourced["initial_spacing"], "sourced"))
        years = self.model["building"]["years"]
        for name, year in sourced["years"].items():
            self.assertEqual(years[name], year)
        self.assertEqual(self.model["building"]["architects"], sourced["architects"])
        self.assertEqual(self.model["site"]["name"]["en"], sourced["site"])
        for tag in ("SP-TERRACE", "SP-SOLARIUM"):
            self.assertNotIn("reference_area", self.items[tag])
            self.assertAlmostEqual(self.api.quantities(tag, self.model)["area"], shoelace(self.items[tag]["outline"]), places=6)

    def test_measured_geometry_and_assumed_vehicle_dimensions_are_plausible(self):
        diameter = self.items[CASES["tags"]["piloti_type"]]["profile"]["diameter"]
        self.assertEqual(diameter["basis"], "sourced")
        self.assertTrue(0.2 <= diameter["value"] <= 0.6)
        levels = [storey["elevation"]["value"] for storey in self.model["storeys"]]
        self.assertAlmostEqual(levels[0], -2.42)
        self.assertEqual(levels, sorted(levels))
        self.assertEqual(levels[1], 0)
        self.assertAlmostEqual(levels[2], 3.17)
        self.assertLess(levels[-1], self.model["building"]["box"]["height"]["value"])
        for tag in CASES["tags"]["slabs"]:
            slab = self.slab(tag)
            minimum=0.005 if tag in CASES["tags"]["covers"] else 0.05
            self.assertTrue(minimum <= slab["thickness"] <= 0.5)
        self.assertTrue(1.0 <= self.model["car"]["width"]["value"] <= 2.2)
        self.assertTrue(3.0 <= self.model["car"]["length"]["value"] <= 6.0)

    def test_every_element_is_complete_and_references_exist(self):
        required = {"guid", "tag", "class", "name", "storey", "geometry", "attributes", "psets", "metadata"}
        domains = set(CASES["model"]["domains"])
        for element in self.model["elements"]:
            with self.subTest(tag=element["tag"]):
                self.assertTrue(required <= set(element))
                attributes = {name for level in self.api.class_chain(element["class"],self.model) for name in level["adds"]}
                self.assertEqual("predefined_type" in element,"PredefinedType" in attributes)
                self.assertIn(element["class"], self.model["classes"])
                self.assertIn(element["storey"], {storey["tag"] for storey in self.model["storeys"]})
                if "type" in element:
                    self.assertEqual(self.items[element["type"]]["class"], element["class"] + "Type")
                if "material" in element:
                    self.assertIn(self.items[element["material"]]["class"], ("IfcMaterial", "IfcMaterialLayerSet"))
                self.assertTrue(element["metadata"]["author"].strip())
                self.assertGreaterEqual(element["metadata"]["revision"], 1)
                self.assertTrue(element["metadata"]["domains"])
                self.assertTrue(set(element["metadata"]["domains"]) <= domains)
        tags = [item["tag"] for group in ("elements", "types", "spaces", "storeys", "materials", "relations") for item in self.model[group]]
        self.assertEqual(len(tags), len(set(tags)))

    def test_every_opening_has_a_native_host_and_optional_filling(self):
        openings = {e["tag"]: e for e in self.model["elements"] if e["class"] == "IfcOpeningElement"}
        fillings = [e for e in self.model["elements"] if "opening" in e["geometry"]]
        hosts = {e["tag"]:e["class"] for e in self.model["elements"] if e["class"] in {"IfcWall","IfcSlab","IfcCurtainWall"}}
        self.assertEqual(set(openings), set(CASES["tags"]["openings"]))
        for opening in openings.values():
            choices = [(field,cls) for field,cls in (("wall","IfcWall"),("slab","IfcSlab"),("curtain_wall","IfcCurtainWall")) if field in opening["geometry"]]
            self.assertEqual(len(choices),1)
            field,cls = choices[0]
            self.assertEqual(hosts[opening["geometry"][field]],cls)
        self.assertEqual(sorted(e["geometry"]["opening"] for e in fillings), sorted(set(openings) - set(CASES["tags"]["unfilled_openings"])))
        self.assertEqual(dict(Counter(e["class"] for e in self.model["elements"])), CASES["model"]["class_counts"])
        self.assertEqual({e["tag"] for e in self.model["elements"] if e["class"]=="IfcFooting"},set(CASES["tags"]["foundations"]))

    def test_service_bore_labels_match_all_explicit_model_memberships(self):
        """🕳️ Named native service bores retain their physical host and agreed discipline labels."""
        case=CASES['service_bore_domains']
        expected=set(case['models'])
        explicit=[];labels=[]
        for tag in case['tags']:
            with self.subTest(tag=tag):
                opening=self.items[tag]
                self.assertEqual(opening['class'],'IfcOpeningElement')
                self.assertIn('slab',opening['geometry'])
                members={model['id'] for model in self.model['models'] if tag in model['element_tags']}
                self.assertEqual(members,expected)
                self.assertEqual(set(opening['metadata']['domains']),expected)
                explicit.append([name in members for name in CASES['models']['ids']])
                labels.append([name in opening['metadata']['domains'] for name in CASES['models']['ids']])
        np.testing.assert_array_equal(np.asarray(labels,dtype=bool),np.asarray(explicit,dtype=bool))

    def test_class_registry_reaches_ifc_root_for_every_used_class(self):
        classes = self.model["classes"]
        used = {item["class"] for group in ("elements","types") for item in self.model[group]} | {"IfcSpace", "IfcBuildingStorey", "IfcProject", "IfcSite", "IfcBuilding"}
        for name in sorted(used):
            with self.subTest(name=name):
                chain, current = [], name
                while current is not None:
                    chain.append(current)
                    current = classes[current]["supertype"]
                self.assertEqual(chain[-1], "IfcRoot")
                self.assertEqual(len(chain), len(set(chain)))

    def test_demo_events_are_valid_design_events(self):
        base = self.api.load_model()
        for name, payload in self.model["demo_events"].items():
            with self.subTest(event=name):
                updated = self.api.apply_event(payload, base)
                self.assertEqual(updated["revision"], base["revision"] + 1)
                self.assertEqual(self.api.demo_event(name, base), payload)
        with self.assertRaises(ValueError):
            self.api.demo_event("unknown", base)

    def test_element_returns_independent_resolved_copies(self):
        column = self.api.element(self.api.role("column_example", self.model), self.model)
        self.assertEqual(column["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"], True)
        self.assertEqual(column["material"]["tag"], "MAT-RC")
        self.assertEqual(column["guid"], oracle_global_id(self.api.role("column_example", self.model)))
        column["psets"].clear()
        column["name"]["en"] = "changed"
        self.assertEqual(self.api.element(self.api.role("column_example", self.model), self.model)["name"]["en"], self.items[self.api.role("column_example", self.model)]["name"]["en"])
        self.assertTrue(self.api.element("SL-R", self.model)["material"]["layers"])


class BimGridContract(ModelCase):
    """🏛️ The numbered support register preserves its measured profiles and axes."""

    def test_grid_lines_are_equally_spaced_and_labelled(self):
        grid, expected = self.model["grid"], CASES["grid"]
        self.assertEqual(grid["x"], expected["x"])
        self.assertEqual(grid["y"], expected["y"])
        self.assertEqual("".join(grid["labels_x"]), expected["columns"])
        self.assertEqual("".join(grid["labels_y"]), expected["rows"])
        spacing = grid["spacing"]["value"]
        self.assertTrue(np.allclose(np.diff(grid["y"]), spacing))
        self.assertTrue(np.allclose(np.diff(grid["x"]), spacing))
        self.assertGreater(grid["initial_spacing"]["value"], spacing)

    def test_authored_columns_preserve_grid_and_off_grid_axes(self):
        """📍 Registered sections preserve measured axes and their real footprints."""
        from shapely.geometry import Polygon
        self.assertEqual([p["tag"] for p in self.geometry["pilotis"]], CASES["grid"]["column_tags"])
        self.assertEqual({str(i) for i in range(1, 34)} - set(self.items), set())
        for pil in self.geometry["pilotis"]:
            authored = self.items[pil["tag"]]
            np.testing.assert_allclose([pil["x"], pil["y"]], authored["geometry"]["axis"], atol=1e-9)
            self.assertEqual(pil["guid"], oracle_global_id(pil["tag"]))
            for part in pil["parts"]:
                profile = part["profile"]
                expected=profile_area(profile,part["outline"],part.get("holes",[]))
                self.assertAlmostEqual(part["area"], expected, delta=2e-5)
                self.assertGreater(part["z1"], part["z0"])


    def test_columns_connect_their_storey_to_their_authored_slab_soffit(self):
        """🏛️ Explicit numbered sections retain physical vertical extents, including cellar lengths."""
        for pil in self.geometry["pilotis"]:
            self.assertGreater(pil["top"], pil["base"])
            self.assertAlmostEqual(pil["base"], min(part["z0"] for part in pil["parts"]))
            self.assertAlmostEqual(pil["top"], max(part["z1"] for part in pil["parts"]))
        self.assertAlmostEqual(next(p for p in self.geometry["pilotis"] if p["tag"] == "19")["top"], 2.91)
        self.assertLess(next(p for p in self.geometry["pilotis"] if p["tag"] == "14")["base"], 0)


    def test_each_column_type_has_its_explicit_occurrences_in_source_order(self):
        for type_tag, expected in CASES["grid"]["type_occurrences"].items():
            tags = self.api.type_occurrences(type_tag, self.model)
            self.assertEqual(tags, expected)
            self.assertTrue(all(self.items[tag]["type"] == type_tag for tag in tags))
        for key in (self.api.role("column_example", self.model), "MISSING"):
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.api.type_occurrences(key, self.model)


class BimGeometryContract(ModelCase):
    """🗺️ Drawings preserve the actual source geometry, datums and occupied regions."""

    def test_box_and_storeys_come_from_the_authored_values(self):
        box = self.geometry["box"]
        self.assertEqual((box["length"], box["width"], box["height"]), tuple(CASES["sourced"]["box"][k] for k in ("length", "width", "height")))
        self.assertEqual(self.geometry["storeys"], {s["id"]: s["elevation"]["value"] for s in self.model["storeys"]})

    def test_primary_slabs_and_canopy_match_their_authored_footprints(self):
        from shapely.geometry import Polygon

        self.assertEqual({slab["tag"] for slab in self.geometry["slabs"]}, set(CASES["tags"]["slabs"]))
        for tag in CASES["tags"]["slabs"]:
            slab = self.slab(tag)
            with self.subTest(tag=tag):
                authored = self.items[tag]["geometry"]
                np.testing.assert_allclose(slab["outline"], authored["outline"], atol=1e-9)
                self.assertGreater(slab["thickness"], 0)
                outline = Polygon(slab["outline"])
                self.assertTrue(outline.is_valid)
                for void in slab["voids"]:
                    aperture = Polygon(void)
                    self.assertTrue(aperture.is_valid)
                    self.assertGreater(outline.intersection(aperture).area,0)
                    self.assertGreater(aperture.area, 0)
                from shapely.ops import unary_union
                gross=unary_union([Polygon(region["outline"]) for region in authored.get("regions",[{"outline":authored["outline"]}])])
                net = gross.difference(unary_union([Polygon(void) for void in slab["voids"]]))
                self.assertAlmostEqual(net.area, self.api.quantities(tag, self.model)["area"], places=6)
        self.assertFalse(self.slab("SL-KIOSK")["voids"])
        self.assertLess(shoelace(self.slab("SL-KIOSK")["outline"]), shoelace(self.slab("SL-R")["outline"]))

    def test_slab_levels_match_storeys(self):
        """🥞 Finished datums remain distinct from regional structural top levels."""
        floor, roof = self.slab(self.api.role("floor", self.model)), self.slab(self.api.role("roof", self.model))
        self.assertEqual(floor["kind"], "FLOOR")
        self.assertEqual(roof["kind"], "ROOF")
        self.assertAlmostEqual(self.geometry["storeys"]["first"], 3.17)
        self.assertAlmostEqual(self.geometry["storeys"]["roof"], 6.60)
        self.assertEqual({round(region["thickness"], 2) for region in floor["regions"]}, {.17, .20, .23})
        self.assertEqual({round(region["thickness"], 2) for region in roof["regions"]}, {.17, .23})
        self.assertEqual({round(part["z0"], 2) for part in floor["parts"]}, {2.91})
        self.assertEqual({round(part["z0"], 2) for part in roof["parts"]}, {6.34})
        self.assertEqual([layer["id"] for layer in roof["layers"]], ["structural", "paving"])


    def test_walls_and_openings_follow_the_authored_storeys(self):
        box = self.geometry["box"]
        for wall in self.geometry["walls"]:
            start, end = np.asarray(wall["start"]), np.asarray(wall["end"])
            self.assertAlmostEqual(float(np.linalg.norm(end - start)), wall["length"])
            for point in (start, end):
                self.assertTrue(np.isfinite(point).all())
            np.testing.assert_allclose(start, self.items[wall["tag"]]["geometry"]["path"][0], atol=1e-9)
            np.testing.assert_allclose(end, self.items[wall["tag"]]["geometry"]["path"][-1], atol=1e-9)
            self.assertGreater(wall["thickness"], 0)
            self.assertGreater(wall["top"], wall["base"])
            for opening in wall["openings"]:
                self.assertTrue(0 <= opening["start"] < opening["end"] <= wall["length"])
                self.assertTrue(0 <= opening["sill"] < opening["head"] <= wall["top"] - wall["base"])
                self.assertEqual(opening["guid"], oracle_global_id(opening["tag"]))
                if opening["tag"] in CASES["tags"]["unfilled_openings"]:
                    self.assertIsNone(opening["fill_tag"])
                    self.assertIsNone(opening["fill_guid"])
                    self.assertIsNone(opening["fill_class"])
                else:
                    self.assertIn(opening["fill_class"], {"IfcWindow", "IfcDoor"})
                    self.assertEqual(opening["fill_guid"], oracle_global_id(opening["fill_tag"]))
                    self.assertNotEqual(opening["guid"], opening["fill_guid"])

    def test_curtain_wall_paths_preserve_registered_stations_and_native_portals(self):
        from scipy.interpolate import BPoly
        from shapely.geometry import LineString
        wall=self.geometry["curtain_wall"]
        spec=self.items["CW-1"]["geometry"]
        self.assertEqual(len(wall["paths"]),len(spec["glazing_paths"]))
        for drawn,authored in zip(wall["paths"],spec["glazing_paths"]):
            reference=np.asarray(drawn["reference_path"])
            literals=[point for point in authored["path"] if isinstance(point,list)]
            curves=[point for point in authored["path"] if isinstance(point,dict)]
            self.assertEqual(drawn["tag"],authored["tag"])
            self.assertEqual(drawn["stations"],authored["stations"])
            for point in reference:
                matched=any(np.allclose(point,p,atol=1e-6) for p in literals)
                for curve in curves:
                    if "arc" in curve:
                        arc=curve["arc"]
                        matched|=abs(np.linalg.norm(point-arc["centre"])-arc["radius"])<1e-5
                    if "bezier" in curve:
                        controls=np.asarray(curve["bezier"],dtype=float)
                        expected=BPoly(controls[:,None,:],[0,1])(np.linspace(0,1,33))
                        matched|=bool(np.any(np.all(np.isclose(expected,point,atol=1e-6),axis=1)))
                self.assertTrue(matched,point)
            line=LineString(reference)
            self.assertAlmostEqual(drawn["length"],line.length,places=7)
            stations=[s["position"]["value"] for s in authored["stations"]]
            self.assertTrue(all(b>a for a,b in zip(stations,stations[1:])))
            self.assertAlmostEqual(stations[0],0)
            self.assertAlmostEqual(stations[-1],line.length,places=7)
            np.testing.assert_allclose(drawn["path"],[line.interpolate(s).coords[0] for s in stations],atol=1e-8)
        hosts={e["tag"] for e in self.model["elements"] if e["class"]=="IfcOpeningElement" and e["geometry"].get("curtain_wall")==wall["tag"]}
        self.assertEqual(set(wall["opening_tags"]),hosts)
        self.assertEqual(hosts,{"OP-D-ENTRY","OP-D-SERVICE"})
        self.assertEqual(wall["base"],0)
        self.assertAlmostEqual(wall["top"],min(p["z0"] for p in self.slab("SL-1")["parts"]))

    def test_ramp_flights_climb_one_storey_at_a_constant_gentle_slope(self):
        storeys = self.geometry["storeys"]
        void_low = {tag: np.asarray(v).min(axis=0) for tag, v in self.geometry["ramp"]["voids"].items()}
        void_high = {tag: np.asarray(v).max(axis=0) for tag, v in self.geometry["ramp"]["voids"].items()}
        spans = {"RM-1": ("ground", "first"), "RM-2": ("first", "roof")}
        self.assertEqual([r["tag"] for r in self.geometry["ramp"]["ramps"]], CASES["tags"]["ramps"])
        for ramp in self.geometry["ramp"]["ramps"]:
            low, high = spans[ramp["tag"]]
            flights = ramp["flights"]
            with self.subTest(ramp=ramp["tag"]):
                self.assertEqual(len(flights), 2)
                for before, after in zip(flights, flights[1:]):
                    self.assertAlmostEqual(before["z1"], after["z0"])
                self.assertAlmostEqual(flights[0]["z0"], storeys[low])
                self.assertAlmostEqual(flights[-1]["z1"], storeys[high])
                self.assertAlmostEqual(flights[0]["z1"], flights[1]["z0"])
                for flight in flights:
                    run = float(np.linalg.norm(np.asarray(flight["end"]) - np.asarray(flight["start"])))
                    slope = (flight["z1"] - flight["z0"]) / run
                    self.assertAlmostEqual(run, flight["run"])
                    self.assertAlmostEqual(slope, flight["slope"])
                    self.assertTrue(0.05 <= slope <= 0.2)
                    self.assertGreater(flight["width"], 0)
                    for point in (flight["start"], flight["end"]):
                        self.assertTrue(any((np.asarray(point) >= void_low[t] - 1e-9).all() and (np.asarray(point) <= void_high[t] + 1e-9).all() for t in void_low))

    def test_screens_boiler_radiator_drains_and_downpipes_are_placed_consistently(self):
        """🏛️ Native roof datums and original service identities retain their actual source placements."""
        box=self.geometry["box"]
        screen=next(item for item in self.geometry["screens"] if item["tag"]=="SC-1")
        self.assertEqual(screen["base"],self.geometry["storeys"]["roof"])
        source_cap=self.items["SC-1"]["geometry"]
        crest=max(p[2] for part in screen["parts"] for p in self.api.part_vertices(part))
        self.assertAlmostEqual(crest,float(source_cap["top_level"])+source_cap["cap"]["rise"]["value"])
        self.assertLessEqual(crest,box["height"])
        services={item["tag"]:item for item in self.geometry["services"]}
        self.assertTrue(set(CASES["native_services"]["added"])<=set(services))
        self.assertFalse(set(CASES["native_services"]["removed"])&set(self.items))
        for case in CASES["native_services"]["rainwater"]:
            drain,pipe=services[case["drain"]],services[case["pipe"]]
            self.assertEqual(drain["points"][0][:2],pipe["points"][0][:2])
            self.assertEqual(pipe["points"][0][:2],self.items[case["column"]]["geometry"]["axis"])
            self.assertEqual(pipe["points"][0][:2],pipe["points"][1][:2])
            self.assertAlmostEqual(pipe["points"][0][2],self.geometry["storeys"]["roof"])
            self.assertAlmostEqual(pipe["points"][1][2],CASES["native_services"]["outlet_z"])
        self.assertEqual(services["BL-1"]["points"][0][2],self.geometry["storeys"]["cellar"])
        for service in services.values():
            self.assertTrue(service["parts"])
            for part in service["parts"]:
                vertices=self.api.part_vertices(part)
                self.assertTrue(vertices)
                for point in vertices:
                    self.assertTrue(all(math.isfinite(value) for value in point))
                    self.assertLessEqual(point[2],box["height"])

    def test_spaces_report_their_polygon_areas(self):
        self.assertEqual([s["tag"] for s in self.geometry["spaces"]], CASES["tags"]["spaces"])
        for space in self.geometry["spaces"]:
            with self.subTest(tag=space["tag"]):
                self.assertAlmostEqual(space["area"], shoelace(space["outline"]), places=6)
                self.assertAlmostEqual(space["area"], self.api.quantities(space["tag"], self.model)["area"], places=6)
                self.assertGreater(space["area"], 1)
                self.assertIn(space["storey"], self.geometry["storeys"])

    def test_geometry_of_a_modified_model_changes_with_the_model(self):
        updated = self.api.apply_event(self.event("sill"), self.model)
        before, after = self.wall("W-S")["openings"][0], next(w for w in self.api.building_geometry(updated)["walls"] if w["tag"] == "W-S")["openings"][0]
        self.assertAlmostEqual(after["sill"], CASES["events"]["sill"]["sill"])
        self.assertNotEqual(before["sill"], after["sill"])
        self.assertEqual(before["guid"], after["guid"])


class BimRouteCacheContract(ModelCase):
    """🚘 Route memoisation respects content changes, bounded storage and result ownership."""

    def test_equivalent_inputs_reuse_computation_and_mutations_are_isolated(self):
        """🧪 Portable cache operations exercise the wrapper independently of route geometry."""
        from unittest.mock import patch
        self.assertTrue(hasattr(self.api, "_car_path_compute"))
        spec=CASES["route_cache"]
        self.api._car_path_cached.cache_clear()
        def compute(model,samples):
            return {"poses":[[0.0,0.0,0.0]],"width":model["car"]["width"]["value"],"samples":samples}
        with patch.object(self.api,"_car_path_compute",side_effect=compute) as spy:
            twin=copy.deepcopy(self.model)
            first=self.api.car_path(twin,samples=spec["samples"][0])
            twin["car"]=dict(reversed(list(twin["car"].items())))
            second=self.api.car_path(twin,samples=spec["samples"][0])
            self.assertEqual(spy.call_count,1)
            self.assertEqual(first,second)
            first["poses"][0][0]=99
            self.assertEqual(self.api.car_path(twin,samples=spec["samples"][0])["poses"][0][0],0)
            twin["car"]["width"]["value"]+=spec["width_delta"]
            changed=self.api.car_path(twin,samples=spec["samples"][0])
            self.assertEqual(spy.call_count,2)
            self.assertAlmostEqual(changed["width"],second["width"]+spec["width_delta"])
            self.api.car_path(twin,samples=spec["samples"][1])
            self.assertEqual(spy.call_count,3)
            self.api.car_path()
            self.api.car_path()
            self.assertEqual(spy.call_count,4)
            for samples in range(2,3+spec["max_entries"]):
                self.api.car_path(twin,samples=samples)
            self.assertEqual(self.api._car_path_cached.cache_info().currsize,spec["max_entries"])
        self.api._car_path_cached.cache_clear()


class BimClearanceLimits(ModelCase):
    """🚧 The straight-through threshold and the closest approaches of the assumed car, checked against bisection and NumPy."""

    def test_limits_match_a_bisection_of_the_clearance_function(self):
        """🚧 Independently bisect the straight-passage inequality."""
        pair=self.api.car_path(self.model)["entry_pair"]
        obstruction=self.entry_obstruction(self.model)
        width,margin=(self.model["car"][key]["value"] for key in ("width","min_side_margin"))
        low,high=0.0,10.0
        for _ in range(40):
            mid=(low+high)/2
            low,high=(low,mid) if (mid-obstruction-width)/2>=margin else (mid,high)
        limits=self.api.clearance_limits(self.model)
        self.assertAlmostEqual(limits["min_spacing"],high,places=6)
        self.assertAlmostEqual(limits["clear_width"],limits["spacing"]-obstruction,places=9)


    def test_limits_follow_diameter_and_car_width_overrides(self):
        margin = self.model["car"]["min_side_margin"]["value"]
        wide = self.api.clearance_limits(self.model, diameter=0.6, car_width=2.0)
        self.assertAlmostEqual(wide["min_spacing"], 2.0 + 2 * margin + 0.6, places=9)
        self.assertTrue(wide["fits"])
        self.assertFalse(self.api.clearance_limits(self.model, spacing=wide["min_spacing"] - 0.01, diameter=0.6, car_width=2.0)["fits"])

    def test_pinches_equal_independent_profile_distances(self):
        """🚘 Independent GEOS distances use actual contours and vehicle-height occupancy."""
        from shapely.geometry import Polygon
        car=self.model["car"]
        half=np.array([car["length"]["value"],car["width"]["value"]])/2
        corners=np.array([[-half[0],-half[1]],[half[0],-half[1]],[half[0],half[1]],[-half[0],half[1]]])
        bodies=[]
        for x,y,heading in self.api.car_path(self.model,samples=1201)["poses"]:
            c,s=math.cos(math.radians(heading)),math.sin(math.radians(heading))
            bodies.append(Polygon(corners@np.array([[c,s],[-s,c]])+[x,y]))
        expected=[]
        for pil in self.geometry["pilotis"]:
            sections=[Polygon(part["outline"],part.get("holes",[])) for part in pil["parts"] if part["z0"]<car["height"]["value"] and part["z1"]>0]
            if sections:
                expected.append((min(body.distance(section) for section in sections for body in bodies),pil["tag"]))
        expected.sort()
        found=self.api.path_pinches(self.model,count=2)
        threshold=expected[1][0]
        expected_by_tag={tag:value for value,tag in expected}
        self.assertTrue({item["tag"] for item in found} <= {tag for value,tag in expected if value <= threshold+2e-4})
        self.assertTrue({tag for value,tag in expected if value < threshold-2e-4} <= {item["tag"] for item in found})
        for item in found:
            self.assertAlmostEqual(item["clearance"],expected_by_tag[item["tag"]],delta=2e-4)


class BimInformationContract(ModelCase):
    """📊 Raw values become information: unit, basis, object identity, language and meaning."""

    def test_information_requires_an_explicit_known_language(self):
        for args in ((self.api.role("column_example", self.model), "diameter", "fr"), (self.api.role("column_example", self.model), "diameter", ""), (self.api.role("column_example", self.model), "unknown-property", "en"), ("MISSING", "diameter", "en")):
            with self.subTest(args=args), self.assertRaises(ValueError):
                self.api.information(*args, model=self.model)
        with self.assertRaises(TypeError):
            self.api.information(self.api.role("column_example", self.model), "diameter")

    def test_portable_information_cases(self):
        for case in CASES["information"]:
            for language in CASES["languages"]:
                with self.subTest(case=case["property"], language=language):
                    info = self.api.information(case["tag"], case["property"], language, model=self.model)
                    self.assertEqual(info["tag"], case["tag"])
                    self.assertEqual(info["guid"], oracle_global_id(case["tag"]))
                    self.assertEqual(info["class"], self.items[case["tag"]]["class"])
                    self.assertEqual(info["unit"], case["unit"])
                    self.assertEqual(info["basis"], case["basis"])
                    self.assertTrue(info["label"].strip() and info["meaning"].strip())
                    if "value" in case:
                        self.assertEqual(info["value"], case["value"])
            self.assertNotEqual(self.api.information(case["tag"], case["property"], "en", model=self.model)["meaning"],
                                self.api.information(case["tag"], case["property"], "de", model=self.model)["meaning"])

    def test_information_values_agree_with_the_authored_model(self):
        spacing = self.api.information(self.api.role("column_example", self.model), "grid.spacing", "en", model=self.model)
        self.assertEqual(spacing["value"], self.model["grid"]["spacing"]["value"])
        diameter = self.api.information(self.api.role("column_example", self.model), "diameter", "de", model=self.model)
        self.assertEqual(diameter["value"], self.piloti_radius() * 2)
        volume = self.api.information(self.api.role("column_example", self.model), "volume", "en", model=self.model)
        self.assertAlmostEqual(volume["value"], math.pi * self.piloti_radius() ** 2 * self.api.quantities(self.api.role("column_example", self.model), self.model)["height"], places=6)
        self.assertEqual(volume["unit"], "m³")
        self.assertEqual(self.api.information(self.api.role("column_example", self.model), "GlobalId", "en", model=self.model)["value"], oracle_global_id(self.api.role("column_example", self.model)))
        self.assertEqual(self.api.information(self.api.role("column_example", self.model), "material", "en", model=self.model)["value"], self.items["MAT-RC"]["name"]["en"])
        self.assertEqual(self.api.information(self.api.role("column_example", self.model), "material", "de", model=self.model)["value"], self.items["MAT-RC"]["name"]["de"])

    def test_quantity_reads_authored_derived_and_layer_values(self):
        slab = max(region["thickness"]["value"] for region in self.items["SL-1"]["geometry"]["regions"])
        self.assertEqual(self.api.quantity("SL-1", "thickness", self.model), slab)
        self.assertEqual(self.api.quantity(self.items["SL-1"], "thickness", self.model), slab)
        self.assertEqual(self.api.quantity("SL-R", "paving", self.model), next(l for l in self.items["LS-ROOF"]["layers"] if l["id"] == "paving")["thickness"]["value"])
        self.assertAlmostEqual(self.api.quantity(self.api.role("column_example", self.model), "volume", self.model), self.api.quantities(self.api.role("column_example", self.model), self.model)["volume"])
        with self.assertRaises(ValueError):
            self.api.quantity(self.api.role("column_example", self.model), "unknown", self.model)


    def test_sill_event_preserves_head_and_source_centimetres(self):
        """🪟 Source datums and a sill-only event retain the full nineteen-centimetre gain."""
        c=CASES["window_datums"]
        base=self.api.quantities(c["opening"],self.model)
        event=self.api.demo_event("sill",self.model)
        changed=self.api.quantities(c["opening"],self.api.apply_event(event,self.model))
        for key in ("sill","head","height"):
            self.assertAlmostEqual(base[key],c[key])
        self.assertAlmostEqual(event["sill"],c["changed_sill"])
        self.assertAlmostEqual(event["head"],c["head"])
        self.assertAlmostEqual(changed["head"],base["head"])
        self.assertAlmostEqual(changed["height"],c["changed_height"])
        self.assertAlmostEqual(changed["height"]-base["height"],c["height_gain"])


class BimClearanceContract(ModelCase):
    """🚗 The car question: clear width, margin, curved wall and swept path."""

    def test_clear_width_and_margin_follow_the_spacing_formula(self):
        """↔️ Unequal actual entry profiles contribute their facing half-widths."""
        result,car=self.api.clearance(self.model),self.model["car"]
        pair=self.api.car_path(self.model)["entry_pair"]
        obstruction=self.entry_obstruction(self.model)
        spacing=result["spacing"]
        self.assertAlmostEqual(result["obstruction_width"],obstruction)
        self.assertAlmostEqual(result["clear_width"],spacing-obstruction)
        self.assertAlmostEqual(result["clear_width_initial"],self.model["grid"]["initial_spacing"]["value"]-obstruction)
        self.assertAlmostEqual(result["margin"],(spacing-obstruction-car["width"]["value"])/2)
        self.assertEqual(result["car_width"],car["width"]["value"])
        self.assertEqual(result["min_margin"],car["min_side_margin"]["value"])


    def test_fit_decision_flips_exactly_at_the_required_margin(self):
        car, diameter = self.model["car"], self.api.clearance(self.model)["obstruction_width"]
        threshold = diameter + car["width"]["value"] + 2 * car["min_side_margin"]["value"]
        for spacing, expected in ((threshold + 0.01, True), (threshold, True), (threshold - 0.01, False), (diameter + car["width"]["value"] - 0.1, False)):
            with self.subTest(spacing=spacing):
                result = self.api.clearance(self.model, spacing=spacing)
                self.assertEqual(result["fits"], expected)
                self.assertAlmostEqual(result["clear_width"], spacing - diameter)
                self.assertEqual(result["grid_spacing"], self.model["grid"]["spacing"]["value"])

    def test_initial_spacing_gave_more_room_than_the_built_one(self):
        result = self.api.clearance(self.model)
        self.assertGreater(result["margin_initial"], result["margin"])
        self.assertTrue(result["fits_initial"])

    def test_car_path_never_touches_a_piloti_and_enters_between_two(self):
        """🚗 The physical route opening uses actual registered entry profiles."""
        path=self.api.car_path(self.model)
        self.assertEqual(len(path["poses"]),120)
        self.assertGreater(path["min_clearance"],0)
        self.assertEqual(self.api.clearance(self.model)["min_path_clearance"],path["min_clearance"])
        first,second=path["entry_pair"]
        self.assertIn(first,CASES["grid"]["ground_tags"])
        self.assertIn(second,CASES["grid"]["ground_tags"])
        a,b=(next(p for p in self.geometry["pilotis"] if p["tag"]==tag) for tag in (first,second))
        self.assertAlmostEqual(abs(path["gap"]["end"][0]-path["gap"]["start"][0]),abs(b["x"]-a["x"])-self.entry_obstruction(self.model))
        poses=np.asarray(path["poses"])
        self.assertAlmostEqual(path["length"],float(np.linalg.norm(np.diff(poses[:,:2],axis=0),axis=1).sum()),delta=.05*path["length"])


    def test_arrival_turn_follows_the_independent_assumed_traffic_route(self):
        path=self.api.car_path(self.model,samples=1201)
        self.assertNotIn("wall_curve_centre",path)
        self.assertNotIn("centre",self.geometry["curtain_wall"])
        self.assertNotIn("radius",self.geometry["curtain_wall"])
        radius = self.model["car"]["turning_radius"]["value"] - self.model["car"]["width"]["value"] / 2
        self.assertAlmostEqual(path["centre_radius"], radius)
        turning = [pose for pose in path["poses"] if abs(pose[2] - 90) > 1e-4 and abs(pose[2] - 270) > 1e-4]
        self.assertTrue(turning)
        distances = np.linalg.norm(np.asarray(turning)[:, :2] - path["curve_centre"], axis=1)
        np.testing.assert_allclose(distances, radius, atol=1e-5)
        self.assertTrue(path["collision_free"])
        self.assertGreater(path["certified_min_clearance"], 0)


class BimQuantityContract(ModelCase):
    """🧮 Derived quantities are recomputed with independent formulas."""

    def test_piloti_volume_is_pi_r_squared_times_height(self):
        """🧮 Independently integrate the actual circular example section."""
        from scipy import integrate
        tag = self.api.role("column_example", self.model)
        radius = self.piloti_radius()
        pil = next(p for p in self.geometry["pilotis"] if p["tag"] == tag)
        height = pil["top"] - pil["base"]
        area = integrate.quad(lambda rho: 2 * math.pi * rho, 0, radius)[0]
        result = self.api.quantities(tag, self.model)
        self.assertAlmostEqual(result["cross_section_area"], area, places=7)
        self.assertAlmostEqual(result["height"], height)
        self.assertAlmostEqual(result["volume"], area * height, places=6)
        self.assertAlmostEqual(result["outer_surface_area"], 2 * math.pi * radius * height, places=6)


    def test_type_total_is_the_sum_over_all_occurrences(self):
        total = self.api.quantities(CASES["tags"]["piloti_type"], self.model)
        each = self.api.quantities(self.api.role("column_example", self.model), self.model)["volume"]
        self.assertEqual(total["count"], CASES["grid"]["piloti_type_count"])
        self.assertGreater(total["volume"], len([tag for tag in CASES["grid"]["ground_tags"] if self.items[tag]["type"] == CASES["tags"]["piloti_type"]]) * each)
        self.assertAlmostEqual(total["volume"], sum(self.api.quantities(tag, self.model)["volume"] for tag in total["occurrences"]), places=6)
        self.assertEqual(total["occurrences"], self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model))

    def test_each_column_type_volume_matches_independent_profile_integrals(self):
        """🧮 Integrate actual profiles with independent GEOS section areas."""
        from shapely.geometry import Polygon
        for type_tag,tags in CASES["grid"]["type_occurrences"].items():
            expected=0.0
            for tag in tags:
                pil=next(p for p in self.geometry["pilotis"] if p["tag"]==tag)
                volume=0.0
                for part in pil["parts"]:
                    profile=part["profile"]
                    area=profile_area(profile,part["outline"],part.get("holes",[]))
                    volume+=area*(part["z1"]-part["z0"])
                self.assertAlmostEqual(self.api.quantities(tag,self.model)["volume"],volume,delta=1e-4)
                expected+=volume
            result=self.api.quantities(type_tag,self.model)
            self.assertEqual(result["count"],len(tags))
            self.assertAlmostEqual(result["volume"],expected,delta=1e-4)


    def test_wall_gross_opening_and_net_areas_and_glazing_share(self):
        """📊 Independent GEOS unions validate regional solid, envelope and member-net air volumes."""
        from shapely.geometry import Polygon,box
        from shapely.ops import unary_union
        owners=[p for group in ("pilotis","beams","fillings") for obj in self.geometry[group] for p in obj["parts"]+obj.get("shared_parts",[])]
        def volume(parts,cutters=()):
            if not parts:
                return 0.0
            low,high=min(p["z0"] for p in parts),max(p["z1"] for p in parts)
            shapes=[(p,Polygon(p["outline"])) for p in parts]
            xmin=min(s.bounds[0] for _,s in shapes);ymin=min(s.bounds[1] for _,s in shapes)
            xmax=max(s.bounds[2] for _,s in shapes);ymax=max(s.bounds[3] for _,s in shapes)
            cutters=[p for p in cutters if p["z0"]<high and p["z1"]>low and min(v[0] for v in p["outline"])<xmax and max(v[0] for v in p["outline"])>xmin and min(v[1] for v in p["outline"])<ymax and max(v[1] for v in p["outline"])>ymin]
            cut_shapes=[(p,Polygon(p["outline"])) for p in cutters]
            levels=sorted({low,high,*(max(low,min(high,p[k])) for p in parts+cutters for k in ("z0","z1"))})
            total=0.0
            for lo,hi in zip(levels,levels[1:]):
                mid=(lo+hi)/2
                occupied=unary_union([shape for p,shape in shapes if p["z0"]<mid<p["z1"]])
                removed=unary_union([shape for p,shape in cut_shapes if p["z0"]<mid<p["z1"]])
                total+=(hi-lo)*occupied.difference(removed).area
            return total
        for tag in CASES["tags"]["walls"]:
            wall,result=self.wall(tag),self.api.quantities(tag,self.model)
            with self.subTest(tag=tag):
                height=wall["top"]-wall["base"]
                gross=wall["length"]*height
                apertures=[box(o["start"],o["sill_z"],o["end"],o["head_z"]) for o in wall["openings"]]
                opening=unary_union(apertures).intersection(box(0,wall["base"],wall["length"],wall["top"])).area
                self.assertAlmostEqual(result["gross_side_area"],gross)
                self.assertAlmostEqual(result["opening_area"],opening)
                self.assertAlmostEqual(result["net_side_area"],gross-opening)
                glazing=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"])/f["panel_thickness"] for f in self.geometry["fillings"] if f["host"]==tag for p in f["parts"] if p["role"]=="glass")
                self.assertAlmostEqual(result["glazing_share"],glazing/gross)
                expected_volume=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in wall["parts"])
                self.assertAlmostEqual(result["volume"],expected_volume,places=6)
                if tag in ("W-S","W-E","W-N","W-W","W-G-E"):
                    self.assertAlmostEqual(expected_volume,volume(wall["parts"]),places=6)
                source=self.items[tag]
                material=self.api.element(tag,self.model)["material"]
                bands=[]
                def station(spec):
                    datum=spec["datum"]
                    value=wall[datum] if datum in ("base","top") else next(o for o in wall["openings"] if o["tag"]==spec["opening"])[datum+"_z"]
                    return value+spec["offset"]["value"]
                if "layer_regions" in source["geometry"]:
                    bands=[(station(r["from"]),station(r["to"]),self.items[r["material"]]["layers"]) for r in source["geometry"]["layer_regions"]]
                else:
                    bands=[(wall["base"],wall["top"],material.get("layers",[{"thickness":{"value":wall["thickness"]}}]))]
                expected_gross=expected_envelope=0.0
                air=[]
                direction=(np.asarray(wall["end"])-np.asarray(wall["start"]))/wall["length"]
                normal=np.array([-direction[1],direction[0]])
                side=source["geometry"].get("exterior_side",1)
                def ring(start,end,lo,hi):
                    return [np.asarray(wall["start"])+direction*s+normal*t for s,t in ((start,lo),(end,lo),(end,hi),(start,hi))]
                for z0,z1,layers in bands:
                    offset=wall["thickness"]/2
                    for layer in layers:
                        t=layer["thickness"]["value"]
                        lo,hi=sorted([side*(offset-t),side*offset])
                        expected_envelope+=wall["length"]*(z1-z0)*t
                        if layer.get("is_void"):
                            air.append({"outline":ring(0,wall["length"],lo,hi),"z0":z0,"z1":z1})
                        else:
                            expected_gross+=wall["length"]*(z1-z0)*t
                        offset-=t
                portals=[{"outline":ring(o["start"],o["end"],-wall["thickness"],wall["thickness"]),"z0":o["sill_z"],"z1":o["head_z"]} for o in wall["openings"]]
                self.assertAlmostEqual(result["gross_volume"],expected_gross,places=6)
                self.assertAlmostEqual(result["envelope_volume"],expected_envelope,places=6)
                self.assertAlmostEqual(result["cavity_volume"],volume(air,owners+portals),places=6)
                self.assertAlmostEqual(result["bounding_volume"],gross*wall["thickness"],places=6)
                self.assertTrue(0<=result["glazing_share"]<=1)

    def test_window_and_opening_quantities_agree(self):
        """🚪 Rough host cuts and frame-clear dimensions remain explicitly separate."""
        for filling_tag in CASES["tags"]["windows"]+CASES["tags"]["doors"]:
            opening_tag=self.items[filling_tag]["geometry"]["opening"]
            aperture,filling=self.api.quantities(opening_tag,self.model),self.api.quantities(filling_tag,self.model)
            for key in ("width","height","area"):
                self.assertAlmostEqual(aperture[key],filling[key])
            host=self.items[opening_tag]["geometry"]
            self.assertEqual(aperture["host"],host.get("wall",host.get("slab",host.get("curtain_wall"))))
            self.assertAlmostEqual(aperture["area"],aperture["width"]*aperture["height"])
            kind=self.items[self.items[filling_tag]["type"]]["filling"]
            wing=kind.get("side_wings",{}).get("width",{}).get("value",0)
            expected=filling["rough_width"]-2*filling["frame_width"]-2*wing
            if "panel_layout" in kind:
                member=kind["panel_layout"]["member_width"]["value"]
                intervals=[(expected*cell["u"][0]["value"]+(member/2 if cell["u"][0]["value"]>0 else 0),expected*cell["u"][1]["value"]-(member/2 if cell["u"][1]["value"]<1 else 0)) for cell in kind["panel_layout"]["cells"] if cell["operation"]!="fixed"]
                if intervals:
                    expected=max(hi for _,hi in intervals)-min(lo for lo,_ in intervals)
            self.assertAlmostEqual(filling["clear_width"],expected)
            self.assertLessEqual(filling["clear_height"],filling["rough_height"])
            self.assertGreater(filling["volume"],0)


    def test_slab_areas_and_volumes_use_polygon_areas_minus_voids(self):
        """📐 Independently union openings and integrate occupied regional depths."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        for tag in CASES["tags"]["slabs"]:
            slab,result=self.slab(tag),self.api.quantities(tag,self.model)
            authored=self.items[tag]["geometry"]
            shape=unary_union([Polygon(region["outline"]) for region in authored.get("regions",[{"outline":authored["outline"]}])])
            occupied=shape.difference(unary_union([Polygon(v) for v in slab["voids"]]))
            self.assertAlmostEqual(result["gross_area"],shape.area)
            self.assertAlmostEqual(result["void_area"],shape.area-occupied.area)
            self.assertAlmostEqual(result["area"],occupied.area)
            volume=lambda parts:sum(Polygon(p["outline"],p.get("holes",[])).area*(p["z1"]-p["z0"]) for p in parts)
            self.assertAlmostEqual(result["volume"],volume(slab["full_parts"]),places=5)
            self.assertAlmostEqual(result["exclusive_volume"],volume(slab["parts"]),places=5)
            self.assertAlmostEqual(result["shared_volume"],volume(slab["shared_parts"]),places=5)
            self.assertAlmostEqual(result["volume"],result["exclusive_volume"]+result["shared_volume"],places=6)
            self.assertAlmostEqual(result["finish_volume"],volume(slab["full_finish_parts"]),places=5)
            self.assertAlmostEqual(result["exclusive_finish_volume"],volume(slab["finish_parts"]),places=5)
            self.assertAlmostEqual(result["shared_finish_volume"],volume(slab["shared_finish_parts"]),places=5)
            self.assertAlmostEqual(result["finish_volume"],result["exclusive_finish_volume"]+result["shared_finish_volume"],places=6)
            self.assertAlmostEqual(result["perimeter"],shape.length)


    def test_roof_build_up_total_is_the_sum_of_its_layers(self):
        result, layers = self.api.quantities("SL-R", self.model), self.items["LS-ROOF"]["layers"]
        self.assertAlmostEqual(result["total_thickness"], sum(layer["thickness"]["value"] for layer in layers))
        self.assertAlmostEqual(result["thickness"], max(region["thickness"]["value"] for region in self.items["SL-R"]["geometry"]["regions"]))

    def test_ramp_curtain_wall_pipe_and_type_quantities(self):
        ramp = self.api.quantities("RM-1", self.model)
        flights = self.geometry["ramp"]["ramps"][0]["flights"]
        self.assertEqual(ramp["flight_count"], 2)
        self.assertAlmostEqual(ramp["length"], sum(f["run"] for f in flights))
        self.assertAlmostEqual(ramp["rise"], self.geometry["storeys"]["first"])
        glass = self.api.quantities("CW-1", self.model)
        wall = self.geometry["curtain_wall"]
        self.assertAlmostEqual(glass["height"], wall["top"] - wall["base"])
        self.assertAlmostEqual(glass["area"], glass["length"] * glass["height"])
        from shapely.geometry import LineString
        self.assertAlmostEqual(glass["length"],sum(LineString(record["reference_path"]).length for record in wall["paths"]),places=6)
        pipe = self.api.quantities("DP-1", self.model)
        authored=np.asarray(self.items["DP-1"]["geometry"]["points"],dtype=float)
        self.assertAlmostEqual(pipe["length"],float(np.linalg.norm(np.diff(authored,axis=0),axis=1).sum()))
        from scipy.spatial import ConvexHull
        physical=next(item for item in self.geometry["services"] if item["tag"]=="DP-1")
        independent=sum(ConvexHull(np.asarray(self.api.part_vertices(part))).volume for part in physical["parts"])
        self.assertAlmostEqual(pipe["volume"],independent,places=8)
        self.assertAlmostEqual(self.api.quantities(CASES["tags"]["wall_type"], self.model)["thickness"], self.wall("W-S")["thickness"])
        for key in ("PROJECT", "MISSING"):
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.api.quantities(key, self.model)


class BimClassChainContract(ModelCase):
    """🧬 IFC class chains list the attributes every level adds."""

    def test_portable_chains_from_ifc_root_to_the_class(self):
        for name, expected in CASES["class_chains"].items():
            with self.subTest(name=name):
                chain = self.api.class_chain(name, self.model)
                self.assertEqual([(level["class"], level["adds"]) for level in chain], [(level[0], level[1]) for level in expected])
                self.assertEqual(chain[0]["class"], "IfcRoot")
                self.assertEqual(chain[-1]["class"], name)
                self.assertEqual(len({level["class"] for level in chain}), len(chain))

    def test_column_slab_and_wall_chains_have_as_many_attributes_as_step_lines(self):
        for name in ("IfcColumn", "IfcSlab", "IfcWall"):
            chain = self.api.class_chain(name, self.model)
            self.assertEqual(sum(len(level["adds"]) for level in chain), CASES["step"]["attribute_counts"][name.upper()])

    def test_chain_attributes_are_not_repeated_and_unknown_classes_fail(self):
        for name in CASES["class_chains"]:
            names = [attribute for level in self.api.class_chain(name, self.model) for attribute in level["adds"]]
            self.assertEqual(len(names), len(set(names)))
        for name in ("IfcBanana", "", "ifccolumn"):
            with self.subTest(name=name), self.assertRaises(ValueError):
                self.api.class_chain(name, self.model)

    def test_every_authored_element_class_has_a_chain(self):
        for name in {e["class"] for e in self.model["elements"]}:
            with self.subTest(name=name):
                self.assertEqual(self.api.class_chain(name, self.model)[-1]["class"], name)


class BimRelationContract(ModelCase):
    """🔗 Relationships are explicit objects: aggregation, containment, type, voids, fills, material."""

    def relations(self, key, cls):
        return [r for r in self.api.relations_of(key, self.model) if r["class"] == cls]

    def test_spatial_hierarchy_project_site_building_storeys(self):
        parts = lambda key: [s["tag"] for r in self.relations(key, "IfcRelAggregates") if r["role"] == "relating" for s in r["related"]]
        self.assertEqual(parts("PROJECT"), ["SITE"])
        self.assertEqual(parts("SITE"), ["BUILDING"])
        storeys = parts("BUILDING")
        self.assertEqual(storeys, CASES["tags"]["storeys"])

    def test_every_element_is_contained_in_exactly_its_storey(self):
        for element in self.model["elements"]:
            with self.subTest(tag=element["tag"]):
                contained = [r for r in self.api.relations_of(element["tag"], self.model) if r["class"] == "IfcRelContainedInSpatialStructure"]
                self.assertEqual(len(contained), 1)
                self.assertEqual(contained[0]["relating"]["tag"], element["storey"])
                self.assertEqual(contained[0]["role"], "related")

    def test_all_authored_pilotis_are_defined_by_the_piloti_type(self):
        link = self.relations(CASES["tags"]["piloti_type"], "IfcRelDefinesByType")
        self.assertEqual(len(link), 1)
        self.assertEqual([x["tag"] for x in link[0]["related"]], self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model))
        self.assertEqual(len(link[0]["related"]), CASES["grid"]["piloti_type_count"])
        for tag in self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model):
            own = self.relations(tag, "IfcRelDefinesByType")
            self.assertEqual(own[0]["relating"]["tag"], CASES["tags"]["piloti_type"])

    def test_every_opening_has_a_void_relation_and_optional_filling(self):
        fillings = {e["geometry"]["opening"]: e["tag"] for e in self.model["elements"] if "opening" in e["geometry"]}
        for opening in CASES["tags"]["openings"]:
            geometry = self.items[opening]["geometry"]
            host = geometry.get("wall",geometry.get("slab",geometry.get("curtain_wall")))
            voids = self.relations(opening, "IfcRelVoidsElement")
            self.assertEqual([(r["relating"]["tag"], r["related"][0]["tag"]) for r in voids], [(host, opening)])
            fills = self.relations(opening, "IfcRelFillsElement")
            expected = [] if opening in CASES["tags"]["unfilled_openings"] else [(opening, fillings[opening])]
            self.assertEqual([(r["relating"]["tag"], r["related"][0]["tag"]) for r in fills], expected)
            classes = {"IfcRelVoidsElement", "IfcRelContainedInSpatialStructure"}
            if expected:
                classes.add("IfcRelFillsElement")
            self.assertEqual({r["class"] for r in self.api.relations_of(opening, self.model)}, classes)

    def test_materials_are_associated_and_relation_ends_exist(self):
        associated = {tag for r in self.model["relations"] if r["class"] == "IfcRelAssociatesMaterial" for tag in r["related"]}
        for tag in (self.api.role("column_type_example", self.model), "SL-1", "SL-R", "TY-FACADE-29", "CW-1"):
            self.assertTrue(tag in associated or self.items[tag].get("type") in associated)
        for relation in self.model["relations"]:
            with self.subTest(tag=relation["tag"]):
                self.assertIn(relation["relating"], self.items)
                self.assertTrue(set(relation["related"]) <= set(self.items))
                self.assertTrue(relation["related"])
                self.assertEqual(relation["guid"], oracle_global_id(relation["tag"]))
        for key in ("MISSING",):
            with self.assertRaises(ValueError):
                self.api.relations_of(key, self.model)

    def test_relations_of_reports_both_sides_with_guids(self):
        relation = self.relations(self.api.role("column_example", self.model), "IfcRelDefinesByType")[0]
        self.assertEqual(relation["role"], "related")
        self.assertEqual(relation["relating"]["guid"], oracle_global_id(CASES["tags"]["piloti_type"]))
        self.assertEqual([o["tag"] for o in relation["others"]], [CASES["tags"]["piloti_type"]])
        self.assertTrue(relation["name"]["en"] and relation["name"]["de"])


class BimRepresentationContract(ModelCase):
    """👁️ One object has many representations that derive from the same data."""

    def test_identifiers_and_types_follow_ifc_representation_conventions(self):
        for tag in CASES["representations"]["objects"]:
            for kind, expected in CASES["representations"]["kinds"].items():
                with self.subTest(tag=tag, kind=kind):
                    result = self.api.represent(tag, kind, self.model)
                    self.assertEqual(result["identifier"], expected["identifier"])
                    self.assertIn(result["type"], expected["types"])
                    self.assertEqual(result["guid"], oracle_global_id(tag))
                    self.assertEqual(result["kind"], kind)
                    self.assertTrue(result["primitives"])

    def test_all_kinds_of_one_object_share_the_same_derived_values(self):
        for tag in CASES["representations"]["objects"]:
            derived = [self.api.represent(tag, kind, self.model)["derived"] for kind in CASES["representations"]["kinds"]]
            self.assertTrue(all(item == derived[0] for item in derived), tag)

    def test_piloti_representations_match_diameter_and_height(self):
        """🔩 Actual source outlines survive body, plan, section and footprint representations."""
        from shapely.geometry import Polygon
        tag=self.api.role("column_example",self.model)
        q=self.api.quantities(tag,self.model)
        body=self.api.represent(tag,"body",self.model)["primitives"][0]
        self.assertEqual(body["kind"],"extrusion")
        self.assertAlmostEqual(body["depth"],q["height"])
        shape=Polygon(body["profile"]["points"])
        self.assertAlmostEqual(shape.area,q["cross_section_area"],delta=3e-5)
        axis=np.asarray(self.api.represent(tag,"axis",self.model)["primitives"][0]["points"])
        self.assertAlmostEqual(float(np.linalg.norm(axis[1]-axis[0])),q["height"])
        plan=self.api.represent(tag,"plan",self.model)["primitives"][0]
        footprint=self.api.represent(tag,"footprint",self.model)["primitives"][0]
        self.assertEqual(plan["kind"],"polygon")
        self.assertTrue(Polygon(plan["points"]).equals(Polygon(footprint["points"])))


    def test_wall_representations_match_footprint_elevation_and_glazing(self):
        """🧱 Occupied representation solids retain material cavities and every opening."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        wall, quantities = self.wall("W-S"), self.api.quantities("W-S", self.model)
        footprint = self.api.represent("W-S", "footprint", self.model)["primitives"]
        actual=unary_union([Polygon(p["points"]) for p in footprint])
        expected=unary_union([Polygon(p["outline"]) for p in wall["parts"]])
        self.assertLess(actual.symmetric_difference(expected).area,1e-9)
        solids=self.api.represent("W-S","3d",self.model)["primitives"]
        levels=sorted({p[k] for p in solids for k in ("z0","z1")})
        volume=sum((hi-lo)*unary_union([Polygon(p["polygon"]) for p in solids if p["z0"]<(lo+hi)/2<p["z1"]]).area for lo,hi in zip(levels,levels[1:]))
        self.assertAlmostEqual(volume,quantities["volume"],places=6)
        self.assertTrue(all(not p["holes"] for p in solids))
        self.assertTrue(all(p["material"]!="MAT-AIR" for p in solids))
        self.assertEqual(len(solids),len(wall["parts"]))
        derived=self.api.represent("W-S","plan",self.model)["derived"]
        self.assertAlmostEqual(derived["volume"],volume,places=6)


    def test_slab_representations_have_outline_and_void_rings(self):
        """🥞 Regional footprint pieces retain occupied areas and local extrusion depths."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        tag=self.api.role("floor",self.model)
        q=self.api.quantities(tag,self.model)
        primitives=self.api.represent(tag,"footprint",self.model)["primitives"]
        occupied=[Polygon(p["points"]) for p in primitives if p["role"]=="outline"]
        expected=self.slab_domain(self.slab(tag))
        self.assert_partition(occupied,expected)
        self.assertAlmostEqual(sum(piece.area for piece in occupied),q["area"],places=6)
        body=self.api.represent(tag,"body",self.model)["primitives"]
        self.assertEqual({round(p["depth"],2) for p in body},{.17,.20,.23})
        self.assertAlmostEqual(sum(Polygon(p["profile"]["outer"],p["profile"].get("inner",[])).area*p["depth"] for p in body),q["volume"],places=5)


    def test_window_representation_is_the_opening_rectangle(self):
        """🪟 A physical frame and inset glass jointly fill the rough aperture."""
        from shapely.geometry import Polygon,box
        from shapely.ops import unary_union
        opening=self.wall("W-S")["openings"][0]
        elevation=self.api.represent("RW-S","elevation",self.model)["primitives"]
        occupied=unary_union([Polygon(p["points"]) for p in elevation])
        aperture=box(min(opening["p0"][0],opening["p1"][0]),opening["sill_z"],max(opening["p0"][0],opening["p1"][0]),opening["head_z"])
        self.assertLess(occupied.symmetric_difference(aperture).area,1e-8)
        self.assertEqual(len([p for p in elevation if p["role"]=="frame"]),4)
        self.assertEqual(len([p for p in elevation if p["role"]=="glass"]),1)
        frame=self.items["TY-RIBBON"]["filling"]["frame_width"]["value"]
        q=self.api.quantities("RW-S",self.model)
        self.assertAlmostEqual(q["clear_width"],q["rough_width"]-2*frame)


    def test_unknown_kinds_and_objects_without_representation_are_rejected(self):
        for key, kind in ((self.api.role("column_example", self.model), "mesh"), (self.api.role("column_example", self.model), ""), ("MISSING", "plan"), ("MAT-RC", "plan"), ("ground", "plan")):
            with self.subTest(key=key, kind=kind), self.assertRaises(ValueError):
                self.api.represent(key, kind, self.model)

    def test_event_regenerates_all_representations_and_keeps_the_guid(self):
        updated = self.api.apply_event(self.event("sill"), self.model)
        for kind in ("plan", "section", "elevation", "3d"):
            with self.subTest(kind=kind):
                before, after = self.api.represent("W-S", kind, self.model), self.api.represent("W-S", kind, updated)
                self.assertEqual(before["guid"], after["guid"])
                self.assertNotEqual(before["derived"], after["derived"])
        elevation = self.api.represent("RW-S", "elevation", updated)["primitives"][0]["points"]
        floor = self.geometry["storeys"]["first"]
        self.assertAlmostEqual(min(p[1] for p in elevation), floor + CASES["events"]["sill"]["sill"])


class BimPartialModelContract(ModelCase):
    """🧩 Discipline models share one schema, differ in content and overlap by GlobalId."""

    def test_models_share_schema_and_differ_in_content(self):
        partials = {identifier: self.api.partial_model(identifier, self.model) for identifier in CASES["models"]["ids"]}
        known = {item["guid"] for item in self.items.values()}
        self.assertEqual({p["schema_id"] for p in partials.values()}, {self.model["schema_id"]})
        self.assertEqual(len({tuple(sorted(p["element_guids"])) for p in partials.values()}), 3)
        for identifier, partial in partials.items():
            with self.subTest(model=identifier):
                self.assertEqual(partial["id"], identifier)
                self.assertTrue(set(partial["element_guids"]) <= known)
                self.assertEqual(len(partial["element_guids"]), len(set(partial["element_guids"])))
                self.assertEqual(sum(partial["counts_by_class"].values()), len(partial["element_guids"]))
                self.assertTrue(partial["author"].strip())
                self.assertGreaterEqual(partial["revision"], 1)
        self.assertEqual(len({p["author"] for p in partials.values()}), 3)

    def test_author_labels_are_bilingual_teaching_roles(self):
        for identifier in CASES["models"]["ids"]:
            label = self.api.partial_model(identifier, self.model)["author_label"]
            self.assertEqual(set(label), {"en", "de"})
            self.assertTrue(label["en"].strip() and label["de"].strip())
            self.assertNotEqual(label["en"], label["de"])
            self.assertTrue(label["en"].endswith("teaching role") and label["de"].endswith("Lehrrolle"))

    def test_counts_by_class_are_recomputed_from_the_guids(self):
        by_guid = {item["guid"]: item for item in self.items.values()}
        for identifier in CASES["models"]["ids"]:
            partial = self.api.partial_model(identifier, self.model)
            counts = {}
            for guid in partial["element_guids"]:
                counts[by_guid[guid]["class"]] = counts.get(by_guid[guid]["class"], 0) + 1
            self.assertEqual(partial["counts_by_class"], counts)

    def test_structure_holds_pilotis_and_architecture_holds_windows(self):
        architecture, structure, services = (self.api.partial_model(i, self.model) for i in CASES["models"]["ids"])
        self.assertEqual(structure["counts_by_class"].get("IfcColumn"), CASES["model"]["class_counts"]["IfcColumn"])
        self.assertEqual({tag for tag in architecture["element_tags"] if self.items[tag]["class"] == "IfcColumn"},set(CASES["models"]["architectural_columns"]))
        self.assertEqual(architecture["counts_by_class"].get("IfcWindow"), len(CASES["tags"]["windows"]))
        self.assertNotIn("IfcWindow", structure["counts_by_class"])
        self.assertTrue(set(services["counts_by_class"]) <= set(CASES["models"]["services_classes"]))
        for partial in (architecture,structure):
            shared={self.items[tag]["guid"] for tag in CASES["models"]["shared_services_by_model"][partial["id"]]}
            self.assertEqual(set(services["element_guids"]) & set(partial["element_guids"]),shared)
        self.assertTrue(set(CASES["models"]["shared_services"])<=set(services["element_tags"])&set(structure["element_tags"])&set(architecture["element_tags"]))

    def test_slabs_are_shared_between_architecture_and_structure_by_global_id(self):
        architecture, structure = (self.api.partial_model(i, self.model) for i in CASES["models"]["ids"][:2])
        shared = set(architecture["element_guids"]) & set(structure["element_guids"])
        for tag in CASES["models"]["shared"]:
            self.assertIn(self.items[tag]["guid"], shared)
        self.assertEqual(len(shared & {self.items[t]["guid"] for t in CASES["tags"]["slabs"]}), len(CASES["tags"]["slabs"]))

    def test_unknown_model_is_rejected(self):
        with self.assertRaises(ValueError):
            self.api.partial_model("unknown", self.model)


class BimStepExchangeContract(ModelCase):
    """📄 STEP excerpts are validated by an independent ISO 10303-21 parser."""

    def parsed(self, key, **kwargs):
        excerpt = self.api.step_excerpt(key, self.model, **kwargs)
        return excerpt, step_parse(excerpt["lines"])

    def test_envelope_header_and_attribute_counts(self):
        counts = CASES["step"]["attribute_counts"]
        for key in CASES["step"]["keys"]:
            for language in CASES["languages"]:
                with self.subTest(key=key, language=language):
                    excerpt, parsed = self.parsed(key, language=language)
                    self.assertTrue(any(h.startswith("FILE_SCHEMA") and self.model["schema_id"] in h for h in parsed["header"]))
                    self.assertTrue(any(h.startswith("FILE_DESCRIPTION") for h in parsed["header"]))
                    self.assertTrue(any(h.startswith("FILE_NAME") for h in parsed["header"]))
                    for number, (name, arguments) in parsed["entities"].items():
                        self.assertIn(name, counts, name)
                        self.assertEqual(len(arguments), counts[name], f"#{number} {name}")
                    self.assertEqual(excerpt["guid"], oracle_global_id(key))
                    self.assertEqual(excerpt["schema"], self.model["schema_id"])

    def test_element_entity_carries_guid_name_tag_and_predefined_type(self):
        for key in CASES["step"]["keys"]:
            for language in CASES["languages"]:
                with self.subTest(key=key, language=language):
                    element = self.items[key]
                    _, parsed = self.parsed(key, language=language)
                    entity = [(n, a) for n, a in parsed["entities"].items() if a[0] == element["class"].upper()]
                    self.assertEqual(len(entity), 1)
                    arguments = entity[0][1][1]
                    self.assertEqual(arguments[0], element["guid"])
                    self.assertRegex(arguments[0], GLOBAL_ID)
                    self.assertEqual(step_decode(arguments[2]), element["name"][language])
                    self.assertEqual(arguments[7], key)
                    self.assertEqual(arguments[8], ("enum", element["predefined_type"]))
                    self.assertEqual(arguments[1], None)

    def test_piloti_geometry_numbers_match_the_model(self):
        _, parsed = self.parsed(self.api.role("column_example", self.model))
        entities = parsed["entities"]
        by_name = {}
        for number, (name, arguments) in entities.items():
            by_name.setdefault(name, []).append((number, arguments))
        column = by_name["IFCCOLUMN"][0][1]
        placement = entities[column[5][1]][1]
        axis = entities[placement[1][1]][1]
        point = entities[axis[0][1]][1][0]
        piloti = next(p for p in self.geometry["pilotis"] if p["tag"] == self.api.role("column_example", self.model))
        self.assertTrue(np.allclose(point, [0, 0, 0]))
        shape = entities[column[6][1]][1]
        representation = entities[shape[2][0][1]][1]
        self.assertEqual(representation[1:3], ["Body", "SweptSolid"])
        solid = entities[representation[3][0][1]][1]
        profile = entities[solid[0][1]][1]
        self.assertAlmostEqual(profile[3], piloti["radius"])
        profile_axis = entities[profile[2][1]][1]
        centre = entities[profile_axis[0][1]][1][0]
        np.testing.assert_allclose(centre, [piloti["x"], piloti["y"]], atol=1e-9)
        self.assertAlmostEqual(solid[3], piloti["top"] - piloti["base"])
        direction = entities[solid[2][1]][1][0]
        self.assertTrue(np.allclose(direction, [0, 0, 1]))
        self.assertEqual(profile[0], ("enum", "AREA"))

    def test_pset_material_type_and_storey_entities_link_back_to_the_element(self):
        _, parsed = self.parsed(self.api.role("column_example", self.model))
        entities = parsed["entities"]
        element = next(n for n, (name, _) in entities.items() if name == "IFCCOLUMN")
        relations = {name: [(n, a) for n, (nm, a) in entities.items() if nm == name] for name in ("IFCRELASSOCIATESMATERIAL", "IFCRELDEFINESBYTYPE", "IFCRELCONTAINEDINSPATIALSTRUCTURE")}
        for name, items in relations.items():
            self.assertEqual(len(items), 1, name)
            self.assertEqual(items[0][1][4], [("ref", element)], name)
            self.assertRegex(items[0][1][0], GLOBAL_ID)
        self.assertEqual([n for n, (nm, _) in entities.items() if nm == "IFCRELDEFINESBYPROPERTIES"], [])
        kind_arguments = entities[relations["IFCRELDEFINESBYTYPE"][0][1][5][1]][1]
        self.assertEqual(len(kind_arguments[5]), 1)
        pset = entities[kind_arguments[5][0][1]]
        self.assertEqual(pset[0], "IFCPROPERTYSET")
        self.assertEqual(pset[1][2], "Pset_ColumnCommon")
        properties = {entities[ref[1]][1][0]: entities[ref[1]][1][2] for ref in pset[1][4]}
        self.assertEqual(properties["LoadBearing"], ("typed", "IFCBOOLEAN", [("enum", "T")]))
        self.assertIn("IsExternal", properties)
        material = entities[relations["IFCRELASSOCIATESMATERIAL"][0][1][5][1]]
        self.assertEqual((material[0], material[1][0]), ("IFCMATERIAL", self.items["MAT-RC"]["name"]["en"]))
        kind = entities[relations["IFCRELDEFINESBYTYPE"][0][1][5][1]]
        self.assertEqual((kind[0], kind[1][0]), ("IFCCOLUMNTYPE", self.items[self.api.role("column_type_example", self.model)]["guid"]))
        storey = entities[relations["IFCRELCONTAINEDINSPATIALSTRUCTURE"][0][1][5][1]]
        self.assertEqual((storey[0], storey[1][0], storey[1][9]), ("IFCBUILDINGSTOREY", self.items["ground"]["guid"], 0.0))

    def test_slab_and_wall_geometry_numbers_match_the_model(self):
        """🧾 IFC regional polygon extrusions retain occupied source volume."""
        from shapely.geometry import Polygon
        _,parsed=self.parsed("SL-1")
        entities=parsed["entities"]
        volume=0.0
        for name,solid in entities.values():
            if name!="IFCEXTRUDEDAREASOLID":
                continue
            profile=entities[solid[0][1]][1]
            curve=entities[profile[2][1]][1]
            points=[entities[ref[1]][1][0] for ref in curve[0]]
            volume+=Polygon(points).area*solid[3]
        self.assertAlmostEqual(volume,self.api.quantities("SL-1",self.model)["volume"],places=5)
        _,parsed=self.parsed("SL-R")
        layers=[a for n,a in parsed["entities"].values() if n=="IFCMATERIALLAYER"]
        self.assertEqual([l[1] for l in layers],[l["thickness"]["value"] for l in self.items["LS-ROOF"]["layers"]])
        _,parsed=self.parsed("W-S")
        entities=parsed["entities"]
        volume=0.0
        for name,solid in entities.values():
            if name!="IFCEXTRUDEDAREASOLID":
                continue
            profile=entities[solid[0][1]][1]
            curve=entities[profile[2][1]][1]
            points=[entities[ref[1]][1][0] for ref in curve[0]]
            volume+=Polygon(points).area*solid[3]
        self.assertAlmostEqual(volume,self.api.quantities("W-S",self.model)["volume"],places=5)

    def test_renumbering_changes_entity_numbers_but_never_the_global_id(self):
        first, parsed_first = self.parsed(self.api.role("column_example", self.model))
        second, parsed_second = self.parsed(self.api.role("column_example", self.model), first_id=500)
        self.assertEqual(min(parsed_first["entities"]), 101)
        self.assertEqual(min(parsed_second["entities"]), 500)
        self.assertEqual(sorted(parsed_second["entities"]), [number + 399 for number in sorted(parsed_first["entities"])])
        self.assertEqual([name for name, _ in parsed_first["entities"].values()], [name for name, _ in parsed_second["entities"].values()])
        guids = lambda parsed: [a[0] for n, a in parsed["entities"].values() if n == "IFCCOLUMN"]
        self.assertEqual(guids(parsed_first), guids(parsed_second))
        self.assertEqual(first["guid"], second["guid"])
        self.assertNotEqual(first["lines"], second["lines"])

    def test_spans_point_at_the_tokens_they_name(self):
        for key in CASES["step"]["keys"]:
            with self.subTest(key=key):
                excerpt = self.api.step_excerpt(key, self.model)
                element = self.items[key]
                lines, by_role = excerpt["lines"], {}
                spans = excerpt["spans"]
                for span in spans:
                    self.assertTrue(0 <= span["line"] < len(lines))
                    self.assertTrue(0 <= span["start"] < span["end"] <= len(lines[span["line"]]))
                    by_role.setdefault(span["role"], []).append(lines[span["line"]][span["start"]:span["end"]])
                for line in range(len(lines)):
                    ranges = sorted((s["start"], s["end"]) for s in spans if s["line"] == line)
                    self.assertTrue(all(a[1] <= b[0] for a, b in zip(ranges, ranges[1:])))
                self.assertIn(element["guid"], [text.strip("'") for text in by_role["guid"]])
                self.assertEqual(by_role["class"], [element["class"].upper()])
                self.assertIn(key, [text.strip("'") for text in by_role["tag"]])
                self.assertEqual(by_role["predefined"], [f".{element['predefined_type']}."])
                self.assertTrue(all(text.startswith("#") for text in by_role["number"]))
                self.assertGreaterEqual(len(by_role["placement"]), 1)
                self.assertIn("Body", [text.strip("'") for text in by_role["representation"]])

    def test_property_and_value_spans_hold_the_pset_content(self):
        excerpt = self.api.step_excerpt(self.api.role("column_example", self.model), self.model)
        text = lambda role: [excerpt["lines"][s["line"]][s["start"]:s["end"]] for s in excerpt["spans"] if s["role"] == role]
        self.assertEqual(text("property"), ["LoadBearing", "IsExternal"])
        self.assertEqual(text("value"), [".T.", ".F."])
        self.assertEqual(text("pset"), ["Pset_ColumnCommon"])
        self.assertEqual(text("material"), [self.items["MAT-RC"]["name"]["en"]])
        self.assertAlmostEqual(float(text("depth")[0]), self.api.quantities(self.api.role("column_example", self.model), self.model)["height"])
        self.assertAlmostEqual(float(text("profile")[0]), self.piloti_radius())

    def test_invalid_requests_are_rejected(self):
        for key, kwargs in (("MISSING", {}), ("MAT-RC", {}), (self.api.role("column_example", self.model), {"language": "fr"}), (self.api.role("column_example", self.model), {"first_id": 0}), (self.api.role("column_example", self.model), {"first_id": True})):
            with self.subTest(key=key, kwargs=kwargs), self.assertRaises(ValueError):
                self.api.step_excerpt(key, self.model, **kwargs)

    def test_parser_oracle_rejects_malformed_fragments(self):
        good = self.api.step_excerpt(self.api.role("column_example", self.model), self.model)["lines"]
        broken = {
            "unbalanced": [line.replace(");", ";", 1) if line.startswith("#102") else line for line in good],
            "undefined-ref": [line.replace("#102", "#999", 1) if line.startswith("#101") else line for line in good],
            "open-string": [line.replace("'19'", "'C2", 1) for line in good],
            "no-data": [line for line in good if line != "DATA;"],
            "duplicate": good[:8] + [good[7]] + good[8:],
            "no-end": good[:-1],
        }
        for name, lines in broken.items():
            with self.subTest(name=name), self.assertRaises(ValueError):
                step_parse(lines)


class BimMeshExchangeContract(ModelCase):
    """🔺 The triangle export is a closed watertight cylinder that has lost its semantics."""

    def test_counts_euler_characteristic_and_watertightness_for_many_resolutions(self):
        for segments in CASES["mesh"]["segments"]:
            with self.subTest(segments=segments):
                result = self.api.mesh(self.api.role("column_example", self.model), segments=segments, model=self.model)
                report = mesh_report(result["vertices"], result["faces"])
                self.assertEqual(result["vertex_count"], len(result["vertices"]))
                self.assertEqual(result["triangle_count"], len(result["faces"]))
                self.assertEqual(report["euler"], 2)
                self.assertEqual(report["edge_use"], {2})
                self.assertTrue(report["oriented"])
                self.assertEqual(report["used"], result["vertex_count"])
                self.assertEqual(result["triangle_count"], 2 * (result["vertex_count"] - 2))
                self.assertEqual(report["edges"], 3 * result["triangle_count"] // 2)
                self.assertEqual(result["segments"], segments)
                self.assertEqual(result["vertex_count"], 2 * segments)
                self.assertEqual(result["triangle_count"], 4 * segments - 4)

    def test_volume_and_area_equal_the_inscribed_prism_and_the_convex_hull(self):
        from scipy.spatial import ConvexHull

        radius, height = self.piloti_radius(), self.api.quantities(self.api.role("column_example", self.model), self.model)["height"]
        for segments in (4, 6, 8, 16, 64):
            with self.subTest(segments=segments):
                result = self.api.mesh(self.api.role("column_example", self.model), segments=segments, model=self.model)
                report = mesh_report(result["vertices"], result["faces"])
                prism = segments / 2 * radius ** 2 * math.sin(2 * math.pi / segments) * height
                self.assertAlmostEqual(report["volume"], prism, places=6)
                hull = ConvexHull(np.asarray(result["vertices"]))
                self.assertAlmostEqual(report["volume"], hull.volume, places=6)
                self.assertAlmostEqual(report["area"], hull.area, places=6)
                self.assertLessEqual(report["volume"], math.pi * radius ** 2 * height + 1e-9)
        fine = mesh_report(*[self.api.mesh(self.api.role("column_example", self.model), segments=256, model=self.model)[k] for k in ("vertices", "faces")])
        self.assertAlmostEqual(fine["volume"], math.pi * radius ** 2 * height, delta=1e-4)

    def test_vertices_lie_on_the_cylinder_between_base_and_top(self):
        piloti = next(p for p in self.geometry["pilotis"] if p["tag"] == self.api.role("column_example", self.model))
        result = self.api.mesh(self.api.role("column_example", self.model), segments=12, model=self.model)
        vertices = np.asarray(result["vertices"])
        distance = np.hypot(vertices[:, 0] - piloti["x"], vertices[:, 1] - piloti["y"])
        self.assertTrue(np.all(np.isclose(distance, piloti["radius"], atol=1e-8) | np.isclose(distance, 0, atol=1e-8)))
        self.assertTrue(np.all(np.isclose(vertices[:, 2], piloti["base"]) | np.isclose(vertices[:, 2], piloti["top"])))
        self.assertEqual(int(np.isclose(distance, piloti["radius"]).sum()), 2 * 12)
        self.assertTrue(result["guid"] == piloti["guid"])

    def test_mesh_lists_what_it_keeps_and_what_it_loses(self):
        result = self.api.mesh(self.api.role("column_example", self.model), model=self.model)
        kept, lost = {k["id"] for k in result["kept"]}, {k["id"] for k in result["lost"]}
        self.assertEqual(kept, {"vertices", "triangles"})
        self.assertTrue({"class", "guid", "property", "material"} <= lost)
        for item in result["kept"] + result["lost"]:
            self.assertTrue(item["en"].strip() and item["de"].strip())
        self.assertNotIn("class", result)
        self.assertNotIn("psets", result)

    def test_invalid_requests_are_rejected(self):
        for key, segments in ((self.api.role("column_example", self.model), 2), (self.api.role("column_example", self.model), 0), (self.api.role("column_example", self.model), -4), (self.api.role("column_example", self.model), True), (self.api.role("column_example", self.model), "8"), (self.api.role("column_example", self.model), 8.0), ("SL-1", 8), ("MISSING", 8)):
            with self.subTest(key=key, segments=segments), self.assertRaises(ValueError):
                self.api.mesh(key, segments=segments, model=self.model)


class BimIdsContract(ModelCase):
    """✅ An IDS-style requirement separates what the model says from what the mesh lost."""

    def test_default_requirement_passes_for_all_pilotis_in_ifc_and_finds_none_in_the_mesh(self):
        semantic = self.api.ids_check(model=self.model, source="semantic")
        mesh = self.api.ids_check(model=self.model, source="mesh")
        self.assertEqual((semantic["found"], semantic["checked"], semantic["passed"], semantic["failed"]), (CASES["grid"]["pilotis"],) * 3 + ([],))
        self.assertEqual(semantic["applicable"], CASES["grid"]["column_tags"])
        self.assertEqual(semantic["passed_tags"], semantic["applicable"])
        self.assertEqual((mesh["found"], mesh["checked"], mesh["passed"], mesh["failed"], mesh["applicable"]), (0, 0, 0, [], []))
        self.assertEqual(mesh["expected"], CASES["grid"]["pilotis"])
        for result in (semantic, mesh):
            self.assertTrue(result["requirement"]["en"].strip() and result["requirement"]["de"].strip())
            self.assertTrue(result["applicability"]["en"].strip() and result["applicability"]["de"].strip())

    def test_counts_equal_an_independent_recount_of_the_json(self):
        for case in CASES["ids"]["cases"]:
            with self.subTest(case=case["id"]):
                result = self.api.ids_check(model=self.model, spec=case["spec"], source="semantic")
                entity = case["spec"]["entity"] if isinstance(case["spec"], dict) else case["spec"].split()[0]
                members = [e for e in self.model["elements"] if e["class"] == entity]
                self.assertEqual(result["found"], len(members))
                self.assertEqual(result["passed"], case["passed"])
                self.assertEqual(len(result["failed"]), case["failed"])
                self.assertEqual(result["passed"] + len(result["failed"]), result["checked"])
                self.assertEqual(set(result["passed_tags"]) | set(result["failed"]), {e["tag"] for e in members})

    def test_removing_the_property_makes_every_piloti_fail(self):
        broken = copy.deepcopy(self.model)
        for kind in broken["types"]:
            if kind["class"] == "IfcColumnType":
                del kind["psets"]["Pset_ColumnCommon"]["LoadBearing"]
        for element in broken["elements"]:
            if element["class"] == "IfcColumn":
                element["psets"].get("Pset_ColumnCommon", {}).pop("LoadBearing", None)
        result = self.api.ids_check(model=broken, source="semantic")
        self.assertEqual((result["passed"], len(result["failed"])), (0, CASES["grid"]["pilotis"]))
        self.assertEqual(self.api.ids_check(model=self.model, source="semantic")["passed"], CASES["grid"]["pilotis"])

    def test_invalid_specs_and_sources_are_rejected(self):
        for kwargs in ({"source": "dwg"}, {"spec": "IfcColumn"}, {"spec": {"entity": "IfcColumn"}}, {"spec": {"entity": "IfcColumn", "property": "X", "cardinality": "maybe"}}, {"spec": 7}):
            with self.subTest(kwargs=kwargs), self.assertRaises(ValueError):
                self.api.ids_check(model=self.model, **kwargs)


class BimAnalysisContract(ModelCase):
    """🏗️ Actual regional mid-surfaces and profile sections define the translated structure."""

    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.analysis = cls.api.translate_structure(cls.model)

    def floor(self):
        return self.slab(self.api.role("floor", self.model))

    def test_source_and_target_schema_and_revision(self):
        analysis = self.analysis
        self.assertEqual(analysis["source_schema"], self.model["schema_id"])
        self.assertNotEqual(analysis["target_schema"], analysis["source_schema"])
        self.assertEqual(analysis["source_revision"], self.api.partial_model("structure", self.model)["revision"])
        self.assertEqual(analysis["model_class"], "IfcStructuralAnalysisModel")

    def test_mesh_nodes_match_independent_shapely_clipped_grid(self):
        from shapely.geometry import Point, Polygon, box
        from scipy.spatial import cKDTree
        slab = self.floor()
        roof = self.slab(self.api.role("roof", self.model))
        physical = self.physical_supports(self.model, slab) + self.physical_supports(self.model, roof)
        xs = sorted({p[0] for region in slab["regions"] for p in region["outline"]} | set(self.model["grid"]["x"]) | {p["x"] for p in physical})
        ys = sorted({p[1] for region in slab["regions"] for p in region["outline"]} | set(self.model["grid"]["y"]) | {p["y"] for p in physical})
        self.assertEqual(self.analysis["mesh"]["x_lines"], xs)
        self.assertEqual(self.analysis["mesh"]["y_lines"], ys)
        decimals=CASES["analysis_geometry"]["coordinate_decimals"]
        expected = set()
        def add(point):
            expected.add(tuple(float(c) for c in point))
        for part in slab["full_parts"]:
            polygon = Polygon(part["outline"])
            z = (part["z0"] + part["z1"]) / 2
            xmin,ymin,xmax,ymax = polygon.bounds
            for x0,x1 in zip(xs,xs[1:]):
                if x1 <= xmin + 1e-9 or x0 >= xmax - 1e-9:
                    continue
                for y0,y1 in zip(ys,ys[1:]):
                    if y1 <= ymin + 1e-9 or y0 >= ymax - 1e-9:
                        continue
                    shape = polygon.intersection(box(x0,y0,x1,y1))
                    for cell in getattr(shape,"geoms",[shape]):
                        if cell.geom_type == "Polygon" and cell.area > 1e-9:
                            for x,y in cell.exterior.coords:
                                add([x,y,z])
        for column in physical:
            point = Point(column["x"],column["y"])
            candidates = [part for part in slab["full_parts"] if Polygon(part["outline"]).covers(point)]
            if not candidates:
                distance = min(Polygon(part["outline"]).distance(point) for part in slab["full_parts"])
                candidates = [part for part in slab["full_parts"] if Polygon(part["outline"]).distance(point) <= distance + 1e-8]
            add([column["x"],column["y"],max((p["z0"]+p["z1"])/2 for p in candidates)])
        actual = {tuple(n["position"]) for n in self.analysis["nodes"] if n["role"] == "slab"}
        self.assertTrue(all(c==round(c,decimals) for point in actual for c in point))
        a,b=np.asarray(list(actual)),np.asarray(list(expected))
        bound=math.sqrt(3)*0.5*10**(-decimals)+32*np.finfo(float).eps*max(abs(a).max(),abs(b).max())
        self.assertLessEqual(float(cKDTree(a).query(b)[0].max()),bound)
        self.assertLessEqual(float(cKDTree(b).query(a)[0].max()),bound)
        self.assertEqual(len({n["id"] for n in self.analysis["nodes"]}),len(self.analysis["nodes"]))

    def test_shells_tile_the_actual_regions_with_local_depth_and_mid_surface(self):
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        slab = self.floor()
        nodes = {node["id"]:node["position"] for node in self.analysis["nodes"]}
        by_region = {}
        for shell in self.analysis["shells"]:
            self.assertGreaterEqual(len(shell["nodes"]),3)
            points = [nodes[node] for node in shell["nodes"]]
            polygon = Polygon([p[:2] for p in points])
            self.assertTrue(polygon.is_valid and polygon.area > 1e-9)
            self.assertAlmostEqual(shoelace([p[:2] for p in points]),shell["area"],places=6)
            region = next(region for region in slab["regions"] if region["tag"] == shell["region"])
            self.assertAlmostEqual(shell["thickness"],region["thickness"])
            self.assertAlmostEqual(shell["mid_surface"],region["top"]-region["thickness"]/2)
            self.assertTrue(all(abs(p[2]-shell["mid_surface"]) < 1e-8 for p in points))
            self.assertEqual(shell["source_guid"],slab["guid"])
            by_region.setdefault(shell["region"],[]).append(polygon)
        self.assertEqual(set(by_region),{part["region"] for part in slab["full_parts"]})
        for tag,polygons in by_region.items():
            region=next(region for region in slab["regions"] if region["tag"]==tag)
            expected=Polygon(region["outline"]).intersection(self.slab_domain(slab))
            self.assert_partition(polygons,expected)
        self.assertAlmostEqual(sum(shell["area"] for shell in self.analysis["shells"]),self.api.quantities(slab["tag"],self.model)["area"],places=6)

    def test_members_map_to_physical_supports_and_preserve_actual_profile_sections(self):
        from shapely.geometry import Polygon
        members = self.analysis["members"]
        curves = [member for member in members if member["kind"] == "curve"]
        surfaces = [member for member in members if member["kind"] == "surface"]
        physical = {p["tag"]:p for p in self.physical_supports(self.model,self.floor())}
        self.assertEqual({m["source_tag"] for m in curves},set(physical))
        self.assertEqual({m["ifc_class"] for m in curves},{"IfcStructuralCurveMember"})
        self.assertEqual({m["ifc_class"] for m in surfaces},{"IfcStructuralSurfaceMember"})
        self.assertEqual({m["source_guid"] for m in surfaces},{self.floor()["guid"]})
        self.assertEqual({m["shell"] for m in surfaces},{s["id"] for s in self.analysis["shells"]})
        nodes = {node["id"]:node["position"] for node in self.analysis["nodes"]}
        for member in curves:
            column = physical[member["source_tag"]]
            self.assertEqual(member["source_guid"],column["guid"])
            self.assertEqual(len(member["sections"]),len(column["parts"]))
            for section,part in zip(member["sections"],column["parts"]):
                self.assertEqual(section["profile"],part["profile"])
                profile = section["profile"]
                area=profile_area(profile,part["outline"],part.get("holes",[]))
                self.assertAlmostEqual(section["cross_section_area"],area,places=8)
                self.assertAlmostEqual(section["base"],part["z0"])
                self.assertAlmostEqual(section["top"],part["z1"])
            if len(member["sections"]) == 1 and member["sections"][0]["profile"]["shape"] == "circle":
                self.assertAlmostEqual(member["diameter"],member["sections"][0]["profile"]["diameter"]["value"])
            else:
                self.assertNotIn("diameter",member)
            self.assertTrue(set(member["nodes"]) <= nodes.keys())
            self.assertAlmostEqual(member["length"],nodes[member["nodes"][1]][2]-nodes[member["nodes"][0]][2])
        self.assertEqual(len({m["id"] for m in members}),len(members))

    def test_supports_use_actual_column_births_and_the_declared_condition(self):
        physical = {p["tag"]:p for p in self.physical_supports(self.model,self.floor())}
        nodes = {n["id"]:n["position"] for n in self.analysis["nodes"]}
        self.assertEqual({s["source_tag"] for s in self.analysis["supports"]},set(physical))
        for support in self.analysis["supports"]:
            column = physical[support["source_tag"]]
            self.assertEqual(nodes[support["node"]],[column["x"],column["y"],column["base"]])
            self.assertEqual(support["condition"],self.model["translation"]["boundary"]["class"])
            self.assertTrue(support["label"]["en"].strip() and support["label"]["de"].strip())
            self.assertEqual(support["source_guid"],oracle_global_id(support["source_tag"]))

    def test_losses_name_exact_source_content_and_do_not_discard_preserved_floor_voids(self):
        losses = self.analysis["losses"]
        expected = set()
        for rule in self.model["translation"]["losses"]:
            target = rule["applies_to"]
            expected.update((item["tag"],rule["property"]) for item in [*self.model["elements"],*self.model["spaces"]] if item["class"] == target["class"] and ("predefined_type" not in target or item["predefined_type"] == target["predefined_type"]))
        self.assertEqual({(loss["source_tag"],loss["property"]) for loss in losses},expected)
        self.assertNotIn((self.floor()["tag"],"voids"),expected)
        for loss in losses:
            self.assertEqual(loss["source_guid"],oracle_global_id(loss["source_tag"]))
            self.assertTrue(loss["en"].strip() and loss["de"].strip() and loss["property"].strip())
        physical = {column["tag"] for column in self.geometry["pilotis"]}
        self.assertFalse(any(loss["source_tag"] in physical and loss["property"] == "element" for loss in losses))

    def test_assumptions_are_bilingual_explicit_and_do_not_claim_uniform_solid_concrete(self):
        assumptions = self.analysis["assumptions"]
        self.assertEqual(len({a["id"] for a in assumptions}),len(assumptions))
        for item in assumptions:
            self.assertEqual(item["basis"],"assumed")
            self.assertTrue(item["en"].strip() and item["de"].strip())
            self.assertNotIn("{",item["en"]+item["de"])
        ids = {a["id"] for a in assumptions}
        self.assertTrue({"mid-surface","support-fixed","material","loads"} <= ids)
        material = next(a for a in assumptions if a["id"] == "material")
        self.assertNotIn("all members are reinforced concrete",material["en"].lower())

    def test_floor_load_integrates_actual_solid_and_composite_material_regions(self):
        patches = self.load_patches(self.model,"first")
        area = sum(p["area"] for p in patches)
        expected = {key:sum(p["area"]*p[field] for p in patches)/area for key,field in (("self_weight","self_weight"),("finishes","finish"),("imposed","imposed"))}
        loads = self.analysis["loads"]
        for key,value in expected.items():
            self.assertAlmostEqual(loads[key]["value"],value,places=7)
            self.assertEqual(loads[key]["unit"],"kN/m²")
            self.assertEqual(loads[key]["basis"],"assumed")
        self.assertAlmostEqual(loads["area_load"],sum(expected.values()),places=7)
        self.assertAlmostEqual(loads["total"]["value"],self.floor_load(self.model),places=7)

    def test_point_loads_carry_actual_roof_reactions_to_the_slab_nodes(self):
        roof = self.api.tributary(self.model,level="roof")
        nodes = {node["id"]:node["position"] for node in self.analysis["nodes"]}
        reactions = {entry["tag"]:entry["reaction"] for entry in roof["pilotis"] if entry["reaction"] > 0}
        self.assertEqual({load["source_tag"] for load in self.analysis["point_loads"]},set(reactions))
        for load in self.analysis["point_loads"]:
            column = next(p for p in self.geometry["pilotis"] if p["tag"] == load["source_tag"])
            self.assertAlmostEqual(load["value"],reactions[load["source_tag"]])
            self.assertEqual(nodes[load["node"]][:2],[column["x"],column["y"]])
            self.assertEqual(load["unit"],"kN")
        self.assertAlmostEqual(sum(load["value"] for load in self.analysis["point_loads"]),roof["total_reaction"],places=6)

    def test_mapping_rules_and_counts_match_the_derived_objects(self):
        counts = self.analysis["counts"]
        curves = sum(member["kind"] == "curve" for member in self.analysis["members"])
        surfaces = sum(member["kind"] == "surface" for member in self.analysis["members"])
        supports = len(self.analysis["supports"])
        self.assertEqual({rule["kind"]:rule["count"] for rule in self.analysis["mapping"]},{"curve":curves,"surface":surfaces,"support":supports})
        for key,value in (("curve_members",curves),("surface_members",surfaces),("supports",supports),("members",len(self.analysis["members"])),("nodes",len(self.analysis["nodes"])),("shells",len(self.analysis["shells"])),("point_loads",len(self.analysis["point_loads"]))):
            self.assertEqual(counts[key],value)
        self.assertEqual(counts["slab_nodes"],sum(node["role"] == "slab" for node in self.analysis["nodes"]))
        self.assertEqual(counts["base_nodes"],supports)

    def test_regional_metadata_and_offset_links_follow_actual_node_eccentricities(self):
        slab = self.floor()
        area = sum(shoelace(part["outline"]) for part in slab["full_parts"])
        mean = sum(shoelace(part["outline"])*(part["z0"]+part["z1"])/2 for part in slab["full_parts"])/area
        self.assertAlmostEqual(self.analysis["mid_surface"]["z"],mean,places=8)
        self.assertEqual({region["tag"] for region in self.analysis["mid_surface"]["regions"]},{region["tag"] for region in slab["regions"]})
        by_plan = {}
        for node in self.analysis["nodes"]:
            if node["role"] == "slab":
                by_plan.setdefault(tuple(node["position"][:2]),[]).append(node)
        expected = {(a["id"],b["id"]) for group in by_plan.values() for ordered in [sorted(group,key=lambda node:node["position"][2])] for a,b in zip(ordered,ordered[1:])}
        self.assertEqual({tuple(link["nodes"]) for link in self.analysis["offset_links"]},expected)
        nodes = {node["id"]:node["position"] for node in self.analysis["nodes"]}
        for link in self.analysis["offset_links"]:
            a,b = (nodes[node] for node in link["nodes"])
            self.assertEqual(a[:2],b[:2])
            self.assertAlmostEqual(link["offset"],b[2]-a[2],places=8)
            self.assertEqual(link["basis"],"assumed")

    def test_translation_follows_the_model_state(self):
        updated = self.api.apply_event(self.event("paving"),self.model)
        after = self.api.translate_structure(updated)
        self.assertEqual(after["source_revision"],self.api.partial_model("structure",updated)["revision"])
        self.assertEqual(self.analysis["source_revision"],self.api.partial_model("structure",self.model)["revision"])
        self.assertEqual(after["loads"]["area_load"],self.analysis["loads"]["area_load"])
        before_roof = {p["source_tag"]:p["value"] for p in self.analysis["point_loads"]}
        after_roof = {p["source_tag"]:p["value"] for p in after["point_loads"]}
        self.assertEqual(set(before_roof),set(after_roof))
        self.assertTrue(all(after_roof[tag] > before_roof[tag] for tag in before_roof))
        expected_delta = sum(p["area"]*p["q"] for p in self.load_patches(updated,"roof"))-sum(p["area"]*p["q"] for p in self.load_patches(self.model,"roof"))
        self.assertAlmostEqual(sum(after_roof.values())-sum(before_roof.values()),expected_delta,places=5)
        sill = self.api.translate_structure(self.api.apply_event(self.event("sill"),self.model))
        self.assertEqual(sill["source_revision"],self.analysis["source_revision"])
        self.assertEqual(sill["point_loads"],self.analysis["point_loads"])


class BimTributaryContract(ModelCase):
    """📐 Independent SciPy cells and GEOS material-patch integrals verify regional reactions."""

    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.results = {level:cls.api.tributary(cls.model,level=level) for level in ("first","roof")}
        cls._sample_cache = {}
        cls._cell_cache = {}

    def primary_slab(self,level):
        return self.slab(self.api.role({"first":"floor","roof":"roof"}[level],self.model))

    def oracle(self,level):
        from scipy.spatial import cKDTree
        if level not in self._sample_cache:
            slab = self.primary_slab(level)
            step = CASES["tributary"]["step"] / 2
            points = sample_surface(slab["outline"],slab["voids"],step)
            supports = self.physical_supports(self.model,slab)
            nearest = cKDTree(np.array([[p["x"],p["y"]] for p in supports])).query(points)[1]
            areas = np.bincount(nearest,minlength=len(supports))*step**2
            self._sample_cache[level] = slab,points,nearest,areas,supports
        return self._sample_cache[level]

    def independent_cells(self,level):
        from scipy.spatial import HalfspaceIntersection
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        if level not in self._cell_cache:
            slab = self.primary_slab(level)
            domain = self.slab_domain(slab)
            supports = self.physical_supports(self.model,slab)
            xmin,ymin,xmax,ymax = domain.bounds
            cells = {}
            for pil in supports:
                x,y = pil["x"],pil["y"]
                halfspaces = [[1,0,-xmax-10],[-1,0,xmin-10],[0,1,-ymax-10],[0,-1,ymin-10]]
                for other in supports:
                    if other["tag"] != pil["tag"]:
                        ox,oy = other["x"],other["y"]
                        halfspaces.append([ox-x,oy-y,-(ox*ox+oy*oy-x*x-y*y)/2])
                vertices = HalfspaceIntersection(np.asarray(halfspaces,dtype=float),np.array([x,y],dtype=float)).intersections
                centre = vertices.mean(axis=0)
                order = np.argsort(np.arctan2(vertices[:,1]-centre[1],vertices[:,0]-centre[0]))
                cells[pil["tag"]] = Polygon(vertices[order]).intersection(domain)
            self._cell_cache[level] = cells
        return self._cell_cache[level]

    def test_areas_sum_to_actual_surface_and_match_the_independent_nearest_support_sampler(self):
        for level,result in self.results.items():
            with self.subTest(level=level):
                slab,_,_,areas,supports = self.oracle(level)
                net = sum(part["area"] for part in self.load_patches(self.model,level))
                self.assertEqual({entry["tag"] for entry in result["pilotis"]},{p["tag"] for p in supports})
                sampled = {support["tag"]:areas[i] for i,support in enumerate(supports)}
                self.assertAlmostEqual(sum(entry["area"] for entry in result["pilotis"]),net,places=6)
                self.assertAlmostEqual(result["slab_area"],net,places=6)
                self.assertAlmostEqual(areas.sum(),net,delta=.02*net)
                for entry in result["pilotis"]:
                    self.assertAlmostEqual(entry["area"],sampled[entry["tag"]],delta=CASES["tributary"]["tolerance"]*max(entry["area"],1))
                self.assertTrue(result["balanced"])
                self.assertEqual(result["slab_tag"],slab["tag"])

    def test_each_reaction_integrates_local_solid_composite_and_finish_patches(self):
        for level,result in self.results.items():
            with self.subTest(level=level):
                patches = self.load_patches(self.model,level)
                cells = self.independent_cells(level)
                expected_total = sum(p["area"]*p["q"] for p in patches)
                area = sum(p["area"] for p in patches)
                self.assertAlmostEqual(result["area_load"],expected_total/area,places=7)
                self.assertAlmostEqual(result["total_reaction"],expected_total,places=5)
                for entry in result["pilotis"]:
                    expected = sum(p["shape"].intersection(cells[entry["tag"]]).area*p["q"] for p in patches)
                    self.assertAlmostEqual(entry["reaction"],expected,places=5)
                self.assertAlmostEqual(sum(p["reaction"] for p in result["pilotis"]),expected_total,places=5)

    def test_roof_zones_integrate_local_depth_and_material_and_match_sampling(self):
        result = self.results["roof"]
        patches = self.load_patches(self.model,"roof")
        zones = {zone["id"]:zone for zone in result["zones"]}
        self.assertEqual(set(zones),{patch["zone"] for patch in patches})
        cells = self.independent_cells("roof")
        for tag,zone in zones.items():
            local = [p for p in patches if p["zone"] == tag]
            area = sum(p["area"] for p in local)
            force = sum(p["area"]*p["q"] for p in local)
            self.assertAlmostEqual(zone["area"],area,places=6)
            self.assertAlmostEqual(zone["area_load"],force/area,places=7)
            self.assertAlmostEqual(zone["reaction"],force,places=5)
            self.assertAlmostEqual(zone["load_breakdown"]["paving"]["value"],sum(p["area"]*p["finish"] for p in local)/area,places=7)
            for entry in result["pilotis"]:
                expected_area = sum(p["shape"].intersection(cells[entry["tag"]]).area for p in local)
                expected_force = sum(p["shape"].intersection(cells[entry["tag"]]).area*p["q"] for p in local)
                self.assertAlmostEqual(entry["zones"][tag]["area"],expected_area,places=6)
                self.assertAlmostEqual(entry["zones"][tag]["reaction"],expected_force,places=5)
        sampled = self.roof_reactions(self.model,step=CASES["tributary"]["step"]/2)
        for entry in result["pilotis"]:
            self.assertAlmostEqual(entry["reaction"],sampled[entry["tag"]],delta=.012*max(entry["reaction"],10))
        self.assertAlmostEqual(sum(zone["reaction"] for zone in zones.values()),result["total_reaction"],places=5)

    def test_cells_are_disjoint_actual_polygons_that_cover_both_slabs(self):
        from shapely.geometry import Polygon
        for level,result in self.results.items():
            domain=self.slab_domain(self.primary_slab(level))
            pieces=[Polygon(ring) for entry in result["pilotis"] for ring in entry["pieces"]]
            self.assert_partition(pieces,domain)

    def test_cell_kinds_follow_actual_axis_positions_without_parsing_archival_tags(self):
        xs,ys = self.model["grid"]["x"],self.model["grid"]["y"]
        for result in self.results.values():
            for entry in result["pilotis"]:
                edges = int(entry["x"] in (xs[0],xs[-1]))+int(entry["y"] in (ys[0],ys[-1]))
                self.assertEqual(entry["kind"],("interior","edge","corner")[edges])

    def test_cells_match_independent_scipy_halfspaces_clipped_by_geos(self):
        from shapely.geometry import Polygon
        for level,result in self.results.items():
            expected=self.independent_cells(level)
            for entry in result["pilotis"]:
                pieces=[Polygon(ring) for ring in entry["pieces"]]
                self.assert_partition(pieces,expected[entry["tag"]])
                self.assertAlmostEqual(sum(piece.area for piece in pieces),entry["area"],places=6)

    def test_unknown_level_is_rejected(self):
        with self.assertRaises(ValueError):
            self.api.tributary(self.model,level="basement")


class BimClashContract(ModelCase):
    """🚧 Native occupied material and explicit conduit hosting define the coordination result."""

    def scenario(self):
        """🚧 Route one copied conduit assembly beside an actual support for a hypothetical coordination exercise."""
        result = copy.deepcopy(self.model)
        spec = CASES["clash"]["scenario"]
        target = next(p for p in self.geometry["pilotis"] if p["tag"] == spec["column"])
        pipe = next(e for e in result["elements"] if e["tag"] == spec["pipe"])
        required = result["rules"]["clearance"][0]["required"]["value"]
        offset = target["radius"] + pipe["attributes"]["diameter"]["value"] / 2 + required * spec["margin_fraction"]
        delta=np.array([target["x"]+offset,target["y"]])-np.array(pipe["geometry"]["points"][0][:2])
        affected={pipe["tag"],pipe["tag"]+"-OUTLET"}
        for service in result["elements"]:
            if service["tag"] not in affected:
                continue
            geometry=service["geometry"]
            geometry["points"]=[[p[0]+delta[0],p[1]+delta[1],p[2]] for p in geometry["points"]]
            for name,plane in geometry.get("tube",{}).items():
                if name.startswith("clip_"):
                    p=plane["point"]
                    plane["point"]=[p[0]+delta[0],p[1]+delta[1],p[2]]
            geometry.pop("penetrations",None)
        result["relations"]=[relation for relation in result["relations"] if not (relation.get("connection",{}).get("kind")=="hosted_conduit" and (relation["relating"] in affected or set(relation["related"])&affected))]
        return result

    def distances(self, model=None):
        """🚧 Independently measure occupied material through the test-only GEOS/SciPy interface."""
        reference=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🕳️clashes")
        model=self.model if model is None else model
        rule=model["rules"]["clearance"][0]
        geometry=self.api.building_geometry(model)
        hosted={(tag,relation["relating"]) for relation in model["relations"] if relation.get("connection",{}).get("kind")=="hosted_conduit" and relation["connection"]["clearance_rule"]==rule["id"] for tag in relation["related"]}
        pipes=[item for item in geometry["services"] if item["class"]==rule["a_class"] and item["system"]==rule["a_system"]]
        def bounds(item):
            points=np.asarray([point for part in item["parts"] for point in self.api.part_vertices(part)])
            return points.min(axis=0),points.max(axis=0)
        columns=[(column,bounds(column)) for column in geometry["pilotis"]]
        out={}
        for pipe in pipes:
            lo,hi=bounds(pipe)
            candidates=sorted((float(np.linalg.norm(np.maximum(np.maximum(a-hi,lo-b),0))),column["tag"],column) for column,(a,b) in columns)
            best=math.inf
            for lower,tag,column in candidates:
                if lower>max(best,rule["required"]["value"])+1e-8:
                    continue
                pair=(pipe["tag"],tag)
                measured=dict(reference.independent_distance(pipe,column))
                measured["hosted"]=pair in hosted
                measured["required"]=rule["hosted"]["required"]["value"] if pair in hosted else rule["required"]["value"]
                out[pair]=measured
                best=min(best,measured["distance"])
        return rule,out

    def test_canonical_hosted_routes_and_hypothetical_external_clearance(self):
        """🕳️ Intentional hosted conduits remain physically separate; a copied external route violates its own rule."""
        rule,base=self.distances()
        for pair,measured in base.items():
            self.assertFalse(measured["intersects"],pair)
            if measured["hosted"]:
                self.assertGreater(measured["distance"],measured["required"],pair)
            else:
                self.assertGreaterEqual(measured["distance"],measured["required"],pair)
        self.assertEqual(self.api.clash_check(self.model),[])
        scenario=self.scenario()
        _,gaps=self.distances(scenario)
        clash=next(item for item in self.api.clash_check(scenario) if item["a_tag"]==CASES["clash"]["scenario"]["pipe"] and item["b_tag"]==CASES["clash"]["scenario"]["column"])
        independent=gaps[(clash["a_tag"],clash["b_tag"])]
        self.assertAlmostEqual(clash["distance"],independent["distance"],places=6)
        self.assertAlmostEqual(clash["distance"],rule["required"]["value"]*CASES["clash"]["scenario"]["margin_fraction"],places=6)
        self.assertFalse(clash["intersects"])
        self.assertEqual(clash["status"],"violation")
        self.assertAlmostEqual(clash["deficit"],clash["required"]-clash["distance"],places=6)
        self.assertNotIn("clearance",clash)
        self.assertEqual(self.model,self.snapshot)

    def test_clash_point_lies_between_the_pipe_and_the_piloti(self):
        """📍 The marker is the midpoint of actual occupied-material closest witnesses."""
        scenario=self.scenario()
        clash=next(item for item in self.api.clash_check(scenario) if item["a_tag"]==CASES["clash"]["scenario"]["pipe"] and item["b_tag"]==CASES["clash"]["scenario"]["column"])
        a,b=np.asarray(clash["a_point"]),np.asarray(clash["b_point"])
        np.testing.assert_allclose(clash["point"],(a+b)/2,atol=1e-6)
        self.assertAlmostEqual(float(np.linalg.norm(a-b)),clash["distance"],places=6)
        self.assertEqual(clash["level"],self.items[clash["b_tag"]]["storey"])

    def test_passed_pairs_are_listed_on_request_and_each_pipe_appears_once(self):
        rule,gaps=self.distances()
        everything=self.api.clash_check(self.model,include_passed=True)
        pipes={pipe for pipe,_ in gaps}
        self.assertEqual({item["a_tag"] for item in everything},pipes)
        self.assertEqual(len(everything),len(pipes))
        for clash in everything:
            self.assertEqual(clash["status"],"ok")
            self.assertFalse(clash["intersects"])
            nearest=min(value["distance"] for (pipe,_),value in gaps.items() if pipe==clash["a_tag"])
            self.assertAlmostEqual(clash["distance"],nearest,places=6)
            measured=gaps[(clash["a_tag"],clash["b_tag"])]
            self.assertEqual(clash["required"],measured["required"])
            self.assertAlmostEqual(float(np.linalg.norm(np.asarray(clash["a_point"])-clash["b_point"])),clash["distance"],places=6)

    def test_relaxing_the_rule_removes_the_hypothetical_violation(self):
        scenario = self.scenario()
        self.assertTrue(self.api.clash_check(scenario))
        scenario["rules"]["clearance"][0]["required"]["value"] *= CASES["clash"]["scenario"]["margin_fraction"] / 2
        self.assertEqual(self.api.clash_check(scenario), [])
        self.assertEqual(self.api.clash_check(self.model), [])


class BimConsistencyContract(ModelCase):
    """🔗 Events keep identity, raise revisions, change derived values and expose stale analyses."""

    def test_events_never_mutate_their_input_and_raise_the_revision_by_one(self):
        for name in ("sill", "paving", "paving_alt"):
            before = copy.deepcopy(self.model)
            updated = self.api.apply_event(self.event(name), before)
            self.assertEqual(before, self.model)
            self.assertEqual(updated["revision"], self.model["revision"] + 1)
            self.assertIsNot(updated, before)
            self.assertNotEqual(updated, before)

    def test_global_ids_and_structure_stay_while_values_change(self):
        for name in ("sill", "paving"):
            updated = self.api.apply_event(self.event(name), self.model)
            self.assertEqual({t: i["guid"] for t, i in registry(updated).items()}, {t: i["guid"] for t, i in self.items.items()})
            self.assertEqual(len(updated["elements"]), len(self.model["elements"]))
            self.assertEqual(updated["relations"], self.model["relations"])

    def test_changed_objects_get_the_new_revision_and_their_models_follow(self):
        updated = self.api.apply_event(self.event("paving"), self.model)
        new = updated["revision"]
        changed = {t for t,i in registry(updated).items() if i.get("metadata") != self.items[t].get("metadata")}
        self.assertEqual(changed, {"SL-R"})
        for identifier in CASES["models"]["ids"]:
            before=self.api.partial_model(identifier,self.model)
            expected=new if self.items["SL-R"]["guid"] in before["element_guids"] else before["revision"]
            self.assertEqual(self.api.partial_model(identifier, updated)["revision"], expected, identifier)
        sill = self.api.apply_event(self.event("sill"), self.model)
        changed = {t for t,i in registry(sill).items() if i.get("metadata") != self.items[t].get("metadata")}
        self.assertEqual(changed, set(CASES["events"]["sill_touches"]))

    def test_lowering_the_sill_enlarges_window_opening_and_glazing_share(self):
        updated = self.api.apply_event(self.event("sill"), self.model)
        window = self.wall("W-S")["openings"][0]
        spec = CASES["events"]["sill"]
        old, new = self.api.quantities("RW-S", self.model), self.api.quantities("RW-S", updated)
        self.assertAlmostEqual(new["area"], (window["end"] - window["start"]) * (spec["head"] - spec["sill"]))
        self.assertAlmostEqual(new["height"], spec["head"] - spec["sill"])
        self.assertGreater(new["area"], old["area"])
        before, after = self.api.quantities("W-S", self.model), self.api.quantities("W-S", updated)
        self.assertAlmostEqual(after["opening_area"], new["area"])
        self.assertAlmostEqual(after["net_side_area"], after["gross_side_area"] - new["area"])
        self.assertGreater(after["glazing_share"], before["glazing_share"])
        self.assertEqual(after["gross_side_area"], before["gross_side_area"])
        self.assertEqual(self.api.quantities("W-N", updated), self.api.quantities("W-N", self.model))

    def test_deeper_paving_raises_the_roof_build_up_load_and_every_reaction(self):
        updated = self.api.apply_event(self.event("paving"), self.model)
        spec = CASES["events"]["paving"]
        self.assertAlmostEqual(self.api.quantity("SL-R", "paving", updated), spec["value"])
        old = self.api.quantities("SL-R", self.model)["total_thickness"]
        new = self.api.quantities("SL-R", updated)["total_thickness"]
        self.assertAlmostEqual(new - old, spec["value"] - self.api.quantity("SL-R", "paving", self.model))
        before, after = self.api.tributary(self.model, level="roof"), self.api.tributary(updated, level="roof")
        self.assertAlmostEqual(after["area_load"], self.roof_load(updated))
        self.assertGreater(after["area_load"], before["area_load"])
        paving_material = next(layer["material"] for layer in self.items["LS-ROOF"]["layers"] if layer["id"] == "paving")
        paving = self.items[paving_material]["density"]["value"]
        delta = paving * (spec["value"] - self.api.quantity("SL-R", "paving", self.model))
        zones = {z["id"]: z for z in before["zones"]}
        new_zones = {z["id"]: z for z in after["zones"]}
        self.assertAlmostEqual(new_zones["roof"]["area_load"] - zones["roof"]["area_load"], delta, places=6)
        self.assertAlmostEqual(new_zones["solarium"]["area_load"] - zones["solarium"]["area_load"], delta, places=6)
        expected = self.roof_reactions(updated, step=CASES["tributary"]["step"] / 2)
        for a, b in zip(before["pilotis"], after["pilotis"]):
            with self.subTest(tag=a["tag"]):
                self.assertAlmostEqual(b["area"], a["area"])
                self.assertAlmostEqual(b["reaction"] - a["reaction"], delta * a["area"], places=6)
                self.assertAlmostEqual(b["reaction"], expected[b["tag"]], delta=0.012 * max(b["reaction"], 10))
        self.assertAlmostEqual(after["total_reaction"] - before["total_reaction"], delta * before["slab_area"], places=5)
        unchanged = [a["tag"] for a, b in zip(before["pilotis"], after["pilotis"]) if a["reaction"] == b["reaction"]]
        self.assertEqual(unchanged, [a["tag"] for a in before["pilotis"] if a["area"] == 0])
        self.assertEqual(self.api.tributary(updated, level="first")["area_load"], self.api.tributary(self.model, level="first")["area_load"])

    def test_the_base_model_is_consistent_and_a_roof_change_makes_the_analysis_link_stale(self):
        self.assertEqual(self.api.consistency_issues(self.model), [])
        updated = self.api.apply_event(self.event("paving"), self.model)
        issues = self.api.consistency_issues(updated)
        codes = {(i["code"], i["id"]) for i in issues}
        self.assertIn(("stale-link", "LK-ROOF"), codes)
        self.assertIn(("stale-value", "LK-ROOF"), codes)
        self.assertFalse({i["id"] for i in issues} - {"LK-ROOF"})
        link = next(i for i in issues if i["code"] == "stale-link")
        self.assertEqual((link["expected"], link["actual"]), (updated["revision"], self.model["revision"]))
        self.assertEqual(link["object_guid"], self.items["SL-R"]["guid"])
        stale = {i["key"]: i for i in issues if i["code"] == "stale-value"}
        self.assertIn("area_load", stale)
        self.assertIn("roof_area_load", stale)
        self.assertIn("solarium_area_load", stale)
        self.assertAlmostEqual(stale["area_load"]["expected"], self.roof_load(updated))
        self.assertAlmostEqual(stale["area_load"]["actual"], self.roof_load(self.model))

    def test_resolving_the_link_makes_the_model_consistent_again_without_mutation(self):
        updated = self.api.apply_event(self.event("paving"), self.model)
        before = copy.deepcopy(updated)
        resolved = self.api.resolve_link(updated, "LK-ROOF")
        self.assertEqual(updated, before)
        self.assertEqual(self.api.consistency_issues(resolved), [])
        self.assertEqual(resolved["revision"], updated["revision"])
        link = next(l for l in resolved["links"] if l["id"] == "LK-ROOF")
        self.assertEqual(link["source_revision"], updated["revision"])
        self.assertAlmostEqual(link["used"]["area_load"], self.roof_load(updated))
        self.assertEqual({l["id"] for l in resolved["links"]}, {l["id"] for l in updated["links"]})
        for name in ("LK-SLAB", "LK-PILOTI"):
            self.assertEqual(next(l for l in resolved["links"] if l["id"] == name), next(l for l in updated["links"] if l["id"] == name))
        with self.assertRaises(ValueError):
            self.api.resolve_link(updated, "LK-MISSING")

    def test_changes_outside_the_analysis_leave_it_current(self):
        updated = self.api.apply_event(self.event("sill"), self.model)
        self.assertEqual(self.api.consistency_issues(updated), [])

    def test_two_changes_are_independent_events_on_the_same_identity(self):
        first = self.api.apply_event(self.event("paving"), self.model)
        second = self.api.apply_event(self.event("paving_alt"), first)
        self.assertEqual(second["revision"], self.model["revision"] + 2)
        self.assertAlmostEqual(self.api.quantity("SL-R", "paving", second), CASES["events"]["paving_alt"]["value"])
        issues = {(i["code"], i["id"]) for i in self.api.consistency_issues(second)}
        self.assertIn(("stale-link", "LK-ROOF"), issues)
        self.assertEqual(self.api.consistency_issues(self.api.resolve_link(second, "LK-ROOF")), [])

    def test_unknown_objects_are_reported(self):
        broken = copy.deepcopy(self.model)
        broken["links"][0]["object_guid"] = "MISSING"
        self.assertIn("unknown-object", {i["code"] for i in self.api.consistency_issues(broken)})
        broken = copy.deepcopy(self.model)
        broken["models"][0]["element_tags"].append("MISSING")
        self.assertIn("unknown-object", {i["code"] for i in self.api.consistency_issues(broken)})

    def test_invalid_events_are_rejected(self):
        for case in CASES["invalid_events"]:
            with self.subTest(case=case["id"]):
                payload = self.event(case["base"], **case.get("set", {}))
                for key in case.get("remove", []):
                    del payload[key]
                with self.assertRaises(ValueError):
                    self.api.apply_event(payload, self.model)
        self.assertEqual(self.api.load_model(), self.snapshot)

    def test_consistency_narration_uses_model_derived_values(self):
        scene = next(n for n in ast.parse((CLIP / "🔗️consistency/🐍️.py").read_text(encoding="utf-8")).body if isinstance(n, ast.ClassDef) and n.name == "Beat6_Consistency")
        rows = next(ast.literal_eval(n.value) for n in scene.body if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in n.targets))
        for clause in rows:
            self.assertTrue(clause[1].strip() and clause[2].strip())
            if clause[0] in {"grid", "derived", "stale", "check", "retranslate", "resolve"}:
                self.assertFalse(re.search(r"\b(?:105|146|25|24)\b", " ".join(clause[1:])))
        text = (CLIP / "🔗️consistency/🐍️.py").read_text(encoding="utf-8")
        self.assertIn('self.reaction_new[tag]', text)
        self.assertIn('self.quantity(self.cell[tag]["area"], "m²", 2)', text)
        self.assertIn('self.trib_new["pilotis"]', text)


class BimClipContract(unittest.TestCase):
    """🎬 Verify six authored chapters and a complete bilingual narration path."""

    def test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene(self):
        keys = set()
        for chapter in CASES["chapters"]:
            with self.subTest(chapter=chapter["folder"]):
                path = CLIP / chapter["folder"] / "🐍️.py"
                tree = ast.parse(path.read_text(encoding="utf-8"))
                scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
                narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign)
                                 and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
                self.assertGreaterEqual(len(narration), 3)
                local_keys = [clause[0] for clause in narration]
                self.assertEqual(len(set(local_keys)), len(local_keys))
                for clause in narration:
                    self.assertEqual(len(clause), 3)
                    self.assertTrue(all(isinstance(text, str) and text.strip() for text in clause))
                    self.assertNotEqual(clause[1], clause[2])
                keys.add(chapter["scene"])
                from manim.utils.module_ops import scene_classes_from_file

                discovered = scene_classes_from_file(path, full_list=True)
                self.assertEqual([item.__name__ for item in discovered], [chapter["scene"]])
        self.assertEqual(len(keys), 6)

    def test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences(self):
        import importlib
        import os
        import tempfile
        from unittest.mock import patch
        from manim import tempconfig

        visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
        visuals._fonts.apply_body_font()
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as folder, tempconfig({"media_dir": folder}):
            for chapter in CASES["chapters"]:
                tree = ast.parse((CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8"))
                scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
                narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign)
                                 and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
                for audience in CASES["audiences"]:
                    for language_index, language in enumerate(CASES["languages"], 1):
                        with patch.dict(os.environ, SEMIO_TEACHING_LANGUAGE=language,
                                        SEMIO_TEACHING_AUDIENCE=audience["id"]):
                            for clause in narration:
                                with self.subTest(video=chapter["video_id"], audience=audience["id"],
                                                  language=language, clause=clause[0]):
                                    caption = visuals._art.caption_bar(clause[language_index])
                                    self.assertEqual(len(caption), 2 if audience["captions"] else 1)
                                    self.assertLessEqual(caption.width, visuals._engine.config.frame_width)
                                    if audience["captions"]:
                                        self.assertGreater(caption.height, 0)
                                        self.assertGreaterEqual(caption.get_bottom()[1], -visuals._engine.config.frame_height / 2)

    def test_scenes_call_only_published_model_functions_and_no_abstract_wall_remains(self):
        api = model_api()
        published = {name for name in dir(api) if not name.startswith("_")}
        sources = [path for path in CLIP.rglob("*.py") if "🧪️tests" not in path.parts and "🧱️model" not in path.parts and "__pycache__" not in path.parts]
        self.assertGreaterEqual(len(sources), 8)
        for path in sources:
            text = path.read_text(encoding="utf-8")
            with self.subTest(path=path.relative_to(CLIP).as_posix()):
                for forbidden in CASES["forbidden"]:
                    self.assertNotIn(forbidden, text)
                for node in ast.walk(ast.parse(text)):
                    if isinstance(node, ast.Attribute) and isinstance(node.value, ast.Name) and node.value.id == "_model":
                        self.assertIn(node.attr, published)

    def test_curriculum_registers_six_standalone_videos(self):
        manifest = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))
        videos = {video["id"]: video for video in manifest["videos"]}
        expected = [chapter["video_id"] for chapter in CASES["chapters"]]
        self.assertNotIn("bim", videos)
        self.assertEqual([video["id"] for video in manifest["videos"] if video["id"] in expected], expected)
        outputs = []
        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                video = videos[chapter["video_id"]]
                self.assertEqual(video["script"], chapter["entry"])
                self.assertEqual(len(video["chapters"]), 1)
                self.assertEqual(len(video["chapters"][0]["segments"]), 1)
                segment = video["chapters"][0]["segments"][0]
                self.assertEqual(segment["scene"], chapter["scene"])
                self.assertEqual((ROOT / "🎓️teaching" / segment["script"]).resolve(),
                                 (CLIP / chapter["folder"] / "🐍️.py").resolve())
                self.assertEqual(video["output"], chapter["output"])
                outputs.append(video["output"])
        self.assertEqual(len(set(outputs)), len(CASES["chapters"]))
        presentation = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json").read_text(encoding="utf-8"))
        topic = next(topic for topic in presentation["topics"] if topic["id"] == "bim")
        self.assertEqual([video["id"] for video in topic["videos"]], expected)
        for video, chapter in zip(topic["videos"], CASES["chapters"], strict=True):
            self.assertEqual(video["title"], chapter["title"])

    def test_each_video_entry_routes_only_its_own_scene(self):
        from unittest.mock import patch

        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                path = ROOT / "🎓️teaching" / chapter["entry"]
                spec = importlib.util.spec_from_file_location(chapter["video_id"], path)
                module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(module)
                self.assertEqual(module.VIDEO_ID, chapter["video_id"])
                for language in CASES["languages"]:
                    with patch.object(module._runtime, "compose_videos", return_value={}) as compose:
                        self.assertEqual(module.compose(language=language, audience="spectator", force=True), {})
                    compose.assert_called_once_with([chapter["video_id"]], language=language, chapter_scenes=None,
                                                    audience="spectator", force=True)
                    with patch.object(module._runtime, "compose_videos", return_value={}) as selected:
                        module.compose(language=language, chapters=[chapter["scene"]], audience="speaker")
                    selected.assert_called_once_with([chapter["video_id"]], language=language,
                                                     chapter_scenes=[chapter["scene"]], audience="speaker")
                with patch.object(module._runtime, "video_main", return_value=0) as main:
                    self.assertEqual(module.main(["--language", "en", "--audience", "speaker"]), 0)
                main.assert_called_once_with(chapter["video_id"], ["--language", "en", "--audience", "speaker"])

    def test_playlist_routes_all_six_videos_without_a_combined_registration(self):
        from unittest.mock import patch

        path = CLIP / "🐍️.py"
        spec = importlib.util.spec_from_file_location("bim_playlist", path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        identifiers = [chapter["video_id"] for chapter in CASES["chapters"]]
        self.assertEqual(list(module.VIDEO_IDS), identifiers)
        self.assertFalse(hasattr(module, "VIDEO_ID"))
        with patch.object(module._runtime, "compose_videos", return_value={}) as compose:
            self.assertEqual(module.compose(language="de", audience="speaker", force=True), {})
        compose.assert_called_once_with(identifiers, language="de", chapter_scenes=None, audience="speaker", force=True)


class BimLanguageContract(unittest.TestCase):
    """🌐 Require explicit content language throughout speech and render boundaries."""

    @classmethod
    def setUpClass(cls):
        import importlib

        sys.path.insert(0, str(ROOT))
        cls.runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        cls.segment = {"script": "🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py", "scene": CASES["chapters"][0]["scene"]}

    def test_bilingual_audio_has_distinct_paths_and_requires_language(self):
        import os
        import tempfile
        from unittest.mock import patch

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        script = CLIP / "📊️information/🐍️.py"
        tree = ast.parse(script.read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == self.segment["scene"])
        narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign)
                         and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
        with tempfile.TemporaryDirectory(dir=output) as folder:
            with patch.object(self.runtime._output, "speech_root", return_value=Path(folder) / "speech" / "source"):
                roots = [self.runtime.source_speech_root(script, language=language) for language in CASES["languages"]]
                self.assertNotEqual(roots[0], roots[1])
                for language, root in zip(CASES["languages"], roots):
                    directory = root / "beat_1_clauses"
                    directory.mkdir(parents=True)
                    for index in range(len(narration)):
                        (directory / f"clause_{index:02d}.mp3").write_bytes(language.encode())
                    resolved = self.runtime.source_narration(self.segment, language=language)
                    self.assertEqual(len(resolved), len(narration))
                    self.assertTrue(all(path.is_relative_to(root) for _, path in resolved))
                    self.assertTrue(all(path.read_bytes() == language.encode() for _, path in resolved))
        for call, argument in ((self.runtime.source_speech_root, script), (self.runtime.source_narration, self.segment)):
            with self.assertRaises(TypeError):
                call(argument)
            with self.assertRaises(ValueError):
                call(argument, language="fr")

    def test_render_children_receive_language_and_isolated_media(self):
        import os
        import tempfile
        from unittest.mock import patch

        settings = {"audience": "speaker", "quality": "-ql", "resolution": "320,180", "fps": 4}
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        calls = []
        with tempfile.TemporaryDirectory(dir=output) as folder:
            root = Path(folder)
            for language in CASES["languages"]:
                media = root / language / "speaker"
                with patch.object(self.runtime, "source_narration", return_value=[]), \
                        patch.object(self.runtime, "segment_state", return_value={"language": language}), \
                        patch.object(self.runtime, "render_config", return_value=root / "config.ini"), \
                        patch.object(self.runtime, "run_command", side_effect=InterruptedError("contract boundary")) as run, \
                        self.assertRaises(InterruptedError):
                    self.runtime.render_segment(self.segment, media, {**settings, "language": language},
                                                force=True, output_root=root)
                args, kwargs = run.call_args
                self.assertEqual(kwargs["env"]["SEMIO_TEACHING_LANGUAGE"], language)
                self.assertEqual(kwargs["env"]["SEMIO_TEACHING_AUDIENCE"], "speaker")
                self.assertIn(language, Path(kwargs["env"]["SEMIO_TEACHING_SPEECH_ROOT"]).parts)
                command = args[0]
                self.assertEqual(command[command.index("--media_dir") + 1], str(media.resolve()))
                calls.append(command)
        self.assertNotEqual(calls[0], calls[1])

    def test_composition_partitions_media_trace_and_settings_by_language(self):
        import os
        import tempfile
        from unittest.mock import patch

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)

        def execute(tasks, render, *, max_workers):
            for task in tasks:
                render(task, None)
            raise InterruptedError("composition contract boundary")

        with tempfile.TemporaryDirectory(dir=output) as folder:
            root = Path(folder)
            for language in CASES["languages"]:
                with patch.object(self.runtime, "_logical_state", return_value={}), \
                        patch.object(self.runtime, "_write_state") as write, \
                        patch.object(self.runtime, "reusable_clip", return_value=False), \
                        patch.object(self.runtime, "render_segment") as render, \
                        patch.object(self.runtime, "execute_jobs", side_effect=execute), \
                        self.assertRaises(InterruptedError):
                    self.runtime.compose_videos([CASES["chapters"][0]["video_id"]], language=language, audience="spectator",
                                                output_root=root / "output", media_dir=root / "media")
                self.assertEqual(render.call_count, 1)
                for call in render.call_args_list:
                    _, media, settings = call.args
                    self.assertTrue(media.is_relative_to(root / "media" / language / "spectator" / "s"))
                    self.assertEqual(settings["language"], language)
                    self.assertEqual(call.kwargs["output_root"], root / "media" / language / "spectator")
                settings_path, settings = write.call_args_list[0].args
                self.assertTrue(settings_path.is_relative_to(root / "media" / language / "spectator"))
                self.assertEqual(settings["language"], language)


class BimSelectedSourceContract(unittest.TestCase):
    """🔎 Validate selected assets while preserving global manifest containment."""

    @classmethod
    def setUpClass(cls):
        import importlib

        cls.runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")

    def test_selected_source_validation_matches_portable_cases(self):
        import os
        import tempfile
        from unittest.mock import patch

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as directory:
            root = Path(directory)
            manifest = root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
            manifest.parent.mkdir(parents=True)
            manifest.write_text(json.dumps(CASES["source_validation"]["manifest"]), encoding="utf-8")
            ready = root / CASES["source_validation"]["manifest"]["videos"][0]["script"]
            ready.parent.mkdir(parents=True)
            ready.write_text("class Beat1_Ready:\n    NARRATION = [('a', 'A', 'A'), ('b', 'B', 'B')]\n", encoding="utf-8")
            with patch.object(self.runtime, "teaching_root", return_value=root):
                for case in CASES["source_validation"]["selections"]:
                    with self.subTest(case=case["id"]):
                        if case["accepted"]:
                            self.assertEqual(len(self.runtime.load_manifest(source_ids=case["source_ids"])), 2)
                        else:
                            with self.assertRaises(ValueError):
                                self.runtime.load_manifest(source_ids=case["source_ids"])

                for selection in CASES["source_validation"]["compose_all"]:
                    with self.subTest(compose_all=selection), self.assertRaisesRegex(ValueError, "Invalid lesson entry: missing"):
                        self.runtime.compose_videos(selection, language="en", audience="speaker")

                data = copy.deepcopy(CASES["source_validation"]["manifest"])
                data["videos"][0]["chapters"][0]["segments"][0]["script"] = CASES["source_validation"]["missing_segment"]
                manifest.write_text(json.dumps(data), encoding="utf-8")
                with self.assertRaisesRegex(ValueError, "Invalid curriculum segment"):
                    self.runtime.load_manifest(source_ids=["ready"])

    def test_unselected_paths_and_clause_structure_remain_strict(self):
        import os
        import tempfile
        from unittest.mock import patch

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as directory:
            root = Path(directory)
            manifest = root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
            manifest.parent.mkdir(parents=True)
            for case in CASES["source_validation"]["invalid_structure"]:
                data = copy.deepcopy(CASES["source_validation"]["manifest"])
                parent = data
                for key in case["path"][:-1]:
                    parent = parent[key]
                parent[case["path"][-1]] = case["value"]
                manifest.write_text(json.dumps(data), encoding="utf-8")
                with self.subTest(case=case["id"]), patch.object(self.runtime, "teaching_root", return_value=root):
                    with self.assertRaises(ValueError):
                        self.runtime.load_manifest(source_ids=[])

    def test_selected_clause_range_requires_the_declared_source_keys(self):
        import os
        import tempfile
        from unittest.mock import patch

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as directory:
            root = Path(directory)
            manifest = root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
            manifest.parent.mkdir(parents=True)
            data = copy.deepcopy(CASES["source_validation"]["manifest"])
            data["videos"][0]["chapters"][0]["segments"][0]["clauses"] = {"first": "missing", "last": "b"}
            manifest.write_text(json.dumps(data), encoding="utf-8")
            ready = root / CASES["source_validation"]["manifest"]["videos"][0]["script"]
            ready.parent.mkdir(parents=True)
            ready.write_text("class Beat1_Ready:\n    NARRATION = [('a', 'A', 'A'), ('b', 'B', 'B')]\n", encoding="utf-8")
            with patch.object(self.runtime, "teaching_root", return_value=root):
                self.assertEqual(len(self.runtime.load_manifest(source_ids=[])), 2)
                with self.assertRaises(ValueError):
                    self.runtime.load_manifest(source_ids=["ready"])

    def test_cli_metadata_load_does_not_require_unselected_source_files(self):
        import importlib
        from unittest.mock import patch

        renderer = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🖌️render.🐍️")
        videos = CASES["source_validation"]["manifest"]["videos"]
        with patch.object(renderer, "load_manifest", return_value=videos) as load, \
                patch.object(renderer, "compose_videos", return_value={}) as compose:
            self.assertEqual(renderer.main(["--language", "en", "--video", "ready", "--audience", "speaker", "--no-play"]), 0)
        load.assert_called_once_with(source_ids=[])
        self.assertEqual(compose.call_args.args[0], ["ready"])


class BimBrowserMetadataContract(unittest.TestCase):
    """🌐 Discover browser sources without loading or provisioning browser tools."""

    def test_source_discovery_skips_browser_runtime(self):
        import importlib

        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        runtime.run_command(["repo", "tools", "exec", "--light", "node", "--", str(CLIP / "🧪️tests/🟨️metadata.mjs")], cwd=ROOT)


def load_tests(loader, tests, pattern):
    """🧩 Include every co-located remodelling contract in the standard BIM test target."""
    import inspect

    prefix = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
    suite = unittest.TestSuite(tests)
    for module in ("🧱️model.🧮️analysis.🧪️tests.🐍️", "🧱️model.🧪️tests.🚗️parking", "🧪️tests.🧩️chapter-source.🐍️",
                   "🔌️exchange.🧪️tests.🐍️slab", "🎨️visuals.🏛️savoye.🧊️axon.🧪️tests", "🎨️visuals.🏛️savoye.🧊️axon.📏️geometry",
                   "🎨️visuals.🏛️savoye.🧱️core.🧪️surfaces", "🎨️visuals.🧮️analysis.🧪️profiles.🐍️", "🧱️model.🧪️tests.📍️placements", "🧱️model.🧪️tests.⚡️slabs", "🧱️model.🧪️tests.⚡️partitions", "🧱️model.🧪️tests.🏗️beams", "🧱️model.🧪️tests.🏗️joints", "🧱️model.🧪️tests.🏗️bearing", "🧱️model.🧪️tests.🔺️frustums", "🧱️model.🧪️tests.🪚️ownership", "🧱️model.🧪️tests.🕳️openings", "🧱️model.🧪️tests.🏗️services", "🧱️model.🧪️tests.🚰️services", "🧱️model.🧪️tests.🧊️sections", "🧱️model.🧪️tests.☀️rooflights", "🧱️model.🧪️tests.🌬️garden", "🧱️model.🧪️tests.🛝️ramps", "🧱️model.🧪️tests.🪟️curtains", "🧱️model.🧮️analysis.🧪️tests.🕳️clashes", "🧱️model.🧪️tests.🕳️profiles", "🧱️model.🧮️analysis.🧪️tests.🕳️passages", "🧱️model.🔌️exchange.🧪️tests.🪟️fillings", "🧱️model.🔌️exchange.🧪️tests.🧵spans", "🧱️model.🔌️exchange.🧪️tests.🪑furnishings"):
        suite.addTests(loader.loadTestsFromName(prefix + module))
    for module in ("🧱️model.🧪️tests.🏛️remodel", "🧱️model.🧪️tests.🛁️details", "🧱️model.🧪️tests.📐️representations",
                   "🧱️model.🧪️tests.🕳️cellar", "🧱️model.🧪️tests.🏗️foundations", "🧱️model.🧪️tests.🏛️architecture", "🧱️model.🧪️tests.🏛️suite", "🧱️model.🧪️tests.🪨️regions",
                   "🧱️model.🧪️tests.🧱️walls", "🧱️model.🧪️tests.🪟️fillings", "🧱️model.🧪️tests.🪚️joinery", "🧱️model.🧪️tests.🚪️garage", "🧱️model.🧪️tests.🚪️roof", "🧱️model.🧪️tests.🏠ground", "🧱️model.🧪️tests.🚬️chimneys", "🧱️model.🧪️tests.🚬️prisms", "🧱️model.🧪️tests.🪜️stair-parts", "🧱️model.🧪️tests.🪜️stairs", "🧱️model.🧪️tests.🕸️mesh-clipping", "🧱️model.🧪️tests.🌀️wall-routing", "🧱️model.🧪️tests.🏗️facade"):
        source = importlib.import_module(prefix + module)
        suite.addTests(unittest.FunctionTestCase(fn) for name, fn in inspect.getmembers(source, inspect.isfunction) if name.startswith("test_"))
    for module in ("🧱️model.🧪️tests.📏️columns", "🧱️model.🧪️tests.🚘️profiles"):
        source = importlib.import_module(prefix + module)
        suite.addTest(unittest.FunctionTestCase(lambda source=source: source.run(ROOT)))
    suite.addTests(loader.discover(str(CLIP / "🎨️visuals/🏛️savoye/🧪️ortho-remodel"), pattern="test_*.py"))
    return suite


if __name__ == "__main__":
    unittest.main()
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                                                                                                                                                                        
                                      
    