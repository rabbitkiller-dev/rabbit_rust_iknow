
#[test]
fn imageToSmall() {
    let img = image::open("assets/bodengke.png").unwrap();
    let newimg = img.resize(40, 40, image::imageops::FilterType::Triangle);
    newimg.save("assets/square.png").unwrap();
    let result = image_base64::to_base64("assets/square.png");
    println!("{result}")
}