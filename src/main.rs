mod hw3_tests;
use tests_lib::TestManager;

fn main() {
    let mut tm = TestManager::new("assignment3", "testee", 5);
    hw3_tests::create_dir_structure();
    println!("----- Directory Structure -----");
    println!("├── dir1");
    println!("│   ├── dir2");
    println!("│   │   ├── dir3");
    println!("│   │   │   ├── avi_example.avi");
    println!("│   │   │   ├── gif_example.gif");
    println!("│   │   │   ├── html_example.html");
    println!("│   │   │   ├── jpg_example.jpg");
    println!("│   │   │   ├── mov_example.mov");
    println!("│   │   │   ├── mp3_example.mp3");
    println!("│   │   │   ├── png_example.png");
    println!("│   │   ├── dir4");
    println!("│   │   │   ├── no_permission");
    println!("│   │   ├── fifo_file");
    println!("│   │   ├── index.html");
    println!();
    
    assignment3_tester::add_tests(&mut tm, 5000);

    let compilation =
        tm.compile_assignment("gcc -Wall -Wextra *.c *.h -o server -lpthread");
    if compilation != "error" {
        println!("----- Tests Results -----");
        for (name, ok) in tm.run_tests() {
            if ok {
                println!("[+] {}... \x1b[32mok\x1b[0m", name);
            } else {
                println!("[-] {}... \x1b[31mfailed\x1b[0m", name);
            }
        }
    } else {
        println!("Failed to compile assignment");
    }
}
