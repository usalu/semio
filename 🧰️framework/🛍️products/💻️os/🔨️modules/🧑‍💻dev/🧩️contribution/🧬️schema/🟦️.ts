import { parseBrowserSessionFactoriesV1, type BrowserSessionFactoryContributionV1 } from "../../⚙️engine/🧭️selection/🟦️.ts";
import { parseDevLocalHubProviderV1, type DevLocalHubProviderV1 } from "../../🚀️local-hub/🧬️schema/🟦️.ts";

/** 🧩️ An application's explicit development entry points and service provider. */
export type DevContribution = { readonly ownerRoot: string; readonly viteConfig: string; readonly browserEntry: string; readonly browserSessionFactories?: readonly BrowserSessionFactoryContributionV1[]; readonly localHub?: DevLocalHubProviderV1 };

/** 📍️ Accepts canonical workspace paths without platform-specific escape syntax. */
export function ownedDevPath(value: unknown): string {
  if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /^[A-Za-z]:/.test(value) || /[\\\x00-\x1f]/.test(value) || value.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("Invalid development owner path");
  return value;
}

/** 🧬️ Validates contributions before a build, browser entry, or provider can be selected. */
export function parseDevContribution(value: unknown): DevContribution {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid development contribution");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !["ownerRoot", "viteConfig", "browserEntry", "localHub", "browserSessionFactories"].includes(key))) throw new Error("Unknown development contribution field");
  const ownerRoot = ownedDevPath(row.ownerRoot), viteConfig = ownedDevPath(row.viteConfig), browserEntry = ownedDevPath(row.browserEntry);
  if (![viteConfig, browserEntry].every((path) => path.startsWith(ownerRoot + "/"))) throw new Error("Development entry escapes its owner");
  return { ownerRoot, viteConfig, browserEntry, ...(row.browserSessionFactories === undefined ? {} : { browserSessionFactories: parseBrowserSessionFactoriesV1(row.browserSessionFactories) }), ...(row.localHub === undefined ? {} : { localHub: parseDevLocalHubProviderV1(row.localHub) }) };
}

