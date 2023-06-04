use axum::response::Response;
use maud::html;

use crate::partials::layout::Layout;

pub async fn get(layout: Layout) -> Response {
    layout.render(html! {
        p { "Other page" }
    })
}
