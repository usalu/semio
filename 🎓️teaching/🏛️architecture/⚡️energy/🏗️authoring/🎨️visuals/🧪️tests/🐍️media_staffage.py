"""♻️ Staffage media freshness at render checkpoints, without importing the renderer."""
import ast
import functools
import hashlib
import json
import os
import re
import tempfile
import types
import unittest
from pathlib import Path

BRIDGE = Path(__file__).parent.parent / "🧍️staffage/🐍️.py"
CASES = json.loads((BRIDGE.parent / "🧪️tests/🔣️.json").read_text(encoding="utf-8"))
FUNCTIONS = {"_lf", "_number", "canonical_json", "revision_of", "_source_digest", "_catalog_bytes", "_presets", "asset_revision", "preset_revision", "_preset_revisions", "entry_issues", "cast_key", "casts_in"}
CONSTANTS = {"_INDEX", "_VIEWPOINTS", "_VIEWS", "_THEMES", "_SCALES", "_TRANSPORT_LAYERS", "_CAST_KEYS", "_BUNDLED", "_KEY_LENGTH"}


def bridge():
    """🧩 Load actual dependency-identity functions independently of Manim."""
    tree = ast.parse(BRIDGE.read_text(encoding="utf-8"))
    nodes = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name in FUNCTIONS or isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id in CONSTANTS for target in node.targets)]
    module = types.ModuleType("staffage_freshness")
    module.__dict__.update({"ast": ast, "hashlib": hashlib, "json": json, "re": re, "Path": Path, "lru_cache": functools.lru_cache, "_SOURCES": BRIDGE.parent})
    exec(compile(ast.Module(body=nodes, type_ignores=[]), str(BRIDGE), "exec"), module.__dict__)
    return module


class MediaFreshness(unittest.TestCase):
    def test_scene_dependency_closure_includes_visuals_bridge_and_manifest_owner(self):
        teaching = next(parent for parent in BRIDGE.parents if parent.name == "🎓️teaching")
        runtime = BRIDGE.parents[2] / "🎬️clip/⚙️runtime/🐍️.py"
        tree = ast.parse(runtime.read_text(encoding="utf-8"))
        function = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == "source_dependencies")
        namespace = {"ast": ast, "Path": Path, "__file__": str(runtime), "teaching_root": lambda _: teaching}
        exec(compile(ast.Module(body=[function], type_ignores=[]), str(runtime), "exec"), namespace)
        scene = teaching / "🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py"
        sources = {source.resolve() for source in namespace["source_dependencies"](scene)}
        self.assertIn(scene.resolve(), sources)
        self.assertIn(BRIDGE.resolve(), sources)
        self.assertIn((BRIDGE.parent.parent / "🐍️.py").resolve(), sources)

    def test_each_checkpoint_observes_authored_content_changes_without_manual_cache_resets(self):
        for vector in CASES["mediaFreshness"]:
            with self.subTest(vector["path"]), tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
                root = Path(directory)
                staffage = bridge()
                for path, content in CASES["mediaInputs"].items():
                    file = root / path
                    file.parent.mkdir(parents=True, exist_ok=True)
                    file.write_text(content, encoding="utf-8")
                source = root / "scene.py"
                source.write_text('draw("case-cast")\n', encoding="utf-8")
                entry = {**CASES["key"]["entry"], "view": "thermal", "environment": "summer"}
                from jsonschema import Draft7Validator

                teaching = next(parent for parent in BRIDGE.parents if parent.name == "🎓️teaching")
                schema = json.loads((teaching / "🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
                manifest = {"version": 1, "casts": {"case-cast": entry}}
                self.assertEqual(list(Draft7Validator(schema).iter_errors(manifest)), [])
                self.assertEqual(staffage.entry_issues(entry), [])
                before = staffage.casts_in([source], manifest, root)
                self.assertEqual(before, staffage.casts_in([source], manifest, root))
                target = root / vector["path"]
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(vector["after"], encoding="utf-8")
                after = staffage.casts_in([source], manifest, root)
                self.assertNotEqual(before, after)


if __name__ == "__main__":
    unittest.main()
