public class MemoryCategories {
    public static void main(String[] args) {
        var x = new Object();
        x = new Object();
        var y = new Object();
        y = new Object();
        MemoryCategories.f();
    }

    private static void f() {
        var z = new Object();
    }
}
