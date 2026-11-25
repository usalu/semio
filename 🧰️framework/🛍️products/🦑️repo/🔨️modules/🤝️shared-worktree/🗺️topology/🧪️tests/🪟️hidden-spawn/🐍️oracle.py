#!/usr/bin/env python3
"""🐍️ Independent oracle of the hidden-spawn law. It shares no code with `🟦️.ts`: it reads the same scope and cases from `🔣️.json`, finds every child-process call that can open a console window on Windows (a call of `node:child_process` or `Bun.spawn*` without `windowsHide`, the program not a literal POSIX-only tool) and prints them as JSON.
usage: 🐍️oracle.py text <file>              print the violating line numbers of one source text
       🐍️oracle.py scan <repo-root>         print `path:line` of every violation below the roots of `🔣️.json`
Python 3.9 compatible."""
import json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
DATA=json.load(open(os.path.join(HERE,"🔣️.json"),encoding="utf8"))
FUNCS=DATA["functions"]
MOD=r'["\'](?:node:)?child_process["\']'
def imports(t):
    names={};spaces=set()
    for m in re.finditer(r'import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*'+MOD,t):
        for part in m.group(1).split(','):
            part=part.strip()
            if part and not part.startswith('type '):names[re.split(r'\s+as\s+',part)[-1].strip()]=1
    for m in re.finditer(r'import\s+\*\s+as\s+(\w+)\s+from\s*'+MOD,t):spaces.add(m.group(1))
    for m in re.finditer(r'import\s+(\w+)\s+from\s*'+MOD,t):spaces.add(m.group(1))
    for m in re.finditer(r'(?:const|let|var)\s*\{([^}]*)\}\s*=\s*(?:await\s+import|require)\(\s*'+MOD+r'\s*\)',t):
        for part in m.group(1).split(','):
            part=part.strip()
            if part:names[re.split(r'\s*:\s*',part)[-1].strip()]=1
    for m in re.finditer(r'(?:const|let|var)\s+(\w+)\s*=\s*(?:await\s+import|require)\(\s*'+MOD+r'\s*\)',t):spaces.add(m.group(1))
    return names,spaces
def close(t,i):
    depth=0;quote=None;j=i
    while j<len(t):
        c=t[j]
        if quote:
            if c=='\\':j+=1
            elif quote=='`' and c=='$' and t[j+1:j+2]=='{':j=close(t,j+1)
            elif c==quote:quote=None
        elif c in '"\'`':quote=c
        elif c=='/' and t[j+1:j+2]=='/':
            j=t.find('\n',j)
            if j<0:return -1
        elif c=='/' and t[j+1:j+2]=='*':j=t.find('*/',j)+1
        elif c in '([{':depth+=1
        elif c in ')]}':
            depth-=1
            if depth==0:return j
        j+=1
    return -1
def arguments(t,start,end):
    out=[];depth=0;quote=None;j=start+1;last=j
    while j<end:
        c=t[j]
        if quote:
            if c=='\\':j+=1
            elif quote=='`' and c=='$' and t[j+1:j+2]=='{':j=close(t,j+1)
            elif c==quote:quote=None
        elif c in '"\'`':quote=c
        elif c in '([{':depth+=1
        elif c in ')]}':depth-=1
        elif c==',' and depth==0:out.append(t[last:j]);last=j+1
        j+=1
    if t[last:end].strip():out.append(t[last:end])
    return out
def violations(t):
    names,spaces=imports(t);pats=[]
    if names:pats.append(r'(?<![\w.$])(?:'+'|'.join(map(re.escape,names))+r')\s*(?:<[^()]*>)?\s*\(')
    if spaces:pats.append(r'(?<![\w$])(?:'+'|'.join(map(re.escape,spaces))+r')\.(?:'+'|'.join(FUNCS)+r')\s*\(')
    pats.append(r'(?<![\w$])Bun\.(?:spawn|spawnSync)\s*\(')
    pats.append(r'import\(\s*'+MOD+r'\s*\)\s*\)?\.(?:'+'|'.join(FUNCS)+r')\s*\(')
    seen=set();lines=set()
    for p in pats:
        for m in re.finditer(p,t):
            k=m.end()-1
            if k in seen:continue
            seen.add(k)
            if t[t.rfind('\n',0,m.start())+1:m.start()].lstrip().startswith(('//','*','/*')):continue
            e=close(t,k)
            if e<0:continue
            args=arguments(t,k,e)
            if args and re.match(r'\s*\w+\??\s*:\s*[A-Za-z]',args[0]):continue
            if re.search(r'(?<![\w"\'`])windowsHide\s*(?::\s*(?!false\b)|[,}])',t[k:e+1]):continue
            lit=re.fullmatch(r'\s*(?:"([^"]*)"|\'([^\']*)\')\s*',args[0]) if args else None
            program=(lit.group(1) or lit.group(2)) if lit else None
            if program is not None and (program.startswith('/') or program in DATA["posixOnlyPrograms"]):continue
            lines.add(t.count('\n',0,m.start())+1)
    return sorted(lines)
def scan(root):
    out=[]
    for rel in DATA["roots"]:
        for d,ds,fs in os.walk(os.path.join(root,rel)):
            ds[:]=sorted(x for x in ds if x not in DATA["skipDirectories"])
            for f in sorted(fs):
                if any(f.endswith(x) for x in DATA["extensions"]):
                    p=os.path.join(d,f)
                    for line in violations(open(p,encoding="utf8").read()):out.append(os.path.relpath(p,root).replace(os.sep,"/")+":"+str(line))
    return sorted(out)
if __name__=="__main__":
    if len(sys.argv)==3 and sys.argv[1]=="text":print(json.dumps(violations(open(sys.argv[2],encoding="utf8").read())))
    elif len(sys.argv)==3 and sys.argv[1]=="scan":print(json.dumps(scan(sys.argv[2]),ensure_ascii=False))
    else:sys.exit(__doc__)
