/** 🧫 The spr module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const channelDocumentIdentitySchemaUrl = new URL("../../🧵️channel/🧬️schema/🪪️document-identity/🔣️.json", import.meta.url);
export const loadChannelMediaExportWireFixture = () => import("../../🧵️channel/🧪️tests/🧫️fixtures/🎬️media-export-wire/🔣️.json").then(module => module.default);
