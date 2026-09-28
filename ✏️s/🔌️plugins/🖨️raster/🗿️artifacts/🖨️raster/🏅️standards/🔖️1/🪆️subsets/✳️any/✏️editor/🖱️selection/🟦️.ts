/** 🖱️ Addresses the framework-owned layer selection without persisting another editor state. */
export function layerSelectionArgs(id:string):{domainId:"layers";targets:string;merge:"replace";method:"pick"} {
  if(!id)throw new Error("Layer selection requires an identity");
  return {domainId:"layers",targets:JSON.stringify([{granularity:"layer",id}]),merge:"replace",method:"pick"};
}
