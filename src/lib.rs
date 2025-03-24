//! # Restriction Enzyme Data
//!
//! The data in this package are derived from the [Biopython Restriction Dictionary](https://github.com/biopython/biopython/blob/master/Bio/Restriction/Restriction_Dictionary.py)
//! which in turn originate from [Rebase](http://rebase.neb.com).
//!
//! Those seeking to distribute REBASE files with their software packages are welcome to do so, providing it is clear to your users that they are not being charged for the REBASE data. It should be transparent that REBASE is a free and independent resource, with the following bibliographical reference:
//!  LATEST REVIEW: PDF file...
//!  Roberts, R.J., Vincze, T., Posfai, J., Macelis, D.
//!  REBASE-a database for DNA restriction and modification: enzymes, genes and genomes.
//!  Nucleic Acids Res. 43: D298-D299 (2015).
//!
//!
//!
//! name  pattern  len ncuts blunt c1 c2 c3 c4
//! Where:
//! name = name of enzyme
//! pattern = recognition site
//! len = length of pattern
//! ncuts = number of cuts made by enzyme
//!      Zero represents unknown
//! blunt = true if blunt end cut, false if sticky
//! c1 = First 5' cut
//! c2 = First 3' cut
//! c3 = Second 5' cut
//! c4 = Second 3' cut
//!
//! Examples:
//! AAC^TGG -> 6 2 1 3 3 0 0
//! A^ACTGG -> 6 2 0 1 5 0 0
//! AACTGG  -> 6 0 0 0 0 0 0
//! AACTGG(-5/-1) -> 6 2 0 1 5 0 0
//! (8/13)GACNNNNNNTCA(12/7) -> 12 4 0 -9 -14 24 19
//!
//! i.e. cuts are always to the right of the given
//! residue and sequences are always with reference to
//! the 5' strand.
//! Sequences are numbered ... -3 -2 -1 1 2 3 ... with
//! the first residue of the pattern at base number 1.
//!

pub mod enzyme_trait;
pub mod enzymes;
// pub mod rebase;

pub use enzyme_trait::RestrictionEnzymeTrait;
