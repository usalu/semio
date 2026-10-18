//! 🔗️ Synapse and port editing of the flow host: connecting, the synapse ids, adding and removing input and output ports and disconnecting. See [`super`].

use super::*;

impl FlowHost {
    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, FlowCoreError> {
        let from_port = first_output_port(from_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        let to_port = first_input_port(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        self.connect_ports(from_id, &from_port, to_id, &to_port)
    }

    pub(in super::super) fn next_synapse_id(&mut self) -> String {
        loop {
            self.next_synapse_serial = self.next_synapse_serial.wrapping_add(1);
            let id = format!("s{}", self.next_synapse_serial);
            if self.host_snapshot.synapses.iter().all(|synapse| synapse.id != id) {
                return id;
            }
        }
    }

    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, FlowCoreError> {
        self.begin_change();
        if from_id == to_id {
            return Err(FlowCoreError::SelfConnection);
        }
        if !widget_has_output(from_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(from_id.to_string()));
        }
        if !widget_has_input(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(to_id.to_string()));
        }
        if !widget_has_declared_port(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::UnknownOutputPort(from_port.to_string()));
        }
        if !widget_has_declared_port(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::UnknownInputPort(to_port.to_string()));
        }
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        if self.host_snapshot.synapses.iter().any(|s| s.from == from_id && s.from_port == from_port && s.to == to_id && s.to_port == to_port) {
            return Err(FlowCoreError::ConnectionAlreadyExists);
        }
        let source_types = widget_port_value_types(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        let target_types = widget_port_value_types(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        if !port_value_types_compatible(&source_types, &target_types) {
            return Err(FlowCoreError::IncompatiblePortTypes {
                source: format!("{from_id}@{from_port}"),
                source_type: source_types.join(","),
                target: format!("{to_id}@{to_port}"),
                target_type: target_types.join(","),
            });
        }
        let displaced: Vec<String> = self.host_snapshot.synapses.iter().filter(|s| s.to == to_id && s.to_port == to_port).map(|s| s.id.clone()).collect();
        self.host_snapshot.synapses.retain(|s| !(s.to == to_id && s.to_port == to_port));
        let synapse_id = self.next_synapse_id();
        let synapse = SynapseSpec { id: synapse_id.clone(), from: from_id.to_string(), to: to_id.to_string(), from_port: from_port.to_string(), to_port: to_port.to_string() };
        let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(self.host_snapshot.synapses.len()).unwrap_or(u32::MAX), synapse: synapse.clone() });
        self.host_snapshot.synapses.push(synapse);
        self.note_leaves(displaced.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain([added]));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(synapse_id)
    }

    pub fn add_input_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxInputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.to_port = (old_index + 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        *input_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn remove_input_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinInputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownInputPort(port_id.to_string()));
        };
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.to == widget_id && synapse.to_port == port_id).map(|synapse| synapse.id.clone()).collect();
        self.host_snapshot.synapses.retain(|synapse| !(synapse.to == widget_id && synapse.to_port == port_id));
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.to_port = (old_index - 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *input_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse }))).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn add_output_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxOutputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.from_port = (old_index + 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        *output_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn remove_output_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinOutputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownOutputPort(port_id.to_string()));
        };
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id && synapse.from_port == port_id).map(|synapse| synapse.id.clone()).collect();
        self.host_snapshot.synapses.retain(|synapse| !(synapse.from == widget_id && synapse.from_port == port_id));
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.from_port = (old_index - 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *output_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse }))).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.host_snapshot.synapses.len();
        self.host_snapshot.synapses.retain(|s| s.id != synapse_id);
        if self.host_snapshot.synapses.len() == before {
            return Err(FlowCoreError::UnknownSynapse(synapse_id.to_string()));
        }
        self.note_leaves([FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.to_string() })]);
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }
}
