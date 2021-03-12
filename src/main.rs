#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;
use anyhow;
use log::LevelFilter;

#[derive(Deserialize, Clone)]
struct CustomEvent {}


#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn handler(_: CustomEvent, _c: lambda::Context) -> Result<CustomOutput, HandlerError> {
	info!("Incoming request...");
    Ok(CustomOutput {
        message: format!("Hello World!"),
    })
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    lambda!(handler);
    Ok(())
}
