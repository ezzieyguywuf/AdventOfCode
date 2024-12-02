//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.
const std = @import("std");

pub fn main() !u8 {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("Welcome to Advent of Code, 2024 edition. Wolfgang E. Sanyer did this.\n", .{});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    try stdout.print("Run `zig build run -- --help` to (maybe?) see documentation\n", .{});

    // Apparently arena.deinit() is easy? Chosen based on https://ziglang.org/documentation/master/#Choosing-an-Allocator
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    const problem = process_args(allocator) catch return 1;
    try stdout.print("Got problem for day {d}\n", .{problem.day});

    try bw.flush(); // Don't forget to flush!

    return 0;
}

fn process_args(allocator: std.mem.Allocator) ArgParseError!Problem {
    var args = try std.process.argsWithAllocator(allocator);
    // Is this strictly necessary? does arena.deinit() take care of it?
    defer args.deinit();

    var problem: Problem = Problem{
        .day = 0,
        .data = "",
    };
    while (args.next()) |arg| {
        // try out.print("Got arg {s}\n", .{arg});
        if (std.mem.eql(u8, arg, "--day") or std.mem.eql(u8, arg, "-d")) {
            const day_str = args.next() orelse {
                std.debug.print("When using --day or -d, please specify which day to solve for after, e.g. `--day 1`\n", .{});
                return ArgParseError.MissingArgument;
            };
            const day = std.fmt.parseInt(u8, day_str, 10) catch |err| switch (err) {
                error.InvalidCharacter => {
                    std.debug.print("Unable to convert '{s}' into an integer\n", .{day_str});
                    return ArgParseError.InvalidArgument;
                },
                error.Overflow => {
                    std.debug.print("Expect a value between 1 and 31 inclusive, instead `{s}` overflowed a u8\n", .{day_str});
                    return ArgParseError.InvalidArgument;
                },
            };
            if (day == 0 or day > 31) {
                std.debug.print("Expected a --day value between 1 and 31 inclusive, got {d}\n", .{day});
                return ArgParseError.InvalidArgument;
            }
            problem.day = day;
        }
    }

    return problem;
}

const Problem = struct {
    day: u8,
    data: []const u8,
};

const ArgParseError = error{
    MissingArgument,
    InvalidArgument,
};
