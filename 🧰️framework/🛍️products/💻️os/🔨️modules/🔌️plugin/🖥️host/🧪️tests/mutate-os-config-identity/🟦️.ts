// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🟦️ Independent TypeScript implementation of `os.config.identity`'s two-kind mutation vocabulary —
 * the second producer the recorded no-oracle decision `os-config-identity-mutation-semantics`
 * (`../../../../../🎚️config/🧪️oracle/🔣️.json`) claims via its `independent-implementations`
 * substitute. `applyIdentityConfigMutation`/`inverseIdentityConfigMutation`
 * (`../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts`) are the authoritative direct TypeScript
 * leaves for `signIn`/`signOut` — written independently of `../../../🖥️host/📦️packages/🦀️rust`'s
 * Rust dispatch, from the same committed specification the Rust adapter reads. This file drives THAT
 * TypeScript dispatch as a second SUBJECT, over the identical committed
 * `(before, mutation, after, outcome)` vectors the Rust adapter reads literally — never recomputed,
 * never re-derived — so the two implementations are compared against the same fixed evidence rather
 * than against each other's opinion of it.
 *
 * @see ../🥒️.feature
 * @see ../🦀️.rs — the Rust subject and the no-oracle "oracle" role (the committed vectors, read literally)
 */

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { applyIdentityConfigMutation, inverseIdentityConfigMutation, type Identity, type IdentityConfigMutation } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔖️Fixtures
const HERE = dirname(fileURLToPath(import.meta.url));
const CONFIG_MUTATIONS = join(HERE, "../../../../../🎚️config/🧬️schema/🧬️mutations");

const FIXTURE_DIR: Record<string, string> = {
  "sign-in": join(CONFIG_MUTATIONS, "🪪️sign-in/🧪️tests/replaces-the-active-session-with-a-second-account"),
  "sign-out": join(CONFIG_MUTATIONS, "🚪️sign-out/🧪️tests/clears-the-active-session"),
};

type Vectors = { before: Identity | null; mutation: IdentityConfigMutation; after: Identity | null; outcome: { status: string } };

/** 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
 * literally — this IS the independently handcrafted vector the no-oracle decision rests on, never
 * recomputed. Mirrors `../🦀️.rs::fixture_text`. */
function fixtures(kind: string): Vectors {
  const dir = FIXTURE_DIR[kind];
  if (dir === undefined) throw new Error(`mutate-os-config-identity (typescript): no specification vector registered for kind ${JSON.stringify(kind)}`);
  return {
    before: JSON.parse(readFileSync(join(dir, "📸️snapshot/⬅️before/🔣️.json"), "utf8")) as Identity | null,
    mutation: JSON.parse(readFileSync(join(dir, "🦠️mutation/🔣️.json"), "utf8")) as IdentityConfigMutation,
    after: JSON.parse(readFileSync(join(dir, "📸️snapshot/➡️after/🔣️.json"), "utf8")) as Identity | null,
    outcome: JSON.parse(readFileSync(join(dir, "🎯️outcome/🔣️.json"), "utf8")) as { status: string },
  };
}

function account(record: Identity | null): string {
  return record?.userId ?? "none";
}

function projectionOf(record: Identity | null): AdapterOutcome {
  const text = JSON.stringify(record);
  return { raw: text, projection: record, productionDispatch: { invoked: true, operation: record === null ? "sign-out" : "sign-in", bridgeVersion: 1 } };
}

function whenPayload(ctx: AdapterContext): { kind: string; account: string; wasAccount: string } {
  const when = ctx.scenario.steps.find((step) => step.keyword === "When" && step.docString !== undefined)?.docString;
  if (when === undefined) throw new Error(`${ctx.scenario.id}: no When docString to read the kind/account/wasAccount row from`);
  return JSON.parse(when) as { kind: string; account: string; wasAccount: string };
}
// #endregion 🔖️Fixtures

// #region 🎯️Handlers
/** 🎯️ Applies the kind to the committed before-record through the TypeScript leaf and asserts, in
 * role, that the result IS the committed after-record and that the declared session claim holds —
 * the same laws `../🦀️.rs::subject::mutate` asserts, against the SAME committed evidence. */
function mutate(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation, after, outcome } = fixtures(row.kind);
  if (account(before) !== row.wasAccount) throw new Error(`mutate-${row.kind}: the feature declares the record starts signed in as ${JSON.stringify(row.wasAccount)}, but the committed before-record holds ${JSON.stringify(account(before))}`);
  const applied = applyIdentityConfigMutation(before, mutation);
  if (JSON.stringify(applied) !== JSON.stringify(after)) throw new Error(`mutate-${row.kind}: the TypeScript-applied record does not match the committed after-record\n     got: ${JSON.stringify(applied)}\nexpected: ${JSON.stringify(after)}`);
  if (JSON.stringify(applied) === JSON.stringify(before)) throw new Error(`mutate-${row.kind}: the mutation left the identity record unchanged — the scenario would report a pass for a mutation it never observed`);
  if (account(applied) !== row.account) throw new Error(`mutate-${row.kind}: the feature declares the record holds ${JSON.stringify(row.account)} afterwards, but it holds ${JSON.stringify(account(applied))}`);
  if (row.account !== "none" && applied?.userId === before?.userId) throw new Error(`mutate-${row.kind}: a replaced identity must carry its own user id, but the prior identity survived`);
  if (outcome.status !== "applied") throw new Error(`mutate-${row.kind}: both committed identity vectors are clean applied vectors, but this one declares ${JSON.stringify(outcome.status)}`);
  return projectionOf(applied);
}

/** ↩️ The metamorphic inverse law, driven by the TypeScript leaf: applying the kind and then its OWN
 * computed inverse must restore the committed before-record exactly. Mirrors `../🦀️.rs::subject::inverse`. */
function inverse(ctx: AdapterContext): AdapterOutcome {
  const row = whenPayload(ctx);
  const { before, mutation } = fixtures(row.kind);
  let current = applyIdentityConfigMutation(before, mutation);
  if (JSON.stringify(current) === JSON.stringify(before)) throw new Error(`inverse-${row.kind}: the forward mutation left the record untouched, so restoring it proves nothing`);
  for (const step of inverseIdentityConfigMutation(mutation, before)) current = applyIdentityConfigMutation(current, step);
  if (JSON.stringify(current) !== JSON.stringify(before)) throw new Error(`inverse law violated: applying ${row.kind} and then its own inverse did not restore the original\n     got: ${JSON.stringify(current)}\nexpected: ${JSON.stringify(before)}`);
  if (account(current) !== row.wasAccount) throw new Error(`inverse-${row.kind}: the restored record must hold ${JSON.stringify(row.wasAccount)} once more, but it holds ${JSON.stringify(account(current))}`);
  if (current?.userId !== before?.userId) throw new Error(`inverse-${row.kind}: the undo restored the account but fabricated a user id`);
  return projectionOf(current);
}

/** 🚧️ The branch no committed vector can express: an inverse read off a record that holds no
 * session must be EMPTY. Mirrors `../🦀️.rs::subject::signed_out_guard`. */
function signedOutGuard(_ctx: AdapterContext): AdapterOutcome {
  const { mutation, after: base } = fixtures("sign-out");
  if (base !== null) throw new Error("signed-out-inverse-is-empty: the committed after-record of the sign-out vector must hold no session");
  const current = applyIdentityConfigMutation(base, mutation);
  if (current !== null) throw new Error("signed-out-inverse-is-empty: signing out of a signed-out record must leave it exactly where it was");
  const steps = inverseIdentityConfigMutation(mutation, base);
  if (steps.length !== 0) throw new Error(`signed-out-inverse-is-empty: the inverse must be empty, but the TypeScript leaf offered ${steps.length} step(s)`);
  return projectionOf(current);
}

/** 🔁️ The identity law for a record whose only carrier is its own JSON projection, driven by the
 * TypeScript decode/re-encode round trip. Mirrors `../🦀️.rs::subject::round_trip`. */
function identityRoundTrip(_ctx: AdapterContext): AdapterOutcome {
  const { before } = fixtures("sign-in");
  if (account(before) !== "ada" || before?.userId !== "user-ada") throw new Error(`identity-round-trip: the committed record holds Ada's identity, but the decoded value holds ${JSON.stringify(before)}`);
  const reencoded = JSON.parse(JSON.stringify(before)) as Identity;
  if (JSON.stringify(reencoded) !== JSON.stringify(before)) throw new Error("identity-round-trip: decoding the re-encoded record did not reproduce the typed value");
  return projectionOf(reencoded);
}
// #endregion 🎯️Handlers

// #region 🧭️Adapter
/** 🧭️ Registration by FULL expanded scenario id, mirroring the feature's `Examples` tables and
 * `../🦀️.rs::adapter`'s own registration exactly — every scenario the Rust file serves as `subject`,
 * this file serves as `subject` too, so the case genuinely has two implementations. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "mutate-sign-in": { subject: mutate },
    "mutate-sign-out": { subject: mutate },
    "inverse-sign-in": { subject: inverse },
    "inverse-sign-out": { subject: inverse },
    "signed-out-inverse-is-empty": { subject: signedOutGuard },
    "identity-round-trip": { subject: identityRoundTrip },
  },
});
// #endregion 🧭️Adapter
