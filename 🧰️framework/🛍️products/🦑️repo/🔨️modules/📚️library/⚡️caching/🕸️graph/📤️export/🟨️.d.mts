/** 📤️ Projects native task hash instructions into their shared first-party debug DATA. */
export function compactTaskPlans(plans:Readonly<Record<string,readonly string[]>>):{version:1;instructions:string[];tasks:Record<string,number[]>};
