/** 🧾️ Kit validation
 *
 * Deserialization and diagnostics for kit documents.
 */

// #region 🔌️Adapters
import * as vscode from "vscode";
import { COMPOSE_KIT_LANGUAGE } from "../🎞️constants/🟦️.ts";
import { kitDiagnostics } from "../🎩️globals/🟦️.ts";
import { logError } from "../🎼️utilities/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔌️Adapters
// 💿️Problem holds the data fields for a Problem record.
type Problem = { message: string };
// 🔷️deserializeKit holds the data fields for a deserializeKit record.
function deserializeKit(text: string): unknown {
  return JSON.parse(text);
}

// 🔶️validateKit holds the data fields for a validateKit record.
function validateKit(_kit: unknown): { problems: Problem[] } {
  return { problems: [] };
}
// #endregion 🔌️Adapters

// #region 🗺️File Analysis & Diagnostics
/**
 * isKitDocument holds the data fields for a isKitDocument record.
 **/
export function isKitDocument(document: vscode.TextDocument): boolean {
  if (document.languageId !== COMPOSE_KIT_LANGUAGE) return false;
  const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
  return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

/**
 * validateKitDocument holds the data fields for a validateKitDocument record.
 **/
export function validateKitDocument(document: vscode.TextDocument): void {
  if (!isKitDocument(document)) return;
  try {
    const text = document.getText();
    const kit = deserializeKit(text);
    const result = validateKit(kit);
    const diagnostics = result.problems.map((problem: Problem) => {
      return new vscode.Diagnostic(new vscode.Range(0, 0, 0, 0), problem.message);
    });
    kitDiagnostics().set(document.uri, diagnostics);
  } catch (error) {
    logError("Failed to validate compose kit:", error);
    kitDiagnostics().delete(document.uri);
  }
}
// #endregion 🗺️File Analysis & Diagnostics
