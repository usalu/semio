/** 🔶️ The assigns one auto animate tests of the expandThoughtSlides suite. */
import { expect, it } from "vitest";
import { arrangementRestDispositions, buildResolutionScope, expandThoughtSlides, resolveArrangement, type Thought } from "../../../📦️packages/🟦️typescript/🟦️.ts";

/** 🔷 Registers the assigns one auto animate tests of the expandThoughtSlides suite. */
export function registerAssignsOneAutoAnimateTests(): void {
	it("assigns one auto-animate id per morph run", () => {
		const thought: Thought = {
			id: "morph",
			participants: [{ id: "label" }],
			embodiments: [
				{ kind: "text", id: "source", lines: ["Reuse"], level: "heading" },
				{ kind: "text", id: "target", lines: ["Remanufacture"], level: "heading" },
			],
			slides: [
				{
					arrangement: {
						id: "source",
						dispositions: [{ participantId: "label", embodimentId: "source", emphasis: "active" }],
					},
					transition: { kind: "morph" },
				},
				{
					arrangement: {
						id: "mapping",
						dispositions: [{ participantId: "label", embodimentId: "target", emphasis: "active" }],
					},
				},
			],
		};
		const expanded = expandThoughtSlides(thought);
		expect(expanded.map((slide) => slide.id)).toEqual(["source", "mapping"]);
		expect(expanded.every((slide) => slide.autoAnimateId === "morph--m0")).toBe(true);
	});
	it("keeps morphFrom on label slides without expanding arrangements", () => {
		const thought: Thought = {
			id: "merge",
			participants: [{ id: "col1" }, { id: "labels" }],
			embodiments: [
				{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
				{ kind: "text", id: "label", lines: ["A"], level: "heading" },
				{ kind: "text", id: "stack", lines: ["A"], level: "heading", morphRoot: "heading-block" },
			],
			slides: [
				{
					arrangement: {
						id: "focus",
						dispositions: [
							{
								participantId: "col1",
								embodimentId: "crop",
								emphasis: "active",
								position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
							},
						],
					},
					transition: { kind: "morph" },
				},
				{
					arrangement: {
						id: "labels",
						dispositions: [
							{
								participantId: "labels",
								embodimentId: "stack",
								emphasis: "active",
								position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
								morphFrom: [
									{
										participantId: "col1",
										embodimentId: "crop",
										position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
										targetLineIndex: 0,
									},
								],
							},
						],
					},
				},
			],
		};
		const labels = thought.slides.find((slide) => slide.arrangement.id === "labels")!.arrangement;
		const morphFrom = labels.dispositions[0]?.morphFrom?.[0];
		expect(morphFrom?.targetLineIndex).toBe(0);
		expect(morphFrom?.position).toEqual({ x: 0.38, y: 0.12, width: 0.24, height: 0.24 });
		expect(arrangementRestDispositions(labels)).toHaveLength(1);
	});
	it("resolves morph-into targets with --label morph ids", () => {
		const scope = buildResolutionScope([
			{
				participants: [{ id: "catalogue-col1" }],
				embodiments: [
					{ kind: "text", id: "catalogue-col1--label", lines: ["Rippenplatte"], level: "heading" },
				],
			},
		]);
		const resolved = resolveArrangement(scope, {
			id: "labels",
			dispositions: [
				{
					participantId: "catalogue-col1",
					embodimentId: "catalogue-col1--label",
					emphasis: "active",
					position: { x: 0.1, y: 0.4, width: 0.2, height: 0.1 },
					morphFrom: [
						{
							participantId: "Rippenplatte 1",
							embodimentId: "Rippenplatte 1-figure",
							position: { x: 0.1, y: 0.4, width: 0.2, height: 0.1 },
						},
					],
				},
			],
		});
		expect(resolved[0]?.morphId).toBe("catalogue-col1--label");
	});
	it("preserves declarative settleBeforeMorphTo on arrangements", () => {
		const thought: Thought = {
			id: "media",
			participants: [{ id: "col1" }],
			embodiments: [
				{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
				{ kind: "text", id: "label", lines: ["A"], level: "heading" },
			],
			slides: [
				{
					arrangement: {
						id: "focus",
						settleBeforeMorphTo: ["labels"],
						dispositions: [
							{
								participantId: "col1",
								embodimentId: "crop",
								emphasis: "active",
								position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
								style: { opacity: 0 },
							},
						],
					},
					transition: { kind: "morph" },
				},
				{
					arrangement: {
						id: "labels",
						dispositions: [
							{
								participantId: "col1",
								embodimentId: "label",
								emphasis: "active",
								position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
							},
						],
					},
				},
			],
		};
		const focus = expandThoughtSlides(thought).find((slide) => slide.id === "focus");
		expect(focus?.arrangement.settleBeforeMorphTo).toEqual(["labels"]);
	});
}
