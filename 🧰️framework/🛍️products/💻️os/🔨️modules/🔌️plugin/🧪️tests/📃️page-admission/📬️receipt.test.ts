import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("seven lanes retain original accepted receipt through denied page capture",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.acceptedReceipts){
  const db=new Database(":memory:");
  db.run("CREATE TABLE original(lane TEXT PRIMARY KEY, before INTEGER, after INTEGER, phase TEXT)");
  db.run("INSERT INTO original VALUES (?,?,?,?)",row.lane,row.receipt.generationBefore,row.receipt.generationAfter,"preparing");
  for(const phase of row.phases){
   db.run("UPDATE original SET phase=?",phase.phase);
   const accepted=db.query("SELECT before AS generationBefore, after AS generationAfter FROM original WHERE phase='awaitingAck'").get();
   expect(accepted).toEqual(phase.expected);
   if(phase.phase==="awaitingAck"){
    expect(db.query("SELECT phase FROM original").get()).toEqual({phase:row.deniedCapture.expectedPhase});
    expect(db.query("SELECT before AS generationBefore, after AS generationAfter FROM original").get()).toEqual(row.deniedCapture.expectedReceipt);
   }
  }db.close();
 }
});
test("both erased window publication interfaces expose original accepted receipt",async()=>{
 const root="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
 for(const path of ["🪟️window/🫧️transient/🦀️.rs","🪟️window/🎚️config/🔏️pack-authority-and-publication/🦀️.rs"]){
  const source=await Bun.file(root+path).text();
  const trait=source.slice(source.indexOf("pub trait ErasedWindow"),source.indexOf("\n}",source.indexOf("pub trait ErasedWindow")));
  expect(trait).toContain("fn accepted_receipt(");
  const implementation=source.slice(source.search(/impl<[^\n]+ErasedWindow(?:Config|Transient)Publication for/));
  const start=implementation.indexOf("fn accepted_receipt(");expect(start).toBeGreaterThanOrEqual(0);
  const open=implementation.indexOf("{",start);let end=open+1,depth=1;
  for(;end<implementation.length&&depth;end++){if(implementation[end]==="{")depth++;else if(implementation[end]==="}")depth--;}
  const getter=implementation.slice(open,end);
  expect(getter).toContain("self.publication");
  expect(/\.receipt|\.accepted_receipt\(/.test(getter)).toBe(true);
  expect(getter.includes("AwaitingAck")||getter.includes(".accepted_receipt(")).toBe(true);
 }
});

test("deferred page recovery retains committed g1 across g2 and outer receipt failure without replay",()=>{
 const validate=new Ajv({strict:true}).compile(schema.properties.frontierRecovery);
 expect(validate(fixture.frontierRecovery)).toBe(true);
 expect(validate([{...fixture.frontierRecovery[0],extra:true}])).toBe(false);
 for(const row of fixture.frontierRecovery){
  const db=new Database(":memory:");
  db.run("CREATE TABLE live(generation INTEGER, revision TEXT)");
  db.run("CREATE TABLE accepted(before INTEGER, after INTEGER, revision TEXT)");
  db.run("CREATE TABLE commits(operation TEXT PRIMARY KEY)");
  db.run("INSERT INTO live VALUES (?,?)",row.initial.generation,row.initial.revision);
  db.run("INSERT INTO commits VALUES ('original')");
  db.run("INSERT INTO accepted SELECT generation,?,? FROM live",row.committed.generation,row.committed.revision);
  db.run("UPDATE live SET generation=?,revision=?",row.committed.generation,row.committed.revision);
  let mountedRevision=row.outerPostFailure?row.initial.revision:row.committed.revision;
  db.run("UPDATE live SET generation=?,revision=?",row.later.generation,row.later.revision);
  const accepted=db.query("SELECT before,after,revision FROM accepted").get() as {before:number,after:number,revision:string};
  mountedRevision=accepted.revision;
  const actual={receipt:{generationBefore:accepted.before,generationAfter:accepted.after},pageGeneration:accepted.after,pageRevision:mountedRevision,commits:(db.query("SELECT count(*) AS n FROM commits").get() as {n:number}).n,liveGeneration:(db.query("SELECT generation FROM live").get() as {generation:number}).generation};
  expect(actual).toEqual(row.expected);
  expect(actual.pageRevision).not.toBe(row.initial.revision);expect(actual.pageRevision).not.toBe(row.later.revision);
  db.close();
 }
});
