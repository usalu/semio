/** 📨️ A donated request belongs to one actual mounted window and explicit locale. */
import {stagedFormToken,type StagedFormRequest} from './🟦️.ts';
export function stagedFormContextMatches(request:StagedFormRequest,window:string|undefined,locale:'en'|'de'):boolean{return stagedFormToken(request.token)&&!!window&&request.token.window===window&&request.locale===locale;}
