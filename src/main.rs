/*
Terminal Graphics v 1.0.0

Do basic terminal rendering.

Author : Merwin
 */



//! # Terminal Graphics
//! 
//! A small package for rust which could do wonders when in right hands :)
//! For rendering things on terminal 
//! 






#![allow(dead_code)]
#![allow(unused)]



use std::array;
use std::cmp::Ordering;
use std::thread::{Thread, JoinHandle};
use colored::Colorize;
use std::io::{self, Write, Empty};
use std::collections::HashMap;
use std::{thread::sleep, time::Duration};
use terminal_size::{Width, Height, terminal_size};




// Terminal Basics

/// Get the size of the terminal
/// 
/// Returns (0,0) is couldnt get the size
pub fn get_terminal_size() -> (u16,u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        return (w, h);
    } else {
        return (0, 0);
    }
}

/// Clears the terminal for you
/// 
pub fn clear_terminal(){
    print!("{}[2J", 27 as char);
    io::stdout().flush().unwrap();
}

/// Set the cursor 
/// 
/// Takses in the row and column
pub fn set_cursor(row_: u16, column_: u16){
    print!("\x1b[{};{}H", row_, column_ );
    io::stdout().flush().unwrap();

}


// Basic printing stuff


/// Print a pixel on the terminal
/// 
/// The function takes a color, \[r,g,b]
pub fn print_pixel(color: [u8; 3]){
    print_color(color, String::from(" "));

}

/// Print a number of pixels
/// 
/// It takes the color as \[r,g,b] and how many pixels to be printed
pub fn print_pixels(color: [u8; 3], times: u32){
    for e in 0..times{
    print_color(color, String::from(" "));
    }
}

/// Remove a pixel from the current row
pub fn remove_pixel(){
    print_backspace();
}

/// Remove a number of pixels from current row
/// 
/// Takes in the number of pixels to be removed
pub fn remove_pixels(times: u32){
    for e in 0..times{
        print_backspace();
    }
}

/// Jump into the next row
pub fn new_row(){
    println!("");
}

/// Prints a tab
pub fn print_tab(){
    print!("\t");
    io::stdout().flush().unwrap();
}

/// Prints a String
pub fn print(t:String) {
    print!("{}", t);
    io::stdout().flush().unwrap();
}

/// Prints a String along with a newline
pub fn println(t:String) {
    println!("{}", t);
}

/// Prints a line 
/// 
/// Takes in the len
pub fn print_dash(times: usize){
    let mut t = String::from("-");
    t = t.repeat(times);
    print!("{}", t);
    io::stdout().flush().unwrap();
}

/// Prints a String which can be replaced 
/// 
/// Use the same function to replace the String
/// 
/// Takes in the String to print
pub fn print_fake(t:String){
    print!("\r{}", t);
    io::stdout().flush().unwrap();
}

/// Makes a backspace in terminal
pub fn print_backspace(){
    print!("\u{8}");
    io::stdout().flush().unwrap();
}


// Colored prints

/// Print a string with custom Background color
/// 
/// Takes in the color as \[r,b,g] and text as String
pub fn print_color(color: [u8; 3], text: String){
    print!("{}", text.on_truecolor(color[0], color[1], color[2]));
    io::stdout().flush().unwrap();
}

/// Print a string with custom Background color and newline
/// 
/// Takes in the color as [r,b,g] and text as String
pub fn println_color(color: [u8; 3], text: String){
    println!("{}", text.truecolor(color[0], color[1], color[2]));
}


// Logging

/// Print warning
/// 
/// Takes in a String
pub fn print_warning(warning : String){
    println!("  [=] {}", warning.truecolor(237, 174, 47));
}

/// Print Error
/// 
/// Takes in a String
pub fn print_error(error : String){
    println!("  [-] {}", error.truecolor(245, 49, 49));
}

/// Print Info
/// 
/// Takes in a String
pub fn print_info(info : String){
    println!("  [~] {}", info.truecolor(59, 245, 136));
}

/// Print Success
/// 
/// Takes in a String
pub fn print_success(success : String){
    println!("  [+] {}", success.truecolor(79, 255, 66));
}



// Animations

/// Spinner
/// 
/// Takes in the number of times it should spin
pub fn spinner(times : u32){
    for e in 0..times{

        print!("\r[ | ]");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(400));

        print!("\r[ / ]");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(400));

        print!("\r[ - ]");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(400));

        print!("\r[ \\ ]");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(400));

    }
    print!("\r[ | ]");
    io::stdout().flush().unwrap();
}


/// Makes a spin
/// 
/// The function can be reused to achive the spinner motion 
/// 
/// `println!("");` should be called after spinning is done
pub fn spin(){
    print!("\r[ | ]");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(400));

    print!("\r[ / ]");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(400));

    print!("\r[ - ]");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(400));

    print!("\r[ \\ ]");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(400));

}

/// Animated line
/// 
/// Takes in if dotter or not 
/// The number of time and color
pub fn animated_line(dotted : bool, times : u32, color : [u8;3]){ 
    let mut x;
    match dotted {
        true =>  x = String::from("┅").truecolor(color[0], color[1], color[2]),
        false => x = String::from("━").truecolor(color[0], color[1], color[2])
        };
    for e in 0..times{
        print!("{}",&mut x);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(300));
    }
}


/// Make a bar animation
/// 
/// The animation takes in number of times and color
pub fn bar_animation(times : u32, color : [u8;3]){
    let mut x = "|".truecolor(color[0], color[1], color[2]);
    for e in 0..times{
        print!("{}",&mut x );
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));        
    }
}


// Counters

/// Make a BarCounter which can be updated
///  
pub struct  BarCounter {
    color : [u8;3],
}

impl BarCounter{
    /// Takes a step 
    pub fn make_step(&self){
        print!("{}", "|".truecolor(self.color[0], self.color[1], self.color[2]));
        io::stdout().flush().unwrap();
    }
    
} 


/// Makes a LineCounter which can be updated
pub struct LineCounter {
    dotted : bool,
    color : [u8;3]
}

impl LineCounter{
    /// Makes a step 
    pub fn make_step(&self){ 
        let mut x;
        match self.dotted {
            true =>  x = String::from("┅").truecolor(self.color[0], self.color[1], self.color[2]),
            false => x = String::from("━").truecolor(self.color[0], self.color[1], self.color[2])
            };
        print!("{}",x);      
        io::stdout().flush().unwrap();
    }
}


/// Rectangular Obj which can be rendered on the terminal
pub struct RectangularObject {
    pub color : [u8;3],
    pub cords : [u16;2],
    pub height : u32,
    pub width : u32,
}


impl RectangularObject{
    ///  Draw / render it
    pub fn draw(&self){
        set_cursor(self.cords[1], self.cords[0]);
        for y in 0..self.height{
            set_cursor(( self.cords[1] + y as u16), self.cords[0]);
            for x in 0..self.width{
                print_pixel(self.color);
            }
            print(String::from("\n"));
        }
    }

    /// Removes the obj from the terminal
    pub fn remove(&self){
        set_cursor(self.cords[1], self.cords[0]);
        for y in 0..self.height{
            set_cursor(( self.cords[1] + y as u16), self.cords[0]);
            for x in 0..self.width{
                print(String::from(" "));
            }
            print(String::from("\n"));
        }
    }

    /// Move the object 
    pub fn move_obj(&mut self, x : u16, y : u16){
        self.remove();
        self.cords = [x,y];
        self.draw();
    }
    
    /// Move the object with a delay 
    pub fn move_obj_w_delay(&mut self, delay : u16, mut x : u16, mut y : u16){
    
        while x != self.cords[0] || y != self.cords[1]{
            self.remove();


            match self.cords[0].cmp(&x){
                Ordering::Less => self.cords[0] += 1,
                Ordering::Greater => self.cords[0] -= 1,
                Ordering::Equal => x = x
            };

            match self.cords[1].cmp(&y){
                Ordering::Less => self.cords[1] += 1,
                Ordering::Greater => self.cords[1] -= 1,
                Ordering::Equal => y = y
            };


            self.draw();

            std::thread::sleep(std::time::Duration::from_millis(delay as u64));
        }
    }

    /// Change the color of the obj
    pub fn change_color(&mut self, color : [u8;3]){
        self.color = color;
    }


}




/// Diamond shaped Obj which can be rendered on the terminal
pub struct DiamondObject {
    pub color : [u8;3],
    pub cords : [u16;2],
    pub radius : u32,
}


impl DiamondObject{
    ///  Draw / render it
    pub fn draw(&self){

        set_cursor(self.cords[1], self.cords[0]);

        for y in 1..(self.radius/2)+1{
            set_cursor(( self.cords[1] + y as u16),   self.cords[0] + ((self.radius - 2 * y)/2) as u16 );
            for e in 0..y*2{
            print_pixel(self.color);
            }
        
        let mut  z:u16 = 1;
        for y in (1..(self.radius/2)+1).rev(){
            set_cursor(( self.cords[1] + (self.radius/2) as u16) + z,   self.cords[0] + ((self.radius - 2 * y)/2) as u16 );
            z+=1;
            for e in 0..y*2{
                print_pixel(self.color);
        }
    }

        }
    }
    /// Removes the obj from the terminal
    pub fn remove(&self){
        set_cursor(self.cords[1], self.cords[0]);
        for y in 1..(self.radius/2)+1{
            set_cursor(( self.cords[1] + y as u16),   self.cords[0] + ((self.radius - 2 * y)/2) as u16 );
            for e in 0..y*2{
            print(String::from(" "));
            }
        
        let mut  z:u16 = 1;
        for y in (1..(self.radius/2)+1).rev(){
            set_cursor(( self.cords[1] + (self.radius/2) as u16) + z,   self.cords[0] + ((self.radius - 2 * y)/2) as u16 );
            z+=1;
            for e in 0..y*2{
            print(String::from(" "));
        }
    }}       
    }
    /// Move the object 
    pub fn move_obj(&mut self, x : u16, y : u16){
        self.remove();
        self.cords = [x,y];
        self.draw();
    }
    
    /// Move the object  with a delay
    pub fn move_obj_w_delay(&mut self, delay : u16, mut x : u16, mut y : u16){
    
        while x != self.cords[0] || y != self.cords[1]{
            self.remove();


            match self.cords[0].cmp(&x){
                Ordering::Less => self.cords[0] += 1,
                Ordering::Greater => self.cords[0] -= 1,
                Ordering::Equal => x = x
            };

            match self.cords[1].cmp(&y){
                Ordering::Less => self.cords[1] += 1,
                Ordering::Greater => self.cords[1] -= 1,
                Ordering::Equal => y = y
            };


            self.draw();

            std::thread::sleep(std::time::Duration::from_millis(delay as u64));
        }
    }
    
    /// Change the color of the object
    pub fn change_color(&mut self, color : [u8;3]){
        self.color = color;
    }


}




fn main(){
    
}
