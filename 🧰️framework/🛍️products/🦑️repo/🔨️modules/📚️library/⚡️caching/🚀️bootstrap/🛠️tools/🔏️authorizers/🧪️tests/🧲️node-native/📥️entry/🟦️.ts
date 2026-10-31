import {createHash} from "node:crypto";
import {nativeSystemFromProviderV1} from "../../../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts";
import {defaultNodeNativePortsV1,planNodeNativeAddonV1} from "../../../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts";

/** 🧲️ Observes the actual resource selected by the bundled framework module after import completes. */
export function observeNativeSourceV1():Readonly<{source:string;sha256:string;linker:unknown}>{const ports=defaultNodeNativePortsV1();return{source:ports.host.source,sha256:createHash("sha256").update(ports.read(ports.host.source)).digest("hex"),linker:planNodeNativeAddonV1(ports.host,ports.read(ports.host.source)).linker??null};}
/** 🔌️ Executes the real default or explicit-provider branch through the public native-system interface. */
export function nativePointerV1():string{return String(nativeSystemFromProviderV1().ptr(Buffer.alloc(1)));}
