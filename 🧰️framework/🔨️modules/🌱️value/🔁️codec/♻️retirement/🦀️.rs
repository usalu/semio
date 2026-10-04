//! ♻️ Constant-storage destructive retirement of the complete intrinsic owner.
use crate::DslValue;

/// 🧵️ Retains traversal links inside vacated owned container slots.
pub struct IntrinsicRetirement { current: Option<DslValue>, parent: DslValue }
impl IntrinsicRetirement {
    /// 🔎️ Reports whether every retained owner has been released.
    pub fn terminal_is_empty(&self) -> bool { self.current.is_none() }
    pub fn new(value: DslValue) -> Self { Self { current: Some(value), parent: DslValue::Null } }
    pub fn close_step(&mut self, maximum_items: usize) -> usize {
        let mut completed = 0;
        while completed < maximum_items {
            let Some(current) = self.current.take() else { break };
            completed += 1;
            match current {
                DslValue::Array(mut items) => {
                    if let Some(child) = items.pop() {
                        items.push(std::mem::replace(&mut self.parent, DslValue::Null));
                        self.parent = DslValue::Array(items);
                        self.current = Some(child);
                        continue;
                    }
                }
                DslValue::Object(mut items) => {
                    if let Some((_, child)) = items.pop() {
                        items.push((String::new(), std::mem::replace(&mut self.parent, DslValue::Null)));
                        self.parent = DslValue::Object(items);
                        self.current = Some(child);
                        continue;
                    }
                }
                DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) | DslValue::String(_) | DslValue::Bytes(_) => {},
            }
            match std::mem::replace(&mut self.parent, DslValue::Null) {
                DslValue::Array(mut items) => {
                    self.parent = items.pop().expect("retirement array retains its parent link");
                    self.current = Some(DslValue::Array(items));
                }
                DslValue::Object(mut items) => {
                    self.parent = items.pop().expect("retirement object retains its parent link").1;
                    self.current = Some(DslValue::Object(items));
                }
                DslValue::Null => {},
                DslValue::Bool(_) | DslValue::Number(_) | DslValue::String(_) | DslValue::Bytes(_) => unreachable!("retirement parent is an owned container or terminal marker"),
            }
        }
        completed
    }
}
impl Drop for IntrinsicRetirement {
    fn drop(&mut self) { while self.current.is_some() { self.close_step(256); } }
}
