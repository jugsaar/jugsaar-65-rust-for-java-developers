package demo;

import demo.ffi.RustLib;

/**
 * --enable-preview --enable-native-access=ALL-UNNAMED -Djava.library.path=../rust-java-lib/target/debug
 */
public class CallRustFunction {

    public static void main(String[] args) {

        RustLib.hello_world();

//        long sum = lib_h.compute_sum(3, 4);
//        System.out.println(sum);
    }
}
