import json,sys
def same(a,b):
    if isinstance(a,bool) or isinstance(b,bool):
        return isinstance(a,bool) and isinstance(b,bool) and a==b
    if isinstance(a,(int,float)) and isinstance(b,(int,float)):
        return a==b
    if isinstance(a,list):
        return isinstance(b,list) and len(a)==len(b) and all(same(x,y) for x,y in zip(a,b))
    if isinstance(a,dict):
        return isinstance(b,dict) and set(a)==set(b) and all(same(a[k],b[k]) for k in a)
    return a==b and type(a)==type(b)
print(json.dumps([same(a,b) for a,b in json.load(sys.stdin)]))
