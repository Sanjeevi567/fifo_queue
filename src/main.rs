mod fifo;
use fifo::Queue;
use std::time::Duration; //for printing element in slow for interactive at command line
use std::thread;
fn main(){
    //char queue
    let mut fifo_char=Queue::new();
    println!("Pushing char elements to the fifo queue\n");
    thread::sleep(Duration::from_secs(1));
    for i in 'a'..='e'{
        fifo_char.push(i);
    }
    println!("Printing char elements in FIFO order\n");
for _ in 0..5{
   println!("{:?} ",fifo_char.pop().unwrap());
   thread::sleep(Duration::from_millis(100));
}
   
    //First In First Out for f64
    let mut fifo_f64=Queue::new();
    println!("Pushing f64 elements to the fifo queue\n");
    thread::sleep(Duration::from_secs(1));
    for i in 1..6{
        fifo_f64.push(i as f64); //Rust won't convert implicitly
    }
    println!("Printing f64 elements in fifo order\n");
    for _ in 0..5{ //if you change the range more than 10 it will panic
        println!("{:#?}",fifo_f64.pop().unwrap()); //if you don't want to panic ,then dont't call unwrap 
       // since Option implements debug
       thread::sleep(Duration::from_millis(100));
          
    }

}