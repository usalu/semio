"""Independent reading of where the provisioned Bazel lives: registry partition, release pin of the host, first 16 digits of its digest."""
import json
import os
import platform
import sys

root = sys.argv[1]
registry_path = os.path.join(root, "🧰️framework", "🔨️modules", "🏃️process", "🧰️tools", "🔣️.json")
registry = json.load(open(registry_path, encoding="utf8"))
bazel = [tool for tool in registry["tools"] if tool["id"] == "bazel"][0]
release = json.load(open(os.path.join(root, bazel["pin"]["digests"]["file"]), encoding="utf8"))["definitions"]["Release"]["const"]["distributions"]
host = {"darwin": "darwin", "linux": "linux", "windows": "win32"}[platform.system().lower()]
arch = {"arm64": "arm64", "aarch64": "arm64", "x86_64": "x64", "amd64": "x64"}[platform.machine().lower()]
row = [candidate for candidate in release if candidate["platform"] == host and candidate["arch"] == arch][0]
print(os.path.join(root, registry["partitions"]["tools"], "bazel", "bin", row["sha256"][:16], "bazel" + (".exe" if host == "win32" else "")))
