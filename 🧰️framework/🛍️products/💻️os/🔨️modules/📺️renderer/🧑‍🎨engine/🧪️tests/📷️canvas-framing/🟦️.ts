/** 📷️ Shared framing fixtures checked against Three.js orthographic projection. */
import { expect, it } from "vitest";
import { OrthographicCamera, Vector3 } from "three";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { fitCanvasFrame } from "@semio-tech/ui/scene/framing";
const root = resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../../../🔨️modules/🖱️ui/🎬️scene/📷️framing");
const fixture = JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
const validate = new Ajv().compile(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));
for (const test of fixture.cases) it(`frames ${test.name}`, () => {
  expect(validate(test.request)).toBe(test.name !== "negative-padding");
  const result = fitCanvasFrame(test.request,test.viewport[0],test.viewport[1]);
  if (test.expected === null) { expect(result).toBeNull(); return; }
  expect(result).not.toBeNull();
  [result!.x,result!.y,result!.zoom].forEach((value,index) => expect(value).toBeCloseTo(test.expected[index],10));
  const [width,height] = test.viewport;
  const camera = new OrthographicCamera(-width/2,width/2,height/2,-height/2,.1,10);
  camera.position.set(result!.x,result!.y,1);
  camera.zoom = result!.zoom;
  camera.updateProjectionMatrix(); camera.updateMatrixWorld();
  const [x0,y0,x1,y1] = test.request.bounds;
  const center = new Vector3((x0+x1)/2,(y0+y1)/2,0).project(camera);
  expect(center.x).toBeCloseTo(0,10); expect(center.y).toBeCloseTo(0,10);
  if (result!.zoom > .05) for (const x of [x0,x1]) for (const y of [y0,y1]) {
    const point = new Vector3(x,y,0).project(camera);
    expect(Math.abs(point.x)).toBeLessThanOrEqual(1-2*test.request.padding/width+1e-10);
    expect(Math.abs(point.y)).toBeLessThanOrEqual(1-2*test.request.padding/height+1e-10);
  }
});
it("rejects nonfinite requests and unmeasured viewports", () => {
  for (const value of [NaN,Infinity,-Infinity]) {
    expect(fitCanvasFrame({ revision: 1,bounds: [0,0,100,value],padding: 40 },800,600)).toBeNull();
    expect(fitCanvasFrame({ revision: 1,bounds: [0,0,100,100],padding: 40 },value,600)).toBeNull();
  }
});

it("fits once after measurement and preserves subsequent wheel navigation until a new revision", async () => {
  const { JsonLayersCanvasSession } = await import("../../🧱️elements/📐️Canvas2dHost/🗂️layers-session/🟦️.ts");
  let request = { revision: 1,bounds: [-100,-50,300,150] as const,padding: 40 };
  const cameras: any[] = [];
  const session = new JsonLayersCanvasSession(() => "[]",{ x: 0,y: 0,zoom: 1 },camera => cameras.push(camera),undefined,() => request);
  session.setSize(0,0,1);
  expect(cameras).toHaveLength(0);
  session.setSize(800,600,1);
  expect(cameras).toEqual([{ x: 100,y: 50,zoom: 1.8 }]);
  session.wheel(400,300,1);
  const navigated = cameras.at(-1);
  session.setSize(900,700,1);
  expect(cameras).toHaveLength(2);
  expect(cameras.at(-1)).toEqual(navigated);
  request = { ...request,revision: 2 };
  session.setSize(900,700,1);
  expect(cameras).toHaveLength(3);
  expect(cameras.at(-1)).toEqual({ x: 100,y: 50,zoom: 2.05 });
});

it("follows the canvas size with the framing until the camera moves off it", async () => {
  const { JsonLayersCanvasSession } = await import("../../🧱️elements/📐️Canvas2dHost/🗂️layers-session/🟦️.ts");
  let request: { revision: number; bounds: readonly [number, number, number, number]; padding: number } | null = { revision: 1, bounds: [-100, -50, 300, 150], padding: 40 };
  const cameras: any[] = [];
  const session = new JsonLayersCanvasSession(() => "[]", { x: 0, y: 0, zoom: 1 }, camera => cameras.push(camera), undefined, () => request);
  session.setSize(300, 200, 1);
  expect(cameras).toEqual([{ x: 100, y: 50, zoom: 0.55 }]);
  request = null;
  session.setSize(800, 600, 1);
  expect(cameras.at(-1)).toEqual({ x: 100, y: 50, zoom: 1.8 });
  session.setSize(800, 600, 1);
  expect(cameras).toHaveLength(2);
  session.updateCamera({ x: 0, y: 0, zoom: 1 });
  session.setSize(900, 700, 1);
  expect(cameras).toHaveLength(2);
});

it("acknowledges a new frame revision without publishing an unchanged camera", async () => {
  const { JsonLayersCanvasSession } = await import("../../🧱️elements/📐️Canvas2dHost/🗂️layers-session/🟦️.ts");
  const { request, viewport, expected } = fixture.cases.find((item: { name: string }) => item.name === "landscape");
  let revision = request.revision;
  const fitted = { x: expected[0], y: expected[1], zoom: expected[2] };
  const cameras: unknown[] = [];
  const session = new JsonLayersCanvasSession(() => "[]", fitted, camera => cameras.push(camera), undefined,
    () => ({ ...request, revision }));
  session.setSize(viewport[0], viewport[1], 1);
  revision++;
  session.setSize(viewport[0], viewport[1], 1);
  expect(cameras).toEqual([fitted]);
  session.wheel(viewport[0] / 2, viewport[1] / 2, 1);
  session.setSize(viewport[0], viewport[1], 1);
  expect(cameras).toHaveLength(2);
});
