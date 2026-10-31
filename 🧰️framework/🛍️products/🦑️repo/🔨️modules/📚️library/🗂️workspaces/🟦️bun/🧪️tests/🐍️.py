"""📦️ Workspace output exclusion, Python side: the fixture `gitignoredOutputPackages` of `🧫️fixtures/🔣️.json` that `🟦️.ts` checks against fast-glob. Here the independent oracles are stdlib `glob` and `git ls-files --others --exclude-standard` over a scratch repository holding the repository's own ignore rules for the generated names. Run with `python3 <this file>`."""

import glob
import json
import os
import shutil
import subprocess
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
REPOSITORY = os.path.abspath(os.path.join(HERE, *[".."] * 8))
with open(os.path.join(HERE, "🧫️fixtures", "🔣️.json"), encoding="utf-8") as handle:
    ROW = json.load(handle)["gitignoredOutputPackages"]
with open(os.path.join(HERE, "..", "🧬️schema", "🔣️.json"), encoding="utf-8") as handle:
    NAMES = json.load(handle)["$defs"]["OutputDirectories"]["const"]


def tree(root):
    for path in ROW["authored"] + ROW["outputs"]:
        os.makedirs(os.path.join(root, path))
        with open(os.path.join(root, path, "package.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")


def members(root):
    found = glob.glob(os.path.join(root, ROW["pattern"], "package.json"), recursive=True)
    return sorted(os.path.relpath(os.path.dirname(path), root) for path in found)


class OutputExclusion(unittest.TestCase):
    def test_the_schema_names_every_ignored_output(self):
        for name in ROW["ignoredNames"]:
            self.assertIn(name, NAMES)

    def test_members_are_the_package_dirs_outside_every_output_directory(self):
        with tempfile.TemporaryDirectory() as root:
            tree(root)
            kept = [path for path in members(root) if not set(path.split(os.sep)) & set(NAMES)]
            self.assertEqual(kept, sorted(ROW["authored"]))

    @unittest.skipUnless(shutil.which("git"), "git is absent")
    def test_git_ignores_the_generated_names_and_keeps_the_authored_packages(self):
        with open(os.path.join(REPOSITORY, ".gitignore"), encoding="utf-8") as handle:
            rules = [line for line in handle.read().splitlines() if any(line.startswith(f"**/{name}/") for name in ROW["ignoredNames"])]
        self.assertEqual(len(rules), len(ROW["ignoredNames"]))
        with tempfile.TemporaryDirectory() as root:
            tree(root)
            with open(os.path.join(root, ".gitignore"), "w", encoding="utf-8") as handle:
                handle.write("\n".join(rules) + "\n")
            subprocess.run(["git", "init", "-q"], cwd=root, check=True)
            listed = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "--others", "--exclude-standard"], cwd=root, check=True, capture_output=True, text=True).stdout.split("\n")
            dirs = sorted(os.path.dirname(path) for path in listed if path.endswith("package.json"))
            self.assertEqual(dirs, sorted(ROW["authored"]))


if __name__ == "__main__":
    unittest.main()
