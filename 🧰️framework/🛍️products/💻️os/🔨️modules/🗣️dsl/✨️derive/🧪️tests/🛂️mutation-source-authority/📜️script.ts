#!/usr/bin/env bun
/** 🧬️ Independent language-neutral mutation payload placement oracle. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import picomatch from "picomatch";
import { mutationDomainOwnersProblems, mutationOwnerIdentity, loadCatalogTaxonomy } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

type LayoutCase = { readonly name: string; readonly accepted: boolean; readonly relativeSource?: string };

const fixture = await Bun.file(new URL("./🧫️fixtures/🔣️.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🛂️schema/🔣️.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));

const acceptsLayout = (source: string): boolean => {
  const segments = source.split("/");
  const collection = segments.indexOf("🧬️mutations");
  if (collection < 0 || segments.at(-1) !== "🦀️.rs") return false;
  const tail = segments.slice(collection + 1);
  return tail.length === 2 || tail.length === 3 && tail[1] === "🦠️mutation";
};

const layoutCases = (fixture.cases as LayoutCase[]).filter((item) => item.relativeSource !== undefined);
for (const item of layoutCases) assert.equal(acceptsLayout(item.relativeSource!), item.accepted, item.name);
const hostile = [
  { ...fixture, extra: true },
  { ...fixture, cases: [...fixture.cases, { name: "invalid", accepted: true, relativeSource: "" }] },
];
for (const item of hostile) assert(!validate(item));
assert(!acceptsLayout("domain/🧬️mutations/🆕️insert-page/unknown/🦀️.rs"));
assert(!acceptsLayout("domain/🧬️mutations/🆕️insert-page/🦠️mutation/nested/🦀️.rs"));
console.log(`[DEBUG] mutation-source-authority layoutCases=${layoutCases.length} accepted=${layoutCases.filter((item) => item.accepted).length} rejected=${layoutCases.filter((item) => !item.accepted).length} ajv=true hostileRejections=4`);

const domainFixture = await Bun.file(new URL("./🧫️fixtures/🧭️domains.json", import.meta.url)).json();
const domainSchema = await Bun.file(new URL("./🛂️schema/🧭️domains.json", import.meta.url)).json();
const validateDomains = new Ajv({ strict: true, allErrors: true }).compile(domainSchema);
assert(validateDomains(domainFixture), JSON.stringify(validateDomains.errors));
const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [domainFixture.mutationRoot]: domainFixture.domains } };
assert.deepEqual(mutationDomainOwnersProblems(taxonomy.mutationDomainOwners), []);
const explicitOwners = Object.entries(domainFixture.domains).flatMap(([domain, operations]) => Object.entries(operations as Record<string, string>).map(([operation, identity]) => ({ owner: `${domain}/${operation}`, identity })));
for (const vector of domainFixture.cases) {
  const sourceAllowed = vector.source === "🦀️.rs" || vector.source === "🦠️mutation/🦀️.rs";
  const independent = !vector.fault && sourceAllowed && explicitOwners.some((item) => picomatch(item.owner, { literalBrackets: true })(vector.owner) && item.identity === vector.semanticKind);
  const subject = !vector.fault && sourceAllowed && mutationOwnerIdentity(domainFixture.mutationRoot, vector.owner, taxonomy) === vector.semanticKind;
  assert.equal(independent, vector.accepted, vector.name);
  assert.equal(subject, independent, vector.name);
}
console.log(`[DEBUG] mutation-source-authority exactDomainCases=${domainFixture.cases.length} ajv=true independent=picomatch`);
