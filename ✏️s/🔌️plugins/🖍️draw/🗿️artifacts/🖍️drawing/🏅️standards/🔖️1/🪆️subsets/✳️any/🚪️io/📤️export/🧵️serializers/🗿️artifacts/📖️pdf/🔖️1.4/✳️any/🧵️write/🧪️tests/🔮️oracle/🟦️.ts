/** 🔮️ Independent PDF.js, PDFium and Canvas stay behind the owning PDF pixel-test interface. */
import {expect} from "bun:test";
import {createCanvas,DOMMatrix,ImageData,Path2D,type Canvas,type SKRSContext2D} from "@napi-rs/canvas";
type Surface={canvas:Canvas|null;context:SKRSContext2D|null};
class OracleCanvasFactory{create(width:number,height:number):Surface{const canvas=createCanvas(width,height);return{canvas,context:canvas.getContext("2d")};}reset(surface:Surface,width:number,height:number):void{surface.canvas!.width=width;surface.canvas!.height=height;}destroy(surface:Surface):void{surface.canvas!.width=surface.canvas!.height=0;surface.canvas=null;surface.context=null;}}
export async function pdfNodePixels(bytes:Uint8Array):Promise<Uint8ClampedArray>{const old={DOMMatrix:globalThis.DOMMatrix,ImageData:globalThis.ImageData,Path2D:globalThis.Path2D};Object.assign(globalThis,{DOMMatrix,ImageData,Path2D});try{const{getDocument}=await import("pdfjs-dist/legacy/build/pdf.mjs"),task=getDocument({data:Uint8Array.from(bytes),useSystemFonts:false,CanvasFactory:OracleCanvasFactory});try{const document=await task.promise;expect(document.numPages).toBe(1);const page=await document.getPage(1),viewport=page.getViewport({scale:1}),canvas=createCanvas(viewport.width,viewport.height),context=canvas.getContext("2d");expect([viewport.width,viewport.height]).toEqual([2,1]);await page.render({canvasContext:context,canvas,viewport,background:"rgb(255,255,255)"} as never).promise;return context.getImageData(0,0,2,1).data;}finally{await task.destroy();}}finally{Object.assign(globalThis,old);}}
/** 🖨️ Samples each Draw PDF through the repo-provisioned independent PDFium renderer. */
export async function pdfPixels(documents: readonly Uint8Array[]): Promise<Uint8Array[]> {
 const script = await Bun.file(new URL("./🐍️.py", import.meta.url)).text();
 const stdin = Buffer.concat(documents.flatMap(document => {
  const header = Buffer.alloc(4);
  header.writeUInt32BE(document.byteLength);
  return [header, document];
 }));
 const command = ["repo", "tools", "exec", "--light", "uv", "--", "run", "--quiet", "--isolated", "--no-project", "--no-build", "--with", "pypdfium2==5.15.2", "python", "-I", "-c", script];
 const child = Bun.spawn(command, {stdin, stdout: "pipe", stderr: "pipe"});
 const [pixels, diagnostics, exit] = await Promise.all([new Response(child.stdout).arrayBuffer(), new Response(child.stderr).text(), child.exited]);
 if (exit !== 0) throw Error(`PDFium PDF oracle failed ${exit}: ${diagnostics}`);
 expect(pixels.byteLength).toBe(documents.length * 8);
 return documents.map((_, index) => new Uint8Array(pixels, index * 8, 8));
}
