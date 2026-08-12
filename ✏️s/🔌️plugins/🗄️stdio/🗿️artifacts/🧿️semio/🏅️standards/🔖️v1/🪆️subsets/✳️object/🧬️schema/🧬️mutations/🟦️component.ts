/** 🧬️ SemioObjectMutation dispatch — real facet mirror. Nine variants: move/rotate/scale (domain
 * transform verbs) plus create/delete pairs for the three CHILD slots (brep/mesh/properties). */
export type SemioObjectMutation =
  | { mutation: "moveObject"; payload: { translation: { x: number; y: number; z: number } } }
  | { mutation: "rotateObject"; payload: { rotation: { x: number; y: number; z: number; w: number } } }
  | { mutation: "scaleObject"; payload: { scale: { x: number; y: number; z: number } } }
  | { mutation: "createBrep"; payload: { childId: string; target: string } }
  | { mutation: "deleteBrep"; payload: Record<string, never> }
  | { mutation: "createMesh"; payload: { childId: string; target: string } }
  | { mutation: "deleteMesh"; payload: Record<string, never> }
  | { mutation: "createProperties"; payload: { childId: string; target: string } }
  | { mutation: "deleteProperties"; payload: Record<string, never> };
