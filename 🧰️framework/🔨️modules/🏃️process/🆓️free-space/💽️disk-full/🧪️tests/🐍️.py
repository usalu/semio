import json,re,sys

TEXT=re.compile(r"ENOSPC|EDQUOT|no space left on device|database or disk is full|SQLITE_FULL|disk quota exceeded",re.I)

def full(row,depth=4):
    if row is None or depth<0:return False
    if row.get("code") in ("ENOSPC","EDQUOT") or TEXT.search(row.get("code") or ""):return True
    if TEXT.search(row.get("message") or ""):return True
    return full(row.get("cause"),depth-1)

print(json.dumps([full(case) for case in json.load(sys.stdin)]))
