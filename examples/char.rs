
fn invert_array(arr: [char; 4]) -> [char; 4] {
    let mut inverted: [char; 4] = arr;
    inverted.reverse(); // Inverte o array no lugar
    inverted
}


fn main(){

    let emoji_1:char = '🎀';
    let emoji_2:char = '🦄';
    let emoji_3:char = '🐜';
    let emoji_4:char = '🐪';

    let emojis: [char; 4] = [emoji_1, emoji_2, emoji_3, emoji_4];
    
    println!("\t --> {}, {}, {}, {}\n", emoji_1, emoji_2, emoji_3, emoji_4);
    println!("Original array: {:?}", emojis);

    let inverted_emoji_array: [char; 4] = invert_array(emojis);

    println!("Inverted array: {:?}", inverted_emoji_array);
}