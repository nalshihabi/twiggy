// Fun times ahead!
//
// Apparently, proc-macros don't play well with `cfg_attr` yet, and their
// combination is buggy. So we can't use cfg_attr to choose between
// `wasm-bindgen` and `structopt` depending on if we're building the CLI or the
// wasm API respectively. Instead, we have `build.rs` remove unwanted attributes
// for us by invoking `grep`.
//
// It's terrible! But it works for now.

/// Options for configuring `twiggy`.
#[derive(Clone, Debug)]
#[derive(StructOpt)]
#[structopt(about = "\n`twiggy` is a code size profiler.\n\nIt analyzes a binary's call graph to answer questions like:\n\n* Why was this function included in the binary in the first place?\n\n* What is the retained size of this function? I.e. how much space\n  would be saved if I removed it and all the functions that become\n  dead code after its removal.\n\nUse `twiggy` to make your binaries slim!")]
pub enum Options {
    /// List the top code size offenders in a binary.
    #[structopt(name = "top")]
    Top(Top),

    /// Compute and display the dominator tree for a binary's call graph.
    #[structopt(name = "dominators")]
    Dominators(Dominators),

    /// Find and display the call paths to a function in the given binary's call
    /// graph.
    #[structopt(name = "paths")]
    Paths(Paths),
}

/// List the top code size offenders in a binary.
#[derive(Clone, Debug, Default)]
#[derive(StructOpt)]
#[wasm_bindgen]
pub struct Top {
    /// The path to the input binary to size profile.
    #[structopt(parse(from_os_str))]
    pub input: path::PathBuf,

    /// The destination to write the output to. Defaults to `stdout`.
    #[structopt(short = "o", default_value = "-")]
    pub output_destination: OutputDestination,

    /// The format the output should be written in.
    #[structopt(short = "f", long = "format", default_value = "text")]
    pub output_format: traits::OutputFormat,

    /// The maximum number of items to display.
    #[structopt(short = "n")]
    pub number: Option<u32>,

    /// Display retaining paths.
    #[structopt(short = "r", long = "retaining-paths")]
    pub retaining_paths: bool,

    /// Sort list by retained size, rather than shallow size.
    #[structopt(long = "retained")]
    pub retained: bool,
}

#[wasm_bindgen]
impl Top {
    /// Construct a new, default `Top`.
    pub fn new() -> Top {
        Top::default()
    }

    /// The maximum number of items to display.
    pub fn number(&self) -> u32 {
        self.number.unwrap_or(u32::MAX)
    }

    /// Display retaining paths.
    pub fn retaining_paths(&self) -> bool {
        self.retaining_paths
    }

    /// Sort list by retained size, rather than shallow size.
    pub fn retained(&self) -> bool {
        self.retained
    }

    /// Set the maximum number of items to display.
    pub fn set_number(&mut self, n: u32) {
        self.number = Some(n);
    }

    /// Set whether to display and compute retaining paths.
    pub fn set_retaining_paths(&mut self, do_it: bool) {
        self.retaining_paths = do_it;
    }

    /// Set whether to sort list by retained size, rather than shallow size.
    pub fn set_retained(&mut self, do_it: bool) {
        self.retained = do_it;
    }
}

/// Compute and display the dominator tree for a binary's call graph.
#[derive(Clone, Debug, Default)]
#[derive(StructOpt)]
#[wasm_bindgen]
pub struct Dominators {
    /// The path to the input binary to size profile.
    #[structopt(parse(from_os_str))]
    pub input: path::PathBuf,

    /// The destination to write the output to. Defaults to `stdout`.
    #[structopt(short = "o", default_value = "-")]
    pub output_destination: OutputDestination,

    /// The format the output should be written in.
    #[structopt(short = "f", long = "format", default_value = "text")]
    pub output_format: traits::OutputFormat,

    /// The maximum depth to print the dominators tree.
    #[structopt(short = "d")]
    pub max_depth: Option<u32>,

    /// The maximum number of rows, regardless of depth in the tree, to display.
    #[structopt(short = "r")]
    pub max_rows: Option<u32>,
}

#[wasm_bindgen]
impl Dominators {
    /// Construct a new, default `Dominators`.
    pub fn new() -> Dominators {
        Dominators::default()
    }

    /// The maximum depth to print the dominators tree.
    pub fn max_depth(&self) -> u32 {
        self.max_depth.unwrap_or(u32::MAX)
    }

    /// The maximum number of rows, regardless of depth in the tree, to display.
    pub fn max_rows(&self) -> u32 {
        self.max_rows.unwrap_or(u32::MAX)
    }

    /// Set the maximum depth to print the dominators tree.
    pub fn set_max_depth(&mut self, max_depth: u32) {
        self.max_depth = Some(max_depth);
    }

    /// Set the maximum number of rows, regardless of depth in the tree, to display.
    pub fn set_max_rows(&mut self, max_rows: u32) {
        self.max_rows = Some(max_rows);
    }
}

/// Find and display the call paths to a function in the given binary's call
/// graph.
#[derive(Clone, Debug)]
#[derive(StructOpt)]
#[wasm_bindgen]
pub struct Paths {
    /// The path to the input binary to size profile.
    #[structopt(parse(from_os_str))]
    pub input: path::PathBuf,

    /// The functions to find call paths to.
    pub functions: Vec<String>,

    /// The destination to write the output to. Defaults to `stdout`.
    #[structopt(short = "o", default_value = "-")]
    pub output_destination: OutputDestination,

    /// The format the output should be written in.
    #[structopt(short = "f", long = "format", default_value = "text")]
    pub output_format: traits::OutputFormat,

    /// The maximum depth to print the paths.
    #[structopt(short = "d", default_value = "10")]
    pub max_depth: u32,

    /// The maximum number of paths, regardless of depth in the tree, to display.
    #[structopt(short = "r", default_value = "10")]
    pub max_paths: u32,
}

impl Default for Paths {
    fn default() -> Paths {
        Paths {
            input: Default::default(),
            functions: Default::default(),
            output_destination: Default::default(),
            output_format: Default::default(),
            max_depth: 10,
            max_paths: 10,
        }
    }
}

#[wasm_bindgen]
impl Paths {
    /// Construct a new, default `Paths`.
    pub fn new() -> Paths {
        Paths::default()
    }

    // TODO: wasm-bindgen doesn't support sending Vec<String> across the wasm
    // ABI boundary yet.
    //
    // /// The functions to find call paths to.
    // pub fn functions(&self) -> Vec<String> {
    //     self.functions.clone()
    // }
    //
    // /// Set the functions to find call paths to.
    // pub fn set_functions(&mut self, functions: Vec<String>) {
    //     self.functions = functions;
    // }

    /// Add a function to find call paths for.
    pub fn add_function(&mut self, function: String) {
        self.functions.push(function);
    }

    /// The maximum depth to print the paths.
    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    /// The maximum number of paths, regardless of depth in the tree, to display.
    pub fn max_paths(&self) -> u32 {
        self.max_paths
    }

    /// Set the maximum depth to print the paths.
    pub fn set_max_depth(&mut self, max_depth: u32) {
        self.max_depth = max_depth;
    }

    /// Set the maximum number of paths, regardless of depth in the tree, to display.
    pub fn set_max_paths(&mut self, max_paths: u32) {
        self.max_paths = max_paths;
    }
}