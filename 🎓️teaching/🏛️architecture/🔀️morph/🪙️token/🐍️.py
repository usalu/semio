"""🪙 Shared morph primitives: abstract tokens, travel along a leader and tether leaders. Used by both render profiles; Manim only.

Reference: https://docs.manim.community/en/stable/reference/manim.animation.movement.MoveAlongPath.html
"""

from __future__ import annotations

import importlib

import numpy as np

_engine = importlib.import_module("manim")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")

INK, MUTED, DATA, GOOD, WARN, DARK = _art.P_WHITE, _art.PASTEL_GREY, _art.P_YELLOW, _art.P_GREEN, _art.P_ORANGE, _art.P_DEEP_DARK
KINDS = ("item", "cell", "path", "number", "plane", "text")
BASE = 0.4


def _text(label, size, color, mono=False):
    """🔤 Token label in the bundled sans or mono face."""
    if mono:
        return _engine.Text(label, font=_theme.FONT_MONO, font_size=size, color=color, disable_ligatures=True)
    return _fonts.body_text(label, font_size=size, color=color)


def _pill(text, color, corner, fill, text_color=None):
    """💊 Rounded box around a label; returns (box, text)."""
    height = BASE
    width = max(height, text.width + 0.26)
    box = _engine.RoundedRectangle(width=width, height=height, corner_radius=min(corner, height / 2, width / 2), stroke_color=color, stroke_width=2.0, fill_color=fill, fill_opacity=1.0 if text_color else 0.18)
    return box, text.move_to(box)


def token(label, *, kind="item", color=DATA, size=1.0):
    """🪙 Abstract stand-in with one visual language: item (dot and value), cell (chip), path (`{0;1;3;1}` pill), number (filled badge), plane (square with axes), text (glyph)."""
    if kind not in KINDS:
        raise ValueError(f"unknown token kind {kind!r}")
    label = str(label)
    small = 16.2 * size
    if kind == "item":
        dot = _engine.Dot(radius=0.1 * size, color=color)
        ring = _engine.Circle(radius=0.19 * size, stroke_color=color, stroke_width=1.5, stroke_opacity=0.6, fill_opacity=0).move_to(dot)
        parts = [ring, dot] + ([_text(label, small, INK).next_to(ring, _engine.RIGHT, buff=0.08 * size)] if label else [])
    elif kind == "cell":
        parts = list(_pill(_text(label, small, INK), color, 0.05, color))
    elif kind == "path":
        parts = list(_pill(_text(label, small, color, mono=True), color, 1.0, color))
    elif kind == "number":
        parts = list(_pill(_text(label, small, DARK), color, 1.0, color, text_color=DARK))
    elif kind == "plane":
        side = BASE * 1.1 * size
        square = _engine.Square(side_length=side, stroke_color=color, stroke_width=2.0, fill_color=color, fill_opacity=0.18)
        origin = square.get_corner(_engine.DL)
        x = _engine.Arrow(origin, origin + _engine.RIGHT * side * 0.8, buff=0, stroke_width=3, color=_art.P_RED, max_tip_length_to_length_ratio=0.35)
        y = _engine.Arrow(origin, origin + _engine.UP * side * 0.8, buff=0, stroke_width=3, color=GOOD, max_tip_length_to_length_ratio=0.35)
        parts = [square, x, y] + ([_text(label, small, INK).next_to(square, _engine.RIGHT, buff=0.1 * size)] if label else [])
    else:
        parts = [_text(label, small * 1.15, color)]
    group = _engine.VGroup(*parts)
    group.kind = kind
    return group.move_to(_engine.ORIGIN)


def travel(tok, path, *, run_time=1.2, rate_func=_engine.smooth):
    """🚚 Run a token along a wire or leader (a path mobject or a list of points); it rests at the end."""
    if not isinstance(path, _engine.Mobject):
        path = _engine.VMobject().set_points_smoothly([np.array(p, dtype=float) for p in path])
    return _engine.MoveAlongPath(tok, path, run_time=run_time, rate_func=rate_func)


def _centre(x):
    """🎯 Centre of a mobject or a point."""
    return np.array(x.get_center() if isinstance(x, _engine.Mobject) else x, dtype=float)


def _edge(x, toward):
    """🧭 Point where the ray from the centre of `x` towards `toward` leaves its bounding box."""
    centre = _centre(x)
    if not isinstance(x, _engine.Mobject):
        return centre
    delta = toward - centre
    scale = min((x.width / 2) / abs(delta[0]) if abs(delta[0]) > 1e-9 else np.inf, (x.height / 2) / abs(delta[1]) if abs(delta[1]) > 1e-9 else np.inf, 1.0)
    return centre + delta * scale


def tether(a, b, *, color=DATA, dashed=True, width=2.0):
    """🪢 Leader between two mobjects (or points) that follows both ends while they move."""

    def build():
        ca, cb = _centre(a), _centre(b)
        p, q = _edge(a, cb), _edge(b, ca)
        if np.dot(cb - ca, q - p) <= 1e-4:
            return _engine.VMobject()
        if dashed:
            return _engine.DashedLine(p, q, color=color, stroke_width=width, dash_length=0.08, dashed_ratio=0.55)
        return _engine.Line(p, q, color=color, stroke_width=width)

    return _engine.always_redraw(build)
