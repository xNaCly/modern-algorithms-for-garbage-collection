// see Garbage Collection/Scope
class Scope {
    static class Test {
    }

    public static void main(String[] args) {
        var test1 = new Scope.Test();
        var test2 = new Scope.Test();
    }
}
