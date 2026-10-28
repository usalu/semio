"""📏️ Verify the actual live loss-record footer against independent native text bounds."""
from pathlib import Path
import ast
import importlib
import json
import os
import sys
import unittest
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
SEGMENT = {"script": "🏛️architecture/🏗️bim/🎬️clip/🧩️models/🐍️.py", "scene": "Beat4_PartialModels"}
STATE = runtime.segment_state(SEGMENT, dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator"))
module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
mn = module._engine
SOURCE = Path(module.__file__)

class ModelsFooter(unittest.TestCase):
    """🧪 Keep localized phrases readable throughout both live counter ranges."""
    def test_actual_trace_footer_has_measured_gaps_through_live_counts(self):
        """🔢 Compare all four native bounds with NumPy and preserve live text dimensions."""
        import numpy as np
        from PIL import Image
        self.assertTrue(runtime.state_inputs_current(STATE), "Inputs changed after importing the oracle")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        tree = ast.parse(SOURCE.read_text(encoding="utf-8"))
        scene_class = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == SEGMENT["scene"])
        trace = next(node for node in scene_class.body if isinstance(node, ast.FunctionDef) and node.name == "_trace")
        expression = next(node.value for node in trace.body if isinstance(node, ast.Assign)
            and any(isinstance(target, ast.Name) and target.id == "footer" for target in node.targets))
        expression = compile(ast.Expression(expression), str(SOURCE), "eval")
        analysis = module._model.translate_structure()
        totals = [len(analysis["losses"]), len(analysis["loss_groups"])]
        self.assertEqual(totals, CASES["model_counts"])
        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), mn.tempconfig({
                    "media_dir": str(output / "media"), "text_dir": str(output / "text"), "dry_run": True,
                    "pixel_width": CASES["pixel_size"][0], "pixel_height": CASES["pixel_size"][1]}):
                scene = module.Beat4_PartialModels()
                scene.analysis = analysis
                records, kinds = mn.ValueTracker(0), mn.ValueTracker(0)
                footer = eval(expression, vars(module), dict(self=scene, records=records, kinds=kinds))
                self.assertEqual(len(footer), 4)
                counters = [footer[0], footer[2]]
                self.assertTrue(all(counter.updaters for counter in counters))
                seen = set()
                samples = [*map(tuple, CASES["extremes"]), tuple(totals)]
                if CASES["sweep_each_integer"]:
                    samples += [(value, totals[1]) for value in range(totals[0] + 1)]
                    samples += [(totals[0], value) for value in range(totals[1] + 1)]
                for values in samples:
                    if values in seen:
                        continue
                    seen.add(values)
                    records.set_value(values[0])
                    kinds.set_value(values[1])
                    footer.update(0)
                    edges = np.array([[item.get_left()[0], item.get_right()[0]] for item in footer])
                    gaps = edges[1:, 0] - edges[:-1, 1]
                    self.assertTrue(np.all(gaps >= CASES["minimum_gap"] - 1e-8),
                        (language, values, "gaps", gaps.tolist()))
                    self.assertGreaterEqual(float(edges[:, 0].min()), CASES["content_bounds"][0])
                    self.assertLessEqual(float(edges[:, 1].max()), CASES["content_bounds"][1])
                    np.testing.assert_allclose([item.get_center()[1] for item in footer], CASES["y"], atol=1e-8)
                    for index, counter in enumerate(counters):
                        reference = scene.text(scene.number(values[index], 0), (0, 0, 0),
                            size=CASES["sizes"]["counter"], color=module.WARN)
                        np.testing.assert_allclose([counter.width, counter.height], [reference.width, reference.height], atol=1e-8)
                records.set_value(totals[0])
                kinds.set_value(totals[1])
                footer.update(0)
                scene.camera.reset()
                scene.camera.capture_mobjects([footer])
                pixels = scene.camera.pixel_array
                self.assertTrue(np.any(pixels != pixels[0, 0]))
                Image.fromarray(pixels).save(output / ("models-footer-" + language + ".png"))
        self.assertTrue(runtime.state_inputs_current(STATE), "Inputs changed during the oracle")

if __name__ == "__main__":
    unittest.main()
