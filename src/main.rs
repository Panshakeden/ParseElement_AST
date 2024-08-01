// extern crate scraper;

// use scraper::{Html, Selector};

// fn parse_html_to_ast(html: &str) -> Html {
//     Html::parse_document(html)
// }

// fn print_ast(element: scraper::ElementRef, depth: usize) {
//     println!("<{}>", element.value().name());

//     for (name, value) in element.value().attrs() {
//         println!(" Attribute: {}=\"{}\"",  name, value);
//     }

//     for child in element.children() {
//         if let Some(_element) = child.value().as_element() {
//             print_ast(scraper::ElementRef::wrap(child).unwrap(), depth + 1);
//         } else if let Some(text) = child.value().as_text() {
//             println!("  Text: {:?}", text);
//         }
//     }
// }

// fn main() {
//     let html = r#"
//         <html>
//             <head>
//                 <title>Parse Element</title>
//             </head>
//             <body>
//                 <div class="container">
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                    <p>Helo world</p>
//                 </div>
//             </body>
//         </html>
//     "#;

//     let html_document = parse_html_to_ast(html);
//     let selector = Selector::parse("html").unwrap();

//     for element in html_document.select(&selector) {
//         print_ast(element, 0);
//     }
// }



use std::collections::HashMap;

#[derive(Debug)]
struct Element {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

#[derive(Debug)]
enum Node {
    Element(Element),
    Text(String),
}

fn parse_html_to_ast(html: &str) -> Node {
    let mut elements = vec![];
    let mut text = String::new();

    let mut inside_tag = false;
    let mut current_tag = String::new();
    let mut current_attrs = HashMap::new();
    let mut current_text = String::new();
    let mut tag_name = String::new();
    let mut is_closing_tag = false;

    for c in html.chars() {
        if c == '<' {
            inside_tag = true;
            if !current_text.trim().is_empty() {
                elements.push(Node::Text(current_text.clone()));
            }
            current_text.clear();
        } else if c == '>' {
            inside_tag = false;
            if is_closing_tag {
                if let Some(Node::Element(mut parent_element)) = elements.pop() {
                    parent_element.children.push(Node::Text(current_text.clone()));
                    current_text.clear();
                    elements.push(Node::Element(parent_element));
                }
            } else {
                let element = Element {
                    name: tag_name.clone(),
                    attributes: current_attrs.clone(),
                    children: vec![],
                };
                if let Some(Node::Element(mut parent_element)) = elements.pop() {
                    parent_element.children.push(Node::Element(element));
                    elements.push(Node::Element(parent_element));
                } else {
                    elements.push(Node::Element(element));
                }
            }
            current_tag.clear();
            current_attrs.clear();
            tag_name.clear();
            is_closing_tag = false;
        } else if inside_tag {
            if c == '/' {
                is_closing_tag = true;
            } else if c == ' ' && tag_name.is_empty() {
                tag_name = current_tag.clone();
                current_tag.clear();
            } else if c == '=' {
                current_attrs.insert(current_tag.clone(), current_text.clone());
                current_tag.clear();
                current_text.clear();
            } else {
                current_tag.push(c);
            }
        } else {
            current_text.push(c);
        }
    }

    if !elements.is_empty() {
        elements.remove(0)
    } else {
        Node::Text(html.to_string())
    }
}

// Function to print the AST
fn print_ast(node: &Node, depth: usize) {
    match node {
        Node::Element(element) => {
            println!("<{}>", element.name);
            for (name, value) in &element.attributes {
                println!("Attribute: {}=\"{}\"", name, value);
            }
            for child in &element.children {
                print_ast(child, depth + 1);
            }
        }
        Node::Text(text) => {
            println!("Text: {:?}",text);
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
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                   <p>Hello world</p>
                </div>
            </body>
        </html>
    "#;

    let ast = parse_html_to_ast(html);
    print_ast(&ast, 0);
}
