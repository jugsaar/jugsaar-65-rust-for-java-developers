cargo new rust-java-lib --lib

cargo build

# wget https://download.java.net/java/early_access/jextract/1/openjdk-20-jextract+1-2_linux-x64_bin.tar.gz
# jextract -d classes -t org.openjdk --include-function hello_world -l rust_panama_helloworld -- lib.h

scratch/jextract-20/bin/jextract \
  --output ../rust-java-demo/src/main/java \
  --source \
  --target-package demo.ffi \
  --include-function hello_world \
  --include-function compute_sum \
  --library rust_panama \
  lib.h