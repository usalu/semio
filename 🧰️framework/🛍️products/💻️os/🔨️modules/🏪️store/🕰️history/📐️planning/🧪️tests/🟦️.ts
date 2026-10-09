/** 📐️ The real history planner owns original inputs and ordinal positions through cancellation. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import Ajv from "ajv";
test("🖼️ completed preview moves its projection while retaining original input residue",()=>{
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  for(const frontier of law.cancelAt){const original={inputs:structuredClone(law.drafts),snapshot:law.previewSnapshot,frontier};const result=applyPatch(structuredClone(original),[{op:"remove",path:"/snapshot"}],true).newDocument;expect(result).toEqual({inputs:law.drafts,frontier});expect(original.snapshot).toEqual(law.previewSnapshot);}
  const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("fn take_finished_snapshot_owner(")).toBe(true);const start=source.indexOf("pub fn finish_derived_history_preview(");const end=source.indexOf("\n    }",start);const handoff=source.slice(start,end);expect(handoff.includes("owner.as_mut()")).toBe(true);expect(handoff.includes("take_finished_snapshot_owner()")).toBe(true);expect(handoff.includes("owner.take()")).toBe(false);
  const actor=readFileSync(new URL("../../../../🔌️plugin/⏪️time-travel/🦀️.rs",import.meta.url),"utf8");expect(actor.includes("self.cancel_preview(store)?;\n        let (preview, outcome)")).toBe(true);
});
test("⏪️ original raw replay routes preserve parent custody after child completion",()=>{
  const grantSchema=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:true}).addSchema(grantSchema).compile({$ref:grantSchema.$id+"#/$defs/Grant"});for(const copy of law.copyGrants){expect(validate({maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1})).toBe(true);}
  const db=new Database(":memory:");try{db.run("CREATE TABLE owners(child INTEGER, header INTEGER, registry INTEGER, issuer INTEGER)");for(const route of law.replayRetirementRoutes){db.run("DELETE FROM owners");db.run("INSERT INTO owners VALUES(1,1,?,1)",route==="derived"?1:0);db.run("UPDATE owners SET child=0");expect((db.query("SELECT header+registry+issuer AS pending FROM owners").get()as{pending:number}).pending).toBeGreaterThan(0);const original={route,inputs:structuredClone(law.drafts),result:law.previewSnapshot};expect(applyPatch(structuredClone(original),[],true).newDocument).toEqual(original);db.run("UPDATE owners SET header=0,registry=0,issuer=0");expect(db.query("SELECT child+header+registry+issuer AS pending FROM owners").get()).toEqual({pending:0});}}finally{db.close();}
  const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("fn raw_replay_demands(")).toBe(true);expect(source.includes("fn close_raw_replay(")).toBe(true);expect(source.includes("owner.retirement_demands(body)")).toBe(true);expect(source.includes("owner.close_original_step(child)")).toBe(true);expect(source.includes("fn replay_message_demands(")).toBe(false);
});
test("📐️ actual history plan retains first-party ordinal and draft custody",()=>{
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  const db=new Database(":memory:");try{db.exec("CREATE TABLE positions(id TEXT PRIMARY KEY, edit INTEGER, operation INTEGER)");const insert=db.prepare("INSERT INTO positions VALUES(?,?,?) ON CONFLICT(id) DO UPDATE SET edit=excluded.edit,operation=excluded.operation");for(const row of law.positions){insert.run(row.id,row.edit,row.operation);expect(Buffer.byteLength(row.id)).toBeLessThanOrEqual(law.textCapacity);}expect(db.query("SELECT id,edit,operation FROM positions ORDER BY id COLLATE BINARY").all()).toEqual(law.expectedPositions);
    for(const frontier of law.cancelAt){const original=structuredClone(law.order),remaining=original.slice(0,Math.max(0,original.length-frontier));const result=applyPatch(remaining,[{op:"replace",path:"",value:[]}],true).newDocument;expect(result).toEqual([]);expect(original).toEqual(law.order);}
  }finally{db.close();}
  const owner=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(owner.includes("positions: protocol::HistoryFoldIndex<MutationId, (usize, usize)>")).toBe(true);expect(owner.includes("order: crate::os_vcs::HistoryPageStack::empty(), counts: crate::os_vcs::HistoryPageStack::empty()")).toBe(true);expect(owner.includes('mod history_plan_retirement;')).toBe(true);
  console.log("[DEBUG] neutral original planner ordinal replacement and cancellation match SQLite/RFC6902; exact first-party planner custody is mounted");
});
test("🏁️ actual preview cancellation retains each original owner through full grant retirement",()=>{
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  for(const frontier of law.cancelAt){const original={target:law.target,drafts:structuredClone(law.drafts),order:law.order.slice(0,frontier),counts:law.counts.slice(0,frontier),snapshot:structuredClone(law.previewSnapshot)};const rendered=applyPatch(structuredClone(original),[{op:"replace",path:"/drafts",value:[]},{op:"replace",path:"/order",value:[]},{op:"replace",path:"/counts",value:[]},{op:"remove",path:"/target"},{op:"remove",path:"/snapshot"}],true).newDocument;expect(rendered).toEqual({drafts:[],order:[],counts:[]});expect(original.snapshot).toEqual(law.previewSnapshot);}
  const owner=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(owner.includes("fn preview_demands(")).toBe(true);expect(owner.includes("fn close_preview(")).toBe(true);expect(owner.includes("snapshot_registry_alias_close_step(&mut self.registry_retirement,grant)")).toBe(true);
  console.log("[DEBUG] original preview cancellation keeps canonical snapshot unchanged across every neutral frontier; direct grant-preserving owner closure is mounted");
});
test("♻️ completed planning transfers result fields and retains original residue",()=>{
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  for(const route of law.completionRoutes){
    const original={target:law.target,drafts:structuredClone(law.drafts),positions:structuredClone(law.expectedPositions),order:structuredClone(law.order),counts:structuredClone(law.counts)};
    const paths=route==="preview"?["drafts"]:["drafts","order"];
    const residual=applyPatch(structuredClone(original),paths.map(key=>({op:"replace" as const,path:`/${key}`,value:[]})),true).newDocument;
    for(const key of paths)expect(residual[key as keyof typeof residual]).toEqual([]);
    expect(residual.positions).toEqual(law.expectedPositions);expect(residual.counts).toEqual(law.counts);expect(original.drafts).toEqual(law.drafts);
  }
  const owner=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(owner.includes("fn retain_completed_plan(")).toBe(true);expect(owner.includes("fn planning_retirement_demands(")).toBe(true);expect(owner.includes("fn retire_completed_plan(")).toBe(true);
  console.log("[DEBUG] completed preview/replay result partition preserves original ordinal/count residue; direct controlled plan retention and grant-preserving close are mounted");
});


test("original history cleanup authority is independent of its retirement demand",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 expect(law.cleanup).toBeDefined();
 const contract=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json",import.meta.url),"utf8"));
 const validate=new Ajv({strict:false}).compile({...contract,$ref:"#/$defs/Grant"});
 expect(validate(law.cleanup)).toBe(true);
 expect(law.bodyRefusalCapacityBytes).toBe(0);
 for(const copy of law.copyGrants)expect(validate({...law.cleanup,maximumCopyBytes:copy,maximumCapacityBytes:law.bodyRefusalCapacityBytes})).toBe(true);
 for(const field of contract.$defs.Grant.required){const omitted={...law.cleanup};delete omitted[field];expect(validate(omitted)).toBe(false);expect(validate({...law.cleanup,[field]:-1})).toBe(false);}
 const db=new Database(":memory:");
 try{
  db.run("CREATE TABLE caller(items INTEGER,copy INTEGER,capacity INTEGER,released INTEGER,depth INTEGER)");
  const p=law.cleanup;db.run("INSERT INTO caller VALUES(?,?,?,?,?)",[p.maximumItems,p.maximumCopyBytes,p.maximumCapacityBytes,p.maximumReleaseBytes,p.maximumDepth]);
  expect(db.query("SELECT items,copy,capacity,released,depth FROM caller").get()).toEqual({items:p.maximumItems,copy:p.maximumCopyBytes,capacity:p.maximumCapacityBytes,released:p.maximumReleaseBytes,depth:p.maximumDepth});
 }finally{db.close();}
 const native=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");
 expect(native.includes("fn cleanup_grant(")).toBe(true);
 expect(native.includes(".max(work)")).toBe(false);expect(native.includes(".max(demand.copy_bytes)")).toBe(false);
 const grants=[...native.matchAll(/let grant=([^;]+);/g)].map(match=>match[1]);expect(grants).toHaveLength(6);for(const grant of grants)expect(grant).toBe("cleanup_grant(&law)");
 console.log("[DEBUG] original history caller authority / genuine Grant Ajv omissions / independent SQLite all axes / actual native Source preserves supplied policy; under-copy refusal denies alternate child-shell birth");
});
