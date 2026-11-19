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


def install_opengl_cairo_surface() -> None:
    """🪟 Expose Cairo stacking and bounds helpers on OpenGL mobjects for live previews."""
    import numpy as np
    from manim.mobject.opengl.opengl_mobject import OpenGLMobject

    if not hasattr(OpenGLMobject, "set_z_index"):
        def set_z_index(self, z_index: float, family: bool = True):
            """📚 Match Manim's Cairo ``Mobject.set_z_index`` on the OpenGL scene graph.

            https://docs.manim.community/en/stable/reference/manim.mobject.mobject.Mobject.html#manim.mobject.mobject.Mobject.set_z_index
            """
            self.z_index = z_index
            if family:
                for child in getattr(self, "submobjects", ()):
                    set_z_index(child, z_index, family=True)
            return self

        OpenGLMobject.set_z_index = set_z_index

    if not hasattr(OpenGLMobject, "get_critical_point"):
        def get_critical_point(self, direction):
            """📍 Match Cairo ``Mobject.get_critical_point`` via the OpenGL bounding box.

            https://docs.manim.community/en/stable/reference/manim.mobject.mobject.Mobject.html#manim.mobject.mobject.Mobject.get_critical_point
            """
            return self.get_bounding_box_point(np.asarray(direction, dtype=float))

        OpenGLMobject.get_critical_point = get_critical_point


def configure_manim() -> None:
    """🎞️ Replace upstream cwd-relative defaults while retaining explicit render configuration."""
    from manim import config

    if Path(config.media_dir) == Path("media"):
        config.update({"media_dir": str(output_root() / "m"), **MANIM_DIRECTORIES})
    install_opengl_cairo_surface()
