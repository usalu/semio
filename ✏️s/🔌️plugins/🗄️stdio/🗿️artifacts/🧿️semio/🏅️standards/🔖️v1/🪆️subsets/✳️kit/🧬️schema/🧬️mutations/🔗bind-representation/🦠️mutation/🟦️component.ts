export interface BindRepresentation { target: string; pin: { kind: "head" } | { kind: "checkpoint"; id: string } | { kind: "snapshot"; hash: string; size: number; mediaType: string }; role: string; }
