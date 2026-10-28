"""🗄️ Independent reader of a lease database: prints the SQLite application id and the table list through Python's own sqlite3 (a
second SQLite binding), which also proves nothing keeps the file locked. usage: python3 -I 🐍️.py <database>"""
import json, sqlite3, sys

connection = sqlite3.connect("file:" + sys.argv[1] + "?mode=ro", uri=True, timeout=0)
print(json.dumps({"applicationId": connection.execute("PRAGMA application_id").fetchone()[0],
                  "tables": [row[0] for row in connection.execute("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name")]}))
connection.close()
