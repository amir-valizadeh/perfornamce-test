public class MainTest {
    public static void main(String[] args) {
        assert Main.countPrimes(0) == 0 : "Expected 0";
        assert Main.countPrimes(2) == 0 : "Expected 0";
        assert Main.countPrimes(10) == 4 : "Expected 4 but got " + Main.countPrimes(10);
        assert Main.countPrimes(20) == 8 : "Expected 8";
        assert Main.countPrimes(100) == 25 : "Expected 25";
        System.out.println("Java tests passed.");
    }
}
