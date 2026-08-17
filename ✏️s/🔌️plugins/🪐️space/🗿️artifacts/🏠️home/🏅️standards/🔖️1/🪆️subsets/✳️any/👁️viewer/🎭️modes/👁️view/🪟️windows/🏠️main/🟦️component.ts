/** 🏠️ S Home viewer — the main window: typed twin of `🦀️component.rs`'s read-only virtual-file-system
 * render boundary. Mirrors the framework's own `VirtualFileSystemScene` row shape rather than
 * importing it, matching this taxonomy's per-component TS twin convention (no cross-package TS
 * import). No mutation-shaped fields (no row commands, no drag/drop payloads), matching the viewer's
 * `ViewEmit`-only contract. */

/** 🗂️ One read-only VFS row — a studio catalog entry (or the synthetic root row). */
export interface HomeViewMainVfsRow {
  id: string;
  fileNodeKindId: "studio";
  name: string;
  path: string;
  parentId: string | null;
  hasChildren: boolean;
  navigateUri: string | null;
  descriptorValues: { apps: string };
}

/** 🧱️ The main window's typed view-model — the TS mirror of the Rust `render()` boundary's input (the
 * live studio catalog listing, no per-session state: a viewer has none) and output (a non-editable
 * `VirtualFileSystemScene`). */
export interface HomeViewMainViewModel {
  windowKindId: "s-home-view-main";
  bodyKey: "s.home.view.vfs";
  surfaceId: "vfs:home:view:main";
  rows: HomeViewMainVfsRow[];
  emptyMessage: string;
  editable: false;
}

export const S_HOME_VIEW_WINDOW = "s-home-view-main" as const;
export const S_HOME_VIEW_BODY = "s.home.view.vfs" as const;
export const S_HOME_VIEW_SURFACE = "vfs:home:view:main" as const;
