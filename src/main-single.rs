/*
two threads expanding a tree at the same time.
thread routine: 
- Select Node
	- Get an explored children
	- If all explored, pick first.
- Generate children
- Append children
- Stop after certain layer reached

Testing Protocol:
- Expected result: 
	- Binary Tree with one side explored to level N 

- Execute with one thread, mark time.
- Execute with multiple threads, mark time.
*/

use rand::prelude::*;

#[derive(Debug)]
struct A {
    n: i32,
    v: Vec<Box<A>>
}
struct B {
    a: Box<A>
}
impl A {
    pub fn select<'a> (&'a mut self) -> Result<&'a mut A, &'a mut A> {
        if self.v.is_empty() {
            Err(self)
        } else {
            let out: &mut Box<A> = self.v.iter_mut().min_by(|a, b| a.n.cmp(&b.n)).unwrap();
            Ok(out)
        }
    }
}

fn routine (mref_root: &mut B) {
    let mref_child = {
        let mut r: &mut A = &mut mref_root.a;
        loop {
            match A::select(r) {
                Ok(o)   => r = o,
                Err(o)  => { r = o; break; }
            };
        }
        let cr = &*r;
        println!("{:?}", cr); 
        r
    };
    
    let children = { // Gen children
        let mut rng = rand::thread_rng();
        let mut children = vec![];
        for _ in 0..2 {
            children.push( Box::new( A{
                n: (rng.gen::<f64>() * 100.0) as i32,
                v: vec![]
            }));
        }
        children
    };

    { // Add children
        mref_child.v = children;
    }
}

fn main () {
    let mut root = B { a: Box::new(A{ n: 0, v: vec![] }) };

    let mref_root = &mut root;
    for _ in 0..10 {
        routine(mref_root)
    }

    println!("result: {:?}", mref_root.a);
}
