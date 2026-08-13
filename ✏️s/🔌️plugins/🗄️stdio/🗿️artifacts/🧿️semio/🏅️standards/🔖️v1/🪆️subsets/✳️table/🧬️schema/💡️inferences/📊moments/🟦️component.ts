/** 📊 real facet mirror of `ColumnMoments`'s `InferredField::Value`. Keyed by column NAME (the
 * native key — see the Rust sibling's `SemioTableColumn` doc comment). Not yet wired into the
 * parent `SemioTableInference` aggregate's TS surface (see the Rust sibling's own doc comment). */
export interface SemioColumnMoments {
  count: number;
  mean: number;
  variance: number;
  stdDev: number;
}
