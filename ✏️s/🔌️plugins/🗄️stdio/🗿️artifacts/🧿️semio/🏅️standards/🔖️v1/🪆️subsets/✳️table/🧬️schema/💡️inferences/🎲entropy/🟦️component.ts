/** 🎲 real facet mirror of `ColumnEntropy`'s `InferredField::Value`. Keyed by column NAME (the
 * native key — see the Rust sibling's `SemioTableColumn` doc comment). Unlike `SemioColumnMoments`,
 * defined over every declared column regardless of `kind` (entropy is a property of any discrete
 * symbol alphabet, not just numeric data). Not yet wired into the parent `SemioTableInference`
 * aggregate's TS surface (see the Rust sibling's own doc comment). */
export interface SemioColumnEntropy {
  count: number;
  distinct: number;
  bits: number;
}
