import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetSocketAddress;
import java.net.SocketException;


public class SendThread implements Runnable {
    private DatagramSocket client;
    private BufferedReader reader;
    private String toIP;
    private int toPort;
    State state;

    public SendThread(State state, int port, String toIP, int toPort) {
        this.state = state;

        this.toIP = toIP;
        this.toPort = toPort;
        try {
            client = new DatagramSocket(port);
            reader = new BufferedReader(new InputStreamReader(System.in));
        } catch (SocketException e) {
            // TODO Auto-generated catch ble.printStackTrace();
        }
    }

    @Override
    public void run() {
        while (true) {
            String data;
            try {

                data = reader.readLine();
                byte[] plaintext = data.getBytes();
                /**
                 * call a native JNI.send function to obtain a ciphertext and update the State
                 */

                //JNI.send(State, plaintext); -> ciphertext + newState
                //System.out.println("the ciphertext is: " + ciphertext);
                DatagramPacket packet = new DatagramPacket(plaintext, 0, plaintext.length, new InetSocketAddress(this.toIP, this.toPort));
                /**
                 * send the ciphertext
                 */
                //DatagramPacket packet = new DatagramPacket(ciphertext, 0, ciphertext.length, new InetSocketAddress(this.toIP, this.toPort));
                //this.State = newState;

                client.send(packet);
                if (data.equals("bye")) {
                    break;
                }
            } catch (IOException e) {
                e.printStackTrace();
            }

        }

        client.close();


    }
}
