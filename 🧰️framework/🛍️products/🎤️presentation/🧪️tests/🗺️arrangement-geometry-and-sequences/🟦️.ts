/** 🗺️ The presentation core, arrangement geometry and sequences: centring, figure frames, scroll origins, figure grids, source crops, text morph roots, sequences and slide hashes. */
import { describe, expect, it } from "vitest";
import { MEDIA_SCROLL_ORIGIN_CENTER, MEDIA_SCROLL_ORIGIN_TOP_LEFT, PRESENTATION_DEFAULT_SLIDE_ASPECT, centerResolvedArrangement, collectPresentationSlides, figureFrameForSourceAspect, formatPresentationSlideHash, formatPresentationUrlHash, mediaScrollPercentForAxis, parsePresentationSlideHash, presentationSequences, presentationSlideAt, presentationSlideBookmarkParamKeys, resolveArrangement, resolveMediaScrollOrigin, resolveTextMorphRoot, splitFigureGrid, tile, unionSourceCrops, type Presentation, type ResolvedDisposition, type TextEmbodiment } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { sampleIntro, thoughtScope } from "../🧷️presentation-core-kit/🟦️.ts";

describe("centerResolvedArrangement", () => {
	it("offsets placements so their bounding box is centered in the unit slide", () => {
		const resolved: ResolvedDisposition[] = [
			{
				participant: { id: "a" },
				embodiment: { kind: "figure", id: "a--figure", src: "/a.png" },
				embodimentId: "a--figure",
				emphasis: "active",
				morphId: "a",
				position: { x: 0.2, y: 0.1, width: 0.3, height: 0.5 },
			},
		];
		const centered = centerResolvedArrangement(resolved);
		expect(centered[0]?.position?.x).toBeCloseTo(0.35);
		expect(centered[0]?.position?.y).toBeCloseTo(0.25);
	});
});

describe("figureFrameForSourceAspect", () => {
	const slideAspect = PRESENTATION_DEFAULT_SLIDE_ASPECT;

	it("matches the source physical aspect inside default padding", () => {
		const sourceAspect = 1536 / 1024;
		const frame = figureFrameForSourceAspect(sourceAspect, slideAspect);
		expect((frame.width / frame.height) * slideAspect).toBeCloseTo(sourceAspect, 10);
		expect(frame.x).toBeCloseTo((1 - frame.width) / 2, 10);
		expect(frame.y).toBeCloseTo((1 - frame.height) / 2, 10);
	});

	it("prefers full width for wider-than-slide sources", () => {
		const frame = figureFrameForSourceAspect(2, slideAspect);
		expect(frame.width).toBeCloseTo(0.92, 10);
	});
});

describe("resolveMediaScrollOrigin", () => {
	it("defaults to center when scroll origin is omitted", () => {
		expect(resolveMediaScrollOrigin(undefined)).toEqual({ x: 50, y: 50 });
	});

	it("resolves partial scroll origins and axis percents", () => {
		expect(resolveMediaScrollOrigin({ x: 0 })).toEqual({ x: 0, y: 50 });
		expect(mediaScrollPercentForAxis("x", MEDIA_SCROLL_ORIGIN_TOP_LEFT)).toBe(0);
		expect(mediaScrollPercentForAxis("y", MEDIA_SCROLL_ORIGIN_TOP_LEFT)).toBe(0);
		expect(mediaScrollPercentForAxis("y", MEDIA_SCROLL_ORIGIN_CENTER)).toBe(50);
	});
});

describe("splitFigureGrid", () => {
	const frame = { x: 0.1, y: 0.2, width: 0.8, height: 0.6 };

	it("builds rows×columns tiles with frame-relative source crops", () => {
		const tiles = splitFigureGrid({ rows: 3, columns: 5, frame });
		expect(tiles).toHaveLength(15);
		expect(tiles[0]?.key).toBe("tile-r0-c0");
		expect(tiles[0]?.crop.x).toBeCloseTo(0.1, 10);
		expect(tiles[0]?.crop.y).toBeCloseTo(0.2, 10);
		expect(tiles[0]?.crop.width).toBeCloseTo(0.16, 10);
		expect(tiles[0]?.crop.height).toBeCloseTo(0.2, 10);
		expect(tiles[14]?.key).toBe("tile-r2-c4");
		expect(tiles[14]?.crop.x).toBeCloseTo(0.74, 10);
		expect(tiles[14]?.crop.y).toBeCloseTo(0.6, 10);
	});

	it("reconstructs the frame at gap zero", () => {
		const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
		expect(tiles[0]?.position).toEqual({ x: 0.1, y: 0.2, width: 0.4, height: 0.3 });
		expect(tiles[3]?.position).toEqual({ x: 0.5, y: 0.5, width: 0.4, height: 0.3 });
	});

	it("inserts gap between tile cells", () => {
		const tiles = splitFigureGrid({ rows: 2, columns: 2, frame, gap: 0.05 });
		expect(tiles[1]?.position.x).toBeCloseTo(0.1 + 0.375 + 0.05, 5);
		expect(tiles[1]?.position.width).toBeCloseTo(0.375, 5);
	});

	it("applies default emphasis to every tile when set", () => {
		const tiles = splitFigureGrid({ rows: 1, columns: 2, frame, emphasis: "muted" });
		expect(tiles.every((tile) => tile.emphasis === "muted")).toBe(true);
	});
});

describe("unionSourceCrops", () => {
	it("unions normalized crops from grid cells", () => {
		const cells = splitFigureGrid({ rows: 2, columns: 2, frame: { x: 0, y: 0, width: 1, height: 1 } });
		const union = unionSourceCrops(cells.map((cell) => cell.crop));
		expect(union).toEqual({ x: 0, y: 0, width: 1, height: 1 });
	});
});

describe("resolveTextMorphRoot", () => {
	it("maps intro title embodiments like eg-ice-25", () => {
		const full: TextEmbodiment = {
			kind: "text",
			id: "full",
			lines: ["A", "B"],
			level: "heading",
			morphRoot: "heading-block",
		};
		const short: TextEmbodiment = {
			kind: "text",
			id: "short",
			lines: ["Short"],
			level: "subheading",
			morphRoot: "subheading-line",
		};
		expect(resolveTextMorphRoot(full)).toBe("heading-block");
		expect(resolveTextMorphRoot(short)).toBe("subheading-line");
	});
});

describe("resolveArrangement morphId", () => {
	it("resolves morphId per disposition", () => {
		const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
		const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
		const resolved = resolveArrangement(thoughtScope(thought), goal);
		expect(resolved.map((resolvedDisposition) => resolvedDisposition.morphId)).toEqual([
			"title",
			"description",
			"goal",
			"authors",
		]);
	});
});

describe("presentationSequences", () => {
	it("flattens sequences in chapter order", () => {
		const deck: Presentation = {
			id: "flat",
			name: "Flat",
			chapters: [
				{ id: "c1", sequences: [{ id: "s1", thoughts: [] }, { id: "s2", thoughts: [] }] },
				{ id: "c2", sequences: [{ id: "s3", thoughts: [] }] },
			],
		};
		expect(presentationSequences(deck).map((sequence) => sequence.id)).toEqual(["s1", "s2", "s3"]);
	});
});

describe("collectPresentationSlides", () => {
	it("orders slides by sequence h then arrangement v", () => {
		const deck: Presentation = {
			id: "multi",
			name: "Multi",
			chapters: [
				{
					id: "chapter-a",
					sequences: [
						{
							id: "seq-a",
							thoughts: [
								{
									id: "thought-a",
									participants: [],
									slides: [
										{ arrangement: { id: "a1", dispositions: [] } },
										{ arrangement: { id: "a2", dispositions: [] } },
									],
								},
							],
						},
					],
				},
				{
					id: "chapter-b",
					sequences: [
						{
							id: "seq-b",
							thoughts: [
								{
									id: "thought-b",
									participants: [],
									slides: [{ arrangement: { id: "b1", dispositions: [] } }],
								},
							],
						},
					],
				},
			],
		};
		expect(collectPresentationSlides(deck)).toEqual([
			{ h: 0, v: 0, chapter: "chapter-a", sequence: "seq-a", thought: "thought-a", slide: "a1" },
			{ h: 0, v: 1, chapter: "chapter-a", sequence: "seq-a", thought: "thought-a", slide: "a2" },
			{ h: 1, v: 0, chapter: "chapter-b", sequence: "seq-b", thought: "thought-b", slide: "b1" },
		]);
		expect(presentationSlideAt(deck, { h: 1, v: 0 })?.slide).toBe("b1");
	});
});

describe("parsePresentationSlideHash", () => {
	it("round-trips reveal.js hash paths", () => {
		expect(parsePresentationSlideHash("#/")).toEqual({ h: 0, v: 0 });
		expect(parsePresentationSlideHash("#/2/3")).toEqual({ h: 2, v: 3 });
		expect(parsePresentationSlideHash("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({ h: 0, v: 2 });
		expect(formatPresentationSlideHash({ h: 2, v: 3 })).toBe("/2/3");
	});

	it("formats chapter, sequence, thought, and slide bookmark params after the hash path", () => {
		const bookmark = {
			chapter: "Main",
			sequence: "Introduction",
			thought: "Introduction",
			slide: "Title",
		};
		expect(formatPresentationUrlHash({ h: 0, v: 0 }, bookmark)).toBe(
			"#/?chapter=Main&sequence=Introduction&thought=Introduction&slide=Title",
		);
		expect(formatPresentationUrlHash({ h: 0, v: 2 }, { ...bookmark, slide: "Goal" })).toBe(
			"#/0/2?chapter=Main&sequence=Introduction&thought=Introduction&slide=Goal",
		);
	});

	it("uses German bookmark query keys and titleized bookmark names for de presentations", () => {
		const bookmark = {
			chapter: "Hauptteil",
			sequence: "Einführung",
			thought: "Einleitung",
			slide: "Universitäten",
		};
		const hash = formatPresentationUrlHash({ h: 0, v: 5 }, bookmark, "de");
		expect(hash.startsWith("#/0/5?")).toBe(true);
		const params = new URLSearchParams(hash.split("?")[1] ?? "");
		expect(params.get("kapitel")).toBe("Hauptteil");
		expect(params.get("sequenz")).toBe("Einführung");
		expect(params.get("gedanke")).toBe("Einleitung");
		expect(params.get("folie")).toBe("Universitäten");
		expect(presentationSlideBookmarkParamKeys("de")).toEqual({
			chapter: "kapitel",
			sequence: "sequenz",
			thought: "gedanke",
			slide: "folie",
		});
	});
});
