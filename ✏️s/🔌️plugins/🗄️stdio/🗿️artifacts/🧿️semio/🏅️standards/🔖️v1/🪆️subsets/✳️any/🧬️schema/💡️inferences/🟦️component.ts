/** 💡️ Semio envelope inference schema — the wrapped subset's dispatch tag/ordinal. */

export interface SemioKind {
  tag: string;
  ordinal: number;
}

export interface SemioInference {
  /** @state inferred */
  kind: SemioKind;
}
