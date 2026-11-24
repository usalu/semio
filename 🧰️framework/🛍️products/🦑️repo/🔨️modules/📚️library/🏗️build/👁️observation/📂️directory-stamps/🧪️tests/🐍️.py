import hashlib,json,os,sys
generated=set(json.load(open(sys.argv[1]))["generatedDirectories"])
noise={".DS_Store","Thumbs.db","desktop.ini",".localized"}
def hollow(path):
    for name in os.listdir(path):
        full=os.path.join(path,name)
        if name in noise or name.endswith(("~",".swp",".swo")):continue
        if os.path.islink(full):return False
        if os.path.isdir(full):
            if name in generated:continue
            if not hollow(full):return False
            continue
        return False
    return True
def fact(path):
    kept=[]
    for name in os.listdir(path):
        full=os.path.join(path,name)
        if name in noise or name.endswith(("~",".swp",".swo")):continue
        link=os.path.islink(full);directory=os.path.isdir(full) and not link
        if (directory or link) and name in generated:continue
        if directory and hollow(full):continue
        kept.append([name,"symlink" if link else "directory" if directory else "file"])
    kept.sort()
    return hashlib.sha256(json.dumps(kept,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
print(json.dumps({p:fact(p) for p in sys.argv[2:]}))
