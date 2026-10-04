/** 📤️ The declared Jack JSON field boundary emits every canonical IEEE word exactly. */
import {parseJackArtifact,type JackArtifact} from "../../../../../../../🧬️schema/🟦️.ts";
/** 🖨️ Publish literal persisted fields with closed, fixed-width camera words. */
export function jackToJsonValue(snapshot:JackArtifact):unknown{const value=parseJackArtifact(snapshot);return{schema:value.schema,name:value.name,...(value.manifestId===undefined?{}:{manifestId:value.manifestId}),manifest:value.manifest,camera:{x:{bits:value.camera.x.bits.toString(16).padStart(16,"0")},y:{bits:value.camera.y.bits.toString(16).padStart(16,"0")},zoom:{bits:value.camera.zoom.bits.toString(16).padStart(16,"0")}},content:value.content,...(value.rootNodeId===undefined?{}:{rootNodeId:value.rootNodeId}),query:value.query};}
