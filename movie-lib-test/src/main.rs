extern crate movies_lib;
use movies_lib::movies::play;
fn main(){
    println!("indside main of test");
    play("Movie Test".to_string())
}