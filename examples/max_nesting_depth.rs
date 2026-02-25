#![allow(unused_variables, dead_code, unused_must_use, clippy::all)]
#![allow(max_lines_per_file, unused_async)]

// --- Should NOT trigger (depth exactly 4) ---

fn four_for_loops_ok() {
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                for _d in 0..3 {
                    // depth 4 – OK
                }
            }
        }
    }
}

fn three_for_one_if_ok() {
    let x = 1;
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                if x == 1 {
                    // depth 4 – OK
                }
            }
        }
    }
}

// else-if chain: the whole chain is ONE nesting level.
fn else_if_chain_ok() {
    let x = 1;
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                if x == 1 {
                    // depth 4 – OK
                } else if x == 2 {
                    // still depth 4 (same chain)
                } else if x == 3 {
                    // still depth 4
                } else {
                    // still depth 4
                }
            }
        }
    }
}

// while loop counts as ONE nesting level (not double-counted via desugaring).
fn while_loops_ok() {
    let mut i = 0;
    while i < 3 {
        while i < 3 {
            while i < 3 {
                while i < 3 {
                    // depth 4 – OK
                    i += 1;
                }
                i += 1;
            }
            i += 1;
        }
        i += 1;
    }
}

// match counts as one level.
fn match_ok() {
    let x = 1;
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                match x {
                    // depth 4 – OK
                    _ => {}
                }
            }
        }
    }
}

// closure counts as one level.
fn closure_ok() {
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                let _f = || {
                    // depth 4 – OK
                };
            }
        }
    }
}

// --- Should trigger (depth 5) ---

fn five_for_loops() {
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                for _d in 0..3 {
                    for _e in 0..3 {
                        // depth 5 – LINT
                    }
                }
            }
        }
    }
}

fn five_while_loops() {
    let mut i = 0;
    while i < 3 {
        while i < 3 {
            while i < 3 {
                while i < 3 {
                    while i < 3 {
                        // depth 5 – LINT
                        i += 1;
                    }
                    i += 1;
                }
                i += 1;
            }
            i += 1;
        }
        i += 1;
    }
}

fn if_at_depth_five() {
    let x = 1;
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                for _d in 0..3 {
                    if x == 1 {
                        // depth 5 – LINT
                    }
                }
            }
        }
    }
}

fn match_at_depth_five() {
    let x = 1;
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                for _d in 0..3 {
                    match x {
                        // depth 5 – LINT
                        _ => {}
                    }
                }
            }
        }
    }
}

fn closure_at_depth_five() {
    for _a in 0..3 {
        for _b in 0..3 {
            for _c in 0..3 {
                for _d in 0..3 {
                    let _f = || {
                        // depth 5 – LINT
                    };
                }
            }
        }
    }
}

fn main() {}
