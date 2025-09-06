mod cli;
mod diagram_input;
mod diagram_output;
mod error;
mod internal_message;
mod list_all_files;
mod task_all_files;
mod task_create_diagram;
mod task_request;
mod task_save;
mod task_watch;

use std::path::PathBuf;

use clap::Parser;
use tokio::{sync::mpsc::channel, task::JoinSet};

use diagram_input::DiagramInput;
use diagram_output::DiagramOutput;
use error::Error;
use internal_message::InternalMessage;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cli = cli::Cli::parse();

    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    let (ch_tx_all_files_to_watch, ch_rx_all_files_to_watch) =
        channel::<InternalMessage<PathBuf>>(100);
    let (ch_tx_watch_to_dia_input, ch_rx_watch_to_dia_input) =
        channel::<InternalMessage<PathBuf>>(100);
    let (ch_tx_dia_input, ch_rx_dia_input) = channel::<InternalMessage<DiagramInput>>(10);
    let (ch_tx_dia_output, ch_rx_dia_output) = channel::<InternalMessage<DiagramOutput>>(10);

    let task = task_all_files::TaskAllFiles {
        output: ch_tx_all_files_to_watch,
    };
    task_set.spawn(task.spawn());

    let task = task_watch::TaskWatch {
        input: ch_rx_all_files_to_watch,
        output: ch_tx_watch_to_dia_input,
        watch: cli.command == cli::Commands::Watch,
    };
    task_set.spawn_blocking(|| task.spawn());

    let task = task_create_diagram::TaskCreateDiagram {
        input: ch_rx_watch_to_dia_input,
        output: ch_tx_dia_input,
    };
    task_set.spawn(task.spawn());

    let task = task_request::TaskRequest {
        input: ch_rx_dia_input,
        output: ch_tx_dia_output,
        endpoint: cli.url,
    };
    task_set.spawn(task.spawn());

    let task = task_save::TaskSave {
        input: ch_rx_dia_output,
    };
    task_set.spawn(task.spawn());

    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    info!("Program end")
}
