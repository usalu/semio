#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree boundary. It shares no code with `../🟦️.ts`: it reads the same language-agnostic fixture, recomputes the path and link policy with the standard library and validates the gitignore expectations with the third-party `pathspec` library when it is installed.
usage: 🐍️.py            exit 0 when every vector agrees, 1 on any mismatch (the gitignore part is skipped with a message when `pathspec` is missing)
Python 3.9 compatible (no match statements)."""
import json,os,re,sys,unicodedata
HERE=os.path.dirname(os.path.abspath(__file__))
FIXTURE=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))
ILLEGAL=re.compile(r'[<>:"|?*\\\x00-\x1f\x7f-\x9f]')
RESERVED=re.compile("(CON|PRN|AUX|NUL|COM[1-9\u00b9\u00b2\u00b3]|LPT[1-9\u00b9\u00b2\u00b3])")

def refuse_path(path):
    if path!=unicodedata.normalize("NFC",path): return "not-nfc"
    if len(path)>256: return "too-long"
    parts=path.split("/")
    if any(len(part.encode("utf-8","surrogatepass"))>255 for part in parts): return "component-too-long"
    if ILLEGAL.search(path): return "illegal-character"
    if any(part.endswith(".") or part.endswith(" ") for part in parts): return "trailing-dot-or-space"
    if any(RESERVED.fullmatch(part.split(".")[0].rstrip(" ").upper()) for part in parts): return "reserved-name"
    return None

def refuse_link(path,target):
    normal=target.replace("\\","/")
    if normal=="" or "\0" in normal or normal.startswith("/") or re.match(r"[A-Za-z]:",normal): return "unsafe-link"
    depth=len(path.split("/"))-1
    for part in normal.split("/"):
        if part in ("","."): continue
        if part=="..":
            depth-=1
            if depth<0: return "unsafe-link"
        else: depth+=1
    return None

def path_of(row): return row["path"]*row.get("times",1)+row.get("suffix","")

def repository_lines():
    directory=HERE
    while not os.path.exists(os.path.join(directory,".gitignore")):
        parent=os.path.dirname(directory)
        if parent==directory: raise SystemExit("the repository .gitignore was not found above the oracle")
        directory=parent
    return open(os.path.join(directory,".gitignore"),encoding="utf8",newline="").read().replace("\r\n","\n").split("\n")

def matches(spec,path,directory):
    verdict=False
    for pattern in spec.patterns:
        if pattern.include is None: continue
        subject=path if not directory or pattern.pattern.endswith("/**") else path+"/"
        if pattern.match_file(subject): verdict=pattern.include
    return verdict

def ignored(spec,path,directory):
    parts=path.split("/")
    for index in range(1,len(parts)):
        if matches(spec,"/".join(parts[:index]),True): return True
    return matches(spec,path,directory)

def main():
    failures=[];checked=0
    for row in FIXTURE["paths"]:
        checked+=1;got=refuse_path(path_of(row))
        if got!=row["refusal"]: failures.append("path %r: fixture %r, oracle %r"%(path_of(row)[:60],row["refusal"],got))
    for row in FIXTURE["links"]:
        checked+=1;got=refuse_link(row["path"],row["target"])
        if got!=row["refusal"]: failures.append("link %r -> %r: fixture %r, oracle %r"%(row["path"],row["target"],row["refusal"],got))
    try:
        import pathspec
        spec_class=pathspec.GitIgnoreSpec
    except (ImportError,AttributeError):
        print("SKIP gitignore vectors: pathspec.GitIgnoreSpec is not importable (pip install pathspec); path and link policy were checked")
        spec_class=None
    if spec_class is not None:
        for group in FIXTURE["ignore"]:
            lines=repository_lines() if group.get("source")=="repository" else group["lines"]
            spec=spec_class.from_lines(lines)
            for case in group["cases"]:
                checked+=1;got=ignored(spec,case["path"],case["directory"])
                if got!=case["ignored"]: failures.append("ignore [%s] %r directory=%s: fixture %s, pathspec %s"%(group["name"],case["path"],case["directory"],case["ignored"],got))
    for line in failures: print("MISMATCH "+line)
    print("%d vectors checked, %d mismatches"%(checked,len(failures)))
    return 1 if failures else 0

if __name__=="__main__": sys.exit(main())
