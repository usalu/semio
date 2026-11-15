"""🗺️ Every energy video starts with its roadmap, declared as data on the shared roadmap builder."""

import ast
import importlib
import json
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
ENERGY = ROOT / "🎓️teaching/🏛️architecture/⚡️energy"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
MANIFEST = json.loads((ENERGY / "🎬️clip/🔣️.json").read_text(encoding="utf-8"))["videos"]
MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🗺️roadmap.🐍️"


def declared(path: Path, name: str, *default):
    """📜 Literal value of a class attribute of ``Beat1_Fahrplan`` (or the default when it is not declared), read without importing the scene."""
    tree = ast.parse(path.read_text(encoding="utf-8"))
    scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == CASES["scene"])
    assignment = next((node for node in scene.body if isinstance(node, ast.Assign) and node.targets[0].id == name), None)
    if assignment is None:
        if default:
            return default[0]
        raise KeyError(name)
    return ast.literal_eval(assignment.value)


def video(case):
    """🎞️ Manifest video and the roadmap script of one case."""
    entry = next(item for item in MANIFEST if item["id"] == case["id"])
    return entry, ENERGY / case["family"] / "🎬️clip/🗺️roadmap/🐍️.py"


class RoadmapFirstContract(unittest.TestCase):
    """🧭 The roadmap is the first segment of the first chapter and never the chapter poster."""

    def test_first_segment_of_first_chapter_is_the_roadmap_of_its_family(self):
        """1️⃣ Compare the manifest with the independent golden cases."""
        for case in CASES["videos"]:
            with self.subTest(video=case["id"]):
                entry, script = video(case)
                chapter = entry["chapters"][0]
                first = chapter["segments"][0]
                self.assertEqual(first["scene"], CASES["scene"])
                self.assertEqual(first["script"], "🏛️architecture/⚡️energy/" + case["family"] + "/🎬️clip/🗺️roadmap/🐍️.py")
                self.assertNotIn("clauses", first)
                self.assertTrue(script.is_file())
                self.assertGreater(len(chapter["segments"]), 1)
                self.assertNotEqual(chapter.get("poster", {}).get("segment"), 0)
                self.assertEqual(sum(1 for item in chapter["segments"] if item["scene"] == CASES["scene"]), 1)

    def test_no_other_chapter_repeats_the_roadmap(self):
        """🔁 Only the first chapter of each video carries it."""
        for case in CASES["videos"]:
            entry, _ = video(case)
            later = [segment for chapter in entry["chapters"][1:] for segment in chapter["segments"] if segment["scene"] == CASES["scene"]]
            self.assertEqual(later, [], case["id"])

    def test_every_script_has_a_speech_and_a_composition_tool(self):
        """🔊 The per-module narration and composition tools exist next to every roadmap scene."""
        for case in CASES["videos"]:
            _, script = video(case)
            self.assertTrue((script.parent / "🔊️speech/🐍️.py").is_file(), case["id"])
            self.assertTrue((script.parent / "🏗️composition/🐍️.py").is_file(), case["id"])


class RoadmapDeclarationContract(unittest.TestCase):
    """📜 Narration, steps, chips and bracket of every roadmap, read straight from the source."""

    def test_narration_is_an_intro_plus_one_bilingual_clause_per_step(self):
        """🗣️ Keys follow the steps, both languages are written and symbols stay inside dollar signs."""
        for case in CASES["videos"]:
            with self.subTest(video=case["id"]):
                _, script = video(case)
                narration, steps = declared(script, "NARRATION"), declared(script, "STEPS")
                self.assertEqual([row[0] for row in narration], ["intro", *case["steps"]])
                self.assertEqual([step["key"] for step in steps], case["steps"])
                for key, english, german in narration:
                    self.assertTrue(english.strip() and german.strip(), key)
                    self.assertEqual(english.count("$") % 2, 0, key)
                    self.assertEqual(german.count("$") % 2, 0, key)

    def test_the_intro_names_the_number_of_steps(self):
        """🔢 The first clause counts the cards, German and English."""
        words = {5: ("five", "fünf"), 6: ("six", "sechs"), 7: ("seven", "sieben")}
        for case in CASES["videos"]:
            _, script = video(case)
            intro = declared(script, "NARRATION")[0]
            english, german = words[len(case["steps"])]
            self.assertIn(english, intro[1])
            self.assertIn(german, intro[2])

    def test_steps_carry_name_symbols_and_a_known_pictogram(self):
        """🧩 Every card has a bilingual name, symbols and a pictogram that the builder can draw."""
        for case in CASES["videos"]:
            _, script = video(case)
            for step in declared(script, "STEPS"):
                self.assertEqual(len(step["name"]), 2, step["key"])
                self.assertTrue(step["symbols"], step["key"])
                self.assertIn(step["pictogram"], PICTOGRAM_NAMES, step["key"])

    def test_hand_over_chips_name_the_other_video(self):
        """🔖 Outgoing chips hang under the last card, incoming chips under their own card; both name the partner video."""
        for case in CASES["videos"]:
            with self.subTest(video=case["id"]):
                _, script = video(case)
                chips = [chip["text"][1] for chip in declared(script, "HANDOVERS", [])]
                self.assertEqual(len(chips), len(case["handovers"]))
                for chip, partner in zip(chips, case["handovers"]):
                    self.assertIn(partner, chip)
                received = {step["key"]: [chip["text"][1] for chip in step["receives"]] for step in declared(script, "STEPS") if "receives" in step}
                self.assertEqual(sorted(received), sorted(case["receives"]))
                for key, partners in case["receives"].items():
                    self.assertEqual(len(received[key]), len(partners))
                    for chip, partner in zip(received[key], partners):
                        self.assertIn(partner, chip)

    def test_bracket_spans_the_steps_that_hold_in_every_season(self):
        """🌗 Only ventilation brackets cards, from the first up to the one before the hand-over."""
        for case in CASES["videos"]:
            _, script = video(case)
            if case["bracket_last"] is None:
                self.assertIsNone(declared(script, "BRACKET", None), case["id"])
            else:
                self.assertEqual(declared(script, "BRACKET")["last"], case["bracket_last"])
                self.assertEqual(len(case["steps"]) - 2, case["bracket_last"])


class RoadmapGeometryContract(unittest.TestCase):
    """📐 The card row stays inside the frame for every step count, checked independently with numpy."""

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable the roadmap module with SEMIO_TEST_MANIM=1")
    def test_layout_fits_the_frame_and_keeps_every_gap(self):
        """🧱 Cards neither overlap nor leave the safe span; the row is centred."""
        import numpy as np

        module = importlib.import_module(MODULE)
        for count in range(2, 8):
            with self.subTest(count=count):
                width, gap, centres = module.layout(count)
                centres = np.asarray(centres)
                left, right = centres - width / 2, centres + width / 2
                self.assertEqual(len(centres), count)
                self.assertTrue(np.allclose(np.diff(centres), width + gap))
                self.assertTrue(np.all(left[1:] - right[:-1] > gap - 1e-9))
                self.assertLessEqual(right[-1] - left[0], CASES["frame_span"] + 1e-9)
                self.assertAlmostEqual(float(centres.sum()), 0.0, places=9)
                self.assertGreaterEqual(width, 1.6)

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable the roadmap module with SEMIO_TEST_MANIM=1")
    def test_validation_rejects_inconsistent_declarations(self):
        """🛂 Mismatching keys, a missing intro or a single card never reach the drawing code."""
        module = importlib.import_module(MODULE)
        for case in CASES["rejected"]:
            with self.subTest(reason=case["reason"]):
                narration = [(key, "x", "y") for key in case["narration_keys"]]
                steps = [{"key": key, "pictogram": "bars"} for key in case["step_keys"]]
                with self.assertRaises(ValueError):
                    module.validate_roadmap(narration, steps)
        with self.assertRaises(ValueError):
            module.validate_roadmap([("intro", "x", "y"), ("a", "x", "y"), ("b", "x", "y")], [{"key": "a", "pictogram": "bars"}, {"key": "b", "pictogram": "unknown"}])

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable the roadmap module with SEMIO_TEST_MANIM=1")
    def test_every_declared_roadmap_validates_against_the_builder(self):
        """✅ The real scenes of all five videos pass the builder's own checks."""
        module = importlib.import_module(MODULE)
        for case in CASES["videos"]:
            with self.subTest(video=case["id"]):
                _, script = video(case)
                module.validate_roadmap(declared(script, "NARRATION"), declared(script, "STEPS"),
                                        declared(script, "BRACKET", None),
                                        declared(script, "HANDOVERS", []))


def _pictogram_names() -> set[str]:
    """🖼️ Pictogram keys declared in the builder source, read without importing manim."""
    tree = ast.parse((ENERGY / "🏗️authoring/🗺️roadmap/🐍️.py").read_text(encoding="utf-8"))
    table = next(node for node in tree.body if isinstance(node, ast.Assign) and node.targets[0].id == "PICTOGRAMS")
    return {key.value for key in table.value.keys}


PICTOGRAM_NAMES = _pictogram_names()


if __name__ == "__main__":
    unittest.main()
