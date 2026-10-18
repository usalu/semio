/** 🔐️ The scope-closure law over `🧪️tests/🧫️fixtures/🧫️scope-closure/🔣️.json` (plain examples): `taxonomyClosedScope` widens exactly the scopes
 * inside a mutation facet to the root that owns the facet, and an independent oracle agrees — picomatch tells whether a scope lies
 * in a facet (`**\/{🧬️schema,🧫️fixtures}/🧬️mutations{,/**}`) and its owner is the longest ancestor no facet glob matches below. */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import picomatch from "picomatch";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { taxonomyClosedScope } from "../../🌳️tree-digests/🟦️.ts";

type Case = { readonly id: string; readonly scope: string; readonly holdsMutations: readonly string[]; readonly closed: string };

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧫️scope-closure/🔣️.json", import.meta.url), "utf8")) as { readonly cases: readonly Case[] };
const inFacet = picomatch(["**/{🧬️schema,🧪️tests/🧫️fixtures}/🧬️mutations", "**/{🧬️schema,🧪️tests/🧫️fixtures}/🧬️mutations/**"]);
const facetRoot = picomatch("**/{🧬️schema,🧪️tests/🧫️fixtures}");

/** 🔮️ The independent oracle: a scope in a mutation facet (or a facet root that holds mutations) closes to the parent of the first
 * facet root among its ancestors-or-self whose own subtree picomatch places in the facet; any other scope is itself. */
function oracle(entry: Case): string {
  const scope = entry.scope.replace(/\/+$/u, "");
  const segments = scope.split("/");
  const widened = inFacet(scope) || (facetRoot(scope) && entry.holdsMutations.includes(scope));
  if (!widened) return scope;
  for (let length = 2; length <= segments.length; length += 1) {
    const prefix = segments.slice(0, length).join("/");
    if (facetRoot(prefix) && (length === segments.length || inFacet(segments.slice(0, length + 1).join("/")))) return segments.slice(0, length - (segments[length - 1] === "🧫️fixtures" ? 2 : 1)).join("/");
  }
  return scope;
}

describe("🔐️ taxonomy scope closure", () => {
  test("the examples cover widened and closed scopes", () => {
    
    
    expect(fixture.cases.some((entry) => entry.closed !== entry.scope.replace(/\/+$/u, ""))).toBe(true);
    expect(fixture.cases.some((entry) => entry.closed === entry.scope)).toBe(true);
  });
  for (const entry of fixture.cases) {
    test(entry.id, () => {
      expect(taxonomyClosedScope(entry.scope, (path) => entry.holdsMutations.includes(path), loadCatalogTaxonomy())).toBe(entry.closed);
      expect(oracle(entry)).toBe(entry.closed);
    });
  }
});
