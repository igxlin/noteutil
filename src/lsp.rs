use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;

#[derive(Debug)]
struct Service {
    config: crate::Config,
    documents: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            config: crate::Config::default(),
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Service {
    async fn replace_document_text(&self, uri: &str, text: String) {
        let mut documents = self.documents.write().await;
        documents.insert(String::from(uri), text);
    }

    async fn document_text(&self, uri: &str) -> Option<String> {
        let documents = self.documents.read().await;
        match documents.get(uri) {
            Some(text) => Some(text.clone()),
            None => None,
        }
    }

    async fn complete_notes(
        &self,
        comp_params: &CompletionParams,
        comp_items: &mut Vec<CompletionItem>,
    ) -> crate::Result<()> {
        let position = &comp_params.text_document_position.position;
        if position.character == 0 {
            return Ok(());
        }
        let uri = &comp_params.text_document_position.text_document.uri;
        let text = self.document_text(uri.as_str()).await;
        if text.is_none() {
            return Ok(());
        }

        let text = text.unwrap();

        let linenumber = usize::try_from(position.line)?;
        let lines: Vec<&str> = text.split("\n").collect();
        let line = lines.get(linenumber).unwrap();

        let index = usize::try_from(position.character)?;
        let mut should_compl = false;
        for i in (0..index - 1).rev() {
            let c = line.chars().nth(i).unwrap();
            if c.is_whitespace() {
                break;
            }
            if c == '[' {
                should_compl = true;
            }
        }

        if !should_compl {
            return Ok(());
        }

        let notes = crate::Note::all(&self.config.root_dir);
        notes.into_iter().for_each(|note| {
            let relative_path =
                pathdiff::diff_paths(note.path.as_path(), Path::new(uri.path()).parent().unwrap())
                    .unwrap();
            comp_items.push(CompletionItem {
                label: format!("[{}]({})", note.title, relative_path.display()).to_string(),
                ..CompletionItem::default()
            });
        });

        Ok(())
    }
}

#[derive(Debug)]
struct Backend {
    client: tower_lsp::Client,
    service: Service,
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn completion(
        &self,
        comp_params: CompletionParams,
    ) -> jsonrpc::Result<Option<CompletionResponse>> {
        let mut comp_items = Vec::new();
        let _ = self
            .service
            .complete_notes(&comp_params, &mut comp_items)
            .await;
        Ok(Some(CompletionResponse::Array(comp_items)))
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let text = params.content_changes.first();
        if text.is_none() {
            return;
        }

        let filepath = params.text_document.uri.as_str();
        let text = text.unwrap().text.clone();
        self.service.replace_document_text(filepath, text).await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text.clone();
        let filepath = params.text_document.uri.as_str();
        self.service.replace_document_text(filepath, text).await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}

pub async fn serve(ctx: crate::Context) {
    log::info!("LSP server started.");
    let (service, socket) = tower_lsp::LspService::new(|client| Backend {
        client: client,
        service: Service {
            config: ctx.config.clone(),
            documents: Arc::new(RwLock::new(HashMap::new())),
        },
    });
    tower_lsp::Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn replace_document_text() -> crate::Result<()> {
        let text = String::from("text");
        let mut service = Service::default();
        tokio::spawn(async {
            service.replace_document_text("uri", text.clone()).await;
        });
        tokio::spawn(async {
            assert_eq!(service.documents.read().await.get("uri").unwrap(), "text");
        });
        tokio::join!();
        Ok(())
    }
}
