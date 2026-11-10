import {GENERATION_PIN_POLICY_V1,generationPinStateV1,type GenerationPinV1} from "../🕰️generations/🧷️pin/🟨️.mjs";


/** 📍️ The pin of the generation the newest graph load of this process served; the descendants that answer to this invocation read exactly that generation. */
let sealedGenerationPinTextV1:string|undefined;

/** 🎁️ Hands the pin of the newest generation load to the ones that start nested commands; with a `tool` only when that build tool issues every task environment itself (the others keep the graph of their own nested commands). */
export function sealedGenerationPinV1(tool?:string):string|undefined{return tool===undefined||GENERATION_PIN_POLICY_V1.tools.includes(tool)?sealedGenerationPinTextV1:undefined;}

/** 🏷️ Takes over the pin a graph child of this invocation reported (or forgets it with undefined). */
export function recordSealedGenerationPinV1(text:string|undefined):void{sealedGenerationPinTextV1=text;}

/** 🧭️ The pin this command is bound by: the one its Original invocation put in the environment while that process is alive; a pin of anyone else is none. */
export function boundGenerationPinV1():GenerationPinV1|undefined{const state=generationPinStateV1(process.env);return state.state==="bound"?state.pin:undefined;}
