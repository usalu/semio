/** 🧫 The spr module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export const channelDocumentIdentitySchemaUrl = fixtureUrl(import.meta.url, "../../🧵️channel/🧬️schema/🪪️document-identity/🔣️.json");
export const loadChannelMediaExportWireFixture = () => import("../../🧵️channel/🧪️tests/🧫️fixtures/🎬️media-export-wire/🔣️.json").then(module => module.default);
