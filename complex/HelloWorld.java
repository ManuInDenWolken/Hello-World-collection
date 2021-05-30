import java.util.Arrays;

public final class HelloWorldComplex {

    public static final Character[] HELLO_WORLD_CHARS = {
            'H',
            'e',
            'l',
            'l',
            'o',
            ' ',
            'W',
            'o',
            'r',
            'l',
            'd',
            '!'
    };

    public static void main(String[] args) {
        System.out.println(getHelloWorld());
    }

    public static String getHelloWorld() {
        return assembleString(HELLO_WORLD_CHARS);
    }

    public static String assembleString(Character[] array) {
        return Arrays.stream(array).reduce("", (accumulator, object) -> accumulator + object, (s, s2) -> s + " " + s2);
    }
}