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
            let tag_name :String = chars.by_ref().take_while(|&tok| tok != '<').collect();
            tokens.push(Token::Text(tag_name));
        }
    }
   tokens
}


#[derive(Debug)]

enum Node{
    Element{
        tag_name: String,
        attributes: Vec<(String,String)>,
        children: Vec<Node>,
    },
    Text(String),
}

fn parser(tokens:&[Token])-> Node{
    let mut children = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next(){
          match token{
            Token::StartTag(tag_name) => {
                let mut attributes = Vec::new();

                while let Some(Token::Attribute(name,value))= iter.peek(){
                    attributes.push((name.clone(),value.clone()));
                    iter.next();
                }

                let mut nest_child = Vec::new();

                while let Some(next_token) = iter.peek(){
                    if let Token::EndTag(end_tag) = next_token{
                        if end_tag == tag_name{
                            iter.next();

                            break;
                        }
                    }

                    nest_child.push(parser(&[iter.next().unwrap().clone()]));

                }

                children.push(Node::Element{
                    tag_name:tag_name.clone(),
                    attributes,
                    children:nest_child,
                });
                
            }

            Token::Text(content) => {
                children.push(Node::Text(content.clone()));
            }
            _ => {}
          }
    }
    Node::Element{
        tag_name: "root".to_string(),
        attributes: Vec::new(),
        children,
    }

}

fn main(){


}