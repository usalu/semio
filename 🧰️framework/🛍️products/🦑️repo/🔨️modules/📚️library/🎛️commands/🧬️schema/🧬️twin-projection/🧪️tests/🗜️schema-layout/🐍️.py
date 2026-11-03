"""🗜️ Third-party mirror of `🟦️.ts`: the indented layout is `json.dumps(indent=2)` of the independent Python library, the compact layout is rebuilt from `json.dumps`, and both must equal the expected text of every vector. `python -I 🐍️.py`."""
import json, pathlib, sys

def indented(document):
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"

def compact(document):
    inline = lambda value: json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    rows = []
    for key, value in document.items():
        if key in ("$defs", "definitions") and isinstance(value, dict):
            members = ",\n".join(f"    {json.dumps(name, ensure_ascii=False)}:{inline(node)}" for name, node in value.items())
            rows.append(f"  {json.dumps(key)}:{{\n{members}\n  }}")
        else:
            rows.append(f"  {json.dumps(key)}:{inline(value)}")
    return "{\n" + ",\n".join(rows) + "\n}\n"

def main():
    vectors = json.loads((pathlib.Path(__file__).parent / "🔣️.json").read_text(encoding="utf-8"))
    failures = []
    for row in vectors["cases"]:
        render = compact if row["layout"] == "compact" else indented
        if render(row["document"]) != row["expected"]:
            failures.append(row["id"])
        if json.loads(row["expected"]) != row["document"]:
            failures.append(row["id"] + " (value)")
    print(f"schema-layout: {len(vectors['cases'])} vectors, {len(failures)} failures {failures}")
    return 1 if failures else 0

sys.exit(main())
