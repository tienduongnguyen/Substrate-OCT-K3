
// Bài tập cho trait
// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau

// use std::iter::Iterator;
// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// impl Iterator for Fibonacci {
//     type Item = u32;

//     fn next(&mut self) -> Option<u32> {
//         let new_next = self.curr + self.next;
//         self.curr = self.next;
//         self.next = new_next;
//         Some(self.curr)
//     }
// }

// impl Fibonacci {
//     fn new() -> Fibonacci {
//         Fibonacci {
//             curr: 0,
//             next: 1,
//         }
//     }
// }
    
// fn main() {
//     for number in Fibonacci::new().take(20){
//         println!("{}", number);
//     }
// }


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl<'a> fmt::Display for StrDisplayable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_foo = StrDisplayable(vec);
        println!("{}",vec_foo);
}