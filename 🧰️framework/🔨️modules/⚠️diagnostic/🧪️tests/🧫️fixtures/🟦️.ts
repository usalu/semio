/** 🧫 The diagnostic module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export { default as diagnosticControlledFixture } from "./🎛️controlled/🔣️.json" with { type: "json" };
export { default as diagnosticControlledSchema } from "../../🧬️schema/🎛️controlled/🔣️.json" with { type: "json" };
export const diagnosticControlledSchemaUrl = new URL("../../🧬️schema/🎛️controlled/🔣️.json", import.meta.url);
export const diagnosticFaultFixtureUrl = new URL("./🧯️fault/🔣️.json", import.meta.url);
