// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🎨️ Independent TypeScript implementation of `os.config.ui-preferences`' nine-kind mutation
 * vocabulary, driven as a second SUBJECT. `applyUiPreferencesConfigMutation`/`inverseUiPreferencesConfigMutation`
 * (`../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts`) are the authoritative direct TypeScript leaves,
 * written independently of the Rust dispatch the host package re-exports. This file applies THAT
 * TypeScript dispatch to the identical committed `(before, mutation, after, outcome)` vectors the Rust
 * adapter reads literally — never recomputed — so both implementations answer to the same fixed evidence.
 * The TypeScript leaves raise no diagnostics, so a `no-op` vector is held to its observable half: the
 * record comes back unchanged.
 *
 * @see ../🥒️.feature
 * @see ../🦀️.rs — the Rust subject and the no-oracle "oracle" role (the committed vectors, read literally)
 */

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import type { UiPreferences } from "../../../../../🎚️config/🧬️schema/🟦️.ts";
import { applyUiPreferencesConfigMutation, inverseUiPreferencesConfigMutation, type UiPreferencesConfigMutation } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔖️Fixtures
const CONFIG_MUTATIONS = join(dirname(fileURLToPath(import.meta.url)), "../../../../../🎚️config/🧬️schema/🧬️mutations");

/** 🏷️ Every committed vector as `scenario id → [leaf directory, vector directory]`, mirroring `../🦀️.rs::fixture_text`. */
const VECTOR_DIR: Record<string, readonly [string, string]> = {
  "sets-appearance": ["🌗️set-appearance", "✏️sets-appearance"],
  "keeps-appearance": ["🌗️set-appearance", "🟰️keeps-appearance"],
  "sets-layout": ["📐️set-layout", "✏️sets-layout"],
  "keeps-layout": ["📐️set-layout", "🟰️keeps-layout"],
  "sets-driver": ["🕹️set-driver", "✏️sets-driver"],
  "keeps-driver": ["🕹️set-driver", "🟰️keeps-driver"],
  "sets-custom-driver": ["🚗️set-custom-driver", "✏️sets-custom-driver"],
  "keeps-custom-driver": ["🚗️set-custom-driver", "🟰️keeps-custom-driver"],
  "sets-locale": ["🗣️set-locale", "✏️sets-locale"],
  "keeps-locale": ["🗣️set-locale", "🟰️keeps-locale"],
  "sets-terminology": ["📖️set-terminology", "✏️sets-terminology"],
  "keeps-terminology": ["📖️set-terminology", "🟰️keeps-terminology"],
  "sets-theme": ["🖼️set-theme", "✏️sets-theme"],
  "keeps-theme": ["🖼️set-theme", "🟰️keeps-theme"],
  "sets-custom-theme": ["🎨️set-custom-theme", "✏️sets-custom-theme"],
  "keeps-custom-theme": ["🎨️set-custom-theme", "🟰️keeps-custom-theme"],
  "sets-keybinding": ["⌨️set-keybinding-override", "✏️sets-keybinding"],
  "keeps-keybinding": ["⌨️set-keybinding-override", "🟰️keeps-keybinding"],
};

const MEMBERS = ["appearance", "layout", "driverId", "customDrivers", "locale", "terminology", "themeId", "customThemes", "keybindingOverrides"] as const;

type Vectors = { before: UiPreferences; mutation: UiPreferencesConfigMutation; after: UiPreferences; outcome: { status: string } };
type Row = { vector: string; kind: string; field: string; status: string };

/** 🧫️ The committed specification vector for one scenario, read literally. */
function fixtures(scenario: string): Vectors {
  const entry = VECTOR_DIR[scenario];
  if (entry === undefined) throw new Error(`mutate-os-config-ui-preferences (typescript): no specification vector registered for scenario ${JSON.stringify(scenario)}`);
  const dir = join(CONFIG_MUTATIONS, entry[0], "🧫️fixtures", entry[1]);
  const read = (path: string): unknown => JSON.parse(readFileSync(join(dir, path), "utf8"));
  return { before: read("📸️snapshot/⬅️before/🔣️.json") as UiPreferences, mutation: read("🦠️mutation/🔣️.json") as UiPreferencesConfigMutation, after: read("📸️snapshot/➡️after/🔣️.json") as UiPreferences, outcome: read("🎯️outcome/🔣️.json") as { status: string } };
}

/** 🔤️ Canonical text with object members in key order, so comparisons state content rather than insertion order. */
function canonical(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value !== null && typeof value === "object") return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical((value as Record<string, unknown>)[key])}`).join(",")}}`;
  return JSON.stringify(value);
}

function movedFields(before: UiPreferences, after: UiPreferences): string[] {
  return MEMBERS.filter((member) => canonical(before[member]) !== canonical(after[member]));
}

function projectionOf(record: UiPreferences, operation: string): AdapterOutcome {
  return { raw: JSON.stringify(record), projection: record, productionDispatch: { invoked: true, operation, bridgeVersion: 1 } };
}

function whenPayload(ctx: AdapterContext): Row {
  const when = ctx.scenario.steps.find((step) => step.keyword === "When" && step.docString !== undefined)?.docString;
  if (when === undefined) throw new Error(`${ctx.scenario.id}: no When docString to read the vector/kind/field/status row from`);
  return JSON.parse(when) as Row;
}
// #endregion 🔖️Fixtures

// #region 🎯️Handlers
/** 🎯️ Applies the vector through the TypeScript leaf and asserts the committed after-record, the
 * `field` claim and the committed status — the laws `../🦀️.rs::subject::mutate` asserts. */
function mutate(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation, after, outcome } = fixtures(row.vector);
  const applied = applyUiPreferencesConfigMutation(before, mutation);
  if (canonical(applied) !== canonical(after)) throw new Error(`mutate-${row.vector}: the TypeScript-applied record does not match the committed after-record\n     got: ${canonical(applied)}\nexpected: ${canonical(after)}`);
  if (outcome.status !== row.status) throw new Error(`mutate-${row.vector}: the feature declares a ${JSON.stringify(row.status)} vector, but the committed outcome declares ${JSON.stringify(outcome.status)}`);
  const expected = row.status === "applied" ? [row.field] : [];
  const moved = movedFields(before, applied);
  if (canonical(moved) !== canonical(expected)) throw new Error(`mutate-${row.vector}: the feature declares the ${row.status} vector moves ${JSON.stringify(expected)}, but the TypeScript leaf moved ${JSON.stringify(moved)}`);
  return projectionOf(applied, row.kind);
}

/** ↩️ The metamorphic inverse law through the TypeScript leaf. Mirrors `../🦀️.rs::subject::inverse`. */
function inverse(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation } = fixtures(row.vector);
  let current = applyUiPreferencesConfigMutation(before, mutation);
  if (canonical(current) === canonical(before)) throw new Error(`inverse-${row.vector}: the forward mutation left the record untouched, so restoring it proves nothing`);
  for (const step of inverseUiPreferencesConfigMutation(mutation, before)) current = applyUiPreferencesConfigMutation(current, step);
  if (canonical(current) !== canonical(before)) throw new Error(`inverse law violated: applying ${row.vector} and then its own inverse did not restore the original\n     got: ${canonical(current)}\nexpected: ${canonical(before)}`);
  return projectionOf(current, row.kind);
}

/** 🔁️ The identity law for a record whose only carrier is its JSON projection. Mirrors `../🦀️.rs::subject::round_trip`. */
function identityRoundTrip(_ctx: AdapterContext): AdapterOutcome {
  const { before } = fixtures("keeps-keybinding");
  const scale = (before.customDrivers["studio"]?.config as { scale?: number } | undefined)?.scale;
  if (before.locale !== "de" || scale !== 1.25 || before.keybindingOverrides["edit.undo"] !== "Meta+Z") throw new Error(`identity-round-trip: the committed record holds locale de, driver scale 1.25 and Meta+Z for edit.undo, but the decoded value holds ${canonical(before)}`);
  const reencoded = JSON.parse(JSON.stringify(before)) as UiPreferences;
  if (canonical(reencoded) !== canonical(before)) throw new Error("identity-round-trip: decoding the re-encoded record did not reproduce the typed value");
  return projectionOf(reencoded, "set-keybinding-override");
}
// #endregion 🎯️Handlers

// #region 🧭️Adapter
/** 🧭️ Registration by FULL expanded scenario id, mirroring the feature's `Examples` tables and `../🦀️.rs::adapter`. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    ...Object.fromEntries(Object.keys(VECTOR_DIR).map((scenario) => [`mutate-${scenario}`, { subject: mutate }])),
    ...Object.fromEntries(Object.keys(VECTOR_DIR).filter((scenario) => scenario.startsWith("sets-")).map((scenario) => [`inverse-${scenario}`, { subject: inverse }])),
    "identity-round-trip": { subject: identityRoundTrip },
  },
});
// #endregion 🧭️Adapter
