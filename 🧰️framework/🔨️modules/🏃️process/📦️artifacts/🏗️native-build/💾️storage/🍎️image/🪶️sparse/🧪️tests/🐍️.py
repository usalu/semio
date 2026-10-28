"""🐍️ Independent oracle for image sparse."""
import ctypes,json,os,sys
root,probe,payload=sys.argv[1],sys.argv[2],int(sys.argv[3])
libc=ctypes.CDLL("/usr/lib/libSystem.B.dylib",use_errno=True)
for name in("a","b"):os.makedirs(os.path.join(probe,name),exist_ok=True)
open(os.path.join(probe,"a","x"),"w").write("a");open(os.path.join(probe,"b","y"),"w").write("b")
swap=libc.renameatx_np(-2,os.path.join(probe,"a").encode(),-2,os.path.join(probe,"b").encode(),2)
open(os.path.join(probe,"Case"),"w").write("1");open(os.path.join(probe,"case"),"w").write("2")
with open(os.path.join(probe,"payload"),"wb") as f:
    f.write(os.urandom(payload));f.flush();os.fsync(f.fileno())
s=os.statvfs(root)
print(json.dumps({"swap":swap,"after":[sorted(os.listdir(os.path.join(probe,"a"))),sorted(os.listdir(os.path.join(probe,"b")))],"caseDistinct":sorted(n for n in os.listdir(probe) if n.lower()=="case"),"volumeBytes":s.f_blocks*s.f_frsize}))
