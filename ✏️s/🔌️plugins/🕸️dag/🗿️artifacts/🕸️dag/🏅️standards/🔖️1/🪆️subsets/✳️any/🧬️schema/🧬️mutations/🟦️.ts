/** 🧩 dag 🧬️mutations WASM facade — mirrors `DagMutation` (see `🦀️.rs`). */

/** 📸️ Structural mirror of the Rust `DagNodeSpec` — only the address/scalar fields this facade
 * needs; `kind`/`properties` cross the WASM boundary as opaque JSON (see `ReplaceNodeKind`/
 * `ReplaceNodeProperties` below). */
export interface DagNodeSpecFacade {
  id: string;
  name: string;
  abbreviation: string;
  icon: string;
  x: number;
  y: number;
  width: number;
  height: number;
  operatorKind?: string;
  /** 🧬️ Opaque JSON — `DagNodeKind` (11-variant tagged enum), decoded by the host. */
  kindJson: string;
  /** 🧬️ Opaque JSON — `PropertyBag`. */
  propertiesJson: string;
}

export type DagMutation =
  | { mutation: "createNode"; node: DagNodeSpecFacade }
  | { mutation: "deleteNode"; id: string }
  | { mutation: "renameNode"; id: string; newId: string }
  | { mutation: "changeNodeName"; id: string; newName: string }
  | { mutation: "moveNode"; id: string; x: number; y: number }
  | { mutation: "resizeNode"; id: string; width: number; height: number }
  | { mutation: "changeNodeIcon"; id: string; newIcon: string }
  | { mutation: "changeNodeAbbreviation"; id: string; newAbbreviation: string }
  | { mutation: "changeNodeOperatorKind"; id: string; newOperatorKind?: string }
  | { mutation: "replaceNodeKind"; id: string; newKindJson: string }
  | { mutation: "replaceNodeProperties"; id: string; newPropertiesJson: string }
  | { mutation: "reorderNodes"; order: string[] }
  | { mutation: "connectNodes"; id: string; source: string; target: string; routeStyle: "bezier" | "sharpSz"; propertiesJson: string }
  | { mutation: "disconnectNodes"; id: string };
