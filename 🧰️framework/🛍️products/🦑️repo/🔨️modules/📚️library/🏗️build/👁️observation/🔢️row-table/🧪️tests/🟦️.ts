/** 🔢️ Contract: the compact row store gives equal observation rows (same kind, path and fact JSON) one number and distinct rows distinct numbers, exactly as an independent Python model groups them (numbers of nested rows are interleaved, so the grouping is compared in first-occurrence order), also when every row hashes to the same bucket; a row made from the columns serialises to the very JSON text the model gives; a row object that is not the canonical one has the number of its canonical row; clearing (also by budget) never reuses a number; and a store of a large generated corpus stays under its heap bound while no row object is held. */
import{expect,test}from"bun:test";
import{join}from"node:path";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{heapStats}from"bun:jsc";
import{createRowTableV1,createStringTableV1,factKeyV1}from"../🟨️.mjs";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

type Row={kind:string;path:string;fact:unknown};
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
const model=()=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({cases:fixture.cases}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()).cases as {rows:[string,string,unknown][];groups:number[];texts:string[];distinct:number}[];
};
const firstOccurrence=(ids:number[])=>{const seen=new Map<number,number>();return ids.map(id=>{if(!seen.has(id))seen.set(id,seen.size+1);return seen.get(id)!;});};
const numbers=(rows:[string,string,unknown][],options:Parameters<typeof createRowTableV1>[0])=>{
 const table=createRowTableV1<Row>(options),first=new Map<number,Row>(),out:number[]=[];
 for(const [kind,path,fact] of rows){
  const made={kind,path,fact},row=table.canonical(kind,path,fact,()=>made),id=table.id(row);
  out.push(id);if(!first.has(id))first.set(id,row);expect(first.get(id)).toBe(row);
 }
 return{table,out};
};

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the numbers of the table group the rows exactly as the model does, with the real hash and with every row in one bucket",()=>{
 for(const answer of model())for(const options of[{},{hash:()=>1}]){
  const{out,table}=numbers(answer.rows,options);
  expect(firstOccurrence(out)).toEqual(answer.groups);
  expect(table.size()).toBeGreaterThanOrEqual(answer.distinct);
 }
});

test("a row made from the columns serialises to the JSON text of the model, for every shape of fact",()=>{
 for(const answer of model()){
  const table=createRowTableV1<Row>(),ids=answer.rows.map(([kind,path,fact])=>table.id({kind,path,fact}));
  answer.rows.forEach((_,at)=>{
   const made=table.materialize(ids[at])!;
   expect(JSON.stringify(made),"row "+at).toBe(answer.texts[answer.groups.indexOf(answer.groups[at])]);
   expect(table.id(made)).toBe(ids[at]);
  });
 }
});

test("a row whose object died is made again from the columns with the same number",async()=>{
 const table=createRowTableV1<Row>(),fact={size:1,mtimeMs:2,ctimeMs:3,ino:4,sha256:"ab"};
 const id=(()=>table.id({kind:"file",path:"/w/f",fact}))();
 for(let round=0;round<5;round++){await new Promise(resolve=>setTimeout(resolve,5));Bun.gc(true);}
 const again=table.row(id)!;
 expect(again).toEqual({kind:"file",path:"/w/f",fact});expect(table.id(again)).toBe(id);expect(table.row(id)).toBe(again);
});

test("a different object of an equal fact has the number of the canonical row and the canonical row is the first one made",()=>{
 const table=createRowTableV1<Row>(),first={kind:"file",path:"/w/f",fact:{size:1,mtimeMs:2,ctimeMs:3,ino:4,sha256:"ab"}},second={kind:"file",path:"/w/f",fact:{size:1,mtimeMs:2,ctimeMs:3,ino:4,sha256:"ab"}};
 expect(table.canonical(first.kind,first.path,first.fact,()=>first)).toBe(first);
 expect(table.canonical(second.kind,second.path,second.fact,()=>second)).toBe(first);
 expect(table.id(second)).toBe(table.id(first));expect(table.size()).toBe(1);
});

test("an unknown row is registered on the first ask for its number, a cleared store never reuses a number",()=>{
 const table=createRowTableV1<Row>(),row={kind:"exists",path:"/w/a",fact:true},other={kind:"exists",path:"/w/b",fact:true};
 expect(table.id(row)).toBe(1);expect(table.id(other)).toBe(2);expect(table.id(row)).toBe(1);
 table.clear();expect(table.size()).toBe(0);expect(table.row(1)).toBeUndefined();
 const again=table.id(other);expect(again).toBeGreaterThan(2);expect(table.id(row)).not.toBe(again);expect(table.id(other)).toBe(again);expect(table.id(row)).toBeGreaterThan(again);
});

test("a store over its row or character budget empties itself and keeps numbering distinct rows apart",()=>{
 for(const options of[{maximumRows:50},{maximumCharacters:2000}]){
  const table=createRowTableV1<Row>(options),rows:Row[]=[],seen=new Map<number,string>();
  for(let at=0;at<400;at++)rows.push({kind:"exists",path:"/w/"+"d".repeat(40)+at%90,fact:at%3===0});
  for(let round=0;round<3;round++)for(const row of rows){
   const id=table.id(row),key=JSON.stringify(row),known=seen.get(id);
   if(known!==undefined)expect(known).toBe(key);seen.set(id,key);
   expect(table.id(row)).toBe(id);
  }
  expect(table.size()).toBeLessThanOrEqual(options.maximumRows??400);
  expect(table.stats().characters).toBeLessThanOrEqual((options.maximumCharacters??Infinity)+400);
 }
});

test("the string table gives equal texts one number and a cleared table starts from zero",()=>{
 const table=createStringTableV1();
 expect([table.id("a"),table.id("b"),table.id("a")]).toEqual([0,1,0]);expect(table.text(1)).toBe("b");expect(table.size()).toBe(2);expect(table.characters()).toBe(2);
 table.clear();expect(table.size()).toBe(0);expect(table.id("b")).toBe(0);
});

test("the fact text keeps the type of primitives and the key order of objects",()=>{
 expect(factKeyV1(1)).not.toBe(factKeyV1("1"));expect(factKeyV1({a:1,b:2})).not.toBe(factKeyV1({b:2,a:1}));expect(factKeyV1(null)).toBe("objectnull");
});

test("a large generated corpus of settled rows stays under its heap bound when no row object is held",async()=>{
 const rows=Number(process.env.SEMIO_ROW_STORE_ROWS??300000),bound=Number(process.env.SEMIO_ROW_STORE_HEAP_MB??96),table=createRowTableV1<Row>();
 await new Promise(resolve=>setTimeout(resolve,10));Bun.gc(true);const before=heapStats().heapSize+heapStats().extraMemorySize;
 const kinds=["exists","lstat","file","readdir","analysis","fileExists"],base="/home/deck/semio/"+"🧰️framework/".repeat(3);
 const add=(from:number,to:number)=>{for(let at=from;at<to;at++){
  const kind=kinds[at%kinds.length],path=base+"module"+(at%60000)+"/"+"d".repeat(60)+".ts",sha="9".repeat(64-String(at).length)+at;
  const fact=kind==="file"?{size:at,mtimeMs:1731000000000+at,ctimeMs:1731000000000+at,ino:at,sha256:sha}:kind==="analysis"?{provider:"ts",revision:"r",source:{size:at,mtimeMs:1,ctimeMs:2,ino:3,sha256:sha},resultSha256:sha,nestedObservations:[{kind:"exists",path:base+"x"+at%700,fact:true},{kind:"lstat",path:base+"y"+at%700,fact:"directory"}]}:kind==="readdir"?"d"+at%97:at%2===0;
  table.id({kind,path,fact});
 }};
 for(let from=0;from<rows;from+=10000){add(from,Math.min(rows,from+10000));await new Promise(resolve=>setTimeout(resolve,0));}
 for(let round=0;round<3;round++){await new Promise(resolve=>setTimeout(resolve,10));Bun.gc(true);}
 const grown=(heapStats().heapSize+heapStats().extraMemorySize-before)/1048576,stats=table.stats();
 expect(stats.rows).toBeGreaterThan(rows*0.5);
 expect(grown,"heap growth "+grown.toFixed(1)+" MB for "+stats.rows+" rows").toBeLessThan(bound);
 expect(stats.bytes/1048576,"estimated store bytes").toBeLessThan(bound);
});
