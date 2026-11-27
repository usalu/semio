/** 🌳️ Contract: the closure digest of every owner equals an independent Python oracle (plain recursive reachability over a fixture graph with cycles, cut at declared tool entries unless they are roots); an edit of one node moves exactly the owners that reach it; incremental updates equal a graph built from scratch; the digest-addressed store round-trips, rewrites one chunk per edited bucket and keeps the previous head whole when a save is cancelled. */
import{expect,setDefaultTimeout,test}from"bun:test";
import{mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{CLOSURE_GRAPH_POLICY_V1,createClosureGraphV1,loadClosureGraphSyncV1,loadClosureGraphV1,moduleNodeDigestV1,saveClosureGraphV1}from"../🟨️.mjs";

setDefaultTimeout(120000);
const oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py"),python=process.env.SEMIO_PYTHON??"python3";
type Node={key:string;name:string;kind:string;edges:string[];body:string};
type Owner={name:string;roots:string[];stops:Record<string,string>};
const random=(seed:number)=>()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
const content=(node:Node,nodes:Map<string,Node>)=>({kind:node.kind,name:node.name,edges:node.edges.map(edge=>nodes.get(edge)!.name).sort(),body:node.body});
const fixture=(seed:number,size=360)=>{
 const draw=random(seed),nodes=new Map<string,Node>(),key=(at:number)=>"k"+at;
 for(let at=0;at<size;at++)nodes.set(key(at),{key:key(at),name:(at%7===0?"🧰️lib/":"src/")+"m"+at+(at%11===0?".json":".ts"),kind:at%11===0?"resource":"module",edges:[],body:"b"+at});
 for(let at=0;at<size;at++){const node=nodes.get(key(at))!;if(node.kind==="resource")continue;const count=Math.floor(draw()*4);for(let edge=0;edge<count;edge++){const forward=draw()<0.8,target=forward?Math.min(size-1,at+1+Math.floor(draw()*40)):Math.floor(draw()*at);if(target!==at)node.edges.push(key(target));}}
 for(const[from,to]of[[10,30],[30,20],[20,10],[100,140],[140,100]])nodes.get(key(from))!.edges.push(key(to));
 const tools=[45,90,135,180,225].map(at=>key(at)),stops=Object.fromEntries(tools.map((tool,at)=>[tool,"tool-"+at]));
 const owners:Owner[]=Array.from({length:40},(_,at)=>({name:"owner-"+at,roots:Array.from({length:1+Math.floor(draw()*3)},()=>key(Math.floor(draw()*size))),stops:at%2===0?stops:{}}));
 owners.push({name:"tool-root",roots:[tools[1]!],stops});
 return{nodes,owners,tools};
};
const ask=(nodes:Map<string,Node>,owners:Owner[])=>{
 const input={nodes:[...nodes.values()].map(node=>({key:node.key,name:node.name,kind:node.kind,content:content(node,nodes),edges:[...new Set(node.edges)]})),owners};
 const run=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify(input))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as{owners:{name:string;digest:string;names:string[];tools:string[];keys:string[]}[];digests:Record<string,string>};
};
const recordOf=(node:Node,nodes:Map<string,Node>)=>({key:node.key,name:node.name,kind:node.kind,digest:moduleNodeDigestV1(content(node,nodes)),edges:node.edges});
const build=(nodes:Map<string,Node>)=>{const graph=createClosureGraphV1();graph.update([...nodes.values()].map(node=>recordOf(node,nodes)));return graph;};
const digestsOf=(graph:ReturnType<typeof createClosureGraphV1>,owners:Owner[])=>owners.map(owner=>graph.closureDigest(owner.roots,{stops:new Map(Object.entries(owner.stops))}));
const byCodeUnit=(list:string[])=>[...list].sort();

test("every owner's closure digest, reach and node digests equal the Python oracle, through cycles, resources and tool cuts",()=>{
 for(const seed of[3,17,91]){
  const{nodes,owners}=fixture(seed),graph=build(nodes),model=ask(nodes,owners);
  for(const node of nodes.values())expect(recordOf(node,nodes).digest).toBe(model.digests[node.key]!);
  owners.forEach((owner,at)=>{
   const stops=new Map(Object.entries(owner.stops));
   expect(graph.closureDigest(owner.roots,{stops})).toBe(model.owners[at]!.digest);
   const reach=graph.reach(owner.roots,{stops});
   expect(byCodeUnit(reach.names)).toEqual(byCodeUnit(model.owners[at]!.names));
   expect(reach.tools).toEqual(model.owners[at]!.tools);
   expect(byCodeUnit(reach.keys)).toEqual(byCodeUnit(model.owners[at]!.keys));
  });
  expect(graph.stats().edges).toBe([...nodes.values()].reduce((sum,node)=>sum+new Set(node.edges).size,0));
 }
});

test("an edit of one node moves exactly the owners that reach it; a node inside a tool moves no owner cut at the tool, only the tool's own closure; an unchanged digest moves nothing",()=>{
 const{nodes,owners,tools}=fixture(29),graph=build(nodes),draw=random(5);
 for(let round=0;round<12;round++){
  const before=digestsOf(graph,owners),reached=ask(nodes,owners).owners.map(owner=>new Set(owner.keys));
  const at=round===0?Number(tools[1]!.slice(1))+1:Math.floor(draw()*nodes.size),node=nodes.get("k"+at)!;
  if(round%3===0)node.body+="-edited";else node.edges.push("k"+Math.floor(draw()*nodes.size));
  graph.update([recordOf(node,nodes)]);
  const after=digestsOf(graph,owners),moved=owners.filter((_,index)=>before[index]!==after[index]).map(owner=>owner.name),expected=owners.filter((_,index)=>reached[index]!.has(node.key)).map(owner=>owner.name);
  expect(moved).toEqual(expected);
  const model=ask(nodes,owners);owners.forEach((_,index)=>expect(after[index]).toBe(model.owners[index]!.digest));
 }
 const same=[...nodes.values()][7]!,before=digestsOf(graph,owners);
 expect(graph.update([recordOf(same,nodes)]).forgotten).toBe(0);
 expect(digestsOf(graph,owners)).toEqual(before);
});

test("a node inside a declared tool is cut: its edit moves the tool's own closure and no owner that stops at the tool",()=>{
 const nodes=new Map<string,Node>(),add=(key:string,edges:string[])=>nodes.set(key,{key,name:key+".ts",kind:"module",edges,body:key});
 add("owner",["shared","tool"]);add("tool",["inner"]);add("inner",["deep"]);add("deep",["inner"]);add("shared",[]);
 const graph=build(nodes),stops=new Map([["tool","t"]]),owner=()=>graph.closureDigest(["owner"],{stops}),toolOwn=()=>graph.closureDigest(["tool"],{stops}),whole=()=>graph.closureDigest(["owner"]);
 const before=[owner(),toolOwn(),whole()];
 expect(graph.reach(["owner"],{stops})).toEqual({names:["owner.ts","shared.ts"],tools:["t"],keys:["owner","shared"]});
 nodes.get("deep")!.body="moved";graph.update([recordOf(nodes.get("deep")!,nodes)]);
 expect(owner()).toBe(before[0]!);expect(toolOwn()).not.toBe(before[1]!);expect(whole()).not.toBe(before[2]!);
 const model=ask(nodes,[{name:"o",roots:["owner"],stops:{tool:"t"}},{name:"t",roots:["tool"],stops:{tool:"t"}},{name:"w",roots:["owner"],stops:{}}]);
 expect([owner(),toolOwn(),whole()]).toEqual(model.owners.map(row=>row.digest));
 graph.update([],["inner"]);
 expect(toolOwn()).toBeUndefined();expect(owner()).toBe(before[0]!);expect(graph.missing(["tool"])).toEqual(["inner"]);
});

test("a router read with selected command words is another node: a redirected question reaches only the selected commands, equal to the oracle, and the whole question is unchanged",()=>{
 const nodes=new Map<string,Node>(),add=(key:string,edges:string[])=>nodes.set(key,{key,name:key.split("#")[0]+".ts",kind:"module",edges,body:key});
 add("entry",["router","util"]);add("router",["cmd-a","cmd-b"]);add("router#a",["cmd-a"]);add("cmd-a",["util"]);add("cmd-b",["heavy"]);add("heavy",["cmd-b"]);add("util",["router"]);
 const graph=build(nodes),redirect=new Map([["router","router#a"]]),owners=[{name:"selected",roots:["entry"],stops:{},redirect:Object.fromEntries(redirect)},{name:"whole",roots:["entry"],stops:{}}] as(Owner&{redirect?:Record<string,string>})[];
 const model=ask(nodes,owners);
 expect(graph.closureDigest(["entry"],{redirect})).toBe(model.owners[0]!.digest);expect(graph.closureDigest(["entry"])).toBe(model.owners[1]!.digest);
 expect(graph.reach(["entry"],{redirect}).names).toEqual(["cmd-a.ts","entry.ts","router.ts","util.ts"]);
 const whole=graph.closureDigest(["entry"]);nodes.get("heavy")!.body="moved";graph.update([recordOf(nodes.get("heavy")!,nodes)]);
 expect(graph.closureDigest(["entry"],{redirect})).toBe(model.owners[0]!.digest);expect(graph.closureDigest(["entry"])).not.toBe(whole);
});

test("incremental updates over 40 random edits and removals equal a graph built from scratch and the oracle",()=>{
 const{nodes,owners}=fixture(47),graph=build(nodes),draw=random(11);
 for(const owner of owners)graph.closureDigest(owner.roots,{stops:new Map(Object.entries(owner.stops))});
 const roots=new Set(owners.flatMap(owner=>owner.roots));
 for(let round=0;round<40;round++){
  const node=nodes.get("k"+Math.floor(draw()*nodes.size))!,choice=draw();
  if(choice<0.3)node.body+=round;else if(choice<0.6)node.edges.push("k"+Math.floor(draw()*nodes.size));else if(choice<0.85)node.edges=node.edges.slice(1);
  else if(!roots.has(node.key)&&node.edges.length===0){const fresh="n"+round;nodes.set(fresh,{key:fresh,name:"src/new"+round+".ts",kind:"module",edges:[node.key],body:""});node.edges.push(fresh);nodes.get(fresh)!.edges=[];graph.update([recordOf(nodes.get(fresh)!,nodes),recordOf(node,nodes)]);continue;}
  graph.update([recordOf(node,nodes)]);
  if(round%8===0)digestsOf(graph,owners);
 }
 const incremental=digestsOf(graph,owners),scratch=digestsOf(build(nodes),owners),model=ask(nodes,owners).owners.map(owner=>owner.digest);
 expect(incremental).toEqual(scratch);expect(incremental).toEqual(model);
});

test("the store round-trips, rewrites only the edited bucket, keeps the previous head whole when a save stops, and refuses a damaged chunk",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"closure-graph-")));
 try{
  const{nodes,owners}=fixture(61),graph=build(nodes),before=digestsOf(graph,owners),steps:number[]=[],policy={...CLOSURE_GRAPH_POLICY_V1,chunkGraceMilliseconds:0};
  const first=await saveClosureGraphV1(graph,directory,{policy,progress:({done})=>{steps.push(done);}});
  expect(first.nodes).toBe(nodes.size);expect(steps.at(-1)).toBe(256);expect(first.written).toBe(first.chunks);
  expect(digestsOf((await loadClosureGraphV1(directory))!,owners)).toEqual(before);
  const node=nodes.get("k3")!;node.edges.push("k300");graph.update([recordOf(node,nodes)]);
  const edited=digestsOf(graph,owners),second=await saveClosureGraphV1(graph,directory,{policy});
  expect(second.written).toBe(1);expect(second.chunks).toBe(first.chunks);expect(readdirSync(join(directory,"c")).length).toBe(first.chunks);
  expect(digestsOf((await loadClosureGraphV1(directory))!,owners)).toEqual(edited);
  const other=nodes.get("k5")!;other.body="again";graph.update([recordOf(other,nodes)]);
  const control=new AbortController();
  await expect(saveClosureGraphV1(graph,directory,{policy,signal:control.signal,progress:({done})=>{if(done===200)control.abort();}})).rejects.toThrow();
  expect(digestsOf((await loadClosureGraphV1(directory))!,owners)).toEqual(edited);
  await saveClosureGraphV1(graph,directory,{policy});
  const loaded=(await loadClosureGraphV1(directory))!;expect(digestsOf(loaded,owners)).toEqual(digestsOf(graph,owners));expect(loaded.get("k3")!.edges).toEqual(graph.get("k3")!.edges);
  const name=readdirSync(join(directory,"c"))[0]!;writeFileSync(join(directory,"c",name),Buffer.from("damaged"));
  await expect(loadClosureGraphV1(directory)).rejects.toThrow(/graph-invalid/);
  expect(await loadClosureGraphV1(join(directory,"absent"))).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("processes of one machine save into one directory: a reader of the replaced head still finds its chunks within the grace, the chunks no head names go after it, and the synchronous load equals the asynchronous one",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"closure-graph-shared-")));
 try{
  const{nodes,owners}=fixture(62),graph=build(nodes),first=await saveClosureGraphV1(graph,directory),headBefore=readdirSync(join(directory,"c")).sort();
  expect(first.head).toMatch(/^[0-9a-f]{64}$/u);expect(loadClosureGraphSyncV1(directory)!.head).toBe(first.head);
  const reader=JSON.parse(await Bun.file(join(directory,"head.json")).text()) as{chunks:(string|null)[]};
  const node=nodes.get("k7")!;node.edges.push("k301");graph.update([recordOf(node,nodes)]);
  const second=await saveClosureGraphV1(graph,directory);
  expect(second.head).not.toBe(first.head);expect(readdirSync(join(directory,"c")).length).toBe(headBefore.length+1);
  for(const digest of reader.chunks)if(digest!==null)expect(readdirSync(join(directory,"c"))).toContain(digest+".json.br");
  const named=new Set((JSON.parse(readFileSync(join(directory,"head.json"),"utf8")).chunks as(string|null)[]).filter(Boolean).map(digest=>digest+".json.br")),stale=readdirSync(join(directory,"c")).filter(name=>!named.has(name));
  expect(stale.length).toBe(1);expect(headBefore).toContain(stale[0]);
  const past=new Date(Date.now()-CLOSURE_GRAPH_POLICY_V1.chunkGraceMilliseconds-60000);utimesSync(join(directory,"c",stale[0]!),past,past);
  await saveClosureGraphV1(graph,directory);
  expect(readdirSync(join(directory,"c"))).not.toContain(stale[0]);expect(readdirSync(join(directory,"c")).length).toBe(second.chunks);
  const sync=loadClosureGraphSyncV1(directory)!,async_=(await loadClosureGraphV1(directory))!;
  expect(sync.head).toBe(async_.head);expect(digestsOf(sync,owners)).toEqual(digestsOf(async_,owners));expect(digestsOf(sync,owners)).toEqual(digestsOf(graph,owners));
  expect(loadClosureGraphSyncV1(join(directory,"absent"))).toBeUndefined();
  const name=[...named][0]!;writeFileSync(join(directory,"c",name),Buffer.from("damaged"));
  expect(()=>loadClosureGraphSyncV1(directory)).toThrow(/graph-invalid/);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("one pass over 30 thousand modules answers 5 thousand owner closure digests in bounded memory",()=>{
 const draw=random(77),size=30000,graph=createClosureGraphV1(),records=[];
 for(let at=0;at<size;at++){const edges=[];for(let edge=0;edge<4;edge++){const target=at<1000?Math.floor(draw()*1000):Math.floor(draw()*at);if(target!==at)edges.push("k"+target);}records.push({key:"k"+at,name:"src/m"+at+".ts",kind:"module",digest:moduleNodeDigestV1({at}),edges});}
 const started=performance.now();graph.update(records);
 const answers=new Set<string|undefined>();for(let owner=0;owner<5000;owner++)answers.add(graph.closureDigest(["k"+(size-1-Math.floor(draw()*size/2))]));
 const elapsed=performance.now()-started,stats=graph.stats();
 expect(answers.has(undefined)).toBe(false);expect(stats.nodes).toBe(size);expect(stats.digests).toBeLessThanOrEqual(size);
 console.log("[closure-graph] 30k modules, 5k owners: "+Math.round(elapsed)+" ms, "+stats.digests+" component digests, heap "+Math.round(process.memoryUsage().heapUsed/1048576)+" MB");
});

test("a loaded graph holds one copy of each path its witnesses name: 200 stored nodes that read the same 2000 paths cost the arrays, not 400 000 strings",async()=>{
 const root=mkdtempSync(join(tmpdir(),"closure-heap-")),heap=()=>{Bun.gc(true);return process.memoryUsage().heapUsed/1048576;};
 try{
  const graph=createClosureGraphV1(),path=(at:number)=>"/tree/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/📦️packages/file-"+at+".ts";
  const witness=()=>({paths:Array.from({length:2000},(_,at)=>path(at)),tuples:Array.from({length:2000},()=>"1:2:3:4:5"),typed:Array.from({length:2000},(_,at)=>["exists",path(at),true]),code:[],live:[]});
  graph.update(Array.from({length:200},(_,at)=>({key:"k"+at,name:"src/m"+at+".ts",kind:"module",digest:at.toString(16).padStart(64,"0"),edges:[],extra:{witness:witness()}})));
  await saveClosureGraphV1(graph,root);
  const before=heap(),loaded=loadClosureGraphSyncV1(root),held=heap()-before;
  expect([...loaded.records()].length).toBe(200);
  const unshared=heap(),copies=Array.from({length:200},witness),reference=heap()-unshared;
  expect(copies.length).toBe(200);
  expect(held).toBeLessThan(reference*0.75);
 }finally{rmSync(root,{recursive:true,force:true});}
});
