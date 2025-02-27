//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
use rand::{distributions::Alphanumeric, Rng};
pub mod scroll_group;
use scroll_group::ScrollGroup;


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);
    let sg_width = 300;
    let mut test_pack = Pack::new(0,0,sg_width, 100, "");

    
    test_pack.end();
    for x in 0..10{
        let but = Button::new(0,0,100,50,&x.to_string());
        test_pack.add(&but);
        // test_pack.add(&Button::default().with_label(&x.to_string()))
    }
    let sg = ScrollGroup::new((win.width()-sg_width)/2, 300, sg_width, 500, test_pack.clone());


    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
            }
        }
    }
}

