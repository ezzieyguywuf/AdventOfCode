//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.
const std = @import("std");

pub fn main() !u8 {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("Welcome to Advent of Code, 2024 edition. Wolfgang E. Sanyer did this.\n", .{});

    std.debug.print("Run `zig build run -- --help` to (maybe?) see documentation\n", .{});

    // Apparently arena.deinit() is easy? Chosen based on https://ziglang.org/documentation/master/#Choosing-an-Allocator
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    const problem = processArgs(allocator) catch return 1;

    std.debug.print("Got problem for day {d:02}\n", .{problem.day});
    std.debug.print("Got data: (see below)\n", .{});

    switch (problem.day) {
        1 => {
            const solution = try solveDay01(&problem.data);
            std.debug.print("Solution, Day01, parta: {d}\n", .{solution});
        },
        else => {
            std.debug.print("I don't yet know how to solve day {d:02}\n", .{problem.day});
        },
    }
    return 0;
}

fn solveDay01(data: *const lines) !u64 {
    var total: u64 = 0;
    for (data.*) |line| {
        std.debug.print("got line {s}\n", .{line});
        var it = std.mem.splitScalar(u8, line, ' ');
        const left = try parseIntFromOptional(it.next());
        const right = try parseIntFromOptional(it.next());

        const tail = it.next();
        if (tail) |trailing| {
            std.debug.print("Expect line to split into two values delimited by a space, got line {s} with trailing '{s}'\n", .{ line, trailing });
            return RuntimeError.TooMuchData;
        }

        if (left > right) {
            total = try std.math.add(u64, total, try std.math.sub(u64, left, right));
        } else if (right > left) {
            total = try std.math.add(u64, total, try std.math.sub(u64, right, left));
        }
    }

    return total;
}

fn parseIntFromOptional(val: ?string) !u64 {
    const unpacked = val orelse {
        std.debug.print("parseIntFromOptional expects a value, got null.\n", .{});
        return ValueError.UnexpectedNull;
    };

    return try std.fmt.parseInt(u64, unpacked, 10);
}

fn processArgs(allocator: std.mem.Allocator) !Problem {
    var args = try std.process.argsWithAllocator(allocator);
    // Is this strictly necessary? does arena.deinit() take care of it?
    defer args.deinit();

    var problem: Problem = Problem{
        .day = 0,
        .data = &.{},
    };
    while (args.next()) |arg| {
        // try out.print("Got arg {s}\n", .{arg});
        if (std.mem.eql(u8, arg, "--day") or std.mem.eql(u8, arg, "-d")) {
            const day_str = try nextArg("--day or -d", &args);
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

        if (std.mem.eql(u8, arg, "--filename") or std.mem.eql(u8, arg, "-f")) {
            const filename = try nextArg("--filename or -f", &args);
            problem.data = try readFile(allocator, filename);
        }
    }

    if (problem.day == 0) {
        std.debug.print("Please specify which day to solve for with --day or -d\n", .{});
        return ArgParseError.MissingArgument;
    }

    return problem;
}

// Caller is responsible for deinit'ing the lines ArrayList
fn readFile(allocator: std.mem.Allocator, filename: string) !lines {
    var dir = std.fs.cwd();
    defer dir.close();
    var targetFile = filename;

    if (std.fs.path.isAbsolute(targetFile)) {
        const dirname = std.fs.path.dirname(targetFile) orelse {
            std.debug.print("'{s}' looks like an absolute path, but cannot parse the dirname", .{targetFile});
            return FileReadError.CannotParseDirname;
        };
        targetFile = std.fs.path.basename(targetFile);

        dir = try std.fs.openDirAbsolute(dirname, .{});
    }

    const file = try dir.openFile(targetFile, .{});
    defer file.close();

    // stolen from https://stackoverflow.com/a/68879352
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;
    var out = std.ArrayList(string).init(allocator);
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        try out.append(try allocator.dupe(u8, line));
    }

    return out.toOwnedSlice();
}

fn nextArg(flagname: string, args: *std.process.ArgIterator) ArgParseError!string {
    return args.next() orelse {
        std.debug.print("Missing argument after flag {s}\n", .{flagname});
        return ArgParseError.MissingArgument;
    };
}

const Problem = struct {
    day: u8,
    data: lines,
};

const ArgParseError = error{
    MissingArgument,
    InvalidArgument,
};

const FileReadError = error{
    CannotParseDirname,
};

const ValueError = error{
    UnexpectedNull,
};

const RuntimeError = error{
    TooMuchData,
};

const string = []const u8;
const lines = []string;
