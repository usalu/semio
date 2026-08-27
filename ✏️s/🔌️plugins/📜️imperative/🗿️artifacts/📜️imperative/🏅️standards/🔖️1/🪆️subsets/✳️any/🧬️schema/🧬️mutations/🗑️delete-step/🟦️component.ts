/** 🗑️ Direct Imperative delete-step payload. */
export interface PathRef { owner?: string; slot?: string }
export interface DeleteStep { pathRef: PathRef; id: string }
