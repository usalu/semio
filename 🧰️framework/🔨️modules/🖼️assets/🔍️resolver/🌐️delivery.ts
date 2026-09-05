import authority from "./🚚️delivery.json" with { type: "json" };

export interface AssetDeliveryAuthority {
  readonly $schema: "./🧬️delivery.schema.json";
  readonly version: 1;
  readonly directoryName: "🖼️assets";
}

/** 🚚️ Admits the single asset-owned publication directory without aliases. */
export function parseAssetDeliveryAuthority(value: unknown): AssetDeliveryAuthority {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid asset delivery authority");
  const record = value as Record<string, unknown>;
  if (Object.keys(record).length !== 3 || record.$schema !== "./🧬️delivery.schema.json" || record.version !== 1 || record.directoryName !== "🖼️assets") throw new Error("Unknown asset delivery authority");
  return value as AssetDeliveryAuthority;
}

export const SEMIO_ASSET_DIRECTORY = parseAssetDeliveryAuthority(authority).directoryName;
export const SEMIO_ASSET_ROUTE = `/${SEMIO_ASSET_DIRECTORY}`;

function assetRelativePath(path: string): boolean {
  return !/[\\%?#\u0000-\u001f\u007f]/u.test(path) && path.split("/").every(segment => segment.length > 0 && segment !== "." && segment !== "..");
}

/** 🔎️ Resolves raw or once-encoded request paths inside the exact asset namespace. */
export function assetPathFromRequest(target: string): string | null {
  const encoded = target.split(/[?#]/u, 1)[0]!;
  if (/%(?:2f|5c)/iu.test(encoded)) return null;
  let path: string;
  try { path = decodeURIComponent(encoded); } catch { return null; }
  if (!path.startsWith(`${SEMIO_ASSET_ROUTE}/`)) return null;
  const relative = path.slice(SEMIO_ASSET_ROUTE.length + 1);
  return assetRelativePath(relative) ? relative : null;
}

/** 🔗️ Encodes an admitted source-relative asset path without changing its identity. */
export function assetTransportUrl(path: string): string {
  if (!assetRelativePath(path)) throw new Error("Invalid relative asset path");
  return `/${encodeURIComponent(SEMIO_ASSET_DIRECTORY)}/${path.split("/").map(encodeURIComponent).join("/")}`;
}
