import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import schema from"../../../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import{nativeStorageStateV1,nativeStorageEventV1,type NativeStorageEventV1}from"../../🟦️.ts";
import{classifyNativeStorageDescendantV1}from"../🟦️.ts";
const capacity=25769803776;

test("descendant fixture is closed schema DATA agreeing with independent Ajv",()=>{
 const validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/DescendantCorpus"});
 expect(validate(corpus)).toBe(true);
 expect(validate({...corpus,extra:true})).toBe(false);
 expect(validate({...corpus,cases:[{...corpus.cases[0],expected:"adopted"}]})).toBe(false);
});

test("a journal replayed by the law classifies the half-born persistent child, agreeing with an independent SQLite replay",()=>{
 const db=new Database(":memory:");
 try{
  for(const row of corpus.cases){
   let state=nativeStorageStateV1();for(const event of row.journal)state=nativeStorageEventV1(state,event as NativeStorageEventV1,capacity);
   db.exec("DROP TABLE IF EXISTS alloc;CREATE TABLE alloc(id TEXT PRIMARY KEY,kind TEXT,phase TEXT,child_dir TEXT)");
   for(const event of row.journal as {type:string;id?:string;allocation?:{id:string;kind:string;phase:string;child?:{directory:string}}}[]){
    if(event.type==="reserved")db.query("INSERT INTO alloc VALUES(?,?,?,?)").run(event.allocation!.id,event.allocation!.kind,event.allocation!.phase,event.allocation!.child?.directory??null);
    else if(event.type==="retired")db.query("DELETE FROM alloc WHERE id=?").run(event.id!);
    else db.query("UPDATE alloc SET phase=? WHERE id=?").run(event.type,event.id!);
   }
   const namespace=corpus.parentRoot+"/persistent",oracle=db.query("SELECT CASE WHEN EXISTS(SELECT 1 FROM alloc WHERE child_dir=?1) THEN 'funded' WHEN ?1 LIKE ?2||'/%' AND instr(substr(?1,length(?2)+2),'/')=0 AND EXISTS(SELECT 1 FROM alloc WHERE kind='persistent-state' AND child_dir IS NULL AND phase<>'retiring') THEN 'persistent-state' ELSE 'unfunded' END AS kind").get(row.directory,namespace)as{kind:string};
   expect<string>(classifyNativeStorageDescendantV1(state,corpus.parentRoot,row.directory),row.name).toBe(row.expected);
   expect(oracle.kind,row.name).toBe(row.expected);
  }
 }finally{db.close();}
});

test("persistent state stays unretirable and a child still needs a compiler-state charge",()=>{
 const reserved=corpus.cases[0]!.journal.slice(0,3) as NativeStorageEventV1[];let state=nativeStorageStateV1();for(const event of reserved)state=nativeStorageEventV1(state,event,capacity);
 const orphan=state.allocations.find(row=>row.kind==="persistent-state")!;
 for(const type of["retiring","retired"]as const)expect(()=>nativeStorageEventV1(state,{type,id:orphan.id},capacity)).toThrow("Persistent state cannot be retired");
 expect(()=>nativeStorageEventV1(nativeStorageStateV1(),{type:"reserved",allocation:{...orphan,child:{version:1,allocation:orphan.id,parentIdentity:"[1]",directory:"/fixture/m/persistent/development",filesystemBytes:1,maximumPhysicalBytes:1,policySha256:"a".repeat(64)}}},capacity)).toThrow("Exact child funding allocation association required");
});
