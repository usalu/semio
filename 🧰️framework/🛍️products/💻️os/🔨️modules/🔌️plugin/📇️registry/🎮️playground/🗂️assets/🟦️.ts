import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** ⏱️ Finite transport budgets authored with the neutral portable asset contract. */
export const TILE_PROXY_TRANSPORT_LIMITS_V1 = Object.freeze({ ...schema.definitions.TileProxyTransportLimitsV1.default });

/** 🗂️ One owner-authored upstream and workspace cache route. */
export type TileProxyAssetSpecV1 = { readonly kind: "tile-proxy"; readonly route: string; readonly upstream: string; readonly cache: string; readonly userAgent: string };

/** 🛂️ Admits only the exact fields and bounds authored in the portable asset schema. */
export function parseTileProxyAssetSpecV1(value: unknown): TileProxyAssetSpecV1 {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("tile asset must be a record");
  const record = value as Record<string, unknown>;
  if (Object.keys(record).some(key => !Object.hasOwn(schema.properties, key)) || schema.required.some(key => !Object.hasOwn(record, key))) throw Error("tile asset fields differ from its schema");
  for (const [key, raw] of Object.entries(schema.properties)) {
    const rule = raw as { const?: string; minLength?: number; maxLength?: number; pattern?: string };
    const field = record[key];
    if (typeof field !== "string" || field.normalize("NFC") !== field || (rule.const !== undefined && field !== rule.const) || (rule.minLength !== undefined && Array.from(field).length < rule.minLength) || (rule.maxLength !== undefined && Array.from(field).length > rule.maxLength) || (rule.pattern !== undefined && !new RegExp(rule.pattern, "u").test(field))) throw Error(`invalid tile asset ${key}`);
  }
  return record as TileProxyAssetSpecV1;
}
