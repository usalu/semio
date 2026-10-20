/** 🔸️ The keeps consecutive morph slides tests of the expandThoughtSlides suite. */
import { expect, it } from "vitest";
import { arrangementRestDispositions, expandThoughtSlides, type Thought } from "../../../📦️packages/🟦️typescript/🟦️.ts";

/** 🔹 Registers the keeps consecutive morph slides tests of the expandThoughtSlides suite. */
export function registerKeepsConsecutiveMorphSlidesTests(): void {
	it("keeps consecutive morph slides without extra render slides", () => {
		const thought: Thought = {
			id: "move",
			participants: [{ id: "box" }],
			embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
			slides: [
				{
					arrangement: {
						id: "left",
						dispositions: [
							{
								participantId: "box",
								embodimentId: "box--main",
								emphasis: "active",
								position: { x: 0.1, y: 0.2, width: 0.3, height: 0.2 },
							},
						],
					},
					transition: { kind: "morph" },
				},
				{
					arrangement: {
						id: "right",
						dispositions: [
							{
								participantId: "box",
								embodimentId: "box--main",
								emphasis: "active",
								position: { x: 0.6, y: 0.2, width: 0.3, height: 0.2 },
							},
						],
					},
				},
			],
		};
		expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual(["left", "right"]);
	});
	it("keeps morphTo on the source slide without expanding arrangements", () => {
		const thought: Thought = {
			id: "split",
			participants: [{ id: "whole" }, { id: "tile-a" }],
			embodiments: [
				{ kind: "figure", id: "whole--figure", src: "/a.png" },
				{ kind: "figure", id: "tile-a--figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
			],
			slides: [
				{
					arrangement: {
						id: "whole",
						dispositions: [
							{
								participantId: "whole",
								embodimentId: "whole--figure",
								emphasis: "active",
								morphTo: [
									{
										participantId: "tile-a",
										position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 },
									},
								],
							},
						],
					},
					transition: { kind: "morph" },
				},
				{
					arrangement: {
						id: "tiles",
						dispositions: [
							{
								participantId: "tile-a",
								embodimentId: "tile-a--figure",
								emphasis: "active",
							},
						],
					},
				},
			],
		};
		const whole = thought.slides[0]!.arrangement;
		expect(whole.dispositions[0]?.morphTo).toHaveLength(1);
		expect(arrangementRestDispositions(whole)).toHaveLength(1);
	});
	it("starts a new morph run after a fade transition", () => {
		const thought: Thought = {
			id: "fade",
			participants: [{ id: "box" }],
			embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
			slides: [
				{
					arrangement: {
						id: "a",
						dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
					},
					transition: { kind: "morph" },
				},
				{
					arrangement: {
						id: "b",
						dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
					},
					transition: { kind: "fade" },
				},
				{
					arrangement: {
						id: "c",
						dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
					},
				},
			],
		};
		const expanded = expandThoughtSlides(thought);
		expect(expanded[0]?.autoAnimateId).toBe("fade--m0");
		expect(expanded[1]?.autoAnimateId).toBe("fade--m0");
		expect(expanded[2]?.autoAnimateId).toBeUndefined();
	});
}
