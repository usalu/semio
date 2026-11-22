#!/usr/bin/env python3
import json,re,sys
POLICY=json.load(open(sys.argv[2]))["definitions"]["Policy"]["const"]["source"]
ALLOWED_ATTR=set(POLICY["attributesAllowed"]);FORBID=set(POLICY["bodyKeywordsForbidden"]);FORBID_ATTR=set(POLICY["bodyAttributesForbidden"]);MACROS=set(POLICY["bodyMacrosAllowed"]);FEATURES=set(POLICY["featuresVoiding"])
LIT=re.compile(r'''//[^\n]*|/\*|b?r(#*)"|b?"|b?'(?:\\(?:u\{[^}]*\}|x..|.)|[^\\'\n])'|'[A-Za-z_]\w*''',re.S)
def skeleton(text):
    out=list(text);i=0;n=len(text)
    while True:
        m=LIT.search(text,i)
        if not m:break
        s=m.group(0)
        if s.startswith("//"):
            end=m.end()
        elif s=="/*":
            depth=1;j=m.end()
            while j<n and depth:
                if text.startswith("/*",j):depth+=1;j+=2
                elif text.startswith("*/",j):depth-=1;j+=2
                else:j+=1
            if depth:return None
            end=j
        elif re.match(r'b?r#*"',s):
            hashes=m.group(1) or "";close='"'+hashes;j=text.find(close,m.end())
            if j<0:return None
            end=j+len(close)
        elif s.endswith('"') and not s.endswith("'"):
            j=m.end()
            while j<n and text[j]!='"':j+=2 if text[j]=="\\" else 1
            if j>=n:return None
            end=j+1
        else:
            end=m.end()
            if s[0]=="'" and s[1:].isidentifier() and not (s.endswith("'") and len(s)>2):
                for k in range(m.start(),end):out[k]="\x02"
                i=end;continue
        for k in range(m.start(),end):
            if out[k]!="\n":out[k]="\x01" if s.startswith(('"',"b\"","r","br","'","b'")) and not s.startswith("//") and s!="/*" else " "
        i=end
    return "".join(out)
def match_close(sk,i):
    pairs={"(":")","[":"]","{":"}"};stack=[]
    for j in range(i,len(sk)):
        c=sk[j]
        if c in pairs:stack.append(pairs[c])
        elif c in ")]}":
            if not stack or stack.pop()!=c:return -1
            if not stack:return j
    return -1
def strip_attrs(h):
    names=[]
    while True:
        h=h.lstrip()
        m=re.match(r"#\s*!?\s*\[",h)
        if not m:break
        close=match_close(h,m.end()-1)
        if close<0:return None,None
        inner=h[m.end():close];nm=re.match(r"\s*([A-Za-z_]\w*)",inner);names.append(nm.group(1) if nm else "?")
        h=h[close+1:]
    return h,names
def analyse(text):
    sk=skeleton(text)
    if sk is None:return dict(masked=text,bodies=0,void=True,splice=False)
    splice=re.search(r"\binclude\s*!",sk) is not None
    voided=False;cuts=[]
    for m in re.finditer(r"#\s*!?\s*\[\s*feature\s*\(([^\]]*)\)",sk):
        if any(w in FEATURES for w in re.findall(r"\w+",m.group(1))):voided=True
    def body_observable(a,b):
        body=sk[a+1:b]
        for w in re.findall(r"[A-Za-z_]\w*",body):
            if w in FORBID:return True
        for m in re.finditer(r"([A-Za-z_]\w*)\s*!\s*[(\[{]",body):
            if m.group(1) not in MACROS:return True
        for m in re.finditer(r"#\s*!?\s*\[\s*(?:unsafe\s*\(\s*)?([A-Za-z_]\w*)",body):
            if m.group(1) in FORBID_ATTR:return True
        for w in FORBID_ATTR:
            for m in re.finditer(r"#[^;{}]{0,12}\b"+w+r"\b",body):return True
        return False
    def walk(a,b,ctx):
        nonlocal voided
        i=a;start=a
        while i<b:
            c=sk[i]
            if c==";":start=i+1;i+=1;continue
            if c in "([":
                j=match_close(sk,i);i=j+1;continue
            if c=="}":start=i+1;i+=1;continue
            if c=="{":
                header=sk[start:i];h,attrs=strip_attrs(header)
                close=match_close(sk,i)
                h=re.sub(r"^\s*pub\s*(\([^)]*\))?","",h).strip()
                if re.search(r"\btype\b[^;=]*=.*\bimpl\b",h+" "):voided=True
                mfn=re.match(r"((?:(?:unsafe|const|async|default|safe|auto|gen|extern(?:\s+\x01+)?)\s+)*)fn\s+([A-Za-z_]\w*)(.*)$",h,re.S)
                mimpl=re.match(r"((?:unsafe\s+)*)impl\b(.*)$",h,re.S)
                mmod=re.match(r"mod\s+\w+\s*$",h)
                if mfn and not re.match(r"const\s*$",mfn.group(1) or "x") :
                    mods=mfn.group(1).split();rest=mfn.group(3)
                    ok=ctx!="opaque" and all(m=="unsafe" for m in mods) and all(x in ALLOWED_ATTR for x in attrs) and not rest.lstrip().startswith("<") and not re.search(r"\bwhere\b",rest) and not re.search(r"\bimpl\b",rest) and not body_observable(i,close)
                    if ok:cuts.append((i,close+1))
                elif mimpl:
                    rest=mimpl.group(2);mono=all(x in ALLOWED_ATTR for x in attrs) and not rest.lstrip().startswith("<") and not re.search(r"\b(where|const|impl)\b",rest)
                    walk(i+1,close,"impl" if mono else "opaque")
                elif mmod:
                    walk(i+1,close,"module")
                i=close+1;start=i;continue
            i+=1
    walk(0,len(sk),"module")
    if voided:return dict(masked=text,bodies=0,void=True,splice=splice)
    cuts.sort();res="";last=0
    for a,b in cuts:res+=text[last:a]+POLICY["maskedBody"];last=b
    res+=text[last:]
    return dict(masked=res,bodies=len(cuts),void=False,splice=splice)
def digest_equal(old,new):
    a=analyse(old);b=analyse(new)
    wa=a["void"] or a["splice"];wb=b["void"] or b["splice"]
    if wa!=wb:return False,a,b
    return ((old==new) if wa else (a["masked"]==b["masked"])),a,b

def closure_cases(fixture):
    from pathlib import PurePosixPath
    crate = fixture["crate"]
    unit = crate.rsplit("-", 1)[1]
    results = []
    for row in fixture["cases"]:
        artifacts = {PurePosixPath(item["path"]).name: item["hash"] for item in row["artifacts"]}
        metadata = artifacts.get(f"lib{crate}.rmeta")
        archive = f"lib{crate}.rlib" in artifacts
        managed = any(PurePosixPath(path).match(f"build/*/{unit}/out") for path in row["paths"])
        present = not managed or archive or metadata is not None
        results.append(dict(code=present and metadata in (None, fixture["expectedSvh"]), metadata=present))
    return results

vec=json.load(open(sys.argv[1]))
if sys.argv[3:] == ["closure"]:
    print(json.dumps(closure_cases(vec["closure"])))
    sys.exit(0)
for case in vec["cases"]:
    eq,a,b=digest_equal(case["old"],case["new"])
    print(json.dumps(dict(label=case["label"],equal=eq,oldBodies=a["bodies"],newBodies=b["bodies"],oldMasked=a["masked"]),ensure_ascii=False))
