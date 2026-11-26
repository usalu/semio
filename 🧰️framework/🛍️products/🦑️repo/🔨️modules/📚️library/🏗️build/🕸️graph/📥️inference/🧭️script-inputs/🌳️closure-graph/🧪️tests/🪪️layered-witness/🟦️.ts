/** 🪪️ Contract: a node witness is two layers, what the node read itself and a `base` that thousands of nodes share; the layered verdict equals the verdict of the one flat witness of all their rows (oracle), the graph stores each distinct base once whatever the number of nodes, a loaded graph holds one copy of it, and the bytes and the heap of a graph of many nodes grow with what the nodes read themselves, not with the base times the nodes. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
import{createClosureGraphV1,loadClosureGraphSyncV1,loadClosureGraphV1,moduleNodeDigestV1,saveClosureGraphV1}from"../../🟨️.mjs";
import{observationWitnessHoldsV1,observationWitnessV1}from"../../../../../../👁️observation/🟨️.mjs";

const later=()=>Date.now()+60_000;
const scratch=()=>realpathSync(mkdtempSync(join(tmpdir(),"layered-witness-")));
const record=(key:string,witness:Readonly<Record<string,unknown>>)=>({key,name:key,kind:"module",digest:moduleNodeDigestV1({key}),edges:[] as string[],extra:{witness,urls:[],sha256:null,provider:"p"}});

test("the layered verdict equals the verdict of the flat witness of the same rows under edits of either layer",async()=>{
 const root=scratch();
 try{
  const files=Array.from({length:6},(_,at)=>join(root,"f"+at+".txt"));files.forEach((file,at)=>writeFileSync(file,"x"+at));
  const rows=(names:string[])=>names.map(path=>({kind:"lstat",path,fact:"file"}));
  const own=rows(files.slice(0,2)),shared=rows(files.slice(2));
  const flat=observationWitnessV1([...own,...shared],{since:later()})!,ownWitness=observationWitnessV1(own,{since:later()})!,base=observationWitnessV1(shared,{since:later()})!;
  const layered={...ownWitness,base};
  expect(await observationWitnessHoldsV1(layered)).toBe(await observationWitnessHoldsV1(flat));
  expect(await observationWitnessHoldsV1(layered)).toBe(true);
  for(const[at,expected]of[[0,false],[5,false]]as const){
   const original=readFileSync(files[at]!,"utf8");rmSync(files[at]!);
   expect(await observationWitnessHoldsV1(layered)).toBe(expected);expect(await observationWitnessHoldsV1(flat)).toBe(expected);
   writeFileSync(files[at]!,original);
  }
  const verdicts=new Map<unknown,boolean>(),tuples=new Map<string,string>();
  expect(await observationWitnessHoldsV1(layered,{verdicts,tuples})).toBe(true);expect(verdicts.get(base)).toBe(true);
  rmSync(files[3]!);verdicts.set(base,false);
  expect(await observationWitnessHoldsV1({...ownWitness,base},{verdicts,tuples})).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a graph of many nodes stores and loads each distinct base once: bytes and heap are bound by what the nodes read themselves",async()=>{
 const directory=scratch(),nodes=400,sharedRows=6000;
 try{
  const base={paths:Array.from({length:1500},(_,at)=>"/base/path/"+at),tuples:Array.from({length:1500},(_,at)=>"1:"+at+":1:1:33188"),typed:Array.from({length:sharedRows},(_,at)=>["lstat","/base/typed/"+at,"file"]),code:[],live:[]};
  const other={...base,typed:base.typed.slice(0,10)};
  const make=(at:number)=>({paths:["/own/"+at],tuples:["1:"+at+":1:1:33188"],typed:[["lstat","/own/typed/"+at,"file"]],code:[],live:[],base:at%100===99?other:base});
  const graph=createClosureGraphV1();graph.update(Array.from({length:nodes},(_,at)=>record("n"+at,make(at))));
  const saved=await saveClosureGraphV1(graph,directory);
  expect(saved.pool).not.toBeNull();
  const files=readdirSync(join(directory,"c")),bytes=files.reduce((sum,name)=>sum+statSync(join(directory,"c",name)).size,0),flatBytes=nodes*sharedRows*40;
  expect(bytes).toBeLessThan(flatBytes/50);
  const pool=JSON.parse(brotliDecompressSync(readFileSync(join(directory,"c",saved.pool+".json.br"))).toString());
  expect(pool.bases).toHaveLength(2);
  Bun.gc(true);const before=process.memoryUsage().heapUsed;
  const loaded=loadClosureGraphSyncV1(directory)!;Bun.gc(true);const grown=process.memoryUsage().heapUsed-before;
  expect(grown).toBeLessThan(flatBytes/20);
  const a=loaded.get("n0")!.extra!.witness!,b=loaded.get("n1")!.extra!.witness!,c=loaded.get("n99")!.extra!.witness!;
  expect(a.base).toBe(b.base);expect(c.base).not.toBe(a.base);
  for(let at=0;at<nodes;at++)expect(JSON.parse(JSON.stringify(loaded.get("n"+at)!.extra!.witness!))).toEqual(JSON.parse(JSON.stringify(make(at))));
  const again=(await loadClosureGraphV1(directory))!;expect(again.get("n5")!.extra!.witness!.base).toBe(again.get("n6")!.extra!.witness!.base);
  expect((await saveClosureGraphV1(loaded,directory)).written).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a head without its pool field, or naming a pool that is missing or altered, refuses the graph",async()=>{
 const directory=scratch();
 try{
  const graph=createClosureGraphV1();graph.update([record("n0",{paths:[],tuples:[],typed:[],code:[],live:[],base:{paths:[],tuples:[],typed:[["lstat","/a","file"]],code:[],live:[]}})]);
  const saved=await saveClosureGraphV1(graph,directory);
  const head=JSON.parse(readFileSync(join(directory,"head.json"),"utf8"));
  writeFileSync(join(directory,"head.json"),JSON.stringify({version:1,chunks:head.chunks,nodes:head.nodes,edges:head.edges}));
  expect(()=>loadClosureGraphSyncV1(directory)).toThrow();
  writeFileSync(join(directory,"head.json"),JSON.stringify(head));expect(loadClosureGraphSyncV1(directory)).toBeDefined();
  rmSync(join(directory,"c",saved.pool+".json.br"));expect(()=>loadClosureGraphSyncV1(directory)).toThrow();
  mkdirSync(join(directory,"c"),{recursive:true});writeFileSync(join(directory,"c",saved.pool+".json.br"),"garbage");expect(()=>loadClosureGraphSyncV1(directory)).toThrow();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
