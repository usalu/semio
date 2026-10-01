import React from "react";
import { createRoot } from "react-dom/client";
import { FrameworkOsShell } from "../../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import { EMPTY_SHELL_LOCKS } from "../../../🧱️elements/🐚️Shell/🟦️.tsx";
import type { AppSurfaceSessionFactory } from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { activeShellRoot } from "@semio-tech/ui-react";

type EmbeddedReactOptions = {
  readonly rootId: string;
  readonly plugin: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly locale: "en" | "de";
  readonly suppressAutoIntroduction: boolean;
  readonly sessionFactoryModules: readonly { readonly moduleUrl: string; readonly exportName: string }[];
};
type EmbeddedReactMount = (() => Promise<void>) & { readonly setIntroductionSuppressed: (suppressed: boolean) => void };
const mounted = new WeakMap<HTMLElement, EmbeddedReactMount>();

/** 🎯️ Reads the real public shell activity authority used for body-targeted shortcuts. */
export function embeddedReactFocusOwner(): string | null {
  return activeShellRoot()?.closest('[id^="shell-"]')?.id ?? null;
}

/** 🪆️ Mounts the actual scoped React shell as the browser acceptance oracle. */
export async function mountEmbeddedReactOracle(options: EmbeddedReactOptions): Promise<EmbeddedReactMount> {
  const element = document.getElementById(options.rootId);
  if (!element) throw new Error("missing embedded React root");
  await mounted.get(element)?.();
  const factories: AppSurfaceSessionFactory[] = [];
  for (const source of options.sessionFactoryModules) {
    const exports = await import(source.moduleUrl);
    const values: unknown = exports[source.exportName];
    if (!Array.isArray(values) || values.length > 64) throw new Error("invalid embedded React session factory provision");
    factories.push(...values);
  }
  const root = createRoot(element);
  let retired = false;
  const render = (suppressed: boolean) => {
    if (retired) throw new Error("react-mount-retired");
    root.render(<FrameworkOsShell shellId={options.rootId} storageNamespace={options.rootId} ownsPage={false} plugins={options.plugins} pluginFilter={options.plugin} surfaceSessionFactories={factories} locks={{ ...EMPTY_SHELL_LOCKS, locale: options.locale }} suppressAutoIntroduction={suppressed} />);
  };
  const dispose = Object.assign(async () => {
    if (retired) return;
    retired = true;
    root.unmount();
    if (mounted.get(element) === dispose) mounted.delete(element);
  }, { setIntroductionSuppressed: render });
  mounted.set(element, dispose);
  render(options.suppressAutoIntroduction);
  return dispose;
}
