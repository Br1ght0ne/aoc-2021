const std = @import("std");
const utils = @import("../utils.zig");

const part1 = @import("part1.zig");
const part2 = @import("part2.zig");

test "day 1 part 1" {
    var input = utils.readInputFile("../../inputs/1.txt");
    try std.testing.expectEqual(try part1.solve(input), 1154);
}

test "day 1 part 2" {
    var input = utils.readInputFile("../../inputs/1.txt");
    try std.testing.expectEqual(try part2.solve(input), 1127);
}
