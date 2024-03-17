fn main() {
    let mut words1 = [""; 3];

    words1[0] = "Hello";
    words1[1] = "I'm";
    words1[2] = "getting";

    let words2 = ["World!", "just", "started."];

    let phrase = [ words1[0], words2[0], words1[1], words2[1], words1[2], words2[2] ];

    // investigate "join"
    println!("Phrase {}", phrase.join(" "));
    
}