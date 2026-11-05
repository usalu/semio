import json,ntpath,posixpath,sys
out=[]
for row in json.load(sys.stdin):
 p=ntpath if row["style"]=="win32" else posixpath
 try:
  roots=[p.splitdrive(row[k])[0] for k in ("target","parent")]
  if row["style"]=="win32" and (not all(len(r)==2 and r[1]==":" for r in roots) or roots[0].lower()!=roots[1].lower()): base=None
  else:
   base=p.commonpath([row["target"],row["parent"]])
   if any(p.normcase(base)==p.normcase(row[k]) for k in ("target","parent")):
    above=p.dirname(base)
    base=None if above==base else above
 except ValueError: base=None
 out.append({"base":base,"shared":base is not None and row["device"]=="same" and not row["canceled"]})
print(json.dumps(out))
