public class HelloJNI {
    static {
        System.loadLibrary("hello");
    }

    private native void sayHello();
    private native double average(int a, int b);
    private native void callFromRust();

    public static void main(String[] args) {
        HelloJNI jni = new HelloJNI();
            
        jni.sayHello();
        double avg = jni.average(5, 10);
        System.out.print("The average is: ");
        System.out.println(avg);

        jni.callFromRust();
    }

    void showFromJava() {
        System.out.println("This method is in Java");
    }
}
