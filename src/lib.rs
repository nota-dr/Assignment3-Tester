pub mod hw3_tests;

use tests_lib::*;

pub fn add_tests(tm: &mut TestManager, mut port: u16) -> Vec<String> {
    let test_templates = vec![
        TestTemplateBuilder::new("Usage")
            .args_template("./server {} 1 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::Usage)))
            .validate_timeout(5)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("Validate Input")
            .args_template("./server {} 1 1 3")
            .agent(Box::new(|| Box::new(hw3_tests::ValidateInput)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(7)
            .build(),
        TestTemplateBuilder::new("Only GET Method")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::OnlyGETMethod)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("Path Does Not Exist")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::PathDoesNotExist)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("Temporary Redirect")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::TemporaryRedirect)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("Search for Index HTML")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::SearchForIndexHtml)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("Return Dir Content")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::ReturnDirContent)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("Forbidden")
            .args_template("./server {} 1 1 2")
            .agent(Box::new(|| Box::new(hw3_tests::Forbidden)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("File Size Exceeds OS Buffer")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::FileSizeExceedsOSBuffer)))
            .validate_timeout(25)
            .communicate(true)
            .communicate_timeout(10)
            .build(),
        TestTemplateBuilder::new("Deadlock")
            .args_template("./server {} 4 5 15")
            .agent(Box::new(|| Box::new(hw3_tests::Deadlock)))
            .validate_timeout(40)
            .valgrind(true)
            .communicate(true)
            .communicate_timeout(15)
            .build(),
        TestTemplateBuilder::new("Valgrind Path Coverage")
            .args_template("./server {} 3 5 8")
            .agent(Box::new(|| Box::new(hw3_tests::Valgrind)))
            .validate_timeout(50)
            .valgrind(true)
            .communicate(true)
            .communicate_timeout(7)
            .build(),
    ];

    let mut keys = Vec::new();
    for template in test_templates {
        let key = tm.register_template(template);
        tm.instantiate_test(&key, Some(port));
        keys.push(key);
        port += 1;
    }

    keys
}
