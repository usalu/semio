/** 🎟️ Independent original-budget test interface. */
export function repositoryRoot():string;
export function run(root:string):{values:number;admissions:number;blocked:number;sharedLedger:boolean;materialized:number;admission:{turns:number;glyphs:number;denied:number};sourceIdentity:{refused:number;resumed:number};retirement:{turns:number;blocked:number;copied:number;released:number}};
export function runOutline(root:string):Promise<{glyphs:number;pairs:number;turns:number;denied:number;maximumError:number}>;
