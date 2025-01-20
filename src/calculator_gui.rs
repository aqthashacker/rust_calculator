use fltk::{
    app,  
    button::*, 
    enums::{Align,Color, Event, FrameType, Key, Shortcut, Font}, 
    frame::*,  
    prelude::*, 
    window};

    use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Ops {// Enum to represent different operations
    None,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    CE,
    PlusMinus,
    Back,
}

#[derive(Debug, Copy, Clone)]
enum Message {// Enum to represent different messages passed between components
    Number(i32),
    Op(Ops),
    Dot,
}

struct MyButton {
    btn: Button,
}

impl MyButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32,title: &'static str) -> MyButton {
        // Custom button struct with additional settings and event handling
        let mut b = MyButton {
            btn: Button::new(x, y, w, h, title),
        };// Set button appearance based on its title
        // Add event handlers for specific button behavior
        // Set shortcuts for specific buttons
        // Set colors and fonts
        b.set_label_size(24);
        b.set_frame(FrameType::FlatBox);
        match title {
            "CE" => {
                b.set_color(Color::from_hex(0xC8974B));
                b.set_selection_color(Color::from_hex(0xd50000));
                b.set_shortcut(Shortcut::None | Key::Delete);
                b.handle(move |b, ev| match ev {
                    Event::Enter => {
                        b.set_color(Color::from_hex(0xC8974B).darker());
                        b.redraw();
                        true
                    }
                    Event::Leave => {
                        b.set_color(Color::from_hex(0xC8974B));
                        b.redraw();
                        true
                    }
                    _ => false,
                });
            }
            "x" | "÷" | "+" | "-" | "=" | "±" | "@<-" => {
                b.set_color(Color::from_hex(0xffee58));
                b.set_selection_color(Color::from_hex(0xffee58).lighter());
                b.set_label_color(Color::Black);
                b.handle(move |b, ev| match ev {
                    Event::Enter => {
                        b.set_color(Color::from_hex(0xffee58).darker());
                        b.redraw();
                        true
                    }
                    Event::Leave => {
                        b.set_color(Color::from_hex(0xffee58));
                        b.redraw();
                        true
                    }
                    _ => false,
                });
                let shortcut = if title == "x" {
                    '*'
                } else if title == "÷" {
                    '/'
                } else {
                    title.chars().next().unwrap()
                };
                b.set_shortcut(Shortcut::None | shortcut);
                if shortcut == '@' {
                    b.set_shortcut(Shortcut::None | Key::BackSpace);
                }
                if shortcut == '=' {
                    b.set_shortcut(Shortcut::None | Key::Enter);
                }
            }
            _ => {
                
                b.set_color(Color::from_hex( 0x424242));
                b.set_label_color(Color::White);
                b.set_selection_color(Color::from_hex(0x1b1b1b));
                b.set_shortcut(Shortcut::None | title.chars().next().unwrap());
                b.handle(move |b, ev| match ev {
                    Event::Enter => {
                        b.set_color(Color::from_hex(0x2b2b2b));
                        b.redraw();
                        true
                    }
                    Event::Leave => {
                        b.set_color(Color::from_hex(0x424242));
                        b.redraw();
                        true
                    }
                    _ => false,
                });
            }
        }
        b
    }
}

impl Deref for MyButton {
    // Implementing Deref trait for MyButton struct to access Button methods
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl DerefMut for MyButton {
    // Implementing DerefMut trait for MyButton struct to access Button methods mutably
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}

fn main(){
    
    // Create a window buttons, frames, and other UI elements
    // Set properties, colors, fonts, event handlers for UI elements
    // Handle messages received from buttons and update the display accordingly

    // Enter the event loop and handle events until the application is closed
    let mut is_negative = false;
    let a = app::App::default();
    let mut win = window::Window::default().with_size(381, 459).with_label("Rust Calculator");
    win.set_color(Color::Black);


    let mut operation = Ops::None;
    let mut txt = String::with_capacity(14);

    let mut old_val = String::from("0");
    let mut new_val: String;

    let mut frame = Frame::new(20, 30, 350, 70, "0").with_align(Align::Right | Align::Inside);
    frame.set_frame(FrameType::FlatBox);
    frame.set_label_color(Color::Black);
    frame.set_label_size(36);
    frame.set_color(Color::White);
    

    let mut btn_ce = MyButton::new(20, 120, 80, 48, "CE");
    btn_ce.set_label_font(Font::HelveticaBold);
    btn_ce.clear_visible_focus();
    btn_ce.set_color(Color::from_rgb(191,148,55));
    btn_ce.set_selection_color(Color::from_rgb(191,148,55));
    let mut btn_plusminus = MyButton::new(110, 120, 80, 48, "±");
    btn_plusminus.set_label_font(Font::HelveticaBold);
    let mut btn_back = MyButton::new(200, 120, 80, 48, "@<-");
    btn_back.set_label_font(Font::HelveticaBold);
    let mut btn_div = MyButton::new(290, 120, 80, 48, "÷");
    btn_div.set_label_font(Font::HelveticaBold);
    let mut btn_7 = MyButton::new(20, 180, 80, 48, "7");
    btn_7.set_label_font(Font::HelveticaBold);
    let mut btn_8 = MyButton::new(110, 180, 80, 48, "8");
    btn_8.set_label_font(Font::HelveticaBold);
    let mut btn_9 = MyButton::new(200, 180, 80, 48, "9");
    btn_9.set_label_font(Font::HelveticaBold);
    let mut btn_mul = MyButton::new(290, 180, 80, 48, "x");
    btn_mul.set_label_font(Font::HelveticaBold);
    let mut btn_4 = MyButton::new(20, 240, 80, 48, "4");
    btn_4.set_label_font(Font::HelveticaBold);
    let mut btn_5 = MyButton::new(110, 240, 80, 48, "5");
    btn_5.set_label_font(Font::HelveticaBold);
    let mut btn_6 = MyButton::new(200, 240, 80, 48, "6");
    btn_6.set_label_font(Font::HelveticaBold);
    let mut btn_sub = MyButton::new(290, 240, 80, 48, "-");
    btn_sub.set_label_font(Font::HelveticaBold);
    let mut btn_1 = MyButton::new(20, 300, 80, 48, "1");
    btn_1.set_label_font(Font::HelveticaBold);
    let mut btn_2 = MyButton::new(110, 300, 80, 48, "2");
    btn_2.set_label_font(Font::HelveticaBold);
    let mut btn_3 = MyButton::new(200, 300, 80, 48, "3");
    btn_3.set_label_font(Font::HelveticaBold);
    let mut btn_add = MyButton::new(290, 300, 80, 48, "+");
    btn_add.set_label_font(Font::HelveticaBold);
    let mut btn_dot = MyButton::new(20, 360, 80, 48, ".");
    btn_dot.set_label_font(Font::HelveticaBold);
    let mut btn_0 = MyButton::new(110, 360, 170, 48, "0");
    btn_0.set_label_font(Font::HelveticaBold);
    let mut btn_equal = MyButton::new(290, 360, 80, 48, "=");
    btn_equal.set_label_font(Font::HelveticaBold);
    win.end();
    win.show();

    app::set_focus(&*btn_1);
    app::get_system_colors();

    let btn_vec = vec![
        &mut btn_1, &mut btn_2, &mut btn_3, &mut btn_4, &mut btn_5, 
        &mut btn_6, &mut btn_7, &mut btn_8,
        &mut btn_9, &mut btn_0,
    ];

    let btn_op_vec = vec![
        btn_add, btn_sub, btn_mul, btn_div, 
        btn_plusminus, btn_ce, btn_back, btn_equal,
    ];

    let (s, r) = app::channel::<Message>();

    for btn in btn_vec {
        let label = btn.label();
        btn.emit(s, Message::Number(label.parse().unwrap()));
    }

    for mut btn in btn_op_vec {
        let op = match btn.label().as_str() {
            "+" => Ops::Add,
            "-" => Ops::Sub,
            "x" => Ops::Mul,
            "÷" => Ops::Div,
            "=" => Ops::Eq,
            "CE" => Ops::CE,
            "±" => Ops::PlusMinus,
            "@<-" => Ops::Back,
            _ => Ops::None,
        };
        btn.emit(s, Message::Op(op));
    }

    btn_dot.emit(s, Message::Dot);

    while a.wait() {
        if let Some(val) = r.recv() {
            match val {
                Message::Number(num) => {
                    if frame.label() == "0" {
                        txt.clear();
                    }
                    if txt.len() <=14  || (txt.contains('.') && txt.len() <= 15){
                        txt.push_str(&num.to_string());
                    frame.set_label(txt.as_str());
                    }
                    
                    
                }
                Message::Dot => {
                    if operation == Ops::Eq {
                        txt.clear();
                        operation = Ops::None;
                        frame.set_label("0.");
                        txt.push_str("0.");
                    }
                    if !txt.contains('.') && txt.len() < 14  {
                        txt.push('.');
                        frame.set_label(txt.as_str());
                    }
                }
                Message::Op(op) => match op {
                    Ops::Add | Ops::Sub | Ops::Div | Ops::Mul => {
                        old_val.clear();
                        old_val.push_str(&frame.label());
                        operation = op;
                        frame.set_label("0");
                    }
                    Ops::Back => {
                        let val = frame.label();
                        txt.pop();
                        if val.len() > 1 {
                            frame.set_label(txt.as_str());
                        } else {
                            frame.set_label("0");
                        }
                    }
                    Ops::CE => {
                        txt.clear();
                        old_val.clear();
                        txt.push('0');
                        frame.set_label(txt.as_str());
                    }
                    Ops::PlusMinus => {

    let old_val = frame.label();
    let parsed_val: Result<i32, _> = old_val.parse();

        if let Ok(number) = parsed_val {
            let negative_number = number * -1;
            let newtxt_val = negative_number.to_string();
            frame.set_label(&newtxt_val);
        }
     
        
                    }
                    Ops::Eq => {
                        new_val = frame.label();
                        let old: f64 = old_val.parse().unwrap();
                        let new: f64 = new_val.parse().unwrap();
                        let operator = match operation {
                            Ops::Add => "+",
                            Ops::Sub => "-",
                            Ops::Mul => "x",
                            Ops::Div => "÷",
                            _ => panic!("Invalid operation"),
                        };
                        let mut result = match operation {
                            Ops::Add =>{
                                
                                old + new}, 
                            Ops::Sub => {
                                
                                old - new},
                            Ops::Mul => {
                                
                                old * new},
                            Ops::Div => {
                                
                                old / new},
                            Ops::None | Ops::Eq => new,
                            _ => panic!("Invalid operation"),
                        };
                        println!("{} {} {} = {}", old, operator, new, result);
                        //println!("Result={}",result);
                        let formatted_result = if (result.to_string().len()) > 15  {
                            let exponent = result.log10().floor() as i32;
                            let adjusted_result = result / 10_f64.powi(exponent);
                            println!("Exponent={}",exponent);
                            if exponent == 0{
                                format!("{:.4}", adjusted_result)
                            }
                            else {
                                //println!("fd");
                                format!("{:.4} x 10 ^ {}", adjusted_result, exponent)
                            }
                            
                        }  else if result.fract() == 0.0 && result.to_string().len() <= 15 {
                            format!("{:.0}", result)
                        } else {
                            
                            
                            format!("{:.}" , result)
                        };
                    
                        frame.set_label(&formatted_result);
                    
                        operation = Ops::None;
                        txt = String::from("0");
                        println!(" ");
                    }
                    
                    _ => (),
                },
            }}}}