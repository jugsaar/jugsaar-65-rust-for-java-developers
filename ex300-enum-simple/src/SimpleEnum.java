public class SimpleEnum {
    
// java --enable-preview --source 21 SimpleEnum.java 

    public static void main(String[] args) {
        
        Activity activity = new Activity.Sleeping(6);
        var message = switch(activity) {
            case Activity.Sleeping(int hours) when hours > 8 yield "Wake up!";
            case Activity.Sleeping(int hours) yield "Shhhh";
            case Activity.Swimming(String location) yield "Awesome!";
            case Activity.Coding() yield "Java or Rust is fine!";
        }
        System.out.println(message);

    }

    sealed interface Activity {
        record Sleeping(int hours) implements Activity{}
        record Swimming(String location) implements Activity {}
        record Coding() implements Activity{}
    }
}


