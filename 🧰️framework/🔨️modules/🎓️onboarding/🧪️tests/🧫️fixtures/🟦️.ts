/** 🧫 The onboarding module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export { default as tutorialDocumentTrackFixture } from "./🎞️tutorial-document-track.json" with { type: "json" };
export const tutorialLocalInteractionFixtureUrl = new URL("./🖱️tutorial-local-interaction.json", import.meta.url);
