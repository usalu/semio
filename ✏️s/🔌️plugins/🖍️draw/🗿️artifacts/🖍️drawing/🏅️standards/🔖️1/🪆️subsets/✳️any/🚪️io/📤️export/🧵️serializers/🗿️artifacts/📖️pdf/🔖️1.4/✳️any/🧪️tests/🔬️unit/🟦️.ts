/** 📖️ Independently render native PDF exports of the language-neutral compositing corpus. */
import { expect, test } from "bun:test";
import { join } from "node:path";
import { createCanvas, DOMMatrix, ImageData, Path2D } from "@napi-rs/canvas";
import sharp from "sharp";
import cases from "../../../../../../../../../🧬️schema/🎬️scene/🧩️compositing/🧫️fixtures/🔣️.json";

type Surface = {canvas:ReturnType<typeof createCanvas> | null; context:ReturnType<ReturnType<typeof createCanvas>["getContext"]> | null};
class OracleCanvasFactory {
  create(width:number,height:number):Surface { const canvas=createCanvas(width,height); return {canvas,context:canvas.getContext("2d")}; }
  reset(surface:Surface,width:number,height:number):void { surface.canvas!.width=width; surface.canvas!.height=height; }
  destroy(surface:Surface):void { surface.canvas!.width=surface.canvas!.height=0; surface.canvas=null; surface.context=null; }
}

const directory = process.env.SEMIO_DRAW_PDF_ORACLE_DIRECTORY;
test.skipIf(!directory)("native PDF transparency matches independent SVG rasterization", async () => {
  const previous = { DOMMatrix:globalThis.DOMMatrix, ImageData:globalThis.ImageData, Path2D:globalThis.Path2D };
  Object.assign(globalThis,{DOMMatrix,ImageData,Path2D});
  try {
    const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs");
    for (const fixture of cases) {
      const bytes = new Uint8Array(await Bun.file(join(directory!,`${fixture.name}.pdf`)).arrayBuffer());
      const task = getDocument({ data:bytes, useSystemFonts:false, isEvalSupported:false, CanvasFactory:OracleCanvasFactory });
      try {
        const document = await task.promise;
        expect(document.numPages).toBe(1);
        const page = await document.getPage(1);
        const viewport = page.getViewport({scale:1});
        expect([viewport.width,viewport.height]).toEqual([24,16]);
        const canvas = createCanvas(24,16);
        const context = canvas.getContext("2d");
        await page.render({canvasContext:context,canvas,viewport,background:"rgb(255,255,255)"} as never).promise;
        const actual = context.getImageData(0,0,24,16).data;
        const reference = await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="24" height="16">${fixture.svg}</svg>`)).flatten({background:"white"}).ensureAlpha().raw().toBuffer();
        let maximum = 0;
        for (let index=0;index<actual.length;index++) maximum=Math.max(maximum,Math.abs(actual[index]!-reference[index]!));
        expect(maximum,fixture.name).toBeLessThanOrEqual(2);
        console.error(`[DEBUG] ${fixture.name}: native PDF.js and Sharp SVG agree within ${maximum}/255 over ${24*16} pixels`);
      } finally { await task.destroy(); }
    }
  } finally { Object.assign(globalThis,previous); }
});
