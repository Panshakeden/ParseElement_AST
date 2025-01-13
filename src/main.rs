#[derive(Debug,Clone)]

enum Token{
    StartTag(String),
    EndTag(String),
    Attribute(String,String),
    Text(String),
}

fn tokenizer(input:&str) -> Vec<Token>{

}

fn main(){

}