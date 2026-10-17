/** 🧫 The store module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export { default as storePresenceSchema } from "../../👥️presence/🧬️schema/🔣️.json" with { type: "json" };
export { default as workerVisibleSurfacesDocument } from "../../👷️worker/🪟️visible-surfaces/🔣️.json" with { type: "json" };
export { default as syncCommandRejectionSchema } from "../../🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json" with { type: "json" };
export const groupHistoryCasesFixtureUrl = fixtureUrl(import.meta.url, "../../🧩️composition/📬️publication/🤝️group/🧪️tests/🧫️fixtures/📚️history-cases/🔣️.json");
export { default as compositionOpenGenesisFixture } from "../../🧩️composition/🚪️open/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
export { default as storeCommandRejectionFixture } from "./🧫️command-rejection/🔣️.json" with { type: "json" };
export { default as storeNativeIdentitySchema } from "../../🧬️schema/📸️native-identity/🔣️.json" with { type: "json" };
export { default as workerVisibleSurfacesFixture } from "../../👷️worker/🪟️visible-surfaces/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
export { default as workerVisibleSurfacesSchema } from "../../👷️worker/🪟️visible-surfaces/🧬️schema/🔣️.json" with { type: "json" };
export const loadDocumentLinkShortageSchema = () => import("../../🔄️sync/🧬️schema/document-link-shortage/🔣️.json").then(module => module.default);
export const persistenceDataClassSchemaUrl = fixtureUrl(import.meta.url, "../../🔄️sync/🧬️schema/persistence-data-class/🔣️.json");
export const loadEchoSuppressionV1Fixture = () => import("./document-echo-suppression-v1/🔣️.json").then(module => module.default);
export const loadLinkShortageV1Fixture = () => import("./document-link-shortage-v1/🔣️.json").then(module => module.default);
export const dataClassV1FixtureUrl = fixtureUrl(import.meta.url, "./persistence-data-class-v1/🔣️.json");
export const loadStoreOutboundAnnouncementFixture = () => import("./📤️outbound-announcement/🔣️.json").then(module => module.default);
