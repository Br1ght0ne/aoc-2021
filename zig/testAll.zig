const std = @import("std");

const day1 = @import("day1/test.zig");

test {
    std.testing.refAllDecls(@This());
}
