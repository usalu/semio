import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {readSourceFamilySource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import {expect,test} from "bun:test";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const sample=(index:number)=>(index*law.sample.multiplier+law.sample.offset)%law.sample.modulo;

test("bulk bitwise vector clone chunks agree with an independent page-wise typed-array copy",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,pageBytes:4096})).toBe(false);expect(validate({...law,overheadTurns:0})).toBe(false);
  for(const count of law.sampleCounts){
    const source=new Uint16Array(count);for(let index=0;index<count;index++)source[index]=sample(index);
    const bytes=new Uint8Array(source.buffer),target=new Uint8Array(bytes.byteLength);let chunks=0;
    for(let start=0;start<bytes.byteLength;start+=law.pageBytes){target.set(bytes.subarray(start,Math.min(start+law.pageBytes,bytes.byteLength)),start);chunks++;}
    expect(Array.from(new Uint16Array(target.buffer))).toEqual(Array.from(source));
    expect(chunks).toBe(Math.ceil(bytes.byteLength/law.pageBytes));
    const turns=law.overheadTurns+chunks;
    expect(turns).toBeLessThanOrEqual(law.overheadTurns+Math.ceil(count*law.elementBytes/law.pageBytes));
    if(count===2097152){expect(bytes.byteLength).toBe(4*1024*1024);expect(turns).toBe(law.maximumTurnsForFourMebibytes);expect(count*law.elementwiseTurnsPerElement/turns).toBeGreaterThan(400000);}
  }
  const zeroSized=Array.from({length:law.zeroSizedSampleCount},()=>null),zeroOracle=structuredClone(zeroSized),zeroCopy=[] as null[];
  let zeroTurns=0;for(let start=0;start<zeroSized.length;start+=law.maximumZeroSizedElementsPerTurn){zeroCopy.push(...zeroSized.slice(start,start+law.maximumZeroSizedElementsPerTurn));zeroTurns++;}
  expect(zeroCopy).toEqual(zeroOracle);expect(zeroTurns).toBe(Math.ceil(law.zeroSizedSampleCount/law.maximumZeroSizedElementsPerTurn));
  const largeArray=Array.from({length:law.largeZeroSizedArrayCount},()=>null);let arrayTurns=0,arrayCopied=0;
  while(arrayCopied<largeArray.length){arrayCopied+=Math.min(law.maximumZeroSizedElementsPerTurn,largeArray.length-arrayCopied);arrayTurns++;}
  expect(arrayCopied).toBe(structuredClone(largeArray).length);expect(arrayTurns).toBe(Math.ceil(law.largeZeroSizedArrayCount/law.maximumZeroSizedElementsPerTurn));
  expect(law.ownerHeaderBytes).toBe(3*8);
  console.log("[DEBUG] Bulk vector clone chunk arithmetic and sample bytes agree with an independent typed-array page copy");
});


test("bulk grant boundaries agree with independent SQLite physical and logical pages",()=>{
  const db=new Database(":memory:");
  try{
    db.run("CREATE TABLE grants(items INTEGER, copy_bytes INTEGER, width INTEGER, expected INTEGER)");
    for(const row of law.grantCases)db.run("INSERT INTO grants VALUES(?,?,?,?)",[row.items,row.copyBytes,row.width,row.elements]);
    const rows=db.query("SELECT expected, CASE WHEN items=0 THEN 0 WHEN width=0 THEN ? ELSE MIN(copy_bytes, ?)/width END AS actual FROM grants ORDER BY rowid").all(law.maximumZeroSizedElementsPerTurn,law.pageBytes) as {expected:number;actual:number}[];
    for(const row of rows)expect(row.actual).toBe(row.expected);
    const original=Array.from({length:law.largeZeroSizedArrayCount},(_,rank)=>rank);
    db.run("CREATE TABLE ranks(rank INTEGER PRIMARY KEY)");
    for(const rank of original)db.run("INSERT INTO ranks VALUES(?)",[rank]);
    const pages=db.query("SELECT CAST(rank/? AS INTEGER) AS page, COUNT(*) AS rows FROM ranks GROUP BY page ORDER BY page").all(law.maximumZeroSizedElementsPerTurn) as {page:number;rows:number}[];
    expect(pages.map(row=>row.rows)).toEqual(original.reduce<number[]>((pages,rank)=>{const page=Math.floor(rank/law.maximumZeroSizedElementsPerTurn);pages[page]=(pages[page]??0)+1;return pages;},[]));
    expect(pages.every(row=>row.rows<=law.maximumZeroSizedElementsPerTurn)).toBe(true);
    expect(pages.reduce((sum,row)=>sum+row.rows,0)).toBe(law.largeZeroSizedArrayCount);
    console.log(`[DEBUG] SQLite verifies ${rows.length} bulk grant boundaries and ${pages.length} finite zero-sized array pages; native custody laws remain separate`);
  }finally{db.close();}
});

test("defining clone contract bounds zero-sized runs and retains only initialized array prefixes",()=>{
  const source=readSourceFamilySource(new URL("../../🧬️contract/",import.meta.url));
  expect(source).toContain(`pub const RETAINED_CLONE_BULK_PAGE_ITEMS: usize = ${law.maximumZeroSizedElementsPerTurn};`);
  const bulk=source.slice(source.indexOf("pub fn bulk_run_elements"),source.indexOf("pub fn bulk_run_progress"));
  expect(bulk).toContain("if width == 0 { return RETAINED_CLONE_BULK_PAGE_ITEMS; }");
  expect(bulk).not.toContain("usize::MAX");
  const array=source.slice(source.indexOf("pub struct ArrayCursor"),source.indexOf("impl<T: RetainedClone, const N: usize> RetainedClone for [T; N]"));
  expect(array).toContain("slots: [std::mem::MaybeUninit<T>; N]");
  expect(array).toContain("N<=RETAINED_CLONE_BULK_PAGE_ITEMS");
  expect(array).toContain("(N-self.index).min(bulk_run_elements(RetainedCloneGrant{maximum_copy_bytes:body,..grant},size_of::<T>()).max(1))");
  expect(array).toContain("self.slots[self.index].assume_init_read()");
  expect(array).not.toContain("self.slots.iter_mut()");
  expect(array).toContain("self.index == 0");
  expect(array).toContain("fn close_demands");
});


test("bulk native selector reaches the defining retained clone crate",()=>{
  const project=JSON.parse(readFileSync(new URL("../../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8"));
  expect(project.targets["test-bulk-vec-retained-native"].options.command).toBe("repo ./📜️script.ts test-bulk-vec-retained native");
  const router=readFileSync(new URL("../../../📦️packages/🦀️rust/📜️script.ts",import.meta.url),"utf8");
  expect(router).toContain('.register("test-bulk-vec-retained",BulkVecRetainedCheckScript)');
  const start=router.indexOf("class BulkVecRetainedCheckScript"),owner=router.slice(start,router.indexOf("\nclass ",start+1));
  expect(owner).toContain('"semio-framework-value-retained-clone"');
  expect(owner).toContain('"semio_framework_value_retained_clone"');
  expect(owner).toContain('"retained_clone::bulk_vec_tests::bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes"');
  expect(owner).toContain("invocation:this.invocation");
  expect(owner).toContain("configuredExactCargoLawPolicyV1()");
});


test("logical clone bodies remain independent from admitted physical headers",()=>{
 expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");try{
  const query=db.query("SELECT MIN(?1,MAX(1,MIN(?2,65536)/?3)) AS elements");
  for(const row of law.logicalCases){
   const independent=query.get(row.remaining,row.body,row.width) as {elements:number};
   expect(independent.elements).toBe(row.elements);
   const original=Uint8Array.from({length:row.remaining*row.width},(_,index)=>index%251);
   const copy=Buffer.from(original.subarray(0,row.elements*row.width));
   expect([...copy]).toEqual([...original.slice(0,independent.elements*row.width)]);
   expect(copy.byteLength).toBeLessThanOrEqual(row.physical);
  }
 }finally{db.close();}
 const source=readSourceFamilySource(new URL("../../🧬️contract/",import.meta.url));
 expect(source).toContain("source: RetainedCloneRef<'_, T>, body: usize, grant: RetainedCloneGrant");
 expect(source).not.toContain("self.advance_demands(source,grant.maximum_copy_bytes)");
 expect(source).toContain("let demand = self.advance_demands(source,body)?;");
 expect(source).toContain("remaining.min(demand.copy_bytes)");
 expect(source).toContain("size_of::<[T;N]>()<=body");
 expect(source).toContain("if T::BITWISE && self.index==0");
 expect(source).toContain("copy_bytes:if self.output.is_none(){size_of::<Vec<T>>()}else{0}");
 const field=readFileSync(new URL("../../📋️field/🦀️.rs",import.meta.url),"utf8");expect(field).toContain("child.advance(source,body,");
 const derive=readFileSync(new URL("../../../✨️derive/🧬️retained-clone/🦀️.rs",import.meta.url),"utf8");expect(derive).toContain("advance_demands(self,source,body)");
 console.log("[DEBUG] original logical1/7/4096 clone extents agree with independent SQLite and NodeBuffer; native cursor laws remain separate");
});


test("production clone wrappers require the original body through quoted child loans",()=>{
 const owner=(name:string)=>readFileSync(new URL("../../../../../../"+name,import.meta.url),"utf8");
 const projection=owner("🧰️framework/🔨️modules/🌱️value/📽️projection/🦀️.rs");expect(projection).toContain("body:usize,grant:RetainedCloneGrant");
 const field=owner("🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🦀️.rs");expect(field).toContain("self.cursor.advance(source, body, grant)");expect(field).not.toContain("self.demand(source, grant.maximum_copy_bytes)");
 const semantic=owner("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs");expect(semantic).toContain("self.demands(body)");expect(semantic).not.toContain("self.demands(grant.maximum_copy_bytes)");
 const drivers=owner("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔌️drivers/🦀️.rs");expect(drivers).toContain("advance(s,body,g)");
});
