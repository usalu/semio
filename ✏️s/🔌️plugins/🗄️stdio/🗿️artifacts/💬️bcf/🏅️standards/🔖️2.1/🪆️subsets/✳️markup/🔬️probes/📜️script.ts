#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.bcf@2.1/✳️markup`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. `jszip` opens the bcfzip container, `fast-xml-parser` parses every XML part inside it
// (`bcf.version`, each topic's `markup.bcf`, each viewpoint's `.bcfv`) — both are already vendored in
// this repo's own `node_modules` (`jszip` 3.10.1, MIT/GPL-3.0-or-later; `fast-xml-parser` 5.11.1,
// MIT — versions read directly off their own `package.json`, not guessed). The pipeline compares the
// emitted `measurements`; this file performs no mutation semantics of its own.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts bcf-import  --input <a.bcf>
//   bun 📜️script.ts bcf-project --input <a.bcf>
//   bun 📜️script.ts bcf-compare --input <expected.bcf> --input <actual.bcf>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch shape is mirrored from

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import JSZip from "jszip";
import { XMLParser } from "fast-xml-parser";
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

const ENGINE = { family: "zip-xml", implementation: "jszip + fast-xml-parser", version: "jszip@3.10.1 + fast-xml-parser@5.11.1" } as const;
const PROBE_VERSION = "jszip@3.10.1 + fast-xml-parser@5.11.1";
//#endregion 🧬️Contract

//#region 📥️Model
type Camera =
  | { kind: "perspective"; viewPoint: [number, number, number]; direction: [number, number, number]; upVector: [number, number, number]; fieldOfView: number }
  | { kind: "orthogonal"; viewPoint: [number, number, number]; direction: [number, number, number]; upVector: [number, number, number]; viewToWorldScale: number };

type Components = { selection: string[]; visibility: { defaultVisibility: boolean; exceptions: string[] }; coloring: { color: string; components: string[] }[] };

type Viewpoint = { guid: string; camera: Camera | null; components: Components | null; snapshotSize: number | null; snapshotDigest: string | null };

type Comment = { guid: string; date: string; author: string; text: string; viewpointRef: string | null };

type Topic = { guid: string; title: string; description: string; status: string; priority: string; labels: string[]; creationDate: string; creationAuthor: string; comments: Record<string, Comment>; viewpoints: Record<string, Viewpoint> };

type BcfDoc = { version: string; topics: Record<string, Topic>; parts: Record<string, { size: number; digest: string }> };
//#endregion 📥️Model

//#region 🔓️Parse
const XML = new XMLParser({ ignoreAttributes: false, attributeNamePrefix: "@_", trimValues: true, parseTagValue: false, parseAttributeValue: false, ignoreDeclaration: true, textNodeName: "#text" });

/** 🌳 fast-xml-parser folds a single occurrence into an object and 2+ into an array — normalize. */
function arr<T>(value: T | T[] | undefined): T[] {
  if (value === undefined) return [];
  return Array.isArray(value) ? value : [value];
}

function textOf(node: unknown): string {
  if (node === undefined || node === null) return "";
  if (typeof node === "string") return node;
  if (typeof node === "number" || typeof node === "boolean") return String(node);
  if (typeof node === "object" && node !== null && "#text" in (node as Record<string, unknown>)) return String((node as Record<string, unknown>)["#text"] ?? "");
  return "";
}

function attr(node: Record<string, unknown> | undefined, name: string): string {
  if (!node) return "";
  const value = node[`@_${name}`];
  return value === undefined ? "" : String(value);
}

function digestOf(bytes: Buffer): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

function point(node: Record<string, unknown> | undefined): [number, number, number] {
  if (!node) return [0, 0, 0];
  return [Number.parseFloat(attr(node, "X")) || 0, Number.parseFloat(attr(node, "Y")) || 0, Number.parseFloat(attr(node, "Z")) || 0];
}

function parseCamera(visInfo: Record<string, unknown>): Camera | null {
  const perspective = visInfo["PerspectiveCamera"] as Record<string, unknown> | undefined;
  if (perspective) {
    return { kind: "perspective", viewPoint: point(perspective["CameraViewPoint"] as never), direction: point(perspective["CameraDirection"] as never), upVector: point(perspective["CameraUpVector"] as never), fieldOfView: Number.parseFloat(textOf(perspective["FieldOfView"])) || 0 };
  }
  const orthogonal = visInfo["OrthogonalCamera"] as Record<string, unknown> | undefined;
  if (orthogonal) {
    return { kind: "orthogonal", viewPoint: point(orthogonal["CameraViewPoint"] as never), direction: point(orthogonal["CameraDirection"] as never), upVector: point(orthogonal["CameraUpVector"] as never), viewToWorldScale: Number.parseFloat(textOf(orthogonal["ViewToWorldScale"])) || 0 };
  }
  return null;
}

function componentList(node: Record<string, unknown> | undefined): string[] {
  if (!node) return [];
  return arr(node["Component"] as never).map((c) => attr(c as never, "IfcGuid"));
}

function parseComponents(node: Record<string, unknown> | undefined): Components | null {
  if (!node) return null;
  const selectionNode = node["Selection"] as Record<string, unknown> | undefined;
  const visibilityNode = node["Visibility"] as Record<string, unknown> | undefined;
  const coloringNode = node["Coloring"] as Record<string, unknown> | undefined;
  return {
    selection: componentList(selectionNode),
    visibility: { defaultVisibility: attr(visibilityNode, "DefaultVisibility") !== "false", exceptions: componentList(visibilityNode?.["Exceptions"] as never) },
    coloring: arr(coloringNode?.["Color"] as never).map((c) => ({ color: attr(c as never, "Color"), components: componentList(c as never) })),
  };
}

async function readBcf(path: string): Promise<BcfDoc> {
  const bytes = readFileSync(path);
  const zip = await JSZip.loadAsync(bytes);
  const entries = Object.values(zip.files).filter((entry) => !entry.dir);
  const byLowerName = new Map(entries.map((entry) => [entry.name.toLowerCase(), entry]));
  const consumed = new Set<string>();

  let version = "";
  const versionEntry = byLowerName.get("bcf.version");
  if (versionEntry) {
    const doc = XML.parse(await versionEntry.async("string"));
    version = attr(doc["Version"] as never, "VersionId");
    consumed.add(versionEntry.name);
  }

  const folders = new Map<string, typeof entries>();
  for (const entry of entries) {
    const slash = entry.name.indexOf("/");
    if (slash < 0) continue;
    const folder = entry.name.slice(0, slash);
    if (!folders.has(folder)) folders.set(folder, []);
    folders.get(folder)!.push(entry);
  }

  const topics: Record<string, Topic> = {};
  for (const [, folderEntries] of folders) {
    const markupEntry = folderEntries.find((entry) => entry.name.toLowerCase().endsWith("/markup.bcf"));
    if (!markupEntry) continue;
    const markup = XML.parse(await markupEntry.async("string"));
    consumed.add(markupEntry.name);
    const root = markup["Markup"] as Record<string, unknown>;
    const topicNode = root["Topic"] as Record<string, unknown>;
    const guid = attr(topicNode, "Guid");

    const comments: Record<string, Comment> = {};
    for (const commentNode of arr(root["Comment"] as never)) {
      const c = commentNode as Record<string, unknown>;
      const viewpointNode = c["Viewpoint"] as Record<string, unknown> | undefined;
      const commentGuid = attr(c, "Guid");
      comments[commentGuid] = { guid: commentGuid, date: textOf(c["Date"]), author: textOf(c["Author"]), text: textOf(c["Comment"]), viewpointRef: viewpointNode ? attr(viewpointNode, "Guid") : null };
    }

    const viewpoints: Record<string, Viewpoint> = {};
    for (const refNode of arr(root["Viewpoints"] as never)) {
      const ref = refNode as Record<string, unknown>;
      const vGuid = attr(ref, "Guid");
      const viewpointFile = textOf(ref["Viewpoint"]);
      const snapshotFile = textOf(ref["Snapshot"]);
      let camera: Camera | null = null;
      let components: Components | null = null;
      if (viewpointFile) {
        const full = `${markupEntry.name.slice(0, markupEntry.name.lastIndexOf("/"))}/${viewpointFile}`;
        const bcfvEntry = byLowerName.get(full.toLowerCase());
        if (bcfvEntry) {
          const bcfv = XML.parse(await bcfvEntry.async("string"));
          const visInfo = bcfv["VisualizationInfo"] as Record<string, unknown>;
          camera = parseCamera(visInfo);
          components = parseComponents(visInfo["Components"] as never);
          consumed.add(bcfvEntry.name);
        }
      }
      let snapshotSize: number | null = null;
      let snapshotDigest: string | null = null;
      if (snapshotFile) {
        const full = `${markupEntry.name.slice(0, markupEntry.name.lastIndexOf("/"))}/${snapshotFile}`;
        const snapEntry = byLowerName.get(full.toLowerCase());
        if (snapEntry) {
          const bytes2 = await snapEntry.async("nodebuffer");
          snapshotSize = bytes2.length;
          snapshotDigest = digestOf(bytes2);
          consumed.add(snapEntry.name);
        }
      }
      viewpoints[vGuid] = { guid: vGuid, camera, components, snapshotSize, snapshotDigest };
    }

    topics[guid] = {
      guid,
      title: textOf(topicNode["Title"]),
      description: textOf(topicNode["Description"]),
      status: attr(topicNode, "TopicStatus"),
      priority: textOf(topicNode["Priority"]),
      labels: arr(topicNode["Labels"] as never).map((l) => textOf(l)),
      creationDate: textOf(topicNode["CreationDate"]),
      creationAuthor: textOf(topicNode["CreationAuthor"]),
      comments,
      viewpoints,
    };
  }

  const parts: Record<string, { size: number; digest: string }> = {};
  for (const entry of entries) {
    if (consumed.has(entry.name)) continue;
    const bytes2 = await entry.async("nodebuffer");
    parts[entry.name] = { size: bytes2.length, digest: digestOf(bytes2) };
  }

  return { version, topics, parts };
}
//#endregion 🔓️Parse

//#region ⚖️Compare
/** ⚖️ Set/map-keyed structural equality — mirrors the `semantic-bcf-v1` comparisonProfile's own
 *  `arrays: "set"` rule (topics/comments/viewpoints already keyed by guid above, so this is a plain
 *  deep-equal over that keyed shape, with array FIELDS inside it — `labels`, `selection`,
 *  `exceptions`, `coloring`'s member lists — compared as sets, per the same profile). */
function asSet(items: string[]): string {
  return JSON.stringify([...items].sort());
}

function diffAt(path: string, expected: unknown, actual: unknown, diffs: string[]): void {
  if (Array.isArray(expected) && Array.isArray(actual) && expected.every((item) => typeof item === "string")) {
    if (asSet(expected as string[]) !== asSet(actual as string[])) diffs.push(`${path}: set {${(expected as string[]).join(",")}} ≠ {${(actual as string[]).join(",")}}`);
    return;
  }
  if (typeof expected === "object" && expected !== null && typeof actual === "object" && actual !== null && !Array.isArray(expected)) {
    const keys = new Set([...Object.keys(expected as object), ...Object.keys(actual as object)]);
    for (const key of keys) diffAt(`${path}.${key}`, (expected as Record<string, unknown>)[key], (actual as Record<string, unknown>)[key], diffs);
    return;
  }
  if (JSON.stringify(expected) !== JSON.stringify(actual)) diffs.push(`${path}: ${JSON.stringify(expected)} ≠ ${JSON.stringify(actual)}`);
}

function compareDocs(expected: BcfDoc, actual: BcfDoc): { equal: boolean; diffCount: number; diffs: string[] } {
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
  "bcf-import": async (inputs) => {
    requireInputs(inputs, 1, "bcf-import");
    const results = await Promise.allSettled(inputs.map((input) => readBcf(input)));
    const perInput = results.map((result, index) => ({ path: inputs[index], ok: result.status === "fulfilled", error: result.status === "rejected" ? String((result.reason as Error).message ?? result.reason) : undefined }));
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "bcf-project": async (inputs) => {
    requireInputs(inputs, 1, "bcf-project");
    const doc = await readBcf(inputs[0]!);
    return { status: "ok", measurements: { topicCount: Object.keys(doc.topics).length, partCount: Object.keys(doc.parts).length, projection: doc } };
  },
  "bcf-compare": async (inputs) => {
    requireInputs(inputs, 2, "bcf-compare");
    const expected = await readBcf(inputs[0]!);
    const actual = await readBcf(inputs[1]!);
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
