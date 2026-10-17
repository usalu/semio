/** 🌱️ The presentation core, intro arrangements and resolution: the intro morph labels, embodiment and scope resolution, morph ids and split participants. */
import { describe, expect, it } from "vitest";
import { INTRO_EMBODIMENT_AUTHORS_PLAIN, INTRO_EMBODIMENT_DESCRIPTION_SHORT, INTRO_PARTICIPANT_AUTHORS, INTRO_PARTICIPANT_DESCRIPTION, INTRO_PARTICIPANT_INSTITUTIONS, abbreviateAuthorFirstName, affiliationEmbodimentMorphLabels, affiliationLineName, authorLinesForAffiliationStep, buildResolutionScope, collectPresentationSlides, expandThoughtSlides, intro, isIntroArrangementId, morphId, resolveArrangement, resolveEmbodiment, resolveTextMorphRoot, split, type TextEmbodiment } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { sampleIntro, thoughtScope } from "../🧷️presentation-core-kit/🟦️.ts";

describe("intro", () => {
	it("recognizes intro arrangement ids regardless of bookmark language", () => {
		expect(isIntroArrangementId("affiliations-3")).toBe(true);
		expect(isIntroArrangementId("Lehrstühle")).toBe(false);
	});

	it("builds seven slides in one thought", () => {
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		expect(thought.slides.map((slide) => slide.arrangement.id)).toEqual([
			"title",
			"description",
			"goal",
			"authors",
			"affiliations-1",
			"affiliations-2",
			"affiliations-3",
		]);
		expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual([
			"title",
			"description",
			"goal",
			"authors",
			"affiliations-1",
			"affiliations-2",
			"affiliations-3",
		]);
	});

	it("keeps a single universities bookmark at v=5", () => {
		const uniSlides = collectPresentationSlides(sampleIntro).filter((slide) => slide.slide === "Universities");
		expect(uniSlides).toHaveLength(1);
		expect(uniSlides[0]?.v).toBe(5);
	});

	it("uses fixed-size heading blocks without fit-text", () => {
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const textEmbodiments = (thought.embodiments ?? []).filter((e): e is TextEmbodiment => e.kind === "text");
		expect(textEmbodiments.every((e) => e.fit !== true)).toBe(true);
		expect(textEmbodiments.every((e) => resolveTextMorphRoot(e) === "heading-block")).toBe(true);
	});

	it("uses affiliation short names when chairs are introduced", () => {
		expect(
			affiliationLineName({
				mark: "1",
				name: "Leibniz Universität Hannover",
				shortName: "LUH",
			}),
		).toBe("LUH");
	});

	it("includes muted authors on goal so reveal can morph into the authors slide", () => {
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
		const goalAuthors = resolveArrangement(thoughtScope(thought), goal).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_AUTHORS,
		);
		expect(goalAuthors?.emphasis).toBe("muted");
		expect(goalAuthors?.embodiment.id).toBe(INTRO_EMBODIMENT_AUTHORS_PLAIN);
	});

	it("uses plain short description embodiment on goal and later intro slides", () => {
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
		const authors = thought.slides.find((slide) => slide.arrangement.id === "authors")!.arrangement;
		const goalDescription = resolveArrangement(thoughtScope(thought), goal).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_DESCRIPTION,
		)!;
		const authorsDescription = resolveArrangement(thoughtScope(thought), authors).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_DESCRIPTION,
		)!;
		if (goalDescription.embodiment.kind === "text" && authorsDescription.embodiment.kind === "text") {
			expect(goalDescription.embodiment.id).toBe(INTRO_EMBODIMENT_DESCRIPTION_SHORT);
			expect(goalDescription.embodiment.morphFromLines).toBeUndefined();
			expect(authorsDescription.embodiment.id).toBe(INTRO_EMBODIMENT_DESCRIPTION_SHORT);
		}
	});

	it("records prior affiliation labels for embodiment morph on chairs slide", () => {
		const previous = [
			{ mark: "a", name: "Faculty" },
			{ mark: "1", name: "Leibniz Universität Hannover" },
		] as const;
		const current = [
			{ mark: "a", name: "Faculty" },
			{
				mark: "1",
				name: "Leibniz Universität Hannover",
				shortName: "LUH",
				suffix: { mark: "x", name: "Chair X" },
			},
		] as const;
		expect(affiliationEmbodimentMorphLabels(previous, current)).toEqual({
			"1": "Leibniz Universität Hannover",
		});
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const affiliations3 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-3")!.arrangement;
		const step3 = resolveArrangement(thoughtScope(thought), affiliations3).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
		)!;
		if (step3.embodiment.kind === "affiliations") {
			expect(step3.embodiment.morphLineLabels).toEqual({
				"1": "University",
				"2": "Other University",
			});
		}
	});

	it("abbreviates author first names on affiliation slides", () => {
		expect(abbreviateAuthorFirstName("Ueli Saluz")).toBe("U. Saluz");
		expect(abbreviateAuthorFirstName("Christoph Gengnagel")).toBe("C. Gengnagel");
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const affiliations1 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-1")!.arrangement;
		const authors = resolveArrangement(thoughtScope(thought), affiliations1).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_AUTHORS,
		)!;
		if (authors.embodiment.kind === "authors") {
			expect(authors.embodiment.abbreviateFirstName).toBe(true);
		}
	});

	it("introduces author marks with each affiliation step", () => {
		const lines = [[{ name: "Alice", marks: ["a", "1", "x"] }]] as const;
		const rawSteps = [
			[{ mark: "a", name: "Faculty" }],
			[
				{ mark: "a", name: "Faculty" },
				{ mark: "1", name: "University" },
			],
			[
				{ mark: "a", name: "Faculty" },
				{ mark: "1", name: "University", suffix: { mark: "x", name: "Chair X" } },
			],
		] as const;
		const aff1 = authorLinesForAffiliationStep(lines, rawSteps[0], [])[0]![0]!;
		expect(aff1.markEntries?.map((m) => m.mark)).toEqual(["a"]);
		const aff2 = authorLinesForAffiliationStep(lines, rawSteps[1], rawSteps[0])[0]![0]!;
		expect(aff2.markEntries?.map((m) => m.mark)).toEqual(["a", "1"]);
		expect(aff2.markEntries?.find((m) => m.mark === "a")?.emphasis).toBe("muted");
		expect(aff2.markEntries?.find((m) => m.mark === "1")?.emphasis).toBe("active");
		const aff3 = authorLinesForAffiliationStep(lines, rawSteps[2], rawSteps[1])[0]![0]!;
		expect(aff3.markEntries?.map((m) => m.mark)).toEqual(["a", "1", "x"]);
		expect(aff3.markEntries?.find((m) => m.mark === "x")?.emphasis).toBe("active");
	});

	it("highlights only new affiliation marks per slide", () => {
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const affiliations2 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-2")!.arrangement;
		const step2 = resolveArrangement(thoughtScope(thought), affiliations2).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
		)!;
		if (step2.embodiment.kind === "affiliations") {
			expect(step2.embodiment.entries.find((e) => e.mark === "1")?.lineEmphasis).toBe("active");
			expect(step2.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
		}
		const affiliations3 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-3")!.arrangement;
		const step3 = resolveArrangement(thoughtScope(thought), affiliations3).find(
			(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
		)!;
		if (step3.embodiment.kind === "affiliations") {
			const uni = step3.embodiment.entries.find((e) => e.mark === "1");
			expect(uni?.lineEmphasis).toBe("muted");
			expect(uni?.suffixEmphasis).toBe("active");
			expect(step3.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
		}
	});
	it("assigns German bookmark names when language is de", () => {
		const deck = intro({
			language: "de",
			title: { full: ["A"], short: "Short" },
			description: { full: ["D"], short: "D short" },
			goal: ["G"],
			authors: { lines: [[{ name: "Alice" }]] },
			affiliations: {
				steps: [
					[{ mark: "a", name: "Faculty" }],
					[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
					[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
				],
			},
		});
		const chapter = deck.chapters[0]!;
		const sequence = chapter.sequences[0]!;
		const thought = sequence.thoughts[0]!;
		expect(chapter.name).toBe("Hauptteil");
		expect(sequence.name).toBe("Einführung");
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
		const goalSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Ziel");
		expect(goalSlide).toMatchObject({
			h: 0,
			chapter: "Hauptteil",
			sequence: "Einführung",
			thought: "Einleitung",
			slide: "Ziel",
		});
		expect(goalSlide?.v).toBeGreaterThan(0);
	});
});

describe("resolveEmbodiment", () => {
	it("throws when embodiment id is missing from scope", () => {
		const scope = buildResolutionScope([
			{
				participants: [{ id: "x" }],
				embodiments: [{ kind: "text", id: "x--body", lines: ["a"], level: "body" }],
			},
		]);
		expect(() => resolveEmbodiment(scope, "missing")).toThrow(/Unknown embodiment/);
	});
});

describe("buildResolutionScope", () => {
	it("lets inner scopes override embodiment ids", () => {
		const scope = buildResolutionScope([
			{ embodiments: [{ kind: "text", id: "a", lines: ["outer"], level: "body" }] },
			{ embodiments: [{ kind: "text", id: "a", lines: ["inner"], level: "body" }] },
		]);
		expect((resolveEmbodiment(scope, "a") as TextEmbodiment).lines[0]).toBe("inner");
	});
});

describe("morphId", () => {
	it("uses participant id as reveal data-id", () => {
		expect(morphId("title")).toBe("title");
	});
});

describe("split", () => {
	it("produces one participant, embodiment, and disposition per grid cell", () => {
		const artifacts = split({
			source: "/catalogue.png",
			rows: 2,
			columns: 2,
			frame: { x: 0, y: 0, width: 1, height: 1 },
		});
		expect(artifacts.participants).toHaveLength(4);
		expect(artifacts.embodiments).toHaveLength(4);
		expect(artifacts.dispositions).toHaveLength(4);
		expect(artifacts.dispositions[0]?.participantId).toBe("tile-r0-c0");
		expect(artifacts.dispositions[0]?.embodimentId).toBe("tile-r0-c0-figure");
	});
});
