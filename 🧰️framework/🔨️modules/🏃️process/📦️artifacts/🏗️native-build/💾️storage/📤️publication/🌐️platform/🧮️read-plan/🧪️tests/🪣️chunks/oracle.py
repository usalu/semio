import json,sys
doc=json.load(open(sys.argv[1]))
out=[]
for case in doc["plans"]:
    want=max(1,min(case["totalBytes"]//4,case["freeMemoryBytes"]//128))
    e=18
    while e<23 and (2**(e+1))*3//4<=want:e+=1
    out.append({"encoded":2**e,"decoded":2**e*3//4})
print(json.dumps(out))
