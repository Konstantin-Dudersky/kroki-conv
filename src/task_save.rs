use std::fs::write;

use tokio::sync::mpsc;

use crate::{DiagramOutput, Error, InternalMessage};

pub struct TaskSave {
    pub input: mpsc::Receiver<InternalMessage<DiagramOutput>>,
}

impl TaskSave {
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            match msg {
                InternalMessage::Value(dia) => write(dia.path, dia.content).unwrap(),
                InternalMessage::EndExecution => break,
            }
        }

        Ok(())
    }
}
