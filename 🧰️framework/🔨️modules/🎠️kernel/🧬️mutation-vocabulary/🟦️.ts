/** 🧬️ The mutation vocabulary an invocation answers with: logical timestamps, artifact diffs, undo policy, inverse and kernel mutations, edit references, undo groups and app events.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region InvocationResponse
/** 🕰️ Exact replication HLC carried by every packed kernel operation. */
export type HybridLogicalTimestamp = { readonly actor: number; readonly physical_ms: number; readonly logical: number };

/** 🩹️ A schema-tagged artifact mutation payload (forward diff or inverse diff). */
export type ArtifactDiff = { readonly schema: string; readonly payload: readonly number[] };

/** ↩️ Undo semantics for a single kernel operation. */
export type UndoPolicy = "ExactBaseOnly" | "TransformAgainstConcurrent" | "SemanticUndo" | "CompensatingAction";

/** 🗄️ The true inverse of a kernel operation, recorded from the store's `Edit.backwards`. */
export type InverseMutation = {
  readonly targetMutation: string;
  readonly inverseDiff: ArtifactDiff;
  readonly baseVersion: number;
  readonly dependencies?: readonly string[];
  readonly undoPolicy: UndoPolicy;
};

/** 🔁️ One typed document operation with its true inverse — the CQRS wire unit. */
export type KernelMutation = {
  readonly id: string;
  readonly document: string;
  readonly baseVersion: number;
  readonly invocationId: string;
  readonly diff: ArtifactDiff;
  readonly inverse: InverseMutation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** 🧩️ One member edit folded into a group undo — pairs the owning document handle with the
 * edit id inside it (composite/child-document dispatch). Mirrors Rust `kernel::EditRef`. */
export type EditRef = {
  readonly document: string;
  readonly editId: string;
};

/** 🎁️ The undo group binding an invocation (action or command) to its operations + inverses. */
export type UndoGroup = {
  readonly invocationId: string;
  readonly mutations: readonly string[];
  readonly inverseMutations: readonly InverseMutation[];
  readonly memberEdits?: readonly EditRef[];
};

/** 📣️ An out-of-band app event surfaced to the shell (e.g. history changed). */
export type AppEvent = { readonly kind: string; readonly payload: unknown };
//#endregion InvocationResponse
// #endregion 🎠️Kernel
