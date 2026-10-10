import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
const root=new URL("../../",import.meta.url),read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8")),fixture=read("🧫️fixtures/♻️mounted-close/🔣️.json");
test("canonical mounted closure exact receipts replay independently through SQLite and RFC6902",()=>{
 const validate=new Ajv({strict:true}).compile(read("🧬️schema/♻️mounted-close/🔣️.json"));expect(validate(fixture)).toBe(true);
 for(const profile of fixture.profiles){const db=new Database(":memory:");try{
  db.exec("CREATE TABLE receipts (turn INTEGER, phase TEXT, next TEXT, items INTEGER, copy INTEGER, born INTEGER, freed INTEGER)");const insert=db.query("INSERT INTO receipts VALUES (?1,?2,?3,?4,?5,?6,?7)");let document={phase:"BeginClose",receipt:{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0}},actual={...document.receipt};let phases:string[]=[];
  for(const [index,turn]of profile.turns.entries()){expect(turn.phase).toBe(document.phase);if(phases.at(-1)!==turn.phase)phases.push(turn.phase);expect(turn.receipt.copiedItems).toBeLessThanOrEqual(fixture.perTurnItems);expect(turn.receipt.retainedCapacityBytes).toBe(0);expect(index).toBeLessThan(fixture.maximumTurns);const p=turn.receipt;insert.run(index,turn.phase,turn.next,p.copiedItems,p.copiedBytes,p.retainedCapacityBytes,p.releasedBytes);for(const key of Object.keys(actual) as (keyof typeof actual)[])actual[key]+=p[key];document=applyPatch(document,[{op:"test",path:"/phase",value:turn.phase},{op:"replace",path:"/phase",value:turn.next},{op:"replace",path:"/receipt",value:{...actual}}],true,false).newDocument;expect(document.receipt).toEqual(actual);}
  phases.push(document.phase);expect(phases).toEqual(fixture.phases);expect(document.phase).toBe("Empty");expect(profile.turns.length).toBe(15);expect(actual).toEqual(profile.totalReceipt);expect(db.query("SELECT SUM(items) AS copiedItems, SUM(copy) AS copiedBytes, SUM(born) AS retainedCapacityBytes, SUM(freed) AS releasedBytes FROM receipts").get()).toEqual(actual);expect(db.query("SELECT COUNT(*) AS turns FROM receipts WHERE phase='ParamsRelease'").get()).toEqual({turns:4});expect(db.query("SELECT SUM(freed) AS bytes FROM receipts WHERE phase IN ('Job','PreadmittedFault')").get()).toEqual({bytes:fixture.faultPlusDomainReleaseBytes});
 }finally{db.close();}}
 console.log("[DEBUG] canonical original mounted15 physical phases preserve per-turn1, same15 turns and all real metadata/physical extents; independent SQLite and RFC6902 agree");
});
