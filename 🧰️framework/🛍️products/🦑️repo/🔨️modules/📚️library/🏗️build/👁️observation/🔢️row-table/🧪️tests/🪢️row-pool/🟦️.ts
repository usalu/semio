/** 🪢️ Contract: however many crates of a synthetic workspace hold equal analysis rows (each parsed afresh, as a stored closure entry is), the weak row pool leaves one live object per distinct row, exactly as many as an independent Python model counts, and the objects the crates keep alive grow with the distinct rows, not with the crates times the rows they read; the very same row text is one instance, a changed fact is another.
 * @see ../../🟨️.mjs `createRowPoolV1` */
import{expect,test}from"bun:test";
import{join}from"node:path";
import Ajv from"ajv";
import{heapStats}from"bun:jsc";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createRowPoolV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

type Row={kind:string;path:string;fact:{nestedObservations?:Row[]}&Record<string,unknown>};
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
const model=()=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({workspaces:fixture.workspaces}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()).workspaces as {crates:Row[][];distinct:number;distinctNested:number}[];
};
const live=()=>heapStats().objectCount;
const settle=async()=>{for(let round=0;round<3;round++){await new Promise(resolve=>setTimeout(resolve,5));Bun.gc(true);}};

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the pool leaves one live object per distinct row, as the model counts them, and the held objects grow with the distinct rows only",async()=>{
 const answers=model();
 for(const[at,spec]of fixture.workspaces.entries()){
  const answer=answers[at]!,pool=createRowPoolV1<Row>(),held:Row[][]=[];
  await settle();const before=live(),top=new Set<Row>(),nested=new Set<Row>();
  for(const crate of answer.crates){
   const parsed=JSON.parse(JSON.stringify(crate)) as Row[];
   held.push(pool.rows(parsed));
   for(const row of parsed){top.add(row);for(const inner of row.fact.nestedObservations??[])nested.add(inner);}
  }
  expect(top.size,spec.name).toBe(answer.distinct);
  expect(nested.size,spec.name).toBe(answer.distinctNested);
  held.forEach((rows,crate)=>expect(JSON.stringify(rows),spec.name+" crate "+crate).toBe(JSON.stringify(answer.crates[crate])));
  await settle();
  const grown=live()-before,distinct=answer.distinct+answer.distinctNested;
  expect(grown,spec.name+": "+grown+" objects for "+distinct+" distinct rows").toBeLessThan(distinct*spec.maximumObjectsPerDistinctRow+answer.crates.length*8);
  expect(held.length).toBe(spec.crates);
 }
},60000);

test("equal rows are one instance whichever crate parsed them first and a changed fact is another row",()=>{
 const pool=createRowPoolV1<Row>(),make=(sha:string):Row=>JSON.parse(JSON.stringify({kind:"file",path:"/w/f",fact:{size:1,mtimeMs:2,ctimeMs:3,ino:4,sha256:sha}}));
 const first=make("aa"),second=make("aa"),other=make("ab");
 expect(pool.share(first)).toBe(first);expect(pool.share(second)).toBe(first);expect(pool.share(other)).toBe(other);expect(pool.size()).toBe(2);
});

test("a row dies with its last holder and leaves no key behind",async()=>{
 const pool=createRowPoolV1<Row>();
 (()=>{for(let at=0;at<500;at++)pool.share({kind:"file",path:"/w/f"+at,fact:{size:at,mtimeMs:0,ctimeMs:0,ino:0,sha256:"aa"}});})();
 for(let round=0;round<8&&pool.size()>0;round++)await settle();
 expect(pool.size()).toBeLessThan(50);
});

test("a list and a row that were shared once are never serialised again, however many entries carry them",()=>{
 const pool=createRowPoolV1<Row>(),counts={serialised:0},make=(at:number):Row=>({kind:"file",path:"/w/f"+at,fact:{size:at,mtimeMs:0,ctimeMs:0,ino:0,sha256:"aa",toJSON(){counts.serialised++;return{size:at};}} as never});
 const layer=Array.from({length:200},(_,at)=>make(at));
 pool.rows(layer);
 const afterFirst=counts.serialised;
 expect(afterFirst).toBeGreaterThan(0);
 for(let entry=0;entry<50;entry++){const carried=pool.rows(layer);expect(carried).toBe(layer);for(const row of layer)expect(pool.share(row)).toBe(row);}
 expect(counts.serialised).toBe(afterFirst);
 const copy=Array.from({length:200},(_,at)=>make(at));
 pool.rows(copy);
 copy.forEach((row,at)=>expect(row).toBe(layer[at]));
});
