use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct HarmonicArgs {
    /// please provide the path to the snp estimation file across the pangenome
    pub harmonicmean: String,
}
