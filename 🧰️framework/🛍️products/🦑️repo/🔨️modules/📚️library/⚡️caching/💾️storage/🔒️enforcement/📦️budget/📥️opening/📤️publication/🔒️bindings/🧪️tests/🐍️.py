import json
import sqlite3
from pathlib import Path

fixture = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
for case in fixture["cases"]:
    with sqlite3.connect(":memory:") as database:
        database.execute("create table lease(resource text, mode text, owner text)")
        database.execute("insert into lease values ('image', 'exclusive', 'maintenance')")
        if case["peerBinding"]:
            database.execute("insert into lease values ('binding', ?, 'peer')", (case["peerBinding"],))
        query = "select exists(select 1 from lease where resource = ? and (? = 'exclusive' or mode = 'exclusive'))"
        outcomes = {}
        for name, resource, mode in [
            ("maintenanceBinding", "binding", "exclusive"),
            ("readerDuringMaintenance", "image", "shared"),
        ]:
            outcomes[name] = "busy" if database.execute(query, (resource, mode)).fetchone()[0] else "acquired"
        database.execute("delete from lease where owner = 'maintenance'")
        outcomes["readerAfterMaintenance"] = "busy" if database.execute(query, ("image", "shared")).fetchone()[0] else "acquired"
        assert outcomes == {name: case[name] for name in outcomes}, (outcomes, case)
print(f"{len(fixture['cases'])} SQLite lease scenarios passed")
