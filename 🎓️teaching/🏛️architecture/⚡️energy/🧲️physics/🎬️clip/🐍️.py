"""🎬 Render this lesson from the shared curriculum and canonical source selections."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

sys.dont_write_bytecode = True
_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_ID = "physical_fundamentals"


def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    """📚 Render the complete lesson or selected chapters through the same semantic source plan."""
    return _runtime.compose_videos([VIDEO_ID], language=language, chapter_scenes=chapters, **settings)


def main(argv: list[str] | None = None) -> int:
    """▶️ Share progress, cancellation and source reuse with the curriculum renderer."""
    return _runtime.video_main(VIDEO_ID, argv)


if __name__ == "__main__":
    raise SystemExit(main())
