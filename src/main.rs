// Rust学习程序
mod basics;

fn main() {
    println!("=== Rust学习程序 ===");
    println!("请修改main.rs文件以学习不同的知识点");
    
    // 取消注释下面的行来学习相应的知识点
    basics::variables::run();
    // basics::data_types::run();
    // basics::control_flow::run();
    // basics::ownership::run();
    // basics::structs::run();
}

fn hello_world(){
    let english ="Hello";
    let chinese="你好";
    let japanese="こんにちは";
    let german="Grüß Gott!";
    let french="Bonjour le monde!";
    let spanish="¡Hola, mundo!";
    let italian="Ciao, mondo!";
    let _korean="안녕하세요, 세계!";
    let _russian="Привет, мир!";
    let region_english="en";
    let region_chinese="zh";
    let region_japanese="ja";
    let region_german="de";
    let region_french="fr";
    let region_spanish="es";
    let region_italian="it";
    let region =[region_english,region_chinese,region_japanese,region_german,region_french,region_spanish,region_italian];
    let language =[english,chinese,japanese,german,french,spanish,italian];
    region.iter().zip(&language).for_each(|(r, l)| println!("{r}: {l}"));
}