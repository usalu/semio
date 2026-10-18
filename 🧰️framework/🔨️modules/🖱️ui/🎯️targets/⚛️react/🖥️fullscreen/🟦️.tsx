import { reactHostPort } from "../../../🧱️elements/🔌️Ports/🟦️.tsx";
import { resolveElementsSurfaceChromeRoot } from "../🌓️appearance/🟦️.ts";
import { shellNavbarTrailingEndWidthByRoot, shellNavbarTrailingEndWidthListenersByRoot } from "../../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
import { useShellScopeOptional } from "../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import { useLabel } from "../../../🧱️elements/🏷️Label/🟦️.tsx";
import { Toggle } from "../../../🧱️elements/🔀️Toggle/🟦️.tsx";
import { Minimize2Icon, Maximize2Icon, Icon } from "../../../🧱️elements/🔣️Icons/🟦️.tsx";
import * as React from "react";
import { setNavbarTrailingChrome } from "../../../🧱️elements/🔝️Navbar/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { borderNormalBottomClass } from "../../../🔨️modules/📏️border-presentation/🟦️.ts";

// #region 🖥️Fullscreen

const DOCUMENT_FULLSCREEN_CHANGE_EVENTS = ["fullscreenchange", "webkitfullscreenchange"] as const;

type FullscreenDocument = Document & {
  webkitFullscreenElement?: Element | null;
  webkitExitFullscreen?: () => Promise<void> | void;
};

type FullscreenHTMLElement = HTMLElement & {
  webkitRequestFullscreen?: () => Promise<void> | void;
};

/** @emoji 🖥️ Whether the browser document is in fullscreen mode. */
export function readDocumentFullscreenActive(doc: Document = document): boolean {
  const typed = doc as FullscreenDocument;
  return !!(typed.fullscreenElement ?? typed.webkitFullscreenElement);
}

/** @emoji 🖥️ Enter or exit browser fullscreen. `root` is the element requesting fullscreen — a shell's
 * own root (e.g. `ShellScope.rootRef.current`) so going fullscreen from within one embedded shell fills
 * the screen with just that shell's content, not the whole page (other mounted shells included);
 * omitted, defaults to `document.documentElement` (the single-shell-per-page case, unchanged). The
 * Fullscreen API only ever has one fullscreen element at a time regardless of which root requested it. */
export async function toggleDocumentFullscreen(root: Element = document.documentElement): Promise<void> {
  const typedRoot = root as FullscreenHTMLElement;
  const doc = root.ownerDocument;
  const typedDoc = doc as FullscreenDocument;
  if (readDocumentFullscreenActive(doc)) {
    if (typedDoc.exitFullscreen) await typedDoc.exitFullscreen();
    else typedDoc.webkitExitFullscreen?.();
    return;
  }
  if (typedRoot.requestFullscreen) await typedRoot.requestFullscreen();
  else typedRoot.webkitRequestFullscreen?.();
}

/** @emoji 🖥️ Tracks browser fullscreen state for shell chrome. `root` scopes which element requests
 * fullscreen — see {@link toggleDocumentFullscreen}'s doc. */
export function useDocumentFullscreen(root?: Element): { isFullscreen: boolean; toggle: () => void } {
  const [isFullscreen, setIsFullscreen] = reactHostPort.useState(() => (typeof document !== "undefined" ? readDocumentFullscreenActive() : false));

  reactHostPort.useEffect(() => {
    if (typeof document === "undefined") return;
    const sync = () => setIsFullscreen(readDocumentFullscreenActive());
    sync();
    for (const event of DOCUMENT_FULLSCREEN_CHANGE_EVENTS) {
      document.addEventListener(event, sync);
    }
    return () => {
      for (const event of DOCUMENT_FULLSCREEN_CHANGE_EVENTS) {
        document.removeEventListener(event, sync);
      }
    };
  }, []);

  const toggle = reactHostPort.useCallback(() => {
    void toggleDocumentFullscreen(root).catch((error) => console.error("Fullscreen request was rejected", error));
  }, [root]);

  return { isFullscreen, toggle };
}

export function publishShellNavbarTrailingEndWidthPx(root: HTMLElement | undefined, width: number): void {
  const key = resolveElementsSurfaceChromeRoot(root);
  if (!key || width < 0 || width === shellNavbarTrailingEndWidthByRoot.get(key)) return;
  shellNavbarTrailingEndWidthByRoot.set(key, width);
  for (const listener of shellNavbarTrailingEndWidthListenersByRoot.get(key) ?? []) listener();
}

function NavbarFullscreenToggle({ onToggle }: { readonly onToggle?: () => void }) {
  const shellScope = useShellScopeOptional();
  const { isFullscreen, toggle } = useDocumentFullscreen(shellScope?.rootRef.current ?? undefined);
  const enterLabel = useLabel("ui.fullscreen.toggle");
  const exitLabel = useLabel("ui.fullscreen.exit");
  return (
    <Toggle
      id="ui.fullscreen.toggle"
      text={isFullscreen ? exitLabel : enterLabel}
      pressed={isFullscreen}
      onPressedChange={onToggle ?? toggle}
      icon={isFullscreen ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />}
    />
  );
}

/** @emoji 🖥️ Navbar trailing chrome (chat toggle, fullscreen, …) — parks width so center labels do not collapse when panels open. */
export function NavbarTrailingChromeSlot({
  beforeFullscreen,
  showFullscreenToggle = true,
  onFullscreenToggle,
}: {
  readonly beforeFullscreen?: React.ReactNode;
  readonly showFullscreenToggle?: boolean;
  readonly onFullscreenToggle?: () => void;
} = {}) {
  const shellScope = useShellScopeOptional();
  // 🐚️ Resolved at render time (not read from `shellScope.rootRef` inside the effect): the ref object's
  // identity never changes, so a dep array holding the ref itself would never re-fire this effect once
  // the root attaches after this component's own mount commit (shell roots attach bottom-up, after
  // deeply-nested descendants like this one). Reading `.current` here and depending on the resolved
  // element instead picks up the populated root on the guaranteed re-render `FrameworkOsShell` triggers
  // once its own root ref callback fires.
  const root = shellScope?.rootRef.current ?? undefined;
  const shellRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [parkedMinWidth, setParkedMinWidth] = reactHostPort.useState(0);

  reactHostPort.useLayoutEffect(() => {
    const shell = shellRef.current;
    if (!shell) return;
    const measure = () => {
      const width = shell.getBoundingClientRect().width;
      if (width > 0) {
        setParkedMinWidth(width);
        publishShellNavbarTrailingEndWidthPx(root, width);
      }
    };
    measure();
    const resizeObserver = typeof ResizeObserver !== "undefined" ? new ResizeObserver(measure) : null;
    resizeObserver?.observe(shell);
    return () => resizeObserver?.disconnect();
  }, [root]);

  return (
    <div ref={shellRef} key="navbarTrailingChrome" data-slot="navbar-trailing-chrome" className="ms-auto flex h-medium shrink-0 min-w-fit items-center gap-single" style={parkedMinWidth > 0 ? { minWidth: parkedMinWidth } : undefined}>
      {beforeFullscreen}
      {showFullscreenToggle ? <NavbarFullscreenToggle onToggle={onFullscreenToggle} /> : null}
    </div>
  );
}
setNavbarTrailingChrome(NavbarTrailingChromeSlot);
// #endregion 🧪️NavbarExampleSelect

// Draggable Electron-style window title bar with minimize/maximize/close controls.
// Consumers MUST omit `controls` on non-Electron hosts; no control buttons render without it.

/** @emoji 🪟️ Configuration for {@link DesktopTitlebar} window controls. */
export interface DesktopTitlebarProps {
  title: string;
  controls?: { minimize(): void; maximize(): void; close(): void };
  children?: React.ReactNode;
}

/** @emoji 🪟️ Draggable title bar row with a title, extra chrome, and window controls. @see https://www.electronjs.org/docs/latest/tutorial/custom-title-bar */
export function DesktopTitlebar({ title, controls, children }: DesktopTitlebarProps): React.ReactElement {
  const minimizeLabel = useLabel("ui.common.minimize");
  const maximizeLabel = useLabel("ui.common.maximize");
  const closeLabel = useLabel("ui.common.close");
  return (
    <div data-slot="desktop-titlebar" className={cn(borderNormalBottomClass, "flex h-large shrink-0 items-center gap-single px-single")} style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
      <span className="truncate text-sm font-semibold">{title}</span>
      {children}
      {controls ? (
        <div className="ms-auto flex items-center gap-tiny" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button type="button" onClick={controls.minimize} aria-label={minimizeLabel} className="cursor-pointer rounded-sm p-single text-muted-foreground transition-colors hover:bg-hover-window hover:text-foreground">
            <Icon icon="minus" size="tiny" />
          </button>
          <button type="button" onClick={controls.maximize} aria-label={maximizeLabel} className="cursor-pointer rounded-sm p-single text-muted-foreground transition-colors hover:bg-hover-window hover:text-foreground">
            <Icon icon="square" size="tiny" />
          </button>
          <button type="button" onClick={controls.close} aria-label={closeLabel} className="cursor-pointer rounded-sm p-single text-muted-foreground transition-colors hover:bg-destructive-bg hover:text-destructive-foreground">
            <Icon icon="x" size="tiny" />
          </button>
        </div>
      ) : null}
    </div>
  );
}
