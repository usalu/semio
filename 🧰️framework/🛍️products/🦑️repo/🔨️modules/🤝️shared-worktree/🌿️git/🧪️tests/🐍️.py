#!/usr/bin/env python3
"""🐍️ Independent oracle of the index register fold. It shares no code with `../🟦️.ts`: it reads the same language-agnostic fixture, applies the registers newer than the base one by one in hlc order with the third-party git command line (`hash-object`, `update-index --cacheinfo`/`--force-remove`, `write-tree`, `ls-tree`) in a scratch repository, and prints the tree id of every fold.
usage: 🐍️.py            prints one JSON line {"trees":[...]} and exits 0 when every fold equals the listing the fixture expects, 1 otherwise
Python 3.9 compatible (no match statements)."""
import json,os,subprocess,sys,tempfile
HERE=os.path.dirname(os.path.abspath(__file__))
FIXTURE=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))

def git(repo,*args,**kwargs):
    environment=dict(os.environ,GIT_DIR=os.path.join(repo,".git"),LC_ALL="C")
    environment.pop("GIT_INDEX_FILE",None)
    result=subprocess.run(["git","-c","core.quotepath=off"]+list(args),cwd=repo,env=environment,input=kwargs.get("stdin"),stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if result.returncode!=0: raise SystemExit("git %s failed: %s"%(" ".join(args),result.stderr.decode("utf8","replace")))
    return result.stdout

def listing(repo,tree):
    rows=[]
    for line in git(repo,"ls-tree","-r","-z",tree).decode("utf8").split("\0"):
        if not line: continue
        head,path=line.split("\t",1)
        mode,kind,oid=head.split(" ")
        rows.append((path,mode,oid))
    return rows

def main():
    failures=[];trees=[]
    for fold in FIXTURE["folds"]:
        with tempfile.TemporaryDirectory(prefix="git-oracle-") as repo:
            git(repo,"init","-q")
            blobs={item["name"]:git(repo,"hash-object","-w","--stdin",stdin=item["content"].encode("utf8")).decode().strip() for item in fold["files"]}
            index=os.path.join(repo,"oracle-index")
            os.environ["GIT_INDEX_FILE"]=index
            for entry in fold["base"]: git(repo,"update-index","--add","--cacheinfo","%s,%s,%s"%(entry["mode"],blobs[entry["file"]],entry["path"]))
            for register in sorted([item for item in fold["registers"] if item["hlc"]>fold["baseHlc"]],key=lambda item:item["hlc"]):
                if register.get("remove"): git(repo,"update-index","--force-remove","--",register["path"])
                else: git(repo,"update-index","--add","--replace","--cacheinfo","%s,%s,%s"%(register["mode"],blobs[register["file"]],register["path"]))
            tree=git(repo,"write-tree").decode().strip()
            del os.environ["GIT_INDEX_FILE"]
            trees.append(tree)
            got=[(path,mode,oid) for path,mode,oid in listing(repo,tree)]
            wanted=[(entry["path"],entry["mode"],blobs[entry["file"]]) for entry in fold["expect"]]
            if got!=wanted: failures.append("%s: fixture %r, git %r"%(fold["name"],wanted,got))
    print(json.dumps({"trees":trees}))
    for line in failures: print("MISMATCH "+line,file=sys.stderr)
    return 1 if failures else 0

if __name__=="__main__": sys.exit(main())
