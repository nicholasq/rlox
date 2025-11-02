use rlox::{environment::Environment, interpreter::Interpreter, RLox};

struct TestCase<'a> {
    source: &'a str,
    expected_output: &'a [u8],
    description: &'static str,
}

#[test]
fn test_scripts() {
    let test_cases = vec![
        TestCase {
            source: "print 1;",
            expected_output: b"1\n",
            description: "Print number",
        },
        TestCase {
            source: r#"print "hello";"#,
            expected_output: b"hello\n",
            description: "Print string",
        },
        TestCase {
            source: "print true;",
            expected_output: b"true\n",
            description: "Print boolean",
        },
        TestCase {
            source: "print nil;",
            expected_output: b"nil\n",
            description: "Print nil",
        },
        TestCase {
            source: r#"

            var count = 0;
            while (count < 10) {
                count = count + 1;
            }
            print count;

            "#,
            expected_output: b"10\n",
            description: "While loop",
        },
        TestCase {
            source: r#"

            var count = 0;
            for(var i = 0; i < 5; i = i + 1) {
              if (i == 3) {
                count = 42;
              }
            }

            print count;

            "#,
            expected_output: b"42\n",
            description: "For loop with if statement",
        },
    ];

    for TestCase {
        source,
        expected_output,
        description,
    } in test_cases
    {
        let mut output = Vec::new();
        let interpreter = Interpreter::new(Environment::default(), &mut output);
        let mut rlox = RLox::new(interpreter);
        rlox.run(source);

        assert_eq!(output, expected_output, "{}", description);
    }
}
