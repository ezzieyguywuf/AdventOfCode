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

    const problem = try processArgs(allocator);

    std.debug.print("Got problem for day {d:02}\n", .{problem.day});

    switch (problem.day) {
        1 => {
            const solutionA = try solveDay01PartA(allocator, problem.data);
            std.debug.print("Solution, Day01, partA: {d}\n", .{solutionA});
            const solutionB = try solveDay01PartB(allocator, problem.data);
            std.debug.print("Solution, Day01, partB: {d}\n", .{solutionB});
        },
        2 => {
            const solutionA = try solveDay02(allocator, problem.data, false);
            std.debug.print("Solution, Day02, partA: {d}\n", .{solutionA});
            const solutionB = try solveDay02(allocator, problem.data, true);
            std.debug.print("Solution, Day02, partB: {d}\n", .{solutionB});
        },
        3 => {
            const solutionA = try solveDay03(problem.data);
            std.debug.print("Solution, Day03, partA: {d}\n", .{solutionA});
        },
        else => {
            std.debug.print("I don't yet know how to solve day {d:02}\n", .{problem.day});
        },
    }
    return 0;
}

fn solveDay03(data: lines) !u64 {
    var total: u64 = 0;
    // slide a window searching for `mul(`
    for (data) |line| {
        var i: usize = 0;
        while (i < line.len) {
            // std.debug.print("i: {d}, c: {c}\n", .{ i, line[i] });
            // match mul(
            if (i >= line.len - 4) {
                // std.debug.print("  cannot check for mul(, breaking, i = {d}\n", .{i});
                break;
            }
            const mul = line[i .. i + 4];
            if (!std.mem.eql(u8, mul, "mul(")) {
                i = incrementIndex(i, 1, line) catch break;
                continue;
            }
            // std.debug.print("  found mul(, moving on\n", .{});
            i = incrementIndex(i, 3, line) catch break;

            // match ###
            const consumedInteger1 = consumeInteger(i, line);
            if (!consumedInteger1.has_data) {
                // std.debug.print("  did not find number at i = {d}, moving on\n", .{i});
                // i = incrementIndex(i, 1, line) catch break;
                continue;
            }
            if (consumedInteger1.i >= line.len) break;

            // match ,
            // i = incrementIndex(consumedInteger1.i, 1, line) catch break;
            if (line[consumedInteger1.i] != ',') {
                // std.debug.print("  did not find , at i = {d}, moving on\n", .{i});
                continue;
            }

            // match ###
            const consumedInteger2 = consumeInteger(consumedInteger1.i, line);
            if (!consumedInteger2.has_data) {
                // std.debug.print("  did not find number at i = {d}, moving on\n", .{i});
                // i = incrementIndex(i, 1, line) catch break;
                continue;
            }
            if (consumedInteger2.i >= line.len) break;

            // match )
            // i = incrementIndex(consumedInteger2.i, 1, line) catch break;
            if (line[consumedInteger2.i] != ')') {
                // std.debug.print("  did not find ) at i = {d}, moving on\n", .{i});
                continue;
            }

            // std.debug.print("  multiplying {d} by {d} to add to total of {d}\n", .{
            //     consumedInteger1.val,
            //     consumedInteger2.val,
            //     total,
            // });

            total += consumedInteger1.val * consumedInteger2.val;

            i = incrementIndex(consumedInteger2.i, 1, line) catch break;
        }
    }
    return total;
}

const ConsumedInteger = struct {
    i: usize = 0,
    val: u64 = 0,
    has_data: bool = false,
};

fn incrementIndex(i: usize, amt: usize, data: string) !usize {
    if (i + amt < data.len) {
        return i + amt;
    }

    return ValueError.OutOfBounds;
}

fn consumeInteger(i: usize, data: string) ConsumedInteger {
    var out: ConsumedInteger = .{
        .i = i,
        .val = 0,
        .has_data = false,
    };

    var n: usize = 3;
    var operand: [3:0]u8 = .{'0'} ** 3;
    // std.debug.print("  next 5 chars: {s}\n", .{data[i .. i + 5]});
    while (n > 0) {
        out.i = incrementIndex(out.i, 1, data) catch return out;
        const char = data[out.i];
        // std.debug.print("  char: {c}\n", .{char});
        if (char == ',' or char == ')') {
            // std.debug.print("  breaking, found {c} at i = {d}\n", .{ char, out.i });
            break;
        }

        // std.debug.print("  Trying to parse {c} as int\n", .{char});
        _ = std.fmt.parseInt(u8, &.{char}, 10) catch return out;

        // shift all values in the array left by one, then "append" the new
        // value to the end.
        std.mem.rotate(u8, &operand, 1);
        operand[2] = char;
        // std.debug.print("  ...success, operand currently {s}\n", .{operand});
        n -= 1;
    }

    if (n == 3) {
        return out;
    }

    // std.debug.print("  trying to parse '{s}'\n", .{operand});
    const num = std.fmt.parseInt(u64, &operand, 10) catch return out;
    // std.debug.print("  Found number, {d}\n", .{num});

    // out.i = incrementIndex(out.i, 1, data) catch return out;
    out.val = num;
    out.has_data = true;

    // std.debug.print("  returning out.i: {d}\n", .{out.i});

    return out;
}

test "day03" {
    // 2
    // 4
    // 3
    // 7
    // 5
    // 5
    // 32
    // 64
    // 11
    // 8
    // 8
    // 5
    const data: [1]string = .{
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    };

    try std.testing.expectEqual(161, try solveDay03(&data));
}

// NOTE: I never actually finished this, I just brute-forced it in c++ somewhere
fn solveDay02(allocator: std.mem.Allocator, data: lines, allow_dampening: bool) !u64 {
    var total: u64 = 0;

    for (data) |line| {
        // std.debug.print("LINE: {s}\n", .{line});
        var levels = std.ArrayList(u64).init(allocator);
        defer levels.deinit();
        var it = std.mem.splitScalar(u8, line, ' ');

        while (it.next()) |val| {
            try levels.append(try std.fmt.parseInt(u64, val, 10));
        }

        var isSafe = false;
        var i_prev: ?u64 = null;
        var i: usize = 0;
        var i_next: ?u64 = 1;
        var dampened = false;

        while (i < levels.items.len and i_next != null) {
            if (i_prev == null) {
                i_prev = if (i > 0) i - 1 else null;
            }
            const prev: ?u64 = if (i_prev) |index| levels.items[index] else null;
            const cur = levels.items[i];
            const next: ?u64 = if (i_next) |index| levels.items[index] else null;

            isSafe = checkSafe(prev, cur, next);

            // std.debug.print("  isSafe: {any}, i: {d}, dampened: {any}, prev: {?d}, cur: {d}, next: {?d}\n", .{ isSafe, i, dampened, prev, cur, next });
            if (!isSafe) {
                if (allow_dampening and !dampened) {
                    // try removing cur
                    const localCur = next;
                    const localNext = if (i < (levels.items.len - 2)) levels.items[i + 2] else null;
                    // std.debug.print("  tried removing cur, prev: {?d}, localCur: {?d}, localNext: {?d}\n", .{ prev, localCur, localNext });
                    if (localCur != null and checkSafe(prev, localCur.?, localNext)) {
                        // std.debug.print("  dampened\n", .{});
                        dampened = true;
                        isSafe = true;
                        i += 1;
                    }
                }
                if (allow_dampening and !dampened) {
                    // try remove prev
                    const local_i_prev = if (i > 1) i - 2 else null;
                    const localPrev = if (local_i_prev) |index| levels.items[index] else null;
                    // std.debug.print("  tried removing prev localPrev: {?d}, cur: {d}, next: {?d}\n", .{ localPrev, cur, next });

                    if (checkSafe(localPrev, cur, next)) {
                        // std.debug.print("  dampened2\n", .{});
                        dampened = true;
                        isSafe = true;
                        i_prev = local_i_prev;
                    }
                }
                if (allow_dampening and !dampened) {
                    // try remove next
                    const local_i_next = if (i < (levels.items.len - 2)) i + 2 else null;
                    const localNext = if (local_i_next) |index| levels.items[index] else null;
                    // std.debug.print("  tried removing next prev: {?d}, cur: {d}, localNext: {?d}\n", .{ prev, cur, localNext });
                    if (checkSafe(prev, cur, localNext)) {
                        // std.debug.print("  dampened3\n", .{});
                        dampened = true;
                        isSafe = true;
                        // this is incremented below;
                        if (local_i_next) |val| {
                            // this is incremented below
                            i = val - 1;
                        }
                    }
                }
            }

            if (!isSafe) {
                break;
            }

            i_prev = i;
            i += 1;

            // std.debug.print("  i_next: {?d}, levels.items.len: {d}\n", .{ i_next, levels.items.len });
            if (i < levels.items.len - 1) {
                i_next = i + 1;
            } else {
                i_next = null;
            }
        }

        if (isSafe) {
            total += 1;
            // std.debug.print("  increasing total, new total: {d}\n", .{total});
            // std.debug.print("SAFE   {s}\n", .{line});
        } else {
            // std.debug.print("  NOT INCREASING, total still: {d}, LINE: {s}\n", .{ total, line });
            // std.debug.print("UNSAFE {s}\n", .{line});
        }
    }

    return total;
}

fn checkSafe(left: ?u64, mid: u64, right: ?u64) bool {
    // std.debug.print("    checking safe, left: {?d}, mid: {d}, right: {?d}\n", .{ left, mid, right });
    if (left == null and right == null) {
        return true;
    }

    if (left != null and right != null) {
        const leftOrdinality = calculateOrdinality(left.?, mid);
        const rightOrdinality = calculateOrdinality(mid, right.?);
        if (leftOrdinality != rightOrdinality) {
            return false;
        }
    }

    // std.debug.print("    checking left\n", .{});
    if (left != null) {
        // std.debug.print("    left not null\n", .{});
        if (left.? == mid) {
            // std.debug.print("    left equals mid, returning false\n", .{});
            return false;
        }
        const diff = calculateDiff(left.?, mid);
        // std.debug.print("    diff: {d}\n", .{diff});
        if (diff < 1 or diff > 3) {
            return false;
        }
    }
    if (right != null) {
        if (mid == right.?) {
            return false;
        }
        const diff = calculateDiff(mid, right.?);
        if (diff < 1 or diff > 3) {
            return false;
        }
    }

    return true;
}

fn calculateOrdinality(left: u64, right: u64) Ordinality {
    if (left < right) {
        return Ordinality.Increasing;
    } else if (left > right) {
        return Ordinality.Decreasing;
    }

    return Ordinality.Equal;
}

fn calculateDiff(left: u64, right: u64) u64 {
    const ordinality = calculateOrdinality(left, right);

    return switch (ordinality) {
        Ordinality.Increasing => right - left,
        Ordinality.Decreasing => left - right,
        else => 0,
    };
}

const Ordinality = enum {
    Increasing,
    Decreasing,
    Equal,
};

test "day02" {
    const data: [6]string = .{
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    };
    try std.testing.expectEqual(2, try solveDay02(std.testing.allocator, &data, false));
    try std.testing.expectEqual(4, try solveDay02(std.testing.allocator, &data, true));
}

// test "day02, edge cases" {
//     const n: usize = 20;
//     const data: [n]string = .{
//         "7 7 6 5 4 3",
//         "20 7 6 5 4 3",
//         "1 2 3 4 5 5",
//         "1 2 3 4 5 20",
//         // stolen from reddit
//         "48 46 47 49 51 54 56",
//         "1 1 2 3 4 5",
//         "1 2 3 4 5 5",
//         "5 1 2 3 4 5",
//         "1 4 3 2 1",
//         "1 6 7 8 9",
//         "1 2 3 4 3",
//         "9 8 7 6 7",
//         "7 10 8 10 11",
//         "29 28 27 25 26 25 22 20",
//         "7 10 8 10 11",
//         "29 28 27 25 26 25 22 20",
//         "31 34 32 30 28 27 24 22",
//         "75 77 72 70 69",
//         "7 10 8 10 11",
//         // stole soeone's brute-force to find these diffs with my solution
//         "52 53 54 52 55",
//     };
//     try std.testing.expectEqual(n, try solveDay02(std.testing.allocator, &data, true));

//     const data_fail: [3]string = .{
//         "20 2 3 4 5 20",
//         "9 8 7 7 7",
//         // stole soeone's brute-force to find these diffs with my solution
//         "68 71 74 76 77 74 71",
//     };
//     try std.testing.expectEqual(0, try solveDay02(std.testing.allocator, &data_fail, true));
// }

// test "day02, final case" {
//     const n: usize = 1;
//     const data: [n]string = .{
//         "52 53 54 52 55",
//     };
//     try std.testing.expectEqual(n, try solveDay02(std.testing.allocator, &data, true));

//     const data_fail: [1]string = .{
//         "68 71 74 76 77 74 71",
//     };
//     try std.testing.expectEqual(0, try solveDay02(std.testing.allocator, &data_fail, true));
//     return error.SkipZigTest;
// }

fn solveDay01PartA(allocator: std.mem.Allocator, data: lines) !u64 {
    var total: u64 = 0;
    var left_list = std.ArrayList(u64).init(allocator);
    var right_list = std.ArrayList(u64).init(allocator);
    defer left_list.deinit();
    defer right_list.deinit();

    for (data) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');
        const left = try parseIntFromOptional(it.next());

        // consume any number of spaces
        var right: u64 = 0;
        while (it.next()) |val| {
            if (std.mem.eql(u8, val, "")) {
                continue;
            }

            right = try parseIntFromOptional(val);
            break;
        }

        try left_list.append(left);
        try right_list.append(right);

        if (it.peek() != null) {
            std.debug.print("Expect line to split into two values delimited by one or more spaces, got line {s} with trailing '{s}'\n", .{ line, it.rest() });
            return RuntimeError.TooMuchData;
        }
    }

    std.mem.sort(u64, left_list.items, {}, std.sort.asc(u64));
    std.mem.sort(u64, right_list.items, {}, std.sort.asc(u64));

    for (left_list.items, 0..) |left, i| {
        const right = right_list.items[i];

        if (left > right) {
            total = try std.math.add(u64, total, try std.math.sub(u64, left, right));
        } else if (right > left) {
            total = try std.math.add(u64, total, try std.math.sub(u64, right, left));
        }
    }

    return total;
}

fn solveDay01PartB(allocator: std.mem.Allocator, data: lines) !u64 {
    var total: u64 = 0;
    var left_list = std.ArrayList(u64).init(allocator);
    var right_map = std.AutoHashMap(u64, u64).init(allocator);
    defer left_list.deinit();
    defer right_map.deinit();

    for (data) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');
        const left = try parseIntFromOptional(it.next());

        // consume any number of spaces
        var right: u64 = 0;
        while (it.next()) |val| {
            if (std.mem.eql(u8, val, "")) {
                continue;
            }

            right = try parseIntFromOptional(val);
            break;
        }

        try left_list.append(left);

        var right_count: u64 = 1;
        if (right_map.get(right)) |val| {
            right_count += val;
        }
        try right_map.put(right, right_count);

        if (it.peek() != null) {
            std.debug.print("Expect line to split into two values delimited by one or more spaces, got line {s} with trailing '{s}'\n", .{ line, it.rest() });
            return RuntimeError.TooMuchData;
        }
    }

    for (left_list.items) |left| {
        if (right_map.get(left)) |right| {
            total = try std.math.add(u64, total, try std.math.mul(u64, left, right));
        }
    }

    return total;
}

const Day01Data = struct {
    left_data: []const u64,
    right_data: []const u64,
};

test "day01, part A" {
    const data: [6]string = .{
        "3 4",
        "4 3",
        "2 5",
        "1 3",
        "3 9",
        "3 3",
    };
    try std.testing.expectEqual(11, try solveDay01PartA(std.testing.allocator, &data));
}

test "day01, part B" {
    const data: [6]string = .{
        "3 4",
        "4 3",
        "2 5",
        "1 3",
        "3 9",
        "3 3",
    };
    try std.testing.expectEqual(31, try solveDay01PartB(std.testing.allocator, &data));
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
    // defer dir.close();
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
    OutOfBounds,
};

const RuntimeError = error{
    TooMuchData,
};

const string = []const u8;
const lines = []const string;
