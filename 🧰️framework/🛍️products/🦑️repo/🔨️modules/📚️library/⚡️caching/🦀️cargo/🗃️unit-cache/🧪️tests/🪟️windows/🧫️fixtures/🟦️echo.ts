/** 🪟️ Fixture for cargo unit-cache windows. */
const exitStatus=process.env.XPLAT_EXIT;
if(exitStatus!==undefined)process.exit(Number(exitStatus));
process.stdout.write(JSON.stringify(process.argv.slice(2)));
