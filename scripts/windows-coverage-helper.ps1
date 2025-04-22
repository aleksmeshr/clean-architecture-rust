# This file is a Windows-friendly coverage solution based on the approach documented here:
# https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html
# Origin: https://github.com/cooperwalbrun/rust-template/blob/master/scripts/windows-coverage-helper.ps1

$html_output_directory = "./coverage/"
$profile_data_file = "./tests.profdata"
$coverage_ignore_regex = '\.cargo.registry|\.rustup' # Matches .cargo/registry and .rustup/

# Generate log messages via Cargo that we can parse to extract the executables' paths
# Note that this command will not actually run any tests
$Env:RUSTFLAGS = "-C instrument-coverage"
$json = cargo test --lib --no-run --message-format=json | ConvertFrom-Json

# Extract the names of the executable(s) that we instrumented for code coverage analysis
$artifacts = $json | Where-Object { $_.profile.test -eq $TRUE }
$executables = @()
ForEach ($artifact in $artifacts) {
  ForEach ($filename in $artifact.filenames) {
    if ([IO.Path]::GetExtension($filename) -eq ".exe") {
      $executables += "$filename"
    }
  }
}
$executables = $executables -Join " -object "

# Show the coverage stats
cargo cov -- report -ignore-filename-regex "\.cargo.registry|\.rustup" -instr-profile "tests.profdata" $executables

# Create the HTML report with in-depth coverage information
cargo cov -- show `
  -Xdemangler=rustfilt `
  -ignore-filename-regex "$coverage_ignore_regex" `
  -show-line-counts-or-regions `
  -format html `
  -output-dir "$html_output_directory" `
  -instr-profile "$profile_data_file" `
  $executables
