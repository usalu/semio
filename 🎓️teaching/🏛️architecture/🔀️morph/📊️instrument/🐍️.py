"""📊 Per-job work counters written as work.json beside the scene output."""

from __future__ import annotations

import atexit
import importlib
import json
import os
from pathlib import Path

WORK_ENV = "SEMIO_TEACHING_WORK"
_COUNTS: dict[str, int] = {}


def count(name: str, n: int = 1) -> None:
    """➕ Increment a named counter; nothing else happens until a snapshot is taken."""
    _COUNTS[name] = _COUNTS.get(name, 0) + n


def snapshot() -> dict:
    """📸 Counters keyed by the process job id and profile."""
    job = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️").job()
    return {"job": job.id, "profile": job.profile.name, "fingerprint": job.profile.fingerprint, "counts": dict(sorted(_COUNTS.items()))}


def write(path: Path) -> Path:
    """💾 Publish the snapshot atomically."""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(snapshot(), indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)
    return path


def reset() -> None:
    """🧹 Forget all counters (tests)."""
    _COUNTS.clear()


def _flush() -> None:
    write(Path(os.environ[WORK_ENV]))


if os.environ.get(WORK_ENV):
    atexit.register(_flush)
