use std::fmt::Write;

use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Publish Newsletter</title>
                </head>
                <body>
                    {msg_html}
                    <form action="/admin/newsletters" method="post">
                        <label>
                            Title
                            <input type="text" placeholder="Enter title" name="title">
                        </label>
                        <br><br>
                        <label>
                            HTML<br>
                            <textarea placeholder="Enter html" name="html" rows="4" cols="50"></textarea>
                        </label>
                        <br><br>
                        <label>
                            Text<br>
                            <textarea placeholder="Enter text" name="text" rows="4" cols="50"></textarea>
                        </label>
                        <br><br>
                        <button type="submit">Publish Newsletter</button>
                    </form>
                    <p><a href="/admin/dashboard">&lt;- Back</a></p>
                </body>
            </html>
            "#
        ))
}
