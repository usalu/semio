/** 🧬️ SemioGraphMutation schema — real facet mirror of the Rust `🦀️component.rs` sibling. Closed,
 * eleven-variant dispatch: one interface per triad payload, tagged by `mutation`. */
export type SemioGraphMutation =
  | { mutation: "createNode"; payload: import("../🏗️create-node/🦠️mutation/🟦️component.ts").CreateNode }
  | { mutation: "deleteNode"; payload: import("../🗑️delete-node/🦠️mutation/🟦️component.ts").DeleteNode }
  | { mutation: "changeNodeKind"; payload: import("../🔧change-node-kind/🦠️mutation/🟦️component.ts").ChangeNodeKind }
  | { mutation: "changeNodeLabel"; payload: import("../🖍️change-node-label/🦠️mutation/🟦️component.ts").ChangeNodeLabel }
  | { mutation: "moveNode"; payload: import("../📍move-node/🦠️mutation/🟦️component.ts").MoveNode }
  | { mutation: "addNodePort"; payload: import("../🔌add-node-port/🦠️mutation/🟦️component.ts").AddNodePort }
  | { mutation: "removeNodePort"; payload: import("../🔚remove-node-port/🦠️mutation/🟦️component.ts").RemoveNodePort }
  | { mutation: "addNodeProperty"; payload: import("../➕add-node-property/🦠️mutation/🟦️component.ts").AddNodeProperty }
  | { mutation: "removeNodeProperty"; payload: import("../➖remove-node-property/🦠️mutation/🟦️component.ts").RemoveNodeProperty }
  | { mutation: "createEdge"; payload: import("../🔗create-edge/🦠️mutation/🟦️component.ts").CreateEdge }
  | { mutation: "deleteEdge"; payload: import("../✂️delete-edge/🦠️mutation/🟦️component.ts").DeleteEdge };
