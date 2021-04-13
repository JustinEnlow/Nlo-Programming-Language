use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    //Debug argument list
    //println!("{:?}", args);

    process_args(&args);
}

fn process_args(args: &[String]){
    if args.len() < 2{println!("Not enough arguments.");}
    else if args.len() > 4{println!("Too many arguments.");}
    else{
        match args[1].as_str(){
            "new" => {
                if args.len() < 3{println!("Not enough arguments.");}
                else{
                    match args[2].as_str(){
                        "exe" => {
                            if args.len() < 4{println!("Not enough arguments.");}
                            else{
                                let project_name = args[3].as_str();
                                create_project(&project_name, true);
                            }
                        },
                        "lib" => {
                            if args.len() < 4{println!("Not enough arguments.");}
                            else{
                                let project_name = args[3].as_str();
                                create_project(&project_name, false);
                            }
                        },
                        _ => {println!("Invalid argument.");}
                    };
                }
            },
            "build" => {build();},
            "run" => {run();},
            "test" => {test();},
            _ => {println!("Invalid argument");}
        }
    }
}

fn create_project(project_name: &str, is_executable: bool){
    let mut directory: PathBuf = [".", project_name].iter().collect();
    fs::create_dir(&directory).expect("Couldn't create directory.");

    directory.push("src");
    fs::create_dir(&directory).expect("Couldn't create directory.");
    if is_executable == true{
        directory.push("Main.nlo");
        let mut main_file = fs::File::create(&directory).expect("Couldn't create file.");
        let main_text = "use CommandLineInterface::PrintLine.\n\nfunction Main(){\n\tPrintLine(\"Fuck off, World!\").\n}";
        main_file.write(main_text.as_bytes()).expect("Couldn't write to file.");
        directory.pop();
    }
    directory.pop();

    directory.push("test");
    fs::create_dir(&directory).expect("Couldn't create directory.");
    directory.push("Test.nlo");
    let mut test_file = fs::File::create(&directory).expect("Couldn't create file.");
    let test_text = "// Tests who's assert evaluates to true will pass, false will fail.\n\ntest FailingTest{\n\tassert false.\n}";
    test_file.write(test_text.as_bytes()).expect("Couldn't write to file.");
    directory.pop();
    directory.pop();

    directory.push(".gitignore");
    let mut ignore_file = fs::File::create(&directory).expect("Couldn't create file.");
    ignore_file.write(b"/target").expect("Couldn't write to file.");
    directory.pop();

    let toml_text = format!("[package]\nname = {}\nversion = \"0.1.0\"\n\n[dependencies]", project_name);
    directory.push("Nlo.toml");
    let mut toml_file = fs::File::create(&directory).expect("Couldn't create file.");
    toml_file.write(&toml_text.as_bytes()).expect("Couldn't write to file.");
    directory.pop();

    if is_executable == true{
        println!("Created executable project: {}", project_name);
    }
    else{
        println!("Created library project: {}", project_name);
    }
}

fn build(){
    let main_file: PathBuf = [".", "src", "Main.nlo"].iter().collect();
    let contents = fs::read_to_string(&main_file).expect("Couldn't read file contents.");
    //TODO: actually build
    println!("{}", contents);
}
fn run(){println!("run");}
fn test(){println!("test");}