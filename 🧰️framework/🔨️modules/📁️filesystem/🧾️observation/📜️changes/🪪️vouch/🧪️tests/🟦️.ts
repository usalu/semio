import {afterAll,beforeAll,expect,test} from "bun:test";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,renameSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import {formatChangeCursorV1,mergeChangeRowsV1,type ChangeCursorV1,type ChangeRowV1} from "../../🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../🧭️client/🟦️.ts";
import {VOUCH_POLICY_V1,openVouchV1,provenUnchangedV1,readVouchRecordV1,vouchFileV1,writeVouchRecordV1} from "../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),EPOCH="0123456789abcdef";
const python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"vouch-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
let counter=0;
const made=():string=>{const directory=join(sandbox,"d"+(counter++));mkdirSync(directory,{recursive:true});return directory;};

type Vector={paths:string[];rows:ChangeRowV1[];excluded:string[];linked:string[];insensitive:boolean};
const vectors:Vector[]=[
 {paths:["a","a/b","a/b/c.ts","d","d/e.ts"],rows:[["m",0,"a/b/c.ts"]],excluded:[],linked:[],insensitive:false},
 {paths:["a","a/b","a/b/c.ts","d","d/e.ts"],rows:[["m",1,"a/b"]],excluded:[],linked:[],insensitive:false},
 {paths:["a","a/b","a/b/c.ts","d","d/e.ts"],rows:[["r",1,"a"]],excluded:[],linked:[],insensitive:false},
 {paths:["a","a/b","a/b/c.ts","d","d/e.ts"],rows:[["a",0,"d/new.ts"]],excluded:[],linked:[],insensitive:false},
 {paths:["a/B","a/b","A/b/c"],rows:[["m",0,"a/b"]],excluded:[],linked:[],insensitive:true},
 {paths:["a/B","a/b","A/b/c"],rows:[["m",0,"a/b"]],excluded:[],linked:[],insensitive:false},
 {paths:["",".","/abs","../up","a\\b","x","x/y","x/y/z"],rows:[],excluded:["x/y"],linked:["x"],insensitive:false},
 {paths:["p","p/q","p/q/r","s"],rows:[["n",0,"p/q"],["n",0,"s"]],excluded:[],linked:[],insensitive:false},
 {paths:["caf\u00e9/x","cafe\u0301/x"],rows:[["m",0,"caf\u00e9/x"]],excluded:[],linked:[],insensitive:true},
];
function random(seed:number):Vector{
 let state=seed;const next=(limit:number)=>{state=(state*1103515245+12345)&0x7fffffff;return state%limit;};
 const names=["a","b","A","c"],path=()=>Array.from({length:1+next(3)},()=>names[next(names.length)]).join("/");
 return{paths:Array.from({length:12},path),rows:Array.from({length:next(5)},()=>[(["a","m","r","n"] as const)[next(4)]!,next(2) as 0|1,path()] as ChangeRowV1),excluded:next(3)?[]:[path()],linked:next(4)?[]:[path()],insensitive:next(2)===1};
}

test("the record is closed under its schema and its policy is the one the module uses",()=>{
 const ajv=new Ajv({strict:false}),check=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Record"});
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4",linked:[]})).toBe(true);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4",linked:["alias","a/b"]})).toBe(true);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4"}),"a record without its link list").toBe(false);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4",linked:[1]})).toBe(false);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:"nope",linked:[]})).toBe(false);
 expect(check({version:1,consumer:"x",key:"k",cursor:EPOCH+":4",linked:[],extra:1})).toBe(false);
 expect(VOUCH_POLICY_V1 as unknown).toEqual(schema.definitions.Policy.const as unknown);
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Mode"},"journal")).toBe(true);
});

test("language-agnostic vectors: the proven set equals the independent Python model on fixed and 400 seeded random vectors",()=>{
 const all=[...vectors,...Array.from({length:400},(_,index)=>random(index+1))],file=join(made(),"vectors.json");
 writeFileSync(file,JSON.stringify(all));
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),file],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const model=JSON.parse(run.stdout) as string[][];
 all.forEach((vector,index)=>{
  const own=[...provenUnchangedV1(vector.paths,vector.rows,path=>vector.excluded.includes(path),vector.insensitive,vector.linked)].sort();
  expect(own,JSON.stringify(vector)).toEqual(model[index]!);
 });
 expect(model.some(row=>row.length>0)).toBe(true);expect(model.some(row=>row.length===0)).toBe(true);
});

const stamp=(path:string):string=>{
 try{const facts=lstatSync(path,{bigint:true});return[facts.isDirectory()?"d:"+readdirSync(path).sort().join(","):"f",facts.ino,facts.mode,facts.size,facts.mtimeNs,facts.ctimeNs].join("|");}catch{return"absent";}
};
test("soundness against the file system: after random edits everything the vouch proves has the same stat identity and listing",()=>{
 let proven=0,total=0;
 for(let seed=1;seed<=40;seed++){
  const root=made();let state=seed*7919;const next=(limit:number)=>{state=(state*1103515245+12345)&0x7fffffff;return state%limit;};
  const dirs=["a","a/b","a/b/c","d","d/e"],files:string[]=[];
  for(const directory of dirs)mkdirSync(join(root,directory),{recursive:true});
  for(const directory of dirs)for(let index=0;index<3;index++){const file=directory+"/f"+index+".ts";writeFileSync(join(root,file),"x"+index);files.push(file);}
  const watched=new Set<string>([...dirs,...files]),before=new Map([...watched].map(path=>[path,stamp(join(root,path))])),rows:ChangeRowV1[]=[];
  const later=new Date(Date.now()+5000);
  for(let step=0;step<1+next(6);step++){
   const file=files[next(files.length)]!,directory=dirs[next(dirs.length)]!,fresh=directory+"/n"+seed+"_"+step+".ts";
   const operation=next(7);
   if(operation===0){writeFileSync(join(root,file),"y"+step);rows.push(["m",0,file]);}
   else if(operation===1){writeFileSync(join(root,fresh),"n");watched.add(fresh);rows.push(["a",0,fresh]);}
   else if(operation===2){rmSync(join(root,file),{force:true});rows.push(["r",0,file]);}
   else if(operation===3){const target=directory+"/m"+seed+"_"+step+".ts";try{renameSync(join(root,file),join(root,target));watched.add(target);rows.push(["n",0,file],["n",0,target]);}catch{}}
   else if(operation===4){chmodSync(join(root,file),0o600+next(2)*0o100);rows.push(["m",0,file]);}
   else if(operation===5){try{utimesSync(join(root,file),later,later);rows.push(["m",0,file]);}catch{}}
   else{const created=directory+"/sub"+seed+"_"+step;mkdirSync(join(root,created),{recursive:true});watched.add(created);rows.push(["a",1,created]);}
  }
  const result=provenUnchangedV1(watched,mergeChangeRowsV1(rows),()=>false,false);
  for(const path of result){total++;expect(stamp(join(root,path)),"seed "+seed+" "+path).toBe(before.get(path)??"absent");}
  proven+=result.size;
 }
 expect(proven).toBeGreaterThan(0);expect(proven).toBeLessThan(total*40);
});

function fakeJournal(){
 const log:ChangeRowV1[][]=[],state={epoch:EPOCH,refuse:undefined as undefined|"epoch"|"gap"|"no-service",excludedNames:["node_modules"],calls:0};
 const cursor=():ChangeCursorV1=>({epoch:state.epoch,seq:log.length});
 const client:ChangeJournalClientV1={
  excluder:()=>path=>state.excludedNames.some(name=>path.split("/").includes(name)),
  async cursor(){state.calls++;return state.refuse==="no-service"?{exact:false,reason:"no-service"}:cursor();},
  async since(from):Promise<ChangeAnswerV1>{
   state.calls++;
   if(state.refuse==="no-service")return{exact:false,reason:"no-service"};
   if(from.epoch!==state.epoch||state.refuse==="epoch")return{exact:false,reason:"epoch"};
   if(state.refuse==="gap")return{exact:false,reason:"gap"};
   return{exact:true,cursor:cursor(),rows:mergeChangeRowsV1(log.slice(from.seq).flat())};
  },
 };
 return{client,state,change:(...rows:ChangeRowV1[])=>{log.push(rows);}};
}
const paths=["a","a/x.ts","b","b/y.ts","node_modules/p/i.js",""];

test("lifecycle: first look takes a cursor and proves nothing, a second look of the same pass is answered by the journal, a sealed cursor lets the next run skip",async()=>{
 const directory=made(),journal=fakeJournal();
 const first=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k1",insensitive:false});
 expect(first.mode).toBe("cursor");expect([...first.skippable(paths)]).toEqual([]);
 journal.change(["m",0,"a/x.ts"]);
 expect([...await first.settled(paths)].sort()).toEqual(["b","b/y.ts"]);
 await first.seal();
 expect(readVouchRecordV1(directory,"c","k1")).toEqual({epoch:EPOCH,seq:1});
 journal.change(["m",0,"b/y.ts"]);
 const second=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k1",insensitive:false});
 expect(second.mode).toBe("journal");
 expect([...second.skippable(paths)].sort()).toEqual(["a","a/x.ts"]);
 journal.change(["a",0,"a/new.ts"]);
 expect([...await second.settled(paths)]).toEqual(["a/x.ts"]);
 await second.seal();
 expect(readVouchRecordV1(directory,"c","k1")?.seq).toBe(3);
 const other=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k2",insensitive:false});
 expect(other.mode).toBe("cursor");expect([...other.skippable(paths)]).toEqual([]);
});

test("every refusal of the journal leaves a walk: no skips, no settled paths, nothing sealed",async()=>{
 for(const refuse of["epoch","gap","no-service"] as const){
  const directory=made(),journal=fakeJournal();
  writeVouchRecordV1(directory,"c","k",{epoch:EPOCH,seq:0});
  journal.state.refuse=refuse;
  const vouch=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k",insensitive:false});
  expect(vouch.mode,refuse).toBe(refuse==="no-service"?"walk":"cursor");
  if(refuse!=="no-service")expect(vouch.refusal).toBe(refuse);
  expect([...vouch.skippable(paths)],refuse).toEqual([]);
  const kept=readVouchRecordV1(directory,"c","k");
  expect([...await vouch.settled(paths)],refuse).toEqual([]);
  await vouch.seal();
  expect(readVouchRecordV1(directory,"c","k")).toEqual(kept);
 }
});

test("a restarted journal (new epoch) makes the stored cursor useless and sealing never moves a cursor backwards",async()=>{
 const directory=made(),journal=fakeJournal();
 writeVouchRecordV1(directory,"c","k",{epoch:EPOCH,seq:5});writeVouchRecordV1(directory,"c","k",{epoch:EPOCH,seq:2});
 expect(readVouchRecordV1(directory,"c","k")).toEqual({epoch:EPOCH,seq:5});
 writeVouchRecordV1(directory,"c","k",{epoch:"fedcba9876543210",seq:1});
 expect(readVouchRecordV1(directory,"c","k")).toEqual({epoch:"fedcba9876543210",seq:1});
 const vouch=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k",insensitive:false});
 expect(vouch.mode).toBe("cursor");
 expect(formatChangeCursorV1({epoch:EPOCH,seq:1})).toBe(EPOCH+":1");
 expect(readVouchRecordV1(directory,"other","k")).toBeUndefined();
 expect(vouchFileV1("a")).not.toBe(vouchFileV1("b"));
});

test("an unreadable, torn or foreign record is no record, and a failing journal never fails the caller",async()=>{
 const directory=made(),journal=fakeJournal();
 writeFileSync(join(directory,vouchFileV1("c")),"{torn");
 expect(readVouchRecordV1(directory,"c","k")).toBeUndefined();
 writeFileSync(join(directory,vouchFileV1("c")),JSON.stringify({version:2,consumer:"c",key:"k",cursor:EPOCH+":1"}));
 expect(readVouchRecordV1(directory,"c","k")).toBeUndefined();
 const broken:ChangeJournalClientV1={...journal.client,async cursor(){throw Error("socket");},async since(){throw Error("socket");}};
 const vouch=await openVouchV1({client:broken,directory,consumer:"c",key:"k",insensitive:false});
 expect(vouch.mode).toBe("walk");expect(vouch.refusal).toBe("error");expect([...vouch.skippable(paths)]).toEqual([]);
 const aborted=new AbortController();aborted.abort();
 await expect(openVouchV1({client:{...journal.client,async cursor(signal){signal?.throwIfAborted();return{epoch:EPOCH,seq:0};}},directory,consumer:"c",key:"k",insensitive:false,signal:aborted.signal})).rejects.toBeDefined();
});
