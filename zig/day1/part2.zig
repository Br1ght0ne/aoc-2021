const std = @import("std");
const InputReader = @import("../utils.zig").InputReader;
const readMeasurement = @import("part1.zig").readMeasurement;

const MeasurementWindow = [3]u32;

fn shiftAndSum(window: *MeasurementWindow, newMeasurement: u32) u32 {
    std.mem.rotate(u32, window[0..], 1);
    window[window.len - 1] = newMeasurement;
    var sum: u32 = 0;
    for (window) |measurement| sum += measurement;
    return sum;
}

pub fn solve(input: InputReader) !u32 {
    var measurementWindow: MeasurementWindow = undefined;
    var measurementSum: u32 = 0;
    for (measurementWindow) |*measurement| {
        measurement.* = (try readMeasurement(input)) orelse unreachable;
        measurementSum += measurement.*;
    }
    var increases: u32 = 0;
    while (try readMeasurement(input)) |newMeasurement| {
        const prevMeasurementSum = measurementSum;
        measurementSum = shiftAndSum(&measurementWindow, newMeasurement);
        if (measurementSum > prevMeasurementSum) increases += 1;
    }
    return increases;
}

pub fn main() !void {
    const input = InputReader.initFile(std.io.getStdIn());
    const stdout = std.io.getStdOut().writer();
    try stdout.print("{d}\n", .{try solve(input)});
}
