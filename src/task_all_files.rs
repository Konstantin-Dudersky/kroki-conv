use std::{path::PathBuf, time::Duration};

use tokio::sync::mpsc;

use crate::{Error, InternalMessage, list_all_files::list_all_files};

pub struct TaskAllFiles {
    pub output: mpsc::Sender<InternalMessage<PathBuf>>,
}

impl TaskAllFiles {
    pub async fn spawn(self) -> Result<(), Error> {
        let files = list_all_files(".").unwrap();

        for file in files {
            self.output
                .send_timeout(InternalMessage::Value(file), Duration::from_millis(1000))
                .await
                .unwrap();
        }

        self.output
            .send(InternalMessage::EndExecution)
            .await
            .unwrap();

        Ok(())
    }
}
