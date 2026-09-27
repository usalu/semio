/** 📝️ Explicit line layout in local drawing coordinates; CRLF is one break. */
export const DRAWING_TEXT_LINE_HEIGHT = 1.2;

export function* drawingTextLines(content: string): IterableIterator<string> {
  const breaks = /\r\n|[\r\n]/g;
  let start = 0;
  for (let match = breaks.exec(content); match; match = breaks.exec(content)) {
    yield content.slice(start, match.index);
    start = match.index + match[0].length;
  }
  yield content.slice(start);
}

/** 📏️ Font-independent fallback extent until a font measurement port supplies shaped glyph bounds. */
export function drawingTextFallbackExtent(content: string, size: number): [number, number] {
  let columns = 0, count = 0;
  for (const line of drawingTextLines(content)) { columns = Math.max(columns, Array.from(line).length); count++; }
  return [columns * size * 0.6, count * size * DRAWING_TEXT_LINE_HEIGHT];
}
