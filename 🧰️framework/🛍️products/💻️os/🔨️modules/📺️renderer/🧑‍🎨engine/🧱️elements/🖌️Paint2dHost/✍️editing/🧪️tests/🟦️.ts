/** 🧪️ Pixel tools respect nested layer transforms and lossless selection transport. */
import { expect, test } from "bun:test";
import sharp from "sharp";
import { pixelLayers, layerPoint, selectionSpans, selectionBounds } from "../🟦️.ts";
import fixtures from "../🧫️fixtures/🔣️.json";

for(const fixture of fixtures.cases) test(fixture.name,()=>{
  const [layer]=pixelLayers(JSON.stringify(fixture.document),JSON.stringify(fixture.assets));
  const point=layerPoint(layer!,fixture.worldPoint[0]!,fixture.worldPoint[1]!);
  for(let i=0;i<2;i++)expect(point[i]!).toBeCloseTo(fixture.pixelPoint[i]!,10);
});

test("nested transforms map painting into the selected layer", () => {
  const layers=pixelLayers(JSON.stringify({layers:[{kind:"group",id:"g",visible:true,transform:{x:10,y:20,scaleX:2,scaleY:2},children:[{kind:"pixel",id:"p",name:"Pixels",width:20,height:10,transform:{x:3,y:4}}]}]}));
  expect(layers).toHaveLength(1);
  expect(layerPoint(layers[0]!,18,30)).toEqual([11,6]);
});

test("the compositor centers pixel layers on their transforms",()=>{
  const [layer]=pixelLayers(JSON.stringify({layers:[{kind:"pixel",id:"p",width:512,height:256,transform:{x:0,y:0}}]}));
  expect(layerPoint(layer!,0,0)).toEqual([256,128]);
  expect(layerPoint(layer!,-256,-128)).toEqual([0,0]);
});

test("hidden ancestors prevent painting", () => {
  const layers=pixelLayers(JSON.stringify({layers:[{kind:"group",visible:false,children:[{kind:"pixel",id:"p"}]}]}));
  expect(layers[0]!.visible).toBe(false);
});

test("selection spans retain empty versus unrestricted selection", () => {
  expect(selectionSpans(undefined)).toBe(null);
  expect(selectionSpans(new Uint8Array(4))).toBe("[]");
  expect(JSON.parse(selectionSpans(Uint8Array.of(0,255,255,0,128))!)).toEqual([[1,2,255],[4,1,128]]);
});

test("selection bounds use actual mask coverage", () => {
  expect(selectionBounds(Uint8Array.of(0,255,0,0,255,0),3)).toEqual({x:1,y:0,width:1,height:2});
  expect(selectionBounds(new Uint8Array(6),3)).toBe(null);
});

test("singular transforms reject interaction instead of inventing a location", () => {
  const layers=pixelLayers(JSON.stringify({layers:[{kind:"pixel",id:"p",transform:{scaleX:0}}]}));
  expect(()=>layerPoint(layers[0]!,0,0)).toThrow();
});

for(const fixture of fixtures.cases) test(`${fixture.name}: SVG oracle maps the fixture pixel to its world coordinate`,async()=>{
  const layer=fixture.document.layers[0]!;
  const transform=layer.transform as {x:number;y:number;rotation?:number};
  const asset="imageKey" in layer?fixture.assets[layer.imageKey as keyof typeof fixture.assets]:undefined;
  const sx=asset?layer.width/asset.width:1,sy=asset?layer.height/asset.height:1;
  const {x,y,zoom}=fixture.camera;
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><g transform="translate(${50-x*zoom} ${50-y*zoom}) scale(${zoom}) translate(${transform.x} ${transform.y}) rotate(${transform.rotation??0}) translate(${-layer.width/2} ${-layer.height/2}) scale(${sx} ${sy})"><rect x="${fixture.pixelPoint[0]!-0.5}" y="${fixture.pixelPoint[1]!-0.5}" width="1" height="1" fill="red"/></g></svg>`;
  const {data}=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer({resolveWithObject:true});
  const px=Math.floor(50+(fixture.worldPoint[0]!-x)*zoom),py=Math.floor(50+(fixture.worldPoint[1]!-y)*zoom),at=(py*100+px)*4;
  expect(data[at]).toBe(255);expect(data[at+3]).toBeGreaterThan(200);
});
