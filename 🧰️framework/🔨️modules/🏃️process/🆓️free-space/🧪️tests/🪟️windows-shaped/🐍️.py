import json,math,sys

def judge(r):
    b,k,a=r['bsize'],r['blocks'],r['bavail']
    if not(b>=1) or not(a>=0) or not math.isfinite(a*b):
        return None
    if not(k>0) and a==0:
        return None
    files,ffree=r.get('files'),r.get('ffree')
    inodes=ffree if files is not None and files>0 and ffree is not None and 0<=ffree<=files else None
    return {'bytes':a*b,'total':b*k if (k>0 and k>=a and math.isfinite(b*k)) else None,'inodes':inodes}

print(json.dumps([judge(r) for r in json.loads(sys.stdin.read())]))
