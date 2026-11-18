"""🐍️ Independent oracle of the tool boundary: the files a script reaches through relative imports, stopping at (and not listing) the entry modules of declared tools other than the script itself. usage: oracle.py ROOT START STOPS_JSON"""
import json,os,re,sys
root,start,stops=sys.argv[1],sys.argv[2],{os.path.normpath(path) for path in json.loads(sys.argv[3])}
pattern=re.compile(r'''(?:^|\n)[ \t]*import[ \t]+(?:[^"\'\n]*?[ \t]+from[ \t]+)?["\'](\.{1,2}/[^"\']+)["\']''')
first=os.path.normpath(os.path.join(root,start)),
seen,stack=[],[os.path.join(root,start)]
while stack:
    path=os.path.normpath(stack.pop())
    if path in seen or path in stops and path not in first:continue
    seen.append(path)
    with open(path,encoding="utf8") as handle:text=handle.read()
    for specifier in pattern.findall(text):stack.append(os.path.join(os.path.dirname(path),specifier))
print(json.dumps(sorted(os.path.relpath(path,root) for path in seen)))
