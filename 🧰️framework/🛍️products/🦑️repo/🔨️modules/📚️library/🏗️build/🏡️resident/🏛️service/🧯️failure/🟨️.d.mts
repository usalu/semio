export function accessDeniedOfV1(error:unknown,options?:{root?:string;platform?:string;home?:string}):string|undefined;
export function failureRefusalV1(error:unknown,options:{log:(event:string,detail?:string)=>void;logPath:string;retry?:"later"|"never";root?:string;platform?:string;home?:string;environment?:Record<string,string|undefined>;pid?:number;ppid?:number}):{code:"disk"|"internal"|"unserved"|"consent";key:string;fields:Record<string,string>;retry:"later"|"never"};
export function wantedRefusalOfV1(error:unknown,depth?:number):Record<"wanted"|"arrived"|"seconds"|"missing"|"local",string>|undefined;
export function deniedPlaceOfV1(error:unknown,depth?:number):string|undefined;
export function serviceFactsV1(options?:{environment?:Record<string,string|undefined>;pid?:number;ppid?:number}):string;
export function serviceRouteV1(environment?:Record<string,string|undefined>):"launchd"|"direct"|"foreground";
