"""🧪 Run browser lifecycle contracts through the regular authoring test workflow."""

import importlib
import json
import os
import shutil
import tempfile
import unittest
from pathlib import Path

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())


class BrowserLifecycleTest(unittest.TestCase):
    def test_registered_node_adapter_runs_all_portable_lifecycle_cases(self):
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
        output.mkdir(parents=True,exist_ok=True)
        work = Path(tempfile.mkdtemp(prefix="lifecycle-",dir=output)).resolve()
        log = work / "run.log"
        runtime.run_command([browser._repo(),"tools","exec","--light","node","--",str(Path(__file__).with_name("🟨️.mjs"))],cwd=ROOT,log=log)
        transcript = log.read_text(encoding="utf-8")
        cases = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
        for case in cases["cases"]:
            self.assertIn("PASS "+case["id"],transcript)
        if work.parent != output or not work.name.startswith("lifecycle-"):
            raise RuntimeError("Lifecycle test cleanup escaped its owned output directory")
        shutil.rmtree(work)


class BrowserHandoffAssetTest(unittest.TestCase):
    def setUp(self):
        from copy import deepcopy
        from unittest.mock import patch

        self.browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
        self.fixture = json.loads((Path(__file__).parent / "🏁️handoff/🔣️.cases.json").read_text(encoding="utf-8"))
        self.directory = self.enterContext(tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]))
        self.work = Path(self.directory)
        self.source = self.work / "ui.tsx"
        self.source.write_text("export const paint = 1;",encoding="utf-8")
        self.sources = [self.source]
        self.enterContext(patch.object(self.browser,"source_files",side_effect=lambda:tuple(self.sources)))
        self.enterContext(patch.object(self.browser,"_HANDOFF_ASSETS",{}))
        frame = {"output":str(self.work/"frame.png"),"canonical_overview":True,"media":[],
                 "hierarchy":deepcopy(self.fixture["hierarchy"]),"cursor":deepcopy(self.fixture["cursor"])}
        self.batch = {"schema":"semio-presentation-browser/v1","catalog":hierarchy.hierarchy_contract(),
                      "language":"de","viewport":self.fixture["viewport"],"work_directory":str(self.work),"frames":[frame]}

    def test_portable_handoff_eligibility_excludes_interaction_and_media(self):
        from copy import deepcopy

        base = self.batch["frames"][0]
        for sample in self.fixture["eligibility"]:
            with self.subTest(case=sample["id"]):
                frame = deepcopy(base)
                for key,value in sample["change"].items():
                    frame[key] = {**frame[key],**value} if isinstance(value,dict) else value
                self.assertEqual(self.browser._canonical_overview(frame),sample["eligible"])

    def test_producer_metadata_is_preserved_with_its_versioned_handoff_pixels(self):
        from copy import deepcopy
        from unittest.mock import patch
        from PIL import Image

        calls = []

        def capture(batch,**kwargs):
            calls.append(deepcopy(batch))
            for frame in batch["frames"]:
                Image.new("RGB",tuple(batch["viewport"][key] for key in ("width","height")),(len(calls),20,30)).save(frame["output"])
            return {"outputs":[frame["output"] for frame in batch["frames"]],"metadata":[{"actualCapture":len(calls)}]}

        with patch.object(self.browser,"_call",side_effect=capture):
            first = self.browser.render_batch(deepcopy(self.batch))
            reference = Path(self.batch["frames"][0]["output"]).read_bytes()
            second = self.browser.render_batch(deepcopy(self.batch))
            self.assertEqual(len(calls),2)
            self.assertEqual(second["metadata"][0]["actualCapture"],1)
            self.assertFalse(first["metadata"][0]["handoff"]["reused"])
            self.assertTrue(second["metadata"][0]["handoff"]["reused"])
            self.assertEqual(Path(self.batch["frames"][0]["output"]).read_bytes(),reference)
            reordered = deepcopy(self.batch)
            reordered["catalog"] = dict(reversed(list(reordered["catalog"].items())))
            reordered["viewport"] = dict(reversed(list(reordered["viewport"].items())))
            self.assertTrue(self.browser.render_batch(reordered)["metadata"][0]["handoff"]["reused"])
            for variant in self.fixture["identityVariants"]:
                with self.subTest(input=variant):
                    self.source.write_text("export const paint = 1;",encoding="utf-8")
                    self.sources[:] = [self.source]
                    batch = deepcopy(self.batch)
                    if variant == "catalog":
                        batch["catalog"]["labels"]["view"]["en"] = "A changed label"
                    elif variant == "language":
                        batch["language"] = "en"
                    elif variant == "viewport":
                        batch["viewport"] = {"width":64,"height":36}
                    elif variant == "ui-code":
                        self.source.write_text("export const paint = 2;",encoding="utf-8")
                    elif variant == "ui-membership":
                        added = self.work / "new-ui.tsx"
                        added.write_text("export const newPaint = true;",encoding="utf-8")
                        self.sources.append(added)
                    elif variant == "camera":
                        batch["frames"][0]["hierarchy"]["camera"]["x"] = 0
                    elif variant == "cursor":
                        batch["frames"][0]["cursor"]["point"] = [1500,800]
                    result = self.browser.render_batch(batch)
                    self.assertFalse(result["metadata"][0]["handoff"]["reused"])
                    self.assertNotEqual(result["metadata"][0]["handoff"]["identity"],first["metadata"][0]["handoff"]["identity"])

    def test_ui_source_change_during_capture_cannot_publish_a_handoff_asset(self):
        from unittest.mock import patch
        from PIL import Image

        def capture(batch,**kwargs):
            self.source.write_text("export const paint = 2;",encoding="utf-8")
            Image.new("RGB",(32,18)).save(batch["frames"][0]["output"])
            return {"metadata":[{}]}

        with patch.object(self.browser,"_call",side_effect=capture),self.assertRaisesRegex(RuntimeError,"sources changed"):
            self.browser.render_batch(self.batch)
        self.assertEqual(self.browser._HANDOFF_ASSETS,{})


    def test_each_ancestor_prefix_reuses_only_its_own_exact_captured_pixels(self):
        """🧩 Compare per-prefix raster reuse with independently painted Pillow samples."""
        from copy import deepcopy
        from unittest.mock import patch
        from PIL import Image

        calls, originals = [], []

        def capture(batch, **kwargs):
            """🎨 Produce a unique native raster for every actual capture."""
            calls.append(deepcopy(batch))
            for frame in batch["frames"]:
                Image.new("RGB", (32, 18), (len(calls), 20, 30)).save(frame["output"])
            return {"outputs": [frame["output"] for frame in batch["frames"]],
                    "metadata": [{"actualCapture": len(calls)} for frame in batch["frames"]]}

        with patch.object(self.browser, "_call", side_effect=capture):
            for prefix in self.fixture["prefixes"]:
                with self.subTest(prefix=prefix):
                    batch = deepcopy(self.batch)
                    batch["frames"][0]["hierarchy"]["opened_path"] = prefix
                    first = self.browser.render_batch(deepcopy(batch))
                    path = Path(batch["frames"][0]["output"])
                    reference = path.read_bytes()
                    originals.append(reference)
                    second = self.browser.render_batch(deepcopy(batch))
                    self.assertFalse(first["metadata"][0]["handoff"]["reused"])
                    self.assertTrue(second["metadata"][0]["handoff"]["reused"])
                    self.assertEqual(path.read_bytes(), reference)
                    self.assertEqual(second["metadata"][0]["actualCapture"], len(calls) - 1)
                    self.assertEqual(Image.open(path).getpixel((0, 0)), (len(calls) - 1, 20, 30))
        self.assertEqual(len(set(originals)), len(self.fixture["prefixes"]))


class LayeredBrowserOracleTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
        output.mkdir(parents=True, exist_ok=True)
        work = Path(tempfile.mkdtemp(prefix="layered-", dir=output)).resolve()
        script = Path(__file__).parent / "🥞️layered/🟨️.mjs"
        runtime.run_command([browser._repo(), "tools", "exec", "--light", "node", "--", str(script), str(work)], cwd=ROOT, log=work / "run.log")
        fixture = json.loads(script.with_name("🔣️.cases.json").read_text(encoding="utf-8"))
        measured = json.loads((work / "oracle.json").read_text(encoding="utf-8"))
        hierarchy = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
        cls.work, cls.output, cls.fixture, cls.measured, cls.catalog = work, output, fixture, measured, hierarchy.hierarchy_contract()

    @classmethod
    def tearDownClass(cls):
        work, output = cls.work, cls.output
        if work.parent != output or not work.name.startswith("layered-"):
            raise RuntimeError("Layered oracle cleanup escaped its owned output directory")
        shutil.rmtree(work)

    def test_shared_components_match_portable_pan_reveal_open_and_return_cases(self):
        self.assertEqual(self.measured["schema"], self.fixture["schema"])
        self.assertEqual(self.measured["errors"], [])
        self.assertEqual([(sample["language"], sample["scenario"]) for sample in self.measured["samples"]],
                         [(language, scenario["id"]) for language in self.fixture["locales"] for scenario in self.fixture["scenarios"]])

    def test_production_four_levels_match_shared_glass_and_exact_hover_placement(self):
        """🥞️ Compare every hierarchy grid with independently rendered shared components."""
        browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
        branch = self.fixture["branch"]
        course = self.catalog["courses"][0]
        topic = next(entry for entry in course["topics"] if entry["id"] == branch[1])
        clip = next(entry for entry in topic["videos"] if entry["id"] == branch[2])
        levels = [(self.catalog["courses"], "courses"), (course["topics"], "topics"), (topic["videos"], "videos"), (clip["chapters"], "chapters")]
        for language in self.fixture["locales"]:
            with self.subTest(language=language):
                frames = []
                for depth, (entries, scenario) in enumerate(levels):
                    prefix, target = branch[:depth], entries[-1]["id"]
                    states = [{"opened_path": prefix, "revealed_path": [], "camera": {"x": 0, "y": 0}},
                              {"opened_path": prefix, "revealed_path": [], "camera": {"x": 1, "y": 1}},
                              {"opened_path": prefix, "revealed_path": [*prefix, target], "camera": {"x": 1, "y": 1}},
                              {"opened_path": [*prefix, target], "revealed_path": [], "camera": {"x": 1, "y": 1}}]
                    for index, hierarchy in enumerate(states):
                        frames.append({"output": str(self.work / f"production-{language}-{depth}-{index}.png"),
                            "media": [], "hierarchy": hierarchy, "cursor": {"point": None}, "transition_elapsed": 1800})
                batch = {"schema": "semio-presentation-browser/v1", "viewport": {"width": 320, "height": 180},
                    "catalog": self.catalog, "language": language, "work_directory": str(self.work), "frames": frames}
                result = browser.render_batch(batch)
                for depth, (entries, scenario) in enumerate(levels):
                    with self.subTest(depth=depth):
                        identity = branch[depth - 1] if depth else "root"
                        layers = [next(layer for layer in result["metadata"][4 * depth + offset]["layers"] if layer["id"] == identity) for offset in range(4)]
                        resting, panned, hover, opened = layers
                        self.assertEqual([card["id"] for card in resting["cards"]], [entry["id"] for entry in entries])
                        if len(entries) > 1:
                            self.assertNotEqual(resting["strip_transform"], panned["strip_transform"])
                        oracle = next(sample["resting"] for sample in self.measured["samples"] if sample["language"] == language and sample["scenario"] == scenario)
                        self.assertEqual(resting["veil"]["background"], oracle["veil"]["background"])
                        self.assertEqual(resting["veil"]["backdrop_filter"], oracle["veil"]["backdropFilter"])
                        for card in resting["cards"]:
                            entry = next(entry for entry in entries if entry["id"] == card["id"])
                            self.assertEqual(card["title_text"].strip(), entry["title"][language])
                            self.assertEqual(card["chip_text"].strip(), self.fixture["labels"][language])
                            self.assertEqual(card["chip_icons"], self.fixture["openChip"]["footerIcons"])
                            self.assertEqual(card["fullscreen_icons"], [self.fixture["openChip"]["icon"]])
                            self.assertTrue(card["fullscreen_in_title_chip"])
                            self.assertEqual(card["glass_background"], oracle["cards"][0]["background"])
                            self.assertEqual(card["backdrop_filter"], oracle["cards"][0]["backdropFilter"])
                            self.assertGreater(card["chip_rect"]["x"], card["rect"]["x"] + card["rect"]["width"] / 2)
                            self.assertGreater(card["chip_rect"]["y"], card["rect"]["y"] + card["rect"]["height"] / 2)
                            for axis, size in (("x", "width"), ("y", "height")):
                                self.assertGreaterEqual(card["fullscreen_rect"][axis], card["title_chip_rect"][axis])
                                self.assertLessEqual(card["fullscreen_rect"][axis] + card["fullscreen_rect"][size], card["title_chip_rect"][axis] + card["title_chip_rect"][size])
                        target = entries[-1]["id"]
                        pane = next(pane for pane in hover["panes"] if pane["id"] == target)
                        self.assertTrue(pane["inert"])
                        self.assertFalse(pane["opened"])
                        self.assertTrue(next(card for card in hover["cards"] if card["id"] == target)["revealed"])
                        self.assertEqual(hover["veil"]["visibility"], "hidden")
                        for key in ("x", "y", "width", "height"):
                            self.assertAlmostEqual(pane["rect"][key], hover["rect"][key], delta=self.fixture["geometryTolerance"])
                        pane = next(pane for pane in opened["panes"] if pane["id"] == target)
                        self.assertTrue(pane["opened"])
                        self.assertFalse(pane["inert"])
                control = next(window for window in result["metadata"][-1]["windows"] if window["id"] == branch[-1])
                entry = clip["chapters"][0]
                self.assertEqual(control["title_text"].strip(), entry["title"][language])
                self.assertEqual(control["control_text"].strip(), "")
                self.assertEqual(control["control_aria_label"], self.fixture["restoreChip"]["labels"][language])
                self.assertEqual(control["control_icons"], [self.fixture["restoreChip"]["icon"]])
                self.assertEqual(control["control_count"], self.fixture["restoreChip"]["controlCount"])
                self.assertTrue(control["control_in_title_chip"])
                self.assertEqual(control["title_native_button_count"], self.fixture["restoreChip"]["titleButtonCount"])
                self.assertEqual(control["right_cap_control_count"], self.fixture["restoreChip"]["rightCapControlCount"])
                for axis, size in (("x", "width"), ("y", "height")):
                    self.assertGreaterEqual(control["control_rect"][axis], control["title_chip_rect"][axis])
                    self.assertLessEqual(control["control_rect"][axis] + control["control_rect"][size], control["title_chip_rect"][axis] + control["title_chip_rect"][size])
                self.assertAlmostEqual(control["title_chip_rect"]["x"], control["rect"]["x"], delta=self.fixture["geometryTolerance"])
                self.assertGreater(control["title_chip_rect"]["width"], control["title_chip_rect"]["height"])

    def test_native_pointer_hover_and_press_require_same_target_release_to_open_each_level(self):
        browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
        branch = self.fixture["branch"]
        for depth, target in enumerate(branch):
            prefix = branch[:depth]
            level = branch[depth - 1] if depth else "root"
            with self.subTest(level=level):
                hierarchy = {"opened_path": prefix, "revealed_path": [], "camera": {"x": 0, "y": 0}}
                batch = {"schema": "semio-presentation-browser/v1", "viewport": {"width": 320, "height": 180},
                         "catalog": self.catalog, "language": "de", "work_directory": str(self.work), "frames": []}
                seed = {"output": str(self.work / f"click-{level}-seed.png"), "media": [], "hierarchy": hierarchy,
                        "cursor": {"point": None}, "transition_elapsed": 1000}
                initial = browser.render_batch({**batch, "frames": [seed]})["metadata"][0]
                layer = next(layer for layer in initial["layers"] if layer["id"] == level)
                rect = next(card["fullscreen_rect"] for card in layer["cards"] if card["id"] == target)
                point = [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]
                frames = [{**seed, "output": str(self.work / f"click-{level}-{index}.png"),
                           "cursor": {"point": point, "pressed": pressed}}
                          for index, pressed in enumerate((False, True, False))]
                metadata = browser.render_batch({**batch, "frames": frames})["metadata"]
                self.assertFalse(any(event["type"] == "click" for event in metadata[0]["pointer_events"]))
                self.assertEqual([event["type"] for event in metadata[1]["pointer_events"]], ["pointerdown"])
                self.assertEqual([event["type"] for event in metadata[2]["pointer_events"]], ["pointerup", "click"])
                self.assertTrue(all(event["target"] == target for state in metadata[1:] for event in state["pointer_events"]))
                for index, state in enumerate(metadata[:2]):
                    layer = next(layer for layer in state["layers"] if layer["id"] == level)
                    pane = next(pane for pane in layer["panes"] if pane["id"] == target)
                    self.assertTrue(pane["inert"])
                    self.assertFalse(pane["opened"])
                    if index == 0:
                        self.assertTrue(next(card for card in layer["cards"] if card["id"] == target)["revealed"])
                layer = next(layer for layer in metadata[2]["layers"] if layer["id"] == level)
                pane = next(pane for pane in layer["panes"] if pane["id"] == target)
                self.assertTrue(pane["opened"])
                self.assertFalse(pane["inert"])
                cancelled = [{**frames[1], "output": str(self.work / f"cancel-{level}-down.png")},
                             {**frames[2], "output": str(self.work / f"cancel-{level}-up.png"),
                              "cursor": {"point": [1599, 899], "pressed": False}}]
                last = browser.render_batch({**batch, "frames": cancelled})["metadata"][-1]
                layer = next(layer for layer in last["layers"] if layer["id"] == level)
                self.assertFalse(any(pane["opened"] for pane in layer["panes"]))

    def test_native_fused_restore_chip_closes_chapter_on_release_in_each_locale(self):
        browser = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
        branch = self.fixture["branch"]
        chapter, clip_id = branch[-1], branch[-2]
        for language in self.fixture["locales"]:
            with self.subTest(language=language):
                state = {"opened_path": branch, "revealed_path": [], "camera": {"x": 0, "y": 0}}
                seed = {"output": str(self.work / f"restore-{language}-seed.png"), "media": [],
                        "hierarchy": state, "cursor": {"point": None}, "transition_elapsed": 1000}
                batch = {"schema": "semio-presentation-browser/v1", "viewport": {"width": 320, "height": 180},
                         "catalog": self.catalog, "language": language, "work_directory": str(self.work), "frames": [seed]}
                initial = browser.render_batch(batch)["metadata"][0]
                window = next(window for window in initial["windows"] if window["id"] == chapter)
                entry = next(chapter for topic in self.catalog["topics"] for clip in topic["videos"] for chapter in clip["chapters"] if chapter["id"] == branch[-1])
                self.assertEqual(window["control_text"].strip(), "")
                self.assertEqual(window["control_aria_label"], self.fixture["restoreChip"]["labels"][language])
                self.assertEqual(window["title_text"].strip(), entry["title"][language])
                self.assertTrue(window["control_in_title_chip"])
                self.assertEqual(window["control_icons"], [self.fixture["restoreChip"]["icon"]])
                self.assertEqual(window["control_count"], self.fixture["restoreChip"]["controlCount"])
                self.assertEqual(window["title_native_button_count"], self.fixture["restoreChip"]["titleButtonCount"])
                self.assertEqual(window["right_cap_control_count"], self.fixture["restoreChip"]["rightCapControlCount"])
                for axis, size in (("x", "width"), ("y", "height")):
                    self.assertGreaterEqual(window["control_rect"][axis], window["title_chip_rect"][axis])
                    self.assertLessEqual(window["control_rect"][axis] + window["control_rect"][size], window["title_chip_rect"][axis] + window["title_chip_rect"][size])
                name = window["title_name_rect"]
                name_point = [name["x"] + name["width"] / 2, name["y"] + name["height"] / 2]
                name_frames = [{**seed, "output": str(self.work / f"name-{language}-{index}.png"),
                                "cursor": {"point": name_point, "pressed": pressed}}
                               for index, pressed in enumerate((False, True, False))]
                name_metadata = browser.render_batch({**batch, "frames": name_frames})["metadata"]
                for frame in name_metadata:
                    energy = next(layer for layer in frame["layers"] if layer["id"] == clip_id)
                    self.assertTrue(next(pane for pane in energy["panes"] if pane["id"] == chapter)["opened"])
                self.assertFalse(any(event["target"] == chapter + ":fullscreen" for frame in name_metadata for event in frame["pointer_events"]))
                rect = window["control_icon_rect"]
                for axis, size in (("x", "width"), ("y", "height")):
                    self.assertGreaterEqual(rect[axis], window["control_rect"][axis])
                    self.assertLessEqual(rect[axis] + rect[size], window["control_rect"][axis] + window["control_rect"][size])
                point = [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]
                frames = [{**seed, "output": str(self.work / f"restore-{language}-{index}.png"),
                           "cursor": {"point": point, "pressed": pressed}}
                          for index, pressed in enumerate((False, True, False))]
                metadata = browser.render_batch({**batch, "frames": frames})["metadata"]
                for frame in metadata[:2]:
                    energy = next(layer for layer in frame["layers"] if layer["id"] == clip_id)
                    self.assertTrue(next(pane for pane in energy["panes"] if pane["id"] == chapter)["opened"])
                self.assertEqual([event["type"] for event in metadata[1]["pointer_events"]], ["pointerdown"])
                self.assertEqual([event["type"] for event in metadata[2]["pointer_events"]], ["pointerup", "click"])
                self.assertTrue(all(event["target"] == chapter + ":fullscreen" for frame in metadata[1:] for event in frame["pointer_events"]))
                energy = next(layer for layer in metadata[2]["layers"] if layer["id"] == clip_id)
                self.assertFalse(any(pane["opened"] for pane in energy["panes"]))
                self.assertTrue(next(pane for pane in energy["panes"] if pane["id"] == chapter)["inert"])
