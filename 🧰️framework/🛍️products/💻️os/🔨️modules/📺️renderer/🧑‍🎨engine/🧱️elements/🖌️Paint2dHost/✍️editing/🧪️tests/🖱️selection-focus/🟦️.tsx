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
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false),10_000);
  const disclosure=view.container.querySelector("details")!;fireEvent.click(disclosure.querySelector("summary")!);disclosure.open=true;
  fireEvent.change(view.getByRole("combobox",{name:"Adjustment"}),{target:{value:"invert"}});
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  const command=action==="Apply"?"editPixels":"maskFromSelection";
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]===command)).toBe(true),10_000);
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
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
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

for(const kind of ["pixel","group"]) test(kind+" mask target sends coverage strokes and clears the pixel selection",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind,id:"p",...(kind==="pixel"?{width:3,height:1,imageKey:"source"}:{children:[]}),mask:{enabled:true,linked:true,invert:false,width:3,height:1,imageKey:"mask",transform:{}}}]})} assetsJson="{}" assetExtentsJson='{"source":{"width":3,"height":1},"mask":{"width":3,"height":1}}' selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={0.5} brushColor="#ff0000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  if(kind==="pixel"){
    fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
    await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  }
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  expect(view.getByRole("option",{name:"Choose a layer with a mask"})).toBeDefined();
  fireEvent.click(view.getByRole("button",{name:"Eraser",exact:true}));
  const canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
  });
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]==="editMask")).toBe(true),10_000);
  const payload=dispatch.mock.calls.find(call=>call[0]==="editMask")![1];
  expect(payload.layerId).toBe("p");expect(JSON.parse(payload.expectedMask).imageKey).toBe("mask");expect(payload.selection).toBe(null);
  expect(JSON.parse(payload.operation)).toEqual({kind:"alphaStroke",points:[[1.5,0.5],[1.5,0.5]],size:1,opacity:0.5,hardness:1,alpha:0});
  expect(dispatch.mock.calls.some(call=>call[0]==="editPixels")).toBe(false);
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.change(view.getByRole("spinbutton",{name:"Mask value"}),{target:{value:"64"}});
  fireEvent.click(view.getByRole("button",{name:"Fill mask",exact:true}));
  await waitFor(()=>expect(dispatch.mock.calls.filter(call=>call[0]==="editMask").length).toBe(2),10_000);
  const fill=dispatch.mock.calls.filter(call=>call[0]==="editMask")[1]![1];
  expect(JSON.parse(fill.operation)).toEqual({kind:"alphaFill",alpha:64,opacity:0.5});
  expect(JSON.parse(fill.selection)).toEqual([[0,3,255]]);
  expect(JSON.parse(fill.expectedMask).imageKey).toBe("mask");
});

for(const action of fixture.completionActions)for(const completion of fixture.completionCases)test(action.label+" waits for "+completion.outcome.kind+" completion",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div");
  let settle!:(value:unknown)=>void;
  const pending=new Promise(resolve=>{settle=resolve;});
  const dispatch=vi.fn((_action:string,_args?:Record<string,unknown>)=>pending);
  const view=render(<PixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":3,"height":1,"mask":{"enabled":true,"linked":true,"invert":false,"width":3,"height":1,"transform":{}}}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  view.container.querySelector("details")!.open=true;
  if(action.target==="mask")fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  if(action.selection)fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  const apply=view.getByRole("button",{name:action.label,exact:true}) as HTMLButtonElement;
  act(()=>{fireEvent.click(apply);fireEvent.click(apply);});
  await waitFor(()=>expect(dispatch).toHaveBeenCalledTimes(1));
  expect(dispatch.mock.calls[0]![0]).toBe(action.command);
  await act(async()=>{await Promise.resolve();});
  expect(apply.disabled).toBe(true);
  expect(view.getByText("Editing image; progress and cancellation are available in Tasks.")).toBeDefined();
  fireEvent.click(view.getByRole("button",{name:"Pan canvas",exact:true}));
  expect(apply.disabled).toBe(true);
  expect([...view.container.querySelectorAll("button")].some(button=>button.textContent==="Cancel")).toBe(false);
  await act(async()=>{settle(completion.outcome);await pending;});
  await waitFor(()=>expect(apply.disabled).toBe(false));
  if(completion.error)expect(view.getByRole("alert").textContent).toContain(completion.message);
  else expect(view.getByText(completion.message)).toBeDefined();
  expect(view.container.querySelector('[role="alert"]')!==null).toBe(completion.error);
});

for(const action of fixture.selectionControls.actions)for(const cancellation of fixture.selectionControls.cancelActions)test(action+" can be cancelled with "+cancellation,async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height,firstProgress}=fixture.selectionControls;
  const host=document.createElement("div"),dispatch=vi.fn();
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width,height,transform:{}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  expect(view.container.querySelector("progress")?.value).toBe(firstProgress);
  fireEvent.click(view.getByRole("button",{name:cancellation,exact:true}));
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(view.container.querySelector("progress")).toBeNull();
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(cancellation==="Deselect");
  expect(dispatch).not.toHaveBeenCalled();
  view.container.querySelector("details")!.open=true;
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledTimes(1));
  expect(dispatch.mock.calls[0]![1].selection).toBe(cancellation==="Deselect"?null:JSON.stringify([[0,width*height,255]]));
});

for(const modifier of fixture.selectionControls.keyboardModifiers)test(modifier+"+D cancels a pending keyboard Select All",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height,firstProgress}=fixture.selectionControls;
  const host=document.createElement("div"),dispatch=vi.fn();
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width,height,transform:{}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  const canvas=view.getByRole("application",{name:"Image tools"});
  fireEvent.keyDown(canvas,{key:"a",code:"KeyA",[modifier]:true});
  expect(view.container.querySelector("progress")?.value).toBe(firstProgress);
  fireEvent.keyDown(canvas,{key:"d",code:"KeyD",[modifier]:true});
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(view.container.querySelector("progress")).toBeNull();
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  expect(dispatch).not.toHaveBeenCalled();
});
