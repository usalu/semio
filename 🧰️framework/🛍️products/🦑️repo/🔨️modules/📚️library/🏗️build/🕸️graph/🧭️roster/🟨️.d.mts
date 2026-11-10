export const PROJECT_BASENAME_V1:string;
export interface RosterRulesV1{descends(name:string,path:string,parent:string):boolean;inCollection(path:string):boolean;admitsCandidate(supplied:unknown):boolean;isAuthoredProjectConfig(path:string):boolean;admitsManifest(path:string):boolean;candidates(files:readonly string[]):{projectFiles:string[];cargoFiles:string[]}}
export function createRosterRulesV1(options:{generatedDirectories:readonly string[];collectionNames:readonly string[];moduleMember:string;projectBasename?:string;isOpaque?:(path:string)=>boolean}):Readonly<RosterRulesV1>;
export const ROSTER_RULES_V1:Readonly<RosterRulesV1>;
