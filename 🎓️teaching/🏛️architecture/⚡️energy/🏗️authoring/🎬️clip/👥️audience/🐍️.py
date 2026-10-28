"""👥 Resolve the two authored video audiences through a portable contract."""

from __future__ import annotations

import json
import os
from pathlib import Path

_ROOT = Path(__file__).resolve().parent
_VARIANTS = tuple(json.loads((_ROOT / "🔣️.json").read_text(encoding="utf-8"))["variants"])


def audiences() -> tuple[str, ...]:
    """🎭 List the distinct videos produced by the render command."""
    return tuple(variant["id"] for variant in _VARIANTS)


def subtitles_visible(audience: str | None = None) -> bool:
    """💬 Apply the selected audience to every authored caption, including direct scene renders."""
    selected = os.environ.get("SEMIO_TEACHING_AUDIENCE", "speaker") if audience is None else audience
    for variant in _VARIANTS:
        if selected == variant["id"]:
            return variant["subtitles"] == "overlay"
    raise ValueError(f"Unknown clip audience: {selected!r}")


def source_files() -> tuple[Path, ...]:
    """🔏 Include audience policy in every render fingerprint."""
    return (Path(__file__), _ROOT / "🔣️.json", _ROOT / "🧬️schema.json")
