import hashlib,json,sys
rows=json.load(open(sys.argv[1]))
print(json.dumps([hashlib.sha256(open(r["path"],"rb").read()).hexdigest()==r["fact"]["sha256"] for r in rows if r["kind"]=="file"]))
