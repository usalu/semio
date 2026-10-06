// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🟦️ Independent TypeScript implementation of `os.config.local-catalog`'s two-kind mutation vocabulary — the second
 * producer the recorded no-oracle decision `os-config-local-catalog-mutation-semantics`
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
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { applyLocalCatalogConfigMutation, inverseLocalCatalogConfigMutation, type LocalCatalog, type LocalCatalogConfigMutation } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔖️Fixtures
const HERE = dirname(fileURLToPath(import.meta.url));
const CONFIG_MUTATIONS = join(HERE, "../../../../../🎚️config/🧬️schema/🧬️mutations");

const FIXTURE_DIR: Record<string, string> = {
  "admit-local-document": join(CONFIG_MUTATIONS, "📥️admit-local-document/🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one"),
  "retire-local-document": join(CONFIG_MUTATIONS, "📤️retire-local-document/🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling"),
};

type Vectors = { before: LocalCatalog; mutation: LocalCatalogConfigMutation; after: LocalCatalog; outcome: { status: string } };

/** 🧫️ The committed specification vector for one kind, read literally. Mirrors `../🦀️.rs::fixture_text`. */
function fixtures(kind: string): Vectors {
  const dir = FIXTURE_DIR[kind];
  if (dir === undefined) throw new Error(`mutate-os-config-local-catalog (typescript): no specification vector registered for kind ${JSON.stringify(kind)}`);
  return {
    before: JSON.parse(readFileSync(join(dir, "📸️snapshot/⬅️before/🔣️.json"), "utf8")) as LocalCatalog,
    mutation: JSON.parse(readFileSync(join(dir, "🦠️mutation/🔣️.json"), "utf8")) as LocalCatalogConfigMutation,
    after: JSON.parse(readFileSync(join(dir, "📸️snapshot/➡️after/🔣️.json"), "utf8")) as LocalCatalog,
    outcome: JSON.parse(readFileSync(join(dir, "🎯️outcome/🔣️.json"), "utf8")) as { status: string },
  };
}

const listed = (catalog: LocalCatalog, documentId: string): boolean => catalog.documents.some((entry) => entry.documentId === documentId);

function projectionOf(catalog: LocalCatalog, operation: string): AdapterOutcome {
  return { raw: JSON.stringify(catalog), projection: catalog, productionDispatch: { invoked: true, operation, bridgeVersion: 1 } };
}

function whenPayload(ctx: AdapterContext): { kind: string; document: string; listed: string; sibling: string } {
  const when = ctx.scenario.steps.find((step) => step.keyword === "When" && step.docString !== undefined)?.docString;
  if (when === undefined) throw new Error(`${ctx.scenario.id}: no When docString to read the kind/document/listed/sibling row from`);
  return JSON.parse(when) as { kind: string; document: string; listed: string; sibling: string };
}
// #endregion 🔖️Fixtures

// #region 🎯️Handlers
/** 🎯️ Applies the kind through the TypeScript leaf and asserts the committed after-catalog, the listing claim and the
 * untouched sibling. Mirrors `../🦀️.rs::subject::mutate`. */
function mutate(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation, after, outcome } = fixtures(row.kind);
  const applied = applyLocalCatalogConfigMutation(before, mutation);
  if (JSON.stringify(applied) !== JSON.stringify(after)) throw new Error(`mutate-${row.kind}: the TypeScript-applied catalog does not match the committed after-catalog\n     got: ${JSON.stringify(applied)}\nexpected: ${JSON.stringify(after)}`);
  if (JSON.stringify(applied) === JSON.stringify(before)) throw new Error(`mutate-${row.kind}: the mutation left the catalog unchanged`);
  if (listed(applied, row.document) !== (row.listed === "yes")) throw new Error(`mutate-${row.kind}: the feature declares ${JSON.stringify(row.document)} listed = ${row.listed} afterwards, but the catalog says otherwise`);
  const sibling = (catalog: LocalCatalog) => JSON.stringify(catalog.documents.find((entry) => entry.documentId === row.sibling) ?? null);
  if (sibling(before) === "null" || sibling(before) !== sibling(applied)) throw new Error(`mutate-${row.kind}: the sibling ${JSON.stringify(row.sibling)} must survive untouched`);
  if (outcome.status !== "applied") throw new Error(`mutate-${row.kind}: both committed local-catalog vectors are clean applied vectors, but this one declares ${JSON.stringify(outcome.status)}`);
  return projectionOf(applied, row.kind);
}

/** ↩️ Applying the kind and then its OWN computed inverse restores the committed before-catalog exactly. Mirrors
 * `../🦀️.rs::subject::inverse`. */
function inverse(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation } = fixtures(row.kind);
  let current = applyLocalCatalogConfigMutation(before, mutation);
  if (JSON.stringify(current) === JSON.stringify(before)) throw new Error(`inverse-${row.kind}: the forward mutation left the catalog untouched, so restoring it proves nothing`);
  for (const step of inverseLocalCatalogConfigMutation(mutation, before)) current = applyLocalCatalogConfigMutation(current, step);
  if (JSON.stringify(current) !== JSON.stringify(before)) throw new Error(`inverse law violated: applying ${row.kind} and then its own inverse did not restore the original\n     got: ${JSON.stringify(current)}\nexpected: ${JSON.stringify(before)}`);
  return projectionOf(current, row.kind);
}

/** 🚧️ Retiring an unlisted id changes nothing and offers no undo step. Mirrors `../🦀️.rs::subject::unlisted_guard`. */
function unlistedGuard(_ctx: AdapterContext): AdapterOutcome {
  const { mutation, after: base } = fixtures("retire-local-document");
  if (listed(base, "studio-alpha")) throw new Error("unlisted-retirement-has-no-undo: the committed after-catalog of the retire vector must no longer list studio-alpha");
  const current = applyLocalCatalogConfigMutation(base, mutation);
  if (JSON.stringify(current) !== JSON.stringify(base)) throw new Error("unlisted-retirement-has-no-undo: retiring an unlisted id must leave the catalog exactly where it was");
  const steps = inverseLocalCatalogConfigMutation(mutation, base);
  if (steps.length !== 0) throw new Error(`unlisted-retirement-has-no-undo: the inverse must be empty, but the TypeScript leaf offered ${steps.length} step(s)`);
  return projectionOf(current, "retire-local-document");
}

/** 🔁️ Decode/re-encode round trip of the committed two-entry catalog. Mirrors `../🦀️.rs::subject::round_trip`. */
function localCatalogRoundTrip(_ctx: AdapterContext): AdapterOutcome {
  const { before } = fixtures("retire-local-document");
  if (before.documents.map((entry) => entry.documentId).join(",") !== "studio-alpha,studio-imported" || before.documents[1]!.storage !== "file") throw new Error(`local-catalog-round-trip: the committed catalog lists studio-alpha and a file-stored studio-imported, but the decoded value holds ${JSON.stringify(before)}`);
  const reencoded = JSON.parse(JSON.stringify(before)) as LocalCatalog;
  if (JSON.stringify(reencoded) !== JSON.stringify(before)) throw new Error("local-catalog-round-trip: decoding the re-encoded catalog did not reproduce the typed value");
  return projectionOf(reencoded, "retire-local-document");
}
// #endregion 🎯️Handlers

// #region 🧭️Adapter
/** 🧭️ Registration by FULL expanded scenario id, mirroring the feature's `Examples` tables and `../🦀️.rs::adapter`. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "mutate-admit-local-document": { subject: mutate },
    "mutate-retire-local-document": { subject: mutate },
    "inverse-admit-local-document": { subject: inverse },
    "inverse-retire-local-document": { subject: inverse },
    "unlisted-retirement-has-no-undo": { subject: unlistedGuard },
    "local-catalog-round-trip": { subject: localCatalogRoundTrip },
  },
});
// #endregion 🧭️Adapter
