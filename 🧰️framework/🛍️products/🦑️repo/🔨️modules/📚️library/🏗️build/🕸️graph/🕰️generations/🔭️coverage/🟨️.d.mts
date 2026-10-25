/** 🔭️ The declared constants of the verified stamp. */
export interface CoveragePolicyV1{readonly version:1;readonly maximumOwners:number;readonly bitOrder:"least-significant-first";readonly alphabet:"base64"}
export const COVERAGE_POLICY_V1:CoveragePolicyV1;
/** 🔍️ A closed stamp: when the observation started, how many owners and which of them were verified. */
export interface VerifiedStampV1{readonly atMs:number;readonly count:number;readonly owners:string}
export interface CoverageV1{readonly atMs:number;readonly names:ReadonlySet<string>}
export function encodeVerifiedScopeV1(owners:readonly string[],covered:ReadonlySet<string>):{count:number;owners:string};
export function readVerifiedStampV1(value:unknown):VerifiedStampV1;
export function decodeVerifiedScopeV1(owners:readonly string[],stamp:VerifiedStampV1):Set<string>;
export function mergeVerifiedStampV1(options:{owners:readonly string[];scope:ReadonlySet<string>|undefined;observedAtMs:number;carried?:{atMs:number;names:ReadonlySet<string>}|undefined}):VerifiedStampV1;
export function coverageOfV1(options:{stamp:VerifiedStampV1|undefined;owners:readonly string[];implementation:string;manifestImplementation:string;requestedAtMs:number}):CoverageV1|undefined;
export function uncoveredOfV1(options:{needed:ReadonlySet<string>|undefined;owners:readonly string[];coverage:CoverageV1}):Set<string>;
