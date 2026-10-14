import nativeCatalog from "../../🧩️catalog/🤖️generated/🦀️catalog.json";
import {readNativeCatalogV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import {useMemo} from "react";
import {OsBootHost as GeneralOsBootHost,WgpuBootHost as GeneralWgpuBootHost,type OsBootHostProps as GeneralOsBootHostProps,type WgpuBootHostProps as GeneralWgpuBootHostProps} from "../../../../🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx";
import {composeSpecificOsCatalogV1} from "../../🧩️catalog/🟦️.ts";
import {PLUGIN_BUILD_TARGETS,pluginModuleUrl} from "../../🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts";
export {PLUGIN_BUILD_TARGETS};
export type OsBootHostProps=Omit<GeneralOsBootHostProps,"inventory">;
export type WgpuBootHostProps=Omit<GeneralWgpuBootHostProps,"inventory">;
function useInventory(){return useMemo(()=>({nativeCatalog:readNativeCatalogV1(nativeCatalog),targets:PLUGIN_BUILD_TARGETS,moduleUrl:pluginModuleUrl,catalog:composeSpecificOsCatalogV1(window.location.href,{maxBytes:2097152,maxRows: 512,maxEdges:4096,maxWork:65536,deadlineMs:performance.now()+30000,now:()=>performance.now(),cancelled:()=>false,progress:()=>{}}).catalog}),[]);}
/** 🎭️ Application stories contribute their installed inventory to the neutral host. */
export function OsBootHost(props:OsBootHostProps){return <GeneralOsBootHost {...props} inventory={useInventory()}/>;}
export function WgpuBootHost(props:WgpuBootHostProps){return <GeneralWgpuBootHost {...props} inventory={useInventory()}/>;}
