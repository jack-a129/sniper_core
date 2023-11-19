use curl::easy::Easy;
use curl::easy::List;

pub fn keyget() -> String{
    include_str!("../.env").to_string()
}
//
pub fn rubymake(key :String,str :&str) -> Result<Vec<u8>,std::io::Error>{
    let mut vec = Vec::new();
    let mut list = List::new();
    list.append("Content-Type: application/json")?;
    let mut handle = Easy::new();
    handle.http_headers(list)?;
    handle.url("https://labs.goo.ne.jp/api/hiragana")?;
    handle.post(true)?;
    let json = format!(r#"{{"app_id":"{}","sentence":"{}","output_type":"hiragana"}}"#,key,str);
    let post = json.as_bytes();
    handle.post_fields_copy(post)?;
    {
        let mut trance = handle.transfer();
        trance.write_function(|new_data| {
                vec.extend_from_slice(new_data);
                Ok(new_data.len())
        }).unwrap();
        trance.perform().unwrap();
    }
    Ok(vec)
}