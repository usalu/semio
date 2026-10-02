import {SelectionCombineJob} from "../../../../../../../../../../🔨️modules/🔲️pixels/✍️editing/🟦️.ts";
/** 🧪️ Moving focus to the tools must preserve an asynchronously computed pixel selection. */
import {act,cleanup,fireEvent,render,waitFor} from "@semio-tech/ui-react/test";
import {afterEach,expect,test,vi} from "vitest";
import sharp from "sharp";
import {PixelEditingOverlay as ControlledPixelEditingOverlay} from "../../🟦️.tsx";
import {useState,type ComponentProps} from "react";

/** 🎛️ Applies semantic config commands as the editor does before republishing its scene. */
function PixelEditingOverlay(props:Omit<ComponentProps<typeof ControlledPixelEditingOverlay>,"paintTarget"|"maskValue">){
  const [paintTarget,setTarget]=useState<"pixels"|"mask">("pixels"),[maskValue,setValue]=useState(255),[pixelSelectionJson,setSelection]=useState(props.pixelSelectionJson);
  return <ControlledPixelEditingOverlay {...props} pixelSelectionJson={pixelSelectionJson} paintTarget={paintTarget} maskValue={maskValue} dispatch={(action,args)=>{
    if(action==="setPaintTarget"){if(args!.value!==paintTarget)setSelection(undefined);setTarget(args!.value as "pixels"|"mask");}
    if(action==="setMaskValue")setValue(args!.value as number);
    const result=props.dispatch(action,args);
    const apply=(outcome:unknown)=>{if(action==="setPixelSelection"&&!(outcome&&typeof outcome==="object"&&"kind" in outcome&&["refused","superseded"].includes(String(outcome.kind))))setSelection(args?.selection?JSON.stringify(args.selection):undefined);return outcome;};
    if(result instanceof Promise)return result.then(apply);
    apply(result);
  }}/>;
}
const editCalls=(dispatch:{mock:{calls:any[][]}})=>dispatch.mock.calls.filter(call=>["editPixels","editMask","maskFromSelection","paintStroke"].includes(call[0]));
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
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:fixture.width,height:fixture.height,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
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
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:256,height:256,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
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


for(const kind of ["pixel","group"]) test(kind+" mask target sends coverage strokes and clears the pixel selection",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind,id:"p",...(kind==="pixel"?{width:3,height:1,imageKey:"source"}:{children:[]}),mask:{enabled:true,linked:true,invert:false,width:3,height:1,imageKey:"mask",transform:{x:0,y:0,a:1,b:0,c:0,d:1}}}]})} assetsJson="{}" assetExtentsJson='{"source":{"width":3,"height":1},"mask":{"width":3,"height":1}}' selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={0.5} brushColor="#ff0000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  if(kind==="pixel"){
    fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
    await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  }
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  const prompt=(view.getByRole("combobox",{name:"Layer",exact:true}) as HTMLSelectElement).options[0]!;expect(prompt.text).toBe("Choose a layer with a mask");expect(prompt.disabled).toBe(true);
  fireEvent.click(view.getByRole("button",{name:"Eraser",exact:true}));
  const canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
  });
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]==="paintStroke")).toBe(true),10_000);
  const payload=dispatch.mock.calls.find(call=>call[0]==="paintStroke")![1];
  expect(payload).toEqual({layerId:"p",tool:"eraser",xs:[1.5,1.5],ys:[0.5,0.5]});
  expect(dispatch.mock.calls.some(call=>call[0]==="editPixels"||call[0]==="editMask")).toBe(false);
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.change(view.getByRole("spinbutton",{name:"Mask value"}),{target:{value:"64"}});
  fireEvent.click(view.getByRole("button",{name:"Fill mask",exact:true}));
  await waitFor(()=>expect(dispatch.mock.calls.filter(call=>call[0]==="editMask").length).toBe(1),10_000);
  const fill=dispatch.mock.calls.filter(call=>call[0]==="editMask")[0]![1];
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
  const dispatch=vi.fn((action:string,_args?:Record<string,unknown>)=>action==="setPixelSelection"?Promise.resolve({kind:"applied"}):pending);
  const view=render(<PixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":3,"height":1,"mask":{"enabled":true,"linked":true,"invert":false,"width":3,"height":1,"transform":{"x":0,"y":0,"a":1,"b":0,"c":0,"d":1}}}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  view.container.querySelector("details")!.open=true;
  if(action.target==="mask"){fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();}
  if(action.selection)fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  const apply=view.getByRole("button",{name:action.label,exact:true}) as HTMLButtonElement;
  act(()=>{fireEvent.click(apply);fireEvent.click(apply);});
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  expect(editCalls(dispatch)[0]![0]).toBe(action.command);
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
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width,height,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  expect(view.container.querySelector("progress")?.value).toBe(firstProgress);
  fireEvent.click(view.getByRole("button",{name:cancellation,exact:true}));
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(view.container.querySelector("progress")).toBeNull();
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(cancellation==="Deselect");
  expect(editCalls(dispatch)).toEqual([]);
  view.container.querySelector("details")!.open=true;
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  expect(editCalls(dispatch)[0]![1].selection).toBe(cancellation==="Deselect"?null:JSON.stringify([[0,width*height,255]]));
});

for(const modifier of fixture.selectionControls.keyboardModifiers)test(modifier+"+D cancels a pending keyboard Select All",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height,firstProgress}=fixture.selectionControls;
  const host=document.createElement("div"),dispatch=vi.fn();
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width,height,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  const canvas=view.getByRole("application",{name:"Image tools"});
  fireEvent.keyDown(canvas,{key:"a",code:"KeyA",[modifier]:true});
  expect(view.container.querySelector("progress")?.value).toBe(firstProgress);
  fireEvent.keyDown(canvas,{key:"d",code:"KeyD",[modifier]:true});
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(view.container.querySelector("progress")).toBeNull();
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  expect(editCalls(dispatch)).toEqual([]);
});

for(const cancellation of fixture.combinationCancellation.actions)test("selection combination cancels with "+cancellation+" after shape preparation",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height}=fixture.combinationCancellation,host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width,height} as DOMRect);
  const editor=(activeUtility:string)=><PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width,height,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility={activeUtility} brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>;
  const view=render(editor("select"));
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:"Rectangle selection"}));
  fireEvent.change(view.getByRole("combobox",{name:"Selection",exact:true}),{target:{value:"subtract"}});
  const advance=SelectionCombineJob.prototype.advance;let cancelled=false,observedProgress:number|undefined;
  const spy=vi.spyOn(SelectionCombineJob.prototype,"advance").mockImplementation(function(this:SelectionCombineJob,budget?:number){
    const progress=advance.call(this,budget);
    if(!progress.done)setTimeout(()=>{observedProgress=view.container.querySelector("progress")?.value;act(()=>{if(cancellation==="Switch utility")view.rerender(editor("paintBrush"));else fireEvent.click(view.getByRole("button",{name:cancellation,exact:true}));});cancelled=true;},0);
    return progress;
  });
  const canvas=view.getByRole("application",{name:"Image tools"});canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:0,clientY:0}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:width,clientY:height}));
  });
  await waitFor(()=>expect(cancelled).toBe(true),10_000);
  await waitFor(()=>expect(view.container.querySelector("progress")).toBeNull());
  expect(spy).toHaveBeenCalledTimes(1);expect(observedProgress).toBe(0.75);expect(editCalls(dispatch)).toEqual([]);
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(cancellation==="Deselect");
  view.container.querySelector("details")!.open=true;fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  expect(editCalls(dispatch)[0]![1].selection).toBe(cancellation==="Deselect"?null:JSON.stringify([[0,width*height,255]]));
});

for(const inherited of [false,true])test((inherited?"Inherited":"Own")+" layer protection preserves selection and refuses pixel and mask edits",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const layer={kind:"pixel",id:"p",width:3,height:1,locked:!inherited,mask:{enabled:true,linked:true,width:3,height:1,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}};
  const layers=inherited?[{kind:"group",id:"g",locked:true,children:[layer]}]:[layer];
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  expect(view.getByText("Unlock this layer or its parent in Inspection to edit pixels or masks.")).toBeDefined();
  const canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    fireEvent.keyDown(canvas,{key:"Delete"});
  });
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false));
  view.container.querySelector("details")!.open=true;
  for(const name of ["Apply","Crop to selection","Mask from selection","Resize layer","Clear selected pixels"])expect((view.getByRole("button",{name,exact:true}) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();
  expect((view.getByRole("button",{name:"Fill mask"}) as HTMLButtonElement).disabled).toBe(true);
  expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false);
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(editCalls(dispatch)).toEqual([]);
});

for(const inherited of [false,true])test((inherited?"Parent":"Layer")+" locking cancels pixel edit preparation before dispatch",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height}=fixture.selectionControls,host=document.createElement("div"),dispatch=vi.fn();
  const editor=(locked:boolean)=>{
    const layer={kind:"pixel",id:"p",width,height,locked:!inherited&&locked};
    return <PixelEditingOverlay documentJson={JSON.stringify({layers:inherited?[{kind:"group",id:"g",locked,children:[layer]}]:[layer]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>;
  };
  const view=render(editor(false));
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  view.container.querySelector("details")!.open=true;
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  expect(view.container.querySelector("progress")).not.toBeNull();
  view.rerender(editor(true));
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(view.container.querySelector("progress")).toBeNull();expect(editCalls(dispatch)).toEqual([]);
  expect((view.getByRole("button",{name:"Apply",exact:true}) as HTMLButtonElement).disabled).toBe(true);
  view.rerender(editor(false));
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  expect(editCalls(dispatch)[0]![1].selection).toBe(JSON.stringify([[0,width*height,255]]));
});

for(const kind of ["pixel","group"])for(const sample of fixture.affineMaskTargets)test(kind+" strokes preserve "+sample.name+" coordinates",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const mask={enabled:true,linked:sample.linked,invert:false,width:4,height:4,transform:sample.mask};
  const layer={kind,id:"p",width:4,height:4,children:[],transform:sample.layer,mask};
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[layer]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={1} brushOpacity={1} brushColor="#ffffff" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();
  const canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{for(const type of ["pointerdown","pointerup"])canvas.dispatchEvent(new PointerEvent(type,{bubbles:true,pointerId:1,button:0,clientX:sample.client[0],clientY:sample.client[1]}));});
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  const [command,payload]=editCalls(dispatch)[0]!;
  expect(command).toBe("paintStroke");
  expect(payload).toEqual({layerId:"p",tool:"brush",xs:[sample.point[0],sample.point[0]],ys:[sample.point[1],sample.point[1]]});
});

import patch,{type Operation} from "fast-json-patch";
import revisions from "../../../../../../../../../../🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🖌️stroke-revision/🔣️.json";
for(const row of revisions.cases)test("active stroke handles target revision "+row.name,async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const props={documentJson:JSON.stringify(revisions.document),assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["paint"]',activeUtility:"paintBrush",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<PixelEditingOverlay {...props}/>);
  const canvas=view.getByRole("application",{name:"Image tools"});canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  fireEvent.pointerDown(canvas,{pointerId:1,button:0,clientX:50,clientY:50});
  const changed=patch.applyPatch(revisions.document,row.patch as Operation[],true,false).newDocument;
  view.rerender(<PixelEditingOverlay {...props} documentJson={JSON.stringify(changed)} selectionJson={JSON.stringify(row.selected)} activeUtility={row.utility}/>);
  fireEvent.pointerUp(canvas,{pointerId:1,button:0,clientX:50,clientY:50});
  if(row.cancel){await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});expect(dispatch.mock.calls.filter(call=>call[0]==="paintStroke")).toEqual([]);}
  else await waitFor(()=>expect(dispatch.mock.calls.filter(call=>call[0]==="paintStroke")).toHaveLength(1));
});

for(const spans of ["[]","[[1,2,128]]"])test("restores shared selection "+spans+" across image content updates",async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  const props={documentJson:JSON.stringify({layers:[{kind:"pixel",id:"p",width:3,height:2}]}),assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',pixelSelectionJson:JSON.stringify({layerId:"p",target:"pixels",width:3,height:2,spans}),activeUtility:"select",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<ControlledPixelEditingOverlay {...props}/>);
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false));
  view.rerender(<ControlledPixelEditingOverlay {...props} documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:3,height:2,imageKey:"new-image"}]})} assetExtentsJson='{"new-image":{"width":3,"height":2}}'/>);
  view.container.querySelector("details")!.open=true;fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("editPixels",expect.objectContaining({expectedImageKey:"new-image",selection:spans})));
});
test("refused completed selection does not replace authoritative coverage",{timeout:20000},async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn(async()=>({kind:"refused"}));
  const view=render(<ControlledPixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":3,"height":2}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} paintTarget="pixels" maskValue={255} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("setPixelSelection",{selection:{layerId:"p",target:"pixels",width:3,height:2,spans:"[[0,6,255]]"},expectedImageKey:null}));
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  expect(view.getByRole("alert").textContent).toContain("could not be applied");
});

test("incoming shared coverage cancels an in-flight brush stroke",async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const props={documentJson:'{"layers":[{"kind":"pixel","id":"p","width":3,"height":2}]}',assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',activeUtility:"paintBrush",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<ControlledPixelEditingOverlay {...props}/>),canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  view.rerender(<ControlledPixelEditingOverlay {...props} pixelSelectionJson='{"layerId":"p","target":"pixels","width":3,"height":2,"spans":"[]"}'/>);
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(editCalls(dispatch)).toEqual([]);
});

test("the bucket dispatches ONE parametric fillRegion click and never floods or fills on the host",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:3,height:1,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#ff0000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Fill",exact:true}));
  const canvas=view.getByRole("application",{name:"Image tools"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]==="fillRegion")).toBe(true),10_000);
  expect(dispatch.mock.calls.filter(call=>call[0]==="fillRegion")).toEqual([["fillRegion",{layerId:"p",x:1.5,y:0.5,tolerance:24}]]);
  expect(editCalls(dispatch)).toEqual([]);
});
