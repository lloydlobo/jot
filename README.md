# Jot

Jot allows you to jot down anything and upload it to a repository.

Version control your jots efficiently with git and github(optional).

A fork of [eureka](https://github.com/simeg/eureka).

## Setup

### Path

```bash
$ jot
############################################################
####                  First Time Setup                  ####
############################################################

This tool requires you to have a repository with a README.md
in the root folder. The markdown file is where your jots
will be stored.
Once first time setup has completed, simply run Jot again
to start jotting down your snippets, haiku, tips & tricks.

Absolute path to your repository
> /$HOME/path/to/repository/
```

-   `user_input` - path `/$HOME/path/to/your/repository/`
-   Links and Writes `user_input` `Repo` path to config in `~/.config/jot/config.json`.

### Jot Summary

```bash
$ jot
>> Jot summary
> Hello, world!
Adding and committing you new jot to main..
```

## Structure

<!-- https://jojozhuang.github.io/tutorial/mermaid-cheat-sheet/  -->

### Mermaid Flowchart

```mermaid
graph TD;

%%subgraph main rs main run
    A[(fn: main: bin/jot.rs)]==>A1[fn: clap::Command::new];
    A1-->A11(fn: Jot::new);
    A11-->A2(fn: jot.run opts)-->B;
    A1-->A12(struct: JotOptions)-->A2;
    B[(fn: run: lib.rs)] ==> |if: opts.clear_config|B1[fn: Jot::clear_config];
    B==>B3>if: fn: Jot::is_config_missing];
    B==>|if: opts.view|B2[fn: Jot::open_jot_file];
    B3-->B31(is true);
    B3-->B32(is false);
%%end

%% Lead to fn if config_read
%%subgraph lib rs if cfg missing dir doesnt create dir
    B31-->B31B(if fn: !Jot::cm.config_dir_exists)-->|is true|B3A[fn: cm.config_dir_create];
    B3A-->B3AA(self.config_dir_path.and_then fs::create_dir_all);
    B3AA-->B3AB(fn Jot.resolve_xdg_config_home .or_else fn: home_dir.join'.config.dot');
    B3AB-.->|err|B3AB2(Failed to resolve $HOME dir);
    B3AB-->|ok|B3AB1(PathBuf);
%%end

B3AB1-->|success: created dir|B31A;

%%subgraph lib rs if cfg missing config read repo loop
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
%%end

%%subgraph IMPLJOT
    %% Main happy path conjunction
    B31A-->|is false|E;
    D11-->|setup completed|E;
%%end

%%subgraph lib rs success ask for jot summary
    B32-->E((fn: ask_for_jot lib.rs))-->FvarJotSumary;
    %% have no summary from user
    FvarJotSumary(var: jot_summary =<br> String::new)==>GWhileLoop;
    %% WHILE LOOP
    GWhileLoop>while: fn: jot_summary.is_empty]-.->|true: is_empty|GWhilePrinter;
    %% Got summary from user
    GWhilePrinter(fn Jot::printer.input_header);
    G11(jot_summary =<br> Jot::reader.read_input)-.->GWhileLoop;
%%end

%%subgraph lib rs success ask for jot repo path
    GWhileLoop==>|false: is_empty, got summary|HRepoPath;
    HRepoPath(var: repo_path = Jot::cm.config_read Repo);
    HRepoPath==>HJOTgitinitrepo(fn: Jot::git.init repo_path <br>   .map_err _git_err_ Error::new<br>ErrorKind::InvalidInput, git_err);
    %% Open editor then, add, commit and push to git
%%end

%%subgraph lib rs success commit push summary
    %% Success
    %%FvarJotSumary-.->HSUCCESS;
    HSUCCESS>if git init success]==>I;
    I([fn: Jot::program_opener<br>.open_editor 'repo_path/README.md'])==>FINAL;
    FINAL([.and Jot::git_add_commit_push jot_summary]);

%%end


%% GIT(checkout_branch in gitrs).->GIT1-.->H;
 %% git2::Repository::open(Path::new(&repo_path)).map(|repo| self.repo = Some(repo))
%%subgraph git rs
    GITinit(fn: Git::Repository::open repo_path<br>.map _repo_ Git.repo = Some repo)==>|Ok|HSUCCESS;
    GITinit-.->|Err|HJOTgitinitrepo;
    HJOTgitinitrepo-->GITinit;
%%end


%%subgraph printer rs
    PRINT1-.->G11;
    GWhilePrinter-.->PRINT;
    PRINT(fn: Printer::println_styled)-.->PRINT1(fn: Printer::writer.flush);
%%end

classDef gray fill:#ccc,stroke:#ccc,stroke-width:2px
classDef cyan fill:#7bb,stroke:#ccc,stroke-width:2px
class E,I,FINAL gray
class FvarJotSumary,HRepoPath cyan
```
