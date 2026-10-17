/** 🧷️ Shared fixtures of the presentation core suites: the thought scope helper and the sample intro deck. */
import { buildResolutionScope, intro, type ResolutionScope, type Thought } from "../../📦️packages/🟦️typescript/🟦️.ts";

export function thoughtScope(thought: Thought): ResolutionScope {
	return buildResolutionScope([thought]);
}

export const sampleIntro = intro({
	title: {
		full: ["Line A", "Line B", "Line C"],
		short: "Short title",
	},
	description: { full: ["D1", "D2"], short: "D short" },
	goal: ["G1"],
	authors: {
		lines: [[{ name: "Alice" }, { name: "Bob" }], [{ name: "Carol", marks: ["1", "b"] }]],
	},
	affiliations: {
		steps: [
			[{ mark: "a", name: "Faculty" }],
			[
				{ mark: "a", name: "Faculty" },
				{ mark: "1", name: "University" },
				{ mark: "2", name: "Other University" },
			],
			[
				{ mark: "a", name: "Faculty" },
				{
					mark: "1",
					name: "University",
					shortName: "LUH",
					suffix: { mark: "x", name: "Chair X" },
				},
				{
					mark: "2",
					name: "Other University",
					shortName: "UdK",
					suffix: { mark: "y", name: "Chair Y" },
				},
			],
		],
	},
});
