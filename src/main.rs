use clipboard_win::{Clipboard, Getter};
use clipboard_win::formats::FileList;
use clipboard_win::raw::format_name;

fn main() -> Result<(), u32> {
    let _clipboard = Clipboard::new()
        .expect("Get clipboard failed");


    let format = 'l: {
        for x in (49400..49500).into_iter().chain(clipboard_win::EnumFormats::new()) {
            if let Some(name) = format_name(x) {
                if name == "HTML Format" {
                    println!("Got HTML Format for number {}", x);
                    break 'l x;
                } else {
                    println!("{x}: {name}")
                }
            }
        }

        eprintln!("Cannot get HTML Format from clipboard");
        return Err(100);
    };

    let mut data = Vec::new();
    if let Ok(_) = FileList.read_clipboard(&mut data) {
        if data.len() == 1 {
            let file_path = &data[0];
            if file_path.ends_with(".gif") {
                clipboard_win::raw::set(format,
                                        format!(r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.0 Transitional//EN"><html><body><!--StartFragment--><p><img src="file:///{file_path}"></p><!--EndFragment--></body></html>"#)
                                            .as_ref())
                    .expect("Set clipboard failed");
                Ok(())
            } else {
                eprintln!("File path not ends with gif");
                Err(2)
            }
        } else {
            eprintln!("Too many files");
            Err(3)
        }
    } else {
        eprintln!("Clipboard data not file");
        Err(1)
    }
}
