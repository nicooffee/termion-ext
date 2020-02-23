#[allow(unused_imports)]

use std::io::Stdout;
use termion::screen::AlternateScreen;
use std::io::{ Write};
use termion::{cursor,color};

pub trait AdvWrite {
    fn w_go_str(&mut self,x:u16,y:u16,string: String);
    fn w_str_go(&mut self,x:u16,y:u16,string: String);
    fn w_go_str_color<T: color::Color,Q: color::Color>(&mut self,x:u16,y:u16,string: String,fg: T,bg: Q);
    fn w_str_go_color<T: color::Color,Q: color::Color>(&mut self,x:u16,y:u16,string: String,fg: T,bg: Q);
    fn w_line_h(&mut self,x: u16,y: u16,n: u16,c: char) ;
    fn w_line_v(&mut self,x: u16,y: u16,n: u16,c: char) ;
    fn w_box(&mut self,min_x: u16,min_y: u16,max_x: u16, max_y: u16,c_h: Option<char>,c_v: Option<char>);
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
    

    fn w_go_str_color<T: color::Color,Q: color::Color>(&mut self,x:u16,y:u16,string: String,fg: T,bg: Q){
        self.w_set_fg(fg);
        self.w_set_bg(bg);
        self.w_go_str(x,y,string);
        self.w_reset_color();
    }


    fn w_str_go_color<T: color::Color,Q: color::Color>(&mut self,x:u16,y:u16,string: String,fg: T,bg: Q){
        self.w_set_fg(fg);
        self.w_set_bg(bg);
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
        for i in 0..n {
            write!(self,"{}{}",cursor::Goto(x,y+i),c).unwrap();
        }
    }
    

    fn w_box(&mut self,min_x: u16,min_y: u16,max_x: u16, max_y: u16,c_h: Option<char>,c_v: Option<char>){
        let c_h = match c_h{Some(x) => x,None => '─'};//―
        let c_v = match c_v{Some(x) => x,None => '│'};//│
        self.w_line_h(min_x,min_y,max_x-min_x+1,c_h);
        self.w_line_v(min_x,min_y,max_y-min_y+1,c_v);
        self.w_line_h(min_x,max_y,max_x-min_x+1,c_h);
        self.w_line_v(max_x,min_y,max_y-min_y+1,c_v);
        write!(self,"{}{}",cursor::Goto(min_x,min_y),'┌').unwrap(); //╭
        write!(self,"{}{}",cursor::Goto(max_x,min_y),'┐').unwrap();//╮
        write!(self,"{}{}",cursor::Goto(min_x,max_y),'└').unwrap();//╰
        write!(self,"{}{}",cursor::Goto(max_x,max_y),'┘').unwrap();//╯
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

/*

┌───────────────────┐
│  ╔═══╗ Some Text  │▒
│  ╚═╦═╝ in the box │▒
╞═╤══╩══╤═══════════╡▒
│ ├──┬──┤           │▒
│ └──┴──┘           │▒
└───────────────────┘
*/