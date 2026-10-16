/** 🎩️ Extension globals
 *
 * Output channel, diagnostic collections, caches and the CLI concurrency gate.
 */

// #region 🔌️Adapters
import { ephemeralBox, ephemeralMap } from "@semio-tech/framework";
import * as vscode from "vscode";
import { type Breach } from "../⚙️data-models/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎩️Globals
// 🔌️Globals MUST hold module-level state for output channel, diagnostics, caches, and providers.
export const outputChannel = ephemeralBox<vscode.OutputChannel | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.outputChannel", undefined);
/**
 * repoDiagnosticCollection.current holds the data fields for a repoDiagnosticCollection.current record.
 **/
export const repoDiagnosticCollection = ephemeralBox<vscode.DiagnosticCollection | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.repoDiagnosticCollection", undefined);
/**
 * kitDiagnosticCollection.current holds the data fields for a kitDiagnosticCollection.current record.
 **/
export const kitDiagnosticCollection = ephemeralBox<vscode.DiagnosticCollection | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.kitDiagnosticCollection", undefined);
/**
 * fileBreachsMap holds the data fields for a fileBreachsMap record.
 **/
/** 🩺️ The repository diagnostic collection, which `activate` creates — reading it earlier is a lifecycle bug, not an empty collection. */
export function repoDiagnostics(): vscode.DiagnosticCollection {
  if (!repoDiagnosticCollection.current) throw new Error("repo diagnostics were read before the extension activated");
  return repoDiagnosticCollection.current;
}

/** 🧰️ The compose-kit diagnostic collection, created by `activate` alongside [[repoDiagnostics]]. */
export function kitDiagnostics(): vscode.DiagnosticCollection {
  if (!kitDiagnosticCollection.current) throw new Error("compose-kit diagnostics were read before the extension activated");
  return kitDiagnosticCollection.current;
}

export const fileBreachsMap = ephemeralMap<string, Breach[]>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.fileBreachsMap");
/**
 * BundleInfo holds the data fields for a BundleInfo record.
 **/
export interface BundleInfo {
  id: string;
  root: string;
}
/**
 * bundleCache.current holds the data fields for a bundleCache.current record.
 **/
export const bundleCache = ephemeralBox<BundleInfo[]>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.bundleCache", []);
/**
 * cachedRepoBaseUrl.current holds the data fields for a cachedRepoBaseUrl.current record.
 **/
export const cachedRepoBaseUrl = ephemeralBox<string | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.cachedRepoBaseUrl", undefined);
/**
 * runningProcesses holds the data fields for a runningProcesses record.
 **/
export const runningProcesses = ephemeralMap<string, AbortController>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.runningProcesses");
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
