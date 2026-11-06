import {createWatchSupervisorV1} from "../🟨️.mjs";

/** 🎟️ Gives the watcher the resident caller's original signal and remaining deadline. */
export function createResidentWatcherV1(options,startupControl,factory=createWatchSupervisorV1){return factory({...options,...(startupControl?{startupControl}:{})});}
