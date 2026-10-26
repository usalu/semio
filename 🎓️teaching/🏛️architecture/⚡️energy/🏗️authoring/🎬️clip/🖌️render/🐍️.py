"""▶️ Render complete teaching videos with progress and keyboard cancellation."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import argparse
import os

sys.dont_write_bytecode = True

_video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
compose_videos = _video_module.compose_videos
load_manifest = _video_module.load_manifest


def main(argv: list[str] | None = None) -> int:
    """🎬 Render the selected lessons in curriculum order beneath one output root."""
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8")
    videos = load_manifest()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--video", action="append", choices=[video["id"] for video in videos])
    parser.add_argument("--output-root", type=Path)
    parser.add_argument("--jobs", type=int, default=max(1, min(4, (os.cpu_count() or 2) - 1)), help="Global concurrent segment limit")
    parser.add_argument("--log-output", action="store_true", help="Save immutable per-segment attempt logs")
    parser.add_argument("-q", "--quality", choices=("l", "m", "h"), default="l")
    parser.add_argument("--resolution", help="Optional width,height override for complete preview renders")
    parser.add_argument("--fps", type=float, help="Optional frame rate override")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--concat-only", action="store_true")
    parser.add_argument("--no-play", action="store_true")
    parser.add_argument("--play", action="store_true")
    args = parser.parse_args(argv)
    if args.force and args.concat_only:
        parser.error("--force and --concat-only cannot be combined")
    if args.jobs < 1:
        parser.error("--jobs must be positive")
    if args.fps is not None and args.fps <= 0:
        parser.error("--fps must be positive")
    if args.resolution:
        dimensions = args.resolution.split(",")
        if len(dimensions) != 2 or not all(value.isdigit() and int(value) > 0 and int(value) % 2 == 0 for value in dimensions):
            parser.error("--resolution requires two positive even dimensions: width,height")
    if args.output_root:
        os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(args.output_root.resolve())
    try:
        compose_videos(args.video, output_root=args.output_root, quality_flag="-q" + args.quality,
                       play=args.play and not args.no_play, force=args.force, concat_only=args.concat_only,
                       resolution=args.resolution, fps=args.fps, jobs=args.jobs, log_output=args.log_output)
    except KeyboardInterrupt:
        print("Render cancelled; completed segments and chapters can be reused.", flush=True)
        return 130
    except RuntimeError as error:
        print(str(error), file=sys.stderr, flush=True)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
