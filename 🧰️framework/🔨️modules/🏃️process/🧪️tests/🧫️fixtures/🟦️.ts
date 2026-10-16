/** 🧫 The process module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const loadNativeBuildCaptureDocument = () => import("../../📦️artifacts/🏗️native-build/📥️capture/🧪️tests/🔣️.json").then(module => module.default);
export const loadNativeBuildCaptureSchema = () => import("../../📦️artifacts/🏗️native-build/📥️capture/🧬️schema/🔣️.json").then(module => module.default);
export { default as receiptRetentionReferenceSchema } from "../../📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📋️reference/🧬️schema/🔣️.json" with { type: "json" };
export const loadProcessArtifactsFilesDocument = () => import("../../📦️artifacts/🗂️files/🧪️tests/🔣️.json").then(module => module.default);
export const artifactsPublicationSourcesSchemaUrl = new URL("../../📦️artifacts/📤️publication/📥️sources/🧬️schema/🔣️.json", import.meta.url);
