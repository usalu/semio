/** 🔤️ Exposes only a completed owned string before its close or transfer. */
export function stringCloneOutputReady(state:Readonly<{complete:boolean;closing:boolean;output:boolean}>):boolean{return state.complete&&!state.closing&&state.output;}
