/** 🏷️ Content revisions that run in a browser and in Node alike: a dependency-free SHA-256, the revision of a catalog file and of an environment preset.
 *
 * The cast exporter, the Python loader and the library browser must name one revision for one file. Nothing here touches `node:crypto`, so the browser computes the same text the exporter writes into `provenance.asset.revision`.
 *
 * @see https://csrc.nist.gov/publications/detail/fips/180/4/final — FIPS 180-4
 * @see ../../🧫️fixtures/🏷️revision/🔣️.json — the language-agnostic vectors, checked against `node:crypto`
 */
import type { Asset, EnvironmentPreset } from "../../🧬️schema/🟦️.ts";
import type { Cast, CastProvenance, CastScenario, Revision } from "../📤️export/🎭️cast/🧬️schema/🟦️.ts";
import * as portable from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

const PRIMES = Array.from({ length: 311 }, (_, n) => n + 2)
  .filter((n, _, all) => all.every((d) => d >= n || n % d !== 0))
  .slice(0, 64);
const fraction = (value: number): number => Math.floor((value - Math.floor(value)) * 2 ** 32);
const ROUND = PRIMES.map((prime) => fraction(portable.cbrt(prime)));
const START = PRIMES.slice(0, 8).map((prime) => fraction(Math.sqrt(prime)));
const rotate = (word: number, bits: number): number => (word >>> bits) | (word << (32 - bits));

/** 🔢️ The SHA-256 digest of `bytes` as 64 lower-case hex digits. */
export function sha256Hex(bytes: Uint8Array): string {
  const padded = new Uint8Array(Math.ceil((bytes.length + 9) / 64) * 64);
  padded.set(bytes);
  padded[bytes.length] = 0x80;
  const view = new DataView(padded.buffer);
  view.setUint32(padded.length - 8, Math.floor((bytes.length * 8) / 2 ** 32));
  view.setUint32(padded.length - 4, (bytes.length * 8) >>> 0);
  const state = Uint32Array.from(START);
  const schedule = new Uint32Array(64);
  for (let block = 0; block < padded.length; block += 64) {
    for (let i = 0; i < 16; i += 1) schedule[i] = view.getUint32(block + i * 4);
    for (let i = 16; i < 64; i += 1) {
      const a = schedule[i - 15]!;
      const b = schedule[i - 2]!;
      schedule[i] = (schedule[i - 16]! + (rotate(a, 7) ^ rotate(a, 18) ^ (a >>> 3)) + schedule[i - 7]! + (rotate(b, 17) ^ rotate(b, 19) ^ (b >>> 10))) >>> 0;
    }
    let [a, b, c, d, e, f, g, h] = state as unknown as [number, number, number, number, number, number, number, number];
    for (let i = 0; i < 64; i += 1) {
      const first = (h + (rotate(e, 6) ^ rotate(e, 11) ^ rotate(e, 25)) + ((e & f) ^ (~e & g)) + ROUND[i]! + schedule[i]!) >>> 0;
      const second = ((rotate(a, 2) ^ rotate(a, 13) ^ rotate(a, 22)) + ((a & b) ^ (a & c) ^ (b & c))) >>> 0;
      [h, g, f, e, d, c, b, a] = [g, f, e, (d + first) >>> 0, c, b, a, (first + second) >>> 0];
    }
    [a, b, c, d, e, f, g, h].forEach((word, i) => void (state[i] = (state[i]! + word) >>> 0));
  }
  return Array.from(state, (word) => word.toString(16).padStart(8, "0")).join("");
}

/** 🔖️ The revision of a catalog file: sha256 of its text with CRLF turned into LF, so a checkout with and one without `core.autocrlf` agree. */
export const revisionOfText = (text: string): Revision => `sha256:${sha256Hex(new TextEncoder().encode(text.replace(/\r\n/gu, "\n")))}`;

/** 🧾️ Canonical JSON: sorted keys, no whitespace, non-ASCII kept, numbers as ECMAScript prints them. */
export function canonicalJson(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(",")}]`;
  if (typeof value === "object" && value !== null) return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalJson((value as Record<string, unknown>)[key])}`).join(",")}}`;
  return JSON.stringify(value);
}

/** 🔖️ The revision of an environment preset: sha256 of its canonical JSON. */
export const presetRevision = (preset: EnvironmentPreset): Revision => `sha256:${sha256Hex(new TextEncoder().encode(canonicalJson(preset)))}`;

/** 🔖️ The provenance record of a cast: every dimension of the drawing named by the cast contract, with the asset and environment revisions. The exporter writes it into the index, the library browser shows the same record for what it draws.
 * @see ../📤️export/🎭️cast/🧬️schema/🟦️.ts — `CastProvenance` */
export function castProvenance(cast: Cast, asset: Asset, revision: Revision, presets: ReadonlyMap<string, EnvironmentPreset>, clock: { readonly fps: number; readonly durationS: number; readonly loop: boolean; readonly frames: number }): CastProvenance {
  const revisionOf = (slug: string): Revision => presetRevision(presets.get(slug)!);
  const thermal = { spanK: cast.thermalSpanK ?? null, transport: cast.transport ?? [] };
  const scenario: CastScenario =
    cast.view === "thermal"
      ? cast.timeline
        ? { kind: "thermal", timeline: [...cast.timeline].sort((a, b) => a.t - b.t).map((key) => ({ t: key.t, id: key.environment, ease: key.ease, revision: revisionOf(key.environment) })), ...thermal }
        : { kind: "thermal", environment: { id: cast.environment!, revision: revisionOf(cast.environment!) }, ...thermal }
      : cast.view === "forces" || cast.view === "moments"
        ? { kind: "structural", dynamics: asset.structure.dynamics, gravityMS2: asset.structure.gravityMS2 }
        : { kind: "geometry" };
  return {
    version: 1,
    asset: { id: asset.id, category: asset.category, revision },
    view: { style: cast.view, viewpoint: cast.viewpoint, theme: cast.theme, mirror: cast.mirror === true },
    scale: { denominator: cast.scale },
    state: cast.clip !== undefined ? { kind: "clip", id: cast.clip } : { kind: "pose", id: cast.pose! },
    clock: { ...clock, offsetS: cast.clip !== undefined ? cast.transportAgeS ?? 0 : 0 },
    scenario,
  };
}
