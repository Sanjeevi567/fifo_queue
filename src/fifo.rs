use std::mem::swap;
#[derive(Clone,Debug)]
    pub struct Queue<T>{
        old:Vec<T>,
        young:Vec<T>,
    }
    impl<T:Clone +  std::fmt::Debug> Queue<T>{
        pub fn new()->Self{
            Self{
                old:Vec::with_capacity(20),//old:Vec::new(),
                young:Vec::with_capacity(20),//young:Vec::new(),
            }
        }
        fn capacity(&mut self)->usize{
              if self.old.capacity() == self.young.capacity(){
                self.old.capacity()
              }
              else{
                0
              }
            
        }
        pub fn push(&mut self,elt:T){
            self.young.push(elt);
        }
        fn is_empty_old(&mut self)->bool{
            self.old.len() == 0
        }
        fn is_empty_young(&mut self)->bool{
            self.young.len()==0
        }
        pub fn pop(&mut self)->Option<T>{
             if self.old.is_empty(){
             
             if self.young.is_empty() {
                    return None;
                }
            
            
            //self.old=self.young.clone();//Allocating new buffer i.e syscall
            //self.young.clear(); //if you are not doing this you are cloning existing and new values together and 
            //test will fail
           swap(&mut self.young,&mut self.old); //this is efficient than previous because we don't introduce 
            //any allocation  
            self.old.reverse();
            }
            self.old.pop()
        }
    }

#[test]
fn check_queue(){
 let mut queue = Queue::new();
  assert_eq!(queue.capacity(),20);
 queue.push(56);
 queue.push(67);
  assert_eq!(queue.capacity(),20);
 assert!(!queue.is_empty_young());
 for _ in 0..2{
    queue.pop();
 }
 assert!(queue.is_empty_young());
 
}
#[test]
fn push_pop_test(){
    let mut fifo= Queue::new();
    fifo.push('z');
    fifo.push('a');
    assert_eq!(fifo.pop(),Some('z')); //first in first out
    assert_eq!(fifo.pop(),Some('a')); //last in last out
}

