export type PreparationIdentity={standard:"IFC4"|"IFC2X3";instance:number;operation:string;generation:string;revision:string;source:string};
/** 🪪️ Compare complete original captured identities without narrowing integer strings. */
export const samePreparationIdentity=(a:PreparationIdentity,b:PreparationIdentity):boolean=>a.standard===b.standard&&a.instance===b.instance&&a.operation===b.operation&&a.generation===b.generation&&a.revision===b.revision&&a.source===b.source;
