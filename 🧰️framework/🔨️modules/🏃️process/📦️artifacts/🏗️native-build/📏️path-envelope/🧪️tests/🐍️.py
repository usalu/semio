"""📏️ Independent pathlib witness of selected Cargo output path extents."""
import json
import pathlib
import sys
policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]
fixture = json.load(sys.stdin)
answers = []
for row in fixture["cases"]:
    path = pathlib.PureWindowsPath if row["flavor"] == "win32" else pathlib.PurePosixPath
    paths = set()
    for unit in row["units"]:
        if unit["kind"] in ("build-script-run", "documentation") or unit["operation"] in ("doc", "doctest", "run-custom-build"):
            continue
        target = unit["platform"] or ""
        if target and path(target).is_absolute():
            target = path(target).stem
        profile = "debug" if unit["profileName"] in ("dev", "test") else unit["profileName"]
        name = unit["name"].replace("-", "_")
        fingerprint = "0" * policy["fingerprintCharacters"]
        out = path(row["buildDirectory"]) / target / profile / "build" / unit["packageName"] / fingerprint / "out"
        recipe = path(row["buildDirectory"]) / "t" / ("0" * policy["recipeCharacters"]) / target / profile
        for extension in policy["extensions"]:
            paths.add(str(out / ("lib" + name + "-" + fingerprint + "." + extension)))
            paths.add(str(recipe / ("lib" + name + "." + extension)))
        paths.add(str(out / (name + "-" + fingerprint + "." + "0" * policy["codegenUnitCharacters"] + "." + "0" * policy["codegenDisambiguatorCharacters"] + ".rcgu.o")))
        for extension in ("", "exe", "pdb"):
            paths.add(str(recipe / (unit["name"] + ("." + extension if extension else ""))))
    ordered = sorted(paths)
    maximum = max((len(value.encode("utf-16-le")) // 2 for value in ordered), default=0)
    admitted = all(len(value) <= policy["maximumCharacters"] and len(value.encode("utf-16-le")) // 2 <= policy["maximumCharacters"] for value in ordered)
    answers.append(dict(paths=ordered, maximum=maximum, admitted=admitted))
json.dump(answers, sys.stdout)
