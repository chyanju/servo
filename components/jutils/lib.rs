/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use app_units::Au;

use std::io::{stdin, stdout, Read, Write};
pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn profile_compare_au(filename: &str, funcname: &str, attrname: &str, before: &Au, after: &Au) {
    println!("----[profile] {:?}, {:?}, {:?}, before={:?}, after={:?}, same={:?}",  
        filename,
        funcname,
        attrname,
        before,
        after,
        before==after
    );
}