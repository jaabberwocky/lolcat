use assert_cmd::Command;

#[test]
fn test_executable_1() {
    let mut cmd = Command::cargo_bin("lolcat").unwrap();
    cmd.arg("-f tests/inputs/test1.txt");
    let output = cmd.unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert_eq!(output_string, "meoww 🐱🐱🐱🐱 🐱🐱 🐱 🐱🐱🐱🐱 🐱🐱🐱🐱\n");
}

#[test]
fn test_executable_2() {
    let mut cmd = Command::cargo_bin("lolcat").unwrap();
    cmd.arg("-f tests/inputs/test2.txt");
    let output = cmd.unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert_eq!(output_string, "🐱🐱🐱🐱 🐱🐱🐱🐱 2\n");
}

#[test]
fn test_executable_3() {
    let mut cmd = Command::cargo_bin("lolcat").unwrap();
    cmd.arg("-f tests/inputs/test3.txt");
    cmd.arg("-n");
    let output = cmd.unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert_eq!(output_string, "1 Meoww meoww meoww 🐱🐱🐱 meow, meowwwwwwww meowwwwwww meow. Meowwww meowwwwwwwww meowww meowww, 🐱🐱🐱 meowww meoww meowww 🐱🐱. Meowwwwwww 🐱🐱🐱 🐱🐱 🐱 🐱🐱🐱🐱 meowwww meowwwwww. \n2 Meowwwww meowwwww, meoww 🐱🐱🐱🐱 meowwww meowwww, meoww 🐱🐱🐱🐱 meowwwwwww meoww, 🐱🐱🐱🐱 meowwwwww 🐱🐱🐱 🐱🐱🐱🐱 🐱🐱🐱 🐱🐱🐱🐱 meowww. Meoww 🐱🐱🐱🐱 meowwwwww meow. Meoww 🐱🐱 meowww meow.\n3 \n4 🐱🐱🐱 🐱🐱 meowww meoww. Meowww meowww 🐱🐱🐱 🐱🐱 meowwww meowwww. Meowwwwwww 🐱🐱🐱 meoww 🐱🐱 🐱🐱 meowww meowwwww. Meowwwwwwwww meowwwww meoww meowwwwww meowwwww 🐱🐱 meoww 🐱🐱 meowwwwww meoww 🐱🐱 meowww meowwww. Meoww 🐱🐱\n");
}
