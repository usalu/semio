"""🐍️ Independent oracle of the caching budget classes: reads the given test files below a module root and prints the sorted list of files that mention a process-spawning token of the fixture."""
import json, os, sys

root, files_path, fixture_path = sys.argv[1:4]
tokens = json.load(open(fixture_path, encoding="utf8"))["spawnTokens"]
files = json.load(open(files_path, encoding="utf8"))
spawning = []
for file in files:
    with open(os.path.normpath(os.path.join(root, file)), encoding="utf8") as handle:
        text = handle.read()
    if any(token in text for token in tokens):
        spawning.append(file)
print(json.dumps(sorted(spawning), ensure_ascii=False))
