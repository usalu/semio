import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import { Database } from "bun:sqlite";
import { existsSync, readFileSync } from "node:fs";

const engine = new URL("../../", import.meta.url);
const read = (path: string) => readFileSync(new URL(path, engine), "utf8");
const fixture = JSON.parse(read("🧫️fixtures/♻️frame-close/🔣️.json"));

const grantKey = "https://semio.dev/schema/value/retained-clone/grant";
const authoritySchema = () => new Ajv2020({ strict: true }).addMetaSchema(draft7).addSchema(JSON.parse(read("../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")), grantKey);

test("actual frame caller grants require every canonical independent axis", () => {
  const validate = authoritySchema().getSchema(grantKey)!;
  for (const grant of [fixture.caller, ...fixture.cases.map((row: { grant: object }) => row.grant)]) {
    expect(validate(grant)).toBe(true);
    for (const name of Object.keys(grant)) {
      const changed = structuredClone(grant);
      delete changed[name];
      expect(validate(changed)).toBe(false);
      const negative = structuredClone(grant);
      negative[name] = -1;
      expect(validate(negative)).toBe(false);
    }
  }
});

test("actual draft caller grant agrees with the canonical neutral grant", () => {
  const scene = new URL("../../../../../🔨️modules/🖱️ui/🎬️scene/", engine);
  const fixture = JSON.parse(readFileSync(new URL("✂️text-splice/📡️draft-wire/🧫️fixtures/🔣️.json", scene), "utf8"));
  const validate = authoritySchema().getSchema(grantKey)!;
  expect(validate(fixture.callerGrant)).toBe(true);
  for (const name of Object.keys(fixture.callerGrant)) {
    const changed = structuredClone(fixture.callerGrant);
    delete changed[name];
    expect(validate(changed)).toBe(false);
    const negative = structuredClone(fixture.callerGrant);
    negative[name] = -1;
    expect(validate(negative)).toBe(false);
  }
});

test("frame close grants and all receipt currencies agree with the independent SQL oracle", () => {
  const database = new Database(":memory:");
  try {
    const oracle = database.query("SELECT CASE WHEN ?1 > ?5 OR ?2 > ?6 OR ?3 > ?7 OR ?4 > ?8 THEN 'ownershipLimit' WHEN ?9 = 'complete' AND ?10 = 0 THEN 'notTerminalEmpty' ELSE ?9 END AS verdict");
    for (const row of fixture.cases) {
      const g = row.grant, p = row.step.progress;
      const fits = p.copiedItems <= g.maximumItems && p.copiedBytes <= g.maximumCopyBytes && p.retainedCapacityBytes <= g.maximumCapacityBytes && p.releasedBytes <= g.maximumReleaseBytes;
      const verdict = !fits ? "ownershipLimit" : row.step.kind === "complete" && !row.terminal ? "notTerminalEmpty" : row.step.kind;
      expect(verdict).toBe(row.expected);
      expect((oracle.get(p.copiedItems, p.copiedBytes, p.retainedCapacityBytes, p.releasedBytes, g.maximumItems, g.maximumCopyBytes, g.maximumCapacityBytes, g.maximumReleaseBytes, row.step.kind, Number(row.terminal)) as { verdict: string }).verdict).toBe(row.expected);
    }
  } finally {
    database.close();
  }
});

test("actual scalar frame job declares inline ownership and receives a full grant", () => {
  const source = read("🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
  const body = source.slice(source.indexOf("impl InteractiveJob for FrameBuildJob"), source.indexOf("//#endregion 🧩️FrameBuildJob"));
  for (const marker of ["grant: RetainedCloneGrant", "next_close_copy_byte_demand", "next_close_capacity_byte_demand", "next_close_release_byte_demand", "next_close_depth_demand", "copied_bytes", "size_of::<FrameDirectives>()"]) expect(body.includes(marker)).toBe(true);
  expect(body.includes("maximum_items: usize")).toBe(false);
  expect(body.includes("released_items")).toBe(false);
});

test("actual phase and aggregate receivers preserve the full typed close chain", () => {
  const source = read("🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
  const phase = source.slice(source.indexOf("fn retire_active_phase"), source.indexOf("impl ActiveFrameBuild"));
  expect(phase.includes("grant: RetainedCloneGrant")).toBe(true);
  expect(phase.includes("-> InteractiveJobCloseStep")).toBe(true);
  expect(phase.includes("let _ =")).toBe(false);
  expect(source.includes("close_step(1,")).toBe(false);
  const renderer = read("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
  for (const name of ["FrameTransaction", "AppFramePreparation", "AppFramePresentation"]) {
    const start = renderer.indexOf(`impl ${name} {`), next = renderer.indexOf("\nimpl ", start + 1);
    const owner = renderer.slice(start, next === -1 ? undefined : next);
    expect(owner.includes("grant: RetainedCloneGrant")).toBe(true);
    expect(owner.includes("-> InteractiveJobCloseStep")).toBe(true);
  }
});

test("the actual StoreSync receiving boundary forwards the original caller identity authority", () => {
  const source = read("../../🏪️store/🔄️sync/🦀️.rs");
  const body = source.slice(source.indexOf("pub async fn receive(&mut self"), source.indexOf("pub async fn reconcile_branch"));
  expect(body.includes("identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>")).toBe(true);
  expect(body.includes("ArtifactCommand::IngestRemote { envelope }, identity")).toBe(true);
  expect(body.includes("EntityIdentityAuthority::new")).toBe(false);
  const callers = read("../../🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs");
  expect(callers.includes("session.receive(envelope, &mut identity).await")).toBe(true);
  expect(callers.includes("identity.pause().expect(\"original fixture receive identity receipt\")")).toBe(true);
  expect(callers.includes("original receive identity owned={observed} ceiling={CEILING}")).toBe(true);
});

test("the actual draft writer forwards the original full caller grant and receipt", () => {
  const scene = new URL("../../../../../🔨️modules/🖱️ui/🎬️scene/", engine);
  const readScene = (path: string) => readFileSync(new URL(path, scene), "utf8");
  const source = readScene("✂️text-splice/🦀️.rs");
  const owner = source.slice(source.indexOf("impl DraftChangesJsonCursor{"), source.indexOf("impl protocol::value::retirement::RetireOwned for DraftChangesJsonCursor"));
  expect(owner.includes("grant:protocol::value::RetainedCloneGrant")).toBe(true);
  expect(owner.includes("self.writer.step(maximum_units,control,grant)")).toBe(true);
  expect(owner.includes("self.writer.normal_step_progress()")).toBe(true);
  expect(owner.includes("RetainedCloneGrant{")).toBe(false);
});

test("the actual empty engine backing quotes and receipts its physical array release", () => {
  const source = read("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
  const owner = source.slice(source.indexOf("struct FrameEnginePackets {"), source.indexOf("pub(crate) struct AppFrameBuild"));
  for (const marker of ["slots: Option<Box<", "fn terminal_is_empty", "fn backing_retirement_demands", "grant: semio_framework_job::RetainedCloneGrant", "grant.maximum_release_bytes < demand.release_bytes", "drop(self.slots.take())", "released_bytes: demand.release_bytes"]) expect(owner.includes(marker)).toBe(true);
  expect(fixture.enginePacketBacking.slots).toBe(256);
  const library = read("🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/🦀️.rs");
  expect(library.includes("static RENDERER_HEAP_WITNESS: semio_framework_trace::HeapWitness")).toBe(true);
});

test("native and browser bootstrap frame authority is explicit, finite and preserves all five neutral axes",()=>{
 const law=JSON.parse(read("🪙️authority/🖼️frame/🔣️.json")),schema=JSON.parse(read("🪙️authority/🖼️frame/🧬️schema/🔣️.json"));const ajv=authoritySchema(),validate=ajv.compile(schema);expect(validate(law)).toBe(true);for(const axis of Object.keys(law.grant)){const missing=structuredClone(law);delete missing.grant[axis];expect(validate(missing)).toBe(false);const negative=structuredClone(law);negative.grant[axis]=-1;expect(validate(negative)).toBe(false);}
 const db=new Database(":memory:");try{const expected=db.query("SELECT 1 AS maximumItems,65536 AS maximumCopyBytes,65536 AS maximumCapacityBytes,16777216 AS maximumReleaseBytes,64 AS maximumDepth").get();expect(law.grant).toEqual(expected);}finally{db.close();}
 const native=read("🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs"),browser=read("🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"),frame=read("🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs"),renderer=read("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");for(const source of [native,browser])expect(source.includes("OsHost::new(runtime, presenter, crate::frame_authority::FRAME_BOOTSTRAP_ROOT_GRANT)")).toBe(true);expect(frame.includes("pub(crate) fn new(retained: RetainedCloneGrant)")).toBe(true);expect(frame.includes("retained, site: \"os_renderer_frame_build\"")).toBe(true);expect(renderer.replace(/\s/g,"").includes("retained:self.retained,site:\"os_renderer.prepare.worker\"" )).toBe(true);expect(frame.replace(/\s/g,"").includes("receive_frame_outcome(&mutowner,self.retained,&mutself.retained_progress)")).toBe(true);
});

test("original Scene camera storage has strict packed identities and independent SQLite ownership semantics",()=>{
 expect(existsSync(new URL("🧱️elements/🎞️Scenes/⏱️camera/🧬️schema/🔣️.json",engine))).toBe(false);
 const base="🧱️elements/🎞️Scenes/⏱️camera/",law=JSON.parse(read(base+"🧫️fixtures/🔣️.json")),validate=authoritySchema().getSchema(grantKey)!;expect(validate(law.rootGrant)).toBe(true);for(const axis of Object.keys(law.rootGrant)){const missing=structuredClone(law.rootGrant);delete missing[axis];expect(validate(missing)).toBe(false);}
 const db=new Database(":memory:");try{const sql=db.query("SELECT lower(hex(CAST(?1||?2||?3||?4||?5||?6 AS BLOB))) AS hex,length(CAST(?1||?2||?3||?4||?5||?6 AS BLOB)) AS bytes");for(const row of law.cases){const o=row.owner,texts=[row.host,row.surface,o?.host??"",o?.window??"",o?.surface??"",o?.key.explicit??""];expect(sql.get(...texts)).toEqual({hex:row.packed.hex,bytes:row.packed.bytes});let offset=0;expect(texts.map(value=>{const bytes=Buffer.byteLength(value);const range=[offset,bytes];offset+=bytes;return range;})).toEqual(row.packed.ranges);}db.run("CREATE TABLE deadline(host TEXT PRIMARY KEY,at REAL,owner TEXT)");db.run("INSERT INTO deadline VALUES ('host',120,'old')");db.run("INSERT INTO deadline VALUES ('host',240,'new') ON CONFLICT(host) DO UPDATE SET at=excluded.at,owner=excluded.owner");expect(db.query("SELECT at,owner FROM deadline WHERE host='host'").get()).toEqual({at:240,owner:"new"});}finally{db.close();}
 expect(law.maximumEntries).toBe(256);expect(law.maximumIdentifierBytes).toBe(256);expect(existsSync(new URL(base+"🦀️.rs",engine))).toBe(true);const source=read("🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs");expect(source.includes("hash_map::IntoIter<String, SceneCameraDeadline>")).toBe(false);expect(source.includes("camera_storage::DirectoryOwner")).toBe(true);const owner=read(base+"🦀️.rs");for(const marker of ["RetainedCloneGrant","RetainedCloneProgress","size_of","Layout","original_body_ptr","maximum_release_bytes","maximum_capacity_bytes"])expect(owner.includes(marker)).toBe(true);
});


test("camera exact capacity and identifier refusals agree with independent SQLite state transitions", () => {
  const base = "🧱️elements/🎞️Scenes/⏱️camera/";
  const corpus = JSON.parse(read(base + "🧫️fixtures/🔣️.json"));
  const law = corpus.boundaries;
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE deadline(host TEXT PRIMARY KEY,at REAL,metadata INTEGER)");
    db.run("WITH RECURSIVE ids(n) AS (SELECT 0 UNION ALL SELECT n+1 FROM ids WHERE n+1<?1) INSERT INTO deadline SELECT ?2||n,?3,?4 FROM ids", [law.identities.count, law.identities.prefix, law.identities.deadlineMs, law.identities.metadata]);
    db.run("INSERT INTO deadline SELECT ?1,120,7 WHERE (SELECT count(*) FROM deadline)<?2", [law.overflow.host, corpus.maximumEntries]);
    expect(db.query("SELECT count(*) AS retainedEntries FROM deadline").get()).toEqual({ retainedEntries: law.overflow.retainedEntries });
    expect(db.query("SELECT count(*) AS overflowEntries FROM deadline WHERE host=?1").get(law.overflow.host)).toEqual({ overflowEntries: 0 });
    db.run("CREATE TABLE cursor AS SELECT * FROM deadline");
    db.run("DELETE FROM deadline");
    const counts = () => db.query("SELECT (SELECT count(*) FROM deadline) AS globalEntries,(SELECT count(*) FROM cursor) AS cursorEntries").get();
    expect(counts()).toEqual(law.checkout);
    db.run("INSERT INTO deadline SELECT * FROM cursor");
    db.run("DELETE FROM cursor");
    expect(counts()).toEqual({ globalEntries: law.restore.globalEntries, cursorEntries: law.restore.cursorEntries });
    db.run("INSERT INTO cursor SELECT 'original-'||host,at,metadata FROM deadline");
    db.run("INSERT INTO deadline SELECT * FROM cursor LIMIT max(0,?1-(SELECT count(*) FROM deadline))", [corpus.maximumEntries]);
    expect(counts()).toEqual({ globalEntries: law.occupiedRestore.globalEntries, cursorEntries: law.occupiedRestore.cursorEntries });
    expect(db.query("SELECT length(CAST(?1 AS BLOB)) AS attemptedBytes").get("x".repeat(law.identifier.attemptedBytes))).toEqual({ attemptedBytes: law.identifier.attemptedBytes });
    expect(law.identifier.attemptedBytes).toBe(corpus.maximumIdentifierBytes + 1);
  } finally { db.close(); }
  const native = read(base + "🧪️tests/🦀️.rs");
  expect(native.includes("fn camera_exact_capacity_restores_originals_and_retains_a_full_cursor_on_refusal()")).toBe(true);
  expect(native.includes("fn camera_identifier_maximum_plus_one_is_inert_on_a_vacant_original_owner()")).toBe(true);
});

test("actual atlas original-owner contract prices fixed backing with independent full authorities", () => {
  const base = "../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/";
  expect(existsSync(new URL(base+"🖼️atlas/🧬️schema/🔣️.json",engine))).toBe(false);
  const corpus = JSON.parse(read(base + "🖼️atlas/🧫️fixtures/🔣️.json"));
  const validate = authoritySchema().getSchema(grantKey)!;
  for (const mode of ["normal", "retirement"]) {
    const grant = corpus.authority[mode];
    expect(validate(grant)).toBe(true);
    for (const axis of Object.keys(grant)) {
      const missing = structuredClone(grant);
      delete missing[axis];
      expect(validate(missing)).toBe(false);
    }
  }
  const db = new Database(":memory:");
  try {
    const sql = db.query("WITH RECURSIVE page(startRow) AS (SELECT 0 UNION ALL SELECT startRow+?1/(?2*?3) FROM page WHERE startRow+?1/(?2*?3)<?4) SELECT startRow,min(?1/(?2*?3),?4-startRow) AS rows,min(?1/(?2*?3),?4-startRow)*?2*?3 AS payloadBytes,?1 AS allocationBytes FROM page");
    for (const row of corpus.cases) {
      expect(sql.all(corpus.policy.pageBytes, row.width, row.channels, row.height)).toEqual(row.pages);
      expect(db.query("SELECT ?1*?2*?3 AS sourceBytes").get(row.width, row.height, row.channels)).toEqual({ sourceBytes: row.sourceBytes });
      expect(db.query("SELECT sum(json_extract(value,'$.allocationBytes')) AS bytes FROM json_each(?1)").get(JSON.stringify(row.pages))).toEqual({ bytes: row.pages.length * corpus.policy.pageBytes });
    }
  } finally { db.close(); }
  const native = read(base + "🦀️.rs");
  expect(native.includes("pub struct PreparedAtlasAuthority")).toBe(true);
  expect(native.includes("fn original_owner_progress(")).toBe(true);
  expect(native.includes("fn close_original_step(")).toBe(true);
});

test("the actual typed ViewContext dictionaries preserve canonical UTF8 field order and the original wire oracle",()=>{
 expect(existsSync(new URL("../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔢️integer-carriers.json",engine))).toBe(false);
 const manifest="../../../../../🔨️modules/🛂️manifest/",law=JSON.parse(read(manifest+"🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json")),viewSchema=JSON.parse(read(manifest+"🪟️view-context/🧬️schema/🔣️.json")),ajv=authoritySchema(),validate=ajv.compile(viewSchema);expect(validate(law.viewContext)).toBe(true);
 const db=new Database(":memory:");try{for(const [name,fields]of Object.entries(law.canonicalDictionaryOrder)){const ordered=db.query("SELECT value FROM json_each(?1) ORDER BY CAST(value AS BLOB)").all(JSON.stringify(fields)).map((row)=>row.value);expect(fields).toEqual(ordered);const source=read(name==="ViewModel"?manifest+"🦀️.rs":"../../../../../🔨️modules/⏯️tool-run/🦀️.rs"),start=source.indexOf(`pub struct ${name} {`),body=source.slice(start,source.indexOf("\n}",start));const actual=[...body.matchAll(/pub ([a-z_]+):/g)].map(match=>match[1].replace(/_([a-z])/g,(_,letter)=>letter.toUpperCase()));expect(actual).toEqual(fields);}}finally{db.close();}
 expect(law.packHex.length).toBe(384);const bridge=read("🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs");expect(bridge.includes("ToValue::to_value(view_state)")).toBe(true);
});


test("borrowed key presentation releases Scene storage before entering UI and matches independent keys", () => {
  const base = new URL("../../../../../🔨️modules/🖱️ui/", engine);
  const corpus = JSON.parse(readFileSync(new URL("🔑️node-key/🧫️fixtures/🔣️.json", base), "utf8"));
  expect(existsSync(new URL("🔑️node-key/🧬️schema/🔣️.json",base))).toBe(false);
  const database = new Database(":memory:");
  try {
    const sql = database.query("SELECT CASE WHEN json_type(?1,'$.explicit') IS NOT NULL AND json_type(?2,'$.explicit') IS NOT NULL THEN CAST(json_extract(?1,'$.explicit') AS BLOB) = CAST(json_extract(?2,'$.explicit') AS BLOB) WHEN json_type(?1,'$.positional') IS NOT NULL AND json_type(?2,'$.positional') IS NOT NULL THEN json_extract(?1,'$.positional[0]') = json_extract(?2,'$.positional[0]') AND json_extract(?1,'$.positional[1]') = json_extract(?2,'$.positional[1]') ELSE 0 END AS matched");
    for (const row of corpus.cases) expect(Boolean((sql.get(JSON.stringify(row.owned), JSON.stringify(row.borrowed)) as { matched: number }).matched)).toBe(row.matches);
  } finally { database.close(); }
  const tree = readFileSync(new URL("🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs", base), "utf8");
  expect(tree.includes("pub enum NodeKeyRef<'a>")).toBe(true);
  const reconcile = readFileSync(new URL("🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs", base), "utf8");
  expect(reconcile.includes("pub key: NodeKeyRef<'a>")).toBe(true);
  const scene = read("🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs");
  const body = scene.slice(scene.indexOf("fn camera_owns_surface("), scene.indexOf("fn remove_scene_camera_deadline("));
  const unlocked = body.indexOf("    });");
  expect(unlocked).toBeGreaterThan(0);
  expect(body.slice(0, unlocked).includes("component_scene_is_presented")).toBe(false);
  expect(body.slice(unlocked).includes("component_scene_is_presented(&witness)")).toBe(true);
  expect(body.includes(".clone()")).toBe(false);
});
