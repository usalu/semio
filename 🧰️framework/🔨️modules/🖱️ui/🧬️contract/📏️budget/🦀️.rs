//! 📏️ Schema-owned UI capacities and their shared page arithmetic.
const SOURCE:&[u8]=include_bytes!("🧬️schema/🔣️.json");
const fn setting(name:&str)->usize{
 let key=name.as_bytes();let mut index=0;
 while index+key.len()+2<SOURCE.len(){
  if SOURCE[index]==b'"'{let mut equal=true;let mut offset=0;while offset<key.len(){if SOURCE[index+1+offset]!=key[offset]{equal=false;break}offset+=1;}
   if equal&&SOURCE[index+1+key.len()]==b'"'{let mut at=index+key.len()+2;while at<SOURCE.len()&&(SOURCE[at]==b' '||SOURCE[at]==b':'||SOURCE[at]==b'\n'){at+=1;}let mut value=0;let start=at;while at<SOURCE.len()&&SOURCE[at]>=b'0'&&SOURCE[at]<=b'9'{value=value*10+(SOURCE[at]-b'0')as usize;at+=1;}if at==start{panic!("numeric UI capacity required")}return value;}
  }index+=1;
 }panic!("schema UI capacity missing")
}
pub const UI_DOCUMENT_NODES:usize=setting("documentNodes");
pub const UI_NODE_BINDINGS:usize=setting("nodeBindings");
pub const UI_DOCUMENT_PATCH_OPS:usize=UI_DOCUMENT_NODES*9+1;
pub const UI_PAGE_WIDTH:usize=UI_DOCUMENT_NODES;
pub const UI_BUILT_CHILDREN_MAX:usize=UI_PAGE_WIDTH;
pub const UI_BUILT_CHILD_RETIRE_SLOTS:usize=setting("builtChildRetireSlots");
pub const UI_TEXT_MAX_BYTES:usize=setting("textMaxBytes");
pub const UI_FIXED_LIST_ITEMS:usize=setting("fixedListItems");
pub const UI_FIXED_BYTES:usize=setting("fixedBytes");
pub const UI_VALUE_PAGE_ITEMS:usize=setting("valuePageItems");
pub const UI_VALUE_MAX_ITEMS:usize=setting("valueMaxItems");
pub const UI_VALUE_NESTING_DEPTH:usize=setting("valueNestingDepth");
pub const UI_VALUE_ARGUMENT_ENTRIES:usize=setting("valueArgumentEntries");
pub const UI_VALUE_ROW_ACTIONS:usize=setting("valueRowActions");
pub const UI_VALUE_ROW_COLLECTIONS:usize=1+2*UI_VALUE_ROW_ACTIONS;
pub const UI_VALUE_ROW_ITEMS:usize=(1+UI_VALUE_ROW_ACTIONS)*UI_VALUE_ARGUMENT_ENTRIES+UI_VALUE_ROW_ACTIONS;
pub const UI_VALUE_PAGE_ROWS:usize=UI_PAGE_WIDTH;
pub const UI_VALUE_LIVE_PAGES:usize=setting("valueLivePages");
pub const UI_VALUE_ADMISSION_SLOTS:usize=UI_VALUE_LIVE_PAGES*UI_VALUE_PAGE_ROWS*UI_VALUE_ROW_COLLECTIONS;
pub const UI_VALUE_AGGREGATE_ITEMS:usize=UI_VALUE_LIVE_PAGES*UI_VALUE_PAGE_ROWS*UI_VALUE_ROW_ITEMS;
pub const UI_VALUE_HANDBACK_WORDS:usize=UI_VALUE_ADMISSION_SLOTS.div_ceil(64);
pub const TREE_WINDOW_FIXED_NODE_HEADROOM:usize=setting("treeFixedHeadroom");
pub const TREE_WINDOW_BODY_NODE_BUDGET:usize=UI_DOCUMENT_NODES-1-TREE_WINDOW_FIXED_NODE_HEADROOM;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
