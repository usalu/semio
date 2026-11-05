import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import schema from "./schema.json";
import ownerSchema from "../schema.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("settlement refusal keeps original causes and returns both owners on repeat",()=>{
 const validate=new Ajv({strict:true});expect(validate.compile(schema)(fixture)).toBe(true);
 const error=(message:string)=>({kind:"InvalidValue",message});
 const original={cause:{type:"receiving",primary:error(fixture.messages[0]),quoteRefusal:error(fixture.messages[1]),allocationRefusal:error(fixture.messages[2]),progress:fixture.progress},receiptRefusal:null,contextRefusal:error(fixture.messages[3]),settlementRefusal:error(fixture.messages[4]),progress:fixture.progress};
 expect(validate.compile(ownerSchema)(original)).toBe(true);
 expect(validate.compile(ownerSchema)({...original,settlementRefusal:{kind:fixture.sqlite.settlementKind,message:fixture.sqlite.settlementMessage}})).toBe(true);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE causes(channel TEXT PRIMARY KEY,message TEXT); CREATE TABLE returned(message TEXT)");
  for(const [channel,message]of["primary","quote","allocation","context","settlement"].map((channel,index)=>[channel,fixture.messages[index]]))db.query("INSERT INTO causes VALUES(?,?)").run(channel,message);
  const repeat=db.query("INSERT OR IGNORE INTO causes VALUES('settlement',?)").run(fixture.messages[5]);expect(repeat.changes!==0).toBe(fixture.expected.repeatAccepted);
  if(repeat.changes===0)db.query("INSERT INTO returned VALUES(?)").run(fixture.messages[5]);
  expect(db.query("SELECT message FROM causes ORDER BY rowid").all()).toEqual(fixture.messages.slice(0,5).map(message=>({message})));
  expect(db.query("SELECT message FROM returned").all()).toEqual([{message:fixture.messages[5]}]);
  expect(Number((db.query("SELECT (SELECT count(*) FROM causes)+(SELECT count(*) FROM returned) AS owners").get()as {owners:number}).owners)).toBe(fixture.expected.preservedOwners);
 }finally{db.close();}
});
