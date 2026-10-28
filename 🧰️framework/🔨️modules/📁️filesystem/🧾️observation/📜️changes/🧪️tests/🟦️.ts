import {afterAll,beforeAll,expect,test} from "bun:test";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import vectors from "./🔣️.json";
import {
 CHANGE_JOURNAL_POLICY_V1,changeAffectsV1,changedWatchedPathsV1,changeHeadV1,changeRootExcludedV1,createChangeLogWriterV1,decodeChangeRecordV1,encodeChangeRecordV1,formatChangeCursorV1,listChangeSegmentsV1,mergeChangeRowsV1,parseChangeCursorV1,readChangeRecordsAfterV1,readChangeStateV1,renderChangeRefusalV1,writeChangeStateV1,
 type ChangeRefusalV1,type ChangeRowV1,type ChangeStateV1,
} from "../🟦️.ts";
import {createChangeJournalClientV1} from "../🧭️client/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),EPOCH="0123456789abcdef",policy=CHANGE_JOURNAL_POLICY_V1;
const python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"changes-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
let counter=0;
const made=():string=>{const directory=join(sandbox,"j"+(counter++));mkdirSync(directory,{recursive:true});return directory;};

test("the policy is closed under its schema and every refusal has the same placeholders in every language",()=>{
 const ajv=new Ajv({strict:false}),check=(name:string)=>ajv.compile({definitions:schema.definitions,$ref:"#/definitions/"+name});
 const refusals=schema.definitions.Refusal.enum as ChangeRefusalV1[];
 for(const language of schema.definitions.Language.enum){
  expect(Object.keys(policy.texts[language]!).sort()).toEqual([...refusals].sort());
 }
 for(const reason of refusals){
  const holders=(language:string)=>[...policy.texts[language]![reason]!.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(holders("de")).toEqual(holders("en"));
  expect(renderChangeRefusalV1({exact:false,reason,path:"p",detail:"d",count:3},"en").length).toBeGreaterThan(0);
  expect(renderChangeRefusalV1({exact:false,reason,path:"p",detail:"d",count:3},"de").length).toBeGreaterThan(0);
 }
 expect(()=>renderChangeRefusalV1({exact:false,reason:"gap"},"fr")).toThrow();
 expect(check("Record")(JSON.parse(encodeChangeRecordV1({seq:1,at:2,rows:[["m",0,"a/b"]]})))).toBe(true);
 expect(check("Record")(JSON.parse(encodeChangeRecordV1({seq:2,at:2,lost:["overflow",""]})))).toBe(true);
 expect(check("Record")({s:1,t:1,r:[],l:["x",""]})).toBe(false);
 expect(check("Path")("a/../b")).toBe(false);
 expect(policy.segmentBytes).toBeGreaterThan(0);expect(policy.heartbeatStaleMilliseconds).toBeGreaterThan(policy.heartbeatMilliseconds);
});

const line=(entry:number|string,at:number)=>typeof entry==="number"?encodeChangeRecordV1({seq:entry,at:1,rows:[["m",0,"f"+entry]]}):entry.startsWith("lost:")?encodeChangeRecordV1({seq:Number(entry.slice(5)),at:1,lost:["must-scan","d"]}):entry+"\n";
function materialise(directory:string,spec:typeof vectors.read[number]){
 for(const segment of spec.segments as {first:number;lines:(number|string)[];torn?:string;epoch?:string}[]){
  const body=segment.lines.map(entry=>line(entry,1)).join("")+(segment.torn??"");
  writeFileSync(join(directory,(segment.epoch??EPOCH)+"."+String(segment.first).padStart(16,"0")+".log"),body);
 }
}

test("language-agnostic vectors: record decoding, affects, merge, cursor text, exclusion and log reads are decided identically by this library and by the independent Python model",()=>{
 const reads=vectors.read.map(spec=>{const directory=made();materialise(directory,spec);return{spec,directory};});
 const state=(rooted:string[],names:string[]):ChangeStateV1=>({version:1,epoch:EPOCH,instance:"i",pid:1,startedAt:0,root:"/r",fence:".f",exclude:{rooted,names},backend:"fsevents",trust:"exact",ready:true});
 const exclusion=state([".git"],["node_modules"]);
 const input={
  decode:vectors.decode.map(row=>row.line),
  affects:vectors.affects.map(row=>({row:row.row,root:row.root,insensitive:row.insensitive})),
  merge:vectors.merge.map(row=>row.rows),
  cursor:vectors.cursor.map(row=>row.text),
  excluded:vectors.excluded.map(row=>({state:exclusion,root:row.root,insensitive:row.insensitive})),
  read:reads.map(row=>({directory:row.directory,epoch:EPOCH,after:row.spec.after})),
 };
 const file=join(sandbox,"input.json");writeFileSync(file,JSON.stringify(input));
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),"vectors",file],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout);
 vectors.decode.forEach((row,index)=>{expect(decodeChangeRecordV1(row.line)!==undefined).toBe(row.valid);expect(model.decode[index]).toBe(row.valid);});
 vectors.affects.forEach((row,index)=>{expect(changeAffectsV1(row.row as unknown as ChangeRowV1,row.root,row.insensitive)).toBe(row.expect);expect(model.affects[index]).toBe(row.expect);});
 vectors.merge.forEach((row,index)=>{expect(mergeChangeRowsV1(row.rows as unknown as ChangeRowV1[])).toEqual(row.expect as unknown as ChangeRowV1[]);expect(model.merge[index]).toEqual(row.expect);});
 vectors.cursor.forEach((row,index)=>{expect(parseChangeCursorV1(row.text)??null).toEqual(row.expect);expect(model.cursor[index]).toEqual(row.expect);});
 vectors.excluded.forEach((row,index)=>{expect(changeRootExcludedV1(exclusion,row.root,row.insensitive)).toBe(row.expect);expect(model.excluded[index]).toBe(row.expect);});
 reads.forEach(({spec,directory},index)=>{
  for(const chunk of[16,64,65536]){
   const result=readChangeRecordsAfterV1(directory,EPOCH,spec.after,chunk),expected=spec.expect as {ok:boolean;seqs?:number[];head?:number;reason?:string};
   expect({name:spec.name,ok:result.ok}).toEqual({name:spec.name,ok:expected.ok});
   if(result.ok){expect(result.records.map(record=>record.seq)).toEqual(expected.seqs!);expect(result.head).toBe(expected.head!);}else expect(result.reason as string).toBe(expected.reason!);
  }
  expect({name:spec.name,...model.read[index]}).toEqual({name:spec.name,...spec.expect});
 });
 expect(formatChangeCursorV1({epoch:EPOCH,seq:7})).toBe(EPOCH+":7");
});

test("the writer rotates at the declared size, keeps the declared number of segments, drops other epochs and turns oversized or unreadable batches into a lost record",()=>{
 const directory=made(),other="fedcba9876543210";
 writeFileSync(join(directory,other+".0000000000000001.log"),encodeChangeRecordV1({seq:1,at:1,rows:[]}));
 const small={...policy,segmentBytes:100,segmentsKept:3,maximumRowsPerRecord:4};
 const writer=createChangeLogWriterV1({directory,epoch:EPOCH,policy:small});
 expect(listChangeSegmentsV1(directory,other).length).toBe(0);
 for(let index=0;index<12;index++)writer.append([["m",0,"file/"+index]],index);
 expect(writer.seq()).toBe(12);
 const segments=listChangeSegmentsV1(directory,EPOCH);
 expect(segments.length).toBe(3);expect(segments[0]!.first).toBeGreaterThan(1);
 expect(changeHeadV1(directory,EPOCH)).toBe(12);
 const all=readChangeRecordsAfterV1(directory,EPOCH,segments[0]!.first-1);
 expect(all.ok&&all.records.length).toBe(12-segments[0]!.first+1);
 expect(readChangeRecordsAfterV1(directory,EPOCH,0)).toEqual({ok:false,reason:"floor"});
 expect(writer.append(Array.from({length:5},(_,index)=>["m",0,"x"+index] as ChangeRowV1),99)).toBe(13);
 expect(writer.append([["m",0,"/absolute"]],100)).toBe(14);
 writer.lost("must-scan","a",101);
 const tail=readChangeRecordsAfterV1(directory,EPOCH,12);
 expect(tail.ok&&tail.records.map(record=>record.lost)).toEqual([["overflow",""],["unreadable-row",""],["must-scan","a"]]);
 writer.close();
});

function service(options:{trust?:ChangeStateV1["trust"];exclude?:ChangeStateV1["exclude"]}={}){
 const directory=made(),root=join(sandbox,"w"+(counter++)),fence=".fence";
 mkdirSync(join(root,fence),{recursive:true});
 const writer=createChangeLogWriterV1({directory,epoch:EPOCH});
 const state:ChangeStateV1={version:1,epoch:EPOCH,instance:"t",pid:process.pid,startedAt:Date.now(),root,fence,exclude:options.exclude??{rooted:[".git"],names:["node_modules"]},backend:"fsevents",trust:options.trust??"exact",ready:true};
 writeChangeStateV1(directory,state);appendFileSync(join(directory,"heartbeat"),"");utimesSync(join(directory,"heartbeat"),new Date(),new Date());
 const seen=new Set<string>();
 let echo=true;
 const timer=setInterval(()=>{
  if(!echo)return;
  const rows:ChangeRowV1[]=[];
  for(const name of readdirSync(join(root,fence)))if(!seen.has(name)){seen.add(name);rows.push(["a",0,fence+"/"+name]);}
  if(rows.length)writer.append(rows,Date.now());
 },1);
 return{directory,root,writer,state,client:(extra:Record<string,unknown>={})=>createChangeJournalClientV1({directory,root,insensitive:false,environment:{},...extra}),stop:()=>{clearInterval(timer);writer.close();},silence:()=>{echo=false;}};
}

test("a client fences, then answers exactly the rows under its roots after its cursor, without fence rows, merged per path",async()=>{
 const s=service();
 try{
  const client=s.client(),first=await client.cursor();
  expect("epoch" in first).toBe(true);
  const cursor=first as {epoch:string;seq:number};
  s.writer.append([["m",0,"src/a.ts"],["m",0,"docs/x.md"]],1);
  s.writer.append([["a",0,"src/b.ts"],["m",0,"src/a.ts"]],2);
  const answer=await client.since(cursor,["src"]);
  expect(answer.exact).toBe(true);
  if(answer.exact){expect(answer.rows).toEqual([["m",0,"src/a.ts"],["a",0,"src/b.ts"]]);expect(answer.cursor.seq).toBeGreaterThan(cursor.seq+1);}
  const quiet=await client.since((answer as {cursor:{epoch:string;seq:number}}).cursor,["src","docs"]);
  expect(quiet.exact&&quiet.rows).toEqual([]);
  expect(existsSync(join(s.root,".fence"))&&readdirSync(join(s.root,".fence")).length).toBe(0);
 }finally{s.stop();}
});

test("a lost record under a root refuses, a lost record elsewhere does not, and a structural directory row above a root counts",async()=>{
 const s=service();
 try{
  const client=s.client(),cursor=await client.cursor() as {epoch:string;seq:number};
  s.writer.lost("must-scan","other/dir",1);
  expect((await client.since(cursor,["src"])).exact).toBe(true);
  const refused=await client.since(cursor,["other/dir/deep"]);
  expect(refused).toMatchObject({exact:false,reason:"lost",path:"other/dir"});
  s.writer.append([["r",1,"src"]],2);
  const above=await client.since(cursor,["src/deep/file.ts"]);
  expect(above.exact&&above.rows).toEqual([["r",1,"src"]]);
  s.writer.lost("watcher-exit","",3);
  expect(await client.since(cursor,["src"])).toMatchObject({exact:false,reason:"lost"});
 }finally{s.stop();}
});

test("every way the journal cannot vouch is a refusal: another epoch, excluded root, untrusted watcher, no fence record, stale heartbeat, gap, switch off",async()=>{
 const s=service();
 try{
  const client=s.client(),cursor=await client.cursor() as {epoch:string;seq:number};
  expect(await client.since({epoch:"fedcba9876543210",seq:1},["src"])).toMatchObject({exact:false,reason:"epoch"});
  expect(await client.since(cursor,["node_modules/x"])).toMatchObject({exact:false,reason:"excluded"});
  expect(await client.since({epoch:EPOCH,seq:cursor.seq+50},["src"])).toMatchObject({exact:false,reason:"gap"});
  expect(await s.client({environment:{SEMIO_REPO_CHANGES:"off"}}).since(cursor,["src"])).toMatchObject({exact:false,reason:"disabled"});
  await expect(s.client({environment:{SEMIO_REPO_CHANGES:"maybe"}}).since(cursor,["src"])).rejects.toThrow(/SEMIO_REPO_CHANGES/u);
  s.silence();
  const fast={...policy,fenceTimeoutMilliseconds:60};
  expect(await s.client({policy:fast}).since(cursor,["src"])).toMatchObject({exact:false,reason:"fence-timeout"});
  writeChangeStateV1(s.directory,{...s.state,trust:"verify",reason:"canary"});
  expect(await client.since(cursor,["src"])).toMatchObject({exact:false,reason:"untrusted",detail:"canary"});
  writeChangeStateV1(s.directory,{...s.state,ready:false});
  expect(await client.since(cursor,["src"])).toMatchObject({exact:false,reason:"starting"});
  writeChangeStateV1(s.directory,s.state);
  utimesSync(join(s.directory,"heartbeat"),new Date(Date.now()-60000),new Date(Date.now()-60000));
  expect(await client.since(cursor,["src"])).toMatchObject({exact:false,reason:"no-service"});
  const controller=new AbortController();controller.abort(new Error("stop"));
  await expect(client.since(cursor,["src"],controller.signal)).rejects.toThrow("stop");
 }finally{s.stop();}
});

test("a missing service is started on demand once and a recent failure to start is not retried",async()=>{
 const s=service();
 try{
  utimesSync(join(s.directory,"heartbeat"),new Date(Date.now()-60000),new Date(Date.now()-60000));
  let starts=0;
  const client=s.client({start:async()=>{starts++;utimesSync(join(s.directory,"heartbeat"),new Date(),new Date());}});
  expect("epoch" in await client.cursor()).toBe(true);
  expect(starts).toBe(1);
  utimesSync(join(s.directory,"heartbeat"),new Date(Date.now()-60000),new Date(Date.now()-60000));
  writeFileSync(join(s.directory,"unavailable.json"),JSON.stringify({at:Date.now(),reason:"no watcher"}));
  expect(await client.cursor()).toMatchObject({exact:false,reason:"no-service"});
  expect(starts).toBe(1);
 }finally{s.stop();}
});


test("many watched paths are matched against rows exactly like one root at a time (seeded random worlds, Python model)",()=>{
 let state=12345;
 const random=(limit:number)=>{state=(state*1103515245+12345)&0x7fffffff;return (state>>>8)%limit;};
 const names=["a","b","A","c.ts","d"],path=(depth:number)=>Array.from({length:1+random(depth)},()=>names[random(names.length)]).join("/");
 const worlds=Array.from({length:150},(_,index)=>({
  insensitive:index%2===0,
  watched:Array.from({length:1+random(12)},()=>path(4)),
  rows:Array.from({length:random(8)},()=>[["a","m","r","n"][random(4)],random(2),path(4)] as ChangeRowV1),
 }));
 const file=join(sandbox,"watched.json");writeFileSync(file,JSON.stringify({decode:[],affects:[],merge:[],cursor:[],excluded:[],read:[],watched:worlds}));
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),"vectors",file],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout).watched as string[][];
 let hits=0;
 worlds.forEach((world,index)=>{
  const oracle=[...new Set(world.watched.filter(watched=>world.rows.some(row=>changeAffectsV1(row,watched,world.insensitive))))].sort();
  const got=[...changedWatchedPathsV1(world.watched,world.rows,world.insensitive)].sort();
  expect({index,got}).toEqual({index,got:oracle});
  expect({index,model:model[index]}).toEqual({index,model:oracle});
  hits+=oracle.length;
 });
 expect(hits).toBeGreaterThan(50);
});

test("the memoised excluder of a client equals the exclusion rule on every path of seeded random worlds",()=>{
 const s=service({exclude:{rooted:["a/b",".git"],names:["node_modules","Target"]}});
 try{
  let state=99;
  const random=(limit:number)=>{state=(state*1103515245+12345)&0x7fffffff;return (state>>>8)%limit;};
  const parts=["a","b","c","node_modules","target","Target",".git","x.ts","é","é"];
  for(const insensitive of[false,true]){
   const excluded=s.client({insensitive}).excluder(),current=readChangeStateV1(s.directory)!;
   for(let index=0;index<400;index++){
    const path=Array.from({length:1+random(5)},()=>parts[random(parts.length)]).join("/");
    expect({path,insensitive,got:excluded(path)}).toEqual({path,insensitive,got:changeRootExcludedV1(current,path,insensitive)});
   }
  }
 }finally{s.stop();}
});
