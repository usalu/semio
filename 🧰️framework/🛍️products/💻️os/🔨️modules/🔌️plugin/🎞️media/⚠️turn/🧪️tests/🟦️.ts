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
 expect(validate({...fixture,sourceIdentity:[0,3]})).toBe(false);expect(validate({...fixture,mediaSources:fixture.mediaSources.map(source=>source.kind==="incompatible"?{...source,produced:{class:8,form:6}}:source)})).toBe(false);console.log("[DEBUG] independent Ajv/JSONPatch media pending primary retains exact Unicode fields and original business identity");
});


import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
test("media output birth requires caller-funded ownership before any allocation",()=>{
 for(const row of fixture.birth.cases){const db=new Database(":memory:");db.exec("CREATE TABLE owners (id TEXT)");db.exec("BEGIN");db.query("INSERT INTO owners VALUES (?)").run(row.id);db.exec(row.items>0&&row.capacityBytes>0?"COMMIT":"ROLLBACK");expect(db.query("SELECT COUNT(*) AS count FROM owners").get()).toEqual({count:Number(row.expected)});db.close();}
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");expect(declaration.includes("original_birth_admit")).toBe(true);const receiver=readFileSync(resolve(import.meta.dir,"../../../🧵️instance/🦀️.rs"),"utf8");expect(receiver.includes("ArtifactMediaExportCredit::original_birth_admit")).toBe(true);
 console.log("[DEBUG] media output cold birth rollback preserves denial; SQLite independent admission transaction");
});

test("media producer request contains only current source and export capabilities",()=>{
 const declaration=readFileSync(resolve(import.meta.dir,"../../../📜️declaration/🎞️media/🦀️.rs"),"utf8");const body=declaration.split("pub struct ArtifactMediaExportJobRequest<A: ArtifactApp> {")[1]!.split("\n}")[0]!;const actual=[...body.matchAll(/^    pub (\w+):/gm)].map(match=>match[1]!);
 const db=new Database(":memory:");db.exec("CREATE TABLE input (field TEXT PRIMARY KEY)");for(const field of fixture.requestFields)db.query("INSERT INTO input VALUES (?)").run(field);expect(db.query("SELECT field FROM input ORDER BY field").all()).toEqual(actual.toSorted().map(field=>({field})));db.close();
 expect(actual).toEqual(fixture.requestFields);console.log("[DEBUG] actual media producer source roster is closed; no unrelated cache/history/wire inputs; SQLite independent exact inventory");
});

test("snapshot preparation retains its original source and installed supplier through denial",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.snapshotPreparation.cases){const db=new Database(":memory:");db.exec("CREATE TABLE received (phase TEXT)");for(const [index,phase]of fixture.snapshotPreparation.phases.entries()){db.exec("BEGIN");db.query("INSERT INTO received VALUES (?)").run(phase);db.exec(row.sourceRoot===row.incomingRoot&&row.items>0&&(row.capacityBytes>0||index<2)?"COMMIT":"ROLLBACK");if(row.sourceRoot!==row.incomingRoot||row.items===0||row.capacityBytes===0&&index===2)break;}expect(db.query("SELECT phase FROM received").all()).toEqual(row.expectedPhases.map(phase=>({phase})));db.close();}
 const source=readFileSync(resolve(import.meta.dir,"../../../♻️lifecycle/🦀️.rs"),"utf8");expect(source.includes("snapshot_source")).toBe(true);
 console.log("[DEBUG] original snapshot source+installed supplier retained; SQLite independent phased denial and foreign-root oracle");
});
