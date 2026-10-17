/** 🪟️ Desktop window chrome: the window control channels, their handler registration and the renderer bridge. */

//#region 🔖️DesktopWindowChrome
/** 🪟️ IPC channel names for the desktop window chrome controls (minimize/maximize/close) — shared literal between a host's `ipcMain.handle` registration and the renderer's `invoke` bridge. */
export const DESKTOP_WINDOW_CONTROL_CHANNELS = {
  minimize: "framework.window.minimize",
  maximize: "framework.window.maximize",
  close: "framework.window.close",
} as const;

/** 🎛️ Renderer-facing surface for the three desktop window chrome controls. */
export type DesktopWindowControls = { minimize(): Promise<unknown>; maximize(): Promise<unknown>; close(): Promise<unknown> };

/**
 * 🔌️ Registers host-side handlers for {@link DESKTOP_WINDOW_CONTROL_CHANNELS} against a structural
 * `ipc.handle`-shaped port — no `electron` types leak into this signature; a real Electron app wires
 * its `ipcMain`/`BrowserWindow` in at the call site. `maximize` toggles based on `isMaximized()`;
 * a null `focusedWindow()` is a no-operation.
 */
export function registerDesktopWindowControlHandlers(
  ipc: { handle(channel: string, fn: () => void): void },
  focusedWindow: () => { minimize(): void; isMaximized(): boolean; maximize(): void; unmaximize(): void; close(): void } | null,
): void {
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.minimize, () => {
    focusedWindow()?.minimize();
  });
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.maximize, () => {
    const window = focusedWindow();
    if (!window) return;
    if (window.isMaximized()) window.unmaximize();
    else window.maximize();
  });
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.close, () => {
    focusedWindow()?.close();
  });
}

/** 🌉️ Renderer-side {@link DesktopWindowControls} backed by a structural `invoke`-shaped port (e.g. `electron`'s `ipcRenderer.invoke`). */
export function desktopWindowControlsBridge(invoke: (channel: string) => Promise<unknown>): DesktopWindowControls {
  return {
    minimize: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.minimize),
    maximize: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.maximize),
    close: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.close),
  };
}
//#endregion 🔖️DesktopWindowChrome
