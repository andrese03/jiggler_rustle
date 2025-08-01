use clap::Parser;
use enigo::{Enigo, Key, Keyboard, Mouse, Settings};
use rand::Rng;
use std::time::Duration;
use tokio::time;

#[derive(Parser)]
#[command(name = "jiggler")]
#[command(
    about = "A tiny 🧰 Rust-powered companion that gently reminds your machine you're still around 👀. Perfect for those deep focus sessions where stepping away is not really an option... or at least, that's what the system thinks 😏\n\nCross-platform. Lightweight. Practically invisible. Just a little... rustle now and then. 🌬️🖱️"
)]
#[command(version = "0.1.1")]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "60",
        help = "Interval in seconds between actions"
    )]
    interval: u64,

    #[arg(short, long, help = "Use key press instead of mouse movement")]
    key_mode: bool,

    #[arg(short, long, help = "Run once and exit")]
    once: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("🐭 Jiggler started!");
    println!(
        "Mode: {}",
        if cli.key_mode {
            "Key press"
        } else {
            "Mouse movement"
        }
    );
    println!("Interval: {} seconds", cli.interval);

    if cli.once {
        perform_action(cli.key_mode)?;
        println!("✅ Action performed once. Exiting.");
        return Ok(());
    }

    println!("Press Ctrl+C to stop...\n");

    let mut interval = time::interval(Duration::from_secs(cli.interval));

    loop {
        interval.tick().await;

        match perform_action(cli.key_mode) {
            Ok(_) => {
                let action = if cli.key_mode {
                    "key pressed"
                } else {
                    "mouse moved"
                };
                println!("🎯 Action performed: {}", action);
            }
            Err(e) => {
                eprintln!("❌ Error performing action: {}", e);
            }
        }
    }
}

fn perform_action(key_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&Settings::default())?;

    if key_mode {
        press_safe_key(&mut enigo)?;
    } else {
        jiggle_mouse(&mut enigo)?;
    }

    Ok(())
}

fn jiggle_mouse(enigo: &mut Enigo) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    // Get current mouse position (if possible, otherwise use relative movement)
    let dx = rng.gen_range(-5..=5);
    let dy = rng.gen_range(-5..=5);

    // Move mouse by small random amount
    enigo.move_mouse(dx, dy, enigo::Coordinate::Rel)?;

    // Move back to roughly original position
    std::thread::sleep(Duration::from_millis(50));
    enigo.move_mouse(-dx, -dy, enigo::Coordinate::Rel)?;

    Ok(())
}

fn press_safe_key(enigo: &mut Enigo) -> Result<(), Box<dyn std::error::Error>> {
    // Use Shift key as it's safe and won't interfere with most applications
    // Shift is available on all platforms and generally safe to press
    enigo.key(Key::Shift, enigo::Direction::Press)?;
    std::thread::sleep(Duration::from_millis(50));
    enigo.key(Key::Shift, enigo::Direction::Release)?;

    Ok(())
}
