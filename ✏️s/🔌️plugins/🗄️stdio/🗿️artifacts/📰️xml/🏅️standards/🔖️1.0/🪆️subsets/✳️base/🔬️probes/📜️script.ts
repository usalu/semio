#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.xml@1.0/✳️base`'s READER oracle (`quick-xml-1-0-mutate-reader`).
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual XML decode is performed by the sibling standalone `quick-xml-oracle-codec`
// binary (`../🏭️generator/🦀️quick-xml-oracle-codec`, depends on nothing but `quick-xml` 0.42) via
// its `project` subcommand — this file only shells out to it and performs the GATING structural
// comparison itself, over the SAME JSON shape this subset's own `semantic-xml-v1` comparison profile
// describes and this subset's own `🦀️oracle.rs::project_xml_1_0` independently produces
// (declaration, doctype, prolog, and the full element tree with attributes as an unordered
// name/value map) — no XML semantics computed here, only projection + compare.
//
// This subset ALSO carries a `🦀️oracle.rs` registered `cross-semio-implementation` —
// that module COMPUTES what a mutation should produce. This probe suite is a DIFFERENT mechanism:
// the expected state is never computed, it is COMMITTED as the `after` half of a byte-reproducible
// fixture, and `quick-xml` reads BOTH sides independently. See `../🔣️oracle.json`'s own
// `quick-xml-1-0-mutate-reader` oracle rationale.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts xml-import  --input <a.xml>
//   bun 📜️script.ts xml-project --input <a.xml>
//   bun 📜️script.ts xml-compare --input <expected.xml> --input <actual.xml>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from (both hand the
//      structural equality itself to this file, never to a computed prediction)
// @see ../🏭️generator/🦀️quick-xml-oracle-codec/src/main.rs — the `project` subcommand this file calls

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

const ENGINE = { family: "quick-xml", implementation: "quick-xml-oracle-codec (quick-xml 0.42 + this subset's own hand-rolled DOCTYPE-subset parser)", version: "quick-xml@0.42.0" } as const;
const PROBE_VERSION = "quick-xml@0.42.0";
const CODEC_MANIFEST = join(import.meta.dir, "..", "🏭️generator", "🦀️quick-xml-oracle-codec", "Cargo.toml");
//#endregion 🧬️Contract

//#region 📥️Model
type XDecl = { version: string; encoding: string | null; standalone: boolean | null };
type XExternalId = { kind: "system"; systemId: string } | { kind: "public"; publicId: string; systemId: string };
type XEntity = { parameter: boolean; name: string; value: string };
type XDoctype = { name: string; externalId: XExternalId | null; entities: XEntity[] };
type XNode = { kind: "element"; name: string; attrs: Record<string, string>; children: XNode[] } | { kind: "text" | "cdata" | "comment"; text: string } | { kind: "pi"; target: string; data: string };
/** 🌳 What `quick-xml-oracle-codec project` emits verbatim — already the `semantic-xml-v1` shape
 *  (attrs as an unordered map, entities reassembled), so no further projection step is needed here
 *  unlike AVI's opaque-binary-payload digesting — XML carries no large binary payloads. */
type XDoc = { declaration: XDecl | null; doctype: XDoctype | null; prolog: XNode[]; root: XNode | null };
//#endregion 📥️Model

//#region 🔓️Read
/** 📥️ Runs the standalone codec's `project` subcommand and parses its typed JSON. */
function readXml(path: string): XDoc {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "project", path], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`quick-xml-oracle-codec project ${path} failed (exit ${result.status}): ${result.stderr}`);
  return JSON.parse(result.stdout) as XDoc;
}
//#endregion 🔓️Read

//#region ⚖️Compare
/** ⚖️ Structural equality over the projected JSON — mirrors `semantic-xml-v1`'s own rules: object
 *  keys (including `attrs`) are compared unordered (an unordered map already carries no positional
 *  identity), arrays (siblings/children, doctype entities) are compared IN ORDER — sibling and
 *  child order is normative in XML and never sorted. */
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

function compareDocs(expected: XDoc, actual: XDoc): { equal: boolean; diffCount: number; diffs: string[] } {
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
  "xml-import": async (inputs) => {
    requireInputs(inputs, 1, "xml-import");
    const perInput = inputs.map((input) => {
      try {
        readXml(input);
        return { path: input, ok: true, error: undefined as string | undefined };
      } catch (error) {
        return { path: input, ok: false, error: String((error as Error).message ?? error) };
      }
    });
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "xml-project": async (inputs) => {
    requireInputs(inputs, 1, "xml-project");
    const doc = readXml(inputs[0]!);
    const countNodes = (node: XNode): number => (node.kind === "element" ? 1 + node.children.reduce((total, child) => total + countNodes(child), 0) : 1);
    return { status: "ok", measurements: { hasDeclaration: doc.declaration !== null, hasDoctype: doc.doctype !== null, prologNodeCount: doc.prolog.length, rootNodeCount: doc.root ? countNodes(doc.root) : 0, projection: doc } };
  },
  "xml-compare": async (inputs) => {
    requireInputs(inputs, 2, "xml-compare");
    const expected = readXml(inputs[0]!);
    const actual = readXml(inputs[1]!);
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
