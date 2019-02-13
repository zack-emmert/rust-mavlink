#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(feature = "std")]
use std::thread;
#[cfg(feature = "std")]
use std::env;
#[cfg(feature = "std")]
use std::time::Duration;

#[cfg(not(feature = "std"))]
fn main() {}

#[cfg(feature = "std")]
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mavlink-dump (tcpout|udpin|udpout|serial):(ip|dev):(port|baud)");
        return;
    }

    let vehicle = Arc::new(mavlink::connect(&args[1]).unwrap());
    
    vehicle.send(&mavlink::MavHeader::get_default_header(), &request_parameters()).unwrap();
    vehicle.send(&mavlink::MavHeader::get_default_header(), &request_stream()).unwrap();

    thread::spawn({
        let vehicle = vehicle.clone();
        move || {
            loop {
                vehicle.send(&mavlink::MavHeader::get_default_header(), &heartbeat_message()).ok();
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    loop {
        if let Ok(msg) = vehicle.recv() {
            println!("{:?}", msg);
        } else {
            break;
        }
    }
}

/// Create a heartbeat message
#[cfg(feature = "std")]
pub fn heartbeat_message() -> mavlink::combined::MavMessage {
    mavlink::combined::MavMessage::HEARTBEAT(mavlink::combined::HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: mavlink::combined::MavType::MAV_TYPE_QUADROTOR,
        autopilot: mavlink::combined::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: mavlink::combined::MavModeFlag::empty(),
        system_status: mavlink::combined::MavState::MAV_STATE_STANDBY,
        mavlink_version: 0x3,
    })
}

/// Create a message requesting the parameters list
#[cfg(feature = "std")]
pub fn request_parameters() -> mavlink::combined::MavMessage {
    mavlink::combined::MavMessage::PARAM_REQUEST_LIST(mavlink::combined::PARAM_REQUEST_LIST_DATA {
        target_system: 0,
        target_component: 0,
    })
}

/// Create a message enabling data streaming
#[cfg(feature = "std")]
pub fn request_stream() -> mavlink::combined::MavMessage {
    mavlink::combined::MavMessage::REQUEST_DATA_STREAM(mavlink::combined::REQUEST_DATA_STREAM_DATA {
        target_system: 0,
        target_component: 0,
        req_stream_id: 0,
        req_message_rate: 10,
        start_stop: 1,
    })
}