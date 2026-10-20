#!/usr/bin/env python3
"""🐍️ Independent oracle of the TypeScript unit planner: reads the language-agnostic fixture on stdin, writes the plan of every case as JSON on stdout.
Written from the schema text only (seeds, refinement, collapse, coarsening); shares no code with the TypeScript engine."""
import json,re,sys,hashlib

def glob_re(g):
    out='';i=0
    while i<len(g):
        if g.startswith('**/',i): out+='(?:.*/)?';i+=3
        elif g.startswith('**',i): out+='.*';i+=2
        elif g[i]=='*': out+='[^/]*';i+=1
        else: out+=re.escape(g[i]);i+=1
    return re.compile('^'+out+'$')

def parent(p): return p.rsplit('/',1)[0] if '/' in p else ''

def scc(nodes,adj):
    index={};low={};on=set();stack=[];res=[];n=[0]
    def visit(v):
        index[v]=low[v]=n[0];n[0]+=1;stack.append(v);on.add(v)
        for w in sorted(adj.get(v,())):
            if w not in index: visit(w);low[v]=min(low[v],low[w])
            elif w in on: low[v]=min(low[v],index[w])
        if low[v]==index[v]:
            c=[]
            while True:
                w=stack.pop();on.discard(w);c.append(w)
                if w==v:break
            res.append(sorted(c))
    sys.setrecursionlimit(100000)
    for v in nodes:
        if v not in index: visit(v)
    return res

def collide(files):
    seen=set()
    for f in files:
        if re.search(r'\.tsx?$',f) and not f.endswith('.d.ts'):
            st=re.sub(r'\.tsx?$','',f)
            if st in seen: return True
            seen.add(st)
    return False

def plan(doc,files,manifests,edges,jsons,family):
    seeds=doc['seeds']
    amb=[glob_re(g) for g in seeds['ambient']]
    ambient=sorted(f for f in files if any(m.match(f) for m in amb))
    aset=set(ambient)
    files=sorted(f for f in files if f not in aset)
    fset=set(files)
    markers=set(seeds['markers']);globs=[g.split('/') for g in seeds['roots']]
    roots={''}
    if seeds['manifests']:
        for d in manifests:
            if not any(s in markers for s in d.split('/')): roots.add(d)
    for f in files:
        segs=f.split('/')[:-1]
        for i in range(1,len(segs)+1):
            pre=segs[:i]
            if pre[-1] in markers: roots.add('/'.join(pre[:-1]))
            for g in globs:
                if len(g)==len(pre) and all(a=='*' or a==b for a,b in zip(g,pre)): roots.add('/'.join(pre))
    def root_of(f):
        d=parent(f)
        while d and d not in roots: d=parent(d)
        return d
    iso=set(seeds['isolate'])
    spec={}  # id -> (kind,prefix,isolate)
    def uid(kind,prefix,isolate=None):
        i=kind+':'+prefix+('#'+isolate if isolate else '')
        spec[i]=(kind,prefix,isolate);return i
    assign={}
    for f in files:
        r=root_of(f);rest=(f[len(r)+1:] if r else f).split('/')[:-1]
        name=next((s for s in rest if s in iso),None)
        assign[f]=uid('dir',r,name)
    stems={}
    for f in files:
        if re.search(r'\.tsx?$',f) and not f.endswith('.d.ts'): stems.setdefault((assign[f],re.sub(r'\.tsx?$','',f)),[]).append(f)
    for g in stems.values():
        if len(g)>1:
            for f in g:
                if f.endswith('.tsx'): assign[f]=uid('file',f)
    E=[(a,b) for a,b in edges if a in fset and b in fset and a!=b]
    frozen=set()
    def quotient():
        mem={}
        for f in files: mem.setdefault(assign[f],[]).append(f)
        adj={u:set() for u in mem}
        for a,b in E:
            x,y=assign[a],assign[b]
            if x!=y: adj[x].add(y)
        return mem,adj
    rnd=0
    while True:
        rnd+=1
        mem,adj=quotient()
        cyc=[c for c in scc(sorted(mem),adj) if len(c)>1]
        if not cyc: break
        for c in cyc:
            cf=[f for u in c for f in mem[u]]
            clash=collide(cf)
            def splittable(u):
                k=spec[u][0]
                return len(mem[u])>1 and k not in('file','group') and (clash or (k!='self' and u not in frozen))
            divisible=any(splittable(u) for u in c)
            if (not divisible) if clash else (len(cf)<=doc['cycles']['collapseFiles'] or rnd>doc['cycles']['refineRounds'] or not divisible):
                gid=uid('group',re.sub(r'^[a-z]+:','',sorted(c)[0]))
                for u in c:
                    for f in mem[u]: assign[f]=gid
                frozen.add(gid);continue
            for u in c:
                k,p,iso_=spec[u];lst=mem[u]
                if not splittable(u): frozen.add(u);continue
                if k=='self':
                    for f in lst: assign[f]=uid('file',f)
                    continue
                for f in lst:
                    rest=(f[len(p)+1:] if p else f).split('/')
                    assign[f]=uid('self',p,iso_) if len(rest)==1 else uid('dir',(p+'/'+rest[0]) if p else rest[0],iso_)
    # coarsening
    m=doc['coarsen']['minimumFiles']
    if m>=2:
        mem,adj=quotient()
        out={u:set(v) for u,v in adj.items()};into={u:set() for u in mem}
        for u,vs in out.items():
            for v in vs: into[v].add(u)
        members={u:list(l) for u,l in mem.items()}
        def reach(starts,g):
            seen=set(starts);st=list(starts)
            while st:
                for n in g.get(st.pop(),()):
                    if n not in seen: seen.add(n);st.append(n)
            return seen
        def key(u):
            k,p,iso_=spec[u];return (parent(p) if '/' in p else '')+'|'+(iso_ or '')+'|'+(u if k=='group' else '')
        buckets={}
        for u,l in members.items():
            if len(l)<m and spec[u][0]!='group': buckets.setdefault(key(u),[]).append(u)
        def merge(cl):
            gid=uid('group','cluster/'+re.sub(r'^[a-z]+:','',sorted(cl)[0]));s=set(cl);no=set();ni=set()
            for u in cl:
                for f in members[u]: assign[f]=gid
                no|=out[u];ni|=into[u]
            members[gid]=[f for u in cl for f in members[u]]
            for u in cl: del members[u];del out[u];del into[u]
            no-=s;ni-=s;out[gid]=no;into[gid]=ni
            for n in no: into[n]=(into[n]-s)|{gid}
            for p in ni: out[p]=(out[p]-s)|{gid}
        for k in sorted(buckets):
            cl=[];size=0
            for u in sorted(buckets[k]):
                if u not in members: continue
                trial=set(cl)|{u};down=reach(trial,out)-trial;up=reach(trial,into)-trial
                if down&up: continue
                seen=set();clash=False
                for x in trial:
                    for f in members[x]:
                        if re.search(r'\.tsx?$',f) and not f.endswith('.d.ts'):
                            st=re.sub(r'\.tsx?$','',f)
                            if st in seen: clash=True
                            seen.add(st)
                if clash: continue
                cl=cl+[u];size+=len(members[u])
                if size>=m:
                    if len(cl)>1: merge(cl)
                    cl=[];size=0
            if len(cl)>1: merge(cl)
    mem,adj=quotient()
    ids=sorted(mem)
    jb={}
    for a,b in jsons:
        if a in assign: jb.setdefault(assign[a],set()).add(b)
    units=[{'id':i,'label':spec[i][1] or '.','files':sorted(mem[i]),'json':sorted(jb.get(i,())),'dependencies':sorted(adj[i])} for i in ids]
    rem={i:len(adj[i]) for i in ids};dep={}
    for i in ids:
        for d in adj[i]: dep.setdefault(d,[]).append(i)
    ready=sorted(i for i in ids if rem[i]==0);order=[]
    while ready:
        ready.sort();i=ready.pop(0);order.append(i)
        for x in dep.get(i,[]):
            rem[x]-=1
            if rem[x]==0: ready.append(x)
    assert len(order)==len(ids),'cyclic'
    return {'family':family,'ambient':ambient,'order':order,'units':units}

if __name__=='__main__':
    fx=json.load(sys.stdin);out={}
    for c in fx['cases']:
        doc=json.loads(json.dumps(fx['document']))
        for k,v in c.get('override',{}).items(): doc[k].update(v)
        out[c['name']]=plan(doc,c['files'],c['manifests'],c['edges'],c['json'],'tsconfig.json')
    json.dump(out,sys.stdout,ensure_ascii=False)
