/** 🔢️ Transfers one admitted scalar only before closure or an earlier transfer. */
export function scalarCloneOutputReady(state:Readonly<{output:boolean;closing:boolean;spent:boolean}>):boolean{return state.output&&!state.closing&&!state.spent;}
/** 🪆️ An enclosing owner admits child closure before asking the child to quote it. */
export function childCloneCloseStage(state:Readonly<{started:boolean;empty:boolean}>):"begin"|"retire"|"advance"{return !state.started?"begin":state.empty?"retire":"advance";}
/** 📏️ Structural depth refusal precedes any cursor allocation. */
export function cloneDepthRefused(maximumDepth:number,requiredDepth:number):boolean{return maximumDepth<requiredDepth;}
