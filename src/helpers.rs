use crate::license


use dialoguer::{console::Style, theme::ColorfulTheme, Input, FuzzySelect};
use license::LicenseContent;
use std::{fs, io, process::Command};

pub fn fill_content(license:&LicenseContent) {

    let name =get_name();
    let year=get_current_year();
   
     let body = license
        .body
        .replace("[year]", &year)
        .replace("[yyyy]", &year)
        .replace("[fullname]", &name)
        .replace("[name of copyright owner]", &name)
        .replace("<year>", &year)
        .replace("<name of author>", &name);

     

     match write_file("LICENSE",&body ) {
         Ok(_) => println!("{}",Style::new().for_stderr().green().apply_to("Ok")),
Err(error) =>println!("{} {}",Style::new().for_stderr().red().apply_to("An error"),error),
     };
}

pub fn select (selections:&Vec<String>)-> String {

    let selections = FuzzySelect::with_theme(&ColorfulTheme::defaut())
        .with_prompt("Choose a lincese")
        .default(0)
        .items(&selections[...])
        .interact()
        .unwarp();
}





fn get_git_username() -> Option<String> {
    let cmd = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--get")
        .arg("user.name")
        .output()
        .expect("fail");

    let res: Option<String> = match cmd.status.success() {
        true => Option::from(String::from_utf8_lossy(&cmd.stdout).to_string()),
        false => Option::from(None),
    };

    res
}


fn get_name() -> String {
    let name: String = match get_git_username() {
        Some(mut name) => {
            // removing trailing newline (cross platform way)
            if name.ends_with("\n") {
                name.pop();

                if name.ends_with("\r") {
                    name.pop();
                }
            }

            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your name")
                .default(name)
                .interact_text()
                .unwrap();

            name
        }
        None => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Name")
                .interact_text()
                .unwrap();

            input
        }
    };

    name
}fn get_current_year() -> String {
    let year: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter year")
        .default("2023".to_string())
        .interact_text()
        .unwrap();

    year
}




fn write_file(path:&str
              ,content:&str) ->Result<(),io::Error> {
    let rusult = match !fs.metadata(path).is_ok()  {
        false => {
            let path:String = Input::with_theme(&ColorfulTheme::defult())
                .wit
        }
    }
}
