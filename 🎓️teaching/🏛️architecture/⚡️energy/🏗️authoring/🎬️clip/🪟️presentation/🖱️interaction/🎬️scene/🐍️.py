"""👆 Activate authored content through canonical native cursor events and ancestor emphasis."""

from __future__ import annotations

import atexit
import hashlib
import importlib
import json
import math
import os
import sys
from pathlib import Path

_core = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🐍️")
_ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
_RECORDS = {}


def cursor_shape(kind: str, unit: float):
    """🖱️ Draw the actual UI asset around its unshifted cursor hotspot."""
    import numpy as np
    from manim import Polygon, VGroup

    data = _core.cursor_outline(kind)
    return VGroup(*[Polygon(*[np.array([x * unit, -y * unit, 0.0]) for x, y in part["points"]],
                           fill_color=part["fill"], fill_opacity=1, stroke_color=part["stroke"],
                           stroke_width=part["stroke_width"]) for part in data["paths"]])


def _rect(mob) -> dict:
    """📏 Describe native object hit bounds independently of rendering coordinates."""
    return {"x": float(mob.get_left()[0]), "y": float(mob.get_bottom()[1]),
            "width": float(mob.width), "height": float(mob.height)}


def _draw_control(mob, role: str, *, hovered: bool, pressed: bool, selected: bool, disabled: bool) -> None:
    """🎨 Apply canonical button paint without changing its geometry or callback content."""
    style = _core.resolve_visual(role, {"hovered": hovered, "pressed": pressed, "selected": selected, "disabled": disabled})
    frame = mob[0] if len(mob.submobjects) >= 2 else mob
    frame.set_stroke(style["stroke"], width=getattr(mob, "_cursor_border_width", style["stroke_width"]), opacity=style["opacity"]).set_fill(
        style["fill"], opacity=style["fill_opacity"] * style["opacity"])
    if len(mob.submobjects) >= 2:
        mob[1].set_color(style["text"])
        for member in mob[1].family_members_with_points():
            if not hasattr(member, "_cursor_base_opacity"):
                member._cursor_base_opacity = (float(member.get_fill_opacity()), float(member.get_stroke_opacity()))
            fill, stroke = member._cursor_base_opacity
            member.set_fill(opacity=fill * style["opacity"], family=False)
            member.set_stroke(opacity=stroke * style["opacity"], family=False)


def _record(scene, events: list[dict], interval: dict) -> None:
    """🧾 Retain activity for the source render's enclosing virtual-window hover projection."""
    source = getattr(sys.modules.get(scene.__class__.__module__), "__file__", None)
    if not source:
        return
    owner = Path(source).resolve().parent
    if not owner.is_relative_to(_ROOT):
        return
    identity = hashlib.sha256(owner.relative_to(_ROOT).as_posix().encode()).hexdigest()[:10]
    key = (identity, scene.__class__.__name__)
    record = _RECORDS.setdefault(key, {"version": 1, "intervals": [], "events": []})
    record["intervals"].append(interval)
    record["events"].extend(events)


def _dump() -> None:
    """💾 Publish source activity alongside the existing measured narration trace."""
    output = os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")
    if not output:
        return
    for (identity, scene), record in _RECORDS.items():
        target = Path(output) / "trace" / identity / scene / "👆️.json"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(record, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


atexit.register(_dump)


def cursor_click(scene, target, on_activate, *, parents=(), enabled: bool = True, on_event=None,
                 move_time: float = 0.45, hover_time: float = 0.2, press_time: float = 0.15,
                 release_time: float = 0.15, role: str = "device") -> bool:
    """⚡ Hover, press and release a real control before activating its authored animation."""
    import numpy as np
    from manim import UpdateFromAlphaFunc, linear

    durations = (move_time, hover_time, press_time, release_time)
    if any(not isinstance(value, (int, float)) or not math.isfinite(value) or value < 0 for value in durations):
        raise ValueError("Cursor choreography durations must be nonnegative")
    width, height = float(scene.camera.frame_width), float(scene.camera.frame_height)
    center = np.array(getattr(scene.camera, "frame_center", [0, 0, 0]), dtype=float)
    presentation = json.loads((_ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json").read_text(encoding="utf-8"))
    cursor_unit = height / presentation["focus"]["viewport"]["height"]
    nodes = [{"id": "content", "parent": None, "role": "window",
              "rect": {"x": float(center[0]) - width / 2, "y": float(center[1]) - height / 2, "width": width, "height": height}}]
    parent_id = "content"
    for index, parent in enumerate(parents):
        identity = f"parent-{index}"
        nodes.append({"id": identity, "parent": parent_id, "role": "group", "rect": _rect(parent)})
        parent_id = identity
    identity = f"control-{id(target)}"
    nodes.append({"id": identity, "parent": parent_id, "role": role, "rect": _rect(target), "enabled": enabled})
    states = getattr(scene, "_cursor_projections", {})
    state = states.get(identity, _core.initial_state())
    point = target.get_center().copy()
    outside = np.array([center[0] + width / 2 + 0.6, point[1], 0.0])
    if abs(point[0] - center[0]) >= width / 2 or abs(point[1] - center[1]) >= height / 2:
        raise ValueError("An authored cursor target must be inside its source viewport")
    cursor = cursor_shape("arrow", cursor_unit)
    anchor = outside.copy()
    cursor.shift(anchor).set_z_index(100000)
    scene.add(cursor)
    events = []
    started = float(getattr(scene.renderer, "time", 0.0))
    planned = 0.0

    def paint(location):
        path = _core.hit_path(location[:2], nodes)
        selected = bool(state["toggles"].get(identity, False))
        _draw_control(target, role, hovered=identity in path, pressed=state["pressed"] == identity,
                      selected=selected, disabled=not enabled)
        for index, parent in enumerate(parents):
            style = _core.resolve_visual("group", {"hovered": f"parent-{index}" in path})
            frame = getattr(parent, "_cursor_frame", parent[0] if len(parent.submobjects) >= 2 else parent)
            frame.set_stroke(style["stroke"], width=style["stroke_width"])
        kind = _core.resolve_visual(role, {})["cursor"] if identity in path and enabled else "arrow"
        cursor.become(cursor_shape(kind, cursor_unit).shift(location).set_z_index(100000))

    def emit(kind, location):
        nonlocal state
        event = {"type": kind, "point": [float(location[0]), float(location[1])],
                 "time": max(started + planned, float(getattr(scene.renderer, "time", 0.0)))}
        state = _core.apply_event(state, event, nodes)
        events.append(event)
        paint(location)
        if on_event:
            on_event(event)

    travel = max(move_time, 0.001)
    entry = travel * (outside[0] - center[0] - width / 2) / (outside[0] - point[0])
    activated = False
    try:
        emit("enter", outside)
        scene.play(UpdateFromAlphaFunc(cursor, lambda _mob, alpha: paint(outside * (1 - alpha) + point * alpha)),
                   run_time=travel, rate_func=linear)
        planned += travel
        emit("move", point)
        scene.wait(hover_time)
        planned += hover_time
        emit("press", point)
        scene.wait(press_time)
        planned += press_time
        emit("release", point)
        emit("click", point)
        activated = bool(state["effects"])
        if activated:
            on_activate()
        scene.wait(release_time)
        planned += release_time
        scene.play(UpdateFromAlphaFunc(cursor, lambda _mob, alpha: paint(point * (1 - alpha) + outside * alpha)),
                   run_time=travel, rate_func=linear)
        planned += travel
        emit("leave", outside)
        states[identity] = state
        scene._cursor_projections = states
        ended = max(started + planned, float(getattr(scene.renderer, "time", 0.0)))
        _record(scene, events, {"start": started + entry, "end": ended - entry})
    finally:
        scene.remove(cursor)
    return activated
