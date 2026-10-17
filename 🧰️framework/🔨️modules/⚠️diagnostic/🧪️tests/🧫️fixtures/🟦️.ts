/** 🧫 The diagnostic module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export { default as diagnosticControlledFixture } from "./🎛️controlled/🔣️.json" with { type: "json" };
export { default as diagnosticControlledSchema } from "../../🧬️schema/🎛️controlled/🔣️.json" with { type: "json" };
export const diagnosticControlledSchemaUrl = fixtureUrl(import.meta.url, "../../🧬️schema/🎛️controlled/🔣️.json");
export const diagnosticFaultFixtureUrl = fixtureUrl(import.meta.url, "./🧯️fault/🔣️.json");
