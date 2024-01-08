mod vulkan_debug;

use std::collections::HashSet;
use anyhow::{anyhow, Result};
use vulkanalia::{Instance, vk};
use vulkanalia::prelude::v1_2::*;
use vulkanalia::vk::{ExtDebugUtilsExtension, InstanceCreateFlags, make_version};
use vulkanalia::window as vk_window;
use winit::window::Window;
use crate::vulkan_core::vulkan_debug::debug_callback;


pub unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Test Program")
        .application_version(make_version(1, 0 ,0))
        .engine_name(b"FexEngine_Rust_Variant")
        .engine_version(make_version(1, 0, 0))
        .api_version(make_version(1, 2, 0));

    let available_layers =
        entry.enumerate_instance_layer_properties()?
        .iter().map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if !available_layers.contains(&VALIDATION_LAYER) { return Err(anyhow!("Validation Layer not available!")) }

    let layers = vec![VALIDATION_LAYER.as_ptr()];

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter().map(|e| e.as_ptr()).collect::<Vec<_>>();
    extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());

    let instance_info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions)
        .enabled_layer_names(&layers)
        .flags(InstanceCreateFlags::empty());

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .user_callback(Some(debug_callback));

    instance_info.push_next(&mut debug_info);

    let instance = entry.create_instance(&instance_info, None)?;

    // TODO: assign messenger to permanent variable
    let _messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;

    Ok(instance)
}

