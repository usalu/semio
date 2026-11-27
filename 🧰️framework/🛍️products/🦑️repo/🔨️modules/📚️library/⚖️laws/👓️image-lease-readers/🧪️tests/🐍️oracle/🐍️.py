import json,os,re,sys
PUNCT=re.compile(r"\.\.\.|===|!==|\*\*=|<<=|&&=|\|\|=|\?\?=|=>|==|!=|<=|&&|\|\||\?\?|\?\.|\+\+|--|[-+*/%&|^]=|\*\*|<<|.",re.S)
IDENT=re.compile(r"[A-Za-z_$][\w$]*")
NUMBER=re.compile(r"\d[\w.]*")
BEFORE_REGEX={"return","typeof","case","in","of","delete","void","throw","new","else","do","instanceof","yield","await"}
OPEN={"(","[","{"};CLOSE={")","]","}"}
CONTROL={"if","for","while","switch","catch","with"}
STARTS={"async","declare","enum","abstract","namespace","const","let","var","return","if","for","while","switch","try","throw","export","import","function","class","await","type","interface"}
CONTINUES={"=",",","(","?",":","=>","&&","||","??","+","-","*","/","|","&",".","?.","["}
TYPE_TOKENS={".","<",">","[","]","|","&",",","?"}
def string_end(src,i):
    quote=src[i];j=i+1
    while j<len(src) and src[j]!=quote and src[j]!="\n":j+=2 if src[j]=="\\" else 1
    return min(j+1,len(src))
def template_end(src,i):
    i+=1
    while i<len(src):
        c=src[i]
        if c=="\\":i+=2;continue
        if c=="`":return i+1
        if src.startswith("${",i):i=expression_end(src,i+2);continue
        i+=1
    return len(src)
def expression_end(src,i):
    depth=1
    while i<len(src):
        c=src[i]
        if c=="`":i=template_end(src,i);continue
        if c in "\"'":i=string_end(src,i);continue
        if c=="{":depth+=1
        elif c=="}":
            depth-=1
            if depth==0:return i+1
        i+=1
    return len(src)
def regex_end(src,i):
    j=i+1;inside=False
    while j<len(src) and src[j]!="\n":
        c=src[j]
        if c=="\\":j+=2;continue
        if c=="[":inside=True
        elif c=="]":inside=False
        elif c=="/" and not inside:
            j+=1
            while j<len(src) and src[j].isalpha():j+=1
            return j
        j+=1
    return -1
def tokenize(src):
    out=[];i=0;line=1
    while i<len(src):
        c=src[i]
        if c=="\n":line+=1;i+=1;continue
        if c.isspace():i+=1;continue
        if src.startswith("//",i):
            j=src.find("\n",i);i=len(src) if j<0 else j;continue
        if src.startswith("/*",i):
            j=src.find("*/",i+2);j=len(src) if j<0 else j+2;line+=src.count("\n",i,j);i=j;continue
        if c=="`":
            j=template_end(src,i);out.append(("str",src[i:j],i,j,line));line+=src.count("\n",i,j);i=j;continue
        if c in "\"'":
            j=string_end(src,i);out.append(("str",src[i:j],i,j,line));i=j;continue
        if c=="/":
            last=out[-1] if out else None
            allowed=last is None or (last[0]=="id" and last[1] in BEFORE_REGEX) or (last[0]=="p" and last[1] not in CLOSE)
            j=regex_end(src,i) if allowed else -1
            if j>0:out.append(("str",src[i:j],i,j,line));i=j;continue
        m=IDENT.match(src,i)
        if m:out.append(("id",m.group(),i,m.end(),line));i=m.end();continue
        m=NUMBER.match(src,i)
        if m:out.append(("num",m.group(),i,m.end(),line));i=m.end();continue
        m=PUNCT.match(src,i);out.append(("p",m.group(),i,m.end(),line));i=m.end()
    return out
def brackets(toks):
    match={};stack=[]
    for index,token in enumerate(toks):
        if token[0]!="p":continue
        if token[1] in OPEN:stack.append(index)
        elif token[1] in CLOSE and stack:
            start=stack.pop();match[start]=index;match[index]=start
    return match
def generic_end(toks,match,k):
    if k<1 or toks[k-1][0]!="id" or not toks[k-1][1][:1].isupper():return None
    depth=0;i=k
    while i<len(toks) and i<k+400:
        x=toks[i]
        if x[0]=="p" and x[1] in OPEN and i in match:i=match[i]+1;continue
        if x[0]=="p" and x[1]=="<":depth+=1
        elif x[0]=="p" and x[1]==">":
            depth-=1
            if depth==0:return i+1
        elif not (x[0] in ("id","str","num") or x[1] in TYPE_TOKENS or x[1] in (":",";","=>")):return None
        i+=1
    return None
def expression_range(src,toks,match,start):
    k=start
    while k<len(toks):
        token=toks[k]
        if token[0]=="p" and token[1] in OPEN and k in match:k=match[k]+1;continue
        if token[0]=="p" and token[1]=="<":
            g=generic_end(toks,match,k)
            if g is not None:k=g;continue
        if token[0]=="p" and (token[1] in (";",",") or token[1] in CLOSE):return k
        if k>start and token[0]=="id" and token[1] in STARTS and toks[k-1][1] not in CONTINUES and "\n" in src[toks[k-1][3]:token[2]]:return k
        k+=1
    return k
def units(src,toks,match):
    out=[];unit=0;k=0
    while k<len(toks):
        token=toks[k]
        if k>0 and token[0]=="id" and token[1] in STARTS and toks[k-1][1] not in CONTINUES and "\n" in src[toks[k-1][3]:token[2]]:unit+=1
        out.append(unit)
        if token[0]=="p" and token[1] in OPEN and k in match:
            out.extend([unit]*(match[k]-k))
            k=match[k]+1
            continue
        if token[0]=="p" and token[1]==";":unit+=1
        k+=1
    return out
def callee_of(toks,index):
    k=index;parts=[toks[index][1]]
    while k>=2 and toks[k-1][1] in (".","?.") and toks[k-2][0]=="id":parts.insert(0,toks[k-2][1]);k-=2
    if k>=1 and toks[k-1][1] in (".","?."):return "."+toks[index][1]
    return ".".join(parts)
def function_ranges(src,toks,match,parameters):
    out=[]
    def head(text,after,limit):
        if not any(re.search(p,text) for p in parameters):return
        nxt=toks[after][1] if after<len(toks) else ""
        if nxt=="=>":
            body=after+1
            if body<len(toks) and toks[body][1]=="{" and body in match:out.append((body,match[body]))
            else:out.append((body,expression_range(src,toks,match,body)))
        elif nxt=="{" and after in match:out.append((after,match[after]))
    for i,token in enumerate(toks):
        if token[0]=="p" and token[1]=="(" and i in match:
            if i>0 and toks[i-1][1] in CONTROL:continue
            after=match[i]+1
            if after<len(toks) and toks[after][1]==":":
                after+=1
                while after<len(toks) and (toks[after][0] in ("id","str","num") or toks[after][1] in TYPE_TOKENS):after+=1
            head(src[token[2]:toks[match[i]][3]],after,None)
        elif token[0]=="p" and token[1]=="=>" and i>0 and toks[i-1][0]=="id" and not (i>1 and toks[i-2][1]==")"):
            head(toks[i-1][1],i,None)
    return out
MODIFIERS={"public","private","protected","readonly"}
def binding_names(toks,start,end):
    return [x[1] for k,x in enumerate(toks[start:end],start) if x[0]=="id" and not (k+1<end and toks[k+1][0]=="p" and toks[k+1][1]==":")]
def references(text):
    toks=tokenize(text);stack=[];out=[]
    for k,token in enumerate(toks):
        if token[0]=="p":
            if token[1] in OPEN:stack.append(token[1])
            elif token[1] in CLOSE and stack:stack.pop()
            continue
        if token[0]=="str" and token[1].startswith("`"):
            body=token[1];i=1
            while i<len(body):
                if body[i]=="\\":i+=2;continue
                if body.startswith("${",i):
                    end=expression_end(body,i+2);out.extend(references(body[i+2:end-1]));i=end;continue
                i+=1
            continue
        if token[0]!="id":continue
        before=toks[k-1][1] if k>0 and toks[k-1][0]=="p" else ""
        after=toks[k+1][1] if k+1<len(toks) and toks[k+1][0]=="p" else ""
        if before in (".","?."):continue
        if after==":" and before in ("{",",") and stack and stack[-1]=="{":continue
        out.append(token[1])
    return out
def declarations(src,toks,match):
    found=[]
    for i,token in enumerate(toks):
        if token[0]!="id" or token[1] not in ("const","let","var") or i+1>=len(toks):continue
        j=i+1
        while j<len(toks):
            if toks[j][1] in ("{","[") and toks[j][0]=="p":
                if j not in match:break
                names=binding_names(toks,j+1,match[j]);j=match[j]+1
            elif toks[j][0]=="id":names=[toks[j][1]];j+=1
            else:break
            kind=None
            if j<len(toks) and toks[j][1]==":" and toks[j][0]=="p":
                first=j+1;j=first
                angle=0
                while j<len(toks):
                    x=toks[j]
                    if x[0]=="p" and x[1] in OPEN and j in match:j=match[j]+1;continue
                    if x[0]=="p" and x[1]=="<":angle+=1
                    elif x[0]=="p" and x[1]==">":angle-=1
                    if x[0]=="p" and x[1] in ("=",";",",") and angle<=0:break
                    j+=1
                if j>first:kind=src[toks[first][2]:toks[j-1][3]]
            init=None
            if j<len(toks) and toks[j][1]=="=" and toks[j][0]=="p":
                end=expression_range(src,toks,match,j+1)
                if end>j+1:init=src[toks[j+1][2]:toks[end-1][3]]
                j=end
            found.append({"names":names,"init":init,"kind":kind,"at":i})
            if j<len(toks) and toks[j][1]=="," and toks[j][0]=="p":j+=1;continue
            break
    return found
def loops(src,toks,match):
    found=[]
    for i,token in enumerate(toks):
        if token[0]!="id" or token[1]!="for":continue
        j=i+1
        if j<len(toks) and toks[j][1]=="await":j+=1
        if j>=len(toks) or toks[j][1]!="(" or j not in match:continue
        close=match[j];k=j+1
        if k>=close or toks[k][1] not in ("const","let","var"):continue
        k+=1
        if k>=close:continue
        if toks[k][1] in ("{","[") and toks[k][0]=="p" and k in match:names=binding_names(toks,k+1,match[k]);k=match[k]+1
        elif toks[k][0]=="id":names=[toks[k][1]];k+=1
        else:continue
        if k+1<close and toks[k][0]=="id" and toks[k][1] in ("of","in"):found.append({"names":names,"text":src[toks[k+1][2]:toks[close-1][3]],"at":i})
    return found
def segments(toks,match,start,end):
    out=[];k=start;first=start;angle=0
    while k<end:
        x=toks[k]
        if x[0]=="p" and x[1] in OPEN and k in match:k=match[k]+1;continue
        if x[0]=="p" and x[1]=="<":angle+=1
        elif x[0]=="p" and x[1] in (">",">>",">>>"):angle-=len(x[1])
        if x[0]=="p" and x[1]=="," and angle<=0:out.append((first,k));first=k+1
        k+=1
    if first<end:out.append((first,end))
    return out
def parameter(src,toks,match,s,e):
    k=s
    while k<e and toks[k][0]=="id" and toks[k][1] in MODIFIERS:k+=1
    if k<e and toks[k][0]=="p" and toks[k][1]=="...":k+=1
    if k>=e:return None
    if toks[k][0]=="p" and toks[k][1] in ("{","[") and k in match:names=binding_names(toks,k+1,match[k]);k=match[k]+1
    elif toks[k][0]=="id":names=[toks[k][1]];k+=1
    else:return None
    if k<e and toks[k][0]=="p" and toks[k][1]=="?":k+=1
    kind=None;default=None
    if k<e and toks[k][0]=="p" and toks[k][1]==":":
        first=k+1;k=first
        while k<e:
            x=toks[k]
            if x[0]=="p" and x[1] in OPEN and k in match:k=match[k]+1;continue
            if x[0]=="p" and x[1]=="=":break
            k+=1
        if k>first:kind=src[toks[first][2]:toks[min(k,e)-1][3]]
    if k+1<e and toks[k][0]=="p" and toks[k][1]=="=":default=src[toks[k+1][2]:toks[e-1][3]]
    return {"names":names,"kind":kind,"default":default}
def heads(src,toks,match):
    out=[];consumed=set()
    def declared_name(m):
        if m>=0 and toks[m][1]=="*" and toks[m][0]=="p":m-=1
        if m>=0 and toks[m][1]=="function":m-=1
        if m>=0 and toks[m][1]=="async":m-=1
        return toks[m-1][1] if m>=1 and toks[m][1]=="=" and toks[m][0]=="p" and toks[m-1][0]=="id" else None
    for i,token in enumerate(toks):
        if token[0]!="p" or token[1]!="(" or i not in match:continue
        if i>0 and toks[i-1][1] in CONTROL:continue
        after=match[i]+1
        if after<len(toks) and toks[after][1]==":" and toks[after][0]=="p":
            after+=1
            while after<len(toks) and (toks[after][0] in ("id","str","num") or toks[after][1] in TYPE_TOKENS):after+=1
        nxt=toks[after][1] if after<len(toks) else ""
        if nxt not in ("=>","{"):continue
        if nxt=="=>":consumed.add(after)
        name=None
        if i>0:
            m=i-1
            if toks[m][0]=="p" and toks[m][1] in (">",">>"):
                depth=0
                while m>=0:
                    if toks[m][0]=="p" and toks[m][1] in (">",">>"):depth+=len(toks[m][1])
                    elif toks[m][0]=="p" and toks[m][1]=="<":depth-=1
                    m-=1
                    if depth<=0:break
            if m>=1 and toks[m][0]=="id" and toks[m-1][1]=="function":name=toks[m][1]
            elif m>=2 and toks[m][0]=="id" and toks[m-1][1]=="*" and toks[m-2][1]=="function":name=toks[m][1]
            elif nxt=="=>" or toks[i-1][1]=="function":name=declared_name(i-1)
        params=[p for s,e in segments(toks,match,i+1,match[i]) for p in [parameter(src,toks,match,s,e)] if p is not None]
        out.append({"name":name,"params":params,"at":i})
    for k,token in enumerate(toks):
        if token[1]!="=>" or token[0]!="p" or k in consumed or k<1 or toks[k-1][0]!="id":continue
        if k>=2 and (toks[k-2][1]==")" or toks[k-2][1]==":" or toks[k-2][1] in TYPE_TOKENS):continue
        name=declared_name(k-2) if k>=2 else None
        out.append({"name":name,"params":[{"names":[toks[k-1][1]],"kind":None,"default":None}],"at":k-1})
    return out
def sites(path,src,policy):
    mounts=policy["mountPatterns"]+policy["rootPatterns"];types=policy["rootTypes"];openers=policy["openers"];callees=policy["leaseCallees"];parameters=policy["leasedParameters"];carried=policy["leasedArguments"];carriers=set(policy["carrierMethods"])
    toks=tokenize(src);match=brackets(toks);unit=units(src,toks,match)
    depth=[0]*len(toks)
    for a,b in sorted((a,b) for a,b in match.items() if a<b):
        for k in range(a+1,b):depth[k]+=1
    shared=set();local={}
    rules=[];top_names=set()
    def rule(names,text,kind,read,write,top):rules.append({"names":names,"text":text,"type":kind,"read":read,"write":write,"top":top})
    for d in declarations(src,toks,match):
        at=d["at"];top=depth[at]==0
        if top:top_names.update(d["names"])
        if d["init"] is not None:rule(d["names"],d["init"],False,unit[at],unit[at],top)
        if d["kind"] is not None:rule(d["names"],d["kind"],True,unit[at],unit[at],top)
    for row in loops(src,toks,match):rule(row["names"],row["text"],False,unit[row["at"]],unit[row["at"]],False)
    functions={}
    for head in heads(src,toks,match):
        at=head["at"]
        for p in head["params"]:
            if p["default"] is not None:rule(p["names"],p["default"],False,unit[at],unit[at],False)
            if p["kind"] is not None:rule(p["names"],p["kind"],True,unit[at],unit[at],False)
        if head["name"] is not None:functions[head["name"]]=head
    ranges=[];calls=[]
    for i,token in enumerate(toks):
        if token[0]!="id" or i+1>=len(toks):continue
        at=i+1
        if toks[at][1]=="<" and toks[at][0]=="p":
            depth_=0;k=at
            while k<len(toks) and k<at+400:
                x=toks[k]
                if x[0]=="p" and x[1] in OPEN and k in match:k=match[k]+1;continue
                if x[0]=="p" and x[1]=="<":depth_+=1
                elif x[0]=="p" and x[1] in (">",">>",">>>"):
                    depth_-=len(x[1])
                    if depth_<=0:break
                elif not (x[0] in ("id","str","num") or x[1] in TYPE_TOKENS or x[1] in (":",";","=>")):break
                k+=1
            at=k+1 if depth_==0 and k<len(toks) else -1
        if at<0 or at>=len(toks) or toks[at][1]!="(" or toks[at][0]!="p" or at not in match:continue
        if token[1] in CONTROL or token[1] in ("function","return","typeof","await","import","new","super"):continue
        if i>0 and toks[i-1][1] in ("function","new"):continue
        close=match[at]
        if close+1<len(toks) and toks[close+1][1]=="{" and toks[close+1][0]=="p":continue
        callee=callee_of(toks,i)
        calls.append((i,callee,close,at))
        if any(re.search(p,callee) for p in callees):ranges.append((at+1,close))
    ranges+=function_ranges(src,toks,match,parameters)
    for k,token in enumerate(toks):
        if token[0]!="id" or k+1>=len(toks) or toks[k+1][1]!="=" or toks[k+1][0]!="p":continue
        if k>0 and toks[k-1][1] in (".","?.","const","let","var","type"):continue
        end=expression_range(src,toks,match,k+2)
        if end>k+2:rule([token[1]],src[toks[k+2][2]:toks[end-1][3]],False,unit[k],unit[k],token[1] in top_names)
    for i,callee,close,at in calls:
        parts=callee.split(".")
        if len(parts)==2 and parts[1] in carriers:rule([parts[0]],src[toks[at][3]:toks[close][2]],False,unit[i],unit[i],parts[0] in top_names)
        head=functions.get(callee)
        if head is None or (i>0 and toks[i-1][1] in (".","?.")) or any(a<=i<b for a,b in ranges):continue
        for index,(s,e) in enumerate(segments(toks,match,at+1,close)):
            if index<len(head["params"]):rule(head["params"][index]["names"],src[toks[s][2]:toks[e-1][3]],False,unit[i],unit[head["at"]],False)
    def tainted(name,at_unit):return name in shared or name in local.get(at_unit,set())
    def bears(text,at_unit):return any(re.search(p,text) for p in mounts) or any(tainted(w,at_unit) for w in references(text))
    changed=True
    while changed:
        changed=False
        for r in rules:
            target=shared if r["top"] else local.setdefault(r["write"],set())
            if any(n not in target for n in r["names"]) and (any(re.search(p,r["text"]) for p in types) if r["type"] else bears(r["text"],r["read"])):target.update(r["names"]);changed=True
    found=[]
    for i,callee,close,at in calls:
        if not any(re.search(p,callee) for p in openers):continue
        args=src[toks[at][3]:toks[close][2]]
        if not bears(args,unit[i]):continue
        if any(re.search(p,args) for p in carried):continue
        if any(a<=i<b for a,b in ranges):continue
        found.append({"path":path,"line":toks[i][4],"opener":callee,"text":callee+"("+re.sub(r"\s+","",args)+")"})
    return found
def scan(repository,policy):
    skip=set(policy["skipDirectories"]);found=[]
    def walk(directory,relative):
        for name in os.listdir(directory):
            path=os.path.join(directory,name);child=name if relative=="" else relative+"/"+name
            if os.path.isdir(path):
                if name not in skip:walk(path,child)
            elif os.path.isfile(path) and any(name.endswith(e) for e in policy["extensions"]) and not any(name.endswith(s) for s in policy["skipSuffixes"]):found.append(child)
    for root in policy["roots"]:walk(os.path.join(repository,*root.split("/")),root)
    return sorted(found)
def named(source,policy):return any(re.search(p,source) for p in policy["mountPatterns"]+policy["rootPatterns"]+policy["rootTypes"])
def judge(sources,policy):
    entries=policy["entries"];used=set();violations=[];hits=[]
    for path,source in sources.items():
        if not named(source,policy):continue
        hits.append(path)
        for site in sites(path,source,policy):
            matching=[at for at,row in enumerate(entries) if row["path"]==path and row["site"] in site["text"]]
            used.update(matching)
            if not matching:violations.append(site)
    violations.sort(key=lambda s:(s["path"],s["line"]))
    return {"named":sorted(hits),"unrouted":sorted({s["path"] for s in violations}),"sites":[s["path"]+":"+str(s["line"])+":"+s["opener"] for s in violations],"stale":sorted(row["path"]+"::"+row["site"] for at,row in enumerate(entries) if at not in used)}
if sys.argv[1]=="repository":
    policy=json.load(open(sys.argv[3],encoding="utf8"));repository=sys.argv[2]
    sources={p:open(os.path.join(repository,*p.split("/")),encoding="utf8").read() for p in scan(repository,policy)}
    print(json.dumps(judge(sources,policy),ensure_ascii=False))
elif sys.argv[1]=="cases":
    corpus=json.load(open(sys.argv[2],encoding="utf8"));policy=corpus["policy"]
    print(json.dumps([judge({"x.ts":row["source"]},policy) for row in corpus["cases"]],ensure_ascii=False))
