/** 🟠️ The sets revealmorphfrommorphtoframe previous slide tests of the PresentationDeck suite. */
import { buildResolutionScope, splitFigureGrid } from "@semio-tech/presentation";
import { expect, it } from "vitest";
import { figureCropBackgroundVars, mosaicWindowedCoverVars, resolveRevealArrangement } from "@semio-tech/presentation-react";

/** 🟡 Registers the sets revealmorphfrommorphtoframe previous slide tests of the PresentationDeck suite. */
export function registerSetsRevealmorphfrommorphtoframePreviousSlideTests(): void {
  it("sets revealMorphFromMorphToFrame from the previous slide morphTo slots", () => {
    const scope = buildResolutionScope([
      {
        participants: [{ id: "tile" }],
        embodiments: [
          {
            kind: "figure",
            id: "tile-figure",
            src: "/a.png",
            crop: { x: 0, y: 0, width: 0.5, height: 1 },
          },
        ],
      },
    ]);
    const gridFrame = { x: 0.1, y: 0.1, width: 0.2, height: 0.3 };
    const focusFrame = { x: 0.4, y: 0.2, width: 0.15, height: 0.18 };
    const resolved = resolveRevealArrangement(
      scope,
      {
        id: "focus",
        dispositions: [
          {
            participantId: "tile",
            embodimentId: "tile-figure",
            emphasis: "active",
            position: focusFrame,
          },
        ],
      },
      {
        previousSlide: {
          arrangement: {
            id: "catalogue",
            dispositions: [
              {
                participantId: "catalogue",
                embodimentId: "catalogue--figure",
                emphasis: "active",
                morphTo: [{ participantId: "tile", embodimentId: "tile-figure", position: gridFrame }],
              },
            ],
          },
        },
      },
    );
    expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphFromMorphToFrame).toEqual(gridFrame);
  });
  it("assigns windowed mosaic cover per split tile", () => {
    const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
    const mosaic = { rows: 2, columns: 2, frame };
    const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
    const embodiment = {
      kind: "figure" as const,
      id: "",
      src: "/catalogue.png",
      mosaic,
    };
    const expected = tiles.map((tile, index) => {
      const column = index % 2;
      const row = Math.floor(index / 2);
      return mosaicWindowedCoverVars({ column, row }, mosaic, frame, 1);
    });
    const varsList = tiles.map((tile) => figureCropBackgroundVars(embodiment, tile.crop, tile.position));
    expect(varsList.map((vars) => vars["--presentation-figure-bg-size" as keyof typeof vars])).toEqual(expected.map((entry) => entry.size));
    expect(varsList.map((vars) => vars["--presentation-figure-bg-position" as keyof typeof vars])).toEqual(expected.map((entry) => `${entry.posX}% ${entry.posY}%`));
  });
}
