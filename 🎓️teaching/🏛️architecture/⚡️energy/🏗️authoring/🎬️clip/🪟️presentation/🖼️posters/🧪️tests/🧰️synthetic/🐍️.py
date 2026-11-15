"""🧰 Synthetic poster images for browser tests: one solid, distinct colour per chapter, so a test can sample a card and know which poster it shows without rendering any scene."""

from __future__ import annotations

import hashlib
import importlib
import os
from contextlib import contextmanager
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
SIZE = (320, 180)


def colour(scene: str) -> tuple[int, int, int]:
    """🎨 The solid colour of a chapter's synthetic poster, derived from its scene id."""
    digest = hashlib.sha256(scene.encode("utf-8")).digest()
    return 64 + digest[0] % 160, 64 + digest[1] % 160, 64 + digest[2] % 160


def posters(directory: Path | None = None) -> dict[str, dict]:
    """🖼️ Write one synthetic poster for every authored chapter and return the `{scene: {file, sha256}}` map of a render."""
    from PIL import Image

    posters_module = importlib.import_module(_PACKAGE + ".🖼️posters.🐍️")
    root = Path(directory or Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "synthetic-posters").resolve()
    root.mkdir(parents=True, exist_ok=True)
    result = {}
    for scene in posters_module.chapters():
        path = root / f"{scene}.png"
        if not path.is_file():
            Image.new("RGB", SIZE, colour(scene)).save(path)
        result[scene] = {"file": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
    return result


def catalog() -> dict:
    """📚 The hierarchy contract with every synthetic poster attached."""
    hierarchy = importlib.import_module(_PACKAGE + ".🥞️hierarchy.🐍️")
    return {**hierarchy.hierarchy_contract(), "posters": posters()}


@contextmanager
def captured():
    """📸 Render inside a captured navigation model that owns every synthetic poster."""
    hierarchy = importlib.import_module(_PACKAGE + ".🥞️hierarchy.🐍️")
    with hierarchy.capture_catalog(catalog()) as model:
        yield model
