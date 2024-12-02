//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.
const std = @import("std");

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("Welcome to Advent of Code, 2024 edition. Wolfgang E. Sanyer did this.\n", .{});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build run -- --help` to (maybe?) see documentation", .{});

    // Apparently arena.deinit() is easy? Chosen based on https://ziglang.org/documentation/master/#Choosing-an-Allocator
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    // Is this strictly necessary? does arena.deinit() take care of it?
    defer args.deinit();
    while (args.next()) |arg| {
        try stdout.print("Got arg {s}\n", .{arg});
    }

    try bw.flush(); // Don't forget to flush!
}
