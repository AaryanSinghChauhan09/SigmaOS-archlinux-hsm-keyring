// Integration test for SigmaOS-archlinux-hsm-keyring
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_archlinux_hsm_keyring as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
