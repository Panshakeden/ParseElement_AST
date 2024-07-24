extern crate scraper;

use scraper::{Html, Selector};

fn parse_html_to_ast(html: &str) -> Html {
    Html::parse_document(html)
}

fn print_ast(element: scraper::ElementRef, depth: usize) {
    println!("<{}>", element.value().name());

    for (name, value) in element.value().attrs() {
        println!(" Attribute: {}=\"{}\"",  name, value);
    }

    for child in element.children() {
        if let Some(_element) = child.value().as_element() {
            print_ast(scraper::ElementRef::wrap(child).unwrap(), depth + 1);
        } else if let Some(text) = child.value().as_text() {
            println!("  Text: {:?}", text);
        }
    }
}

fn main() {
    let html = r#"
        <html>
            <head>
                <title>Parse Element</title>
            </head>
            <body>
                <div class="container">
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                   <p>Helo world</p>
                </div>
            </body>
        </html>
    "#;

    let html_document = parse_html_to_ast(html);
    let selector = Selector::parse("html").unwrap();

    for element in html_document.select(&selector) {
        print_ast(element, 0);
    }
}
