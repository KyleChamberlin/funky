# Architecture

## Design Philosophy

Funky's core principle is simple: **live inside the ecosystem, don't replace it.**

Your terminal already has a rich set of tools — a shell that loads functions, an editor you've configured, a history file that remembers what you've typed. Funky doesn't try to compete with any of these. It wires into them.

### We Use Your Editor

When you run `funky edit`, Funky doesn't open a custom UI or a built-in editor. It resolves your editor the same way Git does — `$VISUAL`, then `$EDITOR`, then falls back to `vim` or `nano`. If you've set up VS Code as your editor, that's what opens. If you're a `helix` person, that's what opens. Funky doesn't care and shouldn't.

### We Use Your Shell

Functions created by Funky aren't Funky functions — they're zsh functions (or fish functions, or bash functions). They use the shell's own `fpath` and `autoload` machinery. If you uninstall Funky, every function you created keeps working. There's no runtime, no daemon, no shim sitting between you and your shell. Funky generates the file and gets out of the way.

### We Use Your History

When you say `--from history`, Funky reads `$HISTFILE` — the same file your shell writes to. It doesn't maintain a shadow history or ask you to log commands through it. Your workflow doesn't change. You run commands as you always have, and when one is worth keeping, Funky captures it from the record your shell already made.

### We Use Your Filesystem

Functions are plain text files, one per function, in a directory you control (`~/.funky/` by default). There's no database, no lock file, no index. You can `cat` a function, `grep` across them, `rsync` them to another machine, or check them into Git. They're files — you already know how to manage files.

### The Goal Is to Melt Away

The best tool is the one you forget is there. Once you've created a function with Funky, you never interact with Funky again to use it. You just type the function name. Funky did its job at creation time and then disappeared. The function lives in your shell's native infrastructure, invoked by your shell's native function call mechanism. There's nothing in between.

---

## Code Organization

The codebase is organized around clear boundaries. Each concern is isolated so that extending Funky — adding a shell, adding a source, changing storage — means touching one module, not rewriting the world.

```
src/
├── main.rs              # Entry point: parse args → dispatch command
├── args.rs              # CLI definition (clap derive)
├── file.rs              # Path expansion and validation
├── commands/            # One module per subcommand
│   ├── init.rs          #   Shell configuration
│   ├── new.rs           #   Function creation + source resolution
│   ├── list.rs          #   Enumerate stored functions
│   └── edit.rs          #   Open function in editor
└── functions/           # Shell function abstractions
    ├── mod.rs           #   FunctionSpec, Function trait, Slug type
    ├── repository.rs    #   Repository trait + filesystem implementation
    └── zsh.rs           #   Zsh-specific function generation
```

### The `Function` Trait

The central abstraction for shell support. Each shell implements this trait:

```rust
pub trait Function {
    fn create(&self, spec: &FunctionSpec) -> Result<()>;
}
```

`FunctionSpec` carries the function's name, command body, and arguments. The implementing struct (like `Zsh`) decides how to turn that into the shell's native syntax.

`Zsh` is parameterized by a `Repository`, which handles storage. This separation means you can test function generation without touching the filesystem — the test suite uses a `MockRepository` for exactly this.

### Templates

Each shell has a [Tera](https://keats.github.io/tera/) template in `template/functions/` that defines the output format:

```
template/
├── functions/
│   ├── zsh.tera     # Zsh function syntax
│   └── fish         # Fish function syntax (planned)
└── zsh/
    └── config       # Zsh init snippet (fpath + autoload)
```

Templates keep shell dialect concerns out of Rust code. The zsh template generates a standard autoloadable function file. When fish support lands, it'll have its own template producing `function ... end` syntax.

### The `Repository` Trait

Storage is abstracted behind a trait:

```rust
pub trait Repository {
    fn create(&self, id: &Slug, contents: &str) -> Result<()>;
    fn list(&self) -> Result<Vec<String>>;
    fn read(&self, id: &Slug) -> Result<String>;
    fn update(&self, id: &Slug, contents: &str) -> Result<()>;
}
```

`FileSystemRepository` is the only production implementation — one `.zsh` file per function in a directory. But the trait exists so the rest of the code doesn't know or care that it's a filesystem. Tests use `MockRepository` to verify behavior without disk I/O.

### Source Resolution

The `FunctionSource` enum in `args.rs` defines where commands come from:

```rust
pub enum FunctionSource {
    History,    // Last line of $HISTFILE
    StdIn,      // Piped input
    Clipboard,  // System clipboard (planned)
    Vargs,      // CLI arguments after --
}
```

Source resolution lives in `commands/new.rs` in a single `match` block. Each variant reads the command from its source and returns a `String`. Adding a new source means adding an enum variant and a match arm — nothing else changes.

### Extending Funky

| Task | What to touch |
|------|---------------|
| **Add a shell** | New template in `template/functions/`, new struct implementing `Function` in `src/functions/`, init snippet in `template/{shell}/config`, match arm in `main.rs` |
| **Add a source** | New `FunctionSource` variant in `args.rs`, match arm in `commands/new.rs` |
| **Change storage** | New `Repository` implementation — all existing code works unchanged |
| **Add a command** | New `Command` variant in `args.rs`, new module in `src/commands/`, match arm in `main.rs` |

Each extension is additive. Existing code doesn't change to accommodate new shells, new sources, or new storage backends.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `tera` | Template rendering |
| `serde` | Serialization for template context |
| `color-eyre` | Error reporting |
| `shellexpand` | Path expansion (`~`, env vars) |
| `slug` | Name normalization |
| `ctrlc` | Graceful Ctrl+C handling |
| `tempfile` | Safe temp files for editing |
