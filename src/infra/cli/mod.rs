use quicli::prelude::*;
use structopt::StructOpt;

use crate::domain::use_cases::{CreateAppInput, CreateAppUseCase};
use crate::infra::{memory::MemoryAppRepo, term::StatefulTable};
use crate::prelude::*;

#[derive(Debug, StructOpt)]
enum Cli {
    ListApps,
    CreateApp(CreateApp),
}

#[derive(Debug, StructOpt)]
struct CreateApp {
    /// Name
    #[structopt(long = "name", short = "n")]
    name: String,
}

pub async fn run() -> CliResult {
    let args = Cli::from_args();

    match args {
        Cli::CreateApp(inner) => handle_create_app(inner).await?,
        Cli::ListApps => handle_list_apps().await?,
    };

    Ok(())
}

async fn handle_create_app(input: CreateApp) -> Result<()> {
    dbg!("creating");
    let use_case = CreateAppUseCase::new(MemoryAppRepo::new());
    let apps = use_case.run(CreateAppInput { name: input.name }).await?;
    let table = StatefulTable::new();
    table.render()?;

    // Pipe that to an output device
    Ok(())
}

async fn handle_list_apps() -> Result<()> {
    Ok(())
}
