"""🫥 Silhouettes: one closed outline for a multi-part drawing (convex hull of its sampled curves, mean fill). Domain-neutral geometry shared by the full morphs and by clips that need a stand-in outline; Manim and NumPy only.

Reference: https://en.wikipedia.org/wiki/Convex_hull
"""

from __future__ import annotations

import importlib

import numpy as np

_engine = importlib.import_module("manim")
_instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")

SAMPLES = (0.0, 0.25, 0.5, 0.75)


def _members(mob):
    """🧩 Every family member that owns points."""
    return [m for m in mob.get_family() if isinstance(m, _engine.VMobject) and len(m.points) and np.isfinite(m.points).all()]


def _rgb(color):
    """🎨 Colour as an RGB triple."""
    return np.array(_engine.ManimColor(color).to_rgb())


def _style(mob):
    """🎨 Mean fill, first stroke and strongest opacity of a family: (fill, fill_opacity, stroke, stroke_width, stroke_opacity)."""
    parts = _members(mob)
    weights = np.array([max(m.get_fill_opacity(), 1e-6) * max(len(m.points), 1) for m in parts])
    fills = np.array([_rgb(m.get_fill_color()) for m in parts])
    fill = _engine.ManimColor.from_rgb(tuple(np.clip((fills * weights[:, None]).sum(0) / weights.sum(), 0, 1)))
    opacity = max(m.get_fill_opacity() for m in parts)
    lead = max(parts, key=lambda m: m.get_stroke_width() * m.get_stroke_opacity())
    return fill, opacity, lead.get_stroke_color(), lead.get_stroke_width(), lead.get_stroke_opacity()


_BASIS = np.array([[(1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t**2, t**3] for t in SAMPLES])


def _outline(mob):
    """📍 Sampled outline points of every curve in a family, shape (n, 3), evaluated for all curves at once."""
    curves, loose = [], []
    for m in _members(mob):
        n = len(m.points) // 4 * 4
        if n:
            curves.append(m.points[:n].reshape(-1, 4, 3))
        else:
            loose.append(m.points)
    chunks = [np.einsum("sp,cpd->csd", _BASIS, np.concatenate(curves)).reshape(-1, 3)] if curves else []
    return np.concatenate([*chunks, *loose])


def _inside(points, corners):
    """🔹 Mask of points strictly inside a counter-clockwise convex polygon."""
    mask = np.ones(len(points), dtype=bool)
    for a, b in zip(corners, corners[1:] + corners[:1]):
        mask &= (b[0] - a[0]) * (points[:, 1] - a[1]) - (b[1] - a[1]) * (points[:, 0] - a[0]) > 1e-9
    return mask


def _hull(points):
    """🔺 Convex hull (monotone chain after a vectorised interior filter) of planar points, counter-clockwise."""
    flat = np.unique(np.round(np.asarray(points, dtype=float)[:, :2], 5), axis=0)
    if len(flat) < 3:
        return [(float(x), float(y)) for x, y in flat]
    extremes = [flat[np.argmin(flat[:, 0])], flat[np.argmin(flat[:, 1])], flat[np.argmax(flat[:, 0])], flat[np.argmax(flat[:, 1])]]
    corners = list(dict.fromkeys(tuple(e) for e in extremes))
    if len(corners) >= 3:
        flat = flat[~_inside(flat, corners)]
    unique = [(float(x), float(y)) for x, y in flat]

    def half(sequence):
        out = []
        for p in sequence:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (p[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (p[0] - out[-2][0]) <= 0:
                out.pop()
            out.append(p)
        return out[:-1]

    return half(unique) + half(unique[::-1])


def _restyle(shape, style, color):
    """🖌 Apply a captured style (and optional colour override) to a shape."""
    fill, opacity, stroke, width, stroke_opacity = style
    shape.set_fill(color or fill, opacity=opacity)
    shape.set_stroke(color or stroke, width=width, opacity=stroke_opacity)
    return shape


def _nothing():
    """⚪ Invisible zero-size shape standing in for geometry without points."""
    shape = _engine.VMobject()
    point = np.zeros(3)
    shape.set_points_as_corners([point, point + 1e-6])
    return shape.set_fill(opacity=0).set_stroke(opacity=0)


def _centre_of(mob):
    """🎯 Centre of the pointed parts of a family (origin when there are none)."""
    return _outline(mob).mean(axis=0) if _members(mob) else np.zeros(3)


def silhouette(mob, *, color=None):
    """🫥 One closed outline for a multi-part group (convex hull of its curves, mean fill) as a clean morph source; a single shape is copied; parts without points are skipped and a pointless input gives an invisible stand-in."""
    _instrument.count("full.prepare")
    parts = _members(mob)
    if not parts:
        return _nothing()
    if len(parts) == 1 and len(mob.get_family()) == 1:
        shape = mob.copy()
        return shape.set_color(color) if color else shape
    hull = _hull(_outline(mob))
    shape = _engine.VMobject()
    shape.set_points_as_corners([np.array([x, y, 0.0]) for x, y in (*hull, hull[0])])
    return _restyle(shape, _style(mob), color)
