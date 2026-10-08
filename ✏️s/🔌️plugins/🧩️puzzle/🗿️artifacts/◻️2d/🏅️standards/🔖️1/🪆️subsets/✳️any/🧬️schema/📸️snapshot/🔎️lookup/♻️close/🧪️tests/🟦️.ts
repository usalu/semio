import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import lookup from "../../🧫️fixtures/🔣️.json";

test("Puzzle2d lookup preserves original topology while closing actual source custody through independent grant axes",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE owner(outer_idx INTEGER,inner_idx INTEGER,id TEXT);CREATE TABLE close_owner(ordinal INTEGER PRIMARY KEY,role TEXT)");
 try{
  for(const row of lookup.cases){
   db.exec("DELETE FROM owner");
   const text=(value:string)=>value.startsWith("$large:")?lookup.control.largePrefix.repeat(lookup.control.largeRepeats)+value.slice(7):value;
   if(row.scope==="handle")row.handles.forEach((handles,outer)=>handles.forEach((id,inner)=>db.query("INSERT INTO owner VALUES(?,?,?)").run(outer,inner,text(id))));
   else(row.scope==="node"?row.nodes:row.scope==="edge"?row.edges:row.regions).forEach((id,outer)=>db.query("INSERT INTO owner VALUES(?,NULL,?)").run(outer,text(id)));
   const actual=db.query("SELECT outer_idx AS outer,inner_idx AS inner FROM owner WHERE id=? ORDER BY outer_idx,inner_idx LIMIT 1").get(text(row.target));expect(actual).toEqual(row.expected);
  }
  fixture.order.forEach((role,ordinal)=>db.query("INSERT INTO close_owner VALUES(?,?)").run(ordinal,role));
  const retained=db.query("SELECT role FROM close_owner ORDER BY ordinal").all();
  expect(retained.map(row=>(row as {role:string}).role)).toEqual(fixture.order);
  expect(fixture.zero).toEqual({items:0,copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0});
  const drained=[];for(const role of fixture.order){drained.push((db.query("DELETE FROM close_owner WHERE ordinal=(SELECT MIN(ordinal) FROM close_owner) RETURNING role").get()as {role:string}).role);expect(drained.at(-1)).toBe(role)}
  expect(db.query("SELECT COUNT(*) AS count FROM close_owner").get()).toEqual({count:0});
 }finally{db.close()}
 console.log("[DEBUG] six SQLite first-owner topologies retain original UTF8 IDs; ordered output/comparison/source/target custody closes without conflating copy, capacity and release");
 const source=readFileSync(resolve(import.meta.dir,"../../🦀️.rs"),"utf8");
 for(const marker of["next_close_copy_byte_demand","next_close_capacity_byte_demand","next_close_release_byte_demand","next_close_depth_demand","grant: RetainedCloneGrant","RetainedCloneBinding::close_one"])expect(source).toContain(marker);
 expect(source).not.toContain("SnapshotRetirementStep");
});

test("Puzzle2d literal and optional text producers expose the exact active native close frontier",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE owner(ordinal INTEGER PRIMARY KEY,role TEXT)");
 try{
  for(const producer of fixture.producers){
   producer.order.forEach((role,ordinal)=>db.query("INSERT INTO owner VALUES(?,?)").run(ordinal,role));
   const retained=db.query("SELECT role FROM owner ORDER BY ordinal").all();expect(retained.map(row=>(row as {role:string}).role)).toEqual(producer.order);
   for(const role of producer.order)expect(db.query("DELETE FROM owner WHERE ordinal=(SELECT MIN(ordinal) FROM owner) RETURNING role").get()).toEqual({role});
   expect(db.query("SELECT COUNT(*) AS count FROM owner").get()).toEqual({count:0});
  }
 }finally{db.close()}
 console.log("[DEBUG] SQLite literal inverse and connection edge close original field, displaced output, then immutable mutation custody in exact native frontier order");
 const mutations=resolve(import.meta.dir,"../../../../🧬️mutations");
 for(const path of["🏷️literal-id-inverse/🦀️.rs","🪢️connect-handles/🎮️prepare/🪢️edge/🦀️.rs","🔤️text/↩️inverse/🦀️.rs","🔤️text/🎮️prepare/📸️candidate/🦀️.rs","📍move-node/🎮️prepare/↩️inverse/🦀️.rs","📍move-node/🎮️prepare/📸️candidate/🦀️.rs","⚑️flag/🎮️prepare/↩️inverse/🦀️.rs","⚑️flag/🎮️prepare/📸️candidate/🦀️.rs","🌱create-node/🎮️prepare/📸️candidate/🦀️.rs","🗑️delete-node/🎮️prepare/↩️inverse/🦀️.rs","🗑️delete-node/🎮️prepare/📸️candidate/🦀️.rs","🪢️connect-handles/🎮️prepare/📸️candidate/🦀️.rs","🎮️prepare/🧰️child/🦀️.rs"]){
  const source=readFileSync(resolve(mutations,path),"utf8");
  for(const marker of["next_close_copy_byte_demand","next_close_capacity_byte_demand","next_close_release_byte_demand","next_close_depth_demand"])expect(source).toContain(marker);
  expect(source).not.toContain("SnapshotRetirementStep");
 }
});


test("Puzzle2d snapshot clones admit original immutable source custody and close it after every alias",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE backing(role TEXT PRIMARY KEY,bytes INTEGER);CREATE TABLE close_owner(ordinal INTEGER PRIMARY KEY,role TEXT)");
 try{
  const admission=fixture.sourceCustody.admission;
  const birth=admission.leaseBytes+admission.payloadBytes;
  const admit=(capacity:number)=>db.transaction(()=>{if(capacity<birth)return false;db.query("INSERT INTO backing VALUES(?,?)").run("lease",admission.leaseBytes);db.query("INSERT INTO backing VALUES(?,?)").run("payload",admission.payloadBytes);return true})();
  expect(admit(admission.belowCapacity)).toBe(false);expect(db.query("SELECT SUM(bytes) AS bytes FROM backing").get()).toEqual({bytes:null});
  expect(admit(admission.admittedCapacity)).toBe(true);expect(db.query("SELECT SUM(bytes) AS bytes FROM backing").get()).toEqual({bytes:birth});
  fixture.sourceCustody.order.forEach((role,ordinal)=>db.query("INSERT INTO close_owner VALUES(?,?)").run(ordinal,role));
  for(const role of fixture.sourceCustody.order)expect(db.query("DELETE FROM close_owner WHERE ordinal=(SELECT MIN(ordinal) FROM close_owner) RETURNING role").get()).toEqual({role});
 }finally{db.close()}
 console.log("[DEBUG] SQLite source birth admits lease+payload only together; cursor/output aliases close before immutable original source metadata");
 const root=resolve(import.meta.dir,"../../../../../../../../..");
 const helper=readFileSync(resolve(root,"🧪️tests/🔗️source-custody/🦀️.rs"),"utf8");
 expect(helper).toContain("pub(crate) fn admit_source");
 expect(helper).toContain("pub(crate) fn close_source");
 const native=readFileSync(resolve(import.meta.dir,"../../../🧪️tests/🧬️retained-clone/🦀️.rs"),"utf8");
 expect(native).toContain("test_source_custody::admit_source");
 expect(native).toContain("test_source_custody::close_source");
 expect(native).not.toContain("from_authority");
 expect(native).not.toContain("RetainedCloneBorrowAuthority::new");
 expect(native).not.toContain("close_granted");
});
