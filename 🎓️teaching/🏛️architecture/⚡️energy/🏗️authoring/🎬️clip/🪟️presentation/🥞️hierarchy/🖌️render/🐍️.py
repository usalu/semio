"""🥞 Render topic and video navigation through the actual shared layered browser UI."""
from __future__ import annotations

import copy
import importlib
import math
import os
import uuid
from pathlib import Path

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
_hierarchy = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
_core = importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🖱️interaction.🐍️")


def _presentation():
    """🧩 Resolve the presentation port without an import-time cycle."""
    return importlib.import_module(_PACKAGE + ".🎬️clip.🪟️presentation.🐍️")


def _camera(point):
    """📷 Normalize the actual cursor against the shared canvas bounds."""
    contract = _hierarchy.hierarchy_contract()
    top, bottom = (contract["chrome"][key] for key in ("navbarHeight", "footerHeight"))
    return {"x": min(1, max(0, point[0] / 1600)), "y": min(1, max(0, (point[1] - top) / (900 - top - bottom)))}


def initial_hierarchy():
    """🌱 Return the common root boundary camera and empty opened path."""
    return {"opened_path": [], "revealed_path": [],
            "camera": _camera(_core.interaction_contract()["timeline"]["park"])}


def _center(rect):
    """🎯 Aim at a measured native DOM control without estimating card geometry."""
    return [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]


def _mix(start, end, fraction):
    """🖱️ Ease cursor travel while leaving all layered camera movement to shared UI."""
    value = max(0, min(1, fraction))
    value = value * value * (3 - 2 * value)
    return [start[index] * (1 - value) + end[index] * value for index in (0, 1)]


def _packet(frames, work, settings):
    """📦 Own browser outputs under one explicitly localized finite batch."""
    width, height, _, language = _presentation()._settings(settings)
    return {"schema": "semio-presentation-browser/v1", "work_directory": str(work.resolve()),
            "viewport": {"width": width, "height": height}, "language": language, "frames": frames,
            "catalog": _hierarchy.hierarchy_contract()}


def _probes(video_id, work, settings, session):
    """📐 Measure actual controls on every ancestor grid and the chapter window."""
    branch = _hierarchy.path(video_id)
    token = uuid.uuid4().hex[:8]
    paths = [work / f"probe-{token}-{index}.png" for index in range(5)]
    states = [{**initial_hierarchy(), "opened_path": branch[:index]} for index in range(5)]
    frames = [{"output": str(path), "hierarchy": state, "media": [],
        "cursor": {"point": None, "kind": "arrow", "pressed": False},
        "transition_elapsed": 2000, "canonical_overview": index < 4}
        for index, (path, state) in enumerate(zip(paths, states, strict=True))]
    try:
        metadata = _presentation()._browser.render_batch(_packet(frames, work, settings), session=session)["metadata"]
        opening, closing, preview = {}, {}, {}
        for depth, identity in enumerate(branch):
            layer_id = branch[depth - 1] if depth else "root"
            layer = next(layer for layer in metadata[depth]["layers"] if layer["id"] == layer_id)
            card = next(card for card in layer["cards"] if card["id"] == identity)
            opening[identity] = _center(card["fullscreen_rect"])
            preview[identity] = _center(card["chip_rect"])
            if depth == 3:
                window = next(window for window in metadata[4]["windows"] if window["id"] == identity)
                closing[identity] = _center(window["control_rect"])
            else:
                target = "footer:path:" + branch[depth - 1] if depth else "footer:root"
                closing[identity] = _center(next(control for control in metadata[depth + 1]["chrome"]["ancestors"] if control["id"] == target)["rect"])
        return {"open": opening, "close": closing, "preview": preview}
    finally:
        for path in paths:
            path.unlink(missing_ok=True)


def _route(video_id, phase):
    """🧭 Resolve only the ancestor operations needed for the adjacent authored chapter."""
    if phase not in ("focus", "restore"):
        raise ValueError("A focus or restore transition phase is required")
    branch = _hierarchy.path(video_id)
    neighbors = _hierarchy.chapter_neighbors(video_id)
    shared = _hierarchy.boundary(neighbors["previous"], video_id) if phase == "focus" else _hierarchy.boundary(video_id, neighbors["next"])
    prefix = shared["path"]
    steps = branch[len(prefix):] if phase == "focus" else list(reversed(branch[len(prefix):]))
    timing = _hierarchy.hierarchy_contract()["timing" if phase == "focus" else "restoreTiming"]
    durations = [timing[key] for key in ("topicTravel", "topicReveal", "topicPress", "topicHold")]
    return branch, prefix, steps, durations, timing["focusHold"]


def transition_duration(video_id, phase):
    """⏱️ Account for every real ancestor click and the content-only camera move."""
    _, _, steps, durations, park = _route(video_id, phase)
    return len(steps) * sum(durations) + park + _hierarchy.hierarchy_contract()["contentCamera"]["duration"]


def choreography(video_id, phase, t, controls):
    """🎬 Project explicit pointer clicks through the complete four-level course path."""
    if not isinstance(t, (int, float)) or not math.isfinite(t):
        raise ValueError("A finite hierarchy time is required")
    t = min(1, max(0, t))
    branch, prefix, steps, durations, park_duration = _route(video_id, phase)
    step_duration = sum(durations)
    camera_duration = _hierarchy.hierarchy_contract()["contentCamera"]["duration"]
    navigation_duration = len(steps) * step_duration + park_duration
    elapsed = t * (navigation_duration + camera_duration)
    offset = camera_duration if phase == "restore" else 0
    seconds = max(0, min(navigation_duration, elapsed - offset))
    fraction = max(0, min(1, (elapsed - navigation_duration) / camera_duration if phase == "focus" else elapsed / camera_duration))
    eased = fraction * fraction * (3 - 2 * fraction)
    content_camera = eased if phase == "focus" else 1 - eased
    park, idle = (_core.interaction_contract()["timeline"][key] for key in ("park", "idlePark"))
    initial, terminal = (park, idle) if phase == "focus" else (idle, park)
    opened = list(prefix if phase == "focus" else branch)
    revealed, effects, events, point, pressed, stage = [], [], [], list(initial), False, "park"
    previous = initial
    operation = "open" if phase == "focus" else "close"
    for index, identity in enumerate(steps):
        start = index * step_duration
        travel, reveal, press, hold = durations
        release = start + travel + reveal + press
        target = controls[operation][identity]
        depth = branch.index(identity)
        if seconds >= release:
            opened = branch[:depth + 1] if phase == "focus" else branch[:depth]
            effect = {"type": ("course", "topic", "clip", "chapter")[depth] + "." + operation, "target": identity}
            effects.append(effect)
        hover = controls["preview"][identity] if phase == "focus" else target
        events.extend({"type": kind, "time": at + offset, "point": list(position)}
            for kind, at, position in (("move", start + travel, hover), ("press", start + travel + reveal, target),
                ("release", release, target), ("click", release, target)) if at <= seconds)
        if start <= seconds < start + step_duration:
            local = seconds - start
            stage = ("course", "topic", "clip", "chapter")[depth] + "." + operation
            hover = controls["preview"][identity] if phase == "focus" else target
            if travel and local < travel:
                point = _mix(previous, hover, local / travel)
            elif phase == "focus" and local < travel + reveal:
                move = min(0.2, reveal)
                point = _mix(hover, target, (local - travel - reveal + move) / move) if move else list(target)
            else:
                point = list(target)
            pressed = travel + reveal <= local < travel + reveal + press
            if phase == "focus" and travel <= local < travel + reveal + press:
                revealed = branch[:depth + 1]
        previous = target
    if seconds >= len(steps) * step_duration:
        point = _mix(previous, terminal, (seconds - len(steps) * step_duration) / park_duration) if park_duration else list(terminal)
    if t == 0:
        point = list(initial)
    if t == 1:
        point = list(terminal)
    hierarchy = {"opened_path": list(opened), "revealed_path": revealed, "camera": _camera(point)}
    return {"phase": phase, "t": t, "seconds": elapsed, "navigation_seconds": seconds,
        "content_camera": content_camera, "stage": stage, "point": point, "pressed": pressed,
        "hierarchy": hierarchy, "fullscreen": video_id if len(opened) == 4 else None,
        "zoom": int(len(opened) == 4), "events": events, "effects": effects,
        "action": effects[-1] if effects else None}


def _nodes(browser):
    """🌳 Expose only measured actual DOM targets and active ancestry for verification."""
    contract = _hierarchy.hierarchy_contract()
    nodes = [{"id": "app", "parent": None, "role": "root", "rect": {"x": 0, "y": 0, "width": 1600, "height": 900}},
             {"id": "canvas", "parent": "app", "role": "canvas", "rect": {"x": 0, "y": contract["chrome"]["navbarHeight"],
              "width": 1600, "height": 900 - contract["chrome"]["navbarHeight"] - contract["chrome"]["footerHeight"]}}]
    branch = browser["hierarchy"]["opened_path"]
    ancestors = branch[:-1] if len(branch) == 4 else branch
    parent = "canvas"
    for identity in ancestors:
        nodes.append({"id": identity, "parent": parent, "role": "window", "rect": dict(nodes[1]["rect"]), "z": 1})
        parent = identity
    for layer in browser["layers"]:
        if not layer["interactive"]:
            continue
        for card in layer["cards"]:
            nodes.extend(({"id": card["id"], "parent": parent, "role": "window", "rect": card["rect"], "z": 30},
                          {"id": card["id"] + ":tab", "parent": card["id"], "role": "tab", "rect": card["title_chip_rect"], "z": 31},
                          {"id": card["id"] + ":fullscreen", "parent": card["id"], "role": "fullscreen", "rect": card["fullscreen_rect"], "z": 32},
                          {"id": card["id"] + ":view", "parent": card["id"], "role": "button", "rect": card["chip_rect"], "z": 31}))
    for window in browser["windows"]:
        layer = next((layer for layer in browser["layers"] if any(pane["id"] == window["id"] and pane["opened"] for pane in layer["panes"])), None)
        if layer is not None:
            identity = window["id"]
            nodes.extend(({"id": identity, "parent": branch[-2] if len(branch) == 4 else parent, "role": "window", "rect": window["rect"], "z": 10},
                          {"id": identity + ":tab", "parent": identity, "role": "tab", "rect": window["title_rect"], "z": 10},
                          {"id": identity + ":fullscreen", "parent": identity, "role": "fullscreen", "rect": window["control_rect"], "z": 10}))
    nodes.extend(({"id": "navbar", "parent": "app", "role": "navbar", "rect": browser["navbar"], "z": 100},
                  {"id": "footer", "parent": "app", "role": "footer", "rect": browser["footer"], "z": 100}))
    for identity, owner, rect in (("navbar:energy", "navbar", browser["chrome"]["energy"]), ("footer:range", "footer", browser["chrome"]["range"])):
        nodes.append({"id": identity, "parent": owner, "role": "button", "rect": rect, "z": 101})
    if browser["menu"]:
        menu_rect = browser["menu"]["rect"]
        nav = next(node for node in nodes if node["id"] == "navbar")
        nav["rect"] = {**nav["rect"], "height": menu_rect["y"] + menu_rect["height"]}
        nodes.append({"id": "navbar:tree", "parent": "navbar", "role": "group", "rect": menu_rect, "z": 110})
        for row in browser["chrome"]["treeRows"]:
            nodes.append({"id": row["id"].removesuffix(".row"), "parent": "navbar:tree", "role": "tree", "rect": row["rect"], "z": 111})
    return nodes


@_hierarchy.capture_catalog()
def frame_batch(requests, settings, *, session=None):
    """🎞️ Render finite hierarchy requests with measured controls and actual layered transitions."""
    if not requests:
        return []
    p = _presentation()
    width, height, _, _ = p._settings(settings)
    paths = [Path(request["output"]).resolve() for request in requests]
    work = Path(os.path.commonpath([str(path.parent) for path in paths]))
    work.mkdir(parents=True, exist_ok=True)
    controls = {}
    for request in requests:
        _hierarchy.chapter(request["video_id"])
        if request.get("hierarchy") is None and request["video_id"] not in controls:
            controls[request["video_id"]] = _probes(request["video_id"], work, settings, session)
    projected, previous = [], None
    for request, output in zip(requests, paths):
        video_id = request["video_id"]
        if request.get("hierarchy") is None:
            timeline = choreography(video_id, request.get("phase", "focus"), request.get("t", 0), controls[video_id])
        else:
            state = copy.deepcopy(request["hierarchy"])
            timeline = {"phase": request.get("phase", "focus"), "t": request.get("t", 0), "seconds": None, "stage": "explicit",
                        "hierarchy": state, "point": request.get("point"), "pressed": bool(request.get("pressed")),
                        "fullscreen": state["opened_path"][-1] if len(state["opened_path"]) == 4 else None, "zoom": int(len(state["opened_path"]) == 4),
                        "events": [], "effects": [], "action": None, "content_camera": 0}
        point = request.get("point") if request.get("point") is not None else timeline["point"]
        if request.get("point") is not None:
            timeline["hierarchy"]["camera"] = _camera(point)
        timeline["point"] = point
        pressed = timeline["pressed"] if request.get("pressed") is None else bool(request["pressed"])
        timeline["pressed"] = pressed
        boundary = timeline["seconds"] is None or timeline["phase"] == "focus" and timeline["t"] == 0 or timeline["phase"] == "restore" and timeline["t"] == 1
        canonical = (boundary and len(timeline["hierarchy"]["opened_path"]) < 4 and not timeline["hierarchy"]["revealed_path"]
                     and point == _core.interaction_contract()["timeline"]["park"])
        elapsed = request.get("transition_elapsed")
        if elapsed is None:
            continuous = previous is not None and previous["phase"] == timeline["phase"] and previous["seconds"] is not None and timeline["seconds"] is not None and timeline["seconds"] >= previous["seconds"]
            elapsed = max(0, (timeline["seconds"] - previous["seconds"]) * 1000) if continuous else 1000
            if canonical:
                elapsed = 2000
        media = [{"id": video_id, "body_image": str(Path(request["snapshot"]).resolve()), "body_opacity": 1}] if request.get("snapshot") and not canonical else []
        frame = {"output": str(output), "hierarchy": timeline["hierarchy"], "media": media,
                 "cursor": {"point": point, "visible": timeline["content_camera"] == 0, "kind": "pointer" if pressed else "arrow", "pressed": pressed},
                 "menu_open": bool(request.get("menu_open")), "transition_elapsed": elapsed, "canonical_overview": canonical}
        frame["content_camera"] = {"video_id": video_id, "progress": timeline["content_camera"]}
        projected.append((frame, timeline, request))
        previous = timeline
    result = p._browser.render_batch(_packet([frame for frame, _, _ in projected], work, settings), session=session)
    output = []
    for (frame, timeline, request), browser in zip(projected, result["metadata"]):
        nodes = _nodes(browser)
        state = _core.interaction_state(nodes, timeline["point"], pressed=timeline["pressed"])
        timeline.update(hovered=state["ancestor_hovered"], cursor=browser["cursor"]["css"] if browser["cursor"] else "arrow")
        styles = {node["id"]: _core.resolve_visual(node["role"], {"hovered": node["id"] in state["ancestor_hovered"],
                  "pressed": node["id"] == state["pressed"], "selected": node["id"] == timeline["fullscreen"]}) for node in nodes}
        actual = next((window for window in browser["windows"] if window["id"] == request["video_id"]), None)
        metadata = {"path": Path(frame["output"]), "timeline": timeline, "browser": browser, "nodes": nodes, "state": state,
                    "styles": styles, "point": timeline["point"], "menu_open": frame["menu_open"],
                    "rect": actual["rect"] if actual else None, "body": actual["content_rect"] if actual else None,
                    "control": actual["control_rect"] if actual else None}
        if timeline["content_camera"] == 1 and request.get("snapshot"):
            from PIL import Image
            import shutil

            with Image.open(request["snapshot"]) as source:
                if source.size == (width, height):
                    shutil.copyfile(request["snapshot"], frame["output"])
        output.append(metadata)
    return output


def frame(video_id, phase, t, output, settings, *, point=None, pressed=None, menu_open=False, snapshot=None, hierarchy=None, transition_elapsed=None):
    """📸 Capture one normalized or explicit hierarchy state and its real DOM metadata."""
    return frame_batch([{"video_id": video_id, "phase": phase, "t": t, "output": output, "point": point, "pressed": pressed,
                         "menu_open": menu_open, "snapshot": snapshot, "hierarchy": hierarchy, "transition_elapsed": transition_elapsed}], settings)[0]
