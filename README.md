# About
Implementation of Canfield-Dafilou and Abel's ["Group Delay-Based Allpass
Filters for Abstract Sound Synthesis and Audio Effect Processing"][paper].

# Usage
First build the package itself with `cargo build --release`, then create some
_example.rs_ file inside the _irs/_ directory with a `fn tau(w: f64) -> f64`
function, like 
``` rust
fn tau(w: f64) -> f64 {
    48000. * (50. * w).cos()
}
```

To build the example, run `./build.sh example.rs`. The generated executable
_example_ now accepts two parameters: an output file and a number of samples for
the impulse response. By default the output wav file will have a sample rate of
48000 _Hz_.

[paper]: https://www.dafx.de/paper-archive/2018/papers/DAFx2018_paper_22.pdf
