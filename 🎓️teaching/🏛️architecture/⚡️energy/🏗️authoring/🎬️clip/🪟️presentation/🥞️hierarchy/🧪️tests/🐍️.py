"""🥞 Validate the portable topic and video hierarchy against authored curriculum and independent array geometry."""
import importlib
import json
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class HierarchyContractTest(unittest.TestCase):
    def setUp(self):
        self.hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")

    def test_six_topics_own_only_authored_videos_and_explicit_locales(self):
        curriculum = json.loads((ROOT / "🎓️teaching/🏛️architecture/📚️course/🔣️.json").read_text(encoding="utf-8"))
        source_topics = {topic["id"]: topic for topic in curriculum["topics"]}
        contract = self.hierarchy.hierarchy_contract()
        self.assertEqual([topic["id"] for topic in contract["topics"]], [topic["id"] for topic in CASES["topics"]])
        self.assertNotIn("windows", contract)
        self.assertEqual([course["id"] for course in contract["courses"]], [CASES["course"]])
        self.assertEqual(contract["courses"][0]["topics"], contract["topics"])
        for topic, sample in zip(contract["topics"], CASES["topics"], strict=True):
            with self.subTest(topic=topic["id"]):
                self.assertEqual(topic["cell"], sample["cell"])
                self.assertEqual(topic["title"], source_topics[topic["id"]]["title"])
                self.assertEqual(set(topic["title"]), set(CASES["locales"]))
                self.assertEqual([video["id"] for video in topic["videos"]], sample["videos"])
                self.assertEqual(self.hierarchy.topic(topic["id"]), topic)
                for video in topic["videos"]:
                    self.assertEqual(set(video["title"]), set(CASES["locales"]))
                    self.assertEqual(self.hierarchy.video(video["id"]), video)
                    self.assertEqual(self.hierarchy.video_topic(video["id"]), topic)
        for lookup in (self.hierarchy.topic, self.hierarchy.video, self.hierarchy.video_topic):
            with self.assertRaises((ValueError, KeyError)):
                lookup("missing")

    def test_cells_match_independent_numpy_row_major_oracle(self):
        import numpy as np

        contract = self.hierarchy.hierarchy_contract()
        for entries in [contract["courses"], contract["topics"], *[topic["videos"] for topic in contract["topics"]], *[video["chapters"] for topic in contract["topics"] for video in topic["videos"]]]:
            if not entries:
                continue
            index = np.arange(len(entries))
            expected = np.column_stack((index % 3, index // 3))
            actual = np.array([[entry["cell"]["column"], entry["cell"]["row"]] for entry in entries])
            np.testing.assert_array_equal(actual, expected)

    def test_contract_reads_are_independent_values(self):
        first = self.hierarchy.hierarchy_contract()
        first["topics"][0]["title"]["en"] = "changed"
        self.assertNotEqual(first, self.hierarchy.hierarchy_contract())

    def test_four_level_open_and_close_paths_commit_only_on_release_and_click(self):
        """🪜 Compare all four commits with independent cumulative timing arrays."""
        import numpy as np

        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        for phase in ("focus", "restore"):
            timing = self.hierarchy.hierarchy_contract()["timing" if phase == "focus" else "restoreTiming"]
            durations = np.array([timing[key] for key in CASES["commits"]["stepStages"]])
            path = CASES["commits"]["paths"][phase]
            total = 4 * durations.sum() + timing[CASES["commits"]["parkStage"]] + CASES["contentCamera"]["duration"]
            offset = CASES["contentCamera"]["duration"] if phase == "restore" else 0
            seconds = np.arange(4) * durations.sum() + durations[:3].sum() + offset
            for index, elapsed in enumerate(seconds):
                with self.subTest(phase=phase, click=index):
                    before = renderer.choreography(path[-1], phase, (elapsed - 1e-7) / total, CASES["commits"]["controls"])
                    clicked = renderer.choreography(path[-1], phase, (elapsed + 1e-7) / total, CASES["commits"]["controls"])
                    self.assertEqual(len(before["effects"]), index)
                    self.assertEqual([effect["type"] for effect in clicked["effects"]], CASES["commits"][phase][:index + 1])
                    self.assertEqual([event["type"] for event in clicked["events"]][-2:], ["release", "click"])
                    self.assertNotEqual(before["hierarchy"]["opened_path"], clicked["hierarchy"]["opened_path"])
                    self.assertEqual(len(clicked["hierarchy"]["opened_path"]), index + 1 if phase == "focus" else 3 - index)
                    self.assertTrue(before["pressed"])
                    self.assertFalse(clicked["pressed"])

    def test_content_camera_has_distinct_smooth_bounds_outside_native_navigation(self):
        """🔎 Keep content zoom separate from all four ancestor navigation operations."""
        import numpy as np

        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        for phase in ("focus", "restore"):
            timing = self.hierarchy.hierarchy_contract()["timing" if phase == "focus" else "restoreTiming"]
            total = 4 * sum(timing[key] for key in CASES["commits"]["stepStages"]) + timing[CASES["commits"]["parkStage"]] + CASES["contentCamera"]["duration"]
            path = CASES["commits"]["paths"][phase]
            for seconds, expected in CASES["contentCamera"][phase]:
                with self.subTest(phase=phase, seconds=seconds):
                    state = renderer.choreography(path[-1], phase, seconds / total, CASES["commits"]["controls"])
                    np.testing.assert_allclose(state["content_camera"], expected, atol=1e-12)
                    if phase == "restore" and seconds < CASES["contentCamera"]["duration"]:
                        self.assertEqual(state["effects"], [])
                        self.assertFalse(state["pressed"])
                        self.assertEqual(state["hierarchy"]["opened_path"], path)

    def test_json_object_order_does_not_change_choreography(self):
        """🧾 Preserve event order independently of serialized timing object key order."""
        from unittest.mock import patch

        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        original = self.hierarchy.hierarchy_contract()
        reordered = {**original, **{key: dict(reversed(list(original[key].items()))) for key in ("timing", "restoreTiming")}}
        controls = CASES["commits"]["controls"]
        for phase in ("focus", "restore"):
            scene = CASES["commits"]["paths"][phase][-1]
            for time in (0, .25, .5, .75, 1):
                expected = renderer.choreography(scene, phase, time, controls)
                with patch.object(self.hierarchy, "hierarchy_contract", return_value=reordered):
                    self.assertEqual(renderer.choreography(scene, phase, time, controls), expected)

    def test_catalog_capture_defensively_owns_inputs_reads_and_nested_scope_restoration(self):
        from copy import deepcopy

        outer, inner = (deepcopy(self.hierarchy.hierarchy_contract()) for _ in range(2))
        outer["labels"]["view"]["en"] = "Outer captured view"
        inner["labels"]["view"]["en"] = "Inner captured view"
        with self.hierarchy.capture_catalog(outer):
            outer["labels"]["view"]["en"] = "Changed caller input"
            self.assertEqual(self.hierarchy.hierarchy_contract()["labels"]["view"]["en"], "Outer captured view")
            read = self.hierarchy.hierarchy_contract()
            read["labels"]["view"]["en"] = "Changed read"
            with self.hierarchy.capture_catalog():
                self.assertEqual(self.hierarchy.hierarchy_contract()["labels"]["view"]["en"], "Outer captured view")
            with self.assertRaisesRegex(RuntimeError, "scope exit"):
                with self.hierarchy.capture_catalog(inner):
                    inner["labels"]["view"]["en"] = "Changed nested input"
                    self.assertEqual(self.hierarchy.hierarchy_contract()["labels"]["view"]["en"], "Inner captured view")
                    raise RuntimeError("scope exit")
            self.assertEqual(self.hierarchy.hierarchy_contract()["labels"]["view"]["en"], "Outer captured view")
        self.assertNotIn(self.hierarchy.hierarchy_contract()["labels"]["view"]["en"], ("Outer captured view", "Inner captured view"))

    def test_concurrent_async_catalog_captures_remain_independent(self):
        import asyncio
        from copy import deepcopy

        model = self.hierarchy.hierarchy_contract()

        async def capture(label):
            snapshot = deepcopy(model)
            snapshot["labels"]["view"]["en"] = label
            with self.hierarchy.capture_catalog(snapshot):
                await asyncio.sleep(0)
                return self.hierarchy.hierarchy_contract()["labels"]["view"]["en"]

        async def observe():
            return await asyncio.gather(capture("First captured view"), capture("Second captured view"))

        self.assertEqual(asyncio.run(observe()), ["First captured view", "Second captured view"])
