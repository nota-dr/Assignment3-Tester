mod hw3_tests;
use tests_lib::TestManager;

fn main() {
    let mut tm = TestManager::new("assignment3", "testee", 5);
    hw3_tests::create_dir_structure();
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
