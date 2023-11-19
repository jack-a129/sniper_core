use std::collections::HashMap;
use levenshtein::levenshtein;

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

fn word_search(s :&str,wordvec :Vec<String>,before_word :&Vec<String>) -> Result<String,std::io::Error>{
    let mut count = 0;
    let mut loopcount = 0;
    let mut num = 9999;
    let mut is_word = false;
    for mut w in wordvec.clone(){
        w = w.replace("{", "");
        w = w.replace("}", "");
        let res = &s.replace("{", "").replace("}", "");
        let x = levenshtein(&res,&w);
        if x < num{
            is_word = true;
            count = loopcount;
            num = x;
        }
        loopcount += 1;
    }
    if is_word{
        Ok(before_word[count].clone())
    }else{
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "kaeuta_not_found"))
    }
}

fn change_word(word :&str,wordvec :&Vec<String>,before_word :&Vec<String>) -> Result<String,std::io::Error>{
    if let Ok(n) = word_search(word, wordvec.clone(),before_word){
        Ok(n)
    }else{
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "kaeuta_not_found"))
    }    
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
        if let Ok(after_word) = change_word(&m,&ruby_word,&word){
            change_after = change_after.replace(old.get(&m).unwrap(), &after_word);
        }else{
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "error"));
        }
    }
    Ok(change_after)
}