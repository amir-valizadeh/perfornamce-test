public class Main {
    public static int countPrimes(int limit) {
        int count = 0;
        for (int i = 2; i < limit; i++) {
            boolean isPrime = true;
            for (int j = 2; j <= i / 2; j++) {
                if (i % j == 0) {
                    isPrime = false;
                    break;
                }
            }
            if (isPrime) count++;
        }
        return count;
    }

    public static void main(String[] args) {
        long startTime = System.currentTimeMillis();
        int count = countPrimes(300000);
        long endTime = System.currentTimeMillis();
        System.out.println(count + " - " + (endTime - startTime) + "ms");
    }
}
