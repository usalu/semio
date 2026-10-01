import { readFileSync, realpathSync } from "node:fs";
import { isAbsolute, relative, resolve } from "node:path";

/** 🧬️ Node preparation admits only explicitly selected owner's schema-first factory declarations. */
export function declaredBrowserSessionEnginesV1(workspace, contributionPath) {
  if (contributionPath === undefined) return [];
  if (typeof contributionPath !== "string" || !contributionPath || contributionPath.split("/").some(part => !part || part === "." || part === "..") || /[\\:\x00-\x1f]/u.test(contributionPath)) throw Error("Invalid browser contribution path");
  const root = realpathSync(workspace), file = realpathSync(resolve(root, contributionPath));
  const local = relative(root, file);
  if (isAbsolute(local) || local === ".." || local.startsWith("../") || local.startsWith("..\\")) throw Error("Browser contribution escapes workspace");
  const contribution = JSON.parse(readFileSync(file, "utf8"));
  const rows = contribution.browserSessionFactories;
  if (rows === undefined) return [];
  if (!Array.isArray(rows)) throw Error("Browser session contributions must be an array");
  const engines = new Set(), identities = new Set();
  for (const row of rows) {
    if (!row || typeof row !== "object" || Array.isArray(row) || Object.keys(row).length !== 2 || !Object.keys(row).every(key => ["module", "engine"].includes(key))) throw Error("Invalid browser session contribution");
    if (typeof row.module !== "string" || !/^(?:@[a-z0-9-]+\/)?[a-z0-9][a-z0-9._-]*$/.test(row.module) || typeof row.engine !== "string" || !/^\.\/(?!.*(?:^|\/)\.{1,2}(?:\/|$))(?!.*[\\:\x00-\x1f])[^/]+(?:\/[^/]+)+$/.test(row.engine)) throw Error("Invalid browser session contribution path");
    const identity = row.module + "\0" + row.engine;
    if (identities.has(identity)) throw Error("Duplicate browser session contribution");
    identities.add(identity); engines.add(row.engine);
  }
  return [...engines];
}
