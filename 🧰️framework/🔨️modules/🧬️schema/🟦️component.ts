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
