import json,sys

def key(row):
    text=row["kind"]+"\u0000"+row["path"]+"\u0000"+json.dumps(row.get("fact"),ensure_ascii=False,separators=(",",":"))
    return text.encode("utf-16-be")

def sorted_rows(rows):
    return all(key(rows[i])<=key(rows[i+1]) for i in range(len(rows)-1))

bundle=json.load(open(sys.argv[1],encoding="utf8"))
facts=[f["row"]["path"] for f in bundle["sidecar"]["facts"] if f]
members=bundle["sidecar"]["members"]
print(json.dumps({
    "rowsSorted":sorted_rows(bundle["rows"]),
    "layersSorted":[sorted_rows(layer) for layer in bundle["layers"]],
    "membersInFacts":all(all(w in facts for w in m) for m in members if m),
    "membersSorted":all(m==sorted(m,key=lambda w:w.encode("utf-16-be")) for m in members if m),
}))
