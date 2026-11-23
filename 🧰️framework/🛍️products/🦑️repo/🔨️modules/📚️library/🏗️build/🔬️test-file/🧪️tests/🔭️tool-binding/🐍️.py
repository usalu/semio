import json,os,re,sys
request=json.load(sys.stdin)
hints=request["hints"]
closure=request["closure"]
pattern=re.compile(r"""(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+\.ts)["']""")
inside=os.sep+closure["directory"]+os.sep
def read(path):
    try:
        with open(path,encoding="utf-8") as handle:
            return handle.read()
    except OSError:
        return None
def tools(file):
    found,seen,queue=set(),set(),[os.path.abspath(file)]
    while queue:
        path=queue.pop()
        if path in seen or len(seen)>=closure["maximumFiles"]:
            continue
        seen.add(path)
        text=read(path)
        if text is None:
            continue
        for tool,words in hints.items():
            if any(word in text for word in words):
                found.add(tool)
        for match in pattern.finditer(text):
            target=os.path.abspath(os.path.join(os.path.dirname(path),match.group(1)))
            if inside in target and target not in seen:
                queue.append(target)
    return sorted(found)
json.dump({file:tools(file) for file in request["files"]},sys.stdout)
