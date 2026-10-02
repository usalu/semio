// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🟦️ Independent TypeScript implementation of `os.config.local-folders`' two-kind mutation vocabulary — the second
 * producer the recorded no-oracle decision `os-config-local-folders-mutation-semantics`
 * (`../../../../../🎚️config/🔮️oracles/🔣️.json`) claims via its `independent-implementations` substitute. It drives the
 * authoritative TypeScript leaves (`../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts`) as a second SUBJECT over the
 * identical committed `(before, mutation, after, outcome)` vectors the Rust adapter reads literally.
 *
 * @see ../🥒️.feature
 * @see ../🦀️.rs — the Rust subject and the no-oracle "oracle" role (the committed vectors, read literally)
 */

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { applyLocalFoldersConfigMutation, inverseLocalFoldersConfigMutation, type LocalFolderBindings, type LocalFoldersConfigMutation } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔖️Fixtures
const HERE = dirname(fileURLToPath(import.meta.url));
const CONFIG_MUTATIONS = join(HERE, "../../../../../🎚️config/🧬️schema/🧬️mutations");

const FIXTURE_DIR: Record<string, string> = {
  "attach-local-folder": join(CONFIG_MUTATIONS, "📎️attach-local-folder/🧫️fixtures/📎️remembers-the-folder-beside-another-document"),
  "detach-local-folder": join(CONFIG_MUTATIONS, "✂️detach-local-folder/🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling"),
};

type Vectors = { before: LocalFolderBindings; mutation: LocalFoldersConfigMutation; after: LocalFolderBindings; outcome: { status: string } };

/** 🧫️ The committed specification vector for one kind, read literally. Mirrors `../🦀️.rs::fixture_text`. */
function fixtures(kind: string): Vectors {
  const dir = FIXTURE_DIR[kind];
  if (dir === undefined) throw new Error(`mutate-os-config-local-folders (typescript): no specification vector registered for kind ${JSON.stringify(kind)}`);
  return {
    before: JSON.parse(readFileSync(join(dir, "📸️snapshot/⬅️before/🔣️.json"), "utf8")) as LocalFolderBindings,
    mutation: JSON.parse(readFileSync(join(dir, "🦠️mutation/🔣️.json"), "utf8")) as LocalFoldersConfigMutation,
    after: JSON.parse(readFileSync(join(dir, "📸️snapshot/➡️after/🔣️.json"), "utf8")) as LocalFolderBindings,
    outcome: JSON.parse(readFileSync(join(dir, "🎯️outcome/🔣️.json"), "utf8")) as { status: string },
  };
}

const bound = (bindings: LocalFolderBindings, documentId: string): boolean => bindings.bindings.some((entry) => entry.documentId === documentId);

function projectionOf(bindings: LocalFolderBindings, operation: string): AdapterOutcome {
  return { raw: JSON.stringify(bindings), projection: bindings, productionDispatch: { invoked: true, operation, bridgeVersion: 1 } };
}

function whenPayload(ctx: AdapterContext): { kind: string; document: string; bound: string; sibling: string } {
  const when = ctx.scenario.steps.find((step) => step.keyword === "When" && step.docString !== undefined)?.docString;
  if (when === undefined) throw new Error(`${ctx.scenario.id}: no When docString to read the kind/document/bound/sibling row from`);
  return JSON.parse(when) as { kind: string; document: string; bound: string; sibling: string };
}
// #endregion 🔖️Fixtures

// #region 🎯️Handlers
/** 🎯️ Applies the kind through the TypeScript leaf and asserts the committed after-bindings, the binding claim and the
 * untouched sibling. Mirrors `../🦀️.rs::subject::mutate`. */
function mutate(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation, after, outcome } = fixtures(row.kind);
  const applied = applyLocalFoldersConfigMutation(before, mutation);
  if (JSON.stringify(applied) !== JSON.stringify(after)) throw new Error(`mutate-${row.kind}: the TypeScript-applied bindings do not match the committed after-bindings\n     got: ${JSON.stringify(applied)}\nexpected: ${JSON.stringify(after)}`);
  if (JSON.stringify(applied) === JSON.stringify(before)) throw new Error(`mutate-${row.kind}: the mutation left the bindings unchanged`);
  if (bound(applied, row.document) !== (row.bound === "yes")) throw new Error(`mutate-${row.kind}: the feature declares ${JSON.stringify(row.document)} bound = ${row.bound} afterwards, but the bindings say otherwise`);
  const sibling = (bindings: LocalFolderBindings) => JSON.stringify(bindings.bindings.find((entry) => entry.documentId === row.sibling) ?? null);
  if (sibling(before) === "null" || sibling(before) !== sibling(applied)) throw new Error(`mutate-${row.kind}: the sibling ${JSON.stringify(row.sibling)} must survive untouched`);
  if (outcome.status !== "applied") throw new Error(`mutate-${row.kind}: both committed local-folders vectors are clean applied vectors, but this one declares ${JSON.stringify(outcome.status)}`);
  return projectionOf(applied, row.kind);
}

/** ↩️ Applying the kind and then its OWN computed inverse restores the committed before-bindings exactly. Mirrors
 * `../🦀️.rs::subject::inverse`. */
function inverse(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation } = fixtures(row.kind);
  let current = applyLocalFoldersConfigMutation(before, mutation);
  if (JSON.stringify(current) === JSON.stringify(before)) throw new Error(`inverse-${row.kind}: the forward mutation left the bindings untouched, so restoring them proves nothing`);
  for (const step of inverseLocalFoldersConfigMutation(mutation, before)) current = applyLocalFoldersConfigMutation(current, step);
  if (JSON.stringify(current) !== JSON.stringify(before)) throw new Error(`inverse law violated: applying ${row.kind} and then its own inverse did not restore the original\n     got: ${JSON.stringify(current)}\nexpected: ${JSON.stringify(before)}`);
  return projectionOf(current, row.kind);
}

/** 🚧️ Detaching an unbound document changes nothing and offers no undo step. Mirrors `../🦀️.rs::subject::unbound_guard`. */
function unboundGuard(_ctx: AdapterContext): AdapterOutcome {
  const { mutation, after: base } = fixtures("detach-local-folder");
  if (bound(base, "cad.drawing.fixture")) throw new Error("unbound-detachment-has-no-undo: the committed after-bindings of the detach vector must no longer bind cad.drawing.fixture");
  const current = applyLocalFoldersConfigMutation(base, mutation);
  if (JSON.stringify(current) !== JSON.stringify(base)) throw new Error("unbound-detachment-has-no-undo: detaching an unbound document must leave the bindings exactly where they were");
  const steps = inverseLocalFoldersConfigMutation(mutation, base);
  if (steps.length !== 0) throw new Error(`unbound-detachment-has-no-undo: the inverse must be empty, but the TypeScript leaf offered ${steps.length} step(s)`);
  return projectionOf(current, "detach-local-folder");
}

/** 🔁️ Decode/re-encode round trip of the committed two-binding record. Mirrors `../🦀️.rs::subject::round_trip`. */
function localFoldersRoundTrip(_ctx: AdapterContext): AdapterOutcome {
  const { before } = fixtures("detach-local-folder");
  if (before.bindings.map((entry) => entry.documentId).join(",") !== "cad.drawing.fixture,puzzle.2d.fixture" || before.bindings[1]!.folder.path !== "/Users/ada/Documents/puzzles") throw new Error(`local-folders-round-trip: the committed bindings attach cad.drawing.fixture and puzzle.2d.fixture to /Users/ada/Documents/puzzles, but the decoded value holds ${JSON.stringify(before)}`);
  const reencoded = JSON.parse(JSON.stringify(before)) as LocalFolderBindings;
  if (JSON.stringify(reencoded) !== JSON.stringify(before)) throw new Error("local-folders-round-trip: decoding the re-encoded bindings did not reproduce the typed value");
  return projectionOf(reencoded, "detach-local-folder");
}
// #endregion 🎯️Handlers

// #region 🧭️Adapter
/** 🧭️ Registration by FULL expanded scenario id, mirroring the feature's `Examples` tables and `../🦀️.rs::adapter`. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "mutate-attach-local-folder": { subject: mutate },
    "mutate-detach-local-folder": { subject: mutate },
    "inverse-attach-local-folder": { subject: inverse },
    "inverse-detach-local-folder": { subject: inverse },
    "unbound-detachment-has-no-undo": { subject: unboundGuard },
    "local-folders-round-trip": { subject: localFoldersRoundTrip },
  },
});
// #endregion 🧭️Adapter
