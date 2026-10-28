"""🎬 Compose the curriculum chapter with canonical source excerpts."""

from importlib import import_module
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")


def main(argv: list[str] | None = None) -> int:
    """▶️ Render the declared chapter through the shared range-aware execution plan."""
    return _runtime.video_main("energy_balance", ["--chapter", "EnergyBalance_04_NutzZuEndenergie", *(sys.argv[1:] if argv is None else argv)])


if __name__ == "__main__":
    raise SystemExit(main())
