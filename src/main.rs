use std::process::ExitCode;

use std::io;

fn sub() -> Result<(), io::Error> {
    rs_keyval2asn1::stdin2key2env2val2der2stdout_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
