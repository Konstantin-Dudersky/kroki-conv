use std::{fs::read_to_string, path::PathBuf};

use tracing::info;

pub enum DiagramKind {
    Graphviz,
    Structurizr,
}

pub struct DiagramInput {
    path: PathBuf,
    diagram_kind: DiagramKind,
    content: String,
}
impl DiagramInput {
    pub fn check_file(path: PathBuf) -> Option<Self> {
        let ext = path.extension()?.to_str()?;

        match ext {
            "dot" => {
                let content = read_to_string(&path).unwrap();

                info!("Found file: {}", path.display());

                Some(Self {
                    path,
                    diagram_kind: DiagramKind::Graphviz,
                    content,
                })
            }

            "dsl" => {
                let content = read_to_string(&path).unwrap();

                info!("Found file: {}", path.display());

                Some(Self {
                    path,
                    diagram_kind: DiagramKind::Structurizr,
                    content,
                })
            }

            _ => None,
        }
    }

    pub fn endpoint(&self) -> &'static str {
        match self.diagram_kind {
            DiagramKind::Graphviz => "/graphviz",
            DiagramKind::Structurizr => "/structurizr",
        }
    }

    pub fn output_files(&self) -> Vec<(OutputFormat, PathBuf)> {
        let mut path_svg = self.path.clone();
        path_svg.set_extension("svg");

        let mut path_png = self.path.clone();
        path_png.set_extension("png");

        let mut path_jpeg = self.path.clone();
        path_jpeg.set_extension("jpeg");

        let mut path_pdf = self.path.clone();
        path_pdf.set_extension("pdf");

        match self.diagram_kind {
            DiagramKind::Graphviz => {
                vec![
                    (OutputFormat::Svg, path_svg),
                    (OutputFormat::Png, path_png),
                    (OutputFormat::Jpeg, path_jpeg),
                    (OutputFormat::Pdf, path_pdf),
                ]
            }
            DiagramKind::Structurizr => {
                vec![
                    (OutputFormat::Svg, path_svg),
                    (OutputFormat::Png, path_png),
                    (OutputFormat::Pdf, path_pdf),
                ]
            }
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub enum OutputFormat {
    Svg,
    Png,
    Jpeg,
    Pdf,
}
impl OutputFormat {
    pub fn header_accept(&self) -> &'static str {
        match self {
            OutputFormat::Svg => "image/svg+xml",
            OutputFormat::Png => "image/png",
            OutputFormat::Jpeg => "image/jpeg",
            OutputFormat::Pdf => "application/pdf",
        }
    }
}
