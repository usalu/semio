import json,sys

UNITS=["B","KiB","MiB","GiB","TiB"]

def fmt(n):
    v=float(max(0,n));u=0
    while v>=1024 and u<len(UNITS)-1:v/=1024;u+=1
    r=int(v+0.5) if u==0 else int(v*100+0.5)/100
    return ("%d"%r if float(r).is_integer() else repr(float(r)))+" "+UNITS[u]

print(json.dumps([fmt(n) for n in json.load(sys.stdin)]))
