import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "@semio-tech/process/leases";
import { printPdfArrayV1, printPdfGlyphsV1, type PrintPdfPageV1 } from "../../../../📦️artifacts/📖️pdf/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";

export interface ThesisPaintRunV1 { readonly text: string; readonly color: string }
export interface ThesisFilledHeadingV1 { readonly title: string; readonly number: string }
const compact = (value: string): string => value.normalize("NFC").replace(/\s+/g, "");

/** 🎨️ Reads actual PDF text and filled-path paints through the retained page/control interface. */
export function thesisPdfPaintFactsV1(page: PrintPdfPageV1, control: ResourceLeaseControl): { readonly runs: readonly ThesisPaintRunV1[]; readonly fills: readonly string[] } {
  const runs: ThesisPaintRunV1[] = [], stack: string[] = [], fills = new Set<string>();
  let fill = "#000000";
  for (const [index, operation] of page.operators.fnArray.entries()) {
    if (index % 256 === 0) checkResourceLeaseControl(control);
    const args = printPdfArrayV1(page.operators.argsArray[index]);
    if (operation === "save" || operation === "paintFormXObjectBegin") stack.push(fill);
    else if (operation === "restore" || operation === "paintFormXObjectEnd") {
      const previous = stack.pop();
      if (!previous) throw Error("Balanced thesis PDF paint state required");
      fill = previous;
    } else if (operation === "setFillRGBColor") {
      if (typeof args[0] !== "string" || !/^#[a-f\d]{6}$/i.test(args[0])) throw Error("Actual thesis RGB fill required");
      fill = args[0].toLowerCase();
    } else if (operation === "constructPath" && ["fill", "eoFill", "fillStroke", "eoFillStroke", "closeFillStroke", "closeEOFillStroke"].includes(String(args[0]))) fills.add(fill);
    else if (operation === "showText") runs.push(Object.freeze({ text: printPdfGlyphsV1(args[0]).flatMap(glyph => typeof glyph === "number" ? [] : [glyph.unicode ?? ""]).join(""), color: fill }));
  }
  if (stack.length) throw Error("Balanced thesis PDF paint state required");
  checkResourceLeaseControl(control);
  return Object.freeze({ runs: Object.freeze(runs), fills: Object.freeze([...fills]) });
}

/** 🔢️ Requires every contiguous prefix/leaf numeral run after one filled title to use its actual ink. */
export function thesisFilledHeadingNumberPaintV1(runs: readonly ThesisPaintRunV1[], heading: ThesisFilledHeadingV1, foreground: string): readonly ThesisPaintRunV1[] {
  if (validateJsonSchemaSubset(schema, { runs, heading, foreground }).length) throw Error("Closed thesis heading paint facts required");
  let size = 0;
  const chunks = runs.flatMap((run, index) => {
    const text = compact(run.text), start = size;
    size += text.length;
    if (size > 4194304) throw Error("Bounded thesis heading paint text required");
    return text ? [{ index, start, end: size, text }] : [];
  });
  const text = chunks.map(chunk => chunk.text).join(""), title = compact(heading.title), ink = foreground.toLowerCase(), starts = new Map(chunks.map(chunk => [chunk.start, chunk.index])), ends = new Map(chunks.map(chunk => [chunk.end, chunk.index])), matches: { start: number; end: number }[] = [];
  if (!title) throw Error("Visible thesis heading title required");
  for (let cursor = text.indexOf(title); cursor >= 0; cursor = text.indexOf(title, cursor + 1)) {
    const start = starts.get(cursor), end = ends.get(cursor + title.length);
    if (start !== undefined && end !== undefined) matches.push({ start, end });
  }
  if (matches.length !== 1) throw Error("One actual filled heading title required: " + heading.title);
  const selected = matches[0]!;
  for (let index = selected.start; index <= selected.end; index++) if (compact(runs[index]!.text) && runs[index]!.color.toLowerCase() !== ink) throw Error("Filled heading title ink mismatch: " + heading.title);
  const numbers: ThesisPaintRunV1[] = [];
  for (let index = selected.end + 1; index < runs.length; index++) {
    const run = runs[index]!, value = compact(run.text);
    if (!value) continue;
    if (!/^[0-9.]+$/.test(value)) break;
    numbers.push(run);
    if (run.color.toLowerCase() !== ink) throw Error("Filled heading numeral ink mismatch: " + heading.title + " / " + value + " / " + run.color);
  }
  if (numbers.map(run => compact(run.text)).join("") !== heading.number) throw Error("Exact filled heading numeral required: " + heading.title + " / " + heading.number);
  return Object.freeze(numbers.map(run => Object.freeze({ ...run })));
}
