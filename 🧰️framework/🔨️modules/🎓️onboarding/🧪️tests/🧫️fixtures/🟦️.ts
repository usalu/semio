/** 🧫 The onboarding module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "../../../🧪️test/📖️source-text/🟦️.ts";
export { default as tutorialDocumentTrackFixture } from "./🎞️tutorial-document-track.json" with { type: "json" };
export const tutorialLocalInteractionFixtureUrl = fixtureUrl(import.meta.url, "./🖱️tutorial-local-interaction.json");
