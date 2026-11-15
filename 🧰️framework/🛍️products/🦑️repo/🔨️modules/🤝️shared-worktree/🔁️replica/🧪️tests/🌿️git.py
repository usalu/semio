#!/usr/bin/env python3
"""🐍️ Independent oracle of the concurrent-commit rule. It shares no code with `../🟦️.ts`: with the third-party git command line only (`rev-list --parents`, `log`, `fsck --lost-found`, `status --porcelain`) it checks that the history of a worktree is linear (no merge, one root), that it contains every expected message exactly as many times as given, that every dangling commit is a superseded copy of a message in the history, and that the stage is clean.
usage: 🌿️git.py <worktree> <count> <message>...   prints one JSON line {"head":...,"subjects":[...],"dangling":N} and exits 0 when every check holds, 1 otherwise
Python 3.9 compatible (no match statements)."""
import json,os,subprocess,sys

def git(worktree,*args):
    environment=dict(os.environ,LC_ALL="C",GIT_CONFIG_GLOBAL=os.devnull,GIT_CONFIG_NOSYSTEM="1")
    result=subprocess.run(["git"]+list(args),cwd=worktree,env=environment,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if result.returncode!=0: raise SystemExit("git %s failed: %s"%(" ".join(args),result.stderr.decode("utf8","replace")))
    return result.stdout.decode("utf8")

def main():
    worktree,count,expected=sys.argv[1],int(sys.argv[2]),sys.argv[3:]
    failures=[]
    rows=[line.split() for line in git(worktree,"rev-list","--parents","HEAD").splitlines()]
    if len(rows)!=count: failures.append("history has %d commits, expected %d"%(len(rows),count))
    if any(len(row)>2 for row in rows): failures.append("history holds a merge")
    if sum(1 for row in rows if len(row)==1)!=1: failures.append("history has more than one root")
    subjects=git(worktree,"log","--format=%s","HEAD").splitlines()
    for message in expected:
        if subjects.count(message)!=1: failures.append("message %r appears %d times"%(message,subjects.count(message)))
    dangling=[line.split()[2] for line in git(worktree,"fsck","--lost-found","--no-reflogs").splitlines() if line.startswith("dangling commit")]
    for oid in dangling:
        subject=git(worktree,"log","-1","--format=%s",oid).strip()
        if subject not in subjects: failures.append("dangling commit %s (%s) is not in the history"%(oid,subject))
    if git(worktree,"status","--porcelain").strip()!="": failures.append("stage or worktree is not clean")
    print(json.dumps({"head":git(worktree,"rev-parse","HEAD").strip(),"subjects":subjects,"dangling":len(dangling)}))
    for line in failures: print("MISMATCH "+line,file=sys.stderr)
    return 1 if failures else 0

if __name__=="__main__": sys.exit(main())
