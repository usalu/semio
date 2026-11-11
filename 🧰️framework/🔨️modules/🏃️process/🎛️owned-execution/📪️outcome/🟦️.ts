import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import { constants } from "node:os";

/** 📪️ Preserves the actual operating-system terminal status and signal. */
export type OwnedCommandOutcome = Readonly<{ version: 1; status: number | null; signal: string | null }>;

/** 🔐️ Admits the schema-owned outcome without substituting terminal values. */
export function readOwnedCommandOutcome(value: unknown): OwnedCommandOutcome {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw Error("Original owned command outcome required");
  const row = value as Record<string, unknown>, status = schema.properties.status, signal = schema.properties.signal;
  if (Object.keys(row).length !== schema.required.length || schema.required.some(key => !Object.hasOwn(row, key)) || row.version !== schema.properties.version.const || (row.status !== null && (!Number.isInteger(row.status) || Number(row.status) < status.minimum || Number(row.status) > status.maximum)) || (row.signal !== null && (typeof row.signal !== "string" || row.signal.length < signal.minLength || row.signal.length > signal.maxLength || !new RegExp(signal.pattern, "u").test(row.signal))) || (row.status === null && row.signal === null)) throw Error("Original owned command outcome required");
  return value as OwnedCommandOutcome;
}

/** 🚪️ The exit status a failed child passes on: its own status, or 128 plus the number of the signal that killed it. */
export function ownedCommandExitCodeV1(outcome: OwnedCommandOutcome): number {
  return outcome.status ?? 128 + (constants.signals[(outcome.signal ?? "") as keyof typeof constants.signals] ?? 0);
}

/** 🛑️ Retains exact child failure evidence through the receiving boundary. */
export class OwnedCommandFailure extends Error {
  readonly label: string;
  readonly outcome: OwnedCommandOutcome;
  constructor(label: string, outcome: OwnedCommandOutcome) {
    const original = readOwnedCommandOutcome(outcome);
    super(original.signal !== null ? `${label} was killed by ${original.signal}` : `${label} failed (${original.status})`);
    this.name = "OwnedCommandFailure";
    this.label = label;
    this.outcome = original;
  }
}

/** 🔎️ Recognizes a failed child command even when another instance of this module built it. */
export function isOwnedCommandFailureV1(value: unknown): value is OwnedCommandFailure {
  const row = value as Partial<OwnedCommandFailure> | null;
  return typeof value === "object" && row !== null && (value instanceof OwnedCommandFailure || (row.name === "OwnedCommandFailure" && typeof row.label === "string" && typeof row.outcome === "object" && row.outcome !== null));
}
