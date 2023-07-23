package demo;

import demo.ffi.lib_h;

/**
 * --enable-preview --enable-native-access=ALL-UNNAMED -Djava.library.path=../rust-java-lib/target/debug
 */
public class CallRustFunction {

    static {
        System.loadLibrary("rust_java_lib"); // librust_java_lib.so
    }

    public static void main(String[] args) {

        lib_h.hello_world();

        long sum = lib_h.compute_sum(3, 4);
        System.out.println(sum);
    }
}
