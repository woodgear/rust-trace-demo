use failure::Fail;
use std::fmt::Display;
use sugar::SResultExt;

#[derive(Debug)]
pub enum ErrKind {
    AppFail,
    OtherFail,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.custom_msg)
    }
}

#[derive(Debug)]
struct MyError {
    err_kind: String,
    custom_msg: String,
    err: failure::Error,
}

impl MyError {
    pub fn new(err_kind: ErrKind, msg: String, err: failure::Error) -> Self {
        let err_kind = format!("{:?}", err_kind);
        Self {
            err_kind,
            custom_msg: msg,
            err,
        }
    }
}

impl Fail for MyError {
    fn name(&self) -> Option<&str> {
        Some(&self.err_kind)
    }

    fn cause(&self) -> Option<&dyn Fail> {
        Some(self.err.as_fail())
    }

    fn backtrace(&self) -> Option<&failure::Backtrace> {
        // Some(self.err.backtrace())
        None
    }
}
use sentry::{integrations::failure::exception_from_single_fail, protocol::Exception};

pub trait LogErrorExt {
    fn log_err(self, err_kind: ErrKind, msg: &str);
}

fn build_exceptions(err: &failure::Error) -> Vec<Exception> {
    let mut exceptions: Vec<Exception> = vec![];
    for (idx, cause) in err.iter_chain().enumerate() {
        let bt = match cause.backtrace() {
            Some(bt) => Some(bt),
            None => None,
        };
        exceptions.push(exception_from_single_fail(cause, bt));
    }

    let mut backtraces_set = std::collections::HashSet::new();
    for mut e in exceptions.iter_mut() {
        let trace_hash = calculate_hash(&format!("{:?}", e.stacktrace));
        if !backtraces_set.insert(trace_hash) {
            e.stacktrace = None;
            e.raw_stacktrace = None;
        }
    }
    exceptions
}

fn capture_error_to_log(err: &failure::Error) {
    let mut err_msg = "".to_string();
    let exceptions = build_exceptions(err);
    for (ei, e) in exceptions.iter().enumerate() {
        err_msg = format!("{} -> {}", err_msg, e.ty);
        if let Some(ref val) = e.value {
            err_msg = format!("{} {}", err_msg, val);
        }
        if let Some(ref b) = e.stacktrace {
            let first = {
                let mut first = None;

                for (i, f) in b.frames.iter().enumerate() {
                    if let Some(ref name) = f.function {
                        if name.starts_with("rust_trace_demo") && first.is_none() {
                            first = Some(i);
                            break;
                        }
                    }
                }
                first.unwrap_or_default()
            };
            for f in &b.frames[first..] {
                // for f in &b.frames {
                let function_name = f.function.clone().unwrap_or_default();
                let lineno = f.lineno.clone().unwrap_or_default();
                let colno = f.colno.clone().unwrap_or_default();
                let abs_path = f.abs_path.clone().unwrap_or_default();
                if abs_path.contains("failure-0.")
                    || abs_path.contains("libcore")
                    || abs_path.contains("backtrace-")
                    || abs_path.contains("src/errors.rs")
                    || abs_path.contains("sugar-rs")
                {
                    continue;
                }
                err_msg = format!(
                    "{} \n-> {}:{} {} {}|",
                    err_msg, abs_path, lineno, function_name, colno
                );
            }
            err_msg = format!("{}\n", err_msg);
        }
    }
    println!("{}", err_msg);
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

use sentry::integrations::failure::capture_error;
fn capture_error_to_sentry(e: &failure::Error) {
    use sentry::{Level,protocol::Event,Hub};
    let mut exceptions = build_exceptions(e);
    exceptions.reverse();
    let event = Event {
        exception: exceptions.into(),
        level: Level::Error,
        ..Default::default()
    };
    Hub::with_active(|hub| hub.capture_event(event));
    // capture_error(e);
}

impl<T> LogErrorExt for Result<T, failure::Error> {
    fn log_err(self, err_kind: ErrKind, msg: &str) {
        self.catch_err(|e| {
            let e = MyError::new(err_kind, msg.to_string(), e).into();
            // capture_error_to_log(&e);
            // capture_error_to_sentry(&e);
            // let e = e1.into();
            // println!("===> {:?}",e.backtrace());
            // let e: failure::Error = e.context(err_kind.to_string()).into();
            // println!("=====> {:?}",e.backtrace());

            // capture_error_to_sentry(&e.context(err_kind).into());
            // let e = e.context(err_kind).into();
            capture_error_to_log(&e);
            capture_error_to_sentry(&e);
        })
    }
}
