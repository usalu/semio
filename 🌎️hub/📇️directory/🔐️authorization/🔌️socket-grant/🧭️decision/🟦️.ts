export type DirectorySocketScope = Readonly<{ spaceId: string; documentId: string }>;
export type DirectorySocketMessage = Readonly<{ class: string; scope: DirectorySocketScope | null }>;
export type DirectorySocketGrantVector = Readonly<{
  grantScope: DirectorySocketScope;
  urlScope: DirectorySocketScope;
  binding: "active" | "unauthorized" | "unavailable";
  descriptor: boolean;
  live: boolean;
  message: DirectorySocketMessage;
  gateWinner: "send" | "removal" | "neither";
}>;
export type DirectorySocketGrantDecision = Readonly<{ outcome: "deliver" | "skip-unrelated" | "close-unauthorized" | "close-unavailable" | "deny-before-upgrade"; closeCode: number | null; cursorAdvance: boolean; textFrames: number }>;

const scopedClasses = new Set(["document-announced", "checkpoint", "retention", "rebootstrap", "presence", "connection"]);
const cursorClasses = new Set(["document-announced", "checkpoint", "retention"]);

/** 🔌️ Decides one socket-grant delivery from its exact scope and live membership state. */
export function directorySocketGrantDecision(vector: DirectorySocketGrantVector): DirectorySocketGrantDecision {
  const sameScope = (left: DirectorySocketScope, right: DirectorySocketScope): boolean => left.spaceId === right.spaceId && left.documentId === right.documentId;
  if (!sameScope(vector.grantScope, vector.urlScope)) return { outcome: "deny-before-upgrade", closeCode: 4401, cursorAdvance: false, textFrames: 0 };
  if (vector.gateWinner === "removal" || vector.binding === "unauthorized" || !vector.descriptor || !vector.live) return { outcome: "close-unauthorized", closeCode: 4401, cursorAdvance: false, textFrames: 0 };
  if (vector.binding === "unavailable") return { outcome: "close-unavailable", closeCode: 1013, cursorAdvance: false, textFrames: 0 };
  if (!scopedClasses.has(vector.message.class) || vector.message.scope === null || !sameScope(vector.grantScope, vector.message.scope)) return { outcome: "skip-unrelated", closeCode: null, cursorAdvance: false, textFrames: 0 };
  return { outcome: "deliver", closeCode: null, cursorAdvance: cursorClasses.has(vector.message.class), textFrames: 1 };
}
