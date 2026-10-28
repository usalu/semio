"""🎬 Render the standalone optimization sampling video through the shared curriculum interface."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

sys.dont_write_bytecode = True
_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_ID = "optimization_sampling"


def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    """📚 Compose one independent output from its canonical authored scene."""
    return _runtime.compose_videos([VIDEO_ID], language=language, chapter_scenes=chapters, **settings)


def main(argv: list[str] | None = None) -> int:
    """▶️ Reuse explicit language, progress, cancellation and audience controls."""
    return _runtime.video_main(VIDEO_ID, argv)


if __name__ == "__main__":
    raise SystemExit(main())
