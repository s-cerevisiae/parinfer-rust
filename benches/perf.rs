use criterion::{criterion_group, criterion_main, Criterion};
use serde_json::json;
use std::ffi::CString;

const LONG_MAP_WITH_STRINGS: &str = include_str!("perf/long_map_with_strings");
const REALLY_LONG_FILE: &str = include_str!("perf/really_long_file");
const REALLY_LONG_FILE_WITH_UNCLOSED_PAREN: &str =
    include_str!("perf/really_long_file_with_unclosed_paren");
const REALLY_LONG_FILE_WITH_UNCLOSED_QUOTE: &str =
    include_str!("perf/really_long_file_with_unclosed_quote");

fn build_case(mode: &str, text: &str) -> CString {
    unsafe { parinfer_rust::INITIALIZED = true };
    CString::new(
        json!({
            "mode": mode,
            "text": text,
            "options": {
                "forceBalance": false,
                "partialResult": false,
                "returnParens": false
            }
        })
        .to_string(),
    )
    .unwrap()
}

fn bench_paren_long_map_with_strings(c: &mut Criterion) {
    unsafe {
        let options = build_case("paren", LONG_MAP_WITH_STRINGS);
        c.bench_function("bench_paren_long_map_with_strings", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_indent_long_map_with_strings(c: &mut Criterion) {
    unsafe {
        let options = build_case("indent", LONG_MAP_WITH_STRINGS);
        c.bench_function("bench_indent_long_map_with_strings", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_smart_long_map_with_strings(c: &mut Criterion) {
    unsafe {
        let options = build_case("smart", LONG_MAP_WITH_STRINGS);
        c.bench_function("bench_smart_long_map_with_strings", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_paren_really_long_file(c: &mut Criterion) {
    unsafe {
        let options = build_case("paren", REALLY_LONG_FILE);
        c.bench_function("bench_paren_really_long_file", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_indent_really_long_file(c: &mut Criterion) {
    unsafe {
        let options = build_case("indent", REALLY_LONG_FILE);
        c.bench_function("bench_indent_really_long_file", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_smart_really_long_file(c: &mut Criterion) {
    unsafe {
        let options = build_case("smart", REALLY_LONG_FILE);
        c.bench_function("bench_smart_really_long_file", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_paren_really_long_file_with_unclosed_paren(c: &mut Criterion) {
    unsafe {
        let options = build_case("paren", REALLY_LONG_FILE_WITH_UNCLOSED_PAREN);
        c.bench_function("bench_paren_really_long_file_with_unclosed_paren", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_indent_really_long_file_with_unclosed_paren(c: &mut Criterion) {
    unsafe {
        let options = build_case("indent", REALLY_LONG_FILE_WITH_UNCLOSED_PAREN);
        c.bench_function("bench_indent_really_long_file_with_unclosed_paren", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_smart_really_long_file_with_unclosed_paren(c: &mut Criterion) {
    unsafe {
        let options = build_case("smart", REALLY_LONG_FILE_WITH_UNCLOSED_PAREN);
        c.bench_function("bench_smart_really_long_file_with_unclosed_paren", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_quote_really_long_file_with_unclosed_quote(c: &mut Criterion) {
    unsafe {
        let options = build_case("paren", REALLY_LONG_FILE_WITH_UNCLOSED_QUOTE);
        c.bench_function("bench_quote_really_long_file_with_unclosed_quote", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_indent_really_long_file_with_unclosed_quote(c: &mut Criterion) {
    unsafe {
        let options = build_case("indent", REALLY_LONG_FILE_WITH_UNCLOSED_QUOTE);
        c.bench_function("bench_indent_really_long_file_with_unclosed_quote", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

fn bench_smart_really_long_file_with_unclosed_quote(c: &mut Criterion) {
    unsafe {
        let options = build_case("smart", REALLY_LONG_FILE_WITH_UNCLOSED_QUOTE);
        c.bench_function("bench_smart_really_long_file_with_unclosed_quote", |b| {
            b.iter(|| parinfer_rust::run_parinfer(options.as_ptr()))
        });
    }
}

criterion_group!(
    benches,
    bench_paren_long_map_with_strings,
    bench_indent_long_map_with_strings,
    bench_smart_long_map_with_strings,
    bench_paren_really_long_file,
    bench_indent_really_long_file,
    bench_smart_really_long_file,
    bench_paren_really_long_file_with_unclosed_paren,
    bench_indent_really_long_file_with_unclosed_paren,
    bench_smart_really_long_file_with_unclosed_paren,
    bench_quote_really_long_file_with_unclosed_quote,
    bench_indent_really_long_file_with_unclosed_quote,
    bench_smart_really_long_file_with_unclosed_quote,
);

criterion_main!(benches);
