"""📏️ Independent pathlib witness of selected Cargo output path extents, with the codegen temporaries in the lane temporary directory."""
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
        stem = name + "-" + fingerprint
        session = "s-" + "0" * policy["incrementalTimestampCharacters"] + "-" + "0" * policy["codegenDisambiguatorCharacters"] + "-" + "0" * policy["incrementalSessionCharacters"]
        paths.add(str(path(row["buildDirectory"]) / target / profile / "incremental" / (name + "-" + "0" * policy["incrementalCrateCharacters"]) / session / ("0" * policy["codegenUnitCharacters"] + ".o")))
        for value in (stem + "." + "0" * policy["codegenUnitCharacters"] + "." + "0" * policy["codegenDisambiguatorCharacters"] + ".rcgu.no-opt.bc", "0" * 16 + "-" + stem + "." + name + "." + "0" * policy["wholeCodegenHashCharacters"] + "-cgu." + "0" * policy["wholeCodegenIndexCharacters"] + ".rcgu.thin-lto-after-rename.bc"):
            paths.add(str(path(row["buildDirectory"]) / policy["temporaryDirectory"] / value[: policy["temporaryFileCharacters"]]))
        for extension in ("", "exe", "pdb"):
            paths.add(str(recipe / (unit["name"] + ("." + extension if extension else ""))))
    ordered = sorted(paths)
    maximum = max((len(value.encode("utf-16-le")) // 2 for value in ordered), default=0)
    admitted = all(len(value) <= policy["maximumCharacters"] and len(value.encode("utf-16-le")) // 2 <= policy["maximumCharacters"] for value in ordered)
    answers.append(dict(paths=ordered, maximum=maximum, admitted=admitted))
json.dump(answers, sys.stdout)
