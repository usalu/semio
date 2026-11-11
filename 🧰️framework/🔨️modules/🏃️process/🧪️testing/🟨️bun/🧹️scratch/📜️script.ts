/** 🧹️ Test preload: every test process gets a private scratch directory below the declared scratch home as its temporary directory and removes it when it ends, so no test leaves entries in the (RAM backed on many hosts) operating system temporary directory and a killed process is swept by `repo cache gc`.
 * @see ../../../🧹️scratch/🟦️.ts */
import {isolateProcessScratchV1} from "../../../🧹️scratch/🟦️.ts";

isolateProcessScratchV1({root:import.meta.dir});
