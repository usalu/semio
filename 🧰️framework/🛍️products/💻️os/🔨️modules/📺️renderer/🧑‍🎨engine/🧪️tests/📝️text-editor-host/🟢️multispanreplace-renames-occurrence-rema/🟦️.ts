/** 🟢️ The multispanreplace renames occurrence remaps tests of the framework renderer hosts suite. */
import { expect, it } from "vitest";
import { lineRangeAt, multiSpanReplace } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔵 Registers the multispanreplace renames occurrence remaps tests of the framework renderer hosts suite. */
export function registerMultispanreplaceRenamesOccurrenceRemapsTests(): void {
  it("multiSpanReplace renames every occurrence and remaps spans", () => {
    const result = multiSpanReplace(
      "MATCH (a:Piece) RETURN a.name",
      [
        { start: 7, end: 8 },
        { start: 23, end: 24 },
      ],
      "piece",
    );
    expect(result.text).toBe("MATCH (piece:Piece) RETURN piece.name");
    expect(result.occurrences).toEqual([
      { start: 7, end: 12 },
      { start: 23, end: 28 },
    ]);
  });
  it("lineRangeAt finds the line containing an offset", () => {
    const text = "MATCH (a)\nWHERE a.x = 1\nRETURN a";
    const range = lineRangeAt(text, 15);
    expect(text.slice(range.start, range.end)).toBe("WHERE a.x = 1");
  });
}
