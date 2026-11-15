"""📐️ Independent pathlib witness of the worst Cargo output path per canonical root, target and profile."""
import json
import pathlib
import sys

request = json.load(sys.stdin)
budget, policy, packages = request["budget"], request["policy"], request["packages"]
zero = lambda count: "0" * count
rows = []
for root in budget["roots"]:
    flavor = pathlib.PureWindowsPath if root["flavor"] == "win32" else pathlib.PurePosixPath
    base = flavor(root["template"].replace("{account}", "a" * budget["accountCharacters"]), *budget["cacheDirectory"].split("/"), budget["laneDirectory"], "q" + zero(budget["laneCharacters"] - 1))
    for target in budget["targets"]:
        for profile in budget["profiles"]:
            worst = (0, "")
            for package in packages:
                if target["zone"] and target["zone"] not in package["zones"]:
                    continue
                units = ([package["lib"]] if package["lib"] is not None else []) + ([] if target["zone"] else package["others"])
                for name in units:
                    ident = name.replace("-", "_")
                    fingerprint = zero(policy["fingerprintCharacters"])
                    out = base / target["directory"] / profile / "build" / package["name"] / fingerprint / "out"
                    recipe = base / "t" / zero(policy["recipeCharacters"]) / target["directory"] / profile
                    stem = ident + "-" + fingerprint
                    paths = [out / ("lib" + stem + "." + extension) for extension in policy["extensions"]]
                    paths += [recipe / ("lib" + ident + "." + extension) for extension in policy["extensions"]]
                    paths += [recipe / (name + suffix) for suffix in ("", ".exe", ".pdb")]
                    paths.append(base / target["directory"] / profile / "incremental" / (ident + "-" + zero(policy["incrementalCrateCharacters"])) / ("s-" + zero(policy["incrementalTimestampCharacters"]) + "-" + zero(policy["codegenDisambiguatorCharacters"]) + "-" + zero(policy["incrementalSessionCharacters"])) / (zero(policy["codegenUnitCharacters"]) + ".o"))
                    temporary = [stem + "." + zero(policy["codegenUnitCharacters"]) + "." + zero(policy["codegenDisambiguatorCharacters"]) + ".rcgu.no-opt.bc", zero(16) + "-" + stem + "." + ident + "." + zero(policy["wholeCodegenHashCharacters"]) + "-cgu." + zero(policy["wholeCodegenIndexCharacters"]) + ".rcgu.thin-lto-after-rename.bc"]
                    paths += [base / policy["temporaryDirectory"] / value[: policy["temporaryFileCharacters"]] for value in temporary]
                    length = max(len(str(path).encode("utf-16-le")) // 2 for path in paths)
                    if length > worst[0]:
                        worst = (length, package["name"])
            rows.append({"root": root["id"], "target": target["id"], "profile": profile, "length": worst[0]})
json.dump(rows, sys.stdout)
