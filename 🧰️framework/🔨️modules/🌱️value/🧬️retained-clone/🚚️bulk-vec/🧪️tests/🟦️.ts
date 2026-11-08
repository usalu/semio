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
    const turns=(count===0?law.emptyOverheadTurns:law.overheadTurns)+chunks;
    expect(turns).toBeLessThanOrEqual((count===0?law.emptyOverheadTurns:law.overheadTurns)+Math.ceil(count*law.elementBytes/law.pageBytes));
    if(count===2097152){expect(bytes.byteLength).toBe(4*1024*1024);expect(turns).toBe(law.maximumTurnsForFourMebibytes);expect(count*law.elementwiseTurnsPerElement/turns).toBeGreaterThan(400000);}
  }
  const zeroSized=Array.from({length:law.zeroSizedSampleCount},()=>null),zeroOracle=structuredClone(zeroSized),zeroCopy=[] as null[];
  let zeroTurns=0;for(let start=0;start<zeroSized.length;start+=law.maximumZeroSizedElementsPerTurn){zeroCopy.push(...zeroSized.slice(start,start+law.maximumZeroSizedElementsPerTurn));zeroTurns++;}
  expect(zeroCopy).toEqual(zeroOracle);expect(zeroTurns).toBe(Math.ceil(law.zeroSizedSampleCount/law.maximumZeroSizedElementsPerTurn));
  const largeArray=Array.from({length:law.largeZeroSizedArrayCount},()=>null);let arrayTurns=0,arrayCopied=0;
  while(arrayCopied<largeArray.length){arrayCopied+=Math.min(law.maximumZeroSizedElementsPerTurn,largeArray.length-arrayCopied);arrayTurns++;}
  expect(arrayCopied).toBe(structuredClone(largeArray).length);expect(arrayTurns).toBe(Math.ceil(law.largeZeroSizedArrayCount/law.maximumZeroSizedElementsPerTurn));
  expect(law.ownerHeaderBytes).toBe(3*8);
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
  }finally{db.close();}
});

test("defining clone contract bounds zero-sized runs and retains only initialized array prefixes",()=>{
  const source=readSourceFamilySource(new URL("../../🧬️contract/",import.meta.url));
  expect(source).toContain(`pub const RETAINED_CLONE_BULK_PAGE_ITEMS: usize = ${law.maximumZeroSizedElementsPerTurn};`);
  const bulk=source.slice(source.indexOf("pub fn bulk_run_elements"),source.indexOf("pub fn bulk_run_progress"));
  expect(bulk).toContain("if width == 0 { return RETAINED_CLONE_BULK_PAGE_ITEMS; }");
  expect(bulk).not.toContain("usize::MAX");
  const array=source.slice(source.indexOf("pub struct ArrayCursor"),source.indexOf("impl<T: RetainedClone, const N: usize> RetainedClone for [T; N]"));
  const prefix=readFileSync(new URL("../../⚙️engine/📋️prefix/🦀️.rs",import.meta.url),"utf8");
  const selection=readFileSync(new URL("../../🧬️contract/🧮️array-cursor/🔎️selection/🦀️.rs",import.meta.url),"utf8");
  expect(array).toContain("prefix: NativeArrayPrefix<T,N>");
  expect(prefix).toContain("slots:[std::mem::MaybeUninit<T>;N]");
  expect(array).toContain("N<=RETAINED_CLONE_BULK_PAGE_ITEMS");
  expect(selection).toContain("remaining.min(RETAINED_CLONE_BULK_PAGE_ITEMS).min(by_work).min(by_copy)");
  expect(prefix).toContain("assume_init_read()");
  expect(prefix).not.toContain("self.slots.iter_mut()");
  expect(prefix).toContain("self.initialized==0");
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
 expect(source).toContain("source:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect");
 expect(source).not.toContain("self.advance_demands(source,grant.maximum_copy_bytes)");
 expect(source).toContain("self.advance_demands(source,body,maximum)?;");
 expect(source).toContain("body.min(RETAINED_CLONE_BULK_PAGE_BYTES)/width");
 expect(source).toContain("size_of::<[T;N]>()<=body");
 expect(source).toContain("T::native_copy()");
 expect(source).toContain("maximum_work_units:u64");
 const field=readFileSync(new URL("../../⚙️engine/📋️field/🦀️.rs",import.meta.url),"utf8");expect(field).toContain("child.advance_into(source,body,maximum,&mut effect,reply)");
 const derive=readFileSync(new URL("../../../✨️derive/🧬️retained-clone/🦀️.rs",import.meta.url),"utf8");expect(derive).toContain("advance_demands(self,source,body,maximum_work_units)");
});


test("production clone wrappers require the original body through quoted child loans",()=>{
 const owner=(name:string)=>readFileSync(new URL("../../../../../../"+name,import.meta.url),"utf8");
 const projection=owner("🧰️framework/🔨️modules/🌱️value/📽️projection/🦀️.rs");expect(projection).toContain("body:usize,effect:&mut crate::RetainedAllocationEffect");
 const field=owner("🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🦀️.rs");expect(field).toContain("self.cursor.advance_into(source,body,maximum_work_units");expect(field).not.toContain("self.demand(source, grant.maximum_copy_bytes)");
 const semantic=owner("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs");expect(semantic).toContain("self.demands(body)");expect(semantic).not.toContain("self.demands(grant.maximum_copy_bytes)");
 const drivers=owner("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔌️drivers/🦀️.rs");expect(drivers).toContain("advance(s,body,g)");
});


test("bulk selected work agrees with SQLite",()=>{
 const db=new Database(":memory:");try{
  const q=db.query("SELECT MIN(?1, CASE WHEN ?2=0 THEN ?5 ELSE MAX(1,MIN(?3,?4)/?2) END, ?6/MAX(1,?2)) AS elements");
  for(const row of law.workCases){const oracle=q.get(row.remaining,row.width,row.body,law.pageBytes,law.maximumZeroSizedElementsPerTurn,row.work) as {elements:number};expect(oracle.elements).toBe(row.elements);expect(oracle.elements*Math.max(1,row.width)).toBeLessThanOrEqual(row.work);}
 }finally{db.close();}
});

test("publication and foreign original facts remain independent",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{const array=law.arrayRetirement;expect(db.query("with recursive cuts(n) as (select 0 union all select n+1 from cuts where n+1<?) select count(*) as cases, sum(min(?,max(0,n-2)*?)) as elements from cuts").get(array.cutCases,law.largeZeroSizedArrayCount,law.maximumZeroSizedElementsPerTurn)).toEqual({cases:array.cutCases,elements:array.maximumRetiredElements});const w=law.originalWorkCeilings;expect(db.query("select min(?/?,?-?-?) as elements").get(law.pageBytes,law.elementBytes,w.bounded,w.binding,w.header)).toEqual({elements:w.boundedElements});expect(db.query("select ?>=(?/?+?+?) as funded").get(w.throughput,law.pageBytes,law.elementBytes,w.binding,w.header)).toEqual({funded:1});const d=law.sourceStockDerivation,st=law.sourceStock;const q=db.query("select ?+?*(?+?+?*?+?) as work, ?*(?*4+?*8+?*?) as copy, ?*?*2+? as capacity, (?+1)*?*2+? as release").get(d.sourceBirthWork,d.cases,d.cancellationRows,d.closeChunksPerCase,d.copyTurnsPerCase,d.workPerMetadataTurn,d.perCaseFixedWork,d.cases,d.cancellationRows,d.closeChunksPerCase,d.copyTurnsPerCase,d.bytesPerMetadataTurn,d.cases,d.cancellationRows,d.sourceHeaderBytes,d.cases,d.cancellationRows,d.sourceHeaderBytes);expect(q).toEqual({work:st.work,copy:st.maximumCopyBytes,capacity:st.maximumCapacityBytes,release:d.cases*d.cancellationRows*2+d.cancellationRows*2+d.sourceHeaderBytes});expect(db.query("select ?>=(?+2*?+?*?+?+?) as work, ?>=(3*?*?+2*?*8+?*?+?) as copy").get(st.work,d.throughputRows,d.throughputCloseChunks,d.throughputPageTurns,d.workPerMetadataTurn,d.perCaseFixedWork,d.sourceBirthWork,st.maximumCopyBytes,d.throughputRows,law.elementBytes,d.throughputCloseChunks,d.throughputPageTurns,d.bytesPerMetadataTurn,d.sourceHeaderBytes)).toEqual({work:1,copy:1});expect(db.query("select max(?,2*?+?*?+?) as items, max(?,2*?*?+?) as released").get(st.work-d.sourceBirthWork,d.throughputRows,d.throughputPageTurns,d.workPerMetadataTurn,d.perCaseFixedWork, (d.cases+1)*d.cancellationRows*2+d.sourceHeaderBytes,d.throughputRows,law.elementBytes,d.sourceHeaderBytes)).toEqual({items:law.throughputTerminalItems.refusalStockItems,released:st.maximumReleaseBytes});const terminal=law.throughputTerminalItems;expect(db.query("select ?+(?-?)/?+?+? as items").get(terminal.refusalStockItems,terminal.selectedTailCopyBytes,terminal.nativeLengthHeaderBytes,law.elementBytes,terminal.terminalTraceItems,terminal.allocatedBufferReleaseItems)).toEqual({items:st.maximumItems});expect(db.query("select ? >= ? as capacity, ? >= ? as work").get(law.sourceStock.maximumCapacityBytes,4*1024*1024,law.sourceStock.work,2*1024*1024)).toEqual({capacity:1,work:1});expect(db.query("select count(*) as fields from json_each(?)").get(JSON.stringify(law.birthHeaderFields))).toEqual({fields:2});expect(db.query("select case when ?=0 then ? else ? end as turns").get(0,law.emptyOverheadTurns,law.overheadTurns)).toEqual({turns:2});expect(db.query("select json_extract(?, '$.matchesDemand') as matched, json_extract(?, '$.work') as work").get(JSON.stringify(law.publication),JSON.stringify(law.publication))).toEqual({matched:1,work:1});expect(db.query("select json_extract(?, '$.kind') as kind, json_extract(?, '$.work') as work").get(JSON.stringify(law.foreignQuote),JSON.stringify(law.foreignQuote))).toEqual({kind:"InvariantViolated",work:0})}finally{db.close()}});
