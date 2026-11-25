/** 🧺️ Contract: a take pass verifies the inventory names of all its stored records in a few asks before its looks ask one by one: 600 stored records cost one ask per `prefetchRecordsPerAsk` records, never more than `prefetchAsksInFlight` at once, each distinct name asked once, in bounded time; a record that cannot be read is skipped, a failed ask only loses its hint, a cancelled pass stops reading. The names equal those an independent reader (SQLite `json_tree` over the same results) finds.
 * @see ../../🧺️presence-prefetch/🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{createHash}from"node:crypto";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{brotliCompressSync}from"node:zlib";
import{TAKE_POLICY_V1}from"../../🟨️.mjs";
import{prefetchTakePresenceV1,takeStoredRecordResultV1}from"../../🧺️presence-prefetch/🟨️.mjs";

const pool=Array.from({length:900},(_,at)=>"SEMIO_"+(at%3===0?"CARGO_LOCK_":"SOURCE_CLOSURE_")+createHash("sha256").update(String(at)).digest("hex"));
const resultOf=(at:number)=>({name:"owner-"+at,targets:{build:{options:{env:pool[at%900]}},check:{options:{env:pool[(at*7+3)%900]},inputs:[{env:pool[(at*13+5)%900]}]}}});
const capture=(value:unknown)=>{const names=new Set<string>();const visit=(row:unknown)=>{if(Array.isArray(row))row.forEach(visit);else if(row!==null&&typeof row==="object")for(const[key,item]of Object.entries(row)){if(key==="env"&&typeof item==="string")names.add(item);else visit(item);}};visit(value);return[...names].sort();};
const store=(directory:string,count:number)=>Array.from({length:count},(_,at)=>{
 const path=join(directory,at+".json"),result=JSON.stringify(resultOf(at)),header=JSON.stringify({digest:"d"+at,resultSha256:createHash("sha256").update(result).digest("hex"),observations:[]});
 if(at===11)writeFileSync(path,"not brotli");else if(at===12)writeFileSync(path,brotliCompressSync(Buffer.from(JSON.stringify({digest:"x",resultSha256:"0".repeat(64)})+"\n"+result)));else if(at!==13)writeFileSync(path,brotliCompressSync(Buffer.from(header+"\n"+result)));
 return path;
});
const independentNames=(count:number,skip:ReadonlySet<number>)=>{const db=new Database(":memory:");try{db.run("create table r(doc text)");const insert=db.prepare("insert into r values (?)");for(let at=0;at<count;at++)if(!skip.has(at))insert.run(JSON.stringify(resultOf(at)));return(db.query("select distinct t.value as name from r, json_tree(r.doc) t where t.key='env' order by 1").all() as {name:string}[]).map(row=>row.name);}finally{db.close();}};

test("600 stored records are verified in one ask per 64 records, at most four at once, every distinct name once, in bounded time",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"take-prefetch-"));
 try{
  const records=store(directory,600),asked:string[][]=[];let flying=0,most=0;
  const began=performance.now();
  const counts=await prefetchTakePresenceV1({records,capture,verify:async names=>{flying++;most=Math.max(most,flying);asked.push([...names]);await Bun.sleep(25);flying--;return{present:names.length};}});
  const seconds=(performance.now()-began)/1000,expected=independentNames(600,new Set([11,12,13]));
  expect(counts.records).toBe(597);expect(counts.skipped).toBe(3);expect(counts.failed).toBe(0);
  expect(counts.asks).toBe(asked.length);expect(asked.length).toBe(Math.ceil(597/TAKE_POLICY_V1.prefetchRecordsPerAsk));
  expect(most).toBeLessThanOrEqual(TAKE_POLICY_V1.prefetchAsksInFlight);expect(most).toBeGreaterThan(1);
  expect(asked.flat().sort()).toEqual(expected);expect(new Set(asked.flat()).size).toBe(asked.flat().length);
  expect(counts.names).toBe(expected.length);expect(counts.present).toBe(expected.length);
  expect(seconds).toBeLessThan(10);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a record that is missing, damaged or names another result is skipped; a failed ask loses only its own hint",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"take-prefetch-"));
 try{
  const records=store(directory,200);
  expect(takeStoredRecordResultV1(records[11]!)).toBeUndefined();expect(takeStoredRecordResultV1(records[12]!)).toBeUndefined();expect(takeStoredRecordResultV1(records[13]!)).toBeUndefined();
  expect(takeStoredRecordResultV1(records[14]!)).toEqual(resultOf(14));
  let calls=0;
  const counts=await prefetchTakePresenceV1({records,capture,verify:async names=>{if(calls++===1)throw Error("refused");return{present:names.length};}});
  expect(counts.records).toBe(197);expect(counts.failed).toBe(1);expect(counts.asks).toBe(4);expect(counts.present).toBeLessThan(counts.names);expect(counts.present).toBeGreaterThan(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a cancelled pass stops reading records and sends nothing more",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"take-prefetch-"));
 try{
  const records=store(directory,600),stop=new AbortController();let asks=0;
  const counts=await prefetchTakePresenceV1({records,capture,signal:stop.signal,verify:async names=>{asks++;stop.abort();await Bun.sleep(5);return{present:names.length};}});
  expect(asks).toBeLessThanOrEqual(TAKE_POLICY_V1.prefetchAsksInFlight);expect(counts.records).toBeLessThan(600);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
