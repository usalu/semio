/** ⚙️ LAW: the TypeScript twin of the central applier folds the OS config facets by the SAME sparse diff Rust names. Over the
 * committed fixtures next to each leaf (`🧬️mutations/*\/🧫️fixtures/*`, written by the Rust leaves and read literally here) every leaf's `diff` is
 * exactly the committed `🔺️diff`, the twin applier turns it into exactly the committed `➡️after`, and the concrete inverse
 * mutations' diffs sum (`absorb`, replayed last to first) to the negative diff read off the base and restore `⬅️before`. A
 * seeded random history of admissions, replacements and retirements holds the same laws row by row and undoes to its start. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  admitLocalDocument,
  applyIdentityConfigMutation,
  applyLocalCatalogConfigMutation,
  diffIdentityConfigMutation,
  diffLocalCatalogConfigMutation,
  identityMutationDiffPair,
  inverseIdentityConfigMutation,
  inverseLocalCatalogConfigMutation,
  localCatalogMutationDiffPair,
  retireLocalDocument,
  signIn,
  signOut,
  type Identity,
  type IdentityConfigMutation,
  type IdentityDiff,
  type LocalCatalog,
  type LocalCatalogConfigMutation,
  type LocalCatalogDiff,
  type LocalDocument,
} from "@semio-tech/framework/config/schema/mutations";
import {
  absorbIdentityDiff,
  absorbLocalCatalogDiff,
  applyIdentityDiff,
  applyLocalCatalogDiff,
  decodeIdentityDiff,
  decodeIdentityValue,
  decodeLocalCatalogDiff,
  decodeLocalCatalogValue,
  inverseIdentityDiff,
  inverseLocalCatalogDiff,
} from "@semio-tech/framework/config/schema/mutations";

const MUTATIONS = new URL("../../🎚️config/🧬️schema/🧬️mutations/", import.meta.url);
const read = <T>(directory: string, ...segments: string[]): T => JSON.parse(readFileSync(new URL(`${directory}/${segments.join("/")}/🔣️.json`, MUTATIONS), "utf8")) as T;
const vector = <T, M>(leaf: string, scenario: string) => ({
  before: read<T>(`${leaf}/🧫️fixtures/${scenario}`, "📸️snapshot", "⬅️before"),
  mutation: read<M>(`${leaf}/🧫️fixtures/${scenario}`, "🦠️mutation"),
  diff: read<unknown>(`${leaf}/🧫️fixtures/${scenario}`, "🔺️diff"),
  after: read<T>(`${leaf}/🧫️fixtures/${scenario}`, "📸️snapshot", "➡️after"),
});

const CATALOG_VECTORS = {
  "admit-local-document": vector<LocalCatalog, LocalCatalogConfigMutation>("📥️admit-local-document", "📥️lists-a-persisted-studio-beside-an-imported-one"),
  "retire-local-document": vector<LocalCatalog, LocalCatalogConfigMutation>("📤️retire-local-document", "📤️unlists-the-studio-and-keeps-its-sibling"),
};
const IDENTITY_VECTORS = {
  "sign-in": vector<Identity | null, IdentityConfigMutation>("🪪️sign-in", "🪪️replaces-the-active-session-with-a-second-account"),
  "sign-out": vector<Identity | null, IdentityConfigMutation>("🚪️sign-out", "🪪️clears-the-active-session"),
};

/** 🧮️ The inverse-sum law for a catalog mutation: returns Σ and the state the replay ends in. */
function catalogInverseSum(mutation: LocalCatalogConfigMutation, before: LocalCatalog): { sigma: LocalCatalogDiff; restored: LocalCatalog } {
  let state = applyLocalCatalogConfigMutation(before, mutation);
  let sigma: LocalCatalogDiff = {};
  for (const step of [...inverseLocalCatalogConfigMutation(mutation, before)].reverse()) {
    const stepDiff = diffLocalCatalogConfigMutation(step, state);
    state = applyLocalCatalogDiff(state, stepDiff);
    sigma = absorbLocalCatalogDiff(sigma, stepDiff);
  }
  return { sigma, restored: state };
}

function identityInverseSum(mutation: IdentityConfigMutation, before: Identity | null): { sigma: IdentityDiff; restored: Identity | null } {
  let state = applyIdentityConfigMutation(before, mutation);
  let sigma: IdentityDiff = {};
  for (const step of [...inverseIdentityConfigMutation(mutation, before)].reverse()) {
    const stepDiff = diffIdentityConfigMutation(step, state);
    state = applyIdentityDiff(state, stepDiff);
    sigma = absorbIdentityDiff(sigma, stepDiff);
  }
  return { sigma, restored: state };
}

/** 🎲️ xorshift32 — the random history is reproducible from its seed alone. */
function random(seed: number): () => number {
  let state = seed >>> 0 || 1;
  return () => {
    state ^= state << 13;
    state >>>= 0;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 0x1_0000_0000;
  };
}

describe("config diff twin", () => {
  for (const [leaf, row] of Object.entries(CATALOG_VECTORS)) {
    it(`${leaf}: names the committed sparse diff, applies to the committed after-catalog and inverts to the negative diff`, () => {
      const diff = diffLocalCatalogConfigMutation(row.mutation, row.before);
      expect(JSON.parse(JSON.stringify(diff))).toEqual(row.diff);
      expect(decodeLocalCatalogDiff(row.diff)).toEqual(row.diff);
      expect(applyLocalCatalogDiff(row.before, diff)).toEqual(row.after);
      expect(applyLocalCatalogConfigMutation(row.before, row.mutation)).toEqual(row.after);
      expect(decodeLocalCatalogValue(row.after)).toEqual(row.after);
      const { sigma, restored } = catalogInverseSum(row.mutation, row.before);
      expect(restored).toEqual(row.before);
      expect(sigma).toEqual(inverseLocalCatalogDiff(diff, row.before));
      expect(localCatalogMutationDiffPair(row.mutation, row.before)).toEqual({ diff, inverse: sigma });
      expect(applyLocalCatalogDiff(applyLocalCatalogDiff(row.before, diff), inverseLocalCatalogDiff(diff, row.before))).toEqual(row.before);
    });
  }

  for (const [leaf, row] of Object.entries(IDENTITY_VECTORS)) {
    it(`${leaf}: names the committed sparse diff, applies to the committed after-record and inverts to the negative diff`, () => {
      const diff = diffIdentityConfigMutation(row.mutation, row.before);
      expect(JSON.parse(JSON.stringify(diff))).toEqual(row.diff);
      expect(decodeIdentityDiff(row.diff)).toEqual(row.diff);
      expect(applyIdentityDiff(row.before, diff)).toEqual(row.after);
      expect(applyIdentityConfigMutation(row.before, row.mutation)).toEqual(row.after);
      expect(decodeIdentityValue(row.after)).toEqual(row.after);
      const { sigma, restored } = identityInverseSum(row.mutation, row.before);
      expect(restored).toEqual(row.before);
      expect(sigma).toEqual(inverseIdentityDiff(diff, row.before));
      expect(identityMutationDiffPair(row.mutation, row.before)).toEqual({ diff, inverse: sigma });
    });
  }

  it("a mutation that changes nothing names an empty diff and an empty inverse", () => {
    const { before: catalog } = CATALOG_VECTORS["retire-local-document"];
    const listed = catalog.documents[0]!;
    expect(diffLocalCatalogConfigMutation(admitLocalDocument(listed), catalog)).toEqual({});
    expect(diffLocalCatalogConfigMutation(retireLocalDocument("nobody"), catalog)).toEqual({});
    expect(localCatalogMutationDiffPair(retireLocalDocument("nobody"), catalog)).toEqual({ diff: {}, inverse: {} });
    const { before: identity } = IDENTITY_VECTORS["sign-in"];
    expect(diffIdentityConfigMutation(signIn(identity!), identity)).toEqual({});
    expect(diffIdentityConfigMutation(signOut(), null)).toEqual({});
    expect(identityMutationDiffPair(signOut(), null)).toEqual({ diff: {}, inverse: {} });
  });

  it("refuses a payload that is not the facet's diff and keeps a catalog ordered by id whatever the row order", () => {
    expect(decodeLocalCatalogDiff({ documents: "all" })).toBeUndefined();
    expect(decodeLocalCatalogDiff({ documents: [{ key: 4 }] })).toBeUndefined();
    expect(decodeLocalCatalogDiff({ documents: [{ key: "a", value: { documentId: "b" } }] })).toBeUndefined();
    expect(decodeLocalCatalogDiff(null)).toBeUndefined();
    expect(decodeIdentityDiff({ session: "grace" })).toBeUndefined();
    expect(decodeIdentityDiff({ session: { value: { userId: "grace" } } })).toBeUndefined();
    const documents: LocalDocument[] = ["c", "a", "b"].map((id) => ({ documentId: id, schema: "s", name: id, storage: "folder", target: `/${id}`, admittedAtMs: 1 }));
    const folded = applyLocalCatalogDiff({ documents: [] }, { documents: documents.map((value) => ({ key: value.documentId, value })) });
    expect(folded.documents.map((entry) => entry.documentId)).toEqual(["a", "b", "c"]);
  });

  it("holds the laws row by row over seeded random catalog histories, and undoes each history to its start", () => {
    for (let seed = 1; seed <= 60; seed += 1) {
      const next = random(seed);
      const ids = ["a", "b", "c", "d", "e"];
      let catalog: LocalCatalog = { documents: [] };
      const history: { mutation: LocalCatalogConfigMutation; before: LocalCatalog }[] = [];
      for (let step = 0; step < 24; step += 1) {
        const id = ids[Math.floor(next() * ids.length)]!;
        const mutation: LocalCatalogConfigMutation = next() < 0.4
          ? retireLocalDocument(id)
          : admitLocalDocument({ documentId: id, schema: "s.space", name: `${id}${Math.floor(next() * 3)}`, storage: next() < 0.5 ? "folder" : "file", target: `/t/${id}`, admittedAtMs: Math.floor(next() * 4) });
        const { sigma, restored } = catalogInverseSum(mutation, catalog);
        expect(restored, `seed ${seed} step ${step}`).toEqual(catalog);
        const diff = diffLocalCatalogConfigMutation(mutation, catalog);
        expect(sigma, `seed ${seed} step ${step}`).toEqual(inverseLocalCatalogDiff(diff, catalog));
        const after = applyLocalCatalogDiff(catalog, diff);
        expect(after.documents.map((entry) => entry.documentId)).toEqual([...after.documents.map((entry) => entry.documentId)].sort());
        history.push({ mutation, before: catalog });
        catalog = after;
      }
      let state = catalog;
      for (const { mutation, before } of [...history].reverse()) {
        for (const step of [...inverseLocalCatalogConfigMutation(mutation, before)].reverse()) state = applyLocalCatalogConfigMutation(state, step);
        expect(state, `seed ${seed}`).toEqual(before);
      }
      expect(state).toEqual({ documents: [] });
    }
  });

  it("holds the laws row by row over seeded random identity histories, and undoes each history to its start", () => {
    for (let seed = 1; seed <= 60; seed += 1) {
      const next = random(seed);
      let identity: Identity | null = null;
      const history: { mutation: IdentityConfigMutation; before: Identity | null }[] = [];
      for (let step = 0; step < 16; step += 1) {
        const mutation = next() < 0.3 ? signOut() : signIn({ userId: `u${Math.floor(next() * 3)}`, email: "e@x", displayName: "d", hubBaseUrl: "https://hub", issuedAtMs: Math.floor(next() * 3) });
        const { sigma, restored } = identityInverseSum(mutation, identity);
        expect(restored, `seed ${seed} step ${step}`).toEqual(identity);
        expect(sigma, `seed ${seed} step ${step}`).toEqual(inverseIdentityDiff(diffIdentityConfigMutation(mutation, identity), identity));
        history.push({ mutation, before: identity });
        identity = applyIdentityConfigMutation(identity, mutation);
      }
      let state = identity;
      for (const { mutation, before } of [...history].reverse()) {
        for (const step of [...inverseIdentityConfigMutation(mutation, before)].reverse()) state = applyIdentityConfigMutation(state, step);
        expect(state, `seed ${seed}`).toEqual(before);
      }
      expect(state).toBeNull();
    }
  });
});
