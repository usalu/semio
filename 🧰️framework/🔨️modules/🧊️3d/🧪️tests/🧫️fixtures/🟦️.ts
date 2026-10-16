/** 🧫 The 3d module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const loadEngineVertexProvenanceFixture = () => import("../../📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json").then(module => module.default);
export const meshKnifeCutFixtureUrl = new URL("../../🥽️mesh/🧪️tests/🧫️fixtures/✂️knife-cut/🔣️.json", import.meta.url);
export const fixture3dMeshModelingFixtureUrl = new URL("../../🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/🔣️.json", import.meta.url);
export const meshKnifeCutSchemaUrl = new URL("../../🥽️mesh/🧬️schema/✂️knife-cut/🔣️.json", import.meta.url);
export const meshLoopCutSchemaUrl = new URL("../../🥽️mesh/🧬️schema/✂️loop-cut/🔣️.json", import.meta.url);
