"""🧲 Fit the Nakagin building drawing into a bounded screen area from the model numbers alone, without drawing it. Shared by the Grasshopper and Excel clips."""

from __future__ import annotations

import importlib

import numpy as np

_model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_view = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️").make_view


def building_points(*, towers=(0, 1), podium: bool = True, capital: bool = True, core: bool = True, capsules: bool = True) -> np.ndarray:
    """📦 World vertices (m) of the model envelopes that ``tower_axon`` draws: capsule casings, cores, capitals and podium."""
    solids = [_model.podium_solid()] if podium else []
    for tower in (_model.towers()[i] for i in towers):
        solids += ([_model.core_solid(tower)] if core else []) + ([_model.capital_solid(tower)] if capital else [])
    if capsules:
        solids += [_model.slot_solid(slot) for slot in _model.slots() if slot.tower in towers]
    return np.array([vertex for solid in solids for vertex in solid.vertices], float)


def building_fit(width: float, height: float, *, centre=(0.0, 0.0, 0.0), view="axon", top: float = 0.0, bottom: float = 0.0, margin: float = 0.12, **parts) -> dict:
    """🧲 ``at`` and ``scale`` of a ``tower_axon``/``tower_elevation`` call that fits the drawn envelopes inside a ``width`` × ``height`` area around ``centre``,
    below a ``top`` label row, above a ``bottom`` row and inside ``margin``; ``parts`` are the drawing flags (``towers``, ``podium``, ``capital``, ``core``, ``capsules``).
    The drawing is centred on the middle of its model box, so its projected box lands centred in the free area."""
    points = building_points(**parts)
    projection = _view(view, (0, 0, 0), 1.0, (points.min(axis=0) + points.max(axis=0)) / 2)
    flat = np.array([projection(*point)[:2] for point in points])
    low, high = flat.min(axis=0), flat.max(axis=0)
    room = np.array([width - 2 * margin, height - top - bottom - 2 * margin])
    scale = float(min(room / (high - low)))
    area = np.array(tuple(centre)[:2], float) + np.array([0.0, (bottom - top) / 2])
    return {"at": np.append(area - scale * (low + high) / 2, 0.0), "scale": scale}
