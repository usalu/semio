import policySchema from "./🧬️schema/🔣️.json";
import eventSchema from "./🧬️schema/🔣️event.json";

export interface GeneratorPreviewProgressPolicyV1 { readonly protocol: "physical-file-progress-v1"; readonly maxEventBytes: number; readonly maxBytes: number; readonly maxEvents: number; readonly maxWork: number; readonly maxDurationMs: number; readonly maxSourceBytes: number; readonly maxSources: number }
export interface GeneratorPreviewProgressEventV1 { readonly protocol: "physical-file-progress-v1"; readonly sourceIndex: number; readonly bytes: number; readonly totalBytes: number; readonly work: number; readonly elapsedMs: number }
export interface GeneratorPreviewProgressControlV1 { cancelled(): boolean; remainingMs(): number }

/** 📋️ Admits explicit finite authority for one owned generator progress stream. */
export function parseGeneratorPreviewProgressPolicyV1(value: unknown): GeneratorPreviewProgressPolicyV1 {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid declared preview progress policy");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== policySchema.required.length || policySchema.required.some(key => !Object.hasOwn(row, key)) || row.protocol !== policySchema.properties.protocol.const) throw new Error("Invalid declared preview progress policy");
  for (const key of policySchema.required.filter(key => key !== "protocol")) { const limit = policySchema.properties[key as keyof typeof policySchema.properties] as { minimum: number; maximum: number }; if (!Number.isSafeInteger(row[key]) || (row[key] as number) < limit.minimum || (row[key] as number) > limit.maximum) throw new Error("Invalid finite preview progress authority"); }
  return Object.freeze(Object.fromEntries(policySchema.required.map(key => [key, row[key]]))) as unknown as GeneratorPreviewProgressPolicyV1;
}

/** 📈️ Serializes only bounded first-party progress, keeping artifact stdout separate. */
export function serializeGeneratorPreviewProgressV1(value: unknown, declaration: GeneratorPreviewProgressPolicyV1): string {
  const policy = parseGeneratorPreviewProgressPolicyV1(declaration);
  const row = value as Record<string, unknown>;
  if (!row || typeof row !== "object" || Array.isArray(row) || Object.keys(row).length !== eventSchema.required.length || eventSchema.required.some(key => !Object.hasOwn(row, key)) || row.protocol !== policy.protocol) throw new Error("Invalid owned preview progress event");
  for (const key of eventSchema.required.filter(key => key !== "protocol")) if (!Number.isSafeInteger(row[key]) || (row[key] as number) < (key === "work" ? 1 : 0)) throw new Error("Invalid preview progress quantity");
  if ((row.sourceIndex as number) >= policy.maxSources || (row.totalBytes as number) > policy.maxSourceBytes || (row.bytes as number) > (row.totalBytes as number) || (row.work as number) > policy.maxWork || (row.elapsedMs as number) > policy.maxDurationMs) throw new Error("Preview progress exceeds declared authority");
  const line = `[DEBUG] ${JSON.stringify(row)}\n`;
  if (new TextEncoder().encode(line).byteLength > policy.maxEventBytes) throw new Error("Preview progress event exceeds byte authority");
  return line;
}

/** 🧾️ Receives exactly the declared bounded progress protocol and refuses every other stderr body. */
export function receiveGeneratorPreviewProgressV1(stderr: string, declaration: GeneratorPreviewProgressPolicyV1 | undefined, control: GeneratorPreviewProgressControlV1): readonly GeneratorPreviewProgressEventV1[] {
  const guard = (): void => { const cancelled = control.cancelled(), remaining = control.remainingMs(); if (typeof cancelled !== "boolean" || cancelled || !Number.isFinite(remaining) || remaining <= 0) throw new Error("Preview progress operation cancelled or expired"); };
  guard();
  if (typeof stderr !== "string") throw new Error("Invalid preview stderr body");
  if (!declaration) { if (stderr) throw new Error("Undeclared generator stderr refused"); return []; }
  const policy = parseGeneratorPreviewProgressPolicyV1(declaration);
  if (new TextEncoder().encode(stderr).byteLength > policy.maxBytes || (stderr && !stderr.endsWith("\n"))) throw new Error("Preview progress stream exceeds byte authority or framing");
  const events: GeneratorPreviewProgressEventV1[] = [], sources = new Map<number, GeneratorPreviewProgressEventV1>();
  let work = 0, elapsed = 0;
  for (const line of stderr.split("\n").slice(0, -1)) {
    guard();
    if (events.length >= policy.maxEvents || !line.startsWith("[DEBUG] ")) throw new Error("Unknown or excessive generator stderr refused");
    const event = JSON.parse(line.slice(8)) as GeneratorPreviewProgressEventV1;
    if (serializeGeneratorPreviewProgressV1(event, policy) !== line + "\n" || event.work <= work || event.elapsedMs < elapsed) throw new Error("Malformed or reversing preview progress refused");
    const previous = sources.get(event.sourceIndex);
    if (previous && (previous.totalBytes !== event.totalBytes || previous.bytes > event.bytes)) throw new Error("Preview source progress reversed");
    sources.set(event.sourceIndex, event); events.push(Object.freeze({ ...event })); work = event.work; elapsed = event.elapsedMs;
  }
  if ([...sources.values()].some(event => event.bytes !== event.totalBytes)) throw new Error("Preview progress stream omitted source completion");
  guard();return Object.freeze(events);
}
