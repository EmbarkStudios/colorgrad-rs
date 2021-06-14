use colorgrad::{parse_ggr, Color};
use std::io::BufReader;

#[test]
fn parse_gimp_gradients() {
    let col = Color::default();
    let red = Color::from_rgb(1.0, 0.0, 0.0);
    let blue = Color::from_rgb(0.0, 0.0, 1.0);

    // Black to white
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 0 0";
    let (grad, name) = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    assert_eq!(name, "My Gradient");
    assert_eq!(grad.domain(), (0.0, 1.0));
    assert_eq!(grad.at(0.0).rgba_u8(), (0, 0, 0, 255));
    assert_eq!(grad.at(1.0).rgba_u8(), (255, 255, 255, 255));
    assert_eq!(grad.at(-0.5).rgba_u8(), (0, 0, 0, 255));
    assert_eq!(grad.at(1.5).rgba_u8(), (255, 255, 255, 255));

    // Foreground to background
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 1 3";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(grad.at(1.0).rgba_u8(), (0, 0, 255, 255));

    // Foreground transparent to background transparent
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 2 4";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).rgba_u8(), (255, 0, 0, 0));
    assert_eq!(grad.at(1.0).rgba_u8(), (0, 0, 255, 0));

    // Blending function: step
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 0 0 1 0 0 1 1 5 0 0 0";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    assert_eq!(grad.at(0.00).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(grad.at(0.25).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(grad.at(0.49).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(grad.at(0.51).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(grad.at(0.75).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(grad.at(1.00).rgba_u8(), (0, 0, 255, 255));
}
