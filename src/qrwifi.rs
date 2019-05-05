use qrcode::QrCode;
use qrcode::render::svg;
use std::fs;

pub fn create_wifi_qrcode(wpa: &str, psk: &str, width: u32, height: u32, svg_path: Option<&str>) -> String {
    let code = QrCode::new(format!("WIFI:T:WPA;S:{};P:{};;", wpa, psk)).unwrap();
    let image = code.render()
        .min_dimensions(width, height)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    if let Some(path) = svg_path {
        fs::write(path, &image).expect("Unable to write svg file");
    }
    image
}