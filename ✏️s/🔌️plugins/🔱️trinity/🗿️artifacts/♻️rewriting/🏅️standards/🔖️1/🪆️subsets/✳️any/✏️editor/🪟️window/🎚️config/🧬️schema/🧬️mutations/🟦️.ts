import type { RewritingWindowConfig } from "../🟦️";

/** 🧬️ A change to the addressed window's persisted local view. */
export type RewritingWindowConfigMutation =
  | { readonly kind: "set-camera"; readonly camera: RewritingWindowConfig["camera"] }
  | { readonly kind: "set-lod-mode"; readonly value: string };
