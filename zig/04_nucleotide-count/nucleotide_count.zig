pub const NucleotideError = error{Invalid};

pub const Counts = struct {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
};

pub fn countNucleotides(s: []const u8) NucleotideError!Counts {
    var nucleo = Counts {
        .a = 0,
        .c = 0,
        .g = 0,
        .t = 0
    };

    for (s) |i| {
        switch (i) {
            'A' => nucleo.a += 1,
            'C' => nucleo.c += 1,
            'G' => nucleo.g += 1,
            'T' => nucleo.t += 1,
            else => return NucleotideError.Invalid
        }
    }

    return nucleo;
}
