"""Independent oracle of the targets of a Rust owner: the targets its project.json declares plus the three Cargo targets (build, check, test) that every owner with a Cargo manifest that has a package gets. Reads the owner root (argv[1], the workspace root argv[2]) and prints the sorted names."""
import json
import sys
from pathlib import Path

root = Path(sys.argv[2]) / sys.argv[1]
declared = set(json.loads((root / "📋️project.json").read_text(encoding="utf8")).get("targets", {}).keys())
manifest = (root / "Cargo.toml").read_text(encoding="utf8") if (root / "Cargo.toml").exists() else ""
derived = {"build", "check", "test"} if "[package]" in manifest else set()
print(json.dumps(sorted(declared | derived), ensure_ascii=False))
