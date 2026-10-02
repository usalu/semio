import { expect, test } from "bun:test";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, join, parse, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";

type ChildIdentity = Readonly<{ id: string; artifact: string; kind: string; standard: string; subset: string }>;
type ChildNode = Readonly<{ tag: "child"; identity: ChildIdentity }> | Readonly<{ tag: "none" }> | Readonly<{ tag: "some"; value: ChildNode }> | Readonly<{ tag: "list"; values: readonly ChildNode[] }>;
type Projection = Readonly<{ accepted: boolean; steps: number; rows: readonly (readonly string[])[] }>;
type CompositionCase = Readonly<{ id: string; fields: readonly Readonly<{ slot: string; value: ChildNode }>[]; maximumSteps: number; expected: Projection }>;
type FacetLeaves = Readonly<Record<"rust" | "typescript" | "graphql" | "json_schema" | "proto", string>>;
type Descriptor = Readonly<{ id: string }> & Readonly<Record<string, string | FacetLeaves>>;
type Registration = Readonly<{ accepted: boolean; entries: readonly Descriptor[]; exports: readonly Descriptor[]; error: "descriptor-conflict" | "export-conflict" | null }>;
type CatalogCase = Readonly<{ id: string; family: "artifact" | "inference" | "app"; initial: readonly Descriptor[]; mode: "single" | "batch"; descriptors: readonly Descriptor[]; exports: readonly Descriptor[]; expected: Registration }>;
type Schedule = Readonly<{ id: string; family: "inference"; initial: readonly Descriptor[]; actors: readonly Readonly<{ id: string; descriptors: readonly Descriptor[] }>[]; order: readonly string[]; expected: Readonly<{ decisions: readonly Readonly<{ actor: string; accepted: boolean }>[]; entries: readonly Descriptor[] }> }>;
type Corpus = Readonly<{
  contract: "schema-neutral-deletion-v1";
  ownership: Readonly<Record<"schemaManifest" | "schemaRoot" | "schemaComponent" | "compositionSource" | "compositionManifest" | "stateSource" | "stateManifest" | "registrySource" | "registryManifest" | "protocolSource" | "protocolManifest" | "kernelRoot" | "componentUnit" | "higherLawSource" | "deriveSource", string>>;
  providers: Readonly<Record<"composition" | "state" | "registry" | "forbidden", string>>;
  compositionTypes: readonly string[];
  descriptorFields: readonly Readonly<{ name: string; fields: readonly string[] }>[];
  versionFunctions: readonly Readonly<{ name: string; descriptor: "ArtifactSchemaDescriptor" | "AppSchemaDescriptor"; facet: string }>[];
  states: readonly Readonly<{ variant: string; wire: string; kebab: string; graphql: string }>[];
  retiredStates: readonly string[];
  laws: readonly string[];
  higherLaw: string;
  originalUnitSha256: string;
  composition: readonly CompositionCase[];
  catalogs: readonly CatalogCase[];
  catalogContract: Readonly<{ single: "replace"; batch: "atomic-strict-batch"; artifactMirror: "refuse-conflicting-export-before-write"; concurrency: "atomic-owner-lock"; nativeProof: "pending"; callback: "snapshot-before-callback-unlocked"; descriptorOwnedMirrorFollowsSingleReplacement: true; externallyPublishedMirrorIsProtected: true; equalExternalPublicationClaimsAuthority: true }>;
  interleaving: Readonly<{ id: string; initial: readonly Descriptor[]; first: readonly Descriptor[]; second: readonly Descriptor[]; expectedAfterSeparatePreflights: readonly Descriptor[]; proof: string }>;
  concurrentSchedules: readonly Schedule[];
}>;

const root = resolve(import.meta.dir, "../../../../..");
const owner = resolve(import.meta.dir, "../..");
const corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧱️neutrality/🔣️.json"), "utf8")) as Corpus;
const schema: unknown = JSON.parse(readFileSync(join(owner, "🧬️schema/🧱️neutrality/🔣️.json"), "utf8"));
const validate = new Ajv({ strict: true }).compile<Corpus>(schema as object);
const read = (relative: string): string => readFileSync(join(root, relative), "utf8");
const source = (relative: string): string => existsSync(join(root, relative)) ? read(relative) : "";

/** 🧒️ Portable contract projection; native source execution remains a separate required receipt. */
function project(row: CompositionCase): Projection {
  let steps = 0;
  const rows: string[][] = [];
  const visit = (slot: string, value: ChildNode): boolean => {
    if (steps === row.maximumSteps) return false;
    steps++;
    if (value.tag === "child") {
      const child = value.identity;
      rows.push([slot, child.id, child.artifact, child.kind, child.standard, child.subset]);
      return true;
    }
    if (value.tag === "none") return true;
    if (value.tag === "some") return visit(slot, value.value);
    return value.values.every(child => visit(slot, child));
  };
  return { accepted: row.fields.every(field => visit(field.slot, field.value)), steps, rows };
}

/** 📇️ Sequential portable registration contract; it makes no concurrency guarantee. */
function register(row: CatalogCase): Registration {
  const entries = [...row.initial];
  const exports = row.family === "artifact" ? [...row.initial] : [];
  const external = new Set(row.exports.map(item => item.id));
  for (const declaration of row.exports) { const index = exports.findIndex(item => item.id === declaration.id); if (index < 0) exports.push(declaration); else exports[index] = declaration; }
  const result = (error: Registration["error"]): Registration => ({ accepted: error === null, entries: entries.toSorted((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0), exports: exports.toSorted((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0), error });
  if (row.mode === "batch") {
    const proposed: Descriptor[] = [];
    for (const candidate of row.descriptors) {
      const existing = proposed.find(item => item.id === candidate.id) ?? entries.find(item => item.id === candidate.id);
      if (existing && JSON.stringify(existing) !== JSON.stringify(candidate)) return result("descriptor-conflict");
      proposed.push(candidate);
    }
  }
  if (row.family === "artifact") {
    for (const candidate of row.descriptors) {
      const existing = exports.find(item => item.id === candidate.id);
      if (existing && external.has(candidate.id) && JSON.stringify(existing) !== JSON.stringify(candidate)) return result("export-conflict");
    }
  }
  for (const candidate of row.descriptors) {
    const index = entries.findIndex(item => item.id === candidate.id);
    if (index < 0) entries.push(candidate);
    else if (row.mode === "single") entries[index] = candidate;
    if (row.family === "artifact") { const index = exports.findIndex(item => item.id === candidate.id); if (index < 0) exports.push(candidate); else exports[index] = candidate; }
  }
  return result(null);
}

/** 🔬️ Observes simple explicit public fields; it does not infer aliases or native execution. */
function publicFields(bytes: string, name: string): readonly string[] {
  const body = bytes.match(new RegExp("pub struct " + name + "\\s*\\{([^}]+)\\}"))?.[1];
  return body ? [...body.matchAll(/pub\s+(\w+)\s*:/g)].map(match => match[1]!) : [];
}

/** 🧪️ Independent Node/AJV oracle uses iterative child events and Map transactions. */
function nodeOracle(): Readonly<{ admitted: boolean; hostileAdmission: boolean; states: readonly unknown[]; composition: readonly unknown[]; catalogs: readonly unknown[]; interleaving: readonly Descriptor[]; concurrentSchedules: readonly unknown[] }> {
  const child = spawnSync("node", ["--eval", `const fs=require("node:fs"),Ajv=require("ajv"),equal=require("fast-deep-equal");
const {corpus,schema}=JSON.parse(fs.readFileSync(0,"utf8")),admit=new Ajv({strict:true}).compile(schema);
const composition=corpus.composition.map(row=>{let steps=0,accepted=true;const rows=[],pending=row.fields.toReversed().map(field=>[field.slot,field.value]);while(pending.length){const [slot,node]=pending.pop();if(steps>=row.maximumSteps){accepted=false;break;}steps++;if(node.tag==="child")rows.push([slot,...["id","artifact","kind","standard","subset"].map(key=>node.identity[key])]);else if(node.tag==="some")pending.push([slot,node.value]);else if(node.tag==="list")for(const value of node.values.toReversed())pending.push([slot,value]);}return{id:row.id,accepted,steps,rows};});
const catalogs=corpus.catalogs.map(row=>{const original=new Map(row.initial.map(item=>[item.id,item])),mirrors=new Map((row.family==="artifact"?row.initial:[]).map(item=>[item.id,item])),external=new Set(row.exports.map(item=>item.id));for(const item of row.exports)mirrors.set(item.id,item);const candidate=new Map(original),mirrorCandidate=new Map(mirrors);let error=null;for(const item of row.descriptors){if(row.mode==="batch"&&candidate.has(item.id)&&!equal(candidate.get(item.id),item)){error="descriptor-conflict";break;}candidate.set(item.id,item);}if(!error&&row.family==="artifact")for(const item of row.descriptors){if(external.has(item.id)&&mirrors.has(item.id)&&!equal(mirrors.get(item.id),item)){error="export-conflict";break;}mirrorCandidate.set(item.id,item);}const snapshot=map=>[...map].sort(([a],[b])=>a<b?-1:a>b?1:0).map(([,descriptor])=>descriptor);return{id:row.id,accepted:!error,entries:snapshot(error?original:candidate),exports:snapshot(error?mirrors:mirrorCandidate),error};});
const interleaved=new Map(corpus.interleaving.initial.map(item=>[item.id,item]));for(const transaction of [corpus.interleaving.first,corpus.interleaving.second])for(const entry of transaction)interleaved.set(entry.id,entry);
const concurrentSchedules=corpus.concurrentSchedules.map(row=>{let owner=new Map(row.initial.map(item=>[item.id,item]));const decisions=[];for(const actorId of row.order){const actor=row.actors.find(item=>item.id===actorId),staged=new Map(owner);let accepted=true;for(const descriptor of actor.descriptors){if(staged.has(descriptor.id)&&!equal(staged.get(descriptor.id),descriptor)){accepted=false;break;}staged.set(descriptor.id,descriptor);}if(accepted)owner=staged;decisions.push({actor:actorId,accepted});}return{id:row.id,decisions,entries:[...owner.values()].sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0)};});
process.stdout.write(JSON.stringify({admitted:admit(corpus),hostileAdmission:admit({...corpus,catalogs:corpus.catalogs.map(row=>row.id==="artifact-mirror-export-conflict"?{...row,expected:{...row.expected,accepted:true,error:null}}:row)}),states:corpus.states.map(row=>({variant:row.variant,wire:JSON.parse(JSON.stringify(row.variant)),kebab:row.variant.toLowerCase(),graphql:row.variant.toUpperCase()})),composition,catalogs,interleaving:[...interleaved.values()].sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0),concurrentSchedules}));`], { input: JSON.stringify({ corpus, schema }), encoding: "utf8", timeout: 4000 });
  expect(child.status, child.stderr || String(child.error)).toBe(0);
  const result = JSON.parse(child.stdout ?? "");
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  const allowed = resolve(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated");
  const destination = output ? resolve(output) : "";
  const member = relative(allowed, destination);
  if (!output || !member || isAbsolute(member) || member === ".." || member.startsWith("..\\") || member.startsWith("../")) throw Error("Caller-owned ticket output required");
  let directory = parse(destination).root;
  for (const part of destination.slice(directory.length).split(/[\\/]/).filter(Boolean)) {
    directory = join(directory, part);
    if (!existsSync(directory)) mkdirSync(directory);
    const observation = lstatSync(directory);
    if (!observation.isDirectory() || observation.isSymbolicLink()) throw Error("Unsafe output directory: " + directory);
  }
  writeFileSync(join(destination, "schema-neutrality-node-oracle.json"), child.stdout ?? "");
  return result;
}

test("closed schema neutrality corpus binds every original law and hostile expected output", () => {
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, extra: true })).toBe(false);
  expect(validate({ ...corpus, laws: corpus.laws.slice(1) })).toBe(false);
  expect(validate({ ...corpus, catalogs: corpus.catalogs.map(row => row.id === "artifact-mirror-export-conflict" ? { ...row, expected: { ...row.expected, accepted: true } } : row) })).toBe(false);
  expect(new Set(corpus.laws).size).toBe(30);
  expect(new Set(corpus.composition.map(row => row.id)).size).toBe(8);
  expect(new Set(corpus.catalogs.map(row => row.id)).size).toBe(19);
  expect(validate({ ...corpus, concurrentSchedules: corpus.concurrentSchedules.map(row => ({ ...row, expected: { ...row.expected, decisions: row.expected.decisions.map(decision => ({ ...decision, accepted: true })) } })) })).toBe(false);
});

test("portable composition state and three catalogs match independent Node and AJV", () => {
  const oracle = nodeOracle();
  expect(oracle.admitted).toBe(true);
  expect(oracle.hostileAdmission).toBe(false);
  expect(oracle.states).toEqual(corpus.states);
  expect(oracle.composition).toHaveLength(8);
  expect(oracle.catalogs).toHaveLength(19);
  for (const [index, row] of corpus.composition.entries()) {
    expect(project(row), row.id).toEqual(row.expected);
    expect(oracle.composition[index], row.id).toEqual({ id: row.id, ...row.expected });
  }
  for (const [index, row] of corpus.catalogs.entries()) {
    expect(register(row), row.id).toEqual(row.expected);
    expect(oracle.catalogs[index], row.id).toEqual({ id: row.id, ...row.expected });
  }
  expect(oracle.interleaving).toEqual(corpus.interleaving.expectedAfterSeparatePreflights);
  for (const [index, row] of corpus.concurrentSchedules.entries()) {
    let entries = row.initial;
    const decisions: { actor: string; accepted: boolean }[] = [];
    for (const actorId of row.order) {
      const actor = row.actors.find(candidate => candidate.id === actorId)!;
      const result = register({ id: row.id, family: row.family, initial: entries, mode: "batch", descriptors: actor.descriptors, exports: [], expected: { accepted: true, entries: [], exports: [], error: null } });
      entries = result.entries;
      decisions.push({ actor: actorId, accepted: result.accepted });
    }
    expect({ decisions, entries }, row.id).toEqual(row.expected);
    expect(oracle.concurrentSchedules[index], row.id).toEqual({ id: row.id, ...row.expected });
  }
  expect(corpus.catalogContract.nativeProof).toBe("pending");
  expect(corpus.catalogContract.descriptorOwnedMirrorFollowsSingleReplacement).toBe(true);
  expect(corpus.catalogContract.externallyPublishedMirrorIsProtected).toBe(true);
  expect(corpus.catalogContract.equalExternalPublicationClaimsAuthority).toBe(true);
  console.log("[DEBUG] schema neutrality: 8 child projections, 4 states, 19 typed public-lifecycle transactions, two atomic schedule contracts and separate-lock defect model match independent Node/AJV; native transaction proof pending");
});

test("actual composition source charges original Option and Vec traversal before children", () => {
  const bytes = read(corpus.ownership.compositionSource);
  expect(bytes).toMatch(/impl<T: ChildFieldRefs> ChildFieldRefs for Option<T>[\s\S]*?visitor\.step\(\)\?;[\s\S]*?Some\(value\) => value\.visit_child_field/);
  expect(bytes).toMatch(/impl<T: ChildFieldRefs> ChildFieldRefs for Vec<T>[\s\S]*?visitor\.step\(\)\?;[\s\S]*?for value in self[\s\S]*?value\.visit_child_field/);
  const unit = read(corpus.ownership.componentUnit);
  expect(unit).toContain("assert_eq!(visitor.steps, 7);");
  expect(unit).toContain("assert_eq!(visitor.steps, 4);");
  expect(unit).toContain('assert_eq!(visitor.rows, [("optionalChild", "child")]);');
});

test("actual state source retains all four wire and kebab spellings without retired lanes", () => {
  const definitions = [corpus.ownership.stateSource, corpus.ownership.protocolSource].map(path => ({ path, bytes: source(path) })).filter(row => row.bytes.includes("pub enum StateClass"));
  expect(definitions).toHaveLength(1);
  const bytes = definitions[0]!.bytes;
  const component = read(corpus.ownership.stateSource);
  for (const row of corpus.states) {
    expect(bytes).toContain('"' + row.wire + '" => Ok(StateClass::' + row.variant + ')');
    expect(component).toContain('"' + row.kebab + '" => Some(StateClass::' + row.variant + ')');
  }
  for (const word of corpus.retiredStates) expect(component).not.toContain('"' + word + '" => Some(StateClass::');
});

test("real main schema manifest and source have no product provider", () => {
  const manifest = TOML.parse(read(corpus.ownership.schemaManifest));
  expect(Object.keys(manifest.dependencies ?? {})).not.toContain(corpus.providers.forbidden);
  expect(read(corpus.ownership.schemaComponent)).not.toContain("semio_framework_os_kernel");
});

test("composition has one lower compile provider and no surviving OS mount", () => {
  expect(existsSync(join(root, corpus.ownership.compositionManifest)), "missing canonical composition package").toBe(true);
  const manifest = TOML.parse(read(corpus.ownership.compositionManifest));
  expect(manifest.package).toHaveProperty("name", corpus.providers.composition);
  expect(Object.keys(manifest.dependencies ?? {})).toEqual([]);
  expect(read(corpus.ownership.kernelRoot)).not.toContain("os_schema_composition");
  expect(read(corpus.ownership.kernelRoot)).not.toMatch(/pub\s+use\s+semio_framework_schema_(?:state|registry|composition)\b/);
  const derive = read(corpus.ownership.deriveSource);
  for (const name of corpus.compositionTypes.filter(name => name !== "ChildRefFields")) expect(derive).toContain("::" + corpus.providers.composition.replaceAll("-", "_") + "::" + name);
  expect(derive).toContain("::" + corpus.providers.state.replaceAll("-", "_") + "::StateClass");
  expect(derive).not.toMatch(/::semio_framework_schema::(?:StateClass|ArtifactCompositionFields|ChildFieldRefs|ChildRefVisitor|ChildSlotSpec|LinkSlotSpec)\b/);
  for (const name of corpus.compositionTypes) expect(read(corpus.ownership.compositionSource)).toMatch(new RegExp("pub (?:struct|trait) " + name + "\\b"));
});

test("StateClass has one Value-backed lower owner rather than a protocol or product declaration", () => {
  expect(existsSync(join(root, corpus.ownership.stateManifest)), "missing canonical state package").toBe(true);
  const manifest = TOML.parse(read(corpus.ownership.stateManifest));
  expect(manifest.package).toHaveProperty("name", corpus.providers.state);
  expect(Object.keys(manifest.dependencies ?? {})).toEqual(["semio-framework-value"]);
  expect(read(corpus.ownership.stateSource)).toContain("pub enum StateClass");
  expect(read(corpus.ownership.protocolSource)).not.toContain("pub enum StateClass");
});

for (const descriptor of corpus.descriptorFields) test("single lower descriptor identity owns " + descriptor.name, () => {
  expect(publicFields(read(corpus.ownership.registrySource), descriptor.name)).toEqual(descriptor.fields);
  if (descriptor.name !== "FacetLeaves") expect(read(corpus.ownership.schemaComponent)).not.toMatch(new RegExp("pub struct " + descriptor.name + "\\b"));
  expect(read(corpus.ownership.protocolSource)).not.toMatch(new RegExp("pub struct Kernel" + descriptor.name + "\\b"));
});

test("three catalog registrations use canonical registry identity without kernel aliases or cycle", () => {
  expect(read(corpus.ownership.schemaComponent)).not.toMatch(/pub\s+use\s+semio_framework_schema_(?:registry|state|composition)\b/);
  expect(read(corpus.ownership.schemaRoot)).not.toMatch(/pub\s+use\s+(?:state|semio_framework_schema_(?:registry|state|composition))\b/);
  expect(read(corpus.ownership.protocolSource)).not.toMatch(/\bKernel(?:FacetLeaves|ArtifactSchemaDescriptor|ArtifactInferenceDescriptor|AppSchemaDescriptor)\b/);
  expect(read(corpus.ownership.schemaComponent)).not.toMatch(/\b(?:facet_leaves|descriptor|inference_descriptor|app_descriptor)_(?:to|from)_kernel\b/);
  const protocol = TOML.parse(read(corpus.ownership.protocolManifest));
  expect(Object.keys(protocol.dependencies ?? {})).toContain(corpus.providers.registry);
  expect(Object.keys(protocol.dependencies ?? {})).not.toContain("semio-framework-schema");
  const registry = TOML.parse(read(corpus.ownership.registryManifest));
  expect(Object.keys(registry.dependencies ?? {})).toEqual([]);
  for (const manifest of [corpus.ownership.registryManifest, corpus.ownership.stateManifest, corpus.ownership.compositionManifest]) {
    const directory = manifest.slice(0, -"Cargo.toml".length);
    const entry = JSON.parse(read(directory + "package.json"));
    const project = JSON.parse(read(directory + "📋️project.json"));
    expect(entry.nx.includedScripts).toEqual([]);
    expect(project.name).toBe(entry.name);
    expect(project.targets.test.executor).toBe("nx:run-commands");
    expect(project.targets.test.options.command).toBe("bun ./📜️script.ts test");
    expect(entry.scripts.test.split(/\s+/).filter((part: string) => part !== "bun" && part !== "--skip-nx-cache")).toEqual(["nx", "run", project.name + ":test"]);
  }
});

test("artifact mirror conflicts are exposed before writes rather than discarded export results", () => {
  expect(read(corpus.ownership.schemaComponent)).not.toMatch(/let\s+_\s*=\s*register_scope_facet_leaves/);
  expect(corpus.catalogs.filter(row => row.expected.error === "export-conflict")).toHaveLength(3);
});

test("schema versions are owned free functions over the one lower descriptor identity", () => {
  const component = read(corpus.ownership.schemaComponent);
  for (const version of corpus.versionFunctions) expect(component).toMatch(new RegExp("pub fn " + version.name + "\\s*\\(descriptor\\s*:\\s*&" + version.descriptor));
});

test("all thirty original laws retain exact lower and real-child higher ownership", () => {
  const unit = read(corpus.ownership.componentUnit);
  const names = [...unit.matchAll(/^(?:async\s+)?fn\s+(\w+)\s*\(/gm)].map(match => match[1]!).filter(name => corpus.laws.includes(name));
  expect(names.toSorted()).toEqual(corpus.laws.filter(name => name !== corpus.higherLaw).toSorted());
  const higher = source(corpus.ownership.higherLawSource);
  expect(higher).toContain("fn " + corpus.higherLaw + "(");
  expect(higher).toContain("(0..64)");
  expect(higher).toContain("ReferenceLimit");
  expect(higher).toContain("(0..257)");
  expect(higher).toContain("TraversalLimit");
  expect(higher).toContain('"ä".repeat(128)');
  expect(higher).toContain("InvalidReference");
});
