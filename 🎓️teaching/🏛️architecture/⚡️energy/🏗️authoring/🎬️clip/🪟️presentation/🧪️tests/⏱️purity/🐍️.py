"""⏱️ Verify presentation frames are pure functions of their canonical timestamp and share the timeline frame counts."""

from __future__ import annotations

import collections
import importlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
SETTINGS = {"language": "en", "audience": "spectator", "resolution": "480,270", "fps": 15.0, "profile": "preview"}


class Captured(Exception):
    """📦 Carry the browser packet out of the renderer before any browser starts."""


class FrameCounts(unittest.TestCase):
    def setUp(self):
        self.presentation = importlib.import_module(PACKAGE + ".🐍️")

    def test_counts_match_numpy_arange_oracle_with_floor(self):
        import numpy as np

        for case in CASES["frameCounts"]:
            with self.subTest(case=case):
                actual = self.presentation.frame_count(case["duration"], case["fps"], minimum=case["minimum"])
                self.assertEqual(actual, case["count"])
                self.assertEqual(actual, max(case["minimum"], len(np.arange(0, round(case["duration"], 9), 1 / case["fps"]))))

    def test_body_clock_is_the_timeline_count_of_the_decoded_duration(self):
        for case in CASES["bodyClocks"]:
            with self.subTest(case=case):
                self.assertEqual(self.presentation.frame_count(case["frames"] / case["sourceFps"], case["fps"]), case["count"])

    def test_chapter_clock_and_transitions_use_one_rule(self):
        collection = importlib.import_module(PACKAGE + ".📚️collection.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        scene = hierarchy.ordered_chapters()[0]["id"]
        clock = collection.chapter_clock(scene, 10, SETTINGS)
        for key, phase in (("focus_frames", "focus"), ("restore_frames", "restore")):
            self.assertEqual(clock[key], self.presentation.frame_count(self.presentation.transition_duration(phase, scene), 15.0, minimum=3))


class FramePurity(unittest.TestCase):
    def setUp(self):
        self.render = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        self.presentation = importlib.import_module(PACKAGE + ".🐍️")
        self.hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")

    def packet(self, requests):
        controls = {name: collections.defaultdict(lambda: [800, 450]) for name in ("open", "close", "preview")}
        captured = []

        def capture(batch, *, session=None):
            captured.extend(batch["frames"])
            raise Captured

        original_probes, original_browser = self.render._probes, self.presentation._browser.render_batch
        self.render._probes = lambda *args, **kwargs: controls
        self.presentation._browser.render_batch = capture
        try:
            with self.assertRaises(Captured):
                self.render.frame_batch(requests, SETTINGS, session=None)
        finally:
            self.render._probes, self.presentation._browser.render_batch = original_probes, original_browser
        return captured

    def test_packet_of_a_frame_is_independent_of_its_batch_neighbours(self):
        scene = self.hierarchy.ordered_chapters()[0]["id"]
        for phase in ("focus", "restore"):
            requests = [{"video_id": scene, "phase": phase, "t": t, "output": Path(tempfile.gettempdir()) / f"{phase}-{t}.png"} for t in CASES["samples"]]
            together = self.packet(requests)
            for index, request in enumerate(requests):
                with self.subTest(phase=phase, t=request["t"]):
                    alone = self.packet([request])[0]
                    self.assertEqual({key: value for key, value in alone.items() if key != "output"},
                                     {key: value for key, value in together[index].items() if key != "output"})

    def test_reordered_batch_yields_the_same_packets(self):
        scene = self.hierarchy.ordered_chapters()[0]["id"]
        requests = [{"video_id": scene, "phase": "focus", "t": t, "output": Path(tempfile.gettempdir()) / f"o-{t}.png"} for t in CASES["samples"]]
        forward = self.packet(requests)
        backward = self.packet(list(reversed(requests)))[::-1]
        strip = lambda frames: [{key: value for key, value in frame.items() if key != "output"} for frame in frames]
        self.assertEqual(strip(forward), strip(backward))


@unittest.skipUnless(os.environ.get("SEMIO_TEST_BROWSER"), "real browser pixel check: set SEMIO_TEST_BROWSER=1 (heavy; queue it)")
class BrowserFramePurity(unittest.TestCase):
    def test_frame_rendered_alone_equals_the_same_frame_inside_forward_reverse_and_shuffled_batches(self):
        render = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        scene = hierarchy.ordered_chapters()[0]["id"]
        work = Path(tempfile.mkdtemp(prefix="purity-", dir=Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()))
        samples = CASES["samples"]

        def pixels(order, tag):
            requests = [{"video_id": scene, "phase": "focus", "t": t, "output": work / f"{tag}-{t}.png"} for t in order]
            render.frame_batch(requests, SETTINGS)
            return {t: (work / f"{tag}-{t}.png").read_bytes() for t in order}

        forward = pixels(samples, "fwd")
        backward = pixels(list(reversed(samples)), "rev")
        shuffled = pixels([samples[i] for i in (3, 0, 5, 2, 4, 1)], "shuf")
        for t in samples:
            alone = pixels([t], f"one{t}")[t]
            with self.subTest(t=t):
                self.assertEqual(alone, forward[t])
                self.assertEqual(alone, backward[t])
                self.assertEqual(alone, shuffled[t])


if __name__ == "__main__":
    unittest.main()
