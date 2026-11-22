"""Independent model of the native Nx shard size: factor times the file map baseline plus the image overhead, aligned up; the filesystem is the image minus its overhead."""
import json,sys
p=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["NativePolicy"]["const"]
s=p["sizing"]
raw=s["factor"]*s["fileMapBaselineBytes"]+s["imageOverheadBytes"]
maximum=-(-raw//s["alignBytes"])*s["alignBytes"]
print(json.dumps({"maximumPhysicalBytes":maximum,"filesystemBytes":maximum-s["imageOverheadBytes"]}))
