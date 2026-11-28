import {afterAll,expect,test} from "bun:test";
import {appendFileSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,sep} from "node:path";
import {GRAPH_READ_CODE_SCOPE_V1,graphReadCodeIdentityV1,graphReadCodeWatchV1} from "../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(root:string,name:string,text:string):string=>{const path=join(root,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);return path;};
const scope={entries:["graph/host.ts"],excluded:["resident/","../caching/"]};
const fixture=()=>{
 const base=mkdtempSync(join(tmpdir(),"read-identity-"));scratch.push(base);const library=join(base,"library","build");
 write(library,"graph/host.ts",'import {a} from "./a.ts";\nimport {service} from "../resident/service.ts";\nexport const run=async()=>(await import("../caching/store.ts")).store+a+service;\n');
 write(library,"graph/a.ts",'import data from "./data.json" with {type:"json"};\nexport const a=data;\n');write(library,"graph/data.json",'{"v":1}');
 write(library,"resident/service.ts",'export const service=1;');write(base+"/library","caching/store.ts",'export const store=2;');write(library,"unrelated.ts","export {};");
 return{base,library};
};
const python=async(program:string,...args:string[]):Promise<string>=>{const child=Bun.spawn(["python3","-I","-c",program,...args],{stdout:"pipe",stderr:"pipe"}),[out,code]=await Promise.all([new Response(child.stdout).text(),child.exited]);expect(code).toBe(0);return out.trim();};
const ORACLE=`import hashlib,os,re,sys
lib,entry,*excluded=sys.argv[1:]
excluded=[os.path.normpath(os.path.join(lib,e))+os.sep for e in excluded]
seen={};stack=[os.path.join(lib,entry)]
while stack:
    p=stack.pop()
    if p in seen or any(p.startswith(e) for e in excluded) or not os.path.isfile(p): continue
    data=open(p,'rb').read();seen[p]=hashlib.sha256(data).hexdigest()
    if p.endswith('.ts'):
        for m in re.findall(r'(?:from|import)\\s*\\(?\\s*["\\'](\\.{1,2}/[^"\\']+)["\\']',data.decode()):
            stack.append(os.path.normpath(os.path.join(os.path.dirname(p),m)))
root=lib+os.sep
print(hashlib.sha256("\\n".join(sorted(p[len(root):]+" "+h for p,h in seen.items())).encode()).hexdigest())`;

test("the code identity of a read moves only with a file of its own closure: an edit in an excluded service folder, in a folder the read imports dynamically but excludes, or in an unrelated file hands nothing over; an edit inside the closure does",async()=>{
 const{library}=fixture(),identity=await graphReadCodeIdentityV1(join(library,".."),{library,scope,environment:{}});
 expect(identity.digest).toMatch(/^[0-9a-f]{64}$/u);expect(await identity.current()).toBe(identity.digest);
 appendFileSync(join(library,"resident/service.ts"),"// edit");expect(await identity.current(),"excluded service folder").toBe(identity.digest);
 appendFileSync(join(library,"../caching/store.ts"),"// edit");expect(await identity.current(),"excluded sibling folder reached by a dynamic import").toBe(identity.digest);
 appendFileSync(join(library,"unrelated.ts"),"// edit");expect(await identity.current(),"a file nobody imports").toBe(identity.digest);
 appendFileSync(join(library,"graph/data.json")," ");const moved=await identity.current();
 expect(moved,"a json file of the closure").not.toBe(identity.digest);expect(moved).toMatch(/^[0-9a-f]{64}$/u);
});

test("a changed file that does not parse blocks the identity (nothing is handed over to code that cannot load), a parsed one is answered",async()=>{
 const{library}=fixture(),broken=await graphReadCodeIdentityV1(join(library,".."),{library,scope,environment:{},syntaxRows:async()=>[{path:"graph/a.ts",line:1,column:1,message:"bad"}]});
 appendFileSync(join(library,"graph/a.ts"),"// edit");expect(await broken.current()).toBeUndefined();
});

test("the digest equals an independent CPython closure digest of the same fixture, with and without the excluded folders",async()=>{
 const{library}=fixture();
 const mine=graphReadCodeWatchV1({library,scope}).closure().digest,expected=await python(ORACLE,library,"graph/host.ts",...scope.excluded);
 expect(mine).toBe(expected);
 const wide=graphReadCodeWatchV1({library,scope:{entries:scope.entries,excluded:[]}}).closure().digest,wideExpected=await python(ORACLE,library,"graph/host.ts");
 expect(wide).toBe(wideExpected);expect(wide).not.toBe(mine);
});

test("the declared scope of the real tree: the closure of the read holds graph code and no file of the services around it (caching, process runtime, resident host, remote execution, memory, host capacity)",()=>{
 const files=[...graphReadCodeWatchV1().closure().files.keys()].map(path=>path.split(sep).join("/"));
 expect(files.length).toBeGreaterThan(100);expect(files.some(path=>path.includes("/🕸️graph/🕰️generations/"))).toBe(true);
 for(const folder of ["/📚️library/⚡️caching/","/🧰️framework/🔨️modules/🏃️process/","/🏡️resident/","/🌐️remote-execution/","/🧠️memory/","/🌡️host-capacity/","/🧭️invocation/"])expect(files.filter(path=>path.includes(folder)),folder).toEqual([]);
 expect(GRAPH_READ_CODE_SCOPE_V1.entries.length).toBe(3);
 for(const landed of ["⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts","🏗️build/🧭️invocation/🔄️reload/🟦️.ts","🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts"])expect(files.some(path=>path.endsWith(landed)),landed).toBe(false);
});
