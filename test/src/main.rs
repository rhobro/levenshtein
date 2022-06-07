use levenshtein::measure;

fn main() {
    let src = "Johnsons Pure & Gentle Daily Care Baby Oil - 300ml";
    let cmps = r#"3 x 200ml Baby Oil Pure & Gentle Daily Care Massage Mild Formula Lock In Moisture Delicate Skin Dryness Hypoallergenic
Johnson's Baby Oil x 1
Baby Oil For The Body 300 Ml
Johnson's Pure and Gentle Baby Oil, 300 ml
Johnson's Baby Oil, 300ml, White"#.split("\n");

    for c in cmps {
        println!("{} <-- {:#?}", measure(src, c), c);
    }
}
