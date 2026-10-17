import {test,expect} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("media domain admission retains complete original fields until paid publication",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const row of fixture.cases){const state={source:row.source,published:null as typeof row.source|null};const next=row.expected==="published"?applyPatch(state,[{op:"copy",from:"/source",path:"/published"}],true,false).newDocument:state;expect(next.source).toEqual(row.source);expect(next.published).toEqual(row.expected==="published"?row.source:null);}
 for(const source of[...fixture.mediaSources,...fixture.dispatchSources,...fixture.artifactSources]){const copied=applyPatch({source,projected:null as typeof source|null},[{op:"copy",from:"/source",path:"/projected"}],true,false).newDocument;expect(copied.projected).toEqual(source);}
 expect(Buffer.from(fixture.closing.payload).toString("hex")).toBe("0102030405");expect(fixture.closing.copyWidths).toEqual([1,3,64]);expect(fixture.closing.structuralCopyBytes).toBe(512);
 expect(validate({...fixture,sourceIdentity:[0,3]})).toBe(false);expect(validate({...fixture,mediaSources:fixture.mediaSources.map(source=>source.kind==="incompatible"?{...source,produced:{class:8,form:6}}:source)})).toBe(false);
});


import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import { familyText } from "../../../🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";
test("media output birth requires caller-funded ownership before any allocation",()=>{
 for(const row of fixture.birth.cases){const db=new Database(":memory:");db.exec("CREATE TABLE owners (id TEXT)");db.exec("BEGIN");db.query("INSERT INTO owners VALUES (?)").run(row.id);db.exec(row.items>0&&row.capacityBytes>0&&row.copyBytes>0&&row.depth>0?"COMMIT":"ROLLBACK");expect(db.query("SELECT COUNT(*) AS count FROM owners").get()).toEqual({count:Number(row.expected)});db.close();}
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");expect(declaration.includes("original_birth_admit")).toBe(true);const receiver=[["🖼️render/🦀️.rs","🧷️command-routes/🦀️.rs","🧾️typed-operations/🦀️.rs","🔌️plugin-app-impl/🦀️.rs"].map(s=>readFileSync(resolve(import.meta.dir,"../../../🧵️instance/"+s),"utf8")).join("\n"),familyText()].join("\n");expect(receiver.includes("ArtifactMediaExportCredit::original_birth_admit")).toBe(true);
});

test("media producer request contains only current source and export capabilities",()=>{
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🧩️app/🛬️lifted/🎬️media-export/🦀️.rs"),"utf8");const body=declaration.split("pub struct ArtifactMediaExportJobRequest<A: ArtifactApp> {")[1]!.split("\n}")[0]!;const actual=[...body.matchAll(/^    pub (\w+):/gm)].map(match=>match[1]!);
 const db=new Database(":memory:");db.exec("CREATE TABLE input (field TEXT PRIMARY KEY)");for(const field of fixture.requestFields)db.query("INSERT INTO input VALUES (?)").run(field);expect(db.query("SELECT field FROM input ORDER BY field").all()).toEqual(actual.toSorted().map(field=>({field})));db.close();
 expect(actual).toEqual(fixture.requestFields);
});

test("snapshot preparation retains its original source and installed supplier through denial",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.snapshotPreparation.cases){const db=new Database(":memory:");db.exec("CREATE TABLE received (phase TEXT)");for(const [index,phase]of fixture.snapshotPreparation.phases.entries()){db.exec("BEGIN");db.query("INSERT INTO received VALUES (?)").run(phase);db.exec(row.sourceRoot===row.incomingRoot&&row.items>0&&(row.capacityBytes>0||index<2)?"COMMIT":"ROLLBACK");if(row.sourceRoot!==row.incomingRoot||row.items===0||row.capacityBytes===0&&index===2)break;}expect(db.query("SELECT phase FROM received").all()).toEqual(row.expectedPhases.map(phase=>({phase})));db.close();}
 const source=readFileSync(resolve(import.meta.dir,"../../../♻️lifecycle/🦀️.rs"),"utf8");expect(source.includes("snapshot_source")).toBe(true);
});


test("media producer keeps paired input and body through each denied publication phase",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.producerPreparation.cases){const db=new Database(":memory:");db.exec("CREATE TABLE owners (phase TEXT)");for(const [index,phase]of fixture.producerPreparation.phases.entries()){db.exec("BEGIN");db.query("INSERT INTO owners VALUES (?)").run(phase);const admitted=row.sourceRoot===row.incomingRoot&&index<row.deniedPhase;db.exec(admitted?"COMMIT":"ROLLBACK");if(!admitted)break;}expect(db.query("SELECT phase FROM owners").all()).toEqual(row.expectedPhases.map(phase=>({phase})));db.close();}
 const sdk=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🧩️app/🦀️.rs"),"utf8");expect(sdk.includes("body:&mut Option<Box<dyn crate::tool_contract::ArtifactReservedJob+Send>>")).toBe(true);
});


test("completion publication preserves original result on every refused receiving slot",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.completionPublication){const db=new Database(":memory:");db.exec("CREATE TABLE custody (owner TEXT PRIMARY KEY)");db.query("INSERT INTO custody VALUES (?)").run("source");db.exec("BEGIN");db.query("UPDATE custody SET owner=? WHERE owner=?").run("receiver","source");db.exec(row.funded&&!row.busy&&!row.occupied?"COMMIT":"ROLLBACK");expect(db.query("SELECT owner FROM custody").all()).toEqual([{owner:row.expectedReceiver?"receiver":"source"}]);expect(row.expectedSource).toBe(!row.expectedReceiver);db.close();}
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");expect(declaration.includes("pub fn complete_original")).toBe(true);
});

test("media handle receiving retains custody before posting even when the receipt refuses",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.handleReceiving){const db=new Database(":memory:");db.exec("CREATE TABLE custody (owner TEXT PRIMARY KEY)");db.exec("INSERT INTO custody VALUES ('source')");if(row.occupied)db.exec("INSERT INTO custody VALUES ('receiver')");db.exec("BEGIN");if(!row.occupied)db.exec("UPDATE custody SET owner='receiver' WHERE owner='source'");db.exec(row.items&&row.copy&&!row.occupied?"COMMIT":"ROLLBACK");const owners=db.query("SELECT owner FROM custody ORDER BY owner").all();expect(owners).toEqual([...(row.expectedReceiver?[{owner:"receiver"}]:[]),...(row.expectedSource?[{owner:"source"}]:[])]);db.close();}
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");expect(declaration.includes("pub fn receive_original")).toBe(true);const runtime=readFileSync(resolve(import.meta.dir,"../../../⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs"),"utf8");expect(runtime.includes("media_submission:ManuallyDrop<Option<ArtifactMediaExportHandle>>")).toBe(true);
});

test("terminal download receiving keeps every result root until its paid registry recipient",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.downloadReceiving.cases){const db=new Database(":memory:");db.exec("CREATE TABLE received (phase TEXT)");for(const [index,phase]of fixture.downloadReceiving.phases.entries()){db.exec("BEGIN");db.query("INSERT INTO received VALUES (?)").run(phase);const accepted=row.sameSource&&!row.occupied&&index<row.deniedPhase;db.exec(accepted?"COMMIT":"ROLLBACK");if(!accepted)break;}expect(db.query("SELECT phase FROM received").all()).toEqual(row.expectedPhases.map(phase=>({phase})));db.close();}
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");expect(declaration.includes("receive_media_export_original")).toBe(true);expect(declaration.includes("media_export_metadata: Option<(MediaType,String)>")).toBe(true);
});

test("original media chunk donation retains its receiving bytes before paid EOF close",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.chunkReceiving){const db=new Database(":memory:");db.exec("CREATE TABLE custody (owner TEXT)");if(!row.empty)db.exec("INSERT INTO custody VALUES ('queue')");db.exec("BEGIN");db.exec("UPDATE custody SET owner='receiver'");db.exec(row.funded&&!row.busy?"COMMIT":"ROLLBACK");expect(db.query("SELECT owner FROM custody").all()).toEqual(row.expectedOwner==="empty"?[]:[{owner:row.expectedOwner}]);expect(row.terminal).toBe(row.empty&&row.funded&&!row.busy);db.close();}
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");expect(declaration.includes("take_chunk_original")).toBe(true);const media=readFileSync(resolve(import.meta.dir,"../../../🎞️media/🦀️.rs"),"utf8");expect(media.includes("MediaDownloadRetirement")).toBe(true);
});
