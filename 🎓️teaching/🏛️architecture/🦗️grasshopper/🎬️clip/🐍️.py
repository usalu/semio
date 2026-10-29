"""🎬 Render the five independent Grasshopper videos through their shared source playlist."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

sys.dont_write_bytecode = True
_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_IDS = ("grasshopper_anatomy", "grasshopper_capsule", "grasshopper_tower", "grasshopper_casting", "grasshopper_trees")


def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    """📚 Compose five separate outputs from the canonical topic source scenes."""
    return _runtime.compose_videos(list(VIDEO_IDS), language=language, chapter_scenes=chapters, **settings)


def main(argv: list[str] | None = None) -> int:
    """▶️ Select all five topic videos with explicit language and shared render controls."""
    renderer = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🖌️render.🐍️")
    selection = [argument for video in VIDEO_IDS for argument in ("--video", video)]
    return renderer.main([*selection, *(sys.argv[1:] if argv is None else argv)])


if __name__ == "__main__":
    raise SystemExit(main())
