use clap::Parser;
use hueclient::Bridge;
use std::env;
use std::net::IpAddr;
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Parser)]
#[command(
    name = "chromatic-hue",
    about = "Chromatic Hue remotely syncs multiple Philips Hue lights to the same pattern."
)]
struct Opt {
    /// IP of the bridge
    #[arg(short = 'i', long = "hue_bridge_ip")]
    bridge_ip: Option<String>,

    /// Username of the bridge
    #[arg(short = 'u', long = "hue_bridge_username")]
    bridge_username: Option<String>,

    /// Light IDs
    #[arg(short = 'l', long = "hue_light_ids")]
    light_ids: Option<String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let bridge_ip: IpAddr = opt
        .bridge_ip
        .or_else(|| {
            env::var("HUE_BRIDGE_IP")
                .ok()
                .and_then(|ip_str| ip_str.parse().ok())
        })
        .and_then(|ip_str| ip_str.parse().ok())
        .unwrap_or_else(|| {
            println!("Bridge IP is not provided");
            process::exit(1);
        });

    let bridge_username: String = opt
        .bridge_username
        .or_else(|| env::var("HUE_BRIDGE_USERNAME").ok())
        .unwrap_or_else(|| {
            println!("Bridge username is not provided");
            process::exit(1);
        });

    let light_ids: String = opt
        .light_ids
        .or_else(|| env::var("HUE_LIGHT_IDS").ok())
        .unwrap_or_else(|| {
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

async fn run_animation(
    bridge_ip: &IpAddr,
    bridge_username: &String,
    light_ids: &String,
    frame_limit: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the light IDs
    let light_ids: Vec<String> = light_ids
        .split(',')
        .take(100) // Limit to 100 light IDs
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect();

    let bridge: Bridge = hueclient::Bridge::for_ip(*bridge_ip).with_user(bridge_username);

    // Currently, we only support one pattern but we could switch here
    run_spectrum_pattern(frame_limit, light_ids, bridge).await?;

    Ok(())
}

async fn run_spectrum_pattern(
    frame_limit: Option<usize>,
    light_ids: Vec<String>,
    bridge: Bridge,
) -> Result<(), Box<dyn std::error::Error>> {
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

        let (x, y) = hue_to_xy(hue);
        let cmd: hueclient::CommandLight = hueclient::CommandLight::default()
            .with_brightness(100.0)
            .with_xy(x, y);

        let should_emit_message: bool =
            last_message < elapsed.as_secs() && elapsed.as_secs() % 30 == 0;
        if should_emit_message {
            last_message = elapsed.as_secs();
        }

        for l in light_ids.iter() {
            let result = bridge.set_light_state(l, &cmd).await;
            if should_emit_message {
                println!("{:?}", result);
            }
            result?;
        }

        tokio::time::sleep(Duration::from_millis(50)).await; // Adjust the delay as needed
    }

    Ok(())
}

fn hue_to_xy(hue: u16) -> (f32, f32) {
    let h = hue as f32 / u16::MAX as f32 * 360.0;
    let c = 1.0;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let red = if r > 0.04045 {
        ((r + 0.055) / 1.055).powf(2.4)
    } else {
        r / 12.92
    };
    let green = if g > 0.04045 {
        ((g + 0.055) / 1.055).powf(2.4)
    } else {
        g / 12.92
    };
    let blue = if b > 0.04045 {
        ((b + 0.055) / 1.055).powf(2.4)
    } else {
        b / 12.92
    };
    let x_xyz = red * 0.649_926 + green * 0.103_455 + blue * 0.197_109;
    let y_xyz = red * 0.234_327 + green * 0.743_075 + blue * 0.022_598;
    let z_xyz = green * 0.053_077 + blue * 1.035_763;
    let sum = x_xyz + y_xyz + z_xyz;

    if sum == 0.0 {
        (0.0, 0.0)
    } else {
        (x_xyz / sum, y_xyz / sum)
    }
}
