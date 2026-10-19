/** 🎩️ Extension globals
 *
 * Output channel, diagnostic collections, caches and the CLI concurrency gate.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import * as vscode from "vscode";
// #endregion 🔌️Adapters

// #region 🎩️Globals
// 🔌️Globals MUST hold module-level state for output channel, diagnostics, caches, and providers.
export const outputChannel = ephemeralBox<vscode.OutputChannel | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.outputChannel", undefined);
/**
 * 🎒️ kitDiagnosticCollection.current holds the data fields for a kitDiagnosticCollection.current record.
 **/
export const kitDiagnosticCollection = ephemeralBox<vscode.DiagnosticCollection | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.kitDiagnosticCollection", undefined);

/** 🧰️ The compose-kit diagnostic collection, created by `activate` for local language editing. */
export function kitDiagnostics(): vscode.DiagnosticCollection {
  if (!kitDiagnosticCollection.current) throw new Error("compose-kit diagnostics were read before the extension activated");
  return kitDiagnosticCollection.current;
}
/**
 * 🗜️ BundleInfo holds the data fields for a BundleInfo record.
 **/
export interface BundleInfo {
  id: string;
  root: string;
}
/**
 * 📦️ bundleCache.current holds the data fields for a bundleCache.current record.
 **/
export const bundleCache = ephemeralBox<BundleInfo[]>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.bundleCache", []);
/**
 * 🗄️ cachedRepoBaseUrl.current holds the data fields for a cachedRepoBaseUrl.current record.
 **/
export const cachedRepoBaseUrl = ephemeralBox<string | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.cachedRepoBaseUrl", undefined);
// 🔢️Maximum number of concurrent CLI subprocess spawns to prevent system overload.
const CLI_CONCURRENCY_LIMIT = 2;
const cliActiveCount = ephemeralBox("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.cliActiveCount", 0);
const cliWaitQueue: (() => void)[] = [];

export function acquireCliSlot(): Promise<void> {
  if (cliActiveCount.current < CLI_CONCURRENCY_LIMIT) {
    cliActiveCount.current++;
    return Promise.resolve();
  }
  return new Promise<void>((resolve) => {
    cliWaitQueue.push(() => {
      cliActiveCount.current++;
      resolve();
    });
  });
}

export function releaseCliSlot(): void {
  cliActiveCount.current--;
  const next = cliWaitQueue.shift();
  if (next) next();
}
// #endregion 🎩️Globals
