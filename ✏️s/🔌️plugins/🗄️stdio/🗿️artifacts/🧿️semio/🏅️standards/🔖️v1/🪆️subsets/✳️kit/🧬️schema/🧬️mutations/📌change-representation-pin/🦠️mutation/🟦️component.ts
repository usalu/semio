export interface ChangeRepresentationPin { index: number; pin: { kind: "head" } | { kind: "checkpoint"; id: string } | { kind: "snapshot"; hash: string; size: number; mediaType: string }; }
