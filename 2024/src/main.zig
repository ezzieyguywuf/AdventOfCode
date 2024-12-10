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
            const solutionA = try solveDay03(problem.data, false);
            std.debug.print("Solution, Day03, partA: {d}\n", .{solutionA});
            const solutionB = try solveDay03(problem.data, true);
            std.debug.print("Solution, Day03, partB: {d}\n", .{solutionB});
        },
        4 => {
            const solutionA = try solveDay04(allocator, problem.data);
            std.debug.print("Solution, Day04, partA: {d}\n", .{solutionA});
            const solutionB = try solveDay04B(allocator, problem.data);
            std.debug.print("Solution, Day04, partB: {d}\n", .{solutionB});
        },
        5 => {
            const solution = try solveDay05(allocator, problem.data);
            std.debug.print("Solution, Day05, partA: {d}\n", .{solution.partA});
            std.debug.print("Solution, Day05, partB: {d}\n", .{solution.partB});
        },
        6 => {
            // const solution = try solveDay05(allocator, problem.data);
            std.debug.print("Solution, Day06, partA: {d}\n", .{try solveDay06(allocator, problem.data)});
            // std.debug.print("Solution, Day06, partB: {d}\n", .{solution.partB});
        },
        else => {
            std.debug.print("I don't yet know how to solve day {d:02}\n", .{problem.day});
        },
    }
    return 0;
}

fn solveDay06(allocator: std.mem.Allocator, data: lines) !u64 {
    var dir: ?Direction = null;
    var row: usize = 0;
    var col: usize = 0;

    outer: for (data, 0..) |line, curRow| {
        row = curRow;
        // std.debug.print("row: {d}, line: {s}\n", .{ row, line });
        for (line, 0..) |char, curCol| {
            // std.debug.print("  dir: {?any}, col: {d}, char: {c}\n", .{ dir, col, char });
            col = curCol;
            switch (char) {
                '^' => dir = Direction.Up,
                '>' => dir = Direction.Right,
                'v' => dir = Direction.Down,
                '<' => dir = Direction.Left,
                else => continue,
            }
            if (dir != null) {
                break :outer;
            }
        }
    }

    var total: u64 = 0;
    var visited = std.AutoHashMapUnmanaged(Coord, void){};
    defer visited.deinit(allocator);
    while (true) {
        const coord = Coord{
            .row = row,
            .col = col,
        };
        if (!visited.contains(coord)) {
            total += 1;
            try visited.put(allocator, coord, {});
        }
        // std.debug.print("row: {d}, col: {d}, total: {d}, dir: {?s}\n", .{ row, col, total, std.enums.tagName(Direction, dir.?) });
        switch (dir.?) {
            Direction.Up => {
                if (row == 0) {
                    // std.debug.print("  breaking, row less than 0\n", .{});
                    break;
                }
                row -= 1;
            },
            Direction.Right => col += 1,
            Direction.Down => row += 1,
            Direction.Left => {
                if (col == 0) {
                    // std.debug.print("  breaking, col less than 0\n", .{});
                    break;
                }
                col -= 1;
            },
        }
        // std.debug.print("  new dir: {?s}\n", .{std.enums.tagName(Direction, dir.?)});
        if (row >= data.len or col >= data[0].len) {
            // std.debug.print("  breaking, row > data.len: {any}, col >= data[0].len: {any}\n", .{ row >= data.len, col >= data[0].len });
            break;
        }
        const char = data[row][col];
        if (char == '#') {
            switch (dir.?) {
                Direction.Up => {
                    row += 1;
                    dir = Direction.Right;
                },
                Direction.Right => {
                    col -= 1;
                    dir = Direction.Down;
                },
                Direction.Down => {
                    row -= 1;
                    dir = Direction.Left;
                },
                Direction.Left => {
                    col += 1;
                    dir = Direction.Up;
                },
            }
        }
        // std.debug.print("  newRow: {d}, newCol: {d}, newDir: {?s}\n", .{ row, col, std.enums.tagName(Direction, dir.?) });
    }

    return total;
}

test "Day 06, part A" {
    const data: [10]string = .{
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    };
    try std.testing.expectEqual(41, solveDay06(std.testing.allocator, &data));
}

const Solution = struct {
    partA: u64,
    partB: u64,
};

fn solveDay05(allocator: std.mem.Allocator, data: lines) !Solution {
    var parsedRules = try parseRules(allocator, data);
    defer {
        var it = parsedRules.rules.iterator();
        while (it.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        parsedRules.rules.deinit(allocator);
    }

    var i = parsedRules.line;
    var solution = Solution{
        .partA = 0,
        .partB = 0,
    };
    var invalids = std.ArrayListUnmanaged(OrderedSet){};
    defer {
        for (invalids.items) |*invalid| {
            invalid.deinit(allocator);
        }
        invalids.deinit(allocator);
    }

    while (i < data.len) {
        // the values that we've already seen. This memory is owned by the
        // invalids ArrayList defined in the outer scope of this function.
        var prev = OrderedSet{};
        errdefer prev.deinit(allocator);

        var isValid = true;
        // std.debug.print("Checking data {s}\n", .{data[i]});
        var it = std.mem.splitScalar(u8, data[i], ',');
        while (it.next()) |val| {
            // std.debug.print("  Got val: {s}\n", .{val});
            const key = try std.fmt.parseInt(u8, val, 10);
            // I call it "befores" because key must come before all of them
            if (parsedRules.rules.get(key)) |befores| {
                for (befores.keys()) |before| {
                    if (prev.contains(before)) {
                        // std.debug.print("    INVALID due to {d} exists before {d}\n", .{ before, key });
                        isValid = false;
                        // keep going, since we need the fully converted line
                        // for filling in the invalids ArrayList
                        // break;
                    }
                }
            }
            try prev.put(allocator, key, {});
        }

        if (isValid) {
            const middle: usize = prev.count() / 2;
            solution.partA += prev.keys()[middle];
            prev.deinit(allocator);
            // std.debug.print("    VALID adding {d} to total, total is now {d}\n", .{ prev.keys()[middle], total });
        } else {
            try invalids.append(allocator, prev);
        }
        i += 1;
    }

    for (invalids.items) |*sorted| {
        std.mem.sort(u64, sorted.keys(), parsedRules.rules, day05Cmp);
        const middle: usize = sorted.keys().len / 2;
        solution.partB += sorted.keys()[middle];
    }

    return solution;
}

fn day05Cmp(rules: Rules, left: u64, right: u64) bool {
    if (rules.get(left)) |entry| {
        return entry.contains(right);
    }

    return false;
}

test "day 05, part A" {
    const data: [28]string = .{
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    };

    const solution = try solveDay05(std.testing.allocator, &data);
    try std.testing.expectEqual(143, solution.partA);
    try std.testing.expectEqual(123, solution.partB);
}

const OrderedSet = std.AutoArrayHashMapUnmanaged(u64, void);
// The keys are the values that must come before the values, e.g.:
//  {
//    43: [12, 70],
//    50: [43, 45],
//  }
//
//  43 must come before 12 and 70
//  50 must come before 43 and 45
const Rules = std.AutoHashMapUnmanaged(u64, OrderedSet);

const ParsedRules = struct {
    rules: Rules,
    line: usize,
};
// Caller owns all the memory
fn parseRules(allocator: std.mem.Allocator, data: lines) !ParsedRules {
    var parsedRules = ParsedRules{
        .rules = .{},
        .line = 0,
    };
    for (data) |line| {
        parsedRules.line += 1;
        // std.debug.print("Got line #{d}: {s}\n", .{ parsedRules.line, line });
        if (std.mem.eql(u8, line, "")) {
            break;
        }
        var it = std.mem.splitScalar(u8, line, '|');
        const key = try std.fmt.parseInt(u64, it.next().?, 10);
        const before = try std.fmt.parseInt(u64, it.next().?, 10);
        if (it.next() != null) {
            std.debug.print("Expected each line to look like '42|69', instead found more than one '|'\n", .{});
            return RuntimeError.TooMuchData;
        }

        if (parsedRules.rules.getPtr(key)) |befores| {
            // This may be wasteful, since before may already exist. *shrug*
            try befores.put(allocator, before, {});
        } else {
            var befores = OrderedSet{};
            try befores.put(allocator, before, {});
            try parsedRules.rules.put(allocator, key, befores);
        }
    }

    return parsedRules;
}

fn solveDay04(allocator: std.mem.Allocator, data: lines) !u64 {
    // (row, col) location of each X
    var xLocs = std.ArrayListUnmanaged([2]usize){};
    defer xLocs.deinit(allocator);

    for (data, 0..) |row, i| {
        for (row, 0..) |char, j| {
            if (char == 'X') {
                try xLocs.append(allocator, .{ i, j });
            }
        }
    }

    var total: u64 = 0;
    for (xLocs.items) |coords| {
        total += countXmas(coords, data);
    }
    return total;
}

fn solveDay04B(allocator: std.mem.Allocator, data: lines) !u64 {
    // (row, col) location of each X
    var mLocs = std.ArrayListUnmanaged(Coord){};
    defer mLocs.deinit(allocator);

    for (data, 0..) |row, i| {
        for (row, 0..) |char, j| {
            if (char == 'M') {
                try mLocs.append(allocator, .{
                    .row = i,
                    .col = j,
                });
            }
        }
    }

    var masMap = std.AutoHashMapUnmanaged(Coord, []MatchedDiagonal){};
    defer {
        var it = masMap.iterator();
        while (it.next()) |diag| {
            allocator.free(diag.value_ptr.*);
        }
        masMap.deinit(allocator);
    }

    for (mLocs.items) |coord| {
        const diags = try findDiagonals(allocator, coord, data);
        if (diags.len > 0) {
            try masMap.put(allocator, coord, diags);
            // std.debug.print("  FOUND MAS at row: {d}, col: {d}\n", .{ coord.row, coord.col });
        }
    }

    var it = masMap.iterator();
    var total: u64 = 0;
    outer: while (it.next()) |entry| {
        const row = entry.key_ptr.row;
        const col = entry.key_ptr.col;
        // std.debug.print("Checking for Xes at row: {d}, col {d}\n", .{ row, col });

        for (entry.value_ptr.*) |*diagonal| {
            // std.debug.print("  Checking dir: {?s}\n", .{std.enums.tagName(Diagonal, diagonal.dir)});
            if (diagonal.isMatched) {
                // std.debug.print("  skipping b/c that diag is already matched\n", .{});
                continue;
            }
            var check1: ?Mas = null;
            var check2: ?Mas = null;
            switch (diagonal.dir) {
                Diagonal.UpLeft => {
                    check1 = Mas{
                        .coord = Coord{
                            .row = row,
                            .col = col - 2,
                        },
                        .dir = Diagonal.UpRight,
                    };
                    check2 = Mas{
                        .coord = Coord{
                            .row = row - 2,
                            .col = col,
                        },
                        .dir = Diagonal.DownLeft,
                    };
                },
                Diagonal.UpRight => {
                    check1 = Mas{
                        .coord = Coord{
                            .row = row,
                            .col = col + 2,
                        },
                        .dir = Diagonal.UpLeft,
                    };
                    check2 = Mas{
                        .coord = Coord{
                            .row = row - 2,
                            .col = col,
                        },
                        .dir = Diagonal.DownRight,
                    };
                },
                Diagonal.DownRight => {
                    check1 = Mas{
                        .coord = Coord{
                            .row = row,
                            .col = col + 2,
                        },
                        .dir = Diagonal.DownLeft,
                    };
                    check2 = Mas{
                        .coord = Coord{
                            .row = row + 2,
                            .col = col,
                        },
                        .dir = Diagonal.UpRight,
                    };
                },
                Diagonal.DownLeft => {
                    check1 = Mas{
                        .coord = Coord{
                            .row = row,
                            .col = col - 2,
                        },
                        .dir = Diagonal.DownRight,
                    };
                    check2 = Mas{
                        .coord = Coord{
                            .row = row + 2,
                            .col = col,
                        },
                        .dir = Diagonal.UpLeft,
                    };
                },
            }
            // std.debug.print("  check1 row: {d}, col: {d}\n", .{ check1.?.coord.row, check1.?.coord.col });
            if (masMap.getPtr(check1.?.coord)) |checkDiags| {
                // std.debug.print("    FOUND in masMap\n", .{});
                for (checkDiags.*) |*checkDiag| {
                    // std.debug.print("    checking diag: {?s}\n", .{std.enums.tagName(Diagonal, checkDiag.dir)});
                    if (checkDiag.dir == check1.?.dir and checkDiag.isMatched == false) {
                        checkDiag.*.isMatched = true;
                        diagonal.*.isMatched = true;
                        total += 1;
                        continue :outer;
                    }
                }
            }
            // std.debug.print("  check2 row: {d}, col: {d}\n", .{ check2.?.coord.row, check2.?.coord.col });
            if (masMap.getPtr(check2.?.coord)) |checkDiags| {
                // std.debug.print("    FOUND in masMap\n", .{});
                for (checkDiags.*) |*checkDiag| {
                    // std.debug.print("    checking diag: {?s}\n", .{std.enums.tagName(Diagonal, checkDiag.dir)});
                    if (checkDiag.dir == check2.?.dir and checkDiag.isMatched == false) {
                        checkDiag.*.isMatched = true;
                        diagonal.*.isMatched = true;
                        total += 1;
                        continue :outer;
                    }
                }
            }
        }
    }
    return total;
}

fn findDiagonals(allocator: std.mem.Allocator, coord: Coord, data: lines) ![]MatchedDiagonal {
    var diags = std.ArrayListUnmanaged(MatchedDiagonal){};
    defer diags.deinit(allocator);

    if (searchForMas(coord, data, Diagonal.UpLeft)) {
        try diags.append(allocator, .{
            .dir = Diagonal.UpLeft,
            .isMatched = false,
        });
    }
    if (searchForMas(coord, data, Diagonal.UpRight)) {
        try diags.append(allocator, .{
            .dir = Diagonal.UpRight,
            .isMatched = false,
        });
    }
    if (searchForMas(coord, data, Diagonal.DownRight)) {
        try diags.append(allocator, .{
            .dir = Diagonal.DownRight,
            .isMatched = false,
        });
    }
    if (searchForMas(coord, data, Diagonal.DownLeft)) {
        try diags.append(allocator, .{
            .dir = Diagonal.DownLeft,
            .isMatched = false,
        });
    }

    return diags.toOwnedSlice(allocator);
}

fn searchForMas(coord: Coord, data: lines, dir: Diagonal) bool {
    const MAS = "MAS";
    var targetIndex: usize = 0;
    var row = coord.row;
    var col = coord.col;

    while (targetIndex < 3 and row >= 0 and row < data.len and col >= 0 and col < data[0].len) {
        const char = data[row][col];
        if (char == MAS[targetIndex]) {
            targetIndex += 1;
        } else {
            return false;
        }
        if (targetIndex == 3) {
            return true;
        }
        switch (dir) {
            Diagonal.UpLeft => {
                if (row == 0) {
                    return false;
                }
                row -= 1;

                if (col == 0) {
                    return false;
                }

                col -= 1;
            },
            Diagonal.UpRight => {
                if (row == 0) {
                    return false;
                }

                row -= 1;
                col += 1;
            },
            Diagonal.DownRight => {
                row += 1;
                col += 1;
            },
            Diagonal.DownLeft => {
                if (col == 0) {
                    return false;
                }

                col -= 1;
                row += 1;
            },
        }
    }

    return false;
}

const Coord = struct {
    row: usize,
    col: usize,
};

const Mas = struct {
    coord: Coord,
    dir: Diagonal,
};

const MatchedDiagonal = struct {
    isMatched: bool,
    dir: Diagonal,
};

const Diagonal = enum {
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
};

fn countXmas(coords: [2]usize, data: lines) u64 {
    var total: u64 = 0;

    // std.debug.print("Checking row: {d}, col{d}\n", .{ coords[0], coords[1] });
    if (checkDirection(coords, data, .{ Direction.Up, Direction.Left })) {
        // std.debug.print("  match UpLeft\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ Direction.Up, null })) {
        // std.debug.print("  match Up\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ Direction.Up, Direction.Right })) {
        // std.debug.print("  match UpRight\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ null, Direction.Right })) {
        // std.debug.print("  match Right\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ Direction.Down, Direction.Right })) {
        // std.debug.print("  match DownRight\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ Direction.Down, null })) {
        // std.debug.print("  match Down\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ Direction.Down, Direction.Left })) {
        // std.debug.print("  match DownLeft\n", .{});
        total += 1;
    }
    if (checkDirection(coords, data, .{ null, Direction.Left })) {
        // std.debug.print("  match Left\n", .{});
        total += 1;
    }

    return total;
}

fn checkDirection(coords: [2]usize, data: lines, dir: [2]?Direction) bool {
    const xmas = "XMAS";
    var checkIndex: usize = 0;
    var row = coords[0];
    var col = coords[1];

    // std.debug.print("    want {c} , checkIndex: {d}, row: {d}, col: {d}, dir[0]: {?s}, dir[1]: {?s}, data.len: {d}, data[0].len: {d}\n", .{
    //     xmas[checkIndex],
    //     checkIndex,
    //     row,
    //     col,
    //     if (dir[0]) |val| std.enums.tagName(Direction, val) else "null",
    //     if (dir[1]) |val| std.enums.tagName(Direction, val) else "null",
    //     data.len,
    //     data[0].len,
    // });
    while (checkIndex < 4 and row >= 0 and row < data.len and col >= 0 and col < data[0].len) {
        const char = data[row][col];
        // std.debug.print("    want {c} have {c}, checkIndex: {d}, row: {d}, col: {d}, dir[0]: {?s}, dir[1]: {?s}\n", .{
        //     xmas[checkIndex],
        //     char,
        //     checkIndex,
        //     row,
        //     col,
        //     if (dir[0]) |val| std.enums.tagName(Direction, val) else "null",
        //     if (dir[1]) |val| std.enums.tagName(Direction, val) else "null",
        // });
        if (char == xmas[checkIndex]) {
            // std.debug.print("    MATCH row: {d}, col{d}, char: {c}\n", .{
            //     row,
            //     col,
            //     char,
            // });
            checkIndex += 1;
        } else {
            return false;
        }
        if (checkIndex == 4) {
            return true;
        }
        if (dir[0]) |val| {
            switch (val) {
                Direction.Up => {
                    if (row == 0) {
                        return false;
                    }
                    row -= 1;
                },
                Direction.Down => row += 1,
                else => {},
            }
        }
        if (dir[1]) |val| {
            switch (val) {
                Direction.Left => {
                    if (col == 0) {
                        return false;
                    }

                    col -= 1;
                },
                Direction.Right => col += 1,
                else => {},
            }
        }
    }
    return false;
}

const Direction = enum {
    Up,
    Right,
    Down,
    Left,
};

test "day 04, sample" {
    const data: [10]string = .{
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    };
    try std.testing.expectEqual(18, try solveDay04(std.testing.allocator, &data));
}

test "day 04, sample B" {
    const data: [10]string = .{
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    };
    try std.testing.expectEqual(9, try solveDay04B(std.testing.allocator, &data));
}

fn solveDay03(data: lines, withEnableDisable: bool) !u64 {
    var total: u64 = 0;
    var enabled: bool = true;
    // slide a window searching for `mul(`
    for (data) |line| {
        var i: usize = 0;
        while (i < line.len) {
            if (withEnableDisable and !enabled) {
                if (i >= line.len - 4) {
                    break;
                }

                const do = line[i .. i + 4];
                if (std.mem.eql(u8, do, "do()")) {
                    enabled = true;
                    i = incrementIndex(i, 3, line) catch break;
                }
            }
            if (!enabled) {
                i = incrementIndex(i, 1, line) catch break;
                continue;
            }
            if (withEnableDisable and enabled) {
                // std.debug.print("  Checking for don't\n", .{});
                if (i < line.len - 7) {
                    const dont = line[i .. i + 7];
                    // std.debug.print("  Checking string {s}\n", .{dont});
                    if (std.mem.eql(u8, dont, "don't()")) {
                        // std.debug.print("  got don't!\n", .{});
                        enabled = false;
                        i = incrementIndex(i, 6, line) catch break;
                        continue;
                    }
                }
            }

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
                continue;
            }
            if (consumedInteger1.i >= line.len) break;

            // match ,
            if (line[consumedInteger1.i] != ',') {
                // std.debug.print("  did not find , at i = {d}, moving on\n", .{consumedInteger1.i});
                continue;
            }

            // match ###
            const consumedInteger2 = consumeInteger(consumedInteger1.i, line);
            if (!consumedInteger2.has_data) {
                // std.debug.print("  did not find number at i = {d}, moving on\n", .{consumedInteger2.i});
                continue;
            }
            if (consumedInteger2.i >= line.len) break;

            // match )
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
            // std.debug.print("mul({d},{d})\n", .{
            //     consumedInteger1.val,
            //     consumedInteger2.val,
            // });

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
        // std.debug.print("  n == 3, returning\n", .{});
        return out;
    }
    if (n == 0) {
        out.i = incrementIndex(out.i, 1, data) catch return out;
    }

    // std.debug.print("  trying to parse '{s}'\n", .{operand});
    const num = std.fmt.parseInt(u64, &operand, 10) catch return out;
    // std.debug.print("  Found number, {d}\n", .{num});

    out.val = num;
    out.has_data = true;

    // std.debug.print("  returning out.i: {d}\n", .{out.i});

    return out;
}

test "day03, sample" {
    const dataA: [1]string = .{
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    };

    try std.testing.expectEqual(161, try solveDay03(&dataA, false));

    const dataB: [1]string = .{
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    };
    try std.testing.expectEqual(48, try solveDay03(&dataB, true));
}

test "day03, real life" {
    const data: [1]string = .{
        "$  mul(402,190))&<why(",
    };

    try std.testing.expectEqual(76380, try solveDay03(&data, false));
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

    // see
    // https://discord.com/channels/605571803288698900/1314058119067734089/1314060567576580116
    // for more context/details
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var out = std.ArrayListUnmanaged(string){};
    defer {
        for (out.items) |line| allocator.free(line);
        out.deinit(allocator);
    }
    var line = std.ArrayListUnmanaged(u8){};
    defer line.deinit(allocator);

    while (in_stream.streamUntilDelimiter(line.writer(allocator), '\n', null)) {
        defer line.clearRetainingCapacity();
        try out.append(allocator, try line.toOwnedSlice(allocator));
    } else |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    }

    return out.toOwnedSlice(allocator);
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
