// #region 🧲️Header
/** @emoji 🐚️ Multi-shell dev harness — mounts several independent `FrameworkOsShell` instances on one
 * page (no iframes) so the per-shell scoping work (`ShellScope` and everything built on it) can be
 * smoke-tested wave by wave, ahead of the mit-bestand demonstrator rebuild that depends on it. */
// #endregion 🧲️Header

import "../../🎨️.css";

import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../🔌️plugin/📇️registry/🟦️.ts";
import { FrameworkOsShell } from "@semio-tech/framework-renderer-react";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-js";
import * as React from "react";
import { createRoot } from "react-dom/client";

//#region 🐚️MultiShellHarnessPanes
/** @emoji 🐚️ Two deliberately plain, unbranded playground variants (no shell locks beyond what's set
 * here) — this harness exercises the generic multi-instance mechanism, not any one product's brand. */
const MULTI_HARNESS_PANES = [
  { variant: "cad", shellId: "harness-cad", locale: "en", appearance: "light" },
  { variant: "gis2d", shellId: "harness-gis2d", locale: "de", appearance: "dark" },
] as const;

type MultiHarnessPane = (typeof MULTI_HARNESS_PANES)[number];

/** @emoji 👁️✏️ Boot-time surface role (contract §5), shared by every harness pane — mirrors
 * `🟦️.ts`'s own `VITE_SEMIO_APP_ROLE` resolution so both dev entry points agree on the same
 * default-editor, viewer-on-request rule. */
const MULTI_HARNESS_APP_ROLE: "viewer" | "editor" = import.meta.env.VITE_SEMIO_APP_ROLE === "viewer" ? "viewer" : "editor";
//#endregion 🐚️MultiShellHarnessPanes

function MultiShellHarnessPane({ pane }: { readonly pane: MultiHarnessPane }): React.ReactElement {
  const [mounted, setMounted] = React.useState(true);
  const boot = React.useMemo(() => resolvePlaygroundBoot(PLUGIN_CATALOG, pane.variant), [pane.variant]);
  return (
    <div style={{ display: "flex", flexDirection: "column", flex: "1 1 0", minWidth: 0 }}>
      <div style={{ display: "flex", alignItems: "center", gap: 8, padding: "4px 8px", background: "#111827", color: "#e5e7eb", fontFamily: "monospace", fontSize: 12 }}>
        <span>
          {pane.shellId} · {pane.variant} · {pane.locale} · {pane.appearance}
        </span>
        <button type="button" onClick={() => setMounted((value) => !value)}>
          {mounted ? "unmount" : "mount"}
        </button>
      </div>
      <div style={{ flex: "1 1 0", minHeight: 0, position: "relative" }}>
        {mounted ? (
          <FrameworkOsShell shellId={pane.shellId} storageNamespace={pane.shellId} pluginFilter={boot.variant} plugins={boot.plugins} surfaceSessionFactories={PUZZLE_BOARD_SESSION_FACTORIES} appId={boot.defaultAppId} appRole={MULTI_HARNESS_APP_ROLE} locks={{ locale: pane.locale, appearance: pane.appearance }} />
        ) : null}
      </div>
    </div>
  );
}

function MultiShellHarness(): React.ReactElement {
  return (
    <div style={{ display: "flex", width: "100vw", height: "100vh" }}>
      {MULTI_HARNESS_PANES.map((pane) => (
        <MultiShellHarnessPane key={pane.shellId} pane={pane} />
      ))}
    </div>
  );
}

const root = document.getElementById("root");
if (root && !import.meta.vitest) {
  createRoot(root).render(<MultiShellHarness />);
}
