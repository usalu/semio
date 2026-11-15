"""🖼️ Capture the stored chapter posters with progress and keyboard cancellation."""

from __future__ import annotations

import argparse
import importlib
import os
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

sys.dont_write_bytecode = True

_PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"


def main(argv: list[str] | None = None) -> int:
    """📸 (Re)capture posters from an up-to-date rendered clip of the output root, else by one single frame of the scene."""
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8")
    posters = importlib.import_module(_PACKAGE + ".🪟️presentation.🖼️posters.🐍️")
    profile = importlib.import_module(_PACKAGE + ".🎚️profile.🐍️")
    audience = importlib.import_module(_PACKAGE + ".👥️audience.🐍️")
    known = posters.chapters()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--language", choices=("en", "de"), required=True)
    parser.add_argument("--profile", choices=profile.NAMES, required=True)
    parser.add_argument("--audience", choices=audience.audiences(), default="speaker")
    parser.add_argument("--video", action="append", choices=sorted({info["video"]["id"] for info in known.values()}))
    parser.add_argument("--chapter", action="append", choices=list(known))
    parser.add_argument("--output-root", type=Path)
    parser.add_argument("--jobs", type=int, default=max(1, min(4, (os.cpu_count() or 2) - 1)))
    parser.add_argument("--force", action="store_true", help="Recapture posters that are not stale")
    args = parser.parse_args(argv)
    if args.output_root:
        os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(args.output_root.resolve())
    output = importlib.import_module(_PACKAGE + ".📁️output.🐍️").output_root() / args.profile / args.language / args.audience
    scenes = [scene for scene, info in known.items() if (not args.video or info["video"]["id"] in args.video) and (not args.chapter or scene in args.chapter)]
    settings = {"language": args.language, "profile": args.profile, "audience": args.audience, "resolution": None, "fps": None}
    try:
        how = posters.capture(scenes, settings, output, jobs=args.jobs, force=args.force)
    except KeyboardInterrupt:
        print("Poster capture cancelled; stored posters remain.", flush=True)
        return 130
    except (RuntimeError, ValueError) as error:
        print(str(error), file=sys.stderr, flush=True)
        return 1
    for kind in ("current", "clip", "frame"):
        print(f"[poster] {kind}: {sum(1 for value in how.values() if value == kind)}", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
