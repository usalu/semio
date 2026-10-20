#!/usr/bin/env python3
"""Independent Python implementation of the Bazel test classes: reads {authored, fixture} JSON on stdin and prints the answer to every fixture section. Globs use the standard library's fnmatch."""
import fnmatch, json, re, sys

def plan(table, row):
    chosen = None
    for rule in table["rules"]:
        if any(fnmatch.fnmatchcase(row["target"], g) for g in rule["targets"]) and ("projects" not in rule or any(fnmatch.fnmatchcase(row["project"], g) for g in rule["projects"])):
            chosen = rule
            break
    size = (chosen or {}).get("size", table["default"]["size"])
    return {"size": size, "timeout": table["sizes"][size]["timeout"], "shards": (chosen or {}).get("shards", table["default"]["shards"]), "flaky": (chosen or {}).get("flaky", table["default"]["flaky"]), "witness": row["files"] > table["witness"]["from"], "rule": chosen["id"] if chosen else None}

def attributes(p):
    out = [["size", json.dumps(p["size"])], ["timeout", json.dumps(p["timeout"])]]
    if p["shards"] > 1:
        out.append(["shard_count", str(p["shards"])])
    if p["flaky"]:
        out.append(["flaky", "True"])
    return out

def environment(bazel):
    row = {k: v for k, v in bazel.items() if v != ""}
    if "XML_OUTPUT_FILE" in row and not row["XML_OUTPUT_FILE"].startswith("/"):
        return "refused"
    total = int(row.get("TEST_TOTAL_SHARDS", "1")) if re.fullmatch(r"[1-9][0-9]*", row.get("TEST_TOTAL_SHARDS", "1")) else None
    if total is None:
        return "refused"
    out = {}
    if total > 1:
        index = row.get("TEST_SHARD_INDEX")
        if index is None or not re.fullmatch(r"0|[1-9][0-9]*", index) or int(index) >= total:
            return "refused"
        out["SEMIO_TEST_SHARD"] = "%d/%d" % (int(index) + 1, total)
        if "TEST_SHARD_STATUS_FILE" in row:
            if not row["TEST_SHARD_STATUS_FILE"].startswith("/"):
                return "refused"
            out["SEMIO_TEST_SHARD_STATUS_FILE"] = row["TEST_SHARD_STATUS_FILE"]
    if "XML_OUTPUT_FILE" in row:
        out["SEMIO_TEST_XML_OUTPUT_FILE"] = row["XML_OUTPUT_FILE"]
    return out

def main():
    data = json.load(sys.stdin)
    authored, fixture = data["authored"], data["fixture"]
    tables = dict(fixture["tables"], authored=authored)
    json.dump({
        "plans": [plan(tables[row["table"]], row["input"]) for row in fixture["plans"]],
        "attributes": [attributes(row["plan"]) for row in fixture["attributes"]],
        "environments": [environment(row["bazel"]) for row in fixture["environments"]],
        "refusedEnvironments": [environment(row) for row in fixture["refusedEnvironments"]],
    }, sys.stdout, ensure_ascii=False)

if __name__ == "__main__":
    main()
