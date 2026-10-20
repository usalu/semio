/** 🧫 The value module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "../../../../📦️packages/🟦️typescript/🧪️test/📖️source-text/🟦️.ts";
export const valueRetirementSchemaUrl = fixtureUrl(import.meta.url, "../../♻️retirement/🧬️contract/🧬️schema/🔣️.json");
export { default as valueRefusalSchema } from "../../⚠️refusal/🧬️schema/🔣️.json" with { type: "json" };
export const valueTypeSchemaUrl = fixtureUrl(import.meta.url, "../../🏷️type/🧬️schema/🔣️.json");
export const loadValueResidentAdmissionContract = () => import("../../💾️resident/📨️admission/🤝️contract.json").then(module => module.default);
export const loadValueResidentFixture = () => import("../../💾️resident/🧪️tests/🧫️fixtures/🔣️.json").then(module => module.default);
export { default as valueResidentSchema } from "../../💾️resident/🧬️schema/🔣️.json" with { type: "json" };
export const loadValueResidentSchema = () => import("../../💾️resident/🧬️schema/🔣️.json").then(module => module.default);
export const valueOrderedRetirementSchemaUrl = fixtureUrl(import.meta.url, "../../🗂️ordered/♻️retirement/🧬️schema/🔣️.json");
export { default as retainedCloneWireSchema } from "../../🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json" with { type: "json" };
export const retainedCloneWireSchemaUrl = fixtureUrl(import.meta.url, "../../🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json");
export const cloneWireProgressSchemaUrl = fixtureUrl(import.meta.url, "../../🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json");
export const retainedCloneNestedFixtureUrl = fixtureUrl(import.meta.url, "../../🧬️retained-clone/🧪️tests/🧫️fixtures/📦️nested/🔣️.json");
export const valueSchemaUrl = fixtureUrl(import.meta.url, "../../🧬️schema/🔣️.json");
export const valueRetirementFactoryFixtureUrl = fixtureUrl(import.meta.url, "../../♻️retirement/🏭️factory/🧪️tests/🧫️fixtures/🔣️.json");
export const valueRefusalCodecSchemaUrl = fixtureUrl(import.meta.url, "../../⚠️refusal/🔁️codec/🧬️schema/🔣️.json");
export const valueRefusalSchemaUrl = fixtureUrl(import.meta.url, "../../⚠️refusal/🧬️schema/🔣️.json");
export const valueTypeFixtureUrl = fixtureUrl(import.meta.url, "../../🏷️type/🧪️tests/🧫️fixtures/🔣️.json");
