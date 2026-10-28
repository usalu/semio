"""🖌️ Render shared native scientific controls and cursors without a duplicate presentation shell."""

from __future__ import annotations

import importlib
from functools import lru_cache
from pathlib import Path

_core = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🐍️")
_scene = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🎬️scene.🐍️")


def _presentation():
    """🧩 Resolve the shared native presentation port without import-time cycles."""
    return importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")


@lru_cache(maxsize=2)
def _icon(name):
    """🔲 Load the authored control SVG behind the native scientific rendering interface."""
    from xml.etree import ElementTree
    from manim import Rectangle, SVGMobject, VGroup

    p = _presentation()
    source = p._theme.ROOT / _core.interaction_contract()["icons"][name]
    document = ElementTree.parse(source).getroot()
    x, y, width, height = (float(value) for value in document.attrib["viewBox"].split())
    glyph = SVGMobject(str(source), height=None, width=None, should_center=False)
    glyph.shift([0, -float(glyph.get_top()[1] + glyph.get_bottom()[1]), 0])
    viewport = Rectangle(width=width, height=height, stroke_width=0, stroke_opacity=0, fill_opacity=0)
    viewport.move_to([x + width / 2, -y - height / 2, 0])
    icon = VGroup(viewport, glyph).center()
    icon._svg_viewport_width = width
    icon._svg_stroke_width = float(document.get("stroke-width", "2"))
    return icon


def _button(rect, style, *, icon=None, label=None, text_height=11.2):
    """🔘 Draw a real canonical control cell and its state-dependent ink."""
    from manim import VGroup

    p = _presentation()
    frame = p._box(rect, color=style["stroke"], fill=style["fill"], opacity=style["fill_opacity"], stroke=style["stroke_width"])
    parts = [frame]
    if icon:
        glyph = _icon(icon).copy()
        size = _core.interaction_contract()["control"]["iconSize"]
        glyph.set_stroke(style["text"], width=glyph._svg_stroke_width * size / glyph._svg_viewport_width).set_fill(opacity=0)
        glyph[0].set_stroke(width=0, opacity=0)
        glyph.set_width(size * p._UNIT).move_to(p._point(rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2))
        parts.append(glyph)
    if label:
        parts.append(p._text(label, rect["x"] + p._geometry.geometry_metrics()["spacing"], rect["y"] + rect["height"] / 2, font_size=text_height, line_height=text_height, color=style["text"]))
    group = VGroup(*parts).fade(1 - style["opacity"])
    group._cursor_border_width = style["stroke_width"]
    return group


def cursor_layer(image, settings, point, kind):
    """👆 Composite the canonical native cursor after authored bitmap content, preserving its hotspot."""
    from manim import Camera

    p = _presentation()
    width, height, _, _ = p._settings(settings)
    camera = Camera(pixel_width=width, pixel_height=height, frame_width=128 / 9, frame_height=8,
                    background_color=p._theme.color("dark"), background_opacity=0, cairo_line_width_multiple=p._UNIT)
    cursor = _scene.cursor_shape(kind, p._UNIT).shift(p._point(*point))
    camera.capture_mobjects([cursor])
    image.alpha_composite(camera.get_image().convert("RGBA"))
    return image
