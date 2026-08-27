/** 🔧 Direct Imperative edit-step-params payload. */
export interface PathRef { owner?: string; slot?: string }
export interface EditStepParams { pathRef: PathRef; id: string; newParams: Record<string, unknown> }
