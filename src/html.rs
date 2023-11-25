use axum::{
    extract::{Path, State},
    http::header,
    http::StatusCode,
    response::Html,
    response::{IntoResponse, Response},
};

pub async fn serve_page(
    State(state): State<crate::http::ServerState>,
    Path(path): Path<String>,
) -> Response {
    let filepath = state.config.root_dir.join(&path);
    if !filepath.exists() || !filepath.is_file() {
        return (StatusCode::NOT_FOUND, "Failed to found related files").into_response();
    }

    if filepath.extension().is_some_and(|ext| ext != "md") {
        return serve_asset(&filepath).await;
    }

    let content = match std::fs::read_to_string(&filepath) {
        Ok(content) => content,
        Err(err) => {
            log::error!("{}: Unable to open file {}", err, filepath.display());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to read file content",
            )
                .into_response();
        }
    };

    let html = match markdown::to_html_with_options(&content, &markdown::Options::gfm()) {
        Ok(html) => html,
        Err(err) => {
            log::error!(
                "{}: Fail to convert markdown file {} to html",
                err,
                filepath.display()
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to read convert markdown to html",
            )
                .into_response();
        }
    };

    // TODO: make the stylesheet costomizable
    let html = String::from(
        r##"<head>
          <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" crossorigin="anonymous">
          <link rel="stylesheet" href="https://esm.sh/@wooorm/starry-night@3/style/both.css">
          <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL" crossorigin="anonymous"></script>
          <style>
            p img {
              border-radius: 8px;
              max-width: 100%;
              height: auto;
            }
          </style>
        </head>
        <body>
        <div class="container-xxl px-4 px-xxl-2">
          <div class="content mx-auto my-5" style="max-width: 760px">"##,
    ) + html.as_str()
        + r##"
          </div>
        </div>
        <script type="module">
          import {
            common,
            createStarryNight
          } from 'https://esm.sh/@wooorm/starry-night@3?bundle'
          import {toDom} from 'https://esm.sh/hast-util-to-dom@4?bundle'

          const starryNight = await createStarryNight(common)
          const prefix = 'language-'

          const nodes = Array.from(document.body.querySelectorAll('code'))

          for (const node of nodes) {
            const className = Array.from(node.classList).find(function (d) {
              return d.startsWith(prefix)
            })
            if (!className) continue
            const scope = starryNight.flagToScope(className.slice(prefix.length))
            if (!scope) continue
            const tree = starryNight.highlight(node.textContent, scope)
            node.replaceChildren(toDom(tree, {fragment: true}))
          }
        </script>
        </body>
    "##;
    Html(html).into_response()
}

async fn serve_asset(filepath: &std::path::Path) -> Response {
    let content_type = match mime_guess::from_path(&filepath).first_raw() {
        Some(mime) => mime,
        None => {
            return (StatusCode::BAD_REQUEST, "MIME Type couldn't be determined").into_response()
        }
    };

    let file = match tokio::fs::File::open(filepath).await {
        Ok(file) => file,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = axum::body::StreamBody::new(stream);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str(content_type).unwrap(),
    );

    (headers, body).into_response()
}
