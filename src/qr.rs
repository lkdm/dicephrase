use qrcode::{QrCode, render::unicode};

pub fn print_qr(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let code = QrCode::new(text)?;

    let image = code.render::<unicode::Dense1x2>().build();

    println!("{image}");

    Ok(())
}
