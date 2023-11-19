use std::collections::HashMap;

use serde::Deserialize;
use regex::Regex;

#[path="request.rs"]
mod request;

#[derive(Deserialize)]
struct Json{
    converted :String,
}

pub fn ruby_gen(str :String) -> Result<String,std::io::Error>{
    let key = request::keyget();
    let r = request::rubymake(key,&str);
    if let Ok(n) = r{
        let text = String::from_utf8_lossy(&n).to_string();
        let j :Result<Json,serde_json::Error> = serde_json::from_str(&text);
        if let Ok(n) = j{
            Ok(n.converted)   
        }else{
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "error"))
        }
    }else{
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "error"))
    }    
}

fn change_word(word :&str,wordvec :Vec<String>,before_word :&Vec<String>) -> Result<String,std::io::Error>{
    let i = word.len();
    let mut count = 0;
    for mut w in wordvec{
        w = w.replace("{", "");
        w = w.replace("}", "");
        if w.len() == i-2{
            return Ok(before_word[count].clone());
        }
        count += 1;
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "error"))
}

pub fn make_kaeuta(change :&str,word :&Vec<String>) -> Result<String,std::io::Error>{
    let re = Regex::new(r"\{.*?\}").unwrap();
    let mut ruby_word = Vec::new();
    for x in word{
        let v = ruby_gen(x.to_string())?;
        ruby_word.push(v);
    }
    let mut old = HashMap::new();
    let mut ruby_vec = Vec::new();
    for x in re.find_iter(change) {
        let r = ruby_gen(x.as_str().to_string())?; 
        old.insert(r.clone(), x.as_str());
        ruby_vec.push(r);
    }
    let mut change_after = change.clone().to_string();
    for m in ruby_vec{
        if m == ""{break;}
        if let Ok(after_word) = change_word(&m,ruby_word.clone(),&word){
            change_after = change_after.replace(old.get(&m).unwrap(), &after_word);
        }else{
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "error"));
        }
    }
    Ok(change_after)
}