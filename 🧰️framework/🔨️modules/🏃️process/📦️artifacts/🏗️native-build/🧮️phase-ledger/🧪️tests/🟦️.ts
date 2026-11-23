import{test,expect}from"bun:test";
import{mkdtempSync,mkdirSync,writeFileSync,readFileSync,readdirSync,rmSync,appendFileSync,utimesSync}from"node:fs";
import{join}from"node:path";
import{tmpdir}from"node:os";
import{spawnSync}from"node:child_process";
import Ajv from"ajv";
import{createPhaseLedgerV1,phaseLedgerV1,withPhaseLedgerV1,phaseTableLinesV1,phaseTableBalancedV1,phaseTableCoveredV1,cargoTimingsSummaryV1,cargoTimingsLinesV1,readCargoTimingsV1,unitEventRowsV1,createUnitReuseTallyV1,readUnitEventsV1,phaseLedgerFileSinkV1,readPhaseLedgersV1,phaseLedgerRunLinesV1,phaseLedgerTaskV1,PHASE_LEDGER_POLICY_V1}from"../🟦️.ts";
import{emitUnitEventV1}from"../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";

const python=(program:string,input:unknown):any=>{const run=spawnSync("python3",["-I","-c",program],{input:JSON.stringify(input),encoding:"utf8"});if(run.status!==0)throw Error(run.stderr);return JSON.parse(run.stdout);};
const cargo=spawnSync("cargo",["--version"],{encoding:"utf8"}).status===0;
const synthetic=(root:string)=>{
 const write=(path:string,text:string)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);};
 write("Cargo.toml",'[workspace]\nmembers=["a","b","c","d"]\nresolver="2"\n');
 for(const[name,dependency]of[["a",""],["b",'a={path="../a"}'],["c",'b={path="../b"}'],["d",'c={path="../c"}']])write(name+"/Cargo.toml",`[package]\nname="${name}"\nversion="0.1.0"\nedition="2021"\n[dependencies]\n${dependency}\n`);
 write("a/src/lib.rs","pub fn a()->u32{1}\n");write("b/src/lib.rs","pub fn b()->u32{a::a()+1}\n");write("c/src/lib.rs","pub fn c()->u32{b::b()+1}\n");write("d/src/main.rs","fn main(){println!(\"{}\",c::c());}\n");
};
const buildWithTimings=(root:string,target:string)=>spawnSync("cargo",["build","--offline","--timings","--manifest-path",join(root,"Cargo.toml")],{cwd:root,env:{...process.env,CARGO_TARGET_DIR:target},encoding:"utf8"});

/** 🧪️ Exclusive attribution agrees with an independent Python replay of 150 seeded nested scripts, and the rows plus the residual always add up to the total. */
test("phase ledger: exclusive phases and the residual equal an independent replay and sum to the total",()=>{
 let seed=20261118;const random=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
 const scripts=Array.from({length:150},()=>{
  const ops:Array<{op:string;name?:string;ms?:number}>=[],stack:string[]=[];
  for(let step=0;step<40;step++){
   const pick=random();
   if(pick<0.4){ops.push({op:"wait",ms:Math.floor(random()*900)});}
   else if(pick<0.7&&stack.length<5){const name="p"+Math.floor(random()*6);stack.push(name);ops.push({op:"begin",name});}
   else if(stack.length){stack.pop();ops.push({op:"end"});}
  }
  while(stack.length){stack.pop();ops.push({op:"end"});}
  return ops;
 });
 const oracle=python(`
import json,sys
scripts=json.load(sys.stdin);out=[]
for ops in scripts:
    t=0;last=0;stack=[];sums={};counts={};other=0
    def mark():
        global last,other
        d=t-last;last=t
        if stack: sums[stack[-1]]=sums.get(stack[-1],0)+d
        else: other+=d
    for o in ops:
        if o['op']=='wait': t+=o['ms']
        elif o['op']=='begin': mark();stack.append(o['name'])
        else:
            mark();n=stack.pop();counts[n]=counts.get(n,0)+1
    mark()
    out.append({'sums':sums,'counts':counts,'other':other,'total':t})
print(json.dumps(out))`,scripts);
 scripts.forEach((ops,index)=>{
  let clock=0;const ledger=createPhaseLedgerV1({task:"t",now:()=>clock}),ends:Array<()=>void>=[];
  for(const op of ops){if(op.op==="wait")clock+=op.ms!;else if(op.op==="begin")ends.push(ledger.begin(op.name!));else ends.pop()!();}
  const table=ledger.finish(),want=oracle[index];
  expect(Math.round(table.otherSeconds*1000)).toBe(want.other);
  expect(Math.round(table.totalSeconds*1000)).toBe(want.total);
  for(const row of table.rows){expect(Math.round(row.seconds*1000)).toBe(want.sums[row.phase]??0);expect(row.count).toBe(want.counts[row.phase]??0);}
  expect(phaseTableBalancedV1(table)).toBe(true);
 });
});

/** 🧪️ A nested phase pauses its parent, a phase left open by a failure is closed at the end, and a task without a ledger records nothing. */
test("phase ledger: nesting, failure, bytes, ambient ledger and the no-op ledger",async()=>{
 let clock=0;const ledger=createPhaseLedgerV1({task:"p:t",now:()=>clock});
 const outer=ledger.begin("outer");clock+=100;await ledger.run("inner",async()=>{clock+=300;},3);clock+=50;ledger.bytes("inner",1000.9);ledger.bytes("inner",24);
 await expect(ledger.run("failing",async()=>{clock+=10;throw Error("x");})).rejects.toThrow("x");
 ledger.begin("dangling");clock+=5;
 const table=ledger.finish();outer();
 const by=Object.fromEntries(table.rows.map(row=>[row.phase,row]));
 expect(by.outer!.seconds).toBeCloseTo(0.15,9);expect(by.inner!.seconds).toBeCloseTo(0.3,9);expect(by.inner!.count).toBe(3);expect(by.inner!.bytes).toBe(1024);expect(by.failing!.seconds).toBeCloseTo(0.01,9);expect(by.failing!.count).toBe(1);expect(by.dangling!.seconds).toBeCloseTo(0.005,9);
 expect(table.otherSeconds).toBe(0);expect(phaseTableBalancedV1(table)).toBe(true);
 expect(await phaseLedgerV1().run("x",async()=>7)).toBe(7);expect(phaseLedgerV1().finish().rows.length).toBe(0);
 const ambient=createPhaseLedgerV1({task:"a",now:()=>clock});
 await withPhaseLedgerV1(ambient,async()=>{await Promise.resolve();await phaseLedgerV1().run("seen",async()=>{clock+=1;});});
 expect(ambient.finish().rows.map(row=>row.phase)).toEqual(["seen"]);
 expect(phaseLedgerTaskV1({NX_TASK_TARGET_PROJECT:"@s/p",NX_TASK_TARGET_TARGET:"build",NX_TASK_TARGET_CONFIGURATION:"prototype"},"f")).toBe("@s/p:build:prototype");
 expect(phaseLedgerTaskV1({},"f")).toBe("f");
});

/** 🧪️ A real offline Cargo run: the wall-clock time measured outside equals the ledger total within a small tolerance, the phases sum to it, the unclaimed time is covered, and Cargo's own timing report agrees with an independent Python reading (units, wall, front end, code generation, critical path). */
test.skipIf(!cargo)("phase ledger: a real offline cargo workspace sums to its wall time and the timing report matches an independent reading",async()=>{
 const root=mkdtempSync(join(tmpdir(),"r206-cargo-")),target=join(root,"t");
 try{
  synthetic(root);
  const ledger=createPhaseLedgerV1({task:"syn:build"}),outside=performance.now();
  await withPhaseLedgerV1(ledger,async()=>{
   await phaseLedgerV1().run("setup",async()=>{await new Promise(done=>setTimeout(done,120));});
   const run=await phaseLedgerV1().run("cargo",async()=>buildWithTimings(root,target));
   expect(run.status).toBe(0);
   await phaseLedgerV1().run("stage",async()=>{await new Promise(done=>setTimeout(done,60));});
  });
  const table=ledger.finish(),measured=(performance.now()-outside)/1000;
  expect(Math.abs(table.totalSeconds-measured)).toBeLessThan(0.05);
  expect(phaseTableBalancedV1(table)).toBe(true);expect(phaseTableCoveredV1(table)).toBe(true);
  expect(table.otherSeconds).toBeLessThan(0.05);
  const cargoSeconds=table.rows.find(row=>row.phase==="cargo")!.seconds;expect(cargoSeconds).toBeGreaterThan(0.2);
  const summary=readCargoTimingsV1(target,0)!;expect(summary).toBeDefined();
  expect(summary.wallSeconds).toBeLessThanOrEqual(cargoSeconds+0.05);
  const html=readFileSync(join(target,"cargo-timings","cargo-timing.html"),"utf8");
  const want=python(`
import json,re,sys
t=sys.stdin.read()
d=json.loads(re.search(r'const UNIT_DATA = (\\[.*?\\n\\]);',json.loads(t),re.S).group(1))
end=lambda u:u['start']+u['duration']
def sec(u,n): return sum(s[1]['end']-s[1]['start'] for s in (u.get('sections') or []) if s[0]==n)
def rm(u):
    f=[s for s in (u.get('sections') or []) if s[0]=='frontend']
    return u['start']+(f[0][1]['end'] if f else u['duration'])
cur=max(d,key=end);chain=[]
while cur is not None and cur['i'] not in [c['i'] for c in chain]:
    chain.append(cur);c=cur;cands=[]
    for u in d:
        if c['i'] in u['unblocked_units']: cands.append((end(u),u))
        elif c['i'] in u['unblocked_rmeta_units']: cands.append((rm(u),u))
    cur=max(cands,key=lambda x:x[0])[1] if cands else None
print(json.dumps({'units':len(d),'wall':max(end(u) for u in d),'start':min(u['start'] for u in d),'frontend':sum(sec(u,'frontend') for u in d),'codegen':sum(sec(u,'codegen') for u in d),'critical':sum(u['duration'] for u in chain),'chain':len(chain)}))`,html);
  expect(summary.units).toBe(want.units);expect(summary.units).toBe(4);
  expect(summary.wallSeconds).toBeCloseTo(want.wall,9);expect(summary.startSeconds).toBeCloseTo(want.start,9);expect(summary.frontendSeconds).toBeCloseTo(want.frontend,9);expect(summary.codegenSeconds).toBeCloseTo(want.codegen,9);expect(summary.criticalSeconds).toBeCloseTo(want.critical,9);expect(summary.criticalUnits).toBe(want.chain);
  expect(summary.criticalUnits).toBe(4);
  expect(cargoTimingsLinesV1("syn:build",summary)[0]).toContain("units=4");
  const lines=phaseTableLinesV1(table);expect(lines[0]).toMatch(/^\[phase\] task=syn:build total=\d+\.\d{3} phases=\d+\.\d{3} other=\d+\.\d{3}$/u);expect(lines.length).toBe(1+3);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

/** 🧪️ Slow units are named only when they exceed the derived fraction of the wall; an unreadable report gives nothing. */
test("phase ledger: slow-unit lines are derived from the wall and a broken report is refused",()=>{
 const unit=(i:number,duration:number,start:number)=>({i,name:"u"+i,version:"1",mode:"todo",target:"",features:[],start,duration,unblocked_units:i<9?[i+1]:[],unblocked_rmeta_units:[],sections:[["frontend",{start:0,end:duration/2}],["codegen",{start:duration/2,end:duration}]]});
 const page=(units:object[])=>"<script>\nDURATION = 1;\nconst UNIT_DATA = "+JSON.stringify(units,null,2)+";\nconst CONCURRENCY_DATA = [];\n</script>";
 const even=cargoTimingsSummaryV1(page(Array.from({length:10},(_,i)=>unit(i,1,i))))!;
 expect(even.units).toBe(10);expect(even.wallSeconds).toBe(10);expect(even.criticalUnits).toBe(10);expect(even.criticalSeconds).toBe(10);expect(even.slow.length).toBe(10);
 const skewed=cargoTimingsSummaryV1(page([unit(0,0.01,0),...Array.from({length:8},(_,i)=>unit(i+1,0.01,0.01*(i+1))),unit(9,50,1)]))!;
 expect(skewed.slow.map(row=>row.name)).toEqual(["u9"]);
 const cargoLines=cargoTimingsLinesV1("t",skewed);expect(cargoLines.length).toBe(2);expect(cargoLines[1]).toContain("slow=u9");
 expect(cargoTimingsSummaryV1("no report")).toBeUndefined();expect(cargoTimingsSummaryV1("const UNIT_DATA = [broken\n];")).toBeUndefined();expect(cargoTimingsSummaryV1(page([]))).toBeUndefined();
 expect(PHASE_LEDGER_POLICY_V1.slowUnitFraction).toBe(0.1);
});

/** 🧪️ A fake fleet tier writes the real unit event format; the ledger sums restores, lookups, compiles, publications and waits per lane and window exactly as an independent Python sum. */
test("phase ledger: unit events of a fake fleet tier aggregate to counts and seconds per lane and window",()=>{
 const root=mkdtempSync(join(tmpdir(),"r206-events-"));
 try{
  let seed=7;const random=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
  const kinds=["hit","remote-hit","remote-miss","miss","publish","slot-wait","lock-wait","remote-wait","bypass","failed"] as const;
  const written:Array<{lane:string;kind:string;milliseconds:number;waitedMs:number}>=[];
  for(let index=0;index<400;index++){const row={lane:random()<0.8?"g":"o",kind:kinds[Math.floor(random()*kinds.length)]!,milliseconds:Math.floor(random()*5000),waitedMs:Math.floor(random()*900)};written.push(row);emitUnitEventV1(root,{lane:row.lane,kind:row.kind,crate:"c"+index,key:null,milliseconds:row.milliseconds,waitedMs:row.waitedMs});}
  const events=readUnitEventsV1(root);expect(events.length).toBe(400);
  const rows=unitEventRowsV1(events,"g",0,Date.now()+1000);
  const want=python(`
import json,sys
d=json.load(sys.stdin);m={'hit':'unit:restore-local','remote-hit':'unit:restore-fleet','remote-miss':'unit:lookup-fleet-miss','miss':'unit:compile','publish':'unit:publish','slot-wait':'unit:wait','lock-wait':'unit:wait','remote-wait':'unit:wait'}
s={}
for r in d:
    if r['lane']!='g' or r['kind'] not in m: continue
    p=m[r['kind']];ms=r['waitedMs'] if p=='unit:wait' else r['milliseconds'];a=s.setdefault(p,[0,0]);a[0]+=ms;a[1]+=1
print(json.dumps(s))`,written);
  expect(rows.length).toBe(Object.keys(want).length);
  for(const row of rows){expect(row.within).toBe(true);expect(row.count).toBe(want[row.phase][1]);expect(Math.round(row.seconds*1000)).toBe(want[row.phase][0]);}
  expect(unitEventRowsV1(events,"g",Date.now()+10000,Date.now()+20000).length).toBe(0);
  expect(unitEventRowsV1(events,"absent",0,Date.now()+1000).length).toBe(0);
  appendFileSync(join(root,PHASE_LEDGER_POLICY_V1.eventsDirectory,"g.jsonl"),"not json\n{\"atMs\":\"x\"}\n");
  expect(readUnitEventsV1(root).length).toBe(400);expect(readUnitEventsV1(join(root,"missing")).length).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧪️ The table survives in a file after every closed phase (a killed task leaves its last), validates against the schema, is read back newest per process after the run began, and the front prints one line per phase plus the part of the run outside the longest task. */
test("phase ledger: file sink, schema, killed task and the lines of the front",()=>{
 const workspace=mkdtempSync(join(tmpdir(),"r206-sink-"));
 try{
  let clock=0;
  const killed=createPhaseLedgerV1({task:"killed:build",now:()=>clock,sink:phaseLedgerFileSinkV1(workspace,111)});
  killed.begin("a")();clock+=2000;const open=killed.begin("b");clock+=500;open();clock+=700;
  clock=0;const complete=createPhaseLedgerV1({task:"done:build",now:()=>clock,sink:phaseLedgerFileSinkV1(workspace,222)});
  const end=complete.begin("cargo");clock+=4000;end();complete.add({phase:"unit:restore-fleet",seconds:12.5,count:688,bytes:null,within:true});complete.finish();
  const ajv=new Ajv({strict:false}),validate=ajv.compile(schema as object);
  const directory=readdirSync(workspace,{recursive:true}).map(String).filter(name=>name.endsWith(".jsonl"));expect(directory.length).toBe(2);
  for(const file of directory)for(const line of readFileSync(join(workspace,file),"utf8").trim().split("\n"))expect(validate(JSON.parse(line))).toBe(true);
  const tables=readPhaseLedgersV1(workspace,0);expect(tables.map(table=>table.task).sort()).toEqual(["done:build","killed:build"]);
  const finished=tables.find(table=>table.task==="done:build")!;expect(finished.rows.find(row=>row.within)!.count).toBe(688);expect(finished.totalSeconds).toBe(4);
  const lines=phaseLedgerRunLinesV1(workspace,0,10);
  expect(lines.filter(line=>line.startsWith("[phase] task=done:build name=")).length).toBe(2);
  expect(lines.at(-1)).toMatch(/^\[phase\] run total=10\.000 longest-task=4\.000 outside-task=6\.000$/u);
  expect(phaseLedgerRunLinesV1(workspace,Date.now()+60000,10)).toEqual([]);
  for(const file of directory)utimesSync(join(workspace,file),new Date(0),new Date(0));
  expect(readPhaseLedgersV1(workspace,Date.now()-1000)).toEqual([]);
 }finally{rmSync(workspace,{recursive:true,force:true});}
});

/** 🧪️ The Cargo path prints the ledger and records its phases: the task source names every phase of the ledger, asks Cargo for its timing report, prints the table at the end and the front prints the ledgers of the run. */
test("phase ledger: the cargo path and the front of the run are wired",()=>{
 const native=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),front=readFileSync(new URL("../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts",import.meta.url),"utf8");
 for(const phase of["plan:graph","plan:envelope","plan:sources","plan:recipe","plan:facts","recheck:graph","recheck:envelope","recheck:sources","recheck:recipe","recheck:facts","lease:target","wait:capacity","lock-current","cargo","stage","provenance","custody","consume","consume:source-digests","setup","ledger"])expect(native).toContain('"'+phase+'"');
 expect(native).toContain('"--timings"');expect(native).toContain("phaseTableLinesV1(ledger.finish())");expect(native).toContain("endSetup()");
 expect(front).toContain("phaseLedgerRunLinesV1(this.root,runStartedMs");
});

/** 🧪️ The reuse tally counts only this lane's cache restores written after its creation (local and fleet apart), reads a line in pieces, and restarts when the log shrank. */
test("unit reuse tally: counts local and fleet restores of one lane after its creation, incrementally",()=>{
 const root=mkdtempSync(join(tmpdir(),"reuse-tally-"));
 try{
  const emit=(lane:string,kind:string)=>emitUnitEventV1(root,{lane,kind:kind as never,crate:"c",key:null,milliseconds:1,waitedMs:0});
  emit("g","hit");
  const tally=createUnitReuseTallyV1(root,"g");
  expect(tally.read()).toEqual({local:0,fleet:0});
  for(const kind of["hit","script-hit","remote-hit","miss","publish","remote-miss"])emit("g",kind);
  emit("other","hit");
  expect(tally.read()).toEqual({local:2,fleet:1});
  const file=join(root,"x","g.jsonl"),line=JSON.stringify({version:1,atMs:1,lane:"g",kind:"remote-hit",crate:"c",key:null,milliseconds:1,waitedMs:0})+"\n";
  appendFileSync(file,line.slice(0,20));
  expect(tally.read()).toEqual({local:2,fleet:1});
  appendFileSync(file,line.slice(20));
  expect(tally.read()).toEqual({local:2,fleet:2});
  writeFileSync(file,line);
  expect(tally.read()).toEqual({local:2,fleet:3});
  expect(createUnitReuseTallyV1(join(root,"missing"),"g").read()).toEqual({local:0,fleet:0});
 }finally{rmSync(root,{recursive:true,force:true});}
});
