/** 🧫 The pack module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export { causeCodecFixture as errorCauseCodecFixture } from "@semio-tech/framework-pack-error-ts/testing/fixtures/cause/codec";
export const valueScalarWitnessFixtureUrl = fixtureUrl(import.meta.url, "../../🌱️value/🔎️scalar-witness/🧪️tests/🧫️fixtures/🔣️.json");
export { intrinsicMediaFixture as valueIntrinsicMediaFixture } from "@semio-tech/framework-pack-record-ts/testing/fixtures/intrinsic-media";
export const valueIntrinsicMediaFixtureUrl = fixtureUrl(import.meta.url, "../../🌱️value/🧪️tests/🧫️fixtures/🎞️intrinsic-media/🔣️.json");
export { wireMaterializationFixture as valueWireMaterializationFixture } from "@semio-tech/framework-pack-record-ts/testing/fixtures/wire-materialization";
