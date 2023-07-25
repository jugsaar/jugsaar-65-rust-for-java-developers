package demo.ffi;

import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.SymbolLookup;
import java.lang.invoke.MethodHandle;

import static java.lang.foreign.ValueLayout.JAVA_LONG;

public class RustLib {

    private static final Linker LINKER = Linker.nativeLinker();
    private static final SymbolLookup SYMBOL_LOOKUP;

    static {
        System.loadLibrary("rust_java_lib"); // librust_java_lib.so

        SymbolLookup loaderLookup = SymbolLookup.loaderLookup();
        SYMBOL_LOOKUP = name -> loaderLookup.find(name).or(() -> LINKER.defaultLookup().find(name));
    }

    private static MethodHandle downcallHandle(String symbolName, FunctionDescriptor fdesc) {
        return SYMBOL_LOOKUP //
                .find(symbolName) //
                .map(addr -> LINKER.downcallHandle(addr, fdesc)) //
                .orElseThrow(() -> new UnsatisfiedLinkError("unresolved symbol: " + symbolName));
    }

    /**
     * {@snippet :
     * void hello_world();
     *}
     */
    public static void hello_world() {

        class Holder {
            private static final MethodHandle mh = downcallHandle("hello_world", FunctionDescriptor.ofVoid());
        }

        try {
            Holder.mh.invokeExact();
        } catch (Throwable ex) {
            throw new AssertionError("should not reach here", ex);
        }
    }

    /**
     * {@snippet :
     * uintptr_t compute_sum(uintptr_t left, uintptr_t right);
     *}
     */
    public static long compute_sum(long left, long right) {

        class Holder {
            private static final MethodHandle mh = downcallHandle("compute_sum", FunctionDescriptor.of(JAVA_LONG, JAVA_LONG, JAVA_LONG));
        }

        try {
            return (long) Holder.mh.invokeExact(left, right);
        } catch (Throwable ex) {
            throw new AssertionError("should not reach here", ex);
        }
    }

}
