#![windows_subsystem = "windows"]
use rdev::{listen, Event};
use std::sync::{Arc, Mutex};
use enigo::*;
use webbrowser;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(Vec::new()));

    let data_clone = Arc::clone(&data);
    let callback = move |event: Event| {
        match event.name {
            Some(string) => {
                // println!("{}", string);
                let mut data = data_clone.lock().unwrap();
                data.push(string.clone());
                println!("{:?}", data);
                if data.len() >= 10 {
                    let b = data.clone().join("");
                    data.clear();
                    println!("cleared");
                    tokio::spawn(async move {
                        matchinput(b.clone()).await;
                    });
                }
                else if &string == "q" {
                    data.clear();
                    println!("data cleared")
                }
            },
            None => (),
        }
    };
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
async fn matchinput (value: String) {
    if value == "openinpcrs" {
        let mut enigo = Enigo::new();
        webbrowser::open("http://github.com/nikunjthebest").expect("failed to open URL");
            thread::sleep(time::Duration::from_millis(4000));
        webbrowser::open("https://music.youtube.com/").expect("failed to open URL");
            thread::sleep(time::Duration::from_millis(2500));
        enigo.mouse_move_to(50, 20);
            thread::sleep(time::Duration::from_millis(1000));
        enigo.mouse_click(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(1000));
        enigo.key_sequence_parse("{+CTRL}{+SHIFT}p{-SHIFT}{-CTRL}");
            thread::sleep(time::Duration::from_millis(2000));
        enigo.key_sequence_parse("phind.com");
            thread::sleep(time::Duration::from_millis(300));
        enigo.key_click(Key::Return);
            thread::sleep(time::Duration::from_millis(300));
        enigo.key_click(Key::Windows);
            thread::sleep(time::Duration::from_millis(1000));
        enigo.key_sequence("rust");
            thread::sleep(time::Duration::from_millis(500));
        enigo.key_click(Key::Return);
    }
    else if value == "openinpcmv" {
        let mut enigo = Enigo::new();
        webbrowser::open("https://www.hotstar.com/in/home").expect("failed to open URL");
            thread::sleep(time::Duration::from_millis(4000));
        enigo.mouse_move_to(50, 20);
            thread::sleep(time::Duration::from_millis(1000));
        enigo.mouse_click(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(1000));
        enigo.key_sequence_parse("{+CTRL}{+SHIFT}p{-SHIFT}{-CTRL}");
            thread::sleep(time::Duration::from_millis(2000));
        enigo.key_sequence_parse("youtube.com");
            thread::sleep(time::Duration::from_millis(1000));
        enigo.key_click(Key::Return);
            thread::sleep(time::Duration::from_millis(300));
        enigo.key_click(Key::Windows);
            thread::sleep(time::Duration::from_millis(1000));
        enigo.key_sequence("netflix");
        enigo.key_click(Key::Return);
    }
    else if value == "openinpcmu" {
        let mut enigo = Enigo::new();
        thread::sleep(time::Duration::from_millis(300));
        enigo.key_down(Key::Windows);
        enigo.key_click(Key::A);
        enigo.key_up(Key::Windows);
        thread::sleep(time::Duration::from_millis(1000));
        enigo.mouse_move_to(1330, 450);
        thread::sleep(time::Duration::from_millis(300));
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(2000));
        webbrowser::open("https://music.youtube.com/").expect("failed to open URL");

    }
}