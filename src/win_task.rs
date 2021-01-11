struct WinTask {}
// XXX needs APIs to loopback mount isofs, create partition tables, create filesystems, copy files
// from iso to partition
// - Unless this can all be done in pure rust
// - would be difficult to implement ntfs in rust, or split file for fatfs
//   * othewise, extracting files from a iso, creating a fatfs that won't be modified is probably
//   doable in Rust...
// Looks like rufus has code to extract WIM, and no ntfs code: https://github.com/pbatard/rufus/blob/master/src/vhd.c
// - uses winapi
// wimlib can split a .WIM (as well as mounting, etc.)
// https://github.com/rafalh/rust-fatfs
