import { generatedCatalogSchema as nativeCatalog } from "@semio-tech/s-dev-catalog-ts/generated";
import {readNativeCatalogV1} from "@semio-tech/plugin-registry/runtime";
import {useMemo} from "react";
import {OsBootHost as GeneralOsBootHost,WgpuBootHost as GeneralWgpuBootHost,type OsBootHostProps as GeneralOsBootHostProps,type WgpuBootHostProps as GeneralWgpuBootHostProps} from "@semio-tech/framework-os/stories/coordination";
import {composeSpecificOsCatalogV1} from "@semio-tech/s-dev-catalog-ts";
import {PLUGIN_BUILD_TARGETS,pluginModuleUrl} from "@semio-tech/s-dev-catalog-ts/generated/plugins";
export {PLUGIN_BUILD_TARGETS};
export type OsBootHostProps=Omit<GeneralOsBootHostProps,"inventory">;
export type WgpuBootHostProps=Omit<GeneralWgpuBootHostProps,"inventory">;
function useInventory(){return useMemo(()=>{const composed=composeSpecificOsCatalogV1(window.location.href,{maxBytes:2097152,maxRows: 512,maxEdges:4096,maxWork:65536,deadlineMs:performance.now()+30000,now:()=>performance.now(),cancelled:()=>false,progress:()=>{}});return {nativeCatalog:readNativeCatalogV1(nativeCatalog),targets:PLUGIN_BUILD_TARGETS,moduleUrl:pluginModuleUrl,catalog:composed.catalog,rows:composed.rows};},[]);}
/** 🎭️ Application stories contribute their installed inventory to the neutral host. */
export function OsBootHost(props:OsBootHostProps){return <GeneralOsBootHost {...props} inventory={useInventory()}/>;}
export function WgpuBootHost(props:WgpuBootHostProps){return <GeneralWgpuBootHost {...props} inventory={useInventory()}/>;}
