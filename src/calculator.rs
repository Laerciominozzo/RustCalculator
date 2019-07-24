use gtk::{Builder, Button, Entry};
use gtk::Window;
use gtk::prelude::{*};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
enum Operation{
    add,
    min,
    mul,
    div,
    equals,
    none
}

#[derive(Clone)]
pub struct Calculator{
    builder: Builder,
    operation: Operation,
    m1:f32,
    m2:f32,
    decimal: f32
}

impl Calculator{
    pub fn new() -> Calculator{
        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let window : Window= builder.get_object("window").unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        Calculator{builder:builder,operation:Operation::none,m1: 0.0, m2:20.0, decimal: 0.0}

    }

    pub fn show_all(&self){
        let window : Window= self.builder.get_object("window").unwrap();
        window.show_all();
    }

    fn add_new_algarism(& mut self, algarism: i32){
        self.m1 *= 10.0;
        self.m1 += algarism as f32;
    }

    fn add_new_decimal_algarism(& mut self, algarism: i32){
        self.m1 += algarism as f32 * self.decimal;
        self.decimal /= 10.0;
    }

    pub fn set_decimal(& mut self){
        self.decimal = 0.1;
    }

    pub fn add_operation(& mut self, operation:Operation){

        match operation {
            Operation::none => {
                self.operation = Operation::none;
                self.m1 = 0.0;
            },
            _ => {}
        }
        match self.operation{
            Operation::equals => {
                self.operation = Operation::none;
            }
            Operation::add =>  {
                self.m2=self.m2 + self.m1;
                self.m1 =0.0;
            }
            Operation::min => {
                self.m2 = self.m2 - self.m1;
                self.m1 =0.0;
            }
            Operation::mul => {
                self.m2 = self.m2 * self.m1;
                self.m1 =0.0;
            }
            Operation::div => {
                self.m2 = self.m2 / self.m1;
                self.m1 =0.0;
            }
            _ => {
                self.m2 = self.m1;
                self.m1= 0.0;
            }
        }
        self.decimal = 0.0;
        self.operation = operation;
        let  entry : Entry = self.builder.get_object("display").unwrap();
        entry.set_text(&format!("{}", self.m2));
    }


    pub fn digit_algarism(&mut self, algarism: i32){
        if self.decimal != 0.0 {
            self.add_new_decimal_algarism(algarism);
        }else {
            self.add_new_algarism(algarism);
        }

        let  entry : Entry = self.builder.get_object("display").unwrap();
        entry.set_text(&format!("{}", self.m1));
    }

    pub fn connect_events(& mut self) -> Rc<RefCell<Calculator> >{

        let  connected_self = Rc::new(RefCell::new(self.clone()));

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b1").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(1);
        });


        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b2").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(2);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b3").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(3);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b4").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(4);
        });

        let  borrowed_self = connected_self.clone();
        let button: Button = self.builder.get_object("b5").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(5);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b6").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(6);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b7").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(7);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b8").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(8);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b9").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(9);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b0").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.digit_algarism(0);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b+").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.add_operation(Operation::add);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b-").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.add_operation(Operation::min);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b=").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.add_operation(Operation::equals);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b*").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.add_operation(Operation::mul);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b/").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.add_operation(Operation::div);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("clear").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.add_operation(Operation::none);
        });

        let  borrowed_self = connected_self.clone();
        let button : Button = self.builder.get_object("b,").unwrap();
        button.connect_clicked(  move|b|{
            let mut borrowed_self = borrowed_self.borrow_mut();
            borrowed_self.set_decimal();
        });

        connected_self
    }


}