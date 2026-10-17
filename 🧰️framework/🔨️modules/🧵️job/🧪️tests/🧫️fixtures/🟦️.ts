/** 🧫 The job module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "../../../🧪️test/📖️source-text/🟦️.ts";
export const jobRetirementPayloadFixtureUrl = fixtureUrl(import.meta.url, "../../♻️retirement/📄️payload/🧪️tests/🧫️fixtures/🔣️.json");
export { default as jobWorkGrantSchema } from "../../🏃️work/🎟️grant/🧬️schema/🔣️.json" with { type: "json" };
export { default as jobAdmissionOwnedSchema } from "../../🚪️admission/📦️owned/🧬️schema/🔣️.json" with { type: "json" };
export { default as jobCloseDemandFixture } from "./📏️close-demand/🔣️.json" with { type: "json" };
export const jobSnapshotSchemaUrl = fixtureUrl(import.meta.url, "../../📥️snapshot/🧬️schema/🔣️.json");
export const jobCloseDemandFixtureUrl = fixtureUrl(import.meta.url, "./📏️close-demand/🔣️.json");
