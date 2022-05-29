/*
ANGELCAKE by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

use std::str;
use cleasy::App;
use angelmarkup::*;
use colored::Colorize;
use std::process::Command;
use std::collections::HashMap;

/// Library-wide constants.
pub fn constants() -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert(String::from("build_file"), String::from("Cakefile"));
    data.insert(String::from("author"), String::from("Alexander Abraham"));
    data.insert(String::from("version"), String::from("1.0.0"));
    data.insert(String::from("name"), String::from("Angelcake"));
    return data;
}

/// Runs a command and returns STDOUT as a string.
pub fn run_command(command: String) -> String {
    let mut result: String = String::from("");
    let mut command_vec: Vec<String> = clean_split(command, String::from(" "));
    let root_command: String = command_vec[0].clone();
    command_vec.remove(0);
    let output = Command::new(root_command).args(command_vec).output();
    match output {
        Ok(_x) => {
            result = str::from_utf8(&_x.stdout).unwrap().to_string();
        },
        Err(_e) => {
            result = _e.to_string();
        }
    };
    return result;
}

/// Attempts to run a task routine.
pub fn run(routine: String){
    if file_is(constants()["build_file"].clone()) == true {
        if lint(read_file(constants()["build_file"].clone())) == true {
            let rules: HashMap<String, String> = serialize(read_file(constants()["build_file"].clone())).unwrap();
            for (rule_name, rule) in rules.into_iter() {
                if rule_name == routine {
                    println!("{}", run_command(rule));
                }
                else {}
            }
        }
        else {
            println!("{}", format!("Cake not baked correctly!").red().to_string());
        }
    }
    else {
        println!("{}", format!("Cake not found!").red().to_string());
    }
}

/// ANGELCAKE's CLI.
pub fn cli(){
    let mut angelcake: App = App::new(
        constants()["name"].clone(),
        constants()["version"].clone(),
        constants()["author"].clone()
    );
    angelcake.add_arg("runr".to_string(), "runs a routine".to_string(), "true".to_string());
    if angelcake.version_is() == true {
        println!("{}", format!("{}",angelcake.version()).cyan().to_string());
    }
    else if angelcake.help_is() == true {
        println!("{}", format!("{}",angelcake.help()).cyan().to_string());
    }
    else if angelcake.arg_was_used("runr".to_string()) == true {
        let arg_data: String = angelcake.get_arg_data("runr".to_string());
        run(arg_data);
    }
    else {
        println!("{}", format!("{}",angelcake.help()).red().to_string());
    }
}
