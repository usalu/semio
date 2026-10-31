"""🐍️ JSON import attributes, Python side: walks the same scopes with the pattern of `🔣️.json`, resolves relative specifiers on disk and bare specifiers with Node's own resolver, and requires every `with {type:"json"}` target to be a .json file."""

import json
import os
import re
import subprocess
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(HERE, "🔣️.json"), encoding="utf-8") as handle:
    CORPUS = json.load(handle)
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * len(CORPUS["rootDepth"])))
PATTERN = re.compile(CORPUS["pattern"])
SKIP = set(CORPUS["skipDirectories"])
FIXTURES = {(row["file"], row["specifier"]) for row in CORPUS["fixtureStrings"]}
SOURCE = re.compile(r"\.(?:ts|tsx|mts|cts)$")
NODE = 'const {createRequire}=require("node:module");try{process.stdout.write(createRequire(process.argv[1]+"/x.js").resolve(process.argv[2]))}catch{process.exit(3)}'


def resolve(directory, specifier):
    if specifier.startswith("."):
        path = os.path.normpath(os.path.join(directory, specifier))
        return path if os.path.isfile(path) else None
    done = subprocess.run(["node", "-e", NODE, directory, specifier], capture_output=True, text=True)
    return done.stdout if done.returncode == 0 else None


class JsonImportAttributes(unittest.TestCase):
    def test_every_json_attribute_targets_a_json_file(self):
        mismatches, cache, imports = [], {}, 0
        for scope in CORPUS["scopes"]:
            for directory, names, files in os.walk(os.path.join(ROOT, scope)):
                names[:] = [name for name in names if name not in SKIP]
                for name in files:
                    if not SOURCE.search(name):
                        continue
                    path = os.path.join(directory, name)
                    with open(path, encoding="utf-8", errors="replace") as handle:
                        text = handle.read()
                    if "json" not in text:
                        continue
                    for match in PATTERN.finditer(text):
                        imports += 1
                        specifier = match.group(1) or match.group(2)
                        key = (directory, specifier)
                        if key not in cache:
                            cache[key] = resolve(directory, specifier)
                        resolved = cache[key]
                        relative = os.path.relpath(path, ROOT)
                        if resolved is not None and not resolved.endswith(".json") and (relative, specifier) not in FIXTURES:
                            mismatches.append((relative, specifier, resolved))
        self.assertGreater(imports, CORPUS["minimumImports"])
        self.assertEqual(mismatches, [])


if __name__ == "__main__":
    unittest.main()
