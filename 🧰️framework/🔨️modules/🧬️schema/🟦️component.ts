//#region 🔖️ArtifactSchemaDescriptor
/** 🍃 Five handcrafted leaf bodies for one facet — TS twin of Rust `FacetLeaves`. */
export type FacetLeaves = {
  readonly rust: string;
  readonly typescript: string;
  readonly graphql: string;
  readonly jsonSchema: string;
  readonly proto: string;
};

/** 🧬️ Registered descriptor for one artifact's three schema facets. */
export type ArtifactSchemaDescriptor = {
  readonly id: string;
  readonly artifact: FacetLeaves;
  readonly snapshot: FacetLeaves;
  readonly diff: FacetLeaves;
};
//#endregion 🔖️ArtifactSchemaDescriptor

//#region 🔖️GraphQlStatePreamble
/** 🔗 Shared GraphQL `@state` SDL preamble — declared once, never repeated per artifact. */
export const GRAPHQL_STATE_PREAMBLE =
  "enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT }\n" +
  "directive @state(class: StateClass!) on FIELD_DEFINITION";
//#endregion 🔖️GraphQlStatePreamble

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
