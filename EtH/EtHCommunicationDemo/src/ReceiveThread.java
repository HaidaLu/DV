import java.io.IOException;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.SocketException;


public class ReceiveThread implements Runnable {
    private DatagramSocket server;
    private String form;
    State state;


    public ReceiveThread(State state, int port,String from) {
        this.state = state;
        this.form=from;
        try {
            server=new DatagramSocket(port);
        } catch (SocketException e) {
            e.printStackTrace();
        }
    }

    @Override
    public void run() {
        while(true) {


            byte[]  container=new byte[1024*60];
            DatagramPacket packet=new DatagramPacket(container, 0,container.length);

            try {
                server.receive(packet);
                byte[] ciphertext =packet.getData();
                /**
                 * call a native JNI.receive function to decrypt the msg and update the State
                 */


                // JNI.receive(State, ciphertext); -> plaintext + newState
                //this.State = newState;
                //System.out.println("receive the ciphertext" + ciphertext);

                int len=packet.getLength();

                String data=new String(ciphertext,0,len);
                //String plaintextStr = new String(plaintext, 0, len); replace to plaintext after decrypt
                System.out.println(this.form+"says:"+data);
                //the plaintext
                if(data.equals("bye")) {
                    break;
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }

        server.close();

    }

}