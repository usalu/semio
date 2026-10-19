import { WORKSPACE_ROOT } from "../../../../🗂️workspaces/📍️checkout-root/🟦️.ts";
import { inspectRustModuleGraph, inspectRustModuleGraphFacts } from "../../../../🔍️discovery/🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyAllRustFiles } from "../../../🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { toolJobStoreInitializerRetainedExact } from "../📄️caller-retained-documents/🟦️.ts";
import { toolJobRustBlock, toolJobProductionProtocolSource } from "../../📋️inventory/🟦️.ts";


type RasterEnvelopeSourceEvidence = Readonly<{ store: string; raster: string; editor: string; plugin: string; laws: string; productionPaths: readonly string[]; testPaths: readonly string[] }>;

/** 🖨️ Reads the current declared Raster owners and their mounted tests as distinct source evidence. */
export function toolJobRasterEnvelopeSourceEvidence(root: string = WORKSPACE_ROOT): RasterEnvelopeSourceEvidence {
  const raster = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster", subset = raster + "/🏅️standards/🔖️1/🪆️subsets/✳️any";
  const store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store", plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin", value = "🧰️framework/🔨️modules/🌱️value";
  const domainPaths = [raster + "/🦀️.rs", raster + "/🔨️modules/🏠️host/🧰️owned/🦀️.rs", subset + "/🧬️schema/📸️snapshot/🦀️.rs", subset + "/🧬️schema/🧬️mutations/🦀️.rs", subset + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs", subset + "/🚪️io/🦀️.rs", ...["🔣️json/🔖️rfc8259/✳️any", "📷️png/🔖️1.2/✳️any", "🪟️bmp/🔖️v3/✳️any", "🖼️tiff/🔖️6.0/✳️any", "📸️jpg/🔖️jfif-1.01/♾️any", "🎞️gif/🔖️87a/✳️any", "🎨️svg/🔖️1.1/✳️any"].map(path => subset + "/🚪️io/📤️export/🧵️serializers/🗿️artifacts/" + path + "/🦀️.rs")];
  const editorPaths = [subset + "/✏️editor/🦀️.rs", subset + "/✏️editor/📤️export/🦀️.rs"];
  const testPaths = [raster + "/🔨️modules/🏠️host/🧰️owned/🧪️tests/🦀️.rs", subset + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", subset + "/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", subset + "/✏️editor/📤️export/🧪️tests/🦀️.rs"];
  const retainedMutation = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained";
  const sharedPaths = [store + "/📦️codec/🧬️owned-schema/🦀️.rs", store + "/📬️paged-one-item/🦀️.rs", store + "/📬️paged-one-item/🧬️mutation-apply/🦀️.rs", store + "/🏗️initialization/🔁️replay/🦀️.rs", retainedMutation + "/🦀️.rs", retainedMutation + "/🔺️diff/🦀️.rs", value + "/🧬️retained-clone/🧺️std-collections/🦀️.rs", value + "/♻️retirement/🦀️.rs", value + "/♻️retirement/🔗️shared/🦀️.rs", value + "/♻️retirement/🎮️controlled/🦀️.rs"];
  const pluginPaths = [plugin + "/🦀️.rs", plugin + "/🏛️ownership/🦀️.rs", plugin + "/🧵️instance/🦀️.rs", plugin + "/🌱️initialization/♻️retirement/🦀️.rs", plugin + "/👷️job/🏛️reserved/🦀️.rs", plugin + "/♻️lifecycle/🦀️.rs", plugin + "/🗃️document/🦀️.rs", plugin + "/📜️declaration/🧩️app/🦀️.rs", plugin + "/📜️declaration/🪟️surface/🦀️.rs", plugin + "/👷️job/🚪️session/🦀️.rs"];
  const files = [...policyAllRustFiles(root).filter(path => path.startsWith(raster + "/")), raster + "/📦️packages/🦀️rust/Cargo.toml"];
  const graph = inspectRustModuleGraph(files, path => policyReadFileSafe(root, path) || undefined);
  for (const path of [...domainPaths, ...editorPaths, ...testPaths]) if (!graph.contexts.get(path)?.some(context => context.crateRoot === raster + "/🦀️.rs")) throw new Error("[raster evidence] current owner is not mounted: " + path);
  const read = (path: string) => { const source = policyReadFileSafe(root, path); if (!source) throw new Error("[raster evidence] source absent: " + path); return source; };
  const production = (paths: readonly string[]) => paths.map(path => toolJobProductionProtocolSource(read(path))).join("\n");
  const host = read(domainPaths[1]!);
  const declaredHostTests = inspectRustModuleGraphFacts(host).modules.some(module => module.conditional && module.pathTarget === "🧪️tests/🦀️.rs");
  const mounts = declaredHostTests ? "mounted-raster-host-laws" : "";
  return { store: production(sharedPaths), raster: production(domainPaths), editor: production(editorPaths), plugin: production(pluginPaths), laws: mounts + "\n" + testPaths.map(read).join("\n"), productionPaths: [...domainPaths, ...editorPaths, ...sharedPaths, ...pluginPaths], testPaths };
}

/** 🧾️ Current Raster obligations follow original typed owners, quoted currencies and the single shared apply/seal path. */
export function toolJobRasterEnvelopeCallerFailures(store: string, raster: string, editor: string, plugin: string, laws: string): string[] {
  const failures: string[] = [], require = (id: string, condition: boolean) => { if (!condition) failures.push(id); }, compact = (source: string) => source.replaceAll(/\s+/g, "");
  const body = (source: string, anchor: string) => { const start = source.indexOf(anchor); return start < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", start))?.body ?? ""; };
  const preparationStart = editor.search(/fn\s+build_artifact_store_one_item_preparation_factory\s*\(/);
  const preparation = preparationStart < 0 ? "" : toolJobRustBlock(editor, editor.indexOf("{", preparationStart))?.body ?? "";
  require("central-factory", compact(preparation) === "Some(store::mutation_apply_preparation_factory::<RasterSnapshot,RasterMutation>())");
  const forbidden = ["struct RasterStoreInitializationAuthority", "struct RasterMutationCandidateAuthority", "struct RasterOneItemApply", "struct RasterSnapshotCloneAuthority", "struct RasterLayerCloneAuthority", "struct RasterLayerLocator", "struct RasterStorePreparationFactory", "fn build_document_store_initialization_job", "raster_document_store_initialization_job", "fn node_at_mut", "fn container_mut", "snapshot_mut", "operation.diff(", "diff.apply("];
  require("single-apply-owner", forbidden.every(anchor => !raster.includes(anchor) && !editor.includes(anchor)) && !editor.includes("apply_diff_cold(") && !editor.includes("apply_diff_into("));
  require("original-field-catalog", ["pub struct RasterEnvelopeOwnedFieldCatalog", "artifact_owned_spr_edit_history_decoder", "RasterSnapshotDecodeAuthority", "RasterMutationDecodeAuthority", "OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>", "pub fn raster_envelope_decode_owner_bundle()", "pub fn raster_document_store_owners()"].every(anchor => raster.includes(anchor)));
  require("shared-history-decoder", store.includes("pub fn artifact_owned_spr_edit_history_decoder") && store.includes("struct ArtifactOwnedSprMutationArrayAuthority") && !raster.includes("artifact_bounded_history_entry_decoder"));
  require("owned-retirement-issuers", raster.includes("pub struct RasterSnapshotRetirementFactory") && raster.includes("pub struct RasterMutationRetirementFactory") && (raster.match(/retirement::admit_owned_retirement\(value, grant\)/g) ?? []).length === 2);
  require("shared-retirement-issuer", raster.includes("retirement::shared::admit_shared_retirement(snapshot, grant, true)"));
  const ownedBirth = compact(body(store, "pub fn admit_typed_controlled_retirement<")), sharedBirth = compact(body(store, "pub fn admit_shared_retirement<"));
  require("shared-owned-birth", ownedBirth.includes("grant.maximum_items==0") && ownedBirth.includes("grant.maximum_depth==0") && ownedBirth.includes("layout.size()>grant.maximum_capacity_bytes") && ownedBirth.indexOf("layout.size()>grant.maximum_capacity_bytes") < ownedBirth.indexOf("std::alloc::alloc(layout)") && ownedBirth.includes("retained_capacity_bytes:layout.size()") && ownedBirth.includes("value));"));
  require("shared-arc-birth", sharedBirth.includes("grant.maximum_items==0") && sharedBirth.includes("grant.maximum_depth==0") && sharedBirth.includes("layout.size()>grant.maximum_capacity_bytes") && sharedBirth.indexOf("layout.size()>grant.maximum_capacity_bytes") < sharedBirth.indexOf("std::alloc::alloc(layout)") && sharedBirth.includes("retained_capacity_bytes:layout.size()") && sharedBirth.includes("source));"));
  require("shared-physical-retirement", store.includes("controlled retirement abandoned before terminal-empty") && store.includes("shared retirement abandoned actual Arc allocation ownership") && compact(store).includes("Arc::try_unwrap(source)") && compact(store).includes("Arc::weak_count(source)==0") && compact(store).includes("grant.maximum_release_bytes<extent"));

  const quotedBirth = (owner: string, source: string, message: string) => owner.includes("iflayout.size()>grant.maximum_capacity_bytes{returnErr((ValueError::literal(ValueRefusalKind::OwnershipLimit,\"" + compact(message) + "\")," + source + "));}") && owner.includes("copied_items:1") && !owner.includes("expect(") && !owner.includes("unwrap(") && !owner.includes("panic!(");
  require("exact-controlled-producer-refusal", quotedBirth(ownedBirth, "value", "typed retirement frame exceeds admitted capacity"));
  require("exact-shared-producer-refusal", quotedBirth(sharedBirth, "source", "shared frame exceeds admitted capacity"));
  const controlledOwner = body(store, "impl<T: RetireOwned> ControlledRetirement<T>"), controlledDemand = compact(body(controlledOwner, "pub fn next_demand("));
  const controlled = compact(body(controlledOwner, "fn step_original(&mut self, grant: RetainedCloneGrant)"));
  const controlledAdmission = ["letdemand=self.next_demand(grant.maximum_copy_bytes)?;", "grant.maximum_depth<demand.depth", "grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes", "self.cursors.release_empty_page(grant.maximum_release_bytes)"].map(anchor => controlled.indexOf(anchor));
  require("controlled-body-quote", controlledAdmission.every((position, index) => position >= 0 && (index === 0 || position > controlledAdmission[index - 1]!)) && (controlled.match(/self\.next_demand\(grant\.maximum_copy_bytes\)/g) ?? []).length === 1 && controlledDemand.includes("letmutd=cursor.next_demand(body)?;") && controlled.includes("letbody_bytes=grant.maximum_copy_bytes;") && controlled.includes("letminimum_work_bytes=demand.copy_bytes;") && controlled.includes("letbirth=demand.capacity_bytes;"));
  require("controlled-turn-currencies", controlled.includes("grant.maximum_items==0") && controlled.includes("grant.maximum_depth<demand.depth") && controlled.includes("grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes") && controlled.includes("body_bytes<minimum_work_bytes") && controlled.includes("maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:grant.maximum_depth-1") && controlled.includes("progress.fits(cursor_grant)") && controlled.includes("self.cursors.release_empty_page(grant.maximum_release_bytes)"));
  require("controlled-frame-physical-return", controlled.includes("cursor.terminal_release_bytes()") && controlled.includes("ifbytes>grant.maximum_release_bytes") && controlled.includes("released_bytes:bytes") && controlledDemand.includes("d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,\"retirementfrontierdepthoverflow\"))?;"));
  const bytes = compact(body(store, "impl RetirementCursor for Bytes"));
  require("original-string-byte-capacity", compact(body(store, "impl RetireOwned for String")).includes("bytes(self.into_bytes())") && bytes.includes("letbytes=self.0.capacity()") && bytes.includes("ifbytes>grant.maximum_release_bytes") && bytes.includes("RetirementStep::Bytes(bytes)") && bytes.includes("Some(ifself.0.is_empty(){self.0.capacity()}else{0})") && store.includes("owned bytes retired before terminal-empty"));
  const shared = compact(body(store, "impl<T:RetireOwned+Sync> SharedControlledRetirement<T>"));
  require("exact-shared-owner-return", shared.includes("Arc::strong_count(source)!=1") && shared.includes("Arc::weak_count(source)==0") && shared.includes("Err(source)=>{*self.source=Some(source)") && shared.includes("released_bytes:extent") && shared.includes("self.source.is_none()&&self.owned.is_none()"));

  const collection = compact(body(store, "impl<T: RetireOwned> RetirementCursor for Collection<T>"));
  require("original-vector-backing", collection.includes("self.0.capacity()*size_of::<T>()") && collection.includes("ifbytes>maximum_bytes") && collection.includes("self.0.is_empty()&&(self.0.capacity()==0||size_of::<T>()==0)") && collection.includes("RetirementStep::Child(value.retirement())") && collection.includes("grant.maximum_capacity_bytes<birth") && store.includes("owned collection retired before terminal-empty"));
  const registry = compact(body(plugin, "impl<T> ArtifactFixedRegistry<T>"));
  const registryRelease = compact(body(plugin, "pub(crate) fn close_empty_backing_granted("));
  require("fixed-registry-retains-physical-owner", registry.includes("slots.try_reserve_exact(capacity).is_ok()") && registry.includes("self.allocation_admitted&&self.entry(slot).is_none()") && registry.includes("self.entry(index).is_none_or(|(candidate,_)|*candidate!=id)") && registry.includes("self.occupied&=!(1u64<<index)") && registry.includes("self.slots[index].assume_init_read().1") && registry.includes("size_of_val(self.slots.as_ref())") && registryRelease.includes("grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0") && registryRelease.includes("released_bytes:bytes") && plugin.includes("fixed artifact owner registry reached Drop before every exact owner was transferred to bounded retirement"));
  require("fixed-registry-saturated-original-refusal", compact(plugin).includes("!self.jobs.can_insert(self.operation.0){returnErr(envelope);") && compact(plugin).indexOf("!self.jobs.can_insert(self.operation.0)") < compact(plugin).indexOf("letjob=A::build_document_store_initialization_job(envelope,self.operation,self.generation,self.actor.clone())?"));
  const fields = compact(body(raster, "fn accept_token(")), close = compact(body(raster, "fn close_step(&mut self, grant: RetainedCloneGrant)"));
  require("field-identity", fields.includes("cx.operation()!=self.operation||cx.generation()!=self.generation"));
  require("field-cancellation", fields.includes("ifcx.is_cancelled()"));
  require("field-deadline", fields.includes("ifcx.should_yield()"));
  require("field-close-admission", close.includes("grant.maximum_items==0") && close.includes("grant.maximum_depth<demand.depth") && close.includes("raster_short(grant,demand)") && compact(raster).includes("grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes"));
  require("field-original-close", close.includes("artifact_retirement_box_close_step(&mutself.retirement,grant)") && close.includes("artifact_retirement_admit_owned(&mutself.value,&mutself.retirement,grant)") && raster.includes("decode reached Drop before publication or bounded retirement"));
  require("fixed-map-backing", compact(raster).includes("pubconstRASTER_OWNED_MAP_CAPACITY:usize=64;") && compact(raster).includes("pubconstRASTER_OWNED_MAP_PAGE_CAPACITY:usize=8;") && compact(raster).includes("pages:std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>;RASTER_OWNED_MAP_PAGE_COUNT]>"));
  const mapInsert = compact(body(raster, "pub(crate) fn insert_pre_admitted("));
  require("map-cap-refusal", mapInsert.includes("self.length>=RASTER_OWNED_MAP_CAPACITY") && mapInsert.includes("RasterOwnedMapRejected{key,value,reason:\"raster-map.item-capacity\"}") && mapInsert.includes("std::mem::replace(entry,(key,value))") && mapInsert.includes("RasterOwnedMapEntry::new(previous_key,previous_value)"));
  require("map-key-handback", raster.includes("pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>") && !raster.includes("pub fn remove(&mut self, key: &str)"));
  require("map-terminal-drop", compact(body(raster, "impl<V> Drop for RasterOwnedMap<V>")).includes("self.length==0&&self.pages.iter().all(Option::is_none)"));
  const mapRetirement = compact(body(raster, "impl<V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for RasterOwnedMapRetirement<V>"));
  require("map-owner-retirement", mapRetirement.includes("grant.maximum_items==0||grant.maximum_depth==0") && mapRetirement.includes("RetirementStep::Child(entry.retirement())") && mapRetirement.includes("bytes>grant.maximum_release_bytes") && mapRetirement.includes("take_empty_page_backing()") && mapRetirement.includes("page.release();") && mapRetirement.includes("self.map.length==0&&self.map.allocated_page_count()==0"));
  require("map-page-physical-size", compact(raster).includes("size_of::<RasterOwnedMapPage<V>>()") && compact(raster).includes("Some(size_of::<Self>())") && raster.includes("Raster owned map retirement abandoned before terminal-empty"));
  const mapQuote = compact(body(raster, "fn insertion_demands(building:"));
  require("map-clone-admission", raster.includes("RetainedCloneEntriesCursor<Self>") && mapQuote.includes("map.page_required_for_insert(key)") && mapQuote.includes("size_of::<RasterOwnedMapPage<V>>()") && mapQuote.includes("copy_bytes:size_of::<(String,V)>()+RASTER_OWNED_MAP_CAPACITY+capacity") && mapQuote.includes("capacity_bytes:capacity") && compact(raster).includes("self.insert(key,value).map_err(|rejected|(rejected.key,rejected.value))"));
  const insertAt = store.indexOf("            INSERT => {"), insertEnd = store.indexOf("            DONE =>", insertAt), insert = compact(insertAt < 0 ? "" : store.slice(insertAt, insertEnd));
  const admission = insert.indexOf("grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth");
  require("shared-map-original-quote", insert.includes("M::insertion_demands(self.building.as_ref(),self.key.as_ref()") && admission >= 0 && insert.indexOf("self.key.take()") > admission && insert.indexOf("self.value.take()") > admission && insert.includes("retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes") && compact(store).includes("INSERT=>returnOk(M::insertion_demands(self.building.as_ref(),self.key.as_ref()"));
  require("typed-mutation-retirement", /derive\([^\]]*RetireOwned[^\]]*\)\][\s\S]{0,400}pub enum RasterMutation/.test(raster) && raster.includes("RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset)") && raster.includes("params: RasterOwnedMap<semio_framework_value::DslValue>"));
  require("typed-clone", /derive\([^\]]*RetainedClone[^\]]*\)\][\s\S]{0,200}pub struct RasterSnapshot/.test(raster) && /derive\([^\]]*RetainedClone[^\]]*\)\][\s\S]{0,200}pub enum RasterLayerNode/.test(raster));
  require("paged-original-birth", store.includes("if !grant.permits_one() || self.cancelled || self.closing") && store.includes("base.admit_retained_clone_source(clone_grant)") && store.includes("source.try_borrow()?") && store.includes("admit_retained_clone_progress") && store.includes("self.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)"));
  const central = compact(store), commit = compact(body(store, "pub fn commit_retained_diff<")), inverse = compact(body(store, "pub fn inverse_retained_mutation<"));
  const preflightPhase = compact(body(store, "Stage::Preflight=>")), inversePhase = compact(body(store, "Stage::Inverse=>")), demand = compact(body(store, "pub fn retained_mutation_demand_fits("));
  require("single-central-mutation", preflightPhase.includes("RetainedDiffPreflightStep::Admitted(_,rows)=>{self.messages[1]=Some(rows);self.stage=Stage::Inverse;}") && inversePhase.includes("inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)") && inversePhase.includes("RetainedMutationInverseStep::Ready(_,rows)=result{*self.inverse=Some(rows);self.stage=Stage::Commit;}") && central.includes("preflight_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post,g)") && central.includes("inverse_retained_mutation(self.semantics.as_mut().unwrap(),mutation,&a,g)") && central.includes("commit_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post,g)") && ["g.maximum_items>0", "g.maximum_copy_bytes>=d.copy_bytes", "g.maximum_capacity_bytes>=d.capacity_bytes", "g.maximum_release_bytes>=d.release_bytes", "g.maximum_depth>=d.depth"].every(axis => demand.includes(axis)) && inverse.includes("if!retained_mutation_demand_fits(d,grant)") && commit.includes("if!retained_mutation_demand_fits(demand,grant)") && commit.includes("diff.bind(&mutowner.diff,grant)?.is_some()") && commit.includes("original.bind(&mutowner.original,grant)?.is_some()") && commit.includes("super::super::mint_capability()") && commit.includes("admit_retained_clone_progress(grant,receipt"));
  require("funded-sealer", store.includes("retained.maximum_capacity_bytes < wrapped") && store.includes("retained.maximum_copy_bytes < ArtifactStoreOneItemSealer::<S, M>::constructor_copy_byte_demand()") && store.includes("authority.begin_one_item_seal(edit, Arc::new(post), mutation_retirement, snapshot_retirement, retained)"));
  require("default-replay-owner", plugin.includes("Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))") && compact(plugin).includes("store::ArtifactStoreReplayInitializer::new(envelope,schema,operation,generation,actor)"));
  const replay = compact(body(store, "fn turn(&mut self, cx: &mut StepContext<'_>) -> Turn"));
  require("replay-entry-denial", replay.includes("cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0") && replay.indexOf("cx.should_yield()") < replay.indexOf("self.settle(cx)") && replay.includes("letunclaimed=self.phase==Phase::Complete&&!self.terminal_handoff&&self.candidate.is_some()"));
  require("replay-candidate-cancellation", compact(store).includes("self.phase!=Phase::Complete||self.terminal_handoff||self.cancel_requested") && store.includes("self.generation.0.checked_add(1)"));
  require("retained-replacement-session", compact(plugin).includes("session:ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P,Mutation>>>>") && compact(plugin).includes("session_rejected:ManuallyDrop<Option<crate::reserved_job_session::RefusedWorkerSource<ArtifactStoreInitializationJob<P,Mutation>>>>") && compact(plugin).includes("crate::reserved_job_session::admit_mounted(job,params,retained)") && compact(plugin).includes("self.jobs.insert_admitted(self.operation.0,ActiveArtifactStoreReplacement::new(self.operation,self.generation,self.child_content_generation,job,self.retained))") && plugin.includes("artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty") && plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") && plugin.includes("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close"));
  const credit = laws.indexOf("begin_artifact_envelope_ingress(pages, "), page = laws.indexOf("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]"), construct = laws.indexOf("ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())"), admitted = laws.indexOf("admit_artifact_envelope_ingress_page(handle, page)");
  require("ingress-order", credit >= 0 && page > credit && construct > page && admitted > construct && laws.includes("chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") && laws.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle"));
  require("ingress-owned-pump", laws.includes("seal_artifact_envelope_ingress(handle)") && laws.includes("fixture_mounted_policy().maintenance") && laws.includes("advance_artifact_envelope_load(handle)") && laws.includes("assert_eq!(handle.generation, base_generation)") && laws.includes("duplicate Raster load acknowledgement is a no-op") && laws.includes("cancel_artifact_envelope_load(handle)"));
  require("ingress-no-whole-bypass", !editor.includes("envelope_json: &str") && !editor.includes("reject_whole_buffer_artifact_envelope_ingress") && !editor.includes("ArtifactStore::new"));
  require("editor-owned-decode", compact(body(editor, "fn build_envelope_decode_owner_bundle()")) === "Some(crate::host::owned::raster_envelope_decode_owner_bundle())");
  require("bounded-mounted-output", editor.includes("if context.should_yield(){return Ok(None);}") && editor.includes("context.is_cancelled()") && editor.includes("bytes.len()>BASE64_INPUT_BYTES") && editor.includes("credit.credit(chunk.len())?") && editor.includes("self.chunks.as_ref().unwrap().push(chunk)?") && editor.includes("self.work.preparing.take()") && editor.includes("self.work.encoding.take()") && editor.includes("raster export snapshot has no live close witness"));
  require("output-domain-preflight", raster.includes("if depth>32") && raster.includes("self.nodes>1024") && raster.includes("validate_extent(image.width,image.height)") && raster.includes("frame.rgba8.len()!=count*4") && editor.includes("raster_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1)"));
  require("serializer-real-output", (raster.match(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\)/g) ?? []).length === 7 && raster.includes("write_json_pretty(&serialize(snapshot)?.value)") && raster.includes("let image = raster_composite_image(snapshot)") && raster.includes("raster_document_json_to_svg(snapshot)?") && !raster.includes("snapshot.require_empty_output_shell()"));
  const names = ["raster_generic_initializer_retains_genesis_identity_next_generation_and_second_map_page", "raster_empty_generic_initializer_progresses_under_mounted_sixty_four_fuel", "raster_generic_initializer_zero_fuel_items_and_expired_deadline_preserve_phase_and_owners", "raster_generic_initializer_cancel_stale_and_completed_candidate_retire_all_exact_owners", "raster_mounted_preparation_deep_atomic_inverse_and_unclaimed_result_retirement", "raster_fixed_map_clone_prices_exact_page_births_and_denies_unfunded_insertions", "raster_mounted_partial_clone_and_before_prepared_cancellation_retain_source_and_retire_owners", "raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed", "raster_live_envelope_cancel_closes_retained_pages_without_publication", "a_demo_edit_archive_loads_back_through_the_document_archive_door", "raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant", "raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant", "raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner", "raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses", "raster_maximum_combined_layer_and_value_depth_retires_to_terminal", "raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly", "raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact"];
  require("mounted-laws", laws.includes("mounted-raster-host-laws") && names.every(name => new RegExp("\\b(?:async\\s+)?fn\\s+" + name + "\\s*\\(").test(laws)));

  const mapBody = body(raster, "pub struct RasterOwnedMap<V>");
  require("private-map-backing", !/\bpub(?:\([^)]*\))?\s+(?:pages|order|length)\s*:/.test(mapBody));
  const serializerBodies = Array.from(raster.matchAll(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\)/g), match => compact(toolJobRustBlock(raster, raster.indexOf("{", match.index))?.body ?? ""));
  require("each-mounted-format", serializerBodies.length === 7 && ["encode_png(&target)", "encode_bmp(&target)", "encode_tiff(&target)", "encode_jpg(&target,", "encode_gif(&gif87a::from_89a(&gif89a))", "write_json_pretty(&serialize(snapshot)?.value)", "raster_document_json_to_svg(snapshot)?"].every(call => serializerBodies.some(owner => owner.includes(call))) && serializerBodies.filter(owner => owner.includes("letimage=raster_composite_image(snapshot)")).length === 5 && !serializerBodies.some(owner => owner.includes("Ok(Vec::new())") || owner.includes("print_dsl(")));
  require("exact-replaced-fixture", /assert_eq!\(previous_key\.as_ptr\(\),\s*old_key_pointer\)/.test(laws) && /assert_eq!\(previous_child\.child_id(?:\(\))?\.as_ptr\(\),\s*old_child_pointer\)/.test(laws) && /assert_eq!\(installed_key\.as_ptr\(\),\s*replacement_key_pointer\)/.test(laws) && /assert_eq!\(installed_child\.child_id(?:\(\))?\.as_ptr\(\),\s*replacement_child_pointer\)/.test(laws));
  const capture = laws.indexOf("let plus_one_param_value_pointer = plus_one_param_value.as_ptr();"), rejected = laws.indexOf("let rejected_param = params.insert("), observed = laws.indexOf("assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer"), retired = laws.indexOf("retire_raster_value_cold((rejected_param.key, rejected_param.value))");
  require("rejected-output-value-identity", capture >= 0 && rejected > capture && observed > rejected && retired > observed);
  const childCapture = laws.search(/let plus_one_asset_child_pointer = plus_one_asset_child\.child_id(?:\(\))?\.as_ptr\(\);/), childRejected = laws.indexOf("let rejected_asset = assets.insert("), childObserved = laws.search(/assert_eq!\(rejected_asset\.value\.child_id(?:\(\))?\.as_ptr\(\),\s*plus_one_asset_child_pointer/), childRetired = laws.indexOf("retire_raster_value_cold((rejected_asset.key, rejected_asset.value))");
  require("rejected-output-child-identity", childCapture >= 0 && childRejected > childCapture && childObserved > childRejected && childRetired > childObserved);
  require("exact-physical-fixture", laws.includes("demand.capacity_bytes-1") && laws.includes("output.allocated_page_count(),2") && laws.includes("Arc::try_unwrap(root)") && laws.includes("measured midpoint and final-minus-one cuts remain partial") && laws.includes("serde_json::to_value(&actual)"));
  return failures;
}

export function toolJobRasterEnvelopeCallerRetainedExact(store: string, raster: string, editor: string, plugin: string, editorTestLaws: string): boolean {
  return toolJobRasterEnvelopeCallerFailures(store, raster, editor, plugin, editorTestLaws).length === 0;
}

export function toolJobDrawingEnvelopeCallerRetainedExact(store: string, drawing: string, editor: string, plugin: string, editorTestLaws: string = editor): boolean {
  const secondApplyLane = [
    "struct DrawingStoreInitializationAuthority",
    "struct DrawingMutationCandidateAuthority",
    "struct DrawingContainerRebuildAuthority",
    "struct DrawingDuplicateRewriteAuthority",
    "struct DrawingMutationArenaPool",
    "fn node_at_mut",
    "fn container_mut",
    "fn apply_selection_transform",
    "&mut DrawingSnapshot",
    "snapshot_mut",
    "operation.diff(",
    "diff.apply(",
    "source.assets.remove(",
  ];
  const ownedInitializerHooks = [
    "fn build_document_store_initialization_job",
    "drawing_document_store_initialization_job",
    "request_drawing_mutation_arena_pool",
    "drawing_mutation_arena_pool_fault",
    "arena_boot_fault",
  ];
  const ingress = `${editor}\n${editorTestLaws}`;
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("id_digest: [u8; 32]") &&
    store.includes('Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest)?') &&
    !store.includes("id.clone(), edit_digest") &&
    ingress.includes("pub struct DrawingEnvelopeLoadHandle") &&
    ingress.includes("source: &js_sys::Uint8Array") &&
    ingress.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    ingress.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    ingress.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    ingress.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    ingress.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    ingress.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    ingress.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
    !ingress.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !ingress.includes("envelope_json: Option<String>") &&
    !ingress.includes("while let") &&
    !ingress.includes("loop {") &&
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("id_digest: [u8; 32]") &&
    drawing.includes("pub struct DrawingEnvelopeOwnedFieldCatalog") &&
    drawing.includes("artifact_owned_spr_edit_history_decoder") &&
    drawing.includes("DrawingSnapshotDecodeAuthority") &&
    drawing.includes("DrawingMutationDecodeAuthority") &&
    drawing.includes("OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>") &&
    drawing.includes("const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096") &&
    drawing.includes("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64") &&
    drawing.includes("struct DrawingNativeCloneAuthority") &&
    drawing.includes("struct DrawingLayerCloneWorkAuthority") &&
    drawing.includes("struct DrawingLayerLocator") &&
    secondApplyLane.every((forbidden) => !drawing.includes(forbidden)) &&
    ownedInitializerHooks.every((forbidden) => !editor.includes(forbidden) && !drawing.includes(forbidden)) &&
    editorTestLaws.includes("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed") &&
    editorTestLaws.includes("drawing_live_envelope_cancel_closes_retained_pages_without_publication") &&
    editorTestLaws.includes("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate") &&
    editorTestLaws.includes("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle") &&
    editorTestLaws.includes('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer') &&
    editorTestLaws.includes("drawing_live_envelope_replay_equals_the_central_apply_fold_of_every_forward") &&
    editorTestLaws.includes("protocol::apply_diff_cold(outcome.diff(), &expected)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    plugin.includes("Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))") &&
    !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !editor.includes("envelope_json: Option<String>")
  );
}
