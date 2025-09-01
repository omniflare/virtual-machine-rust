use std::fs;
use std::process::Command;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_vm_file(filename: &str) -> String {
        let output = Command::new("./target/debug/virtual-machine")
            .arg(format!("tests/{}", filename))
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("{}{}", stdout, stderr)
    }

    #[test]
    fn test_add_program() {
        let output = run_vm_file("add.vm");
        assert!(output.contains("Final Result: 15"));
    }

    #[test]
    fn test_mul_program() {
        let output = run_vm_file("mul.vm");
        assert!(output.contains("Final Result: 6"));
    }

    #[test]
    fn test_div_by_zero() {
        let output = run_vm_file("div_by_zero.vm");
        assert!(output.contains("Error: Cannot Divide By Zero"));
        // Note: No "Final Result" on error
    }

    #[test]
    fn test_empty_program() {
        let output = run_vm_file("empty.vm");
        assert!(output.contains("Program finished with empty stack"));
    }

    // Additional test cases

    #[test]
    fn test_sub_program() {
        // Create a temporary sub.vm file
        let program = "PSH 10\nPSH 3\nSUB\nHLT\n";
        fs::write("tests/sub.vm", program).expect("Unable to write file");
        let output = run_vm_file("sub.vm");
        assert!(output.contains("Final Result: 7"));
        fs::remove_file("tests/sub.vm").ok(); // Clean up
    }

    #[test]
    fn test_stack_underflow() {
        let program = "POP\nHLT\n";
        fs::write("tests/underflow.vm", program).expect("Unable to write file");
        let output = run_vm_file("underflow.vm");
        assert!(output.contains("STACK UNDERFLOW !!"));
        fs::remove_file("tests/underflow.vm").ok();
    }

    #[test]
    fn test_unknown_instruction() {
        let program = "UNK\nHLT\n";
        fs::write("tests/unknown.vm", program).expect("Unable to write file");
        let output = run_vm_file("unknown.vm");
        assert!(output.contains("Unknown instruction: 9"));
        fs::remove_file("tests/unknown.vm").ok();
    }

    #[test]
    fn test_set_not_implemented() {
        let program = "SET\nHLT\n";
        fs::write("tests/set.vm", program).expect("Unable to write file");
        let output = run_vm_file("set.vm");
        assert!(output.contains("SET not implemented yet"));
        fs::remove_file("tests/set.vm").ok();
    }

    #[test]
    fn test_multiple_operations() {
        let program = "PSH 5\nPSH 3\nADD\nPSH 2\nMUL\nHLT\n";
        fs::write("tests/multi.vm", program).expect("Unable to write file");
        let output = run_vm_file("multi.vm");
        assert!(output.contains("Final Result: 16")); // (5+3)*2 = 16
        fs::remove_file("tests/multi.vm").ok();
    }

    #[test]
    fn test_negative_result() {
        let program = "PSH 3\nPSH 10\nSUB\nHLT\n";
        fs::write("tests/negative.vm", program).expect("Unable to write file");
        let output = run_vm_file("negative.vm");
        assert!(output.contains("Final Result: -7"));
        fs::remove_file("tests/negative.vm").ok();
    }
}
