/** 📏️ Computes conservative frame bounds from already observed physical DATA. */
export function closureReadReplyMaximumV1(rows:readonly Readonly<{kind?:string;path:string;bytes:number}>[],policy:Readonly<{maximumResultBytes:number;maximumProofBytes:number;maximumReferences:number}>,session:Readonly<Record<string,number>>):number;
/** 🧮️ Admits one bounded request under an unchanged cohort capture. */
export function closureReadCohortAdmitsV1(state:Readonly<{frames:number;inputBytes:number;outputBytes:number;outputLines:number;diagnosticBytes?:number}>,requestBytes:number,resultBytes:number,session:Readonly<Record<string,number>>):boolean;
/** 🧾️ Serializes bounded semantic first-error facts without foreign API types. */
export function closureReadFailurePacketV1(error:unknown,roots:Readonly<{root:string;imageRoot:string}>,document:unknown):string;
export function closureReadReplyCreditV1(state:Readonly<{frames:number;inputBytes:number;outputBytes:number;outputLines:number;diagnosticBytes?:number}>,policy:Readonly<{maximumRequestBytes:number}>,session:Readonly<Record<string,number>>):number|null;
export function closureReadDeferredV1(value:unknown,credit:number,minimumReadBytes:number,remainingReadBytes:number,policy:Readonly<{maximumProofBytes:number}>,session:Readonly<Record<string,number>>):boolean;

/** 📥️ Projects lossless text through the existing finite canonical binary input interface. */
export function closureReadInputFramesV1(text:string,input:Readonly<{maximumBinaryBytes:number}>,session:Readonly<{maximumInputBytes:number;maximumInputFrames:number}>):readonly Readonly<{version:1;base64:string}>[];

/** 🧾️ One bounded diagnostic text with its byte size, digest and truncation flag. */
export type ClosureStartupTextV1=Readonly<{text:string;bytes:number;sha256:string;truncated:boolean}>;
/** 🔎️ Describes bounded actual readiness observations and stderr snapshots around accepted disposal. */
export function closureReadStartupWitnessV1(expectedSource:string,observed:string|null,stderrBefore:string|null,stderrAfter:string|null,session:Readonly<Record<string,number>>):Readonly<{version:1;expectedSource:string;comparison:string;observed:ClosureStartupTextV1|null;stderrBefore:ClosureStartupTextV1|null;stderrAfter:ClosureStartupTextV1|null}>;
