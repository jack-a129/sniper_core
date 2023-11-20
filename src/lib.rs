mod gen;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn kaeuta(word :Vec<String>,str :&str) -> String{
    let result = gen::make_kaeuta(str,&word);
    match result{
        Ok(n) => n,
        Err(_) => String::from("[ERROR] Kaeuta")
    }
}