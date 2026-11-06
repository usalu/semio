//! 📏️ One named bound set per Store generic: every owner above the Store spells `P: StoreState` and `M: StoreMutationFor<P>` instead of its own hand-written copy of the same traits.
use semio_framework_value::{FromValue, ToValue};
use crate::{ColdMutation, Mutation, OpBinary, OpText};

/// 🧱️ What an artifact state must be for the Store to hold, replay, project and retire it.
pub trait StoreState: Clone + ToValue + FromValue + Send + Sync + 'static {}

impl<T: Clone + ToValue + FromValue + Send + Sync + 'static> StoreState for T {}

/// 🧬️ Persistent mutations declare original bounded binary receiving and retirement alongside log, replay and encoding.
pub trait StoreMutationFor<P>: Clone + ToValue + FromValue + Mutation<P> + ColdMutation<P> + protocol::OpBinaryReceiving + OpBinary + OpText + Send + 'static {}

impl<P, T: Clone + ToValue + FromValue + Mutation<P> + ColdMutation<P> + protocol::OpBinaryReceiving + OpBinary + OpText + Send + 'static> StoreMutationFor<P> for T {}
