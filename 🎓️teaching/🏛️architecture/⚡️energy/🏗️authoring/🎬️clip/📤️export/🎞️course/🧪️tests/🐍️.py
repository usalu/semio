"""🧪 Course joiner and delivery: profile-sized chaptered output, provenance and refusal of foreign masters; PyAV decodes the joined movie as oracle."""

from __future__ import annotations

import importlib
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
course = importlib.import_module(PACKAGE + ".📤️export.🎞️course.🐍️")
export = importlib.import_module(PACKAGE + ".📤️export.🐍️")
runtime = importlib.import_module(PACKAGE + ".⚙️runtime.🐍️")
profile = importlib.import_module(PACKAGE + ".🎚️profile.🐍️")

VIDEOS = [{"id": "a", "output": "Alpha"}, {"id": "b", "output": "Beta"}]
WORK = {"jobs": 1, "counts": {"preview.swap": 2}, "sources": []}


def master(folder: Path, name: str, seconds: float, colour: str) -> Path:
    movie = folder / f"{name}.mp4"
    runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-f", "lavfi", "-i", f"color=c={colour}:s=64x36:r=10:d={seconds}",
                         "-f", "lavfi", "-i", f"sine=frequency=440:duration={seconds}", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-c:a", "aac", "-shortest", str(movie)])
    return movie


class CourseTest(unittest.TestCase):
    def test_titles_and_metadata_escape(self):
        self.assertEqual(course.metadata_text("A=B;C#D"), "A\\=B\\;C\\#D")
        with self.assertRaises(ValueError):
            course.metadata_text(" ")
        with self.assertRaises(ValueError):
            course.titles(VIDEOS, "fr")

    def test_join_uses_profile_size_chapters_and_provenance(self):
        import av

        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            parts = []
            for video, seconds, colour in zip(VIDEOS, (2.0, 1.5), ("red", "blue")):
                movie = master(folder, video["output"], seconds, colour)
                parts.append((video, movie, {"work": WORK, "artifact": {"sha256": export.digest(movie)}}))
            with patch.object(course, "current_masters", return_value=parts), patch.object(runtime, "load_manifest", return_value=VIDEOS), \
                    patch.object(course, "titles", return_value={"a": "Alpha lesson", "b": "Beta lesson"}):
                output = course.join_course("en", "speaker", "preview", output_root=folder / "root", work_root=folder)
            self.assertEqual(output.name, "Architecture_course_speaker_en_480p15.mp4")
            with av.open(str(output)) as container:
                stream = container.streams.video[0]
                self.assertEqual((stream.width, stream.height, float(stream.base_rate)), (854, 480, 15.0))
                self.assertEqual([chapter["metadata"]["title"] for chapter in container.chapters()], ["Alpha lesson", "Beta lesson"])
                self.assertAlmostEqual(float(container.duration) / av.time_base, 3.5, delta=0.35)
            record = export.read_record(output)
            self.assertEqual((record["kind"], record["profile"]["name"], record["render"]["width"]), ("course", "preview", 854))
            self.assertEqual(record["inputs"], [{"file": "Alpha.mp4", "profile": "preview", "fingerprint": profile.fingerprint("preview")},
                                                {"file": "Beta.mp4", "profile": "preview", "fingerprint": profile.fingerprint("preview")}])
            self.assertEqual(record["stills"]["profile"], record["profile"])
            self.assertTrue((output.parent / "posters" / record["stills"]["poster"]["file"]).is_file())

    def test_final_course_refuses_preview_masters(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            movie = master(folder, "Alpha", 1.0, "red")
            runtime._write_state(movie.with_suffix(".render.json"), {"settings": {"profile": "preview"}, "profile": profile.fingerprint("preview"), "sources": {}})
            export.publish_record(movie, export.export_record("master", movie, profile="preview", language="en", audience="speaker", work=WORK))
            with self.assertRaisesRegex(export.ProfileMismatch, "preview export but this is a final job"):
                course.current_master(VIDEOS[0], movie, "en", "speaker", "final")
            with patch.object(runtime, "load_manifest", return_value=VIDEOS[:1]), self.assertRaisesRegex(ValueError, "foreign lesson masters for final"):
                course.current_masters("en", "speaker", "final", folder)

    def test_delivery_names_the_profile_of_every_file(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            parts = []
            for video, colour in zip(VIDEOS, ("red", "blue")):
                movie = master(folder, video["output"], 1.0, colour)
                parts.append((video, movie, {"work": WORK, "artifact": {"sha256": export.digest(movie)}}))
            with patch.object(course, "current_masters", return_value=parts):
                target = course.deliver("en", "speaker", "preview", output_root=folder / "root")
            document = json.loads(target.read_text(encoding="utf-8"))
            self.assertEqual((document["schema"], document["profile"]["name"], document["width"], document["fps"]), (course.SCHEMA, "preview", 854, 15))
            for entry in document["videos"]:
                delivered = target.parent / entry["file"]
                self.assertEqual(export.read_record(delivered)["kind"], "delivery")
                self.assertEqual(export.require_record_profile(delivered, "preview")["artifact"]["sha256"], entry["sha256"])
                with self.assertRaises(export.ProfileMismatch):
                    export.require_record_profile(delivered, "final")


if __name__ == "__main__":
    unittest.main()
