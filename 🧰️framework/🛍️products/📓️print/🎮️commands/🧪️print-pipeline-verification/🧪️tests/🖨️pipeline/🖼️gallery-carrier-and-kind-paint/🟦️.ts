/** 🖼️ The print gallery verification: the gallery carrier and the paint of every kind in the compiled PDF fixtures. */
import { PRINT_PDF_OPS_V1 as OPS, printPdfArrayV1, printPdfNumberV1, printPdfVectorV1, printPdfNameV1, printPdfBoxV1 } from "../../../../../📦️artifacts/📖️pdf/🟦️.ts";
import { loadVizCatalog } from "../../../../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { renderVizGalleryDocument } from "../../../../../🔨️modules/📊️visualization-gallery/🏭️artifact-generation/🟦️.ts";
import { withCompiledPrintProbeV1 } from "../../../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { latexRoot, openPrintPdf, outputDirectory, pipelineScope, type PrintPipelineVerificationV1, Util } from "../🟦️.ts";

//#region 🧪️PrintPipelineTests
/** 🪟️ Measures actual gallery content against its effective PGF frame without changing that frame. */
export async function verifyPrintGalleryCarrier(context:PrintPipelineVerificationV1): Promise<void> {
  const{execution}=pipelineScope(context);
  const control = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.galleryCarrier;
  const { extent } = await import("d3-array");
  
  
  const title = { en: "Gallery carrier controls", de: "Galerierahmen-Kontrollen" };
  let source = renderVizGalleryDocument(title, control.cases.map((entry: any) => ({ group: title, leafId: entry.slug, slug: entry.slug })));
  const failures: string[] = [];
  const mark = (name: string) => `\\special{pdf:literal direct /${name} BMC}`;
  const end = "\\special{pdf:literal direct EMC}";
  const reference = (index: number | string) => `\\coordinate (SemioCarrierSW) at (current bounding box.south west);\\coordinate (SemioCarrierNE) at (current bounding box.north east);\n${mark(`SemioFrame${index}`)}\n\\begin{pgfinterruptboundingbox}\\draw[line width=0.01pt] (SemioCarrierSW) rectangle (SemioCarrierNE);\\end{pgfinterruptboundingbox}\n${end}`;
  for (const [index, entry] of control.cases.entries()) source = source.replace(`\\SemioVizChart{${entry.slug}}`, `${mark(`SemioCarrier${index}`)}\n\\SemioVizChart{${entry.slug}}\n${end}\n${reference(index)}`);
  const explicit = `\\begin{VizFigure}[title={\\SemioVizLocalized{Explicit height}{Explizite Höhe}},width=${control.width},height=${control.explicitHeight}]\n${mark("SemioCarrierExplicit")}\n\\draw (0,0) rectangle (10,10);\n${end}\n${reference("Explicit")}\n\\end{VizFigure}`;
  source = source.replace("\\end{document}", explicit + "\n\\end{document}");
  const box = (points: number[][]) => [...extent(points.map(point => point[0]!)), ...extent(points.map(point => point[1]!))] as [number, number, number, number];
  const documents = [["en", "light"], ["de", "dark"]] as const;
  for (const [language, appearance] of documents) {
    const documentSource = source.replace("theme=light,language=de", `theme=${appearance},language=${language}`);
    await withCompiledPrintProbeV1(execution,{version:1,kind:"probe",name:"carrier.tex",source:documentSource},async probe=>{
    const pdf = await openPrintPdf(context,await probe.read("carrier.pdf",67108864),probe.control);
    const frames = new Map<string, number[]>(), origins = new Map<string, number>(), pages = new Map<string, number>(), pageWidths = new Map<number, number>(), panels: { page: number; bounds: number[] }[] = [], geometry = new Map<string, number[][]>(), texts = new Map<string, string[]>();
    try {
      for (let number = 1; number <= pdf.numPages; number++) {
        const page = await pdf.getPage(number), operators = await page.getOperatorList(), content = await page.getTextContent({ includeMarkedContent: true });
        pageWidths.set(number, page.getViewport({ scale: 1 }).width);
        const graphics: { matrix: number[]; width: number; join: number; cap: number; limit: number }[] = [], marked: string[] = [];
        let matrix = [1, 0, 0, 1, 0, 0], width = 1, joinStyle = 0, cap = 0, limit = 10;
        for (const [index, id] of operators.fnArray.entries()) {
          const args=operators.argsArray[index];if(!args)throw Error("Complete PDF argument roster required");
          if (id === OPS.save || id === OPS.paintFormXObjectBegin) { graphics.push({ matrix: [...matrix], width, join: joinStyle, cap, limit }); if (id === OPS.paintFormXObjectBegin && args[0]) matrix = Util.transform(matrix,printPdfVectorV1(args[0],6)); }
          else if (id === OPS.restore || id === OPS.paintFormXObjectEnd) { const saved = graphics.pop(); assert.ok(saved, "gallery PDF graphics stack underflow"); matrix = saved.matrix; width = saved.width; joinStyle = saved.join; cap = saved.cap; limit = saved.limit; }
          else if (id === OPS.transform) matrix = Util.transform(matrix,printPdfVectorV1(args,6));
          else if (id === OPS.setLineWidth) width = printPdfNumberV1(args[0]);
          else if (id === OPS.setLineJoin) joinStyle = printPdfNumberV1(args[0]);
          else if (id === OPS.setLineCap) cap = printPdfNumberV1(args[0]);
          else if (id === OPS.setMiterLimit) limit = printPdfNumberV1(args[0]);
          else if (id === OPS.setGState) { for (const rawState of printPdfArrayV1(args[0])) { const state=printPdfArrayV1(rawState);if (state[0] === "LW") width = printPdfNumberV1(state[1]); else if (state[0] === "LJ") joinStyle = printPdfNumberV1(state[1]); else if (state[0] === "LC") cap = printPdfNumberV1(state[1]); else if (state[0] === "ML") limit = printPdfNumberV1(state[1]); } }
          else if (id === OPS.beginMarkedContent || id === OPS.beginMarkedContentProps) marked.push(printPdfNameV1(args[0]));
          else if (id === OPS.endMarkedContent) marked.pop();
          else if (id === OPS.constructPath && args[2]) {
            const owner = [...marked].reverse().find(name => /^(?:SemioFrame|SemioCarrier)/.test(name)) ?? "", bounds = printPdfVectorV1(args[2],4);
            const points = [[bounds[0]!, bounds[1]!], [bounds[2]!, bounds[1]!], [bounds[2]!, bounds[3]!], [bounds[0]!, bounds[3]!]];
            points.forEach(point => Util.applyTransform(point, matrix));
            if (owner.startsWith("SemioFrame")) { assert.ok(!frames.has(owner), `duplicate ${owner}`); frames.set(owner, box(points)); origins.set(owner, matrix[5]!); pages.set(owner, number); }
            else if (owner.startsWith("SemioCarrier")) {
              const stroked = [OPS.stroke, OPS.closeStroke, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(printPdfNameV1(args[0]));
              const reservation = Math.max(joinStyle === 0 ? limit : 1, cap === 2 ? Math.SQRT2 : 1);
              const horizontal = stroked ? reservation * width / 2 * Math.hypot(matrix[0]!, matrix[2]!) : 0, vertical = stroked ? reservation * width / 2 * Math.hypot(matrix[1]!, matrix[3]!) : 0;
              geometry.set(owner, [...(geometry.get(owner) ?? []), ...points.flatMap(point => [[point[0]! - horizontal, point[1]! - vertical], [point[0]! + horizontal, point[1]! + vertical]])]);
            }
            else if ([OPS.fill, OPS.eoFill].includes(printPdfNameV1(args[0]))) panels.push({ page: number, bounds: box(points) });
          }
        }
        assert.equal(graphics.length, 0, "unbalanced gallery PDF graphics stack");
        const textMarks: string[] = [];
        for (const item of content.items) {
          if (!("str" in item)) { if (item.type === "beginMarkedContent" || item.type === "beginMarkedContentProps") textMarks.push(String((item as { tag?: string }).tag)); else if (item.type === "endMarkedContent") textMarks.pop(); continue; }
          const owner = [...textMarks].reverse().find(name => name.startsWith("SemioCarrier")) ?? "";
          if (!owner.startsWith("SemioCarrier")) continue;
          texts.set(owner, [...(texts.get(owner) ?? []), item.str]);
          const style = content.styles[item.fontName], ascent = style?.ascent ?? 0.8, descent = style?.descent ?? -0.2;
          const [a, b, c, d, x, y] = item.transform, length = Math.hypot(a!, b!);
          const points = [descent, ascent].flatMap(vertical => [0, item.width].map(horizontal => [x! + a! / length * horizontal + c! * vertical, y! + b! / length * horizontal + d! * vertical]));
          geometry.set(owner, [...(geometry.get(owner) ?? []), ...points]);
        }
      }
      for (const [index, entry] of control.cases.entries()) {
        const frame = frames.get(`SemioFrame${index}`), points = geometry.get(`SemioCarrier${index}`);
        assert.ok(frame && points?.length, `${language}/${entry.slug}: marked frame or content missing`);
        const panel = panels.filter(candidate => candidate.page === pages.get(`SemioFrame${index}`) && candidate.bounds[0]! < frame[0]! && candidate.bounds[1]! > frame[1]! && candidate.bounds[2]! < frame[2]! && candidate.bounds[3]! > frame[3]!).sort((left, right) => (left.bounds[1]! - left.bounds[0]!) * (left.bounds[3]! - left.bounds[2]!) - (right.bounds[1]! - right.bounds[0]!) * (right.bounds[3]! - right.bounds[2]!))[0]?.bounds;
        assert.ok(panel && panel[1]! - panel[0]! < pageWidths.get(pages.get(`SemioFrame${index}`)!)! - 0.1, `${language}/${entry.slug}: actual figure panel missing`);
        const bounds = box(points), tolerance = control.tolerancePt;
        if (bounds[0] < panel[0]! - tolerance || bounds[1] > panel[1]! + tolerance || bounds[2] < panel[2]! - tolerance || bounds[3] > panel[3]! + tolerance) failures.push(`${language}/${entry.slug}: actual content ${JSON.stringify(bounds)} escapes figure body ${JSON.stringify(panel)}`);
      }
      const explicitFrame = frames.get("SemioFrameExplicit");
      assert.ok(explicitFrame, "explicit-height reference missing");
      assert.ok(Math.abs(explicitFrame[1]! - explicitFrame[0]! - control.width * 72 / 25.4) < 0.2 && Math.abs(explicitFrame[3]! - explicitFrame[2]! - control.explicitHeight * 72 / 25.4) < 0.2, "explicit width/height contract changed");
      assert.ok(Math.min(...geometry.get("SemioCarrier2")!.map(point => point[1]!)) < origins.get("SemioFrame2")! - 6, "negative legend ink was not measured");
      console.log(`[print] Gallery carrier ${language}/${appearance}: ${control.cases.length} actual frames, marked path/text extents, explicit-height control, ${pdf.numPages} PDF.js pages measured`);
    } finally { await pdf.destroy(); }
    });
  }
  if (failures.length) throw Error(failures.join("\n"));
  assert.doesNotMatch(source, /width=80, height=40/, "gallery must select intrinsic height");
}

type PrintKindPaint = { marker: string; page: number; paths: number[][][]; pathOperations: number[][]; images: number[][][]; glyphs: { text: string; bounds: number[]; font: string; size: number; width: number }[]; bounds: number[]; signature: string; graphicSignature: string; glyphGeometrySignature: string };

/** 🖌️ Extracts only marked chart-body paint through the independent PDF.js operator and glyph readers. */
export async function readPrintKindPaint(pdf: Awaited<ReturnType<typeof openPrintPdf>>): Promise<PrintKindPaint[]> {
  const { extent } = await import("d3-array"), { polygonContains } = await import("d3-polygon"), prefix = "SemioVizLeaf", records = new Map<string, PrintKindPaint>(), seen = new Set<string>();
  const bounds = (points: number[][]): number[] => [...extent(points.map(point => point[0]!)), ...extent(points.map(point => point[1]!))] as number[];
  const filled = (raw: number[], evenOdd: boolean): boolean => {
    const rings: number[][][] = []; let ring: number[][] = [], current: number[] | undefined;
    for (let offset = 0; offset < raw.length;) {
      const operation = raw[offset++]!;
      if (operation === 0) { if (ring.length) rings.push(ring); current = [raw[offset++]!, raw[offset++]!]; ring = [current]; }
      else if (operation === 1) { current = [raw[offset++]!, raw[offset++]!]; ring.push(current); }
      else if (operation === 2 || operation === 3) {
        assert.ok(current, "filled curve requires starting point"); const curve = [current, [raw[offset++]!, raw[offset++]!], ...(operation === 2 ? [[raw[offset++]!, raw[offset++]!]] : []), [raw[offset++]!, raw[offset++]!]];
        for (let sample = 1; sample <= 64; sample++) { const t = sample / 64, u = 1 - t; ring.push([0, 1].map(axis => operation === 2 ? u ** 3 * curve[0]![axis]! + 3 * u * u * t * curve[1]![axis]! + 3 * u * t * t * curve[2]![axis]! + t ** 3 * curve[3]![axis]! : u * u * curve[0]![axis]! + 2 * u * t * curve[1]![axis]! + t * t * curve[2]![axis]!)); } current = curve.at(-1)!;
      } else if (operation === 4) { if (ring.length) rings.push(ring); ring = []; current = undefined; }
      else throw Error("unknown filled path operation " + operation);
    }
    if (ring.length) rings.push(ring);
    const winding = (point: number[]) => rings.reduce((sum, polygon) => sum + polygon.reduce((value, a, index) => { const b = polygon[(index + 1) % polygon.length]!, cross = (b[0]! - a[0]!) * (point[1]! - a[1]!) - (point[0]! - a[0]!) * (b[1]! - a[1]!); return value + (a[1]! <= point[1]! && b[1]! > point[1]! && cross > 0 ? 1 : a[1]! > point[1]! && b[1]! <= point[1]! && cross < 0 ? -1 : 0); }, 0), 0);
    return rings.some(polygon => polygon.some((a, index) => { const b = polygon[(index + 1) % polygon.length]!, dx = b[0]! - a[0]!, dy = b[1]! - a[1]!, length = Math.hypot(dx, dy); if (!length) return false; return [-1, 1].some(sign => { const point = [(a[0]! + b[0]!) / 2 - sign * dy * 1e-7, (a[1]! + b[1]!) / 2 + sign * dx * 1e-7]; return evenOdd ? rings.reduce((count, value) => count + Number(polygonContains(value as [number, number][], point as [number, number])), 0) % 2 !== 0 : winding(point) !== 0; }); }));
  };
  for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber++) {
    const page = await pdf.getPage(pageNumber), operators = await page.getOperatorList(), content = await page.getTextContent({ includeMarkedContent: true }), marked: string[] = [], stack: any[] = [], visibleText = new Set<string>();
    let state = { matrix: [1, 0, 0, 1, 0, 0], width: 1, join: 0, cap: 0, limit: 10, fillAlpha: 1, strokeAlpha: 1, textMode: 0, clip: [-Infinity, Infinity, -Infinity, Infinity] }, pendingClip = false;
    const owner = () => [...marked].reverse().find(name => name.startsWith(prefix));
    for (const [index, id] of operators.fnArray.entries()) {
      const args=operators.argsArray[index];if(!args)throw Error("Complete PDF argument roster required");
      if (id === OPS.save || id === OPS.paintFormXObjectBegin) { stack.push({ ...state, matrix: [...state.matrix] }); if (id === OPS.paintFormXObjectBegin && args[0]) state.matrix = Util.transform(state.matrix,printPdfVectorV1(args[0],6)); }
      else if (id === OPS.restore || id === OPS.paintFormXObjectEnd) { assert.ok(stack.length, "kind paint graphics stack underflow"); state = stack.pop(); }
      else if (id === OPS.transform) state.matrix = Util.transform(state.matrix,printPdfVectorV1(args,6));
      else if (id === OPS.setLineWidth) state.width = printPdfNumberV1(args[0]);
      else if (id === OPS.setLineJoin) state.join = printPdfNumberV1(args[0]);
      else if (id === OPS.setLineCap) state.cap = printPdfNumberV1(args[0]);
      else if (id === OPS.setMiterLimit) state.limit = printPdfNumberV1(args[0]);
      else if (id === OPS.setTextRenderingMode) state.textMode = printPdfNumberV1(args[0]);
      else if (id === OPS.clip || id === OPS.eoClip) pendingClip = true;
      else if (id === OPS.setGState) for (const rawState of printPdfArrayV1(args[0])) { const[key,value]=printPdfArrayV1(rawState);if(typeof key!=="string")throw Error("Actual PDF graphics state key required");if(!["LW","LJ","LC","ML","ca","CA"].includes(key))continue;const amount=printPdfNumberV1(value);if(key==="LW")state.width=amount;else if(key==="LJ")state.join=amount;else if(key==="LC")state.cap=amount;else if(key==="ML")state.limit=amount;else if(key==="ca")state.fillAlpha=amount;else if(key==="CA")state.strokeAlpha=amount; }
      else if (id === OPS.beginMarkedContent || id === OPS.beginMarkedContentProps) {
        const name = printPdfNameV1(args[0]); marked.push(name);
        if (name.startsWith(prefix)) { assert.ok(!seen.has(name), `duplicate kind body ${name}`); seen.add(name); records.set(name, { marker: name, page: pageNumber, paths: [], pathOperations: [], images: [], glyphs: [], bounds: [], signature: "", graphicSignature: "", glyphGeometrySignature: "" }); }
      } else if (id === OPS.endMarkedContent) { assert.ok(marked.length, "kind paint marked stack underflow"); marked.pop(); }
      else if ([OPS.showText, OPS.showSpacedText, OPS.nextLineShowText, OPS.nextLineSetSpacingShowText].includes(id)) { const name = owner(), mode = state.textMode % 4; if (name && (mode === 0 && state.fillAlpha > 0 || mode === 1 && state.strokeAlpha > 0 || mode === 2 && Math.max(state.fillAlpha, state.strokeAlpha) > 0)) visibleText.add(name); }
      else if (id === OPS.constructPath && args[2]) {
        const boundsData=printPdfBoxV1(args[2]),clipPoints=[[boundsData[0],boundsData[1]],[boundsData[2],boundsData[1]],[boundsData[2],boundsData[3]],[boundsData[0],boundsData[3]]]; clipPoints.forEach(point => Util.applyTransform(point, state.matrix));
        if (pendingClip) { const box = bounds(clipPoints); state.clip = [Math.max(state.clip[0]!, box[0]!), Math.min(state.clip[1]!, box[1]!), Math.max(state.clip[2]!, box[2]!), Math.min(state.clip[3]!, box[3]!)]; pendingClip = false; }
        const name = owner(); if (!name) continue;
        const fill = [OPS.fill, OPS.eoFill, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(printPdfNameV1(args[0])) && state.fillAlpha > 0, stroke = [OPS.stroke, OPS.closeStroke, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(printPdfNameV1(args[0])) && state.strokeAlpha > 0 && state.width > 0;
        if (!fill && !stroke) continue;
        const raw = printPdfVectorV1(printPdfArrayV1(args[1])[0]), points: number[][] = [], pathOperations: number[] = [];
        if (!stroke && !filled(raw, [OPS.eoFill, OPS.eoFillStroke, OPS.closeEOFillStroke].includes(printPdfNameV1(args[0])))) continue;
        for (let offset = 0; offset < raw.length;) { const operation = raw[offset++]!, size = [2, 2, 6, 4, 0][operation]; assert.ok(size !== undefined, `unknown PDF.js path operation ${operation}`); pathOperations.push(operation); for (let pair = 0; pair < size; pair += 2) { const point = [raw[offset + pair]!, raw[offset + pair + 1]!]; Util.applyTransform(point, state.matrix); points.push(point); } offset += size; }
        if (!points.length) continue;
        assert.ok(points.flat().every(Number.isFinite), `${name}: nonfinite painted path`);
        const pathBounds = bounds(points), reservation = Math.max(state.join === 0 ? state.limit : 1, state.cap === 2 ? Math.SQRT2 : 1), horizontal = stroke ? reservation * state.width / 2 * Math.hypot(state.matrix[0]!, state.matrix[2]!) : 0, vertical = stroke ? reservation * state.width / 2 * Math.hypot(state.matrix[1]!, state.matrix[3]!) : 0;
        if (!stroke && (pathBounds[0] === pathBounds[1] || pathBounds[2] === pathBounds[3])) continue;
        const box = [Math.max(pathBounds[0]! - horizontal, state.clip[0]!), Math.min(pathBounds[1]! + horizontal, state.clip[1]!), Math.max(pathBounds[2]! - vertical, state.clip[2]!), Math.min(pathBounds[3]! + vertical, state.clip[3]!)]; if (box[0]! >= box[1]! || box[2]! >= box[3]!) continue;
        const record = records.get(name)!; record.paths.push(points); record.pathOperations.push(pathOperations); record.bounds.push(...box);
      } else if ([OPS.paintImageXObject, OPS.paintInlineImageXObject].includes(id)) {
        const name = owner(); if (!name || state.fillAlpha <= 0) continue;
        const points = [[0, 0], [1, 0], [1, 1], [0, 1]]; points.forEach(point => Util.applyTransform(point, state.matrix)); const imageBounds = bounds(points), box = [Math.max(imageBounds[0]!, state.clip[0]!), Math.min(imageBounds[1]!, state.clip[1]!), Math.max(imageBounds[2]!, state.clip[2]!), Math.min(imageBounds[3]!, state.clip[3]!)]; if (box[0]! >= box[1]! || box[2]! >= box[3]!) continue;
        const record = records.get(name)!; record.images.push(points); record.bounds.push(...box);
      }
    }
    assert.equal(stack.length, 0, "unbalanced kind paint graphics stack"); assert.equal(marked.length, 0, "unbalanced kind paint marked stack");
    const textMarks: string[] = [];
    for (const item of content.items) {
      if (!("str" in item)) { if (item.type === "beginMarkedContent" || item.type === "beginMarkedContentProps") textMarks.push(String((item as { tag?: string }).tag)); else if (item.type === "endMarkedContent") textMarks.pop(); continue; }
      const name = [...textMarks].reverse().find(mark => mark.startsWith(prefix)); if (!name || !visibleText.has(name) || !item.str.trim() || item.width <= 0) continue;
      const style = content.styles[item.fontName], [a, b, c, d, x, y] = item.transform, length = Math.hypot(a!, b!); if (!length) continue;
      const points = [style?.descent ?? -0.2, style?.ascent ?? 0.8].flatMap(vertical => [0, item.width].map(horizontal => [x! + a! / length * horizontal + c! * vertical, y! + b! / length * horizontal + d! * vertical]));
      assert.ok(points.flat().every(Number.isFinite), `${name}: nonfinite glyph ink`);
      records.get(name)!.glyphs.push({ text: item.str, bounds: bounds(points), font: item.fontName, size: length, width: item.width });
    }
    page.cleanup();
  }
  for (const record of records.values()) {
    const boxes = Array.from({ length: record.bounds.length / 4 }, (_, index) => record.bounds.slice(index * 4, index * 4 + 4)), glyphBounds = record.glyphs.map(glyph => glyph.bounds), all = [...boxes, ...glyphBounds];
    record.bounds = all.length ? [Math.min(...all.map(box => box[0]!)), Math.max(...all.map(box => box[1]!)), Math.min(...all.map(box => box[2]!)), Math.max(...all.map(box => box[3]!))] : [];
    const paths = record.paths.flat(), x = paths.length ? Math.min(...paths.map(point => point[0]!)) : record.bounds[0] ?? 0, y = paths.length ? Math.min(...paths.map(point => point[1]!)) : record.bounds[2] ?? 0, round = (value: number) => Math.round(value * 1000) / 1000;
    record.signature = createHash("sha256").update(JSON.stringify({ paths: record.paths.map(path => path.map(point => [round(point[0]! - x), round(point[1]! - y)])), images: record.images.map(image => image.map(point => [round(point[0]! - x), round(point[1]! - y)])), glyphs: record.glyphs.map(glyph => ({ text: glyph.text, bounds: glyph.bounds.map((value, index) => round(value - (index < 2 ? x : y))), size: round(glyph.size), width: round(glyph.width) })) })).digest("hex");
    record.graphicSignature = createHash("sha256").update(JSON.stringify({ operations: record.pathOperations, paths: record.paths.map(path => path.map(point => [round(point[0]! - x), round(point[1]! - y)])), images: record.images.map(image => image.map(point => [round(point[0]! - x), round(point[1]! - y)])) })).digest("hex");
    record.glyphGeometrySignature = createHash("sha256").update(JSON.stringify(record.glyphs.map(glyph => ({ bounds: glyph.bounds.map((value, index) => round(value - (index < 2 ? x : y))), size: round(glyph.size), width: round(glyph.width) })))).digest("hex");
  }
  return [...records.values()];
}

/** 🧷️ Joins independently consumed paint with exact authored leaf markers and source-backed paint classes. */
export function assertPrintKindPaintOwnership(path: string, leaves: readonly { leafId: string; slug: string }[], actual: PrintKindPaint[], contract: { markerPrefix: string; glyphKinds: { slug: string }[] }): void {
  const expected = leaves.map(leaf => contract.markerPrefix + Buffer.from(leaf.leafId).toString("hex"));
  assert.deepEqual(actual.map(record => record.marker).sort(), [...expected].sort(), `${path}: exact marked chart-body census`);
  for (const [index, leaf] of leaves.entries()) { const record = actual.find(entry => entry.marker === expected[index])!, glyph = contract.glyphKinds.some(entry => entry.slug === leaf.slug); assert.ok(record.paths.length > 0 || record.images.length > 0 || glyph && record.glyphs.length > 0, `${leaf.leafId}/${leaf.slug}: no authored ${glyph ? "graphic/glyph" : "graphic"} body paint`); }
}

/** 🧫️ Compiles positive and negative marked-body controls against independent PDF.js and D3. */
export async function verifyPrintKindPaintFixtures(context:PrintPipelineVerificationV1): Promise<void> {
  const outputRoot=outputDirectory(context),{execution,control}=pipelineScope(context);
  const root = join(import.meta.dir, "../../🧫️fixtures/🖌️kind-paint"),contract = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")),{ extent } = await import("d3-array");
  
  for (const entry of contract.glyphKinds) { assert.equal(loadVizCatalog().kinds.find(kind => kind.slug === entry.slug)?.family, entry.family, entry.slug); assert.ok(readFileSync(join(latexRoot, entry.source), "utf8").includes(entry.definition), entry.slug + ": source-backed glyph painter"); }
  for (const [language, appearance] of [["en", "light"], ["de", "dark"]] as const) {
    const body = contract.cases.map((entry: any) => `${entry.chart ? "\\begin{VizFigure}[width=80,height=40]" : "\\begin{tikzpicture}[x=1mm,y=1mm]\\draw (-1,-1) rectangle (25,16);"}\\special{pdf:literal direct /${contract.markerPrefix + Buffer.from(entry.id).toString("hex")} BMC}\\begin{scope}${entry.body}\\end{scope}\\special{pdf:literal direct EMC}${entry.chart ? "\\end{VizFigure}" : "\\end{tikzpicture}"}`).join("\\par\n");
    const generated=`\\documentclass[type=report,theme=${appearance},language=${language}]{semio}\\title{Paint}\\author{Semio}\\date{}\\begin{document}\\chapter{Paint}${body}\\end{document}`;
    await withCompiledPrintProbeV1(execution,{version:1,kind:"probe",name:"paint.tex",source:generated,resources:[{source:"🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🖼️image.png",name:"neutral.png"}]},async probe=>{
    const pdf = await openPrintPdf(context,await probe.read("paint.pdf",67108864),probe.control);
    try {
      const actual = await readPrintKindPaint(pdf);
      for (const entry of contract.cases) {
        const record = actual.find(row => row.marker === contract.markerPrefix + Buffer.from(entry.id).toString("hex")); assert.ok(record, entry.id + ": neutral marker missing");
        const glyphClass=entry.catalogueSlug?contract.glyphKinds.some((kind:any)=>kind.slug===entry.catalogueSlug):entry.class==="glyph";
        assert.equal(record.paths.length > 0 || record.images.length > 0 || glyphClass && record.glyphs.length > 0, entry.valid, `${language}/${entry.id}: independently extracted body paint class`);
        if (entry.expectedData) { const { scaleLinear } = await import("d3-scale"), x = scaleLinear(extent<number>(entry.expectedData.map((row: number[]) => row[0]!)) as [number, number], [8, 72]), y = scaleLinear(extent<number>(entry.expectedData.map((row: number[]) => row[1]!)) as [number, number], [8, 32]), centers = record.glyphs.map(glyph => [(glyph.bounds[0]! + glyph.bounds[1]!) / 2, (glyph.bounds[2]! + glyph.bounds[3]!) / 2]); assert.deepEqual(record.glyphs.map(glyph => glyph.text), entry.expectedData.map((row: number[]) => String(row[1]))); for (const [index, row] of entry.expectedData.entries()) { assert.ok(Math.abs((centers[index]![0]! - centers[0]![0]!) - (x(row[0]) - x(entry.expectedData[0][0])) * 72 / 25.4) < contract.tolerancePt, entry.id + ": D3 bound horizontal glyph displacement"); assert.ok(Math.abs((centers[index]![1]! - centers[0]![1]!) - (y(row[1]) - y(entry.expectedData[0][1])) * 72 / 25.4) < contract.tolerancePt, entry.id + ": D3 bound vertical glyph displacement"); } }
        if (entry.expectedBoundsMm) { const points = record.paths.flat(), expected = [entry.expectedBoundsMm[0], entry.expectedBoundsMm[1], entry.expectedBoundsMm[2], entry.expectedBoundsMm[3]], dx = extent(points.map(point => point[0]!)), dy = extent(points.map(point => point[1]!)); assert.ok(Math.abs((dx[1]! - dx[0]!) - (expected[1]! - expected[0]!) * 72 / 25.4) < contract.tolerancePt && Math.abs((dy[1]! - dy[0]!) - (expected[3]! - expected[2]!) * 72 / 25.4) < contract.tolerancePt, entry.id + ": authored D3 transformed dimensions"); }
      }
    } finally { await pdf.destroy(); }
    });
  }
}
//#endregion 🧪️PrintPipelineTests
