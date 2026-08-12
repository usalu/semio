/** 🧬️ SemioKitMutation dispatch — real facet mirror. Fifteen variants covering both composition
 * primitives (create/delete for owned children, bind/unbind/change for the one LINK slot) and
 * domain vocabulary (add/remove/rename for types, add/remove/edit for designs). */
export type SemioKitMutation =
  | { mutation: "createObject"; payload: { childId: string; target: string } }
  | { mutation: "deleteObject"; payload: { childId: string } }
  | { mutation: "createModel"; payload: { childId: string; target: string } }
  | { mutation: "deleteModel"; payload: { childId: string } }
  | { mutation: "createProperties"; payload: { childId: string; target: string } }
  | { mutation: "deleteProperties"; payload: Record<string, never> }
  | { mutation: "bindRepresentation"; payload: { target: string; pin: unknown; role: string } }
  | { mutation: "unbindRepresentation"; payload: { index: number } }
  | { mutation: "changeRepresentationPin"; payload: { index: number; pin: unknown } }
  | { mutation: "addType"; payload: { id: string; name: string; category: string } }
  | { mutation: "removeType"; payload: { id: string } }
  | { mutation: "renameType"; payload: { id: string; newName: string } }
  | { mutation: "addDesign"; payload: { id: string; name: string } }
  | { mutation: "removeDesign"; payload: { id: string } }
  | { mutation: "editDesign"; payload: { id: string; pieces: unknown[]; connections: unknown[] } };
