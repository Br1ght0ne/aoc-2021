const std = @import("std");
const InputReader = @import("../utils.zig").InputReader;

pub fn readMeasurement(input: InputReader) !?u32 {
    if (try input.readLine()) |line| {
        const n: u32 = try std.fmt.parseInt(u32, line, 10);
        return n;
    } else {
        return null;
    }
}

pub fn solve(input: InputReader) !u32 {
    var prev_measurement: u32 = (try readMeasurement(input)) orelse unreachable;
    var increases: u32 = 0;
    while (try readMeasurement(input)) |measurement| {
        if (measurement > prev_measurement) {
            increases += 1;
        }
        prev_measurement = measurement;
    }
    return increases;
}

pub fn main() !void {
    const input = InputReader.initFile(std.io.getStdIn());
    const stdout = std.io.getStdOut().writer();
    try stdout.print("{d}\n", .{try solve(input)});
}
