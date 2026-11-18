import json,sys
ops=json.load(open(sys.argv[1]))
table={}
out=[]
for op in ops:
    if op[0]=="set":
        table[op[1]]=op[2]
    else:
        out.append(table.get(op[1]))
print(json.dumps({"answers":out,"size":len(table)}))
