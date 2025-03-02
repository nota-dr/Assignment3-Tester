pub mod hw3_tests;

use tests_lib::*;

pub fn add_tests(tm: &mut TestManager, mut port: u16) -> Vec<String> {
    let test_templates = vec![
        TestTemplateBuilder::new("Usage")
            .args_template("./server {} 1 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::Usage)))
            .timeout(5)
            .log_output(true)
            .build(),
        TestTemplateBuilder::new("Validate Input")
            .args_template("./server {} 1 1 3")
            .agent(Box::new(|| Box::new(hw3_tests::ValidateInput)))
            .timeout(20)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("Only GET Method")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::OnlyGETMethod)))
            .timeout(10)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("Path Does Not Exist")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::PathDoesNotExist)))
            .timeout(10)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("Temporary Redirect")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::TemporaryRedirect)))
            .timeout(10)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("Search for Index HTML")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::SearchForIndexHtml)))
            .timeout(10)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("Return Dir Content")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::ReturnDirContent)))
            .timeout(15)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("Forbidden")
            .args_template("./server {} 1 1 2")
            .agent(Box::new(|| Box::new(hw3_tests::Forbidden)))
            .timeout(15)
            .communicate(true)
            .operation_timeout(5)
            .build(),
        TestTemplateBuilder::new("File Size Exceeds OS Buffer")
            .args_template("./server {} 1 1 1")
            .agent(Box::new(|| Box::new(hw3_tests::FileSizeExceedsOSBuffer)))
            .timeout(15)
            .communicate(true)
            .operation_timeout(10)
            .build(),
        TestTemplateBuilder::new("Deadlock")
            .args_template("./server {} 4 5 15")
            .agent(Box::new(|| Box::new(hw3_tests::Deadlock)))
            .timeout(50)
            .valgrind(true)
            .communicate(true)
            .operation_timeout(3)
            .build(),
        TestTemplateBuilder::new("Valgrind Path Coverage")
            .args_template("./server {} 3 5 8")
            .agent(Box::new(|| Box::new(hw3_tests::Valgrind)))
            .timeout(45)
            .valgrind(true)
            .communicate(true)
            .operation_timeout(5)
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
