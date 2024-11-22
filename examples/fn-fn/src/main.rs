/*
 * Fn trait, called multiple times with only inmmutable access
 * FnMut trait
 * FnOnce trait is for functions that can only be called once
 *
 */
use std::process::Command;

struct MyCommand {}

impl MyCommand {
    pub fn do_this() {
        Command::new("sleep")
        .arg("0.1")
        .status()
        .expect("Sleep command fails");
    }

    pub fn do_that() {
        Command::new("sleep")
        .arg("0.2")
        .status()
        .expect("Sleep command fails");
    }
    
    pub fn finish_setup() {
        Command::new("sleep")
        .arg("0.3")
        .status()
        .expect("Sleep command fails");
    }
}


struct Installation<'a> {
    command_list: Vec<(&'a str, Box<dyn Fn()>)>, 
}

impl Installation<'_> {
   fn hande(&mut self) {
       let c_total: u8 = self.command_list.len() as u8;
       for c_done in 0..c_total {
               let percent_done: u8 = c_done as u8 * 100 / c_total;
               let item = &self.command_list[c_done as usize];
               println!("{} {}%", item.0, percent_done);
               item.1();
       }
   }
} 

fn f1(i: i32) -> i32 { i * 2 }

fn f2(i: i32) -> i32 { i * 4 }

fn main() {
    let command_list: Vec<(&str, Box<dyn Fn()>)> = vec![
            ("Do this..", Box::new(MyCommand::do_this)),
            ("Do that..", Box::new(MyCommand::do_that)),
            ("Finish..", Box::new(MyCommand::finish_setup)),
    ];
    
    Installation {
        command_list: command_list,
    };

    // 
    let arr: Vec<&dyn Fn(i32) -> i32> = vec![&f1, &f2];

    for f in &arr {
        println!("{}", (f)(1)); // Fn() closures
    }

    // mutable FnMut() closures
    let p1 = &mut f1;
    let p2 = &mut f2;
    let mut m_arr: Vec<&mut dyn FnMut(i32) -> i32> = vec![p1, p2];

    for f in &mut m_arr {
        println!("{}", (f)(2));
    }
}

