public class Bob {
    public static void main(String[] args) {
        Session session = new Session();
        session.Receive(Session.st_a,8989,"Alice");
        session.Send(Session.st_b,5554,"localhost",9696);
    }
}
