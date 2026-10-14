import schema from "./🧬️schema/🔣️.json";
import type { DescriptorProgress } from "../📥️text/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🪪️ The neutral identity a host's admitted package receipt supplies. */
export type PackageReceiptIdentity = Readonly<{ version: 1; pluginId: string; packageId: string; componentSha256: string }>;

/** 📥️ Receives the exact original descriptor request, cancellation and progress. */
export type PackageReceiptRequest = Readonly<{ pluginId: string; moduleUrl: string; signal: AbortSignal | undefined; onProgress: DescriptorProgress }>;

/** 🔌️ Supplies admitted receipt identity without coupling a kernel to a host's wire or build policy. */
export interface PackageReceiptSource {
  read(request: PackageReceiptRequest): Promise<PackageReceiptIdentity>;
}

/** 🔐️ Admits exactly the neutral identity contract after a receipt source returns. */
export function readPackageReceiptIdentity(value: unknown): PackageReceiptIdentity {
  const errors = validateJsonSchemaSubset(schema, value);
  if (errors.length) throw Error("Invalid package receipt identity: " + errors.join("; "));
  return Object.freeze({ ...(value as PackageReceiptIdentity) });
}
