use maud::{html, Markup};

pub fn header(title: &str) -> Markup {
    html! {
        header {
            nav {
                h1 { a href="/" data-turbo-frame="main" { (title) } }
                ul {
                    li { a href="/other" data-turbo-frame="main" { "other" } }
                    li { a href="/log" data-turbo-frame="main" { "log" } }
                }
            }
        }
    }
}
