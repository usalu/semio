/** 🧪️ Moving focus to the tools must preserve an asynchronously computed pixel selection. */
import {act,cleanup,fireEvent,render,waitFor} from "@semio-tech/ui-react/test";
import {afterEach,expect,test,vi} from "vitest";
import sharp from "sharp";
import maskFixture from "../../../../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎭️mask-from-selection/🧫️fixtures/🔣️.json";
import {PixelEditingOverlay} from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/🖱️selection-focus/🔣️.json";

vi.mock("@semio-tech/ui-react",async original=>({...await original<typeof import("@semio-tech/ui-react")>(),useTranslation:()=>({i18n:{resolvedLanguage:"en"}})}));
vi.mock("../../../../📐️Canvas2dHost/🟦️.tsx",()=>({canvasPinchCamera:()=>({x:0,y:0,zoom:1})}));
afterEach(()=>{cleanup();vi.restoreAllMocks();vi.unstubAllGlobals();});

for(const action of fixture.actions) test("selection survives focus moving to "+action,async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div");
  host.getBoundingClientRect=()=>({left:0,top:0,width:fixture.viewportWidth,height:fixture.viewportHeight} as DOMRect);
  const dispatch=vi.fn();
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:fixture.width,height:fixture.height,transform:{}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Rectangle selection"}));
  const canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:fixture.start[0],clientY:fixture.start[1]}));
    canvas.dispatchEvent(new PointerEvent("pointermove",{bubbles:true,pointerId:1,button:0,clientX:fixture.end[0],clientY:fixture.end[1]}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:fixture.end[0],clientY:fixture.end[1]}));
    fireEvent.blur(canvas);
  });
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false));
  const disclosure=view.container.querySelector("details")!;fireEvent.click(disclosure.querySelector("summary")!);disclosure.open=true;
  fireEvent.change(view.getByRole("combobox",{name:"Adjustment"}),{target:{value:"invert"}});
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  const command=action==="Apply"?"editPixels":"maskFromSelection";
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]===command)).toBe(true));
  const call=dispatch.mock.calls.find(call=>call[0]===command);
  expect(call).toBeDefined();
  const spans=JSON.parse(call![1].selection) as number[][];
  const coverage=new Uint8Array(fixture.width*fixture.height);
  for(const [offset,length,value] of spans)coverage.fill(value!,offset!,offset!+length!);
  const b=fixture.pixelBounds;
  const {data}=await sharp(Buffer.from(`<svg width="${fixture.width}" height="${fixture.height}"><rect x="${b.x}" y="${b.y}" width="${b.width}" height="${b.height}" fill="white"/></svg>`)).ensureAlpha().raw().toBuffer({resolveWithObject:true});
  expect([...coverage]).toEqual(Array.from({length:coverage.length},(_,i)=>data[i*4+3]));
});

for(const action of ["Apply","Crop to selection","Mask from selection"]) test(action+" cancels selection preparation before dispatch",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:256,height:256} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:256,height:256,transform:{}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  const disclosure=view.container.querySelector("details")!;disclosure.open=true;
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  expect(view.container.querySelector("progress")?.value).toBe(0.5);
  fireEvent.click(view.getByRole("button",{name:"Cancel",exact:true}));
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(dispatch.mock.calls.filter(call=>["editPixels","maskFromSelection"].includes(call[0]))).toEqual([]);
  expect(view.container.querySelector("progress")).toBeNull();
  expect((view.getByRole("button",{name:action,exact:true}) as HTMLButtonElement).disabled).toBe(false);
});

test("selection mask coverage matches the Sharp alpha-channel oracle",async()=>{
  const {width,height,selection,expectedRgba}=maskFixture;
  const alpha=Buffer.alloc(width*height);
  for(const [start,length,coverage] of selection) alpha.fill(coverage!,start!,start!+length!);
  const rgba=await sharp(Buffer.alloc(width*height*3,255),{raw:{width,height,channels:3}}).joinChannel(alpha,{raw:{width,height,channels:1}}).raw().toBuffer();
  expect([...rgba]).toEqual(expectedRgba);
});
