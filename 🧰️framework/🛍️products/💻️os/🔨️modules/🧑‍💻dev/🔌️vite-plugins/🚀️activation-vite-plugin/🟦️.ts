/** 🚀️ The plugin activation Vite plugin of the dev server. This shell is config-shaped: the receipt observation, freshness report and plugin rebuild runtime loads at server-configure time through one opaque dynamic import, so the dev server's native config parse and watch set stays small (see the config-graph fixture). */
import type { ActivationReceipt } from "../../♻️activation/🟦️.ts";
import type { ActivationServer } from "./🏭️runtime/🟦️.ts";
import type { ActivationVitePluginOptions } from "./🧬️options/🟦️.ts";

//#region 🔌️PluginActivationVitePlugin


const loadRuntime = (): Promise<typeof import("./🏭️runtime/🟦️.ts")> => import(new URL("./🏭️runtime/🟦️.ts", import.meta.url).href);

/** 🔎️ Re-runs the staged-module freshness rule against the receipt the dev server just observed and resolves to one `[stale]` line per component whose served bytes are behind; `signal` cancels a pass a newer receipt superseded. */
export async function reportActivationFreshness(receipt: ActivationReceipt, options: Omit<ActivationVitePluginOptions, "receiptDirectory"> & { readonly signal?: AbortSignal }): Promise<readonly string[]> {
  return (await loadRuntime()).reportActivationFreshness(receipt, options);
}

/** 📡️ Announces explicit build activation completion and releases every server-owned subscription. */
export function semioActivationVitePlugin(options: ActivationVitePluginOptions) {
  let dispose = (): void => {};
  let staleness: readonly string[] = [];
  return {
    name: "semio-activation",
    enforce: "pre" as const,
    /** 📣️ The staged-module verdict belongs in the DEVELOPER's console, not only in the server log
     * they are not reading: a guest module staged behind its own source serves a wire contract the host
     * TypeScript in the same page no longer speaks, and the symptom (`actor-ui-patch.pairing`, a window
     * booting a fallback graph) never names its cause. */
    transformIndexHtml: {
      order: "post" as const,
      handler() {
        if (staleness.length === 0) return [];
        const message = `semio dev · ${staleness.length} staged plugin module(s) are behind their source — the host in this page may speak a newer wire contract than the guest it is talking to:\n${staleness.join("\n")}`;
        return [{ tag: "script", attrs: { type: "module" }, children: `console.warn(${JSON.stringify(message)});` }];
      },
    },
    async configureServer(server: ActivationServer) {
      dispose();
      dispose = (await loadRuntime()).activateServer(server, options, (lines) => { staleness = lines; });
    },
    closeBundle(): void { dispose(); },
  };
}
//#endregion 🔌️PluginActivationVitePlugin
