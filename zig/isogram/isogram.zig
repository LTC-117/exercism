pub fn isIsogram(str: []const u8) bool {
    var mtx: [128]u8 = [_]u8{0} ** 128;

    for (str) |i| {
        if (i == ' ' or i == '-') {
            continue;
        } else if (i < 97) {
            mtx[i + 32] += 1;
        }

        mtx[i] += 1;
    }

    for (mtx) |ch| {
        if (ch > 1) {
            return false;
        }
    }

    return true;
}
