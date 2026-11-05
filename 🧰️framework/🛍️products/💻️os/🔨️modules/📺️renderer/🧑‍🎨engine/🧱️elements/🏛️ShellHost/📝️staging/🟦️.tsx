/** 🪟️ Native owned form controls keep their captured action scope inside the shell dialog. */
import {useId,useLayoutEffect,useRef,useSyncExternalStore} from "../../⚛️react-port/🟦️.ts";
import {OwnedStagedBodyView} from "../../🗣️Interpreter/🎛️control-port/owned.tsx";
import type {StagedActionHost} from "./session.ts";
const labels={en:{title:"Complete Action",pending:"Preparing action…",execute:"Execute",cancel:"Cancel",executing:"Applying action…",closing:"Finishing action…"},de:{title:"Aktion Vervollständigen",pending:"Aktion wird vorbereitet…",execute:"Ausführen",cancel:"Abbrechen",executing:"Aktion wird angewendet…",closing:"Aktion wird abgeschlossen…"}};
/** 🎟️ Readiness is published only by committed original status reads inside the owned body. */
export function StagedActionDialog({host}:{readonly host:StagedActionHost}){
 const state=useSyncExternalStore(host.subscribe,host.snapshot,()=>null),ref=useRef<HTMLDialogElement>(null),visible=state!==null,titleId=useId();
 useLayoutEffect(()=>{if(!visible)return;const dialog=ref.current;if(!dialog)return;dialog.showModal();return()=>{if(dialog.open)dialog.close();};},[visible]);
 if(!state)return null;
 const text=labels[state.locale];
 return <dialog ref={ref} aria-labelledby={titleId} onCancel={event=>{event.preventDefault();host.cancel();}} className="max-h-[90vh] w-[min(42rem,90vw)] overflow-auto rounded-sm border bg-menu p-double text-foreground shadow-xl">
  <h2 id={titleId}>{text.title}</h2>
  {state.pending||state.executing||state.closing?<div role="status" aria-live="polite"><progress aria-label={state.closing?text.closing:state.executing?text.executing:text.pending}/><span>{state.closing?text.closing:state.executing?text.executing:text.pending}</span></div>:null}
  {state.controls?<OwnedStagedBodyView owner={state.controls}/>:null}
  {state.error?<p role="alert">{state.error}</p>:null}
  <div className="flex justify-end gap-single"><button type="button" disabled={state.closing} onClick={host.cancel}>{text.cancel}</button><button type="button" disabled={!state.ready} onClick={host.execute}>{text.execute}</button></div>
 </dialog>;
}
