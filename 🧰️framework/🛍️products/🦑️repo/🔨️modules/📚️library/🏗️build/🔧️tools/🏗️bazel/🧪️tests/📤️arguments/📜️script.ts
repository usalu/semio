/** 📤️ Arguments: command script run by its tests. */
import{appendFile}from"node:fs/promises";
await appendFile(process.env.SEMIO_ARGUMENT_OUTPUT!,JSON.stringify(process.argv.slice(2))+"\n");
