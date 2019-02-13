extern crate mavlink;

#[cfg(test)]
#[cfg(all(feature = "std", feature = "udp"))]
mod test_udp_connections {
    use std::thread;

    /// Create a heartbeat message
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

    /// Test whether we can send a message via UDP and receive it OK
    #[test]
    pub fn test_udp_loopback() {
        const RECEIVE_CHECK_COUNT: i32 = 3;

        let server = mavlink::connect("udpin:0.0.0.0:14551")
            .expect("Couldn't create server");


        // have the client send one heartbeat per second
        thread::spawn({
            move || {
                let client = mavlink::connect("udpout:127.0.0.1:14551")
                    .expect("Couldn't create client");
                loop {
                    client.send_default(&heartbeat_message()).ok();
                }
            }
        });

        //TODO use std::sync::WaitTimeoutResult to timeout ourselves if recv fails?
        let mut recv_count = 0;
        for _i in 0..RECEIVE_CHECK_COUNT {
            if let Ok(_msg) = server.recv() {
                recv_count += 1;
            } else {
                // one message parse failure fails the test
                break;
            }
        }
        assert_eq!(recv_count, RECEIVE_CHECK_COUNT);

    }

}