/** 🔱️ Contract: the warm pass splits the owners into stable lanes, settles them in short-lived children that write durable rows, skips what an earlier try already made durable, keeps the parent flat and stays inside the planned memory.
 * @see ../🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import ordering from"./🧫️fixtures/🔢️warm-order.json" with {type:"json"};
import{spawn}from"node:child_process";
import{appendFileSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{availableParallelism,tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{WARM_POLICY_V1,appendWarmRowV1,isWarmRowV1,planWarmLanesV1,planWarmWidthV1,readStoredRecordV1,readWarmIndexV1,warmDirectoryV1,runWarmLanesV1,sweepWarmPartsV1,warmDigestV1,warmLineV1,warmWaveV1}from"../🟨️.mjs";

const here=fileURLToPath(new URL("./",import.meta.url)),child=join(here,"🧫️fixtures/🧮️child.mjs"),bun=process.execPath;
const names=(count:number)=>Array.from({length:count},(_,index)=>"owner-"+String(index).padStart(4,"0"));
const scratch=()=>mkdtempSync(join(tmpdir(),"semio-warm-"));

type Child={ask(name:string):Promise<{peakMegabytes:number}>;close():void};
function startChild(directory:string,record:string,spin:number):Child{
 const process_=spawn(bun,[child,directory,record,String(spin)],{stdio:["pipe","pipe","inherit"]});let buffer="";const waiting:((rss:number)=>void)[]=[];
 process_.on("exit",code=>{for(const each of waiting.splice(0))each(-1);if(code!==0&&code!==null&&!closing)throw new Error("fixture child exited "+code);});let closing=false;
 process_.stdout.on("data",chunk=>{buffer+=chunk;for(let at=buffer.indexOf("\n");at>=0;at=buffer.indexOf("\n")){const line=JSON.parse(buffer.slice(0,at));buffer=buffer.slice(at+1);waiting.shift()?.(line.rss);}});
 return{ask:name=>new Promise(resolve=>{waiting.push(rss=>resolve({peakMegabytes:rss}));process_.stdin.write(name+"\n");}),close:()=>{closing=true;process_.stdin.end();process_.kill();}};
}
const rowsOf=(directory:string)=>readdirSync(directory).filter(file=>file.endsWith(".part")).flatMap(file=>readFileSync(join(directory,file),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line)));

test("the lanes are stable whatever the order, cover every owner once and spread the costliest owners evenly",()=>{
 const owners=names(200).map((name,index)=>({name,key:warmDigestV1(name),cost:index%17===0?900:index%5===0?100:1}));
 const first=planWarmLanesV1(owners,7),second=planWarmLanesV1([...owners].reverse(),7);
 expect(second).toEqual(first);
 expect(first.flat().sort()).toEqual(owners.map(owner=>owner.name).sort());
 const load=first.map(lane=>lane.reduce((sum,name)=>sum+owners.find(owner=>owner.name===name)!.cost,0)),mean=load.reduce((a,b)=>a+b,0)/load.length;
 expect(Math.max(...load)).toBeLessThanOrEqual(mean*1.15);
 expect(first.map(lane=>lane.filter(name=>owners.find(owner=>owner.name===name)!.cost===900).length).every(count=>Math.abs(count-owners.filter(owner=>owner.cost===900).length/7)<=1)).toBe(true);
});

test("large warm rosters score each owner key once and preserve the independent lane plan",()=>{
 const db=new Database(":memory:"),rows=names(ordering.owners).map((name,index)=>({
  name,
  key:index%ordering.repeatKeyEvery===0?"repeated-key":name,
  cost:index%ordering.heavyEvery===0?ordering.heavyCost:index%ordering.mediumEvery===0?ordering.mediumCost:ordering.baseCost,
 }));
 let reads=0;
 const owners=rows.map(({name,key,cost})=>({name,cost,get key(){reads++;return key;}}));
 try{
  const ranked=rows.map(row=>[row.name,row.cost,new Bun.CryptoHasher("sha1").update(row.key).digest("hex").slice(0,8)]);
  const sorted=db.query(`select json_extract(value,'$[0]') as name, json_extract(value,'$[1]') as cost
   from json_each(?1) order by cost desc, json_extract(value,'$[2]'), name`).all(JSON.stringify(ranked)) as {name:string;cost:number}[];
  const expected=Array.from({length:ordering.lanes},()=>({load:0,names:[] as string[]}));
  for(const owner of sorted){
   const lowest=expected.reduce((first,lane)=>lane.load<first.load?lane:first);
   lowest.names.unshift(owner.name);
   lowest.load+=owner.cost+1;
  }
  expect(planWarmLanesV1(owners,ordering.lanes)).toEqual(expected.map(lane=>lane.names));
  expect(reads).toBeLessThanOrEqual(ordering.owners);
 }finally{db.close();}
});

test("parts: a row is durable once written, a torn last line, an old row and an absent record never count, rows are held as strings only",()=>{
 const directory=scratch();const fresh=Date.now();
 const row=(n:string,extra:{o?:"settled"|"other"|"absent";t?:number}={})=>({v:1 as const,n,k:warmDigestV1(n),d:"10:1",o:"settled" as "settled"|"other"|"absent",t:fresh,...extra});
 appendWarmRowV1(directory,row("a"),1);appendWarmRowV1(directory,row("b",{o:"absent"}),1);appendWarmRowV1(directory,row("old",{t:fresh-WARM_POLICY_V1.maximumAgeMilliseconds-1}),2);
 appendFileSync(join(directory,"2.part"),'{"v":1,"n":"torn","k":"abc');
 const index=readWarmIndexV1(directory,{now:()=>fresh});
 expect(index.has("a",warmDigestV1("a"),"10:1")).toBe(true);
 expect(index.has("a",warmDigestV1("a"),"11:1")).toBe(false);
 expect(index.has("a",warmDigestV1("other"),"10:1")).toBe(false);
 expect([index.has("b",warmDigestV1("b"),"10:1"),index.has("old",warmDigestV1("old"),"10:1"),index.has("torn","x","y")]).toEqual([false,false,false]);
 expect(index.size).toBe(1);expect(isWarmRowV1(row("a"))).toBe(true);expect(isWarmRowV1({...row("a"),o:"done"})).toBe(false);expect(isWarmRowV1({...row("a"),extra:1})).toBe(false);
 expect(sweepWarmPartsV1(directory,{now:()=>fresh+10*WARM_POLICY_V1.maximumAgeMilliseconds})).toBe(2);
 rmSync(directory,{recursive:true});
});

test("the width is one less than the cores, bounded by the ledger and the farm, and the planned tree stays inside the limit of a 16 GB machine",()=>{
 expect(planWarmWidthV1({cores:10})).toBe(9);expect(planWarmWidthV1({cores:1})).toBe(1);
 const limit=6553;const width=planWarmWidthV1({cores:10,limitMegabytes:limit});
 expect(width).toBe(2);expect(WARM_POLICY_V1.parentMegabytes+width*WARM_POLICY_V1.childMegabytes).toBeLessThanOrEqual(limit);
 for(const cores of [2,4,8,10,16])for(const megabytes of [3276,6553,13107,26214]){const k=planWarmWidthV1({cores,limitMegabytes:megabytes});expect(k).toBeLessThanOrEqual(Math.max(1,cores-1));if(k>0)expect(WARM_POLICY_V1.parentMegabytes+k*WARM_POLICY_V1.childMegabytes).toBeLessThanOrEqual(megabytes);}
 expect(planWarmWidthV1({cores:10,farmLoops:3,owners:2})).toBe(2);expect(planWarmWidthV1({cores:10,limitMegabytes:1000})).toBe(0);
 expect(warmLineV1("de",{done:3,total:9,lanes:2})).toContain("abgeglichen 3 von 9");expect(warmLineV1("en",{done:3,total:9})).toContain("warmed 3 of 9");
});

test("real children: K lanes give at least 0.7 K the throughput of one lane on a clean owner (a spin of the owner's cost)",async()=>{
 const owners=names(48),spin=120,K=Math.min(4,Math.max(2,availableParallelism()-1));
 const timed=async(width:number)=>{
  const directory=scratch(),record=join(directory,"record.json");writeFileSync(record,"{}");
  const children=Array.from({length:width},()=>startChild(directory,record,spin)),lanes=planWarmLanesV1(owners.map(name=>({name,key:warmDigestV1(name),cost:1})),width);
  await children[0].ask("warm-up");const started=performance.now();
  const lane=new Map(lanes.map((list,at)=>list.map(name=>[name,at] as const)).flat());
  await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:name=>children[lane.get(name)!].ask(name)});
  const seconds=(performance.now()-started)/1000;children.forEach(each=>each.close());const rows=rowsOf(directory).filter(row=>row.n!=="warm-up");rmSync(directory,{recursive:true});
  expect(new Set(rows.map(row=>row.n)).size).toBe(owners.length);return seconds;
 };
 const one=await timed(1),many=await timed(K);console.log("[warm] 48 owners x "+spin+" ms: 1 lane "+one.toFixed(2)+" s, "+K+" lanes "+many.toFixed(2)+" s, speed-up "+(one/many).toFixed(2));
 expect(one/many).toBeGreaterThanOrEqual(0.7*K);
},120000);

test("real children: a try killed midway leaves durable rows and the next try warms only the owners without one",async()=>{
 const owners=names(40),directory=scratch(),record=join(directory,"record.json");writeFileSync(record,"{}");mkdirSync(directory,{recursive:true});
 const inspect=(name:string)=>({key:warmDigestV1("key:"+name),stamp:"0:0"});
 const first=startChild(directory,record,40);const stopper=new AbortController();let asked=0;
 const lanes=[owners];
 await runWarmLanesV1({lanes,inspect,warm:async name=>{asked++;const reply=await first.ask(name);if(asked===15)stopper.abort();return reply;},signal:stopper.signal});first.close();
 const durable=new Set(rowsOf(directory).map(row=>row.n));expect(durable.size).toBe(15);
 const second=startChild(directory,record,40);const again:string[]=[];
 const index=(()=>{const held=new Set(rowsOf(directory).map(row=>row.n+"\0"+row.k));return{size:held.size,has:(name:string,key:string)=>held.has(name+"\0"+key)};})();
 const result=await runWarmLanesV1({lanes,inspect,index,warm:async name=>{again.push(name);return second.ask(name);}});second.close();
 expect(result.durable).toBe(15);expect(again.length).toBe(25);expect(again.some(name=>durable.has(name))).toBe(false);
 expect(new Set(rowsOf(directory).map(row=>row.n)).size).toBe(40);
 rmSync(directory,{recursive:true});
},60000);

test("the parent stays flat for 1000 owners: no per-owner object is kept and the progress is summed and never goes back",async()=>{
 const owners=names(1000),lanes=planWarmLanesV1(owners.map(name=>({name,key:warmDigestV1(name),cost:1})),8),told:number[]=[];
 Bun.gc(true);const before=process.memoryUsage().rss;let peak=0;
 const counters=await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:async()=>{await Promise.resolve();peak=Math.max(peak,process.memoryUsage().rss);return{peakMegabytes:1500};},tell:fields=>told.push(fields.done),progressMilliseconds:0});
 expect(counters).toMatchObject({total:1000,done:1000,settled:1000,failed:0,peak:1500});
 expect([...told].sort((a,b)=>a-b)).toEqual(told);expect(told.at(-1)).toBe(1000);
 expect((peak-before)/1048576).toBeLessThan(100);expect(peak/1073741824).toBeLessThan(1.5);
});

test("a failed child leaves the owner to the parent and a cancelled signal stops the lanes",async()=>{
 const lanes=[names(10),names(10).map(name=>"b"+name)];let calls=0;
 const counters=await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:async name=>{calls++;return name.startsWith("b")?undefined:{peakMegabytes:1};}});
 expect(counters).toMatchObject({settled:10,failed:10,done:20});
 const stop=new AbortController();stop.abort();calls=0;await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:async()=>{calls++;return{peakMegabytes:1};},signal:stop.signal});expect(calls).toBe(0);
});

test("the wave pass answers undefined (the parent takes the owners itself) without a farm, with the switch off, for small waves and when no child comes",async()=>{
 const base={names:names(100),describe:(name:string)=>({key:warmDigestV1(name),stamp:"1:1",cost:1,job:{name}}),directory:scratch(),cores:8};
 const farm=(ready:boolean,extra={})=>({warm:async()=>({peakMegabytes:1}),whenReady:async()=>ready,width:()=>4,...extra});
 expect(await warmWaveV1({...base,farm:undefined})).toBeUndefined();
 expect(await warmWaveV1({...base,farm:farm(true),environment:{SEMIO_GRAPH_WARM:"off"}})).toBeUndefined();
 expect(await warmWaveV1({...base,farm:farm(true),names:names(10)})).toBeUndefined();
 expect(await warmWaveV1({...base,farm:farm(false)})).toBeUndefined();
 expect(await warmWaveV1({...base,farm:farm(true,{width:()=>0})})).toBeUndefined();
});

test("the wave pass engages the farm, runs every owner on at most the planned lanes and skips the owners that are durable",async()=>{
 const directory=scratch(),owners=names(120);let running=0,most=0,engaged=0;const warmed:string[]=[];
 for(const name of owners.slice(0,30))appendWarmRowV1(directory,{v:1,n:name,k:warmDigestV1(name),d:"1:1",o:"settled",t:Date.now()});
 const told:number[]=[];
 const counters=await warmWaveV1({names:owners,directory,cores:6,limitMegabytes:Infinity,describe:name=>({key:warmDigestV1(name),stamp:"1:1",cost:1,job:{name}}),tell:fields=>told.push(fields.done),
  farm:{engage:()=>{engaged++;},whenReady:async()=>true,width:()=>16,workerMegabytes:()=>1700,warm:async(job:any)=>{running++;most=Math.max(most,running);await new Promise(done=>setTimeout(done,2));running--;warmed.push(job.name);return{peakMegabytes:900};}}});
 expect(engaged).toBe(1);expect(counters).toMatchObject({total:120,done:120,durable:30,settled:90,failed:0,lanes:5,peak:900});
 expect(most).toBeLessThanOrEqual(5);expect(new Set(warmed).size).toBe(90);expect(warmed.some(name=>owners.indexOf(name)<30)).toBe(false);expect(told.at(-1)).toBe(120);
 rmSync(directory,{recursive:true});
});

test("a stored record file is read with plain file access: header digest, value and the closure rows its recipes name; a torn file answers undefined",async()=>{
 const{brotliCompressSync}=await import("node:zlib");const directory=scratch(),path=join(directory,"record.json");
 writeFileSync(path,brotliCompressSync(Buffer.from(JSON.stringify({digest:"abc"})+"\n"+JSON.stringify({closureRecipes:{r1:1,r2:2},x:3}))));
 const record=readStoredRecordV1(path,"/root")!;expect(record.header.digest).toBe("abc");expect((record.value as any).x).toBe(3);
 expect(record.header.observations).toEqual([{kind:"closure",path:JSON.stringify(["/root","r1",["digest"]])},{kind:"closure",path:JSON.stringify(["/root","r2",["digest"]])}]);
 writeFileSync(path,"torn");expect(readStoredRecordV1(path,"/root")).toBeUndefined();expect(readStoredRecordV1(join(directory,"none"),"/root")).toBeUndefined();
 expect(warmDirectoryV1("/w")).toBe("/w/.cache/f/warm");rmSync(directory,{recursive:true});
});
