/** 💡️ Closed client intent; all identity, snapshots, and execution authority remain server-owned. */
export type InferenceRequestV1 = {
  readonly schema: "semio.hub.inference-request/v1";
  readonly version: 1;
  readonly requestId: string;
  readonly serviceId: "s.gis.gismap.inference";
  readonly policyVersion: 1;
  readonly lifetimeMs: number;
};

export function parseInferenceRequestV1(value: unknown): InferenceRequestV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("invalid inference intent");
  const row = value as Record<string, unknown>;
  const keys = ["schema", "version", "requestId", "serviceId", "policyVersion", "lifetimeMs"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))
    || row.schema !== "semio.hub.inference-request/v1" || row.version !== 1
    || typeof row.requestId !== "string" || !/^[0-9a-f]{32}$/.test(row.requestId)
    || row.serviceId !== "s.gis.gismap.inference" || row.policyVersion !== 1
    || typeof row.lifetimeMs !== "number" || !Number.isSafeInteger(row.lifetimeMs) || row.lifetimeMs < 1 || row.lifetimeMs > 120000) throw new Error("invalid inference intent");
  return { schema: row.schema, version: row.version, requestId: row.requestId, serviceId: row.serviceId, policyVersion: row.policyVersion, lifetimeMs: row.lifetimeMs };
}
