"""📂 Keep generated teaching media outside authored source trees."""

from hashlib import sha256
import os
from pathlib import Path

_WORKSPACE = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
MANIM_DIRECTORIES = {
    "video_dir": "{media_dir}/v",
    "partial_movie_dir": "{media_dir}/p/{scene_name}",
    "images_dir": "{media_dir}/i",
    "text_dir": "{media_dir}/t",
    "tex_dir": "{media_dir}/x",
    "log_dir": "{media_dir}/logs",
}


def output_root() -> Path:
    """🧭 Resolve explicit output, cache override, then the workspace cache independently of cwd."""
    cache = _WORKSPACE / os.environ.get("SEMIO_TEACHING_CACHE_ROOT", ".🧬semio/🎓️teaching/⚡️cache")
    return (_WORKSPACE / os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", str(cache / "videos"))).resolve()


def speech_root(owner: Path) -> Path:
    """🗣️ Isolate generated narration and timing by the authored scene's workspace identity."""
    identity = sha256(owner.resolve().relative_to(_WORKSPACE).as_posix().encode()).hexdigest()[:10]
    directory = _WORKSPACE / os.environ.get("SEMIO_TEACHING_SPEECH_ROOT", str(output_root() / "speech"))
    return directory.resolve() / identity


def configure_manim() -> None:
    """🎞️ Replace upstream cwd-relative defaults while retaining explicit render configuration."""
    from manim import config

    if Path(config.media_dir) == Path("media"):
        config.update({"media_dir": str(output_root() / "m"), **MANIM_DIRECTORIES})
