/** 📒️ Stdio artifact ledger: definition catalog, support ledger, structural assertions.
 * @see 📜️script.ts
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

type StdioDialectDefinition = Readonly<{
  id: string;
  artifact: string;
  standard: string;
  subset: string;
  schema: string;
  io: string;
  mutations: string | null;
  inferences: string | null;
}>;

type StdioCodecDefinition = Readonly<{
  id: string;
  status: string;
  from: string;
  to: string;
  executable_registration: boolean;
}>;

type StdioRuntimeCapability = Readonly<{
  id: string;
  category: "schema" | "inference" | "codec" | "representation" | "grammar" | "composer" | "subset-validator";
  descriptor: string;
  claims: readonly Readonly<{ namespace: "schema" | "codec" | "extension" | "mime" | "dialect" | "grammar"; value: string }>[];
}>;

type StdioExecutableLeaf = Readonly<{ id: string; status: "unimplemented" | "implemented" | "verified"; executable_registration: boolean }>;

type StdioArtifactDefinition = Readonly<{
  id: string;
  kind: string;
  directory: string;
  depends: readonly string[];
  component: string;
  dialects: readonly StdioDialectDefinition[];
  representations: StdioSchemaDefinition["representations"];
  codecs: readonly StdioCodecDefinition[];
  support: StdioSupportLedger;
}>;

type StdioSupportState = "unimplemented" | "opaque" | "implemented";

type StdioSupportLedger = Readonly<{
  normative_source: string | null;
  publication_date: string | null;
  source_checksum: string | null;
  redistribution_status: string;
  clauses_or_features: readonly string[];
  profiles: readonly string[];
  registered_code_points: readonly string[];
  read: StdioSupportState;
  write: StdioSupportState;
  lossless: StdioSupportState;
  canonical: StdioSupportState;
  validators: readonly string[];
  mutations: readonly string[];
  inferences: readonly string[];
  fixtures: readonly string[];
}>;

type StdioSchemaDefinition = Readonly<{
  id: string;
  artifact: string;
  directory: string;
  dependencies: readonly string[];
  standards: readonly Readonly<{ id: string; revision: string; status: string }>[];
  profiles: readonly Readonly<{ id: string; standard: string; profile: string; status: string }>[];
  source_dialects: readonly Readonly<{ id: string; standard: string; dialect: string; registered_code_points: readonly string[]; status: string }>[];
  representations: readonly Readonly<{ id: string; standard: string; representation: string; mimes: readonly string[]; extensions: readonly string[]; is_binary: boolean; aliases: readonly string[]; neutral: boolean; status: string }>[];
  codecs: readonly StdioCodecDefinition[];
  mutations: readonly StdioExecutableLeaf[];
  inferences: readonly StdioExecutableLeaf[];
  resources: readonly Readonly<{ id: string; status: string }>[];
  localized_descriptors: readonly Readonly<{ id: string; locale: "en" | "de"; name: string; description: string; status: string }>[];
  conformance_suites: readonly Readonly<{ id: string; status: string; fixtures: readonly string[] }>[];
  runtime_capabilities: readonly StdioRuntimeCapability[];
  support_ledger: StdioSupportLedger;
}>;

type StdioArtifactLedger = Readonly<{
  artifacts: readonly StdioArtifactDefinition[];
  counts: Readonly<{ artifacts: number; registeredMimes: number; standards: number; profiles: number; dialects: number; representations: number; codecs: number; mutations: number; inferences: number; conformanceSuites: number; runtimeCapabilities: number; capabilities: Readonly<{ declared: number; registered: number; implemented: number; verified: number }> }>;
}>;

const STDIO_ROOT_REL = join("✏️s", "🔌️plugins", "🗄️stdio");
const STDIO_ARTIFACTS_DIR = "🗿️artifacts";
const STDIO_CATALOG_REL = join(STDIO_ROOT_REL, "📇️registry", "🔣️.json");
const STDIO_COMPONENT_TS = "🟦️.ts";
const STDIO_COMPONENT_RS = "🦀️.rs";

/** 🗑️ Untracked TypeScript build output (`.gitignore` `**​/📦️packages/*​/dist/`), never a stdio source. */
const STDIO_PACKAGE_BUILD_OUTPUT_DIR = "dist";

/** 🏭️ The optional native codec factory a codec registers with the plugin host; every field is a string id. */
const STDIO_NATIVE_FACTORY_FIELDS = ["factory_id", "artifact_kind", "artifact_schema", "extension", "pack_schema_hash", "runtime_capability_id"] as const;

function stdioRel(workspaceRoot: string, path: string): string {
  return relative(workspaceRoot, path).split("\\").join("/");
}

/**
 * 🚶️ Every source file below `path`. Package build output is skipped: `.gitignore` excludes
 * `**​/📦️packages/*​/dist/`, so walking it would make a tracked catalog disagree with the tree on any
 * machine that has built a TypeScript package, and the gate would fail on build state instead of sources.
 */
function stdioWalkFiles(path: string, files: string[] = []): string[] {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    const full = join(path, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === STDIO_PACKAGE_BUILD_OUTPUT_DIR) continue;
      stdioWalkFiles(full, files);
    } else files.push(full);
  }
  return files;
}

function stdioAssertUnique(label: string, values: readonly string[]): void {
  const duplicateValues = [...new Set(values.filter((value, index) => values.indexOf(value) !== index))];
  if (duplicateValues.length > 0) throw new Error(`[stdio] duplicate ${label}: ${duplicateValues.sort().join(", ")}`);
}

function stdioDefinitionIdentity(id: string, suffix = ""): void {
  if (!/^s\.stdio\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(id)) throw new Error(`[stdio] non-canonical definition identity ${JSON.stringify(id)}.`);
  if (suffix && !id.includes(suffix)) throw new Error(`[stdio] definition identity ${JSON.stringify(id)} is missing ${suffix}.`);
}

function stdioVersionedLeaf(id: string, prefix: string): void {
  if (!id.startsWith(prefix) || !/^[a-z0-9-]+\.v[1-9][0-9]*$/u.test(id.slice(prefix.length))) throw new Error(`[stdio] ${JSON.stringify(id)} must be a canonical vN leaf below ${JSON.stringify(prefix)}.`);
  stdioDefinitionIdentity(id);
}

function stdioRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`[stdio] ${label} must be an object.`);
  return value as Record<string, unknown>;
}

function stdioExactFields(value: unknown, label: string, fields: readonly string[]): Record<string, unknown> {
  const record = stdioRecord(value, label);
  const missing = fields.filter((field) => !(field in record));
  const unknown = Object.keys(record).filter((field) => !fields.includes(field));
  if (missing.length > 0 || unknown.length > 0) throw new Error(`[stdio] ${label} has ${missing.length ? `missing fields ${missing.join(", ")}` : ""}${missing.length && unknown.length ? "; " : ""}${unknown.length ? `unknown fields ${unknown.join(", ")}` : ""}.`);
  return record;
}

function stdioString(value: unknown, label: string): string {
  if (typeof value !== "string" || !value) throw new Error(`[stdio] ${label} must be a non-empty string.`);
  return value;
}

const STDIO_RUNTIME_CLAIM_NAMESPACES = ["schema", "codec", "extension", "mime", "dialect", "grammar"] as const;

/** 🎟️ Admits exactly one member of a closed vocabulary, keeping the literal type the caller declares. */
function stdioMember<T extends string>(value: unknown, label: string, members: readonly T[]): T {
  const text = stdioString(value, label);
  const member = members.find((candidate) => candidate === text);
  if (member === undefined) throw new Error(`[stdio] ${label} must be one of ${members.join(", ")}.`);
  return member;
}

function stdioStringArray(value: unknown, label: string): readonly string[] {
  if (!Array.isArray(value) || !value.every((item) => typeof item === "string" && item)) throw new Error(`[stdio] ${label} must be a string array.`);
  return value;
}

function stdioNullableString(value: unknown, label: string): string | null {
  if (value === null || typeof value === "string") return value;
  throw new Error(`[stdio] ${label} must be a string or null.`);
}

function stdioArray(value: unknown, label: string): readonly unknown[] {
  if (!Array.isArray(value)) throw new Error(`[stdio] ${label} must be an array.`);
  return value;
}

function stdioDefinitionCatalog(workspaceRoot: string): readonly StdioSchemaDefinition[] {
  const catalogPath = join(workspaceRoot, STDIO_CATALOG_REL);
  const catalog = stdioExactFields(JSON.parse(readFileSync(catalogPath, "utf8")), "definition catalog", ["artifact_definition_paths"]);
  if (!Array.isArray(catalog.artifact_definition_paths)) throw new Error("[stdio] catalog must contain artifact_definition_paths.");
  const paths = catalog.artifact_definition_paths;
  if (!paths.every((path): path is string => typeof path === "string" && path.endsWith("📜️artifact-definition.json"))) throw new Error("[stdio] catalog paths must target artifact definition leaves.");
  stdioAssertUnique("artifact definition path", paths);
  const definitions = paths.map((definitionPath) => {
    const absolute = resolve(dirname(catalogPath), definitionPath);
    const artifactsRoot = resolve(workspaceRoot, STDIO_ROOT_REL, STDIO_ARTIFACTS_DIR);
    if (!absolute.startsWith(`${artifactsRoot}/`)) throw new Error(`[stdio] artifact definition escapes the stdio artifact root: ${definitionPath}.`);
    if (!existsSync(absolute)) throw new Error(`[stdio] artifact definition is missing: ${definitionPath}.`);
    return JSON.parse(readFileSync(absolute, "utf8"));
  });
  const artifactRoot = join(workspaceRoot, STDIO_ROOT_REL, STDIO_ARTIFACTS_DIR);
  const discovered = stdioWalkFiles(artifactRoot)
    .filter((path) => path.endsWith("📜️artifact-definition.json"))
    .map((path) => relative(dirname(catalogPath), path).split("\\").join("/"))
    .sort();
  if (discovered.join("\n") !== [...paths].sort().join("\n")) throw new Error("[stdio] catalog definition paths are not the complete schema-owned artifact definition set.");
  if (definitions.length !== 36) throw new Error(`[stdio] expected 36 artifact definitions, got ${definitions.length}.`);
  definitions.forEach(stdioAssertDefinition);
  return definitions as readonly StdioSchemaDefinition[];
}

function stdioAssertDefinition(value: unknown): asserts value is StdioSchemaDefinition {
  const definition = stdioExactFields(value, "artifact definition", ["definition_version", "id", "artifact", "directory", "dependencies", "standards", "profiles", "source_dialects", "representations", "codecs", "mutations", "inferences", "resources", "localized_descriptors", "conformance_suites", "runtime_capabilities", "support_ledger"]);
  if (definition.definition_version !== 1) throw new Error("[stdio] artifact definition_version must equal 1.");
  const artifact = stdioString(definition.artifact, "artifact slug");
  const id = stdioString(definition.id, "artifact definition id");
  const artifactId = `s.stdio.${definition.artifact}`;
  if (id !== artifactId) throw new Error(`[stdio] definition ${id} does not own ${artifactId}.`);
  stdioDefinitionIdentity(id);
  if (!/^[a-z0-9_-]+$/u.test(artifact)) throw new Error(`[stdio] invalid artifact slug ${JSON.stringify(artifact)}.`);
  stdioString(definition.directory, `${id}.directory`);
  stdioStringArray(definition.dependencies, `${id}.dependencies`);
  const standards = stdioArray(definition.standards, `${id}.standards`).map((standard, index) => {
    const record = stdioExactFields(standard, `${id}.standards[${index}]`, ["id", "revision", "normative_source", "publication_date", "source_checksum", "redistribution_status", "clauses_or_features", "status"]);
    stdioString(record.id, `${id}.standards[${index}].id`);
    const revision = stdioString(record.revision, `${id}.standards[${index}].revision`);
    if (!/^[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(revision)) throw new Error(`[stdio] ${id} standard revision ${JSON.stringify(revision)} is not a canonical identity fragment.`);
    stdioNullableString(record.normative_source, `${id}.standards[${index}].normative_source`);
    stdioNullableString(record.publication_date, `${id}.standards[${index}].publication_date`);
    stdioNullableString(record.source_checksum, `${id}.standards[${index}].source_checksum`);
    stdioString(record.redistribution_status, `${id}.standards[${index}].redistribution_status`);
    stdioStringArray(record.clauses_or_features, `${id}.standards[${index}].clauses_or_features`);
    stdioString(record.status, `${id}.standards[${index}].status`);
    return record as unknown as StdioSchemaDefinition["standards"][number];
  });
  if (standards.length === 0) throw new Error(`[stdio] ${id} has no standards.`);
  for (const standard of standards) {
    const id = `${artifactId}.standard.${standard.revision}`;
    if (standard.id !== id) throw new Error(`[stdio] standard ${standard.id} must be ${id}.`);
    stdioDefinitionIdentity(standard.id, ".standard.");
  }
  const standardIds = new Set(standards.map((standard) => standard.id));
  const profiles = stdioArray(definition.profiles, `${artifactId}.profiles`).map((profile, index) => {
    const record = stdioExactFields(profile, `${artifactId}.profiles[${index}]`, ["id", "standard", "profile", "status"]);
    stdioString(record.id, `${artifactId}.profiles[${index}].id`);
    stdioString(record.standard, `${artifactId}.profiles[${index}].standard`);
    stdioString(record.profile, `${artifactId}.profiles[${index}].profile`);
    stdioString(record.status, `${artifactId}.profiles[${index}].status`);
    return record as unknown as StdioSchemaDefinition["profiles"][number];
  });
  if (profiles.length === 0) throw new Error(`[stdio] ${artifactId} has no profiles.`);
  for (const profile of profiles) {
    if (!standardIds.has(profile.standard) || profile.id !== `${profile.standard}.profile.${profile.profile}`) throw new Error(`[stdio] invalid profile identity ${profile.id}.`);
    stdioDefinitionIdentity(profile.id, ".profile.");
  }
  const dialects = stdioArray(definition.source_dialects, `${artifactId}.source_dialects`).map((dialect, index) => {
    const record = stdioExactFields(dialect, `${artifactId}.source_dialects[${index}]`, ["id", "standard", "dialect", "registered_code_points", "status"]);
    stdioString(record.id, `${artifactId}.source_dialects[${index}].id`);
    stdioString(record.standard, `${artifactId}.source_dialects[${index}].standard`);
    stdioString(record.dialect, `${artifactId}.source_dialects[${index}].dialect`);
    stdioStringArray(record.registered_code_points, `${artifactId}.source_dialects[${index}].registered_code_points`);
    stdioString(record.status, `${artifactId}.source_dialects[${index}].status`);
    return record as unknown as StdioSchemaDefinition["source_dialects"][number];
  });
  if (dialects.length === 0) throw new Error(`[stdio] ${artifactId} has no source dialects.`);
  for (const dialect of dialects) {
    if (!standardIds.has(dialect.standard) || dialect.id !== `${dialect.standard}.dialect.${dialect.dialect}`) throw new Error(`[stdio] invalid source dialect identity ${dialect.id}.`);
    stdioDefinitionIdentity(dialect.id, ".dialect.");
  }
  const representations = stdioArray(definition.representations, `${artifactId}.representations`).map((representation, index) => {
    const record = stdioExactFields(representation, `${artifactId}.representations[${index}]`, ["id", "standard", "representation", "mimes", "extensions", "is_binary", "aliases", "neutral", "status"]);
    stdioString(record.id, `${artifactId}.representations[${index}].id`);
    stdioString(record.standard, `${artifactId}.representations[${index}].standard`);
    stdioString(record.representation, `${artifactId}.representations[${index}].representation`);
    stdioStringArray(record.mimes, `${artifactId}.representations[${index}].mimes`);
    stdioStringArray(record.extensions, `${artifactId}.representations[${index}].extensions`);
    if (typeof record.is_binary !== "boolean") throw new Error(`[stdio] ${artifactId}.representations[${index}].is_binary must be boolean.`);
    stdioStringArray(record.aliases, `${artifactId}.representations[${index}].aliases`);
    if (typeof record.neutral !== "boolean") throw new Error(`[stdio] ${artifactId}.representations[${index}].neutral must be boolean.`);
    stdioString(record.status, `${artifactId}.representations[${index}].status`);
    return record as unknown as StdioSchemaDefinition["representations"][number];
  });
  if (representations.length === 0) throw new Error(`[stdio] ${artifactId} has no representations.`);
  for (const representation of representations) {
    if (!standardIds.has(representation.standard) || representation.id !== `${representation.standard}.representation.${representation.representation}`) throw new Error(`[stdio] invalid representation identity ${representation.id}.`);
    if (representation.extensions.length === 0 || representation.extensions.some((extension) => !extension.startsWith("."))) throw new Error(`[stdio] ${representation.id} has no file extension.`);
    stdioAssertUnique(`MIME claim for ${representation.id}`, representation.mimes);
    stdioAssertUnique(`extension claim for ${representation.id}`, representation.extensions);
    stdioDefinitionIdentity(representation.id, ".representation.");
  }
  if (new Set(representations.map((representation) => representation.standard)).size !== standards.length) throw new Error(`[stdio] ${artifactId} must give each declared standard its own representation.`);
  const codecs = stdioArray(definition.codecs, `${artifactId}.codecs`).map((codec, index) => {
    const declared = ["id", "status", "from", "to", "executable_registration"];
    const record = stdioExactFields(codec, `${artifactId}.codecs[${index}]`, stdioRecord(codec, `${artifactId}.codecs[${index}]`).native_factory === undefined ? declared : [...declared, "native_factory"]);
    stdioString(record.id, `${artifactId}.codecs[${index}].id`);
    stdioString(record.status, `${artifactId}.codecs[${index}].status`);
    stdioString(record.from, `${artifactId}.codecs[${index}].from`);
    stdioString(record.to, `${artifactId}.codecs[${index}].to`);
    if (typeof record.executable_registration !== "boolean") throw new Error(`[stdio] ${artifactId}.codecs[${index}].executable_registration must be boolean.`);
    if (record.native_factory !== undefined) {
      const factory = stdioExactFields(record.native_factory, `${artifactId}.codecs[${index}].native_factory`, STDIO_NATIVE_FACTORY_FIELDS);
      for (const field of STDIO_NATIVE_FACTORY_FIELDS) stdioString(factory[field], `${artifactId}.codecs[${index}].native_factory.${field}`);
    }
    return record as unknown as StdioCodecDefinition & { executable_registration: boolean };
  });
  for (const codec of codecs) {
    const standard = standards.find((candidate) => codec.id.startsWith(`${candidate.id}.codec.`));
    if (!standard) throw new Error(`[stdio] codec ${codec.id} is not owned by a declared standard.`);
    stdioVersionedLeaf(codec.id, `${standard.id}.codec.`);
    if (!["unimplemented", "implemented", "verified"].includes(codec.status)) throw new Error(`[stdio] codec ${codec.id} has an invalid implementation status.`);
  }
  const mutations = stdioArray(definition.mutations, `${artifactId}.mutations`).map((mutation, index) => {
    const record = stdioExactFields(mutation, `${artifactId}.mutations[${index}]`, ["id", "status", "executable_registration"]);
    stdioString(record.id, `${artifactId}.mutations[${index}].id`);
    stdioString(record.status, `${artifactId}.mutations[${index}].status`);
    if (typeof record.executable_registration !== "boolean") throw new Error(`[stdio] ${artifactId}.mutations[${index}].executable_registration must be boolean.`);
    return record as unknown as StdioSchemaDefinition["mutations"][number] & { executable_registration: boolean };
  });
  for (const mutation of mutations) {
    stdioVersionedLeaf(mutation.id, `${artifactId}.mutation.`);
    if (artifact === "gltf" && (mutation.id.includes(".no-mutation.") || mutation.id.includes(".set-snapshot.") || mutation.id.includes(".set-"))) throw new Error(`[stdio] GLTF mutation ${mutation.id} is not semantically specific.`);
    if (!["unimplemented", "implemented", "verified"].includes(mutation.status)) throw new Error(`[stdio] mutation ${mutation.id} has an invalid implementation status.`);
  }
  const inferences = stdioArray(definition.inferences, `${artifactId}.inferences`).map((inference, index) => {
    const record = stdioExactFields(inference, `${artifactId}.inferences[${index}]`, ["id", "status", "executable_registration"]);
    stdioString(record.id, `${artifactId}.inferences[${index}].id`);
    stdioString(record.status, `${artifactId}.inferences[${index}].status`);
    if (typeof record.executable_registration !== "boolean") throw new Error(`[stdio] ${artifactId}.inferences[${index}].executable_registration must be boolean.`);
    return record as unknown as StdioSchemaDefinition["inferences"][number] & { executable_registration: boolean };
  });
  for (const inference of inferences) {
    stdioVersionedLeaf(inference.id, `${artifactId}.inference.`);
    if (!["unimplemented", "implemented", "verified"].includes(inference.status)) throw new Error(`[stdio] inference ${inference.id} has an invalid implementation status.`);
  }
  const runtimeCapabilities = stdioArray(definition.runtime_capabilities, `${artifactId}.runtime_capabilities`).map((capability, index) => {
    const record = stdioExactFields(capability, `${artifactId}.runtime_capabilities[${index}]`, ["id", "category", "descriptor", "claims"]);
    const category = stdioString(record.category, `${artifactId}.runtime_capabilities[${index}].category`);
    if (!["schema", "inference", "codec", "representation", "grammar", "composer", "subset-validator"].includes(category)) throw new Error(`[stdio] invalid runtime capability category ${category}.`);
    const capabilityId = stdioString(record.id, `${artifactId}.runtime_capabilities[${index}].id`);
    stdioVersionedLeaf(capabilityId, `${artifactId}.runtime.${category}.`);
    const descriptor = stdioString(record.descriptor, `${artifactId}.runtime_capabilities[${index}].descriptor`);
    const claims = stdioArray(record.claims, `${artifactId}.runtime_capabilities[${index}].claims`).map((claim, claimIndex) => {
      const claimRecord = stdioExactFields(claim, `${artifactId}.runtime_capabilities[${index}].claims[${claimIndex}]`, ["namespace", "value"]);
      const namespace = stdioMember(claimRecord.namespace, `${artifactId}.runtime_capabilities[${index}].claims[${claimIndex}].namespace`, STDIO_RUNTIME_CLAIM_NAMESPACES);
      return { namespace, value: stdioString(claimRecord.value, `${artifactId}.runtime_capabilities[${index}].claims[${claimIndex}].value`) };
    });
    if (claims.length === 0) throw new Error(`[stdio] runtime capability ${capabilityId} has no claims.`);
    stdioAssertUnique(`runtime claim for ${capabilityId}`, claims.map((claim) => `${claim.namespace}:${claim.value}`));
    return { id: capabilityId, category: category as StdioRuntimeCapability["category"], descriptor, claims } satisfies StdioRuntimeCapability;
  });
  stdioAssertUnique(`${artifactId} runtime capability`, runtimeCapabilities.map((capability) => capability.id));
  stdioAssertUnique(`${artifactId} runtime category claims`, runtimeCapabilities.map((capability) => `${capability.category}|${capability.claims.map((claim) => `${claim.namespace}:${claim.value}`).sort().join("|")}`));
  for (const capability of runtimeCapabilities.filter((capability) => capability.category === "representation")) {
    const claims = capability.claims.map((claim) => `${claim.namespace}:${claim.value}`).sort().join("|");
    if (representations.filter((representation) => [...representation.mimes.map((mime) => `mime:${mime}`), ...representation.extensions.map((extension) => `extension:${extension}`)].sort().join("|") === claims).length === 0) throw new Error(`[stdio] runtime representation ${capability.id} does not claim a representation leaf.`);
  }
  const resources = stdioArray(definition.resources, `${artifactId}.resources`).map((resource, index) => {
    const record = stdioExactFields(resource, `${artifactId}.resources[${index}]`, ["id", "external_reference_policy", "status"]);
    stdioString(record.id, `${artifactId}.resources[${index}].id`);
    stdioString(record.external_reference_policy, `${artifactId}.resources[${index}].external_reference_policy`);
    stdioString(record.status, `${artifactId}.resources[${index}].status`);
    return record as unknown as StdioSchemaDefinition["resources"][number];
  });
  const descriptors = stdioArray(definition.localized_descriptors, `${artifactId}.localized_descriptors`).map((descriptor, index) => {
    const record = stdioExactFields(descriptor, `${artifactId}.localized_descriptors[${index}]`, ["id", "locale", "name", "description", "status"]);
    const locale = stdioString(record.locale, `${artifactId}.localized_descriptors[${index}].locale`);
    if (locale !== "en" && locale !== "de") throw new Error(`[stdio] ${artifactId} has unsupported locale ${locale}.`);
    stdioString(record.id, `${artifactId}.localized_descriptors[${index}].id`);
    stdioString(record.name, `${artifactId}.localized_descriptors[${index}].name`);
    stdioString(record.description, `${artifactId}.localized_descriptors[${index}].description`);
    stdioString(record.status, `${artifactId}.localized_descriptors[${index}].status`);
    return record as unknown as StdioSchemaDefinition["localized_descriptors"][number];
  });
  const expectedLocales = ["en", "de"];
  if (descriptors.length !== expectedLocales.length || descriptors.some((descriptor) => descriptor.id !== `${artifactId}.localization.${descriptor.locale}`) || expectedLocales.some((locale) => descriptors.filter((descriptor) => descriptor.locale === locale).length !== 1)) throw new Error(`[stdio] ${artifactId} must own exactly one English and German descriptor.`);
  const suites = stdioArray(definition.conformance_suites, `${artifactId}.conformance_suites`).map((suite, index) => {
    const record = stdioExactFields(suite, `${artifactId}.conformance_suites[${index}]`, ["id", "status", "fixtures"]);
    stdioString(record.id, `${artifactId}.conformance_suites[${index}].id`);
    stdioString(record.status, `${artifactId}.conformance_suites[${index}].status`);
    stdioStringArray(record.fixtures, `${artifactId}.conformance_suites[${index}].fixtures`);
    return record as unknown as StdioSchemaDefinition["conformance_suites"][number];
  });
  if (resources.length === 0 || suites.length === 0) throw new Error(`[stdio] ${artifactId} must own resources and conformance suites.`);
  const ledger = stdioExactFields(definition.support_ledger, `${artifactId}.support_ledger`, ["normative_source", "publication_date", "source_checksum", "redistribution_status", "clauses_or_features", "profiles", "registered_code_points", "read", "write", "lossless", "canonical", "validators", "mutations", "inferences", "fixtures"]);
  const ledgerNormativeSource = stdioNullableString(ledger.normative_source, `${artifactId}.support_ledger.normative_source`);
  const ledgerPublicationDate = stdioNullableString(ledger.publication_date, `${artifactId}.support_ledger.publication_date`);
  const ledgerSourceChecksum = stdioNullableString(ledger.source_checksum, `${artifactId}.support_ledger.source_checksum`);
  const ledgerRedistributionStatus = stdioString(ledger.redistribution_status, `${artifactId}.support_ledger.redistribution_status`);
  const ledgerLists = Object.fromEntries(
    (["clauses_or_features", "profiles", "registered_code_points", "validators", "mutations", "inferences", "fixtures"] as const).map((field) => [field, stdioStringArray(ledger[field], `${artifactId}.support_ledger.${field}`)]),
  ) as Readonly<Record<"clauses_or_features" | "profiles" | "registered_code_points" | "validators" | "mutations" | "inferences" | "fixtures", readonly string[]>>;
  const ledgerStates = (["read", "write", "lossless", "canonical"] as const).map((field) => stdioString(ledger[field], `${artifactId}.support_ledger.${field}`));
  if (ledgerStates.some((state) => !["unimplemented", "opaque", "implemented"].includes(state))) throw new Error(`[stdio] ${definition.id} has an invalid support ledger.`);
  const implemented = ledgerStates.includes("implemented");
  if (implemented && (!ledgerNormativeSource || !ledgerPublicationDate || !ledgerSourceChecksum || ledgerRedistributionStatus === "unknown" || ledgerLists.clauses_or_features.length === 0 || ledgerLists.validators.length === 0 || ledgerLists.fixtures.length === 0)) throw new Error(`[stdio] ${artifactId} claims implemented support without normative, validator, and fixture evidence.`);
  const IDs = [
    id, ...standards.map((item) => item.id), ...profiles.map((item) => item.id), ...dialects.map((item) => item.id), ...representations.map((item) => item.id), ...codecs.map((item) => item.id), ...mutations.map((item) => item.id), ...inferences.map((item) => item.id), ...resources.map((item) => item.id), ...descriptors.map((item) => item.id), ...suites.map((item) => item.id), ...runtimeCapabilities.map((item) => item.id),
  ];
  for (const id of IDs) stdioDefinitionIdentity(id);
  stdioAssertUnique(`definition identity for ${artifactId}`, IDs);
}

export function stdioArtifactLedger(workspaceRoot: string): StdioArtifactLedger {
  const definitions = stdioDefinitionCatalog(workspaceRoot);
  stdioAssertUnique("artifact identity", definitions.map((definition) => definition.id));
  stdioAssertUnique("artifact directory", definitions.map((definition) => definition.directory));
  const knownArtifacts = new Set(definitions.map((definition) => definition.id));
  const knownDialectIds = new Set(definitions.flatMap((definition) => definition.source_dialects.map((dialect) => dialect.id)));
  const knownIdentityIds = new Set(definitions.flatMap((definition) => [
    definition.id,
    ...definition.standards.map((item) => item.id),
    ...definition.profiles.map((item) => item.id),
    ...definition.source_dialects.map((item) => item.id),
    ...definition.representations.map((item) => item.id),
    ...definition.codecs.map((item) => item.id),
    ...definition.mutations.map((item) => item.id),
    ...definition.inferences.map((item) => item.id),
    ...definition.runtime_capabilities.map((item) => item.id),
    ...definition.resources.map((item) => item.id),
    ...definition.localized_descriptors.map((item) => item.id),
    ...definition.conformance_suites.flatMap((item) => [item.id, ...item.fixtures]),
  ]));
  const equalIdSets = (left: readonly string[], right: readonly string[]): boolean => [...new Set(left)].sort().join("\n") === [...new Set(right)].sort().join("\n");
  for (const definition of definitions) {
    stdioAssertUnique(`dependency for ${definition.id}`, definition.dependencies);
    for (const dependency of definition.dependencies) {
      if (!knownArtifacts.has(dependency) || dependency === definition.id) throw new Error(`[stdio] ${definition.id} has an invalid dependency ${dependency}.`);
    }
    const localIds = new Set([
      definition.id,
      ...definition.standards.map((item) => item.id),
      ...definition.profiles.map((item) => item.id),
      ...definition.source_dialects.map((item) => item.id),
      ...definition.representations.map((item) => item.id),
      ...definition.codecs.map((item) => item.id),
      ...definition.mutations.map((item) => item.id),
      ...definition.inferences.map((item) => item.id),
      ...definition.runtime_capabilities.map((item) => item.id),
      ...definition.resources.map((item) => item.id),
      ...definition.localized_descriptors.map((item) => item.id),
      ...definition.conformance_suites.flatMap((item) => [item.id, ...item.fixtures]),
    ]);
    for (const fixture of definition.conformance_suites.flatMap((suite) => suite.fixtures)) {
      stdioDefinitionIdentity(fixture);
      if (!localIds.has(fixture)) throw new Error(`[stdio] ${definition.id} fixture ${fixture} does not resolve locally.`);
    }
    const ledger = definition.support_ledger;
    const declaredCodePoints = definition.source_dialects.flatMap((dialect) => dialect.registered_code_points);
    if (!equalIdSets(ledger.profiles, definition.profiles.map((profile) => profile.id))) throw new Error(`[stdio] ${definition.id} support ledger profiles diverge from schema profiles.`);
    if (!equalIdSets(ledger.registered_code_points, declaredCodePoints)) throw new Error(`[stdio] ${definition.id} support ledger code points diverge from source dialects.`);
    if (!equalIdSets(ledger.mutations, definition.mutations.map((mutation) => mutation.id))) throw new Error(`[stdio] ${definition.id} support ledger mutations diverge from definitions.`);
    if (!equalIdSets(ledger.inferences, definition.inferences.map((inference) => inference.id))) throw new Error(`[stdio] ${definition.id} support ledger inferences diverge from definitions.`);
    if (!equalIdSets(ledger.fixtures, definition.conformance_suites.flatMap((suite) => suite.fixtures))) throw new Error(`[stdio] ${definition.id} support ledger fixtures diverge from conformance suites.`);
    for (const reference of [...ledger.validators, ...ledger.mutations, ...ledger.inferences, ...ledger.fixtures]) {
      if (!localIds.has(reference) || !knownIdentityIds.has(reference)) throw new Error(`[stdio] ${definition.id} support ledger reference ${reference} does not resolve.`);
    }
    for (const codec of definition.codecs) {
      if (!codec.from || !codec.to || !knownDialectIds.has(codec.from) || !knownDialectIds.has(codec.to)) throw new Error(`[stdio] ${definition.id} codec ${codec.id} references an unknown source dialect.`);
    }
  }
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const definitionsById = new Map(definitions.map((definition) => [definition.id, definition]));
  const visit = (id: string): void => {
    if (visiting.has(id)) throw new Error(`[stdio] cyclic definition dependency at ${id}.`);
    if (visited.has(id)) return;
    visiting.add(id);
    definitionsById.get(id)!.dependencies.forEach(visit);
    visiting.delete(id);
    visited.add(id);
  };
  definitions.forEach((definition) => visit(definition.id));
  const artifacts = definitions.map((definition) => ({
      id: definition.artifact,
      kind: definition.id,
      directory: definition.directory,
      depends: definition.dependencies.map((dependency) => dependency.slice("s.stdio.".length)),
      component: stdioRel(workspaceRoot, join(workspaceRoot, STDIO_ROOT_REL, STDIO_ARTIFACTS_DIR, definition.directory, STDIO_COMPONENT_TS)),
      dialects: definition.source_dialects.map((dialect) => ({ id: dialect.id, artifact: definition.artifact, standard: dialect.standard, subset: dialect.dialect, schema: "", io: "", mutations: null, inferences: null })),
      representations: definition.representations,
      codecs: [...definition.codecs],
      support: definition.support_ledger,
    } satisfies StdioArtifactDefinition));
  const representations = definitions.flatMap((definition) => definition.representations);
  for (const [label, claims] of [["registered MIME", representations.flatMap((representation) => representation.mimes.map((mime) => [mime, definitions.find((definition) => definition.representations.includes(representation))!.id] as const))], ["file extension", representations.flatMap((representation) => representation.extensions.map((extension) => [extension, definitions.find((definition) => definition.representations.includes(representation))!.id] as const))]] as const) {
    const owners = new Map<string, string>();
    for (const [claim, owner] of claims) {
      const existing = owners.get(claim);
      if (existing && existing !== owner) throw new Error(`[stdio] ${label} ${claim} is claimed by both ${existing} and ${owner}.`);
      owners.set(claim, owner);
    }
  }
  stdioAssertUnique("source dialect", artifacts.flatMap((artifact) => artifact.dialects.map((dialect) => dialect.id)));
  stdioAssertUnique("codec", artifacts.flatMap((artifact) => artifact.codecs.map((codec) => codec.id)));
  stdioAssertUnique("runtime capability", definitions.flatMap((definition) => definition.runtime_capabilities.map((capability) => capability.id)));
  const epw = artifacts.find((artifact) => artifact.id === "epw");
  if (!epw || epw.representations.some((representation) => representation.mimes.length !== 0)) throw new Error("[stdio] EPW must remain MIME-unregistered.");
  const txt = artifacts.find((artifact) => artifact.id === "txt");
  if (!txt || !txt.representations.some((representation) => representation.mimes.includes("text/plain"))) throw new Error("[stdio] TXT must own text/plain.");
  return {
    artifacts,
    counts: {
      artifacts: artifacts.length,
      registeredMimes: representations.reduce((count, representation) => count + representation.mimes.length, 0),
      standards: definitions.reduce((count, definition) => count + definition.standards.length, 0),
      profiles: definitions.reduce((count, definition) => count + definition.profiles.length, 0),
      dialects: definitions.reduce((count, definition) => count + definition.source_dialects.length, 0),
      representations: definitions.reduce((count, definition) => count + definition.representations.length, 0),
      codecs: definitions.reduce((count, definition) => count + definition.codecs.length, 0),
      mutations: definitions.reduce((count, definition) => count + definition.mutations.length, 0),
      inferences: definitions.reduce((count, definition) => count + definition.inferences.length, 0),
      conformanceSuites: definitions.reduce((count, definition) => count + definition.conformance_suites.length, 0),
      runtimeCapabilities: definitions.reduce((count, definition) => count + definition.runtime_capabilities.length, 0),
      capabilities: {
        declared: definitions.reduce((count, definition) => count + definition.codecs.length + definition.mutations.length + definition.inferences.length, 0),
        registered: definitions.reduce((count, definition) => count + definition.codecs.filter((item) => item.executable_registration).length + definition.mutations.filter((item) => item.executable_registration).length + definition.inferences.filter((item) => item.executable_registration).length, 0),
        implemented: definitions.reduce((count, definition) => count + definition.codecs.filter((item) => item.status === "implemented").length + definition.mutations.filter((item) => item.status === "implemented").length + definition.inferences.filter((item) => item.status === "implemented").length, 0),
        verified: definitions.reduce((count, definition) => count + definition.codecs.filter((item) => item.status === "verified").length + definition.mutations.filter((item) => item.status === "verified").length + definition.inferences.filter((item) => item.status === "verified").length, 0),
      },
    },
  };
}

/** 🗄️ Runs schema-derived stdio catalog, support-ledger, and runtime gates. */
function stdioAssertTypeScriptExports(workspaceRoot: string, ledger: StdioArtifactLedger): void {
  const barrel = readFileSync(join(workspaceRoot, STDIO_ROOT_REL, "📦️packages", "🟦️typescript", "🟦️.ts"), "utf8");
  const exports = [...barrel.matchAll(/^export \* as (\w+) from /gmu)].map((match) => match[1]!);
  stdioAssertUnique("TypeScript export", exports);
  const expected = ledger.artifacts.map((artifact) => artifact.id).sort();
  if (exports.sort().join("\n") !== expected.join("\n")) throw new Error("[stdio] TypeScript exports are not definition-complete.");
  for (const artifact of ledger.artifacts) {
    const component = readFileSync(join(workspaceRoot, artifact.component), "utf8");
    if (!component.includes("📜️artifact-definition.json") || !component.includes("export { definition }")) throw new Error(`[stdio] ${artifact.id} exports a namespace instead of its schema-owned definition.`);
  }
}

function stdioAssertManifestAssembly(workspaceRoot: string, ledger: StdioArtifactLedger): void {
  const manifest = readFileSync(join(workspaceRoot, STDIO_ROOT_REL, "🛂️manifest", STDIO_COMPONENT_RS), "utf8");
  if (!manifest.includes("crate::registry::format_descriptors()")) throw new Error("[stdio] manifest must derive descriptors from schema-owned definitions.");
  if (!manifest.includes("crate::registry::artifact_definitions()")) throw new Error("[stdio] manifest must expose the schema-owned artifact definition assembly.");
  const epw = ledger.artifacts.find((artifact) => artifact.id === "epw");
  if (!epw || epw.representations.some((representation) => representation.mimes.length !== 0)) throw new Error("[stdio] EPW must remain MIME-unregistered in the schema ledger.");
}

export function stdioRequireClosed(ledger: StdioArtifactLedger, gate: string): never {
  const incomplete = ledger.artifacts.filter((artifact) => artifact.support.read !== "implemented" || artifact.support.write !== "implemented" || artifact.support.lossless !== "implemented" || artifact.support.canonical !== "implemented" || artifact.codecs.length === 0);
  throw new Error(`[stdio] ${gate} is not closed: ${incomplete.length} schema-owned support ledgers remain unimplemented; no runtime, fuzz, or cross-platform support is claimed.`);
}

export function stdioRunStructuralGate(workspaceRoot: string, gate: "quick" | "schema-parity" | "standards-coverage" | "codec" | "mutation-law" | "inference"): StdioArtifactLedger {
  const ledger = stdioArtifactLedger(workspaceRoot);
  stdioAssertTypeScriptExports(workspaceRoot, ledger);
  stdioAssertManifestAssembly(workspaceRoot, ledger);
  if (gate !== "quick") stdioRequireClosed(ledger, gate);
  return ledger;
}
