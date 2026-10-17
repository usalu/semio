/** 📽️ The presentation core, loading from slide globs: assembling presentations from slide paths, parsing slide and thought file paths and thought templates.
 *
 * @see ../🧷️presentation-core-kit/🟦️.ts — the shared thought scope and sample intro
 * @see ../🌱️intro-and-resolution/🟦️.ts — intro arrangements and resolution
 * @see ../🪡️thought-expansion/🟦️.ts — `expandThoughtSlides`
 * @see ../🗺️arrangement-geometry-and-sequences/🟦️.ts — arrangement geometry and sequences
 * @see ../🔄️analogy-and-tile-play/🟦️.ts — analogies, video, pdf and tile play
 */
import { describe, expect, it } from "vitest";
import { introThoughtFile, loadPresentationFromSlideGlob, parsePresentationSlideFilePath, parsePresentationThoughtFilePath, presentationSlideFilePath, presentationThoughtFilePath } from "../../📦️packages/🟦️typescript/🟦️.ts";

describe("loadPresentationFromSlideGlob", () => {
	it("assembles chapters, sequences, thoughts, and ordered slides from slide paths", () => {
		const deck = loadPresentationFromSlideGlob(
			{ id: "deck", name: "Deck", language: "de" },
			{
				"./🎞️slide/Hauptteil/🌷️Einführung/🪻️Einleitung/Titel.ts": {
					default: {
						order: 0,
						participants: [{ id: "title" }],
						embodiments: [{ kind: "text", id: "title--main", lines: ["A"], level: "title" }],
						arrangement: {
							id: "title",
							name: "Titel",
							dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
						},
					},
				},
				"./🎞️slide/Hauptteil/🌷️Einführung/🪻️Einleitung/Ziel.ts": {
					default: {
						order: 1,
						arrangement: {
							id: "goal",
							name: "Ziel",
							dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
						},
					},
				},
				"./🎞️slide/Hauptteil/🌷️Einführung/Medien/🟦️Bauteilkatalog.ts": {
					default: {
						order: 0,
						arrangement: {
							id: "catalogue",
							name: "Bauteilkatalog",
							dispositions: [{ participantId: "catalogue", embodimentId: "catalogue--figure", emphasis: "active" }],
						},
					},
				},
			},
		);
		expect(deck.chapters[0]).toMatchObject({
			name: "Hauptteil",
			sequences: [
				{
					name: "Einführung",
					thoughts: [
						{
							name: "Einleitung",
							slides: [{ arrangement: { name: "Titel" } }, { arrangement: { name: "Ziel" } }],
						},
						{
							name: "Medien",
							slides: [{ arrangement: { name: "Bauteilkatalog" } }],
						},
					],
				},
			],
		});
	});
});

describe("parsePresentationSlideFilePath", () => {
	it("round-trips canonical slide module paths", () => {
		const path = presentationSlideFilePath("Hauptteil", "Einführung", "Einleitung", "Titel");
		expect(path).toBe("🎞️slide/Hauptteil/Einführung/Einleitung/Titel.ts");
		expect(parsePresentationSlideFilePath(`./${path}`)).toEqual({
			chapter: "Hauptteil",
			sequence: "Einführung",
			thought: "Einleitung",
			slide: "Titel",
		});
	});
});

describe("parsePresentationThoughtFilePath", () => {
	it("round-trips canonical thought template paths", () => {
		const path = presentationThoughtFilePath("Hauptteil", "Einführung", "Einleitung");
		expect(path).toBe("🎞️slide/Hauptteil/Einführung/Einleitung.ts");
		expect(parsePresentationThoughtFilePath(`./${path}`)).toEqual({
			chapter: "Hauptteil",
			sequence: "Einführung",
			thought: "Einleitung",
		});
		expect(parsePresentationSlideFilePath(`./${path}`)).toBeNull();
	});
});

describe("loadPresentationFromSlideGlob thought templates", () => {
	it("expands intro thought files into ordered slides", () => {
		const deck = loadPresentationFromSlideGlob(
			{ id: "deck", name: "Deck", language: "de" },
			{
				"./🎞️slide/Hauptteil/🌷️Einführung/🟦️Einleitung.ts": {
					default: introThoughtFile({
						language: "de",
						title: { full: ["T"], short: "T" },
						description: { full: ["D"], short: "D" },
						goal: ["G"],
						authors: { lines: [[{ name: "A", marks: ["a"] }]] },
						affiliations: { steps: [[{ mark: "a", name: "Faculty" }], [{ mark: "a", name: "Faculty" }], [{ mark: "a", name: "Faculty" }]] },
					}),
				},
			},
		);
		const thought = deck.chapters[0]!.sequences[0]!.thoughts[0]!;
		expect(thought.name).toBe("Einleitung");
		expect(thought.slides.map((slide) => slide.arrangement.name)).toEqual([
			"Titel",
			"Beschreibung",
			"Ziel",
			"Autoren",
			"Fakultät",
			"Universitäten",
			"Lehrstühle",
		]);
	});
});
