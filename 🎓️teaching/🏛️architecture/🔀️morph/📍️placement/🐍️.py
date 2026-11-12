"""📍 Logical placement for preview swaps: bounds, anchors, coordinate spaces, fit policy and affine interpolation. Pure NumPy, no Manim.

Placement is computed from logical bounds frozen at bind time, never from per-frame screen or reduced-detail mesh bounds.
An affine maps p -> scale * p + translation on x and y only (z is untouched); rotation and deformation are never part of a placement.

Degenerate policy: an axis whose source or target extent is zero cannot define a scale and is skipped; with no usable axis the scale is 1 (a zero-size source is a pure translation).
Reference: https://numpy.org/doc/stable/reference/generated/numpy.interp.html
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, Mapping, NamedTuple, Optional

import numpy as np

ANCHORS = ("center", "bottom", "top", "left", "right", "origin")
FITS = ("none", "contain", "width", "height")
SCENE = "scene"
ENTITY_PREFIX = "entity:"


class PlacementError(ValueError):
    """⛔ A placement that cannot be computed, with an actionable message."""


class Affine(NamedTuple):
    """↗ Axis-aligned scale and translation: p -> scale * p + translation (x, y)."""

    scale: tuple[float, float]
    translation: tuple[float, float]


IDENTITY = Affine((1.0, 1.0), (0.0, 0.0))


@dataclass(frozen=True)
class Placement:
    """📌 Declared placement of a source toward a target (duck-compatible with the spec placement)."""

    space: str = SCENE
    anchor: str = "center"
    fit: str = "none"
    uniform: bool = True


@dataclass(frozen=True)
class Frame:
    """🗺 Logical transform of an entity into its parent entity (None parent = scene); local p -> scale * p + translation."""

    scale: tuple[float, float] = (1.0, 1.0)
    translation: tuple[float, float] = (0.0, 0.0)
    parent: Optional[str] = None


def _pair(values, what):
    """🔢 A finite (x, y) float pair from any sequence of at least two numbers."""
    array = np.asarray(values, dtype=float).reshape(-1)
    if array.size < 2 or not np.isfinite(array[:2]).all():
        raise PlacementError(f"{what} must hold two finite numbers (x, y), got {values!r}")
    return float(array[0]), float(array[1])


@dataclass(frozen=True)
class Bounds:
    """📦 Logical axis-aligned bounds `lo <= hi` with an optional declared origin (e.g. a foot contact point)."""

    lo: tuple[float, float]
    hi: tuple[float, float]
    origin: Optional[tuple[float, float]] = None

    def __post_init__(self):
        lo, hi = _pair(self.lo, "bounds lo"), _pair(self.hi, "bounds hi")
        if lo[0] > hi[0] or lo[1] > hi[1]:
            raise PlacementError(f"bounds are inverted (lo {lo} > hi {hi}); declare logical bounds as min/max corners")
        object.__setattr__(self, "lo", lo)
        object.__setattr__(self, "hi", hi)
        if self.origin is not None:
            object.__setattr__(self, "origin", _pair(self.origin, "bounds origin"))

    @classmethod
    def of(cls, array, origin=None):
        """📦 Bounds of a (2, 2) or (2, 3) [min; max] array."""
        data = np.asarray(array, dtype=float)
        if data.ndim != 2 or data.shape[0] != 2 or data.shape[1] < 2:
            raise PlacementError(f"bounds must be a (2, 2) or (2, 3) [min; max] array, got shape {data.shape}")
        return cls(tuple(data[0, :2]), tuple(data[1, :2]), None if origin is None else tuple(np.asarray(origin, dtype=float)[:2]))

    @property
    def width(self):
        return self.hi[0] - self.lo[0]

    @property
    def height(self):
        return self.hi[1] - self.lo[1]

    @property
    def center(self):
        return (self.lo[0] + self.hi[0]) / 2, (self.lo[1] + self.hi[1]) / 2

    @property
    def degenerate(self):
        return self.width == 0 or self.height == 0

    def point(self, anchor):
        """⚓ The named anchor point of these bounds."""
        cx, cy = self.center
        if anchor == "center":
            return cx, cy
        if anchor == "bottom":
            return cx, self.lo[1]
        if anchor == "top":
            return cx, self.hi[1]
        if anchor == "left":
            return self.lo[0], cy
        if anchor == "right":
            return self.hi[0], cy
        if anchor == "origin":
            if self.origin is None:
                raise PlacementError("anchor 'origin' needs a declared logical origin; bind the entity with anchors={'origin': (x, y, z)}")
            return self.origin
        raise PlacementError(f"unknown anchor {anchor!r}; use one of {', '.join(ANCHORS)}")


def union(bounds: Iterable[Bounds]) -> Bounds:
    """➕ Smallest bounds that cover all given bounds (origin dropped)."""
    items = list(bounds)
    if not items:
        raise PlacementError("union of no bounds is undefined")
    return Bounds((min(b.lo[0] for b in items), min(b.lo[1] for b in items)), (max(b.hi[0] for b in items), max(b.hi[1] for b in items)))


def compose(outer: Affine, inner: Affine) -> Affine:
    """🔗 Affine applying `inner` first and then `outer`."""
    scale = (outer.scale[0] * inner.scale[0], outer.scale[1] * inner.scale[1])
    return Affine(scale, (outer.scale[0] * inner.translation[0] + outer.translation[0], outer.scale[1] * inner.translation[1] + outer.translation[1]))


def inverse(a: Affine) -> Affine:
    """↩ Inverse affine; raises for a zero or non-finite scale."""
    if not all(np.isfinite(a.scale)) or not all(abs(s) > 0 for s in a.scale):
        raise PlacementError(f"affine with scale {a.scale} is not invertible; a logical frame must have a finite non-zero scale")
    scale = (1.0 / a.scale[0], 1.0 / a.scale[1])
    return Affine(scale, (-a.translation[0] * scale[0], -a.translation[1] * scale[1]))


def apply(a: Affine, points):
    """📍 Apply an affine to points of shape (..., 2) or (..., 3); z is untouched."""
    out = np.array(points, dtype=float, copy=True)
    out[..., 0] = out[..., 0] * a.scale[0] + a.translation[0]
    out[..., 1] = out[..., 1] * a.scale[1] + a.translation[1]
    return out


def at(a: Affine, p: float) -> Affine:
    """⏱ The affine at eased progress `p`: exact identity at 0, exactly `a` at 1, extrapolating for overshoot."""
    q = float(p)
    if q == 0.0:
        return IDENTITY
    if q == 1.0:
        return a
    return Affine((a.scale[0] * q + (1 - q), a.scale[1] * q + (1 - q)), (a.translation[0] * q, a.translation[1] * q))


def to_scene(space: str, frames: Optional[Mapping[str, Frame]] = None) -> Affine:
    """🗺 Affine from the declared coordinate space to scene coordinates (`scene` or nested `entity:<id>` through parent frames)."""
    if space == SCENE:
        return IDENTITY
    if not space.startswith(ENTITY_PREFIX) or len(space) == len(ENTITY_PREFIX):
        raise PlacementError(f"unknown coordinate space {space!r}; use 'scene' or 'entity:<id>'")
    chain = IDENTITY
    seen = []
    entity = space[len(ENTITY_PREFIX):]
    while entity is not None:
        if entity in seen:
            raise PlacementError(f"frame cycle {' -> '.join(seen + [entity])}; entity frames must form a tree rooted at the scene")
        seen.append(entity)
        frame = (frames or {}).get(entity)
        if frame is None:
            raise PlacementError(f"no logical frame for entity {entity!r} (space {space!r}); bind it with a frame or declare placement.space 'scene'")
        local = Affine(tuple(float(v) for v in frame.scale), tuple(float(v) for v in frame.translation))
        if not np.isfinite(local.scale + local.translation).all():
            raise PlacementError(f"frame of entity {entity!r} is not finite")
        chain = compose(local, chain)
        entity = frame.parent
    return chain


def _local(bounds: Bounds, frame: Affine) -> Bounds:
    """🗺 Bounds expressed in the local coordinates of a frame (a negative scale swaps the corners)."""
    back = inverse(frame)
    a, b = apply(back, [[*bounds.lo, 0.0], [*bounds.hi, 0.0]])[:, :2]
    origin = None if bounds.origin is None else tuple(apply(back, [[*bounds.origin, 0.0]])[0, :2])
    return Bounds(tuple(np.minimum(a, b)), tuple(np.maximum(a, b)), origin)


def _ratio(target: float, source: float):
    """➗ Scale ratio of one axis or None when either extent is zero."""
    return None if source <= 0 or target <= 0 else target / source


def _scale(source: Bounds, target: Bounds, fit: str, uniform: bool):
    """📏 Scale factors (x, y) for a fit policy; axes without extent are skipped."""
    if fit == "none":
        return 1.0, 1.0
    rx, ry = _ratio(target.width, source.width), _ratio(target.height, source.height)
    if fit == "width":
        axes = [rx]
    elif fit == "height":
        axes = [ry]
    else:
        axes = [rx, ry]
    usable = [r for r in axes if r is not None]
    if not usable:
        return 1.0, 1.0
    if uniform:
        s = min(usable) if fit == "contain" else usable[0]
        return s, s
    return (rx if rx is not None and fit in ("contain", "width") else 1.0), (ry if ry is not None and fit in ("contain", "height") else 1.0)


def affine(source_bounds: Bounds, target_bounds: Bounds, placement, *, frames: Optional[Mapping[str, Frame]] = None) -> Affine:
    """🎯 The affine that places the source's anchor on the target's anchor after the fit scale (scene coordinates in, scene affine out).

    Anchor, fit and `uniform` are evaluated in `placement.space` (nested entity frames resolved through their parents); the returned affine is conjugated back to scene.
    """
    anchor, fit, uniform = getattr(placement, "anchor"), getattr(placement, "fit"), bool(getattr(placement, "uniform"))
    if fit not in FITS:
        raise PlacementError(f"unknown fit {fit!r}; use one of {', '.join(FITS)}")
    if anchor not in ANCHORS:
        raise PlacementError(f"unknown anchor {anchor!r}; use one of {', '.join(ANCHORS)}")
    frame = to_scene(getattr(placement, "space"), frames)
    source, target = (_local(source_bounds, frame), _local(target_bounds, frame)) if frame != IDENTITY else (source_bounds, target_bounds)
    sx, sy = _scale(source, target, fit, uniform)
    a, b = source.point(anchor), target.point(anchor)
    local = Affine((sx, sy), (b[0] - sx * a[0], b[1] - sy * a[1]))
    return local if frame == IDENTITY else compose(frame, compose(local, inverse(frame)))
