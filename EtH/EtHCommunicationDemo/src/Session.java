public class Session {
    static State st_a;
    static State st_b;

    public Session() {
        /**
         * call a native JNI.initall function to build the constructor and get the s/r states
         */

        /*
        if(st_a == null || st_b == null){
            JNI.initall();  -> return st_a, st_b
            this.st_a = st_a
            this.st_b = st_b
        }
        */
    }

    public void Send(State state, int port, String toIP, int toPort) {
        new Thread(new SendThread(state, port,toIP,toPort)).start();
    }
    public void Receive(State state, int port,String from) {
        new Thread(new ReceiveThread(state, port,from)).start();
    }
}
