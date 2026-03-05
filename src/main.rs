//! Discover Bluetooth devices and list them.

use bluer::{AdapterEvent, Address, DeviceEvent, DeviceProperty, DiscoveryFilter, DiscoveryTransport};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use std::{collections::HashSet, env};

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let filter_addr: HashSet<_> = env::args().filter_map(|arg| arg.parse::<Address>().ok()).collect();

    env_logger::init();
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let filter = DiscoveryFilter {
        transport: DiscoveryTransport::Auto,
        ..Default::default()
    };
    adapter.set_discovery_filter(filter).await?;
    adapter.discovery_filter().await;

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);

    let mut all_change_events = SelectAll::new();

    loop {
        tokio::select! {
            Some(device_event) = device_events.next() => {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {
                        if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
                            continue;
                        }

                        let device = match adapter.device(addr){
                            Ok(v) => v,
                            Err(e) => {
                                println!("    Error: {}", &e);
                                continue;
                            }
                        };
                        match device.rssi().await? {
                            None => {
                                println!("Device distance unknown");
                            }
                            Some(v) => {
                                println!("Rssi is {v}");
                                if v > -70 {
                                    println!("Device is close!")
                                } else {
                                    println!("Device is far!")
                                }
                            }
                        }

                        let change_events = device.events().await?.map(move |evt| (addr, evt));
                        all_change_events.push(change_events);
                    }
                    _ => (),
                }
            }
            Some((_addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {
                match property {
                    DeviceProperty::Rssi(v) => {
                        println!("Rssi is {v}");
                        if v > -70 {
                            println!("Device is close!")
                        } else {
                            println!("Device is far!")
                        }
                    }
                    _ => ()
                }
            }
            else => break
        }
    }

    Ok(())
}
