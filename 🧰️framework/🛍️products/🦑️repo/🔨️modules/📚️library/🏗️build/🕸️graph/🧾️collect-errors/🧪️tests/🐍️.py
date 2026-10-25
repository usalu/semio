"""Independent model (stdlib only) of the cause key and the grouping of the collect-errors report; prints the expectations of the fixture as JSON."""
import json,re,sys
def cause(message):
    s=str(message)
    s=re.sub(r"@[a-z0-9._-]+/[a-z0-9._-]+","<project>",s,flags=re.I)
    s=re.sub(r"[A-Za-z]:\\[^\s'\"`)]+|/[^\s'\"`)]+","<path>",s)
    s=re.sub(r"\b[a-f0-9]{12,}\b","<hash>",s)
    s=re.sub(r"[0-9]+","N",s)
    s=re.sub(r"[ \t\r\n\f\v\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000\ufeff]+"," ",s)
    return s.strip()[:200]
def groups(failures):
    out={}
    for f in failures:out.setdefault(cause(f["message"]),[]).append(f["project"])
    return sorted(({"cause":k,"count":len(v),"projects":v} for k,v in out.items()),key=lambda g:(-g["count"],g["cause"]))
if __name__=="__main__":
    d=json.load(open(sys.argv[1],encoding="utf8"))
    print(json.dumps({"causes":[cause(c["message"]) for c in d["causes"]],"groups":groups(d["failures"])},ensure_ascii=False))
