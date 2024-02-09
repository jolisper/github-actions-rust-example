/* A Marco Polo Game

If the name Marco is given, the program will responds with "Polo".
Otherwise, the program will respond with "What is your name?".

*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What is your name?".to_string()
    }
}
