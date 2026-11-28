/** ♿️ What a test may say to find an element: its role and its accessible name, optionally inside another element found the same way. There is no selector field, so CSS and XPath cannot enter a locator.
 * @see https://www.w3.org/TR/wai-aria-1.2/#role_definitions
 * @see https://www.w3.org/TR/accname-1.2/ */
export type AccessibleTargetV1=Readonly<{role:string;name:string;exact?:boolean;within?:AccessibleTargetV1;nth?:number}>;

/** 🔎️ The resolver both runtimes share: role, accessible name, hidden state and the query by role and name. */
export type AccessibleApiV1=Readonly<{
 role(element:Element):string|null;
 name(element:Element):string;
 hidden(element:Element):boolean;
 queryAll(root:ParentNode,target:AccessibleTargetV1):Element[];
}>;

/** 🧩️ Installs the resolver on a window-like scope (`scope.__semioAccessibleV1`) and returns it. Self-contained on purpose: the same function runs in Node over jsdom and, as its source text, inside a browser page (`accessibleInstallScriptV1`), so both runtimes locate with the identical rules. Implements the parts of ARIA 1.2 implicit roles and accname 1.2 the shell uses: `aria-labelledby`, embedded control values, `aria-label`, native labels (`label`, `alt`, `legend`, `caption`, input values), name from content, `title`; hidden through `hidden`, `aria-hidden`, `inert`, `display` and `visibility`.
 * @see https://www.w3.org/TR/html-aam-1.0/#html-element-role-mappings */
export function installAccessibleV1(scope:{getComputedStyle?:(element:Element)=>{display:string;visibility:string};__semioAccessibleV1?:AccessibleApiV1}):AccessibleApiV1{
 const fromContent=new Set(["button","cell","checkbox","columnheader","gridcell","heading","link","menuitem","menuitemcheckbox","menuitemradio","option","radio","row","rowheader","switch","tab","tooltip","treeitem"]);
 const sectioning=new Set(["ARTICLE","ASIDE","MAIN","NAV","SECTION"]);
 const squash=(text:string):string=>text.replace(/\s+/gu," ").trim();
 const attr=(element:Element,name:string):string|null=>element.getAttribute(name);
 const inputRole=(element:Element):string=>{
  const type=(attr(element,"type")??"text").toLowerCase();
  if(["button","submit","reset","image"].includes(type))return"button";
  if(type==="checkbox")return"checkbox";
  if(type==="radio")return"radio";
  if(type==="range")return"slider";
  if(type==="number")return"spinbutton";
  if(type==="search")return attr(element,"list")?"combobox":"searchbox";
  if(type==="hidden")return"";
  return attr(element,"list")?"combobox":"textbox";
 };
 const implicit=(element:Element):string|null=>{
  const tag=element.tagName;
  switch(tag){
   case"A":case"AREA":return element.hasAttribute("href")?"link":null;
   case"BUTTON":return"button";
   case"INPUT":return inputRole(element)||null;
   case"TEXTAREA":return"textbox";
   case"SELECT":return element.hasAttribute("multiple")||Number(attr(element,"size")??"0")>1?"listbox":"combobox";
   case"OPTION":return"option";
   case"H1":case"H2":case"H3":case"H4":case"H5":case"H6":return"heading";
   case"NAV":return"navigation";
   case"MAIN":return"main";
   case"ASIDE":return"complementary";
   case"UL":case"OL":case"MENU":return"list";
   case"LI":return"listitem";
   case"IMG":return attr(element,"alt")===""?"presentation":"img";
   case"TABLE":return"table";
   case"TR":return"row";
   case"TD":return"cell";
   case"TH":return"columnheader";
   case"DIALOG":return"dialog";
   case"ARTICLE":return"article";
   case"FIELDSET":return"group";
   case"HR":return"separator";
   case"PROGRESS":return"progressbar";
   case"OUTPUT":return"status";
   case"FORM":return"form";
   case"SECTION":return authored(element)?"region":null;
   case"HEADER":case"FOOTER":{for(let up=element.parentElement;up;up=up.parentElement)if(sectioning.has(up.tagName))return null;return tag==="HEADER"?"banner":"contentinfo";}
   default:return null;
  }
 };
 const authored=(element:Element):boolean=>!!squash(attr(element,"aria-label")??"")||!!squash(attr(element,"title")??"")||(attr(element,"aria-labelledby")??"").split(/\s+/u).some(id=>!!id&&!!squash(element.ownerDocument.getElementById(id)?.textContent??""));
 const hidden=(element:Element):boolean=>{
  for(let node:Element|null=element;node;node=node.parentElement){
   if(node.hasAttribute("hidden")||attr(node,"aria-hidden")==="true"||node.hasAttribute("inert"))return true;
   const style=scope.getComputedStyle?.(node);
   if(style&&(style.display==="none"||(node===element&&style.visibility==="hidden")))return true;
  }
  return false;
 };
 const controlValue=(element:Element,role:string|null):string|null=>{
  if(role==="textbox"||role==="searchbox")return (element as HTMLInputElement).value??element.textContent??"";
  if(role==="combobox"||role==="listbox"){const selected=element.tagName==="SELECT"?[...(element as HTMLSelectElement).selectedOptions]:[...element.querySelectorAll('[aria-selected="true"]')];return selected.map(option=>option.textContent??"").join(" ");}
  if(role==="slider"||role==="spinbutton")return attr(element,"aria-valuetext")??attr(element,"aria-valuenow")??(element as HTMLInputElement).value??"";
  return null;
 };
 const native=(element:Element):string=>{
  const tag=element.tagName;
  if(tag==="INPUT"||tag==="TEXTAREA"||tag==="SELECT"){
   const labels=[...((element as HTMLInputElement).labels??[])].map(label=>compute(label,{traversing:true,referenced:false,visited:new Set([element])})).join(" ");
   if(squash(labels))return labels;
   if(tag==="INPUT"){const type=(attr(element,"type")??"").toLowerCase();if(["button","submit","reset"].includes(type))return attr(element,"value")??(type==="submit"?"Submit":type==="reset"?"Reset":"");if(type==="image")return attr(element,"alt")??"";}
  }
  if(tag==="IMG"||tag==="AREA")return attr(element,"alt")??"";
  const first=(selector:string)=>{const child=[...element.children].find(node=>node.matches(selector));return child?compute(child,{traversing:true,referenced:false,visited:new Set()}):"";};
  if(tag==="FIELDSET")return first("legend");
  if(tag==="TABLE")return first("caption");
  if(tag==="FIGURE")return first("figcaption");
  return"";
 };
 type Walk={traversing:boolean;referenced:boolean;visited:Set<Node>};
 const compute=(element:Element,walk:Walk):string=>{
  if(walk.visited.has(element))return"";
  walk.visited.add(element);
  if(!walk.referenced&&hidden(element))return"";
  const labelledby=attr(element,"aria-labelledby");
  if(labelledby&&!walk.referenced){
   const text=labelledby.split(/\s+/u).map(id=>element.ownerDocument.getElementById(id)).filter((node):node is HTMLElement=>node!==null).map(node=>compute(node,{traversing:true,referenced:true,visited:walk.visited})).join(" ");
   if(squash(text))return text;
  }
  const role=api.role(element);
  if(walk.traversing&&role!==null&&!["button","link"].includes(role)){const value=controlValue(element,role);if(value!==null)return value;}
  const label=attr(element,"aria-label");
  if(label&&squash(label))return label;
  if(role!=="presentation"&&role!=="none"){const text=native(element);if(squash(text))return text;}
  if(walk.traversing||(role!==null&&fromContent.has(role))||role===null&&walk.referenced){
   const text=[...element.childNodes].map(child=>child.nodeType===3?child.textContent??"":child.nodeType===1?compute(child as Element,{traversing:true,referenced:walk.referenced,visited:walk.visited}):"").join("");
   if(squash(text))return text;
  }
  return attr(element,"title")??"";
 };
 const matches=(element:Element,target:AccessibleTargetV1):boolean=>{
  if(api.role(element)!==target.role)return false;
  const name=api.name(element);
  return target.exact===false?name.toLowerCase().includes(squash(target.name).toLowerCase()):name===squash(target.name);
 };
 const api:AccessibleApiV1={
  role:element=>{const explicit=attr(element,"role")?.trim().split(/\s+/u)[0];return explicit?explicit:implicit(element);},
  name:element=>squash(compute(element,{traversing:false,referenced:false,visited:new Set()})),
  hidden,
  queryAll(root,target){
   const scopes=target.within===undefined?[root]:api.queryAll(root,target.within);
   const found=scopes.flatMap(within=>[...within.querySelectorAll("*")].filter(element=>!hidden(element)&&matches(element,target)));
   const unique=[...new Set(found)];
   return target.nth===undefined?unique:unique.slice(target.nth,target.nth+1);
  },
 };
 scope.__semioAccessibleV1=api;
 return api;
}

/** 📜️ The resolver as a script for a browser page: installs it on `window` before any page script runs.
 * @see installAccessibleV1 */
export function accessibleInstallScriptV1():string{return `(${installAccessibleV1.toString()})(window);`;}

/** 🏷️ Renders a target for messages: `role "name"` and its `within` chain. */
export function describeAccessibleTargetV1(target:AccessibleTargetV1):string{
 return `${target.within===undefined?"":describeAccessibleTargetV1(target.within)+" > "}${target.role} "${target.name}"${target.exact===false?" (contains)":""}${target.nth===undefined?"":` #${target.nth}`}`;
}
