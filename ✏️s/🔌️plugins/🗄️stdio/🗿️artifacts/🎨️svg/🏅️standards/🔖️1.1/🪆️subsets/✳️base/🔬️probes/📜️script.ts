#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.svg@1.1/✳️base`'s READER oracle
// (`quick-xml-svg-1-1-mutate-reader`).
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one
// should produce. The actual XML decode is performed by the sibling standalone
// `quick-xml-svg-codec` binary (`../🏭️generator/🦀️quick-xml-svg-codec`, depends on nothing but
// `quick-xml` 0.42) via its `project` subcommand — this file only shells out to it, sorts each
// element's attributes by name (per this oracle's own `svg-1-1-quick-xml-reader-v1`
// comparisonProfile — SVG attribute order is real writer freedom, never source-order-significant)
// and performs the GATING structural comparison itself. No SVG semantics computed here: `viewBox`
// and `transform` are compared as OPAQUE ATTRIBUTE STRINGS, exactly like every other attribute —
// `quick-xml` itself has no notion of SVG geometry grammar, so this reader never decomposes them
// into numbers the way this subset's own `🧪️oracle/🦀️component.rs` (a computed, NOT a read,
// cross-semio-implementation oracle) does. A `viewBox`/`transform` value that changed is still a
// string that differs, which IS a witnessed, honest reader comparison — just not a semantic one.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts svg-import  --input <a.svg>
//   bun 📜️script.ts svg-project --input <a.svg>
//   bun 📜️script.ts svg-compare --input <expected.svg> --input <actual.svg>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from (both hand the
//      structural equality itself to this file, never to a computed prediction)
// @see ../🏭️generator/🦀️quick-xml-svg-codec/src/main.rs — the `project` subcommand this file calls

//#endregion 🧲️Header

//#region 🔌️Adapters
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never computes them. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed" | "unsupported";
  durationMs: number;
  measurements: Record<string, unknown>;
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

const ENGINE = { family: "quick-xml", implementation: "quick-xml-svg-codec (quick-xml 0.42 + a generic element/text/cdata/comment/pi tree — no SVG semantics)", version: "quick-xml@0.42.0" } as const;
const PROBE_VERSION = "quick-xml@0.42.0";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️quick-xml-svg-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type RawNode =
  | { kind: "text"; text: string }
  | { kind: "cdata"; text: string }
  | { kind: "comment"; text: string }
  | { kind: "pi"; target: string; data: string }
  | { kind: "element"; name: string; attrs: { name: string; value: string }[]; children: RawNode[] };

/** 🌳 What `quick-xml-svg-codec project` emits verbatim — attributes still in SOURCE order. */
type RawDoc = { declaration: { present: boolean; version?: string; encoding?: string | null; standalone?: boolean | null }; doctype: { present: boolean; raw?: string }; root: RawNode | null };

/** ⚖️ The comparisonProfile's own projection: each element's attributes become a NAME-SORTED
 *  array of `[name, value]` pairs (never a raw source-order list) — `viewBox`/`transform`
 *  included as plain strings, never decomposed. */
type ProjectedNode =
  | { kind: "text"; text: string }
  | { kind: "cdata"; text: string }
  | { kind: "comment"; text: string }
  | { kind: "pi"; target: string; data: string }
  | { kind: "element"; name: string; attrs: [string, string][]; children: ProjectedNode[] };
type SvgDoc = { declaration: RawDoc["declaration"]; doctype: RawDoc["doctype"]; root: ProjectedNode | null };
//#endregion 📥️Model

//#region 🔓️Read
function projectNode(node: RawNode): ProjectedNode {
  if (node.kind !== "element") return node;
  const attrs: [string, string][] = node.attrs.map((a) => [a.name, a.value]).sort((a, b) => (a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0));
  return { kind: "element", name: node.name, attrs, children: node.children.map(projectNode) };
}

/** 📥️ Runs the standalone codec's `project` subcommand and turns its raw, source-order
 *  attributes into the profile's own name-sorted projection. */
function readSvg(path: string): SvgDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`quick-xml-svg-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  const raw = JSON.parse(result.stdout) as RawDoc;
  return { declaration: raw.declaration, doctype: raw.doctype, root: raw.root === null ? null : projectNode(raw.root) };
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Structural equality — element/text/cdata/comment/pi tree shape and content exact, children
 *  order-significant (document order is this format's own semantic identity for a node, same as
 *  this subset's own `semantic-svg-1-1-v1` profile), attributes already name-sorted by
 *  `projectNode` above so this is a plain recursive equality, not a second sort. */
function diffAt(path: string, expected: unknown, actual: unknown, diffs: string[]): void {
  if (Array.isArray(expected) && Array.isArray(actual)) {
    const len = Math.max(expected.length, actual.length);
    for (let i = 0; i < len; i += 1) diffAt(`${path}[${i}]`, expected[i], actual[i], diffs);
    return;
  }
  if (typeof expected === "object" && expected !== null && typeof actual === "object" && actual !== null && !Array.isArray(expected)) {
    const keys = new Set([...Object.keys(expected as object), ...Object.keys(actual as object)]);
    for (const key of keys) diffAt(`${path}.${key}`, (expected as Record<string, unknown>)[key], (actual as Record<string, unknown>)[key], diffs);
    return;
  }
  if (JSON.stringify(expected) !== JSON.stringify(actual)) diffs.push(`${path}: ${JSON.stringify(expected)} ≠ ${JSON.stringify(actual)}`);
}

function compareDocs(expected: SvgDoc, actual: SvgDoc): { equal: boolean; diffCount: number; diffs: string[] } {
  const diffs: string[] = [];
  diffAt("$", expected, actual, diffs);
  return { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50) };
}
//#endregion ⚖️Compare

//#region 🔬️Probes
function requireInputs(inputs: readonly string[], count: number, probe: string): void {
  if (inputs.length < count) throw new Error(`${probe} requires ${count} --input path(s), got ${inputs.length}`);
}

type ProbeResult = { status: "ok" | "failed" | "unsupported"; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] };

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "svg-import": async (inputs) => {
    requireInputs(inputs, 1, "svg-import");
    const perInput = inputs.map((input) => {
      try {
        readSvg(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "svg-project": async (inputs) => {
    requireInputs(inputs, 1, "svg-project");
    const doc = readSvg(inputs[0]!);
    const countNodes = (node: ProjectedNode | null): number => (node === null ? 0 : node.kind === "element" ? 1 + node.children.reduce((total, child) => total + countNodes(child), 0) : 1);
    return { status: "ok", measurements: { declarationPresent: doc.declaration.present, doctypePresent: doc.doctype.present, nodeCount: countNodes(doc.root), projection: doc } };
  },
  "svg-compare": async (inputs) => {
    requireInputs(inputs, 2, "svg-compare");
    const expected = readSvg(inputs[0]!);
    const actual = readSvg(inputs[1]!);
    const verdict = compareDocs(expected, actual);
    return { status: "ok", measurements: { ...verdict, expected, actual } };
  },
};
//#endregion 🔬️Probes

//#region 🚀️Entry
function parseArgv(argv: readonly string[]): { probe: string; inputs: string[] } {
  const [probe = "", ...rest] = argv;
  const inputs: string[] = [];
  for (let i = 0; i < rest.length; i += 1) if (rest[i] === "--input") inputs.push(rest[i + 1] ?? "");
  return { probe, inputs };
}

async function main(argv: readonly string[]): Promise<number> {
  const { probe, inputs } = parseArgv(argv);
  const started = Date.now();
  const emit = (report: ProbeReport): number => {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return report.status === "failed" ? 1 : 0;
  };
  const budgetMs = Number(process.env.SEMIO_PROBE_TIMEOUT_MS ?? 60_000);
  const watchdog = new Promise<never>((_, reject) => setTimeout(() => reject(new Error(`probe exceeded ${budgetMs} ms`)), budgetMs).unref?.());
  const run = PROBES[probe];
  if (!run) return emit({ schema: "semio.repository-test.probe-report/v2", probe: probe || "(none)", probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${probe}`, detail: `known: ${Object.keys(PROBES).join(", ")}` }] });
  try {
    const result = await Promise.race([run(inputs), watchdog]);
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
