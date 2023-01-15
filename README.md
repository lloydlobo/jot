# Jot

Jot allows you to jot down anything and upload it to a repository.

Version control your jots efficiently with git and github(optional).

A fork of [eureka](https://github.com/simeg/eureka).

## Structure

<!-- https://jojozhuang.github.io/tutorial/mermaid-cheat-sheet/  -->

### Level 1

```mermaid
graph TD;

subgraph run
    A[(fn: main: bin/jot.rs)]==>A1[fn: clap::Command::new];
    A1-->A11(fn: Jot::new);
    A1-->A12(struct: JotOptions);
    A11-->A2(fn: jot.run opts);
    A12-->A2;
    A2-->B;
    B[(fn: run: lib.rs)] ==> |if: opts.clear_config|B1[fn: Jot::clear_config];
    B==>|if: opts.view|B2[fn: Jot::open_jot_file];
    B==>B3>if: fn: Jot::is_config_missing];
    B3-->B31(is true);
    B3-->B32(is false);
end

%% Lead to fn if config_read
subgraph librs if cfg missing dir doesnt create dir
    B31-->B31B(if fn: !Jot::cm.config_dir_exists)-->|is true|B3A[fn: cm.config_dir_create];
    B3A-->B3AA(self.config_dir_path.and_then fs::create_dir_all);
    B3AA-->B3AB(fn Jot.resolve_xdg_config_home .or_else fn: home_dir.join'.config.dot');
    B3AB-.->|err|B3AB2(Failed to resolve $HOME dir);
    B3AB-->|ok|B3AB1(PathBuf);
end

B3AB1-->|success: created dir|B31A;

subgraph librs if cfg missing config read repo loop
    B31B-->|is false|B31A;
    B31A(if fn: Jot::cm.config_read Repo is_error)-->|is true|C;
    %% Get user input for repository path
    C[setup_repo_path]-->|fn: Jot::reader.read_input|C1(var: user_input);
    C1-->C11>if: fn: user_input.is_empty];
    C11-.->|is true: continue|C;
    C11-->|is false: fn: Path::new user_input|D;
    D1-.->|is false: Error|C;
    D(var:path)-->D1>if fn: path.is_absolute];
    D1-->|is true:|D11(break: fn: Jot.cm.config_write);
end

subgraph success ask for jot
    B31A-->|is false|E;
    B32-->E([fn: ask_for_jot lib.rs]);
    D11-->|setup completed|E;
    %% have no summary from user
    E-->F(var: jot_summary = String::new);
    F-->G;
    G>while: fn: jot_summary.is_empty]-.is true.->G1;
    %% TODO: Input header
    G1(fn Jot::printer.input_header);
    G1-.->|fn: Printer::println_styled<br>fn: Printer::writer.flush|G11;
    G11(var = Jot::reader.read_input);

    G11-.->F;
    G-->H(var: repo_path = Jot::cm.config_read Repo);

    %% Got summary from user
    %% Open editor then, add, commit and push to git
    H-->I([fn: Jot::program_opener<br>.open_editor 'repo_path/README.md' <br>.and Jot::git_add_commit_push jot_summary]);
end

%%classDef green fill:#9f6,stroke:#ccc,stroke-width:2px
classDef orange fill:#f96,stroke:#ccc,stroke-width:2px
class E,I orange
```
