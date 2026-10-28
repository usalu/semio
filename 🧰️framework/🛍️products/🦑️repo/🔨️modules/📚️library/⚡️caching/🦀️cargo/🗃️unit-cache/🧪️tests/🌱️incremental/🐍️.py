"""🌱 Independent oracle for incremental unit-cache misses: from the recorded compiler command lines, the crates whose compiles kept Cargo's `-C incremental=`; from a plain Cargo target directory, the crates Cargo itself keeps an incremental session for.
usage: python3 -I 🐍️.py <record.log> <plain target dir>   prints {"recorded": [...], "plain": [...]}"""
import json, os, re, sys
recorded = set()
for line in open(sys.argv[1], encoding="utf8"):
    words = line.split()
    if "--crate-name" in words and any(w == "incremental" or w.startswith("incremental=") or w.startswith("-Cincremental=") for w in words) or ("--crate-name" in words and re.search(r"(^| )-C incremental=", line)):
        recorded.add(words[words.index("--crate-name") + 1])
plain = set()
for directory, names, _ in os.walk(sys.argv[2]):
    if os.path.basename(directory) == "incremental":
        plain.update(name.rsplit("-", 1)[0] for name in names)
print(json.dumps({"recorded": sorted(recorded), "plain": sorted(plain)}))
