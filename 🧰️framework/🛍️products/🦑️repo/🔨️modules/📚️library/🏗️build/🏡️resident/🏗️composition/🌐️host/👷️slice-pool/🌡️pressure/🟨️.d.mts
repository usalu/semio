export function hostPressureLevelV1(options?:{platform?:string;run?:()=>string;read?:(path:string)=>string;policy?:{fullAvg10Percent:number}}):"normal"|"warn"|"critical"|undefined;
