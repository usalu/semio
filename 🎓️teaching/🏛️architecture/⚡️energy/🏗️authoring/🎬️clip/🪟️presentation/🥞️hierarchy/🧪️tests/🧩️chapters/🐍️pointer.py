"""🖱️ Exercise measured chapter View and fullscreen controls in the actual browser."""
import importlib
import json
import os
from pathlib import Path
import tempfile
import unittest

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"


class ChapterPointerContract(unittest.TestCase):
    """🎯 Observe a real native chapter control press, release and click."""

    def test_view_hover_then_fullscreen_click_commits_the_chapter(self):
        """👆 Observe native click commitment without forcing the committed model path."""
        cases = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
        browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        case = next(case for case in cases["boundaries"] if case["level"] == "chapters")
        scene, branch = case["right"], hierarchy.path(case["right"])
        work = Path(tempfile.mkdtemp(prefix="chapter-pointer-", dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]))
        packet = {"schema": "semio-presentation-browser/v1", "viewport": {"width": 1600, "height": 900},
                  "catalog": hierarchy.hierarchy_contract(), "language": "de", "work_directory": str(work)}
        probe = {"output": str(work / "controls.png"), "media": [], "hierarchy": {
            **renderer.initial_hierarchy(), "opened_path": branch[:-1]}, "cursor": {"point": None}, "transition_elapsed": 2000}
        measured = browser.render_batch({**packet, "frames": [probe]})["metadata"][0]
        layer = next(layer for layer in measured["layers"] if layer["id"] == branch[-2])
        card = next(card for card in layer["cards"] if card["id"] == scene)
        center = lambda rect: [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]
        controls = {operation: {identity: [100, 200] for identity in branch} for operation in ("open", "close", "preview")}
        controls["preview"][scene], controls["open"][scene] = center(card["chip_rect"]), center(card["fullscreen_rect"])
        timing = hierarchy.hierarchy_contract()["timing"]
        duration = renderer.transition_duration(scene, "focus")
        times = [timing["topicTravel"] + timing["topicReveal"] / 4,
                 timing["topicTravel"] + timing["topicReveal"] + timing["topicPress"] / 2,
                 timing["topicTravel"] + timing["topicReveal"] + timing["topicPress"] + 1e-5]
        frames = []
        for index, seconds in enumerate(times):
            state = renderer.choreography(scene, "focus", seconds / duration, controls)
            frames.append({"output": str(work / (str(index) + ".png")), "media": [], "hierarchy": state["hierarchy"],
                           "cursor": {"point": state["point"], "pressed": state["pressed"]}, "transition_elapsed": 1000})
        frames[-1]["hierarchy"] = {**frames[-2]["hierarchy"]}
        result = browser.render_batch({**packet, "frames": frames})
        (work / "pointer.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
        self.assertEqual(result["metadata"][0]["hierarchy"]["opened_path"], branch[:-1])
        self.assertEqual(result["metadata"][0]["hierarchy"]["revealed_path"], branch)
        self.assertEqual(result["metadata"][2]["hierarchy"]["opened_path"], branch)
        events = [event for frame in result["metadata"] for event in frame["pointer_events"]]
        self.assertEqual([event["type"] for event in events], ["pointerdown", "pointerup", "click"])
        self.assertTrue(all(event["target"] == scene for event in events), events)
        active = [next(card for layer in sample["layers"] for card in layer["cards"] if card["id"] == scene)
                  for sample in result["metadata"][:2]]
        for point, rect in ((frames[0]["cursor"]["point"], active[0]["chip_rect"]),
                            (events[0]["point"], active[1]["fullscreen_rect"])):
            self.assertGreaterEqual(point[0], rect["x"])
            self.assertLessEqual(point[0], rect["x"] + rect["width"])
            self.assertGreaterEqual(point[1], rect["y"])
            self.assertLessEqual(point[1], rect["y"] + rect["height"])
        print("[DEBUG] Actual chapter pointer evidence", work, flush=True)


if __name__ == "__main__":
    unittest.main()
