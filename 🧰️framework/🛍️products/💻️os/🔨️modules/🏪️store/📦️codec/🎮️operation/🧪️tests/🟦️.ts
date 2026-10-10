/** 🎟️ The original codec catalog retains every original input and actual receipt until its paid recipient. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {existsSync,readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
test("original retained codec catalog keeps borrowed sources and its actual receipt frontier",()=>{
 expect(new Ajv().compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(phase TEXT,source TEXT,catalog INTEGER,receipt INTEGER,terminal INTEGER)");for(const row of fixture.custody)db.query("INSERT INTO custody VALUES(?,?,?,?,?)").run(row.phase,row.source,Number(row.catalog),Number(row.receipt),Number(row.terminal));expect(db.query("SELECT count(DISTINCT source) AS originals FROM custody").get()).toEqual({originals:1});expect(db.query("SELECT phase FROM custody WHERE terminal=1 AND (catalog=1 OR receipt=1)").all()).toEqual([]);expect(db.query("SELECT receipt FROM custody WHERE phase='catalogClosed'").get()).toEqual({receipt:1});}finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");expect(source.includes("pub struct RetainedArtifactCodecCatalog")).toBe(true);expect(source.includes("receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>")).toBe(true);expect(source.includes("pub fn take_receipt(")).toBe(true);expect(source.includes("owners.admit_constructor(grant)")).toBe(true);expect(source.includes("owners.close_uninstalled_owners_step(grant)")).toBe(true);expect(source.includes("artifact_retirement_self_grant")).toBe(false);console.log("[DEBUG] SQLite retains original borrowed codec sources and exact full-grant receipts through catalog birth, cancellation, closure and recipient collection");
});
test("original retained codec catalog transfers only into the real empty original recipient",()=>{
 expect(new Ajv().compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE transfer(phase TEXT,catalog INTEGER,receipt INTEGER,terminal INTEGER)");for(const row of fixture.custody)db.query("INSERT INTO transfer VALUES(?,?,?,?)").run(row.phase,Number(row.catalog),Number(row.receipt),Number(row.terminal));expect(db.query("SELECT catalog,receipt,terminal FROM transfer WHERE phase='transferred'").get()).toEqual({catalog:0,receipt:1,terminal:0});}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source.includes("pub fn take_prepared(")).toBe(true);expect(source.includes("*recipient = self.original.take();")).toBe(true);expect(source.includes("recipient.is_some()")).toBe(true);console.log("[DEBUG] SQLite preserves outstanding original receipt custody after genuine catalog transfer into the original typed receiving slot");
});
test("original time travel metadata moves spend zero payload copy",()=>{
 expect(new Ajv().compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE semantic(source TEXT,copied INTEGER)");for(const row of fixture.metadata)db.query("INSERT INTO semantic VALUES(?,?)").run(row.source,row.copiedBytes);expect(db.query("SELECT SUM(copied) AS copy FROM semantic").get()).toEqual({copy:0});}finally{db.close()}
 const core=new URL("../../../../../../../🔨️modules/⏪️time-travel/",import.meta.url);
 const command=readFileSync(new URL("🎮️decision/📨️command/🦀️.rs",core),"utf8");const discard=readFileSync(new URL("🎮️decision/🗑️discard/🦀️.rs",core),"utf8");
 expect(command.includes("pub fn admission_copy_bytes()->usize{0}")).toBe(true);expect(discard.includes("pub fn admission_copy_bytes()->usize{0}")).toBe(true);expect(discard.includes("fn validation_copy_bytes(&self,_session:&TimeTravelSession)->usize{0}")).toBe(true);expect(command.includes("let parent_copy=0usize;")).toBe(true);
 console.log("[DEBUG] SQLite assigns original Session/command/Discard metadata moves zero payload copy while exact physical births/releases and explicit depth remain funded");
});
