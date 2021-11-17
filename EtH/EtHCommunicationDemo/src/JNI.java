public class JNI {

    /**
     * initall
     */
    private static native long initall();

    /**
     * send
     */
    public static native long send(long stateText);

    /**
     * receive
     */
    public static native long receive(long stateText);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("EtHlib");
    }
}
