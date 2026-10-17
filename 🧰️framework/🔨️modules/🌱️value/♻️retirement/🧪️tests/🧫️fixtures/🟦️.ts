/** 🪥️ Single test-side owner of the controlled-retirement copy-demand oracle: the rule that decides how many payload bytes a retirement must copy before it may release physical backing, and the readers of the two corpora that state the expected numbers.
 * The rule mirrors production: a `Vec<T>` of plain scalars drains as logical work of `size_of::<T>()` bytes per turn (`RetireOwned::retirement_element_copy_bytes`), while a lone scalar leaf and a `String` byte buffer move to their cursor without payload work and only release their backing.
 * @see ../../🦀️.rs
 * @see ./📏️copy-demand/🔣️.json
 * @see ./⚖️physical-work/🔣️.json */
import { readFileSync } from "node:fs";
import { fixtureUrl } from "../../../../🧪️test/📖️source-text/🟦️.ts";

export type RetirementCopyKind = "u32" | "text" | "u32-list";

export interface RetirementCopyCase {
  readonly kind: RetirementCopyKind;
  readonly value: number | string | readonly number[];
}

export interface RetirementCopyDemand {
  readonly minimumCopyBytes: number;
  readonly totalCopyBytes: number;
}

export interface RetirementCopyDemandLaw {
  readonly schema: string;
  readonly grants: readonly number[];
  readonly cases: readonly (RetirementCopyCase & RetirementCopyDemand)[];
  readonly demandObservationAllocatesBytes: number;
  readonly zeroOrBelowInert: boolean;
  readonly copyTurnReleasesBytes: number;
  readonly maximumTurnBytes: number;
}

export interface RetirementPhysicalWorkLaw {
  readonly schema: string;
  readonly maximumItems: number;
  readonly maximumCopyBytes: number;
  readonly maximumCapacityBytes: number;
  readonly maximumReleaseBytes: number;
  readonly cases: readonly (RetirementCopyCase & { readonly id: string; readonly processedBytes: number })[];
  readonly zeroWorkInert: boolean;
  readonly physicalReleaseMatchesAllocator: boolean;
}

/** ✂️ One marker-delimited item of a Rust source text; a missing or misordered marker is a stale test, so it throws instead of slicing to an empty string. */
export function between(text: string, from: string, to: string): string {
  const start = text.indexOf(from);
  if (start < 0) throw new Error("Source marker missing: " + from);
  const end = text.indexOf(to, start + from.length);
  if (end < 0) throw new Error("Source end marker missing after " + from + ": " + to);
  return text.slice(start, end);
}

/** 🧮️ The payload bytes the production rule demands for one owner: the minimum single grant that advances it and the total it copies before its physical backing may be released. */
export function retirementCopyDemand(row: RetirementCopyCase): RetirementCopyDemand {
  switch (row.kind) {
    case "u32-list": {
      const total = Uint32Array.from(row.value as readonly number[]).byteLength;
      return { minimumCopyBytes: total === 0 ? 0 : Uint32Array.BYTES_PER_ELEMENT, totalCopyBytes: total };
    }
    case "u32":
    case "text":
      return { minimumCopyBytes: 0, totalCopyBytes: 0 };
    default:
      throw new Error(`retirement copy kind has no production rule: ${JSON.stringify((row as { kind: unknown }).kind)}`);
  }
}

/** 📏️ Reads the copy-demand corpus that the native controlled-retirement test asserts against the real cursors. */
export function readRetirementCopyDemandLaw(): RetirementCopyDemandLaw {
  return JSON.parse(readFileSync(fixtureUrl(import.meta.url, "📏️copy-demand/🔣️.json"), "utf8")) as RetirementCopyDemandLaw;
}

/** ⚖️ Reads the physical-work corpus that separates logical payload work from physical release. */
export function readRetirementPhysicalWorkLaw(): RetirementPhysicalWorkLaw {
  return JSON.parse(readFileSync(fixtureUrl(import.meta.url, "⚖️physical-work/🔣️.json"), "utf8")) as RetirementPhysicalWorkLaw;
}
