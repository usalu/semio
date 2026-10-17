/** 🔄️ The presentation core, analogies, video and pdf embodiments and tile play. */
import { describe, expect, it } from "vitest";
import { ANALOGY_PARTICIPANT_VISUAL, FIGURE_TILE_PDF_PAGE_ASPECT, analogy, buildResolutionScope, buildTileMorphPrompt, expandThoughtSlides, figureTileMediaKindFromFile, isFigureTileMediaFile, moveNormalizedRect, parseGridEngagement, populateTileDraftsFromGrid, resizeNormalizedRect, resolveArrangement, type Arrangement, type FigureTileDraft, type FigureTileSource } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { thoughtScope } from "../🧷️presentation-core-kit/🟦️.ts";

describe("analogy", () => {
	const sampleAnalogy = analogy({
		source: { label: "Reuse", figure: "/reuse.png" },
		target: { label: "Remanufacture", figure: "/remanufacture.png" },
	});

	it("builds two morph slides", () => {
		const thought = sampleAnalogy.chapters[0]!.sequences[0]!.thoughts[0]!;
		expect(thought.slides.map((slide) => slide.arrangement.id)).toEqual(["source", "mapping"]);
		expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual(["source", "mapping"]);
	});

	it("resolves positioned visual dispositions", () => {
		const thought = sampleAnalogy.chapters[0]!.sequences[0]!.thoughts[0]!;
		const mapping = thought.slides.find((slide) => slide.arrangement.id === "mapping")!.arrangement;
		const resolved = resolveArrangement(thoughtScope(thought), mapping);
		const visual = resolved.find((resolvedDisposition) => resolvedDisposition.participant.id === ANALOGY_PARTICIPANT_VISUAL);
		expect(visual?.position).toEqual({ x: 0.1, y: 0.35, width: 0.8, height: 0.5 });
		expect(visual?.embodiment.kind).toBe("figure");
	});
});

describe("video and pdf embodiments", () => {
	it("resolves video and pdf kinds", () => {
		const scope = buildResolutionScope([
			{
				participants: [{ id: "clip" }, { id: "doc" }],
				embodiments: [
					{ kind: "video", id: "clip--video", src: "/demo.mp4", muted: true },
					{ kind: "pdf", id: "doc--pdf", src: "/paper.pdf", page: 2 },
				],
			},
		]);
		const arrangement: Arrangement = {
			id: "slide",
			dispositions: [
				{ participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
				{
					participantId: "doc",
					embodimentId: "doc--pdf",
					emphasis: "active",
					position: { x: 0.2, y: 0.2, width: 0.6, height: 0.6 },
				},
			],
		};
		const resolved = resolveArrangement(scope, arrangement);
		expect(resolved[0]?.embodiment.kind).toBe("video");
		expect(resolved[1]?.embodiment.kind).toBe("pdf");
		if (resolved[1]?.embodiment.kind === "pdf") {
			expect(resolved[1].embodiment.page).toBe(2);
			expect(resolved[1].embodiment.pages).toBeUndefined();
		}
		expect(resolved[1]?.position?.width).toBe(0.6);
	});

	it("resolves pdf embodiments with a page subset", () => {
		const scope = buildResolutionScope([
			{
				participants: [{ id: "doc" }],
				embodiments: [
					{
						kind: "pdf",
						id: "doc--pdf",
						src: "/thesis.pdf",
						page: 25,
						pages: [1, 12, 25, 35],
					},
				],
			},
		]);
		const resolved = resolveArrangement(scope, {
			id: "slide",
			dispositions: [{ participantId: "doc", embodimentId: "doc--pdf", emphasis: "active" }],
		});
		expect(resolved[0]?.embodiment.kind).toBe("pdf");
		if (resolved[0]?.embodiment.kind === "pdf") {
			expect(resolved[0].embodiment.pages).toEqual([1, 12, 25, 35]);
		}
	});
});

describe("tile play", () => {
	const source: FigureTileSource = {
		src: "/catalogue.png",
		sourceAspect: 1222 / 896,
		frame: { x: 0.127, y: 0.1, width: 0.746, height: 0.75 },
	};

	it("seeds grid drafts with frame-relative crops", () => {
		const drafts = populateTileDraftsFromGrid({ source, rows: 2, columns: 2 });
		expect(drafts).toHaveLength(4);
		expect(drafts[0]?.id).toBe("tile-r0-c0");
		expect(drafts[0]?.crop.x).toBeCloseTo(source.frame.x, 10);
	});

	it("parses grid engagement tokens", () => {
		expect(parseGridEngagement("3x5")).toEqual({ rows: 3, columns: 5 });
		expect(parseGridEngagement("2×4")).toEqual({ rows: 2, columns: 4 });
		expect(parseGridEngagement("add")).toBeNull();
	});

	it("clamps move and resize to the unit square", () => {
		const rect = { x: 0.8, y: 0.8, width: 0.15, height: 0.15 };
		const moved = moveNormalizedRect(rect, 0.2, 0.2);
		expect(moved.x + moved.width).toBeLessThanOrEqual(1.001);
		expect(moved.y + moved.height).toBeLessThanOrEqual(1.001);
		const resized = resizeNormalizedRect({ x: 0.1, y: 0.1, width: 0.4, height: 0.4 }, "se", 0.5, 0.5);
		expect(resized.width).toBeLessThanOrEqual(0.9);
		expect(resized.height).toBeLessThanOrEqual(0.9);
	});

	it("allows overlapping tile crops in the morph prompt", () => {
		const drafts: FigureTileDraft[] = [
			{ id: "a", name: "Tile A", crop: { x: 0.1, y: 0.1, width: 0.5, height: 0.5 } },
			{ id: "b", name: "Tile B", crop: { x: 0.3, y: 0.3, width: 0.5, height: 0.5 } },
		];
		const prompt = buildTileMorphPrompt(source, drafts);
		expect(prompt).toContain("Tile A");
		expect(prompt).toContain("Tile B");
		expect(prompt).toContain("mit-bestand/präsentation/33.projektetage/spec.ts");
		expect(prompt).toContain("morphTo");
	});

	it("detects supported tile media kinds from file metadata", () => {
		expect(figureTileMediaKindFromFile("image/png", "photo.png")).toBe("figure");
		expect(figureTileMediaKindFromFile("image/svg+xml", "icon.svg")).toBe("figure");
		expect(figureTileMediaKindFromFile("video/mp4", "clip.mp4")).toBe("video");
		expect(figureTileMediaKindFromFile("application/pdf", "doc.pdf")).toBe("pdf");
		expect(figureTileMediaKindFromFile("", "notes.pdf")).toBe("pdf");
		expect(isFigureTileMediaFile("text/plain", "readme.txt")).toBe(false);
	});

	it("embeds video and pdf kind in the morph prompt", () => {
		const prompt = buildTileMorphPrompt(
			{ src: "/clip.mp4", kind: "video", sourceAspect: 16 / 9, frame: { x: 0, y: 0, width: 1, height: 1 } },
			[{ id: "t1", name: "Intro", crop: { x: 0, y: 0, width: 0.5, height: 0.5 } }],
		);
		expect(prompt).toContain("kind: video");
		expect(prompt).toContain("video(...)");
		const pdfPrompt = buildTileMorphPrompt(
			{ src: "/paper.pdf", kind: "pdf", pdfPage: 2, sourceAspect: FIGURE_TILE_PDF_PAGE_ASPECT, frame: { x: 0, y: 0, width: 1, height: 1 } },
			[],
		);
		expect(pdfPrompt).toContain("kind: pdf");
		expect(pdfPrompt).toContain("pdfPage: 2");
	});
});
