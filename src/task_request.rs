use reqwest::header;
use tokio::sync::mpsc;

use crate::{DiagramInput, DiagramOutput, Error, InternalMessage};

pub struct TaskRequest {
    pub input: mpsc::Receiver<InternalMessage<DiagramInput>>,
    pub output: mpsc::Sender<InternalMessage<DiagramOutput>>,
    pub endpoint: String,
}

impl TaskRequest {
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            match msg {
                InternalMessage::Value(dia) => {
                    let client = reqwest::Client::new();

                    for (out_type, path) in &dia.output_files() {
                        let endpoint = format!("{}{}", self.endpoint, dia.endpoint());
                        let out_type_header = out_type.header_accept();
                        let body = dia.content().to_string();

                        let res = client
                            .post(endpoint)
                            .header(header::ACCEPT, out_type_header)
                            .header(header::CONTENT_TYPE, "text/plain")
                            .body(body)
                            .send()
                            .await
                            .unwrap();

                        let response = res.bytes().await.unwrap();

                        let dia_out = DiagramOutput::new(path.clone(), response);

                        self.output
                            .send(InternalMessage::Value(dia_out))
                            .await
                            .unwrap();
                    }
                }
                InternalMessage::EndExecution => {
                    self.output
                        .send(InternalMessage::EndExecution)
                        .await
                        .unwrap();
                    break;
                }
            }
        }

        Ok(())
    }
}
