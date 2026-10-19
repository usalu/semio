import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const bare=(name:string)=>name.replace(/️/g,"");
const at=(base:string,...names:string[])=>names.reduce((directory,name)=>{const entry=readdirSync(directory).find(candidate=>bare(candidate)===bare(name));if(entry===undefined)throw new Error(`${name} is absent from ${directory}`);return join(directory,entry)},base);
const decode=join(import.meta.dir,"..");
const history=join(decode,"..");
const modules=join(history,"..","..");
const framework=join(modules,"..","..","..");
const text=(path:string)=>readFileSync(path,"utf8");
const family=(directory:string):string=>readdirSync(directory).sort().map(name=>{const path=join(directory,name);if(statSync(path).isDirectory())return /(tests|schema|packages)$/.test(bare(name))?"":family(path);return name.endsWith(".rs")?text(path):""}).join("\n");
const variants=(source:string,declaration:string)=>{const start=source.indexOf(declaration);expect(start).toBeGreaterThanOrEqual(0);return source.slice(start+declaration.length,source.indexOf("}",start)).split(",").map(part=>part.trim()).filter(Boolean)};

test("original retained History decoder quotes all native custody before per-turn borrowed pieces",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("create table frontier(position integer primary key,name text unique)");
  for(const [index,name] of law.frontiers.entries())db.query("insert into frontier values(?,?)").run(index,name);
  expect(db.query("select name from frontier order by position").all()).toEqual(law.frontiers.map((name:string)=>({name})));
  db.exec("create table step(position integer primary key,name text unique)");
  for(const [index,name] of law.receiving.steps.entries())db.query("insert into step values(?,?)").run(index,name);
  expect(db.query("select count(*) as count from step").get()).toEqual({count:5});
 }finally{db.close()}

 const engine=text(at(decode,"🗂️file","🦀️.rs"));
 const everything=family(decode);
 const owner=text(at(history,"🦀️.rs"));
 const driver=owner.slice(owner.indexOf(`pub struct ${law.receiving.owner}`),owner.indexOf("async fn flush_dict_delta"));
 expect(driver.length).toBeGreaterThan(0);
 expect(engine.includes(`pub struct ${law.receiving.engine}`)).toBe(true);
 expect(variants(engine,"pub enum RetainedHistorySourceStep{")).toEqual(law.receiving.steps);
 const turn=owner.slice(owner.indexOf("pub trait RetainedHistoryTurn"),owner.indexOf("pub struct RetainedHistoryUninterrupted"));
 expect([...turn.matchAll(/fn (\w+)\(&self\)/g)].map(match=>match[1])).toEqual(law.receiving.turn);

 for(const proof of["source_identity","RetainedCloneGrant","pub fn demands<","pub fn take_receipt(","pub fn take_ready_into(","pub fn take_partial_into(","pub fn close_step<","RetainedSourceDictionary","OriginalWireSourceIdentity","control.charge"])expect(engine.includes(proof),`engine proof ${proof}`).toBe(true);
 expect(everything.includes("RetainedWireFieldDecode")).toBe(true);
 for(const proof of["pub fn source_identity(","pub fn demands(","pub fn take_receipt(","pub fn close_step<","pub fn step<","RetainedCloneGrant","RetainedHistorySourceDecode","NativeForwardedDecodeControl::rebind","control.detach()","take_ready_into(history","take_partial_into(partial","take_fault_into(fault"])expect(driver.includes(proof),`driver proof ${proof}`).toBe(true);
 const dictionary=text(at(framework,"🔨️modules","📡️replication","📖️dictionary","🛬️decode","🦀️.rs"));
 expect(dictionary.includes("original_source_ranges")).toBe(true);

 for(const cold of["decode_frame(","maximum_records","RetainedHistoryDecodeStep","take_auxiliary_owners","new_persisted_document","RetainedSprLimits","validation_stage"]){
  expect(driver.includes(cold),`driver cold ${cold}`).toBe(false);
  if(cold!=="maximum_records"&&cold!=="RetainedSprLimits")expect(everything.includes(cold),`decode family cold ${cold}`).toBe(false);
 }
 expect(/fn (step|close_step)[^{]*maximum_records/.test(everything)).toBe(false);
 for(const cold of["edit_ids:Vec<String>","edit.id.clone()","source.to_vec()"])expect(engine.includes(cold),`engine cold ${cold}`).toBe(false);
 expect(driver.includes("self.decode_frame(frame)")).toBe(false);
 expect(driver.includes("maximum_records: usize")).toBe(false);
});

test("every retained History consumer receives through the granted decoder and none keeps a whole-frame bridge",()=>{
 const facade=text(at(join(history,".."),"🦀️.rs"));
 for(const exported of["RetainedHistoryDecode","RetainedHistoryTurn","RetainedHistoryUninterrupted","RetainedHistorySourceStep"])expect(facade.includes(exported),`facade ${exported}`).toBe(true);
 const plugin=at(modules,"🔌️plugin");
 const consumers=[
  at(plugin,"🗃️document","🗄️document-archive-load","🦀️.rs"),
  at(plugin,"🧵️instance","📄️docs","📥️staging","🪜️maintenance-ladder","🦀️.rs"),
  at(plugin,"🧵️instance","🔌️plugin-app-impl","🗃️document-load","🦀️.rs"),
  at(plugin,"🪟️window","🎚️config","📥️retained","📦️typed-pack-load","🦀️.rs"),
  at(plugin,"🧪️tests","🔬️app-child-member-registry","🦀️.rs"),
  at(history,"🦀️.rs"),
  at(history,"🧪️tests","🔬️unit","🦀️.rs"),
 ];
 for(const path of consumers){
  const source=text(path);
  for(const cold of["new_persisted_document","take_auxiliary_owners","RetainedHistoryDecodeStep","RetireParentHistoryAuxiliary","retained_pending","decode_frame(","maximum_records","RetainedSprLimits"])expect(source.includes(cold),`${path} keeps ${cold}`).toBe(false);
 }
 const archive=text(consumers[0]);
 for(const proof of["document_archive_history_decoder","RetainedHistoryUninterrupted","history_custody","admit_owned_retirement"])expect(archive.includes(proof),`archive ${proof}`).toBe(true);
 const window=text(consumers[3]);
 for(const proof of["window_config_history_limits","RetainedHistoryUninterrupted","RetainedHistorySourceStep::Ready"])expect(window.includes(proof),`window ${proof}`).toBe(true);
 const ladder=text(consumers[1]);
 for(const proof of["StepTurn","RetainedHistoryTurn","decoder.step(","take_receipt()"])expect(ladder.includes(proof),`ladder ${proof}`).toBe(true);
});

test("the retained History decoder shares one language-agnostic corpus with the TypeScript frame oracle",()=>{
 const corpus=at(decode,...law.receiving.corpus.split("/"));
 expect(existsSync(corpus)).toBe(true);
 const shared=JSON.parse(text(corpus));
 expect(shared.grant.items).toBe(law.grant.items);
 expect(shared.records.length).toBeGreaterThanOrEqual(7);
 expect(shared.nativeTrials.cancelAt.length).toBeGreaterThan(0);
 const engineRust=text(at(decode,"🗂️file","🧪️tests","🦀️.rs"));
 expect(engineRust.includes("include_str!")).toBe(true);
 for(const proof of["original_history_driver_settles_every_corpus_trial_over_granted_turns_and_cancellations","RetainedHistoryDecode::new(","assert_corpus_log","nativeTrials"])expect(engineRust.includes(proof),`corpus driver test ${proof}`).toBe(true);
 const driverRust=readModuleTreeSource(at(history,"🧪️tests","🔬️unit","🦀️.rs"));
 expect(bare(driverRust).includes(bare("🛬️decode/🗂️file/🧪️tests/🧫️fixtures/🔣️.json"))).toBe(true);
 for(const proof of["RetainedHistoryDecode::new(","RetainedHistoryTurn","close_all(","nativeTrials","cancelAt"])expect(driverRust.includes(proof),`driver test ${proof}`).toBe(true);
 const oracle=text(at(decode,"🗂️file","🧪️tests","🟦️.ts"));
 for(const reference of["crc-32/crc32c.js","@webassemblyjs/leb128","bun:sqlite","ajv","./🧫️fixtures/🔣️.json"])expect(bare(oracle).includes(bare(reference)),`oracle ${reference}`).toBe(true);
});
