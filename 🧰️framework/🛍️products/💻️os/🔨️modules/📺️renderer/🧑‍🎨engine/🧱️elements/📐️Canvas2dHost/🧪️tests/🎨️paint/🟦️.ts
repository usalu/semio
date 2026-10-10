/** 🎨️ Authored stroke geometry scales with the artwork and disabled paint stays invisible. */
import { afterEach, expect, it, vi } from "vitest";
import { Vector2 } from "three";
import { SVGLoader } from "three/examples/jsm/loaders/SVGLoader.js";
import { drawSceneNode } from "../../🎨️paint/🟦️.ts";
import Ajv2020 from "ajv/dist/2020.js";
import {JSDOM} from "jsdom";
import catalogFixture from "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🧫️fixtures/🎨️catalog/🔣️.json";
import catalogSchema from "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🧬️schema/🎨️catalog/🔣️.json";
import fixture from "../../🧫️fixtures/🎨️paint/🔣️.json";
import { paintDrawingScene, type PathSegment, type StrokeStyle } from "../../../../../../../../../🔨️modules/◻️2d/🟦️.ts";

function context() {
  vi.stubGlobal("Path2D", class { constructor(readonly svg?: string) {} });
  return {clearRect: vi.fn(), save: vi.fn(), restore: vi.fn(), transform: vi.fn(), fill: vi.fn(), stroke: vi.fn(), setLineDash: vi.fn(), lineWidth: 0};
}

afterEach(() => vi.unstubAllGlobals());

it("selects each explicit first-party catalog family with valid CSS font syntax",()=>{
  expect(new Ajv2020({strict:true}).compile(catalogSchema)(catalogFixture)).toBe(true);
  const dom=new JSDOM("<span></span>");const style=dom.window.document.querySelector("span")!.style;
  for(const row of catalogFixture.families){
    const ctx={...context(),font:"",fillText:vi.fn()};
    const fontFamily=row.name as NonNullable<Parameters<typeof drawSceneNode>[1]["text"]>["fontFamily"];
    drawSceneNode(ctx as unknown as CanvasRenderingContext2D,{text:{content:row.character,size:catalogFixture.size,fontFamily},fill:{kind:"solid",color:[0,0,0,1]}},new Map());
    style.font=ctx.font;expect(style.fontSize).toBe(`${catalogFixture.size}px`);expect(style.fontFamily.replaceAll('"',"")).toBe(row.name);expect(ctx.fillText).toHaveBeenCalledTimes(1);
    console.log(`[DEBUG] Browser canvas explicit family=${row.name} font=${ctx.font} CSSOM accepted`);
  }
  dom.window.close();
});

for (const example of fixture.zoomCases) it(`scales the authored stroke at zoom ${example.zoom}`, () => {
  const ctx = context();
  const layer = {segments: fixture.segments, stroke: fixture.stroke} as Parameters<typeof drawSceneNode>[1];
  drawSceneNode(ctx as unknown as CanvasRenderingContext2D, layer, new Map());
  const oracle = SVGLoader.pointsToStroke([new Vector2(0, 0), new Vector2(10, 0)], SVGLoader.getStrokeStyle(fixture.stroke.width, "#000", fixture.stroke.join, fixture.stroke.cap))!;
  oracle.computeBoundingBox();
  const width = oracle.boundingBox!.max.y - oracle.boundingBox!.min.y;
  expect(ctx.lineWidth).toBeCloseTo(width, 12);
  expect(ctx.lineWidth * example.zoom).toBeCloseTo(example.screenWidth, 12);
  expect(ctx.setLineDash).toHaveBeenCalledWith(fixture.stroke.dash);
  expect(ctx.stroke).toHaveBeenCalledTimes(1);
  oracle.dispose();
});

for (const example of fixture.unpainted) it(`leaves ${example.id} invisible`, () => {
  const ctx = context();
  const layer = {segments: fixture.segments, ...example} as Parameters<typeof drawSceneNode>[1];
  drawSceneNode(ctx as unknown as CanvasRenderingContext2D, layer, new Map());
  expect(ctx.fill).not.toHaveBeenCalled();
  expect(ctx.stroke).not.toHaveBeenCalled();
  const raster = context();
  paintDrawingScene(raster as unknown as CanvasRenderingContext2D, {width: 20, height: 20, nodes: [{
    transform: [1, 0, 0, 1, 0, 0], node: {kind: "path", segments: fixture.segments as PathSegment[]},
    stroke: example.stroke as StrokeStyle | undefined,
  }]});
  expect(raster.fill).not.toHaveBeenCalled();
  expect(raster.stroke).not.toHaveBeenCalled();
});

import textFixture from "../../🧫️fixtures/📝️text/🔣️.json";

it("paints authored line breaks at separate baselines", () => {
  const ctx = {...context(), fillText: vi.fn()};
  drawSceneNode(ctx as unknown as CanvasRenderingContext2D, {x: textFixture.x, y: textFixture.y, text: {...textFixture,fontFamily:"Anta"}, fill: {kind: "solid", color: [0, 0, 0, 1]}}, new Map());
  expect(ctx.fillText.mock.calls).toEqual(textFixture.lines.flatMap((line, index) => line ? [[line, textFixture.x, textFixture.baselines[index]]] : []));
});

it("respects disabled fill and authored text stroke", () => {
  const ctx = {...context(), fillText: vi.fn(), strokeText: vi.fn()};
  drawSceneNode(ctx as unknown as CanvasRenderingContext2D, {text: {content: "Outline", size: 10,fontFamily:"Anta"}, stroke: {color: [1, 0, 0, 1], width: 2}}, new Map());
  expect(ctx.fillText).not.toHaveBeenCalled();
  expect(ctx.strokeText).toHaveBeenCalledWith("Outline", 0, 10);
  expect(ctx.lineWidth).toBe(2);
});

it("paints text with its authored gradient", () => {
  const gradient = {addColorStop: vi.fn()};
  const ctx = {...context(), fillText: vi.fn(), createLinearGradient: vi.fn(() => gradient), fillStyle: undefined as unknown};
  drawSceneNode(ctx as unknown as CanvasRenderingContext2D, {text: {content: "Gradient", size: 10,fontFamily:"Anta"}, fill: {kind: "linearGradient", x1: 0, y1: 0, x2: 20, y2: 0, stops: [{offset: 0, color: [1, 0, 0, 1]}, {offset: 1, color: [0, 0, 1, 1]}]}}, new Map());
  expect(ctx.fillStyle).toBe(gradient);
  expect(gradient.addColorStop).toHaveBeenCalledTimes(2);
});

it("raster text uses the same explicit lines with baseline coordinates", () => {
  const ctx = {...context(), fillText: vi.fn(), strokeText: vi.fn()};
  paintDrawingScene(ctx as unknown as CanvasRenderingContext2D, {width: 100, height: 100, nodes: [{
    transform: [1, 0, 0, 1, 0, 0], node: {kind: "text", x: textFixture.x, y: textFixture.y + textFixture.size, content: textFixture.content, size: textFixture.size},
    fill: {kind: "solid", color: [0, 0, 0, 1]}, stroke: {color: [1, 0, 0, 1], width: 2, cap: "butt", join: "miter"},
  }]});
  const calls = textFixture.lines.flatMap((line, index) => line ? [[line, textFixture.x, textFixture.baselines[index]]] : []);
  expect(ctx.fillText.mock.calls).toEqual(calls);
  expect(ctx.strokeText.mock.calls).toEqual(calls);
});
