// #region 🧲️Header
/** @emoji ♻️ Keeps play's merged activation receipt current while its dev server runs. Node-only: imported
 * by `🏗️builder/🌐️vite/🟦️.ts`, never by the browser-shared runtime module. */
// #endregion 🧲️Header

import type { Plugin } from "vite";
import { observeActivationReceipts } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { playActivationLaneReceiptDirectories, publishPlayUnionReceipt } from "../🟦️.ts";

/** @emoji 👀️ Republishes the union receipt whenever ANY lane completes a new activation. Registered BEFORE
 * `semioActivationVitePlugin` so the union is fresh before that plugin's first snapshot. A failed merge is
 * logged, never thrown: a lane can be mid-rebuild and disagree with its peers for a moment. */
export function playUnionReceiptVitePlugin(options: { readonly workspace: string }): Plugin {
  const observers: { close: () => void }[] = [];
  const report = (error: unknown): void => console.error(`[play] union activation receipt refused: ${error instanceof Error ? error.message : String(error)}`);
  const dispose = (): void => { while (observers.length > 0) { try { observers.pop()!.close(); } catch { /* already torn down by its own error */ } } };
  return {
    name: "play-union-activation-receipt",
    apply: "serve",
    configureServer(server) {
      const republish = (): void => { try { publishPlayUnionReceipt(options.workspace); } catch (error) { report(error); } };
      for (const directory of playActivationLaneReceiptDirectories(options.workspace)) {
        try { observers.push(observeActivationReceipts(directory, republish, report)); } catch (error) { report(error); }
      }
      server.httpServer?.once("close", dispose);
    },
    closeBundle: dispose,
  };
}
