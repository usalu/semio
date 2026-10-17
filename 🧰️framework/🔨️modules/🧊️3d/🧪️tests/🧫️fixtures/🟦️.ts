/** 🧫 The 3d module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export const loadEngineVertexProvenanceFixture = () => import("../../📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json").then(module => module.default);
export const meshKnifeCutFixtureUrl = fixtureUrl(import.meta.url, "../../🥽️mesh/🧪️tests/🧫️fixtures/✂️knife-cut/🔣️.json");
export const fixture3dMeshModelingFixtureUrl = fixtureUrl(import.meta.url, "../../🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/🔣️.json");
export const meshKnifeCutSchemaUrl = fixtureUrl(import.meta.url, "../../🥽️mesh/🧬️schema/✂️knife-cut/🔣️.json");
export const meshLoopCutSchemaUrl = fixtureUrl(import.meta.url, "../../🥽️mesh/🧬️schema/✂️loop-cut/🔣️.json");
