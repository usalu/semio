#!/usr/bin/env python3
"""🌪️ Independent oracle of the fleet convergence invariants. It shares no code with `../🟦️.ts`: with the third-party git command line and the Python standard library only, it compares every worktree given on the command line with the first one.
usage: 🌪️chaos.py --count N [--message M ...] <worktree> <worktree>...   prints one JSON line and exits 0 when every check holds, 1 otherwise
checks per worktree: identical HEAD, `write-tree`, `ls-files -s`, `status --porcelain=v2`; linear history with exactly N commits and every --message once; `fsck --no-dangling` clean; and identical working files (relative path, executable bit, symlink target, sha256) for everything git lists as tracked or untracked-not-ignored, `.cache` excluded.
Python 3.9 compatible (no match statements)."""
import argparse,hashlib,json,os,subprocess,sys

def git(worktree,*args):
    environment=dict(os.environ,LC_ALL="C",GIT_CONFIG_GLOBAL=os.devnull,GIT_CONFIG_NOSYSTEM="1")
    result=subprocess.run(["git"]+list(args),cwd=worktree,env=environment,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    return result.returncode,result.stdout.decode("utf8","replace"),result.stderr.decode("utf8","replace")

def must(worktree,*args):
    code,out,err=git(worktree,*args)
    if code!=0: raise SystemExit("git %s failed in %s: %s"%(" ".join(args),worktree,err))
    return out

def files(worktree):
    names=must(worktree,"ls-files","-z","--cached","--others","--exclude-standard").split("\0")
    found={}
    for name in sorted(set(item for item in names if item and not item.startswith(".cache/"))):
        path=os.path.join(worktree,name)
        try: info=os.lstat(path)
        except OSError: found[name]="missing"; continue
        if os.path.islink(path): found[name]="link "+os.readlink(path)
        elif os.path.isfile(path):
            with open(path,"rb") as handle: found[name]="file %s %s"%("x" if info.st_mode&0o111 else "-",hashlib.sha256(handle.read()).hexdigest())
        elif not os.path.isdir(path): found[name]="other"
    return found

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--count",type=int,required=True)
    parser.add_argument("--message",action="append",default=[])
    parser.add_argument("worktrees",nargs="+")
    arguments=parser.parse_args()
    failures=[]
    views=[]
    for worktree in arguments.worktrees:
        rows=[line.split() for line in must(worktree,"rev-list","--parents","HEAD").splitlines()]
        if len(rows)!=arguments.count: failures.append("%s: history has %d commits, expected %d"%(worktree,len(rows),arguments.count))
        if any(len(row)>2 for row in rows): failures.append("%s: history holds a merge"%worktree)
        subjects=must(worktree,"log","--format=%s","HEAD").splitlines()
        for message in arguments.message:
            if subjects.count(message)!=1: failures.append("%s: message %r appears %d times"%(worktree,message,subjects.count(message)))
        code,out,err=git(worktree,"fsck","--no-dangling","--no-reflogs")
        if code!=0 or "missing" in out+err or "broken" in out+err: failures.append("%s: fsck: %s"%(worktree,(out+err).strip()[:300]))
        views.append({"head":must(worktree,"rev-parse","HEAD").strip(),"tree":must(worktree,"write-tree").strip(),"stage":must(worktree,"ls-files","-s","-z"),"status":must(worktree,"--no-optional-locks","status","--porcelain=v2","-z","--untracked-files=all","--no-renames"),"files":files(worktree)})
    for worktree,view in zip(arguments.worktrees[1:],views[1:]):
        for key in ("head","tree","stage","status"):
            if view[key]!=views[0][key]: failures.append("%s differs from %s in %s"%(worktree,arguments.worktrees[0],key))
        left,right=views[0]["files"],view["files"]
        for name in sorted(set(left)|set(right)):
            if left.get(name)!=right.get(name): failures.append("%s: file %s: %s vs %s"%(worktree,name,left.get(name,"absent"),right.get(name,"absent")))
    print(json.dumps({"worktrees":len(arguments.worktrees),"head":views[0]["head"],"files":len(views[0]["files"])}))
    for line in failures: print("MISMATCH "+line,file=sys.stderr)
    return 1 if failures else 0

if __name__=="__main__": sys.exit(main())
