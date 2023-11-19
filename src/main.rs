mod gen;

fn main(){
    let word = vec!["りんご".to_string(),"西".to_string(),"関西".to_string()];
    let str = "{スシ}食いねェ！";
    let result = gen::make_kaeuta(str,&word);
    println!("{}",result.unwrap());
}