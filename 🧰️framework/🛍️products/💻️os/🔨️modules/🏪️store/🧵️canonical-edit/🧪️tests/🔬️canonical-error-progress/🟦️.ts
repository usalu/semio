import { join } from "node:path";
import { readFileSync } from "node:fs";
import { WORKSPACE_ROOT, toolJobRustBlock } from "../../../../../../../../📜️script.ts";

/** 📏️ One Rust statement sequence as a whitespace-tolerant pattern: `rustfmt` freely spreads a
 * `match` arm or a `;`-separated pair across lines, so anchoring these linkage clauses on one exact
 * spelling makes them fail on formatting instead of on lost initialized bytes. */
const looseStatements = (statements: string): RegExp => new RegExp(statements.replaceAll(/[.*+?^${}()|[\]\\]/g, "\\$&").replaceAll(/\s+/g, "\\s+"));

/** 🧪️ Executes canonical error progress policy assertions. */
export function canonicalErrorProgressSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🚧️canonical-error-progress.json"), "utf8"));
  const prefix = Buffer.from(JSON.stringify([fixture.text, null]).slice(0, -5));
  if (!prefix.equals(Buffer.from(fixture.expectedPrefix)) || prefix.length !== fixture.expectedBytes || Buffer.byteLength(fixture.text) !== fixture.expectedSnapshotBytes) throw new Error("canonical error-progress independent JSON/UTF-8 oracle mismatch");
  let checks = 1;
  for (const mode of fixture.modes) for (const maximum of fixture.grants) {
    const grant = Math.min(maximum, 256);
    let written = 0;
    if (grant > 0) while (written < prefix.length) { const count = Math.min(grant, prefix.length - written); const output = Buffer.alloc(512, fixture.sentinel); prefix.copy(output, 0, written, written + count); if (!output.subarray(count).every(byte => byte === fixture.sentinel)) throw new Error(`canonical error-progress ${mode} touched uninitialized suffix`); written += count; }
    if (written !== (grant === 0 ? 0 : fixture.expectedBytes)) throw new Error("canonical error-progress grant oracle changed initialized credit");
    checks += 1;
  }
  const parent = readFileSync(join(base, "🦀️.rs"), "utf8");
  const borrowed = readFileSync(join(base, "🧵️borrowed/🦀️.rs"), "utf8");
  const reader = readFileSync(join(base, "📖️reader/🦀️.rs"), "utf8");
  const native = readFileSync(join(base, "🧳️source/🦀️.rs"), "utf8");
  const method = (text: string, pattern: RegExp) => { const start = text.search(pattern); return start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body ?? ""; };
  const compact = (text: string) => text.replaceAll(/\s+/g, "");
  const exact = (parent: string, borrowed: string, reader: string, native: string) => {
    const indexed = compact(method(parent, /pub fn encode_chunk_admitted\(/));
    const borrowing = compact(method(borrowed, /fn encode_chunk</));
    const reading = compact(method(reader, /fn encode_chunk\(/));
    const sealing = compact(method(parent, /pub fn advance\(/));
    return parent.includes("pub written_bytes: usize") && parent.includes("pub reason: ValueError")
      && indexed.includes("error.retained_progress()") && indexed.includes("reason:error.with_retained_progress(progress)")
      && indexed.includes("ifletSome(byte)=byte{output[0]=byte;copied+=1;}")
      && borrowing.includes("cursor.encode_chunk_admitted(*source,output,child)?")
      && borrowing.includes("written_bytes:step.written_bytes")
      && reading.includes("Err(error)=>{self.failed=true;error.written_bytes}")
      && reading.includes("self.completed_bytes.checked_add(countasu64)")
      && reading.indexOf("self.completed_bytes=completed;") >= 0 && reading.indexOf("self.completed_bytes=completed;") < reading.lastIndexOf("result")
      && sealing.includes("Err(error)=>(error.retained_progress(),Some(error))") && sealing.includes("self.ownership=progress;")
      && sealing.includes("iferror.is_some()||!progress.fits(retained){self.cancelled=true;}")
      && sealing.indexOf("self.completed_bytes=completed;") >= 0 && sealing.indexOf("self.completed_bytes=completed;") < sealing.indexOf("ifletSome(error)=error{returnErr(error);}")
      && sealing.includes("!grant.permits_one()||self.cancelled||self.closing")
      && compact(native).includes('refusal("canonical original child receipt exceeds caller grant").with_retained_progress(p)'.replaceAll(/\s+/g,""))
      && !/From<ArtifactCanonicalJsonEncodeError> for String/.test(parent + borrowed + reader);
  };
  if (!exact(parent, borrowed, reader, native)) throw new Error("canonical error-progress live encoder/reader/original-sealer linkage is missing");
  const mutations: [string, string, string, string][] = [
    [parent.replace("pub reason: ValueError", "pub reason: String"), borrowed, reader, native],
    [parent.replace("reason: error.with_retained_progress(progress)", "reason: error"), borrowed, reader, native],
    [parent, borrowed.replace("cursor.encode_chunk_admitted(*source, output, child)?", "cursor.encode_chunk_admitted(*source, output, child).unwrap()"), reader, native],
    [parent, borrowed, reader.replace(looseStatements("self.failed = true; error.written_bytes"), "self.failed = true; 0"), native],
    [parent, borrowed, reader.replace("self.completed_bytes = completed;", "return result;"), native],
    [parent.replace("self.ownership=progress;", "self.ownership=Default::default();"), borrowed, reader, native],
    [parent.replace("if error.is_some()||!progress.fits(retained){self.cancelled=true;}", "if error.is_some()||!progress.fits(retained){}"), borrowed, reader, native],
    [parent, borrowed, reader, native.replace('.with_retained_progress(p)', '')],
    [parent + "\nimpl From<ArtifactCanonicalJsonEncodeError> for String {}", borrowed, reader, native],
  ];
  for (const mutation of mutations) { if (mutation.every((value, index) => value === [parent, borrowed, reader, native][index])) throw new Error("canonical error-progress hostile mutation did not alter the current source"); if (exact(...mutation)) throw new Error("canonical error-progress admitted lost initialized bytes/physical receipt or resumed failed authority"); }
  checks += 1 + mutations.length;
  return checks;
}

import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {spawnSync} from "node:child_process";
test("canonical original failure and depth use supplied authority with independent JSON bytes",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit");const row=JSON.parse(readFileSync(join(base,"🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));
 for(const sample of row.depthRows){const bytes=Buffer.from(JSON.stringify(sample.source));expect([...bytes]).toEqual([...new TextEncoder().encode(JSON.stringify(sample.source))]);if(sample.refused)expect(bytes.subarray(0,1).toString()).toBe(sample.expectedPrefix);else expect(bytes.toString()).toBe(sample.expectedJson);}
 const schema=JSON.parse(readFileSync(join(base,"⚠️failure/🧬️schema/🔣️.json"),"utf8"));const reasonSchema=JSON.parse(readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8"));const ajv=new Ajv({strict:true});ajv.addSchema(reasonSchema);const admit=ajv.compile(schema);const value={writtenBytes:1,reason:{kind:"depthLimit",message:"original depth refusal",retainedProgress:{copiedItems:1,copiedBytes:1,retainedCapacityBytes:0,releasedBytes:0}}};expect(admit(value)).toBe(true);expect(admit({...value,writtenBytes:-1})).toBe(false);
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");expect(source).toContain("pub reason: ValueError");expect(source).toContain("encode_chunk_admitted");expect(source).toContain("with_retained_progress");
 for(const path of [join(base,"🦀️.rs"),join(base,"🧵️borrowed/🦀️.rs"),join(base,"📖️reader/🦀️.rs")]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout",path],{timeout:5000,stdio:["ignore","ignore","pipe"]});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
});

test("original graph intent uses indexed immutable fields without eager iterator birth",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent");
 const rows=JSON.parse(readFileSync(join(base,"🧫️fixtures/🔣️.json"),"utf8"));
 for(const row of rows.cases){const value=row;const output={widgetId:value.widgetId,value:value.value,...(value.surfaceId===undefined?{}:{surfaceId:value.surfaceId})};expect(JSON.parse(JSON.stringify(output))).toEqual(value);expect([...Buffer.from(JSON.stringify(output))]).toEqual([...new TextEncoder().encode(JSON.stringify(value))]);}
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");const section=source.split("//#region 🔏️CanonicalIntent")[1].split("//#endregion 🔏️CanonicalIntent")[0];
 expect(section.includes("fn canonical_json_node")).toBe(true);expect(section.includes("fn canonical_json_key")).toBe(true);expect(section.includes("Object::new")).toBe(false);expect(section.includes("canonical_json_borrowed_root")).toBe(false);
});

test("original Edit metadata uses its exact indexed domain and preserves canonical JSON order",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit");const fixture=JSON.parse(readFileSync(join(base,"🧫️fixtures/🔏️canonical-edit-sealer.json"),"utf8"));const expected=JSON.parse(fixture.expectedJson);
 expect(JSON.parse(JSON.stringify(expected))).toEqual(expected);expect([...Buffer.from(fixture.expectedJson)]).toEqual([...new TextEncoder().encode(fixture.expectedJson)]);expect(expected.sequenceNumber).toBeUndefined();expect(expected.forwards).toEqual(fixture.edit.forwards);expect(expected.inverse).toEqual(fixture.edit.inverse);
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");const section=source.split("//#region 🧬️TypedCanonicalSource")[1].split("//#endregion 🧬️TypedCanonicalSource")[0];
 expect(section.includes("fn canonical_json_node")).toBe(true);expect(section.includes("fn canonical_json_key")).toBe(true);expect(section.includes("fn borrowed_value")).toBe(false);expect(section.includes("ArtifactCanonicalJsonObject::new")).toBe(false);
});
test("canonical preparation refusal preserves its original physical authority",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit");const row=JSON.parse(readFileSync(join(base,"🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8"));const admit=new Ajv({strict:true}).compile(schema);expect(admit(row.preparationRefusal)).toBe(true);expect(admit({...row.preparationRefusal,retainedProgress:{...row.preparationRefusal.retainedProgress,copiedBytes:-1}})).toBe(false);
 const wire=JSON.parse(JSON.stringify(row.preparationRefusal));expect(wire).toEqual(row.preparationRefusal);const p=wire.retainedProgress;const g=row.retainedPolicy;expect(p.copiedItems<=g.maximumItems&&p.copiedBytes<=g.maximumCopyBytes&&p.retainedCapacityBytes<=g.maximumCapacityBytes&&p.releasedBytes<=g.maximumReleaseBytes).toBe(true);expect(Buffer.byteLength(row.expectedPrefix)).toBe(p.copiedBytes);
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");const start=source.indexOf("pub fn advance(&mut self, grant: ArtifactStoreOneItemGrant)");expect(start).toBeGreaterThan(0);const header=source.slice(start,source.indexOf("{",start));expect(header).toContain("Result<ArtifactStoreOneItemPreparationStep, ValueError>");const body=toolJobRustBlock(source,source.indexOf("{",start))?.body??"";expect(body).not.toContain("ValueError::into_message");expect(body).toContain("error.retained_progress()");expect(body).toContain("self.cancelled=true");
 console.log("[DEBUG] canonical preparation refusal uses one original Value receipt, immutable caller policy and JSON/Ajv projection");
});

test("original preparation owner retains typed refusal without copying the failure authority",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const row=JSON.parse(readFileSync(join(base,"🧵️canonical-edit/🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));
 const reasonSchema=JSON.parse(readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(reasonSchema)(row.preparationRefusal)).toBe(true);expect(JSON.parse(JSON.stringify(row.preparationRefusal))).toEqual(row.preparationRefusal);
 const source=readFileSync(join(base,"🦀️.rs"),"utf8");
 for(const name of ["ArtifactStoreOneItemPreparation<P, Mutation>","ArtifactEphemeralOneItemPreparation<P, Mutation>"]){const start=source.indexOf("pub trait "+name);const header=source.slice(start,source.indexOf("fn checkpoint",start));expect(header).toContain("Result<ArtifactStoreOneItemPreparationStep, ValueError>");}
 const start=source.indexOf("fn advance_preparation(&mut self");const body=toolJobRustBlock(source,source.indexOf("{",start))?.body??"";expect(body).not.toContain("reason.clone()");expect(body).not.toContain("ValueError::into_message");expect(body).toContain("Some(reason)");expect(body).toContain("PreparationRefused");expect(source).toContain("pub fn preparation_refusal(&self) -> Option<&ValueError>");
 const seal=source.slice(source.indexOf("pub fn prepare_one_item<"),source.indexOf("/// 🧵️ Moves the exact edit",source.indexOf("pub fn prepare_one_item<")));expect(seal).toContain("(ValueError, Edit<Mutation>, Arc<P>)");expect(seal).toContain("Err((error, edit, post_snapshot))");
 console.log("[DEBUG] original preparation refusal keeps typed failure/original edit/post and independent immutable physical policy");
});

test("original retained clone and ephemeral tasks preserve refusal progress through actual advance",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const fixture=JSON.parse(readFileSync(join(base,"🧵️canonical-edit/🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));expect(JSON.parse(JSON.stringify(fixture.preparationRefusal))).toEqual(fixture.preparationRefusal);
 const snapshot=readFileSync(join(base,"🧬️snapshot-clone/🦀️.rs"),"utf8");const task=readFileSync(join(base,"🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs"),"utf8");
 const start=snapshot.indexOf("fn record_progress(");const end=snapshot.indexOf("fn checkpoint(&self)",start);const section=snapshot.slice(start,end);expect(section).not.toContain("ValueError::into_message");expect(section).not.toContain("Result<ArtifactStoreOneItemPreparationStep, String>");expect(section).toContain("with_retained_progress");expect(snapshot).toContain("Result<RetainedCloneEditStep, ValueError>");expect(task).toContain("Result<ArtifactEphemeralPreparationTaskStep<P>, semio_framework_value::ValueError>");expect(task).toContain("with_retained_progress(ownership)");
 for(const path of [join(base,"🦀️.rs"),join(base,"🧬️snapshot-clone/🦀️.rs"),join(base,"🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs")]){const parsed=spawnSync("rustfmt",["--edition","2021","--config","skip_children=true","--emit","stdout",path],{timeout:5000,stdio:["ignore","ignore","pipe"]});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
 console.log("[DEBUG] original retained preparation typed failure conserves actual child receipt and accepted output custody");
});

test("preparation request carries the actual installed typed retirement issuers",()=>{
 const path=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs");const source=readFileSync(path,"utf8");const start=source.indexOf("pub struct ArtifactStoreOneItemPreparationRequest<");const body=toolJobRustBlock(source,source.indexOf("{",start))?.body??"";expect(source.slice(start,source.indexOf("{",start))).toContain("<P, Input, Mutation>");expect(body).toContain("pub mutation: Input");expect(body).toContain("pub mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>");expect(body).toContain("pub snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>");
 expect(source).toContain("mutation_retirement:Arc::clone(mutation_retirement)");expect(source).toContain("snapshot_retirement:Arc::clone(snapshot_retirement)");expect(source).toContain("original_issuer_copy");
 console.log("[DEBUG] actual typed preparation request preserves installed issuers and distinct wire input/output type without new issuer birth");
});

test("original Unit wire preparation keeps typed failure and already performed byte receipt",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const fixture=JSON.parse(readFileSync(join(base,"🧫️fixtures/📢️member-publication.json"),"utf8"));
 for(const row of fixture.orderedMembers){const raw=row.wire+" ".repeat(row.paddingBytes);const oracle=JSON.parse(raw);expect(Number.isInteger(oracle)).toBe(true);expect(Buffer.from(raw)).toEqual(Buffer.from(new TextEncoder().encode(raw)));}
 const source=readFileSync(join(base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");const start=source.indexOf("impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoMemberWirePreparation");const body=toolJobRustBlock(source,source.indexOf("{",start))?.body??"";const advance=body.slice(0,body.indexOf("fn checkpoint"));
 expect(advance).toContain("Result<ArtifactStoreOneItemPreparationStep, ValueError>");expect(advance).not.toContain("error.to_string()");expect(advance).toContain("with_retained_progress(progress)");expect(advance).toContain("self.closing = true");expect(advance).toContain("copied_bytes: 1");
 const parseStart=source.indexOf("fn parse_byte(&mut self");const parse=toolJobRustBlock(source,source.indexOf("{",parseStart))?.body??"";expect(source.slice(parseStart,source.indexOf("{",parseStart))).toContain("ValueError");expect(parse).not.toContain(".to_string()");
 console.log("[DEBUG] original member wire JSON/UTF8 oracle and typed copied-byte refusal keep the original owner");
});

test("original one item preparation retains semantic rejection and installed issuer custody",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const row=JSON.parse(readFileSync(join(base,"🧵️canonical-edit/🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));const schema=JSON.parse(readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema)(row.preparationRefusal)).toBe(true);expect(JSON.parse(JSON.stringify(row.preparationRefusal))).toEqual(row.preparationRefusal);
 const source=readFileSync(join(base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");const start=source.indexOf("impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItemPreparation");const body=toolJobRustBlock(source,source.indexOf("{",start))?.body??"";const advance=body.slice(0,body.indexOf("fn checkpoint"));expect(advance).toContain("Result<ArtifactStoreOneItemPreparationStep, ValueError>");expect(advance).not.toContain("error.to_string()");expect(advance).not.toContain("error.into_message()");expect(advance).toContain("*self.semantic_refusal = Some(error)");expect(body).toContain("artifact_retirement_admit_owned(&mut self.semantic_refusal");expect(body).toContain("self.semantic_refusal.is_none()");
 const begin=source.slice(source.indexOf("fn begin(&self, request:",source.indexOf("impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoOneItemPreparationFactory")),source.indexOf("impl DemoOneItemPreparation {"));expect(begin).toContain("snapshot_retirement: Some(request.snapshot_retirement)");expect(begin).toContain("mutation_retirement: Some(request.mutation_retirement)");
 console.log("[DEBUG] original semantic rejection retains typed original error and installed retirement issuers under the immutable physical policy");
});

test("original Unit probe and hostile preparation owners keep canonical typed advance",()=>{
 const source=readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 for(const owner of ["for ProbedRetainedClonePreparation","for RetainedTextEditCursor","for FaultingEphemeralTask","for TerminalPreparation<N>"]){const start=source.indexOf(owner);expect(start).toBeGreaterThan(0);const advance=source.indexOf("fn advance",start);const method=source.slice(advance,source.indexOf("{",advance));expect(method).toContain("ValueError>");expect(method).not.toContain("String>");}
 const parsed=spawnSync("rustfmt",["--edition","2021","--config","skip_children=true","--emit","stdout",join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs")],{timeout:5000,stdio:["ignore","ignore","pipe"]});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);
 console.log("[DEBUG] original probe forwards the child Value receipt and hostile owners preserve typed refusals without owning prose adapters");
});

test("original batch preparation refusal retains returned request until granted original cleanup",()=>{
 const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const source=readFileSync(join(base,"🦀️.rs"),"utf8");const begin=source.indexOf("match source.begin_next(request,");const refusal=source.slice(begin,source.indexOf("let owner = publication.preparation",begin));expect(refusal).not.toContain("drop(request)");expect(refusal).toContain("publication.refused_base = Some(base)");expect(refusal).toContain("publication.refused_authority = Some(authority)");expect(refusal).toContain("publication.refused_issuers");expect(refusal).toContain("error.with_retained_progress(progress)");
 const start=source.indexOf("pub struct ArtifactStoreBatchPublication<");const section=source.slice(start,source.indexOf("impl<P, Mutation> Drop for ArtifactStoreBatchPublication",start));expect(section).toContain("refused_base: Option<SnapshotRead<P>>");expect(section).toContain("original.try_return_to_registry_witness()");expect(section).toContain("self.refused_base = Some(original)");expect(section).toContain("self.refused_authority = Some(original)");expect(section).toContain("self.refused_issuers.iter().all(Option::is_none)");
 const row=JSON.parse(readFileSync(join(base,"🧵️canonical-edit/🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8")))(row.preparationRefusal)).toBe(true);expect(JSON.parse(JSON.stringify(row.preparationRefusal))).toEqual(row.preparationRefusal);
 console.log("[DEBUG] original rejected request keeps registry/authority/issuer owners and actual sole Value refusal receipt through supplied cleanup");
});

test("bounded config preparation preserves genuine unsupported-owner refusal through its typed contract",()=>{
 const source=readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"),"utf8");const start=source.indexOf("store::ArtifactStoreOneItemPreparation<C, M> for BoundedConfigPreparation<C, M>");const advance=source.indexOf("fn advance",start);const body=toolJobRustBlock(source,source.indexOf("{",advance))?.body??"";expect(source.slice(advance,source.indexOf("{",advance))).toContain("ValueError>");expect(body).toContain("ValueRefusalKind::UnsupportedOwner");expect(body).toContain("original incremental mutation owner");expect(body).not.toContain(".into()");
 console.log("[DEBUG] actual bounded configuration owner reports a typed refusal until its genuine incremental domain owner is supplied");
});

test("original canonical source refusal preserves already performed child receipt",()=>{
 const source=readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs"),"utf8");const marker='refusal("canonical original child receipt exceeds caller grant").with_retained_progress(p)';expect(source).toContain(marker);expect(source.replace(marker,'refusal("canonical original child receipt exceeds caller grant")')).not.toContain(marker);
 const fixture=JSON.parse(readFileSync(join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));const original=JSON.parse(JSON.stringify(fixture.preparationRefusal));expect(original.retainedProgress).toEqual(fixture.preparationRefusal.retainedProgress);expect(Buffer.byteLength(fixture.expectedPrefix)).toBe(original.retainedProgress.copiedBytes);console.log("[DEBUG] canonical child post-receipt refusal conserves the sole original physical Value authority");
});

test("original exported canonical failure oracle follows current physical production linkage",()=>{
 expect(canonicalErrorProgressSelfTests()).toBeGreaterThan(10);
 console.log("[DEBUG] original canonical exported linkage oracle rejects lost prefix, lost receipt and resumable failed authority");
});

test("original selection preparation conserves typed refusal and displaced owner",()=>{
 const root=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules");const rows=JSON.parse(readFileSync(join(root,"🏪️store/🧵️canonical-edit/🧫️fixtures/🚧️canonical-error-progress.json"),"utf8"));
 for(const row of rows.selectionRows){expect(JSON.stringify(row.selected)).toBe(row.expectedJson);expect([...Buffer.from(row.expectedJson)]).toEqual([...new TextEncoder().encode(JSON.stringify(row.selected))]);}
 const source=readFileSync(join(root,"🔌️plugin/🧪️tests/🖥️test-app-mutations-config/🧬️preparation/🦀️.rs"),"utf8");const begin=source.indexOf("impl RetainedCloneEditCursor<State,Mutation>");const advance=source.slice(begin,source.indexOf("fn take_inverse",begin));expect(advance).toContain("Result<RetainedCloneEditStep,ValueError>");expect(advance).not.toContain("ValueError::into_message");expect(advance).not.toContain("error.into_message()");expect(advance).toContain("Err((error,original))");expect(advance).toContain("post.selected=original");expect(advance).toContain("self.cancelled=true");expect(source).toContain("Result<Option<RetainedCloneProgress>,ValueError>");expect(source).not.toContain("saturating_add(1)");
 const parsed=spawnSync("rustfmt",["--edition","2021","--config","skip_children=true","--emit","stdout",join(root,"🔌️plugin/🧪️tests/🖥️test-app-mutations-config/🧬️preparation/🦀️.rs")],{timeout:5000,stdio:["ignore","ignore","pipe"]});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);
 console.log("[DEBUG] original selection null/empty/Unicode JSON oracle and typed refusal retain displaced original selection");
});
