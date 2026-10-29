import json, sqlite3, sys
baseline = json.load(open(sys.argv[1], encoding="utf-8"))
selection = json.load(open(sys.argv[2], encoding="utf-8"))
connection = sqlite3.connect(":memory:")
connection.execute("create table edges(parent text, child text)")
connection.executemany("insert into edges values (?, ?)", baseline["expansions"])
connection.execute("create table entries(path text)")
connection.executemany("insert into entries values (?)", [(path,) for path in baseline["entries"]])
leaves = [row[0] for row in connection.execute("with recursive closure(path) as (select path from entries union select child from edges join closure on parent=path) select path from closure where path not in (select parent from edges) order by path")]
print(json.dumps({"leaves": leaves, "selected": sorted(selection["files"]), "entryCount": len(baseline["entries"]), "edgeCount": len(baseline["expansions"])}))
