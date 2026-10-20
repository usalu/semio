import type {PackagePayloadOperations} from "../📦️payload/🟦️.ts";
import type {BunWorkspacePhysicalV1,BunWorkspacePhysicalPolicyV1,BunRepositoryMembership} from "./🟦️.ts";
/** 🪢️ Binds one physical policy baseline without issuing operation authority. */
export function createBunWorkspaceOwnershipV1(source:BunWorkspacePhysicalPolicyV1):BunWorkspacePhysicalV1;
/** 📋️ Computes membership from explicit Source DATA operations without issuing custody. */
export function readBunRepositoryMembership(root:string,operations:PackagePayloadOperations):BunRepositoryMembership;
