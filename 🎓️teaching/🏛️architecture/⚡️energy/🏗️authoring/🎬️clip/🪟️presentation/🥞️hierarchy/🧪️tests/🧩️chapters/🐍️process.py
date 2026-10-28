"""🪟 Capture public boundary assets through one independently launched process."""
import hashlib
import importlib
import json
import os
from pathlib import Path
import sys

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"


def main():
    """📸 Own producer or consumer output while sharing the actual versioned boundary asset port."""
    label, directory = sys.argv[1], Path(sys.argv[2]).resolve()
    if label not in ("producer", "consumer"):
        raise ValueError("A producer or consumer capture is required")
    work = directory / label
    work.mkdir(parents=True, exist_ok=True)
    hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
    renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
    browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
    cases = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
    frames = []
    for index, level in enumerate(("courses", "topics", "clips", "chapters")):
        case = next(case for case in cases["boundaries"] if case["level"] == level)
        branch = hierarchy.path(case["right"])
        controls = {operation: {identity: [100, 200] for identity in branch}
                    for operation in ("open", "close", "preview")}
        state = renderer.choreography(case["right"], "focus", 0, controls)
        if state["hierarchy"]["opened_path"] != case["path"]:
            raise AssertionError("The actual endpoint does not match its authored ancestor prefix")
        frames.append({"output": str(work / (str(index) + ".png")), "media": [],
            "hierarchy": state["hierarchy"], "canonical_overview": True,
            "cursor": {"point": state["point"], "kind": "arrow", "pressed": False},
            "transition_elapsed": 2000})
    packet = {"schema": "semio-presentation-browser/v1", "viewport": {"width": 480, "height": 270},
        "catalog": hierarchy.hierarchy_contract(), "language": "de", "work_directory": str(work), "frames": frames}
    (work / "packet.json").write_text(json.dumps(packet, ensure_ascii=False, indent=2), encoding="utf-8")
    result = browser.render_batch(packet)
    for frame, metadata in zip(frames, result["metadata"], strict=True):
        if metadata["output"] != frame["output"]:
            raise AssertionError("An asset consumer retained the producer's output filename")
    summary = {"pid": os.getpid(), "phase": label, "frames": [
        {"path": frame["output"], "sha256": hashlib.sha256(Path(frame["output"]).read_bytes()).hexdigest(),
         "handoff": metadata["handoff"]} for frame, metadata in zip(frames, result["metadata"], strict=True)]}
    (work / "dom.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    (work / "result.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")
    print("[DEBUG] Public boundary process", label, os.getpid(), work, flush=True)


if __name__ == "__main__":
    main()
