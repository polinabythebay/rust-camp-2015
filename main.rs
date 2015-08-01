
//Hello world
fn main() {
    println!("Hello, world!");
}

//talk on iterators 
pub trait Iterator { //I can yield things
  type Item; //Things I yield
  //How I yield them
  fn next(&mut self) -> Option<Self::Item>;
}
