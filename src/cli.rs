use crate::coordinator::Coordinator;
use crate::rbt::Rbt;
use anyhow::{Context, Result};
use clap::Parser;
use core::mem::MaybeUninit;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// [temporary] Instead of running the Rbt configuration from Roc, load
    /// it from this JSON.
    #[clap(long)]
    load_from_json: Option<PathBuf>,

    #[clap(long)]
    dump_to_json: bool,

    /// Use a runner that does not actually run any tasks. Only useful for
    /// rbt developers!
    #[clap(long)]
    use_fake_runner: bool,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let rbt: Rbt = match &self.load_from_json {
            Some(path) => Self::load_from_json(path).context("could not load from JSON")?,
            None => Self::load_from_roc(),
        };

        if self.dump_to_json {
            println!(
                "{}",
                serde_json::to_string(&rbt).context("could not dump to JSON")?
            )
        }

        let mut coordinator = Coordinator::default();
        coordinator.add_target(&rbt.default);

        let runner: Box<dyn crate::coordinator::Runner> = if self.use_fake_runner {
            Box::new(crate::fake_runner::FakeRunner::default())
        } else {
            Box::new(crate::runner::Runner::default())
        };

        while coordinator.has_outstanding_work() {
            coordinator.run_next(&runner).context("failed to run job")?;
        }

        Ok(())
    }

    pub fn load_from_roc() -> Rbt {
        let rbt = unsafe {
            let mut input = MaybeUninit::uninit();
            roc_init(input.as_mut_ptr());
            input.assume_init()
        };

        rbt.into()
    }

    pub fn load_from_json(path: &Path) -> Result<Rbt> {
        let file =
            File::open(path).with_context(|| format!("could not open {}", path.display()))?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).context("could not deserialize JSON into an rbt instance")
    }
}

extern "C" {
    #[link_name = "roc__initForHost_1_exposed"]
    fn roc_init(init: *mut crate::glue::Rbt);
}
