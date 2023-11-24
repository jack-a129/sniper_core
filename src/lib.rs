mod gen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rubys(word :&str) -> String{
    match gen::ruby_gen(word.to_string()){
        Ok(n) => n,
        Err(_) => String::from("[ERROR] ruby")
    }
}

#[wasm_bindgen]
pub fn kaeuta(word :Vec<String>,str :&str) -> String{
    let result = gen::make_kaeuta(str,&word);
    match result{
        Ok(n) => n,
        Err(_) => String::from("[ERROR] Kaeuta")
    }
}
#[cfg(test)]
mod tests{
    use crate::kaeuta;

    #[test]
    fn test(){
        let word = vec!["マグロ".to_string(),"エンガワ".to_string(),"サーモン".to_string(),"いくら".to_string(),"カルビ".to_string(),"イカ".to_string(),"玉子".to_string()];
        let str = "{蜂}の{羽音}が
        {チューリップ}の花に{消える}
        {微風}の中にひつそりと
        客を迎へた{赤い部屋}";
        assert_eq!("[ERROR] Kaeuta",kaeuta(word, str));
    }
}