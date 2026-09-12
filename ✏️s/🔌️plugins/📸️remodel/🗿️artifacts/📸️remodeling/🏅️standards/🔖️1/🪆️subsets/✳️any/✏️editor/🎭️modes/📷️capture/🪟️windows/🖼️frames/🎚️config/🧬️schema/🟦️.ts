/** 🧬️ Remodeling frame cursor. */
export interface RemodelingFrameCursor {
  streamId?: string | null;
  frameIndex: number;
}

/** 🧬️ Exact Remodeling Frames window configuration. */
export interface RemodelingFramesWindowConfig {
  frameCursor: RemodelingFrameCursor;
}

/** 🧬️ Atomic replacement of one exact Frames window configuration. */
export type RemodelingFramesWindowConfigMutation = { kind: "snapshot"; config: RemodelingFramesWindowConfig };

/** 🔁️ Applies one Frames window configuration mutation. */
export const applyRemodelingFramesWindowConfigMutation = (_base: RemodelingFramesWindowConfig, mutation: RemodelingFramesWindowConfigMutation): RemodelingFramesWindowConfig => structuredClone(mutation.config);
