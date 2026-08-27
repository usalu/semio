/** 🌱 Direct Imperative create-step payload. */
export interface PathRef { owner?: string; slot?: string }
export interface Step { id: string; kind: string; params: Record<string, unknown>; bodies: Record<string, { steps: Step[] }> }
export interface CreateStep { pathRef: PathRef; step: Step }
