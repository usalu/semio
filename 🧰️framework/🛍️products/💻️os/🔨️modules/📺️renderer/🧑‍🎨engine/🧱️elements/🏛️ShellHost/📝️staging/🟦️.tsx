/** 🪟️ Native owned form controls keep their captured action scope inside the shell dialog. */
import {Button,SharedBorderRow,useSharedBorderFrameRef} from "@semio-tech/ui-react";
import {useId,useLayoutEffect,useRef,useSyncExternalStore} from "../../⚛️react-port/🟦️.ts";
import {OwnedStagedBodyView} from "../../🗣️Interpreter/🎛️control-port/owned.tsx";
import type {StagedActionHost} from "./session.ts";
const labels={en:{title:"Complete Action",pending:"Preparing action…",execute:"Execute",cancel:"Cancel",executing:"Applying action…",closing:"Finishing action…"},de:{title:"Aktion Vervollständigen",pending:"Aktion wird vorbereitet…",execute:"Ausführen",cancel:"Abbrechen",executing:"Aktion wird angewendet…",closing:"Aktion wird abgeschlossen…"}};
/** 🎟️ Readiness is published only by committed original status reads inside the owned body. */
export function StagedActionDialog({host}:{readonly host:StagedActionHost}){
 const state=useSyncExternalStore(host.subscribe,host.snapshot,()=>null),ref=useRef<HTMLDialogElement>(null),visible=state!==null,titleId=useId();
 const frameRef=useSharedBorderFrameRef(ref);
 useLayoutEffect(()=>{if(!visible)return;const dialog=ref.current;if(!dialog)return;dialog.showModal();return()=>{if(dialog.open)dialog.close();};},[visible]);
 if(!state)return null;
 const text=labels[state.locale];
 return <dialog ref={frameRef} data-shared-border-frame="" data-shared-border-edges="top right bottom left" aria-labelledby={titleId} onCancel={event=>{event.preventDefault();host.cancel();}} className="max-h-[90vh] w-[min(42rem,90vw)] overflow-visible bg-menu p-0 text-foreground shadow-xl">
  <div className="flex max-h-[90vh] flex-col">
  <div className="min-h-0 overflow-auto p-double"><h2 id={titleId}>{text.title}</h2>
  {state.pending||state.executing||state.closing?<div role="status" aria-live="polite"><progress aria-label={state.closing?text.closing:state.executing?text.executing:text.pending}/><span>{state.closing?text.closing:state.executing?text.executing:text.pending}</span></div>:null}
  {state.controls?<OwnedStagedBodyView owner={state.controls}/>:null}
  {state.error?<p role="alert">{state.error}</p>:null}</div>
  <SharedBorderRow edges={["top"]} className="w-full"><Button icon="x" type="button" variant="outline" aria-label={text.cancel} disabled={state.closing} onClick={host.cancel}>{text.cancel}</Button><Button icon="check" type="button" variant="outline" aria-label={text.execute} disabled={!state.ready} onClick={host.execute}>{text.execute}</Button></SharedBorderRow>
  </div>
 </dialog>;
}
