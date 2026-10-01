"""Independent list oracle for a paged history ledger.

Reads the language-agnostic fixture, pushes that many ids, deletes the named logical index, and appends the replacement. The Rust ledger's live order must equal this JSON array.
"""

import json
import pathlib

fixture = pathlib.Path(__file__).resolve().parents[2] / "🧬️schema" / "📸️paged-history-ledger" / "🔣️.json"
law = json.loads(fixture.read_text())
items = [str(index) for index in range(law["pushes"])]
del items[law["removeLogical"]]
items.append(law["replacement"])
print(json.dumps(items))
