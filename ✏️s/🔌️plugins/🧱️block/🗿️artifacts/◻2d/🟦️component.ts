/** 🩻️ Block 2D artifact — the document entity the ◻2d app edits. Mirrors `🦀️component.rs`. */

/** 🔵️ The node's own rim presentation — mirrors `Puzzle2dNode`'s shape fields, minus placement. */
export interface Block2dPresentation {
  shape?: string;
  radius?: number;
  width?: number;
  height?: number;
  color?: string;
  iconKind?: string;
}

/** 🔘️ One handle-kind catalog row this node kind ships with. */
export interface Block2dHandleKind {
  id: string;
  name: string;
  label: string;
  color: string;
  defaultWireKind: string;
}

/** 🌱️ One rim-handle template — where a handle of `handleKind` sits on the node's rim. */
export interface Block2dHandleTemplate {
  id: string;
  handleKind: string;
  angle: number;
  radius: number;
}
