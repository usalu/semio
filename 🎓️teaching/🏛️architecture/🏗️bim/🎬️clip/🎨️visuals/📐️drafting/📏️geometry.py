"""📏 Pure coordinate and polygon measures shared by drafting providers."""

from __future__ import annotations

import numpy as np


def P(point) -> tuple[float, float, float]:
    """📍 Coerce a 2D or 3D point to a float 3-vector."""
    values = [0.0, 0.0, 0.0]
    for index, value in enumerate(point):
        if index < 3:
            values[index] = float(value)
    return tuple(values)


def signed_area(ring) -> float:
    """📐 Shoelace area; positive when counter-clockwise."""
    pts = np.asarray([P(p)[:2] for p in ring])
    return 0.5 * float(np.sum(pts[:, 0] * np.roll(pts[:, 1], -1) - np.roll(pts[:, 0], -1) * pts[:, 1]))
