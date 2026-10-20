/** 🟠️ The keeps reveal morph data tests of the presentation interaction geometry suite. */
import { buildResolutionScope, resolveArrangement } from "@semio-tech/presentation";
import { expect, it } from "vitest";
import { buildInteractiveSlideLayout } from "../../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource } from "../../🧰️presentation-deck-helpers/🟦️.tsx";

/** 🟡 Registers the keeps reveal morph data tests of the presentation interaction geometry suite. */
export function registerKeepsRevealMorphDataTests(): void {
  it("keeps reveal morph data-id on canvas placements only", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "title" }, { id: "description" }],
        embodiments: [
          { kind: "text", id: "title--full", lines: ["A"], level: "heading", morphRoot: "heading-line" },
          {
            kind: "text",
            id: "description--full",
            lines: ["Long description"],
            level: "heading",
            morphRoot: "heading-block",
          },
        ],
      },
    ]);
    const flow = buildInteractiveSlideLayout(
      "goal",
      resolveArrangement(scope, {
        id: "goal",
        dispositions: [
          { participantId: "title", embodimentId: "title--full", emphasis: "muted" },
          { participantId: "description", embodimentId: "description--full", emphasis: "muted" },
        ],
      }),
      true,
    );
    expect(flow.placements.every((entry) => entry.revealMorphId === undefined)).toBe(true);
    const canvas = buildInteractiveSlideLayout(
      "tiles",
      resolveArrangement(scope, {
        id: "tiles",
        dispositions: [
          {
            participantId: "description",
            embodimentId: "description--full",
            emphasis: "active",
            position: { x: 0.1, y: 0.2, width: 0.8, height: 0.3 },
          },
        ],
      }),
      true,
    );
    expect(canvas.placements[0]?.revealMorphId).toBe("description");
  });
  it("styles source ghosts hidden at rest and visible during auto-animate", async () => {
    const css = globalsCssSource;
    expect(css).toContain(".reveal .presentation-source-ghost");
    expect(css).toContain('section[data-auto-animate="pending"] .presentation-source-ghost');
    expect(css).not.toContain('section[title="catalogue-focus"]');
  });
}
