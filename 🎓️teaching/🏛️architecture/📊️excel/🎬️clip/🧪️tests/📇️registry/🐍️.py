"""📇 Verify that every Excel video is registered, routed and titled consistently across the shared curriculum files."""

from __future__ import annotations

import importlib
import importlib.util
import json
import os
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

sys.dont_write_bytecode = True
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
HERE = Path(__file__).resolve().parent
ROOT = next(parent for parent in HERE.parents if (parent / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
CASES = json.loads((HERE / "🔣️.cases.json").read_text(encoding="utf-8"))
SERIES = CASES["series"]
CLIP = ROOT / "🎓️teaching/🏛️architecture" / CASES["folder"] / "🎬️clip"
AUTHORING = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip"


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def load(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class RegistryTest(unittest.TestCase):
    def test_curriculum_registers_standalone_videos(self):
        """📚 Every video has one manifest entry, one canonical scene, one output and one presentation placement."""
        videos = {video["id"]: video for video in read(ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json")["videos"]}
        expected = [chapter["video_id"] for chapter in CASES["chapters"]]
        manifest_ids = [video["id"] for video in videos.values() if video["id"].startswith(SERIES + "_")]
        self.assertEqual(manifest_ids, expected)
        outputs = []
        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                video = videos[chapter["video_id"]]
                self.assertEqual(video["script"], chapter["entry"])
                self.assertEqual([item["scene"] for item in video["chapters"]], [chapter["chapter"]])
                segments = video["chapters"][0]["segments"]
                self.assertEqual(len(segments), 1)
                self.assertEqual(segments[0]["scene"], chapter["scene"])
                self.assertEqual((ROOT / "🎓️teaching" / segments[0]["script"]).resolve(), (CLIP / chapter["folder"] / "🐍️.py").resolve())
                self.assertTrue((ROOT / "🎓️teaching" / video["script"]).is_file())
                outputs.append(video["output"])
        self.assertEqual(outputs, [chapter["output"] for chapter in CASES["chapters"]])
        self.assertEqual(len(set(outputs)), len(expected))
        presentation = read(AUTHORING / "🪟️presentation/🔣️.json")
        topic = next(topic for topic in presentation["topics"] if topic["id"] == SERIES)
        self.assertEqual([video["id"] for video in topic["videos"]], expected)
        self.assertEqual(len({(video["cell"]["column"], video["cell"]["row"]) for video in topic["videos"]}), len(expected))

    def test_titles_agree_between_scenes_collection_and_presentation(self):
        """🏷️ One bilingual title per video is identical in the scene, the chapter map and the presentation topic."""
        collection = read(AUTHORING / "🪟️presentation/📚️collection/🔣️.json")["chapters"]
        presentation = read(AUTHORING / "🪟️presentation/🔣️.json")
        topic = next(topic for topic in presentation["topics"] if topic["id"] == SERIES)
        titles = {video["id"]: video["title"] for video in topic["videos"]}
        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                scene = getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{CASES['folder']}.🎬️clip.{chapter['folder']}.🐍️"), chapter["scene"])
                self.assertEqual(set(scene.TITLE), set(CASES["languages"]))
                self.assertEqual(collection[chapter["chapter"]], scene.TITLE)
                self.assertEqual(titles[chapter["video_id"]], scene.TITLE)
                self.assertTrue(all(scene.TITLE[language].strip() for language in CASES["languages"]))

    def test_entry_points_route_only_their_own_scenes(self):
        """🎬 Each standalone entry composes exactly its own video id."""
        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                module = load(chapter["video_id"], ROOT / "🎓️teaching" / chapter["entry"])
                self.assertEqual(module.VIDEO_ID, chapter["video_id"])
                for language in CASES["languages"]:
                    with patch.object(module._runtime, "compose_videos", return_value={}) as compose:
                        module.compose(language=language, audience="speaker", chapters=[chapter["chapter"]], force=True)
                    compose.assert_called_once_with([chapter["video_id"]], language=language, audience="speaker", chapter_scenes=[chapter["chapter"]], force=True)

    def test_playlist_routes_all_videos(self):
        """📚 The series playlist composes every video in curriculum order."""
        module = load(SERIES + "_playlist", CLIP / "🐍️.py")
        expected = [chapter["video_id"] for chapter in CASES["chapters"]]
        self.assertEqual(list(module.VIDEO_IDS), expected)
        with patch.object(module._runtime, "compose_videos", return_value={}) as compose:
            module.compose(language="en", audience="spectator", force=True)
        compose.assert_called_once_with(expected, language="en", audience="spectator", chapter_scenes=None, force=True)

    def test_every_scene_is_bilingual_with_unique_keys_and_transcripts_exist(self):
        """🗣️ Each scene narrates in both languages and the series publishes both transcripts."""
        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                scene = getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{CASES['folder']}.🎬️clip.{chapter['folder']}.🐍️"), chapter["scene"])
                keys = [row[0] for row in scene.NARRATION]
                self.assertEqual(len(keys), len(set(keys)))
                self.assertTrue(all(len(row) == 3 and row[1].strip() and row[2].strip() for row in scene.NARRATION))
        for language in CASES["languages"]:
            transcript = (CLIP / f"🗣️transcript.{language}.md").read_text(encoding="utf-8")
            for chapter in CASES["chapters"]:
                self.assertIn(f"`{chapter['video_id']}`", transcript)


if __name__ == "__main__":
    unittest.main()
