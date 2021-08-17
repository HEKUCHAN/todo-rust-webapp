use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;
use askama::Template;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,    
}

// エラーをまとめる enum を定義する
// actix_web::ResponseError として使うために derive マクロで Debug を付与している必要がある
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

// actix_web::ResponseError を MyError に実装する
// 今回はデフォルトの実装をそのまま使うから、新たに実装するものはない
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();

    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });

    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

    let html = IndexTemplate { entries };
    let response_body = html.render()?;

    // HttpResponse::Ok() はステータスコード 200 を持つ HttpResponseBuilderという構造体を返す。
    // HttpResponseBuilderの body() という関数にレスポンスのボディを渡すとHttpResponseが返ってくる。
    // 戻り値の値が Result なので Ok で包みます。
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}