/** 📌️ Reads the exact portable graph pinned by this owned build invocation; the caller injects the implementation-identity check so this leaf imports neither the plugin root nor the executor. */
async function graphPublication(workspaceRoot,environment,body,implementation){
 const captured=Object.freeze({...environment}),pin={path:captured.SEMIO_BUILD_GRAPH_PATH,digest:captured.SEMIO_BUILD_GRAPH_DIGEST},check=()=>{implementation();if(process.env.SEMIO_BUILD_GRAPH_PATH!==pin.path||process.env.SEMIO_BUILD_GRAPH_DIGEST!==pin.digest)throw Error("Original private Nx graph binding changed");};check();const {withReceivedNxGraphTaskV1}=await import("../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts"),{withRepositoryBuildGraphPublicationV1}=await import("../../../../🏗️build/🕸️graph/🟦️.ts");check();
 return withReceivedNxGraphTaskV1(workspaceRoot,captured,actual=>withRepositoryBuildGraphPublicationV1(workspaceRoot,actual,async(graph,digest)=>{check();const value=await body({graph,digest});check();return value;}));
}

/** 🔏️ Exposes only immutable private backend DATA read under its genuine received Graph task. */
export function readPrivateNxBuildGraphV1(workspaceRoot,environment,implementation){if(environment.SEMIO_BUILD_GRAPH_PATH!==process.env.SEMIO_BUILD_GRAPH_PATH||environment.SEMIO_BUILD_GRAPH_DIGEST!==process.env.SEMIO_BUILD_GRAPH_DIGEST)throw Error("Original private Nx graph binding changed");return graphPublication(workspaceRoot,environment,async({graph})=>graph,implementation);}

export {graphPublication as withNxBuildGraphPublicationV1};
