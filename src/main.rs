use std::ffi::CStr;
use clap::Parser;
use clack_host::events::event_types::*;
use clack_host::factory::PluginDescriptor;
use clack_host::prelude::*;

const HOST_NAME: &str = "Clack Info Host";
const HOST_VENDOR: &str = "wymcg";
const HOST_URL: &str = "https://github.com/wymcg";
const HOST_VERSION: &str = env!("CARGO_PKG_VERSION");
const UNKNOWN_FIELD: &str = "Unknown";

mod args;

fn ctos(s: Option<&CStr>) -> Result<String, ()> {
    if let Some(s) = s {
        Ok(s.to_str().expect("Unable to convert CStr to &str!").to_string())
    } else {
        Err(())
    }
}

fn ctos_or_unknown(s: Option<&CStr>) -> String {
    ctos(s).unwrap_or(String::from(UNKNOWN_FIELD))
}

fn print_plugin_info(descriptor: PluginDescriptor) {

    // Basic plugin information
    println!("Name: {}", ctos_or_unknown(descriptor.name()));
    println!("Vendor: {}", ctos_or_unknown(descriptor.vendor()));
    println!("ID: {}", ctos_or_unknown(descriptor.id()));

    // Versions
    println!("Version: {}", ctos_or_unknown(descriptor.version()));

    // Other information
    if let Ok(description) = ctos(descriptor.description()) {
        println!("Description: {description}");
    }
    let feature_string: Vec<String> = descriptor.features().map(|s| ctos_or_unknown(Some(s))).collect();
    let feature_string: String = feature_string.join(", ");
    println!("Features: {feature_string}");

    // URLs
    if let Ok(url) = ctos(descriptor.url()) {
        println!("URL: {url}");
    }
    if let Ok(url) = ctos(descriptor.manual_url()) {
        println!("Manual: {url}");
    }
    if let Ok(url) = ctos(descriptor.support_url()) {
        println!("Support: {url}");
    }
}

fn main() {
    let args = args::ClapInfoHostArgs::parse();

    let host_info = HostInfo::new(HOST_NAME, HOST_VENDOR, HOST_URL, HOST_VERSION).expect("Unable to create host information!");

    let bundle = unsafe { PluginBundle::load(args.path) }.expect("Unable to load plugin bundle!");
    let plugin_factory = bundle.get_plugin_factory().expect("Unable to get plugin factory from bundle!");

    plugin_factory.plugin_descriptors().for_each(|pd| print_plugin_info(pd));
}
