/** 🔬️ Owned independent numeric scratch observation. */
export function numericScratchOutput(operations:readonly {kind:string;key?:number;value?:number}[]):unknown[];

export function numericReservationOutput(rows:[number,number][]):[number,number][];
