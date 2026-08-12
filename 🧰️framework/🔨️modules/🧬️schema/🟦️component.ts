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
/** 🔗 Shared GraphQL `@state` SDL preamble — declared once, never repeated per artifact. */
export const GRAPHQL_STATE_PREAMBLE =
  "enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT INFERRED }\n" +
  "directive @state(class: StateClass!) on FIELD_DEFINITION";
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
