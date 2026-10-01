/** 🩹️ Relative rewriting `patch-working-nodes` payload mirror of `PatchWorkingNodes`. */
export interface PatchWorkingNodes {
  targets: string[];
  field: "name" | "kind";
  value: string;
}
