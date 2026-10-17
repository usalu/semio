/** 🧫 The io module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "../../../🧪️test/📖️source-text/🟦️.ts";
export const snapshotEncodeOwnerFixtureUrl = fixtureUrl(import.meta.url, "../../⏱️control/🛫️snapshot/🧪️tests/🧫️fixtures/🎟️encode-owner/🔣️.json");
export { rfc4648Base64urlVectorsFixture } from "@semio-tech/framework-io-base64-ts/testing/fixtures";
export { schemaSchema as ioSchema } from "@semio-tech/framework-io-schema-ts";
export const controlSnapshotReceivingFixtureUrl = fixtureUrl(import.meta.url, "../../⏱️control/🛫️snapshot/🧪️tests/🧫️fixtures/🪆️receiving/🔣️.json");
