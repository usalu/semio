import type { JackGraphWindowConfig } from "../🟦️";

/** 🧬️ A change to the addressed window's persisted local view. */
export type JackGraphWindowConfigMutation =
  | { readonly kind: "set-camera"; readonly camera: JackGraphWindowConfig["camera"] }
  | { readonly kind: "set-lod-mode"; readonly value: string };
