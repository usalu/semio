//! 📇️ The per-instance metadata registry that gates the task quota.

use crate::tasks_fixed_slots::{PLUGIN_REACTOR_INSTANCE_SLOTS, ReactorFixedSlots};
const PLUGIN_REACTOR_APP_ID_BYTES: usize = 256;

struct InstanceMetadata {
    pub instance: u32,
    pub app_id: String,
    pub quota: semio_framework::kernel::QuotaSchema,
}

#[doc(hidden)]
pub struct InstanceMetadataRegistry {
    pub slots: ReactorFixedSlots<InstanceMetadata>,
}

impl InstanceMetadataRegistry {
    pub fn new() -> Self {
        Self { slots: ReactorFixedSlots::new() }
    }

    pub fn index(instance: u32) -> usize {
        instance as usize % PLUGIN_REACTOR_INSTANCE_SLOTS
    }

    pub fn insert(&mut self, instance: u32, app_id: String, quota: semio_framework::kernel::QuotaSchema) -> Result<(), semio_framework::Fault> {
        if app_id.len() > PLUGIN_REACTOR_APP_ID_BYTES {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.instance-app-id-too-large"), "instance app id exceeds its admitted fixed byte bound"));
        }
        let index = Self::index(instance);
        if !self.slots.allocation_admitted || self.slots.get(index).is_some() {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.instance-metadata-capacity"), "fixed instance metadata authority is saturated or collided"));
        }
        self.slots
            .insert(index, InstanceMetadata { instance, app_id, quota })
            .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.instance-metadata-capacity"), "fixed instance metadata authority changed during insert"))
    }

    pub fn get(&self, instance: u32) -> Option<&InstanceMetadata> {
        self.slots.get(Self::index(instance)).filter(|entry| entry.instance == instance)
    }

    pub fn remove(&mut self, instance: u32) -> Option<InstanceMetadata> {
        let index = Self::index(instance);
        if self.slots.get(index).is_some_and(|entry| entry.instance == instance) {
            self.slots.take(index)
        } else {
            None
        }
    }

    pub fn checkpoint_rows(&self) -> Vec<(u32, String)> {
        self.slots.iter().map(|entry| (entry.instance, entry.app_id.clone())).collect()
    }
}
