"""🧪 Real exports: one Manim scene with a declarative transition is rendered through the runtime as a preview job and as a final job, and the exported movies are decoded.

Preview shows only the source before the terminal frame and exactly the target from it; final shows genuine intermediate morph frames and the same exact target. Work counters, provenance and the poster at the canonical terminal frame are checked on the exported files. Enable with SEMIO_TEST_MANIM=1.
"""

from __future__ import annotations

import importlib
import json
import os
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
export = importlib.import_module(PACKAGE + ".📤️export.🐍️")
stills = importlib.import_module(PACKAGE + ".📤️export.🖼️stills.🐍️")
runtime = importlib.import_module(PACKAGE + ".⚙️runtime.🐍️")
timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
WORKSPACE = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
DOCUMENT = WORKSPACE / "🎓️teaching/🏛️architecture/🔀️morph/🧪️tests/🎭️stage/🧫️cases/🔣️.json"
FPS, WAIT, DURATION = 10, 0.5, 1.0

SCENE = '''import importlib
import sys

sys.path.insert(0, {workspace!r})
from manim import BLUE, RED, Scene, Square, VGroup

stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")


def tile(x, y, color):
    return Square(1.0, fill_color=color, fill_opacity=0.9, stroke_color=color).move_to([x, y, 0])


class Morph(Scene):
    def construct(self):
        stage = stage_module.stage(self, {document!r})
        stage.bind("demo.blocks", {{"a": tile(-4, -2, BLUE), "b": tile(-2, -2, BLUE)}})
        stage.bind("demo.tiles", lambda lod: VGroup(tile(3, 2, RED), tile(5, 2, RED)), bounds=[[2.5, 1.5, 0], [5.5, 2.5, 0]],
                   part_bounds={{"0": [[2.5, 1.5, 0], [3.5, 2.5, 0]], "1": [[4.5, 1.5, 0], [5.5, 2.5, 0]]}})
        self.add(stage.mobject("demo.blocks"))
        self.wait({wait})
        self.play(stage.transition("demo.blocks-to-tiles"))
        self.wait({wait})
'''


def classify(frame: np.ndarray) -> dict:
    red, green, blue = (frame[..., index].astype(int) for index in range(3))
    return {"red": int(((red > blue + 80) & (red > 120)).sum()), "blue": int(((blue > red + 80) & (blue > 120)).sum()),
            "mixed": int(((red > 90) & (blue > 90) & (green < 90)).sum())}


def decode(movie: Path) -> list[np.ndarray]:
    import av

    with av.open(str(movie)) as container:
        return [frame.to_ndarray(format="rgb24") for frame in container.decode(video=0)]


@unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable real segment rendering with SEMIO_TEST_MANIM=1")
class ExportEndToEndTest(unittest.TestCase):
    def render(self, folder: Path, profile: str) -> tuple[Path, Path]:
        script = folder / "clip" / "🐍️.py"
        script.parent.mkdir(parents=True, exist_ok=True)
        script.write_text(SCENE.format(workspace=str(WORKSPACE), document=str(DOCUMENT), wait=WAIT), encoding="utf-8")
        (script.parent / "🔀️transitions").mkdir(exist_ok=True)
        shutil.copyfile(DOCUMENT, script.parent / "🔀️transitions" / "🔣️.json")
        for name in ("🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py", "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py"):
            (folder / name).parent.mkdir(parents=True, exist_ok=True)
            (folder / name).write_text("authored fixture dependency", encoding="utf-8")
        media = folder / profile / "s" / "1"
        settings = {"language": "en", "profile": profile, "resolution": "160,90", "fps": FPS, "audience": "speaker"}
        with patch.object(runtime, "teaching_root", return_value=folder):
            clip = runtime.render_segment({"script": "clip/🐍️.py", "scene": "Morph"}, media, settings, output_root=folder / profile)
        return clip, script

    def test_exported_preview_and_final_movies(self):
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            results = {}
            for name in ("preview", "final"):
                clip, script = self.render(folder, name)
                frames = [classify(frame) for frame in decode(clip)]
                self.assertEqual(len(frames), 20)
                results[name] = (clip, script, frames, decode(clip))
            window, schedule = timeline.Window(WAIT, DURATION), timeline.Schedule(FPS)
            f0, f1 = timeline.first(window, schedule), timeline.terminal(window, schedule)
            self.assertEqual((f0, f1), (5, 14))

            preview = results["preview"][2]
            for frame in range(f0, f1):
                self.assertEqual((preview[frame]["red"], preview[frame]["mixed"]), (0, 0), f"preview frame {frame} shows a target pixel before the terminal frame")
                self.assertGreater(preview[frame]["blue"], 0, f"preview frame {frame} lost the source")
            self.assertEqual(preview[f1]["blue"], 0)
            self.assertGreater(preview[f1]["red"], 200)

            final = results["final"][2]
            self.assertTrue(any(final[frame]["mixed"] > 50 for frame in range(f0 + 1, f1)), "the final export has no genuine intermediate morph frame")
            self.assertEqual(final[f1]["blue"], 0)
            self.assertGreater(final[f1]["red"], 200)
            for name in ("preview", "final"):
                pixels = results[name][3]
                self.assertLess(float(np.abs(pixels[f1].astype(int) - pixels[f1 + 5].astype(int)).mean()), 3.0, f"{name} terminal frame is not the authored target")

            for name, (clip, script, _, pixels) in results.items():
                work = export.collect_work([clip.parent.parent], name)
                counts = work["counts"]
                if name == "preview":
                    self.assertEqual((counts.get("full.prepare", 0), counts.get("full.evaluate", 0)), (0, 0))
                    self.assertGreater(counts["preview.swap"], 0)
                else:
                    self.assertGreater(counts["full.prepare"], 0)
                    self.assertGreater(counts["full.evaluate"], 0)
                export.verify_work(name, work, transitions=export.declared_transitions(script))
                export.write_work(clip, work, name)
                record = export.publish_export("chapter", clip, settings={"language": "en", "audience": "speaker", "profile": name, "resolution": "160,90", "fps": FPS},
                                               movies=[clip], scripts=[script], inputs=[clip],
                                               stills={"folder": folder / name / "posters", "name": "Morph", "policy": {"kind": "transition", "id": "demo.blocks-to-tiles", "scene": "Morph"}}, scene_clips={"Morph": clip})
                reported = json.loads((clip.parent.parent / "transitions.json").read_text(encoding="utf-8"))["transitions"]
                self.assertEqual(reported, [{"id": "demo.blocks-to-tiles", "start": WAIT, "duration": DURATION, "f0": f0, "f1": f1}])
                self.assertEqual((record["profile"]["name"], record["stills"]["frame"], record["render"]["width"]), (name, f1, 160))
                np.testing.assert_allclose(self.poster(folder / name / "posters" / record["stills"]["poster"]["file"]), pixels[f1], atol=3)

            with self.assertRaises(export.WorkMismatch):
                export.verify_work("final", export.collect_work([results["preview"][0].parent.parent], "preview"), transitions=1)
            with self.assertRaises(export.ProfileMismatch):
                export.collect_work([results["preview"][0].parent.parent], "final")
            with self.assertRaises(export.ProfileMismatch):
                export.require_profile(results["preview"][0], "final")
            with self.assertRaises(export.ProfileMismatch):
                export.require_record_profile(results["preview"][0], "final")

    @staticmethod
    def poster(path: Path) -> np.ndarray:
        from PIL import Image

        with Image.open(path) as image:
            return np.asarray(image.convert("RGB"))


if __name__ == "__main__":
    unittest.main()
