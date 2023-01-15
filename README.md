# Jot

Jot allows you to jot down anything and upload it to a repository.

Version control your jots efficiently with git and github(optional).

A fork of [eureka](https://github.com/simeg/eureka).

## Structure

<!-- https://jojozhuang.github.io/tutorial/mermaid-cheat-sheet/  -->

```mermaid
graph TD;

A((fn: main: bin/jot.rs))==>A1[fn: clap::Command::new];
A1-->A11(fn: Jot::new);
A1-->A12(struct: JotOptions);
A11-->|fn: jot.run opts|B((fn: run: lib.rs));
A12-->|fn: jot.run opts|B;

B==>|if: opts.clear_config|B1[fn: Jot::clear_config];
B==>|if: opts.view|B2[fn: Jot::open_jot_file];
B==>B3>if: fn: Jot::is_config_missing];
B3-->B31(is true);
B3-->B32(is false)-->B321(fn: ask_for_jot);
B31-->|if fn: Jot::cm.config_dir_exists|B311[cm.config_dir_create];
B31-->|if fn: Jot::cm.config_read Repo is_error|B312[setup_repo_path];
B312-.->C([loop]);
C-->|fn: Jot::reader.read_input|C1(var: user_input);
C1-->C11>if: fn: user_input.is_empty];
C11-.->|is true: continue|C;
C11-->|is false: fn: Path::new user_input|D(var:path);
D-->D1>if fn: path.is_absolute]-->D11(is true)-->TODO;
D1-.->|is false: Error|C;
```
