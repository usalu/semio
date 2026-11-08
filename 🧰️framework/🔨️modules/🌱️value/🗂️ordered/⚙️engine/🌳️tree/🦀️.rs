//! 🌳️ One immutable native tree data owner backs cold updates and borrowed lookup.
use semio_framework_value_allocation_return::original::SharedOriginalStorage as SharedOwner;
pub type Root<V>=Option<SharedOwner<Node<V>>>;
pub struct Entry<V> { pub key: SharedOwner<String>, pub value: SharedOwner<V> }
pub struct Node<V> { pub entry: SharedOwner<Entry<V>>, pub left: Root<V>, pub right: Root<V>, pub height: usize, pub len: usize }

impl<V> Clone for Entry<V> {
    fn clone(&self) -> Self { Self { key: self.key.clone(), value: self.value.clone() } }
}
impl<V> Clone for Node<V> {
    fn clone(&self) -> Self { Self { entry: SharedOwner::clone(&self.entry), left: self.left.clone(), right: self.right.clone(), height: self.height, len: self.len } }
}

pub fn native_height<V>(root: &Root<V>) -> usize { root.as_ref().map_or(0, |value| value.height) }
pub fn native_len<V>(root: &Root<V>) -> usize { root.as_ref().map_or(0, |value| value.len) }
