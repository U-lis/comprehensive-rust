// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    // filter 가 log 를 감싸서 filtering을 하고, 통과한 것만 실제 logger 인 StderrLogger에 넘겨주는 패턴. 흔한 패턴이라고 함.
    // 같은 trait 를 구현하는 것으로 간단하게 감싸기를 할 수 있다.
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}