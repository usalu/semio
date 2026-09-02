/** 🏠️ S Home launcher editor — the main window: typed twin of `🦀️.rs`'s virtual-file-system
 * render boundary. Mirrors the framework's own `VirtualFileSystemScene` row shape rather than
 * importing it, matching this taxonomy's per-component TS twin convention (no cross-package TS
 * import). */

/** 🗂️ One VFS row — a studio catalog entry (or the synthetic root row). */
export interface HomeMainVfsRow {
  id: string;
  fileNodeKindId: "studio";
  name: string;
  path: string;
  parentId: string | null;
  hasChildren: boolean;
  navigateUri: string | null;
  descriptorValues: { apps: string };
}

/** 🧱️ The main window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (locale-resolved labels + the live studio catalog listing) and output (a read-only
 * `VirtualFileSystemScene`). */
export interface HomeMainViewModel {
  windowKindId: "s-home-main";
  bodyKey: "s.home.vfs";
  surfaceId: "vfs:home:main";
  rows: HomeMainVfsRow[];
  emptyMessage: string;
  editable: true;
}

export const S_HOME_WINDOW = "s-home-main" as const;
export const S_HOME_BODY = "s.home.vfs" as const;
export const S_HOME_SURFACE = "vfs:home:main" as const;
