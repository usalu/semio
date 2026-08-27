/** 🔀 Direct Imperative reorder-steps payload. */
export interface PathRef { owner?: string; slot?: string }
export interface ReorderSteps { pathRef: PathRef; id: string; toIndex: number }
