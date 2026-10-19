import type {BuildTargetV1} from "../../../🕸️graph/🟦️.ts";

/** 📋️ One Bazel-planned task: the build target resolved for a project, its configuration and the task ids it depends on. */
export interface BazelTaskV1{id:string;project:string;target:string;configuration?:string;label:string;dependencies:string[];args:string[];definition:BuildTargetV1;root:string}

/** 📋️ The planned task graph of one request: every task by id and the ids the request selected. */
export interface BazelTaskPlanV1{version:1;tasks:Record<string,BazelTaskV1>;selected:string[]}
