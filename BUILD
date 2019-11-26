load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "release",
    srcs = glob(["src/**/*.rs", "src/*.rs"]),
    deps = [
    ],
    edition = "2018",
)
load('@io_bazel_rules_rust//rust:toolchain.bzl', 'rust_toolchain')

rust_toolchain(
    name = "x86_64-unknown-linux-gnu_impl",
    rust_doc = "@archiver_api//:rustdoc",
    rust_lib = "@archiver_api//:rust_lib-x86_64-unknown-linux-gnu_impl",
    rustc = "@archiver_api//:rustc",
    rustc_lib = "@archiver_api//:rustc_lib",
    staticlib_ext = ".a",
    dylib_ext = ".so",
    os = "linux",
    default_edition = "2018",
    exec_triple = "x86_64-unknown-linux-gnu_impl",
    target_triple = "x86_64-unknown-linux-gnu_impl",
    visibility = ["//visibility:public"],
)

#toolchain(
#  name = "x86_64-unknown-linux-gnu",
#  toolchain_type = "@io_bazel_rules_rust//rust:toolchain",
#  exec_compatible_with = [
#    "@platforms//cpu:cpuX",
#  ],
#  target_compatible_with = [
#    "@platforms//cpu:cpuX",
#  ],
#  toolchain = "@archiver_api//:x86_64-unknown-linux-gnu_impl",
#)
