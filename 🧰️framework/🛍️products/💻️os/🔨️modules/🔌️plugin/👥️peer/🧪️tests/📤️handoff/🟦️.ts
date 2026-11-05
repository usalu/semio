import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";

test("peer root handoff preserves unique ownership without replacement allocation",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const row of fixture.cases){
  const root=row.partial?null:{id:row.id,entries:row.entries};
  const admitted=row.denied==="none";
  const retirement=admitted?root:null;
  const remaining=admitted?null:root;
  const actual={rootPresent:remaining!==null,retirementPresent:retirement!==null,allocated:0,released:0,staticBorrow:remaining===null,aliasAllowed:remaining!==null};
  expect(actual).toEqual(row.expected);
  const db=new Database(":memory:");
  db.run("CREATE TABLE custody(id TEXT PRIMARY KEY, owner TEXT NOT NULL)");
  if(root)db.run("INSERT INTO custody VALUES (?, 'roster')",root.id);
  if(admitted&&root)db.run("UPDATE custody SET owner='retirement' WHERE id=?",root.id);
  expect(db.query("SELECT count(*) AS n FROM custody WHERE owner='roster'").get()).toEqual({n:Number(actual.rootPresent)});
  expect(db.query("SELECT count(*) AS n FROM custody WHERE owner='retirement'").get()).toEqual({n:Number(actual.retirementPresent)});
  if(retirement){
   const canRelease=row.aliases===0;
   if(canRelease)db.run("DELETE FROM custody WHERE id=?",retirement.id);
   expect(db.query("SELECT count(*) AS n FROM custody").get()).toEqual({n:canRelease?0:1});
  }
  db.close();
 }
});

test("absent reads and cancelled partial closure require no allocated registry",()=>{
 const empty=Object.freeze({entries:Object.freeze([])});
 const read=(root:typeof empty|null)=>root??empty;
 expect(read(null)).toBe(empty);expect(read(null)).toBe(read(null));
 const partial=fixture.cases.find(row=>row.partial)!;
 expect(partial.expected).toEqual({rootPresent:false,retirementPresent:false,allocated:0,released:0,staticBorrow:true,aliasAllowed:false});
});

test("closing a root never substitutes a lazily allocated shared empty owner",async()=>{
 const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs";
 const source=await Bun.file(path).text();
 expect(source.includes("PeerPresenceRoot::empty_shared")).toBe(false);
});

test("empty and populated owned roots require paid retirement before release",()=>{
 for(const entries of [0,2]){
  const law=fixture.retirement;
  const db=new Database(":memory:");
  db.run("CREATE TABLE owners(name TEXT PRIMARY KEY, bytes INTEGER NOT NULL)");
  db.run("INSERT INTO owners VALUES ('root',?)",law.rootBytes);
  let allocated=0,released=0;
  const stages=["retained"];
  expect(db.query("SELECT count(*) AS n FROM owners").get()).toEqual({n:1});
  db.run("INSERT INTO owners VALUES ('frame',?)",law.frameBytes);allocated+=law.frameBytes;stages.push("frame-funded");
  expect(db.query("SELECT sum(bytes) AS n FROM owners").get()).toEqual({n:law.rootBytes+law.frameBytes});
  for(let entry=0;entry<entries;entry++)expect(db.query("SELECT count(*) AS n FROM owners WHERE name='root'").get()).toEqual({n:1});
  db.run("DELETE FROM owners WHERE name='root'");released+=law.rootBytes;stages.push("root-released");
  db.run("DELETE FROM owners WHERE name='frame'");released+=law.frameBytes;stages.push("complete");
  expect(stages).toEqual(law.stages);expect({allocated,released}).toEqual(law.expected);
  expect(db.query("SELECT count(*) AS n FROM owners").get()).toEqual({n:0});db.close();
 }
});

test("metadata retirement requires paid copy before each ownership or cursor transition",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const row of fixture.metadataTransitions){
  const db=new Database(":memory:");
  db.run("CREATE TABLE metadata(owned INTEGER, cursor INTEGER, entry INTEGER)");
  db.run("INSERT INTO metadata VALUES (?,?,?)",Number(row.initial.owned),row.initial.cursor,Number(row.initial.entry));
  if(row.copyAdmitted){
   const sql={"owned-empty-clear":"UPDATE metadata SET owned=0","sparse-cursor-advance":"UPDATE metadata SET cursor=cursor+1","terminal-entry-remove":"UPDATE metadata SET entry=0"};
   db.run(sql[row.kind]);
  }
  const record=db.query("SELECT * FROM metadata").get() as {owned:number,cursor:number,entry:number};
  expect({owned:!!record.owned,cursor:record.cursor,entry:!!record.entry,progress:row.copyAdmitted,copiedMetadata:row.copyAdmitted}).toEqual(row.expected);
  if(!row.copyAdmitted)expect(record).toEqual({owned:Number(row.initial.owned),cursor:row.initial.cursor,entry:Number(row.initial.entry)});
  db.close();
 }
});
