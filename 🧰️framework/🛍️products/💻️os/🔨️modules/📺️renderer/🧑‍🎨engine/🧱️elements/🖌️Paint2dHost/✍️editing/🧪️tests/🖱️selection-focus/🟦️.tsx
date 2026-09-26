/** 🧪️ Moving focus to the tools must preserve an asynchronously computed pixel selection. */
import {act,cleanup,fireEvent,render,waitFor} from "@semio-tech/ui-react/test";
import {afterEach,expect,test,vi} from "vitest";
import sharp from "sharp";
import {PixelEditingOverlay} from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/🖱️selection-focus/🔣️.json";

vi.mock("@semio-tech/ui-react",async original=>({...await original<typeof import("@semio-tech/ui-react")>(),useTranslation:()=>({i18n:{resolvedLanguage:"en"}})}));
vi.mock("../../../../📐️Canvas2dHost/🟦️.tsx",()=>({canvasPinchCamera:()=>({x:0,y:0,zoom:1})}));
afterEach(()=>{cleanup();vi.restoreAllMocks();vi.unstubAllGlobals();});

test("selection survives focus moving to the adjustment controls",async()=>{
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
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  const call=dispatch.mock.calls.find(call=>call[0]==="editPixels");
  expect(call).toBeDefined();
  const spans=JSON.parse(call![1].selection) as number[][];
  const coverage=new Uint8Array(fixture.width*fixture.height);
  for(const [offset,length,value] of spans)coverage.fill(value!,offset!,offset!+length!);
  const b=fixture.pixelBounds;
  const {data}=await sharp(Buffer.from(`<svg width="${fixture.width}" height="${fixture.height}"><rect x="${b.x}" y="${b.y}" width="${b.width}" height="${b.height}" fill="white"/></svg>`)).ensureAlpha().raw().toBuffer({resolveWithObject:true});
  expect([...coverage]).toEqual(Array.from({length:coverage.length},(_,i)=>data[i*4+3]));
});
