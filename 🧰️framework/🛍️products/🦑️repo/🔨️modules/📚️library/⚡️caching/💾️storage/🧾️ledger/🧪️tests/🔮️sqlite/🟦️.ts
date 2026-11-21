import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {expect,test} from "bun:test";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {runCacheLedgerCommandV1} from "../../../../../🏗️build/🧹️cache-gc/⌨️command/🟦️.ts";
import {cacheRunnerPortsV1,collectCacheV1,loadCacheSizesV1,machineUsesV1,recordCacheSizesV1,reportCacheLedgerV1} from "../../../../../🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts";
import {readFleetStorageV1} from "../../../../../🏗️build/🚢️fleet-status/📥️readers/🟦️.ts";
import {collectCasV1} from "../../../../../🏗️build/🗄️cas/🧹️collector/🟦️.ts";
import {CACHE_SIZE_FILE_V1,cacheLedgerV1,cacheRootDirectoryV1,cacheTextV1,formatCacheReportV1,planCacheCollectionV1,requireDeclaredCacheSegmentsV1,type CacheTargetV1} from "../../🟦️.ts";
import {machinePolicyV1,planMachineBudgetV1} from "../../🎚️machine/🟦️.ts";
import fixtures from "../🧫️fixtures/🔣️.json" with {type:"json"};

const seeds=[1,2,3,5,8,13,21,34,55,89,144,233],casesPerSeed=20;

test("aged fleet state is measured and preserved while the generic aged neighbor is collected",async()=>{
 const fixture=fixtures.fleetState,ledger=cacheLedgerV1(),now=fixtures.now,workspace=realpathSync(mkdtempSync(join(testScratchRootV1("cache-ledger-fleet"),"fl-")));
 const root=cacheRootDirectoryV1(workspace,ledger,"native"),only=new Set([fixture.area,"undeclared"]);
 const ports=cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>{throw Error("Unexpected lease");},delegate:async()=>{throw Error("Unexpected owner collection");}});
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE homes(name TEXT PRIMARY KEY, age INTEGER)");
  const insert=database.query("INSERT INTO homes VALUES(?,?)");
  for(const home of fixture.homes){
   insert.run(home.name,home.ageMs);
   for(const file of home.files){
    const path=join(root,home.name,file),stamp=new Date(now-home.ageMs);
    mkdirSync(dirname(path),{recursive:true});writeFileSync(path,file);utimesSync(path,stamp,stamp);
   }
  }
  const ages=ledger.roots.native,expected=database.query("SELECT name FROM homes WHERE name != ?1 AND age >= MAX(?2,?3) ORDER BY name").values(fixture.home,ages.undeclaredUnusedAgeMs,ages.undeclaredQuietMs).flat();
  expect(expected).toEqual([fixture.agedNeighbor]);
  const result=await collectCacheV1(workspace,ledger,{only,dryRun:false},ports,{language:"en",write:()=>{}});
  expect(result.removed.map(row=>row.name)).toEqual(expected);
  expect(result.failed).toEqual([]);
  for(const home of fixture.homes)for(const file of home.files)expect(existsSync(join(root,home.name,file))).toBe(!expected.includes(home.name));
  expect(()=>requireDeclaredCacheSegmentsV1([fixture.home,"state.wal"],ledger,"native")).not.toThrow();
  const deep=await reportCacheLedgerV1(workspace,ledger,ports,undefined,only,{deep:true});
  const row=deep.partitions.find(item=>item.id===fixture.area)!;
  expect(row).toMatchObject({kind:"control",directory:join(root,fixture.home),state:"directory",owner:fixture.owner,source:"walked",budgetBytes:null,capacityBytes:null});
  expect(row.usedBytes!).toBeGreaterThan(0);
  expect(deep.undeclared.map(item=>item.path)).toEqual(["native:f-new"]);
  const quick=await reportCacheLedgerV1(workspace,ledger,ports,undefined,only);
  expect(quick.partitions.find(item=>item.id===fixture.area)).toMatchObject({source:"recorded",usedBytes:row.usedBytes,recordedAtMs:now});
  for(const language of ["en","de"] as const)expect(formatCacheReportV1(language,quick).find(line=>line.includes(fixture.area))).toContain(cacheTextV1(language,"note.managed",{owner:fixture.owner}));
 }finally{database.close();rmSync(workspace,{recursive:true,force:true});}
});

test("machine accounting charges fleet state once, reserves its share and excludes it from eviction",async()=>{
 const fixture=fixtures.fleetMachineUse,fleet=fixtures.fleetState,ledger=cacheLedgerV1(),now=fixtures.now;
 const workspace=realpathSync(mkdtempSync(join(testScratchRootV1("cache-ledger-fleet"),"fm-")));
 const root=cacheRootDirectoryV1(workspace,ledger,"native"),only=new Set([fleet.area,"control",fixture.collectionArea]);
 const ports=cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>{throw Error("Unexpected lease");},atimeReliable:async()=>true});
 const machine=machinePolicyV1(),policy={...machine,hostFreeFloorBytes:0,toleranceBytes:0,stores:[{...machine.stores.find(store=>store.loose)!,floorBytes:0,weightBasisPoints:10000}]};
 const database=new Database(":memory:");
 try{
  mkdirSync(join(root,fleet.home),{recursive:true});mkdirSync(join(root,"b","c"),{recursive:true});mkdirSync(join(root,fixture.collectionArea),{recursive:true});
  database.exec("CREATE TABLE usage(bytes INTEGER,included INTEGER,evictable INTEGER)");
  const insert=database.query("INSERT INTO usage VALUES(?,?,?)");
  insert.run(fixture.fleetBytes,1,0);insert.run(fixture.bootstrapBytes,0,0);
  let genericBytes=0;
  for(const unit of fixture.units){
   const file=join(root,fixture.collectionArea,unit),stamp=new Date(now-fixture.ageMs);
   writeFileSync(file,"payload");utimesSync(file,stamp,stamp);
   const measured=lstatSync(file),bytes=measured.blocks?512*measured.blocks:measured.size;
   insert.run(bytes,1,1);genericBytes+=bytes;
  }
  const expected=database.query("SELECT SUM(bytes*included) used, SUM(bytes*included*(1-evictable)) pinned, SUM(bytes*evictable) evictable FROM usage").get() as {used:number;pinned:number;evictable:number};
  const unmeasured=await reportCacheLedgerV1(workspace,ledger,ports,undefined,new Set([fleet.area]));
  expect(unmeasured.roots.find(row=>row.id==="native")).toMatchObject({usedBytes:0,partial:true});
  expect(await recordCacheSizesV1(workspace,ledger,{[fleet.area]:{usedBytes:fixture.fleetBytes,partial:false,atMs:now},control:{usedBytes:fixture.bootstrapBytes,partial:false,atMs:now},[fixture.collectionArea]:{usedBytes:genericBytes,partial:false,atMs:now}})).toBe(true);
  const report=await reportCacheLedgerV1(workspace,ledger,ports,undefined,only);
  expect(report.roots.find(row=>row.id==="native")).toMatchObject({usedBytes:expected.used,partial:false});
  expect(report.partitions.find(row=>row.id==="control")).toMatchObject({usedBytes:fixture.bootstrapBytes});
  const uses=machineUsesV1(report);
  expect(uses.find(row=>row.id==="loose")).toMatchObject({usedBytes:expected.used,evictableBytes:expected.evictable});
  const commandLines:string[]=[];
  expect(await runCacheLedgerCommandV1(["status","--json","--lang=en"],{workspace,ledger,environment:{},write:line=>commandLines.push(line)},ports)).toBe(0);
  expect(JSON.parse(commandLines.join("\n")).machine).toMatchObject({usedBytes:expected.used,footprintBytes:expected.used});
  const fleetFacts=await readFleetStorageV1({root:workspace,environment:{},language:"en",now:()=>now});
  expect(fleetFacts.storage?.machine).toMatchObject({usedBytes:expected.used,footprintBytes:expected.used});
  const limited=planMachineBudgetV1({totalBytes:expected.pinned-1,hostFreeBytes:null,uses},policy);
  expect(limited).toMatchObject({usedBytes:expected.used,footprintBytes:expected.used,evictBytes:expected.evictable,blockedBytes:1});
  const totalBytes=expected.pinned+expected.evictable/fixture.units.length;
  const result=await collectCacheV1(workspace,ledger,{only,dryRun:true,machine:{totalBytes,source:"argument",policy}},ports,{language:"en",write:()=>{}});
  expect(result.machine).toMatchObject({usedBytes:expected.used,footprintBytes:expected.used,evictBytes:expected.used-totalBytes,blockedBytes:0});
  expect(result.removed.map(row=>[row.target,row.name])).toEqual([[fixture.collectionArea,fixture.units[0]]]);
  expect(result.freedBytes).toBe(expected.used-totalBytes);
 }finally{database.close();rmSync(workspace,{recursive:true,force:true});}
});

for(const fixture of fixtures.coldFleetSizes)test("initial collection charges the same fleet walk to machine and host CAS with "+fixture.id,async()=>{
 const ledger=cacheLedgerV1(),fleet=fixtures.fleetState,now=fixtures.now,workspace=realpathSync(mkdtempSync(join(testScratchRootV1("cache-ledger-fleet"),"fc-")));
 const state=join(cacheRootDirectoryV1(workspace,ledger,"native"),fleet.home,"state.ckp"),sizes=join(cacheRootDirectoryV1(workspace,ledger,"repository"),CACHE_SIZE_FILE_V1),casRoot=join(workspace,"host-cas"),stages:string[]=[],lines:string[]=[];
 const ports=cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async stage=>{stages.push(stage);},exclusive:async()=>undefined,atimeReliable:async()=>true});
 try{
  mkdirSync(dirname(state),{recursive:true});writeFileSync(state,"replica state");
  if(fixture.obsoleteBytes!==null||fixture.partialBytes!==null){
   mkdirSync(dirname(sizes),{recursive:true});
   const records=fixture.partialBytes!==null?{[fleet.area]:{usedBytes:fixture.partialBytes,partial:true,atMs:now}}:{"undeclared:native:f":{usedBytes:fixture.obsoleteBytes,partial:false,atMs:now}};
   writeFileSync(sizes,JSON.stringify({version:1,records}));
  }
  const before=existsSync(sizes)?readFileSync(sizes,"utf8"):null,stamp=lstatSync(state),bytes=stamp.blocks?stamp.blocks*512:stamp.size;
  using database=new Database(":memory:");
  const expected=database.query("SELECT SUM(bytes) AS used FROM (SELECT ?1 AS key, ?2 AS bytes UNION ALL SELECT 'undeclared:native:f', ?3) WHERE key = ?1").get(fleet.area,bytes,fixture.obsoleteBytes) as {used:number};
  const status=await reportCacheLedgerV1(workspace,ledger,ports,undefined,new Set([fleet.area]));
  expect(status.partitions.find(row=>row.id===fleet.area)).toMatchObject({usedBytes:fixture.partialBytes,source:fixture.partialBytes===null?"unknown":"recorded"});
  expect(status.roots.find(row=>row.id==="native")).toMatchObject({usedBytes:fixture.partialBytes??0,partial:true});
  expect(stages).toEqual(["status:"+fleet.area]);stages.length=0;
  expect(await runCacheLedgerCommandV1(["gc","--dry-run","--json","--lang=en"],{workspace,ledger,environment:{HOME:workspace,SEMIO_CAS_ROOT:casRoot},write:line=>lines.push(line)},ports)).toBe(0);
  const json=JSON.parse(lines.join("\n"));
  expect(json.machine).toMatchObject({usedBytes:expected.used,footprintBytes:expected.used,evictBytes:0});
  expect(json.hostStore.result.collection.totals.totalBytes).toBe(expected.used);
  expect(stages.filter(stage=>stage===fleet.area)).toEqual([fleet.area]);
  expect(json.removed).toEqual([]);
  expect(readFileSync(state,"utf8")).toBe("replica state");
  expect(existsSync(sizes)?readFileSync(sizes,"utf8"):null).toBe(before);
  const scoped={only:new Set([fleet.area]),machine:{totalBytes:machinePolicyV1().defaultTotalBytes,source:"default" as const}},output={language:"en" as const,write:()=>{}};
  await collectCacheV1(workspace,ledger,{...scoped,dryRun:false},ports,output);
  expect((await loadCacheSizesV1(workspace,ledger)).records[fleet.area]).toEqual({usedBytes:expected.used,partial:false,atMs:now});
  stages.length=0;
  await collectCacheV1(workspace,ledger,{...scoped,dryRun:true},ports,output);
  expect(stages).not.toContain(fleet.area);
 }finally{rmSync(workspace,{recursive:true,force:true});}
});

test("CAS preview leaves stale staging files for a real collection",async()=>{
 const root=realpathSync(mkdtempSync(join(testScratchRootV1("cache-ledger-fleet"),"cs-"))),file=join(root,"v1","t","expired");
 try{
  mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"staging");utimesSync(file,new Date(0),new Date(0));
  await collectCasV1({root,dryRun:true});
  expect(existsSync(file)).toBe(true);
  await collectCasV1({root,dryRun:false});
  expect(existsSync(file)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("unavailable fleet measurement keeps the machine quota incomplete and ordinary aging active",async()=>{
 const ledger=cacheLedgerV1(),fleet=fixtures.fleetState,old=fleet.homes.find(home=>home.name===fleet.agedNeighbor)!,now=fixtures.now,workspace=realpathSync(mkdtempSync(join(testScratchRootV1("cache-ledger-fleet"),"fp-")));
 const root=cacheRootDirectoryV1(workspace,ledger,"native"),unavailable=join(root,fleet.home),generic=join(root,old.name,old.files[0]!);
 const ports=cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>undefined});
 try{
  mkdirSync(dirname(generic),{recursive:true});writeFileSync(unavailable,"state path is occupied");writeFileSync(generic,"aged cache");
  const stamp=new Date(now-old.ageMs);utimesSync(generic,stamp,stamp);
  const lines:string[]=[],result=await collectCacheV1(workspace,ledger,{only:new Set(["undeclared"]),dryRun:true,machine:{totalBytes:1<<30,source:"argument"}},ports,{language:"en",write:line=>lines.push(line)});
  expect(result.machine).toBeNull();
  expect(result.fleetStatePartial).toBe(true);
  expect(result.removed.map(row=>row.name)).toEqual([old.name]);
  expect(lines).toContain(cacheTextV1("en","machine.gc.fleetIncomplete"));
  expect(readFileSync(unavailable,"utf8")).toBe("state path is occupied");
 }finally{rmSync(workspace,{recursive:true,force:true});}
});

function generator(seed:number):()=>number{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};}

function corpus():CacheTargetV1[]{
 const now=fixtures.now,out:CacheTargetV1[]=[];
 for(const seed of seeds){
  const random=generator(seed);
  for(let index=0;index<casesPerSeed;index++){
   const count=Math.floor(random()*14),kind=(["area","retired","undeclared"] as const)[Math.floor(random()*3)]!,units=Array.from({length:count},(_,position)=>({name:"u"+String(position).padStart(2,"0")+(random()<0.2?"/deep":""),bytes:Math.floor(random()*1000),recencyMs:random()<0.15?now-Math.floor(random()*7200000):now-Math.floor(random()*40*86400000),...(random()<0.12?{held:"pid:"+(position+1)}:{})}));
   out.push({id:"s"+seed+"-"+index,kind,budgetBytes:random()<0.4?null:Math.floor(random()*4000),unusedAgeMs:kind==="retired"?0:random()<0.4?null:Math.floor(random()*30)*86400000,quietMs:Math.floor(random()*3)*3600000,...(random()<0.08?{heldBy:"lease"}:{}),units});
  }
 }
 return out;
}

function oracle(target:CacheTargetV1,now:number):{removals:string[][];kept:string[][];retained:number}{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE unit(name TEXT PRIMARY KEY, bytes INTEGER, recency INTEGER, held INTEGER)");
  const insert=database.prepare("INSERT INTO unit VALUES(?,?,?,?)");
  for(const unit of target.units)insert.run(unit.name,unit.bytes,unit.recencyMs,unit.held!==undefined||target.heldBy!==undefined?1:0);
  const base=target.kind==="retired"?"retired":target.kind==="undeclared"?"undeclared":"age";
  database.exec("CREATE TABLE blocked AS SELECT name, CASE WHEN held=1 THEN 'held' WHEN "+now+"-recency<"+target.quietMs+" THEN 'quiet' END AS reason FROM unit");
  const ageSql=target.unusedAgeMs===null?"SELECT name FROM unit WHERE 0":"SELECT unit.name FROM unit JOIN blocked ON blocked.name=unit.name WHERE "+now+"-recency>="+target.unusedAgeMs+" AND blocked.reason IS NULL";
  database.exec("CREATE TABLE aged AS "+ageSql);
  const removals:string[][]=[];
  for(const row of database.query("SELECT name FROM aged ORDER BY name").all() as {name:string}[])removals.push([row.name,base]);
  const total=(database.query("SELECT COALESCE(SUM(bytes),0) AS total FROM unit").get() as {total:number}).total,agedBytes=(database.query("SELECT COALESCE(SUM(bytes),0) AS total FROM unit WHERE name IN (SELECT name FROM aged)").get() as {total:number}).total;
  let remaining=total-agedBytes;
  if(target.budgetBytes!==null&&remaining>target.budgetBytes){
   const evicted=database.query(
    "WITH rest AS (SELECT unit.name, bytes, recency, blocked.reason AS reason FROM unit JOIN blocked ON blocked.name=unit.name WHERE unit.name NOT IN (SELECT name FROM aged)), "+
    "run AS (SELECT name, bytes, recency, reason, COALESCE(SUM(CASE WHEN reason IS NULL THEN bytes ELSE 0 END) OVER (ORDER BY recency, name ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING),0) AS before FROM rest) "+
    "SELECT name, bytes FROM run WHERE reason IS NULL AND "+remaining+"-before>"+target.budgetBytes+" ORDER BY recency, name"
   ).all() as {name:string;bytes:number}[];
   for(const row of evicted){removals.push([row.name,"budget"]);remaining-=row.bytes;}
  }
  const removed=new Set(removals.map(row=>row[0]!)),kept:string[][]=[];
  const wanted=new Set<string>();
  if(target.unusedAgeMs!==null)for(const row of database.query("SELECT name FROM unit WHERE "+now+"-recency>="+target.unusedAgeMs).all() as {name:string}[])wanted.add(row.name);
  if(target.budgetBytes!==null){
   const beforeBudget=total-agedBytes;
   if(beforeBudget>target.budgetBytes){
    let left=beforeBudget;
    for(const row of database.query("SELECT unit.name AS name, bytes, blocked.reason AS reason FROM unit JOIN blocked ON blocked.name=unit.name WHERE unit.name NOT IN (SELECT name FROM aged) ORDER BY recency, unit.name").all() as {name:string;bytes:number;reason:string|null}[]){
     if(left<=target.budgetBytes)break;
     if(row.reason)wanted.add(row.name);else left-=row.bytes;
    }
   }
  }
  for(const row of database.query("SELECT name, reason FROM blocked WHERE reason IS NOT NULL ORDER BY name").all() as {name:string;reason:string}[])if(wanted.has(row.name)&&!removed.has(row.name))kept.push([row.name,row.reason]);
  return{removals,kept,retained:remaining};
 }finally{database.close();}
}

test("a seeded corpus of plans equals an independent SQLite implementation of the same rules",()=>{
 const targets=corpus(),now=fixtures.now;
 expect(targets.length).toBe(seeds.length*casesPerSeed);
 for(const target of targets){
  const[plan]=planCacheCollectionV1([target],now),expected=oracle(target,now);
  const names=(rows:readonly {name:string}[])=>rows.map(row=>row.name);
  const actualRemovals=plan!.removals.map(row=>[row.name,row.reason]);
  const orderOf=(rows:string[][])=>[...rows.filter(row=>row[1]!=="budget").sort((left,right)=>left[0]!<right[0]!?-1:1),...rows.filter(row=>row[1]==="budget")];
  expect(orderOf(actualRemovals),target.id).toEqual(orderOf(expected.removals));
  expect(plan!.retainedBytes,target.id).toBe(expected.retained);
  expect([...plan!.kept.map(row=>[row.name,row.reason])].sort(),target.id).toEqual([...expected.kept].sort());
  for(const row of plan!.removals){const unit=target.units.find(candidate=>candidate.name===row.name)!;expect(unit.held,target.id).toBeUndefined();expect(target.heldBy,target.id).toBeUndefined();expect(now-unit.recencyMs>=target.quietMs,target.id).toBe(true);}
  expect(names(plan!.removals).length).toBe(new Set(names(plan!.removals)).size);
  expect(plan!.removals.reduce((sum,row)=>sum+row.bytes,0)+plan!.retainedBytes).toBe(plan!.totalBytes);
 }
});

test("planning twice over what remains finds nothing more to remove",()=>{
 const now=fixtures.now;
 for(const target of corpus()){
  const[first]=planCacheCollectionV1([target],now),gone=new Set(first!.removals.map(row=>row.name)),[second]=planCacheCollectionV1([{...target,units:target.units.filter(unit=>!gone.has(unit.name))}],now);
  expect(second!.removals.filter(row=>row.reason!=="budget").map(row=>row.name),target.id).toEqual([]);
 }
});

test("the corpus is reproducible and pinned by its digest",()=>{
 const digest=createHash("sha256").update(JSON.stringify(corpus())).digest("hex");
 expect(digest).toBe((fixtures as unknown as {corpusSha256:string}).corpusSha256);
});
