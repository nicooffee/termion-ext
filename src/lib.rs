#[allow(unused_imports)]

use std::io::Stdout;
use termion::screen::AlternateScreen;
use std::io::{ Write};
use termion::{cursor,color};

pub trait AdvWrite {
    fn w_go_str(&mut self,x:u16,y:u16,string: String);
    fn w_str_go(&mut self,x:u16,y:u16,string: String);
    fn w_go_str_color<T,Q: color::Color>(&mut self,x:u16,y:u16,string: String,fg: Option<T>,bg: Option<Q>);
    fn w_str_go_color<T,Q: color::Color>(&mut self,x:u16,y:u16,string: String,fg: Option<T>,bg: Option<Q>);
    fn w_line_h(&mut self,x: u16,y: u16,n: u16,c: char) ;
    fn w_line_v(&mut self,x: u16,y: u16,n: u16,c: char) ;
    fn w_box(&mut self,x: u16,y: u16,len_x: u16, len_y: u16,c_h: Option<char>,c_v: Option<char>);
    fn w_set_fg<T: color::Color>(&mut self,c: T);
    fn w_set_bg<T: color::Color>(&mut self,c: T);
    fn w_reset_color(&mut self);
}

impl<W: Write> AdvWrite for AlternateScreen<W>{
    
    fn w_go_str(&mut self,x:u16,y:u16,string: String){
        write!(self,"{}{}",cursor::Goto(x,y),string).unwrap();
    }


    fn w_str_go(&mut self,x:u16,y:u16,string: String) {
        write!(self,"{}{}",string,cursor::Goto(x,y)).unwrap();
    }
    

    fn w_go_str_color<T: color::Color>(&mut self,x:u16,y:u16,string: String,fg: Option<T>,bg: Option<T>){
        if let Some(color) = fg {self.w_set_fg(color)};
        if let Some(color) = bg {self.w_set_bg(color)};
        self.w_go_str(x,y,string);
        self.w_reset_color();
    }


    fn w_str_go_color<T: color::Color>(&mut self,x:u16,y:u16,string: String,fg: Option<T>,bg: Option<T>){
        if let Some(color) = fg {self.w_set_fg(color)};
        if let Some(color) = bg {self.w_set_bg(color)};
        self.w_str_go(x,y,string);
        self.w_reset_color();
    }


    fn w_line_h(&mut self,x: u16,y: u16,n: u16,c: char){
        let (max_x,_):(u16,u16) = termion::terminal_size().unwrap();
        if x+n>max_x {()}
        write!(self,"{}",cursor::Goto(x,y)).unwrap();
        for _ in 0..n {
            write!(self,"{}",c).unwrap();
        }
    }


    fn w_line_v(&mut self,x: u16,y: u16,n: u16,c: char) {
        let (_,max_y):(u16,u16) = termion::terminal_size().unwrap();
        if y+n>max_y {()}
        write!(self,"{}",cursor::Goto(x,y)).unwrap();
        for i in 1..=n {
            write!(self,"{}{}",c,cursor::Goto(x,y+i)).unwrap();
        }
    }
    

    fn w_box(&mut self,x: u16,y: u16,len_x: u16, len_y: u16,c_h: Option<char>,c_v: Option<char>){
        let c_h = match c_h{Some(x) => x,None => '―'};
        let c_v = match c_v{Some(x) => x,None => '│'};
        self.w_line_h(x,y,len_x,c_h);
        self.w_line_v(x,y,len_y,c_v);
        self.w_line_h(x,y+len_y-1,len_x,c_h);
        self.w_line_v(x+len_x-1,y,len_y,c_v);
        write!(self,"{}{}",cursor::Goto(x,y),'╭').unwrap();
        write!(self,"{}{}",cursor::Goto(x+len_x-1,y),'╮').unwrap();
        write!(self,"{}{}",cursor::Goto(x,y+len_y-1),'╰').unwrap();
        write!(self,"{}{}",cursor::Goto(x+len_x-1,y+len_y-1),'╯').unwrap();
    }

    
    fn w_set_fg<T: color::Color>(&mut self,c: T){
        write!(self,"{}",color::Fg(c)).unwrap();
    }


    fn w_set_bg<T: color::Color>(&mut self,c: T){
        write!(self,"{}",color::Bg(c)).unwrap();
    }


    fn w_reset_color(&mut self){
        write!(self,"{}",color::Fg(color::Reset)).unwrap();
        write!(self,"{}",color::Bg(color::Reset)).unwrap();
    }


}       
/*
     ╭――╮  
     │  │
     ╰——╯
*/