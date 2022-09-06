
use rtl_proc::*;
use rtl_dsl::*;

rtl!(
    Bundle WritePort { 
        input en:   Bool;
        input addr: UInt<5>;
        input data: UInt<32>;
    };

    Bundle ReadPort {
        input  addr: UInt<5>;
        output data: UInt<32>;
    };

    Module RegisterFile {
        ports {
            io rp1: Bundle<ReadPort>;
            io rp2: Bundle<ReadPort>;
            input wp1: Bundle<WritePort>;
        };
        comb {
            foo := bar;
        };
    };
);

fn main() {
}
