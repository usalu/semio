#!/usr/bin/env python3
"""Independent check of the evidence comparison: the first differing byte, the number of differing bytes and the lines only one side has (JSON is indented first), computed with difflib's Counter-free multiset rule; prints ok or the first mismatch. usage: oracle.py <fixture.json>"""
import json,sys
from collections import Counter
data=json.load(open(sys.argv[1],encoding="utf8"))
def text(raw):
    try:return json.dumps(json.loads(raw),indent=1,ensure_ascii=False)
    except ValueError:return raw
def bytes_diff(a,b):
    a=a.encode();b=b.encode();n=min(len(a),len(b));first=None;diff=abs(len(a)-len(b))
    for i in range(n):
        if a[i]!=b[i]:
            first=i if first is None else first;diff+=1
    if first is None and len(a)!=len(b):first=n
    return first,diff
def lines(a,b):
    a=text(a).split("\n");b=text(b).split("\n");ca=Counter(a);cb=Counter(b);rows=[]
    for mark,mine,other in(("-",a,cb),("+",b,ca)):
        used=Counter()
        for line in mine:
            used[line]+=1
            if used[line]>other.get(line,0):rows.append(mark+line[:200])
    return rows[:20]
for case in data["cases"]:
    first,diff=bytes_diff(case["kept"],case["refused"])
    if first!=case["firstDifferenceByte"] or diff!=case["differingBytes"] or lines(case["kept"],case["refused"])!=case["lines"]:
        print("mismatch:",case["name"],first,diff,lines(case["kept"],case["refused"]));sys.exit(1)
print("ok",len(data["cases"]))
