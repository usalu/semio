/** 🔮️ Independent ZIP decoder returns only owned bytes. */
import {unzipSync} from "fflate";
export function decodeEntries(bytes:Uint8Array):Record<string,Uint8Array>{return unzipSync(bytes);}
