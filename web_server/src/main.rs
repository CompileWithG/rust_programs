use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env::args;
use std::fs;
fn main() {
    let args1=args().collect::<Vec<String>>();
    let filename=&args1[1];
    let contents
    =fs::read_to_string(filename).expect("could not read the file");
    println!("this is the contents of the file: {}",contents);
    // showcase of multiple ownership
    {
        let a:Rc<i32>=Rc::new(12);
        let b= Rc::clone(&a);
        println!("this is the value of b: {}",b);
        let c= Rc::clone(&a);   
        println!("this is the value of c: {}",c);
        //the value of a ends after this loop
        println!("the scope of a,b,c ends here");
    }
    //println!("this is the value of a: {}",a);this will not work as a is not in scope
    
    println!("Hello, world!");
    let (tx,rx):(Sender<i32>,Receiver<i32>)=mpsc::channel();
    let a:Vec<i32>=vec![1,2,3,4,5,6,7,8,9,10];
    for i in a.iter(){
        println!("this is the current item in the iteration: {}",i);
    }
    let type11=Type1{
        value:5,
        vector:a
    };
    println!("this is the type1 struct: {:?}",type11);
    println!("we are now going to spawn a thread in rust");
    let a=thread::spawn(|| {
        for i in rx{
            println!("recieved :{}",i);
        }
        println!("this is the thread speaking");
    });
    //we then wait for the thread to complete using join handle
    tx.send(1).unwrap();
    tx.send(67).unwrap();
    a.join().unwrap();
}
#[derive(Debug)]   
struct Type1{
    value:i32,
    vector:Vec<i32>
}
//implementing iterator type for TYpe1 struct
impl Iterator for Type1{
    type Item=i32;
    fn next(&mut self)->Option<Self::Item>{
        println!("this is the enxt item");
        Some(32)
    }
}

