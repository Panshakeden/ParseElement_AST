#[derive(Debug,Clone)]

enum Token{
    StartTag(String),
    EndTag(String),
    Attribute(String,String),
    Text(String),
}

fn tokenizer(input:&str) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut chars= input.chars().peekable();

    while let Some(&token) = chars.peek(){
        if token == '<'{
            chars.next();

            if chars.peek() == Some(&'/'){
                chars.next();

                let tag_name:String= chars.by_ref().take_while(|&token| token != '>').collect();
                tokens.push(Token::EndTag(tag_name));
                chars.next();
            }else{
                let tag_name:String = chars.by_ref().take_while(|&token| token != '>' && token != ' ').collect();
                if chars.peek() == Some(&'>'){
                tokens.push(Token::StartTag(tag_name));
                chars.next();    
                }
            }
        }else{
            let tag_name :String = chars.by_ref().take_while(|&token| token != '<').collect();
            tokens.push(Token::Text(tag_name));
        }
    }
   tokens
}

fn main(){

}