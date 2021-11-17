public class Alice {
    public static void main(String[] args) {
        Session session = new Session();
        session.Send(Session.st_a,7878,"localhost",8989);
        session.Receive(Session.st_b,9696,"Bob");
    }
}
