//#region 🔖️ArtifactSchemaDescriptor
/** 🍃 Five handcrafted leaf bodies for one facet — TS twin of Rust `FacetLeaves`. */
export type FacetLeaves = {
  readonly rust: string;
  readonly typescript: string;
  readonly graphql: string;
  readonly jsonSchema: string;
  readonly proto: string;
};

/** 🧬️ Registered descriptor for one artifact's four schema facets. */
export type ArtifactSchemaDescriptor = {
  readonly id: string;
  readonly artifact: FacetLeaves;
  readonly snapshot: FacetLeaves;
  readonly diff: FacetLeaves;
  readonly mutations: FacetLeaves;
};
//#endregion 🔖️ArtifactSchemaDescriptor

//#region 🔖️ArtifactInferenceDescriptor
/** 💡️ Registered descriptor for one artifact's 💡️inference schema facet — a SIBLING to
 * {@link ArtifactSchemaDescriptor}, not a field on it: adopted per-artifact independently
 * (seed-then-shrink fan-out), so the four-facet descriptor never needs editing as artifacts gain
 * inference one at a time. `id` is the inference schema's own id, `"{artifactId}.inference"`. */
export type ArtifactInferenceDescriptor = {
  readonly id: string;
  readonly inference: FacetLeaves;
};
//#endregion 🔖️ArtifactInferenceDescriptor

//#region 🔖️GraphQlStatePreamble
/** 🔗 Shared GraphQL `@state`/`@derived` SDL preamble — TS twin of Rust `GRAPHQL_STATE_PREAMBLE`.
 * `@state` names one of the four state lanes; `@derived` is the ORTHOGONAL derivation marker, never
 * a fifth lane — a derived field is computed from a snapshot, so it is not state at all. */
export const GRAPHQL_STATE_PREAMBLE =
  "enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }\n" +
  "directive @state(class: StateClass!) on FIELD_DEFINITION\n" +
  "directive @derived on FIELD_DEFINITION";

/** 🗂️ The four — and only four — state lanes, TS twin of Rust `StateClass`. `artifact` = persisted
 * shared, `config` = persisted local-only, `presence` = ephemeral shared, `transient` = ephemeral
 * local-only UI state. Spelled in the canonical kebab `x-semio-state` vocabulary. */
export const STATE_CLASSES = ["artifact", "config", "presence", "transient"] as const;
export type StateClass = (typeof STATE_CLASSES)[number];

/** 🏷️ Canonical JSON Schema key carrying the derivation marker, sibling of `x-semio-state` on the
 * orthogonal axis. Its only legal value is `true`; an absent key means "not derived". */
export const JSON_SCHEMA_DERIVED_KEY = "x-semio-derived";
//#endregion 🔖️GraphQlStatePreamble

//#region 🔖️ArtifactCompositionSpec
/** 🧒 One declared CHILD slot on an artifact snapshot — TS twin of Rust `ChildSlotSpec`. `kind` is a
 * plain canonical artifact kind id string, grammar `s.<plugin>.<artifact>` (e.g. `"s.stdio.mesh"`). */
export type ChildSlotSpec = {
  readonly name: string;
  readonly kind: string;
  readonly many: boolean;
};

/** 🔗 One declared LINK slot on an artifact snapshot — TS twin of Rust `LinkSlotSpec`. */
export type LinkSlotSpec = {
  readonly name: string;
  readonly roles: readonly string[];
  readonly many: boolean;
};

/** 🔗 Shared GraphQL SDL fragment for CHILD/LINK slots — TS twin of Rust `GRAPHQL_COMPOSITION_PREAMBLE`. */
export const GRAPHQL_COMPOSITION_PREAMBLE =
  "type ArtifactLink { targetId: String! kind: String! }\n" +
  "directive @child(kind: String!) on FIELD_DEFINITION\n" +
  "directive @link(roles: [String!]) on FIELD_DEFINITION";
//#endregion 🔖️ArtifactCompositionSpec

//#region 🔖️ArtifactSchemaRegistry
/** 📚 Runtime registry of {@link ArtifactSchemaDescriptor} values. */
export class ArtifactSchemaRegistry {
  readonly #byId = new Map<string, ArtifactSchemaDescriptor>();

  /** 📎 Insert or replace a descriptor by id. */
  register(descriptor: ArtifactSchemaDescriptor): void {
    this.#byId.set(descriptor.id, descriptor);
  }

  /** 🔎 Lookup by artifact schema id. */
  get(id: string): ArtifactSchemaDescriptor | undefined {
    return this.#byId.get(id);
  }

  /** 🚶 Walk every registered descriptor. */
  *iter(): IterableIterator<ArtifactSchemaDescriptor> {
    yield* this.#byId.values();
  }
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️ArtifactInferenceRegistry
/** 📚 Runtime registry of {@link ArtifactInferenceDescriptor} values — inference twin of {@link ArtifactSchemaRegistry}. */
export class ArtifactInferenceRegistry {
  readonly #byId = new Map<string, ArtifactInferenceDescriptor>();

  /** 📎 Insert or replace a descriptor by inference schema id. */
  register(descriptor: ArtifactInferenceDescriptor): void {
    this.#byId.set(descriptor.id, descriptor);
  }

  /** 🔎 Lookup by inference schema id. */
  get(id: string): ArtifactInferenceDescriptor | undefined {
    return this.#byId.get(id);
  }

  /** 🚶 Walk every registered descriptor. */
  *iter(): IterableIterator<ArtifactInferenceDescriptor> {
    yield* this.#byId.values();
  }

  /** 🔢 Count of registered inference facets. */
  get size(): number {
    return this.#byId.size;
  }
}
//#endregion 🔖️ArtifactInferenceRegistry

//#region 🔖️AppSchemaDescriptor
/** 🧬️ Registered descriptor for one app owner's config + presence schema facets. */
export type AppSchemaDescriptor = {
  readonly id: string;
  readonly config: FacetLeaves;
  readonly presence: FacetLeaves;
};
//#endregion 🔖️AppSchemaDescriptor

//#region 🔖️AppSchemaRegistry
/** 📚 Runtime registry of {@link AppSchemaDescriptor} values — app twin of {@link ArtifactSchemaRegistry}. */
export class AppSchemaRegistry {
  readonly #byId = new Map<string, AppSchemaDescriptor>();

  /** 📎 Insert or replace a descriptor by owner id. */
  register(descriptor: AppSchemaDescriptor): void {
    this.#byId.set(descriptor.id, descriptor);
  }

  /** 🔎 Lookup by app schema owner id. */
  get(id: string): AppSchemaDescriptor | undefined {
    return this.#byId.get(id);
  }

  /** 🚶 Walk every registered descriptor. */
  *iter(): IterableIterator<AppSchemaDescriptor> {
    yield* this.#byId.values();
  }

  /** 🔢 Count of registered app schema owner ids. */
  get size(): number {
    return this.#byId.size;
  }

  /** 📭 Whether no owners are registered yet (A6 fills the catalog). */
  get isEmpty(): boolean {
    return this.#byId.size === 0;
  }
}
//#endregion 🔖️AppSchemaRegistry

//#region 🔖️SchemaExportResolution
/** 🗂️ The five schema formats a scope publishes — TS twin of Rust `SchemaFormat`, keyed by the ascii
 * id the derived catalog and the `schema://` resolver use. */
export const SCHEMA_FORMATS = ["rust", "typescript", "graphql", "jsonschema", "protobuf"] as const;
export type SchemaFormat = (typeof SCHEMA_FORMATS)[number];

/** 🧩 Taxonomy `schemaFormats` key each format is the twin of. */
export const SCHEMA_FORMAT_TAXONOMY_KEYS: Readonly<Record<SchemaFormat, string>> = {
  rust: "🦀️rust",
  typescript: "🟦️typescript",
  graphql: "🔗️graphql",
  jsonschema: "🔣️jsonschema",
  protobuf: "🛰️protobuf",
};

/** 🍃 Leaf field each format occupies inside a {@link FacetLeaves}. */
export const SCHEMA_FORMAT_LEAVES: Readonly<Record<SchemaFormat, keyof FacetLeaves>> = {
  rust: "rust",
  typescript: "typescript",
  graphql: "graphql",
  jsonschema: "jsonSchema",
  protobuf: "proto",
};

/** 🔒️ Export ids reserved by the four fixed facets of {@link ArtifactSchemaDescriptor}. */
export const RESERVED_FACET_EXPORT_IDS = ["artifact", "snapshot", "diff", "mutations"] as const;

/** 🏷️ One named export of a scope — TS twin of Rust `SchemaExport`. */
export type SchemaExport = {
  readonly id: string;
  readonly leaves: FacetLeaves;
};

/** 🧬️ A scope's named exports — TS twin of Rust `ScopeSchemaExports`, a SIBLING to
 * {@link ArtifactSchemaDescriptor}'s four fixed facets rather than a field on it. */
export type ScopeSchemaExports = {
  readonly scope: string;
  readonly exports: readonly SchemaExport[];
};

/** 📇️ One resolvable `(scope id, export id, format id)` triple — TS twin of Rust `SchemaExportEntry`. */
export type SchemaExportEntry = {
  readonly scope: string;
  readonly export: string;
  readonly format: SchemaFormat;
};

/** 🪪️ Taxonomy `schemaExportResolution.rustEntriesContractId` — the only `contractId` a registry dump
 * may carry, and the value `schema verify --rust-entries` refuses to read under any other name. */
export const SCHEMA_EXPORT_ENTRIES_CONTRACT_ID = "schema-export-registry-entries-v1";

/** 📤️ The runtime export registry rendered for `schema verify --rust-entries` — TS twin of Rust
 * `SchemaExportEntries`, sorted and deduplicated by `(scope, export, format)`. */
export type SchemaExportEntries = {
  readonly contractId: typeof SCHEMA_EXPORT_ENTRIES_CONTRACT_ID;
  readonly generator: string;
  readonly entries: readonly SchemaExportEntry[];
};

/** ⚠️ Why `(scope id, export id, format id)` did not resolve — TS twin of Rust `SchemaResolveError`. */
export type SchemaResolveError =
  | { readonly kind: "unknown-scope"; readonly scope: string }
  | { readonly kind: "unknown-export"; readonly scope: string; readonly export: string }
  | { readonly kind: "format-absent"; readonly scope: string; readonly export: string; readonly format: SchemaFormat }
  | { readonly kind: "ambiguous-scope"; readonly scope: string; readonly export: string };

/** 🩺 One structural validation failure as the owned draft-07 validator renders it — TS twin of Rust
 * `ValidationDiagnostic`. The emitted message is `${instancePath}: ${reason}`. */
export type ValidationDiagnostic = {
  readonly instancePath: string;
  readonly reason: string;
};
//#endregion 🔖️SchemaExportResolution

//#region 🔖️EntityKindCatalog
/** 🏷️ One entity kind of the catalog — TS twin of Rust `EntityKind` (the generated `🤖️generated.rs`
 * projection) and of `🔣️.json#/$defs/EntityKind`. */
export type EntityKind = {
  readonly id: string;
  readonly emoji: string;
  readonly iconId: string;
  readonly label: string;
  readonly filterable: boolean;
};

/** 📚️ The ordered entity-kind catalog — TS twin of `🔣️.json#/$defs/EntityKindCatalog`, whose single
 * instance document is `🔣️entity-kinds.json`. Declaration order is contract: `emoji` is not unique and
 * every projection's emoji index keeps the FIRST entry of a repeated emoji. */
export type EntityKindCatalog = readonly EntityKind[];

const ENTITY_KIND_KEBAB = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/;
const ENTITY_KIND_NO_SPACE = /^\S+$/;
const ENTITY_KIND_TRIMMED = /^\S(?:.*\S)?$/;

/** ⚠️ Rejects one instance the way the owned draft-07 validator renders a {@link ValidationDiagnostic}:
 * `${instancePath}: ${reason}`. */
function entityKindReject(instancePath: string, reason: string): never {
  throw new Error(`${instancePath}: ${reason}`);
}

function entityKindText(value: unknown, instancePath: string, expression: RegExp, minLength: number, maxLength: number, expectation: string): string {
  if (typeof value !== "string") entityKindReject(instancePath, "value is not a string");
  const length = [...value].length;
  if (length < minLength) entityKindReject(instancePath, `value is shorter than ${minLength} characters`);
  if (length > maxLength) entityKindReject(instancePath, `value is longer than ${maxLength} characters`);
  if (!expression.test(value)) entityKindReject(instancePath, expectation);
  return value;
}

/** 🏷️ Parses one {@link EntityKind}, implementing `🔣️.json#/$defs/EntityKind` exactly — string lengths
 * are counted in code points so they agree with a draft-07 validator's `minLength`/`maxLength` on
 * multi-unit emoji, and no property outside the closed object is tolerated. */
export function parseEntityKind(value: unknown, instancePath = "$"): EntityKind {
  if (typeof value !== "object" || value === null || Array.isArray(value)) entityKindReject(instancePath, "value is not an object");
  const members = value as Record<string, unknown>;
  for (const key of Object.keys(members)) if (!["id", "emoji", "iconId", "label", "filterable"].includes(key)) entityKindReject(`${instancePath}.${key}`, "property is not declared by the schema");
  for (const key of ["id", "emoji", "iconId", "label", "filterable"]) if (!(key in members)) entityKindReject(`${instancePath}.${key}`, "required property is missing");
  if (typeof members.filterable !== "boolean") entityKindReject(`${instancePath}.filterable`, "value is not a boolean");
  return {
    id: entityKindText(members.id, `${instancePath}.id`, ENTITY_KIND_KEBAB, 3, 32, "value is not a kebab-case entity kind id"),
    emoji: entityKindText(members.emoji, `${instancePath}.emoji`, ENTITY_KIND_NO_SPACE, 1, 8, "value contains whitespace"),
    iconId: entityKindText(members.iconId, `${instancePath}.iconId`, ENTITY_KIND_KEBAB, 3, 32, "value is not a kebab-case icon id"),
    label: entityKindText(members.label, `${instancePath}.label`, ENTITY_KIND_TRIMMED, 2, 32, "value is not trimmed"),
    filterable: members.filterable,
  };
}

/** 📚️ Parses the whole {@link EntityKindCatalog}, adding the two invariants draft-07 cannot express —
 * `id` and `label` are unique catalog-wide — on top of `minItems`, `uniqueItems` and the item schema. */
export function parseEntityKindCatalog(value: unknown, instancePath = "$"): EntityKindCatalog {
  if (!Array.isArray(value)) entityKindReject(instancePath, "value is not an array");
  if (value.length === 0) entityKindReject(instancePath, "array has fewer than 1 items");
  const kinds = value.map((item, index) => parseEntityKind(item, `${instancePath}[${index}]`));
  const seenIds = new Set<string>();
  const seenLabels = new Set<string>();
  const seenItems = new Set<string>();
  kinds.forEach((kind, index) => {
    if (seenIds.has(kind.id)) entityKindReject(`${instancePath}[${index}].id`, `id ${kind.id} is already declared`);
    if (seenLabels.has(kind.label)) entityKindReject(`${instancePath}[${index}].label`, `label ${kind.label} is already declared`);
    const item = JSON.stringify([kind.id, kind.emoji, kind.iconId, kind.label, kind.filterable]);
    if (seenItems.has(item)) entityKindReject(`${instancePath}[${index}]`, "array items are not unique");
    seenIds.add(kind.id);
    seenLabels.add(kind.label);
    seenItems.add(item);
  });
  return kinds;
}

/** 🔍️ Builds the FIRST-WINS emoji index of a catalog — the one derivation every projection shares.
 * `new Map(catalog.map((kind) => [kind.emoji, kind]))` would be LAST-wins and silently resolve 🌱️ to
 * `interaction-started` and 📝️ to `draft`'s shadow `todo`. */
export function entityKindIndexByEmoji(catalog: EntityKindCatalog): ReadonlyMap<string, EntityKind> {
  const index = new Map<string, EntityKind>();
  for (const kind of catalog) if (!index.has(kind.emoji)) index.set(kind.emoji, kind);
  return index;
}
//#endregion 🔖️EntityKindCatalog
