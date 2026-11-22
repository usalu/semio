"""🐍️ Independent oracle for stranded images: reads each plist of the corpus with the standard library parser and names the whole-disk device and the mount points of the image when it is attached somewhere other than its canonical mount point."""
import json,plistlib,re,sys
def stranded(text,image,root):
    rows=[r for r in plistlib.loads(text.encode()).get("images",[]) if r.get("image-path")==image]
    if len(rows)!=1:
        return None
    entities=rows[0].get("system-entities",[])
    mounts=[e["mount-point"] for e in entities if "mount-point" in e]
    devices=[e["dev-entry"] for e in entities if re.fullmatch(r"/dev/disk[0-9]+",e.get("dev-entry",""))]
    if not devices or root in mounts:
        return None
    return {"device":devices[0],"mountPoints":mounts}
corpus=json.load(open(sys.argv[1]))
print(json.dumps([stranded(c["plist"],corpus["image"],corpus["root"]) for c in corpus["cases"]]))
