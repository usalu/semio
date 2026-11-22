import hashlib,json,os,sys
path=sys.argv[1]
st=os.lstat(path)
print(json.dumps({"stamp":":".join(str(v) for v in (st.st_dev,st.st_ino,st.st_size,st.st_mtime_ns)),"digest":hashlib.sha256(open(path,"rb").read()).hexdigest()}))
