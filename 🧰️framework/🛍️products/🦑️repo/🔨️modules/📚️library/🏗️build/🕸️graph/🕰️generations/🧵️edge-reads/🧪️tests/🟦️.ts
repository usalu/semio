/** 🧵️ Contract: the lookups the edges of an owner recorded say which owners keep their edges when the environment moves. The rules are written once as plain predicates in an independent Python model; the fixture holds hand-written situations with the answer they must give, and seeded random environments and lookups are answered by both implementations. The recorder, the stored records and the stored environment parts are closed. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createEdgeReadsRecorderV1,diffEnvironmentV1,edgeConsumersV1,readEdgeReadsV1,readEnvironmentPartsV1}from"../🟨️.mjs";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
type Parts={owners:[string,string,string|null][];packages:[string,string][];modules:[string,string][];workspaces:[string,string][];lock:string|null;taxonomy:string|null};
type Row={root:string;nativeRoot?:string;reads?:Record<string,unknown>};
const tableOf=(parts:Parts)=>new Map(parts.owners.map(row=>[row[0],[row[1],row[2]] as (string|null)[]]));
const own=(previous:Parts|null,current:Parts,rows:Record<string,Row>)=>[...edgeConsumersV1({diff:diffEnvironmentV1(previous??undefined,current),rows:new Map(Object.entries(rows)) as never,tables:[...(previous?[tableOf(previous)]:[]),tableOf(current)]})].sort();
const world=(previous:Parts|null,current:Parts,rows:Record<string,Row>)=>({previous,current,rows,tables:[...(previous?[previous]:[]),current].map(parts=>Object.fromEntries(parts.owners.map(row=>[row[0],[row[1],row[2]]])))});
const model=(worlds:ReturnType<typeof world>[])=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({worlds}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return (JSON.parse(run.stdout.toString()) as {answers:{all:boolean;affected:string[]}[]}).answers;
};

test("hand-written situations: the consumers equal the stated answer and the independent model",()=>{
 const answers=model(fixture.cases.map(row=>world(row.previous as Parts|null,row.current as Parts,row.rows as Record<string,Row>)));
 fixture.cases.forEach((row,at)=>{
  expect(own(row.previous as Parts|null,row.current as Parts,row.rows as Record<string,Row>),row.name).toEqual(row.expect);
  expect(answers[at]!.affected,row.name).toEqual(row.expect);
 });
});

const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
test("300 seeded environments and lookups: the consumers equal the independent model",()=>{
 let nonEmpty=0,partial=0;const worlds:ReturnType<typeof world>[]=[],mine:string[][]=[];
 for(let seed=0;seed<300;seed++){
  const random=seeded(99000+seed),pick=<T,>(list:readonly T[])=>list[Math.floor(random()*list.length)]!,segments=["a","b","c","d"];
  const path=()=>Array.from({length:1+Math.floor(random()*3)},()=>pick(segments)).join("/");
  const count=2+Math.floor(random()*9),taken=new Set<string>(),owners:[string,string,string|null][]=[];
  for(let at=0;at<count;at++){let root=path();while(taken.has(root))root+="/"+pick(segments)+at;taken.add(root);owners.push(["o"+at,root,random()<0.2?"native/"+root:null]);}
  const previous:Parts={owners,packages:owners.filter(()=>random()<0.5).map(row=>["@w/"+row[0],row[0]] as [string,string]),modules:owners.filter(()=>random()<0.2).map(row=>["m/"+row[0],row[0]] as [string,string]),workspaces:random()<0.5?[["Cargo.toml","d0"],[pick(segments)+"/Cargo.toml","d1"]]:[],lock:random()<0.5?"l0":null,taxonomy:"t0"};
  const current:Parts=structuredClone(previous);
  for(let edit=0;edit<1+Math.floor(random()*3);edit++){
   const kind=Math.floor(random()*9);
   if(kind===0){let root=path();while(current.owners.some(row=>row[1]===root))root+="/z";current.owners.push(["n"+edit,root,null]);}
   else if(kind===1&&current.owners.length>1)current.owners.splice(Math.floor(random()*current.owners.length),1);
   else if(kind===2){const row=pick(current.owners);row[1]=row[1]+"/moved";}
   else if(kind===3)current.packages.push(["@w/new"+edit,pick(current.owners)[0]]);
   else if(kind===4&&current.packages.length)current.packages[0]![1]="other";
   else if(kind===5)current.workspaces=[["Cargo.toml","d9"],...current.workspaces.slice(1)];
   else if(kind===6)current.lock="l1";
   else if(kind===7&&random()<0.1)current.taxonomy="t1";
   else if(kind===8)current.modules.push(["m/new"+edit,pick(current.owners)[0]]);
  }
  const rows:Record<string,Row>={};
  for(const row of previous.owners)if(random()<0.8){
   const reads:Record<string,unknown>={};
   if(random()<0.9){
    if(random()<0.6)reads.owners=[...new Set(Array.from({length:1+Math.floor(random()*3)},()=>pick([...previous.owners,...current.owners])[0]))].sort();
    if(random()<0.4)reads.paths=[...new Set(Array.from({length:1+Math.floor(random()*3)},()=>random()<0.2?".":path()))].sort();
    if(random()<0.4)reads.packages=[...new Set(Array.from({length:1+Math.floor(random()*2)},()=>"@w/"+pick(previous.owners)[0]))].sort();
    if(random()<0.2)reads.modules=["m/"+pick(previous.owners)[0]];
    if(random()<0.3)reads.workspaces=[...new Set([".",pick(segments)])].sort();
    if(random()<0.4)reads.lock=true;
   }else delete rows[row[0]];
   rows[row[0]]={root:row[1],...(row[2]?{nativeRoot:row[2]}:{}),...(random()<0.1?{}:{reads})};
  }
  worlds.push(world(previous,current,rows));mine.push(own(previous,current,rows));
  if(mine.at(-1)!.length)nonEmpty++;if(mine.at(-1)!.length&&mine.at(-1)!.length<Object.keys(rows).length)partial++;
 }
 model(worlds).forEach((answer,at)=>expect(mine[at],"seed "+at).toEqual(answer.affected));
 expect(nonEmpty).toBeGreaterThan(150);expect(partial).toBeGreaterThan(80);
});

test("the recorder keeps sorted unique lookups per owner and answers nothing for an owner it never saw",()=>{
 const recorder=createEdgeReadsRecorderV1(),handle=recorder.owner("a");
 handle.owner("z");handle.owner("b");handle.owner("b");handle.path("x/y");handle.package("@w/p");handle.module("m");handle.workspace(".");handle.lock();recorder.owner("quiet");
 expect(recorder.finish("a")).toEqual({owners:["b","z"],paths:["x/y"],packages:["@w/p"],modules:["m"],workspaces:["."],lock:true});
 expect(recorder.finish("quiet")).toEqual({});expect(recorder.finish("unseen")).toBeUndefined();
});

test("stored reads and environment parts are closed, sorted and bounded",()=>{
 expect(readEdgeReadsV1({owners:["a","b"],lock:true})).toEqual({owners:["a","b"],lock:true});
 for(const bad of[null,[],{unknown:["a"]},{owners:[]},{owners:["b","a"]},{owners:["a","a"]},{owners:[""]},{owners:[1]},{lock:false},{owners:["x".repeat(4097)]}])expect(()=>readEdgeReadsV1(bad)).toThrow();
 const good={packages:[["@w/a","a"]],modules:[],workspaces:[["Cargo.toml","d"]],lock:null,taxonomy:"t"};
 expect(readEnvironmentPartsV1(good)).toEqual(good);
 for(const bad of[null,{},{...good,owners:[]},{...good,packages:[["a"]]},{...good,lock:1},{...good,taxonomy:undefined}])expect(()=>readEnvironmentPartsV1(bad)).toThrow();
});
