import type { NativeOwnerCapabilities } from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 🏛️ Declares actual native resources granted to an owned build invocation. */
export interface NativeBuildCallerCapabilitiesV1 { readonly tool:string; readonly workspaceRoot:string; readonly arguments:readonly string[]; readonly selection:readonly string[]; readonly buildMode:"prototype"|"release"; readonly native:Readonly<{artifactDirectory:string;transport:NativeOwnerCapabilities["transport"];child:NativeOwnerCapabilities["child"];network:NativeOwnerCapabilities["network"]}>; readonly launches:readonly (readonly string[])[] }
