use std::{path::PathBuf, time::Duration};

use tokio::sync::mpsc;

use crate::{DiagramInput, Error, InternalMessage};

pub struct TaskCreateDiagram {
    pub input: mpsc::Receiver<InternalMessage<PathBuf>>,
    pub output: mpsc::Sender<InternalMessage<DiagramInput>>,
}

impl TaskCreateDiagram {
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            match msg {
                InternalMessage::Value(path_buf) => {
                    let dia = DiagramInput::check_file(path_buf);
                    let Some(dia) = dia else { continue };

                    self.output
                        .send_timeout(InternalMessage::Value(dia), Duration::from_millis(1000))
                        .await
                        .unwrap();
                }
                InternalMessage::EndExecution => {
                    self.output
                        .send_timeout(InternalMessage::EndExecution, Duration::from_millis(1000))
                        .await
                        .unwrap();
                    break;
                }
            };
        }

        Ok(())
    }
}
