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
from pathlib import Path

import numpy as np

sys.dont_write_bytecode = True
CLIP = Path(__file__).resolve().parent.parent
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
ROOT = next(parent for parent in CLIP.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
ALPHABET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$"
GLOBAL_ID = re.compile(r"^[0-3][0-9A-Za-z_$]{21}$")
SOURCE_ID = re.compile(r"^VS-\d+(\.\d+)*$")


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


def inside(polygon, points):
    """📍 Test points against a polygon by vectorised ray casting."""
    p, q = np.asarray(polygon, dtype=float), np.asarray(points, dtype=float).reshape(-1, 2)
    result = np.zeros(len(q), dtype=bool)
    for (x0, y0), (x1, y1) in zip(p, np.roll(p, -1, axis=0)):
        crosses = (y0 > q[:, 1]) != (y1 > q[:, 1])
        with np.errstate(divide="ignore", invalid="ignore"):
            cut = x0 + (q[:, 1] - y0) * (x1 - x0) / (y1 - y0)
        result ^= crosses & (q[:, 0] < cut)
    return result


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

    def roof_zones(self, model):
        geometry = self.api.building_geometry(model)
        roof = next(slab for slab in geometry["slabs"] if slab["kind"] == "ROOF")
        imposed = model["loads"]["imposed"]
        load = lambda zone, extra: sum(l["thickness"] * l["density"] for l in roof["layers"] if not l["zones"] or zone in l["zones"]) + extra
        return roof, {"solarium": load("solarium", imposed["solarium"]["value"]), "planted": load("planted", imposed["roof_planted"]["value"])}

    def roof_load(self, model):
        roof, loads = self.roof_zones(model)
        solarium = shoelace(roof["zones"]["solarium"])
        net = self.api.quantities(roof["tag"], model)["area"]
        return (loads["solarium"] * solarium + loads["planted"] * (net - solarium)) / net

    def roof_reactions(self, model, step=0.025):
        from scipy.spatial import cKDTree

        roof, loads = self.roof_zones(model)
        geometry = self.api.building_geometry(model)
        points = sample_surface(roof["outline"], roof["voids"], step)
        nearest = cKDTree(np.array([[p["x"], p["y"]] for p in geometry["pilotis"]])).query(points)[1]
        solarium = inside(roof["zones"]["solarium"], points)
        cell = step * step
        return {pil["tag"]: float(((nearest == i) & solarium).sum() * cell * loads["solarium"] + ((nearest == i) & ~solarium).sum() * cell * loads["planted"])
                for i, pil in enumerate(geometry["pilotis"])}

    def floor_load(self, model):
        floor = next(slab for slab in self.api.building_geometry(model)["slabs"] if slab["kind"] == "FLOOR")
        concrete = registry(model)["MAT-RC"]["density"]["value"]
        return floor["thickness"] * concrete + model["loads"]["finishes"]["first"]["value"] + model["loads"]["imposed"]["first"]["value"]


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
            self.assertEqual(box[name]["basis"], "inferred")
        grid = self.model["grid"]
        self.assertEqual((grid["spacing"]["value"], grid["spacing"]["basis"]), (sourced["spacing"], "sourced"))
        self.assertEqual((grid["initial_spacing"]["value"], grid["initial_spacing"]["basis"]), (sourced["initial_spacing"], "sourced"))
        years = self.model["building"]["years"]
        for name, year in sourced["years"].items():
            self.assertEqual(years[name], year)
        self.assertEqual(self.model["building"]["architects"], sourced["architects"])
        self.assertEqual(self.model["site"]["name"]["en"], sourced["site"])
        for tag, area in (("SP-TERRACE", sourced["roof_garden_area"]), ("SP-SOLARIUM", sourced["solarium_area"])):
            reference = self.items[tag]["reference_area"]
            self.assertEqual((reference["value"], reference["basis"], reference["unit"]), (area, "sourced", "m²"))
            self.assertLessEqual(abs(self.api.quantities(tag, self.model)["area"] / area - 1), sourced["area_tolerance"])

    def test_assumed_values_are_plausible_for_a_villa_of_1929(self):
        diameter = self.items[CASES["tags"]["piloti_type"]]["profile"]["diameter"]
        self.assertEqual(diameter["basis"], "assumed")
        self.assertTrue(0.2 <= diameter["value"] <= 0.6)
        levels = [storey["elevation"]["value"] for storey in self.model["storeys"]]
        self.assertEqual(levels[0], 0)
        self.assertEqual(levels, sorted(levels))
        self.assertTrue(3.0 <= levels[1] <= 4.5)
        self.assertLess(levels[-1], self.model["building"]["box"]["height"]["value"])
        for tag in CASES["tags"]["slabs"]:
            slab = self.slab(tag)
            self.assertTrue(0.1 <= slab["thickness"] <= 0.5)
        self.assertTrue(1.0 <= self.model["car"]["width"]["value"] <= 2.2)
        self.assertTrue(3.0 <= self.model["car"]["length"]["value"] <= 6.0)

    def test_every_element_is_complete_and_references_exist(self):
        required = {"guid", "tag", "class", "predefined_type", "name", "storey", "geometry", "attributes", "psets", "metadata"}
        domains = set(CASES["model"]["domains"])
        for element in self.model["elements"]:
            with self.subTest(tag=element["tag"]):
                self.assertTrue(required <= set(element))
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

    def test_opening_and_window_chain_is_authored_for_every_wall(self):
        openings = {e["tag"]: e for e in self.model["elements"] if e["class"] == "IfcOpeningElement"}
        windows = {e["tag"]: e for e in self.model["elements"] if e["class"] == "IfcWindow"}
        walls = {e["tag"]: e for e in self.model["elements"] if e["class"] == "IfcWall" and e["predefined_type"] == "STANDARD"}
        self.assertEqual(sorted(walls), sorted(CASES["tags"]["walls"]))
        self.assertEqual(sorted(openings), sorted(CASES["tags"]["openings"]))
        self.assertEqual(sorted(windows), sorted(CASES["tags"]["windows"]))
        for tag, opening in openings.items():
            self.assertIn(opening["geometry"]["wall"], walls)
        self.assertEqual(sorted(w["geometry"]["opening"] for w in windows.values()), sorted(openings))
        self.assertEqual(len({o["geometry"]["wall"] for o in openings.values()}), len(openings))

    def test_class_registry_reaches_ifc_root_for_every_used_class(self):
        classes = self.model["classes"]
        used = {item["class"] for item in self.model["elements"]} | {"IfcSpace", "IfcBuildingStorey", "IfcProject", "IfcSite", "IfcBuilding"}
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
        column = self.api.element("B3", self.model)
        self.assertEqual(column["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"], True)
        self.assertEqual(column["material"]["tag"], "MAT-RC")
        self.assertEqual(column["guid"], oracle_global_id("B3"))
        column["psets"].clear()
        column["name"]["en"] = "changed"
        self.assertEqual(self.api.element("B3", self.model)["name"]["en"], self.items["B3"]["name"]["en"])
        self.assertTrue(self.api.element("SL-R", self.model)["material"]["layers"])


class BimGridContract(ModelCase):
    """🏛️ The structural grid and the 25 pilotis follow the sourced 4.75 m spacing."""

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

    def test_twenty_five_pilotis_sit_on_the_grid_inside_the_box(self):
        pilotis, grid, box = self.geometry["pilotis"], self.model["grid"], self.geometry["box"]
        self.assertEqual(len(pilotis), CASES["grid"]["pilotis"])
        self.assertEqual({p["tag"] for p in pilotis}, {row + column for row in CASES["grid"]["rows"] for column in CASES["grid"]["columns"]})
        radius = self.piloti_radius()
        for pil in pilotis:
            with self.subTest(tag=pil["tag"]):
                row, column = pil["tag"][0], pil["tag"][1:]
                self.assertAlmostEqual(pil["x"], grid["x"][grid["labels_x"].index(column)])
                self.assertAlmostEqual(pil["y"], grid["y"][grid["labels_y"].index(row)])
                self.assertAlmostEqual(pil["radius"], radius)
                self.assertTrue(0 <= pil["x"] <= box["length"] and 0 <= pil["y"] <= box["width"])
                self.assertGreaterEqual(pil["x"] - radius, -1e-9)
                self.assertLessEqual(pil["x"] + radius, box["length"] + 1e-9)
                self.assertEqual(pil["base"], 0)
                self.assertEqual(pil["guid"], oracle_global_id(pil["tag"]))
        points = np.array([[p["x"], p["y"]] for p in pilotis])
        self.assertEqual(len({tuple(p) for p in points.tolist()}), 25)
        distances = np.linalg.norm(points[:, None, :] - points[None, :, :], axis=2) + np.eye(25) * 99
        self.assertGreaterEqual(distances.min(), grid["spacing"]["value"] - 1e-9)

    def test_pilotis_carry_the_first_floor_slab_from_ground_to_soffit(self):
        floor = self.slab(CASES["tags"]["slabs"][0])
        for pil in self.geometry["pilotis"]:
            self.assertAlmostEqual(pil["top"], floor["top"] - floor["thickness"])
        self.assertEqual(self.geometry["storeys"]["first"], floor["top"])

    def test_piloti_type_has_twenty_five_occurrences_in_grid_order(self):
        tags = self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model)
        self.assertEqual(tags, [row + column for row in CASES["grid"]["rows"] for column in CASES["grid"]["columns"]])
        for key in ("A1", "MISSING"):
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.api.type_occurrences(key, self.model)


class BimGeometryContract(ModelCase):
    """🗺️ Drawings read one consistent building geometry; nothing floats outside the box."""

    def test_box_and_storeys_come_from_the_authored_values(self):
        box = self.geometry["box"]
        self.assertEqual((box["length"], box["width"], box["height"]), tuple(CASES["sourced"]["box"][k] for k in ("length", "width", "height")))
        self.assertEqual(self.geometry["storeys"], {s["id"]: s["elevation"]["value"] for s in self.model["storeys"]})

    def test_slabs_are_polygons_with_voids_inside_the_outline(self):
        for tag in CASES["tags"]["slabs"]:
            slab = self.slab(tag)
            with self.subTest(tag=tag):
                self.assertAlmostEqual(shoelace(slab["outline"]), self.geometry["box"]["length"] * self.geometry["box"]["width"])
                self.assertGreater(slab["thickness"], 0)
                self.assertTrue(slab["voids"])
                for void in slab["voids"]:
                    low, high = np.asarray(void).min(axis=0), np.asarray(void).max(axis=0)
                    self.assertTrue((low >= 0).all() and high[0] <= self.geometry["box"]["length"] and high[1] <= self.geometry["box"]["width"])
                    self.assertGreater(shoelace(void), 0)
                    self.assertTrue(inside(slab["outline"], np.asarray(void).mean(axis=0)).all())
        voids = [np.asarray(v) for slab in self.geometry["slabs"] for v in slab["voids"]]
        self.assertGreaterEqual(len(voids), 2)

    def test_slab_levels_match_storeys(self):
        floor, roof = self.slab("SL-1"), self.slab("SL-R")
        self.assertEqual(floor["kind"], "FLOOR")
        self.assertEqual(roof["kind"], "ROOF")
        self.assertEqual(floor["top"], self.geometry["storeys"]["first"])
        self.assertEqual(roof["top"], self.geometry["storeys"]["roof"])
        self.assertAlmostEqual(roof["total_thickness"], sum(layer["thickness"] for layer in roof["layers"]))
        self.assertEqual([layer["id"] for layer in roof["layers"]][0], "concrete")
        self.assertIn("soil", [layer["id"] for layer in roof["layers"]])

    def test_walls_lie_inside_the_box_and_openings_inside_the_walls(self):
        box, floor = self.geometry["box"], self.geometry["storeys"]["first"]
        self.assertEqual([w["tag"] for w in self.geometry["walls"]], CASES["tags"]["walls"])
        for wall in self.geometry["walls"]:
            with self.subTest(tag=wall["tag"]):
                start, end = np.asarray(wall["start"]), np.asarray(wall["end"])
                length = float(np.linalg.norm(end - start))
                self.assertAlmostEqual(length, wall["length"])
                for point in (start, end):
                    self.assertTrue(-1e-9 <= point[0] <= box["length"] + 1e-9 and -1e-9 <= point[1] <= box["width"] + 1e-9)
                self.assertGreater(wall["thickness"], 0)
                self.assertEqual(wall["base"], floor)
                self.assertGreater(wall["top"], wall["base"])
                self.assertLessEqual(wall["top"], self.geometry["storeys"]["roof"] + 1e-9)
                self.assertEqual(len(wall["openings"]), 1)
                for opening in wall["openings"]:
                    self.assertTrue(0 <= opening["start"] < opening["end"] <= length)
                    self.assertTrue(0 <= opening["sill"] < opening["head"] <= wall["top"] - wall["base"])
                    self.assertEqual(opening["guid"], oracle_global_id(opening["tag"]))
                    self.assertEqual(opening["window_guid"], oracle_global_id(opening["window_tag"]))
                    self.assertNotEqual(opening["guid"], opening["window_guid"])

    def test_curtain_wall_follows_the_turning_circle_of_the_assumed_car(self):
        wall, car = self.geometry["curtain_wall"], self.model["car"]
        path, centre = np.asarray(wall["path"]), np.asarray(wall["centre"])
        radius = car["turning_radius"]["value"] - car["width"]["value"] - car["wall_clearance"]["value"]
        self.assertAlmostEqual(wall["radius"], radius)
        arc = path[1:-1]
        self.assertTrue(np.allclose(np.linalg.norm(arc - centre, axis=1), radius, atol=1e-5))
        angles = np.unwrap(np.arctan2(arc[:, 1] - centre[1], arc[:, 0] - centre[0]))
        self.assertAlmostEqual(abs(angles[-1] - angles[0]), math.pi, places=5)
        self.assertTrue(np.all(np.diff(angles) < 0))
        self.assertAlmostEqual(path[0][0], centre[0] - radius)
        self.assertAlmostEqual(path[-1][0], centre[0] + radius)
        self.assertAlmostEqual(path[0][1], path[-1][1])
        self.assertEqual(wall["base"], 0)
        self.assertAlmostEqual(wall["top"], self.slab("SL-1")["top"] - self.slab("SL-1")["thickness"])
        self.assertTrue(path[:, 0].min() >= 0 and path[:, 0].max() <= self.geometry["box"]["length"])
        self.assertLess(radius, car["turning_radius"]["value"] - car["width"]["value"])

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
        box, top = self.geometry["box"], self.geometry["box"]["height"]
        screen = self.geometry["screens"][0]
        path = np.asarray(screen["path"])
        self.assertEqual(screen["base"], self.geometry["storeys"]["roof"])
        self.assertAlmostEqual(screen["top"], box["height"])
        self.assertTrue((path[:, 0] >= 0).all() and (path[:, 0] <= box["length"]).all() and (path[:, 1] >= 0).all() and (path[:, 1] <= box["width"]).all())
        services = {s["tag"]: s for s in self.geometry["services"]}
        self.assertEqual({s["kind"] for s in services.values()}, {"boiler", "radiator", "drain", "downpipe", "pipe"})
        for k in (1, 2, 3, 4):
            drain, pipe = services[f"DR-{k}"], services[f"DP-{k}"]
            self.assertEqual(drain["points"][0][:2], pipe["points"][0][:2])
            self.assertEqual(pipe["points"][0][:2], pipe["points"][1][:2])
            self.assertAlmostEqual(pipe["points"][0][2], self.geometry["storeys"]["roof"])
            self.assertEqual(pipe["points"][1][2], 0)
        for service in services.values():
            for point in service["points"]:
                self.assertTrue(0 <= point[0] <= box["length"] and 0 <= point[1] <= box["width"] and 0 <= point[2] <= top)

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


class BimClearanceLimits(ModelCase):
    """🚧 The straight-through threshold and the closest approaches of the assumed car, checked against bisection and NumPy."""

    def test_limits_match_a_bisection_of_the_clearance_function(self):
        low, high = 0.0, 10.0
        for _ in range(40):
            mid = (low + high) / 2
            low, high = (low, mid) if self.api.clearance(self.model, spacing=mid)["fits"] else (mid, high)
        limits = self.api.clearance_limits(self.model)
        self.assertAlmostEqual(limits["min_spacing"], high, places=6)
        self.assertGreater(limits["spacing"], limits["min_spacing"])
        self.assertTrue(limits["fits"])
        self.assertAlmostEqual(limits["clear_width"], limits["spacing"] - limits["diameter"], places=9)

    def test_limits_follow_diameter_and_car_width_overrides(self):
        margin = self.model["car"]["min_side_margin"]["value"]
        wide = self.api.clearance_limits(self.model, diameter=0.6, car_width=2.0)
        self.assertAlmostEqual(wide["min_spacing"], 2.0 + 2 * margin + 0.6, places=9)
        self.assertTrue(wide["fits"])
        self.assertFalse(self.api.clearance_limits(self.model, spacing=wide["min_spacing"] - 0.01, diameter=0.6, car_width=2.0)["fits"])

    def test_pinches_equal_independent_rectangle_to_circle_distances(self):
        car = self.model["car"]
        half = np.array([car["length"]["value"], car["width"]["value"]]) / 2
        poses = self.api.car_path(self.model, samples=1201)["poses"]

        def distance(pose, piloti):
            x, y, heading = pose
            c, s = math.cos(math.radians(heading)), math.sin(math.radians(heading))
            dx, dy = piloti["x"] - x, piloti["y"] - y
            local = np.array([dx * c + dy * s, -dx * s + dy * c])
            return float(np.linalg.norm(np.maximum(np.abs(local) - half, 0))) - piloti["radius"]

        expected = sorted((min(distance(pose, q) for pose in poses), q["tag"]) for q in self.geometry["pilotis"])[:2]
        found = self.api.path_pinches(self.model, count=2)
        self.assertEqual([item["tag"] for item in found], [tag for _, tag in expected])
        for item, (value, _) in zip(found, expected):
            self.assertAlmostEqual(item["clearance"], value, places=3)
        self.assertAlmostEqual(found[0]["clearance"], self.api.clearance(self.model)["min_path_clearance"], places=3)


class BimInformationContract(ModelCase):
    """📊 Raw values become information: unit, basis, object identity, language and meaning."""

    def test_information_requires_an_explicit_known_language(self):
        for args in (("B3", "diameter", "fr"), ("B3", "diameter", ""), ("B3", "unknown-property", "en"), ("MISSING", "diameter", "en")):
            with self.subTest(args=args), self.assertRaises(ValueError):
                self.api.information(*args, model=self.model)
        with self.assertRaises(TypeError):
            self.api.information("B3", "diameter")

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
        spacing = self.api.information("B3", "grid.spacing", "en", model=self.model)
        self.assertEqual(spacing["value"], self.model["grid"]["spacing"]["value"])
        diameter = self.api.information("B3", "diameter", "de", model=self.model)
        self.assertEqual(diameter["value"], self.piloti_radius() * 2)
        volume = self.api.information("B3", "volume", "en", model=self.model)
        self.assertAlmostEqual(volume["value"], math.pi * self.piloti_radius() ** 2 * self.geometry["pilotis"][0]["top"], places=6)
        self.assertEqual(volume["unit"], "m³")
        self.assertEqual(self.api.information("B3", "GlobalId", "en", model=self.model)["value"], oracle_global_id("B3"))
        self.assertEqual(self.api.information("B3", "material", "en", model=self.model)["value"], self.items["MAT-RC"]["name"]["en"])
        self.assertEqual(self.api.information("B3", "material", "de", model=self.model)["value"], self.items["MAT-RC"]["name"]["de"])

    def test_quantity_reads_authored_derived_and_layer_values(self):
        slab = self.items["SL-1"]["attributes"]["thickness"]["value"]
        self.assertEqual(self.api.quantity("SL-1", "thickness", self.model), slab)
        self.assertEqual(self.api.quantity(self.items["SL-1"], "thickness", self.model), slab)
        self.assertEqual(self.api.quantity("SL-R", "soil", self.model), next(l for l in self.items["LS-ROOF"]["layers"] if l["id"] == "soil")["thickness"]["value"])
        self.assertAlmostEqual(self.api.quantity("B3", "volume", self.model), self.api.quantities("B3", self.model)["volume"])
        with self.assertRaises(ValueError):
            self.api.quantity("B3", "unknown", self.model)


class BimClearanceContract(ModelCase):
    """🚗 The car question: clear width, margin, curved wall and swept path."""

    def test_clear_width_and_margin_follow_the_spacing_formula(self):
        result, car = self.api.clearance(self.model), self.model["car"]
        spacing, diameter = self.model["grid"]["spacing"]["value"], self.piloti_radius() * 2
        self.assertAlmostEqual(result["clear_width"], spacing - diameter)
        self.assertAlmostEqual(result["clear_width_initial"], self.model["grid"]["initial_spacing"]["value"] - diameter)
        self.assertAlmostEqual(result["margin"], (spacing - diameter - car["width"]["value"]) / 2)
        self.assertEqual(result["car_width"], car["width"]["value"])
        self.assertEqual(result["min_margin"], car["min_side_margin"]["value"])
        self.assertAlmostEqual(result["curve_radius"], car["turning_radius"]["value"] - car["width"]["value"] - car["wall_clearance"]["value"])
        self.assertIs(result["fits"], True)
        self.assertGreater(result["spacing"], result["car_width"] + diameter)

    def test_fit_decision_flips_exactly_at_the_required_margin(self):
        car, diameter = self.model["car"], self.piloti_radius() * 2
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
        path, car = self.api.car_path(self.model), self.model["car"]
        centres = np.array([[p["x"], p["y"]] for p in self.geometry["pilotis"]])
        gaps = np.array([rectangle_circle_gap(pose, car["length"]["value"], car["width"]["value"], centres, self.piloti_radius()).min() for pose in path["poses"]])
        self.assertGreater(gaps.min(), 0)
        self.assertLessEqual(path["min_clearance"], gaps.min() + 0.05)
        self.assertGreater(path["min_clearance"], 0)
        self.assertEqual(self.api.clearance(self.model)["min_path_clearance"], path["min_clearance"])
        self.assertEqual(len(path["poses"]), 120)
        poses = np.asarray(path["poses"])
        self.assertAlmostEqual(path["length"], float(np.linalg.norm(np.diff(poses[:, :2], axis=0), axis=1).sum()), delta=0.05 * path["length"])
        first, second = path["entry_pair"]
        self.assertEqual(first[0], second[0])
        self.assertEqual(int(second[1:]) - int(first[1:]), 1)
        self.assertAlmostEqual(path["gap"]["end"][0] - path["gap"]["start"][0], self.api.clearance(self.model)["clear_width"])

    def test_path_curve_is_concentric_with_the_glass_wall(self):
        path, wall = self.api.car_path(self.model), self.geometry["curtain_wall"]
        self.assertEqual(path["curve_centre"], wall["centre"])
        self.assertAlmostEqual(path["centre_radius"], wall["radius"] + self.model["car"]["wall_clearance"]["value"] + self.model["car"]["width"]["value"] / 2)


class BimQuantityContract(ModelCase):
    """🧮 Derived quantities are recomputed with independent formulas."""

    def test_piloti_volume_is_pi_r_squared_times_height(self):
        from scipy import integrate

        radius, height = self.piloti_radius(), self.geometry["pilotis"][0]["top"]
        area, _ = integrate.quad(lambda rho: 2 * math.pi * rho, 0, radius)
        for tag in ("A1", "B3", "E5"):
            result = self.api.quantities(tag, self.model)
            self.assertAlmostEqual(result["cross_section_area"], math.pi * radius ** 2, places=7)
            self.assertAlmostEqual(result["cross_section_area"], area, places=7)
            self.assertAlmostEqual(result["height"], height)
            self.assertAlmostEqual(result["volume"], area * height, places=6)
            self.assertAlmostEqual(result["outer_surface_area"], 2 * math.pi * radius * height, places=6)

    def test_type_total_is_the_sum_over_all_occurrences(self):
        total = self.api.quantities(CASES["tags"]["piloti_type"], self.model)
        each = self.api.quantities("A1", self.model)["volume"]
        self.assertEqual(total["count"], 25)
        self.assertAlmostEqual(total["volume"], 25 * each, places=6)
        self.assertAlmostEqual(total["volume"], sum(self.api.quantities(tag, self.model)["volume"] for tag in total["occurrences"]), places=6)
        self.assertEqual(total["occurrences"], self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model))

    def test_wall_gross_opening_and_net_areas_and_glazing_share(self):
        for tag in CASES["tags"]["walls"]:
            wall, result = self.wall(tag), self.api.quantities(tag, self.model)
            with self.subTest(tag=tag):
                height = wall["top"] - wall["base"]
                gross = wall["length"] * height
                opening = sum((o["end"] - o["start"]) * (o["head"] - o["sill"]) for o in wall["openings"])
                self.assertAlmostEqual(result["gross_side_area"], gross)
                self.assertAlmostEqual(result["opening_area"], opening)
                self.assertAlmostEqual(result["net_side_area"], gross - opening)
                self.assertAlmostEqual(result["glazing_share"], opening / gross)
                self.assertAlmostEqual(result["volume"], (gross - opening) * wall["thickness"], places=6)
                self.assertAlmostEqual(result["gross_volume"], gross * wall["thickness"], places=6)
                layers = self.items[self.items[CASES["tags"]["wall_type"]]["material"]]["layers"]
                self.assertAlmostEqual(wall["thickness"], sum(layer["thickness"]["value"] for layer in layers))
                self.assertTrue(0 < result["glazing_share"] < 1)

    def test_window_and_opening_quantities_agree(self):
        for opening_tag, window_tag in zip(CASES["tags"]["openings"], CASES["tags"]["windows"]):
            first, second = self.api.quantities(opening_tag, self.model), self.api.quantities(window_tag, self.model)
            self.assertEqual(first, second)
            self.assertAlmostEqual(first["area"], first["width"] * first["height"])
            self.assertAlmostEqual(first["height"], first["head"] - first["sill"])

    def test_slab_areas_and_volumes_use_polygon_areas_minus_voids(self):
        for tag in CASES["tags"]["slabs"]:
            slab, result = self.slab(tag), self.api.quantities(tag, self.model)
            with self.subTest(tag=tag):
                gross = shoelace(slab["outline"])
                voids = sum(shoelace(v) for v in slab["voids"])
                self.assertAlmostEqual(result["gross_area"], gross)
                self.assertAlmostEqual(result["void_area"], voids)
                self.assertAlmostEqual(result["area"], gross - voids)
                self.assertAlmostEqual(result["volume"], (gross - voids) * slab["thickness"], places=5)
                self.assertAlmostEqual(result["perimeter"], float(np.linalg.norm(np.roll(slab["outline"], -1, axis=0) - np.asarray(slab["outline"]), axis=1).sum()))
        sample = len(sample_surface(self.slab("SL-1")["outline"], self.slab("SL-1")["voids"], 0.05)) * 0.05 ** 2
        self.assertAlmostEqual(self.api.quantities("SL-1", self.model)["area"], sample, delta=0.3)

    def test_roof_build_up_total_is_the_sum_of_its_layers(self):
        result, layers = self.api.quantities("SL-R", self.model), self.items["LS-ROOF"]["layers"]
        self.assertAlmostEqual(result["total_thickness"], sum(layer["thickness"]["value"] for layer in layers))
        self.assertAlmostEqual(result["thickness"], layers[0]["thickness"]["value"])

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
        self.assertAlmostEqual(glass["length"], float(np.linalg.norm(np.diff(np.asarray(wall["path"]), axis=0), axis=1).sum()), delta=0.03 * glass["length"])
        pipe = self.api.quantities("DP-1", self.model)
        self.assertAlmostEqual(pipe["length"], self.geometry["storeys"]["roof"])
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

    def test_twenty_five_pilotis_are_defined_by_the_piloti_type(self):
        link = self.relations(CASES["tags"]["piloti_type"], "IfcRelDefinesByType")
        self.assertEqual(len(link), 1)
        self.assertEqual([x["tag"] for x in link[0]["related"]], self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model))
        self.assertEqual(len(link[0]["related"]), 25)
        for tag in ("A1", "E5"):
            own = self.relations(tag, "IfcRelDefinesByType")
            self.assertEqual(own[0]["relating"]["tag"], CASES["tags"]["piloti_type"])

    def test_each_ribbon_window_is_a_void_fill_chain_in_its_wall(self):
        for wall, opening, window in zip(CASES["tags"]["walls"], CASES["tags"]["openings"], CASES["tags"]["windows"]):
            with self.subTest(wall=wall):
                voids = self.relations(wall, "IfcRelVoidsElement")
                self.assertEqual([(r["relating"]["tag"], r["related"][0]["tag"]) for r in voids], [(wall, opening)])
                fills = self.relations(opening, "IfcRelFillsElement")
                self.assertEqual([(r["relating"]["tag"], r["related"][0]["tag"]) for r in fills], [(opening, window)])
                self.assertEqual(len(self.relations(window, "IfcRelFillsElement")), 1)
                self.assertEqual(len(self.relations(opening, "IfcRelVoidsElement")), 1)
                self.assertEqual({r["class"] for r in self.api.relations_of(opening, self.model)}, {"IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelContainedInSpatialStructure"})

    def test_materials_are_associated_and_relation_ends_exist(self):
        associated = {tag for r in self.model["relations"] if r["class"] == "IfcRelAssociatesMaterial" for tag in r["related"]}
        for tag in ("TY-PILOTI", "SL-1", "SL-R", "TY-WALL", "CW-1"):
            self.assertIn(tag, associated)
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
        relation = self.relations("B3", "IfcRelDefinesByType")[0]
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
        radius, height = self.piloti_radius(), self.geometry["pilotis"][0]["top"]
        body = self.api.represent("B3", "body", self.model)
        extrusion = body["primitives"][0]
        self.assertEqual(extrusion["kind"], "extrusion")
        self.assertAlmostEqual(extrusion["depth"], height)
        self.assertAlmostEqual(extrusion["profile"]["radius"], radius)
        self.assertEqual(body["derived"]["profile"], "IfcCircleProfileDef")
        axis = np.asarray(self.api.represent("B3", "axis", self.model)["primitives"][0]["points"])
        self.assertAlmostEqual(float(np.linalg.norm(axis[1] - axis[0])), height)
        self.assertTrue(np.allclose(axis[:, :2], [self.geometry["pilotis"][7]["x"], self.geometry["pilotis"][7]["y"]]))
        section = np.asarray(self.api.represent("B3", "section", self.model)["primitives"][0]["points"])
        self.assertAlmostEqual(shoelace(section), 2 * radius * height)
        plan = self.api.represent("B3", "plan", self.model)["primitives"][0]
        footprint = self.api.represent("B3", "footprint", self.model)["primitives"][0]
        self.assertEqual((plan["kind"], footprint["kind"]), ("circle", "circle"))
        self.assertAlmostEqual(plan["radius"], radius)
        self.assertEqual(plan["centre"], footprint["centre"])
        self.assertAlmostEqual(self.api.represent("B3", "3d", self.model)["primitives"][0]["z1"], height)

    def test_wall_representations_match_footprint_elevation_and_glazing(self):
        wall, quantities = self.wall("W-S"), self.api.quantities("W-S", self.model)
        footprint = self.api.represent("W-S", "footprint", self.model)["primitives"][0]["points"]
        self.assertAlmostEqual(shoelace(footprint), wall["length"] * wall["thickness"], places=6)
        elevation = self.api.represent("W-S", "elevation", self.model)["primitives"]
        outline = next(p for p in elevation if p["role"] == "outline")
        holes = [p for p in elevation if p["role"] == "hole"]
        glazing = [p for p in elevation if p["role"] == "glazing"]
        self.assertAlmostEqual(shoelace(outline["points"]) - sum(shoelace(h["points"]) for h in holes), quantities["net_side_area"], places=6)
        self.assertEqual(len(holes), len(glazing))
        self.assertAlmostEqual(sum(shoelace(g["points"]) for g in glazing), quantities["opening_area"], places=6)
        plan = self.api.represent("W-S", "plan", self.model)["primitives"]
        self.assertEqual(len([p for p in plan if p.get("role") == "glazing"]), len(wall["openings"]))
        prism = self.api.represent("W-S", "3d", self.model)["primitives"][0]
        self.assertEqual(len(prism["holes"]), len(wall["openings"]))
        self.assertAlmostEqual(prism["z1"] - prism["z0"], wall["top"] - wall["base"])
        derived = self.api.represent("W-S", "plan", self.model)["derived"]
        self.assertAlmostEqual(derived["volume"], quantities["volume"])

    def test_slab_representations_have_outline_and_void_rings(self):
        slab, result = self.slab("SL-1"), self.api.quantities("SL-1", self.model)
        footprint = self.api.represent("SL-1", "footprint", self.model)["primitives"]
        holes = [p for p in footprint if p["role"] == "hole"]
        outer = next(p for p in footprint if p["role"] == "outline")
        self.assertEqual(len(holes), len(slab["voids"]))
        self.assertAlmostEqual(shoelace(outer["points"]) - sum(shoelace(h["points"]) for h in holes), result["area"])
        section = np.asarray(self.api.represent("SL-1", "section", self.model)["primitives"][0]["points"])
        self.assertAlmostEqual(section[:, 1].max() - section[:, 1].min(), slab["thickness"])
        self.assertAlmostEqual(self.api.represent("SL-1", "body", self.model)["primitives"][0]["depth"], slab["thickness"])

    def test_window_representation_is_the_opening_rectangle(self):
        opening = self.wall("W-S")["openings"][0]
        elevation = np.asarray(self.api.represent("RW-S", "elevation", self.model)["primitives"][0]["points"])
        self.assertAlmostEqual(shoelace(elevation), (opening["end"] - opening["start"]) * (opening["head"] - opening["sill"]))
        plan = np.asarray(self.api.represent("RW-S", "plan", self.model)["primitives"][0]["points"])
        self.assertAlmostEqual(float(np.linalg.norm(plan[1] - plan[0])), opening["end"] - opening["start"])

    def test_unknown_kinds_and_objects_without_representation_are_rejected(self):
        for key, kind in (("B3", "mesh"), ("B3", ""), ("MISSING", "plan"), ("SC-1", "plan"), ("BL-1", "plan")):
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
        self.assertEqual(structure["counts_by_class"].get("IfcColumn"), 25)
        self.assertNotIn("IfcColumn", architecture["counts_by_class"])
        self.assertEqual(architecture["counts_by_class"].get("IfcWindow"), len(CASES["tags"]["windows"]))
        self.assertNotIn("IfcWindow", structure["counts_by_class"])
        self.assertTrue(set(services["counts_by_class"]) <= set(CASES["models"]["services_classes"]))
        self.assertFalse(set(services["element_guids"]) & set(structure["element_guids"]))
        self.assertFalse(set(services["element_guids"]) & set(architecture["element_guids"]))

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
        _, parsed = self.parsed("B3")
        entities = parsed["entities"]
        by_name = {}
        for number, (name, arguments) in entities.items():
            by_name.setdefault(name, []).append((number, arguments))
        column = by_name["IFCCOLUMN"][0][1]
        placement = entities[column[5][1]][1]
        axis = entities[placement[1][1]][1]
        point = entities[axis[0][1]][1][0]
        piloti = next(p for p in self.geometry["pilotis"] if p["tag"] == "B3")
        self.assertTrue(np.allclose(point, [piloti["x"], piloti["y"], piloti["base"]]))
        shape = entities[column[6][1]][1]
        representation = entities[shape[2][0][1]][1]
        self.assertEqual(representation[1:3], ["Body", "SweptSolid"])
        solid = entities[representation[3][0][1]][1]
        profile = entities[solid[0][1]][1]
        self.assertAlmostEqual(profile[3], piloti["radius"])
        self.assertAlmostEqual(solid[3], piloti["top"] - piloti["base"])
        direction = entities[solid[2][1]][1][0]
        self.assertTrue(np.allclose(direction, [0, 0, 1]))
        self.assertEqual(profile[0], ("enum", "AREA"))

    def test_pset_material_type_and_storey_entities_link_back_to_the_element(self):
        _, parsed = self.parsed("B3")
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
        self.assertEqual((kind[0], kind[1][0]), ("IFCCOLUMNTYPE", self.items["TY-PILOTI"]["guid"]))
        storey = entities[relations["IFCRELCONTAINEDINSPATIALSTRUCTURE"][0][1][5][1]]
        self.assertEqual((storey[0], storey[1][0], storey[1][9]), ("IFCBUILDINGSTOREY", self.items["ground"]["guid"], 0.0))

    def test_slab_and_wall_geometry_numbers_match_the_model(self):
        _, parsed = self.parsed("SL-1")
        entities = parsed["entities"]
        solid = next(a for n, a in entities.values() if n == "IFCEXTRUDEDAREASOLID")
        profile = entities[solid[0][1]][1]
        gross = self.slab("SL-1")
        self.assertAlmostEqual(profile[3] * profile[4], shoelace(gross["outline"]))
        self.assertAlmostEqual(solid[3], gross["thickness"])
        _, parsed = self.parsed("SL-R")
        layers = [a for n, a in parsed["entities"].values() if n == "IFCMATERIALLAYER"]
        self.assertEqual([layer[1] for layer in layers], [l["thickness"]["value"] for l in self.items["LS-ROOF"]["layers"]])
        _, parsed = self.parsed("W-S")
        solid = next(a for n, a in parsed["entities"].values() if n == "IFCEXTRUDEDAREASOLID")
        profile = parsed["entities"][solid[0][1]][1]
        wall = self.wall("W-S")
        self.assertAlmostEqual(profile[3], wall["length"])
        self.assertAlmostEqual(profile[4], wall["thickness"])
        self.assertAlmostEqual(solid[3], wall["top"] - wall["base"])

    def test_renumbering_changes_entity_numbers_but_never_the_global_id(self):
        first, parsed_first = self.parsed("B3")
        second, parsed_second = self.parsed("B3", first_id=500)
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
        excerpt = self.api.step_excerpt("B3", self.model)
        text = lambda role: [excerpt["lines"][s["line"]][s["start"]:s["end"]] for s in excerpt["spans"] if s["role"] == role]
        self.assertEqual(text("property"), ["LoadBearing", "IsExternal"])
        self.assertEqual(text("value"), [".T.", ".T."])
        self.assertEqual(text("pset"), ["Pset_ColumnCommon"])
        self.assertEqual(text("material"), [self.items["MAT-RC"]["name"]["en"]])
        self.assertAlmostEqual(float(text("depth")[0]), self.geometry["pilotis"][7]["top"])
        self.assertAlmostEqual(float(text("profile")[0]), self.piloti_radius())

    def test_invalid_requests_are_rejected(self):
        for key, kwargs in (("MISSING", {}), ("BL-1", {}), ("B3", {"language": "fr"}), ("B3", {"first_id": 0}), ("B3", {"first_id": True})):
            with self.subTest(key=key, kwargs=kwargs), self.assertRaises(ValueError):
                self.api.step_excerpt(key, self.model, **kwargs)

    def test_parser_oracle_rejects_malformed_fragments(self):
        good = self.api.step_excerpt("B3", self.model)["lines"]
        broken = {
            "unbalanced": [line.replace(");", ";", 1) if line.startswith("#102") else line for line in good],
            "undefined-ref": [line.replace("#102", "#999", 1) if line.startswith("#101") else line for line in good],
            "open-string": [line.replace("'B3'", "'B3", 1) for line in good],
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
                result = self.api.mesh("B3", segments=segments, model=self.model)
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

    def test_volume_and_area_equal_the_inscribed_prism_and_the_convex_hull(self):
        from scipy.spatial import ConvexHull

        radius, height = self.piloti_radius(), self.geometry["pilotis"][0]["top"]
        for segments in (4, 6, 8, 16, 64):
            with self.subTest(segments=segments):
                result = self.api.mesh("B3", segments=segments, model=self.model)
                report = mesh_report(result["vertices"], result["faces"])
                prism = segments / 2 * radius ** 2 * math.sin(2 * math.pi / segments) * height
                self.assertAlmostEqual(report["volume"], prism, places=6)
                hull = ConvexHull(np.asarray(result["vertices"]))
                self.assertAlmostEqual(report["volume"], hull.volume, places=6)
                self.assertAlmostEqual(report["area"], hull.area, places=6)
                self.assertLessEqual(report["volume"], math.pi * radius ** 2 * height + 1e-9)
        fine = mesh_report(*[self.api.mesh("B3", segments=256, model=self.model)[k] for k in ("vertices", "faces")])
        self.assertAlmostEqual(fine["volume"], math.pi * radius ** 2 * height, delta=1e-4)

    def test_vertices_lie_on_the_cylinder_between_base_and_top(self):
        piloti = next(p for p in self.geometry["pilotis"] if p["tag"] == "D2")
        result = self.api.mesh("D2", segments=12, model=self.model)
        vertices = np.asarray(result["vertices"])
        distance = np.hypot(vertices[:, 0] - piloti["x"], vertices[:, 1] - piloti["y"])
        self.assertTrue(np.all(np.isclose(distance, piloti["radius"], atol=1e-8) | np.isclose(distance, 0, atol=1e-8)))
        self.assertTrue(np.all(np.isclose(vertices[:, 2], piloti["base"]) | np.isclose(vertices[:, 2], piloti["top"])))
        self.assertEqual(int(np.isclose(distance, piloti["radius"]).sum()), 2 * 12)
        self.assertTrue(result["guid"] == piloti["guid"])

    def test_mesh_lists_what_it_keeps_and_what_it_loses(self):
        result = self.api.mesh("B3", model=self.model)
        kept, lost = {k["id"] for k in result["kept"]}, {k["id"] for k in result["lost"]}
        self.assertEqual(kept, {"vertices", "triangles"})
        self.assertTrue({"class", "guid", "property", "material"} <= lost)
        for item in result["kept"] + result["lost"]:
            self.assertTrue(item["en"].strip() and item["de"].strip())
        self.assertNotIn("class", result)
        self.assertNotIn("psets", result)

    def test_invalid_requests_are_rejected(self):
        for key, segments in (("B3", 2), ("B3", 0), ("B3", -4), ("B3", True), ("B3", "8"), ("B3", 8.0), ("SL-1", 8), ("MISSING", 8)):
            with self.subTest(key=key, segments=segments), self.assertRaises(ValueError):
                self.api.mesh(key, segments=segments, model=self.model)


class BimIdsContract(ModelCase):
    """✅ An IDS-style requirement separates what the model says from what the mesh lost."""

    def test_default_requirement_passes_for_all_pilotis_in_ifc_and_finds_none_in_the_mesh(self):
        semantic = self.api.ids_check(model=self.model, source="semantic")
        mesh = self.api.ids_check(model=self.model, source="mesh")
        self.assertEqual((semantic["found"], semantic["checked"], semantic["passed"], semantic["failed"]), (25, 25, 25, []))
        self.assertEqual(semantic["applicable"], self.api.type_occurrences(CASES["tags"]["piloti_type"], self.model))
        self.assertEqual(semantic["passed_tags"], semantic["applicable"])
        self.assertEqual((mesh["found"], mesh["checked"], mesh["passed"], mesh["failed"], mesh["applicable"]), (0, 0, 0, [], []))
        self.assertEqual(mesh["expected"], 25)
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
        del broken["types"][[t["tag"] for t in broken["types"]].index("TY-PILOTI")]["psets"]["Pset_ColumnCommon"]["LoadBearing"]
        result = self.api.ids_check(model=broken, source="semantic")
        self.assertEqual((result["passed"], len(result["failed"])), (0, 25))
        self.assertEqual(self.api.ids_check(model=self.model, source="semantic")["passed"], 25)

    def test_invalid_specs_and_sources_are_rejected(self):
        for kwargs in ({"source": "dwg"}, {"spec": "IfcColumn"}, {"spec": {"entity": "IfcColumn"}}, {"spec": {"entity": "IfcColumn", "property": "X", "cardinality": "maybe"}}, {"spec": 7}):
            with self.subTest(kwargs=kwargs), self.assertRaises(ValueError):
                self.api.ids_check(model=self.model, **kwargs)


class BimAnalysisContract(ModelCase):
    """🏗️ The structural analysis translation is mapped, counted and honest about its losses."""

    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.analysis = cls.api.translate_structure(cls.model)

    def test_source_and_target_schema_and_revision(self):
        analysis = self.analysis
        self.assertEqual(analysis["source_schema"], self.model["schema_id"])
        self.assertNotEqual(analysis["target_schema"], analysis["source_schema"])
        self.assertEqual(analysis["source_revision"], self.api.partial_model("structure", self.model)["revision"])
        self.assertEqual(analysis["model_class"], "IfcStructuralAnalysisModel")

    def test_node_grid_equals_numpy_meshgrid_of_the_authored_lines(self):
        spec = self.model["translation"]["mesh"]
        xs, ys = np.asarray(spec["x_lines"], dtype=float), np.asarray(self.model["grid"]["y"], dtype=float)
        gx, gy = np.meshgrid(xs, ys)
        z = self.slab("SL-1")["top"] - self.slab("SL-1")["thickness"] / 2
        expected = {(round(x, 6), round(y, 6), round(z, 6)) for x, y in zip(gx.ravel(), gy.ravel())}
        slab_nodes = [n for n in self.analysis["nodes"] if n["role"] == "slab"]
        self.assertEqual({tuple(round(c, 6) for c in n["position"]) for n in slab_nodes}, expected)
        self.assertEqual(len(slab_nodes), CASES["analysis"]["slab_nodes"])
        self.assertEqual(len(slab_nodes), len(xs) * len(ys))
        self.assertEqual(len({n["id"] for n in self.analysis["nodes"]}), len(self.analysis["nodes"]))

    def test_shells_tile_the_slab_outline_with_the_slab_thickness(self):
        slab, nodes = self.slab("SL-1"), {n["id"]: n["position"] for n in self.analysis["nodes"]}
        shells = self.analysis["shells"]
        self.assertEqual(len(shells), CASES["analysis"]["shells"])
        total = 0.0
        for shell in shells:
            self.assertEqual(len(shell["nodes"]), 4)
            points = [nodes[i][:2] for i in shell["nodes"]]
            self.assertAlmostEqual(shoelace(points), shell["area"], places=6)
            total += shoelace(points)
            self.assertEqual(shell["source_guid"], slab["guid"])
            self.assertEqual(shell["thickness"], slab["thickness"])
        self.assertAlmostEqual(total, shoelace(slab["outline"]), places=6)
        self.assertGreater(total, self.api.quantities("SL-1", self.model)["area"])
        self.assertTrue(any("ramp" in a["en"].lower() for a in self.analysis["assumptions"]))

    def test_members_map_one_to_one_to_pilotis_and_one_to_many_to_the_slab(self):
        members = self.analysis["members"]
        curves = [m for m in members if m["kind"] == "curve"]
        surfaces = [m for m in members if m["kind"] == "surface"]
        self.assertEqual((len(curves), len(surfaces)), (CASES["analysis"]["curve_members"], CASES["analysis"]["surface_members"]))
        self.assertEqual({m["ifc_class"] for m in curves}, {"IfcStructuralCurveMember"})
        self.assertEqual({m["ifc_class"] for m in surfaces}, {"IfcStructuralSurfaceMember"})
        self.assertEqual(sorted(m["source_guid"] for m in curves), sorted(p["guid"] for p in self.geometry["pilotis"]))
        self.assertEqual({m["source_guid"] for m in surfaces}, {self.slab("SL-1")["guid"]})
        self.assertEqual(len(surfaces), len(self.analysis["shells"]))
        nodes = {n["id"]: n["position"] for n in self.analysis["nodes"]}
        for member in curves:
            self.assertTrue(set(member["nodes"]) <= nodes.keys())
            top = nodes[member["nodes"][1]]
            self.assertAlmostEqual(member["length"], top[2] - nodes[member["nodes"][0]][2])
        self.assertEqual(len({m["id"] for m in members}), len(members))

    def test_supports_sit_under_every_piloti_with_the_declared_condition(self):
        supports, nodes = self.analysis["supports"], {n["id"]: n["position"] for n in self.analysis["nodes"]}
        self.assertEqual(len(supports), CASES["analysis"]["supports"])
        feet = {(p["x"], p["y"]) for p in self.geometry["pilotis"]}
        self.assertEqual({tuple(nodes[s["node"]][:2]) for s in supports}, feet)
        for support in supports:
            self.assertEqual(nodes[support["node"]][2], 0)
            self.assertEqual(support["condition"], self.model["translation"]["boundary"]["class"])
            self.assertTrue(support["label"]["en"].strip() and support["label"]["de"].strip())
            self.assertEqual(support["source_guid"], oracle_global_id(support["source_tag"]))
        self.assertEqual(self.analysis["counts"]["nodes"], len(self.analysis["nodes"]))

    def test_losses_name_the_source_element_and_cover_the_architectural_content(self):
        losses = self.analysis["losses"]
        tags = {loss["source_tag"] for loss in losses}
        for loss in losses:
            self.assertEqual(loss["source_guid"], oracle_global_id(loss["source_tag"]))
            self.assertTrue(loss["en"].strip() and loss["de"].strip() and loss["property"].strip())
        for tag in CASES["tags"]["windows"] + CASES["tags"]["openings"] + CASES["tags"]["walls"] + [CASES["tags"]["curtain_wall"]]:
            self.assertIn(tag, tags)
        for space in self.geometry["spaces"]:
            self.assertIn(space["tag"], tags)
        pilotis = {p["tag"] for p in self.geometry["pilotis"]}
        for loss in losses:
            if loss["source_tag"] in pilotis:
                self.assertNotEqual(loss["property"], "element")
        self.assertTrue(any(loss["property"].startswith("material") for loss in losses))

    def test_assumptions_are_bilingual_and_marked_assumed(self):
        assumptions = self.analysis["assumptions"]
        self.assertGreaterEqual(len(assumptions), 5)
        self.assertEqual(len({a["id"] for a in assumptions}), len(assumptions))
        for item in assumptions:
            self.assertEqual(item["basis"], "assumed")
            self.assertTrue(item["en"].strip() and item["de"].strip())
            self.assertNotIn("{", item["en"] + item["de"])
        ids = {a["id"] for a in assumptions}
        self.assertTrue({"mid-surface", "support-fixed", "material", "loads"} <= ids)

    def test_floor_load_is_self_weight_plus_finishes_plus_imposed(self):
        loads = self.analysis["loads"]
        slab = self.slab("SL-1")
        density = self.items["MAT-RC"]["density"]["value"]
        self.assertAlmostEqual(loads["self_weight"]["value"], slab["thickness"] * density)
        self.assertAlmostEqual(loads["finishes"]["value"], self.model["loads"]["finishes"]["first"]["value"])
        self.assertAlmostEqual(loads["imposed"]["value"], self.model["loads"]["imposed"]["first"]["value"])
        self.assertAlmostEqual(loads["area_load"], self.floor_load(self.model))
        self.assertAlmostEqual(loads["total"]["value"], loads["area_load"])
        for part in ("self_weight", "finishes", "imposed"):
            self.assertEqual(loads[part]["unit"], "kN/m²")
            self.assertEqual(loads[part]["basis"], "assumed")

    def test_point_loads_carry_the_roof_reactions_to_the_slab_nodes(self):
        roof = self.api.tributary(self.model, level="roof")
        nodes = {n["id"]: n["position"] for n in self.analysis["nodes"]}
        reactions = {p["tag"]: p["reaction"] for p in roof["pilotis"] if p["reaction"] > 0}
        self.assertEqual(len(self.analysis["point_loads"]), len(reactions))
        for load in self.analysis["point_loads"]:
            self.assertAlmostEqual(load["value"], reactions[load["source_tag"]])
            piloti = next(p for p in self.geometry["pilotis"] if p["tag"] == load["source_tag"])
            self.assertEqual(nodes[load["node"]][:2], [piloti["x"], piloti["y"]])
            self.assertEqual(load["unit"], "kN")

    def test_mapping_rules_count_their_members(self):
        by_kind = {rule["kind"]: rule["count"] for rule in self.analysis["mapping"]}
        self.assertEqual(by_kind, {"curve": CASES["analysis"]["curve_members"], "surface": CASES["analysis"]["surface_members"], "support": CASES["analysis"]["supports"]})
        self.assertEqual(self.analysis["counts"]["members"], CASES["analysis"]["curve_members"] + CASES["analysis"]["surface_members"])
        self.assertEqual(self.analysis["counts"]["nodes"], CASES["analysis"]["slab_nodes"] + CASES["analysis"]["supports"])

    def test_translation_follows_the_model_state(self):
        updated = self.api.apply_event(self.event("soil"), self.model)
        after = self.api.translate_structure(updated)
        self.assertEqual(after["source_revision"], updated["revision"])
        self.assertEqual(self.analysis["source_revision"], self.model["revision"])
        self.assertEqual(after["loads"]["area_load"], self.analysis["loads"]["area_load"])
        before_roof = {p["source_tag"]: p["value"] for p in self.analysis["point_loads"]}
        after_roof = {p["source_tag"]: p["value"] for p in after["point_loads"]}
        self.assertEqual(set(before_roof), set(after_roof))
        self.assertTrue(all(after_roof[tag] > before_roof[tag] for tag in before_roof))
        sill = self.api.translate_structure(self.api.apply_event(self.event("sill"), self.model))
        self.assertEqual(sill["source_revision"], self.analysis["source_revision"])
        self.assertEqual(sill["point_loads"], self.analysis["point_loads"])


class BimTributaryContract(ModelCase):
    """📐 Tributary areas are recomputed with a nearest-support KD-tree on a fine grid."""

    def oracle(self, level):
        slab = next(s for s in self.geometry["slabs"] if s["kind"] == {"first": "FLOOR", "roof": "ROOF"}[level])
        from scipy.spatial import cKDTree

        points = sample_surface(slab["outline"], slab["voids"], CASES["tributary"]["step"])
        centres = np.array([[p["x"], p["y"]] for p in self.geometry["pilotis"]])
        nearest = cKDTree(centres).query(points)[1]
        cell = CASES["tributary"]["step"] ** 2
        return slab, points, nearest, np.bincount(nearest, minlength=25) * cell

    def test_areas_sum_to_the_net_slab_area_and_match_the_kd_tree_oracle(self):
        for level in ("first", "roof"):
            with self.subTest(level=level):
                result = self.api.tributary(self.model, level=level)
                slab, _, _, areas = self.oracle(level)
                net = self.api.quantities(slab["tag"], self.model)["area"]
                self.assertAlmostEqual(sum(p["area"] for p in result["pilotis"]), net, places=6)
                self.assertAlmostEqual(result["slab_area"], net, places=6)
                self.assertAlmostEqual(areas.sum(), net, delta=0.02 * net)
                for index, entry in enumerate(result["pilotis"]):
                    self.assertAlmostEqual(entry["area"], areas[index], delta=CASES["tributary"]["tolerance"] * max(entry["area"], 1))
                self.assertTrue(result["balanced"])
                self.assertEqual(result["slab_tag"], slab["tag"])

    def test_reactions_equal_area_load_times_area_and_sum_to_the_total_load(self):
        for level, expected in (("first", self.floor_load(self.model)), ("roof", self.roof_load(self.model))):
            with self.subTest(level=level):
                result = self.api.tributary(self.model, level=level)
                self.assertAlmostEqual(result["area_load"], expected)
                self.assertAlmostEqual(sum(p["reaction"] for p in result["pilotis"]), expected * result["slab_area"], places=5)
                self.assertAlmostEqual(result["total_reaction"], expected * result["slab_area"], places=5)
                if level == "first":
                    for entry in result["pilotis"]:
                        self.assertAlmostEqual(entry["reaction"], expected * entry["area"], places=6)

    def test_roof_reactions_follow_the_two_load_zones_and_match_point_sampling(self):
        result = self.api.tributary(self.model, level="roof")
        roof, loads = self.roof_zones(self.model)
        zones = {zone["id"]: zone for zone in result["zones"]}
        self.assertEqual(set(zones), {"solarium", "planted"})
        solarium = shoelace(roof["zones"]["solarium"])
        self.assertAlmostEqual(zones["solarium"]["area"], solarium, places=6)
        self.assertAlmostEqual(zones["planted"]["area"], result["slab_area"] - solarium, places=6)
        self.assertAlmostEqual(zones["solarium"]["area_load"], loads["solarium"])
        self.assertAlmostEqual(zones["planted"]["area_load"], loads["planted"])
        self.assertNotIn("soil", zones["solarium"]["load_breakdown"])
        self.assertIn("soil", zones["planted"]["load_breakdown"])
        self.assertAlmostEqual(sum(z["reaction"] for z in result["zones"]), result["total_reaction"], places=5)
        expected = self.roof_reactions(self.model)
        for entry in result["pilotis"]:
            with self.subTest(tag=entry["tag"]):
                shares = entry["zones"]
                self.assertAlmostEqual(entry["reaction"], sum(loads[z] * shares[z]["area"] for z in loads), places=6)
                self.assertAlmostEqual(sum(shares[z]["area"] for z in loads), entry["area"], places=6)
                self.assertAlmostEqual(entry["reaction"], expected[entry["tag"]], delta=0.012 * max(entry["reaction"], 10))

    def test_cells_are_disjoint_polygons_that_cover_the_slab(self):
        result = self.api.tributary(self.model, level="first")
        slab, points, nearest, _ = self.oracle("first")
        coarse = sample_surface(slab["outline"], slab["voids"], 0.1)
        counts = np.zeros(len(coarse), dtype=int)
        for entry in result["pilotis"]:
            if entry["cell"]:
                counts += inside(entry["cell"], coarse)
                self.assertAlmostEqual(shoelace(entry["cell"]), entry["area"], places=6)
        self.assertTrue(np.all(counts == 1))

    def test_kinds_count_interior_edge_and_corner_cells_of_the_grid(self):
        result = self.api.tributary(self.model)
        kinds = [p["kind"] for p in result["pilotis"]]
        self.assertEqual({k: kinds.count(k) for k in set(kinds)}, CASES["tributary"]["kinds"])
        for entry in result["pilotis"]:
            row, column = entry["tag"][0], entry["tag"][1:]
            edges = int(row in (CASES["grid"]["rows"][0], CASES["grid"]["rows"][-1])) + int(column in (CASES["grid"]["columns"][0], CASES["grid"]["columns"][-1]))
            self.assertEqual(entry["kind"], ("interior", "edge", "corner")[edges])

    def test_cell_areas_equal_exact_rectangle_intersections_with_the_voids(self):
        def cuts(lines, low, high):
            return [low] + [(a + b) / 2 for a, b in zip(lines, lines[1:])] + [high]

        def overlap(a0, a1, b0, b1):
            return max(0.0, min(a1, b1) - max(a0, b0))

        grid = self.model["grid"]
        for level, kind in (("first", "FLOOR"), ("roof", "ROOF")):
            slab = next(s for s in self.geometry["slabs"] if s["kind"] == kind)
            outline = np.asarray(slab["outline"])
            boxes = [(np.asarray(v).min(axis=0), np.asarray(v).max(axis=0)) for v in slab["voids"]]
            for void, (low, high) in zip(slab["voids"], boxes):
                self.assertAlmostEqual(shoelace(void), float(np.prod(high - low)))
            xs, ys = cuts(grid["x"], outline[:, 0].min(), outline[:, 0].max()), cuts(grid["y"], outline[:, 1].min(), outline[:, 1].max())
            result = {p["tag"]: p["area"] for p in self.api.tributary(self.model, level=level)["pilotis"]}
            for j, row in enumerate(grid["labels_y"]):
                for i, column in enumerate(grid["labels_x"]):
                    cell = (xs[i + 1] - xs[i]) * (ys[j + 1] - ys[j])
                    cut = sum(overlap(xs[i], xs[i + 1], low[0], high[0]) * overlap(ys[j], ys[j + 1], low[1], high[1]) for low, high in boxes)
                    with self.subTest(level=level, tag=row + column):
                        self.assertAlmostEqual(result[row + column], cell - cut, places=6)

    def test_unknown_level_is_rejected(self):
        with self.assertRaises(ValueError):
            self.api.tributary(self.model, level="basement")


class BimClashContract(ModelCase):
    """🚧 The clearance rule finds exactly the teaching violation, recomputed with NumPy."""

    def distances(self):
        rule = self.model["rules"]["clearance"][0]
        pipes = [e for e in self.model["elements"] if e["class"] == rule["a_class"] and e.get("system") == rule["a_system"]]
        pilotis = np.array([[p["x"], p["y"]] for p in self.geometry["pilotis"]])
        out = {}
        for pipe in pipes:
            point = np.asarray(pipe["geometry"]["points"][0][:2], dtype=float)
            gap = np.hypot(*(pilotis - point).T) - pipe["attributes"]["diameter"]["value"] / 2 - self.piloti_radius()
            for index, value in enumerate(gap):
                out[(pipe["tag"], self.geometry["pilotis"][index]["tag"])] = float(value)
        return rule["required"]["value"], out

    def test_exactly_one_pair_violates_the_rule(self):
        required, gaps = self.distances()
        violations = {pair for pair, value in gaps.items() if value < required}
        self.assertEqual(violations, {tuple(pair) for pair in CASES["clash"]["violations"]})
        result = self.api.clash_check(self.model)
        self.assertEqual([(c["a_tag"], c["b_tag"]) for c in result], [tuple(pair) for pair in CASES["clash"]["violations"]])
        clash = result[0]
        self.assertAlmostEqual(clash["clearance"], gaps[(clash["a_tag"], clash["b_tag"])], places=6)
        self.assertAlmostEqual(clash["required"], required)
        self.assertEqual(clash["required_basis"], "assumed")
        self.assertEqual(clash["status"], "violation")
        self.assertAlmostEqual(clash["deficit"], required - clash["clearance"], places=6)
        self.assertEqual(clash["a_guid"], oracle_global_id(clash["a_tag"]))
        self.assertEqual(clash["b_guid"], oracle_global_id(clash["b_tag"]))
        self.assertTrue(clash["name"]["en"].strip() and clash["name"]["de"].strip())
        self.assertEqual(clash["level"], "ground")

    def test_clash_point_lies_between_the_pipe_and_the_piloti(self):
        clash = self.api.clash_check(self.model)[0]
        pipe = next(e for e in self.model["elements"] if e["tag"] == clash["a_tag"])
        piloti = next(p for p in self.geometry["pilotis"] if p["tag"] == clash["b_tag"])
        x, y = clash["point"][:2]
        a, b = np.asarray(pipe["geometry"]["points"][0][:2]), np.array([piloti["x"], piloti["y"]])
        self.assertAlmostEqual(np.linalg.norm([x, y] - a) + np.linalg.norm([x, y] - b), np.linalg.norm(a - b), places=4)
        self.assertTrue(0 <= clash["point"][2] <= piloti["top"] + 1e-9)

    def test_passed_pairs_are_listed_on_request_and_each_pipe_appears_once(self):
        required, gaps = self.distances()
        everything = self.api.clash_check(self.model, include_passed=True)
        pipes = {pipe for pipe, _ in gaps}
        self.assertEqual({c["a_tag"] for c in everything}, pipes)
        self.assertEqual(len(everything), len(pipes))
        self.assertEqual([c["status"] for c in everything], ["violation"] + ["ok"] * (len(pipes) - 1))
        for clash in everything:
            nearest = min(value for (pipe, _), value in gaps.items() if pipe == clash["a_tag"])
            self.assertAlmostEqual(clash["clearance"], nearest, places=6)

    def test_moving_the_downpipe_or_relaxing_the_rule_removes_the_violation(self):
        moved = copy.deepcopy(self.model)
        pipe = next(e for e in moved["elements"] if e["tag"] == "DP-1")
        pipe["geometry"]["points"] = [[3.0, 2.375, p[2]] for p in pipe["geometry"]["points"]]
        self.assertEqual(self.api.clash_check(moved), [])
        relaxed = copy.deepcopy(self.model)
        relaxed["rules"]["clearance"][0]["required"]["value"] = 0.1
        self.assertEqual(self.api.clash_check(relaxed), [])
        strict = copy.deepcopy(self.model)
        strict["rules"]["clearance"][0]["required"]["value"] = 0.5
        self.assertGreater(len(self.api.clash_check(strict)), 1)


class BimConsistencyContract(ModelCase):
    """🔗 Events keep identity, raise revisions, change derived values and expose stale analyses."""

    def test_events_never_mutate_their_input_and_raise_the_revision_by_one(self):
        for name in ("sill", "soil", "soil_alt"):
            before = copy.deepcopy(self.model)
            updated = self.api.apply_event(self.event(name), before)
            self.assertEqual(before, self.model)
            self.assertEqual(updated["revision"], self.model["revision"] + 1)
            self.assertIsNot(updated, before)
            self.assertNotEqual(updated, before)

    def test_global_ids_and_structure_stay_while_values_change(self):
        for name in ("sill", "soil"):
            updated = self.api.apply_event(self.event(name), self.model)
            self.assertEqual({t: i["guid"] for t, i in registry(updated).items()}, {t: i["guid"] for t, i in self.items.items()})
            self.assertEqual(len(updated["elements"]), len(self.model["elements"]))
            self.assertEqual(updated["relations"], self.model["relations"])

    def test_changed_objects_get_the_new_revision_and_their_models_follow(self):
        updated = self.api.apply_event(self.event("soil"), self.model)
        new = updated["revision"]
        changed = {t for t, i in registry(updated).items() if "metadata" in i and i["metadata"]["revision"] == new}
        self.assertEqual(changed, {"SL-R"})
        for identifier in CASES["models"]["ids"]:
            expected = new if self.items["SL-R"]["guid"] in self.api.partial_model(identifier, self.model)["element_guids"] else self.model["revision"]
            self.assertEqual(self.api.partial_model(identifier, updated)["revision"], expected, identifier)
        sill = self.api.apply_event(self.event("sill"), self.model)
        changed = {t for t, i in registry(sill).items() if "metadata" in i and i["metadata"]["revision"] == sill["revision"]}
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

    def test_deeper_soil_raises_the_roof_build_up_load_and_every_reaction(self):
        updated = self.api.apply_event(self.event("soil"), self.model)
        spec = CASES["events"]["soil"]
        self.assertAlmostEqual(self.api.quantity("SL-R", "soil", updated), spec["value"])
        old = self.api.quantities("SL-R", self.model)["total_thickness"]
        new = self.api.quantities("SL-R", updated)["total_thickness"]
        self.assertAlmostEqual(new - old, spec["value"] - self.api.quantity("SL-R", "soil", self.model))
        before, after = self.api.tributary(self.model, level="roof"), self.api.tributary(updated, level="roof")
        self.assertAlmostEqual(after["area_load"], self.roof_load(updated))
        self.assertGreater(after["area_load"], before["area_load"])
        soil = self.items["MAT-SOIL"]["density"]["value"]
        delta = soil * (spec["value"] - self.api.quantity("SL-R", "soil", self.model))
        zones = {z["id"]: z for z in before["zones"]}
        new_zones = {z["id"]: z for z in after["zones"]}
        self.assertAlmostEqual(new_zones["planted"]["area_load"] - zones["planted"]["area_load"], delta, places=6)
        self.assertEqual(new_zones["solarium"]["area_load"], zones["solarium"]["area_load"])
        expected = self.roof_reactions(updated)
        for a, b in zip(before["pilotis"], after["pilotis"]):
            with self.subTest(tag=a["tag"]):
                self.assertAlmostEqual(b["area"], a["area"])
                self.assertAlmostEqual(b["reaction"] - a["reaction"], delta * a["zones"]["planted"]["area"], places=6)
                self.assertAlmostEqual(b["reaction"], expected[b["tag"]], delta=0.012 * max(b["reaction"], 10))
        self.assertAlmostEqual(after["total_reaction"] - before["total_reaction"], delta * zones["planted"]["area"], places=5)
        unchanged = [a["tag"] for a, b in zip(before["pilotis"], after["pilotis"]) if a["reaction"] == b["reaction"]]
        self.assertEqual(unchanged, CASES["events"]["soil_unchanged"])
        self.assertEqual(self.api.tributary(updated, level="first")["area_load"], self.api.tributary(self.model, level="first")["area_load"])

    def test_the_base_model_is_consistent_and_a_roof_change_makes_the_analysis_link_stale(self):
        self.assertEqual(self.api.consistency_issues(self.model), [])
        updated = self.api.apply_event(self.event("soil"), self.model)
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
        self.assertIn("planted_area_load", stale)
        self.assertNotIn("solarium_area_load", stale)
        self.assertAlmostEqual(stale["area_load"]["expected"], self.roof_load(updated))
        self.assertAlmostEqual(stale["area_load"]["actual"], self.roof_load(self.model))

    def test_resolving_the_link_makes_the_model_consistent_again_without_mutation(self):
        updated = self.api.apply_event(self.event("soil"), self.model)
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
        first = self.api.apply_event(self.event("soil"), self.model)
        second = self.api.apply_event(self.event("soil_alt"), first)
        self.assertEqual(second["revision"], self.model["revision"] + 2)
        self.assertAlmostEqual(self.api.quantity("SL-R", "soil", second), CASES["events"]["soil_alt"]["value"])
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

    def test_consistency_narration_numbers_are_the_model_values_in_both_languages(self):
        base = self.api.load_model()
        new, alt = self.api.apply_event(self.api.demo_event("roof_soil", base), base), self.api.apply_event(self.api.demo_event("roof_soil_alt", base), base)
        old_t, new_t, alt_t = (self.api.tributary(state, "roof") for state in (base, new, alt))
        zone = lambda trib, name: next(z for z in trib["zones"] if z["id"] == name)
        reaction = lambda trib, tag: next(p for p in trib["pilotis"] if p["tag"] == tag)
        cell = reaction(new_t, "A3")
        xs, ys = [p[0] for p in cell["cell"]], [p[1] for p in cell["cell"]]
        sliver = reaction(new_t, "C3")
        counts = self.api.translate_structure(new)["counts"]
        soil = lambda state: self.api.quantity("SL-R", "soil", state)
        values = {"A3 reaction new": round(cell["reaction"]), "A3 reaction old": round(reaction(old_t, "A3")["reaction"]), "A3 area": round(cell["area"], 2), "C3 area": round(sliver["area"], 2),
                  "q old": zone(old_t, "planted")["area_load"], "q new": zone(new_t, "planted")["area_load"], "q alt": zone(alt_t, "planted")["area_load"],
                  "dq": round(zone(new_t, "planted")["area_load"] - zone(old_t, "planted")["area_load"], 1), "dx": max(xs) - min(xs), "dy": max(ys) - min(ys),
                  "soil old": soil(base), "soil new": soil(new), "surface": counts["surface_members"], "columns": counts["curve_members"]}
        deltas = {p["tag"]: p["reaction"] - reaction(old_t, p["tag"])["reaction"] for p in new_t["pilotis"]}
        self.assertEqual(max(deltas, key=deltas.get), "D3")
        self.assertEqual(round(deltas["C4"]), 0)
        scene = next(n for n in ast.parse((CLIP / "🔗️consistency/🐍️.py").read_text(encoding="utf-8")).body if isinstance(n, ast.ClassDef) and n.name == "Beat6_Consistency")
        rows = next(ast.literal_eval(n.value) for n in scene.body if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in n.targets))
        text = {language: " ".join(clause[index] for clause in rows).replace(chr(160), " ") for index, language in ((1, "en"), (2, "de"))}
        point = {"en": lambda v, d: f"{v:.{d}f}", "de": lambda v, d: f"{v:.{d}f}".replace(".", ",")}
        for language, joined in text.items():
            for name, digits in (("A3 reaction new", 0), ("A3 reaction old", 0), ("A3 area", 2), ("C3 area", 2), ("q old", 2), ("q new", 2), ("q alt", 2), ("dq", 1), ("soil old", 2), ("soil new", 2),
                                 ("surface", 0), ("columns", 0)):
                with self.subTest(language=language, value=name):
                    self.assertIn(point[language](values[name], digits), joined)
            self.assertIn(f"{point[language](values['dx'], 2)} m × {point[language](values['dy'], 3)} m", joined)


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


if __name__ == "__main__":
    unittest.main()
