mod args;
mod estimate;
use args::HarmonicArgs;
use clap::Parser;

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-12

rust-paftools: Building the entire paf tools in the rust for pangenome
construction from the metagenome and the pangenome and the pantranscriptomics.
This allows for the estimation of the harmonic means from the closely diverged
pangenome aligned to the same query genome. You can pass the aligned length from
the same query to different sub-species and then you can estimate the harmonic mean
of the query against all the species.

* */

fn main() {
    let args = HarmonicArgs::parse();
    let result = estimate::estimate(&args.harmonicmean).unwrap();
    println!(
        "The harmonic estimates for the same query across the
           different species have been written: {}",
        result
    );
}
