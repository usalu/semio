/** 🧫 The neural module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const loadEngineRegistryAdmissionFixture = () => import("../../⚙️engine/📔️registry/🏗️admission/🧫️fixtures/🔣️.json").then(module => module.default);
export const loadEngineRegistryAdmissionSchema = () => import("../../⚙️engine/📔️registry/🏗️admission/🧬️schema/🔣️.json").then(module => module.default);
export const loadNeuralEngineProjectionFixture = () => import("../../⚙️engine/📤️projection/🧫️fixtures/🔣️.json").then(module => module.default);
export const loadNeuralEngineProjectionSchema = () => import("../../⚙️engine/📤️projection/🧬️schema/🔣️.json").then(module => module.default);
