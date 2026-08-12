/** 🧬️ SemioBrepMutation facet mirror — real facet mirror of the Rust `🦀️component.rs` sibling.
 * Closed, thirteen-variant dispatch: SMO's approved verb table exactly (`create-loop`/`delete-loop`
 * deliberately absent — see the Rust sibling's module doc comment for why). */
export type SemioBrepMutation =
  | { mutation: "createVertex"; payload: import("../🏗️create-vertex/🦠️mutation/🟦️component.ts").CreateVertex }
  | { mutation: "deleteVertex"; payload: import("../🗑️delete-vertex/🦠️mutation/🟦️component.ts").DeleteVertex }
  | { mutation: "createEdge"; payload: import("../🔗create-edge/🦠️mutation/🟦️component.ts").CreateEdge }
  | { mutation: "deleteEdge"; payload: import("../✂️delete-edge/🦠️mutation/🟦️component.ts").DeleteEdge }
  | { mutation: "createFace"; payload: import("../🔷create-face/🦠️mutation/🟦️component.ts").CreateFace }
  | { mutation: "deleteFace"; payload: import("../🚮delete-face/🦠️mutation/🟦️component.ts").DeleteFace }
  | { mutation: "createShell"; payload: import("../🐚create-shell/🦠️mutation/🟦️component.ts").CreateShell }
  | { mutation: "deleteShell"; payload: import("../💥delete-shell/🦠️mutation/🟦️component.ts").DeleteShell }
  | { mutation: "createSolid"; payload: import("../🧊create-solid/🦠️mutation/🟦️component.ts").CreateSolid }
  | { mutation: "deleteSolid"; payload: import("../🕳️delete-solid/🦠️mutation/🟦️component.ts").DeleteSolid }
  | { mutation: "replaceCurve"; payload: import("../➰replace-curve/🦠️mutation/🟦️component.ts").ReplaceCurve }
  | { mutation: "replaceSurface"; payload: import("../🗺️replace-surface/🦠️mutation/🟦️component.ts").ReplaceSurface }
  | { mutation: "moveVertex"; payload: import("../📍move-vertex/🦠️mutation/🟦️component.ts").MoveVertex };
