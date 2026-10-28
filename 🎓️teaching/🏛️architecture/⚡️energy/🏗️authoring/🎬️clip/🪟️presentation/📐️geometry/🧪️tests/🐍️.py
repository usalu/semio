"""🧪 Verify portable chrome geometry and native typography against Chromium."""

import importlib
import json
import os
import tempfile
import unittest
from pathlib import Path

PRESENTATION = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class ExactUIParityTests(unittest.TestCase):
    def test_source_discovery_recompiles_for_new_candidate_membership_without_touching_ui_sources(self):
        from unittest.mock import patch

        browser = importlib.import_module(PRESENTATION + ".🌐️browser.🐍️")
        case = CASES["sourceMembership"]
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            first,added,ignored = (folder / case[key] for key in ("initial","addition","ignored"))
            first.write_text("export const existing = true;",encoding="utf-8")

            def compile_sources(*args,**kwargs):
                candidates = sorted(path for path in folder.rglob("*") if path.suffix in (".ts",".tsx"))
                return {"source_files":[str(path) for path in candidates],
                        "scan_sources":[{"base":str(folder),"pattern":case["pattern"],"negated":False}]}

            with patch.object(browser,"_SOURCES",()),patch.object(browser,"_IDENTITY",None), \
                    patch.object(browser,"_SCAN_SOURCES",(),create=True),patch.object(browser,"_MEMBERSHIP",None,create=True), \
                    patch.object(browser,"_call",side_effect=compile_sources) as compiler:
                self.assertIn(first,browser.source_files())
                browser.source_files()
                self.assertEqual(compiler.call_count,case["expectedCalls"]["warm"])
                added.parent.mkdir()
                added.write_text("export const added = true;",encoding="utf-8")
                self.assertIn(added,browser.source_files())
                self.assertEqual(compiler.call_count,case["expectedCalls"]["add"])
                ignored.write_text("Unrelated research",encoding="utf-8")
                browser.source_files()
                self.assertEqual(compiler.call_count,case["expectedCalls"]["ignore"])
                added.unlink()
                self.assertNotIn(added,browser.source_files())
                self.assertEqual(compiler.call_count,case["expectedCalls"]["remove"])
            race = folder / "race"
            race.mkdir()
            seed,late = race/"seed.tsx",race/"late.tsx"
            seed.write_text("export const seed = true;",encoding="utf-8")
            calls = []

            def compile_racing_candidate(*args,**kwargs):
                calls.append(True)
                candidates = [str(path) for path in race.glob("*.tsx")]
                late.write_text("export const late = true;",encoding="utf-8")
                return {"source_files":candidates,"scan_sources":[{"base":str(race),"pattern":case["pattern"],"negated":False}]}

            with patch.object(browser,"_SOURCES",()),patch.object(browser,"_IDENTITY",None), \
                    patch.object(browser,"_SCAN_SOURCES",()),patch.object(browser,"_MEMBERSHIP",None), \
                    patch.object(browser,"_call",side_effect=compile_racing_candidate):
                browser.source_files()
                self.assertIn(late,browser.source_files())
                self.assertEqual(len(calls),2)

    def assert_rect(self, actual, expected):
        for key, value in expected.items():
            self.assertAlmostEqual(actual[key], value, delta=CASES["oracle"]["layoutTolerance"], msg=key)

    def test_actual_react_window_cap_and_control(self):
        geometry = importlib.import_module(PRESENTATION + ".📐️geometry.🐍️")
        case = CASES["window"]
        result = geometry.window_geometry(case["rect"], case["titleAdvance"])
        self.assert_rect(result["title"], case["title"])
        self.assert_rect(result["control"], case["control"])
        for key, expected in (("outline", case["outline"]), ("border_outline", case["borderOutline"])):
            self.assertEqual(len(result[key]), len(expected))
            for actual, point in zip(result[key], expected):
                for a, b in zip(actual, point):
                    self.assertAlmostEqual(a, b, delta=CASES["oracle"]["layoutTolerance"])

    def test_native_labels_use_one_css_font_em_and_browser_advance(self):
        typography = importlib.import_module(PRESENTATION + ".🔤️typography.🐍️")
        for case in CASES["text"]:
            with self.subTest(label=case["value"]):
                text = typography._native_text(case["value"], 50, 60, 8 / 900, font_size=case["size"], line_height=case["size"], color="#f7f3e3")
                metrics = text._css_metrics
                self.assertAlmostEqual(metrics["advance"], case["advance"], delta=CASES["oracle"]["advanceTolerance"])
                self.assertAlmostEqual(metrics["font_size"], case["size"])
                unit = 8 / 900
                self.assertAlmostEqual(text.get_left()[0] / unit + 800, 50 - case["left"], delta=CASES["oracle"]["inkEdgeTolerance"])
                self.assertAlmostEqual(text.get_right()[0] / unit + 800, 50 + case["right"], delta=CASES["oracle"]["inkEdgeTolerance"])
                baseline = 60 + (case["fontAscent"] - case["fontDescent"]) / 2
                self.assertAlmostEqual(450 - text.get_top()[1] / unit, baseline - case["ascent"], delta=CASES["oracle"]["inkEdgeTolerance"])
                self.assertAlmostEqual(450 - text.get_bottom()[1] / unit, baseline + case["descent"], delta=CASES["oracle"]["inkEdgeTolerance"])

    def test_portable_typography_matches_browser_font_shaping(self):
        typography = importlib.import_module(PRESENTATION + ".🔤️typography.🐍️")
        for case in CASES["text"]:
            with self.subTest(label=case["value"]):
                metrics = typography.text_metrics(case["value"], case["size"])
                self.assertAlmostEqual(metrics["advance"], case["advance"], delta=CASES["oracle"]["advanceTolerance"])
                self.assertAlmostEqual(metrics["ascent"], case["fontAscent"], delta=0.5)
                self.assertAlmostEqual(metrics["descent"], case["fontDescent"], delta=0.5)

    def test_surface_math_matches_actual_css_and_bounded_browser_rasterization(self):
        surface = importlib.import_module(PRESENTATION + ".🎨️surface.🐍️")
        for case in CASES["surfaceCases"]:
            with self.subTest(level=case["level"], parent=case["parent"]):
                parent = case.get("parentHex") or (surface.surface_paint(case["parent"])["composite"] if case["parent"] else None)
                paint = surface.surface_paint(case["level"], parent=parent)
                self.assertAlmostEqual(paint["alpha"],case["alpha"])
                self.assertAlmostEqual(paint["blur_px"],case["blur"])
                self.assertAlmostEqual(paint["saturate"],case["saturate"])
                for actual,expected in zip(paint["rgb"],case["surfaceRgb"],strict=True):
                    self.assertAlmostEqual(actual / 255,expected,delta=0.00001)
                for actual,expected in zip(paint["composite_rgb"],case["composite"],strict=True):
                    self.assertAlmostEqual(actual,expected,delta=CASES["oracle"]["compositeChannelTolerance"])

    def test_production_browser_metadata_and_pixels_match_independent_real_ui_oracle(self):
        from PIL import Image

        browser = importlib.import_module(PRESENTATION + ".🌐️browser.🐍️")
        hierarchy = importlib.import_module(PRESENTATION + ".🥞️hierarchy.🐍️")
        geometry = importlib.import_module(PRESENTATION + ".📐️geometry.🐍️")
        typography = importlib.import_module(PRESENTATION + ".🔤️typography.🐍️")
        contract = hierarchy.hierarchy_contract()
        video = hierarchy.ordered_chapters()[0]
        expected = geometry.window_geometry(contract["focus"]["rect"], typography.text_metrics(video["title"]["de"], CASES["shell"]["fontSize"])["advance"] + CASES["shell"]["controlSize"] + CASES["shell"]["gap"])
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            output = Path(directory) / "parity.png"
            batch = {"schema":"semio-presentation-browser/v1","catalog":contract,"work_directory":directory,"language":"de",
                     "viewport":CASES["oracle"]["viewport"],"frames":[{"output":str(output),
                     "media":[],"hierarchy":{"opened_path":hierarchy.path(video["id"]),"revealed_path":[],"camera":{"x":0,"y":0}},
                     "cursor":{"point":None,"kind":"arrow","pressed":False}}]}
            result = browser.render_batch(batch)
            self.assertEqual(result["outputs"],[str(output)])
            frame = result["metadata"][0]
            window = next(window for window in frame["windows"] if window["id"] == video["id"])
            self.assert_rect(window["rect"], expected["rect"])
            self.assert_rect(window["title_rect"], expected["title"])
            self.assertEqual(window["title_text"].strip(), video["title"]["de"])
            self.assertEqual(window["control_text"].strip(), "")
            self.assertEqual(window["control_aria_label"], "Wiederherstellen")
            self.assertEqual(window["control_icons"], ["minimize-2"])
            self.assertEqual(window["control_count"], 1)
            self.assertTrue(window["control_in_title_chip"])
            self.assertEqual(window["title_native_button_count"], 1)
            self.assertEqual(window["right_cap_control_count"], 0)
            self.assert_rect(window["title_chip_rect"], window["title_rect"])
            for axis, size in (("x", "width"), ("y", "height")):
                self.assertGreaterEqual(window["control_rect"][axis], window["title_chip_rect"][axis])
                self.assertLessEqual(window["control_rect"][axis] + window["control_rect"][size], window["title_chip_rect"][axis] + window["title_chip_rect"][size])
            self.assert_rect(window["content_rect"], expected["body"])
            self.assertTrue(window["border_path"].startswith("M1,1"))
            self.assertEqual(window["font_size"],"11.2px")
            self.assertEqual(window["line_height"],"11.2px")
            self.assertEqual(window["padding"],"3.2px")
            self.assertEqual(window["backdropfilter"],"blur(8px) saturate(1.45)")
            self.assertAlmostEqual(frame["navbar"]["height"],28.8,delta=0.04)
            self.assertAlmostEqual(frame["footer"]["height"],28.8,delta=0.04)
            with Image.open(output) as source:
                pixels = source.convert("RGB")
                self.assertEqual(pixels.size,(1600,900))
                self.assertEqual(pixels.getpixel((100,100)),tuple(CASES["window"]["bodyPixel"]))
                self.assertEqual(pixels.getpixel((0,100)),tuple(CASES["window"]["gapPixel"]))

    def test_actual_typescript_silhouette_normalizes_small_gaps_and_title_caps(self):
        geometry = importlib.import_module(PRESENTATION + ".📐️geometry.🐍️")
        for case in CASES["silhouetteCases"]:
            with self.subTest(width=case["width"], advance=case["advance"]):
                rect = {"x":0,"y":0,"width":case["width"],"height":case["height"]}
                actual = geometry.window_geometry(rect, case["advance"])
                for key, fixture_key in (("outline","outline"),("border_outline","borderOutline")):
                    self.assertEqual(len(actual[key]),len(case[fixture_key]))
                    for point, expected in zip(actual[key],case[fixture_key]):
                        for a,b in zip(point,expected):
                            self.assertAlmostEqual(a,b)

    def test_nested_catalog_cells_share_the_same_canvas_and_focus_geometry(self):
        presentation = importlib.import_module(PRESENTATION + ".🐍️")
        contract = presentation.presentation_contract()
        for key in ("navbarHeight", "footerHeight", "inset", "titleHeight"):
            self.assertAlmostEqual(contract["chrome"][key], CASES["shell"][key])
        canvas = CASES["grid"]["canvas"]
        grids = [contract["courses"], *[course["topics"] for course in contract["courses"]],
            *[topic["videos"] for topic in contract["topics"]],
            *[clip["chapters"] for topic in contract["topics"] for clip in topic["videos"]]]
        for entries in grids:
            self.assertEqual(len({(entry["cell"]["column"],entry["cell"]["row"]) for entry in entries}), len(entries))
            for entry in entries:
                self.assertNotIn("rect",entry)
        focus = contract["focus"]["rect"]
        self.assertGreaterEqual(focus["x"],canvas["x"])
        self.assertGreaterEqual(focus["y"],canvas["y"])
        self.assertLessEqual(focus["x"]+focus["width"],canvas["x"]+canvas["width"])
        self.assertLessEqual(focus["y"]+focus["height"],canvas["y"]+canvas["height"])

