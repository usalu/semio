/** 🧫 The filesystem module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const loadFilesystemObservationDescriptorSchema = () => import("../../🧾️observation/📁️descriptor/🧬️schema/🔣️.json").then(module => module.default);
export { default as filesystemObservationDescriptorSchema } from "../../🧾️observation/📁️descriptor/🧬️schema/🔣️.json" with { type: "json" };
