const std = @import("std");
const readInputFile = @import("utils.zig").readInputFile;

pub fn part1(input: []u32) u32 {
    var count: u32 = 0;
    for (input[1..]) |measurement, i| {
        if (measurement > input[i]) {
            count += 1;
        }
    }
    return count;
}

pub fn part2(input: []u32) !u32 {
    var slidingSumList = try std.ArrayList(u32).initCapacity(std.heap.page_allocator, input.len - 2);
    for (input[2..]) |measurement, i| {
        try slidingSumList.append(input[i] + input[i + 1] + measurement);
    }
    return part1(slidingSumList.toOwnedSlice());
}

test "part 1" {
    const inputFile = readInputFile("../inputs/1.txt");
    const input = try inputFile.readAllInts();

    const expectedPart1: u32 = 1154;
    try std.testing.expectEqual(expectedPart1, part1(input));
}

test "part 2" {
    const inputFile = readInputFile("../inputs/1.txt");
    const input = try inputFile.readAllInts();

    const expectedPart2: u32 = 1127;
    try std.testing.expectEqual(expectedPart2, try part2(input));
}
