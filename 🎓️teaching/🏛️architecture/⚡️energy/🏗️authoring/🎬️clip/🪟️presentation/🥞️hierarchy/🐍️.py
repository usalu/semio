"""🥞 Resolve the authored topic and video hierarchy without inventing unavailable media."""
from __future__ import annotations

import copy
import importlib
import json
from contextlib import contextmanager
from contextvars import ContextVar
from pathlib import Path

_ROOT = Path(__file__).resolve().parent.parent
_CAPTURE = ContextVar("teaching_catalog", default=None)


def hierarchy_contract() -> dict:
    """📚 Return a fresh explicitly localized teaching navigation contract."""
    captured = _CAPTURE.get()
    if captured is not None:
        return copy.deepcopy(captured)
    collection = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️")
    return collection.enrich(json.loads((_ROOT / "🔣️.json").read_text(encoding="utf-8")))


@contextmanager
def capture_catalog(model: dict | None = None):
    """📸 Read one owned navigation model throughout a finite render without locking live authors."""
    captured = copy.deepcopy(hierarchy_contract() if model is None else model)
    token = _CAPTURE.set(captured)
    try:
        yield copy.deepcopy(captured)
    finally:
        _CAPTURE.reset(token)


def topic(topic_id: str) -> dict:
    """🗂️ Resolve a real first-level topic by its stable course identity."""
    found = next((entry for entry in hierarchy_contract()["topics"] if entry["id"] == topic_id), None)
    if found is None:
        raise ValueError(f"Unknown teaching topic: {topic_id}")
    return copy.deepcopy(found)


def video(video_id: str) -> dict:
    """🎬 Resolve only an authored video, retaining its source identity and placement cell."""
    for entry in hierarchy_contract()["topics"]:
        found = next((item for item in entry["videos"] if item["id"] == video_id), None)
        if found is not None:
            return copy.deepcopy(found)
    raise ValueError(f"Unknown teaching video: {video_id}")


def video_topic(video_id: str) -> dict:
    """🔗 Resolve the owning topic instead of assuming every future video belongs to energy."""
    found = next((entry for entry in hierarchy_contract()["topics"] if any(item["id"] == video_id for item in entry["videos"])), None)
    if found is None:
        raise ValueError(f"Unknown teaching video: {video_id}")
    return copy.deepcopy(found)


def source_files() -> tuple[Path, ...]:
    """🧬 Track the shared catalog, schema and typed hierarchy projection."""
    collection = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️")
    return (Path(__file__), _ROOT / "🔣️.json", _ROOT / "🧬️schema.json", *collection.source_files())


def collection_contract() -> dict:
    """🗃️ Expose the explicitly authored chapter navigation metadata."""
    module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️")
    return module.collection_contract()


def ordered_chapters() -> list[dict]:
    """📑 Preserve the canonical authored order across course, topic, clip and chapter."""
    return [{**copy.deepcopy(chapter), "course_id": course["id"], "topic_id": topic["id"], "clip_id": clip["id"]}
        for course in hierarchy_contract()["courses"] for topic in course["topics"]
        for clip in topic["videos"] for chapter in clip["chapters"]]


def chapter(scene: str) -> dict:
    """🪟 Resolve one independently playable chapter window by its canonical scene."""
    found = next((entry for entry in ordered_chapters() if entry["id"] == scene), None)
    if found is None:
        raise ValueError(f"Unknown teaching chapter: {scene}")
    return found


def path(scene: str) -> list[str]:
    """🪜 Resolve the complete four-level ancestry of a chapter window."""
    entry = chapter(scene)
    return [entry[key] for key in ("course_id", "topic_id", "clip_id", "id")]


def chapter_neighbors(scene: str) -> dict:
    """⏭️ Resolve authored adjacent chapters without guessing from their filenames."""
    scenes = [entry["id"] for entry in ordered_chapters()]
    if scene not in scenes:
        raise ValueError(f"Unknown teaching chapter: {scene}")
    index = scenes.index(scene)
    return {"previous": scenes[index - 1] if index else None,
        "next": scenes[index + 1] if index + 1 < len(scenes) else None}


def boundary(left_scene: str | None, right_scene: str | None) -> dict:
    """🤝 Choose the shared ancestor grid at every independently playable movie join."""
    prefix = []
    if left_scene is not None and right_scene is not None:
        for left, right in zip(path(left_scene)[:-1], path(right_scene)[:-1], strict=True):
            if left != right:
                break
            prefix.append(left)
    else:
        if left_scene is not None:
            chapter(left_scene)
        if right_scene is not None:
            chapter(right_scene)
    return {"path": prefix, "level": ("courses", "topics", "clips", "chapters")[len(prefix)]}
