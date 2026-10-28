/** 🗝️ Contract: shaping a partial inference claims, for every owner, the facts below its roots that its own record does not hold, in the order of the capture and without a duplicate; it equals the independent Python model (🐍️oracle) on seeded corpora, and it builds NO set of keys per owner list: the cold build shapes thousands of owners whose records share one long block, a key set per list was alive until the end of the call (4.4 GB of heap on the sealed generation of 4,019 owners, 3.7 x the time), so the law bounds the insertions into sets by the rows of the capture, whatever the owner count. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {shapeComputeResultV1} from "../../🟨️.mjs";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

type Row={kind:string;path:string;fact:unknown};
const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle","🐍️.py");
const random=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};

/** 🧪️ A corpus: a table of rows (equal rows appear under several numbers, the shaping must tell them apart by key), the capture, owners with the numbers of the rows of their own record, edge-only owners with stubs and the implementation rows. */
function corpus(seed:number){
 const next=random(seed),pick=<T>(list:T[])=>list[Math.floor(next()*list.length)]!,roots=Array.from({length:2+Math.floor(next()*12)},(_,i)=>(next()<0.3?"top/":"")+"a"+i+(next()<0.4?"/sub"+i:"")),table:Row[]=[];
 const add=(row:Row)=>{table.push(row);return table.length-1;};
 const mint=(root:string|undefined)=>add({kind:pick(["file","exists","analysis","lstat","resolve"]),path:root===undefined?(next()<0.3?"bare:"+Math.floor(next()*50):"/w/other/g"+Math.floor(next()*40)):"/w/"+root+"/f"+Math.floor(next()*30),fact:next()<0.5?"v"+Math.floor(next()*4):{size:Math.floor(next()*4),sha256:"h"+Math.floor(next()*3)}});
 const capture:number[]=[];
 for(let n=Math.floor(next()*160);n>0;n--)capture.push(mint(next()<0.2?undefined:pick(roots)));
 for(let n=Math.floor(next()*30);n>0&&capture.length;n--){const source=table[pick(capture)]!;capture.push(add({kind:source.kind,path:source.path,fact:JSON.parse(JSON.stringify(source.fact))}));}
 for(let at=capture.length-1;at>0;at--){const swap=Math.floor(next()*(at+1));[capture[at],capture[swap]]=[capture[swap]!,capture[at]!];}
 const owners=[{name:"root",root:".",nodeRows:[] as number[]}] as {name:string;root:string;nativeRoot?:string;nodeRows:number[]}[];
 for(const[index,root]of roots.entries()){
  const own=capture.filter(uid=>next()<0.5),fresh=next()<0.4&&own.length?[add({...table[pick(own)]!,fact:JSON.parse(JSON.stringify(table[pick(own)]!.fact))})]:[];
  owners.push({name:"o"+index,root,...(next()<0.3?{nativeRoot:pick(roots)}:{}),nodeRows:[...own,...fresh,...(next()<0.2?own.slice(0,3):[])]});
 }
 const stubs:Record<string,{root:string;nativeRoot?:string}>={},edgeOnly=Array.from({length:Math.floor(next()*4)},(_,i)=>{const name="e"+i;stubs[name]={root:pick(roots),...(next()<0.3?{nativeRoot:pick(roots)}:{})};return{name};});
 return{workspaceRoot:"/w",table,part:{owners,edgeOnly,rows:capture},stubs,implementation:{all:Array.from({length:Math.floor(next()*3)},()=>mint(undefined)),environment:Array.from({length:Math.floor(next()*3)},()=>mint(undefined))},env:Array.from({length:Math.floor(next()*3)},()=>mint(undefined))};
}

/** 🏗️ Builds the real part of a corpus (rows are objects, shared by number) and the map from object to number, so the shaping's answer can be read back as numbers. */
function shape(c:ReturnType<typeof corpus>){
 const objects=c.table.map(row=>({...row})),uid=new Map<object,number>(objects.map((row,at)=>[row,at])),rows=(list:number[])=>list.map(at=>objects[at]!);
 const part={owners:c.part.owners.map(owner=>({name:owner.name,root:owner.root,...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),node:{targets:{}},nodeRows:rows(owner.nodeRows)})),edgeOnly:c.part.edgeOnly,edges:[],rows:rows(c.part.rows),env:{digest:"x",rows:rows(c.env)},externalNodes:{},parseUnavailable:[],replaceTags:[]};
 const shaped=shapeComputeResultV1({workspaceRoot:c.workspaceRoot,part:part as never,stubs:c.stubs as never,implementation:{all:rows(c.implementation.all),environment:rows(c.implementation.environment)} as never});
 const numbers=(list:readonly object[])=>list.map(row=>uid.get(row)!);
 return{owners:Object.fromEntries(shaped.owners.map(owner=>[owner.name,numbers(owner.rows as never)])),environment:numbers(shaped.env.rows as never)};
}

test("the fixture is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);});

test("the claim by path equals the independent Python model on seeded corpora (owners, order, duplicates, environment)",()=>{
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"claim-keys-"));
 try{
  let claimedExtra=0,added=0;
  for(let seed=1;seed<=fixture.equality.seeds;seed++){
   const c=corpus(seed),file=join(dir,"corpus-"+seed+".json");writeFileSync(file,JSON.stringify(c));
   const expected=JSON.parse(Bun.spawnSync([python,"-I",oracle,file]).stdout.toString()),actual=shape(c);
   expect(actual,"seed "+seed).toEqual(expected);
   for(const owner of c.part.owners)added+=actual.owners[owner.name]!.length-owner.nodeRows.length-c.implementation.all.length;
   claimedExtra+=actual.environment.length;
  }
  expect(added,"the corpora exercise the claim by path").toBeGreaterThan(100);
  expect(claimedExtra,"the corpora leave facts to the environment").toBeGreaterThan(100);
 }finally{rmSync(dir,{recursive:true,force:true});}
},180000);

test("shaping many owners that share one long block inserts into sets once per row, not once per owner and row",()=>{
 const {owners,sharedRows,foundRows,maximumInsertionsPerRow}=fixture.memory,shared:Row[]=Array.from({length:sharedRows},(_,i)=>({kind:"exists",path:"/w/lib/f"+i,fact:"v"+(i%7)}));
 const capture:Row[]=[...shared],part={owners:[] as unknown[],edgeOnly:[],edges:[],rows:capture,env:{digest:"x",rows:[]},externalNodes:{},parseUnavailable:[],replaceTags:[]};
 for(let owner=0;owner<owners;owner++){
  const own:Row[]=Array.from({length:foundRows},(_,i)=>({kind:"file",path:"/w/p"+owner+"/f"+i,fact:"o"+owner}));
  capture.push(...own);part.owners.push({name:"o"+owner,root:"p"+owner,node:{targets:{}},nodeRows:[...shared,...own]});
 }
 const NativeSet=globalThis.Set;let insertions=0;
 class CountingSet<T> extends NativeSet<T>{add(value:T){insertions++;return super.add(value);}}
 let shaped;
 try{(globalThis as {Set:unknown}).Set=CountingSet;shaped=shapeComputeResultV1({workspaceRoot:"/w",part:part as never,stubs:{} as never});}finally{(globalThis as {Set:unknown}).Set=NativeSet;}
 expect(shaped.owners.length).toBe(owners);expect(shaped.owners[0]!.rows!.length).toBe(sharedRows+foundRows);
 expect(insertions,"set insertions while shaping "+owners+" owners of "+sharedRows+" shared rows").toBeLessThan(capture.length*maximumInsertionsPerRow);
});
