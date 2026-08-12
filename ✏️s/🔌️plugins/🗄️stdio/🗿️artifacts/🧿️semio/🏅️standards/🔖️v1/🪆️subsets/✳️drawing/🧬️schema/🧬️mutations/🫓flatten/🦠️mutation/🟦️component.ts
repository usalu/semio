/** mutation payload — mirrors `FlattenNode`. No-op unless `at` is a `Group` whose every
 * descendant group has an identity transform (see the Rust sibling's own doc comment). */
export interface FlattenNode {
  at: { layer: number; path: number[] };
}
