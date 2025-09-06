use std::path::{Path, PathBuf};

use notify::{Event, EventKind, RecursiveMode, Watcher};
use tokio::sync::mpsc;

use crate::{Error, InternalMessage};

pub struct TaskWatch {
    pub input: mpsc::Receiver<InternalMessage<PathBuf>>,
    pub output: mpsc::Sender<InternalMessage<PathBuf>>,
    pub watch: bool,
}

impl TaskWatch {
    pub fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.blocking_recv() {
            match msg {
                InternalMessage::Value(_) => {
                    self.output.blocking_send(msg).unwrap();
                }
                InternalMessage::EndExecution => {
                    break;
                }
            }
        }

        if !self.watch {
            self.output
                .blocking_send(InternalMessage::EndExecution)
                .unwrap();
            return Ok(());
        }

        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher
            .watch(Path::new("."), RecursiveMode::Recursive)
            .unwrap();

        for res in rx {
            match res {
                Ok(event) => {
                    if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                        continue;
                    }
                    for path in event.paths {
                        self.output
                            .blocking_send(InternalMessage::Value(path))
                            .unwrap();
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }
}
