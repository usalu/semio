/** 🧫 The neural module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const loadEngineRegistryAdmissionFixture = () => import("@semio-tech/neural-engine-ts/registry/admission/fixtures").then(module => module.registryAdmissionSchema);
export const loadEngineRegistryAdmissionSchema = () => import("@semio-tech/neural-engine-ts/registry/admission/schema").then(module => module.registryAdmissionSchema);
export const loadNeuralEngineProjectionFixture = () => import("@semio-tech/neural-engine-ts/projection/fixtures").then(module => module.projectionSchema);
export const loadNeuralEngineProjectionSchema = () => import("@semio-tech/neural-engine-ts/projection/schema").then(module => module.projectionSchema);
