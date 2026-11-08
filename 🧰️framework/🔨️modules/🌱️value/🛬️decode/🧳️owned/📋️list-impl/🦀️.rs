//! 🧳️ Native trait ownership binds persistent rows to their original typed child receiver.
use super::{NativeReceive,NativeReceiverSeedFor};
use semio_framework_value_native_list_receiver::{NativeListSource,NativePersistentListReceiver};
use crate::list::persistent::PersistentList;
impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync> NativeReceive<I> for PersistentList<T>{type Receiver=NativePersistentListReceiver<I,T,T::Receiver,NativeReceiverSeedFor<I::Row,T>>;fn receiver()->Self::Receiver{NativePersistentListReceiver::new()}}
impl NativeListSource for dyn super::tree::NativeValueSource{type Row=dyn super::tree::NativeValueSource;fn row_count(&self)->Result<usize,crate::ValueError>{self.sequence_len()}fn row(&self,index:usize)->Result<&Self::Row,crate::ValueError>{self.sequence_child(index)}}
