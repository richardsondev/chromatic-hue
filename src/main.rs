use hueclient::Bridge;
use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use structopt::StructOpt;
use std::net::IpAddr;

#[derive(Debug, StructOpt)]
#[structopt(name = "chromatic-hue", about = "Chromatic Hue remotely syncs multiple Philips Hue lights to the same pattern.")]
struct Opt {
    /// IP of the bridge
    #[structopt(short = "i", long = "hue_bridge_ip")]
    bridge_ip: Option<String>,
    
    /// Username of the bridge
    #[structopt(short = "u", long = "hue_bridge_username")]
    bridge_username: Option<String>,
    
    /// Light IDs
    #[structopt(short = "l", long = "hue_light_ids")]
    light_ids: Option<String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let bridge_ip: IpAddr = opt.bridge_ip.or_else(|| {
        env::var("HUE_BRIDGE_IP")
            .ok()
            .and_then(|ip_str| ip_str.parse().ok())
    }).and_then(|ip_str| ip_str.parse().ok()).unwrap_or_else(|| {
        println!("Bridge IP is not provided");
        process::exit(1);
    });

    let bridge_username: String = opt.bridge_username.or_else(|| {
        env::var("HUE_BRIDGE_USERNAME").ok()
    }).unwrap_or_else(|| {
        println!("Bridge username is not provided");
        process::exit(1);
    });

    let light_ids: String = opt.light_ids.or_else(|| {
        env::var("HUE_LIGHT_IDS").ok()
    }).unwrap_or_else(|| {
        println!("Light IDs are not provided");
        process::exit(1);
    });

    loop {
        match run_animation(&bridge_ip, &bridge_username, &light_ids, None).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("An error occurred: {}. Restarting in 5 minutes...", err);
                tokio::time::sleep(Duration::from_secs(300)).await; // Wait for 5 minutes
            }
        }
    }
}

fn time_since_midnight() -> Duration {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    
    // Convert to seconds and get the number of seconds since midnight
    let in_seconds = since_the_epoch.as_secs();
    let seconds_since_midnight = in_seconds % (24 * 3600);

    // Convert it back to Duration
    Duration::from_secs(seconds_since_midnight)
}

async fn run_animation(bridge_ip: &IpAddr, bridge_username: &String, light_ids: &String, frame_limit: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the light IDs
    let light_ids: Vec<usize> = light_ids
        .split(',')
        .take(100) // Limit to 100 light IDs
        .map(|id| {
            id.parse().expect("Invalid light ID in HUE_LIGHT_IDS environment variable")
        })
        .collect();

    let bridge: Bridge = hueclient::Bridge::for_ip(*bridge_ip)
        .with_user(bridge_username);

    // Currently, we only support one pattern but we could switch here
    run_spectrum_pattern(frame_limit, light_ids, bridge).await;

    Ok(())
}

async fn run_spectrum_pattern(frame_limit: Option<usize>, light_ids: Vec<usize>, bridge: Bridge) -> Result<(), Box<dyn std::error::Error>> {
    let mut frame_count: usize = 0;
    let mut last_message: u64 = 0;

    // Main loop to change the light colors
    loop {
        if !frame_limit.is_none() {
            frame_count += 1;

            let should_break = match frame_limit {
                Some(limit) => frame_count > limit,
                None => false,
            };

            if should_break {
                break;
            }
        }

        let elapsed: Duration = time_since_midnight();
        let max_value: u64 = u16::MAX as u64;
        let seconds_in_day: u64 = 86400;
        let scaled_value: u64 = (elapsed.as_secs() * max_value) / seconds_in_day;
        let hue: u16 = scaled_value as u16;

        let cmd: hueclient::CommandLight = hueclient::CommandLight::default().with_sat(200).with_hue(hue);

        let should_emit_message: bool = last_message < elapsed.as_secs() && elapsed.as_secs() % 30 == 0;
        if should_emit_message {
            last_message = elapsed.as_secs();
        }

        for l in light_ids.iter() {
            if should_emit_message {
                println!("{:?}", bridge.set_light_state(*l, &cmd));
            }
            else {
                bridge.set_light_state(*l, &cmd).unwrap();
            }
        }

        tokio::time::sleep(Duration::from_millis(50)).await; // Adjust the delay as needed
    }

    Ok(())
}
