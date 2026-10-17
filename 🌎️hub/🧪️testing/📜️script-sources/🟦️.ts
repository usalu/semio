/** 📜️ The sources of the hub package script: its entry and every module it dispatches to, for the gates that read hub command source text.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

/** 🗂️ Every module that holds hub script code, relative to the `🌎️hub` directory; each one starts with a docstring that points back to the script. */
export const HUB_SCRIPT_MODULES: readonly string[] = [
  "🧪️testing/📖️source-readers",
  "🎛️commands/🚦️gate-commands",
  "🚀️local-relay/🛰️servers",
  "🚀️local-bootstrap/🧰️mcp-process-support",
  "🧪️tests/🪪️check-in-process",
  "🧪️tests/🔐️secure-local-smoke",
  "🧪️tests/🌐️relay-checks",
  "🧪️tests/📄️browser-document-open",
  "🧪️tests/🛡️admin-relay-checks",
  "🧪️tests/🔑️directory-authority-checks",
  "🧪️tests/🗺️open-plan-checks",
  "🧪️tests/📇️native-catalog-checks",
  "🧪️tests/🔗️execution-target-lease-checks",
  "🧪️tests/👤️browser-actor-checks",
  "🧪️tests/💡️inference-fixtures",
  "🧪️tests/🌍️gis-map-fixtures",
  "🧪️tests/🏛️trusted-catalog-fixtures",
  "🧪️tests/📮️trusted-publication-fixtures",
  "🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection",
  "🏗️bootstrap/🧾️provenance/🌍️gis-browser-proof",
  "🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims",
  "🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle",
  "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle",
  "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🗺️gis-proofs-and-staging",
  "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🔄️rotation-and-current-pointer",
  "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/📮️receipts-and-publication-child",
  "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication",
  "🧪️tests/🎮️gis-map-process-support",
  "🧪️tests/🕹️gis-map-process",
  "🎛️commands/🏷️trusted-catalog",
  "🧪️tests/🎁️trusted-stdio-gis-bundle-check",
  "🎛️commands/🏗️build-and-dev",
  "🧪️tests/📣️directory-live-support",
  "🧪️tests/📰️event-page-checks",
  "🧪️tests/🏠️home-browser-checks",
  "🧪️tests/🧾️command-receipt-checks",
  "🧪️tests/🎫️invite-redemption-checks",
  "🧪️tests/🛂️space-administration-checks",
  "🧪️tests/🧬️space-artifact-creation-checks",
  "🧪️tests/🧭️space-journey-checks",
];

/** 📖️ Reads one hub script module with the `export` keyword stripped from its top-level declarations, so source markers stay stable when a declaration is shared. A module split into child folders reads as the children listed under it in {@link HUB_SCRIPT_MODULES} followed by the module itself, which keeps the declarations in the order the unsplit module held them. */
export function readHubScriptModule(repoRoot: string, directory: string): string {
  const parts = [...HUB_SCRIPT_MODULES.filter((module) => module.startsWith(`${directory}/`)), directory];
  return parts.map((module) => readFileSync(join(repoRoot, "🌎️hub", module, "🟦️.ts"), "utf8")).join("\n").replace(/\nexport /gu, "\n");
}

/** 📚️ The absolute paths of the hub script entry and of all its modules. */
export function hubScriptSourceFiles(repoRoot: string): readonly string[] {
  return [join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), ...HUB_SCRIPT_MODULES.map((directory) => join(repoRoot, "🌎️hub", directory, "🟦️.ts"))];
}

/** 🧾️ The entry and every module concatenated in listing order, for readers that scan the whole script for a declaration or a call. */
export function readHubScriptSource(repoRoot: string): string {
  return hubScriptSourceFiles(repoRoot).map((file) => readFileSync(file, "utf8")).join("\n");
}
