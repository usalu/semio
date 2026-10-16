/** 🔒️ The actual Cargo lock input identities consumed by the firstparty projection. */
export interface CargoLockIdentityV1{readonly name:string;readonly version:string;readonly source?:string}
export interface CargoLockPackageV1 extends CargoLockIdentityV1{readonly checksum?:string;readonly dependencies?:readonly string[]}
export interface CargoLockResolutionV1 extends CargoLockIdentityV1{readonly checksum?:string;readonly dependencies:readonly string[]}
export function cargoLockIdentity(entry:CargoLockIdentityV1):string;
export function cargoLockClosure(lock:Readonly<{version:4;package:readonly CargoLockPackageV1[]}>,roots:readonly Readonly<{name:string;version?:string}>[]):Readonly<{version:1;packages:readonly CargoLockResolutionV1[]}>;
