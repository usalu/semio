"""🖱️ Replay canonical pointer interactions independently of native rendering tools."""

from __future__ import annotations

import copy
import importlib
import json
import math
import re
from pathlib import Path
from xml.etree import ElementTree

_ROOT = Path(__file__).resolve().parent
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")


def interaction_contract() -> dict:
    """📜 Read the portable canonical interaction contract without selecting a lesson."""
    return json.loads((_ROOT / "🔣️.json").read_text(encoding="utf-8"))


def source_files() -> tuple[Path, ...]:
    """🧬 Bind pointer geometry and visual states to their real UI source assets."""
    contract = interaction_contract()
    assets = [entry["asset"] for entry in contract["cursors"].values()]
    assets.extend(contract["icons"].values())
    return (Path(__file__), _ROOT / "🔣️.json", _ROOT / "🧬️schema.json", _ROOT.parent / "🔣️.json",
            Path(_theme.__file__), _theme.TOKEN_PATH, *(_theme.ROOT / asset for asset in assets))


def _index(nodes: list[dict]) -> dict[str, dict]:
    """🌳 Validate stable node identities and complete acyclic parent chains."""
    index = {node["id"]: node for node in nodes}
    if len(index) != len(nodes):
        raise ValueError("Interaction node identities must be unique")
    for node in nodes:
        seen, current = set(), node
        while current is not None:
            identity = current["id"]
            if identity in seen:
                raise ValueError("Interaction parent chains must be acyclic")
            seen.add(identity)
            parent = current.get("parent")
            if parent is not None and parent not in index:
                raise ValueError(f"Unknown interaction parent: {parent}")
            current = index.get(parent)
    return index


def _path(node: dict, index: dict[str, dict]) -> list[dict]:
    """🪜 Resolve one actual ancestry chain without including unrelated siblings."""
    result = []
    while node is not None:
        result.insert(0, node)
        node = index.get(node.get("parent"))
    return result


def _contains(node: dict, point) -> bool:
    """🎯 Test half-open bounds and optional absolute silhouette polygons."""
    if point is None or len(point) != 2 or not all(math.isfinite(value) for value in point):
        return False
    x, y = point
    rect = node["rect"]
    if not (rect["x"] <= x < rect["x"] + rect["width"] and rect["y"] <= y < rect["y"] + rect["height"]):
        return False
    outline = node.get("outline")
    if not outline:
        return True
    inside = False
    previous = outline[-1]
    for current in outline:
        x1, y1 = previous
        x2, y2 = current
        if (y1 > y) != (y2 > y) and x < (x2 - x1) * (y - y1) / (y2 - y1) + x1:
            inside = not inside
        previous = current
    return inside


def hit_test(nodes: list[dict], point) -> dict | None:
    """🧭 Pick the topmost deepest enabled target clipped by every real ancestor."""
    index = _index(nodes)
    candidates = []
    for order, node in enumerate(nodes):
        path = _path(node, index)
        if all(entry.get("enabled", True) and _contains(entry, point) for entry in path):
            candidates.append(((node.get("z", 0), len(path), order), node))
    return copy.deepcopy(max(candidates, key=lambda item: item[0])[1]) if candidates else None


def hit_path(point, nodes: list[dict]) -> list[str]:
    """🔗 Return hovered ancestors in root-to-target order."""
    target = hit_test(nodes, point)
    return [entry["id"] for entry in _path(target, _index(nodes))] if target else []


def interaction_state(nodes: list[dict], point, pressed: bool = False) -> dict:
    """🪟 Describe transient hover and press emphasis for an explicit pointer location."""
    path = hit_path(point, nodes)
    target = path[-1] if path else None
    return {"target": target, "ancestor_hovered": path, "pressed": target if pressed else None}


def initial_state() -> dict:
    """🌱 Start an immutable event projection with no assumed pointer location."""
    return {"point": None, "hovered": [], "pressed": None, "armed": None,
            "fullscreen": None, "toggles": {}, "events": [], "effects": []}


def apply_event(state: dict, event: dict, nodes: list[dict]) -> dict:
    """⚡ Commit actions only after press, enabled same-target release and an explicit click."""
    contract = interaction_contract()
    kind = event["type"]
    if kind not in contract["events"]:
        raise ValueError(f"Unknown pointer event: {kind}")
    result = copy.deepcopy(state)
    point = copy.deepcopy(event.get("point", result["point"]))
    target = hit_test(nodes, point)
    identity = target["id"] if target else None
    result.update(point=point, hovered=hit_path(point, nodes), effects=[])
    if kind in ("leave", "cancel"):
        result.update(hovered=[], pressed=None, armed=None)
    elif kind == "press":
        result.update(pressed=identity, armed=None)
    elif kind == "release":
        result["armed"] = identity if identity is not None and identity == result["pressed"] else None
        result["pressed"] = None
    elif kind == "click":
        if identity is not None and identity == result["armed"]:
            role = target["role"]
            if role == "fullscreen":
                window = next((entry["id"] for entry in reversed(_path(target, _index(nodes))) if entry["role"] == "window"), None)
                if window is None:
                    raise ValueError("A fullscreen control requires a parent window")
                value = result["fullscreen"] != window
                result["fullscreen"] = window if value else None
                result["effects"] = [{"type": contract["actions"][role], "target": window, "value": value}]
            elif role == "toggle":
                value = not result["toggles"].get(identity, False)
                result["toggles"][identity] = value
                result["effects"] = [{"type": contract["actions"][role], "target": identity, "value": value}]
            elif role in ("device", "button"):
                result["effects"] = [{"type": target.get("action", contract["actions"][role]), "target": identity, "value": True}]
        result["armed"] = None
    result["events"].append({**copy.deepcopy(event), "point": point, "target": identity})
    return result


def _paint(value: dict) -> str:
    """🎨 Resolve canonical token colors with the UI's first-token sRGB mixing rule."""
    if "token" in value:
        return _theme.color(value["token"])
    first, second, ratio = value["mix"]
    a, b = _theme.color(first).lstrip("#"), _theme.color(second).lstrip("#")
    channels = [math.floor(int(a[i:i + 2], 16) * ratio + int(b[i:i + 2], 16) * (1 - ratio) + 0.5) for i in (0, 2, 4)]
    return "#" + "".join(f"{channel:02x}" for channel in channels)


def resolve_visual(role: str, state: dict) -> dict:
    """🖌️ Resolve actual dark chrome states while retaining parent glass on hover."""
    contract = interaction_contract()
    if role not in contract["roles"]:
        raise ValueError(f"Unknown interaction role: {role}")
    entry = contract["roles"][role]
    chrome = _theme.TOKENS["appearances"]["dark"]["chrome"]
    hovered = bool(state.get("hovered")) and not state.get("disabled", False)
    active = bool(state.get("pressed") or state.get("selected"))
    interactive = entry["profile"] == "interactive"
    fill = {"base": _paint(chrome["base"]), "glass": _theme.color("dark-8-9"), "transparent": _paint(chrome["base"])}[entry["fill"]]
    result = {"fill": fill, "fill_opacity": 0 if entry["fill"] == "transparent" else 1,
              "stroke": _paint(chrome["borderNormal"]),
              "text": _paint(chrome["borderElement"] if interactive or role == "tree" else chrome["foreground"]),
              "stroke_width": entry["strokeWidth"], "opacity": 0.5 if state.get("disabled", False) else 1,
              "cursor": "arrow" if state.get("disabled", False) else entry["cursor"]}
    if hovered or role == "tree" and active:
        result.update(stroke=_paint(chrome["borderEmphasized"]), text=_paint(chrome["borderEmphasized"]),
                      stroke_width=entry.get("hoverStrokeWidth", entry["strokeWidth"]))
        if interactive:
            result.update(fill=_paint(chrome["hoverInteractiveFill"]), fill_opacity=1)
    if active and role != "tree":
        result["stroke"] = _paint(chrome["activeBase"])
        if interactive:
            result.update(fill=_paint(chrome["activeHover"] if hovered else chrome["activeBase"]),
                          fill_opacity=1, text=_paint(chrome["activeForeground"]))
    return result


def transition_color(before_hex: str, after_hex: str, elapsed_seconds: float) -> str:
    """🕰️ Interpolate canonical border ink with the CSS ease timing curve."""
    if any(not isinstance(color, str) or re.fullmatch(r"#[0-9a-fA-F]{6}", color) is None
           for color in (before_hex, after_hex)):
        raise ValueError("Transition colors must be six-digit hexadecimal colors")
    if isinstance(elapsed_seconds, bool) or not isinstance(elapsed_seconds, (int, float)) or not math.isfinite(elapsed_seconds):
        raise ValueError("Transition elapsed time must be finite")
    transition = interaction_contract()["states"]["transition"]
    position = max(0.0, min(1.0, elapsed_seconds / (transition["milliseconds"] / 1000)))
    if position in (0.0, 1.0):
        return (before_hex if position == 0 else after_hex).lower()
    x1, y1, x2, y2 = transition["controlPoints"]
    lower, upper = 0.0, 1.0
    for _ in range(64):
        parameter = (lower + upper) / 2
        inverse = 1 - parameter
        x = 3 * inverse * inverse * parameter * x1 + 3 * inverse * parameter * parameter * x2 + parameter ** 3
        if x < position:
            lower = parameter
        else:
            upper = parameter
    parameter = (lower + upper) / 2
    inverse = 1 - parameter
    progress = 3 * inverse * inverse * parameter * y1 + 3 * inverse * parameter * parameter * y2 + parameter ** 3
    channels = [math.floor(int(before_hex[index:index + 2], 16) * (1 - progress) +
                           int(after_hex[index:index + 2], 16) * progress + 0.5) for index in (1, 3, 5)]
    return "#" + "".join(f"{channel:02x}" for channel in channels)


def control_rect(rect: dict, role: str = "fullscreen") -> dict:
    """📐 Place the actual full-width WindowChrome right-cap cell."""
    contract = interaction_contract()
    if role not in ("fullscreen", "toggle", "button"):
        raise ValueError(f"Unknown chrome control role: {role}")
    cell = contract["control"]
    size = cell["size"]
    title_height = min(rect["height"], cell["titleHeight"])
    cap_width = rect.get("title_width", rect["width"])
    if cap_width < size or title_height < size:
        raise ValueError("The title cap is too small for its canonical control")
    return {"x": rect["x"] + cap_width - size,
            "y": rect["y"] + (title_height - size) / 2, "width": size, "height": size}


def _polygon(data: str, offset: list[float]) -> list[list[float]]:
    """📍 Read canonical straight SVG cursor paths into translated native vertices."""
    tokens = re.findall(r"[MmLlHhVvZz]|[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?", data)
    position, start, points, command, index = [0.0, 0.0], None, [], None, 0
    while index < len(tokens):
        if tokens[index].isalpha():
            command = tokens[index]
            index += 1
        if command in ("Z", "z"):
            if start is not None:
                points.append(start[:])
                position = [start[0] - offset[0], start[1] - offset[1]]
            command = None
            continue
        if command is None:
            raise ValueError("Cursor path has no drawing command")
        relative = command.islower()
        kind = command.upper()
        length = 1 if kind in ("H", "V") else 2
        values = [float(token) for token in tokens[index:index + length]]
        index += length
        if kind in ("M", "L"):
            position = [values[i] + (position[i] if relative else 0) for i in (0, 1)]
        elif kind == "H":
            position[0] = values[0] + (position[0] if relative else 0)
        elif kind == "V":
            position[1] = values[0] + (position[1] if relative else 0)
        else:
            raise ValueError(f"Unsupported canonical cursor path command: {command}")
        vertex = [position[i] + offset[i] for i in (0, 1)]
        points.append(vertex)
        if kind == "M":
            start = vertex[:]
            command = "l" if relative else "L"
    return points


def cursor_outline(kind: str = "arrow") -> dict:
    """👆 Read the exact dark-appearance arrow, pointer or selectable native geometry."""
    contract = interaction_contract()
    if kind not in contract["cursors"]:
        raise ValueError(f"Unknown cursor kind: {kind}")
    entry = contract["cursors"][kind]
    document = ElementTree.parse(_theme.ROOT / entry["asset"]).getroot()
    paths = []
    for group in document.findall(".//{*}g"):
        transform = re.search(r"translate\(([-\d.]+)[,\s]+([-\d.]+)\)", group.get("transform", ""))
        offset = [float(transform.group(i)) for i in (1, 2)] if transform else [0, 0]
        for path in group.findall("{*}path"):
            style = dict(part.split(":", 1) for part in path.get("style", "").split(";") if ":" in part)
            paths.append({"points": _polygon(path.attrib["d"], offset), "fill": style["fill"],
                          "stroke": style["stroke"], "stroke_width": float(style["stroke-width"]),
                          "miter_limit": float(style["stroke-miterlimit"])})
    return {"viewBox": [float(value) for value in document.attrib["viewBox"].split()],
            "hotspot": entry["hotspot"], "paths": paths}


