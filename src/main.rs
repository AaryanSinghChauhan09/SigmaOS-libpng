use anyhow::Result;
use clap::Parser;
use libpng::libpng;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = libpng::Config { 
        hardware_acceleration: !cli.no_hw_accel, 
        ..Default::default() 
    };
    let mut component = libpng::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-libpng ready");
    Ok(())
}
