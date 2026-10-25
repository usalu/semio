//! 🧷️ The job render binding registry keyed by job and instance.

use crate::tasks_fixed_slots::REACTOR_TASK_SLOTS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobRenderBinding {
    pub job: u64,
    pub instance: u32,
    pub generation: u64,
}

#[doc(hidden)]
pub struct JobRenderBindingRegistry {
    pub by_job: [Option<JobRenderBinding>; REACTOR_TASK_SLOTS],
    current_by_instance: [Option<JobRenderBinding>; REACTOR_TASK_SLOTS],
    pub next_generation: u64,
}

impl JobRenderBindingRegistry {
    pub const fn new() -> Self {
        Self { by_job: [None; REACTOR_TASK_SLOTS], current_by_instance: [None; REACTOR_TASK_SLOTS], next_generation: 0 }
    }

    pub fn bind(&mut self, instance: u32, job: u64) -> Result<JobRenderBinding, ()> {
        let instance_slot = instance as usize % REACTOR_TASK_SLOTS;
        if self.current_by_instance[instance_slot].is_some_and(|binding| binding.instance != instance) {
            return Err(());
        }
        let job_slot = job as usize % REACTOR_TASK_SLOTS;
        if self.by_job[job_slot].is_some() {
            return Err(());
        }
        let generation = self.next_generation.checked_add(1).ok_or(())?;
        self.next_generation = generation;
        let binding = JobRenderBinding { job, instance, generation };
        self.by_job[job_slot] = Some(binding);
        self.current_by_instance[instance_slot] = Some(binding);
        Ok(binding)
    }

    pub fn owned(&self, job: u64) -> Option<JobRenderBinding> {
        self.by_job[job as usize % REACTOR_TASK_SLOTS].filter(|binding| binding.job == job)
    }

    pub fn accepted(&self, job: u64) -> Option<JobRenderBinding> {
        let binding = self.owned(job)?;
        self.current_by_instance[binding.instance as usize % REACTOR_TASK_SLOTS].filter(|current| *current == binding)
    }

    pub fn complete(&mut self, job: u64) -> Option<JobRenderBinding> {
        let job_slot = job as usize % REACTOR_TASK_SLOTS;
        let binding = self.by_job[job_slot].filter(|binding| binding.job == job)?;
        self.by_job[job_slot] = None;
        let instance_slot = binding.instance as usize % REACTOR_TASK_SLOTS;
        (self.current_by_instance[instance_slot] == Some(binding)).then(|| {
            self.current_by_instance[instance_slot] = None;
            binding
        })
    }

    pub fn close_instance(&mut self, instance: u32) {
        let instance_slot = instance as usize % REACTOR_TASK_SLOTS;
        if self.current_by_instance[instance_slot].is_some_and(|binding| binding.instance == instance) {
            self.current_by_instance[instance_slot] = None;
        }
        for binding in &mut self.by_job {
            if binding.is_some_and(|binding| binding.instance == instance) {
                *binding = None;
            }
        }
    }
}

