import{test,expect}from"vitest";
import Ajv from"ajv";
import{DatabaseSync}from"node:sqlite";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{createRequire}from"node:module";
import{renderPrintSvgResourceV1}from"../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🟦️.ts";
import plan from"../icons.json"with{type:"json"};
import schema from"../icons.schema.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
const root=fileURLToPath(new URL("../../../../../",import.meta.url)),require=createRequire(resolve(root,"package.json")),sharp=createRequire(resolve(root,"🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🟦️.ts"))("sharp"),parser=createRequire(require.resolve("vite/package.json"))("parse5");
test("all sixteen authored icon identities retain their full viewbox under the closed print-quality plan",()=>{
 expect(new Ajv({strict:true}).validate(schema,plan)).toBe(true);expect(plan.resources.map(({source,output,sha256})=>({source,output,sha256}))).toEqual(corpus.identities);expect(readFileSync(new URL("../geometry.tex",import.meta.url),"utf8")).toContain("width=22mm,height=11mm,keepaspectratio");expect(plan.rasterization).toEqual(corpus.quality);
});
test("every real authored SVG agrees with independent DOM and Sharp pixels above the declared physical print density",async()=>{
 const db=new DatabaseSync(":memory:");let pngBytes=0;try{for(const row of plan.resources){const bytes=readFileSync(resolve(root,row.source));expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.sha256);const document=parser.parseFragment(bytes.toString("utf8"));const find=(node:any):any=>node.tagName==="svg"?node:(node.childNodes??[]).map(find).find(Boolean),svg=find(document);if(!svg)throw Error("Actual authored SVG root required");const box=svg.attrs.find((a:any)=>a.name.toLowerCase()==="viewbox").value.trim().split(/[ ,]+/u).map(Number),width=row.width,height=Math.ceil(width*box[3]/box[2]);expect(box).toHaveLength(4);const scale=Math.min(corpus.quality.maximumWidthMillimeters/box[2],corpus.quality.maximumHeightMillimeters/box[3]),physicalWidth=box[2]*scale,physicalHeight=box[3]*scale,dpi=Math.min(width/physicalWidth,height/physicalHeight)*25.4;expect(dpi,row.output).toBeGreaterThanOrEqual(corpus.quality.minimumDotsPerInch);const required=physicalWidth*corpus.quality.minimumDotsPerInch/25.4,aligned=Math.ceil(required/corpus.quality.pixelAlignment)*corpus.quality.pixelAlignment;expect(width,row.output).toBe(aligned);expect(width-corpus.quality.pixelAlignment).toBeLessThan(required);const chosen=db.prepare("select cast(ceil(?*?/25.4/?) as integer)*? width").get(physicalWidth,corpus.quality.minimumDotsPerInch,corpus.quality.pixelAlignment,corpus.quality.pixelAlignment) as {width:number};expect(width).toBe(chosen.width);const sql=db.prepare("select min(1.0*?/?,1.0*?/?) *25.4 dpi").get(width,physicalWidth,height,physicalHeight) as {dpi:number};expect(Math.abs(sql.dpi-dpi)).toBeLessThan(0.00001);
 for(const[name,value]of[["width",width+"px"],["height",height+"px"]]as const){const a=svg.attrs.find((a:any)=>a.name===name);if(a)a.value=value;else svg.attrs.push({name,value});}
 const oracleSource=Buffer.from(parser.serializeOuter(svg)),actual=await renderPrintSvgResourceV1(bytes,width,new AbortController().signal),image=await sharp(actual).ensureAlpha().raw().toBuffer({resolveWithObject:true}),oracle=await sharp(oracleSource,{density:72}).ensureAlpha().raw().toBuffer({resolveWithObject:true});pngBytes+=actual.byteLength;expect([image.info.width,image.info.height]).toEqual([width,height]);expect([oracle.info.width,oracle.info.height]).toEqual([width,height]);expect(image.data.equals(oracle.data),row.output).toBe(true);expect(image.data.some((v:number,i:number)=>i%4===3&&v>0),row.output).toBe(true);
 }expect(pngBytes).toBeGreaterThan(0);}finally{db.close();}
});
