use xterm_js_rs::Terminal;

// this echo is more similar to the bash echo.
// POSIX echo allows for --help and --version.
// Bash doesn't, but it respects -e/-n.
// We take the easy road and allow neither!
pub fn echo(term: &Terminal, args: Vec<&str>) -> i32 {
	let al = args.len();
	if al == 0 {
		term.writeln("");
		return 0;
	}
	if al == 1 {
		term.writeln(args[0]);
		return 0;
	}
	for n in 0..al-1 {
		term.write(args[n]);
		term.write(" ");
	}
	term.writeln(args[al-1]);
	return 0;
}