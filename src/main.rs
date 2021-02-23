use fltk::{app::*, button::*, text::*, window::*, input::*, dialog::*};
use base64::{encode, decode};

fn main() {
    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut wind = Window::default()
        .with_size(600, 380)
        .center_screen()
        .with_label("Encode - Decode Base64 strings");
    let _ = TextDisplay::new(10, 40, 580, 40, "Normal text");
    let normal_text = MultilineInput::new(10, 40, 580, 100, "");
    let _ = TextDisplay::new(10, 180, 580, 40, "Base64 text");
    let base64_text = MultilineInput::new(10, 180, 580, 100, "");

    let mut button_encode = Button::new(220, 310, 80, 40, "Encode");
    let mut button_decode = Button::new(310, 310, 80, 40, "Decode");

    wind.end();
    wind.show();

    let work_normal_text = normal_text.clone();
    let work_base64_text = base64_text.clone();
    button_encode.set_callback(Box::new(move || {
        let text_normal = work_normal_text.value();
        work_base64_text.set_value(&encode(text_normal));
    }));

    button_decode.set_callback(Box::new(move || {
        let text_base64 = base64_text.value();
        match decode(text_base64) {
            Ok(dc) => {
                match String::from_utf8(dc) {
                    Ok(s) => normal_text.set_value(&s),
                    Err(e) => {message(100, 100, ("Error: ".to_string() + &e.to_string()).as_str());}
                }
            },
            Err(e) => {                
                message(100, 100, ("Error: ".to_string() + &e.to_string()).as_str()); 
            }
        };
    }));

    app.run().unwrap();
}