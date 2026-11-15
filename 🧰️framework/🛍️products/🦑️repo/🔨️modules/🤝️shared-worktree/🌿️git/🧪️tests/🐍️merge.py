#!/usr/bin/env python3
"""🐍️ Independent oracle of the per-path three-way tree merge of `../🟦️.ts` (`mergeTreesV1`). It shares no code with it: it lists the three trees with the third-party git command line (`ls-tree -r -z`), applies the rule per path in Python and prints the expected listing as JSON.
usage: 🐍️merge.py <repository> <base-tree|empty> <ours-tree> <theirs-tree> <theirs-wins:0|1>
rule: a path theirs left alone keeps ours; a path ours left alone takes theirs; a path both changed identically stays; a path both changed differently takes theirs when theirs-wins is 1 and ours otherwise.
Python 3.9 compatible (no match statements)."""
import json,os,subprocess,sys

def git(repo,*args):
    environment=dict(os.environ,GIT_DIR=os.path.join(repo,".git"),LC_ALL="C")
    environment.pop("GIT_INDEX_FILE",None)
    result=subprocess.run(["git","-c","core.quotepath=off"]+list(args),cwd=repo,env=environment,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if result.returncode!=0: raise SystemExit("git %s failed: %s"%(" ".join(args),result.stderr.decode("utf8","replace")))
    return result.stdout

def listing(repo,tree):
    rows={}
    if tree=="empty": return rows
    for line in git(repo,"ls-tree","-r","-z",tree).decode("utf8").split("\0"):
        if not line: continue
        head,path=line.split("\t",1)
        mode,kind,oid=head.split(" ")
        rows[path]=(mode,oid)
    return rows

def main():
    repo,base,ours,theirs,wins=sys.argv[1:6]
    b,o,t=listing(repo,base),listing(repo,ours),listing(repo,theirs)
    merged={};conflicts=[]
    for path in sorted(set(b)|set(o)|set(t)):
        bv,ov,tv=b.get(path),o.get(path),t.get(path)
        if tv==bv: value=ov
        elif ov==bv: value=tv
        elif ov==tv: value=ov
        else:
            conflicts.append(path)
            value=tv if wins=="1" else ov
        if value is not None: merged[path]=value
    print(json.dumps({"entries":["%s %s %s"%(mode,oid,path) for path,(mode,oid) in sorted(merged.items())],"conflicts":conflicts}))
    return 0

if __name__=="__main__": sys.exit(main())
