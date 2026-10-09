import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import {Database} from "bun:sqlite";
const base=new URL("../../",import.meta.url),read=(path:string)=>readFileSync(new URL(path,base),"utf8"),law=JSON.parse(read("🧫️fixtures/🧮️compute-retained/🔣️.json"));
test("Services mandatory retained contract validates every neutral axis and an independent SQLite receipt",()=>{
 const ajv=new Ajv2020({strict:false}).addMetaSchema(draft7).addSchema(JSON.parse(read("../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")),"https://semio.dev/schema/value/retained-clone/grant").addSchema(JSON.parse(read("../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json")),"https://semio.dev/schema/value/retained-clone/progress");
 const validate=ajv.compile(JSON.parse(read("🧬️schema/🧮️compute-retained/🔣️.json")));expect(validate(law)).toBe(true);
 for(const key of Object.keys(law.callerGrant)){const absent=structuredClone(law);delete absent.callerGrant[key];expect(validate(absent)).toBe(false);const negative=structuredClone(law);negative.callerGrant[key]=-1;expect(validate(negative)).toBe(false);}
 const db=new Database(":memory:");try{const oracle=db.query("SELECT 1 AS copiedItems, 1 AS copiedBytes, 0 AS retainedCapacityBytes, 0 AS releasedBytes").get();expect(law.metadataTurn.receipt).toEqual(oracle);}finally{db.close();}
});
test("Services and both original router callers preserve caller authority without a scalar close fallback",()=>{
 const services=read("🦀️.rs"),effects=read("../🔌️plugin/🖥️host/⚡️effects/🦀️.rs"),imports=read("../🔌️plugin/🖥️host/📥️imports/🦀️.rs");expect(services.includes("compute_job_close_grant")).toBe(false);expect(services.includes("retained, site: \"os-services.compute-job\"")).toBe(true);expect(services.includes("let authority=state.retained;")).toBe(true);expect(services.includes("let authority = state.retained;")).toBe(true);expect(effects.includes("compute.retain_outcome_for_close(outcome,retained)")).toBe(true);expect(effects.includes("outcome.close_step(1,")).toBe(false);expect(imports.includes("call.router_turn_grant, &call.router_handler")).toBe(true);expect(law.laws.every((row:{sameGrant:boolean})=>row.sameGrant)).toBe(true);
});

test("canonical Native I/O denials preserve distinct work, ownership and depth axes",()=>{
 const db=new Database(":memory:");try{db.run("CREATE TABLE denials(axis TEXT PRIMARY KEY,refusal TEXT)");for(const axis of ["items","release","depth"]){const grant={items:1,release:128,depth:1,[axis]:0};const refusal=db.query("SELECT CASE WHEN ?=0 THEN 'WorkLimit' WHEN ?<1 THEN 'DepthLimit' WHEN ?<128 THEN 'OwnershipLimit' END AS refusal").get(grant.items,grant.depth,grant.release) as {refusal:string};db.run("INSERT INTO denials VALUES(?,?)",axis,refusal.refusal);}expect(db.query("SELECT axis,refusal FROM denials ORDER BY rowid").all()).toEqual(law.nativeIoDenials);}finally{db.close();}
});
