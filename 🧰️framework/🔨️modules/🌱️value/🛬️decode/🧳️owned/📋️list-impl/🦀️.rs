//! 🧳️ Native trait ownership binds persistent rows to their original typed child receiver.
use super::{NativeReceive,NativeReceiverSeedFor};
use semio_framework_value_native_list_receiver::{NativeListSource,NativePersistentListReceiver};
use crate::list::persistent::PersistentList;
impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync> NativeReceive<I> for PersistentList<T>{type Receiver=NativePersistentListReceiver<I,T,T::Receiver,NativeReceiverSeedFor<I::Row,T>>;fn receiver()->Self::Receiver{NativePersistentListReceiver::new()}}
