
use super::*;

fn memory_properties_with(types: &[(u32, vk::MemoryPropertyFlags)]) -> vk::PhysicalDeviceMemoryProperties {
    let mut properties = vk::PhysicalDeviceMemoryProperties::default();
    properties.memory_type_count = types.len() as u32;
    for (index, (heap_index, flags)) in types.iter().enumerate() {
        properties.memory_types[index] = vk::MemoryType { property_flags: *flags, heap_index: *heap_index };
    }
    properties
}

#[test]
fn finds_the_first_type_matching_both_the_bitmask_and_the_required_flags() {
    let properties = memory_properties_with(&[(0, vk::MemoryPropertyFlags::DEVICE_LOCAL), (0, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)]);
    let requirements = vk::MemoryRequirements { size: 256, alignment: 16, memory_type_bits: 0b11 };
    let found = find_memory_type(&properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE);
    assert_eq!(found, Some(1));
}

#[test]
fn refuses_a_type_the_requirements_bitmask_excludes_even_if_flags_match() {
    let properties = memory_properties_with(&[(0, vk::MemoryPropertyFlags::HOST_VISIBLE)]);
    // 🎯️ bit 0 excluded from the mask — the only host-visible type is not actually usable for
    // this resource, and the scan must not fall back to it anyway.
    let requirements = vk::MemoryRequirements { size: 256, alignment: 16, memory_type_bits: 0b0 };
    assert_eq!(find_memory_type(&properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE), None);
}

#[test]
fn returns_none_when_no_type_carries_every_required_flag() {
    let properties = memory_properties_with(&[(0, vk::MemoryPropertyFlags::DEVICE_LOCAL)]);
    let requirements = vk::MemoryRequirements { size: 256, alignment: 16, memory_type_bits: 0b1 };
    assert_eq!(find_memory_type(&properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT), None);
}

#[test]
fn grow_capacity_never_shrinks_and_rounds_up_to_a_power_of_two_floored_at_256() {
    assert_eq!(grow_capacity(0, 10), 256);
    assert_eq!(grow_capacity(256, 10), 256);
    assert_eq!(grow_capacity(256, 300), 512);
    assert_eq!(grow_capacity(1024, 300), 1024);
}

#[test]
fn as_bytes_reports_the_exact_byte_length_of_the_slice() {
    let values: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    assert_eq!(as_bytes(&values).len(), 16);
}
