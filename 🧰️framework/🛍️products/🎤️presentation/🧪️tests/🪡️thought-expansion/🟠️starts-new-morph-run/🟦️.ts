/** 🟠️ The starts new morph run tests of the expandThoughtSlides suite. */
import { expect, it } from "vitest";
import { expandThoughtSlides, type Thought } from "../../../📦️packages/🟦️typescript/🟦️.ts";

/** 🟡 Registers the starts new morph run tests of the expandThoughtSlides suite. */
export function registerStartsNewMorphRunTests(): void {
	it("starts a new morph run when consecutive slides share no participants", () => {
		const thought: Thought = {
			id: "media",
			participants: [{ id: "catalogue" }, { id: "col1" }],
			embodiments: [
				{ kind: "figure", id: "catalogue--figure", src: "/a.png" },
				{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
				{ kind: "text", id: "label", lines: ["A"], level: "heading" },
			],
			slides: [
				{
					arrangement: {
						id: "catalogue",
						dispositions: [
							{ participantId: "catalogue", embodimentId: "catalogue--figure", emphasis: "active" },
						],
					},
					transition: { kind: "morph" },
				},
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
		const expanded = expandThoughtSlides(thought);
		expect(expanded.map((slide) => slide.id)).toEqual(["catalogue", "focus", "labels"]);
		expect(expanded[0]?.autoAnimateId).toBeUndefined();
		expect(expanded[1]?.autoAnimateId).toBe("media--m1");
		expect(expanded[2]?.autoAnimateId).toBe("media--m1");
	});
}
