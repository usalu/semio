import {binary64Value} from "@semio-tech/framework/value/ieee754";
import type {DrawingArtboard,DrawingArtifact} from "../../../../../✳️any/🧬️schema/🟦️.ts";
/** 🖼️ Artboard replacement is authored in the actual Drawing mutation stream. */
export interface SetArtboard{readonly artboard:DrawingArtboard|null}
export interface DrawingArtboardOutcome{readonly diff:{readonly artboard?:DrawingArtboard|null};readonly inverse:({readonly mutation:'setArtboard'}&SetArtboard)|null;readonly error?:'mutation.invariant';readonly noop:boolean}
/** 📐️ Fixed native fields produce the same sparse diff and inverse as retained Rust publication. */
export function decideDrawingArtboard(before:Pick<DrawingArtifact,'artboard'>,command:SetArtboard):DrawingArtboardOutcome{
 const next=command.artboard,old=before.artboard??null;
 if(next&&(!Number.isFinite(binary64Value(next.width))||binary64Value(next.width)<=0||!Number.isFinite(binary64Value(next.height))||binary64Value(next.height)<=0))return{diff:{},inverse:null,error:'mutation.invariant',noop:false};
 if((!next&&!old)||(next&&old&&binary64Value(next.width)===binary64Value(old.width)&&binary64Value(next.height)===binary64Value(old.height)))return{diff:{},inverse:null,noop:true};
 return{diff:{artboard:next?{width:next.width,height:next.height}:null},inverse:{mutation:'setArtboard',artboard:old?{width:old.width,height:old.height}:null},noop:false};
}
