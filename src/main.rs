// use gtk::prelude::*;
// use gtk::{glib, Application, ApplicationWindow, Button};

// const APP_ID: &str = "com.example.myapp";
const FILE_NAME: &str = "Zhongwen-Words.txt";

fn main()  {
    // let app = Application::builder().application_id(APP_ID).build();

    // app.connect_activate(build_ui);

    // app.run()


    let file_content = read_file(FILE_NAME).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        Vec::new()
    });

    for line in &file_content{
        println!("{}", line);
    }

}

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    use std::io::{BufRead, BufReader};
    use std::fs::File;

    let file = File::open("./src/".to_owned() + filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)

}

// fn build_ui(app: &Application) {
//     let button = Button::builder()
//         .label("Luv u adina <33 !")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let button2 = Button::builder()
//         .label("Luv u adina <33 !")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();


//     button.connect_clicked(|button| {
//         button.set_label("Hello World c:");
//     });

//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("Post It Notes")
//         .child(&button)
//         .build();

//     let window2 = ApplicationWindow::builder()
//         .application(app)
//         .title("Another Post It")
//         .child(&button2)
//         .build();

//     window2.present();
//     window.present();
// }
