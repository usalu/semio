/** 👯️ Block 5D artifact — the document entity the 🖐️5d app edits. Mirrors `🦀️component.rs`. */

/** 🔵️ The part's 2D-projection presentation (board node). */
export interface Block5dPart2d {
  shape?: string;
  radius?: number;
  width?: number;
  height?: number;
  color?: string;
  iconKind?: string;
}

/** 🧱️ The part's 3D-projection presentation (world object) — pose defaults only. */
export interface Block5dPart3d {
  orientation?: [number, number, number, number];
  scale?: [number, number, number];
}

/** 🔘️ One grip-kind catalog row this part kind ships with. */
export interface Block5dGripKind {
  id: string;
  name: string;
  label: string;
  color: string;
  defaultRopeKind: string;
}

/** 🌱️ One rim-grip template, unified across both projections — flat scalar fields. */
export interface Block5dGripTemplate {
  id: string;
  gripKind: string;
  angle: number;
  radius2d: number;
  position: [number, number, number];
  direction: [number, number, number];
  radius3d: number;
}
