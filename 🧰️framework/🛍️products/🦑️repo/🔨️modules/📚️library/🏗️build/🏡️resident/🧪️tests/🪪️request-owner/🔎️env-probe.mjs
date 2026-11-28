/** 🔎️ The service of the owner laws: writes the whole environment it was started with to the file named by its first argument.
 * @see ../🟦️.ts */
import {writeFileSync} from "node:fs";
writeFileSync(process.argv[2],JSON.stringify(process.env));
setInterval(()=>{},100);
