import json
import re
import sys

with open(sys.argv[1], encoding="utf-8") as fixture:
    for case in json.load(fixture)["cases"]:
        rows = [{"id": row["id"], "phase": "verify", "status": row["status"], "pin": row["pin"],
                 "found": row["found"], "location": row["location"],
                 "primary": re.split(r"[\r\n]", row["message"], maxsplit=1)[0] if "message" in row else None}
                for row in case["reports"] if row["status"] in {"missing", "differs", "failed"}]
        assert rows == case["expected"], case["name"]
        print(case["name"], len(rows))
