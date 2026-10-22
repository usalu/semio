#!/usr/bin/env python3
"""🐍️ Independent oracle of the analysis-provider law: an analysis row holds while the source file still has the recorded sha256 (plain hashlib, no shared code with the Bun test).

usage: oracle.py FILE < ROWS.json     prints {"rows": n, "holds": bool}"""
import hashlib, json, sys

rows = json.load(sys.stdin)
digest = hashlib.sha256(open(sys.argv[1], "rb").read()).hexdigest()
print(json.dumps({"rows": len(rows), "holds": all(row["fact"]["source"]["sha256"] == digest for row in rows)}))
