#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.bcf@2.1/✳️markup`.
//
// Every recipe below is a BEFORE and (where the outcome is legal) an AFTER bcfzip, each one built
// DIRECTLY by `jszip`+`fast-xml-parser`'s `XMLBuilder` — never by "applying" a mutation in code. Both
// states of every recipe are independently authored here; nothing in this file re-derives one from
// the other by executing mutation semantics, which is the whole reason this counts as independent
// evidence rather than a reflection of this repository's own dispatch.
//
// Generation and execution are SEPARATE operations, same shape as the `mesh`/`brep`/`step@ap214/cc6`
// generators this file's CLI is mirrored from: a normal test run must never be able to rewrite the
// expectation it is measured against.
//
//   bun 📜️script.ts generate --only <fixture-id>
//
// @see ../../../../🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🏭️generator/📜️script.ts — the sibling
//      generator this file's CLI shape is mirrored from
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import JSZip from "jszip";
import { XMLBuilder } from "fast-xml-parser";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const BUILD = new XMLBuilder({ ignoreAttributes: false, attributeNamePrefix: "@_", format: false, suppressEmptyNode: true });

function xmlPart(root: Record<string, unknown>): string {
  return `<?xml version="1.0" encoding="UTF-8"?>${BUILD.build(root)}`;
}

type TopicRecipe = { guid: string; title: string; description?: string; status: string; priority?: string; labels?: string[]; creationDate?: string; creationAuthor?: string; comments?: { guid: string; date: string; author: string; text: string; viewpointRef?: string }[]; viewpoints?: { guid: string; camera?: { kind: "perspective"; x: number; y: number; z: number; fov: number }; components?: { selection: string[] } }[] };

function markupBytes(topic: TopicRecipe): string {
  const children: Record<string, unknown> = { "@_Guid": topic.guid, "@_TopicStatus": topic.status };
  if (topic.title !== undefined) children["Title"] = topic.title;
  if (topic.priority !== undefined) children["Priority"] = topic.priority;
  if (topic.labels) children["Labels"] = topic.labels;
  if (topic.creationDate !== undefined) children["CreationDate"] = topic.creationDate;
  if (topic.creationAuthor !== undefined) children["CreationAuthor"] = topic.creationAuthor;
  if (topic.description !== undefined) children["Description"] = topic.description;
  const root: Record<string, unknown> = { Markup: { Topic: children } };
  const comments = topic.comments ?? [];
  if (comments.length > 0) {
    (root["Markup"] as Record<string, unknown>)["Comment"] = comments.map((c) => ({ "@_Guid": c.guid, Date: c.date, Author: c.author, Comment: c.text, ...(c.viewpointRef ? { Viewpoint: { "@_Guid": c.viewpointRef } } : {}) }));
  }
  const viewpoints = topic.viewpoints ?? [];
  if (viewpoints.length > 0) {
    (root["Markup"] as Record<string, unknown>)["Viewpoints"] = viewpoints.map((v) => ({ "@_Guid": v.guid, Viewpoint: `${v.guid}.bcfv` }));
  }
  return xmlPart(root);
}

function visInfoBytes(viewpoint: NonNullable<TopicRecipe["viewpoints"]>[number]): string {
  const children: Record<string, unknown> = {};
  if (viewpoint.components) children["Components"] = { Selection: { Component: viewpoint.components.selection.map((guid) => ({ "@_IfcGuid": guid })) }, Visibility: { "@_DefaultVisibility": "true" } };
  if (viewpoint.camera) {
    children["PerspectiveCamera"] = {
      CameraViewPoint: { "@_X": viewpoint.camera.x, "@_Y": viewpoint.camera.y, "@_Z": viewpoint.camera.z },
      CameraDirection: { "@_X": 0, "@_Y": 0, "@_Z": -1 },
      CameraUpVector: { "@_X": 0, "@_Y": 1, "@_Z": 0 },
      FieldOfView: viewpoint.camera.fov,
    };
  }
  return xmlPart({ VisualizationInfo: { "@_Guid": viewpoint.guid, ...children } });
}

// 📌️ Fixed epoch for every zip entry: `jszip` stamps each entry's DOS date/time from `Date.now()`
// by default, which makes two runs of an otherwise byte-identical recipe differ in their ZIP LOCAL
// FILE HEADER alone — `fixture reproduce` compares raw bytes, so an undated entry would fail
// reproducibility for a reason that has nothing to do with this recipe's own content.
const FIXED_DATE = new Date(Date.UTC(2026, 0, 1, 0, 0, 0));

async function buildBcf(version: string, topics: TopicRecipe[]): Promise<Buffer> {
  const zip = new JSZip();
  zip.file("bcf.version", xmlPart({ Version: { "@_VersionId": version, DetailedVersion: version } }), { date: FIXED_DATE });
  for (const topic of topics) {
    // 📌️ `jszip` auto-creates an implicit parent-folder entry for a nested path (`sub/` before
    // `sub/a.txt`) and stamps THAT entry with `new Date()`, ignoring the child's own `date` option —
    // confirmed empirically (`sub/a.txt` alone was non-reproducible across a 2-second gap; an
    // explicit dated folder entry created FIRST fixed it). Every topic folder needs one.
    zip.file(`${topic.guid}/`, null, { dir: true, date: FIXED_DATE });
    zip.file(`${topic.guid}/markup.bcf`, markupBytes(topic), { date: FIXED_DATE });
    for (const viewpoint of topic.viewpoints ?? []) zip.file(`${topic.guid}/${viewpoint.guid}.bcfv`, visInfoBytes(viewpoint), { date: FIXED_DATE });
  }
  return zip.generateAsync({ type: "nodebuffer", compression: "DEFLATE" });
}
//#endregion 🧬️Contract

//#region 🍳️Recipes
const BASE_TOPICS: TopicRecipe[] = [
  {
    guid: "topic-clash-01",
    title: "Beam clash at grid C4",
    description: "Structural beam intersects HVAC duct.",
    status: "Open",
    priority: "High",
    labels: ["structural", "mep"],
    creationDate: "2026-01-05T09:00:00Z",
    creationAuthor: "alice@example.com",
    comments: [{ guid: "comment-01", date: "2026-01-05T09:10:00Z", author: "alice@example.com", text: "Please review.", viewpointRef: "viewpoint-01" }],
    viewpoints: [{ guid: "viewpoint-01", camera: { kind: "perspective", x: 10, y: 5, z: 2, fov: 60 }, components: { selection: ["ifc-beam-1", "ifc-duct-1"] } }],
  },
  { guid: "topic-review-02", title: "Facade panel review", status: "In Progress", priority: "Medium", creationDate: "2026-01-06T09:00:00Z", creationAuthor: "bob@example.com" },
];

type Recipe = { id: string; outcome: "applied" | "rejected"; build: () => { before: TopicRecipe[]; after?: TopicRecipe[]; beforeVersion?: string; afterVersion?: string } };

const RECIPES: Recipe[] = [
  { id: "no-mutation-applied", outcome: "applied", build: () => ({ before: BASE_TOPICS, after: BASE_TOPICS }) },
  { id: "set-version-applied", outcome: "applied", build: () => ({ before: BASE_TOPICS, after: BASE_TOPICS, beforeVersion: "2.1", afterVersion: "2.2" }) },
  {
    id: "insert-topic-applied",
    outcome: "applied",
    build: () => ({ before: BASE_TOPICS, after: [...BASE_TOPICS, { guid: "topic-new-03", title: "New topic", status: "Open", creationDate: "2026-01-07T09:00:00Z", creationAuthor: "carol@example.com" }] }),
  },
  { id: "insert-topic-rejected-duplicate", outcome: "rejected", build: () => ({ before: BASE_TOPICS }) },
  { id: "remove-topic-applied", outcome: "applied", build: () => ({ before: BASE_TOPICS, after: BASE_TOPICS.filter((t) => t.guid !== "topic-review-02") }) },
  { id: "remove-topic-rejected-missing", outcome: "rejected", build: () => ({ before: BASE_TOPICS }) },
  {
    id: "set-topic-markup-applied",
    outcome: "applied",
    build: () => ({ before: BASE_TOPICS, after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, status: "Closed", priority: "Low" } : t)) }),
  },
  {
    id: "insert-comment-applied",
    outcome: "applied",
    build: () => ({
      before: BASE_TOPICS,
      after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, comments: [...(t.comments ?? []), { guid: "comment-02", date: "2026-01-05T11:00:00Z", author: "bob@example.com", text: "Confirmed, rerouting duct." }] } : t)),
    }),
  },
  { id: "remove-comment-applied", outcome: "applied", build: () => ({ before: BASE_TOPICS, after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, comments: [] } : t)) }) },
  {
    id: "set-comment-applied",
    outcome: "applied",
    build: () => ({
      before: BASE_TOPICS,
      after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, comments: (t.comments ?? []).map((c) => (c.guid === "comment-01" ? { ...c, text: "Please review — updated." } : c)) } : t)),
    }),
  },
  {
    id: "insert-viewpoint-applied",
    outcome: "applied",
    build: () => ({
      before: BASE_TOPICS,
      after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, viewpoints: [...(t.viewpoints ?? []), { guid: "viewpoint-02", camera: { kind: "perspective", x: -3, y: 8, z: 1, fov: 45 } }] } : t)),
    }),
  },
  { id: "remove-viewpoint-applied", outcome: "applied", build: () => ({ before: BASE_TOPICS, after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, viewpoints: [] } : t)) }) },
  { id: "remove-viewpoint-rejected-missing", outcome: "rejected", build: () => ({ before: BASE_TOPICS }) },
  {
    id: "set-viewpoint-camera-applied",
    outcome: "applied",
    build: () => ({
      before: BASE_TOPICS,
      after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, viewpoints: (t.viewpoints ?? []).map((v) => (v.guid === "viewpoint-01" ? { ...v, camera: { kind: "perspective" as const, x: 20, y: 20, z: 20, fov: 90 } } : v)) } : t)),
    }),
  },
  {
    id: "set-viewpoint-components-applied",
    outcome: "applied",
    build: () => ({
      before: BASE_TOPICS,
      after: BASE_TOPICS.map((t) => (t.guid === "topic-clash-01" ? { ...t, viewpoints: (t.viewpoints ?? []).map((v) => (v.guid === "viewpoint-01" ? { ...v, components: { selection: ["ifc-beam-1"] } } : v)) } : t)),
    }),
  },
];
//#endregion 🍳️Recipes

//#region 🚀️Entry
async function generateOne(id: string, outDir: string): Promise<void> {
  const recipe = RECIPES.find((entry) => entry.id === id);
  if (!recipe) throw new Error(`unknown recipe ${id} — known: ${RECIPES.map((entry) => entry.id).join(", ")}`);
  const { before, after, beforeVersion, afterVersion } = recipe.build();
  const dir = join(outDir, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "before.bcf"), await buildBcf(beforeVersion ?? "2.1", before));
  if (recipe.outcome === "applied") {
    if (!after) throw new Error(`recipe ${id} is declared applied but has no after state`);
    writeFileSync(join(dir, "after.bcf"), await buildBcf(afterVersion ?? beforeVersion ?? "2.1", after));
  }
}

async function main(argv: readonly string[]): Promise<number> {
  const [command, ...rest] = argv;
  if (command !== "generate") {
    console.error(`usage: bun 📜️script.ts generate [--only <fixture-id>]`);
    return 2;
  }
  const onlyIndex = rest.indexOf("--only");
  const only = onlyIndex >= 0 ? rest[onlyIndex + 1] : undefined;
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? join(process.cwd(), "🧫️fixtures");
  const ids = only ? [only] : RECIPES.map((entry) => entry.id);
  for (const id of ids) {
    await generateOne(id, outDir);
    console.log(`[bcf generator] ${id} -> ${join(outDir, id)}`);
  }
  return 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
