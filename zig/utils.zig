const std = @import("std");
const allocator = std.heap.page_allocator;
const File = std.fs.File;

pub const InputReader = struct {
    source: File,
    buf: []u8,

    pub fn initPath(path: []const u8) !InputReader {
        const source = try std.fs.cwd().openFile(path, .{ .read = true });
        return initFile(source);
    }

    pub fn initFile(source: File) InputReader {
        var buf: [1024]u8 = undefined;
        return InputReader{
            .source = source,
            .buf = buf[0..],
        };
    }

    pub fn readLine(self: InputReader) !?[]u8 {
        return self.source.reader().readUntilDelimiterOrEof(self.buf, '\n');
    }
};

pub fn readInputFile(comptime path: []const u8) InputReader {
    return (InputReader.initPath(path) catch @panic("no input file"));
}
