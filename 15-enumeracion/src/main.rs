
#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick{x: i64, y: i64}

#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress)}


fn main() {

    let click = MouseClick{x: 100, y: 250};
    println!("Mouse click location {}, {}", click.x, click.y);
   
    let key = KeyPress("Ctrl + ".to_string(), 'C');
    println!("Key pressed: {}{}", key.0, key.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_keys = WebEvent::WEKeys(key);
    
    
    println!("Enumeraciones: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_keys);
}
