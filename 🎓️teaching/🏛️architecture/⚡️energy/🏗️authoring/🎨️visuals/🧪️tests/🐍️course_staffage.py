"""🎓 Course-wide staffage coverage: every cast the course draws resolves to one authored catalog asset at one authored scale, in one view, state and scenario, and every clip names only casts the manifest holds.

The rules, the enums and the valid and invalid vectors live in ``🧍️staffage/🧫️fixtures/🎓️course-coverage/🔣️.json``; the vitest twin ``🧍️staffage/🧪️tests/🎓️course-coverage`` runs the same vectors.
"""

import copy
import json
import math
import re
import unittest
from pathlib import Path

TEACHING = next(path for path in Path(__file__).resolve().parents if path.name == "🎓️teaching")
CASES = json.loads((TEACHING / "🏛️architecture/🧍️staffage/🧫️fixtures/🎓️course-coverage/🔣️.json").read_text(encoding="utf-8"))
ARCHITECTURE = TEACHING / CASES["architecture"]
ENUMS = CASES["enums"]
QUOTED = re.compile(r"""(["'])([^"'\n\\]*)\1""")
CALLED = re.compile(r"""\b(?:%s)\(\s*(["'])([^"'\n\\]*)\1""" % "|".join(map(re.escape, CASES["sources"]["cast_calls"])))
KEYWORD = re.compile(r"""\b(?:%s)\s*=\s*(["'])([^"'\n\\]*)\1""" % "|".join(map(re.escape, CASES["sources"]["cast_keywords"])))


def number(value, minimum=None, strict=False):
    """🔢 Whether ``value`` is a finite JSON number (not a boolean), optionally above (``strict``) or at ``minimum``."""
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        return False
    return minimum is None or (value > minimum if strict else value >= minimum)


def load_catalog(root):
    """🗂️ Every catalog asset as ``{category, row, asset}``: the folder it lives in, its manifest row and its document."""
    rows = {row["id"]: row for row in json.loads((root / "🔣️.json").read_text(encoding="utf-8"))["entries"]}
    files = sorted(root.glob("*/*/🔣️.json"), key=lambda file: file.as_posix())
    assets = [json.loads(file.read_text(encoding="utf-8"))["asset"] for file in files]
    return [{"category": re.sub(r"^[^a-z0-9]+", "", file.parent.parent.name), "row": rows.get(asset["id"]), "asset": asset} for file, asset in zip(files, assets)]


def load_presets(root):
    """🌤️ Ids of every environment preset of the catalog."""
    return {preset["id"] for file in sorted(root.glob("*environments*/🔣️.json")) for preset in json.loads(file.read_text(encoding="utf-8"))["presets"]}


def asset_issues(entry):
    """📦 Problems of one asset on its own: category, manifest row and analytical units."""
    asset, row, found = entry["asset"], entry["row"], []
    name = asset.get("id")
    if entry["category"] != asset.get("category"):
        found.append(f"category-mismatch: {name}: folder {entry['category']!r} but document {asset.get('category')!r}")
    if row is None:
        found.append(f"row-missing: {name}: no manifest row")
    else:
        height = asset.get("dimensions", {}).get("heightM")
        facts = {
            "category": (row.get("category"), asset.get("category")),
            "heightM": (row.get("heightM"), height),
            "poses": (row.get("poses"), [pose["id"] for pose in asset.get("poses", []) if not pose.get("key")]),
            "clips": (row.get("clips"), [clip["id"] for clip in asset.get("clips", [])]),
            "representations": (row.get("representations"), {key: value["status"] for key, value in asset.get("representations", {}).items()}),
        }
        found += [f"row-mismatch: {name}: {key} {left!r} against {right!r}" for key, (left, right) in facts.items() if left != right]
    dimensions = asset.get("dimensions", {})
    found += [f"units: {name}: dimensions.{key} must be a finite number above 0" for key in ("heightM", "widthM", "depthM") if not number(dimensions.get(key), 0, True)]
    if not number(dimensions.get("massKg"), 0):
        found.append(f"units: {name}: dimensions.massKg must be a finite number of at least 0")
    return found


def cast_issues(cast_id, cast, catalog, presets):
    """🎬 Problems of one manifest cast: its asset, authored scale, view, state, clock and scenario."""
    found = []

    def issue(code, text):
        found.append(f"{code}: {cast_id}: {text}")

    found_fields = [field for field in cast if field not in CASES["cast_fields"]]
    for field in found_fields:
        issue("field-unknown", f"field {field!r} is in no schema")
    matches = [entry for entry in catalog if entry["asset"].get("id") == cast.get("asset")]
    if not matches:
        issue("asset-unknown", f"asset {cast.get('asset')!r} is not in the catalog")
        return found
    if len(matches) > 1:
        issue("asset-ambiguous", f"asset {cast.get('asset')!r} has {len(matches)} catalog files")
        return found
    entry = matches[0]
    asset, row = entry["asset"], entry["row"]
    scale, style, viewpoint = cast.get("scale"), cast.get("view"), cast.get("viewpoint")
    if "scale" not in cast:
        issue("scale-missing", f"names no scale; presets {ENUMS['scales']}")
    elif scale not in ENUMS["scales"] or isinstance(scale, bool):
        issue("scale-preset", f"scale {scale!r} is no preset of {ENUMS['scales']}")
    else:
        representation = asset.get("representations", {}).get(str(scale))
        authored = [int(key) for key, value in asset.get("representations", {}).items() if value.get("status") == "authored"]
        if not representation or representation.get("status") != "authored":
            issue("scale-unauthored", f"asset {asset['id']} has no authored representation at 1:{scale}; authored {sorted(authored, reverse=True)}")
        else:
            if representation.get("denominator") != scale:
                issue("scale-denominator", f"representation {scale} declares denominator {representation.get('denominator')!r}")
            if viewpoint in ENUMS["viewpoints"] and not representation.get("geometry", {}).get(viewpoint):
                issue("viewpoint-empty", f"asset {asset['id']} draws no {viewpoint} geometry at 1:{scale}")
    if style not in ENUMS["styles"]:
        issue("view-style", f"view {style!r} is no style of {ENUMS['styles']}")
    if viewpoint not in ENUMS["viewpoints"]:
        issue("viewpoint-unknown", f"viewpoint {viewpoint!r} is none of {ENUMS['viewpoints']}")
    if cast.get("theme") not in ENUMS["themes"]:
        issue("theme-unknown", f"theme {cast.get('theme')!r} is none of {ENUMS['themes']}")
    if "mirror" in cast and (not isinstance(cast["mirror"], bool) or (cast["mirror"] and viewpoint != "elevation")):
        issue("mirror-viewpoint", f"mirror {cast['mirror']!r} is a boolean and only true with elevation, not {viewpoint!r}")
    named = [key for key in ("pose", "clip") if key in cast]
    if len(named) != 1:
        issue("state-count", f"names {len(named)} of pose and clip, exactly one is required")
    if "pose" in cast:
        pose = next((pose for pose in asset.get("poses", []) if pose["id"] == cast["pose"]), None)
        if pose is None:
            issue("pose-unknown", f"asset {asset['id']} has no pose {cast['pose']!r}; poses {[pose['id'] for pose in asset.get('poses', [])]}")
        elif pose.get("key"):
            issue("pose-key", f"pose {cast['pose']!r} is a clip key frame, not a state of its own")
    if "clip" in cast and cast["clip"] not in [clip["id"] for clip in asset.get("clips", [])]:
        issue("clip-unknown", f"asset {asset['id']} has no clip {cast['clip']!r}; clips {[clip['id'] for clip in asset.get('clips', [])]}")
    timed = "clip" in cast or "timeline" in cast
    if timed and "fps" not in cast:
        issue("fps-missing", "a clip or a timeline needs fps")
    elif "fps" in cast and not timed:
        issue("fps-forbidden", "a still pose has no fps")
    elif "fps" in cast and not number(cast["fps"], 0, True):
        issue("fps-range", f"fps {cast['fps']!r} must be a number above 0")
    scenario = [key for key in ("environment", "timeline", "thermalSpanK", "transport", "transportAgeS", "transportStrokeMm", "transportTrailS") if key in cast]
    if style == "thermal":
        if ("environment" in cast) == ("timeline" in cast):
            issue("thermal-scenario", "the thermal view needs exactly one of environment and timeline")
        keys = cast.get("timeline", [])
        slugs = ([cast["environment"]] if "environment" in cast else []) + [key.get("environment") for key in keys if isinstance(key, dict)]
        for slug in slugs:
            if slug not in presets:
                issue("preset-unknown", f"environment preset {slug!r} is not in {sorted(presets)}")
        for key in keys:
            if key.get("ease") not in ENUMS["eases"]:
                issue("ease-missing", f"timeline key at {key.get('t')!r} needs an ease of {ENUMS['eases']}")
        times = [key.get("t") for key in keys]
        if not all(number(time, 0) for time in times) or len(set(times)) != len(times):
            issue("timeline-key", f"timeline times {times} must be distinct numbers of at least 0")
        if not asset.get("thermal", {}).get("regions"):
            issue("thermal-regions", f"asset {asset['id']} has no thermal regions")
        if "thermalSpanK" in cast and not number(cast["thermalSpanK"], 0, True):
            issue("span-range", f"thermalSpanK {cast['thermalSpanK']!r} must be a number above 0")
        if "transport" in cast and (not isinstance(cast["transport"], list) or any(layer not in ENUMS["transport"] for layer in cast["transport"])):
            issue("transport-layer", f"transport {cast['transport']!r} must list layers of {ENUMS['transport']}")
        if any(key in cast for key in ("transportAgeS", "transportStrokeMm", "transportTrailS")) and (not cast.get("transport") or (
            "transportAgeS" in cast and ("clip" not in cast or not number(cast["transportAgeS"], 0))
        )):
            issue("transport-age", "transportAgeS, transportStrokeMm and transportTrailS need a transport layer, and transportAgeS a clip and a number of at least 0")
    else:
        for key in scenario:
            issue("scenario-forbidden", f"{key!r} belongs to the thermal view, not {style!r}")
    if style in ("forces", "moments"):
        structure = asset.get("structure", {})
        if structure.get("dynamics") not in ENUMS["dynamics"] or not number(structure.get("gravityMS2"), 0, True):
            issue("structural-model", f"asset {asset['id']} needs structure.dynamics of {ENUMS['dynamics']} and a gravityMS2 above 0")
    analytics = (row or {}).get("analytics", [])
    needs = [("thermal", style == "thermal"), ("transport", bool(cast.get("transport"))), ("structural", style in ("forces", "moments"))]
    for name, required in needs:
        if required and row is not None and name not in analytics:
            issue("analytics", f"asset {asset['id']} row lists analytics {analytics} without {name!r}")
    return found


def manifest_issues(manifest, catalog, presets):
    """📜 Problems of a whole cast manifest: version, every cast and every asset it draws."""
    found = [] if manifest.get("version") == 1 else [f"manifest-version: version {manifest.get('version')!r} is not 1"]
    drawn = set()
    for cast_id, cast in manifest["casts"].items():
        found += cast_issues(cast_id, cast, catalog, presets)
        drawn.add(cast.get("asset"))
    for entry in catalog:
        if entry["asset"].get("id") in drawn:
            found += asset_issues(entry)
    return found


def scan_sources(sources, manifest):
    """🔎 Cast names the sources of the course name: exact ids, id prefixes ending in ``-``, and broken names (a call argument or an ``on_cast`` keyword no manifest row carries)."""
    casts = manifest["casts"]
    references, prefixes, broken = set(), set(), []
    for source in sources:
        for match in QUOTED.finditer(source["text"]):
            literal = match.group(2)
            if literal in casts:
                references.add(literal)
            elif literal.endswith("-") and len(literal) > 1 and any(name.startswith(literal) for name in casts):
                prefixes.add(literal)
        for pattern in (CALLED, KEYWORD):
            broken += [(source["path"], match.group(2)) for match in pattern.finditer(source["text"]) if match.group(2) not in casts]
    orphans = sorted(name for name in casts if name not in references and not any(name.startswith(prefix) for prefix in prefixes))
    return {"references": sorted(references), "prefixes": sorted(prefixes), "orphans": orphans, "broken": sorted({name for _, name in broken})}, broken


def source_issues(sources, manifest):
    """📚 Problems of the sources against the manifest: broken names and manifest rows no source names."""
    report, broken = scan_sources(sources, manifest)
    return [f"ref-unknown: {path}: cast {name!r} is in no manifest row" for path, name in broken] + [f"cast-orphan: {name}: no source names this cast" for name in report["orphans"]]


def course_sources():
    """🏫 The Python sources of every course series outside tests, as ``{series, path, text}``, plus the series folders that do not exist."""
    skip, sources, missing = set(CASES["sources"]["skip_folders"]), [], []
    for series in CASES["series"]:
        folder = ARCHITECTURE / series
        if not folder.is_dir():
            missing.append(series)
            continue
        for file in sorted(folder.rglob("*" + CASES["sources"]["extension"]), key=lambda file: file.as_posix()):
            if not skip & set(file.relative_to(folder).parts):
                sources.append({"series": series, "path": file.relative_to(ARCHITECTURE).as_posix(), "text": file.read_text(encoding="utf-8")})
    return sources, missing


def patched(synthetic, vector):
    """🧬 The synthetic manifest and catalog with one invalid vector applied."""
    manifest, catalog = copy.deepcopy(synthetic["valid"]), copy.deepcopy(synthetic["catalog"])
    cast = copy.deepcopy(manifest["casts"][vector["cast"]])
    entry = next(entry for entry in catalog if entry["asset"]["id"] == cast["asset"])
    cast.update(vector.get("patch", {}))
    for key in vector.get("remove", []):
        cast.pop(key, None)
    manifest["casts"] = {"bad": cast}
    entry["asset"].update(vector.get("asset_patch", {}))
    if vector.get("row_remove"):
        entry["row"] = None
    elif entry["row"] is not None:
        entry["row"].update(vector.get("row_patch", {}))
    return manifest, catalog


class CourseCoverage(unittest.TestCase):
    synthetic = CASES["synthetic"]
    presets = set(synthetic["presets"])

    def test_the_valid_vector_has_no_issue_and_every_invalid_vector_names_its_code(self):
        self.assertEqual(manifest_issues(self.synthetic["valid"], self.synthetic["catalog"], self.presets), [])
        for vector in self.synthetic["invalid"]:
            with self.subTest(vector["name"]):
                manifest, catalog = patched(self.synthetic, vector)
                found = manifest_issues(manifest, catalog, self.presets)
                self.assertTrue(any(line.startswith(vector["code"] + ":") for line in found), f"{vector['code']} missing in {found}")
        self.assertEqual(sorted({vector["code"] for vector in self.synthetic["invalid"]} | {vector["code"] for vector in self.synthetic["invalid_sources"]} | {"manifest-version", "series-missing"}), sorted(CASES["codes"]))

    def test_sources_resolve_names_and_prefixes_and_report_broken_names_and_orphans(self):
        valid = {"casts": self.synthetic["valid"]["casts"]}
        report, _ = scan_sources(self.synthetic["valid_sources"], valid)
        self.assertEqual(report, self.synthetic["valid_expect"])
        report, _ = scan_sources(self.synthetic["prefix_sources"], valid)
        self.assertEqual(report, self.synthetic["prefix_expect"])
        for vector in self.synthetic["invalid_sources"]:
            with self.subTest(vector["name"]):
                found = source_issues(vector["sources"], valid)
                self.assertTrue(any(line.startswith(vector["code"] + ":") and vector["mention"] in line for line in found), found)

    def test_every_series_of_the_course_exists_and_its_sources_are_scanned(self):
        sources, missing = course_sources()
        self.assertEqual(missing, [], "series folders missing: series-missing")
        self.assertTrue(sources)

    def test_the_course_manifest_resolves_every_cast_to_an_authored_catalog_asset(self):
        manifest = json.loads((ARCHITECTURE / CASES["manifest"]).read_text(encoding="utf-8"))
        catalog = load_catalog(ARCHITECTURE / CASES["catalog"])
        found = manifest_issues(manifest, catalog, load_presets(ARCHITECTURE / CASES["catalog"]))
        self.assertEqual(found, [])

    def test_every_clip_names_only_manifest_casts_and_every_cast_is_named_by_a_clip(self):
        manifest = json.loads((ARCHITECTURE / CASES["manifest"]).read_text(encoding="utf-8"))
        sources, _ = course_sources()
        self.assertEqual(source_issues(sources, manifest), [])


if __name__ == "__main__":
    unittest.main()
