/** 🎟️ Independent lifecycle admission and actual native contract custody. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { pluginFamilyText } from "../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
import { jobRetirementPayloadFixtureUrl } from "@semio-tech/framework/job/testing/fixtures";
test("live cleanup yields to the owner of the next typed command turn", async () => {
  const rows = await Bun.file(new URL("🧫️fixtures/⏩️foreground/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const owner = db.query("SELECT CASE ? WHEN 'worker' THEN 'maintenance' WHEN 'retirement' THEN 'maintenance' WHEN 'presentedPage' THEN 'host' WHEN 'emissionAdmission' THEN 'dispatch' ELSE 'publication' END AS owner");
    for (const row of rows) expect(owner.get(row.stage)).toEqual({ owner: row.owner });
  } finally { db.close(); }
  const source = readFileSync(new URL("../../🪜️maintenance-ladder/🦀️.rs", import.meta.url), "utf8");
  const live = source.slice(source.indexOf("fn maintenance_tool_selection"), source.indexOf("fn maintenance_stage_demand"));
  expect(live).toContain("MaintenanceTool::AwaitingPublication");
  expect(live).toContain("self.tools.original_emission_admission.is_some()");
});
test("mounted maintenance shares the ladder while preparation and close keep their own authority", async () => {
  const rows = await Bun.file(new URL("🧫️fixtures/🪴️live-owner/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const select = db.query(`SELECT
      CASE WHEN $phase = 'preparation' AND NOT $handoff THEN 'mounted' ELSE 'ladder' END AS live,
      CASE WHEN $resident THEN 'mounted' ELSE 'ladder' END AS close,
      CASE WHEN $handoff AND $resident THEN 'retire'
        WHEN $phase = 'maintenance' AND NOT $handoff THEN 'maintain' END AS mountedStep`);
    for (const row of rows) {
      expect(select.get({ $phase: row.phase, $resident: Number(row.resident), $handoff: Number(row.documentHandoff) }))
        .toEqual({ live: row.live, close: row.close, mountedStep: row.mountedStep });
    }
  } finally { db.close(); }
  const source = readFileSync(new URL("../../🧵️instance/🔌️plugin-app-impl/🦀️.rs", import.meta.url), "utf8");
  const live = source.slice(source.indexOf("fn mounted_owner_maintenance_phase"), source.indexOf("fn mounted_owner_close_step"));
  expect(live).toContain("A::mounted_job_maintenance_phase(identity.instance_id) != Some(MountedOwnerPhaseV1::Preparation)");
  expect(live).toContain("self.ingest.document_blocks_pending_effects()");
  expect(live).not.toContain("mounted_jobs_terminal_is_empty");
  const ladder = readFileSync(new URL("../../🪜️maintenance-ladder/🦀️.rs", import.meta.url), "utf8");
  const mountedQuote = ladder.slice(ladder.indexOf("15 => match self.tools.live_runtime_instance_id"), ladder.indexOf("16 => (!self.tools"));
  expect(mountedQuote).toContain("A::mounted_job_maintenance_phase(instance_id) == Some(MountedOwnerPhaseV1::Maintenance)");
  expect(mountedQuote).toContain("self.ingest.document_blocks_pending_effects()");
  expect(mountedQuote).toContain("A::mounted_job_close_demands(instance_id, body)");
  expect(ladder).toContain("A::mounted_job_close_step(instance_id, cx, grant)");
  expect(ladder).toContain("self.ingest.advance_snapshot_read_returns_one");
});
test("archive dependencies can use the limited maintenance grant", async () => {
  const law = await Bun.file(new URL("🧫️fixtures/🗄️archive-credit/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const eligible = db.query(`SELECT CASE
      WHEN NOT $pending OR $stage IN (8, 14, 15) THEN 1
      WHEN $stage = 24 THEN $items >= $minimum
      ELSE $items > $minimum END AS allowed`);
    for (const row of law.cases) {
      expect(eligible.get({ $pending: Number(row.archivePending), $stage: row.stage, $items: row.items, $minimum: law.archiveMinimumItems }))
        .toEqual({ allowed: Number(row.eligible) });
    }
  } finally { db.close(); }
  const scheduler = readFileSync(new URL("../../🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs", import.meta.url), "utf8");
  expect(scheduler).toContain("matches!(unit, MaintenanceUnit::Stage(8 | 14 | 15))");
  expect(scheduler).toContain("!archive && !archive_dependency && available_items <= DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS");
});
test("mounted worker forwards its original paid turn before a separate borrowed semantic turn",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🪟️outcome-borrow/🔣️.json",import.meta.url)).json(),law=fixture.workerReceipt;
 expect(new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["originalGrant","receipt","consumptionCount","semanticOnReceiptTurn","poolWrapperBirth","metadataCopyCurrency","originalOutcomeUntilAcknowledgement"],properties:{originalGrant:{const:[1,17,8192,65536,9]},receipt:{const:[1,17,0,0]},consumptionCount:{const:1},semanticOnReceiptTurn:{const:false},poolWrapperBirth:{const:false},metadataCopyCurrency:{const:0},originalOutcomeUntilAcknowledgement:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_worker(axis INTEGER,grant INTEGER,receipt INTEGER)");law.receipt.forEach((receipt:number,axis:number)=>db.query("INSERT INTO original_worker VALUES(?,?,?)").run(axis,law.originalGrant[axis],receipt));expect(db.query("SELECT count(*) AS n FROM original_worker WHERE receipt>grant").get()).toEqual({n:0});expect(db.query("SELECT grant-receipt AS remaining FROM original_worker ORDER BY axis").all().map((row:any)=>row.remaining)).toEqual([0,0,8192,65536]);}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("fn drive_worker_step(&mut self,"),body=source.slice(start,source.indexOf("fn next_token",start));expect(start>=0).toBe(true);expect(body.includes("session.take_checked_out_retained_step_receipt()")).toBe(true);expect(body.includes("cx.with_retained_loan(grant,|loaned|loaned.with_original_work_loan(step_fuel,step_deadline_us,|same|session.pump_one_for_interactive_turn(same)))")).toBe(true);expect(body.includes("worker_semantic_pending")).toBe(true);expect(body.includes("Progress(progress)")).toBe(true);expect(body.includes("pump_one_for_interactive_turn(pool")).toBe(false);
});
test("mounted worker borrows its actual descriptor until paid acknowledgement and separate resume",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🪟️outcome-borrow/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["contract","originalGrant","payloadOwner","nativeAddressUnchanged","byValueHandback","semanticBorrowCount","metadataCopyCurrency","turns"],properties:{contract:{const:"framework.plugin.original-mounted-outcome-borrow"},originalGrant:{const:[1,4096,65536,262144,64]},payloadOwner:{const:"original-worker-job"},nativeAddressUnchanged:{const:true},byValueHandback:{const:false},semanticBorrowCount:{const:1},metadataCopyCurrency:{const:0},turns:{type:"array",minItems:4,maxItems:4}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE handback(phase TEXT,items INTEGER,copy INTEGER)");for(const turn of law.turns)db.query("INSERT INTO handback VALUES(?,?,?)").run(turn.phase,turn.items,turn.copy);expect(db.query("SELECT sum(items) AS items,sum(copy) AS copy FROM handback").get()).toEqual({items:4,copy:0});}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("fn drive_worker_step(&mut self,"),body=source.slice(start,source.indexOf("fn next_token",start));expect(start>=0).toBe(true);expect(body.includes("checked_out_outcome()")).toBe(true);expect(body.includes("acknowledge_checked_out_outcome(cx)")).toBe(true);expect(body.includes("JobOutcomeView::")).toBe(true);expect(body.includes("take_checked_out_outcome()")).toBe(false);expect(body.includes("JobOutcomeSlot::from_outcome")).toBe(false);expect(body.includes("worker_outcome_pending")).toBe(true);expect(body.includes("copied_bytes:size_of::<bool>()")).toBe(false);
});
test("mounted fault capture copies original bounded pages before acknowledgement without cold conversion",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🪟️outcome-borrow/🔣️.json",import.meta.url)).json(),capture=law.faultCapture;expect(new Ajv().compile({type:"object",required:["originalGrant","sourceBytes","pageBytes","capacityBytes","metadataCopyCurrency","originalDescriptorUntilCaptureComplete","decodeOrReencodeDuringCapture","copyReceipts","semanticResumeAfterAcknowledgement"],properties:{originalGrant:{const:[1,17,8192,65536,9]},sourceBytes:{const:321},pageBytes:{const:[321]},capacityBytes:{const:480},metadataCopyCurrency:{const:0},originalDescriptorUntilCaptureComplete:{const:true},decodeOrReencodeDuringCapture:{const:false},copyReceipts:{type:"array",minItems:19,maxItems:19},semanticResumeAfterAcknowledgement:{const:true}}})(capture)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_fault(sequence INTEGER,bytes INTEGER)");capture.copyReceipts.forEach((bytes:number,sequence:number)=>db.query("INSERT INTO original_fault VALUES(?,?)").run(sequence,bytes));expect(db.query("SELECT sum(bytes) AS copied,max(bytes) AS largest,count(*) AS turns FROM original_fault").get()).toEqual({copied:321,largest:17,turns:19});}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("fn drive_worker_step(&mut self,"),body=source.slice(start,source.indexOf("fn next_token",start));expect(source.includes("struct MountedWorkerFaultCapture")).toBe(true);expect(body.includes("worker_fault_capture")).toBe(true);expect(body.includes("advance_original_fault_capture")).toBe(true);expect(body.includes("JobOutcomeView::Fault")).toBe(true);expect(body.includes("ArtifactBoundedToolFault::from_retained_fault")).toBe(false);expect(body.includes("reducer_fault_of_detail")).toBe(false);
});
test("mounted retirement preserves its original Close receipt through the actual recipient",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/♻️mounted-receipt/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["contract","originalGrant","phase","currencies","operationSource","recipient","nextProducerBeforeAcknowledgement","receipts","total"],properties:{contract:{const:"framework.plugin.original-mounted-close-receipt"},originalGrant:{const:[1,4096,65536,262144,64]},phase:{const:"Close"},currencies:{const:["items","copy","capacity","release"]},operationSource:{const:"original-mounted-operation"},recipient:{const:"original-reactor-context"},nextProducerBeforeAcknowledgement:{const:false},receipts:{type:"array",items:{type:"array",minItems:4,maxItems:4,items:{type:"integer",minimum:0}}},total:{const:[2,0,0,8192]}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_close(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER)");for(const receipt of law.receipts)db.query("INSERT INTO original_close VALUES(?,?,?,?)").run(...receipt);expect(Object.values(db.query("SELECT sum(items),sum(copy),sum(capacity),sum(release) FROM original_close").get()as object)).toEqual(law.total);}finally{db.close();}
 const source=pluginFamilyText();expect(source.includes("fn take_typed_operation_retirement_receipt(&mut self)")).toBe(true);expect(source.includes("retirement_receipt=app.take_typed_operation_retirement_receipt()")).toBe(true);expect(source.includes("original_retirement_receipt=Some((grant,progress))")).toBe(true);expect(source.includes("collected.1.retirement_receipt=output.retirement_receipt")).toBe(true);
});
test("original exchange borrows current context and preserves one receipt recipient",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🤝️exchange-context/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["phasePolicy","incomingRemaining","admitted","receiptConsumer","secondProducerBeforeReceiptAcknowledgement"],properties:{phasePolicy:{const:[1,4096,65536,262144,64]},incomingRemaining:{const:[3,17,8192,65536,9]},admitted:{const:[1,17,8192,65536,9]},receiptConsumer:{const:"original-reactor-context"},secondProducerBeforeReceiptAcknowledgement:{const:false}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_grants(axis INTEGER,phase INTEGER,incoming INTEGER)");for(let axis=0;axis<5;axis++)db.query("INSERT INTO original_grants VALUES(?,?,?)").run(axis,law.phasePolicy[axis],law.incomingRemaining[axis]);expect(db.query("SELECT min(phase,incoming) AS admitted FROM original_grants ORDER BY axis").all().map((row:any)=>row.admitted)).toEqual(law.admitted);}finally{db.close();}
 const source=pluginFamilyText();for(const name of["plugin_exchange","plugin_continue_typed_operations","advance_typed_operation_output_with_leftover"]){const start=source.indexOf(`fn ${name}<`),signature=source.slice(start,source.indexOf(" -> ",start));expect(start>=0).toBe(true);expect(signature.includes("cx: &mut semio_framework_job::StepContext<'_>")).toBe(true);}
 expect(source.includes("original_plugin_turn_grant")).toBe(true);expect(source.includes("cx.retained_grant()")).toBe(true);expect(source.includes("output.ownership_receipt_pending()")).toBe(true);
});
test("mounted child close uses full original frontier and separate parent removal",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🪆️mounted-child-close/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["originalGrant","phase","metadataCopyCurrency","childCompletionIsParentProgress","originalResultAcknowledgementRequired","parentRemovalIsSeparate"],properties:{originalGrant:{const:[1,4096,65536,262144,64]},phase:{const:"Close"},metadataCopyCurrency:{const:0},childCompletionIsParentProgress:{const:true},originalResultAcknowledgementRequired:{const:true},parentRemovalIsSeparate:{const:true}}})(law)).toBe(true);
 const source=pluginFamilyText(),frontier=readFileSync(new URL("../../🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs",import.meta.url),"utf8");expect(frontier.includes("pending_child_publication")).toBe(true);expect(frontier.includes("original mounted child publication")).toBe(true);expect(frontier.includes("drop(self.pending_child_publication.take())")).toBe(true);const start=source.indexOf("fn retirement_step(&mut self, maximum_items: usize, maximum_bytes: usize)"),owner=source.slice(start,source.indexOf("fn terminal_is_empty",start));expect(owner.includes("pending.close_step(maximum_items, maximum_bytes)")).toBe(false);
});
test("archive retirement admits original history and preserves whole physical custody",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🗃️archive-close/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["originalGrant","historyTypes","archiveOriginalRetained","historyAdmissionBeforeHandoff","logicalTruncationIsPhysicalRelease","childCompletionIsParentProgress","metadataCopyCurrency","work"],properties:{originalGrant:{const:[1,4096,65536,262144,64]},historyTypes:{type:"array",minItems:8,maxItems:8},archiveOriginalRetained:{const:true},historyAdmissionBeforeHandoff:{const:true},logicalTruncationIsPhysicalRelease:{const:false},childCompletionIsParentProgress:{const:true},metadataCopyCurrency:{const:0},work:{type:"object",required:["denied","requiresOriginalCaller","post","preserveIndependentRefusals","wholeEffects","cases"],properties:{denied:{const:0},requiresOriginalCaller:{const:true},post:{const:"max(reported,observed)"},preserveIndependentRefusals:{const:true},wholeEffects:{const:"original-physical-progress"},cases:{type:"array",minItems:4,items:{type:"array",minItems:3,maxItems:3,items:{type:"integer",minimum:0}}}}}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_archive(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");db.query("INSERT INTO original_archive VALUES(?,?,?,?,?)").run(...law.originalGrant);expect(Object.values(db.query("SELECT items,copy,capacity,release,depth FROM original_archive").get()as object)).toEqual(law.originalGrant);db.run("CREATE TABLE original_work(reported INTEGER,observed INTEGER,posted INTEGER)");for(const row of law.work.cases)db.query("INSERT INTO original_work VALUES(?,?,?)").run(...row);expect(db.query("SELECT count(*) AS mismatches FROM original_work WHERE posted != max(reported,observed)").get()).toEqual({mismatches:0});}finally{db.close();}
 const history=readModuleTreeSource(new URL("../../../📡️spr/📜️history/🧬️schema/🦀️.rs",import.meta.url));for(const type of law.historyTypes){const start=history.indexOf(`pub struct ${type} `);expect(start>=0).toBe(true);const derive=history.slice(history.lastIndexOf("#[derive(",start),start);expect(derive.includes("semio_framework_value::RetireOwned")).toBe(true);}
 const source = pluginFamilyText();
 const start = source.indexOf("impl<P, Mutation> ActiveDocumentArchiveLoad<P, Mutation>");
 const owner = source.slice(start, source.indexOf("impl<P, Mutation> Drop for ActiveDocumentArchiveLoad", start));
 expect(owner).toContain("fn retirement_demands(&self, body: usize)");
 expect(owner).toContain("fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect");
 expect(owner).toContain("let grant=effect.grant()");
 expect(owner).toContain("Exit::settle(effect,result)");
 expect(owner).toContain("RetainedWorkOutcome");
 expect(owner).toContain("last_turn_native_work");
 expect(owner).toContain("archive_retirement");
 expect(owner).toContain("history_custody");
 expect(owner).toContain("admit_owned_retirement");
 expect(owner).not.toContain(".truncate(");
 expect(owner).not.toContain("field.pop()");
 expect(owner).toContain("let release_bytes = self.fault.capacity()");
 const retirementStart = source.indexOf("fn drive_document_archive_original(");
 const retirement = source.slice(retirementStart, source.indexOf("pub const DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS", retirementStart));
 expect(retirement).toMatch(/if !matches!\(funded, Ok\(true\)\)[\s\S]*if closing && active\.terminal\(\) && active\.inputs_are_empty\(\)/);
 expect(retirement).toContain("let demand = active.terminal_removal_demands()");
 expect(retirement).toContain("released_bytes: demand.release_bytes");
});
test("prepared child retirement quotes original headers and separates visibility custody",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/📦️prepared-child/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["originalGrant","metadataFields","metadataCopyCurrency","metadataRemovalSeparate","visibilityRemovalSeparate","readReturnPreservesOriginalReceipt","childCompleteIsParentProgress"],properties:{originalGrant:{const:[1,4096,65536,262144,64]},metadataFields:{const:7},metadataCopyCurrency:{const:0},metadataRemovalSeparate:{const:true},visibilityRemovalSeparate:{const:true},readReturnPreservesOriginalReceipt:{const:true},childCompleteIsParentProgress:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_fields(capacity INTEGER,fields INTEGER)");for(const capacity of law.originalCapacities)db.query("INSERT INTO original_fields VALUES(?,?)").run(capacity,law.metadataFields);expect(db.query("SELECT sum(capacity*fields) AS physical FROM original_fields").get()).toEqual({physical:489230});}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("impl PreparedChildContentEntry"),owner=source.slice(start,source.indexOf("impl Drop for PreparedChildContentEntry",start));expect(owner.includes("fn retirement_demands(&self)")).toBe(true);expect(owner.includes("grant.maximum_copy_bytes < demand.copy_bytes")).toBe(true);expect(owner.includes("grant.maximum_capacity_bytes < demand.capacity_bytes")).toBe(true);expect(owner.includes("grant.maximum_depth < demand.depth")).toBe(true);expect(owner.includes("drop(self.visibility.take())")).toBe(true);expect(owner.includes("admit_retained_clone_progress")).toBe(true);
 const group=readFileSync(new URL("../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs",import.meta.url),"utf8");expect(group.includes("row.entry.as_ref().unwrap().retirement_demands()")).toBe(true);
});
test("prepared child admission relays original read and immutable insertion receipts",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/📦️prepared-child/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["metadataCopyCurrency","readAdmissionPreservesOriginalReceipt","immutableInsertionPreservesOriginalReceipt"],properties:{metadataCopyCurrency:{const:0},readAdmissionPreservesOriginalReceipt:{const:true},immutableInsertionPreservesOriginalReceipt:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_policy(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");db.query("INSERT INTO original_policy VALUES(?,?,?,?,?)").run(...law.originalGrant);expect(Object.values(db.query("SELECT items,copy,capacity,release,depth FROM original_policy").get()as object)).toEqual(law.originalGrant);}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("impl PreparedChildContentEntry"),owner=source.slice(start,source.indexOf("impl Drop for PreparedChildContentEntry",start));expect(owner.includes("fn preparation_demands()")).toBe(true);expect(owner.includes("Result<Option<(Self, RetainedCloneProgress)>, ValueError>")).toBe(true);expect(owner.includes("Some((generation, revision, snapshot, progress))")).toBe(true);expect(owner.includes("copy_bytes: std::mem::size_of")).toBe(false);
 const insertStart=source.indexOf("pub fn insert_prepared_entry"),insert=source.slice(insertStart,source.indexOf("pub fn prepared_entry_release_bytes",insertStart));expect(insert.includes("Result<Option<(Self, PreparedChildContentSlot, RetainedCloneProgress)>, ValueError>")).toBe(true);expect(insert.includes("PreparedChildContentEntry::admits(grant, demand)")).toBe(true);expect(insert.includes("retained_capacity_bytes: demand.capacity_bytes")).toBe(true);
 const group=readFileSync(new URL("../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs",import.meta.url),"utf8");expect(group.includes("Some((entry, progress))")).toBe(true);expect(group.includes("Some((next, slot, progress))")).toBe(true);expect(group.includes("Some((_, _, read, progress))")).toBe(true);
});
test("constructor continuations preserve original kind and receipt before semantic publication",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🏗️constructor-continuation/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["kind","originalGrant","constructorReceiptIsSemanticFence","receiptHasOriginalKind","receiptHasNoInventedOperation"],properties:{kind:{const:["Recipient","Document","Config","Draft","Interaction"]},originalGrant:{type:"array",minItems:5,maxItems:5,items:{type:"integer",minimum:1}},constructorReceiptIsSemanticFence:{const:true},receiptHasOriginalKind:{const:true},receiptHasNoInventedOperation:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_constructor(kind TEXT,items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");for(const kind of law.kind)db.query("INSERT INTO original_constructor VALUES(?,?,?,?,?,?)").run(kind,...law.originalGrant);expect(db.query("SELECT count(*) AS originals FROM original_constructor").get()).toEqual({originals:5});expect(db.query("SELECT sum(copy) AS copy,sum(capacity) AS capacity,sum(release) AS release FROM original_constructor").get()).toEqual({copy:20480,capacity:327680,release:1310720});}finally{db.close();}
 const source=pluginFamilyText(),traitStart=source.indexOf("pub trait PluginApp:"),trait=source.slice(traitStart,source.indexOf("pub trait",traitStart+20));expect(trait.includes("fn take_constructor_receipt(&mut self)")).toBe(true);
 const implementation=source.indexOf("PluginApp for VcsArtifactApp<A, M>"),start=source.indexOf("async fn advance_typed_operation_publication(",implementation),body=source.slice(start,source.indexOf("fn take_typed_operation_preparation_receipt",start));expect(implementation>=0&&start>implementation).toBe(true);expect(body.includes("semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation,retained)")).toBe(true);expect(body.includes("self.stores.drive_store_constructor_unit(&mut self.constructor, cx, __a0)")).toBe(true);expect(body.includes("self.constructor.has_constructor_receipt()")).toBe(true);
 expect(law.phase).toEqual(["Preparation","Close"]);expect(law.phaseUsesOriginalPolicy).toBe(true);
 const collectorStart=source.indexOf("pub fn retain_original_ownership_receipts"),collector=source.slice(collectorStart,source.indexOf("///",collectorStart));expect(collector.includes("app.take_constructor_receipt()")).toBe(true);expect(collector.includes("TypedOperationFixtureOwnershipReceipt::Constructor { kind, phase, grant, progress }")).toBe(true);expect(collector.includes("ArtifactStoreConstructorPhase::Preparation => policy.preparation")).toBe(true);expect(collector.includes("ArtifactStoreConstructorPhase::Close => fixture_close_grant(policy)")).toBe(true);const closeGrantStart=source.indexOf("pub fn fixture_close_grant("),closeGrant=source.slice(closeGrantStart,source.indexOf("}",closeGrantStart));expect(closeGrantStart>=0).toBe(true);expect(closeGrant.includes("mounted_policy.close")).toBe(true);
 const outputStart=source.indexOf("pub struct PluginExchangeOutput"),output=source.slice(outputStart,source.indexOf("//#region",outputStart));expect(output.includes("pub constructor_receipt: Option<(semio_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind, semio_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>")).toBe(true);
 const closeStart=source.indexOf("fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)",implementation),close=source.slice(closeStart,source.indexOf("fn close_terminal_is_empty",closeStart));expect(close.includes("self.stores.close_store_constructor_unit(&mut self.constructor, effect)")).toBe(true);expect(close.includes("self.constructor.has_constructor_receipt()")).toBe(true);
 const continuationStart=source.indexOf("fn advance_typed_operation_output_with_leftover<"),continuation=source.slice(continuationStart,source.indexOf("match app.take_reserved_commit_outcome()",continuationStart));expect(continuation.includes("output.collect_original_receipts(app)")).toBe(true);const collectStart=source.indexOf("fn collect_original_receipts<PA:PluginApp>"),collect=source.slice(collectStart,source.indexOf("}",collectStart));expect(collectStart>=0).toBe(true);expect(collect.includes("self.constructor_receipt=app.take_constructor_receipt()")).toBe(true);const postedReceipt = continuation.slice(continuation.lastIndexOf("output.collect_original_receipts(app)"));
 expect(postedReceipt).toMatch(/if output\.ownership_receipt_pending\(\)\s*\{[\s\S]*output\.bind_window_posting\(before,cx\.retained_progress\(\),reserved,original,recipient\)/);
 const postingStart = source.indexOf("fn bind_window_posting(");
 const posting = source.slice(postingStart, source.indexOf("fn original_receipt_family", postingStart));
 expect(posting).toContain("if !self.ownership_receipt_pending(){return Ok(())}");
 expect(posting).toContain("let before = before.checked_add(reserved)?");
 expect(posting).toMatch(/if posted!=RetainedCloneProgress::default\(\)\{self\.original_receiving=Some/);
 expect(postedReceipt.trimEnd().endsWith("return Ok((output, None));\n    }")).toBe(true);
});
test("original replacement retirement preserves full grants and physical pin backing",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔄️replacement-retirement/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["currencies","originalGrant","pins","replacement"],properties:{currencies:{const:["items","copy","capacity","release","depth"]},originalGrant:{type:"array",minItems:5,maxItems:5,items:{type:"integer",minimum:1}},pins:{type:"object",required:["fields","vectorBackingSeparate"],properties:{fields:{const:5},vectorBackingSeparate:{const:true}}},replacement:{type:"object",required:["childCompleteIsParentProgress","borrowedRetainers","cancellationTerminalRequired"]}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE physical_pins(n INTEGER,capacity INTEGER,fields INTEGER)");for(const n of law.pins.counts)for(const capacity of law.pins.capacities)db.query("INSERT INTO physical_pins VALUES(?,?,?)").run(n,capacity,law.pins.fields);expect(db.query("SELECT sum(n*capacity*fields) AS strings FROM physical_pins").get()).toEqual({strings:87380});}finally{db.close();}
 const source=pluginFamilyText(),pinsStart=source.indexOf("impl CompositionPinsRetirement"),pins=source.slice(pinsStart,source.indexOf("impl Drop for CompositionPinsRetirement",pinsStart));
 expect(pins.includes("fn retirement_demands(&self)")).toBe(true);expect(pins.includes("grant: RetainedCloneGrant")).toBe(true);expect(pins.includes("pins.capacity().checked_mul")).toBe(true);expect(pins.includes("drop(std::mem::take(value))")).toBe(true);expect(pins.includes("value.pop()")).toBe(false);
 const start=source.indexOf("fn drive_rejected_members("),body=source.slice(start,source.indexOf("fn drive_initializer(",start));expect(body.includes("grant: RetainedCloneGrant")).toBe(true);expect(body.includes("PluginLifecycleStep")).toBe(true);expect(body.includes("sibling_content_owners")).toBe(false);expect(body.includes("advance_original_child_root_retirement")).toBe(true);expect(body.includes("candidate_content.take()")).toBe(true);expect(body.includes("replacement_member_retirement_demands")).toBe(true);
});
test("publication continuations borrow the original driver identity authority",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🧾️fixture-caller/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["native"],properties:{native:{type:"object",required:["grant","maximumIdentityBytes"],properties:{grant:{type:"array",minItems:5,maxItems:5,items:{type:"integer",minimum:1}},maximumIdentityBytes:{type:"integer",minimum:1}}}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE driver(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");db.query("INSERT INTO driver VALUES(?,?,?,?,?)").run(...law.native.grant);expect(Object.values(db.query("SELECT items,copy,capacity,release,depth FROM driver").get()as object)).toEqual(law.native.grant);}finally{db.close();}
 const source=pluginFamilyText(),identity="identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>";
 for(const marker of["async fn advance_typed_operation_publication(","fn advance_typed_operation_output_with_leftover<","pub async fn plugin_continue_typed_operations<","pub async fn plugin_complete_reserved_spawned_job<","pub async fn settle_registered_typed_operation<","async fn drive_host_typed_continuations<","async fn drive_host_call_with_deferred_acks<"]){const start=source.indexOf(marker),signature=source.slice(start,source.indexOf(" -> ",start));expect(start>=0).toBe(true);expect(signature.includes(identity)).toBe(true);}
 for(const marker of["async fn drive_host_typed_continuations<","async fn drive_host_call_with_deferred_acks<"]){const start=source.indexOf(marker),body=source.slice(start,source.indexOf("    ///",start));expect(body.includes("retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts)")).toBe(true);expect(body.includes("cx.with_retained_loan(mounted_policy.preparation, |same| ::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity, same))")).toBe(true);expect(body.includes("ownership_receipts")).toBe(true);}
 expect(source.includes("pub(crate) async fn drive_tool_run_turn(&mut self,cx:&mut semio_framework_job::StepContext<'_>,identity:&mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)")).toBe(true);expect(source.includes("ctx.drive_tool_run_turn(cx, identity).await?")).toBe(true);expect(source.includes("app.advance_typed_operation_publication(identity,cx)")).toBe(true);expect(source.includes("advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx)")).toBe(true);
 const reactor=readModuleTreeSource(new URL("../../⚛️reactor/🔄️turn/🦀️.rs",import.meta.url));expect(/TypedOperationGrant::turn\(budget,\s*cx\),\s*identity,\s*cx\)/u.test(reactor)).toBe(true);expect(/job,\s*reserved_output,\s*identity,\s*cx\)/u.test(reactor)).toBe(true);
});
test("window publication loans originals and returns before a second paid constructor",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🪟️window-publication/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["grantCurrencies","receiptCurrencies","phases","factorySource","denialRetains","newReceiptStopsCallback"],properties:{grantCurrencies:{const:["items","copy","capacity","release","depth"]},receiptCurrencies:{const:["items","copy","capacity","release"]},factorySource:{const:"installed-original-store-preparation-factory"},denialRetains:{const:["actor","mutation-carrier","mutation-vector","authority"]},newReceiptStopsCallback:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_window_turn(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER)");for(const row of law.receiptExampleTurns)db.query("INSERT INTO original_window_turn VALUES(?,?,?,?)").run(...row);expect(Object.values(db.query("SELECT sum(items) AS items,sum(copy) AS copy,sum(capacity) AS capacity,sum(release) AS release FROM original_window_turn").get()as object)).toEqual(law.exampleTotals);}finally{db.close();}
 const owner=readModuleTreeSource(new URL("../../🪟️window/🎚️config/🦀️.rs",import.meta.url));expect(owner.includes("fn admit_begin(")).toBe(true);expect(owner.includes("actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>")).toBe(true);expect(owner.includes("mutations: &mut Vec<WindowConfigMutation>")).toBe(true);expect(owner.includes("partition.store.admit_member_apply_batch(")).toBe(true);
 const beginStart=owner.indexOf("fn admit_begin(",owner.indexOf("impl<O: WindowConfigOwner> ErasedWindowConfigStoreOwner")),begin=owner.slice(beginStart,owner.indexOf("fn advance(",beginStart));expect(beginStart>=0).toBe(true);expect(begin.includes("O::build_one_item_preparation_factory()")).toBe(false);expect(begin.includes("grant: RetainedCloneGrant")).toBe(true);
 const source=pluginFamilyText(),start=source.indexOf("fn publish_mounted_typed_operation_unit("),body=source.slice(start,source.indexOf("fn ack_mounted",start));expect(body.includes("self.stores.window_config_store.admit_begin(")).toBe(true);expect(body.includes("&mut mounted.actor_capture,&mut emit.window_config_mutations")).toBe(true);
});
test.skipIf(!process.env.SEMIO_CHILD_ROOT_RESULTS)("actual native child root receipts preserve physical custody through separate parent removal",async()=>{
 const rows=await Bun.file(process.env.SEMIO_CHILD_ROOT_RESULTS!).json();const schema={type:"array",minItems:3,maxItems:3,items:{type:"object",additionalProperties:false,required:["retained","birth","release","rootBytes","registryBytes","firstRelease","backingRelease","borrowedCopy","rootHeaderCopy","firstCopy","removalCopy","originalPointer","deniedBirth","deniedRelease","terminalDropBirth","terminalDropRelease"],properties:Object.fromEntries(["retained","birth","release","rootBytes","registryBytes","firstRelease","backingRelease","borrowedCopy","rootHeaderCopy","firstCopy","removalCopy","originalPointer","deniedBirth","deniedRelease","terminalDropBirth","terminalDropRelease"].map(name=>[name,{type:"integer",minimum:0}]))}};expect(new Ajv().compile(schema)(rows)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_child(retained INTEGER,birth INTEGER,release INTEGER,root INTEGER,registry INTEGER,first_release INTEGER,backing_release INTEGER,table_copy INTEGER,header_copy INTEGER,first_copy INTEGER,removal_copy INTEGER,pointer INTEGER,denied_birth INTEGER,denied_release INTEGER,drop_birth INTEGER,drop_release INTEGER)");for(const r of rows)db.query("INSERT INTO original_child VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)").run(r.retained,r.birth,r.release,r.rootBytes,r.registryBytes,r.firstRelease,r.backingRelease,r.borrowedCopy,r.rootHeaderCopy,r.firstCopy,r.removalCopy,r.originalPointer,r.deniedBirth,r.deniedRelease,r.terminalDropBirth,r.terminalDropRelease);expect(db.query("SELECT count(*) AS n FROM original_child WHERE birth=release AND birth=root+registry AND backing_release=registry AND first_copy=table_copy+header_copy AND header_copy>0 AND removal_copy>0 AND pointer>0 AND denied_birth=0 AND denied_release=0 AND drop_birth=0 AND drop_release=0 AND ((retained=0 AND first_release=root) OR (retained IN (1,2) AND first_release=0))").get()).toEqual({n:3});expect(db.query("SELECT count(distinct retained) AS n FROM original_child").get()).toEqual({n:3});}finally{db.close();}
});
test("tool run receipt preserves the original grant through every runtime handoff",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🧾️tool-run/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["owners","receiptCurrencies","grantCurrencies","toolRunOperationIdentity","toolRunGrantSource","collectBeforeNextSemanticUnit"],properties:{owners:{const:["preparation","history","tool-run"]},receiptCurrencies:{const:["items","copy","capacity","release"]},grantCurrencies:{const:["items","copy","capacity","release","depth"]},toolRunOperationIdentity:{type:"null"},toolRunGrantSource:{const:"original-mounted-maintenance-turn"},collectBeforeNextSemanticUnit:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_receipts(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER)");for(const row of law.rows)db.query("INSERT INTO original_receipts VALUES(?,?,?,?)").run(...row);expect(Object.values(db.query("SELECT sum(items) AS items,sum(copy) AS copy,sum(capacity) AS capacity,sum(release) AS release FROM original_receipts").get()as object)).toEqual(law.totals);}finally{db.close();}
 const source=pluginFamilyText();expect(source.includes("fn take_tool_run_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)>")).toBe(true);expect(source.includes("self.tool_runs.take_original_receipt()")).toBe(true);expect(source.includes("self.tool_runs.has_original_receipt()")).toBe(true);expect(source.includes("self.tool_run_receipt=app.take_tool_run_receipt()")).toBe(true);
 const stopStart=source.indexOf("fn spent(&self, output: &PluginExchangeOutput)"),stop=source.slice(stopStart,source.indexOf("fn expired",stopStart));expect(stop.includes("output.tool_run_receipt.is_some()")).toBe(true);
 const continuationStart=source.indexOf("pub async fn plugin_continue_typed_operations"),continuation=source.slice(continuationStart,source.indexOf("//#endregion",continuationStart));expect(continuation.includes("collected.1.tool_run_receipt=output.tool_run_receipt")).toBe(true);
 const inlineStart=source.indexOf("if let Some(operation) = flags.started_typed_operation"),inline=source.slice(inlineStart,source.indexOf("state.effects.append",inlineStart));expect(inline.includes('!state.typed_output.as_mut().expect("retained actual typed output").ownership_receipt_pending()')).toBe(true);expect(source.includes("self.tool_run_receipt.is_some()")).toBe(true);expect(inline.includes("typed_output.tool_run_receipt=more.tool_run_receipt")).toBe(true);expect(source.includes("tool_run_receipt: typed_output.tool_run_receipt")).toBe(true);
 expect(source.includes("TypedOperationFixtureOwnershipReceipt::ToolRun { grant, progress }")).toBe(true);
});
test("child root maintenance borrows original retainers and separates terminal registry removal",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🌳️child-root/🔣️.json",import.meta.url)).json();
 expect(new Ajv().compile({type:"object",required:["currencies","phases","retainers","queryHasEffects","childCompleteIsParentComplete"],properties:{currencies:{const:["items","copy","capacity","release","depth"]},retainers:{const:["current-root","sibling-retirements","member-preview","saved-reviewed-preview"]},queryHasEffects:{const:false},childCompleteIsParentComplete:{const:false}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE owner(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");db.query("INSERT INTO owner VALUES(?,?,?,?,?)").run(...law.originalGrant);const r=law.composedReceipt;expect(r).toEqual(law.childReceipt.map((value:number,index:number)=>value+law.borrowedTableReceipt[index]));expect(law.composedDepth).toBe(law.borrowedTableDepth+law.childDepth);expect(Object.values(db.query("SELECT items-? AS items,copy-? AS copy,capacity-? AS capacity,release-? AS release,depth FROM owner").get(...r)as object)).toEqual(law.remaining);}finally{db.close();}
 const source=pluginFamilyText(),producer=readFileSync(new URL("../../🧩️composition/♻️root/🦀️.rs",import.meta.url),"utf8"),start=producer.indexOf("fn advance_original_child_root_retirement"),step=producer.slice(start);
 expect(step.includes("let grant = effect.grant()")).toBe(true);expect(step.includes("&mut effect.nested(child_grant)")).toBe(true);expect(step.includes("original_content_step_owners")).toBe(true);expect(step.includes("RetainedCloneStep::Complete(progress)=>RetainedCloneStep::Progress")).toBe(true);expect(step.includes("sibling_content_owners")).toBe(false);expect(step.includes("maximum_copy_bytes:grant.maximum_copy_bytes-reserved")).toBe(true);
 const queryStart=producer.indexOf("fn original_child_root_retirement_demands"),query=producer.slice(queryStart,producer.indexOf("fn advance_original_child_root_retirement",queryStart));expect(queryStart>=0).toBe(true);for(const verb of[".clone()",".take()",".remove(",".close_step("])expect(query.includes(verb)).toBe(false);
 const reclaimStart=source.indexOf("fn reclaim_child_retirements("),reclaim=source.slice(reclaimStart,source.indexOf("\n        }",reclaimStart));expect(reclaim.includes("child_root_retirement_step")).toBe(false);expect(reclaim.includes("child_member_retirement_step")).toBe(false);
 expect(source.includes("pub trait ChildContentRetainers")).toBe(true);expect(source.includes("member.review_children.as_ref().map(|review|&review.view)")).toBe(true);expect(source.includes("member.children.as_ref()")).toBe(true);
 const mountedStart=source.indexOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp"),mounted=source.slice(mountedStart);expect(source.includes("if stage==MAINTENANCE_CHILD_ROOT_STAGE{return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.child_root_retirement_step(effect)")).toBe(true);expect(source.includes("CloseRung::ChildRootRetirements=>Ok(RetainedCloneStep::Progress(semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.ladder_ctx().child_root_retirement_step(effect)")).toBe(true);expect(source.includes("self.child_root_retirement_unit(effect, self.close.close_child_root_detached)")).toBe(true);expect(source.includes("advance_original_child_root_retirement(&mut self.child_composition.child_content_retirements, &mut self.child_composition.children, Some(&mut self.child_composition.child_member_retirements), &self.child_composition.child_content_root, member, review, &mut self.maintenance.maintenance_child_root_cursor, effect, closing)")).toBe(true);
});
test("every shared artifact fixture propagates the original caller policy and identity",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🧾️fixture-caller/🔣️.json",import.meta.url)).json();
 expect(new Ajv().compile({type:"object",required:["constructors","drivers","currencies","callerIdentity","grantSource","receiptSource"],properties:{currencies:{const:["items","copy","capacity","release","depth"]},callerIdentity:{const:"original-entity-identity-authority"},grantSource:{const:"explicit-embedding-policy"},receiptSource:{const:"actual-publication-and-maintenance"}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE turns(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");db.query("INSERT INTO turns VALUES(?,?,?,?,?)").run(...law.grant);expect(Object.values(db.query("SELECT items-? AS items,copy-? AS copy,capacity-? AS capacity,release-? AS release,depth FROM turns").get(...law.first)as object)).toEqual(law.remaining);}finally{db.close();}
 const source=pluginFamilyText();
 for(const name of [...law.constructors,...law.drivers]){const marker=`async fn ${name}<`,start=source.indexOf(marker);expect(start, name).toBeGreaterThanOrEqual(0);const signature=source.slice(start,source.indexOf("{",start));expect(signature.includes("mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1"),name).toBe(true);if(law.constructors.includes(name))expect(signature.includes("EntityIdentityAuthority<'_>"),name).toBe(true);}
 const start=source.indexOf("pub async fn settle_extension_invocations"),body=source.slice(start,source.indexOf("pub fn close_registered_fixture_app",start));expect(body.includes("settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx)")).toBe(true);expect(body.includes("settled.ownership_receipts.extend(receipt.ownership_receipts)")).toBe(true);
});
test("runtime forwards one original grant and validates every actual lifecycle currency",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🎟️lifecycle-grant/🔣️.json",import.meta.url)).json();
 const db=new Database(":memory:");try{db.run("CREATE TABLE turns(items INTEGER, copy INTEGER, capacity INTEGER, release INTEGER, depth INTEGER, called INTEGER)");const names={maximumItems:1,maximumCopyBytes:law.demand.copyBytes,maximumCapacityBytes:law.demand.capacityBytes,maximumReleaseBytes:law.demand.releaseBytes,maximumDepth:law.demand.depth};for(const axis of law.refusalAxes){const grant={...law.grant,[axis]:names[axis as keyof typeof names]-1};db.query("INSERT INTO turns VALUES(?,?,?,?,?,?)").run(grant.maximumItems,grant.maximumCopyBytes,grant.maximumCapacityBytes,grant.maximumReleaseBytes,grant.maximumDepth,0);}db.query("INSERT INTO turns VALUES(?,?,?,?,?,?)").run(law.grant.maximumItems,law.grant.maximumCopyBytes,law.grant.maximumCapacityBytes,law.grant.maximumReleaseBytes,law.grant.maximumDepth,1);expect(db.query("SELECT count(*) AS n FROM turns WHERE items>=1 AND copy>=? AND capacity>=? AND release>=? AND depth>=?").get(law.demand.copyBytes,law.demand.capacityBytes,law.demand.releaseBytes,law.demand.depth)).toEqual({n:1});expect(db.query("SELECT sum(called) AS n FROM turns").get()).toEqual({n:1});}finally{db.close();}
 const source=pluginFamilyText(),trait=source.slice(source.indexOf("pub trait PluginApp: Send"),source.indexOf("async fn app_id",source.indexOf("pub trait PluginApp: Send")));expect(trait.includes("fn close_retirement_demands(&self, body: usize)")).toBe(true);expect(trait.includes("fn maintenance_retirement_demands(&self, body: usize,maximum_work_units:u64)")).toBe(true);expect(trait.includes("fn close_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant)")).toBe(true);expect(trait.includes("fn maintenance_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant)")).toBe(true);expect(trait.includes("maximum_bytes")).toBe(false);
 const transport=source.slice(source.indexOf("fn runtime_lifecycle_step("),source.indexOf("struct RuntimeLiveCleanupJob",source.indexOf("fn runtime_lifecycle_step(")));expect(transport).toContain("semio_framework_value::advance_retirement_turn(demand,grant,|grant|");expect(transport).toContain("result:owner(grant)");const native=readFileSync(new URL("../../../../../../🔨️modules/🌱️value/♻️retirement/🎟️turn/🦀️.rs",import.meta.url),"utf8");for(const field of ["maximum_items","maximum_copy_bytes","maximum_capacity_bytes","maximum_release_bytes","maximum_depth","receipt.fits(grant)","owner(grant)","terminal_is_empty"])expect(native.includes(field)).toBe(true);expect(source.includes("impl semio_framework_value::RetirementReceipt for PluginLifecycleStep")).toBe(true);expect(transport.includes("artifact_close_release_grant")).toBe(false);expect(source.includes("instance.app.mounted_owner_maintenance_step(&mut turn)")).toBe(true);expect(source.includes("instance.app.mounted_owner_close_step(&mut turn)")).toBe(true);
});

test("runtime maintenance closes against the caller grant", async () => {
  const law = await Bun.file(new URL("🧫️fixtures/🎟️maintenance-close/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const admits = db.query(`SELECT
      ? <= ? AND ? <= ? AND ? <= ? AND ? <= ? AND (NOT ? OR ?) AS accepted`);
    for (const row of law.cases) {
      for (const [grant, accepted] of [[law.callerGrant, row.callerAccepted], [law.mountedClosePolicy, row.mountedAccepted]]) {
        const axes = row.receipt.flatMap((used: number, index: number) => [used, grant[index]]);
        expect(admits.get(...axes, Number(row.complete), Number(row.empty))).toEqual({ accepted });
      }
    }
  } finally {
    db.close();
  }
  const source = pluginFamilyText();
  const start = source.indexOf("let mut maintenance = match cell.maintenance_pump.try_lock()");
  expect(start).toBeGreaterThanOrEqual(0);
  const close = source.slice(start, source.indexOf("match progress", start));
  expect(close).toMatch(/let grant\s*=\s*cx\.retained_grant\(\);/);
  expect(close).toContain("maintenance.close_step(cx)");
  expect(close).toContain("step.admit(grant, maintenance.terminal_is_empty())");
});

test("runtime cleanup keeps the parent pending until its weak owner returns", async () => {
  const cases = await Bun.file(new URL("🧫️fixtures/🪶️cleanup-parent/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const transition = db.query(`SELECT
      CASE WHEN :child = "complete" AND :weakOwned THEN "pending" ELSE :child END AS parent,
      :items AS items, :copy AS copy, :capacity AS capacity, :release AS release`);
    for (const { child, weakOwned, receipt: [items, copy, capacity, release], parent } of cases) {
      expect(transition.get({ ":child": child, ":weakOwned": Number(weakOwned), ":items": items,
        ":copy": copy, ":capacity": capacity, ":release": release })).toEqual({ parent, items, copy, capacity, release });
    }
  } finally {
    db.close();
  }
  const source = pluginFamilyText();
  for (const job of ["RuntimeLiveCleanupJob", "RuntimeCloseCleanupJob"]) {
    const implementation = source.indexOf(`impl<PA: PluginApp> semio_framework_job::InteractiveJob for ${job}<PA>`);
    expect(implementation).toBeGreaterThanOrEqual(0);
    const start = source.indexOf("fn close_step", implementation);
    const close = source.slice(start, source.indexOf("fn next_close_copy_byte_demand", start));
    expect(close).toMatch(/self\.publication\.close_step\(cx\)\?[\s\S]*Complete \{ progress \} if !self\.terminal_is_empty\(\) => Step::Pending \{ progress \}/);
  }
});

test("runtime cleanup retains original payload headers through the shared slot producer before the separate resume turn",async()=>{
 const law=await Bun.file(jobRetirementPayloadFixtureUrl).json();expect(law.slotStages).toEqual(["payload","metadata","parent-complete"]);
 const db=new Database(":memory:");try{db.run("CREATE TABLE handbacks(variant TEXT,phase INTEGER,header INTEGER,resume INTEGER)");for(const variant of law.variants){for(let phase=0;phase<4;phase++)db.query("INSERT INTO handbacks VALUES(?,?,?,?)").run(variant,phase,Number(phase<2),Number(phase===3));}expect(db.query("SELECT count(*) AS n FROM handbacks WHERE resume=1 AND header=0 AND phase=3").get()).toEqual({n:6});expect(db.query("SELECT count(*) AS n FROM handbacks WHERE resume=1 AND header=1").get()).toEqual({n:0});}finally{db.close();}
 const source = pluginFamilyText();
 for (const name of ["runtime_live_cleanup_pump_one", "runtime_close_cleanup_pump_one"]) {
  const start = source.indexOf(`fn ${name}<`);
  const body = source.slice(start, source.indexOf("if pump.session.is_none()", start));
  expect(body).toContain("if !session.acknowledge_outcome(cx)?");
  expect(body).toContain("pump.outcome_pending=false;");
  expect(body).toContain("pump.resume_pending=true;");
  expect(body.indexOf("session.acknowledge_outcome(cx)")).toBeLessThan(body.indexOf("if pump.resume_pending"));
  expect(body).toContain("session.close_step(cx)?");
  expect(body).not.toContain("outcome.close_step(grant)");
  const outcome = source.slice(start, source.indexOf("pump.outcome_pending = true", start));
  expect(outcome).toMatch(/take_checked_out_retained_step_receipt\(\)[\s\S]*checked_out_continuation\(\)[\s\S]*pump\.faulted = true;[\s\S]*checked_out_outcome\(\)/);
 }
 const start=source.indexOf("impl<PA: PluginApp> RuntimeLiveCleanupPump"),body=source.slice(start,source.indexOf("struct RuntimeCloseCleanupPump",start));expect(body.includes("session.retirement_demands(body)")).toBe(true);expect(body.includes("if session.terminal_is_empty(){cx.consume_retained(metadata)?;drop(self.session.take());self.outcome_pending=false;self.resume_pending=false;")).toBe(true);expect(body.includes("self.closing && self.session.is_none() && self.rejected.is_none() && !self.outcome_pending")).toBe(true);expect(body.includes("close_step_outcome_slot")).toBe(false);
});

test("media and typed worker handbacks retain original outcomes and use all admission currencies",async()=>{
 const law=await Bun.file(jobRetirementPayloadFixtureUrl).json();
 const db=new Database(":memory:");try{db.run("CREATE TABLE private_handbacks(owner TEXT,variant TEXT,phase TEXT,resume INTEGER)");for(const owner of ["media","typed"]){for(const variant of law.variants){for(const phase of [...law.slotStages,"resume"])db.query("INSERT INTO private_handbacks VALUES(?,?,?,?)").run(owner,variant,phase,Number(phase==="resume"));}}expect(db.query("SELECT count(*) AS n FROM private_handbacks WHERE resume=1").get()).toEqual({n:12});expect(db.query("SELECT count(*) AS n FROM private_handbacks WHERE resume=1 AND phase!='resume'").get()).toEqual({n:0});}finally{db.close();}
 const source=pluginFamilyText(),mediaStart=source.indexOf("fn retire_nonterminal_outcome("),media=source.slice(mediaStart,source.indexOf("fn terminal_is_empty",mediaStart)),typedStart=source.indexOf("fn drive_worker_step("),typed=source.slice(typedStart,source.indexOf("fn next_token",typedStart));
 expect(media.includes("grant: RetainedCloneGrant")).toBe(true);expect(media.includes("Result<PluginLifecycleStep, Fault>")).toBe(true);expect(media.includes("session.acknowledge_checked_out_outcome(cx)")).toBe(true);expect(media.includes("self.resume_pending = true")).toBe(true);
 expect(typed.includes("let grant=cx.retained_grant();")).toBe(true);expect(typed.includes("Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure>")).toBe(true);expect(typed.includes("acknowledge_checked_out_outcome(cx)")).toBe(true);expect(typed.includes("take_checked_out_outcome()")).toBe(false);expect(typed.includes("self.worker_resume_pending=true")).toBe(true);for(const body of [media,typed])expect(body.includes("JobPayloadCloseStep")).toBe(false);
 const pollStart=source.indexOf("pub(crate) async fn drive_owned_media_export("),poll=source.slice(pollStart,source.indexOf("if let Some(rejected) = active.session_rejected.as_mut()",pollStart));expect(pollStart>=0).toBe(true);expect(poll.includes("take_checked_out_outcome")).toBe(false);expect(poll.includes("if active.resume_pending || active.outcome_pending {")).toBe(true);expect(poll.includes("active.retire_nonterminal_outcome(cx)")).toBe(true);
});

test("envelope and initializer handbacks admit one original full turn without forced drains",async()=>{
 const law=await Bun.file(jobRetirementPayloadFixtureUrl).json();
 const db=new Database(":memory:");try{db.run("CREATE TABLE decode_turns(owner TEXT,variant TEXT,phase INTEGER,payload INTEGER,resume INTEGER)");for(const owner of ["envelope","initializer"]){for(const variant of law.variants){for(let phase=0;phase<4;phase++)db.query("INSERT INTO decode_turns VALUES(?,?,?,?,?)").run(owner,variant,phase,Number(phase<2),Number(phase===3));}}expect(db.query("SELECT count(*) AS n FROM decode_turns WHERE resume=1 AND payload=0").get()).toEqual({n:12});expect(db.query("SELECT count(*) AS n FROM decode_turns WHERE resume=1 AND payload=1").get()).toEqual({n:0});}finally{db.close();}
 const source=pluginFamilyText(),envelopeStart=source.indexOf("impl<P, Mutation> ActiveArtifactEnvelopeDecode"),envelope=source.slice(envelopeStart,source.indexOf("fn decode_worker_owners_are_terminal",envelopeStart)),initializerStart=source.indexOf("fn drive_initializer("),initializer=source.slice(initializerStart,source.indexOf("fn drive_retained_store",initializerStart));
 for(const body of [envelope,initializer]){expect(body.includes("grant: RetainedCloneGrant")).toBe(true);expect(body.includes("Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>")).toBe(true);expect(body.includes("session.acknowledge_checked_out_outcome(cx)")).toBe(true);expect(body.includes("self.resume_pending=true")).toBe(true);expect(body.includes("JobPayloadCloseStep")).toBe(false);}
 expect(source.includes("fn force_worker_session_terminal(")).toBe(false);const start=source.indexOf("fn advance_ready_envelope_decode_worker_owners("),body=source.slice(start,source.indexOf("pub fn cancel_artifact_envelope_decode",start));expect(start>=0).toBe(true);expect(body.includes("grant: RetainedCloneGrant")).toBe(true);expect(body.includes("for _")).toBe(false);expect(body.includes("4096")).toBe(false);expect(source.includes("rejected.next_close_demands(")).toBe(false);expect(source.includes("session.next_close_demands(")).toBe(false);
});

test("envelope return pumps preserve every original currency and separate owner metadata removal",()=>{
 const db=new Database(":memory:");try{db.run("CREATE TABLE returns(owner TEXT,phase INTEGER,capacity INTEGER,release INTEGER)");for(const owner of ["field","completed"]){db.query("INSERT INTO returns VALUES(?,?,?,?)").run(owner,0,64,0);db.query("INSERT INTO returns VALUES(?,?,?,?)").run(owner,1,0,128);db.query("INSERT INTO returns VALUES(?,?,?,?)").run(owner,2,0,0);}expect(db.query("SELECT sum(capacity) AS capacity,sum(release) AS release FROM returns").get()).toEqual({capacity:128,release:256});}finally{db.close();}
 const source=pluginFamilyText();for(const [name,step] of [["drive_envelope_field_decoder_returns","erased_step(cx, retirement, grant)"],["drive_envelope_completed_record_returns","completed_step(cx, retirement.as_mut(), grant)"]]){const start=source.indexOf(`fn ${name}(`),body=source.slice(start,source.indexOf("\n    }\n",start));expect(start>=0).toBe(true);expect(body.includes("grant: RetainedCloneGrant")).toBe(true);expect(body.includes("Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>")).toBe(true);expect(body.includes("SnapshotRetirementStep")).toBe(false);expect(body.includes(step)).toBe(true);expect(body.includes("plugin_turn_admitted(handoff_demand(), grant)?")).toBe(true);expect(body.includes("PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress })")).toBe(true);expect(body.includes("PluginLifecycleStep::Complete(empty)")).toBe(true);}
 const fundsStart=source.indexOf("pub fn plugin_grant_funds("),funds=source.slice(fundsStart,source.indexOf("\n}\n",fundsStart));expect(funds).toContain("plugin_grant_shortfall(demand, grant).is_none()");
 const shortfallStart=source.indexOf("pub fn plugin_grant_shortfall("),shortfall=source.slice(shortfallStart,source.indexOf("\n}\n",shortfallStart));for(const field of ["maximum_items","maximum_copy_bytes","maximum_capacity_bytes","maximum_release_bytes","maximum_depth"])expect(shortfall).toContain(field);
 for(const helper of ["fn erased_step<","fn completed_step<"]){const start=source.indexOf(helper),body=source.slice(start,source.indexOf("\n}\n",start));expect(body).toContain("original_lifecycle_effect_step(cx, grant, demand, |_, effect|");expect(body).toContain("let child = effect.grant()");expect(body).toContain("admit_retained_clone_close(child, step,");}
});

test("partial envelope returns forward already posted receipts and keep the original owner", async () => {
  const law = await Bun.file(new URL("🧫️fixtures/♻️envelope/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const receipt = db.query(`SELECT json_array(?1, ?2, ?3, ?4) AS progress,
      ?5 AS "reclaimOwner", 0 AS "parentTerminal"`);
    const row = receipt.get(...law.childProgress, Number(law.partialReturn.childTerminal)) as {
      progress: string; reclaimOwner: number; parentTerminal: number;
    };
    expect(JSON.parse(row.progress)).toEqual(law.partialReturn.parentProgress);
    expect(Boolean(row.reclaimOwner)).toBe(law.partialReturn.reclaimOwner);
    expect(Boolean(row.parentTerminal)).toBe(law.partialReturn.parentTerminal);
  } finally { db.close(); }

  const source = pluginFamilyText();
  for (const name of ["drive_envelope_field_decoder_returns", "drive_envelope_completed_record_returns"]) {
    const start = source.indexOf(`fn ${name}(`);
    const body = source.slice(start, source.indexOf("\n    }\n", start));
    const partial = body.match(/let RetainedCloneStep::Complete\(progress\) = step else \{([^}]+)\}/)?.[1];
    expect(partial).toContain("*cursor = index");
    expect(partial).toContain("PluginLifecycleStep::Progress(step.progress())");
  }
});

test("independent lifecycle release preserves original empty allocation extents and fixed refusal",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/📏️live-physical-demand/🔣️.json",import.meta.url)).json();
 const db=new Database(":memory:");try{db.run("CREATE TABLE originals(extent INTEGER,copy INTEGER,release INTEGER,called INTEGER)");for(const extent of [...law.allocationBytes,law.refusedAllocationBytes]){db.query("INSERT INTO originals VALUES(?,?,?,?)").run(extent,law.ordinaryGrantBytes,law.maximumAdmissionBytes,Number(extent<=law.maximumAdmissionBytes));}expect(db.query("SELECT count(*) AS n FROM originals WHERE called=1 AND extent>copy AND extent<=release").get()).toEqual({n:3});expect(db.query("SELECT extent,called FROM originals WHERE extent>release").get()).toEqual({extent:law.refusedAllocationBytes,called:0});}finally{db.close();}
 const source=pluginFamilyText(),policy=readFileSync(new URL("../../🏇️mounted-owner/🦀️.rs",import.meta.url),"utf8");for(const axis of ["maximum_items","maximum_copy_bytes","maximum_capacity_bytes","maximum_release_bytes","maximum_depth"])expect(policy.includes("grant."+axis)).toBe(true);expect(policy.includes("MountedOwnerPhaseV1::Maintenance => self.maintenance")).toBe(true);expect(source.includes("MountedOwnerTurnV1::admit(instance_id, identity, phase, cell.mounted_policy, cx)?")).toBe(true);expect(source.includes("MountedOwnerTurnV1::admit(state.instance_id, identity, MountedOwnerPhaseV1::Close, state.mounted_policy, cx)?")).toBe(true);expect(source.includes("step.progress().unwrap_or_default() != turn.progress()")).toBe(true);
 const native=readFileSync(new URL("🦀️.rs",import.meta.url),"utf8");expect(native.includes("runtime_live_maintenance_step")).toBe(false);expect(native.includes("PluginCloseStep")).toBe(false);expect(native.includes("assert_eq!(actual, grant)")).toBe(true);expect(native.includes("every_actual_progress_currency_resets_close_and_live_stall_credit")).toBe(true);
});

test("mounted callers retain the original inline outcome and remove only its actual paid presence",async()=>{
 const law=await Bun.file(jobRetirementPayloadFixtureUrl).json();expect(new Ajv().compile({type:"object",required:["slotHeaderRemoval","slotPresenceBytes"],properties:{slotHeaderRemoval:{const:"in-place"},slotPresenceBytes:{const:0}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE slots(owner TEXT,phase TEXT,copy INTEGER,move INTEGER)");for(const owner of ["runtime-live","runtime-close","media","typed","envelope","initializer","framework"]){for(const phase of ["payload","metadata","resume"])db.query("INSERT INTO slots VALUES(?,?,?,?)").run(owner,phase,Number(phase==="metadata"&&owner!=="typed")*law.slotPresenceBytes,0);}expect(db.query("SELECT count(*) AS n,sum(copy) AS copy,sum(move) AS move FROM slots WHERE phase='metadata'").get()).toEqual({n:7,copy:0,move:0});}finally{db.close();}
 const source=pluginFamilyText();for(const [name,flag] of [["ActiveMediaExport","outcome_pending:bool"],["MountedTypedCommandFullOperation<","worker_outcome_pending: bool"],["ActiveArtifactEnvelopeDecode<","outcome_pending: bool"],["ActiveArtifactStoreReplacement<","outcome_pending: bool"]]){const start=source.indexOf(`struct ${name}`),body=source.slice(start,source.indexOf("\n}\n",start));expect(start>=0).toBe(true);expect(body.includes(flag)).toBe(true);expect(body.includes("JobOutcomeSlot")).toBe(false);expect(body.includes("Option<semio_framework_job::StepOutcome>")).toBe(false);}
 const framework=readFileSync(new URL("../../👷️job/🏛️reserved/⚙️worker/🦀️.rs",import.meta.url),"utf8");
 expect(framework).toContain("session.take_checked_out_retained_step_receipt()");
 expect(framework).toContain("session.checked_out_outcome()");
 expect(framework).toContain("self.terminal=match &view");
 expect(framework).not.toContain("JobOutcomeSlot");
 expect(framework).toContain("cx.with_retained_loan(grant,|same|session.acknowledge_checked_out_outcome(same))");
 expect(framework).toContain("session.checked_out_outcome_descriptor().is_none()");
});

test("media close forwards one original full grant and preserves rich unsupported owners",()=>{
 const source=pluginFamilyText(),start=source.indexOf("impl ActiveMediaExport {"),body=source.slice(start,source.indexOf("\n}\n",start));expect(body.includes("pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant,canonical:&semio_plugin_operation::ToolCancellationHandle) -> Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>")).toBe(true);expect(body.includes("JobPayloadCloseStep")).toBe(false);expect(body.includes("original_worker_lifecycle")).toBe(true);expect(body.includes("original_rejection_lifecycle")).toBe(true);expect(body.includes("self.session.is_none()&&self.session_rejected.is_none()&&!self.outcome_pending&&!self.resume_pending&&self.parts.is_none()&&self.parts_retirement.is_none()")).toBe(true);expect(body.includes("semio_framework_value::retirement::controlled::ControlledRetirement::new(value)")).toBe(true);
});

test("replacement close preserves the original full child receipt before separate disposer and Store removal",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/📦️retained-store/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["phases","currencies","originalPointerOnDenial","childCompleteIsParentProgress","separateDisposerAndStoreRemoval","parentCompleteRequiresAllOwners"],properties:{phases:{const:["child","disposer","store","parent-frontier"]},currencies:{const:["items","copy","capacity","release","depth"]},originalPointerOnDenial:{const:true},childCompleteIsParentProgress:{const:true},separateDisposerAndStoreRemoval:{const:true},parentCompleteRequiresAllOwners:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE replacement(phase TEXT,original INTEGER,capacity INTEGER,release INTEGER,complete INTEGER)");for(const [phase,original,capacity,release,complete]of [[law.phases[0],1,64,0,0],[law.phases[1],1,0,128,0],[law.phases[2],0,0,256,0],[law.phases[3],0,0,0,0]])db.query("INSERT INTO replacement VALUES(?,?,?,?,?)").run(phase,original,capacity,release,complete);expect(db.query("SELECT sum(capacity) AS capacity,sum(release) AS release,sum(complete) AS complete FROM replacement").get()).toEqual({capacity:64,release:384,complete:0});}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("fn drive_retained_store("),body=source.slice(start,source.indexOf("fn terminal_is_empty",start));expect(body.includes("let grant = effect.grant()")).toBe(true);expect(body.includes("Result<PluginLifecycleStep, Fault>")).toBe(true);expect(body.includes("disposer.retirement_demands(store,grant.maximum_copy_bytes)")).toBe(true);expect(body.includes("disposer.close_step(store,effect)")).toBe(true);expect(body.includes("runtime_lifecycle_step(demand,grant")).toBe(true);expect(body.includes("PluginLifecycleStep::Progress(progress)")).toBe(true);expect(body.includes("maximum_release_bytes")).toBe(true);expect(body.includes("maximum_copy_bytes")).toBe(true);expect(body.includes("PluginCloseStep")).toBe(false);
});

test("mounted envelope returns quote original owners and preserve receipts through the installed maintenance entrance",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/♻️envelope/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["owners","phases","currencies","pureQuote","sameGrant","childCompleteIsParentProgress","pollDoesNotConsumeMaintenance"],properties:{owners:{const:["field","completed"]},phases:{const:["original-child","terminal-metadata","parent-frontier"]},currencies:{const:["items","copy","capacity","release","depth"]},pureQuote:{const:true},sameGrant:{const:true},childCompleteIsParentProgress:{const:true},pollDoesNotConsumeMaintenance:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE receipts(owner TEXT,phase TEXT,items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,parent_complete INTEGER)");for(const owner of law.owners){db.query("INSERT INTO receipts VALUES(?,?,?,?,?,?,?)").run(owner,law.phases[0],...law.childProgress,0);db.query("INSERT INTO receipts VALUES(?,?,?,?,?,?,?)").run(owner,law.phases[1],1,24,0,256,0);}expect(db.query("SELECT sum(items) AS items,sum(copy) AS copy,sum(capacity) AS capacity,sum(release) AS release,sum(parent_complete) AS complete FROM receipts").get()).toEqual({items:4,copy:110,capacity:128,release:768,complete:0});}finally{db.close();}
 const source=pluginFamilyText();for(const [name,helper] of [["envelope_field_decoder_returns_demands","erased_demand("],["envelope_completed_record_returns_demands","completed_demand("]]){const quoteStart=source.indexOf(`fn ${name}(`),quote=source.slice(quoteStart,source.indexOf("\n    }\n",quoteStart));expect(quoteStart>=0).toBe(true);expect(quote.includes("handoff_demand()")).toBe(true);expect(quote.includes(helper)).toBe(true);expect(quote.includes(".take()")).toBe(false);expect(quote.includes(".remove(")).toBe(false);expect(quote.includes(".close_step(")).toBe(false);}
 const demandStart=source.indexOf("pub(super) fn erased_demand<"),demandBody=source.slice(demandStart,source.indexOf("\n}\n",demandStart));for(const part of ["owner.next_demand(body)?","demand.depth=demand.depth.max(1)"])expect(demandBody.includes(part)).toBe(true);for(const field of ["copy_bytes","capacity_bytes","release_bytes"])expect(demandBody.includes(field)).toBe(false);
 const entranceStart=source.indexOf("pub fn envelope_decode_close_step("),entrance=source.slice(entranceStart,source.indexOf("\n    }\n",entranceStart));expect(entranceStart>=0).toBe(true);expect(entrance.includes("drive_envelope_completed_record_returns(cx, grant, true)")).toBe(true);expect(entrance.includes("drive_envelope_field_decoder_returns(cx, grant, true)")).toBe(true);expect(entrance.includes("drive_envelope_decode_jobs(cx,grant, true)")).toBe(true);expect(entrance.includes("PluginLifecycleStep::Progress(progress) if progress != empty => return Ok(step)")).toBe(true);expect(entrance.indexOf("drive_envelope_completed_record_returns")<entrance.indexOf("drive_envelope_field_decoder_returns")).toBe(true);const jobsStart=source.indexOf("fn drive_envelope_decode_jobs("),jobs=source.slice(jobsStart,source.indexOf("pub fn envelope_decode_close_step(",jobsStart));expect(jobsStart>=0).toBe(true);expect(jobs.includes("PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => progress")).toBe(true);expect(jobs.includes("copied_items: progress.copied_items.max(1)")).toBe(true);
 const stageStart=source.indexOf("fn maintenance_stage_step(&mut self,stage:u8"),stageBody=source.slice(stageStart,source.indexOf("if stage==24{",stageStart));expect(stageStart>=0).toBe(true);expect(stageBody.includes("if stage==13{return self.drive_envelope_completed_record_returns(cx,grant,false)}")).toBe(true);expect(stageBody.includes("if stage==12{return self.drive_envelope_field_decoder_returns(cx,grant,false)}")).toBe(true);const pollStart=source.indexOf("async fn advance_typed_operation_publication(&mut self,",source.indexOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp")),poll=source.slice(pollStart,source.indexOf("fn tool_run_presence",pollStart));expect(poll.includes("drive_artifact_envelope_decode_worker")).toBe(false);
});

test("typed publication retains actual preparation currencies without reconstructing them from checkpoints",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🎟️preparation/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["currencies","grantSource","receiptSource","checkpointIsNotReceipt","oneOutstandingReceipt"],properties:{currencies:{const:["items","copy","capacity","release"]},grantSource:{const:"original-phase-policy-and-context-remaining"},receiptSource:{const:"original-store-preparation"},checkpointIsNotReceipt:{const:true},oneOutstandingReceipt:{const:true}}})(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE preparation(cursor INTEGER,items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER)");for(const row of law.rows)db.query("INSERT INTO preparation VALUES(?,?,?,?,?)").run(...row);expect(db.query("SELECT count(distinct cursor) AS checkpoints,sum(items) AS items,sum(copy) AS copy,sum(capacity) AS capacity,sum(release) AS release FROM preparation").get()).toEqual({checkpoints:1,items:2,copy:46,capacity:16384,release:8192});}finally{db.close();}
 const source=pluginFamilyText(),branchStart=source.indexOf("store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)"),branch=source.slice(branchStart,source.indexOf("store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationRefused",branchStart));expect(branchStart>=0).toBe(true);expect(branch.includes("ownership.fits(grant.retained_grant())")).toBe(true);expect(branch.includes("mounted.note_publication_checkpoint(checkpoint)")).toBe(true);expect(branch.includes("mounted.publication_ownership_progress")).toBe(false);
 const settlement=source.lastIndexOf("cx.consume_receiving_retained(funded, ownership)",branchStart);expect(settlement).toBeGreaterThan(0);const receiving=source.lastIndexOf("cx.with_native_receiving_quoted",settlement);expect(receiving).toBeGreaterThan(0);expect(source.slice(receiving,settlement)).toContain("receiving_quote.restricted_by(issued)");expect(source.slice(settlement,branchStart)).toContain("receipt?;advance");expect(source.includes("publication_ownership_progress: Option<(RetainedCloneGrant,RetainedCloneProgress)>")).toBe(true);
 const runStart=source.indexOf("async fn publish_mounted_typed_operation_run_admitted("),run=source.slice(runStart,source.indexOf("fn mounted_typed_interaction_writes_are_next",runStart));expect(run.includes("mounted.publication_ownership_progress.is_some()")).toBe(true);expect(run).toContain("cx.with_retained_loan(remaining");const compactRun=run.replace(/\s+/g,"");expect(compactRun).toContain("original_lifecycle_net_after(cx.retained_progress(),reserved_before,cx.reserved_return_progress())");expect(compactRun).toContain("original_lifecycle_delta(original_before,after)");
 const boundedStart=source.indexOf("fn advance(&mut self, body:usize,grant: store::os_store::one_item::ArtifactStoreOneItemGrant,",source.indexOf("for BoundedConfigPreparation<C, M>")),bounded=source.slice(boundedStart,source.indexOf("fn has_pending_receiving_recovery",boundedStart));expect(boundedStart).toBeGreaterThan(0);expect(bounded.includes("bounded config semantic preparation requires its original incremental mutation owner")).toBe(true);expect(bounded.includes("Mutation::inverse(")).toBe(false);expect(bounded.includes("self.mutation.take()")).toBe(false);
 expect(source.includes("fn take_typed_operation_preparation_receipt(&mut self)")).toBe(true);expect(source.includes("self.preparation_receipt=app.take_typed_operation_preparation_receipt()")).toBe(true);expect(source.includes("let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(body_grant)")).toBe(true);
});

test("mounted preparation counts receiving control and keeps reserved returns on their original receipt", async () => {
 const { receivingTurn: turn } = await Bun.file(new URL("🧫️fixtures/🎟️preparation/🔣️.json", import.meta.url)).json();
 const db = new Database(":memory:");
 try {
  db.run("CREATE TABLE receiving(axis INTEGER, producer INTEGER, control INTEGER, reserved INTEGER, posted INTEGER)");
  turn.contextPosting.forEach((posted: number, axis: number) => db.run("INSERT INTO receiving VALUES (?, ?, ?, ?, ?)", [axis, turn.storePreparation[axis], turn.receivingControl[axis], turn.reservedReturn[axis], posted]));
  expect(db.query("SELECT count(*) AS mismatches FROM receiving WHERE producer + control + reserved <> posted").get()).toEqual({ mismatches: 0 });
  expect(db.query<{ receipt: number }, []>("SELECT posted - reserved AS receipt FROM receiving ORDER BY axis").all().map(({ receipt }) => receipt)).toEqual(turn.preparationReceipt);
  expect(db.query("SELECT count(*) AS axes_missing_control FROM receiving WHERE producer < posted - reserved").get()).toEqual({ axes_missing_control: 2 });
 } finally {
  db.close();
 }
});

test("continuation carries the original preparation and history receipts before another semantic unit",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🎟️preparation/🔣️.json",import.meta.url)).json();
 const db=new Database(":memory:");try{db.run("CREATE TABLE phases(owner TEXT,receipt INTEGER,callbacks INTEGER)");for(const owner of["preparation","history"]){db.query("INSERT INTO phases VALUES(?,?,?)").run(owner,1,1);db.query("INSERT INTO phases VALUES(?,?,?)").run(owner,0,0);}expect(db.query("SELECT sum(receipt) AS receipts,sum(callbacks) AS callbacks FROM phases").get()).toEqual({receipts:2,callbacks:2});}finally{db.close();}
 expect(new Ajv().compile({type:"object",required:["oneOutstandingReceipt"],properties:{oneOutstandingReceipt:{const:true}}})(law)).toBe(true);
 const source=pluginFamilyText();
 expect(source.includes("fn take_history_command_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)>")).toBe(true);expect(source.includes("self.history_command_receipt=app.take_history_command_receipt()")).toBe(true);
 const stopStart=source.indexOf("fn spent(&self, output: &PluginExchangeOutput)"),stop=source.slice(stopStart,source.indexOf("fn expired",stopStart));expect(stop.includes("output.preparation_receipt.is_some()")).toBe(true);expect(stop.includes("output.history_command_receipt.is_some()")).toBe(true);
 const driveStart=source.indexOf("pub async fn plugin_continue_typed_operations"),drive=source.slice(driveStart,source.indexOf("//#endregion",driveStart));expect(drive.includes("collected.1.preparation_receipt=output.preparation_receipt")).toBe(true);expect(drive.includes("collected.1.history_command_receipt=output.history_command_receipt")).toBe(true);
 const inlineStart=source.indexOf("if let Some(operation) = flags.started_typed_operation"),inline=source.slice(inlineStart,source.indexOf("state.effects.append",inlineStart));expect(inline.includes('!state.typed_output.as_mut().expect("retained actual typed output").ownership_receipt_pending()')).toBe(true);expect(source.includes("self.preparation_receipt.is_some()")).toBe(true);expect(source.includes("self.history_command_receipt.is_some()")).toBe(true);expect(inline.includes("typed_output.preparation_receipt=more.preparation_receipt")).toBe(true);expect(inline.includes("typed_output.history_command_receipt=more.history_command_receipt")).toBe(true);
});

test("mounted actor capture borrows original text and yields its paid receipt before publication",async()=>{
 const law=await Bun.file(new URL("../../🏇️mounted-owner/🧪️tests/🧫️fixtures/🎭️actor/🔣️.json",import.meta.url)).json();
 const schema=await Bun.file(new URL("../../🏇️mounted-owner/🧬️schema/🎭️actor/🔣️.json",import.meta.url)).json();expect(new Ajv().compile(schema)(law)).toBe(true);
 const db = new Database(":memory:");
 try {
   const remainingCopy = db.query("SELECT length(CAST($text AS BLOB)) AS bytes, $copy - length(CAST($text AS BLOB)) AS remaining");
   const encoder = new TextEncoder();
   const copyBudget = law.nativeGrant[1];
   for (const sample of law.cases) {
     const text = sample.text ?? sample.repeat.repeat(sample.bytes);
     const bytes = encoder.encode(text).length;
     expect(remainingCopy.get({ $text: text, $copy: copyBudget })).toEqual({ bytes, remaining: copyBudget - bytes });
   }
 } finally { db.close(); }
 const source=pluginFamilyText(),start=source.indexOf("fn publish_mounted_typed_operation_unit("),body=source.slice(start,source.indexOf("fn ack_mounted",start));
 const capture = body.indexOf("mounted.admit_actor_capture(retained,cx)? {return Ok(());}");
 expect(capture).toBeGreaterThanOrEqual(0);
 for (const lane of ["artifact_mutations", "config_mutations", "window_config_mutations", "draft_mutations"]) {
   expect(body.indexOf(`&mut mounted.actor_capture,&mut emit.${lane}`)).toBeGreaterThan(capture);
 }
 expect(body).not.toContain("mounted.meta.actor.clone()");
 expect(source).toContain("actor_capture: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>");
 const frontier=readFileSync(new URL("../../🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs",import.meta.url),"utf8");expect(frontier.includes("self.actor_capture.is_some()")).toBe(true);expect(frontier.includes("self.actor_capture.as_mut()")).toBe(true);
});

test("original member ingress pays native field and registry headers before every physical transfer",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/📮️member-ingress/🔣️.json",import.meta.url)).json();expect(new Ajv().compile({type:"object",required:["currencies","phases","fields","separateRegistrySeal","nativeHeadersMustBePaid"],properties:{currencies:{const:["items","copy","capacity","release","depth"]},phases:{const:["request","field","identity","registry-entry","registry-page","registry-backing"]},fields:{const:10},separateRegistrySeal:{const:true},nativeHeadersMustBePaid:{const:true}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_fields(text TEXT,capacity INTEGER,fields INTEGER)");for(let i=0;i<law.texts.length;i++)db.query("INSERT INTO original_fields VALUES(?,?,?)").run(law.texts[i],law.capacities[i],law.fields);expect(db.query("SELECT sum(capacity*fields) AS backing,count(distinct text) AS vectors FROM original_fields").get()).toEqual({backing:law.capacities.reduce((sum:number,value:number)=>sum+value*law.fields,0),vectors:3});}finally{db.close();}
 const source=pluginFamilyText(),start=source.indexOf("impl OwnedDocumentMemberIngress {"),body=source.slice(start,source.indexOf("impl Drop for OwnedDocumentMemberIngress",start));expect(body.includes("copy_bytes: size_of_val(&*self.request)")).toBe(true);expect(body.includes("size_of::<String>() + size_of::<u8>()")).toBe(true);expect(body.includes("copied_bytes: demand.copy_bytes")).toBe(true);
 const registryStart=source.indexOf("impl OwnedDocumentMemberIngressRegistry {"),registry=source.slice(registryStart,source.indexOf("impl Drop for OwnedDocumentMemberIngressRegistry",registryStart));expect(registry.includes("if !self.sealed")).toBe(true);expect(registry.includes("copy_bytes: size_of::<bool>()")).toBe(true);expect(registry.includes("copied_bytes: demand.copy_bytes")).toBe(true);expect(registry.includes("size_of::<OwnedDocumentMemberIngress>()")).toBe(true);
});

test("unpaid original media control keeps the mounted format worker parked", async () => {
  const fixture = await Bun.file(new URL("🧫️fixtures/🎞️media-control/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const format = db.query("SELECT NOT (? OR ?) AS runnable");
    for (const row of fixture) expect(format.get(Number(row.pagePending), Number(row.controlPending)))
      .toEqual({ runnable: Number(row.formatRunnable) });
  } finally { db.close(); }
  const turn = readFileSync(new URL("../../⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs", import.meta.url), "utf8");
  const ingress = turn.indexOf("::drive_raw_media(runtime, events");
  const format = turn.indexOf("::drive_raw_media_format(runtime");
  expect(ingress).toBeGreaterThanOrEqual(0);
  expect(format).toBeGreaterThan(ingress);
  expect(turn.slice(ingress, format)).toContain("matches!(events.as_ref(), Some(Event::OriginalMediaControl(_)))");
  expect(turn.slice(ingress, format)).toContain("command_page.is_none() && !pending_media_control");
});

test("both close fault observations retain the measured clock", async () => {
  const fixtures = await Bun.file(new URL("🧫️fixtures/⏱️close-fault/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const elapsed = db.query("SELECT CASE WHEN ?1 = 'zero-progress' THEN ?3 ELSE ?2 END AS elapsed");
    for (const row of fixtures) expect(elapsed.get(row.cause, row.callbackUs, row.stallUs)).toEqual({ elapsed: row.reportedUs });
  } finally { db.close(); }
  const registry = await Bun.file(new URL("../../⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🗃️instance-registry/🦀️.rs", import.meta.url)).text();
  const lifetime = await Bun.file(new URL("../../🚪️lifetime/🦀️.rs", import.meta.url)).text();
  const cleanup = await Bun.file(new URL("../../⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs", import.meta.url)).text();
  expect(registry).toMatch(/if cause == RuntimeCleanupFault::ZeroProgress\s*\{\s*self\.stall_credit_spent_us\.load/);
  expect(lifetime).toContain("state.close_fault(cause)");
  expect(cleanup).toContain("state.close_fault(cause)");
});
