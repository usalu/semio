/** 🧾️ One recorded failure of one project (or Cargo manifest). */
export interface CollectedErrorRowV1{readonly project:string;readonly root:string;readonly phase:string;readonly target?:string;readonly code?:string;readonly error:string;readonly cause:string;readonly stackTop:readonly string[]}
export interface CollectedErrorGroupV1{readonly cause:string;readonly count:number;readonly code:string|null;readonly message:string;readonly stackTop:readonly string[];readonly projects:readonly {project:string;root:string;phase:string;target?:string}[]}
export interface CollectedErrorReportV1{readonly version:1;readonly startedAt:string;readonly finishedAt:string;readonly workspace:string;readonly directory:string;readonly projectsTotal:number;readonly projectsFailed:number;readonly groups:readonly CollectedErrorGroupV1[];readonly rows:readonly CollectedErrorRowV1[]}
export interface CollectSummaryTextsV1{readonly [language:string]:{readonly headline:string;readonly group:string;readonly final:string}}
export interface ErrorCollectorV1{guard(meta:{project:string;root:string;phase:string},run:()=>Promise<unknown>):Promise<boolean>;rows():CollectedErrorRowV1[];finish(options:{projects:number}):Promise<never>}
export const component:"collect-errors";
export function collectErrorsDirectoryV1(env?:Record<string,string|undefined>):string|undefined;
export function causeKeyV1(message:unknown):string;
export function stackTopV1(error:unknown,workspaceRoot:string,count?:number):string[];
export function renderCollectSummaryV1(report:CollectedErrorReportV1,texts:CollectSummaryTextsV1,language:string):string;
export function createErrorCollectorV1(options:{directory:string;workspaceRoot:string;texts:CollectSummaryTextsV1;signal?:AbortSignal;now?:()=>Date;write?:(text:string)=>void}):ErrorCollectorV1;
export const collectTextsV1:CollectSummaryTextsV1;
export function createErrorCollectorFromEnvV1(options:{workspaceRoot:string;signal?:AbortSignal;env?:Record<string,string|undefined>}):ErrorCollectorV1|undefined;
