/** 🐳️ Persisted dock arrangement vocabulary shared by the dock layout store and the dock renderers. */

/** 🧭️ The eight anchor ids, mirroring `Anchor` in `🖱️ui/🔨️modules/⚓️panel-anchor` (kept dependency-free of that React module) — shared by every persisted anchor-keyed shape so they cannot drift apart from one another. */
export type PersistedAnchor = "top-left" | "top-middle" | "top-right" | "right-middle" | "bottom-right" | "bottom-middle" | "bottom-left" | "left-middle";

/** 🐳️ One tab (leaf or branch) in a persisted dock panel-arrangement tree; leaves carry `trees`, branches carry `children`. */
export interface DockTabSkeleton {
  id: string;
  children?: readonly DockTabSkeleton[];
  trees?: readonly string[];
}

/** 🐳️ The full persisted dock arrangement, one tab tree per anchor. */
export interface DockSkeleton {
  version: 3;
  anchors: Record<PersistedAnchor, readonly DockTabSkeleton[]>;
}
