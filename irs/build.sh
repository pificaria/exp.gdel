#!/bin/sh

TAU=""

if [ -z "$1" -o ! -f "$1" ]; then
	echo "$0 file"
	exit 1
fi

TAU="$1"
TAUEXEC=${TAU%.rs}
TAUTMP="tmp_build_$TAU" 
TAUCONTENTS=`cat $TAU`

cat > $TAUTMP << EOF
extern crate exp1_gdel;
use exp1_gdel::gdel_writer;

$TAUCONTENTS

fn help(arg0: &str) -> ! {
    use std::process::exit;
    println!("{} output [n] [opa]", arg0);
    exit(1)
}

fn main() {
    use std::env::args;

    let a: Vec<String> = args().collect();
    if a.len() < 2 {
        help(&a[0]);
    } 
    
    let mut time = 16;
    let mut opa = false;
    let path = &a[1];
    if a.len() >= 3 {
        if let Ok(v) = a[2].parse::<u32>() {
            time = v.max(4).min(20);
        }
    }

    if a.len() >= 4 {
        if let Ok(v) = a[3].parse::<bool>() {
            opa = v;
        }
    }

    gdel_writer(path, time, opa, tau);
}
EOF

rustc -o "$TAUEXEC" "$TAUTMP" -L ../../target/release -L ../../target/release/deps/
rm "$TAUTMP"
