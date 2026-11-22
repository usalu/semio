import {expect,test} from "bun:test";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {repositoryStrictStorageV1} from "../../🔒️enforcement/🟦️.ts";
import budgetPolicy from "../../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};
import commandRunner from "../../../../../../../../⚙️configuration/🏃️process/🔣️.json" with {type:"json"};
import {cacheAreaBudgetBytesV1,cacheAreaDirectoryV1,cacheLedgerV1,cacheTextV1,classifyCachePathV1,formatCacheBytesV1,formatCacheReportV1,planCacheCollectionV1,readCacheLedgerV1,requireDeclaredCacheSegmentsV1,type CacheLanguageV1,type CacheReportV1,type CacheTargetV1} from "../🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";

const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value));
const ledger=readCacheLedgerV1(policy);

test("the authored ledger is closed, disjoint and within every budget, and Ajv agrees with the schema validator",()=>{
 expect(cacheLedgerV1()).toEqual(ledger);
 const ajv=new Ajv({strict:true,allowUnionTypes:true}),validate=ajv.compile(schema);
 expect(validate(policy)).toBe(true);
 expect(validateJsonSchemaSubset(schema,policy)).toEqual([]);
 const probes:Array<(document:any)=>void>=[
  document=>{delete document.roots;},
  document=>{document.version=2;},
  document=>{document.areas[0].extra=1;},
  document=>{document.areas[0].kind="folder";},
  document=>{document.areas[0].id="Bad Id";},
  document=>{document.areas[0].writers=[];},
  document=>{document.areas.find((area:any)=>area.id==="go").collect.quietMs=-1;},
  document=>{document.areas.find((area:any)=>area.id==="go").budget={bytes:1.5};},
  document=>{document.areas.find((area:any)=>area.id==="go").budget={bytes:1,basisPoints:1};},
  document=>{document.areas.find((area:any)=>area.id==="go").directory="a\\b";},
  document=>{document.areas.find((area:any)=>area.id==="go").purpose={en:"x"};},
  document=>{document.retirements[0].reason={en:"x",de:""};},
  document=>{document.roots.repository.ignored=[".DS_Store",".DS_Store"];}
 ];
 for(const probe of probes){const document=clone(policy);probe(document);expect(validate(document)).toBe(false);expect(validateJsonSchemaSubset(schema,document).length>0).toBe(true);}
});

test("the semantic closure refuses what the schema cannot see",()=>{
 const refused:Array<[string,(document:any)=>void]>=[
  ["duplicate id",document=>{document.areas[1].id=document.areas[0].id;}],
  ["overlapping retirement",document=>{document.retirements.push({root:"repository",directory:"go/00",quietMs:0,reason:{en:"x",de:"y"}});}],
  ["pattern matching a declared home",document=>{document.areas.find((area:any)=>area.id==="query-index").pattern="^go$";}],
  ["nested budgets beyond the partition",document=>{document.areas.find((area:any)=>area.id==="nx-artifacts").budget={bytes:2**40};}],
  ["root budget exceeded",document=>{document.areas.find((area:any)=>area.id==="go").budget={bytes:30*(1<<30)};}],
  ["partition directory differs from the authored budget",document=>{document.areas.find((area:any)=>area.id==="tools").directory="b/x";}],
  ["tree with both collect and managed",document=>{document.areas.find((area:any)=>area.id==="go").managed={owner:"nx"};}],
  ["tree without a budget",document=>{delete document.areas.find((area:any)=>area.id==="go").budget;}],
  ["lease custody without a resource",document=>{document.areas.find((area:any)=>area.id==="go").collect.custody="lease";}],
  ["resource without lease custody",document=>{document.areas.find((area:any)=>area.id==="go").collect.lease="r";}],
  ["parent segment",document=>{document.areas.find((area:any)=>area.id==="go").directory="a/../b";}],
  ["nested area in an unknown partition",document=>{document.areas.find((area:any)=>area.id==="nx-artifacts").partition="nowhere";}],
  ["two remainders",document=>{const second=clone(document.areas.find((area:any)=>area.id==="cargo-store"));second.id="other";document.areas.push(second);}],
  ["nested overlap",document=>{const second=clone(document.areas.find((area:any)=>area.id==="nx-artifacts"));second.id="other";second.directory="nx/artifacts/deep";document.areas.push(second);}],
  ["share budget on a tree",document=>{document.areas.find((area:any)=>area.id==="go").budget={basisPoints:10};}],
  ["directory and pattern together",document=>{document.areas.find((area:any)=>area.id==="query-index").directory="q";}],
  ["nested area with both a manager and a collection",document=>{document.areas.find((area:any)=>area.id==="compile-cache-rust").managed={owner:"nx"};}],
  ["nested area with neither a manager nor a collection",document=>{delete document.areas.find((area:any)=>area.id==="compile-cache-rust").collect;}],
  ["nested collection under a lease",document=>{const area=document.areas.find((area:any)=>area.id==="compile-cache-rust");area.collect.custody="lease";area.collect.lease="r";}],
  ["case-folded overlap",document=>{document.retirements.push({root:"repository",directory:"GO",quietMs:0,reason:{en:"x",de:"y"}});}]
 ];
 for(const[name,mutate]of refused){const document=clone(policy);mutate(document);expect(()=>readCacheLedgerV1(document),name).toThrow("Closed cache ledger required");}
});

test("budgets resolve exactly: explicit bytes, a share of the partition, and the remainder with its siblings",()=>{
 const bytes=(id:string)=>cacheAreaBudgetBytesV1(ledger,ledger.areas.find(area=>area.id===id)!);
 expect(bytes("nx-artifacts")).toBe(3*1024**3);
 expect(bytes("bazel-actions")).toBe(Math.floor(budgetPolicy.partitions.products.maximumLogicalBytes*4000/10000));
 expect(bytes("fleet-cache-index")).toBe(Math.floor(budgetPolicy.partitions.products.maximumLogicalBytes*1000/10000));
 expect(bytes("cargo-home")).toBe(3*1024**3);expect(bytes("unit-cache")).toBe(24*1024**3);expect(bytes("cargo-store")).toBe(budgetPolicy.partitions.rust.maximumLogicalBytes-31*1024**3);
 expect(bytes("rust")).toBe(Math.min(budgetPolicy.partitions.rust.capacityBytes,budgetPolicy.partitions.rust.maximumLogicalBytes));
 expect(bytes("control")).toBeNull();
 const loose=ledger.areas.filter(area=>area.root==="repository"&&area.kind==="tree").reduce((sum,area)=>sum+(bytes(area.id)??0),0);
 expect(loose).toBeLessThanOrEqual(ledger.roots.repository.maximumBytes);
 expect(ledger.areas.filter(area=>area.kind==="partition").map(area=>area.partition).sort()).toEqual(Object.keys(budgetPolicy.partitions).sort());
 expect(cacheAreaDirectoryV1("/w",ledger,ledger.areas.find(area=>area.id==="nx-artifacts")!)).toBe("/w/"+budgetPolicy.partitions.rust.directory+"/m/nx/artifacts");
 expect(cacheAreaDirectoryV1("/w",ledger,ledger.areas.find(area=>area.id==="cargo-store")!)).toBe("/w/"+budgetPolicy.partitions.rust.directory+"/m");
 expect(cacheAreaDirectoryV1("/w",ledger,ledger.areas.find(area=>area.id==="query-index")!)).toBeNull();
 expect(()=>cacheAreaDirectoryV1("relative",ledger,ledger.areas[0]!)).toThrow("Canonical workspace required");
});

test("the plans of the hand-written cases are exactly what the rules say",()=>{
 for(const row of fixtures.plans){
  const[plan]=planCacheCollectionV1([row.target as CacheTargetV1],fixtures.now);
  expect(plan!.removals.map(removal=>[removal.name,removal.reason]),row.name).toEqual(row.removals);
  expect(plan!.kept.map(kept=>[kept.name,kept.reason]),row.name).toEqual(row.kept);
  expect(plan!.retainedBytes,row.name).toBe(row.retained);
  expect(plan!.overBudgetBytes,row.name).toBe(row.over);
  expect(plan!.removals.reduce((sum,removal)=>sum+removal.bytes,0)+plan!.retainedBytes,row.name).toBe(plan!.totalBytes);
 }
});

test("planning is pure and never removes a held or quiet unit",()=>{
 const target:CacheTargetV1={id:"p",kind:"retired",budgetBytes:0,unusedAgeMs:0,quietMs:60000,units:[{name:"a",bytes:5,recencyMs:1,held:"pid:1"},{name:"b",bytes:5,recencyMs:fixtures.now-1},{name:"c",bytes:5,recencyMs:1}]},frozen=JSON.stringify(target),first=planCacheCollectionV1([target],fixtures.now),second=planCacheCollectionV1([target],fixtures.now);
 expect(JSON.stringify(target)).toBe(frozen);
 expect(first).toEqual(second);
 expect(first[0]!.removals.map(row=>row.name)).toEqual(["c"]);
 expect(first[0]!.kept.map(row=>[row.name,row.reason])).toEqual([["a","held"],["b","quiet"]]);
});

test("every path is classified as an area, a container, a retired root, an ignored file or nobody's",()=>{
 for(const row of fixtures.classification){
  const actual=classifyCachePathV1(ledger,row.root as Parameters<typeof classifyCachePathV1>[1],row.path);
  expect(actual.kind,row.root+":"+row.path).toBe(row.kind as typeof actual.kind);
  if("id" in row)expect(actual.id,row.root+":"+row.path).toBe(row.id);
 }
});

test("the runtime door accepts only declared homes and says so in both languages",()=>{
 for(const parts of fixtures.segments.accepted)expect(()=>requireDeclaredCacheSegmentsV1(parts,ledger),JSON.stringify(parts)).not.toThrow();
 for(const parts of fixtures.segments.refused)expect(()=>requireDeclaredCacheSegmentsV1(parts,ledger),JSON.stringify(parts)).toThrow("Declared cache area required");
 try{requireDeclaredCacheSegmentsV1(["e1"],ledger);}catch(error){expect((error as Error).message).toBe(cacheTextV1("en","error.undeclared",{segment:"e1"})+"\n"+cacheTextV1("de","error.undeclared",{segment:"e1"}));}
});

test("every message exists in English and German with the same placeholders, and English comes first",()=>{
 expect(texts.languages).toEqual(["en","de"]);
 const names=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const[key,row]of Object.entries(texts.messages)){expect(names(row.en),key).toEqual(names(row.de));expect(typeof row.en).toBe("string");expect(typeof row.de).toBe("string");}
 for(const key of Object.keys(texts.messages).filter(key=>!key.startsWith("gc.mode.")))expect((texts.messages as any)[key].en.length>0&&(texts.messages as any)[key].de.length>0,key).toBe(true);
 expect(()=>cacheTextV1("en","nothing.here")).toThrow("Declared cache message required");
 expect(()=>cacheTextV1("en","gc.scan",{id:"x"})).toThrow("Cache message variable missing");
 const used=new Set<string>();for(const area of ledger.areas){expect(area.purpose.en.length>0&&area.purpose.de.length>0).toBe(true);used.add(area.id);}
 expect(used.size).toBe(ledger.areas.length);
});

test("the language is the flag, then the environment, and nothing is assumed",()=>{
 const pick=(args:string[],environment:Record<string,string>)=>resolveLanguageV1(CATALOG_LANGUAGES_V1,args,environment,()=>[]).language;
 expect(pick(["--lang","de"],{LANG:"en_US.UTF-8"})).toBe("de");
 expect(pick([],{SEMIO_LANGUAGE:"de",LANG:"en_US.UTF-8"})).toBe("de");
 expect(pick([],{LC_ALL:"de_DE.UTF-8"})).toBe("de");
 expect(pick([],{LANG:"en_GB.UTF-8"})).toBe("en");
 expect(()=>pick([],{LANG:"fr_FR.UTF-8"})).toThrow("--lang=en|de");
 expect(()=>pick([],{})).toThrow("SEMIO_LANGUAGE=en|de");
 expect(()=>pick(["--lang","fr"],{})).toThrow("Unknown language: fr (en|de)");
});

test("the status report renders the same in both languages from the same data",()=>{
 const report=fixtures.report as unknown as CacheReportV1;
 for(const language of["en","de"] as CacheLanguageV1[])expect(formatCacheReportV1(language,report,ledger),language).toEqual(fixtures.reportLines[language]);
 expect(formatCacheBytesV1(0)).toBe("0 B");
 expect(formatCacheBytesV1(1023)).toBe("1023 B");
 expect(formatCacheBytesV1(1024)).toBe("1.00 KiB");
 expect(formatCacheBytesV1(1536*1024)).toBe("1.50 MiB");
 expect(formatCacheBytesV1(1024**4*3)).toBe("3.00 TiB");
 const clean:CacheReportV1={...report,retired:[],undeclared:[]};
 expect(formatCacheReportV1("en",clean,ledger).at(-1)).toBe("Nothing retired or undeclared remains.");
 expect(formatCacheReportV1("de",clean,ledger).at(-1)).toBe("Es bleibt nichts Stillgelegtes oder Nicht-Deklariertes übrig.");
});

test("Law: the runtime state store of the command runner is a declared area of the storage ledger with a budget and a collector, never an undeclared entry that the disk budget does not see",()=>{
 const store=commandRunner.store.split("/").filter(part=>part!==".."),root=store[0]!,name=store.slice(1).join("/");
 expect(root,"the store lives directly in the native cache root").toBe(".cache");
 const area=policy.areas.find(row=>row.root==="native"&&row.kind==="tree"&&row.directory===name);
 expect(area,"an area for "+commandRunner.store).toBeDefined();
 expect(area!.budget!.bytes).toBeGreaterThan(0);expect(area!.collect!.custody).toBe("quiet");expect(area!.collect!.unusedAgeMs).toBeGreaterThan(0);
 expect(area!.purpose.en.length>0&&area!.purpose.de.length>0).toBe(true);
 expect(classifyCachePathV1(ledger,"native",name+"/capture/x.log").kind,"classified as its area, not as undeclared").not.toBe("undeclared");
});


test("the compact Rust image has an explicit disjoint ledger root",()=>{
 const fixture=fixtures.compactRust,area=ledger.areas.find(row=>row.id==="rust")!;
 expect(fixture.root).toBe(area.root);expect(area.directory).toBe(fixture.partitionDirectory);
 expect(ledger.roots[area.root].directory).toBe(fixture.directory);
 expect(cacheAreaDirectoryV1("/work/semio",ledger,area)).toBe(fixture.absoluteDirectory);
 const strict=repositoryStrictStorageV1("/work/semio");expect(strict.directory).toBe(fixture.absoluteDirectory);expect((strict as {partition?:string}).partition).toBe(area.id);
 expect(ledger.areas.some(row=>row.id==="held-rust-image"),"the held rust image is retired: the rust partition is declared at "+area.directory).toBe(false);
 for(const row of fixture.cases)expect({kind:row.kind,...row.id?{id:row.id}:{}}).toEqual(classifyCachePathV1(ledger,area.root,row.path));
});

/** 📌️ A pinned tool install is provisioned only by a tool setup and recreated by nothing at launch, so host disk pressure never selects it, whatever its age, quiet time or the reclaim asked for; an independent Python model plans the same reclaim over the same targets, and the authored ledger pins the Playwright browsers. */
test("pressure collection never selects a pinned tool area, and agrees with an independent Python model",()=>{
 const now=fixtures.now,hour=3600000,mk=(id:string,units:CacheTargetV1["units"],pinned?:true):CacheTargetV1=>({id,kind:"area",budgetBytes:1e12,unusedAgeMs:null,quietMs:hour,...pinned?{pinned}:{},units});
 const targets:CacheTargetV1[]=[mk("playwright",[{name:"chromium-1",bytes:900,recencyMs:now-400*hour},{name:"ffmpeg-2",bytes:50,recencyMs:now-300*hour}],true),mk("go",[{name:"a",bytes:100,recencyMs:now-10*hour},{name:"b",bytes:200,recencyMs:now-20*hour}])];
 const model=`import json,sys
targets,now,reclaim=json.loads(sys.argv[1]),int(sys.argv[2]),int(sys.argv[3])
cand=sorted((u["recencyMs"],t["id"],u["name"],u["bytes"]) for t in targets if t["kind"]=="area" and not t.get("pinned") for u in t["units"] if now-u["recencyMs"]>=t["quietMs"])
left=reclaim;out=[]
for r,t,n,b in cand:
    if left<=0: break
    out.append(t+"/"+n);left-=b
print(json.dumps(out))`;
 for(const reclaim of[0,1,200,10**6]){
  const run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(targets),String(now),String(reclaim)]);expect(run.exitCode,run.stderr.toString()).toBe(0);
  const plans=planCacheCollectionV1(targets,now,reclaim),pressured=plans.flatMap(plan=>plan.removals.filter(row=>row.reason==="pressure").map(row=>plan.id+"/"+row.name));
  expect(pressured.sort(),String(reclaim)).toEqual((JSON.parse(run.stdout.toString()) as string[]).sort());
  expect(plans.find(plan=>plan.id==="playwright")!.removals,String(reclaim)).toEqual([]);
 }
 const pinned=ledger.areas.filter(area=>area.collect?.pressure==="pinned");
 expect(pinned.map(area=>area.directory)).toContain("tools/ms-playwright");
 const authored:CacheTargetV1[]=pinned.map(area=>({id:area.id,kind:"area",budgetBytes:null,unusedAgeMs:null,quietMs:area.collect!.quietMs,pinned:true,units:[{name:"chromium-1",bytes:1e9,recencyMs:now-1000*hour}]}));
 for(const plan of planCacheCollectionV1(authored,now,10**12))expect(plan.removals,plan.id).toEqual([]);
});

/** 🗜️ Under host disk pressure the stale or superseded units of declared areas are eligible however young their age and however far below budget the area is: oldest first across all areas, never a held or quiet unit, exactly as an independent Python model plans it. */
test("pressure collection frees stale entries of build-owned areas oldest first and never a held or quiet unit, and agrees with an independent Python model",()=>{
 const now=fixtures.now,hour=3600000,mk=(id:string,units:CacheTargetV1["units"],quietMs=hour):CacheTargetV1=>({id,kind:"area",budgetBytes:1e12,unusedAgeMs:30*24*hour,quietMs,units});
 const targets:CacheTargetV1[]=[mk("dependencies",[{name:"g1",bytes:400,recencyMs:now-50*hour},{name:"g2",bytes:300,recencyMs:now-10*hour},{name:"g3",bytes:900,recencyMs:now-5*hour,held:"pid:1"},{name:"g4",bytes:200,recencyMs:now-30*60000}]),mk("tools",[{name:"t1",bytes:500,recencyMs:now-20*hour},{name:"t2",bytes:100,recencyMs:now-20*60000}]),{id:"repository:undeclared",kind:"undeclared",budgetBytes:null,unusedAgeMs:null,quietMs:hour,units:[{name:"u",bytes:50,recencyMs:now-90*hour}]}];
 const frozen=JSON.stringify(targets);
 const model=`import json,sys
targets,now,reclaim=json.loads(sys.argv[1]),int(sys.argv[2]),int(sys.argv[3])
cand=[]
for t in targets:
    if t["kind"]!="area" or t.get("heldBy"): continue
    for u in t["units"]:
        if u.get("held") is None and now-u["recencyMs"]>=t["quietMs"]: cand.append((u["recencyMs"],t["id"],u["name"],u["bytes"]))
cand.sort()
left=reclaim;out=[]
for r,t,n,b in cand:
    if left<=0: break
    out.append([t,n]);left-=b
print(json.dumps(out))`;
 for(const reclaim of[0,1,400,401,700,1500,10**6]){
  const run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(targets),String(now),String(reclaim)]);expect(run.exitCode,run.stderr.toString()).toBe(0);
  const expected=JSON.parse(run.stdout.toString()) as string[][],plans=planCacheCollectionV1(targets,now,reclaim),pressured=plans.flatMap(plan=>plan.removals.filter(row=>row.reason==="pressure").map(row=>[row.target,row.name]));
  expect(pressured.map(row=>row.join("/")).sort(),String(reclaim)).toEqual(expected.map(row=>row.join("/")).sort());
  const freed=plans.reduce((sum,plan)=>sum+plan.removals.reduce((inner,row)=>inner+row.bytes,0),0);
  if(reclaim>0&&reclaim<=1000)expect(freed,String(reclaim)).toBeGreaterThanOrEqual(reclaim);
  for(const plan of plans)for(const row of plan.removals)expect(["g3","g4","t2"].includes(row.name)&&row.reason==="pressure").toBe(false);
 }
 expect(JSON.stringify(targets)).toBe(frozen);expect(planCacheCollectionV1(targets,now,0)).toEqual(planCacheCollectionV1(targets,now));
 expect(planCacheCollectionV1(targets,now,10**6).find(plan=>plan.id==="dependencies")!.removals.map(row=>row.name).sort()).toEqual(["g1","g2"]);
});

test("a retired path inside a control area is carved out of it: accepted only strictly inside a control area of its root, classified retired, and the rest of the area stays its own",()=>{
 const carved=ledger.retirements.find(row=>row.within!==undefined&&row.directory==="f/cache-server")!;
 expect(carved.directory).toBe("f/cache-server");
 expect(classifyCachePathV1(ledger,"native","f/cache-server/v1/main/ab/cd").kind).toBe("retired");
 expect(classifyCachePathV1(ledger,"native","f/boot/beat.json")).toMatchObject({kind:"area",id:"fleet-state"});
 const without=clone(policy);without.retirements=without.retirements.map(row=>{const{within:_,...rest}=row as Record<string,unknown>;return rest;}) as never;
 expect(()=>readCacheLedgerV1(without),"without within it overlaps the control area").toThrow();
 const elsewhere=clone(policy);(elsewhere.retirements.find(row=>(row as {within?:string}).within!==undefined) as {within:string}).within="tools";
 expect(()=>readCacheLedgerV1(elsewhere),"within names a partition, not a control").toThrow();
 const wholeArea=clone(policy);(wholeArea.retirements.find(row=>(row as {within?:string}).within!==undefined) as {directory:string}).directory="f";
 expect(()=>readCacheLedgerV1(wholeArea),"the area itself cannot be retired").toThrow();
 const twice=clone(policy);twice.retirements.push(clone(twice.retirements.find(row=>(row as {within?:string}).within!==undefined)!) as never);
 expect(()=>readCacheLedgerV1(twice),"the same carved-out path twice").toThrow();
});
