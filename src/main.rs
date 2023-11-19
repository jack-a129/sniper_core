mod gen;

pub fn kaeuta(word :Vec<String>,str :&str) -> String{
    let result = gen::make_kaeuta(str,&word);
    match result{
        Ok(n) => n,
        Err(_) => String::from("[ERROR] Kaeuta")
    }
}

fn main(){
    let str = "{ヘイ}ラッシャイ！ トロは{大トロ} {コハダ} アジ";
    let word = vec!["レイ".to_string(),"ババ".to_string(),"ネギトロ".to_string(),"りんご".to_string()];
    println!("{}",kaeuta(word, str));
}