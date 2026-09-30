/** 🔎️ Domain-neutral request identity and opaque recovered-job envelope. */
export const JOB_RECONCILE_REQUEST_SCHEMA_V1 = "semio.framework.job-reconcile/v1";
export const JOB_RECONCILE_RESULT_SCHEMA_V1 = "semio.framework.job-reconcile-result/v1";
export const JOB_RECONCILE_REQUEST_MAX_BYTES = 256;
export type JobReconcileRequestV1 = { readonly schema: typeof JOB_RECONCILE_REQUEST_SCHEMA_V1; readonly version: 1; readonly requestId: string };
export type JobReconcileResultV1<T> = {
  readonly schema: typeof JOB_RECONCILE_RESULT_SCHEMA_V1;
  readonly version: 1;
  readonly requestId: string;
} & ({ readonly found: false; readonly job: null } | { readonly found: true; readonly job: T });

function envelope(value: unknown, keys: readonly string[], schema: string): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("job-reconcile.invalid-envelope");
  const row = value as Record<string, unknown>;
  if (keys.some((key) => !Object.hasOwn(row, key)) || Object.keys(row).some((key) => !keys.includes(key))
    || row.schema !== schema || row.version !== 1 || typeof row.requestId !== "string" || !/^[0-9a-f]{32}$/.test(row.requestId)) throw new Error("job-reconcile.invalid-envelope");
  return row;
}

/** 📨️ Only the original request identity crosses the lookup boundary. */
export function parseJobReconcileRequestV1(value: unknown): JobReconcileRequestV1 {
  const row = envelope(value, ["schema", "version", "requestId"], JOB_RECONCILE_REQUEST_SCHEMA_V1);
  return { schema: JOB_RECONCILE_REQUEST_SCHEMA_V1, version: 1, requestId: row.requestId as string };
}

/** 📦️ The owning application validates recovered payloads through its own decoder. */
export function parseJobReconcileResultV1<T>(value: unknown, decodeJob: (value: Record<string, unknown>) => T): JobReconcileResultV1<T> {
  const row = envelope(value, ["schema", "version", "requestId", "found", "job"], JOB_RECONCILE_RESULT_SCHEMA_V1);
  const base = { schema: JOB_RECONCILE_RESULT_SCHEMA_V1, version: 1, requestId: row.requestId as string } as const;
  if (row.found === false && row.job === null) return { ...base, found: false, job: null };
  if (row.found !== true || typeof row.job !== "object" || row.job === null || Array.isArray(row.job)) throw new Error("job-reconcile.invalid-result");
  return { ...base, found: true, job: decodeJob(row.job as Record<string, unknown>) };
}
