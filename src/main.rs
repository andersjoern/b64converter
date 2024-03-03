use base64::{DecodeError, Engine, engine::general_purpose};
use fltk::{button::*, input::*, text::*, window::*, prelude::*};
use tinyfiledialogs as tfd;

fn main() -> Result<(), FltkError> {
    let app = fltk::app::App::default().with_scheme(fltk::app::AppScheme::Base);
    let mut wind = Window::default()
        .with_size(600, 380)
        .center_screen()
        .with_label("Encode - Decode Base64 text");
    let _ = TextDisplay::new(10, 40, 580, 40, "Normal text");
    let mut normal_text = MultilineInput::new(10, 40, 580, 100, "");
    let _ = TextDisplay::new(10, 180, 580, 40, "Base64 text");
    let base64_text = MultilineInput::new(10, 180, 580, 100, "");

    let mut button_encode = Button::new(220, 310, 80, 40, "Encode");
    let mut button_decode = Button::new(310, 310, 80, 40, "Decode");

    wind.end();
    wind.show();

    let work_normal_text = normal_text.clone();
    let mut work_base64_text = base64_text.clone();
    button_encode.set_callback(move |_| {
        let text_normal = work_normal_text.value();
        work_base64_text.set_value(&general_purpose::STANDARD.encode(text_normal));
    });

    button_decode.set_callback(move |_| {
        let text_base64 = base64_text.value();
        match general_purpose::STANDARD.decode(text_base64) {
            Ok(dc) => match String::from_utf8(dc) {
                Ok(s) => normal_text.set_value(&s),
                Err(e) => tfd::message_box_ok("Error", &e.to_string(), tfd::MessageBoxIcon::Error),
            },
            Err(e) => match e {
                DecodeError::InvalidByte(pos, b) => {
                    tfd::message_box_ok("Error", &format!("Invalid byte {b} at position: {pos}"), tfd::MessageBoxIcon::Error);
                }
                DecodeError::InvalidLastSymbol(pos, s) => {
                    tfd::message_box_ok("Error", &format!("Invalid last symbol {s} at pos: {pos}"), tfd::MessageBoxIcon::Error);
                }
                DecodeError::InvalidLength(len) => tfd::message_box_ok("Error",&format!("Invalid length: {len}"), tfd::MessageBoxIcon::Error),
                DecodeError::InvalidPadding => tfd::message_box_ok("Error", "Invalid padding", tfd::MessageBoxIcon::Error),
            },
        };
    });

    app.run()?;
    Ok(())
}
